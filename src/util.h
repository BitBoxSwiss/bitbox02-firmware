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

#ifndef _UTIL_H_
#define _UTIL_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#define UTIL_BUFFER_LEN 0x1000
#define MIN(a, b)                                       \
    __extension__({                                     \
        _Pragma("GCC diagnostic push");                 \
        _Pragma("GCC diagnostic ignored \"-Wshadow\""); \
        __typeof__(a) _a = (a);                         \
        __typeof__(b) _b = (b);                         \
        _Pragma("GCC diagnostic pop");                  \
        _a > _b ? _b : _a;                              \
    })
#define MAX(a, b)                                       \
    __extension__({                                     \
        _Pragma("GCC diagnostic push");                 \
        _Pragma("GCC diagnostic ignored \"-Wshadow\""); \
        __typeof__(a) _a = (a);                         \
        __typeof__(b) _b = (b);                         \
        _Pragma("GCC diagnostic pop");                  \
        _a > _b ? _a : _b;                              \
    })
#define strlens(s) ((s) == NULL ? 0 : strlen(s))
#define STREQ(a, b) (strcmp((a), (b)) == 0)
#define MEMEQ(a, b, c) (memcmp((a), (b), (c)) == 0)
#define SIGMOID(a) (0.0018F * (a)*abs(a) / (1 + 0.002F * (a) * (a)));

// We define our own true false which are more secure than stdbool true/false becuase it requires
// flipping many more bits.
// It is recommended to always compare against sectrue, even in the false case.
typedef uint32_t secbool_u32;
#define sectrue_u32 0x55555555u
#define secfalse_u32 0x00000000u

typedef uint8_t secbool_u8;
#define sectrue_u8 0x55u
#define secfalse_u8 0x00u

volatile void* util_zero(volatile void* dst, size_t len);
void util_clear_buffers(void);

// `out` must be of size in_len*2+1. Use BB_HEX_SIZE() to compute the size.
void util_uint8_to_hex(const uint8_t* in_bin, size_t in_len, char* out);

#define BB_HEX_SIZE(in_bin) (sizeof((in_bin)) * 2 + 1)

void util_reverse_bin(uint8_t* b, int len);

void util_cleanup_str(char** str);
#define UTIL_CLEANUP_STR(var) \
    char* __attribute__((__cleanup__(util_cleanup_str))) var##_clean __attribute__((unused)) = var;

void util_cleanup_20(uint8_t** buf);
void util_cleanup_32(uint8_t** buf);
void util_cleanup_64(uint8_t** buf);
#define UTIL_CLEANUP_20(var)                                                                     \
    uint8_t* __attribute__((__cleanup__(util_cleanup_20))) var##_clean __attribute__((unused)) = \
        var;
#define UTIL_CLEANUP_32(var)                                                                     \
    uint8_t* __attribute__((__cleanup__(util_cleanup_32))) var##_clean __attribute__((unused)) = \
        var;
#define UTIL_CLEANUP_64(var)                                                                     \
    uint8_t* __attribute__((__cleanup__(util_cleanup_64))) var##_clean __attribute__((unused)) = \
        var;

char* strdup(const char* s);

/**
 * adds b to a safely.
 * @return Returns false if the operation would overflow.
 */
bool safe_uint64_add(uint64_t* a, uint64_t b);

#if defined(__GNUC__)
#define UTIL_WARN_UNUSED_RESULT __attribute__((__warn_unused_result__))
#else
#define UTIL_WARN_UNUSED_RESULT
#endif

#endif
