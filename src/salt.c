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

#include "salt.h"
#include "memory/memory.h"
#include "util.h"
#include <rust/rust.h>

#include <string.h>

bool salt_hash_data(const uint8_t* data, size_t data_len, const char* purpose, uint8_t* hash_out)
{
    if (data_len > 0 && data == NULL) {
        return false;
    }
    if (!purpose || !hash_out) {
        return false;
    }

    uint8_t salt_root[32];
    UTIL_CLEANUP_32(salt_root);
    if (!memory_get_salt_root(salt_root)) {
        return false;
    }

    void* ctx = rust_sha256_new();
    rust_sha256_update(ctx, salt_root, sizeof(salt_root));
    rust_sha256_update(ctx, purpose, strlen(purpose));
    if (data != NULL) {
        rust_sha256_update(ctx, data, data_len);
    }
    rust_sha256_finish(&ctx, hash_out);
    return true;
}
