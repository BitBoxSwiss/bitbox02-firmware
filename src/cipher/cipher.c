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

#include "cipher.h"
#include <ctaes-cbc.h>

#include <stdlib.h>
#include <string.h>

#include <random.h>
#include <util.h>
#include <wally_crypto.h>

#define N_BLOCK (16U)

static bool _derive_hmac_keys(
    const uint8_t* secret,
    uint8_t* encryption_key_out,
    uint8_t* authentication_key_out)
{
    uint8_t hash[64];
    UTIL_CLEANUP_64(hash);
    if (wally_sha512(secret, 32, hash, sizeof(hash)) != WALLY_OK) {
        return false;
    }
    memcpy(encryption_key_out, hash, 32);
    memcpy(authentication_key_out, hash + 32, 32);
    return true;
}

// out_len must be at least in_len + N_BLOCK + N_BLOCK
// necessary in_len/out_len range checks are done in cipher_aes_hmac_encrypt().
static void _aes_encrypt(
    const uint8_t* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* key)
{
    size_t padlen = N_BLOCK - in_len % N_BLOCK;
    size_t inpadlen = in_len + padlen;
    uint8_t inpad[inpadlen];
    *out_len = inpadlen + N_BLOCK;

    // PKCS7 padding
    memcpy(inpad, in, in_len);
    for (size_t i = 0; i < padlen; i++) {
        inpad[in_len + i] = padlen;
    }

    uint8_t iv[32] = {0}; // only 16 bytes needed for IV.
    random_32_bytes(iv);
    memcpy(out, iv, N_BLOCK);

    AES256_CBC_ctx ctx = {0};
    AES256_CBC_init(&ctx, key, iv);
    AES256_CBC_encrypt(&ctx, inpadlen / N_BLOCK, out + N_BLOCK, inpad);
    *out_len = inpadlen + N_BLOCK;

    util_zero(inpad, inpadlen);
    util_zero(&ctx, sizeof(ctx));
}

bool cipher_aes_hmac_encrypt(
    const unsigned char* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* secret)
{
    // in_len + iv + pad + hmac
    if (*out_len != in_len + N_BLOCK + N_BLOCK + 32) {
        return false;
    }
    uint8_t encryption_key[32];
    UTIL_CLEANUP_32(encryption_key);
    uint8_t authentication_key[32];
    UTIL_CLEANUP_32(authentication_key);
    if (!_derive_hmac_keys(secret, encryption_key, authentication_key)) {
        return false;
    }

    size_t encrypt_len = in_len + 32;
    _aes_encrypt(in, in_len, out, &encrypt_len, encryption_key);

    *out_len = encrypt_len + 32;

    return wally_hmac_sha256(
               authentication_key,
               sizeof(authentication_key),
               out,
               encrypt_len,
               out + encrypt_len,
               32) == WALLY_OK;
}

// necessary in_len/out_len range checks are done in cipher_aes_hmac_decrypt().
static bool _aes_decrypt(
    const uint8_t* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* key)
{
    uint8_t dec_pad[in_len - N_BLOCK];
    const uint8_t* iv = in; // first 16 bytes

    AES256_CBC_ctx ctx = {0};
    AES256_CBC_init(&ctx, key, iv);
    AES256_CBC_decrypt(&ctx, in_len / N_BLOCK - 1, dec_pad, in + N_BLOCK);

    // Strip PKCS7 padding
    uint8_t padlen = dec_pad[in_len - N_BLOCK - 1];
    if (padlen > N_BLOCK) {
        goto error;
    }
    if (in_len < N_BLOCK + padlen) {
        goto error;
    }
    for (size_t i = 0; i < padlen; i++) {
        if (dec_pad[in_len - N_BLOCK - 1 - i] != padlen) {
            goto error;
        }
    }
    memcpy(out, dec_pad, in_len - N_BLOCK - padlen);
    *out_len = in_len - N_BLOCK - padlen;
    util_zero(dec_pad, sizeof(dec_pad));
    util_zero(&ctx, sizeof(ctx));
    return true;
error:
    util_zero(dec_pad, sizeof(dec_pad));
    util_zero(&ctx, sizeof(ctx));
    return false;
}

bool cipher_aes_hmac_decrypt(
    const uint8_t* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* key)
{
    // iv + pad + hmac
    if (in_len < N_BLOCK + N_BLOCK + 32) {
        return false;
    }
    // have space for at least in_len - iv - hmac
    if (*out_len != in_len - N_BLOCK - 32) {
        return false;
    }

    uint8_t encryption_key[32];
    UTIL_CLEANUP_32(encryption_key);
    uint8_t authentication_key[32];
    UTIL_CLEANUP_32(authentication_key);

    if (!_derive_hmac_keys(key, encryption_key, authentication_key)) {
        return false;
    }

    uint8_t hmac[32];
    UTIL_CLEANUP_32(hmac);
    if (wally_hmac_sha256(
            authentication_key,
            sizeof(authentication_key),
            in,
            in_len - sizeof(hmac),
            hmac,
            sizeof(hmac)) != WALLY_OK) {
        return false;
    }

    if (!MEMEQ(hmac, in + in_len - sizeof(hmac), sizeof(hmac))) {
        return false;
    }
    return _aes_decrypt(in, in_len - sizeof(hmac), out, out_len, encryption_key);
}
