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

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

#include "util.h"

volatile void* util_zero(volatile void* dst, size_t len)
{
    volatile char* buf;
    for (buf = (volatile char*)dst; len; buf[--len] = 0)
        ;
    return dst;
}

void util_uint8_to_hex(const uint8_t* in_bin, const size_t in_len, char* out)
{
    static char digits[] = "0123456789abcdef";
    size_t i;
    for (i = 0; i < in_len; i++) {
        out[i * 2] = digits[(in_bin[i] >> 4) & 0xF];
        out[i * 2 + 1] = digits[in_bin[i] & 0xF];
    }
    out[in_len * 2] = '\0';
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

char* strdup(const char* s)
{
    const size_t len = strlen(s) + 1;
    char* new = malloc(len);
    if (new == NULL) {
        return NULL;
    }
    snprintf(new, len, "%s", s);
    return new;
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
