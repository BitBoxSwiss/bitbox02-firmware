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
#include <time.h>

#include "rust/rust.h"
#include "util.h"
#include <version.h>

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
        rust_util_bytes(in_bin, in_len), rust_util_bytes_mut((uint8_t*)out, in_len * 2 + 1));
}

const char* util_dbg_hex(const uint8_t* bin, const size_t len)
{
    if (len > UTIL_DBG_HEX_MAX_LEN) {
        util_log("len too large, %u > %d", (unsigned int)len, UTIL_DBG_HEX_MAX_LEN);
    }
    static char buf[UTIL_DBG_HEX_MAX_LEN * 2 + 1] = {0};
    util_uint8_to_hex(bin, MIN(len, (unsigned int)UTIL_DBG_HEX_MAX_LEN), buf);
    return buf;
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

// Max message size is MAX_LOG_LENGTH-1, becuase vsnprintf will always print a null character
#define MAX_LOG_LENGTH 200

void util_log(const char* fmt, ...)
{
#if !defined(NDEBUG)
    char buf[MAX_LOG_LENGTH] = "";

    va_list va;
    va_start(va, fmt);
    int res = vsnprintf(buf, MAX_LOG_LENGTH, fmt, va);
    va_end(va);

    rust_log(buf);
    if (res > MAX_LOG_LENGTH - 1) {
        rust_log("The complete log line didn't fit\n");
    }
#else
    (void)fmt;
#endif
}
