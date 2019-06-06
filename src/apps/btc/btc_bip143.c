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

#include "btc_bip143.h"

#include <crypto/sha2/sha256.h>

void btc_bip143_sighash(
    uint32_t version,
    const uint8_t* hash_prevouts, // 32 bytes
    const uint8_t* hash_sequence, // 32 bytes
    const uint8_t* prevout_hash, // 32 bytes
    uint32_t prevout_index, // 4 bytes
    const uint8_t* sighash_script,
    size_t sighash_script_len,
    uint64_t prevout_value, // 8 bytes
    uint32_t sequence, // 4 bytes
    const uint8_t* hash_outputs, // 32 bytes,
    uint32_t locktime, // 4 bytes
    uint32_t sighash_flags, // 4 bytes,
    uint8_t* out // 32 bytes result
)
{
    static sha256_context_t ctx = {0};
    sha256_reset(&ctx);
    // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
    // 1.
    // assumes little endian environment
    noise_sha256_update(&ctx, &version, sizeof(version));
    // 2.
    noise_sha256_update(&ctx, hash_prevouts, 32);
    // 3.
    noise_sha256_update(&ctx, hash_sequence, 32);
    // 4.
    noise_sha256_update(&ctx, prevout_hash, 32);
    // assumes little endian environment
    noise_sha256_update(&ctx, &prevout_index, sizeof(prevout_index));
    // 5.
    noise_sha256_update(&ctx, sighash_script, sighash_script_len);
    // 6.
    // assumes little endian environment
    noise_sha256_update(&ctx, &prevout_value, sizeof(prevout_value));
    // 7.
    // assumes little endian environment
    noise_sha256_update(&ctx, &sequence, sizeof(sequence));
    // 8.
    noise_sha256_update(&ctx, hash_outputs, 32);
    // 9.
    // assumes little endian environment
    noise_sha256_update(&ctx, &locktime, sizeof(locktime));
    // 10.
    // assumes little endian environment
    noise_sha256_update(&ctx, &sighash_flags, sizeof(sighash_flags));
    sha256_finish(&ctx, out);
    // double hash
    sha256_reset(&ctx);
    noise_sha256_update(&ctx, out, 32);
    sha256_finish(&ctx, out);
}
