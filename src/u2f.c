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
#include "u2f.h"
#include "u2f/u2f_app.h"

#include <stdio.h>
#include <string.h>

#include <keystore.h>
#include <memory.h>
#include <random.h>
#include <securechip/securechip.h>
#include <ui/component.h>
#include <ui/components/confirm.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <usb/u2f/u2f.h>
#include <usb/u2f/u2f_hid.h>
#include <usb/u2f/u2f_keys.h>
#include <usb/usb_packet.h>
#include <usb/usb_processing.h>
#include <wally_crypto.h>
#include <workflow/confirm.h>
#include <workflow/status.h>
#include <workflow/unlock.h>

#define U2F_HIJACK_ORIGIN_TOTAL 4
#define APPID_BOGUS_CHROMIUM "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
#define APPID_BOGUS_FIREFOX "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"

typedef struct {
    uint8_t cla, ins, p1, p2;
    uint8_t lc1, lc2, lc3;
    uint8_t data[];
} USB_APDU;

#define APDU_LEN(A) (uint32_t)(((A).lc1 << 16) + ((A).lc2 << 8) + ((A).lc3))
#define U2F_TIMEOUT 500 // [msec]
#define U2F_KEYHANDLE_LEN (U2F_NONCE_LENGTH + SHA256_LEN)

#if (U2F_EC_KEY_SIZE != SHA256_LEN) || (U2F_EC_KEY_SIZE != U2F_NONCE_LENGTH)
#error "Incorrect macro values for u2f"
#endif

static uint32_t _cid = 0;
// TODO: Implement hijack
static const uint8_t _hijack_code[U2F_HIJACK_ORIGIN_TOTAL][U2F_APPID_SIZE] = {
    {
        /* Corresponds to U2F client challenge filled with `0xdb` */
        /* Origin `https://digitalbitbox.com` */
        0x17, 0x9d, 0xc3, 0x1c, 0x3a, 0xd4, 0x0f, 0x05, 0xf0, 0x71, 0x71,
        0xed, 0xf4, 0x46, 0x4a, 0x71, 0x0a, 0x2d, 0xd4, 0xde, 0xc7, 0xe6,
        0x14, 0x41, 0xc5, 0xbd, 0x24, 0x97, 0x8a, 0x99, 0x2a, 0x1a,
    },
    {
        /* Origin `https://www.myetherwallet.com` */
        0x8e, 0x57, 0xf6, 0x48, 0xb9, 0x1b, 0x24, 0xfe, 0x27, 0x92, 0x3a,
        0x75, 0xef, 0xa1, 0xd0, 0x62, 0xdc, 0xb5, 0x4d, 0x41, 0xfd, 0x0b,
        0xee, 0x33, 0x9e, 0xf2, 0xa2, 0xb4, 0x55, 0x0c, 0xbe, 0x05,
    },
    {
        /* Origin `https://vintage.myetherwallet.com` */
        0x0f, 0x5b, 0x76, 0xef, 0x29, 0x8f, 0x15, 0x0b, 0x4d, 0x39, 0x9d,
        0x2c, 0x3c, 0xb9, 0x0e, 0x86, 0x54, 0xa3, 0x7c, 0x60, 0x5f, 0x73,
        0x35, 0x68, 0xee, 0x68, 0xec, 0x41, 0x48, 0x8d, 0x53, 0x14,
    },
    {
        /* Origin `https://mycrypto.com` */
        0xbd, 0x22, 0x66, 0x24, 0x02, 0x18, 0x8c, 0x4d, 0xba, 0x4b, 0xb3,
        0xd7, 0xe3, 0x98, 0x00, 0x7c, 0x5b, 0x98, 0x6f, 0x46, 0x27, 0x1f,
        0x6d, 0xf9, 0x2e, 0x24, 0x01, 0xa7, 0xce, 0xfd, 0x1a, 0xa8,
    }};

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
#pragma GCC diagnostic ignored "-Wattributes"

typedef struct __attribute__((__packed__)) {
    uint8_t reserved;
    uint8_t appId[U2F_APPID_SIZE];
    uint8_t challenge[U2F_NONCE_LENGTH];
    uint8_t keyHandle[U2F_KEYHANDLE_LEN];
    uint8_t pubKey[U2F_EC_POINT_SIZE];
} U2F_REGISTER_SIG_STR;

typedef struct __attribute__((__packed__)) {
    uint8_t appId[U2F_APPID_SIZE];
    uint8_t flags;
    uint8_t ctr[4];
    uint8_t challenge[U2F_NONCE_LENGTH];
} U2F_AUTHENTICATE_SIG_STR;

#pragma GCC diagnostic pop

static uint32_t _next_cid(void)
{
    do {
        _cid = (random_byte_mcu() << 0) + (random_byte_mcu() << 8) + (random_byte_mcu() << 16) +
               (random_byte_mcu() << 24);
    } while (_cid == 0 || _cid == U2FHID_CID_BROADCAST);
    return _cid;
}

static void _fill_message(const uint8_t* data, const uint32_t len, Packet* out_packet)
{
    util_zero(out_packet->data_addr, sizeof(out_packet->data_addr));
    memcpy(out_packet->data_addr, data, len);
    out_packet->cid = _cid;
    out_packet->cmd = U2FHID_MSG;
    out_packet->len = len;
}

static void _error_hid(uint32_t fcid, uint8_t err, Packet* out_packet)
{
    util_zero(out_packet->data_addr, sizeof(out_packet->data_addr));
    out_packet->data_addr[0] = err;
    out_packet->cid = fcid;
    out_packet->cmd = U2FHID_ERROR;
    out_packet->len = 1;
}

static void _error(const uint16_t err, Packet* out_packet)
{
    uint8_t data[2];
    data[0] = err >> 8 & 0xFF;
    data[1] = err & 0xFF;
    _fill_message(data, sizeof(data), out_packet);
}

static void _version(const USB_APDU* a, Packet* out_packet)
{
    if (APDU_LEN(*a) != 0) {
        _error(U2F_SW_WRONG_LENGTH, out_packet);
        return;
    }

    static const uint8_t version_response[] = {'U', '2', 'F', '_', 'V', '2', 0x90, 0x00};
    _fill_message(version_response, sizeof(version_response), out_packet);
}

/**
 * Generates a key for the given app id, salted with the passed nonce.
 * @param[in] appId The app id of the RP which requests a registration or authentication process.
 * @param[in] nonce A random nonce with which the seed for the private key is salted.
 * @param[out] privkey The generated private key. Size must be HMAC_SHA256_LEN.
 * @param[out] mac The message authentication code for the private key. Size must be
 * HMAC_SHA256_LEN.
 */
USE_RESULT static bool _keyhandle_gen(
    const uint8_t* appId,
    uint8_t* nonce,
    uint8_t* privkey,
    uint8_t* mac)
{
    uint8_t hmac_in[U2F_APPID_SIZE + U2F_NONCE_LENGTH];
    uint8_t seed[32];
    UTIL_CLEANUP_32(seed);
    if (!keystore_get_u2f_seed(seed)) {
        return false;
    }

    // Concatenate AppId and Nonce as input for the first HMAC round
    memcpy(hmac_in, appId, U2F_APPID_SIZE);
    memcpy(hmac_in + U2F_APPID_SIZE, nonce, U2F_NONCE_LENGTH);
    int res = wally_hmac_sha256(
        seed, KEYSTORE_U2F_SEED_LENGTH, hmac_in, sizeof(hmac_in), privkey, HMAC_SHA256_LEN);
    if (res != WALLY_OK) {
        return false;
    }

    // Concatenate AppId and privkey for the second HMAC round
    memcpy(hmac_in + U2F_APPID_SIZE, privkey, HMAC_SHA256_LEN);
    res = wally_hmac_sha256(
        seed, KEYSTORE_U2F_SEED_LENGTH, hmac_in, sizeof(hmac_in), mac, HMAC_SHA256_LEN);
    if (res != WALLY_OK) {
        return false;
    }
    return true;
}

static int _sig_to_der(const uint8_t* sig, uint8_t* der)
{
    int i;
    uint8_t *p = der, *len, *len1, *len2;
    *p = 0x30;
    p++; // sequence
    *p = 0x00;
    len = p;
    p++; // len(sequence)

    *p = 0x02;
    p++; // integer
    *p = 0x00;
    len1 = p;
    p++; // len(integer)

    // process R
    i = 0;
    while (sig[i] == 0 && i < 32) {
        i++; // skip leading zeroes
    }
    if (sig[i] >= 0x80) { // put zero in output if MSB set
        *p = 0x00;
        p++;
        *len1 = *len1 + 1;
    }
    while (i < 32) { // copy bytes to output
        *p = sig[i];
        p++;
        *len1 = *len1 + 1;
        i++;
    }

    *p = 0x02;
    p++; // integer
    *p = 0x00;
    len2 = p;
    p++; // len(integer)

    // process S
    i = 32;
    while (sig[i] == 0 && i < 64) {
        i++; // skip leading zeroes
    }
    if (sig[i] >= 0x80) { // put zero in output if MSB set
        *p = 0x00;
        p++;
        *len2 = *len2 + 1;
    }
    while (i < 64) { // copy bytes to output
        *p = sig[i];
        p++;
        *len2 = *len2 + 1;
        i++;
    }

    *len = *len1 + *len2 + 4;
    return *len + 2;
}

static bool _bogus_confirmation(void)
{
    return workflow_confirm_with_timeout("", "Use U2F?", false, 1000);
}

/**
 * Initiates the U2F registration workflow.
 * @param[in] apdu The APDU packet.
 */
static void _register(const USB_APDU* apdu, Packet* out_packet)
{
    uint8_t privkey[U2F_EC_KEY_SIZE] = {0};
    uint8_t nonce[U2F_NONCE_LENGTH] = {0};
    uint8_t mac[HMAC_SHA256_LEN] = {0};
    uint8_t data[sizeof(U2F_REGISTER_RESP) + 2] = {0};
    uint8_t sig[64] = {0};
    U2F_REGISTER_SIG_STR sig_base;
    U2F_REGISTER_RESP* response = (U2F_REGISTER_RESP*)data;
    const U2F_REGISTER_REQ* reg_request = (const U2F_REGISTER_REQ*)apdu->data;

    if (APDU_LEN(*apdu) != sizeof(U2F_REGISTER_REQ)) {
        _error(U2F_SW_WRONG_LENGTH, out_packet);
        return;
    }

    // If the authentication fails with the "Bad key handle" the browser will execute bogus
    // registrations to make the device blink.
    bool is_bogus = MEMEQ(reg_request->appId, APPID_BOGUS_CHROMIUM, U2F_APPID_SIZE) ||
                    MEMEQ(reg_request->appId, APPID_BOGUS_FIREFOX, U2F_APPID_SIZE);
    if (is_bogus) {
        if (!_bogus_confirmation()) {
            _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
            return;
        }
    } else if (!u2f_app_confirm(U2F_APP_REGISTER, reg_request->appId)) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    if (!workflow_unlock()) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    // Generate keys until a valid one is found
    int i = 0;
    for (;; ++i) {
        if (i == 10) {
            _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
            return;
        }
        random_32_bytes(nonce);
        if (!_keyhandle_gen(reg_request->appId, nonce, privkey, mac)) {
            continue;
        }
        if (securechip_ecc_generate_public_key(privkey, (uint8_t*)&response->pubKey.x)) {
            break;
        }
    }

    response->pubKey.format = U2F_UNCOMPRESSED_POINT;

    response->registerId = U2F_REGISTER_ID;
    response->keyHandleLen = U2F_KEYHANDLE_LEN;

    memcpy(response->keyHandleCertSig, mac, sizeof(mac));
    memcpy(response->keyHandleCertSig + sizeof(mac), nonce, sizeof(nonce));
    memcpy(response->keyHandleCertSig + response->keyHandleLen, U2F_ATT_CERT, sizeof(U2F_ATT_CERT));

    // Add signature using attestation key
    sig_base.reserved = 0;
    memcpy(sig_base.appId, reg_request->appId, U2F_APPID_SIZE);
    memcpy(sig_base.challenge, reg_request->challenge, U2F_NONCE_LENGTH);
    memcpy(sig_base.keyHandle, &response->keyHandleCertSig, U2F_KEYHANDLE_LEN);
    memcpy(sig_base.pubKey, &response->pubKey, U2F_EC_POINT_SIZE);

    uint8_t hash[SHA256_LEN] = {0};
    wally_sha256((uint8_t*)&sig_base, sizeof(sig_base), hash, SHA256_LEN);

    if (!securechip_ecc_unsafe_sign(U2F_ATT_PRIV_KEY, hash, sig)) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    uint8_t* resp_sig = response->keyHandleCertSig + response->keyHandleLen + sizeof(U2F_ATT_CERT);
    int der_len = _sig_to_der(sig, resp_sig);
    size_t kh_cert_sig_len = response->keyHandleLen + sizeof(U2F_ATT_CERT) + der_len;

    // Append success bytes
    memcpy(response->keyHandleCertSig + kh_cert_sig_len, "\x90\x00", 2);

    size_t len =
        1 /* registerId */ + U2F_EC_POINT_SIZE + 1 /* keyhandleLen */ + kh_cert_sig_len + 2;
    _fill_message(data, len, out_packet);
}

static void _hijack(const U2F_AUTHENTICATE_REQ* req, Packet* out_packet)
{
    // TODO - copy from v1 once finalized
    (void)req;
    (void)out_packet;
}

static void _authenticate(const USB_APDU* apdu, Packet* out_packet)
{
    uint8_t privkey[U2F_EC_KEY_SIZE];
    uint8_t nonce[U2F_NONCE_LENGTH];
    uint8_t mac[HMAC_SHA256_LEN];
    uint8_t sig[64] = {0};
    U2F_AUTHENTICATE_SIG_STR sig_base;

    const U2F_AUTHENTICATE_REQ* auth_request = (const U2F_AUTHENTICATE_REQ*)apdu->data;

    if (APDU_LEN(*apdu) < U2F_KEYHANDLE_LEN) { // actual size could vary
        _error(U2F_SW_WRONG_LENGTH, out_packet);
        return;
    }

    if (!workflow_unlock()) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    for (uint8_t i = 0; i < U2F_HIJACK_ORIGIN_TOTAL; i++) {
        // As an alternative interface, hijack the U2F AUTH key handle data field.
        // Slower but works in browsers for specified sites without requiring an extension.
        if (MEMEQ(auth_request->appId, _hijack_code[i], U2F_APPID_SIZE)) {
            _hijack(auth_request, out_packet);
            return;
        }
    }
    memcpy(nonce, auth_request->keyHandle + sizeof(mac), sizeof(nonce));
    if (!_keyhandle_gen(auth_request->appId, nonce, privkey, mac)) {
        _error(U2F_SW_WRONG_DATA, out_packet);
        return;
    }
    if (!MEMEQ(auth_request->keyHandle, mac, SHA256_LEN)) {
        _error(U2F_SW_WRONG_DATA, out_packet);
        return;
    }
    if (apdu->p1 == U2F_AUTH_CHECK_ONLY) {
        // success: "error:test-of-user-presense"
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }
    if (apdu->p1 != U2F_AUTH_ENFORCE) {
        _error(U2F_SW_INS_NOT_SUPPORTED, out_packet);
        return;
    }

    if (!u2f_app_confirm(U2F_APP_AUTHENTICATE, auth_request->appId)) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    uint8_t buf[sizeof(U2F_AUTHENTICATE_RESP) + 2] = {0};
    U2F_AUTHENTICATE_RESP* response = (U2F_AUTHENTICATE_RESP*)&buf;

    uint32_t counter;
    if (!securechip_u2f_counter_inc(&counter)) {
        _error(U2F_SW_CONDITIONS_NOT_SATISFIED, out_packet);
        return;
    }

    response->flags = U2F_AUTH_FLAG_TUP;
    response->ctr[0] = (counter >> 24) & 0xff;
    response->ctr[1] = (counter >> 16) & 0xff;
    response->ctr[2] = (counter >> 8) & 0xff;
    response->ctr[3] = counter & 0xff;

    // Sign
    memset(&sig_base, 0, sizeof(sig_base));
    memcpy(sig_base.appId, auth_request->appId, U2F_APPID_SIZE);
    sig_base.flags = response->flags;
    memcpy(sig_base.ctr, response->ctr, 4);
    memcpy(sig_base.challenge, auth_request->challenge, U2F_NONCE_LENGTH);

    uint8_t hash[SHA256_LEN] = {0};
    wally_sha256((uint8_t*)&sig_base, sizeof(sig_base), hash, SHA256_LEN);

    if (!securechip_ecc_unsafe_sign(privkey, hash, sig)) {
        _error(U2F_SW_WRONG_DATA, out_packet);
        return;
    }

    int der_len = _sig_to_der(sig, response->sig);
    size_t auth_packet_len = sizeof(U2F_AUTHENTICATE_RESP) - U2F_MAX_EC_SIG_SIZE + der_len;

    // Append success bytes
    memcpy(buf + auth_packet_len, "\x90\x00", 2);

    _fill_message(buf, auth_packet_len + 2, out_packet);
}

static void _cmd_ping(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    (void)max_out_len;

    // 0 and broadcast are reserved
    if (in_packet->cid == U2FHID_CID_BROADCAST || in_packet->cid == 0) {
        _error_hid(in_packet->cid, U2FHID_ERR_INVALID_CID, out_packet);
        return;
    }

    util_zero(out_packet->data_addr, sizeof(out_packet->data_addr));
    size_t max = MIN(in_packet->len, USB_DATA_MAX_LEN);
    memcpy(out_packet->data_addr, in_packet->data_addr, max);
    out_packet->len = in_packet->len;
    out_packet->cmd = U2FHID_PING;
    out_packet->cid = in_packet->cid;
}

static void _cmd_wink(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    (void)max_out_len;

    // 0 and broadcast are reserved
    if (in_packet->cid == U2FHID_CID_BROADCAST || in_packet->cid == 0) {
        _error_hid(in_packet->cid, U2FHID_ERR_INVALID_CID, out_packet);
        return;
    }

    if (in_packet->len > 0) {
        _error_hid(in_packet->cid, U2FHID_ERR_INVALID_LEN, out_packet);
        return;
    }

    workflow_status_create("U2F Wink", true);

    util_zero(out_packet->data_addr, sizeof(out_packet->data_addr));
    out_packet->len = 0;
    out_packet->cmd = U2FHID_WINK;
    out_packet->cid = in_packet->cid;
}

/**
 * Synchronize a channel and optionally requests a unique 32-bit channel identifier (CID)
 * that can be used by the requesting application during its lifetime.
 *
 * If the CID is the U2FHID_CID_BROADCAST then the application is requesting a CID from the device.
 * Otherwise the application has chosen the CID.
 */
static void _cmd_init(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    if (U2FHID_INIT_RESP_SIZE >= max_out_len) {
        _error_hid(in_packet->cid, U2FHID_ERR_OTHER, out_packet);
        return;
    }

    // Channel 0 is reserved
    if (in_packet->cid == 0) {
        _error_hid(in_packet->cid, U2FHID_ERR_INVALID_CID, out_packet);
        return;
    }

    const U2FHID_INIT_REQ* init_req = (const U2FHID_INIT_REQ*)&in_packet->data_addr;
    U2FHID_INIT_RESP response;

    out_packet->cid = in_packet->cid;
    out_packet->cmd = U2FHID_INIT;
    out_packet->len = U2FHID_INIT_RESP_SIZE;

    util_zero(&response, sizeof(response));
    memcpy(response.nonce, init_req->nonce, sizeof(init_req->nonce));
    response.cid = in_packet->cid == U2FHID_CID_BROADCAST ? _next_cid() : in_packet->cid;
    response.versionInterface = U2FHID_IF_VERSION;
    response.versionMajor = DIGITAL_BITBOX_VERSION_MAJOR;
    response.versionMinor = DIGITAL_BITBOX_VERSION_MINOR;
    response.versionBuild = DIGITAL_BITBOX_VERSION_PATCH;
    response.capFlags = U2FHID_CAPFLAG_WINK;
    util_zero(out_packet->data_addr, sizeof(out_packet->data_addr));
    memcpy(out_packet->data_addr, &response, sizeof(response));
}

/**
 * Process a U2F message
 */
static void _cmd_msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    (void)max_out_len;
    // By default always use the recieved cid
    _cid = in_packet->cid;

    const USB_APDU* apdu = (const USB_APDU*)in_packet->data_addr;

    if ((APDU_LEN(*apdu) + sizeof(USB_APDU)) > in_packet->len) {
        return;
    }

    if (apdu->cla != 0) {
        _error(U2F_SW_CLA_NOT_SUPPORTED, out_packet);
        return;
    }

    switch (apdu->ins) {
    case U2F_REGISTER:
        _register(apdu, out_packet);
        break;
    case U2F_AUTHENTICATE:
        _authenticate(apdu, out_packet);
        break;
    case U2F_VERSION:
        _version(apdu, out_packet);
        break;
    default:
        _error(U2F_SW_INS_NOT_SUPPORTED, out_packet);
        return;
    }
}

/**
 * Set up the U2F commands.
 */
void u2f_device_setup(void)
{
    const CMD_Callback u2f_cmd_callbacks[] = {
        {U2FHID_PING, _cmd_ping},
        {U2FHID_WINK, _cmd_wink},
        {U2FHID_INIT, _cmd_init},
        {U2FHID_MSG, _cmd_msg},
    };
    usb_processing_register_cmds(
        u2f_cmd_callbacks, sizeof(u2f_cmd_callbacks) / sizeof(CMD_Callback));
}
