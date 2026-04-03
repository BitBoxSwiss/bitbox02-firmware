// SPDX-License-Identifier: Apache-2.0

#include "atecc.h"
#include "command.h"
#include "delay.h"
#include "hardfault.h"
#include "securechip/securechip.h"
#include <i2c_ecc.h>
#include <rust/rust.h>
#include <util.h>

// disabling some warnings, as it's an external library.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wint-conversion"
#pragma GCC diagnostic ignored "-Wpedantic"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <cryptoauthlib.h>
#include <host/atca_host.h>
#pragma GCC diagnostic pop

typedef enum {
    ATECC_SLOT_IO_PROTECTION_KEY = 0,
    ATECC_SLOT_AUTHKEY = 1,
    ATECC_SLOT_ENCRYPTION_KEY = 2,
    ATECC_SLOT_ROLLKEY = 3,
    ATECC_SLOT_KDF = 4,
    ATECC_SLOT_ATTESTATION = 5,
    // Deprecated as the equivalent does not exist in the Optiga chip.
    ATECC_SLOT_ECC_UNSAFE_SIGN_DEPRECATED = 6,
    ATECC_SLOT_DATA0 = 9,
    // The other slots are currently not in use.
} atecc_slot_t;

// Chip Configuration, generated with "make generate-atecc608-config"
// The first 16 bytes, as well as the LockValue/LockConfig can't be changed and are ignored when
// writing the configuration to the device. Locking is performed via the Lock command during setup,
// after writing the configuration.
// UserExtra and UserExtraAdd are setup automatically via the UpdateExtra command based on this
// configuration.
// The Counter0/Counter1 values are overwritten at setup via atcab_write_config_counter().
// Individual slot locking is performed at setup via atcab_lock_data_slot().
#if (ATCA_ECC_CONFIG_SIZE != 128)
    #error "Unexpected configuration size"
#endif
// clang-format off
static uint8_t _configuration[ATCA_ECC_CONFIG_SIZE] = {
    0x01, 0x23, 0x68, 0xee, 0x00, 0x00, 0x60, 0x02,
    0x8a, 0x1d, 0xde, 0x66, 0xee, 0x01, 0x01, 0x00,
    0xc0, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0xa0, 0x20, 0x80, 0x42, 0x83, 0x20,
    0x83, 0x62, 0x83, 0x20, 0xc2, 0x42, 0xc2, 0x42,
    0xc2, 0x42, 0xc2, 0x42, 0xc2, 0x42, 0xc2, 0x42,
    0xc2, 0x42, 0xc2, 0x42, 0x01, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xff, 0xff, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00,
    0x7c, 0x00, 0x7c, 0x00, 0xfc, 0x01, 0xdc, 0x01,
    0xdc, 0x01, 0xf3, 0x01, 0xd3, 0x01, 0xd3, 0x01,
    0xdc, 0x01, 0xdc, 0x01, 0xdc, 0x01, 0xdc, 0x01,
    0xdc, 0x01, 0xdc, 0x01, 0xdc, 0x01, 0xdc, 0x01,
};
// clang-format on

// Number of times the first kdf slot can be used.
#define MONOTONIC_COUNTER_MAX_USE (730500)

// This number of KDF iterations on the 2nd kdf slot when stretching the device
// password.
#define KDF_NUM_ITERATIONS (2)

// The total individual size of the public key data slots (slots 9-15) is 72 bytes. Using encrypted
// read/write it is only possible to transmit 32 bytes. The last block is therefore 8 (72 =
// 32+32+8).
#define DATA_PUBLIC_KEY_SLOT_BLOCK_SIZE 32

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"
typedef union {
    struct __attribute__((__packed__)) {
        uint32_t u2f_counter;
    } fields;
    uint8_t bytes[DATA_PUBLIC_KEY_SLOT_BLOCK_SIZE];
} data_9_0_t;

#pragma GCC diagnostic pop

static const securechip_interface_functions_t* _interface_functions = NULL;

/** \brief initialize an I2C interface using given config.
 * \param[in] hal - opaque ptr to HAL data
 * \param[in] cfg - interface configuration
 * \return ATCA_SUCCESS.
 */
static ATCA_STATUS _init(void* hal, void* cfg)
{
    (void)hal;
    (void)cfg;
    // store opaque data in ((ATCAHAL_t*)hal)->hal_data
    return ATCA_SUCCESS;
}

/** \brief HAL implementation of I2C post init
 * \param[in] iface  instance
 * \return ATCA_SUCCESS
 */
static ATCA_STATUS _post_init(void* iface)
{
    (void)iface;
    return ATCA_SUCCESS;
}

/** \brief HAL implementation of I2C receive function for ASF I2C
 * \param[in]    iface     Device to interact with.
 * \param[out]   rxdata    Data received will be returned here.
 * \param[inout] rxlength  As input, the size of the rxdata buffer.
 *                         As output, the number of bytes received.
 * \return ATCA_SUCCESS on success, otherwise an error code.
 */
static ATCA_STATUS _receive(void* iface, uint8_t word_address, uint8_t* rxdata, uint16_t* rxlength)
{
    (void)iface;
    (void)word_address;
    uint8_t ret = i2c_ecc_read(rxdata, *rxlength);
    if (ret) {
        return ATCA_COMM_FAIL;
    }
    *rxlength = rxdata[0];
    return ATCA_SUCCESS;
}

/** \brief HAL implementation of I2C send over ASF
 * \param[in] iface     instance
 * \param[in] txdata    pointer to space to bytes to send
 * \param[in] txlength  number of bytes to send
 * \return ATCA_SUCCESS on success, otherwise an error code.
 */
static ATCA_STATUS _send(void* iface, uint8_t word_address, uint8_t* txdata, int txlength)
{
    (void)iface;
    (void)word_address;
    // txdata[0] is using _reserved byte of the ATCAPacket
    txdata[0] = I2C_ECC_CHIP_CMD;
    // Account for the _reserved byte, similar to
    // https://github.com/MicrochipTech/cryptoauthlib/blob/411cd5cfc314a875794a8fb5a4b6a1860384ec1b/lib/hal/hal_i2c_start.c#L312
    txlength++;
    return i2c_ecc_write(txdata, txlength) ? ATCA_COMM_FAIL : ATCA_SUCCESS;
}

/** \brief sleep CryptoAuth device using I2C bus
 * \param[in] iface  interface to logical device to sleep
 * \return ATCA_SUCCESS on success, otherwise an error code.
 */
static ATCA_STATUS _sleep(void* iface)
{
    (void)iface;
    return i2c_ecc_sleep() ? ATCA_COMM_FAIL : ATCA_SUCCESS;
}

/** \brief wake up CryptoAuth device using I2C bus
 * \param[in] iface  interface to logical device to wakeup
 * \return ATCA_SUCCESS on success, otherwise an error code.

 */
static ATCA_STATUS _wake(void* iface)
{
    (void)iface;
    return i2c_ecc_wake() == I2C_ECC_WAKE ? ATCA_SUCCESS : ATCA_COMM_FAIL;
}

/** \brief idle CryptoAuth device using I2C bus
 * \param[in] iface  interface to logical device to idle
 * \return ATCA_SUCCESS on success, otherwise an error code.
 */
static ATCA_STATUS _idle(void* iface)
{
    (void)iface;
    return i2c_ecc_idle() ? ATCA_COMM_FAIL : ATCA_SUCCESS;
}

/** \brief cleanup resources created in _init().
 * \param[in] hal_data - opaque pointer to hal data structure.
 * \return ATCA_SUCCESS
 */
static ATCA_STATUS _release(void* hal_data)
{
    (void)hal_data;
    return ATCA_SUCCESS;
}

// set up custom i2c communication interface with cryptoauthlib.
static ATCAIfaceCfg cfg = {
    // TODO: can likely use cryptoauthlib/lib/hal/hal_i2c_start.(c|h) for all or
    // some of the functionality, possibly using cfg_ateccx08a_i2c_default
    .iface_type = ATCA_CUSTOM_IFACE,
    .devtype = ATECC608,
    .atcacustom.halinit = &_init,
    .atcacustom.halpostinit = &_post_init,
    .atcacustom.halreceive = &_receive,
    .atcacustom.halsend = &_send,
    .atcacustom.halsleep = &_sleep,
    .atcacustom.halwake = &_wake,
    .atcacustom.halidle = &_idle,
    .atcacustom.halrelease = &_release,
    .wake_delay = I2C_ECC_TWHI,
    .rx_retries = I2C_ECC_RETRIES,
    .cfg_data = NULL};

#if FACTORYSETUP == 1
/**
 * Individually locks a slot. Used to lock the io protection and auth key so
 * they can never change.
 */
static ATCA_STATUS _lock_slot(atecc_slot_t slot)
{
    bool is_locked = false;
    ATCA_STATUS result = atcab_is_slot_locked(slot, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return atcab_lock_data_slot(slot);
    }
    return ATCA_SUCCESS;
}

static ATCA_STATUS _factory_setup(void)
{
    if (_interface_functions == NULL) {
        return (ATCA_STATUS)SC_ERR_IFS;
    }
    bool is_config_locked = false;
    ATCA_STATUS result = atcab_is_locked(LOCK_ZONE_CONFIG, &is_config_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_config_locked) {
        // The chip is unlocked: configure the chip and lock it. This happens
        // only once.

        result = atcab_write_config_zone(_configuration);
        if (result != ATCA_SUCCESS) {
            return result;
        }

        // Set Counter0 so that it can be incremented
        // `MONOTONIC_COUNTER_MAX_USE` times. This counter is attached to slots
        // with limited use.
        result = atcab_write_config_counter(0, COUNTER_MAX_VALUE - MONOTONIC_COUNTER_MAX_USE);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        // Set Counter1 to 0.
        result = atcab_write_config_counter(1, 0);
        if (result != ATCA_SUCCESS) {
            return result;
        }

        result = atcab_lock_config_zone();
        if (result != ATCA_SUCCESS) {
            return result;
        }
        is_config_locked = true;
    }
    bool is_data_locked;
    result = atcab_is_locked(LOCK_ZONE_DATA, &is_data_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (is_config_locked && !is_data_locked) {
        // Write IO protection key.
        uint8_t io_protection_key[32] = {0};
        UTIL_CLEANUP_32(io_protection_key);
        _interface_functions->get_io_protection_key(io_protection_key);
        result = atcab_write_zone(
            ATCA_ZONE_DATA, ATECC_SLOT_IO_PROTECTION_KEY, 0, 0, io_protection_key, 32);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        // Write auth key.
        uint8_t auth_key[32] = {0};
        UTIL_CLEANUP_32(auth_key);
        _interface_functions->get_auth_key(auth_key);
        result = atcab_write_zone(ATCA_ZONE_DATA, ATECC_SLOT_AUTHKEY, 0, 0, auth_key, 32);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        // Write encryption key.
        uint8_t encryption_key[32] = {0};
        UTIL_CLEANUP_32(encryption_key);
        _interface_functions->get_encryption_key(encryption_key);
        result =
            atcab_write_zone(ATCA_ZONE_DATA, ATECC_SLOT_ENCRYPTION_KEY, 0, 0, encryption_key, 32);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        result = atcab_lock_data_zone();
        if (result != ATCA_SUCCESS) {
            return result;
        }
        is_data_locked = true;
    }
    if (is_config_locked && is_data_locked) {
        result = _lock_slot(ATECC_SLOT_IO_PROTECTION_KEY);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        result = _lock_slot(ATECC_SLOT_AUTHKEY);
        if (result != ATCA_SUCCESS) {
            return result;
        }
        result = _lock_slot(ATECC_SLOT_ENCRYPTION_KEY);
        if (result != ATCA_SUCCESS) {
            return result;
        }
    }
    return ATCA_SUCCESS;
}
#endif

static int _verify_config(void)
{
    bool is_locked;
    ATCA_STATUS result;

    // Check that the config and data zones are locked.
    result = atcab_is_locked(LOCK_ZONE_CONFIG, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return SC_ATECC_ERR_ZONE_UNLOCKED_CONFIG;
    }
    result = atcab_is_locked(LOCK_ZONE_DATA, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return SC_ATECC_ERR_ZONE_UNLOCKED_DATA;
    }

    bool same_config = false;
    result = atcab_cmp_config_zone(_configuration, &same_config);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!same_config) {
        return SC_ERR_CONFIG_MISMATCH;
    }

    // Check that the slots are individually locked.
    result = atcab_is_slot_locked(ATECC_SLOT_IO_PROTECTION_KEY, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return SC_ATECC_ERR_SLOT_UNLOCKED_IO;
    }
    result = atcab_is_slot_locked(ATECC_SLOT_AUTHKEY, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return SC_ATECC_ERR_SLOT_UNLOCKED_AUTH;
    }
    result = atcab_is_slot_locked(ATECC_SLOT_ENCRYPTION_KEY, &is_locked);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    if (!is_locked) {
        return SC_ATECC_ERR_SLOT_UNLOCKED_ENC;
    }
    return ATCA_SUCCESS;
}

int atecc_setup(const securechip_interface_functions_t* ifs)
{
    if (ifs == NULL) {
        return SC_ERR_IFS;
    }
    _interface_functions = ifs;
    ATCA_STATUS result = atcab_init(&cfg);
    if (result != ATCA_SUCCESS) {
        return result;
    }

#if FACTORYSETUP == 1
    result = _factory_setup();
    if (result != ATCA_SUCCESS) {
        return result;
    }
#endif

    return _verify_config();
}

typedef enum {
    _AUTHORIZE_STATE_IDLE = 0,
    _AUTHORIZE_STATE_NONCE,
    _AUTHORIZE_STATE_READ_SERIAL,
    _AUTHORIZE_STATE_CHECKMAC,
} _authorize_state_t;

typedef struct {
    _authorize_state_t state;
    atecc_command_ctx_t command;
    ATCAPacket packet;
    uint8_t num_in[NONCE_NUMIN_SIZE];
    uint8_t rand_out[32];
    uint8_t serial_block[ATCA_BLOCK_SIZE];
    atca_temp_key_t temp_key;
} _authorize_ctx_t;

typedef enum {
    _SLOT_KDF_STATE_IDLE = 0,
    _SLOT_KDF_STATE_AUTHORIZE,
    _SLOT_KDF_STATE_KDF,
} _slot_kdf_state_t;

typedef struct {
    _slot_kdf_state_t state;
    atecc_slot_t slot;
    const uint8_t* msg;
    size_t len;
    uint8_t* out;
    uint8_t nonce_out[32];
    _authorize_ctx_t authorize;
    atecc_command_ctx_t command;
    ATCAPacket packet;
} _slot_kdf_ctx_t;

typedef enum {
    _ROLLKEY_STATE_IDLE = 0,
    _ROLLKEY_STATE_AUTHORIZE,
    _ROLLKEY_STATE_NONCE,
    _ROLLKEY_STATE_DERIVEKEY,
} _rollkey_state_t;

typedef struct {
    _rollkey_state_t state;
    _authorize_ctx_t authorize;
    atecc_command_ctx_t command;
    ATCAPacket packet;
    uint8_t num_in[NONCE_NUMIN_SIZE];
} _rollkey_ctx_t;

typedef enum {
    _UPDATE_KDF_KEY_STATE_IDLE = 0,
    _UPDATE_KDF_KEY_STATE_AUTHORIZE,
    _UPDATE_KDF_KEY_STATE_READ_CONFIG,
    _UPDATE_KDF_KEY_STATE_NONCE,
    _UPDATE_KDF_KEY_STATE_GENDIG,
    _UPDATE_KDF_KEY_STATE_WRITE,
} _update_kdf_key_state_t;

typedef struct {
    _update_kdf_key_state_t state;
    _authorize_ctx_t authorize;
    atecc_command_ctx_t command;
    ATCAPacket packet;
    uint8_t read_buf[ATCA_BLOCK_SIZE];
    uint8_t serial_num[ATCA_BLOCK_SIZE];
    uint8_t new_key[32];
    uint8_t encryption_key[32];
    uint8_t nonce_contribution[32];
    uint8_t rand_out[RANDOM_NUM_SIZE];
    uint8_t other_data[4];
    uint8_t cipher_text[ATCA_KEY_SIZE];
    uint8_t mac[WRITE_MAC_SIZE];
    atca_temp_key_t temp_key;
    uint16_t addr;
} _update_kdf_key_ctx_t;

typedef enum {
    _RESET_KEYS_STATE_IDLE = 0,
    _RESET_KEYS_STATE_ROLLKEY,
    _RESET_KEYS_STATE_UPDATE_KDF_KEY,
} _reset_keys_state_t;

typedef struct {
    _reset_keys_state_t state;
    _rollkey_ctx_t rollkey;
    _update_kdf_key_ctx_t update_kdf_key;
} _reset_keys_ctx_t;

typedef enum {
    _STRETCH_PASSWORD_STATE_IDLE = 0,
    _STRETCH_PASSWORD_STATE_ROLLKEY_KDF,
    _STRETCH_PASSWORD_STATE_KDF,
} _stretch_password_state_t;

typedef struct {
    _stretch_password_state_t state;
    const char* password;
    securechip_password_stretch_algo_t password_stretch_algo;
    uint8_t* stretched_out;
    uint8_t password_salted_hashed[32];
    uint8_t kdf_in[32];
    size_t iteration;
    _slot_kdf_ctx_t slot_kdf;
} _stretch_password_ctx_t;

typedef enum {
    _INIT_NEW_PASSWORD_STATE_IDLE = 0,
    _INIT_NEW_PASSWORD_STATE_RESET_KEYS,
    _INIT_NEW_PASSWORD_STATE_STRETCH_PASSWORD,
} _init_new_password_state_t;

typedef struct {
    _init_new_password_state_t state;
    const char* password;
    securechip_password_stretch_algo_t password_stretch_algo;
    uint8_t* stretched_out;
    _reset_keys_ctx_t reset_keys;
    _stretch_password_ctx_t stretch_password;
} _init_new_password_ctx_t;

_Static_assert(sizeof(_slot_kdf_ctx_t) <= ATECC_KDF_ASYNC_CONTEXT_SIZE, "kdf async ctx too small");
_Static_assert(
    sizeof(_reset_keys_ctx_t) <= ATECC_RESET_KEYS_ASYNC_CONTEXT_SIZE,
    "reset keys async ctx too small");
_Static_assert(
    sizeof(_stretch_password_ctx_t) <= ATECC_STRETCH_PASSWORD_ASYNC_CONTEXT_SIZE,
    "stretch password async ctx too small");
_Static_assert(
    sizeof(_init_new_password_ctx_t) <= ATECC_INIT_NEW_PASSWORD_ASYNC_CONTEXT_SIZE,
    "init new password async ctx too small");

#define _KDF_ASYNC_CTX(ctx) ((_slot_kdf_ctx_t*)(void*)(ctx))
#define _RESET_KEYS_ASYNC_CTX(ctx) ((_reset_keys_ctx_t*)(void*)(ctx))
#define _STRETCH_PASSWORD_ASYNC_CTX(ctx) ((_stretch_password_ctx_t*)(void*)(ctx))
#define _INIT_NEW_PASSWORD_ASYNC_CTX(ctx) ((_init_new_password_ctx_t*)(void*)(ctx))

static void _authorize_async_abort(_authorize_ctx_t* ctx);
static ATCA_STATUS _authorize_async_start(_authorize_ctx_t* ctx, uint16_t* wait_ms_out);
static ATCA_STATUS _authorize_async_poll(_authorize_ctx_t* ctx, uint16_t* wait_ms_out);
static void _rollkey_async_abort(_rollkey_ctx_t* ctx);
static ATCA_STATUS _rollkey_async_start(_rollkey_ctx_t* ctx, uint16_t* wait_ms_out);
static ATCA_STATUS _rollkey_async_poll(_rollkey_ctx_t* ctx, uint16_t* wait_ms_out);
static void _update_kdf_key_async_abort(_update_kdf_key_ctx_t* ctx);
static ATCA_STATUS _update_kdf_key_async_start(_update_kdf_key_ctx_t* ctx, uint16_t* wait_ms_out);
static ATCA_STATUS _update_kdf_key_async_poll(_update_kdf_key_ctx_t* ctx, uint16_t* wait_ms_out);
static void _slot_kdf_async_abort(_slot_kdf_ctx_t* ctx);
static ATCA_STATUS _slot_kdf_async_start(
    _slot_kdf_ctx_t* ctx,
    atecc_slot_t slot,
    const uint8_t* msg,
    size_t len,
    uint8_t* out,
    uint16_t* wait_ms_out);
static ATCA_STATUS _slot_kdf_async_poll(_slot_kdf_ctx_t* ctx, uint16_t* wait_ms_out);

static ATCADevice _get_atecc_device(void)
{
    return atcab_get_device();
}

static void _clear_authorize_ctx(_authorize_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_rollkey_ctx(_rollkey_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_update_kdf_key_ctx(_update_kdf_key_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_slot_kdf_ctx(_slot_kdf_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_reset_keys_ctx(_reset_keys_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_stretch_password_ctx(_stretch_password_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static void _clear_init_new_password_ctx(_init_new_password_ctx_t* ctx)
{
    memset(ctx, 0, sizeof(*ctx));
}

static ATCA_STATUS _start_read_zone(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t zone,
    uint16_t slot,
    uint8_t block,
    uint8_t offset,
    uint8_t len,
    uint16_t* wait_ms_out)
{
    uint16_t addr;
    ATCA_STATUS status;

    if (len != 4 && len != 32) {
        return ATCA_BAD_PARAM;
    }
    if ((status = calib_get_addr(zone, slot, block, offset, &addr)) != ATCA_SUCCESS) {
        return status;
    }
    if (len == ATCA_BLOCK_SIZE) {
        zone |= ATCA_ZONE_READWRITE_32;
    }
    packet->param1 = zone;
    packet->param2 = addr;
    if ((status = atRead(device->mCommands, packet)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _finish_read_zone(const ATCAPacket* packet, uint8_t* data, uint8_t len)
{
    if (packet->data[ATCA_COUNT_IDX] < ATCA_PACKET_OVERHEAD + len) {
        return ATCA_RX_FAIL;
    }
    memcpy(data, &packet->data[ATCA_RSP_DATA_IDX], len);
    return ATCA_SUCCESS;
}

static ATCA_STATUS _start_nonce_rand(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    const uint8_t* num_in,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = NONCE_MODE_SEED_UPDATE;
    packet->param2 = 0;
    memcpy(packet->data, num_in, NONCE_NUMIN_SIZE);
    if ((status = atNonce(device->mCommands, packet)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _finish_nonce_rand(const ATCAPacket* packet, uint8_t* rand_out)
{
    if (packet->data[ATCA_COUNT_IDX] < ATCA_PACKET_OVERHEAD + RANDOM_NUM_SIZE) {
        return ATCA_RX_FAIL;
    }
    if (rand_out != NULL) {
        memcpy(rand_out, &packet->data[ATCA_RSP_DATA_IDX], RANDOM_NUM_SIZE);
    }
    return ATCA_SUCCESS;
}

static ATCA_STATUS _start_checkmac(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t mode,
    uint16_t key_id,
    const uint8_t* challenge,
    const uint8_t* response,
    const uint8_t* other_data,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = mode;
    packet->param2 = key_id;
    if (challenge != NULL) {
        memcpy(&packet->data[0], challenge, CHECKMAC_CLIENT_CHALLENGE_SIZE);
    } else {
        memset(&packet->data[0], 0, CHECKMAC_CLIENT_CHALLENGE_SIZE);
    }
    memcpy(&packet->data[32], response, CHECKMAC_CLIENT_RESPONSE_SIZE);
    memcpy(&packet->data[64], other_data, CHECKMAC_OTHER_DATA_SIZE);
    if ((status = atCheckMAC(device->mCommands, packet)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _start_derivekey(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t mode,
    uint16_t target_key,
    const uint8_t* mac,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = mode;
    packet->param2 = target_key;
    if (mac != NULL) {
        memcpy(packet->data, mac, MAC_SIZE);
    }
    if ((status = atDeriveKey(device->mCommands, packet, mac != NULL)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _start_gendig(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t zone,
    uint16_t key_id,
    const uint8_t* other_data,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = zone;
    packet->param2 = key_id;
    memcpy(&packet->data[0], other_data, ATCA_WORD_SIZE);
    if ((status = atGenDig(device->mCommands, packet, true)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _start_write(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t zone,
    uint16_t address,
    const uint8_t* value,
    const uint8_t* mac,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = zone;
    packet->param2 = address;
    if (zone & ATCA_ZONE_READWRITE_32) {
        memcpy(packet->data, value, 32);
        if (mac != NULL) {
            memcpy(&packet->data[32], mac, 32);
        }
    } else {
        memcpy(packet->data, value, 4);
    }
    if ((status =
             atWrite(device->mCommands, packet, mac != NULL && (zone & ATCA_ZONE_READWRITE_32))) !=
        ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _start_kdf(
    atecc_command_ctx_t* command,
    ATCAPacket* packet,
    ATCADevice device,
    uint8_t mode,
    uint16_t key_id,
    uint32_t details,
    const uint8_t* message,
    uint16_t* wait_ms_out)
{
    ATCA_STATUS status;

    packet->param1 = mode;
    packet->param2 = key_id;
    packet->data[0] = details;
    packet->data[1] = details >> 8;
    packet->data[2] = details >> 16;
    packet->data[3] = details >> 24;
    memcpy(&packet->data[KDF_DETAILS_SIZE], message, packet->data[3]);
    if ((status = atKDF(device->mCommands, packet)) != ATCA_SUCCESS) {
        return status;
    }
    return atecc_command_start(command, packet, device, wait_ms_out);
}

static ATCA_STATUS _finish_kdf(const ATCAPacket* packet, uint8_t* out_data, uint8_t* out_nonce)
{
    const uint16_t expected = ATCA_PACKET_OVERHEAD + 32 + 32;

    if (packet->data[ATCA_COUNT_IDX] < expected) {
        return ATCA_RX_FAIL;
    }
    memcpy(out_data, &packet->data[ATCA_RSP_DATA_IDX], 32);
    memcpy(out_nonce, &packet->data[ATCA_RSP_DATA_IDX + 32], 32);
    return ATCA_SUCCESS;
}

static ATCA_STATUS _authorize_async_start(_authorize_ctx_t* ctx, uint16_t* wait_ms_out)
{
    ATCADevice device = _get_atecc_device();

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (device == NULL) {
        return ATCA_COMM_FAIL;
    }
    _clear_authorize_ctx(ctx);
    ctx->state = _AUTHORIZE_STATE_NONCE;
    return _start_nonce_rand(&ctx->command, &ctx->packet, device, ctx->num_in, wait_ms_out);
}

static ATCA_STATUS _authorize_async_poll(_authorize_ctx_t* ctx, uint16_t* wait_ms_out)
{
    static const uint8_t other_data[13] = {0};
    ATCADevice device = _get_atecc_device();
    ATCA_STATUS status;

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (device == NULL) {
        _clear_authorize_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    switch (ctx->state) {
    case _AUTHORIZE_STATE_NONCE: {
        atca_nonce_in_out_t nonce_params = {
            .mode = NONCE_MODE_SEED_UPDATE,
            .zero = 0,
            .num_in = ctx->num_in,
            .rand_out = ctx->rand_out,
            .temp_key = &ctx->temp_key,
        };
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status != ATCA_SUCCESS) {
            if (status != ATCA_RX_NO_RESPONSE) {
                _clear_authorize_ctx(ctx);
            }
            return status;
        }
        if ((status = _finish_nonce_rand(&ctx->packet, ctx->rand_out)) != ATCA_SUCCESS) {
            _clear_authorize_ctx(ctx);
            return status;
        }
        if ((status = atcah_nonce(&nonce_params)) != ATCA_SUCCESS) {
            _clear_authorize_ctx(ctx);
            return status;
        }
        ctx->state = _AUTHORIZE_STATE_READ_SERIAL;
        return _start_read_zone(
            &ctx->command,
            &ctx->packet,
            device,
            ATCA_ZONE_CONFIG,
            0,
            0,
            0,
            ATCA_BLOCK_SIZE,
            wait_ms_out);
    }
    case _AUTHORIZE_STATE_READ_SERIAL: {
        uint8_t response[32] = {0};
        UTIL_CLEANUP_32(response);
        uint8_t auth_key[32] = {0};
        UTIL_CLEANUP_32(auth_key);
        atca_check_mac_in_out_t checkmac_params = {
            .mode = CHECKMAC_MODE_BLOCK2_TEMPKEY,
            .key_id = ATECC_SLOT_AUTHKEY,
            .sn = ctx->serial_block,
            .client_chal = NULL,
            .client_resp = response,
            .other_data = other_data,
            .otp = NULL,
            .slot_key = auth_key,
            .target_key = NULL,
            .temp_key = &ctx->temp_key,
        };
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status != ATCA_SUCCESS) {
            if (status != ATCA_RX_NO_RESPONSE) {
                _clear_authorize_ctx(ctx);
            }
            return status;
        }
        if ((status = _finish_read_zone(&ctx->packet, ctx->serial_block, ATCA_BLOCK_SIZE)) !=
            ATCA_SUCCESS) {
            _clear_authorize_ctx(ctx);
            return status;
        }
        memmove(&ctx->serial_block[4], &ctx->serial_block[8], 5);
        _interface_functions->get_auth_key(auth_key);
        if ((status = atcah_check_mac(&checkmac_params)) != ATCA_SUCCESS) {
            _clear_authorize_ctx(ctx);
            return status;
        }
        ctx->state = _AUTHORIZE_STATE_CHECKMAC;
        return _start_checkmac(
            &ctx->command,
            &ctx->packet,
            device,
            checkmac_params.mode,
            checkmac_params.key_id,
            checkmac_params.client_chal,
            checkmac_params.client_resp,
            checkmac_params.other_data,
            wait_ms_out);
    }
    case _AUTHORIZE_STATE_CHECKMAC:
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status != ATCA_RX_NO_RESPONSE) {
            _clear_authorize_ctx(ctx);
        }
        return status;
    default:
        return ATCA_BAD_PARAM;
    }
}

static void _authorize_async_abort(_authorize_ctx_t* ctx)
{
    if (ctx == NULL) {
        return;
    }
    atecc_command_abort(&ctx->command);
    _clear_authorize_ctx(ctx);
}

static ATCA_STATUS _slot_kdf_async_start(
    _slot_kdf_ctx_t* ctx,
    atecc_slot_t slot,
    const uint8_t* msg,
    size_t len,
    uint8_t* out,
    uint16_t* wait_ms_out)
{
    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (len > 127 || (slot != ATECC_SLOT_ROLLKEY && slot != ATECC_SLOT_KDF) || msg == out) {
        return (ATCA_STATUS)SC_ERR_INVALID_ARGS;
    }
    _clear_slot_kdf_ctx(ctx);
    ctx->state = _SLOT_KDF_STATE_AUTHORIZE;
    ctx->slot = slot;
    ctx->msg = msg;
    ctx->len = len;
    ctx->out = out;
    return _authorize_async_start(&ctx->authorize, wait_ms_out);
}

static ATCA_STATUS _slot_kdf_async_poll(_slot_kdf_ctx_t* ctx, uint16_t* wait_ms_out)
{
    ATCADevice device = _get_atecc_device();
    ATCA_STATUS status;

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (device == NULL) {
        _clear_slot_kdf_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    switch (ctx->state) {
    case _SLOT_KDF_STATE_AUTHORIZE:
        status = _authorize_async_poll(&ctx->authorize, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_slot_kdf_ctx(ctx);
            return status;
        }
        ctx->state = _SLOT_KDF_STATE_KDF;
        return _start_kdf(
            &ctx->command,
            &ctx->packet,
            device,
            KDF_MODE_SOURCE_SLOT | KDF_MODE_TARGET_OUTPUT_ENC | KDF_MODE_ALG_HKDF,
            ctx->slot,
            KDF_DETAILS_HKDF_MSG_LOC_INPUT | (ctx->len << 24),
            ctx->msg,
            wait_ms_out);
    case _SLOT_KDF_STATE_KDF: {
        uint8_t io_protection_key[32] = {0};
        UTIL_CLEANUP_32(io_protection_key);
        atca_io_decrypt_in_out_t io_dec_params = {
            .io_key = io_protection_key,
            .out_nonce = ctx->nonce_out,
            .data = ctx->out,
            .data_size = 32,
        };
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_slot_kdf_ctx(ctx);
            return status;
        }
        if ((status = _finish_kdf(&ctx->packet, ctx->out, ctx->nonce_out)) != ATCA_SUCCESS) {
            _clear_slot_kdf_ctx(ctx);
            return status;
        }
        _interface_functions->get_io_protection_key(io_protection_key);
        status = atcah_io_decrypt(&io_dec_params);
        _clear_slot_kdf_ctx(ctx);
        return status;
    }
    default:
        return ATCA_BAD_PARAM;
    }
}

static void _slot_kdf_async_abort(_slot_kdf_ctx_t* ctx)
{
    if (ctx == NULL) {
        return;
    }
    _authorize_async_abort(&ctx->authorize);
    atecc_command_abort(&ctx->command);
    _clear_slot_kdf_ctx(ctx);
}

static ATCA_STATUS _rollkey_async_start(_rollkey_ctx_t* ctx, uint16_t* wait_ms_out)
{
    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    _clear_rollkey_ctx(ctx);
    ctx->state = _ROLLKEY_STATE_AUTHORIZE;
    return _authorize_async_start(&ctx->authorize, wait_ms_out);
}

static ATCA_STATUS _rollkey_async_poll(_rollkey_ctx_t* ctx, uint16_t* wait_ms_out)
{
    ATCADevice device = _get_atecc_device();
    ATCA_STATUS status;

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (device == NULL) {
        _clear_rollkey_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    switch (ctx->state) {
    case _ROLLKEY_STATE_AUTHORIZE:
        status = _authorize_async_poll(&ctx->authorize, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_rollkey_ctx(ctx);
            return status;
        }
        ctx->state = _ROLLKEY_STATE_NONCE;
        return _start_nonce_rand(&ctx->command, &ctx->packet, device, ctx->num_in, wait_ms_out);
    case _ROLLKEY_STATE_NONCE:
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_rollkey_ctx(ctx);
            return status;
        }
        ctx->state = _ROLLKEY_STATE_DERIVEKEY;
        return _start_derivekey(
            &ctx->command, &ctx->packet, device, 0, ATECC_SLOT_ROLLKEY, NULL, wait_ms_out);
    case _ROLLKEY_STATE_DERIVEKEY:
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status != ATCA_RX_NO_RESPONSE) {
            _clear_rollkey_ctx(ctx);
        }
        return status;
    default:
        return ATCA_BAD_PARAM;
    }
}

static void _rollkey_async_abort(_rollkey_ctx_t* ctx)
{
    if (ctx == NULL) {
        return;
    }
    _authorize_async_abort(&ctx->authorize);
    atecc_command_abort(&ctx->command);
    _clear_rollkey_ctx(ctx);
}

static ATCA_STATUS _update_kdf_key_async_start(_update_kdf_key_ctx_t* ctx, uint16_t* wait_ms_out)
{
    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    _clear_update_kdf_key_ctx(ctx);
    _interface_functions->random_32_bytes(ctx->new_key);
    _interface_functions->get_encryption_key(ctx->encryption_key);
    _interface_functions->random_32_bytes(ctx->nonce_contribution);
    ctx->state = _UPDATE_KDF_KEY_STATE_AUTHORIZE;
    return _authorize_async_start(&ctx->authorize, wait_ms_out);
}

static ATCA_STATUS _update_kdf_key_async_poll(_update_kdf_key_ctx_t* ctx, uint16_t* wait_ms_out)
{
    ATCADevice device = _get_atecc_device();
    ATCA_STATUS status;

    if (ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (device == NULL) {
        _clear_update_kdf_key_ctx(ctx);
        return ATCA_COMM_FAIL;
    }

    switch (ctx->state) {
    case _UPDATE_KDF_KEY_STATE_AUTHORIZE:
        status = _authorize_async_poll(&ctx->authorize, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        ctx->state = _UPDATE_KDF_KEY_STATE_READ_CONFIG;
        return _start_read_zone(
            &ctx->command,
            &ctx->packet,
            device,
            ATCA_ZONE_CONFIG,
            0,
            0,
            0,
            ATCA_BLOCK_SIZE,
            wait_ms_out);
    case _UPDATE_KDF_KEY_STATE_READ_CONFIG:
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        if ((status = _finish_read_zone(&ctx->packet, ctx->read_buf, ATCA_BLOCK_SIZE)) !=
            ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        memcpy(ctx->serial_num, ctx->read_buf, sizeof(ctx->serial_num));
        memmove(&ctx->serial_num[4], &ctx->serial_num[8], 5);
        ctx->state = _UPDATE_KDF_KEY_STATE_NONCE;
        return _start_nonce_rand(
            &ctx->command, &ctx->packet, device, ctx->nonce_contribution, wait_ms_out);
    case _UPDATE_KDF_KEY_STATE_NONCE: {
        atca_nonce_in_out_t nonce_params = {
            .mode = NONCE_MODE_SEED_UPDATE,
            .zero = 0,
            .num_in = ctx->nonce_contribution,
            .rand_out = ctx->rand_out,
            .temp_key = &ctx->temp_key,
        };
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        if ((status = _finish_nonce_rand(&ctx->packet, ctx->rand_out)) != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        if ((status = atcah_nonce(&nonce_params)) != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        ctx->other_data[0] = ATCA_GENDIG;
        ctx->other_data[1] = GENDIG_ZONE_DATA;
        ctx->other_data[2] = (uint8_t)ATECC_SLOT_ENCRYPTION_KEY;
        ctx->other_data[3] = (uint8_t)(ATECC_SLOT_ENCRYPTION_KEY >> 8);
        ctx->state = _UPDATE_KDF_KEY_STATE_GENDIG;
        return _start_gendig(
            &ctx->command,
            &ctx->packet,
            device,
            GENDIG_ZONE_DATA,
            ATECC_SLOT_ENCRYPTION_KEY,
            ctx->other_data,
            wait_ms_out);
    }
    case _UPDATE_KDF_KEY_STATE_GENDIG: {
        atca_gen_dig_in_out_t gen_dig_param = {
            .zone = GENDIG_ZONE_DATA,
            .key_id = ATECC_SLOT_ENCRYPTION_KEY,
            .is_key_nomac = false,
            .sn = ctx->serial_num,
            .stored_value = ctx->encryption_key,
            .other_data = ctx->other_data,
            .temp_key = &ctx->temp_key,
        };
        atca_write_mac_in_out_t write_mac_param = {
            .zone = ATCA_ZONE_DATA | ATCA_ZONE_READWRITE_32 | ATCA_ZONE_ENCRYPTED,
            .key_id = 0,
            .sn = ctx->serial_num,
            .input_data = ctx->new_key,
            .encrypted_data = ctx->cipher_text,
            .auth_mac = ctx->mac,
            .temp_key = &ctx->temp_key,
        };
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        if ((status = atcah_gen_dig(&gen_dig_param)) != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        if ((status = calib_get_addr(ATCA_ZONE_DATA, ATECC_SLOT_KDF, 0, 0, &ctx->addr)) !=
            ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        write_mac_param.key_id = ctx->addr;
        if ((status = atcah_write_auth_mac(&write_mac_param)) != ATCA_SUCCESS) {
            _clear_update_kdf_key_ctx(ctx);
            return status;
        }
        ctx->state = _UPDATE_KDF_KEY_STATE_WRITE;
        return _start_write(
            &ctx->command,
            &ctx->packet,
            device,
            write_mac_param.zone,
            write_mac_param.key_id,
            write_mac_param.encrypted_data,
            write_mac_param.auth_mac,
            wait_ms_out);
    }
    case _UPDATE_KDF_KEY_STATE_WRITE:
        status = atecc_command_poll(&ctx->command, wait_ms_out);
        if (status != ATCA_RX_NO_RESPONSE) {
            _clear_update_kdf_key_ctx(ctx);
        }
        return status;
    default:
        return ATCA_BAD_PARAM;
    }
}

static void _update_kdf_key_async_abort(_update_kdf_key_ctx_t* ctx)
{
    if (ctx == NULL) {
        return;
    }
    _authorize_async_abort(&ctx->authorize);
    atecc_command_abort(&ctx->command);
    _clear_update_kdf_key_ctx(ctx);
}

int atecc_kdf_async_start(
    atecc_kdf_async_ctx_t* ctx,
    const uint8_t* msg,
    size_t len,
    uint8_t* kdf_out,
    uint16_t* wait_ms_out)
{
    return _slot_kdf_async_start(
        _KDF_ASYNC_CTX(ctx), ATECC_SLOT_KDF, msg, len, kdf_out, wait_ms_out);
}

int atecc_kdf_async_poll(atecc_kdf_async_ctx_t* ctx, uint16_t* wait_ms_out)
{
    return _slot_kdf_async_poll(_KDF_ASYNC_CTX(ctx), wait_ms_out);
}

void atecc_kdf_async_abort(atecc_kdf_async_ctx_t* ctx)
{
    _slot_kdf_async_abort(_KDF_ASYNC_CTX(ctx));
}

int atecc_reset_keys_async_start(atecc_reset_keys_async_ctx_t* ctx, uint16_t* wait_ms_out)
{
    _reset_keys_ctx_t* reset_ctx = _RESET_KEYS_ASYNC_CTX(ctx);

    if (reset_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    _clear_reset_keys_ctx(reset_ctx);
    reset_ctx->state = _RESET_KEYS_STATE_ROLLKEY;
    return _rollkey_async_start(&reset_ctx->rollkey, wait_ms_out);
}

int atecc_reset_keys_async_poll(atecc_reset_keys_async_ctx_t* ctx, uint16_t* wait_ms_out)
{
    _reset_keys_ctx_t* reset_ctx = _RESET_KEYS_ASYNC_CTX(ctx);
    ATCA_STATUS status;

    if (reset_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }

    switch (reset_ctx->state) {
    case _RESET_KEYS_STATE_ROLLKEY:
        status = _rollkey_async_poll(&reset_ctx->rollkey, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_reset_keys_ctx(reset_ctx);
            return status;
        }
        reset_ctx->state = _RESET_KEYS_STATE_UPDATE_KDF_KEY;
        return _update_kdf_key_async_start(&reset_ctx->update_kdf_key, wait_ms_out);
    case _RESET_KEYS_STATE_UPDATE_KDF_KEY:
        status = _update_kdf_key_async_poll(&reset_ctx->update_kdf_key, wait_ms_out);
        if (status != ATCA_RX_NO_RESPONSE) {
            _clear_reset_keys_ctx(reset_ctx);
        }
        return status;
    default:
        return ATCA_BAD_PARAM;
    }
}

void atecc_reset_keys_async_abort(atecc_reset_keys_async_ctx_t* ctx)
{
    _reset_keys_ctx_t* reset_ctx = _RESET_KEYS_ASYNC_CTX(ctx);

    _rollkey_async_abort(&reset_ctx->rollkey);
    _update_kdf_key_async_abort(&reset_ctx->update_kdf_key);
    _clear_reset_keys_ctx(reset_ctx);
}

int atecc_stretch_password_async_start(
    atecc_stretch_password_async_ctx_t* ctx,
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out,
    uint16_t* wait_ms_out)
{
    _stretch_password_ctx_t* stretch_ctx = _STRETCH_PASSWORD_ASYNC_CTX(ctx);

    if (stretch_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (password_stretch_algo != SECURECHIP_PASSWORD_STRETCH_ALGO_V0) {
        return SC_ERR_INVALID_PASSWORD_STRETCH_ALGO;
    }
    _clear_stretch_password_ctx(stretch_ctx);
    stretch_ctx->password = password;
    stretch_ctx->password_stretch_algo = password_stretch_algo;
    stretch_ctx->stretched_out = stretched_out;
    if (!rust_salt_hash_data(
            rust_util_bytes((const uint8_t*)password, strlen(password)),
            "keystore_seed_access_in",
            rust_util_bytes_mut(
                stretch_ctx->password_salted_hashed,
                sizeof(stretch_ctx->password_salted_hashed)))) {
        _clear_stretch_password_ctx(stretch_ctx);
        return SC_ERR_SALT;
    }
    memcpy(stretch_ctx->kdf_in, stretch_ctx->password_salted_hashed, 32);
    stretch_ctx->state = _STRETCH_PASSWORD_STATE_ROLLKEY_KDF;
    return _slot_kdf_async_start(
        &stretch_ctx->slot_kdf,
        ATECC_SLOT_ROLLKEY,
        stretch_ctx->kdf_in,
        32,
        stretch_ctx->stretched_out,
        wait_ms_out);
}

int atecc_stretch_password_async_poll(
    atecc_stretch_password_async_ctx_t* ctx,
    uint16_t* wait_ms_out)
{
    _stretch_password_ctx_t* stretch_ctx = _STRETCH_PASSWORD_ASYNC_CTX(ctx);
    int status;

    if (stretch_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }

    switch (stretch_ctx->state) {
    case _STRETCH_PASSWORD_STATE_ROLLKEY_KDF:
    case _STRETCH_PASSWORD_STATE_KDF:
        status = _slot_kdf_async_poll(&stretch_ctx->slot_kdf, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_stretch_password_ctx(stretch_ctx);
            return status;
        }
        if (stretch_ctx->state == _STRETCH_PASSWORD_STATE_ROLLKEY_KDF) {
            stretch_ctx->state = _STRETCH_PASSWORD_STATE_KDF;
            stretch_ctx->iteration = 0;
        }
        if (stretch_ctx->iteration < KDF_NUM_ITERATIONS) {
            memcpy(stretch_ctx->kdf_in, stretch_ctx->stretched_out, 32);
            stretch_ctx->iteration++;
            return _slot_kdf_async_start(
                &stretch_ctx->slot_kdf,
                ATECC_SLOT_KDF,
                stretch_ctx->kdf_in,
                32,
                stretch_ctx->stretched_out,
                wait_ms_out);
        }
        if (!rust_salt_hash_data(
                rust_util_bytes(
                    (const uint8_t*)stretch_ctx->password, strlen(stretch_ctx->password)),
                "keystore_seed_access_out",
                rust_util_bytes_mut(
                    stretch_ctx->password_salted_hashed,
                    sizeof(stretch_ctx->password_salted_hashed)))) {
            _clear_stretch_password_ctx(stretch_ctx);
            return SC_ERR_SALT;
        }
        rust_hmac_sha256(
            stretch_ctx->password_salted_hashed,
            sizeof(stretch_ctx->password_salted_hashed),
            stretch_ctx->stretched_out,
            32,
            stretch_ctx->stretched_out);
        _clear_stretch_password_ctx(stretch_ctx);
        return ATCA_SUCCESS;
    default:
        return ATCA_BAD_PARAM;
    }
}

void atecc_stretch_password_async_abort(atecc_stretch_password_async_ctx_t* ctx)
{
    _stretch_password_ctx_t* stretch_ctx = _STRETCH_PASSWORD_ASYNC_CTX(ctx);

    _slot_kdf_async_abort(&stretch_ctx->slot_kdf);
    _clear_stretch_password_ctx(stretch_ctx);
}

int atecc_init_new_password_async_start(
    atecc_init_new_password_async_ctx_t* ctx,
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out,
    uint16_t* wait_ms_out)
{
    _init_new_password_ctx_t* init_ctx = _INIT_NEW_PASSWORD_ASYNC_CTX(ctx);

    if (init_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }
    if (password_stretch_algo != SECURECHIP_PASSWORD_STRETCH_ALGO_V0) {
        return SC_ERR_INVALID_PASSWORD_STRETCH_ALGO;
    }
    _clear_init_new_password_ctx(init_ctx);
    init_ctx->password = password;
    init_ctx->password_stretch_algo = password_stretch_algo;
    init_ctx->stretched_out = stretched_out;
    init_ctx->state = _INIT_NEW_PASSWORD_STATE_RESET_KEYS;
    return atecc_reset_keys_async_start(
        (atecc_reset_keys_async_ctx_t*)&init_ctx->reset_keys, wait_ms_out);
}

int atecc_init_new_password_async_poll(
    atecc_init_new_password_async_ctx_t* ctx,
    uint16_t* wait_ms_out)
{
    _init_new_password_ctx_t* init_ctx = _INIT_NEW_PASSWORD_ASYNC_CTX(ctx);
    int status;

    if (init_ctx == NULL || wait_ms_out == NULL) {
        return ATCA_BAD_PARAM;
    }

    switch (init_ctx->state) {
    case _INIT_NEW_PASSWORD_STATE_RESET_KEYS:
        status = atecc_reset_keys_async_poll(
            (atecc_reset_keys_async_ctx_t*)&init_ctx->reset_keys, wait_ms_out);
        if (status == ATCA_RX_NO_RESPONSE) {
            return status;
        }
        if (status != ATCA_SUCCESS) {
            _clear_init_new_password_ctx(init_ctx);
            return SC_ATECC_ERR_RESET_KEYS;
        }
        init_ctx->state = _INIT_NEW_PASSWORD_STATE_STRETCH_PASSWORD;
        return atecc_stretch_password_async_start(
            (atecc_stretch_password_async_ctx_t*)&init_ctx->stretch_password,
            init_ctx->password,
            init_ctx->password_stretch_algo,
            init_ctx->stretched_out,
            wait_ms_out);
    case _INIT_NEW_PASSWORD_STATE_STRETCH_PASSWORD:
        status = atecc_stretch_password_async_poll(
            (atecc_stretch_password_async_ctx_t*)&init_ctx->stretch_password, wait_ms_out);
        if (status != ATCA_RX_NO_RESPONSE) {
            _clear_init_new_password_ctx(init_ctx);
        }
        return status;
    default:
        return ATCA_BAD_PARAM;
    }
}

void atecc_init_new_password_async_abort(atecc_init_new_password_async_ctx_t* ctx)
{
    _init_new_password_ctx_t* init_ctx = _INIT_NEW_PASSWORD_ASYNC_CTX(ctx);

    atecc_reset_keys_async_abort((atecc_reset_keys_async_ctx_t*)&init_ctx->reset_keys);
    atecc_stretch_password_async_abort(
        (atecc_stretch_password_async_ctx_t*)&init_ctx->stretch_password);
    _clear_init_new_password_ctx(init_ctx);
}

static ATCA_STATUS _sync_rollkey_async(void)
{
    _rollkey_ctx_t ctx = {0};
    ATCA_STATUS status;
    uint16_t wait_ms = 0;

    status = _rollkey_async_start(&ctx, &wait_ms);
    while (status == ATCA_RX_NO_RESPONSE) {
        atca_delay_ms(wait_ms);
        status = _rollkey_async_poll(&ctx, &wait_ms);
    }
    _rollkey_async_abort(&ctx);
    return status;
}

static ATCA_STATUS _sync_update_kdf_key_async(void)
{
    _update_kdf_key_ctx_t ctx = {0};
    ATCA_STATUS status;
    uint16_t wait_ms = 0;

    status = _update_kdf_key_async_start(&ctx, &wait_ms);
    while (status == ATCA_RX_NO_RESPONSE) {
        atca_delay_ms(wait_ms);
        status = _update_kdf_key_async_poll(&ctx, &wait_ms);
    }
    _update_kdf_key_async_abort(&ctx);
    return status;
}

static int _sync_slot_kdf_async(atecc_slot_t slot, const uint8_t* msg, size_t len, uint8_t* out)
{
    _slot_kdf_ctx_t ctx = {0};
    ATCA_STATUS status;
    uint16_t wait_ms = 0;

    status = _slot_kdf_async_start(&ctx, slot, msg, len, out, &wait_ms);
    while (status == ATCA_RX_NO_RESPONSE) {
        atca_delay_ms(wait_ms);
        status = _slot_kdf_async_poll(&ctx, &wait_ms);
    }
    _slot_kdf_async_abort(&ctx);
    return status;
}

/**
 * This performs the CheckMac command on ATECC_SLOT_AUTHKEY. This needs to
 * be called before using any slot requiring auth and whose KeyConfig.AuthKey is
 * ATECC_SLOT_AUTHKEY.
 */
static ATCA_STATUS _authorize_key(void)
{
    uint8_t num_in[NONCE_NUMIN_SIZE] = {0};
    uint8_t rand_out[32] = {0};

    atca_temp_key_t temp_key = {0};

    atca_nonce_in_out_t nonce_params = {
        .mode = NONCE_MODE_SEED_UPDATE,
        .zero = 0,
        .num_in = num_in,
        .rand_out = rand_out,
        .temp_key = &temp_key,
    };
    ATCA_STATUS result = atcab_nonce_rand(nonce_params.num_in, rand_out);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    // Calculate contents of TempKey.
    result = atcah_nonce(&nonce_params);
    if (result != ATCA_SUCCESS) {
        return result;
    }
    uint8_t response[32] = {0};
    const uint8_t other_data[13] = {
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
    uint8_t sn[9] = {0};
    result = atcab_read_serial_number(sn);
    if (result != ATCA_SUCCESS) {
        return result;
    }

    uint8_t auth_key[32] = {0};
    UTIL_CLEANUP_32(auth_key);
    _interface_functions->get_auth_key(auth_key);
    atca_check_mac_in_out_t checkmac_params = {
        // First SHA block from slot key, Second SHA block from TempKey.
        .mode = CHECKMAC_MODE_BLOCK2_TEMPKEY,
        .key_id = ATECC_SLOT_AUTHKEY,
        .sn = sn,
        .client_chal = NULL, // unused in this mode
        .client_resp = response,
        .other_data = other_data,
        .otp = NULL, // unused in this mode,
        .slot_key = auth_key,
        .target_key = NULL,
        .temp_key = &temp_key,
    };
    // Compute client response.
    result = atcah_check_mac(&checkmac_params);
    if (result != ATCA_SUCCESS) {
        return result;
    }

    return atcab_checkmac(
        checkmac_params.mode,
        checkmac_params.key_id,
        checkmac_params.client_chal,
        checkmac_params.client_resp,
        checkmac_params.other_data);
}

/**
 * Performs a roll-key operation on a ATECC_SLOT_ROLLKEY.
 * @return ATCA_SUCCESS on success.
 */
static ATCA_STATUS _rollkey(void)
{
    return _sync_rollkey_async();
}

/**
 * Writes a new random key to ATECC_SLOT_KDF.
 * @return ATCA_SUCCESS on success.
 */
static ATCA_STATUS _update_kdf_key(void)
{
    return _sync_update_kdf_key_async();
}

static int _atecc_kdf(atecc_slot_t slot, const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    return _sync_slot_kdf_async(slot, msg, len, kdf_out);
}

int atecc_kdf(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    return _atecc_kdf(ATECC_SLOT_KDF, msg, len, kdf_out);
}

int atecc_init_new_password(
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out)
{
    (void)password;
    if (password_stretch_algo != SECURECHIP_PASSWORD_STRETCH_ALGO_V0) {
        return SC_ERR_INVALID_PASSWORD_STRETCH_ALGO;
    }
    if (!atecc_reset_keys()) {
        return SC_ATECC_ERR_RESET_KEYS;
    }
    return atecc_stretch_password(password, password_stretch_algo, stretched_out);
}

int atecc_stretch_password(
    const char* password,
    securechip_password_stretch_algo_t password_stretch_algo,
    uint8_t* stretched_out)
{
    if (password_stretch_algo != SECURECHIP_PASSWORD_STRETCH_ALGO_V0) {
        return SC_ERR_INVALID_PASSWORD_STRETCH_ALGO;
    }

    uint8_t password_salted_hashed[32] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!rust_salt_hash_data(
            rust_util_bytes((const uint8_t*)password, strlen(password)),
            "keystore_seed_access_in",
            rust_util_bytes_mut(password_salted_hashed, sizeof(password_salted_hashed)))) {
        return SC_ERR_SALT;
    }

    uint8_t kdf_in[32] = {0};
    UTIL_CLEANUP_32(kdf_in);
    memcpy(kdf_in, password_salted_hashed, 32);

    // First KDF on rollkey increments the monotonic counter. Call only once!
    int securechip_result = _atecc_kdf(ATECC_SLOT_ROLLKEY, kdf_in, 32, stretched_out);
    if (securechip_result) {
        return securechip_result;
    }
    // Second KDF does not use the counter and we call it multiple times.
    for (int i = 0; i < KDF_NUM_ITERATIONS; i++) {
        memcpy(kdf_in, stretched_out, 32);
        securechip_result = securechip_kdf(kdf_in, 32, stretched_out);
        if (securechip_result) {
            return securechip_result;
        }
    }

    if (!rust_salt_hash_data(
            rust_util_bytes((const uint8_t*)password, strlen(password)),
            "keystore_seed_access_out",
            rust_util_bytes_mut(password_salted_hashed, sizeof(password_salted_hashed)))) {
        return SC_ERR_SALT;
    }
    rust_hmac_sha256(
        password_salted_hashed, sizeof(password_salted_hashed), stretched_out, 32, stretched_out);
    return 0;
}

bool atecc_reset_keys(void)
{
    if (_rollkey() != ATCA_SUCCESS) {
        return false;
    }
    return _update_kdf_key() == ATCA_SUCCESS;
}

bool atecc_gen_attestation_key(uint8_t* pubkey_out)
{
    ATCA_STATUS result = _authorize_key();
    if (result != ATCA_SUCCESS) {
        return false;
    }
    return atcab_genkey(ATECC_SLOT_ATTESTATION, pubkey_out) == ATCA_SUCCESS;
}

bool atecc_attestation_sign(const uint8_t* msg, uint8_t* signature_out)
{
    ATCA_STATUS result = _authorize_key();
    if (result != ATCA_SUCCESS) {
        return false;
    }
    return atcab_sign(ATECC_SLOT_ATTESTATION, msg, signature_out) == ATCA_SUCCESS;
}

bool atecc_monotonic_increments_remaining(uint32_t* remaining_out)
{
    uint32_t counter;
    if (atcab_counter_read(0, &counter) != ATCA_SUCCESS) {
        return false;
    }
    if (COUNTER_MAX_VALUE < counter) {
        Abort("ATECC returned an invalid value");
    }
    *remaining_out = COUNTER_MAX_VALUE - counter;
    return true;
}

bool atecc_random(uint8_t* rand_out)
{
    for (int retries = 0; retries < 5; retries++) {
        if (atcab_random(rand_out) == ATCA_SUCCESS) {
            return true;
        }
    }
    return false;
}

#if APP_U2F == 1 || FACTORYSETUP == 1
// Read a "standard" sized block from a data slot (must be 32 bytes)
static bool _read_data_slot_block(uint8_t* bytes, uint16_t slot, uint8_t block)
{
    uint8_t encryption_key[32] = {0};
    UTIL_CLEANUP_32(encryption_key);
    _interface_functions->get_encryption_key(encryption_key);

    uint8_t nonce_contribution[32] = {0};
    UTIL_CLEANUP_32(nonce_contribution);
    _interface_functions->random_32_bytes(nonce_contribution);
    #if NONCE_NUMIN_SIZE > 32
        #error "size mismatch"
    #endif

    ATCA_STATUS result = _authorize_key();
    if (result != ATCA_SUCCESS) {
        return false;
    }
    return atcab_read_enc(
               slot, block, bytes, encryption_key, ATECC_SLOT_ENCRYPTION_KEY, nonce_contribution) ==
           ATCA_SUCCESS;
}

// Write a "standard" sized block from a data slot (must be 32 bytes)
static bool _write_data_slot_block(uint8_t* bytes, uint16_t slot, uint8_t block)
{
    uint8_t encryption_key[32] = {0};
    UTIL_CLEANUP_32(encryption_key);
    _interface_functions->get_encryption_key(encryption_key);

    uint8_t nonce_contribution[32] = {0};
    UTIL_CLEANUP_32(nonce_contribution);
    _interface_functions->random_32_bytes(nonce_contribution);
    #if NONCE_NUMIN_SIZE > 32
        #error "size mismatch"
    #endif

    ATCA_STATUS result = _authorize_key();
    if (result != ATCA_SUCCESS) {
        return false;
    }
    result = atcab_write_enc(
        slot, block, bytes, encryption_key, ATECC_SLOT_ENCRYPTION_KEY, nonce_contribution);
    if (result != ATCA_SUCCESS) {
        return false;
    }
    // Double-check by reading it back and comparing.
    uint8_t written_bytes[32] = {0};
    if (!_read_data_slot_block(written_bytes, slot, block)) {
        return false;
    }
    return MEMEQ(written_bytes, bytes, sizeof(written_bytes));
}

bool atecc_u2f_counter_set(uint32_t counter)
{
    data_9_0_t data = {0};
    if (!_read_data_slot_block(&data.bytes[0], ATECC_SLOT_DATA0, 0)) {
        return false;
    }

    data.fields.u2f_counter = counter;

    return _write_data_slot_block(&data.bytes[0], ATECC_SLOT_DATA0, 0);
}
#endif

#if APP_U2F == 1
bool atecc_u2f_counter_inc(uint32_t* counter)
{
    data_9_0_t data = {0};
    if (!_read_data_slot_block(&data.bytes[0], ATECC_SLOT_DATA0, 0)) {
        return false;
    }

    data.fields.u2f_counter += 1;
    *counter = data.fields.u2f_counter;

    return _write_data_slot_block(&data.bytes[0], ATECC_SLOT_DATA0, 0);
}
#endif

bool atecc_model(securechip_model_t* model_out)
{
    uint8_t revision[4] = {0};
    if (atcab_info(revision) != ATCA_SUCCESS) {
        return false;
    }
    *model_out = revision[3] >= 0x03 ? ATECC_ATECC608B : ATECC_ATECC608A;
    return true;
}
