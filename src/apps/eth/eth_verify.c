#include "eth_verify.h"
#include "eth_common.h"
#include "eth_params.h"

#include <hardfault.h>
#include <util.h>
#include <workflow/verify_recipient.h>
#include <workflow/verify_total.h>

#include <bignum.h>
#include <largeprime.h>

#define WEI_DECIMALS (18)

static void _bigendian_to_scalar(const uint8_t* bytes, size_t len, bignum256* out)
{
    if (len > 32) {
        Abort("_bigendian_to_scalar: unexpected size");
    }
    // bn_read_be requires a 32 byte big endian input, so we pad our big endian number to the
    // required size.
    uint8_t buf[32] = {0};
    memcpy(buf + sizeof(buf) - len, bytes, len);
    bn_read_be(buf, out);
}

typedef struct {
    const char* unit;
    unsigned int decimals;
    const bignum256* value;
} _amount_t;

static app_eth_sign_error_t _verify_recipient(const uint8_t* recipient, const _amount_t* amount)
{
    char address[APP_ETH_ADDRESS_HEX_LEN];
    if (!eth_common_hexaddress(recipient, address, sizeof(address))) {
        return APP_ETH_SIGN_ERR_UNKNOWN;
    }
    char formatted_value[100] = {0};
    eth_common_format_amount(
        amount->value, amount->unit, amount->decimals, formatted_value, sizeof(formatted_value));
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
    bignum256 gas_price_scalar;
    _bigendian_to_scalar(request->gas_price.bytes, request->gas_price.size, &gas_price_scalar);
    bignum256 gas_limit_scalar;
    _bigendian_to_scalar(request->gas_limit.bytes, request->gas_limit.size, &gas_limit_scalar);
    // result will be in gas_price_scalar
    bn_multiply(&gas_limit_scalar, &gas_price_scalar, bignum_largeprime());
    const bignum256* fee_scalar = &gas_price_scalar;
    char formatted_fee[100] = {0};
    eth_common_format_amount(
        fee_scalar, fee_unit, WEI_DECIMALS, formatted_fee, sizeof(formatted_fee));
    // total:
    bignum256 sum = *total->value;
    // If fee and total value are in the same unit, include the fee in the total.
    if (STREQ(fee_unit, total->unit)) {
        bn_add(&sum, fee_scalar);
    }
    char formatted_total[100] = {0};
    eth_common_format_amount(
        &sum, total->unit, total->decimals, formatted_total, sizeof(formatted_total));
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
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    const app_eth_erc20_params_t* erc20_params =
        app_eth_erc20_params_get(request->coin, request->recipient);
    if (erc20_params == NULL) {
        // unsupported token.
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
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
    bignum256 value_scalar;
    _bigendian_to_scalar(value, 32, &value_scalar);
    const _amount_t amount = {
        .unit = erc20_params->unit,
        .decimals = erc20_params->decimals,
        .value = &value_scalar,
    };
    app_eth_sign_error_t result = _verify_recipient(recipient, &amount);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    const _amount_t total = {
        .unit = erc20_params->unit,
        .decimals = erc20_params->decimals,
        .value = &value_scalar,
    };
    result = _verify_total_fee(request, &total, params->unit);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    return APP_ETH_SIGN_OK;
}

app_eth_sign_error_t app_eth_verify_standard_transaction(const ETHSignRequest* request)
{
    const app_eth_coin_params_t* params = app_eth_params_get(request->coin);
    if (params == NULL) {
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->value.size == 0) {
        // Must transfer non-zero value.
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    if (request->data.size != 0) {
        // Standard tx has no data.
        return APP_ETH_SIGN_ERR_INVALID_INPUT;
    }
    // a) recipient and value
    bignum256 value_scalar;
    _bigendian_to_scalar(request->value.bytes, request->value.size, &value_scalar);
    const _amount_t amount = {
        .unit = params->unit,
        .decimals = WEI_DECIMALS,
        .value = &value_scalar,
    };
    app_eth_sign_error_t result = _verify_recipient(request->recipient, &amount);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    // b) total and fee
    const _amount_t total = {
        .unit = params->unit,
        .decimals = WEI_DECIMALS,
        .value = &value_scalar,
    };
    result = _verify_total_fee(request, &total, params->unit);
    if (result != APP_ETH_SIGN_OK) {
        return result;
    }
    return APP_ETH_SIGN_OK;
}
