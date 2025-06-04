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

#include <platform_config.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
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
#define SIGMOID(a) (0.0018F * (a) * abs(a) / (1 + 0.002F * (a) * (a)));

// We define our own true false which are more secure than stdbool true/false becuase it requires
// flipping many more bits.
// It is recommended to always compare against sectrue, even in the false case.
typedef uint32_t secbool_u32;
#define sectrue_u32 0x55555555u
#define secfalse_u32 0x00000000u

typedef uint8_t secbool_u8;
#define sectrue_u8 0x55u
#define secfalse_u8 0x00u

void util_zero(volatile void* dst, size_t len);

// `out` must be of size in_len*2+1. Use BB_HEX_SIZE() to compute the size.
void util_uint8_to_hex(const uint8_t* in_bin, size_t in_len, char* out);

#define UTIL_DBG_HEX_MAX_LEN 64
/// This function is for debug purposes only!
///
/// Don't use this in production, the returned pointer is only valid until this function is called
/// again. This function is not thread safe.
///
/// Returns a null terminated string, suitable for printing with printf in C.
///
/// Max `len` is UTIL_DBG_HEX_MAX_LEN. This function panics if `len` is to large.
///
/// Usage:
///    uint8_t arr[2] = {1,2};
///    util_log("%s", util_dbg_hex(arr, sizeof(arr)));
///
const char* util_dbg_hex(const uint8_t* bin, size_t len);

#define BB_HEX_SIZE(in_bin) (sizeof((in_bin)) * 2 + 1)

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

/**
 * Struct definining a rw buffer (buffer + length).
 */
typedef struct {
    /** Data buffer. */
    uint8_t* data;
    /** Actual length of the data. */
    size_t len;
    /** Maximum buffer capacity. */
    const size_t max_len;
} buffer_t;

/**
 * Struct definining a read-only buffer (buffer + length).
 */
typedef struct {
    /** Data buffer. */
    const uint8_t* data;
    /** Length of data contained in data. */
    const size_t len;
} in_buffer_t;

/**
 * Enum used to represent the result of an operation
 * which might return true, false, or "not finished yet".
 */
typedef enum { ASYNC_OP_TRUE, ASYNC_OP_FALSE, ASYNC_OP_NOT_READY } async_op_result_t;

#if !defined(NDEBUG)
void util_log(const char* fmt, ...) __attribute__((format(printf, 1, 2)));
void util_log_flush(void);
#else
#define util_log(...)
#define util_log_flush(...)
#endif

#if !defined(NDEBUG) || FACTORYSETUP == 1
void util_log_init(void);
#else
#define util_log_init(...)
#endif

#endif
