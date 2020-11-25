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
#include <stdlib.h>

#include "btc_common.h"

#include <apps/common/bip32.h>
#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <rust/rust.h>
#include <util.h>
#include <wally_address.h>

#define MULTISIG_P2WSH_MAX_SIGNERS 15

static const uint8_t _xpub_version[4] = {0x04, 0x88, 0xb2, 0x1e};
static const uint8_t _ypub_version[4] = {0x04, 0x9d, 0x7c, 0xb2};
static const uint8_t _zpub_version[4] = {0x04, 0xb2, 0x47, 0x46};
static const uint8_t _tpub_version[4] = {0x04, 0x35, 0x87, 0xcf};
static const uint8_t _vpub_version[4] = {0x04, 0x5f, 0x1c, 0xf6};
static const uint8_t _upub_version[4] = {0x04, 0x4a, 0x52, 0x62};
static const uint8_t _capital_vpub_version[4] = {0x02, 0x57, 0x54, 0x83};
static const uint8_t _capital_zpub_version[4] = {0x02, 0xaa, 0x7e, 0xd3};
static const uint8_t _capital_upub_version[4] = {0x02, 0x42, 0x89, 0xef};
static const uint8_t _capital_ypub_version[4] = {0x02, 0x95, 0xb4, 0x3f};

const char* btc_common_coin_name(BTCCoin coin)
{
    static const char* _coin_btc = "Bitcoin";
    static const char* _coin_tbtc = "BTC Testnet";
    static const char* _coin_ltc = "Litecoin";
    static const char* _coin_tltc = "LTC Testnet";

    switch (coin) {
    case BTCCoin_BTC:
        return _coin_btc;
    case BTCCoin_TBTC:
        return _coin_tbtc;
    case BTCCoin_LTC:
        return _coin_ltc;
    case BTCCoin_TLTC:
        return _coin_tltc;
    default:
        Abort("btc_common_coin_name");
    }
}

bool btc_common_is_valid_keypath_xpub(
    BTCPubRequest_XPubType xpub_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    switch (xpub_type) {
    case BTCPubRequest_XPubType_TPUB:
    case BTCPubRequest_XPubType_XPUB:
    case BTCPubRequest_XPubType_YPUB:
    case BTCPubRequest_XPubType_ZPUB:
    case BTCPubRequest_XPubType_VPUB:
    case BTCPubRequest_XPubType_UPUB:
    case BTCPubRequest_XPubType_CAPITAL_VPUB:
    case BTCPubRequest_XPubType_CAPITAL_ZPUB:
    case BTCPubRequest_XPubType_CAPITAL_UPUB:
    case BTCPubRequest_XPubType_CAPITAL_YPUB:
        return rust_bitcoin_keypath_validate_xpub(keypath, keypath_len, expected_coin);
    default:
        return false;
    }
}

bool btc_common_is_valid_keypath_account_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    return rust_bitcoin_keypath_validate_account_simple(
        keypath, keypath_len, expected_coin, script_type);
}

bool btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    return rust_bitcoin_keypath_validate_address_simple(
        keypath, keypath_len, expected_coin, script_type);
}

bool btc_common_is_valid_keypath_address_multisig(
    BTCScriptConfig_Multisig_ScriptType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    return rust_bitcoin_keypath_validate_address_multisig(
        keypath, keypath_len, expected_coin, script_type);
}

bool btc_common_encode_xpub(
    const struct ext_key* derived_xpub,
    BTCPubRequest_XPubType xpub_type,
    char* out,
    size_t out_len)
{
    char* xpub_string = NULL;
    uint8_t bytes[BIP32_SERIALIZED_LEN] = {0};
    if (bip32_key_serialize(derived_xpub, BIP32_FLAG_KEY_PUBLIC, bytes, sizeof(bytes)) !=
        WALLY_OK) {
        return false;
    }
    const uint8_t* version;
    switch (xpub_type) {
    case BTCPubRequest_XPubType_TPUB:
        version = _tpub_version;
        break;
    case BTCPubRequest_XPubType_VPUB:
        version = _vpub_version;
        break;
    case BTCPubRequest_XPubType_UPUB:
        version = _upub_version;
        break;
    case BTCPubRequest_XPubType_XPUB:
        version = _xpub_version;
        break;
    case BTCPubRequest_XPubType_YPUB:
        version = _ypub_version;
        break;
    case BTCPubRequest_XPubType_ZPUB:
        version = _zpub_version;
        break;
    case BTCPubRequest_XPubType_CAPITAL_VPUB:
        version = _capital_vpub_version;
        break;
    case BTCPubRequest_XPubType_CAPITAL_ZPUB:
        version = _capital_zpub_version;
        break;
    case BTCPubRequest_XPubType_CAPITAL_UPUB:
        version = _capital_upub_version;
        break;
    case BTCPubRequest_XPubType_CAPITAL_YPUB:
        version = _capital_ypub_version;
        break;
    default:
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
    BTCScriptConfig_SimpleType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    switch (script_type) {
    case BTCScriptConfig_SimpleType_P2WPKH:
        memcpy(output_hash, pubkey_hash, HASH160_LEN);
        *output_hash_size = HASH160_LEN;
        break;
    case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
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
    BTCScriptConfig_SimpleType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* script,
    size_t* script_size)
{
    size_t size_in = *script_size;
    switch (script_type) {
    case BTCScriptConfig_SimpleType_P2WPKH_P2SH:
    case BTCScriptConfig_SimpleType_P2WPKH:
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

BTCOutputType btc_common_determine_output_type(BTCScriptConfig_SimpleType script_type)
{
    switch (script_type) {
    case BTCScriptConfig_SimpleType_P2WPKH_P2SH:
        return BTCOutputType_P2SH;
    case BTCScriptConfig_SimpleType_P2WPKH:
        return BTCOutputType_P2WPKH;
    default:
        return BTCOutputType_UNKNOWN;
    }
}

BTCOutputType btc_common_determine_output_type_multisig(const BTCScriptConfig_Multisig* multisig)
{
    switch (multisig->script_type) {
    case BTCScriptConfig_Multisig_ScriptType_P2WSH:
        return BTCOutputType_P2WSH;
    case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH:
        return BTCOutputType_P2SH;
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

bool btc_common_pkscript_from_multisig(
    const BTCScriptConfig_Multisig* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* script_out,
    size_t* script_out_size)
{
    uint8_t pubkeys[MULTISIG_P2WSH_MAX_SIGNERS * EC_PUBLIC_KEY_LEN];

    for (size_t index = 0; index < multisig->xpubs_count; index++) {
        const XPub* xpub_in = &multisig->xpubs[index];
        struct ext_key xpub = {0};
        if (!apps_common_bip32_xpub_from_protobuf(xpub_in, &xpub)) {
            return false;
        }
        struct ext_key derived_cosigner_xpub = {0};
        const uint32_t keypath[2] = {keypath_change, keypath_address};
        if (bip32_key_from_parent_path(
                &xpub, keypath, 2, BIP32_FLAG_KEY_PUBLIC, &derived_cosigner_xpub) != WALLY_OK) {
            return false;
        }
        memcpy(
            &pubkeys[index * EC_PUBLIC_KEY_LEN], derived_cosigner_xpub.pub_key, EC_PUBLIC_KEY_LEN);
    }

    size_t written;
    if (wally_scriptpubkey_multisig_from_bytes(
            pubkeys,
            multisig->xpubs_count * EC_PUBLIC_KEY_LEN,
            multisig->threshold,
            WALLY_SCRIPT_MULTISIG_SORTED,
            script_out,
            *script_out_size,
            &written) != WALLY_OK) {
        return false;
    }
    if (written > *script_out_size) {
        // Double check since the function above sets written to script_len if the buffer was too
        // short.
        return false;
    }
    *script_out_size = written;

    return true;
}

bool btc_common_outputhash_from_multisig(
    const BTCScriptConfig_Multisig* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* output_hash,
    size_t* output_hash_size)
{
    uint8_t script[700] = {0};
    size_t written = sizeof(script);
    if (!btc_common_pkscript_from_multisig(
            multisig, keypath_change, keypath_address, script, &written)) {
        return false;
    }

    // TODO: double check that the witness script must be <= 10,000 bytes /
    // 201 opCounts (consensus rule), resp. 3,600 bytes (standardness rule).
    // See https://bitcoincore.org/en/segwit_wallet_dev/.
    // Note that the witness script has an additional varint prefix.

    switch (multisig->script_type) {
    case BTCScriptConfig_Multisig_ScriptType_P2WSH:
        *output_hash_size = SHA256_LEN;
        return wally_sha256(script, written, output_hash, SHA256_LEN) == WALLY_OK;
    case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH: {
        // script_sha256 contains the hash of the multisig redeem script as used in a P2WSH output.
        uint8_t script_sha256[SHA256_LEN] = {0};
        if (wally_sha256(script, written, script_sha256, sizeof(script_sha256)) != WALLY_OK) {
            return false;
        }
        // create the p2wsh output.
        uint8_t p2wsh_pkscript[WALLY_SCRIPTPUBKEY_P2WSH_LEN] = {0};
        if (wally_witness_program_from_bytes(
                script_sha256,
                sizeof(script_sha256),
                0,
                p2wsh_pkscript,
                sizeof(p2wsh_pkscript),
                &written) != WALLY_OK) {
            return false;
        }
        // hash the output script according to p2sh.
        *output_hash_size = HASH160_LEN;
        return wally_hash160(p2wsh_pkscript, written, output_hash, HASH160_LEN) == WALLY_OK;
    }
    default:
        return false;
    };
}

USE_RESULT bool btc_common_multisig_is_valid(
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin)
{
    if (multisig->xpubs_count < 2 || multisig->xpubs_count > MULTISIG_P2WSH_MAX_SIGNERS) {
        return false;
    }
    if (multisig->threshold == 0 || multisig->threshold > multisig->xpubs_count) {
        return false;
    }
    if (multisig->our_xpub_index >= multisig->xpubs_count) {
        return false;
    }
    if (!rust_bitcoin_keypath_validate_account_multisig(
            keypath, keypath_len, expected_coin, multisig->script_type)) {
        return false;
    }

    struct ext_key our_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!keystore_get_xpub(keypath, keypath_len, &our_xpub)) {
        return false;
    }

    const XPub* maybe_our_xpub_in = &multisig->xpubs[multisig->our_xpub_index];
    struct ext_key maybe_our_xpub = {0};
    if (!apps_common_bip32_xpub_from_protobuf(maybe_our_xpub_in, &maybe_our_xpub)) {
        return false;
    }
    if (!apps_common_bip32_xpubs_equal(&our_xpub, &maybe_our_xpub)) {
        return false;
    }

    // Check for duplicates.
    for (size_t i = 0; i < multisig->xpubs_count; i++) {
        struct ext_key xpub_i = {0};
        if (!apps_common_bip32_xpub_from_protobuf(&multisig->xpubs[i], &xpub_i)) {
            return false;
        }
        for (size_t j = i + 1; j < multisig->xpubs_count; j++) {
            struct ext_key xpub_j = {0};
            if (!apps_common_bip32_xpub_from_protobuf(&multisig->xpubs[j], &xpub_j)) {
                return false;
            }
            if (apps_common_bip32_xpubs_equal(&xpub_i, &xpub_j)) {
                return false;
            }
        }
    }
    return true;
}

// serialized_out must be of size BIP32_SERIALIZED_LEN.
static bool _serialize_xpub(const XPub* xpub, uint8_t* serialized_out)
{
    struct ext_key wally_xpub __attribute__((__cleanup__(keystore_zero_xkey))) = {0};
    if (!apps_common_bip32_xpub_from_protobuf(xpub, &wally_xpub)) {
        return false;
    }
    return bip32_key_serialize(
               &wally_xpub, BIP32_FLAG_KEY_PUBLIC, serialized_out, BIP32_SERIALIZED_LEN) ==
           WALLY_OK;
}

static int _xpubs_sort_comp(const void* elem1, const void* elem2)
{
    const uint8_t* xpub1 = (const uint8_t*)elem1;
    const uint8_t* xpub2 = (const uint8_t*)elem2;

    // Sort by xpub serialization, ignoring the version bytes.
    return memcmp(xpub1 + 4, xpub2 + 4, BIP32_SERIALIZED_LEN - 4);
}

static bool _multisig_hash(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    bool sort_xpubs,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out)
{
    void* ctx __attribute__((__cleanup__(rust_sha256_free))) = rust_sha256_new();

    { // 1. coin
        uint8_t byte;
        switch (coin) {
        case BTCCoin_BTC:
            byte = 0x00;
            break;
        case BTCCoin_TBTC:
            byte = 0x01;
            break;
        case BTCCoin_LTC:
            byte = 0x02;
            break;
        case BTCCoin_TLTC:
            byte = 0x03;
            break;
        default:
            return false;
        }
        rust_sha256_update(ctx, &byte, 1);
    }
    { // 2. script config type
        uint8_t byte;
        switch (multisig->script_type) {
        case BTCScriptConfig_Multisig_ScriptType_P2WSH:
            byte = 0x00;
            break;
        case BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH:
            byte = 0x01;
            break;
        default:
            return false;
        }
        rust_sha256_update(ctx, &byte, 1);
    }
    { // 3. threshold
        // assumes little endian environment
        rust_sha256_update(ctx, &multisig->threshold, sizeof(multisig->threshold));
    }
    { // 4. num xpubs
        uint32_t num = multisig->xpubs_count; // cast to fixed size
        // assumes little endian environment
        rust_sha256_update(ctx, &num, sizeof(num));
    }
    { // 5. xpubs
        uint8_t xpubs_serialized[sizeof(multisig->xpubs) / sizeof(*multisig->xpubs)]
                                [BIP32_SERIALIZED_LEN] = {0};
        for (size_t i = 0; i < multisig->xpubs_count; i++) {
            if (!_serialize_xpub(&multisig->xpubs[i], xpubs_serialized[i])) {
                return false;
            }
        }
        if (sort_xpubs) {
            qsort(
                xpubs_serialized,
                multisig->xpubs_count,
                sizeof(*xpubs_serialized),
                _xpubs_sort_comp);
        }
        for (size_t i = 0; i < multisig->xpubs_count; i++) {
            // Drop the first xpub version, which are the 4 first bytes. They are determined by the
            // above `BIP32_FLAG_KEY_PUBLIC` flag and do not add anything, as the xpub version is
            // chosen ad-hoc depending on the context it is used in.
            rust_sha256_update(ctx, xpubs_serialized[i] + 4, BIP32_SERIALIZED_LEN - 4);
        }
    }
    { // 6. keypath len
        uint32_t len = keypath_len; // cast to fixed size
        rust_sha256_update(ctx, &len, sizeof(len));
    }
    { // 7. keypath
        for (size_t i = 0; i < keypath_len; i++) {
            rust_sha256_update(ctx, &keypath[i], sizeof(keypath[i]));
        }
    }
    rust_sha256_finish(&ctx, hash_out);
    return true;
}

bool btc_common_multisig_hash_unsorted(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out)
{
    return _multisig_hash(coin, multisig, false, keypath, keypath_len, hash_out);
}

bool btc_common_multisig_hash_sorted(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out)
{
    return _multisig_hash(coin, multisig, true, keypath, keypath_len, hash_out);
}

bool btc_common_multisig_name(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    char* name_out)
{
    uint8_t hash[SHA256_LEN] = {0};

    // First try using sorted xpubs (the default registration since v9.3.0).
    if (!btc_common_multisig_hash_sorted(coin, multisig, keypath, keypath_len, hash)) {
        return false;
    }
    if (memory_multisig_get_by_hash(hash, name_out)) {
        return true;
    }

    // If that did not exist, try with unsorted xpubs for backwards compatibility.
    if (!btc_common_multisig_hash_unsorted(coin, multisig, keypath, keypath_len, hash)) {
        return false;
    }
    return memory_multisig_get_by_hash(hash, name_out);
}
