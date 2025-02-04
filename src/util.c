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
// ifdef here so that we don't have to use -Wno-unknown-pragmas on GCC
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincompatible-pointer-types-discards-qualifiers"
#endif
    rust_util_zero(rust_util_bytes_mut(dst, len));
#ifdef __clang__
#pragma clang diagnostic pop
#endif
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
        util_log("len too large, %zu > %d", len, UTIL_DBG_HEX_MAX_LEN);
    }
    static char buf[UTIL_DBG_HEX_MAX_LEN * 2 + 1] = {0};
    util_uint8_to_hex(bin, len, buf);
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
#define MAX_LOG_LENGTH 101

#if !defined(NDEBUG)
void util_log(const char* fmt, ...)
{
    char buf[MAX_LOG_LENGTH] = "";

    va_list va;
    va_start(va, fmt);
    int res = vsnprintf(buf, MAX_LOG_LENGTH, fmt, va);
    va_end(va);

    rust_log(buf);
    if (res > MAX_LOG_LENGTH - 1) {
        rust_log("The complete log line didn't fit\n");
    }
}

// We use these wrapper functions so that we can ifdef them out in release builds
void util_log_flush(void)
{
    rust_rtt_flush();
}

void util_log_init(void)
{
    rust_rtt_init();
}
#endif
