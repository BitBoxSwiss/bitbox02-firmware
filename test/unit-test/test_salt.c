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

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wunused-parameter"

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

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_salt_hash_data),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}

#pragma GCC diagnostic pop
