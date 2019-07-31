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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <sha3.h>

static void _test_sha3_256(void** state)
{
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    const char* msg = "message";
    rhash_sha3_update(&ctx, (const unsigned char*)msg, 7);
    uint8_t hash[32] = {0};
    const uint8_t expected_hash[32] = {
        0x7f, 0x4a, 0x23, 0xd9, 0x0d, 0xe9, 0x0d, 0x10, 0x07, 0x54, 0xf8,
        0x2d, 0x6c, 0x14, 0x07, 0x3b, 0x7f, 0xb4, 0x66, 0xf7, 0x6f, 0xd1,
        0xf6, 0x1b, 0x18, 0x7b, 0x9f, 0x39, 0xc3, 0xff, 0xd8, 0x95,
    };
    rhash_sha3_final(&ctx, hash);
    assert_memory_equal(hash, expected_hash, sizeof(hash));
}

static void _test_keccak_256(void** state)
{
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    const char* msg = "message";
    rhash_sha3_update(&ctx, (const unsigned char*)msg, 7);
    uint8_t hash[32] = {0};
    const uint8_t expected_hash[32] = {
        0xc2, 0xba, 0xf6, 0xc6, 0x66, 0x18, 0xac, 0xd4, 0x9f, 0xb1, 0x33,
        0xce, 0xbc, 0x22, 0xf5, 0x5b, 0xd9, 0x07, 0xfe, 0x9f, 0x0d, 0x69,
        0xa7, 0x26, 0xd4, 0x5b, 0x75, 0x39, 0xba, 0x6b, 0xbe, 0x08,
    };
    rhash_keccak_final(&ctx, hash);
    assert_memory_equal(hash, expected_hash, sizeof(hash));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_sha3_256),
        cmocka_unit_test(_test_keccak_256),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
