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

static void _test_apps_common_bip32_xpub_from_protobuf(void** state)
{
    const char* test_xpub =
        "xpub6CfS9YHFUtkq9akpHuLwuUtzdCHbRzgnEhbb3LjxFoT174qf1QLdMqT665H753doP8UJpP1qCVScAXnW8mXe8X"
        "AHnbjQDvXQgAGHTbYqft1";
    const XPub xpub_in = btc_util_parse_xpub(test_xpub);
    struct ext_key xpub = {0};
    assert_true(apps_common_bip32_xpub_from_protobuf(&xpub_in, &xpub));

    const uint8_t version[4] = {0x04, 0x88, 0xb2, 0x1e};
    char xpub_str[113] = {0};
    assert_true(btc_common_encode_xpub(&xpub, version, xpub_str, sizeof(xpub_str)));
    assert_string_equal(xpub_str, test_xpub);
}

static void _test_apps_common_bip32_xpubs_are_equal(void** state)
{
    uint8_t seed1[16] = "0123456789012345";
    struct ext_key xpub1;
    assert_int_equal(
        bip32_key_from_seed(
            seed1, sizeof(seed1), BIP32_VER_MAIN_PRIVATE, BIP32_FLAG_SKIP_HASH, &xpub1),
        WALLY_OK);

    uint8_t seed2[16] = "1234567890123456";
    struct ext_key xpub2;
    assert_int_equal(
        bip32_key_from_seed(seed2, sizeof(seed2), BIP32_VER_MAIN_PRIVATE, 0, &xpub2), WALLY_OK);

    assert_true(apps_common_bip32_xpubs_equal(&xpub1, &xpub1));
    assert_true(apps_common_bip32_xpubs_equal(&xpub2, &xpub2));
    assert_false(apps_common_bip32_xpubs_equal(&xpub1, &xpub2));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_apps_common_bip32_xpub_from_protobuf),
        cmocka_unit_test(_test_apps_common_bip32_xpubs_are_equal),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
