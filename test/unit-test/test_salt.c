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

#include <salt.h>

#include <stdint.h>
#include <string.h>

static uint8_t _salt_root[32] = {
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
};

bool __wrap_memory_get_salt_root(uint8_t* salt_root_out)
{
    memcpy(salt_root_out, _salt_root, sizeof(_salt_root));
    return true;
}

static void _test_salt_hash_data(void** state)
{
    uint8_t data[9] = {0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88};
    uint8_t hash[32];
    assert_true(salt_hash_data(data, sizeof(data), "test purpose", hash));
    uint8_t expected_result[32] = {
        0x62, 0xdb, 0x8d, 0xcd, 0x47, 0xdd, 0xf8, 0xe8, 0x18, 0x09, 0xc3,
        0x77, 0xed, 0x96, 0x64, 0x38, 0x55, 0xd3, 0x05, 0x2b, 0xb7, 0x32,
        0x37, 0x10, 0x0c, 0xa8, 0x1f, 0x0f, 0x5a, 0x76, 0x11, 0xe6,
    };
    assert_memory_equal(hash, expected_result, 32);
}

static void _test_salt_hash_data_empty(void** state)
{
    const char* data = "";
    uint8_t hash[32];
    uint8_t expected_result[32] = {
        0x2d, 0xbb, 0x05, 0xdd, 0x73, 0xd9, 0x4e, 0xdb, 0xa6, 0x94, 0x66,
        0x11, 0xaa, 0xca, 0x36, 0x7f, 0x76, 0xc8, 0x09, 0xe9, 0x6f, 0x20,
        0x49, 0x9a, 0xd6, 0x74, 0xe5, 0x96, 0x05, 0x0f, 0x98, 0x33,
    };

    assert_true(salt_hash_data((const uint8_t*)data, 0, "", hash));
    assert_memory_equal(hash, expected_result, 32);

    assert_true(salt_hash_data(NULL, 0, "", hash));
    assert_memory_equal(hash, expected_result, 32);

    assert_false(salt_hash_data(NULL, 1, "", hash));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_salt_hash_data),
        cmocka_unit_test(_test_salt_hash_data_empty),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
