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
#include <wally_crypto.h> // for EC_PUBLIC_KEY_UNCOMPRESSED_LEN and EC_PUBLIC_KEY_LEN

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
    KEYSTORE_ERR_SC_KDF,
    KEYSTORE_ERR_SEED_SIZE,
    KEYSTORE_ERR_SALT,
    KEYSTORE_ERR_HASH,
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
   Generates the seed, mixes it with host_entropy, and stores it encrypted with the
   password. The size of the host entropy determines the size of the seed. Can be either 16 or 32
   bytes, resulting in 12 or 24 BIP39 recovery words.
   @param[in] host_entropy bytes of entropy to be mixed in.
   @param[in] host_entropy_size must be 16 or 32.
*/
USE_RESULT bool keystore_create_and_store_seed(
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
 * @return
 * - KEYSTORE_OK if they keystore was successfully unlocked
 * - KEYSTORE_ERR_* if unsuccessful.
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
 * @param[out] mnemonic_out resulting mnemonic
 * @param[in] mnemonic_out_size size of mnemonic_out. Should be at least 216 bytes (longest possible
 *            24 word phrase plus null terminator).
 * @return returns false if the keystore is not unlocked or the mnemonic does not fit.
 * The resulting string should be safely zeroed after use.
 */
USE_RESULT bool keystore_get_bip39_mnemonic(char* mnemonic_out, size_t mnemonic_out_size);

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

/**
 * Return the hash160 of the secp256k1 public key at the keypath.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[out] hash160_out serialized output. Must be HASH160_LEN bytes.
 * @return true on success, false if the keystore is locked or the input is invalid.
 */
USE_RESULT bool keystore_secp256k1_pubkey_hash160(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* hash160_out);

/**
 * Return the serialized secp256k1 public key at the keypath, in uncompressed format.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[out] pubkey_out serialized output. Must be EC_PUBLIC_KEY_UNCOMPRESSED_LEN bytes.
 * @return true on success, false if the keystore is locked or the input is invalid.
 */
USE_RESULT bool keystore_secp256k1_pubkey_uncompressed(
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* pubkey_out);

/**
 * Get a commitment to the original nonce before tweaking it with the host nonce. This is part of
 * the ECDSA Anti-Klepto Protocol. For more details, check the docs of
 * `secp256k1_ecdsa_anti_klepto_signer_commit`.
 * @param[in] keypath derivation keypath
 * @param[in] keypath_len size of keypath buffer
 * @param[in] msg32 32 byte message which will be signed by `keystore_secp256k1_sign`.
 * @param[in] host_commitment must be `sha256(sha256(tag)||shas256(tag)||host_nonce)` where
 * host_nonce is passed to `keystore_secp256k1_sign()`. See
 * `secp256k1_ecdsa_anti_klepto_host_commit()`.
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
 * @param[out] sig resulting signature in compact format. Must be 64 bytes.
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

typedef enum {
    XPUB,
    YPUB,
    ZPUB,
    TPUB,
    VPUB,
    UPUB,
    CAPITAL_VPUB,
    CAPITAL_ZPUB,
    CAPITAL_UPUB,
    CAPITAL_YPUB,
} xpub_type_t;
/**
 * Encode an xpub as a base58 string.
 * @param[in] xpub the xpub to encode.
 * @param[in] xpub_type determines the xpub format.
 * etc.
 * @param[out] out resulting string, must be at least of size `XPUB_ENCODED_LEN` (including the null
 * terminator).
 * @param[in] out_len size of `out`.
 * @return false on failure, true on success.
 */
USE_RESULT bool keystore_encode_xpub(
    const struct ext_key* xpub,
    xpub_type_t xpub_type,
    char* out,
    size_t out_len);

/**
 * Encode an xpub as a base58 string at the given `keypath`.
 * Args the same as `keystore_encode_xpub`.
 */
USE_RESULT bool keystore_encode_xpub_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    xpub_type_t xpub_type,
    char* out,
    size_t out_len);

#endif
