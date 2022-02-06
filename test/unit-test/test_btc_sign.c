// Copyright 2019 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
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

#include <stdio.h>

#include <apps/btc/btc_sign.h>
#include <apps/btc/confirm_locktime_rbf.h>
#include <keystore.h>
#include <rust/rust.h>
#include <secp256k1_ecdsa_s2c.h>
#include <wally_bip32.h>
#include <wally_crypto.h>
#include <workflow/confirm.h>

void __wrap_workflow_status_blocking(const char* msg, bool status_success) {}

bool __wrap_workflow_confirm_blocking(const confirm_params_t* params)
{
    check_expected(params->body);
    return mock();
}

bool __wrap_workflow_verify_recipient(const char* recipient, const char* amount)
{
    check_expected(recipient);
    check_expected(amount);
    return mock();
}

bool __wrap_workflow_verify_total(const char* total, const char* fee)
{
    check_expected(total);
    check_expected(fee);
    return mock();
}

void __wrap_rust_bitcoin_util_format_amount(uint64_t satoshi, CStr unit, CStrMut out)
{
    check_expected(satoshi);
    check_expected(unit.buf);
    snprintf(out.buf, out.cap, "%s", (const char*)(mock()));
}

bool __real_btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    size_t keypath_len,
    uint32_t expected_coin);
bool __wrap_btc_common_is_valid_keypath_address_simple(
    BTCScriptConfig_SimpleType script_type,
    const uint32_t* keypath,
    const size_t keypath_len,
    const uint32_t expected_coin)
{
    check_expected(script_type);
    check_expected(keypath);
    assert_int_equal(keypath_len, 5);
    return __real_btc_common_is_valid_keypath_address_simple(
        script_type, keypath, keypath_len, expected_coin);
}

bool __real_keystore_antiklepto_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out);
bool __wrap_keystore_antiklepto_secp256k1_commit(
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    check_expected(msg32);
    return __real_keystore_antiklepto_secp256k1_commit(
        keypath, keypath_len, msg32, host_commitment, signer_commitment_out);
}

// Mnemonic: purity concert above invest pigeon category peace tuition hazard vivid latin since
// legal speak nation session onion library travel spell region blast estate stay
static uint8_t _mock_seed[32] =
    "\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a"
    "\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e";
static uint8_t _mock_bip39_seed[64] =
    "\x34\xf5\x8c\x62\x94\x0a\xee\x78\x74\x9b\x39\x27\x0e\x3f\x1f\x47\x43\x90\x52\xef\xf9\x90\xf9"
    "\xcd\xd7\xd4\x67\x45\x61\xdb\x11\xf4\x35\xc9\x36\x96\x90\xd2\x21\x3c\x2c\xbf\x15\x45\x9a\x0d"
    "\x12\x32\x10\x6c\x73\x63\x87\xac\x03\x8e\x23\xc3\x52\xbb\xbb\x7c\x39\xdf";

typedef struct {
    // true if the sigs should be checked against fixtures.
    bool check_sigs;
    // the sum of the inputs in the 2nd pass can't be higher than in the first
    // for all inputs.
    bool input_sum_changes;
    // at the last input, the sum of the inputs in the 2nd pass must be the same
    // as the sum of the inputs in the first pass
    bool input_sum_last_mismatch;
    // can't init twice in a row -> first input expected
    bool state_init_after_init;
    // wrong state transition
    bool state_output_after_init;
    // value 0 is invalid
    bool wrong_input_value;
    // if value addition in inputs would overflow
    bool overflow_input_values_pass1;
    bool overflow_input_values_pass2;
    // if outgoing sum overflows
    bool overflow_output_out;
    // if change overflows
    bool overflow_output_ours;
    // exercise the antiklepto protocol
    bool antikepto;
} _modification_t;

typedef struct {
    BTCSignInputRequest input;
} _input_t;

// _sign goes through the whole sign process of an example tx, successfully.
// The passed params malleate the behavior to induce expected failures.
static void _sign(const _modification_t* mod)
{
    // Need keystore to derive change and input scripts
    keystore_mock_unlocked(_mock_seed, sizeof(_mock_seed), _mock_bip39_seed);

    BTCSignInitRequest init_req = {
        .coin = BTCCoin_BTC,
        .script_configs_count = 1,
        .script_configs =
            {
                {
                    // First script config varies per test (testing all possible types)
                    .script_config =
                        {
                            .which_config = BTCScriptConfig_simple_type_tag,
                            .config =
                                {
                                    .simple_type = BTCScriptConfig_SimpleType_P2WPKH,
                                },
                        },
                    .keypath_count = 3,
                    .keypath =
                        {
                            84 + BIP32_INITIAL_HARDENED_CHILD,
                            0 + BIP32_INITIAL_HARDENED_CHILD,
                            10 + BIP32_INITIAL_HARDENED_CHILD,
                        },
                },
            },
        .version = 1,
        .num_inputs = 2,
        .num_outputs = 6,
        .locktime = 0,
    };

    _input_t inputs[2] = {
        {
            .input =
                {
                    .prevOutHash =
                        "\x45\x17\x74\x50\x1b\xaf\xdf\xf7\x46\x09\x0e\x06\x16\xd9\x5e\xd0\x80\xd7"
                        "\x82\x9a\xfe\xa2\xbd\x97\x8a\xf8\x11\xf4\x5e\x43\x81\x39",
                    .prevOutIndex = 1,
                    .prevOutValue = 1010000000, // btc 10.1, matches prevout tx output at index 1.
                    .sequence = 0xffffffff,
                    .keypath_count = 5,
                    .keypath =
                        {
                            init_req.script_configs[0].keypath[0],
                            init_req.script_configs[0].keypath[1],
                            init_req.script_configs[0].keypath[2],
                            0,
                            5,
                        },
                },
        },
        {
            .input =
                {
                    .prevOutHash =
                        "\x40\x9b\x4f\x56\xca\x9f\x06\xcb\x88\x28\x03\xad\x55\x4b\xeb\x1d\x9e\xf8"
                        "\x78\x07\xf0\x52\x29\xe7\x55\x15\xe4\xb2\xaa\x87\x69\x1d",
                    .prevOutIndex = 0,
                    .prevOutValue = 1020000000, // btc 10.2, matches prevout tx output at index 0.
                    .sequence = 0xffffffff,
                    .keypath_count = 5,
                    .keypath =
                        {
                            init_req.script_configs[0].keypath[0],
                            init_req.script_configs[0].keypath[1],
                            init_req.script_configs[0].keypath[2],
                            0,
                            7,
                        },
                },
        },
    };

    if (mod->wrong_input_value) {
        inputs[0].input.prevOutValue = 0;
    }
    if (mod->overflow_input_values_pass1) {
        inputs[1].input.prevOutValue = ULLONG_MAX - inputs[0].input.prevOutValue + 1;
    }

    BTCSignOutputRequest outputs[6] = {
        {
            .ours = false,
            .type = BTCOutputType_P2PKH,
            .value = 100000000, // btc 1
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2SH,
            .value = 1234567890, // btc 12.3456789
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2WPKH,
            .value = 6000, // btc .00006
            .payload =
                {
                    .size = 20,
                    .bytes =
                        {
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                        },
                },
        },
        {
            .ours = false,
            .type = BTCOutputType_P2WSH,
            .value = 7000, // btc .00007
            .payload =
                {
                    .size = 32,
                    .bytes =
                        {
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                        },
                },
        },
        {
            // change
            .ours = true,
            .value = 690000000, // btc 6.9
            .keypath_count = 5,
            .keypath =
                {
                    init_req.script_configs[0].keypath[0],
                    init_req.script_configs[0].keypath[1],
                    init_req.script_configs[0].keypath[2],
                    1,
                    3,
                },
        },
        {
            // change #2
            .ours = true,
            .value = 100,
            .keypath_count = 5,
            .keypath =
                {
                    init_req.script_configs[0].keypath[0],
                    init_req.script_configs[0].keypath[1],
                    init_req.script_configs[0].keypath[2],
                    1,
                    30,
                },
        },
    };
    const uint64_t total = 1339999900; // sum of all non-change outputs + fee
    const uint64_t fee = 5419010; // sum of all inputs - sum of all outputs

    if (mod->overflow_output_out) {
        outputs[0].value = ULLONG_MAX;
    }
    if (mod->overflow_output_ours) {
        outputs[4].value = ULLONG_MAX;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_init(&init_req));

    // === Inputs Pass 1

    // First input, pass1.
    if (!mod->wrong_input_value) {
        expect_value(
            __wrap_btc_common_is_valid_keypath_address_simple,
            script_type,
            init_req.script_configs[inputs[0].input.script_config_index]
                .script_config.config.simple_type);
        expect_memory(
            __wrap_btc_common_is_valid_keypath_address_simple,
            keypath,
            inputs[0].input.keypath,
            inputs[0].input.keypath_count * sizeof(uint32_t));
    }
    if (mod->wrong_input_value) {
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input_pass1(&inputs[0].input, false));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_input_pass1(&inputs[0].input, false));

    // Second input, pass1.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[1].input.keypath,
        inputs[1].input.keypath_count * sizeof(uint32_t));
    if (mod->overflow_input_values_pass1) {
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT, app_btc_sign_input_pass1(&inputs[1].input, true));
        return;
    }
    assert_int_equal(APP_BTC_OK, app_btc_sign_input_pass1(&inputs[1].input, true));

    // === Outputs

    // First output
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[0].value);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount0");
    expect_string(
        __wrap_workflow_verify_recipient, recipient, "12ZEw5Hcv1hTb6YUQJ69y1V7uhcoDz92PH");
    expect_string(__wrap_workflow_verify_recipient, amount, "amount0");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[0], false));

    // Second output
    if (mod->overflow_output_out) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[1], false));
        return;
    }
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[1].value);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount1");
    expect_string(
        __wrap_workflow_verify_recipient, recipient, "34oVnh4gNviJGMnNvgquMeLAxvXJuaRVMZ");
    expect_string(__wrap_workflow_verify_recipient, amount, "amount1");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[1], false));

    // Third output
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[2].value);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount2");
    expect_string(
        __wrap_workflow_verify_recipient, recipient, "bc1qxvenxvenxvenxvenxvenxvenxvenxven2ymjt8");
    expect_string(__wrap_workflow_verify_recipient, amount, "amount2");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[2], false));

    // Fourth output
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, outputs[3].value);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount3");
    expect_string(
        __wrap_workflow_verify_recipient,
        recipient,
        "bc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqd8sxw4");
    expect_string(__wrap_workflow_verify_recipient, amount, "amount3");
    will_return(__wrap_workflow_verify_recipient, true);
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[3], false));

    // Fifth output, change. Last output also invokes verification of total and
    // fee.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        outputs[4].keypath,
        outputs[4].keypath_count * sizeof(uint32_t));
    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[4], false));

    // Sixth output, change. Last output also invokes verification of total and fee.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        outputs[5].keypath,
        outputs[5].keypath_count * sizeof(uint32_t));
    if (mod->overflow_output_ours) {
        assert_int_equal(APP_BTC_ERR_INVALID_INPUT, app_btc_sign_output(&outputs[5], true));
        return;
    }

    expect_string(
        __wrap_workflow_confirm_blocking, params->body, "There are 2\nchange outputs.\nProceed?");
    will_return(__wrap_workflow_confirm_blocking, true);

    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, total);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount total");
    expect_value(__wrap_rust_bitcoin_util_format_amount, satoshi, fee);
    expect_string(__wrap_rust_bitcoin_util_format_amount, unit.buf, "BTC");
    will_return(__wrap_rust_bitcoin_util_format_amount, "amount fee");
    expect_string(__wrap_workflow_verify_total, total, "amount total");
    expect_string(__wrap_workflow_verify_total, fee, "amount fee");
    will_return(__wrap_workflow_verify_total, true);

    assert_int_equal(APP_BTC_OK, app_btc_sign_output(&outputs[5], true));

    // === Inputs Pass 2

    if (mod->input_sum_changes) {
        inputs[0].input.prevOutValue += inputs[1].input.prevOutValue + 1;
    }
    if (mod->input_sum_last_mismatch) {
        inputs[0].input.prevOutValue -= 1; // errors even if we decrease the amount
    }
    if (mod->overflow_input_values_pass2) {
        inputs[1].input.prevOutValue = ULLONG_MAX - inputs[0].input.prevOutValue + 1;
    }

    // First input, pass2.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[inputs[0].input.script_config_index]
            .script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[0].input.keypath,
        inputs[0].input.keypath_count * sizeof(uint32_t));

    uint8_t signature[64] = {0};
    uint8_t anti_klepto_signer_commitment[33] = {0};
    if (mod->input_sum_changes) {
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_input_pass2(
                &inputs[0].input, signature, anti_klepto_signer_commitment, false));
        return;
    }
    assert_int_equal(
        APP_BTC_OK,
        app_btc_sign_input_pass2(
            &inputs[0].input, signature, anti_klepto_signer_commitment, false));
    if (mod->check_sigs) {
        const uint8_t expected_signature[64] =
            "\xa0\xe8\xee\x3f\x59\xa0\xae\x03\xbc\x02\x38\x89\x10\xf8\x7b\x57\xbf\x69\x02"
            "\x07\xd7\x1f\x79\xd8\xec\xb0\xda\x68\x05\x94\xe2\xfd\x1a\xb3\x5a\xcf\x1e\x20"
            "\x02\x03\x81\x32\x23\xd0\x04\x8d\xb6\xc1\x1d\x0e\x03\xd5\xd5\xc4\xad\xba\x90"
            "\xd6\x33\x55\x5a\x24\x1e\xa6";
        assert_memory_equal(signature, expected_signature, sizeof(signature));
    }

    // Second input, pass2.
    expect_value(
        __wrap_btc_common_is_valid_keypath_address_simple,
        script_type,
        init_req.script_configs[0].script_config.config.simple_type);
    expect_memory(
        __wrap_btc_common_is_valid_keypath_address_simple,
        keypath,
        inputs[1].input.keypath,
        inputs[1].input.keypath_count * sizeof(uint32_t));
    if (mod->input_sum_last_mismatch || mod->overflow_input_values_pass2) {
        assert_int_equal(
            APP_BTC_ERR_INVALID_INPUT,
            app_btc_sign_input_pass2(
                &inputs[1].input, signature, anti_klepto_signer_commitment, true));
        return;
    }

    if (mod->antikepto) {
        uint8_t host_nonce[32] = {0};
        memset(host_nonce, 0xAB, sizeof(host_nonce));

        inputs[1].input.has_host_nonce_commitment = true;
        // Make host commitment from host_nonce.
        assert_true(secp256k1_ecdsa_anti_exfil_host_commit(
            wally_get_secp_context(),
            inputs[1].input.host_nonce_commitment.commitment,
            host_nonce));

        uint8_t expected_sighash[32] =
            "\x40\x73\x08\x11\xc2\xb3\xf0\xd3\x4f\x6d\xb8\x58\x22\xe5\xd7\x0f\xbc\x73\x88\xfc\x4e"
            "\xe0\x4f\x8f\xf2\x0e\x07\xdc\x58\x9b\x93\xc7";
        expect_memory(
            __wrap_keystore_antiklepto_secp256k1_commit,
            msg32,
            expected_sighash,
            sizeof(expected_sighash));

        assert_int_equal(
            APP_BTC_OK,
            app_btc_sign_input_pass2(
                &inputs[1].input, signature, anti_klepto_signer_commitment, true));

        AntiKleptoSignatureRequest antiklepto_sig_req = {0};
        memcpy(antiklepto_sig_req.host_nonce, host_nonce, sizeof(host_nonce));
        assert_int_equal(APP_BTC_OK, app_btc_sign_antiklepto(&antiklepto_sig_req, signature));

        { // Verify antiklepto nonce
            secp256k1_ecdsa_signature parsed_signature;
            assert_true(secp256k1_ecdsa_signature_parse_compact(
                wally_get_secp_context(), &parsed_signature, signature));
            uint8_t pubkey[EC_PUBLIC_KEY_UNCOMPRESSED_LEN];
            assert_true(keystore_secp256k1_pubkey_uncompressed(
                inputs[1].input.keypath, inputs[1].input.keypath_count, pubkey));
            secp256k1_pubkey parsed_pubkey;
            assert_true(secp256k1_ec_pubkey_parse(
                wally_get_secp_context(), &parsed_pubkey, pubkey, sizeof(pubkey)));
            secp256k1_ecdsa_s2c_opening opening;
            assert_true(secp256k1_ecdsa_s2c_opening_parse(
                wally_get_secp_context(), &opening, anti_klepto_signer_commitment));
            assert_true(secp256k1_anti_exfil_host_verify(
                wally_get_secp_context(),
                &parsed_signature,
                expected_sighash,
                &parsed_pubkey,
                host_nonce,
                &opening));
        }
    } else {
        assert_int_equal(
            APP_BTC_OK,
            app_btc_sign_input_pass2(
                &inputs[1].input, signature, anti_klepto_signer_commitment, true));
        if (mod->check_sigs) {
            const uint8_t expected_signature[64] =
                "\x2e\x08\x4a\x0a\x5f\x9b\xab\xb3\x5d\xf6\xec\x3a\x89\x72\x0b\xcf\xc0\x88"
                "\xd4\xba\x6a\xee\x47\x97\x3c\x55\xfe\xc3\xb3\xdd\xaa\x60\x07\xc7\xb1\x1c"
                "\x8b\x5a\x1a\x68\x20\xca\x74\xa8\x5a\xeb\x4c\xf5\x45\xc1\xb3\x37\x53\x70"
                "\xf4\x4f\x24\xd5\x3d\x61\xfe\x67\x6e\x4c";
            assert_memory_equal(signature, expected_signature, sizeof(signature));
        }
    }

    app_btc_sign_reset();
}

static const _modification_t _valid = {0};

static void _test_btc_sign(void** state)
{
    _modification_t modified = _valid;
    modified.check_sigs = true;
    _sign(&modified);
}
static void _test_input_sum_changes(void** state)
{
    _modification_t modified = _valid;
    modified.input_sum_changes = true;
    _sign(&modified);
}
static void _test_input_sum_last_mismatch(void** state)
{
    _modification_t modified = _valid;
    modified.input_sum_last_mismatch = true;
    _sign(&modified);
}
static void _test_wrong_input_value(void** state)
{
    _modification_t modified = _valid;
    modified.wrong_input_value = true;
    _sign(&modified);
}
static void _test_overflow_input_values_pass1(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_input_values_pass1 = true;
    _sign(&modified);
}
static void _test_overflow_input_values_pass2(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_input_values_pass2 = true;
    _sign(&modified);
}
static void _test_overflow_output_out(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_output_out = true;
    _sign(&modified);
}
static void _test_overflow_output_ours(void** state)
{
    _modification_t modified = _valid;
    modified.overflow_output_ours = true;
    _sign(&modified);
}
static void _test_antiklepto(void** state)
{
    _modification_t modified = _valid;
    modified.antikepto = true;
    _sign(&modified);
}
int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(_test_btc_sign),
        cmocka_unit_test(_test_input_sum_changes),
        cmocka_unit_test(_test_input_sum_last_mismatch),
        cmocka_unit_test(_test_wrong_input_value),
        cmocka_unit_test(_test_overflow_input_values_pass1),
        cmocka_unit_test(_test_overflow_input_values_pass2),
        cmocka_unit_test(_test_overflow_output_out),
        cmocka_unit_test(_test_overflow_output_ours),
        cmocka_unit_test(_test_antiklepto),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
