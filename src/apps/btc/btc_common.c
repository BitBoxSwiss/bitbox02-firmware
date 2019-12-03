// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <stdio.h>

#include "btc_common.h"

#include <util.h>
#include <wally_address.h>

// keypath_len is assumed to be greater or equal than 3.
static bool _validate_keypath_account(const uint32_t* keypath, uint32_t expected_coin)
{
    uint32_t coin = keypath[1];
    uint32_t account = keypath[2];
    return coin == expected_coin && account >= BIP44_ACCOUNT_MIN && account <= BIP44_ACCOUNT_MAX;
}

static bool _validate_keypath_address(
    const uint32_t* keypath,
    const size_t keypath_len,
    uint32_t expected_coin,
    uint32_t expected_purpose)
{
    if (keypath_len != 5) {
        return false;
    }
    uint32_t purpose = keypath[0];
    if (purpose != expected_purpose) {
        return false;
    }
    if (!_validate_keypath_account(keypath, expected_coin)) {
        return false;
    }
    uint32_t change = keypath[3];
    if (change > 1) {
        return false;
    }
    uint32_t address = keypath[4];
#if (BIP44_ADDRESS_MAX >= BIP32_INITIAL_HARDENED_CHILD)
#error "possibly hardened address"
#endif
    return address <= BIP44_ADDRESS_MAX;
}

bool btc_common_is_valid_keypath(
    BTCPubRequest_OutputType output_type,
    BTCScriptType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    switch (output_type) {
    case BTCPubRequest_OutputType_TPUB:
    case BTCPubRequest_OutputType_VPUB:
    case BTCPubRequest_OutputType_UPUB:
    case BTCPubRequest_OutputType_XPUB:
    case BTCPubRequest_OutputType_YPUB:
    case BTCPubRequest_OutputType_ZPUB:
        if (keypath_len != 3) {
            return false;
        }
        uint32_t purpose = keypath[0];
        switch (purpose) {
        case BTC_PURPOSE_P2PKH:
            return false; // disable legacy
        case BTC_PURPOSE_P2WPKH_P2SH:
        case BTC_PURPOSE_P2WPKH:
            break;
        default:
            return false;
        }
        return _validate_keypath_account(keypath, expected_coin);
    case BTCPubRequest_OutputType_ADDRESS:
        switch (script_type) {
        case BTCScriptType_SCRIPT_P2PKH:
            return false; // disable legacy
        case BTCScriptType_SCRIPT_P2WPKH_P2SH:
            return _validate_keypath_address(
                keypath, keypath_len, expected_coin, BTC_PURPOSE_P2WPKH_P2SH);
        case BTCScriptType_SCRIPT_P2WPKH:
            return _validate_keypath_address(
                keypath, keypath_len, expected_coin, BTC_PURPOSE_P2WPKH);
        default:
            return false;
        }
        break;
    default:
        return false;
    }
}

bool btc_common_encode_xpub(
    const struct ext_key* derived_xpub,
    const uint8_t* version, // must be 4 bytes
    char* out,
    size_t out_len)
{
    char* xpub_string = NULL;
    uint8_t bytes[BIP32_SERIALIZED_LEN] = {0};
    if (bip32_key_serialize(derived_xpub, BIP32_FLAG_KEY_PUBLIC, bytes, sizeof(bytes)) !=
        WALLY_OK) {
        return false;
    }
    // Overwrite bip32 version (libwally doesn't give the option to provide a
    // different one)
    memcpy(bytes, version, 4);
    int ret =
        wally_base58_from_bytes(bytes, BIP32_SERIALIZED_LEN, BASE58_FLAG_CHECKSUM, &xpub_string);
    util_zero(bytes, sizeof(bytes));
    if (ret != WALLY_OK) {
        return false;
    }
    int sprintf_result = snprintf(out, out_len, "%s", xpub_string);
    wally_free_string(xpub_string);
    return sprintf_result >= 0 && sprintf_result < (int)out_len;
}

/**
 * convert uint64_t to string. %llu / %lld not supported by our arm libc.
 * param[in] value value to format.
 * param[out] out 21 bytes of scratch space
 * pram[out_start] will be a pointer to inside out where the resulting string
 starts.
 */
static void _sprint_uint64(uint64_t value, char* out, char** out_start)
{
    char* p = out + 20;
    *p = '\0';
    for (bool first = true; value || first; first = false) {
        const uint32_t digit = value % 10;
        const char c = (char)('0' + digit);
        p--;
        *p = c;
        value = value / 10;
    }
    *out_start = p;
}

bool btc_common_format_amount(uint64_t satoshi, const char* unit, char* out, size_t out_len)
{
    if (unit == NULL || out == NULL || out_len < 31 + strlen(unit)) {
        return false;
    }
    int64_t satoshi_in_btc = 100000000;
    int64_t quotient = satoshi / satoshi_in_btc;
    int64_t remainder = satoshi % satoshi_in_btc;
    char quotient_str[21] = {0};
    char* quotient_str_start = NULL;
    _sprint_uint64(quotient, quotient_str, &quotient_str_start);
    char remainder_str[21] = {0};
    char* remainder_str_start = NULL;
    _sprint_uint64(remainder, remainder_str, &remainder_str_start);
    char amount_str[30] = {0};
    int sprintf_result = snprintf(
        amount_str,
        sizeof(amount_str),
        "%s.%.*s%s",
        quotient_str_start,
        // left pad 'remainder_str' up to 8 zeros
        (int)(8 - strlen(remainder_str_start)),
        "00000000",
        remainder_str_start);
    if (sprintf_result < 0 || sprintf_result >= (int)sizeof(amount_str)) {
        return false;
    }
    // trim right zeroes
    char* end = amount_str + strlen(amount_str) - 1;
    while (end > amount_str && *end == '0') {
        end--;
    }
    // trim potential right '.'
    if (*end == '.') {
        end--;
    }
    end[1] = '\0';
    sprintf_result = snprintf(out, out_len, "%s %s", amount_str, unit);
    if (sprintf_result < 0 || sprintf_result >= (int)out_len) {
        return false;
    }
    return true;
}

bool btc_common_outputhash_from_pubkeyhash(
    BTCScriptType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    switch (script_type) {
    case BTCScriptType_SCRIPT_P2PKH:
    case BTCScriptType_SCRIPT_P2WPKH:
        memcpy(output_hash, pubkey_hash, HASH160_LEN);
        *output_hash_size = HASH160_LEN;
        break;
    case BTCScriptType_SCRIPT_P2WPKH_P2SH: {
        uint8_t script[WALLY_SCRIPTPUBKEY_P2WPKH_LEN] = {0};
        size_t written = 0;
        if (wally_witness_program_from_bytes(
                pubkey_hash, HASH160_LEN, 0, script, sizeof(script), &written) != WALLY_OK) {
            return false;
        }
        if (written != WALLY_SCRIPTPUBKEY_P2WPKH_LEN) {
            return false;
        }
        if (wally_hash160(script, sizeof(script), output_hash, HASH160_LEN) != WALLY_OK) {
            return false;
        }
        *output_hash_size = HASH160_LEN;
        break;
    }
    default:
        return false;
    }
    return true;
}

bool btc_common_sighash_script_from_pubkeyhash(
    BTCScriptType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* script,
    size_t* script_size)
{
    size_t size_in = *script_size;
    switch (script_type) {
    case BTCScriptType_SCRIPT_P2WPKH_P2SH:
    case BTCScriptType_SCRIPT_P2WPKH:
        script[0] = 0x19; // 25 byte data push
        if (wally_scriptpubkey_p2pkh_from_bytes(
                pubkey_hash, HASH160_LEN, 0, script + 1, size_in - 1, script_size) != WALLY_OK) {
            return false;
        }
        *script_size = *script_size + 1;
        return true;
    default:
        return false;
    }
}

BTCOutputType btc_common_determine_output_type(BTCScriptType script_type)
{
    switch (script_type) {
    case BTCScriptType_SCRIPT_P2PKH:
        return BTCOutputType_P2PKH;
    case BTCScriptType_SCRIPT_P2WPKH_P2SH:
        return BTCOutputType_P2SH;
    case BTCScriptType_SCRIPT_P2WPKH:
        return BTCOutputType_P2WPKH;
    default:
        return BTCOutputType_UNKNOWN;
    }
}

/**
 * @param[in] version base58 check version, e.g. 0x05 for the "3" prefix.
 * @param[in] hash hash160 hash of pubkey or script, to bebase58Check encoded.
 * @param[out] out will contain the resulting address.
 * @param[in] out_len size allocation of `out`.
 * @return true on success, false on failure.
 */
static bool _encode_base58_address(uint8_t version, const uint8_t* hash, char* out, size_t out_len)
{
    uint8_t vhash[1 + HASH160_LEN] = {0};
    vhash[0] = version;
    memcpy(vhash + 1, hash, HASH160_LEN);
    char* address_string = NULL;
    if (wally_base58_from_bytes(vhash, sizeof(vhash), BASE58_FLAG_CHECKSUM, &address_string) !=
        WALLY_OK) {
        return false;
    }
    int sprintf_result = snprintf(out, out_len, "%s", address_string);
    wally_free_string(address_string);
    return sprintf_result >= 0 && sprintf_result < (int)out_len;
}

bool btc_common_address_from_outputhash(
    const app_btc_coin_params_t* params,
    BTCOutputType output_type,
    const uint8_t* hash,
    size_t hash_size,
    char* out,
    size_t out_len)
{
    switch (output_type) {
    case BTCOutputType_P2PKH:
        if (hash_size != HASH160_LEN) {
            return false;
        }
        return _encode_base58_address(params->base58_version_p2pkh, hash, out, out_len);
    case BTCOutputType_P2SH:
        if (hash_size != HASH160_LEN) {
            return false;
        }
        return _encode_base58_address(params->base58_version_p2sh, hash, out, out_len);
    case BTCOutputType_P2WPKH:
    case BTCOutputType_P2WSH: {
        uint8_t script[WALLY_SCRIPTPUBKEY_P2WSH_LEN] = {0};
        size_t written = 0;
        if (wally_witness_program_from_bytes(
                hash, hash_size, 0, script, sizeof(script), &written) != WALLY_OK) {
            return false;
        }
        char* address_string = NULL;
        if (wally_addr_segwit_from_bytes(script, written, params->bech32_hrp, 0, &address_string) !=
            WALLY_OK) {
            return false;
        }
        int sprintf_result = snprintf(out, out_len, "%s", address_string);
        wally_free_string(address_string);
        return sprintf_result >= 0 && sprintf_result < (int)out_len;
    }
    default:
        return false;
    }
    return true;
}

bool btc_common_pkscript_from_outputhash(
    BTCOutputType output_type,
    const uint8_t* hash,
    size_t hash_size,
    uint8_t* pk_script,
    size_t* pk_script_len)
{
    if (!hash || !pk_script || !pk_script_len) {
        return false;
    }
    size_t len = *pk_script_len;
    switch (output_type) {
    case BTCOutputType_P2PKH:
        if (hash_size != HASH160_LEN) {
            return false;
        }
        return wally_scriptpubkey_p2pkh_from_bytes(
                   hash, hash_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    case BTCOutputType_P2SH:
        if (hash_size != HASH160_LEN) {
            return false;
        }
        return wally_scriptpubkey_p2sh_from_bytes(
                   hash, hash_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    case BTCOutputType_P2WPKH:
    case BTCOutputType_P2WSH:
        return wally_witness_program_from_bytes(
                   hash, hash_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    default:
        return false;
    }
    return true;
}
