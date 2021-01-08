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

#include <apps/eth/eth_common.h>

#include <hardfault.h>

#include <stdint.h>
#include <string.h>

#include <wally_bip32.h>

typedef struct {
    const uint8_t bigendian[32];
    const size_t bigendian_len;
    const unsigned int decimals;
    const char* unit;
    const char* expected_result;
} _format_test_t;

static void _test_eth_common_format_amount(void** state)
{
    _format_test_t tests[] = {
        {
            // 0
            .bigendian = "",
            .bigendian_len = 0,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "0 LOL",
        },
        {
            // 1000000
            .bigendian = "\x0f\x42\x40",
            .bigendian_len = 3,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "1 LOL",
        },
        {
            // 1100000
            .bigendian = "\x10\xc8\xe0",
            .bigendian_len = 3,
            .decimals = 6,
            .unit = "LOL",
            .expected_result = "1.1 LOL",
        },
        {
            // 38723987932742983742983742
            .bigendian = "\x20\x08\x1f\x97\x9a\x5c\x8d\x47\x29\x0e\x3e",
            .bigendian_len = 11,
            .decimals = 18,
            .unit = "LOL",
            .expected_result = "38723987.9327... LOL",
        },
        {
            // 123456
            .bigendian = "\x01\xe2\x40",
            .bigendian_len = 3,
            .decimals = 8,
            .unit = "LOL",
            .expected_result = "0.00123456 LOL",
        },
        {
            // 123456
            .bigendian = "\x01\xe2\x40",
            .bigendian_len = 3,
            .decimals = 8,
            .unit = "LOL",
            .expected_result = "0.00123456 LOL",
        },
        {
            // 124567890123
            .bigendian = "\x1d\x00\xd3\x28\xcb",
            .bigendian_len = 5,
            .decimals = 10,
            .unit = "LOL",
            .expected_result = "12.4567890123 LOL",
        },
        {
            // 1245678901234
            .bigendian = "\x01\x22\x08\x3f\x97\xf2",
            .bigendian_len = 6,
            .decimals = 11,
            .unit = "LOL",
            .expected_result = "12.4567890123... LOL",
        },
    };
    for (size_t i = 0; i < sizeof(tests) / sizeof(_format_test_t); i++) {
        const _format_test_t* test = &tests[i];

        char out[100];
        eth_common_format_amount(
            rust_util_bytes(test->bigendian, test->bigendian_len),
            test->unit,
            test->decimals,
            out,
            sizeof(out));
        assert_string_equal(out, test->expected_result);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_eth_common_format_amount),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
