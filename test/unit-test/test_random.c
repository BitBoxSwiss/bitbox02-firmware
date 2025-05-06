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

#include <random.h>
#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <test_random.h>
#include <cmocka.h>

int __wrap_rand(void)
{
    return mock();
}

int __wrap_rust_sha256(const unsigned char* data, size_t len, unsigned char* out)
{
    check_expected(data);
    check_expected(len);
    check_expected(out);
    return 0;
}

static void _test_random_32_bytes_mcu(void** state)
{
    uint8_t expected[RANDOM_NUM_SIZE] = {0};
    uint8_t buf[RANDOM_NUM_SIZE] = "12345678901234567890123456789012";
    // mock mcu rand()
    for (int i = 0; i < RANDOM_NUM_SIZE; i++) {
        will_return(__wrap_rand, i);
        expected[i] = buf[i] ^ i;
    }
    random_32_bytes_mcu(buf);
    assert_memory_equal(expected, buf, RANDOM_NUM_SIZE);
}

static void _test_random_32_bytes(void** state)
{
    uint8_t expected[RANDOM_NUM_SIZE] = {0};
    uint8_t buf[RANDOM_NUM_SIZE];
    // mock mcu rand()
    for (int i = 0; i < RANDOM_NUM_SIZE; i++) {
        will_return(__wrap_rand, i);
        expected[i] ^= i;
    }
    // mock sec rand()
    for (int i = 0; i < RANDOM_NUM_SIZE; i++) {
        will_return(__wrap_rand, RANDOM_NUM_SIZE - i);
        expected[i] ^= RANDOM_NUM_SIZE - i;
    }
    expect_memory(__wrap_rust_sha256, data, expected, RANDOM_NUM_SIZE);
    expect_value(__wrap_rust_sha256, len, RANDOM_NUM_SIZE);
    expect_value(__wrap_rust_sha256, out, buf);
    random_32_bytes(buf);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_random_32_bytes_mcu),
        cmocka_unit_test(_test_random_32_bytes),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
