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

#ifndef _APPS_BTC_COMMON_H
#define _APPS_BTC_COMMON_H

#include "btc_params.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

#include <hww.pb.h>

#include <wally_bip32.h>
#include <wally_crypto.h>
#include <wally_script.h>

#define BTC_PURPOSE_P2PKH (44 + BIP32_INITIAL_HARDENED_CHILD)
#define BTC_PURPOSE_P2WPKH_P2SH (49 + BIP32_INITIAL_HARDENED_CHILD)
#define BTC_PURPOSE_P2WPKH (84 + BIP32_INITIAL_HARDENED_CHILD)

#define BIP44_ACCOUNT_MIN (BIP32_INITIAL_HARDENED_CHILD)
#define BIP44_ACCOUNT_MAX (BIP32_INITIAL_HARDENED_CHILD + 99) // 100 accounts
#define BIP44_ADDRESS_MAX (9999) // 10k addresses

#define MAX_SIGHASH_SCRIPT_SIZE (500)

/**
 * Does limit checks the keypath, whitelisting bip44 purposes, accounts and
 * (change) addressses.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath(
    BTCPubRequest_OutputType output_type,
    BTCScriptType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Encode an xpub as a base58 string.
 * @param[in] dervived_xpub the xpub to encode.
 * @param[in] version 4 bytes version determining the prefix (e.g. 0x0488b21e for "xpub...")
 * @param[out] out resulting string, must be at least of size 113 (including the null terminator).
 * @param[in] out_len size of `out`.
 * @return false on failure, true on success.
 */
USE_RESULT bool btc_common_encode_xpub(
    const struct ext_key* derived_xpub,
    const uint8_t* version, // must be 4 bytes
    char* out,
    size_t out_len);

/**
 * Converts a satoshi value to a string with the BTC unit, e.g. 1234567890 -> "12.34567890 BTC".
 * @param[in] satoshi Amount in Satoshi.
 * @param[in] unit suffix.
 * @param[out] out will contain the resulting string.
 * @param[in] out_len size allocation of `out`. Should be at least 31+len(unit) bytes.
 * return true on succes, false on failure.
 */
USE_RESULT bool btc_common_format_amount(
    uint64_t satoshi,
    const char* unit,
    char* out,
    size_t out_len);

/**
 * Converts a pubkeyhash to a hash used in an output script, e.g. pubkeyhash or script hash.
 * The pkScript to be hashed is created based on the script type (output type).
 * @param[in] script_type script type defining the pkScript.
 * @param[in] pubkey_hash hash160 of a public key. Must be of size HASH160_LEN.
 * @param[out] output_hash will have the resulting hash. Must be of size 32.
 * @param[out] output_hash_size will be 32 for p2wsh scripts, HASH160_LEN for
 * all others.
 * return true on succes, false on failure.
 */
USE_RESULT bool btc_common_outputhash_from_pubkeyhash(
    BTCScriptType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* output_hash,
    size_t* output_hash_size);

/**
 * Converts a pubkeyhash to the subscript/scriptCode used in the sighash algo.
 * @param[in] script_type script type of the output to be spent.
 * @param[in] pubkey_hash hash160 of a public key. Must be of size HASH160_LEN.
 * @param[out] script will have the resulting subscript/scriptCode. Must be of size
 * MAX_SIGHASH_SCRIPT_SIZE.
 * @param[out] script_size the size of the produced subscript/scriptCode.
 * return true on succes, false on failure.
 */
USE_RESULT bool btc_common_sighash_script_from_pubkeyhash(
    BTCScriptType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* script,
    size_t* script_size);

/**
 * For an input type (e.g. a script wrapped in p2sh), determine the output type.
 */
USE_RESULT BTCOutputType btc_common_determine_output_type(BTCScriptType script_type);
/**
 * Converts an output script or publickey hash to an address.
 * hash, hash_size can be obtained from btc_common_outputhash_from_pubkeyhash().
 */
USE_RESULT bool btc_common_address_from_outputhash(
    const app_btc_coin_params_t* params,
    BTCOutputType output_type,
    const uint8_t* hash,
    size_t hash_size,
    char* out,
    size_t out_len);

/**
 * Computes the pkScript from a pubkey or script hash depending on the output
 * type.
 * @param[in] output_type type of pkScript.
 * @param[in] hash pubkey hash or script hash
 * @param[inout] pk_script_len: size of pk_script IN, size of the resulting pk_script OUT.
 */
USE_RESULT bool btc_common_pkscript_from_outputhash(
    BTCOutputType output_type,
    const uint8_t* hash,
    size_t hash_size,
    uint8_t* pk_script,
    size_t* pk_script_len);

#endif
