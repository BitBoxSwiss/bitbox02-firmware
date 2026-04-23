// SPDX-License-Identifier: Apache-2.0

#include "optiga_ops.h"

#include <rust/rust.h>
#include <securechip/securechip.h>
#include <util.h>

// The OPTIGA library is asynchronous and will schedule a callback when the command is done. The
// callback will set this shared variable to the result of the command.
static volatile optiga_lib_status_t _optiga_lib_status;

static void _optiga_lib_callback(void* callback_ctx, optiga_lib_status_t event)
{
    (void)callback_ctx;
    _optiga_lib_status = event;
    rust_optiga_callback_wake();
}

optiga_lib_status_t optiga_ops_get_status(void)
{
    return _optiga_lib_status;
}

void optiga_ops_set_status_busy(void)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
}

optiga_lib_status_t optiga_ops_create(optiga_util_t** util_out, optiga_crypt_t** crypt_out)
{
    *util_out = optiga_util_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == *util_out) {
        util_log("couldn't create optiga util");
        return SC_OPTIGA_ERR_CREATE;
    }

    *crypt_out = optiga_crypt_create(OPTIGA_INSTANCE_ID_0, _optiga_lib_callback, NULL);
    if (NULL == *crypt_out) {
        util_log("couldn't create optiga crypt");
        return SC_OPTIGA_ERR_CREATE;
    }

    return 0;
}

// Helper that is used in the main thread to busy wait for the callback to update the shared
// variable.
// It first checks the return status of the command, then busy waits, and then checks the
// asynchronous return status.
// Will return from caller if command failed.
// `return_status` will be updated with the actual return status
// Return statuses are documented in optiga_lib_return_codes.h
#define _WAIT(return_status, optiga_lib_status)          \
    do {                                                 \
        if ((return_status) != OPTIGA_UTIL_SUCCESS) {    \
            return (return_status);                      \
        }                                                \
        while (OPTIGA_LIB_BUSY == (optiga_lib_status)) { \
        }                                                \
        if (OPTIGA_LIB_SUCCESS != (optiga_lib_status)) { \
            return (optiga_lib_status);                  \
        }                                                \
        (return_status) = (optiga_lib_status);           \
    } while (0)

optiga_lib_status_t optiga_ops_util_read_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint16_t offset,
    uint8_t* buffer,
    uint16_t* length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_data(me, optiga_oid, offset, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_util_write_data_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t write_type,
    uint16_t offset,
    const uint8_t* buffer,
    uint16_t length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res =
        optiga_util_write_data(me, optiga_oid, write_type, offset, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_util_read_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    uint8_t* buffer,
    uint16_t* length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_read_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_util_write_metadata_sync(
    optiga_util_t* me,
    uint16_t optiga_oid,
    const uint8_t* buffer,
    uint8_t length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_write_metadata(me, optiga_oid, buffer, length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_util_open_application_sync(optiga_util_t* me, bool_t perform_restore)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_open_application(me, perform_restore);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_util_close_application_sync(
    optiga_util_t* me,
    bool_t perform_hibernate)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_util_close_application(me, perform_hibernate);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_crypt_ecc_generate_keypair_sync(
    optiga_crypt_t* me,
    optiga_ecc_curve_t curve_id,
    optiga_key_usage_t key_usage,
    bool_t export_private_key,
    void* private_key,
    uint8_t* public_key,
    uint16_t* public_key_length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_ecc_generate_keypair(
        me, curve_id, key_usage, export_private_key, private_key, public_key, public_key_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}

optiga_lib_status_t optiga_ops_crypt_random_sync(
    optiga_crypt_t* me,
    optiga_rng_type_t rng_type,
    uint8_t* random_data,
    uint16_t random_data_length)
{
    _optiga_lib_status = OPTIGA_LIB_BUSY;
    optiga_lib_status_t res = optiga_crypt_random(me, rng_type, random_data, random_data_length);
    _WAIT(res, _optiga_lib_status);
    return res;
}
