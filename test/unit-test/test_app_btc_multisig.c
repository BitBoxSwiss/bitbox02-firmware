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

#include <btc_util.h>

#include <apps/btc/btc.h>
#include <apps/btc/btc_common.h>
#include <keystore.h>
#include <memory/memory.h>
#include <util.h>

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#include <wally_bip32.h>

const char* _multisig_name = "foo";

bool __wrap_memory_multisig_get_by_hash(const uint8_t* hash, char* name_out)
{
    snprintf(name_out, MEMORY_MULTISIG_NAME_MAX_LEN, "%s", _multisig_name);
    return true;
}

bool __wrap_apps_btc_confirm_multisig(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig,
    bool verify_xpubs)
{
    assert_string_equal(title, "Receive to");
    check_expected(coin);
    assert_string_equal(name, _multisig_name);
    check_expected(multisig);
    assert_false(verify_xpubs);
    return true;
}

static uint8_t _mock_seed[32] = {
    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
    0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
};

// sudden tenant fault inject concert weather maid people chunk youth stumble grit
static uint8_t _mock_bip39_seed[64] =
    "\x5a\x11\x5b\xcd\xbe\x0f\xe1\x70\x0e\x60\x95\x74\xf3\x57\xb0\x8d\xca\x37\x15\xb0\x35\xe6\xc7"
    "\x76\x77\x0a\xc7\xa0\xab\x2e\x2f\xea\x84\x0b\xa2\x76\x35\x06\xfa\x9c\x39\xde\x4d\xef\x27\xf6"
    "\xf8\xeb\xce\x36\x37\x02\xe9\x83\xe5\x49\xbd\x7d\xef\x14\xa0\x31\xbf\xdd";

typedef struct {
    BTCCoin coin;
    BTCScriptConfig_Multisig_ScriptType script_type;
    uint32_t threshold;
    size_t xpubs_count;
    const char* xpubs[20];
    size_t our_xpub_index;
    const char* out;
    const uint32_t keypath[6];
} testcase_t;

static testcase_t _tests[] = {
    /** P2WSH **/
    {
        .coin = BTCCoin_BTC,
        .threshold = 1,
        .xpubs_count = 2,
        .xpubs =
            {
                "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcL"
                "cYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                "xpub6EGAio99SxruuNxoBtG4fbYx3xM8fs7wjYJLRNcUg7UQin3LTANQiUYyb3RLjZ2EAyLsQBrtbNENUG"
                "h3oWzjHtgfQ3mtjPNFgNMronzTTVR",
            },
        .our_xpub_index = 1,
        .out = "bc1q2fhgukymf0caaqrhfxrdju4wm94wwrch2ukntl5fuc0faz8zm49q0h6ss8",
        .keypath =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
                1,
                2,
            },
    },
    {
        .coin = BTCCoin_TBTC,
        .threshold = 1,
        .xpubs_count = 2,
        .xpubs =
            {
                "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3p"
                "dQPdtnYUCugjeksHSbyZT7rq38VQF",
                "xpub6ERxBysTYfQyY4USv6c6J1HNVv9hpZFN9LHVPu47Ac4rK8fLy6NnAeeAHyEsMvG4G66ay5aFZii2VM"
                "7wT3KxLKX8Q8keZPd67kRGmrD1WJj",
            },
        .our_xpub_index = 0,
        .out = "tb1qw2scxk3zq0znr4ug9xkf3n7nfjsc8ldvemrm9dxjpl847zyu6afsfjjy28",
        .keypath =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
                1,
                2,
            },
    },
    {
        .coin = BTCCoin_TBTC,
        .threshold = 7,
        .xpubs_count = 15,
        .xpubs =
            {
                // clang-format off
            "xpub6Eu7xJRyXRCi4eLYhJPnfZVjgAQtM7qFaEZwUhvgxGf4enEZMxevGzWvZTawCj9USP2MFTEhKQAwnqHwoaPHetTLqGuvq5r5uaLKyGx5QDZ",
            "xpub6EQcxF2jFkYGn89AwoQJEEJkYMbRjED9AZgt7bkxQA5BLhZEoaQpUHcADbB5GxcMrTdDSGmjP7M3u462Q9otyE2PPam66P5KFLWitPVfYz9",
            "xpub6EP4EycVS5dq1PN7ZqsxBtptkYhfLvLGokZjnB3fvPshMiAohh6E5TaJjAafZWoPRjo6uiZxhtDXLgCuk81ooQgwrsnEdfSWSfa4VUtX8nu",
            "xpub6Eszd4BGGmHShcGtys5gbvV2zrBtW1gaorKf9YuvV4L3bePw7XePyyb2DKswZ5AhFfkcQwjQsiJEUTKhfRstRdHZUjQnJ2RJoQqL8g7FS4b",
            "xpub6Df3nbvH6P3FTvjgKaZcSuydyEofK545U4Bb15JY8R9MtFkKrhYrc3bpEF6fHtNM7xQ1qHwsVpS56TJWUjbKcmRwPkQr17ovV2RaVSJaBq3",
            "xpub6FQQ62gUYzS9wnHWHMPLWrpVnzS8xAf8XvfW1xzXEXTkTCtBrfbeww2zNeCgm3PbueMoq8opQvQDzp5Yf9EtiqVd7d1ASDoWSC1m7g1KHza",
            "xpub6EQNZUUAzJAoFAVVetYUrFVrf7mLyYsnHiQihkA3KPhoRHx7m6SgKBYV4z5Rd9CvUc11ACN8Ap5Wxigt6GYRPUqXGFfm3833ezJpjAmvJKt",
            "xpub6EGZy7cizYn2zUf9NT4qJ3Kr1ZrxdzPRcv2CwAnB1BTGWw7n9ZgDYvwmzzJXM6V7AgZ6CL3DrARZk5DzM9o8tz2RVTeC7QoHh9SxbW3b7Pw",
            "xpub6DaV7oCAkm4HJQMoProrrKYq1RvcgpStgYUCzLRaaeJSBSy9WBRFMNnQyAWJUYy9myUFRTvogq1C2f7x4A2yhtYgr7gL6eZXv2eJvzU12pe",
            "xpub6FFVRbdHt5DgHqR69KuWXRVDp93e1xKxv8rRLwhhCGnWaoF1ecnfdxpg2Nf1pvJTgT1UYg28CVt7YbUXFJL86vi9FaPN9QGtWLeCmf9dA24",
            "xpub6FNywxebMjvSSginZrk7DfNmAHvPJAy3j6pJ9FmUQCoh4FKPzNymdHnkA1z77Ke4GK7g5GkdrBhpyXfWTbZkH6Yo1t4v524wDwF8SAKny9J",
            "xpub6F1V9y6gXejomurTy2hN1UDCJidYahVkqtQJSZLYmcPcPDWkxGgWTrrLnCrCkGESSUSq6GpVVQx9kejPV97BEa9F85utABNL9r6xyPZFiDm",
            "xpub6ECHc4kmTC2tQg2ZoAoazwyag9C4V6yFsZEhjwMJixdVNsUibot6uEvsZY38ZLVqWCtyc9gbzFEwHQLHCT8EiDDKSNNsFAB8NQYRgkiAQwu",
            "xpub6F7CaxXzBCtvXwpRi61KYyhBRkgT1856ujHV5AbJK6ySCUYoDruBH6Pnsi6eHkDiuKuAJ2tSc9x3emP7aax9Dc3u7nP7RCQXEjLKihQu6w1",
            "xpub6EMfjyGVUvwhpc3WKN1zXhMFGKJGMaSBPqbja4tbGoYvRBSXeTBCaqrRDjcuGTcaY95JrrAnQvDG3pdQPdtnYUCugjeksHSbyZT7rq38VQF",
                // clang-format on
            },
        .our_xpub_index = 14,
        .out = "tb1qndz49j0arp8g6jc8vcrgf9ugrsw96a0j5d7vqcun6jev6rlv47jsv99y5m",
        .keypath =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                2 + BIP32_INITIAL_HARDENED_CHILD,
                1,
                2,
            },
    },

    /** P2SH **/
    {
        .coin = BTCCoin_BTC,
        .script_type = BTCScriptConfig_Multisig_ScriptType_P2WSH_P2SH,
        .threshold = 2,
        .xpubs_count = 2,
        .xpubs =
            {
                "xpub6FEZ9Bv73h1vnE4TJG4QFj2RPXJhhsPbnXgFyH3ErLvpcZrDcynY65bhWga8PazWHLSLi23PoBhGcL"
                "cYW6JRiJ12zXZ9Aop4LbAqsS3gtcy",
                "xpub6EGAio99SxrurYgGH5BEzSiM4ZNedDX68RTGTSzGt5gk4STbs8B35ASC3RMdysGhJ7dJfffQcQEzFA"
                "kLxvMTyDsdrvMmsd45gr8pDmtTzEX",
            },
        .our_xpub_index = 1,
        .out = "3BKdK5c2kcFrNmmJbMAeWuveaoYDB4BYvu",
        .keypath =
            {
                48 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                0 + BIP32_INITIAL_HARDENED_CHILD,
                1 + BIP32_INITIAL_HARDENED_CHILD,
                1,
                0,
            },
    },
};

static void _test_app_btc_address_multisig(void** state)
{
    mock_state(_mock_seed, _mock_bip39_seed);

    for (size_t test_case_index = 0; test_case_index < sizeof(_tests) / sizeof(testcase_t);
         test_case_index++) {
        const testcase_t* test_case = &_tests[test_case_index];

        BTCScriptConfig_Multisig multisig = {
            .threshold = test_case->threshold,
            .xpubs_count = test_case->xpubs_count,
            .our_xpub_index = test_case->our_xpub_index,
            .script_type = test_case->script_type,
        };
        for (size_t xpub_idx = 0; xpub_idx < test_case->xpubs_count; xpub_idx++) {
            multisig.xpubs[xpub_idx] = btc_util_parse_xpub(test_case->xpubs[xpub_idx]);
        }

        char out[XPUB_ENCODED_LEN] = {0};
        expect_value(__wrap_apps_btc_confirm_multisig, coin, test_case->coin);
        expect_memory(__wrap_apps_btc_confirm_multisig, multisig, &multisig, sizeof(multisig));
        bool result = app_btc_address_multisig(
            test_case->coin,
            &multisig,
            test_case->keypath,
            sizeof(test_case->keypath) / sizeof(uint32_t),
            out,
            sizeof(out),
            false);
        assert_int_equal(APP_BTC_OK, result);
        assert_string_equal(out, test_case->out);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_app_btc_address_multisig),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
