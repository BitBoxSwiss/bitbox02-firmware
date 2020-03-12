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

#include <hardfault.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "rust/rust.h"
#include "util.h"

void util_zero(volatile void* dst, size_t len)
{
// Rust doesn't have a volatile qualifier becuase volatile refers to the act of writing/reading not
// the data type.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdiscarded-qualifiers"
    rust_util_zero(rust_util_bytes_mut(dst, len));
#pragma GCC diagnostic pop
}

void util_uint8_to_hex(const uint8_t* in_bin, const size_t in_len, char* out)
{
    rust_util_uint8_to_hex(
        rust_util_bytes(in_bin, in_len), rust_util_cstr_mut(out, in_len * 2 + 1));
}

void util_cleanup_str(char** str)
{
    util_zero(*str, strlens(*str));
}

void util_cleanup_20(uint8_t** buf)
{
    util_zero(*buf, 20);
}

void util_cleanup_32(uint8_t** buf)
{
    util_zero(*buf, 32);
}

void util_cleanup_64(uint8_t** buf)
{
    util_zero(*buf, 64);
}

char* util_strdup(const char* str)
{
    char* result = strdup(str);
    if (!result) {
        Abort("malloc failed in util_strdup.");
    }
    return result;
}

bool safe_uint64_add(uint64_t* a, uint64_t b)
{
    if (a == NULL) {
        return false;
    }
    if (*a > ULLONG_MAX - b) {
        return false;
    }
    *a += b;
    return true;
}
