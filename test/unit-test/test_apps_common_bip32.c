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

#include <apps/btc/btc_common.h>
#include <apps/common/bip32.h>
#include <btc_util.h>
#include <keystore.h>

static void _test_apps_common_bip32_xpub_from_protobuf(void** state)
{
    const char* test_xpub =
        "xpub6CfS9YHFUtkq9akpHuLwuUtzdCHbRzgnEhbb3LjxFoT174qf1QLdMqT665H753doP8UJpP1qCVScAXnW8mXe8X"
        "AHnbjQDvXQgAGHTbYqft1";
    const XPub xpub_in = btc_util_parse_xpub(test_xpub);
    struct ext_key xpub = {0};
    assert_true(apps_common_bip32_xpub_from_protobuf(&xpub_in, &xpub));

    char xpub_str[XPUB_ENCODED_LEN] = {0};
    assert_true(keystore_encode_xpub(&xpub, XPUB, xpub_str, sizeof(xpub_str)));
    assert_string_equal(xpub_str, test_xpub);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_apps_common_bip32_xpub_from_protobuf),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
