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

// including C file to test static functions
#include <commander/commander.c>
#include <generated/hww.pb.h>
#include <pb_decode.h>
#include <pb_encode.h>
#include <random.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wbad-function-cast"
#pragma GCC diagnostic ignored "-Wmissing-prototypes"

void __wrap_random_32_bytes(uint8_t* buf);
void __wrap_random_32_bytes(uint8_t* buf)
{
    memcpy(buf, (uint8_t*)(mock()), RANDOM_NUM_SIZE);
}

void __wrap_workflow_confirm_dismiss(const char* title, const char* body)
{
    check_expected(title);
    check_expected(body);
}

static void test_random(void** state)
{
    (void)state;
    RandomNumberResponse random_number_response;
    uint8_t expected[RANDOM_NUM_SIZE] = {0x00, 0x11, 0x22, 0x33, 0x00, 0x11, 0x22, 0x33,
                                         0x00, 0x11, 0x22, 0x33, 0x00, 0x11, 0x22, 0x33,
                                         0x00, 0x11, 0x22, 0x33, 0x00, 0x11, 0x22, 0x33,
                                         0x00, 0x11, 0x22, 0x33, 0x00, 0x11, 0x22, 0x33};
    will_return(__wrap_random_32_bytes, expected);
    expect_string(__wrap_workflow_confirm_dismiss, title, "Random");
    expect_string(
        __wrap_workflow_confirm_dismiss,
        body,
        "0011223300112233\n0011223300112233\n0011223300112233\n0011223300112233");
    _api_process_random(&random_number_response);
    assert_memory_equal(&random_number_response.number, expected, sizeof(expected));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_random),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
#pragma GCC diagnostic pop
