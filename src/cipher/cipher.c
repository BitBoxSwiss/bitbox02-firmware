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

#include <random.h>
#include <rust/rust.h>

#ifdef TESTING
    #include <fake_cipher.h>
#endif

bool cipher_aes_hmac_encrypt(
    const unsigned char* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* secret)
{
    uint8_t iv[32] = {0}; // only 16 bytes needed for IV.
#ifdef TESTING
    cipher_fake_iv(iv);
#else
    random_32_bytes(iv);
#endif
    rust_cipher_encrypt(
        rust_util_bytes(iv, 16),
        rust_util_bytes(secret, 32),
        rust_util_bytes(in, in_len),
        rust_util_bytes_mut(out, *out_len),
        out_len);
    return true;
}

bool cipher_aes_hmac_decrypt(
    const uint8_t* in,
    size_t in_len,
    uint8_t* out,
    size_t* out_len,
    const uint8_t* key)
{
    return rust_cipher_decrypt(
        rust_util_bytes(key, 32),
        rust_util_bytes(in, in_len),
        rust_util_bytes_mut(out, *out_len),
        out_len);
}
