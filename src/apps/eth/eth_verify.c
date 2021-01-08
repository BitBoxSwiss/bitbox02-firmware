#include "eth_verify.h"
#include "eth_common.h"
#include "eth_params.h"

#include <hardfault.h>
#include <rust/rust.h>
#include <util.h>
#include <workflow/confirm.h>
#include <workflow/verify_recipient.h>
#include <workflow/verify_total.h>

#define WEI_DECIMALS (18)

typedef struct {
    const char* unit;
    unsigned int decimals;
    const Bytes value; // big endian encoded bignum, max 32 bytes.
} _amount_t;

static app_eth_sign_error_t _verify_recipient(const uint8_t* recipient, const _amount_t* amount)
{
    char address[APP_ETH_ADDRESS_HEX_LEN];

    rust_ethereum_address_from_pubkey_hash(
        rust_util_bytes(recipient, APP_ETH_RECIPIENT_BYTES_LEN),
        rust_util_cstr_mut(address, sizeof(address)));

    char formatted_value[100] = {0};
    if (amount != NULL) {
        eth_common_format_amount(
            amount->value,
            amount->unit,
            amount->decimals,
            formatted_value,
            sizeof(formatted_value));
    } else {
        snprintf(formatted_value, sizeof(formatted_value), "Unknown token");
    }
    // This call blocks.
    if (!workflow_verify_recipient(address, formatted_value)) {
        return APP_ETH_SIGN_ERR_USER_ABORT;
    }
    return APP_ETH_SIGN_OK;
}

static app_eth_sign_error_t _verify_total_fee(
    const ETHSignRequest* request,
    const _amount_t* total,
    const char* fee_unit)
{
    // fee: gas limit * gas price:
    uint8_t fee[32] = {0};
    rust_ethereum_bigint_mul(
        rust_util_bytes(request->gas_price.bytes, request->gas_price.size),
        rust_util_bytes(request->gas_limit.bytes, request->gas_limit.size),
        rust_util_bytes_mut(fee, sizeof(fee)));
    char formatted_fee[100] = {0};
    eth_common_format_amount(
        rust_util_bytes(fee, sizeof(fee)),
        fee_unit,
        WEI_DECIMALS,
        formatted_fee,
        sizeof(formatted_fee));
    // total:
    char formatted_total[100] = {0};
    if (total != NULL) {
        Bytes total_amount = rust_util_bytes(total->value.buf, total->value.len);
        uint8_t sum[32] = {0};
        // If fee and total value are in the same unit, include the fee in the total.
        if (STREQ(fee_unit, total->unit)) {
            rust_ethereum_bigint_add(
                total_amount,
                rust_util_bytes(fee, sizeof(fee)),
                rust_util_bytes_mut(sum, sizeof(sum)));
            total_amount = rust_util_bytes(sum, sizeof(sum));
        }
        eth_common_format_amount(
            total_amount, total->unit, total->decimals, formatted_total, sizeof(formatted_total));
    } else {
        snprintf(formatted_total, sizeof(formatted_total), "Unknown amount");
    }
    // This call blocks.
    if (!workflow_verify_total(formatted_total, formatted_fee)) {
        return APP_ETH_SIGN_ERR_USER_ABORT;
    }
    return APP_ETH_SIGN_OK;
}

// preconditions:
// 1) data starts with 0xa9059cbb and has a total size of 68 bytes.
// 2) value is 0.
app_eth_sign_error_t app_eth_verify_erc20_transaction(const ETHSignRequest* request)
{
    const app_eth_coin_params_t* params = app_eth_params_get(request->coin);
    if (params == NULL) {
        // TODO: This isn't technically invalid, it is just not supported
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    const app_eth_erc20_params_t* erc20_params =
        app_eth_erc20_params_get(request->coin, request->recipient);
    // data is validated to have the following format:
    // <0xa9059cbb><32 bytes recipient><32 bytes value>
    // where recipient 20 bytes, zero padded to 32 bytes, and value is zero padded big endian.
    size_t method = 4;
    const uint8_t zeroes[12] = {0};
    if (!MEMEQ(request->data.bytes + method, zeroes, sizeof(zeroes))) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    const uint8_t* recipient = request->data.bytes + method + 32 - APP_ETH_RECIPIENT_BYTES_LEN;
    const uint8_t* value = recipient + APP_ETH_RECIPIENT_BYTES_LEN;
    uint8_t empty[32] = {0};
    if (MEMEQ(value, empty, sizeof(empty))) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (erc20_params != NULL) {
        const _amount_t amount = {
            .unit = erc20_params->unit,
            .decimals = erc20_params->decimals,
            .value = rust_util_bytes(value, 32),
        };
        app_eth_sign_error_t result = _verify_recipient(recipient, &amount);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
        const _amount_t total = {
            .unit = erc20_params->unit,
            .decimals = erc20_params->decimals,
            .value = rust_util_bytes(value, 32),
        };
        result = _verify_total_fee(request, &total, params->unit);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
    } else {
        app_eth_sign_error_t result = _verify_recipient(recipient, NULL);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
        result = _verify_total_fee(request, NULL, params->unit);
        if (result != APP_ETH_SIGN_OK) {
            return result;
        }
    }
    return APP_ETH_SIGN_OK;
}

app_eth_sign_error_t app_eth_verify_standard_transaction(const ETHSignRequest* request)
{
    const app_eth_coin_params_t* params = app_eth_params_get(request->coin);
    if (params == NULL) {
        // TODO: This isn't technically invalid, it is just not supported
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->data.size == 0 && request->value.size == 0) {
        // TODO: This isn't technically invalid, it is just not supported
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->data.size != 0) {
        char hex[sizeof(request->data.bytes) * 2 + 1] = {0};
        util_uint8_to_hex(request->data.bytes, request->data.size, hex);
        confirm_params_t cparams = {
            .title = "Transaction\ndata",
            .body = hex,
            .scrollable = true,
            .display_size = request->data.size,
            .accept_is_nextarrow = true,
        };
        if (!workflow_confirm_blocking(&cparams)) {
            return APP_ETH_SIGN_ERR_USER_ABORT;
        }
    }
    // a) recipient and value
    const _amount_t amount = {
        .unit = params->unit,
        .decimals = WEI_DECIMALS,
        .value = rust_util_bytes(request->value.bytes, request->value.size),
    };
    app_eth_sign_error_t result = _verify_recipient(request->recipient, &amount);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    // b) total and fee
    const _amount_t total = {
        .unit = params->unit,
        .decimals = WEI_DECIMALS,
        .value = rust_util_bytes(request->value.bytes, request->value.size),
    };
    result = _verify_total_fee(request, &total, params->unit);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    return APP_ETH_SIGN_OK;
}
