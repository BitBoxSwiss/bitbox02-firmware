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

// see https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
#define MAX_VARINT_SIZE (9)
// current expected max pk script size is a m-of-15 multisig. 700 is also enough for m-of-20, which
// is technically possible to extend to if needed.
#define MAX_PK_SCRIPT_SIZE (700)

// Max. length of an xpub string, including the null terminator.
#define XPUB_ENCODED_LEN 113

/**
 * Returns the coin name to be used in confirm dialogs ("Bitcoin", "Litecoin", etc.). Aborts for an
 * invalid coin.
 */
USE_RESULT const char* btc_common_coin_name(BTCCoin coin);

/**
 * Does limit checks the keypath, whitelisting bip44 purposes, accounts and
 * (change) addressses.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath_xpub(
    BTCPubRequest_XPubType xpub_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Does limit checks the keypath, whitelisting bip44 purposes, accounts and
 * (change) addresses.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Checks that the keypath is m/48'/coin'/account'/2'/change/address, limiting the number of valid
 * accounts/addresses.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath_address_multisig_p2wsh(
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Encode an xpub as a base58 string.
 * @param[in] dervived_xpub the xpub to encode.
 * @param[in] xpub_type determines the xpub format, e.g. xpub, ypub, zpub, ...
 * @param[out] out resulting string, must be at least of size `XPUB_ENCODED_LEN` (including the null
 * terminator).
 * @param[in] out_len size of `out`.
 * @return false on failure, true on success.
 */
USE_RESULT bool btc_common_encode_xpub(
    const struct ext_key* derived_xpub,
    BTCPubRequest_XPubType xpub_type,
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
    BTCScriptConfig_SimpleType script_type,
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
    BTCScriptConfig_SimpleType script_type,
    const uint8_t* pubkey_hash,
    uint8_t* script,
    size_t* script_size);

/**
 * For an input type (e.g. a script wrapped in p2sh), determine the output type.
 */
USE_RESULT BTCOutputType btc_common_determine_output_type(BTCScriptConfig_SimpleType script_type);
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

/**
 * Creates a n-of-m multisig script based on OP_CHECKMULTISIG. 0<n<=m<=15.
 * Note that the multisig config and keypaths are *not* validated, this must be done before calling.
 * @param[in] multisig Multisig configuration (threshold, signers). The xpubs are account-level
 * xpubs.
 * @param[in] keypath_change 0 for receive addresses, 1 for change addresses.
 * @param[in] keypath_address receive address index.
 * @param[out] script_out script to be created. Must be at least 517 bytes.
 * @param[out] script_out_size The size of the generated script.
 * @return true on success, false on failure.
 */
USE_RESULT bool btc_common_pkscript_from_multisig(
    const BTCScriptConfig_Multisig* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* script_out,
    size_t* script_out_size);

/**
 * Constructs sha256(<multisig pkScript>) from the provided multisig.
 * Note that the multisig config and keypaths are *not* validated, this must be done before calling.
 * @param[in] multisig Multisig configuration (threshold, signers). The xpubs are account-level
 * xpubs.
 * @param[in] keypath_change 0 for receive addresses, 1 for change addresses.
 * @param[in] keypath_address receive address index.
 * @param[out] output_hash result, must be `SHA256_LEN` bytes.
 */
USE_RESULT bool btc_common_outputhash_from_multisig_p2wsh(
    const BTCScriptConfig_Multisig* multisig,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* output_hash);

/**
 * Validate a m-of-n multisig account. This includes checking that:
 * - 0 < m <= n <= 15
 * - the keypath conforms to bip48 for p2wsh: m/48'/coin'/account'/2'
 * - our designated xpub is actually ours (corresponds to the xpub of the currenty unlocked
 *   keystore).
 * - no two xpubs are the same.
 * @param[in] multisig Multisig configuration (threshold, signers). The xpubs are account-level
 * xpubs.
 * @param[in] keypath account-level keypath, e.g. m/48'/0'/10'/2'
 * @param[in] keypath_len number of elements in keypath
 * @param[in] expected_coin expected bip44 coin in the keypath.
 */
USE_RESULT bool btc_common_multisig_is_valid(
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Creates a hash of this multisig config, useful for multisig account registration and
 * identification. The individual params are not validated, they must be pre-validated!
 * @param[in] coin The coin this multisig is used with.
 * @param[in] multisig The multisig config details.
 * @param[in] keypath Account-level keypath.
 * @param[in] keypath_len number of elements in keypath.
 * @param[out] hash_out resulting hash; must be `SHA256_LEN` bytes.
 * @return true on success, false on failure.
 */
USE_RESULT bool btc_common_multisig_hash(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out);

#endif
