// SPDX-License-Identifier: Apache-2.0

#include "optiga.h"

#include "optiga_ops.h"

#include "pal/pal_i2c.h"
#include "pal/pal_os_datastore.h"
#include "pal/pal_os_timer.h"

#include <hardfault.h>
#include <memory/bitbox02_smarteeprom.h>
#include <optiga_crypt.h>
#include <optiga_util.h>
#include <rust/rust.h>
#include <salt.h>
#include <securechip/securechip.h>
#include <util.h>

// Set this to 1 for a more convenience during development.
// Factory setup will be performed in the normal firmware, which makes it easier to tinker with the
// chip setup and config.
// Must be 0 for the production firmware releases.
#define FACTORY_DURING_PROD 0

// When to do a sanity check of the expected metadata configuration. The check takes a while and
// causes a noticable delay at boot, so we don't enable it for production firmwares. We also enable
// it in debug builds.
#if FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1 || !defined(NDEBUG)
    #define VERIFY_METADATA 1
#else
    #define VERIFY_METADATA 0
#endif

// Number of times the first kdf slot can be used over the lifetime of the device.
// The maxmimum does not seem to be specified, so we use something a little below the endurance
// indication of 600000 updates. See Solution Reference Manual Figure 32.
#define MONOTONIC_COUNTER_MAX_USE (590000)

// Maximum size of metadata. See "Metadata Update Identifier":
// https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linka946a953_def2_41cf_850a_74fb7899fe11
// Two extra bytes for the `0x20 <len>` header bytes.
#define METADATA_MAX_SIZE (44 + 2)

// See Solution Reference Manual Table 79 "Data structure arbitrary data object".
#define ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE 140

// This number of KDF iterations on the external kdf slot when stretching the device
// password.
#define KDF_NUM_ITERATIONS (2)

// Struct stored in the arbitrary data object.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"
typedef union {
    struct __attribute__((__packed__)) {
        uint32_t u2f_counter;
    } fields;
    uint8_t bytes[ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE];
} arbitrary_data_t;
#pragma GCC diagnostic pop

static optiga_util_t* _util;
static optiga_crypt_t* _crypt;

static const securechip_interface_functions_t* _ifs = NULL;

// Values of life cycle states.
// See Table "Life Cycle Status":
// https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#link05d4c12a_5c94_4a05_a05d_102c53684d3d
#define LCSO_STATE_CREATION (0x01)
#define LCSO_STATE_OPERATIONAL (0x07)

#define TAG_LCSO 0xC0

// Set the object LcsO flag to Operational. After this, the metadata cannot be changed anymore.
// During development, set this to `LCSO_STATE_CREATION`.
#define FINAL_LCSO_STATE LCSO_STATE_OPERATIONAL

#if FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1 || VERIFY_METADATA == 1
static const uint8_t _platform_binding_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    17,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Change/Write access. This allows updating the binding secret when LcsO < op.
    0xD0,
    0x03,
    0xE1,
    0xFC,
    LCSO_STATE_OPERATIONAL,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Allow execute
    0xD3,
    0x01,
    0x00,
    // Data object type set to PTFBIND (Platform binding secret)
    0xE8,
    0x01,
    0x22,
};

static const uint8_t _aes_symkey_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    21,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Set key usage to "Enc".
    // See Table "Metadata associated with data and key objects" -> 0xE1
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#link8051b344_ff66_4d6b_bcfd_d21bb87d05d4
    0xE1,
    0x01,
    0x02,
    // Allow writes - GenSymkey requires this to be able to write.
    // However, writes from the host are forbidden.
    // Table "Common key objects with TAG’s and AC‘s" - 0xE200:
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkf056a0f7_a31f_41c3_b1d9_f270a4fe0378
    // "The GetDataObject, and SetDataObject commands are not allowed for the data part of the key
    // object even if the metadata state the access rights differently"
    0xD0,
    0x01,
    0x00,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Attach execution to counter at 0xE120 and enforce shielded connection.
    // See Table 'Access Condition Identifier and Operators" -> "Conf":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkc15dfea4_2cc2_46ae_a53b_1e6ea9487f34
    0xD3,
    0x07,
    0x40,
    0xE1,
    0x20,
    // &&
    0xFD,
    // Enforce shielded connection. According to 4.4.1.7 "EncryptSym" shielded protection is
    // enforced anyway, but better to be explicit.
    0x20,
    0xE1,
    0x40,
};

static const uint8_t _hmac_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    19,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Data object type: PRESSEC
    // See table "Data Object Types":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkaf9aa284_1397_4161_8761_8c44fbbfa69d
    0xE8,
    0x01,
    0x21,
    // Allow writes, enforce shielded connection.
    0xD0,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Execute: enforce shielded connection.
    // See Table 'Access Condition Identifier and Operators" -> "Conf":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkc15dfea4_2cc2_46ae_a53b_1e6ea9487f34
    0xD3,
    0x03,
    0x20,
    0xE1,
    0x40,
};

static const uint8_t _arbitrary_data_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    19,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Data object type: BSTR.
    // See table "Data Object Types":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkaf9aa284_1397_4161_8761_8c44fbbfa69d
    0xE8,
    0x01,
    0x00,
    // Allow writes, enforce shielded connection.
    0xD0,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Allow reads, enforce shielded connection.
    0xD1,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Disallow exe
    0xD3,
    0x01,
    0xFF,
};

static const uint8_t _attestation_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    17,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Key usage associated with key container: Sign
    // See table "Metadata associated with data and key objects":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#link8051b344_ff66_4d6b_bcfd_d21bb87d05d4
    0xE1,
    0x01,
    0x10,
    // Allow writes - GenKeyPair requires this to be able to write.
    // However, writes from the host are forbidden.
    // Table "Common key objects with TAG’s and AC‘s" - 0xE0F1:
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkf056a0f7_a31f_41c3_b1d9_f270a4fe0378
    // "The GetDataObject, and SetDataObject commands are not allowed for the data part of the key
    // object even if the metadata state the access rights differently"
    0xD0,
    0x01,
    0x00,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Execute: enforce shielded connection.
    // See Table 'Access Condition Identifier and Operators" -> "Conf":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#linkc15dfea4_2cc2_46ae_a53b_1e6ea9487f34
    0xD3,
    0x03,
    0x20,
    0xE1,
    0x40,
};

static const uint8_t _counter_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    16,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Change/Write access. This allows updating the counter when LcsO < op.
    0xD0,
    0x03,
    0xE1,
    0xFC,
    LCSO_STATE_OPERATIONAL,
    // Allow reads, enforce shielded connection.
    0xD1,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Allow exe
    0xD3,
    0x01,
    0x00,
};

static const uint8_t _password_secret_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    25,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Allow writes, enforce shielded connection.
    0xD0,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Allow reads, auth referencing 0xF1D3 (`OID_PASSWORD`), enforce shielded connection
    0xD1,
    0x07,
    0x23,
    0xF1,
    0xD3,
    // &&
    0xFD,
    0x20,
    0xE1,
    0x40,
    // Execute: enforce shielded connection
    0xD3,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Data object type set to AUTOREF
    0xE8,
    0x01,
    0x31,
};

static const uint8_t _password_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    27,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Allow writes, auth referencing 0xF1D2 (`OID_PASSWORD_SECRET`), enforce shielded connection.
    0xD0,
    0x07,
    0x23,
    0xF1,
    0xD2,
    // &&
    0xFD,
    0x20,
    0xE1,
    0x40,
    // Disallow reads
    0xD1,
    0x01,
    0xFF,
    // Attach execution to counter at 0xE121 (`OID_COUNTER_PASSWORD`) and enforce shielded
    // connection.
    0xD3,
    0x07,
    0x40,
    0xE1,
    0x21,
    // &&
    0xFD,
    // Enforce shielded connection. According to 4.4.1.7 "EncryptSym" shielded protection is
    // enforced anyway, but better to be explicit.
    0x20,
    0xE1,
    0x40,
    // Data object type set to AUTOREF
    0xE8,
    0x01,
    0x31,
};

static const uint8_t _counter_password_metadata[] = {
    // Metadata tag in the data object
    0x20,
    // Number of bytes that follow
    20,
    // Set LcsO. Refer to macro to see the value or some more notes.
    0xC0,
    0x01,
    FINAL_LCSO_STATE,
    // Allow writes, auth referencing 0xF1D2 (`OID_PASSWORD_SECRET`), enforce shielded connection.
    0xD0,
    0x07,
    0x23,
    0xF1,
    0xD2,
    // &&
    0xFD,
    0x20,
    0xE1,
    0x40,
    // Allow reads, enforce shielded connection
    0xD1,
    0x03,
    0x20,
    0xE1,
    0x40,
    // Allow exe
    0xD3,
    0x01,
    0x00,
};

#endif

static int _authorize(uint16_t oid_auth, const uint8_t* auth_secret, size_t auth_secret_len)
{
    optiga_lib_status_t res;

    uint8_t random_data[32] = {0};

    res = optiga_ops_crypt_generate_auth_code_sync(
        _crypt, OPTIGA_RNG_TYPE_TRNG, NULL, 0, random_data, sizeof(random_data));
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("generate auth code failed: %x", res);
        return res;
    }

    uint8_t hmac[32] = {0};
    rust_hmac_sha256(auth_secret, auth_secret_len, random_data, sizeof(random_data), hmac);
    res = optiga_ops_crypt_hmac_verify_sync(
        _crypt,
        OPTIGA_HMAC_SHA_256,
        oid_auth,
        random_data,
        sizeof(random_data),
        hmac,
        sizeof(hmac));
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("auth failed: %x %x", oid_auth, res);
        return res;
    }
    return 0;
}

static int _reset_counter(uint16_t oid, uint32_t limit)
{
    // Configure the monotonic counter.
    // Table "Common data structures" -> "Counter":
    // https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/OPTIGA%E2%84%A2%20Trust%20M%20Solution%20Reference%20Manual.md#link24b48059_db81_40f5_8b65_7afca4918ab1
    // Bytes 0-3 are the initial counter value, set to 0.
    // Bytes 4-7 are the threshold.
    // Ints are encoded as uint32 big endian.
    uint8_t counter_buf[8] = {0};
    optiga_common_set_uint32(&counter_buf[4], limit);
    return optiga_ops_util_write_data_sync(
        _util, oid, OPTIGA_UTIL_ERASE_AND_WRITE, 0, counter_buf, sizeof(counter_buf));
}

#if APP_U2F == 1 || FACTORYSETUP == 1
static bool _read_arbitrary_data(arbitrary_data_t* data_out)
{
    memset(data_out->bytes, 0x00, sizeof(data_out->bytes));
    uint16_t len = sizeof(data_out->bytes);
    optiga_lib_status_t res =
        optiga_ops_util_read_data_sync(_util, OID_ARBITRARY_DATA, 0, data_out->bytes, &len);
    if (res != OPTIGA_UTIL_SUCCESS) {
        util_log("could not read arbitrary data: %x", res);
        return false;
    }
    if (len != sizeof(data_out->bytes)) {
        util_log(
            "arbitrary data: expected to read size %d, but read %d. Data read: %s",
            (int)sizeof(data_out->bytes),
            (int)len,
            util_dbg_hex(data_out->bytes, len));
        return false;
    }
    return true;
}
#endif

#if APP_U2F == 1 || FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1
static int _write_arbitrary_data(const arbitrary_data_t* data)
{
    optiga_lib_status_t res = optiga_ops_util_write_data_sync(
        _util,
        OID_ARBITRARY_DATA,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        &data->bytes[0],
        sizeof(data->bytes));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("could not write arbitrary %x", res);
    }
    return res;
}
#endif

#if VERIFY_METADATA == 1
// In a metadata object (0x20 <len> <tag> <tag len> <tag data> ...),
// extract tag data for a specific tag.
// Returns false if the metadata is invalid or the tag is not present, or if the tag data is larger
// than the `data_out` buffer as specified by `data_len_inout`.
// data_len_inout holds the buffer size as an in-param and the actual size as an out-param.
static bool _read_metadata_tag(
    const uint8_t* metadata,
    size_t metadata_len,
    uint8_t tag,
    uint8_t* data_out,
    size_t* data_len_inout)
{
    if (metadata_len < 2 || metadata[0] != 0x20) {
        // Metadata does not start with the expected tag or is too short
        return false;
    }

    uint8_t tlvs_len = metadata[1];
    if ((size_t)(tlvs_len + 2) > metadata_len) {
        // Malformed metadata: declared size exceeds buffer length
        return false;
    }

    const uint8_t* tlv_first = &metadata[2];

    size_t offset = 0;

    while (offset + 2 <= tlvs_len) { // Ensure at least <tag><size> are available
        uint8_t current_tag = tlv_first[offset];
        uint8_t size = tlv_first[offset + 1];

        if (offset + 2 + size > tlvs_len) {
            // Malformed TLV: size exceeds remaining length
            return false;
        }

        if (current_tag == tag) {
            // Found the tag, copy data to output
            if (size > *data_len_inout) {
                return false;
            }
            memcpy(data_out, &tlv_first[offset + 2], size);
            *data_len_inout = size;
            return true;
        }

        // Move to the next TLV
        offset += 2 + size;
    }

    // Tag not found
    return false;
}
#endif

#if FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1
// Read the LcsO status from a metadata object. Returns false if the metadata is invalid or LcsO is
// not present.
static bool _read_lcso(const uint8_t* metadata, size_t metadata_len, uint8_t* lcso_out)
{
    uint8_t tag_data[METADATA_MAX_SIZE] = {0};
    size_t tag_data_len = sizeof(tag_data);
    if (!_read_metadata_tag(metadata, metadata_len, TAG_LCSO, tag_data, &tag_data_len)) {
        return false;
    }
    if (tag_data_len != 1) {
        return false;
    }
    *lcso_out = tag_data[0];
    return true;
}

static int _read_lcso_of_object(uint16_t optiga_oid, uint8_t* lcso_out, bool unprotected)
{
    uint8_t metadata[METADATA_MAX_SIZE] = {0};
    uint16_t metadata_size = sizeof(metadata);

    if (unprotected) {
        // Is reset to full protection after the metadata read command.
        OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    }
    optiga_lib_status_t res =
        optiga_ops_util_read_metadata_sync(_util, optiga_oid, metadata, &metadata_size);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: read binding secret metadata: %x", res);
        return res;
    }
    if (!_read_lcso(metadata, metadata_size, lcso_out)) {
        return SC_OPTIGA_ERR_UNEXPECTED_METADATA;
    }
    return 0;
}

// Setup shielded communication.
// Writes the shared secret to the chip 0xE140 data object and sets the metadata.
// See solution reference manual 2.3.4 "Use case: Pair OPTIGA™ Trust M with host (pre-shared secret
// based)".
static int _setup_shielded_communication(void)
{
    const uint16_t oid = OID_PLATFORM_BINDING;
    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, true);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("shared secret already setup");
        return 0;
    }

    util_log("setting up shielded communication");

    uint8_t platform_binding_secret[32];
    uint16_t platform_binding_secret_size = sizeof(platform_binding_secret);

    pal_status_t pal_res = pal_os_datastore_read(
        OPTIGA_PLATFORM_BINDING_SHARED_SECRET_ID,
        platform_binding_secret,
        &platform_binding_secret_size);
    if (PAL_STATUS_SUCCESS != pal_res ||
        platform_binding_secret_size != sizeof(platform_binding_secret)) {
        util_log("failed datastore read: %x", pal_res);
        return SC_OPTIGA_ERR_PAL;
    }

    // We write the binding secret before updating the metadata, as the metadata update locks the
    // slot. Shielded communication is disabled as it is not set up yet and not required for
    // updating the platform binding object.
    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    res = optiga_ops_util_write_data_sync(
        _util,
        oid,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0,
        platform_binding_secret,
        sizeof(platform_binding_secret));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write binding secret to chip: %x", res);
        return res;
    }

    // Shielded communication is disabled as it is not set up yet and not required for updating the
    // platform binding object.
    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    res = optiga_ops_util_write_metadata_sync(
        _util, oid, _platform_binding_metadata, sizeof(_platform_binding_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: write metadata of platform binding: %x", res);
        return res;
    }

    return 0;
}

static int _configure_object_aes_symkey(void)
{
    const uint16_t oid = OID_AES_SYMKEY;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_aes_symkey: already setup");
        return 0;
    }
    util_log("_configure_object_aes_symkey: setting up");
    return optiga_ops_util_write_metadata_sync(
        _util, oid, _aes_symkey_metadata, sizeof(_aes_symkey_metadata));
}

static int _configure_object_hmac(void)
{
    const uint16_t oid = OID_HMAC;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_hmac: already setup");
        return 0;
    }
    util_log("_configure_object_hmac: setting up");
    return optiga_ops_util_write_metadata_sync(_util, oid, _hmac_metadata, sizeof(_hmac_metadata));
}

static int _configure_object_arbitrary_data(void)
{
    const uint16_t oid = OID_ARBITRARY_DATA;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_arbitrary_data: already setup");
        return OPTIGA_LIB_SUCCESS;
    }
    util_log("_configure_object_arbitrary_data: setting up");

    res = optiga_ops_util_write_metadata_sync(
        _util, oid, _arbitrary_data_metadata, sizeof(_arbitrary_data_metadata));
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    // Initialize arbitrary data, all zeroes.
    const arbitrary_data_t arbitrary_data = {0};
    int write_res = _write_arbitrary_data(&arbitrary_data);
    if (write_res != OPTIGA_LIB_SUCCESS) {
        util_log("could not initialize arbitrary data");
        return write_res;
    }
    return OPTIGA_LIB_SUCCESS;
}

static int _configure_object_counter(void)
{
    const uint16_t oid = OID_COUNTER;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_counter: already setup");
        return 0;
    }
    util_log("_configure_object_counter: setting up");

    _reset_counter(oid, MONOTONIC_COUNTER_MAX_USE);

    return optiga_ops_util_write_metadata_sync(
        _util, oid, _counter_metadata, sizeof(_counter_metadata));
}

static int _configure_object_attestation(void)
{
    const uint16_t oid = OID_ATTESTATION;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_attestation: already setup");
        return 0;
    }
    util_log("_configure_attestation: setting up");

    return optiga_ops_util_write_metadata_sync(
        _util, oid, _attestation_metadata, sizeof(_attestation_metadata));
}

static int _configure_object_password_secret(void)
{
    const uint16_t oid = OID_PASSWORD_SECRET;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_password_secret: already setup");
        return 0;
    }
    util_log("_configure_object_password_secret: setting up");
    return optiga_ops_util_write_metadata_sync(
        _util, oid, _password_secret_metadata, sizeof(_password_secret_metadata));
}

static int _configure_object_password(void)
{
    const uint16_t oid = OID_PASSWORD;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_password: already setup");
        return 0;
    }
    util_log("_configure_object_password: setting up");
    return optiga_ops_util_write_metadata_sync(
        _util, oid, _password_metadata, sizeof(_password_metadata));
}

static int _configure_object_counter_password(void)
{
    const uint16_t oid = OID_COUNTER_PASSWORD;

    uint8_t lcso = 0;
    optiga_lib_status_t res = _read_lcso_of_object(oid, &lcso, false);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (lcso >= LCSO_STATE_OPERATIONAL) {
        util_log("_configure_object_counter_password: already setup");
        return 0;
    }
    util_log("_configure_object_counter_password: setting up");

    return optiga_ops_util_write_metadata_sync(
        _util, oid, _counter_password_metadata, sizeof(_counter_password_metadata));
}

static int _factory_write_config(void)
{
    int res_shielded = _setup_shielded_communication();
    if (res_shielded) {
        return res_shielded;
    }

    optiga_lib_status_t res;

    res = _configure_object_aes_symkey();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_hmac();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_arbitrary_data();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_counter();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_attestation();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_password_secret();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_password();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    res = _configure_object_counter_password();
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    util_log("write config OK");

    return 0;
}

static int _factory_setup(void)
{
    optiga_lib_status_t res;

    res = optiga_ops_create(&_util, &_crypt);
    if (res) {
        return res;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(_util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(
        _crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    OPTIGA_UTIL_SET_COMMS_PROTECTION_LEVEL(_util, OPTIGA_COMMS_NO_PROTECTION);
    res = optiga_ops_util_open_application_sync(_util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("failed to open util application: %x", res);
        return res;
    }

    res = _factory_write_config();
    if (res) {
        return res;
    }

    res = optiga_ops_util_close_application_sync(_util, 0);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }

    if (NULL != _crypt) {
        optiga_crypt_destroy(_crypt);
        _crypt = NULL;
    }

    if (NULL != _util) {
        optiga_util_destroy(_util);
        _util = NULL;
    }

    return 0;
}
#endif // FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1

#if VERIFY_METADATA == 1
static int _verify_metadata(
    uint16_t oid,
    const uint8_t* expected_metadata,
    size_t expected_metadata_len,
    const uint8_t* check_tags,
    size_t check_tags_len)
{
    uint8_t actual_metadata[1000] = {0};
    uint16_t actual_metadata_len = sizeof(actual_metadata);

    optiga_lib_status_t res =
        optiga_ops_util_read_metadata_sync(_util, oid, actual_metadata, &actual_metadata_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("fail: read binding secret metadata: %x", res);
        return res;
    }

    uint8_t expected_tag_data[METADATA_MAX_SIZE] = {0};
    size_t expected_tag_data_len;
    uint8_t actual_tag_data[METADATA_MAX_SIZE] = {0};
    size_t actual_tag_data_len;

    for (size_t i = 0; i < check_tags_len; i++) {
        actual_tag_data_len = sizeof(actual_tag_data);
        expected_tag_data_len = sizeof(expected_tag_data);
        uint8_t tag = check_tags[i];
        if (!_read_metadata_tag(
                expected_metadata,
                expected_metadata_len,
                tag,
                expected_tag_data,
                &expected_tag_data_len)) {
            return SC_OPTIGA_ERR_UNEXPECTED_METADATA;
        }
        if (!_read_metadata_tag(
                actual_metadata, actual_metadata_len, tag, actual_tag_data, &actual_tag_data_len)) {
            return SC_OPTIGA_ERR_UNEXPECTED_METADATA;
        }
        if (actual_tag_data_len != expected_tag_data_len ||
            !MEMEQ(actual_tag_data, expected_tag_data, actual_tag_data_len)) {
            return SC_ERR_CONFIG_MISMATCH;
        }
    }
    return 0;
}
#endif

static int _set_password(
    const uint8_t* password_secret,
    size_t password_secret_len,
    const uint8_t* data,
    size_t data_len)
{
    optiga_lib_status_t res = _authorize(OID_PASSWORD_SECRET, password_secret, password_secret_len);
    if (res != OPTIGA_UTIL_SUCCESS) {
        goto cleanup;
    }

    res = optiga_ops_util_write_data_sync(
        _util, OID_PASSWORD, OPTIGA_UTIL_ERASE_AND_WRITE, 0x00, data, data_len);
    if (res != OPTIGA_UTIL_SUCCESS) {
        goto cleanup;
    }

    res = _reset_counter(OID_COUNTER_PASSWORD, SMALL_MONOTONIC_COUNTER_MAX_USE);
    if (res != OPTIGA_LIB_SUCCESS) {
        goto cleanup;
    }

cleanup: {
    optiga_lib_status_t res_clear =
        optiga_ops_crypt_clear_auto_state_sync(_crypt, OID_PASSWORD_SECRET);
    if (res != OPTIGA_UTIL_SUCCESS) {
        return res;
    }
    return res_clear;
}
}

int optiga_init_new_password(const char* password)
{
    // Set new hmac key.
    uint8_t new_hmac_key[32] = {0};
    _ifs->random_32_bytes(new_hmac_key);
    optiga_lib_status_t res = optiga_ops_util_write_data_sync(
        _util, OID_HMAC, OPTIGA_UTIL_ERASE_AND_WRITE, 0x00, new_hmac_key, sizeof(new_hmac_key));
    if (res != OPTIGA_UTIL_SUCCESS) {
        util_log("failed updating the hmac key: %x", res);
        return res;
    }

    // Set new symmetric key.
    optiga_key_id_t keyid = OPTIGA_KEY_ID_SECRET_BASED;
    res = optiga_ops_crypt_symmetric_generate_key_sync(
        _crypt, OPTIGA_SYMMETRIC_AES_256, OPTIGA_KEY_USAGE_ENCRYPTION, false, &keyid);
    if (res != OPTIGA_UTIL_SUCCESS) {
        util_log("failed updating the sym key: %x", res);
        return res;
    }

    uint8_t password_secret[32] = {0};
    _ifs->random_32_bytes(password_secret);

    res = optiga_ops_util_write_data_sync(
        _util,
        OID_PASSWORD_SECRET,
        OPTIGA_UTIL_ERASE_AND_WRITE,
        0x00,
        password_secret,
        sizeof(password_secret));
    if (res != OPTIGA_UTIL_SUCCESS) {
        return res;
    }

    uint8_t password_salted_hashed[32] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "optiga_password",
            password_salted_hashed)) {
        return SC_ERR_SALT;
    }

    res = _set_password(
        password_secret,
        sizeof(password_secret),
        password_salted_hashed,
        sizeof(password_salted_hashed));
    if (res) {
        return res;
    }
    return 0;
}

bool optiga_reset_keys(void)
{
    // This resets the OID_AES_SYMKEY and OID_HMAC keys, as well as the OID_PASSWORD_SECRET and
    // OID_PASSWORD keys. A password is needed because updating the OID_PASSWORD key requires
    // auth using the OID_PASSWORD_SECRET key, but any password is fine for the purpose of resetting
    // the keys.
    return optiga_init_new_password("") == 0;
}

static int _optiga_verify_password(const char* password, uint8_t* password_secret_out)
{
    uint8_t password_salted_hashed[32] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "optiga_password",
            password_salted_hashed)) {
        return SC_ERR_SALT;
    }

    optiga_lib_status_t res =
        _authorize(OID_PASSWORD, password_salted_hashed, sizeof(password_salted_hashed));
    if (res != OPTIGA_LIB_SUCCESS) {
        goto cleanup;
    }

    uint16_t password_secret_size = 32;
    res = optiga_ops_util_read_data_sync(
        _util, OID_PASSWORD_SECRET, 0, password_secret_out, &password_secret_size);
    if (res != OPTIGA_LIB_SUCCESS) {
        goto cleanup;
    }
    if (password_secret_size != 32) {
        res = SC_OPTIGA_ERR_UNEXPECTED_LEN;
        goto cleanup;
    }

    res = _authorize(OID_PASSWORD_SECRET, password_secret_out, password_secret_size);
    if (res != OPTIGA_LIB_SUCCESS) {
        goto cleanup;
    }

    res = _reset_counter(OID_COUNTER_PASSWORD, SMALL_MONOTONIC_COUNTER_MAX_USE);
    if (res != OPTIGA_LIB_SUCCESS) {
        goto cleanup;
    }

cleanup: {
    optiga_lib_status_t res_clear1 = optiga_ops_crypt_clear_auto_state_sync(_crypt, OID_PASSWORD);
    optiga_lib_status_t res_clear2 =
        optiga_ops_crypt_clear_auto_state_sync(_crypt, OID_PASSWORD_SECRET);
    if (res != OPTIGA_UTIL_SUCCESS) {
        return res;
    }
    if (res_clear1) {
        return res_clear1;
    }
    return res_clear2;
}
}

#if VERIFY_METADATA == 1
static int _verify_metadata_config(void)
{
    int res;

    // Verify metadata tags are setup as expected.
    {
        const uint8_t check_tags[] = {0xC0, 0xD0, 0xD1, 0xD3, 0xE8};
        res = _verify_metadata(
            OID_PLATFORM_BINDING,
            _platform_binding_metadata,
            sizeof(_platform_binding_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (platform_binding): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xE1, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_AES_SYMKEY,
            _aes_symkey_metadata,
            sizeof(_aes_symkey_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (aes_symkey): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xE8, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_HMAC, _hmac_metadata, sizeof(_hmac_metadata), check_tags, sizeof(check_tags));
        if (res) {
            util_log("verify config failed (hmac): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xE8, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_ARBITRARY_DATA,
            _arbitrary_data_metadata,
            sizeof(_arbitrary_data_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (arbitrary_data): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xE1, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_ATTESTATION,
            _attestation_metadata,
            sizeof(_attestation_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (attestation): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_COUNTER,
            _counter_metadata,
            sizeof(_counter_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (counter): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xD0, 0xD1, 0xD3, 0xE8};
        res = _verify_metadata(
            OID_PASSWORD_SECRET,
            _password_secret_metadata,
            sizeof(_password_secret_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (password_secret): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xD0, 0xD1, 0xD3, 0xE8};
        res = _verify_metadata(
            OID_PASSWORD,
            _password_metadata,
            sizeof(_password_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (password): %i", res);
            return res;
        }
    }
    {
        const uint8_t check_tags[] = {0xC0, 0xD0, 0xD1, 0xD3};
        res = _verify_metadata(
            OID_COUNTER_PASSWORD,
            _counter_password_metadata,
            sizeof(_counter_password_metadata),
            check_tags,
            sizeof(check_tags));
        if (res) {
            util_log("verify config failed (counter_password): %i", res);
            return res;
        }
    }
    return 0;
}
#endif

int optiga_setup(const securechip_interface_functions_t* ifs)
{
    if (ifs == NULL) {
        return SC_ERR_IFS;
    }
    _ifs = ifs;

    util_log("optiga_setup");

    // A timer is used to provide the OPTIGA library with the ability to schedule work on the main
    // event loop
    pal_timer_init();

    int res;

#if FACTORYSETUP == 1 || FACTORY_DURING_PROD == 1
    res = _factory_setup();
    if (res) {
        util_log("factory setup failed");
        return res;
    }
#endif

    res = optiga_ops_create(&_util, &_crypt);
    if (res) {
        return res;
    }

    OPTIGA_UTIL_SET_COMMS_PROTOCOL_VERSION(_util, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);
    OPTIGA_CRYPT_SET_COMMS_PROTOCOL_VERSION(
        _crypt, OPTIGA_COMMS_PROTOCOL_VERSION_PRE_SHARED_SECRET);

    // Verify shielded connection is active.
    if (_crypt->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("crypt protection level expected to be FULL");
        return SC_ERR_CONFIG_MISMATCH;
    }
    if (_util->protection_level != OPTIGA_COMMS_FULL_PROTECTION) {
        util_log("util protection level expected to be FULL");
        return SC_ERR_CONFIG_MISMATCH;
    }

    res = optiga_ops_util_open_application_sync(_util, 0);
    if (res) {
        return res;
    }

#if VERIFY_METADATA == 1
    res = _verify_metadata_config();
    if (res) {
        return res;
    }
#endif

    return 0;
}

int optiga_kdf_external(const uint8_t* msg, size_t len, uint8_t* mac_out)
{
    if (len != 32) {
        return SC_ERR_INVALID_ARGS;
    }

    optiga_lib_status_t res;
    // The equivalient of python `mac_out = hmac.new(key, msg[:len], hashlib.sha256).digest()`

    uint32_t mac_out_len = 32;

    res = optiga_ops_crypt_hmac_sync(
        _crypt, OPTIGA_HMAC_SHA_256, OID_HMAC, msg, len, mac_out, &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        util_log("kdf fail err=%x", res);
        return res;
    }
    if (mac_out_len != 32) {
        return SC_OPTIGA_ERR_UNEXPECTED_LEN;
    }

    return 0;
}

static int _kdf_internal(const uint8_t* msg, size_t len, uint8_t* kdf_out)
{
    if (len != 32) {
        return SC_ERR_INVALID_ARGS;
    }
    optiga_lib_status_t res;

    uint8_t mac_out[16] = {0};
    uint32_t mac_out_len = sizeof(mac_out);

    res = optiga_ops_crypt_symmetric_encrypt_sync(
        _crypt,
        OPTIGA_SYMMETRIC_CMAC,
        OID_AES_SYMKEY,
        msg,
        len,
        NULL,
        0,
        NULL,
        0,
        mac_out,
        &mac_out_len);
    if (res != OPTIGA_LIB_SUCCESS) {
        return res;
    }
    if (mac_out_len != sizeof(mac_out)) {
        return SC_OPTIGA_ERR_UNEXPECTED_LEN;
    }
    rust_sha256(mac_out, mac_out_len, kdf_out);
    return 0;
}

int optiga_stretch_password(const char* password, uint8_t* stretched_out)
{
    uint8_t password_salted_hashed[32] = {0};
    UTIL_CLEANUP_32(password_salted_hashed);
    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "optiga_password_stretch_in",
            password_salted_hashed)) {
        return SC_ERR_SALT;
    }

    uint8_t kdf_in[32] = {0};
    UTIL_CLEANUP_32(kdf_in);
    memcpy(kdf_in, password_salted_hashed, 32);

    // First KDF on internal key increments the monotonic counter. Call only once!
    int securechip_result = _kdf_internal(kdf_in, 32, stretched_out);
    if (securechip_result) {
        return securechip_result;
    }
    // Second KDF does not use the counter and we call it multiple times.
    for (int i = 0; i < KDF_NUM_ITERATIONS; i++) {
        memcpy(kdf_in, stretched_out, 32);
        securechip_result = optiga_kdf_external(kdf_in, 32, stretched_out);
        if (securechip_result) {
            return securechip_result;
        }
    }

    // Verify password incrementing the small monotonic counter.
    // We do this after the above KDF stretch so the big monotonic counter is also incremented.
    uint8_t password_secret[32] = {0};
    int res = _optiga_verify_password(password, password_secret);
    if (res) {
        if (res == 0x802F) {
            return SC_ERR_INCORRECT_PASSWORD;
        }
        return res;
    }

    rust_hmac_sha256(password_secret, sizeof(password_secret), stretched_out, 32, stretched_out);

    if (!salt_hash_data(
            (const uint8_t*)password,
            strlen(password),
            "optiga_password_stretch_out",
            password_salted_hashed)) {
        return SC_ERR_SALT;
    }
    rust_hmac_sha256(
        password_salted_hashed, sizeof(password_salted_hashed), stretched_out, 32, stretched_out);
    return 0;
}

bool optiga_gen_attestation_key(uint8_t* pubkey_out)
{
    optiga_key_id_t slot = OPTIGA_KEY_ID_E0F1;
    uint8_t pubkey_der[68] = {0};
    uint16_t pubkey_der_size = sizeof(pubkey_der);
    optiga_lib_status_t res = optiga_ops_crypt_ecc_generate_keypair_sync(
        _crypt,
        OPTIGA_ECC_CURVE_NIST_P_256,
        OPTIGA_KEY_USAGE_SIGN,
        false,
        (void*)&slot,
        pubkey_der,
        &pubkey_der_size);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("gen keypair failed: %x", res);
        return false;
    }
    // Parse DER "BIT STRING", see Solution Reference Manual 6.2.2,
    // example for ECC NIST-P256.
    // The 64 byte X/Y values are the last 64 bytes.
    if (pubkey_der_size != 68 || !MEMEQ(pubkey_der, "\x03\x42\x00\x04", 4)) {
        return false;
    }
    memcpy(pubkey_out, pubkey_der + 4, 64);
    return true;
}

bool optiga_attestation_sign(const uint8_t* challenge, uint8_t* signature_out)
{
    uint8_t sig_der[70] = {0};
    uint16_t sig_der_size = sizeof(sig_der);
    optiga_lib_status_t res = optiga_ops_crypt_ecdsa_sign_sync(
        _crypt, challenge, 32, OPTIGA_KEY_ID_E0F1, sig_der, &sig_der_size);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("sign failed: %x", res);
        return false;
    }
    // Parse signature, see Solution Reference Manual 6.2.2,
    // example for ECC NIST-P256 signature.
    // The R/S components are
    return rust_der_parse_optiga_signature(
        rust_util_bytes(sig_der, sig_der_size), rust_util_bytes_mut(signature_out, 64));
}

bool optiga_monotonic_increments_remaining(uint32_t* remaining_out)
{
    uint8_t buf[4] = {0};
    uint16_t size = sizeof(buf);
    optiga_lib_status_t res = optiga_ops_util_read_data_sync(_util, OID_COUNTER, 0, buf, &size);
    if (res != OPTIGA_LIB_SUCCESS) {
        return false;
    }

    uint32_t counter = optiga_common_get_uint32(buf);
    if (counter > MONOTONIC_COUNTER_MAX_USE) {
        Abort("optiga monotonic counter larget than max");
    }
    *remaining_out = MONOTONIC_COUNTER_MAX_USE - counter;
    return true;
}

// rand_out must be 32 bytes
bool optiga_random(uint8_t* rand_out)
{
    optiga_lib_status_t res =
        optiga_ops_crypt_random_sync(_crypt, OPTIGA_RNG_TYPE_TRNG, rand_out, 32);
    if (res != OPTIGA_CRYPT_SUCCESS) {
        util_log("optiga_random failed: %x", res);
        return false;
    }
    return true;
}

#if APP_U2F == 1 || FACTORYSETUP == 1
bool optiga_u2f_counter_set(uint32_t counter)
{
    arbitrary_data_t data = {0};
    if (!_read_arbitrary_data(&data)) {
        return false;
    }
    data.fields.u2f_counter = counter;
    return _write_arbitrary_data(&data) == OPTIGA_LIB_SUCCESS;
}
#endif

#if APP_U2F == 1
bool optiga_u2f_counter_inc(uint32_t* counter)
{
    arbitrary_data_t data = {0};
    if (!_read_arbitrary_data(&data)) {
        return false;
    }
    data.fields.u2f_counter += 1;
    *counter = data.fields.u2f_counter;

    return _write_arbitrary_data(&data) == OPTIGA_LIB_SUCCESS;
}
#endif

bool optiga_model(securechip_model_t* model_out)
{
    *model_out = OPTIGA_TRUST_M_V3;
    return true;
}
