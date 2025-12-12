// SPDX-License-Identifier: Apache-2.0

#include "salt.h"
#include "memory/memory.h"
#include "util.h"
#include <rust/rust.h>

#include <string.h>

bool salt_hash_data(const uint8_t* data, size_t data_len, const char* purpose, uint8_t* hash_out)
{
    if ((data_len > 0 && data == NULL) || purpose == NULL || hash_out == NULL) {
        return false;
    }
    return rust_salt_hash_data(
        rust_util_bytes(data, data_len), purpose, rust_util_bytes_mut(hash_out, 32));
}
