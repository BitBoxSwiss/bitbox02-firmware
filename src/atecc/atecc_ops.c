// SPDX-License-Identifier: Apache-2.0

#include "atecc.h"

#include <i2c_ecc.h>
#include <util.h>

// disabling some warnings, as it's an external library.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wint-conversion"
#pragma GCC diagnostic ignored "-Wpedantic"
#pragma GCC diagnostic ignored "-Wunused-parameter"
// clang-format off
#include <cryptoauthlib.h>
#include <calib/calib_basic.h>
#include <calib/calib_execution.h>
#include <host/atca_host.h>
// clang-format on
#pragma GCC diagnostic pop

// NOLINTBEGIN(bugprone-assignment-in-if-condition)

typedef struct {
    volatile int status;
    struct {
        bool active;
        ATCADevice device;
        uint32_t max_delay_count;
        uint32_t next_poll_delay_ms;
    } command;
    ATCAPacket packet;
    atca_temp_key_t io_temp_key;
    uint8_t io_other_data[4];
} atecc_async_ctx_t;

static atecc_async_ctx_t _async = {
    .status = ATCA_SUCCESS,
};

static void _atecc_async_reset_command(void)
{
    memset(&_async.command, 0, sizeof(_async.command));
    memset(&_async.packet, 0, sizeof(_async.packet));
    _async.status = ATECC_OPS_STATUS_BUSY;
}

static void _atecc_async_finish(int status)
{
    _async.command.active = false;
    _async.command.next_poll_delay_ms = 0;
    _async.status = status;
}

static ATCA_STATUS _atecc_async_init_command_device(void)
{
    _async.command.device = atcab_get_device();
    if (_async.command.device == NULL) {
        return ATCA_NOT_INITIALIZED;
    }
    return ATCA_SUCCESS;
}

static void _atecc_async_complete_command(ATCA_STATUS status)
{
    if (_async.command.device != NULL) {
        atidle(_async.command.device->mIface);
    }
    _atecc_async_finish(status);
}

// Async counterpart to the wake/send half of cryptoauthlib's calib_execute_command().
static ATCA_STATUS _atecc_async_start_command(void)
{
    // Ignored by our ATCA_CUSTOM_IFACE HAL; set only for cryptoauthlib's native I2C iface below.
    uint8_t word_address = 0xFF;
    ATCA_STATUS status;

    _async.command.device = atcab_get_device();
    if (_async.command.device == NULL) {
        return ATCA_NOT_INITIALIZED;
    }

#ifdef ATCA_NO_POLL
    if ((status = calib_get_execution_time(
             _async.packet.opcode, _async.command.device->mCommands)) != ATCA_SUCCESS) {
        return status;
    }
    _async.command.max_delay_count = 0;
    _async.command.next_poll_delay_ms = _async.command.device->mCommands->execution_time_msec;
#else
    _async.command.max_delay_count = ATCA_POLLING_MAX_TIME_MSEC / ATCA_POLLING_FREQUENCY_TIME_MSEC;
    _async.command.next_poll_delay_ms = ATCA_POLLING_INIT_TIME_MSEC;
#endif

    _async.command.active = true;

    if ((status = atwake(_async.command.device->mIface)) != ATCA_SUCCESS) {
        _atecc_async_complete_command(status);
        return status;
    }

    if (ATCA_I2C_IFACE == _async.command.device->mIface->mIfaceCFG->iface_type) {
        word_address = 0x03;
    }
    if ((status = atsend(
             _async.command.device->mIface,
             word_address,
             (uint8_t*)&_async.packet,
             _async.packet.txsize)) != ATCA_SUCCESS) {
        _atecc_async_complete_command(status);
        return status;
    }

    return ATCA_SUCCESS;
}

// Async counterpart to the receive/poll half of cryptoauthlib's calib_execute_command().
static void _atecc_async_poll_command(void)
{
    ATCA_STATUS status;
    uint16_t rxsize;

    if (_async.status != ATECC_OPS_STATUS_BUSY || !_async.command.active) {
        return;
    }

    memset(_async.packet.data, 0, sizeof(_async.packet.data));
    rxsize = sizeof(_async.packet.data);
    status = atreceive(_async.command.device->mIface, 0, _async.packet.data, &rxsize);
    if (status == ATCA_SUCCESS) {
        if (rxsize < 4) {
            status = rxsize > 0 ? ATCA_RX_FAIL : ATCA_RX_NO_RESPONSE;
        } else if ((status = atCheckCrc(_async.packet.data)) == ATCA_SUCCESS) {
            status = isATCAError(_async.packet.data);
        }
        _atecc_async_complete_command(status);
        return;
    }

#ifndef ATCA_NO_POLL
    if (_async.command.max_delay_count-- > 0) {
        _async.command.next_poll_delay_ms = ATCA_POLLING_FREQUENCY_TIME_MSEC;
        return;
    }
#endif

    _atecc_async_complete_command(status);
}

static int _atecc_async_launch(int status)
{
    if (status != ATCA_SUCCESS && _async.status == ATECC_OPS_STATUS_BUSY) {
        _atecc_async_finish(status);
    }
    return status;
}

// Ported from calib_nonce_rand() / calib_nonce_base() in cryptoauthlib/lib/calib/calib_nonce.c.
static ATCA_STATUS _atecc_async_start_nonce_rand(const uint8_t* num_in)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = NONCE_MODE_SEED_UPDATE;
    _async.packet.param2 = 0;
    memcpy(_async.packet.data, num_in, NONCE_NUMIN_SIZE);
    if ((status = atNonce(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_nonce_load() / calib_nonce_base() in cryptoauthlib/lib/calib/calib_nonce.c.
static ATCA_STATUS _atecc_async_start_nonce_load(
    uint8_t target,
    const uint8_t* num_in,
    uint16_t num_in_size)
{
    uint8_t mode = NONCE_MODE_PASSTHROUGH | (NONCE_MODE_TARGET_MASK & target);
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    if (num_in_size == 32) {
        mode |= NONCE_MODE_INPUT_LEN_32;
    } else if (num_in_size == 64) {
        mode |= NONCE_MODE_INPUT_LEN_64;
    } else {
        return ATCA_BAD_PARAM;
    }

    _async.packet.param1 = mode;
    _async.packet.param2 = 0;
    memcpy(_async.packet.data, num_in, num_in_size);
    if ((status = atNonce(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_checkmac() in cryptoauthlib/lib/calib/calib_checkmac.c.
static ATCA_STATUS _atecc_async_start_checkmac(const uint8_t* response)
{
    static const uint8_t other_data[13] = {0};
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = CHECKMAC_MODE_BLOCK2_TEMPKEY;
    _async.packet.param2 = ATECC_SLOT_AUTHKEY;
    memset(&_async.packet.data[0], 0, CHECKMAC_CLIENT_CHALLENGE_SIZE);
    memcpy(&_async.packet.data[32], response, CHECKMAC_CLIENT_RESPONSE_SIZE);
    memcpy(&_async.packet.data[64], other_data, CHECKMAC_OTHER_DATA_SIZE);
    if ((status = atCheckMAC(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_random() in cryptoauthlib/lib/calib/calib_random.c.
static ATCA_STATUS _atecc_async_start_random_command(void)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = RANDOM_SEED_UPDATE;
    _async.packet.param2 = 0;
    if ((status = atRandom(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_counter_read() / calib_counter() in cryptoauthlib/lib/calib/calib_counter.c.
static ATCA_STATUS _atecc_async_start_counter_read_command(uint16_t counter_id)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = COUNTER_MODE_READ;
    _async.packet.param2 = counter_id;
    if ((status = atCounter(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_info() / calib_info_base() in cryptoauthlib/lib/calib/calib_info.c.
static ATCA_STATUS _atecc_async_start_info_revision_command(void)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = INFO_MODE_REVISION;
    _async.packet.param2 = 0;
    if ((status = atInfo(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_kdf() in cryptoauthlib/lib/calib/calib_kdf.c.
static ATCA_STATUS _atecc_async_start_kdf_command(atecc_slot_t slot, const uint8_t* msg, size_t len)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = KDF_MODE_SOURCE_SLOT | KDF_MODE_TARGET_OUTPUT_ENC | KDF_MODE_ALG_HKDF;
    _async.packet.param2 = slot;
    _async.packet.data[0] = KDF_DETAILS_HKDF_MSG_LOC_INPUT;
    _async.packet.data[1] = 0;
    _async.packet.data[2] = 0;
    _async.packet.data[3] = (uint8_t)len;
    memcpy(&_async.packet.data[KDF_DETAILS_SIZE], msg, len);
    if ((status = atKDF(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_derivekey() in cryptoauthlib/lib/calib/calib_derivekey.c.
static ATCA_STATUS _atecc_async_start_derivekey_command(uint8_t mode, uint16_t target_key)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = mode;
    _async.packet.param2 = target_key;
    if ((status = atDeriveKey(_async.command.device->mCommands, &_async.packet, false)) !=
        ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_gendig() in cryptoauthlib/lib/calib/calib_gendig.c.
static ATCA_STATUS _atecc_async_start_gendig_command(
    uint8_t zone,
    uint16_t key_id,
    const uint8_t* other_data,
    uint8_t other_data_size)
{
    ATCA_STATUS status;
    bool is_no_mac_key = false;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = zone;
    _async.packet.param2 = key_id;
    if (_async.packet.param1 == GENDIG_ZONE_SHARED_NONCE && other_data_size >= ATCA_BLOCK_SIZE) {
        memcpy(&_async.packet.data[0], &other_data[0], ATCA_BLOCK_SIZE);
    } else if (_async.packet.param1 == GENDIG_ZONE_DATA && other_data_size >= ATCA_WORD_SIZE) {
        memcpy(&_async.packet.data[0], &other_data[0], ATCA_WORD_SIZE);
        is_no_mac_key = true;
    }
    if ((status = atGenDig(_async.command.device->mCommands, &_async.packet, is_no_mac_key)) !=
        ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_read_zone() in cryptoauthlib/lib/calib/calib_read.c.
static ATCA_STATUS _atecc_async_start_read_zone_command(
    uint8_t zone,
    uint16_t slot,
    uint8_t block,
    uint8_t offset,
    uint8_t len)
{
    ATCA_STATUS status;
    uint16_t addr;
    uint8_t addr_zone = zone & ~ATCA_ZONE_READWRITE_32;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    if ((status = calib_get_addr(addr_zone, slot, block, offset, &addr)) != ATCA_SUCCESS) {
        return status;
    }
    if (len == ATCA_BLOCK_SIZE) {
        zone |= ATCA_ZONE_READWRITE_32;
    }
    _async.packet.param1 = zone;
    _async.packet.param2 = addr;
    if ((status = atRead(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_write() in cryptoauthlib/lib/calib/calib_write.c.
static ATCA_STATUS _atecc_async_start_write_command(
    uint8_t zone,
    uint16_t address,
    const uint8_t* value,
    const uint8_t* mac)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = zone;
    _async.packet.param2 = address;
    if (zone & ATCA_ZONE_READWRITE_32) {
        memcpy(_async.packet.data, value, 32);
        if (mac != NULL) {
            memcpy(&_async.packet.data[32], mac, 32);
        }
    } else {
        memcpy(_async.packet.data, value, 4);
    }
    if ((status = atWrite(
             _async.command.device->mCommands,
             &_async.packet,
             mac && (zone & ATCA_ZONE_READWRITE_32))) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from calib_sign_base() in cryptoauthlib/lib/calib/calib_sign.c.
static ATCA_STATUS _atecc_async_start_sign_base_command(uint8_t mode, uint16_t key_id)
{
    ATCA_STATUS status;

    if ((status = _atecc_async_init_command_device()) != ATCA_SUCCESS) {
        return status;
    }
    _async.packet.param1 = mode;
    _async.packet.param2 = key_id;
    if ((status = atSign(_async.command.device->mCommands, &_async.packet)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_command();
}

// Ported from the write-command portion of calib_write_enc() in
// cryptoauthlib/lib/calib/calib_write.c.
static ATCA_STATUS _atecc_async_start_write_encrypted_data_slot_command(
    uint16_t slot,
    uint8_t block,
    const uint8_t* value,
    const uint8_t* mac)
{
    uint16_t addr;
    ATCA_STATUS status;

    if ((status = calib_get_addr(ATCA_ZONE_DATA, slot, block, 0, &addr)) != ATCA_SUCCESS) {
        return status;
    }
    return _atecc_async_start_write_command(
        ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32 | ATCA_ZONE_ENCRYPTED, addr, value, mac);
}

static int _atecc_async_extract_random_response(uint8_t* out)
{
    if (_async.packet.data[ATCA_COUNT_IDX] != RANDOM_RSP_SIZE) {
        return ATCA_RX_FAIL;
    }
    if (out != NULL) {
        memcpy(out, &_async.packet.data[ATCA_RSP_DATA_IDX], RANDOM_NUM_SIZE);
    }
    return ATCA_SUCCESS;
}

static int _atecc_async_extract_counter_response(uint32_t* counter_out)
{
    if (_async.packet.data[ATCA_COUNT_IDX] != 7) {
        return ATCA_RX_FAIL;
    }
    *counter_out = ((uint32_t)_async.packet.data[ATCA_RSP_DATA_IDX + 0] << 0) |
                   ((uint32_t)_async.packet.data[ATCA_RSP_DATA_IDX + 1] << 8) |
                   ((uint32_t)_async.packet.data[ATCA_RSP_DATA_IDX + 2] << 16) |
                   ((uint32_t)_async.packet.data[ATCA_RSP_DATA_IDX + 3] << 24);
    return ATCA_SUCCESS;
}

static int _atecc_async_extract_info_response(uint8_t* out)
{
    if (_async.packet.data[ATCA_COUNT_IDX] < 7) {
        return ATCA_RX_FAIL;
    }
    memcpy(out, &_async.packet.data[ATCA_RSP_DATA_IDX], 4);
    return ATCA_SUCCESS;
}

static int _atecc_async_extract_kdf_response(uint8_t* out_data, uint8_t* out_nonce)
{
    if (_async.packet.data[ATCA_COUNT_IDX] < (ATCA_PACKET_OVERHEAD + 64)) {
        return ATCA_RX_FAIL;
    }
    memcpy(out_data, &_async.packet.data[ATCA_RSP_DATA_IDX], 32);
    memcpy(out_nonce, &_async.packet.data[ATCA_RSP_DATA_IDX + 32], 32);
    return ATCA_SUCCESS;
}

static int _atecc_async_extract_sign_response(uint8_t* signature_out)
{
    if (_async.packet.data[ATCA_COUNT_IDX] != (ATCA_SIG_SIZE + ATCA_PACKET_OVERHEAD)) {
        return ATCA_RX_FAIL;
    }
    memcpy(signature_out, &_async.packet.data[ATCA_RSP_DATA_IDX], ATCA_SIG_SIZE);
    return ATCA_SUCCESS;
}

static int _atecc_async_extract_read_response(uint8_t* out, size_t len)
{
    if (_async.packet.data[ATCA_COUNT_IDX] < (ATCA_PACKET_OVERHEAD + len)) {
        return ATCA_RX_FAIL;
    }
    memcpy(out, &_async.packet.data[ATCA_RSP_DATA_IDX], len);
    return ATCA_SUCCESS;
}

int atecc_auth_compute_response(
    const uint8_t* num_in,
    const uint8_t* rand_out,
    const uint8_t* auth_key,
    uint8_t* response_out)
{
    static const uint8_t other_data[13] = {0};
    atca_temp_key_t temp_key = {0};
    atca_nonce_in_out_t nonce_params = {
        .mode = NONCE_MODE_SEED_UPDATE,
        .zero = 0,
        .num_in = num_in,
        .rand_out = rand_out,
        .temp_key = &temp_key,
    };
    atca_check_mac_in_out_t checkmac_params;
    ATCA_STATUS status;

    if (!atecc_serial_number_is_cached()) {
        return ATCA_NOT_INITIALIZED;
    }
    if ((status = atcah_nonce(&nonce_params)) != ATCA_SUCCESS) {
        return status;
    }

    memset(&checkmac_params, 0, sizeof(checkmac_params));
    checkmac_params.mode = CHECKMAC_MODE_BLOCK2_TEMPKEY;
    checkmac_params.key_id = ATECC_SLOT_AUTHKEY;
    checkmac_params.sn = atecc_serial_number();
    checkmac_params.client_resp = response_out;
    checkmac_params.other_data = other_data;
    checkmac_params.slot_key = auth_key;
    checkmac_params.temp_key = &temp_key;
    return atcah_check_mac(&checkmac_params);
}

int atecc_kdf_decrypt(
    const uint8_t* io_protection_key,
    const uint8_t* nonce_out,
    uint8_t* data,
    size_t data_size)
{
    atca_io_decrypt_in_out_t io_dec_params;

    if (data_size != ATCA_BLOCK_SIZE && data_size != 2 * ATCA_BLOCK_SIZE) {
        return SC_ERR_INVALID_ARGS;
    }
    memset(&io_dec_params, 0, sizeof(io_dec_params));
    io_dec_params.io_key = io_protection_key;
    io_dec_params.out_nonce = nonce_out;
    io_dec_params.data = data;
    io_dec_params.data_size = data_size;
    return atcah_io_decrypt(&io_dec_params);
}

int atecc_io_prepare_tempkey(const uint8_t* num_in, const uint8_t* rand_out)
{
    atca_nonce_in_out_t nonce_params = {
        .mode = NONCE_MODE_SEED_UPDATE,
        .zero = 0,
        .num_in = num_in,
        .rand_out = rand_out,
        .temp_key = &_async.io_temp_key,
    };
    ATCA_STATUS status;

    memset(&_async.io_temp_key, 0, sizeof(_async.io_temp_key));
    if ((status = atcah_nonce(&nonce_params)) != ATCA_SUCCESS) {
        return status;
    }

    _async.io_other_data[0] = ATCA_GENDIG;
    _async.io_other_data[1] = GENDIG_ZONE_DATA;
    _async.io_other_data[2] = (uint8_t)ATECC_SLOT_ENCRYPTION_KEY;
    _async.io_other_data[3] = (uint8_t)(ATECC_SLOT_ENCRYPTION_KEY >> 8);
    return ATCA_SUCCESS;
}

int atecc_io_apply_gendig(const uint8_t* encryption_key)
{
    atca_gen_dig_in_out_t gen_dig_param;

    if (!atecc_serial_number_is_cached()) {
        return ATCA_NOT_INITIALIZED;
    }
    memset(&gen_dig_param, 0, sizeof(gen_dig_param));
    gen_dig_param.key_id = ATECC_SLOT_ENCRYPTION_KEY;
    gen_dig_param.is_key_nomac = false;
    gen_dig_param.sn = atecc_serial_number();
    gen_dig_param.stored_value = encryption_key;
    gen_dig_param.zone = GENDIG_ZONE_DATA;
    gen_dig_param.other_data = _async.io_other_data;
    gen_dig_param.temp_key = &_async.io_temp_key;
    return atcah_gen_dig(&gen_dig_param);
}

int atecc_io_prepare_encrypted_write(
    uint16_t key_id,
    uint8_t block,
    const uint8_t* input_data,
    uint8_t* encrypted_out,
    uint8_t* mac_out)
{
    uint16_t addr;
    atca_write_mac_in_out_t write_mac_param;
    ATCA_STATUS status;

    if (!atecc_serial_number_is_cached()) {
        return ATCA_NOT_INITIALIZED;
    }
    if ((status = calib_get_addr(ATCA_ZONE_DATA, key_id, block, 0, &addr)) != ATCA_SUCCESS) {
        return status;
    }
    memset(&write_mac_param, 0, sizeof(write_mac_param));
    write_mac_param.zone = ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32 | ATCA_ZONE_ENCRYPTED;
    write_mac_param.key_id = addr;
    write_mac_param.sn = atecc_serial_number();
    write_mac_param.input_data = input_data;
    write_mac_param.encrypted_data = encrypted_out;
    write_mac_param.auth_mac = mac_out;
    write_mac_param.temp_key = &_async.io_temp_key;
    return atcah_write_auth_mac(&write_mac_param);
}

int atecc_io_decrypt_block(uint8_t* data, size_t len)
{
    if (len > ATCA_BLOCK_SIZE) {
        return SC_ERR_INVALID_ARGS;
    }
    for (size_t i = 0; i < len; i++) {
        data[i] ^= _async.io_temp_key.value[i];
    }
    return ATCA_SUCCESS;
}

int atecc_ops_get_status(void)
{
    return _async.status;
}

uint32_t atecc_ops_get_poll_delay_ms(void)
{
    return _async.command.next_poll_delay_ms;
}

void atecc_ops_poll(void)
{
    _atecc_async_poll_command();
}

int atecc_cmd_start_nonce_rand(const uint8_t* num_in)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_nonce_rand(num_in));
}

int atecc_cmd_start_checkmac(const uint8_t* response)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_checkmac(response));
}

int atecc_cmd_start_random(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_random_command());
}

int atecc_cmd_start_counter_read(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_counter_read_command(0));
}

int atecc_cmd_start_info_revision(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_info_revision_command());
}

int atecc_cmd_start_kdf(atecc_slot_t slot, const uint8_t* msg, size_t len)
{
    _atecc_async_reset_command();
    if (len > 127 || (slot != ATECC_SLOT_ROLLKEY && slot != ATECC_SLOT_KDF)) {
        return _atecc_async_launch(SC_ERR_INVALID_ARGS);
    }
    // The KDF command encodes the HKDF input length in one details byte, so len must be <=127
    // before launching the async command. This mirrors the old atcab_kdf() wrapper.
    return _atecc_async_launch(_atecc_async_start_kdf_command(slot, msg, len));
}

int atecc_cmd_start_derivekey_rollkey(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_derivekey_command(0, ATECC_SLOT_ROLLKEY));
}

int atecc_cmd_start_nonce_load_msgdigest(const uint8_t* msg)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_nonce_load(NONCE_MODE_TARGET_MSGDIGBUF, msg, 32));
}

int atecc_cmd_start_sign_attestation(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_sign_base_command(
        SIGN_MODE_EXTERNAL | SIGN_MODE_SOURCE_MSGDIGBUF, ATECC_SLOT_ATTESTATION));
}

int atecc_cmd_start_gendig_encryption_key(void)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_gendig_command(
        GENDIG_ZONE_DATA,
        ATECC_SLOT_ENCRYPTION_KEY,
        _async.io_other_data,
        sizeof(_async.io_other_data)));
}

int atecc_cmd_start_read_block(uint16_t slot, uint8_t block)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(_atecc_async_start_read_zone_command(
        ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32, slot, block, 0, ATCA_BLOCK_SIZE));
}

int atecc_cmd_start_write_encrypted_block(
    uint16_t slot,
    uint8_t block,
    const uint8_t* value,
    const uint8_t* mac)
{
    _atecc_async_reset_command();
    return _atecc_async_launch(
        _atecc_async_start_write_encrypted_data_slot_command(slot, block, value, mac));
}

int atecc_cmd_read_random_response(uint8_t* out)
{
    return _atecc_async_extract_random_response(out);
}

int atecc_cmd_read_counter_response(uint32_t* counter_out)
{
    return _atecc_async_extract_counter_response(counter_out);
}

int atecc_cmd_read_info_response(uint8_t* out)
{
    return _atecc_async_extract_info_response(out);
}

int atecc_cmd_read_kdf_response(uint8_t* out_data, uint8_t* out_nonce)
{
    return _atecc_async_extract_kdf_response(out_data, out_nonce);
}

int atecc_cmd_read_sign_response(uint8_t* signature_out)
{
    return _atecc_async_extract_sign_response(signature_out);
}

int atecc_cmd_read_block_response(uint8_t* out)
{
    return _atecc_async_extract_read_response(out, ATCA_BLOCK_SIZE);
}

// NOLINTEND(bugprone-assignment-in-if-condition)
