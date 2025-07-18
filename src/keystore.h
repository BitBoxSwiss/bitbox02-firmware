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
#include <wally_bip32.h>
#include <wally_bip39.h> // for BIP39_WORDLIST_LEN
#include <wally_crypto.h> // for EC_PUBLIC_KEY_LEN

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
} keystore_error_t;

/**
 * Copies the retained seed into the given buffer. The caller must
 * zero the seed with util_zero once it is no longer needed.
 * @param[out] seed_out The seed bytes copied from the retained seed.
 * The buffer should be KEYSTORE_MAX_SEED_LENGTH bytes long.
 * @param[out] length_out The seed length.
 * @return true if the seed was still retained.
 */
USE_RESULT bool keystore_copy_seed(uint8_t* seed_out, size_t* length_out);

/**
 * Copies the retained bip39 seed into the given buffer. The caller must
 * zero the seed once it is no longer needed.
 * @param[out] bip39_seed_out The seed bytes copied from the retained bip39 seed.
 * The buffer must be 64 bytes long.
 * @return true if the bip39 seed is available.
 */
USE_RESULT bool keystore_copy_bip39_seed(uint8_t* bip32_seed_out);

/**
 * Restores a seed.
 * @param[in] seed The seed that is to be restored.
 * @param[in] seed_length The length of the seed (max. 32 bytes).
 * @param[in] password The password with which we encrypt the seed.
 */
USE_RESULT keystore_error_t
keystore_encrypt_and_store_seed(const uint8_t* seed, size_t seed_length, const char* password);

/**
   Generates the seed, mixes it with host_entropy, and stores it encrypted with the
   password. The size of the host entropy determines the size of the seed. Can be either 16 or 32
   bytes, resulting in 12 or 24 BIP39 recovery words.
   @param[in] host_entropy bytes of entropy to be mixed in.
   @param[in] host_entropy_size must be 16 or 32.
*/
USE_RESULT keystore_error_t keystore_create_and_store_seed(
    const char* password,
    const uint8_t* host_entropy,
    size_t host_entropy_size);

/** Unlocks the keystore seed or checks the password:
 * If the keystore is locked, it decrypts and loads the seed, unlocking the keystore:
 * 1) loads the stored seed and tries to decrypt using password.
 * 2) if successful, the bip39 seed should be derived using keystore_unlock_bip39().
 * If the keystore is already unlocked, this function does *not* change the state (can be used to
 * check the password).
 * @param[in] password keystore password, used to decrypt the seed.
 * If it is false, the keystore is not unlocked.
 * @param[out] remaining_attempts_out will have the number of remaining attempts.
 * If zero, the keystore is locked until the device is reset.
 * @param[out] securechip_result_out, if not NULL, will contain the error code from
 * `securechip_kdf()` if there was a secure chip error, and 0 otherwise.
 * @return
 * - KEYSTORE_OK if they keystore was successfully unlocked
 * - KEYSTORE_ERR_* if unsuccessful.
 * Only call this if memory_is_seeded() returns true.
 */
USE_RESULT keystore_error_t
keystore_unlock(const char* password, uint8_t* remaining_attempts_out, int* securechip_result_out);

/** Unlocks the bip39 seed.
 * @param[in] mnemonic_passphrase bip39 passphrase used in the derivation. Use the
 * empty string if no passphrase is needed or provided.
 * @return returns false if there was a critital memory error, otherwise true.
 */
USE_RESULT bool keystore_unlock_bip39(const char* mnemonic_passphrase);

/**
 * Locks the keystore (resets to state before `keystore_unlock()`).
 */
void keystore_lock(void);

/**
 * @return false if the keystore is unlocked (keystore_unlock() followed by
 * keystore_unlock_bip39()), true otherwise.
 */
USE_RESULT bool keystore_is_locked(void);

/**
 * Converts a 16/24/32 byte seed into a BIP-39 mnemonic string.
 * Returns false if the seed size is invalid or the output string buffer is not large enough.
 */
USE_RESULT bool keystore_bip39_mnemonic_from_seed(
    const uint8_t* seed,
    size_t seed_size,
    char* mnemonic_out,
    size_t mnemonic_out_size);

/**
 * Turn a bip39 mnemonic into a seed. Make sure to use UTIL_CLEANUP_32 to destroy it.
 * Output can be fed into `keystore_encrypt_and_store_seed` to create a keystore from the mnemonic.
 * @param[in] mnemonic 12/18/24 word bip39 mnemonic
 * @param[out] seed_out must be 32 bytes
 * @param[out] seed_len_out will be the size of the seed
 */
USE_RESULT bool keystore_bip39_mnemonic_to_seed(
    const char* mnemonic,
    uint8_t* seed_out,
    size_t* seed_len_out);

/**
 * Returns the pointer to a word in the word list for the given index.
 * @param[in] idx The index into the word list. Must be smaller than BIP39_WORDLIST_LEN.
 * @param[out] word_out The pointer to the character array for the given index.
 */
USE_RESULT bool keystore_get_bip39_word(uint16_t idx, char** word_out);

/**
 * Get a commitment to the original nonce before tweaking it with the host nonce. This is part of
 * the ECDSA Anti-Klepto Protocol. For more details, check the docs of
 * `secp256k1_ecdsa_anti_exfil_signer_commit`.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[in] msg32 32 byte message which will be signed by `keystore_secp256k1_sign`.
 * @param[in] host_commitment must be `sha256(sha256(tag)||shas256(tag)||host_nonce)` where
 * host_nonce is passed to `keystore_secp256k1_sign()`. See
 * `secp256k1_ecdsa_anti_exfil_host_commit()`.
 * @param[out] client_commitment_out EC_PUBLIC_KEY_LEN bytes compressed signer nonce pubkey.
 */
USE_RESULT bool keystore_secp256k1_nonce_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* client_commitment_out);

// clang-format off
/**
 * Sign message with private key at the given keypath. Keystore must be unlocked.
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
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
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
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out);

/**
 * Get the seed to be used for u2f
 * @param seed_out Buffer for seed, must be KEYSTORE_U2F_SEED_LENGTH
 * @return true if succes
 */
USE_RESULT bool keystore_get_u2f_seed(uint8_t* seed_out);

/**
 * Get the seed to be used for ed25519 applications such as Cardano. The output is the root key to
 * BIP32-ED25519.
 * This implements a derivation compatible with Ledger according to
 * https://github.com/LedgerHQ/orakolo/blob/0b2d5e669ec61df9a824df9fa1a363060116b490/src/python/orakolo/HDEd25519.py.
 * @param seed_out Buffer for the seed. Must be 96 bytes. It will contain a 64 byte expanded
 * ed25519 private key followed by a 32 byte chain code.
 */
USE_RESULT bool keystore_get_ed25519_seed(uint8_t* seed_out);

/**
 * Sign a message that verifies against the pubkey tweaked using BIP-86.
 *
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len number of elements in keypath
 * @param[in] msg32 32 byte message to sign
 * @param[in] tweak 32 bytes, tweak private key before signing with this tweak. Use NULL to not
 *            tweak.
 * @param[out] sig64_out resulting 64 byte signature
 */
USE_RESULT bool keystore_secp256k1_schnorr_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* tweak,
    uint8_t* sig64_out);

#ifdef TESTING
/**
 * convenience to mock the keystore state (locked, seed) in tests.
 */
void keystore_mock_unlocked(const uint8_t* seed, size_t seed_len, const uint8_t* bip39_seed);

const uint8_t* keystore_test_get_retained_seed_encrypted(size_t* len_out);
const uint8_t* keystore_test_get_retained_bip39_seed_encrypted(size_t* len_out);
#endif

#endif
