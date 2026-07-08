// SPDX-License-Identifier: Apache-2.0

#include <stddef.h>
#include <stdint.h>

// GCC LTO needs externally_visible; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void* __attribute__((used, externally_visible)) memcpy(void* dst, const void* src, size_t n);
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void* __attribute__((used, externally_visible)) memset(void* dst, int c, size_t n);

// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void* __attribute__((used, externally_visible)) memcpy(void* dst, const void* src, size_t n)
{
    uint8_t* d = (uint8_t*)dst;
    const uint8_t* s = (const uint8_t*)src;
    while (n-- > 0) {
        *d++ = *s++;
    }
    return dst;
}

// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void* __attribute__((used, externally_visible)) memset(void* dst, int c, size_t n)
{
    uint8_t* d = (uint8_t*)dst;
    while (n-- > 0) {
        *d++ = (uint8_t)c;
    }
    return dst;
}
