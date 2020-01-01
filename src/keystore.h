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
#include <stdint.h>

#include <secp256k1.h>
#include <wally_bip32.h>
#include <wally_bip39.h> // for BIP39_WORDLIST_LEN

#define KEYSTORE_MAX_SEED_LENGTH (32)
#define KEYSTORE_U2F_SEED_LENGTH SHA256_LEN

typedef enum {
    KEYSTORE_OK,
    KEYSTORE_ERR_INCORRECT_PASSWORD,
    KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED,
    KEYSTORE_ERR_GENERIC,
} keystore_error_t;

#ifdef TESTING
/**
 * convenience to mock the keystore state (locked, seed) in tests.
 */
void mock_state(const uint8_t* retained_seed, const uint8_t* retained_bip39_seed);
#endif

/**
 * Copies the retained seed into the given buffer. The caller must
 * zero the seed with util_zero once it is no longer needed.
 * @param[out] seed_out The seed bytes copied from the retained seed.
 * The buffer should be KEYSTORE_MAX_SEED_LENGTH bytes long.
 * @param[out] length_out The seed length.
 * @return true if the seed was still retained.
 */
USE_RESULT bool keystore_copy_seed(uint8_t* seed_out, uint32_t* length_out);

/**
 * Restores a seed.
 * @param[in] seed The seed that is to be restored.
 * @param[in] seed_length The length of the seed (max. 32 bytes).
 * @param[in] password The password with which we encrypt the seed.
 */
USE_RESULT bool keystore_encrypt_and_store_seed(
    const uint8_t* seed,
    uint32_t seed_length,
    const char* password);

/**
   Generates 32 bytes of entropy, mixes it with host_entropy, and stores it encrypted with the
   password.
   @param[in] host_entropy 32 bytes of entropy to be mixed in.
*/
USE_RESULT bool keystore_create_and_store_seed(const char* password, const uint8_t* host_entropy);

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
 * @return
 * - KEYSTORE_OK if they keystore was successfully unlocked
 * - KEYSTORE_ERR_INCORRECT_PASSWORD if the password was wrong
 * - KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED if there were too many unsuccessful attempts. The device is
 *   reset in this case.
 * - KEYSTORE_ERR_GENERIC if there was a fatal memory error.
 * Only call this if memory_is_seeded() returns true.
 */
USE_RESULT keystore_error_t keystore_unlock(const char* password, uint8_t* remaining_attempts_out);

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
 * @return returns false if the keystore is not unlocked. String returned
 * should be freed using `wally_free_string`.
 */
USE_RESULT bool keystore_get_bip39_mnemonic(char** mnemonic_out);

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
 * Fills a uint8_t buffer with a fingerprint of the root public key at m/, which are the first four
 * bytes of its hash160 according to:
 * https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
 * @param[out] fingerprint, buffer of the fingerprint that is supposed to be filled, has to be size
 * 4
 */
USE_RESULT bool keystore_get_root_fingerprint(uint8_t* fingerprint);

/**
 * Can be used only if the keystore is unlocked. Returns the derived xpub,
 * using bip32 derivation. Derivation is done from the xprv master, so hardened
 * derivation is allowed.
 * On success, xpub_out must be destroyed using keystore_zero_xkey().
 * @return true on success, false on failure.
 */
USE_RESULT bool keystore_get_xpub(
    const uint32_t* keypath,
    size_t keypath_len,
    struct ext_key* hdkey_neutered_out);

/**
 * Safely destroy a xpub or xprv.
 */
void keystore_zero_xkey(struct ext_key* xkey);

/**
 * Returns the pointer to a word in the word list for the given index.
 * @param[in] idx The index into the word list. Must be smaller than BIP39_WORDLIST_LEN.
 * @param[out] word_out The pointer to the character array for the given index.
 */
USE_RESULT bool keystore_get_bip39_word(uint16_t idx, char** word_out);

typedef enum {
    KEYSTORE_SECP256K1_PUBKEY_HASH160,
    KEYSTORE_SECP256K1_PUBKEY_UNCOMPRESSED,
} keystore_secp256k1_pubkey_format;

/**
 * Return the serialized secp256k1 public key at the keypath.
 * @param[in] format Output format. For HASH160, the output is the hash of the public key.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[out] pubkey_out serialized output
 * @param[in] pubkey_out_len: must be 20 for HASH160, 65 for UNCOMPRESSED.
 * @return true on success, false if the keystore is locked or the input is invalid.
 */
USE_RESULT bool keystore_secp256k1_pubkey(
    keystore_secp256k1_pubkey_format format,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* pubkey_out,
    size_t pubkey_out_len);

/**
 * Sign message with private key at the given keypath. Keystore must be unlocked.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[in] msg32 32 byte message to sign
 * @param[out] sig resulting signature in compact format. Must be 64 bytes.
 * @param[out] recid recoverable id. Can be NULL if not needed.
 * Parse with secp256k1_ecdsa_signature_serialize_compact().
 * @return true on success, false if the keystore is locked.
 */
USE_RESULT bool keystore_secp256k1_sign(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    uint8_t* sig_compact_out,
    int* recid_out);

/**
 * Get the seed to be used for u2f
 * @param seed_out Buffer for seed, must be KEYSTORE_U2F_SEED_LENGTH
 * @return true if succes
 */
USE_RESULT bool keystore_get_u2f_seed(uint8_t* seed_out);

#endif
