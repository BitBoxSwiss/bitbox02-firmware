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
