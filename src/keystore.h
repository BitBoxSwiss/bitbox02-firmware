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

#ifndef _KEYSTORE_H_
#define _KEYSTORE_H_

#include "compiler_util.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <secp256k1.h>

#define KEYSTORE_MAX_SEED_LENGTH (32)
#define KEYSTORE_U2F_SEED_LENGTH SHA256_LEN

// Max. length of an xpub string, including the null terminator.
#define XPUB_ENCODED_LEN 113

typedef enum {
    KEYSTORE_OK,
    KEYSTORE_ERR_INCORRECT_PASSWORD,
    KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED,
    KEYSTORE_ERR_UNSEEDED,
    KEYSTORE_ERR_MEMORY,
    KEYSTORE_ERR_SECURECHIP,
    KEYSTORE_ERR_SEED_SIZE,
    KEYSTORE_ERR_SALT,
    KEYSTORE_ERR_HASH,
    KEYSTORE_ERR_ENCRYPT,
    KEYSTORE_ERR_DECRYPT,
    KEYSTORE_ERR_STRETCH_RETAINED_SEED_KEY,
} keystore_error_t;

/**
 * Restores a seed. This also unlocks the keystore with this seed.
 * @param[in] seed The seed that is to be restored.
 * @param[in] seed_length The length of the seed (max. 32 bytes).
 * @param[in] password The password with which we encrypt the seed.
 */
/** Unlocks the keystore seed or checks the password:
 * If the keystore is locked, it decrypts and loads the seed, unlocking the keystore:
 * 1) loads the stored seed and tries to decrypt using password.
 * 2) if successful, the bip39 seed should be derived using keystore_unlock_bip39().
 * If the keystore is already unlocked, this function does *not* change the state (can be used to
 * check the password).
 * @param[in] password keystore password, used to decrypt the seed.
 * If it is false, the keystore is not unlocked.
 * @param[out] On any error, remaining_attempts_out will have the number of remaining attempts.
 * If zero, the keystore is locked until the device is reset.
 * @param[out] securechip_result_out, if not NULL, will contain the error code from
 * @param[out] seed_out The seed bytes copied from the retained seed.
 * The buffer should be KEYSTORE_MAX_SEED_LENGTH bytes long. The caller must
 * zero the seed once it is no longer needed.
 * @param[out] seed_len_out The seed length.
 * `securechip_kdf()` if there was a secure chip error, and 0 otherwise.
 * @return
 * - KEYSTORE_OK if they keystore was successfully unlocked
 * - KEYSTORE_ERR_* if unsuccessful.
 * Only call this if memory_is_seeded() returns true.
 */
USE_RESULT keystore_error_t keystore_unlock(
    const char* password,
    uint8_t* remaining_attempts_out,
    int* securechip_result_out,
    uint8_t* seed_out,
    size_t* seed_len_out);

/**
 * Retrieves the BIP39 word by index. `word_out` should be of at least 9 bytes long.
 */
USE_RESULT bool keystore_get_bip39_word_stack(uint16_t idx, char* word_out, size_t word_out_size);

/**
 * Get a commitment to the original nonce before tweaking it with the host nonce. This is part of
 * the ECDSA Anti-Klepto Protocol. For more details, check the docs of
 * `secp256k1_ecdsa_anti_exfil_signer_commit`.
 * @param[in] ctx secp256k1 context
 * @param[in] private_key 32 byte private key
 * @param[in] msg32 32 byte message which will be signed by `keystore_secp256k1_sign`.
 * @param[in] host_commitment must be `sha256(sha256(tag)||shas256(tag)||host_nonce)` where
 * host_nonce is passed to `keystore_secp256k1_sign()`. See
 * `secp256k1_ecdsa_anti_exfil_host_commit()`.
 * @param[out] client_commitment_out EC_PUBLIC_KEY_LEN bytes compressed signer nonce pubkey.
 */
USE_RESULT bool keystore_secp256k1_nonce_commit(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* client_commitment_out);

// clang-format off
/**
 * Sign message with private key using the given private key.
 *
 * Details about `host_nonce32`, the host nonce contribution.
 * Instead of using plain rfc6979 to generate the nonce in this signature, the following formula is used:
 *     r = rfc6979(..., additional_data=Hash_d(host_nonce32))
 *     R=r*G (pubkey to secret r)
 *     nonce = r + Hash_p(R, host_nonce32)
 * `Hash_d(msg)` and `Hash_p(msg)` are tagged hashes: `sha256(sha256(tag)||shas256(tag)||msg)`.
 * Tag for `Hash_d`: "s2c/ecdsa/data".
 * Tag for `Hash_p`: "s2c/ecdsa/point".
 * This is part of the ECSDA Anti-Klepto protocol, preventing this function to leak any secrets via
 * the signatures (see the ecdsa-s2c module in secp256k1-zpk for more details).
 *
 * @param[in] ctx secp256k1 context
 * @param[in] private_key 32 byte private key
 * @param[in] msg32 32 byte message to sign
 * @param[in] host_nonce32 32 byte nonce contribution. Cannot be NULL.
 * Intended to be a contribution by the host. If there is none available, use 32 zero bytes.
 * @param[out] sig_compact_out resulting signature in compact format. Must be 64 bytes.
 * @param[out] recid recoverable id. Can be NULL if not needed.
 * Parse with secp256k1_ecdsa_signature_serialize_compact().
 * @return true on success, false if the keystore is locked.
 */
// clang-format on
USE_RESULT bool keystore_secp256k1_sign(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out);

#endif
