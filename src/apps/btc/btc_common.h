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
#include <keystore.h>

#include <hww.pb.h>

#include <wally_bip32.h>
#include <wally_crypto.h>
#include <wally_script.h>

#define MULTISIG_P2WSH_MAX_SIGNERS 15

typedef struct {
    size_t xpubs_count;
    struct ext_key xpubs[MULTISIG_P2WSH_MAX_SIGNERS];
    uint32_t threshold;
} multisig_t;

// see https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
#define MAX_VARINT_SIZE (9)
// current expected max pk script size is a m-of-15 multisig. 700 is also enough for m-of-20, which
// is technically possible to extend to if needed.
#define MAX_PK_SCRIPT_SIZE (700)

USE_RESULT bool btc_common_convert_multisig(
    const BTCScriptConfig_Multisig* multisig,
    multisig_t* multisig_out);

/**
 * Does limit checks the keypath, whitelisting bip44 purposes and accounts.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath_account_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin,
    bool taproot_support);

/**
 * Checks that the keypath is m/48'/coin'/account'/script_type'/change/address, limiting the number
 * of valid accounts/addresses.
 * script_type' is 2' for P2WSH and 1' for P2WSH-P2SH.
 * @return true if the keypath is valid, false if it is invalid.
 */
USE_RESULT bool btc_common_is_valid_keypath_address_multisig(
    BTCScriptConfig_Multisig_ScriptType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);

/**
 * Generate the payload used in an output script, e.g. pubkeyhash or script hash or pubkey.
 * @param[in] keypath address-level keypath, e.g. m/84'/0'/0'/0/0
 * @param[in] keypath_len number of elements in keypath
 * @param[in] script_type script type defining the payload.
 * @param[out] output_payload will have the resulting payload. Must be of size 32.
 * @param[out] output_payload_size will be 32 for p2wsh scripts, HASH160_LEN for
 * all others.
 * return true on succes, false on failure.
 */
USE_RESULT bool btc_common_payload_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    BTCScriptConfig_SimpleType script_type,
    uint8_t* output_payload,
    size_t* output_payload_size);

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
 * For a multisig input type, determine the output type.
 */
USE_RESULT BTCOutputType
btc_common_determine_output_type_multisig(const BTCScriptConfig_Multisig* multisig);

/**
 * Computes the pkScript from a pubkey hash or script hash or pubkey, depending on the output
 * type.
 * @param[in] output_type type of pkScript.
 * @param[in] payload pubkey hash or script hash or pubkey.
 * @param[inout] pk_script_len: size of pk_script IN, size of the resulting pk_script OUT.
 */
USE_RESULT bool btc_common_pkscript_from_payload(
    const app_btc_coin_params_t* params,
    BTCOutputType output_type,
    const uint8_t* payload,
    size_t payload_size,
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
    const multisig_t* multisig,
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
 * @param[out] output_payload result, must be at least `SHA256_LEN` bytes.
 * @param[out] output_payload_size of the output hash. Will be `SHA256_LEN` for P2WSH and
 * `HASH160_LEN` for P2WSH-P2SH.
 */
USE_RESULT bool btc_common_payload_from_multisig(
    const multisig_t* multisig,
    BTCScriptConfig_Multisig_ScriptType script_type,
    uint32_t keypath_change,
    uint32_t keypath_address,
    uint8_t* output_payload,
    size_t* output_payload_size);

/**
 * Validate a m-of-n multisig account. This includes checking that:
 * - 0 < m <= n <= 15
 * - the keypath conforms to bip48 for p2wsh: m/48'/coin'/account'/script_type'
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
 *
 * The xpubs in the multisig config are not sorted before hashing. This was the default for firmware
 * <= v9.2.1
 *
 * @param[in] coin The coin this multisig is used with.
 * @param[in] multisig The multisig config details.
 * @param[in] keypath Account-level keypath.
 * @param[in] keypath_len number of elements in keypath.
 * @param[out] hash_out resulting hash; must be `SHA256_LEN` bytes.
 * @return true on success, false on failure.
 */
USE_RESULT bool btc_common_multisig_hash_unsorted(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out);

/**
 * Same as `btc_common_multisig_hash_unsorted()`, but the xpubs are sorted before hashing.
 */
USE_RESULT bool btc_common_multisig_hash_sorted(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash_out);

/**
 * Get the name of a registered multisig account. If `name` is NULL, this serves as a check whether
 * the account was registered.
 *
 * The individual params are not validated, they must be pre-validated!
 *
 * @param[in] coin The coin this multisig is used with.
 * @param[in] multisig The multisig config details.
 * @param[in] keypath Account-level keypath.
 * @param[in] keypath_len number of elements in keypath.
 * @param[out] name_out will contain the name. Must have at least `MEMORY_MULTISIG_NAME_MAX_LEN`
 * bytes. Can be NULL.
 * @return true on success, false on failure.
 */
USE_RESULT bool btc_common_multisig_name(
    BTCCoin coin,
    const BTCScriptConfig_Multisig* multisig,
    const uint32_t* keypath,
    size_t keypath_len,
    char* name_out);

#endif
