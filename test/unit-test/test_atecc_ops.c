// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include <atecc/atecc.h>
#include <util.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpedantic"
// clang-format off
#include <cryptoauthlib.h>
#include <host/atca_host.h>
// clang-format on
#pragma GCC diagnostic pop

#define MAX_PACKET_SIZE 192
#define MAX_RECEIVE_RESULTS 4

typedef struct {
    ATCA_STATUS status;
    uint8_t response[MAX_PACKET_SIZE];
    uint16_t response_len;
} receive_result_t;

static struct atca_command _command = {
    .dt = ATECC608,
};
static ATCAIfaceCfg _iface_config = {
    .iface_type = ATCA_CUSTOM_IFACE,
    .devtype = ATECC608,
};
static struct atca_iface _iface = {
    .mIfaceCFG = &_iface_config,
};
static struct atca_device _device = {
    .mCommands = &_command,
    .mIface = &_iface,
};

static ATCA_STATUS _wake_status;
static ATCA_STATUS _send_status;
static size_t _wake_calls;
static size_t _send_calls;
static size_t _receive_calls;
static size_t _idle_calls;
static uint8_t _sent_packet[MAX_PACKET_SIZE];
static size_t _sent_packet_len;
static receive_result_t _receive_results[MAX_RECEIVE_RESULTS];
static size_t _receive_result_count;

static void _crc(const uint8_t* data, size_t len, uint8_t* crc_out)
{
    uint16_t crc = 0;
    for (size_t i = 0; i < len; i++) {
        for (uint8_t mask = 1; mask != 0; mask <<= 1) {
            uint8_t data_bit = (data[i] & mask) != 0;
            uint8_t crc_bit = crc >> 15;
            crc <<= 1;
            if (data_bit != crc_bit) {
                crc ^= 0x8005;
            }
        }
    }
    crc_out[0] = (uint8_t)crc;
    crc_out[1] = (uint8_t)(crc >> 8);
}

static void _make_response(const uint8_t* data, size_t data_len, receive_result_t* result)
{
    result->status = ATCA_SUCCESS;
    result->response_len = (uint16_t)(data_len + ATCA_PACKET_OVERHEAD);
    result->response[0] = (uint8_t)result->response_len;
    memcpy(&result->response[ATCA_RSP_DATA_IDX], data, data_len);
    _crc(
        result->response,
        result->response_len - ATCA_CRC_SIZE,
        &result->response[result->response_len - ATCA_CRC_SIZE]);
}

static void _queue_receive_error(ATCA_STATUS status)
{
    assert_true(_receive_result_count < MAX_RECEIVE_RESULTS);
    _receive_results[_receive_result_count++].status = status;
}

static void _queue_response(const uint8_t* data, size_t data_len)
{
    assert_true(_receive_result_count < MAX_RECEIVE_RESULTS);
    _make_response(data, data_len, &_receive_results[_receive_result_count++]);
}

static void _reset_transport(void)
{
    _wake_status = ATCA_SUCCESS;
    _send_status = ATCA_SUCCESS;
    _wake_calls = 0;
    _send_calls = 0;
    _receive_calls = 0;
    _idle_calls = 0;
    _sent_packet_len = 0;
    _receive_result_count = 0;
    memset(_sent_packet, 0, sizeof(_sent_packet));
    memset(_receive_results, 0, sizeof(_receive_results));
}

ATCADevice atcab_get_device(void)
{
    return &_device;
}

ATCA_STATUS atwake(ATCAIface iface)
{
    assert_ptr_equal(iface, &_iface);
    _wake_calls++;
    return _wake_status;
}

ATCA_STATUS atsend(ATCAIface iface, uint8_t word_address, uint8_t* txdata, int txlength)
{
    assert_ptr_equal(iface, &_iface);
    assert_int_equal(word_address, 0xff);
    assert_int_equal(txlength, txdata[1]);
    assert_true((size_t)txlength <= sizeof(_sent_packet));
    _send_calls++;
    _sent_packet_len = (size_t)txlength;
    memcpy(_sent_packet, &txdata[1], _sent_packet_len);
    return _send_status;
}

ATCA_STATUS atreceive(ATCAIface iface, uint8_t word_address, uint8_t* rxdata, uint16_t* rxlength)
{
    assert_ptr_equal(iface, &_iface);
    assert_int_equal(word_address, 0);
    assert_true(_receive_calls < _receive_result_count);
    const receive_result_t* result = &_receive_results[_receive_calls++];
    if (result->status != ATCA_SUCCESS) {
        return result->status;
    }
    assert_true(*rxlength >= result->response_len);
    memcpy(rxdata, result->response, result->response_len);
    *rxlength = result->response_len;
    return ATCA_SUCCESS;
}

ATCA_STATUS atidle(ATCAIface iface)
{
    assert_ptr_equal(iface, &_iface);
    _idle_calls++;
    return ATCA_SUCCESS;
}

ATCA_STATUS atca_trace(ATCA_STATUS status)
{
    return status;
}

ATCA_STATUS atcah_nonce(struct atca_nonce_in_out* param)
{
    (void)param;
    return ATCA_SUCCESS;
}

static void _assert_sent_packet(
    uint8_t opcode,
    uint8_t param1,
    uint16_t param2,
    const uint8_t* data,
    size_t data_len)
{
    uint8_t expected[MAX_PACKET_SIZE] = {0};
    size_t expected_len = ATCA_CMD_SIZE_MIN + data_len;
    assert_true(expected_len <= sizeof(expected));
    expected[ATCA_COUNT_IDX] = (uint8_t)expected_len;
    expected[ATCA_OPCODE_IDX] = opcode;
    expected[ATCA_PARAM1_IDX] = param1;
    expected[ATCA_PARAM2_IDX] = (uint8_t)param2;
    expected[ATCA_PARAM2_IDX + 1] = (uint8_t)(param2 >> 8);
    if (data_len != 0) {
        memcpy(&expected[ATCA_DATA_IDX], data, data_len);
    }
    _crc(expected, expected_len - ATCA_CRC_SIZE, &expected[expected_len - ATCA_CRC_SIZE]);

    assert_int_equal(atecc_ops_get_status(), ATECC_OPS_STATUS_BUSY);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), ATCA_POLLING_INIT_TIME_MSEC);
    assert_int_equal(_wake_calls, 1);
    assert_int_equal(_send_calls, 1);
    assert_int_equal(_sent_packet_len, expected_len);
    assert_memory_equal(_sent_packet, expected, expected_len);

    const uint8_t success = 0;
    _queue_response(&success, sizeof(success));
    atecc_ops_poll();
    assert_int_equal(atecc_ops_get_status(), ATCA_SUCCESS);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), 0);
    assert_int_equal(_idle_calls, 1);
}

static void _fill_sequence(uint8_t* out, size_t len, uint8_t start)
{
    for (size_t i = 0; i < len; i++) {
        out[i] = start + (uint8_t)i;
    }
}

/** Verifies every asynchronous start function emits the same complete ATECC wire packet. */
static void test_atecc_ops_command_packets(void** state)
{
    (void)state;
    uint8_t data
        [CHECKMAC_CLIENT_CHALLENGE_SIZE + CHECKMAC_CLIENT_RESPONSE_SIZE +
         CHECKMAC_OTHER_DATA_SIZE] = {0};
    uint8_t input[64];
    uint8_t mac[WRITE_MAC_SIZE];
    _fill_sequence(input, sizeof(input), 0x10);
    _fill_sequence(mac, sizeof(mac), 0xa0);

    _reset_transport();
    assert_int_equal(atecc_cmd_start_nonce_rand(input), ATCA_SUCCESS);
    _assert_sent_packet(ATCA_NONCE, NONCE_MODE_SEED_UPDATE, 0, input, NONCE_NUMIN_SIZE);

    _reset_transport();
    memcpy(&data[CHECKMAC_CLIENT_CHALLENGE_SIZE], input, CHECKMAC_CLIENT_RESPONSE_SIZE);
    assert_int_equal(atecc_cmd_start_checkmac(input), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_CHECKMAC, CHECKMAC_MODE_BLOCK2_TEMPKEY, ATECC_SLOT_AUTHKEY, data, sizeof(data));

    _reset_transport();
    assert_int_equal(atecc_cmd_start_random(), ATCA_SUCCESS);
    _assert_sent_packet(ATCA_RANDOM, RANDOM_SEED_UPDATE, 0, NULL, 0);

    _reset_transport();
    assert_int_equal(atecc_cmd_start_counter_read(), ATCA_SUCCESS);
    _assert_sent_packet(ATCA_COUNTER, COUNTER_MODE_READ, 0, NULL, 0);

    _reset_transport();
    assert_int_equal(atecc_cmd_start_info_revision(), ATCA_SUCCESS);
    _assert_sent_packet(ATCA_INFO, INFO_MODE_REVISION, 0, NULL, 0);

    _reset_transport();
    const size_t kdf_input_len = 17;
    uint8_t kdf_data[KDF_DETAILS_SIZE + kdf_input_len];
    memset(kdf_data, 0, sizeof(kdf_data));
    kdf_data[0] = KDF_DETAILS_HKDF_MSG_LOC_INPUT;
    kdf_data[3] = kdf_input_len;
    memcpy(&kdf_data[KDF_DETAILS_SIZE], input, kdf_input_len);
    assert_int_equal(atecc_cmd_start_kdf(ATECC_SLOT_KDF, input, kdf_input_len), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_KDF,
        KDF_MODE_SOURCE_SLOT | KDF_MODE_TARGET_OUTPUT_ENC | KDF_MODE_ALG_HKDF,
        ATECC_SLOT_KDF,
        kdf_data,
        sizeof(kdf_data));

    _reset_transport();
    assert_int_equal(atecc_cmd_start_derivekey_rollkey(), ATCA_SUCCESS);
    _assert_sent_packet(ATCA_DERIVE_KEY, 0, ATECC_SLOT_ROLLKEY, NULL, 0);

    _reset_transport();
    assert_int_equal(atecc_cmd_start_nonce_load_msgdigest(input), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_NONCE,
        NONCE_MODE_PASSTHROUGH | NONCE_MODE_TARGET_MSGDIGBUF,
        0,
        input,
        NONCE_NUMIN_SIZE_PASSTHROUGH);

    _reset_transport();
    assert_int_equal(atecc_cmd_start_sign_attestation(), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_SIGN,
        SIGN_MODE_EXTERNAL | SIGN_MODE_SOURCE_MSGDIGBUF,
        ATECC_SLOT_ATTESTATION,
        NULL,
        0);

    _reset_transport();
    assert_int_equal(atecc_io_prepare_tempkey(input, &input[NONCE_NUMIN_SIZE]), ATCA_SUCCESS);
    assert_int_equal(atecc_cmd_start_gendig_encryption_key(), ATCA_SUCCESS);
    const uint8_t gendig_data[] = {
        ATCA_GENDIG,
        GENDIG_ZONE_DATA,
        ATECC_SLOT_ENCRYPTION_KEY,
        0,
    };
    _assert_sent_packet(
        ATCA_GENDIG, GENDIG_ZONE_DATA, ATECC_SLOT_ENCRYPTION_KEY, gendig_data, sizeof(gendig_data));

    _reset_transport();
    assert_int_equal(atecc_cmd_start_read_block(ATECC_SLOT_DATA0, 2), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_READ,
        ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32,
        (2 << 8) | (ATECC_SLOT_DATA0 << 3),
        NULL,
        0);

    _reset_transport();
    uint8_t write_data[ATCA_BLOCK_SIZE + WRITE_MAC_SIZE];
    memcpy(write_data, input, ATCA_BLOCK_SIZE);
    memcpy(&write_data[ATCA_BLOCK_SIZE], mac, WRITE_MAC_SIZE);
    assert_int_equal(
        atecc_cmd_start_write_encrypted_block(ATECC_SLOT_DATA0, 2, input, mac), ATCA_SUCCESS);
    _assert_sent_packet(
        ATCA_WRITE,
        ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32 | ATCA_ZONE_ENCRYPTED,
        (2 << 8) | (ATECC_SLOT_DATA0 << 3),
        write_data,
        sizeof(write_data));
}

/** Verifies the C state needed to complete a delayed command after Rust detaches its future. */
static void test_atecc_ops_delayed_response_for_detached_recovery(void** state)
{
    (void)state;
    uint8_t expected_random[RANDOM_NUM_SIZE];
    uint8_t random_out[RANDOM_NUM_SIZE] = {0};
    _fill_sequence(expected_random, sizeof(expected_random), 0x40);

    _reset_transport();
    _queue_receive_error(ATCA_RX_NO_RESPONSE);
    _queue_response(expected_random, sizeof(expected_random));
    assert_int_equal(atecc_cmd_start_random(), ATCA_SUCCESS);

    atecc_ops_poll();
    assert_int_equal(atecc_ops_get_status(), ATECC_OPS_STATUS_BUSY);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), ATCA_POLLING_FREQUENCY_TIME_MSEC);
    assert_int_equal(_idle_calls, 0);

    // Rust keeps polling after cancellation; C must retain the command until a poll succeeds.
    assert_int_equal(atecc_ops_get_status(), ATECC_OPS_STATUS_BUSY);
    atecc_ops_poll();
    assert_int_equal(atecc_ops_get_status(), ATCA_SUCCESS);
    assert_int_equal(_idle_calls, 1);
    assert_int_equal(atecc_cmd_read_random_response(random_out), ATCA_SUCCESS);
    assert_memory_equal(random_out, expected_random, sizeof(random_out));
}

/** Verifies a valid device error frame becomes the operation result and idles the chip. */
static void test_atecc_ops_device_error(void** state)
{
    (void)state;
    const uint8_t execution_error = 0x0f;
    _reset_transport();
    _queue_response(&execution_error, sizeof(execution_error));

    assert_int_equal(atecc_cmd_start_random(), ATCA_SUCCESS);
    atecc_ops_poll();

    assert_int_equal(atecc_ops_get_status(), ATCA_EXECUTION_ERROR);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), 0);
    assert_int_equal(_idle_calls, 1);
}

/** Verifies wake and send failures finish immediately and leave the next command usable. */
static void test_atecc_ops_start_errors(void** state)
{
    (void)state;
    _reset_transport();
    _wake_status = ATCA_COMM_FAIL;
    assert_int_equal(atecc_cmd_start_random(), ATCA_COMM_FAIL);
    assert_int_equal(atecc_ops_get_status(), ATCA_COMM_FAIL);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), 0);
    assert_int_equal(_send_calls, 0);
    assert_int_equal(_idle_calls, 1);

    _reset_transport();
    _send_status = ATCA_COMM_FAIL;
    assert_int_equal(atecc_cmd_start_random(), ATCA_COMM_FAIL);
    assert_int_equal(atecc_ops_get_status(), ATCA_COMM_FAIL);
    assert_int_equal(atecc_ops_get_poll_delay_ms(), 0);
    assert_int_equal(_send_calls, 1);
    assert_int_equal(_idle_calls, 1);

    _reset_transport();
    const uint8_t success = 0;
    _queue_response(&success, sizeof(success));
    assert_int_equal(atecc_cmd_start_random(), ATCA_SUCCESS);
    atecc_ops_poll();
    assert_int_equal(atecc_ops_get_status(), ATCA_SUCCESS);
    assert_int_equal(_idle_calls, 1);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_atecc_ops_command_packets),
        cmocka_unit_test(test_atecc_ops_delayed_response_for_detached_recovery),
        cmocka_unit_test(test_atecc_ops_device_error),
        cmocka_unit_test(test_atecc_ops_start_errors),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
