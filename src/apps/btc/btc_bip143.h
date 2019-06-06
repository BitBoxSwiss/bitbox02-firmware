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

#ifndef _APPS_BTC_BIP143_H
#define _APPS_BTC_BIP143_H

#include <stddef.h>
#include <stdint.h>

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
);
#endif
