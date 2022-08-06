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

bool btc_common_convert_multisig(const BTCScriptConfig_Multisig* multisig, multisig_t* multisig_out)
{
    multisig_out->xpubs_count = multisig->xpubs_count;
    multisig_out->threshold = multisig->threshold;
    for (size_t i = 0; i < multisig_out->xpubs_count; i++) {
        struct ext_key xpub = {0};
        if (!apps_common_bip32_xpub_from_protobuf(&multisig->xpubs[i], &xpub)) {
            return false;
        }
        if (bip32_key_serialize(
                &xpub,
                BIP32_FLAG_KEY_PUBLIC,
                multisig_out->xpubs[i],
                sizeof(multisig_out->xpubs[i])) != WALLY_OK) {
            return false;
        }
    }
    return true;
}

bool btc_common_is_valid_keypath_account_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin,
    bool taproot_support)
{
    return rust_bitcoin_keypath_validate_account_simple(
        keypath, keypath_len, expected_coin, script_type, taproot_support);
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

bool btc_common_payload_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    BTCScriptConfig_SimpleType script_type,
    uint8_t* output_payload,
    size_t* output_payload_size)
{
    switch (script_type) {
    case BTCScriptConfig_SimpleType_P2WPKH:
        if (!keystore_secp256k1_pubkey_hash160(keypath, keypath_len, output_payload)) {
            return false;
        }
        *output_payload_size = HASH160_LEN;
        break;
    case BTCScriptConfig_SimpleType_P2WPKH_P2SH: {
        uint8_t pubkey_hash[HASH160_LEN];
        UTIL_CLEANUP_20(pubkey_hash);
        if (!keystore_secp256k1_pubkey_hash160(keypath, keypath_len, pubkey_hash)) {
            return false;
        }
        uint8_t script[WALLY_SCRIPTPUBKEY_P2WPKH_LEN] = {0};
        size_t written = 0;
        if (wally_witness_program_from_bytes(
                pubkey_hash, HASH160_LEN, 0, script, sizeof(script), &written) != WALLY_OK) {
            return false;
        }
        if (written != WALLY_SCRIPTPUBKEY_P2WPKH_LEN) {
            return false;
        }
        if (wally_hash160(script, sizeof(script), output_payload, HASH160_LEN) != WALLY_OK) {
            return false;
        }
        *output_payload_size = HASH160_LEN;
        break;
    }
    case BTCScriptConfig_SimpleType_P2TR:
        if (!keystore_secp256k1_schnorr_bip86_pubkey(keypath, keypath_len, output_payload)) {
            return false;
        }
        *output_payload_size = 32;
        break;
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

bool btc_common_pkscript_from_payload(
    const app_btc_coin_params_t* params,
    BTCOutputType output_type,
    const uint8_t* payload,
    size_t payload_size,
    uint8_t* pk_script,
    size_t* pk_script_len)
{
    if (!params || !payload || !pk_script || !pk_script_len) {
        return false;
    }
    size_t len = *pk_script_len;
    switch (output_type) {
    case BTCOutputType_P2PKH:
        if (payload_size != HASH160_LEN) {
            return false;
        }
        return wally_scriptpubkey_p2pkh_from_bytes(
                   payload, payload_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    case BTCOutputType_P2SH:
        if (payload_size != HASH160_LEN) {
            return false;
        }
        return wally_scriptpubkey_p2sh_from_bytes(
                   payload, payload_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    case BTCOutputType_P2WPKH:
    case BTCOutputType_P2WSH:
        return wally_witness_program_from_bytes(
                   payload, payload_size, 0, pk_script, len, pk_script_len) == WALLY_OK;
    case BTCOutputType_P2TR:
        if (!params->taproot_support || payload_size != 32) {
            return false;
        }
        return wally_witness_program_from_bytes_and_version(
                   payload, payload_size, 1, 0, pk_script, len, pk_script_len) == WALLY_OK;
    default:
        return false;
    }
    return true;
}

bool btc_common_pkscript_from_multisig(
    const multisig_t* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* script_out,
    size_t* script_out_size)
{
    uint8_t pubkeys[MULTISIG_P2WSH_MAX_SIGNERS * EC_PUBLIC_KEY_LEN];

    for (size_t index = 0; index < multisig->xpubs_count; index++) {
        struct ext_key xpub = {0};
        if (bip32_key_unserialize(multisig->xpubs[index], sizeof(multisig->xpubs[index]), &xpub) !=
            WALLY_OK) {
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

bool btc_common_payload_from_multisig(
    const multisig_t* multisig,
    BTCScriptConfig_Multisig_ScriptType script_type,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* output_payload,
    size_t* output_payload_size)
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

    switch (script_type) {
    case BTCScriptConfig_Multisig_ScriptType_P2WSH:
        *output_payload_size = SHA256_LEN;
        return wally_sha256(script, written, output_payload, SHA256_LEN) == WALLY_OK;
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
        *output_payload_size = HASH160_LEN;
        return wally_hash160(p2wsh_pkscript, written, output_payload, HASH160_LEN) == WALLY_OK;
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
