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

// TODO: possibly rename to only u2f.c?!

#include <string.h>

#include "queue.h"
#include "u2f.h"
#include "usb/usb_packet.h"
#include "usb/usb_processing.h"
#include "util.h"

#include "usb/u2f/u2f_hid.h"

// typedef struct {
//    uint8_t cla, ins, p1, p2;
//    uint8_t lc1, lc2, lc3;
//    uint8_t data[];
//} USB_APDU;

//#include "bip32.h"
//#include "touch.h"
//#include "ecc.h"
//#include "usb.h"
//#include "sha2.h"
//#include "hmac.h"
//#include "flags.h"
//#include "utils.h"
//#include "screen.h"
//#include "memory.h"
//#include "wallet.h"
//#include "random.h"
//#include "version.h"
//#include "systick.h"
//#include "commander.h"
//
//#include "u2f/u2f.h"
//#include "u2f/u2f_keys.h"
//
//#define APDU_LEN(A)              (uint32_t)(((A).lc1 << 16) + ((A).lc2 << 8) + ((A).lc3))
#define U2F_TIMEOUT 500 // [msec]
//#define U2F_KEYHANDLE_LEN        (U2F_NONCE_LENGTH + SHA256_DIGEST_LENGTH)
//
//#if (U2F_EC_KEY_SIZE != SHA256_DIGEST_LENGTH) || (U2F_EC_KEY_SIZE != U2F_NONCE_LENGTH)
//#error "Incorrect macro values for u2f_device"
//#endif
//
//
// static uint32_t cid = 0;
// const uint8_t U2F_HIJACK_CODE[U2F_HIJACK_ORIGIN_TOTAL][U2F_NONCE_LENGTH] = {
//    {
//        /* Corresponds to U2F client challenge filled with `0xdb` */
//        /* Origin `https://digitalbitbox.com` */
//        57,  55, 173, 209, 178, 255, 144, 175,
//        24, 190, 240, 197, 183,  84,  22, 170,
//        58, 118, 133,  98, 243, 145, 238, 136,
//        137, 134, 248,  90, 247, 202, 114, 148
//    }, {
//        /* Origin `https://www.myetherwallet.com` */
//        240,  97, 125, 208,  85, 124, 251, 127,
//        247, 228, 158, 226, 243,  43,  46,  47,
//        60, 196, 229, 129, 113, 218, 237, 220,
//        200, 151, 111, 248,  63, 168, 101,  51
//    }
//};
//
// typedef struct {
//    uint8_t reserved;
//    uint8_t appId[U2F_APPID_SIZE];
//    uint8_t challenge[U2F_NONCE_LENGTH];
//    uint8_t keyHandle[U2F_KEYHANDLE_LEN];
//    uint8_t pubKey[U2F_EC_POINT_SIZE];
//} U2F_REGISTER_SIG_STR;
//
//
// typedef struct {
//    uint8_t appId[U2F_APPID_SIZE];
//    uint8_t flags;
//    uint8_t ctr[4];
//    uint8_t challenge[U2F_NONCE_LENGTH];
//} U2F_AUTHENTICATE_SIG_STR;
//
// static uint32_t next_cid(void)
//{
//    do {
//        cid = random_uint32(0);
//    } while (cid == 0 || cid == U2FHID_CID_BROADCAST);
//    return cid;
//}
//
// static void u2f_send_error(const uint16_t err, const uint32_t cid)
//{
//    uint8_t data[2];
//    data[0] = err >> 8 & 0xFF;
//    data[1] = err & 0xFF;
//    usb_frame_send_message(data, 2, cid);
//}
//
//
// static void u2f_device_version(const USB_APDU *a, const uint32_t cid)
//{
//    if (APDU_LEN(*a) != 0) {
//        u2f_send_error(U2F_SW_WRONG_LENGTH, cid);
//        return;
//    }
//
//    static const uint8_t version_response[] = {'U', '2', 'F',  '_', 'V', '2', 0x90, 0x00};
//    usb_frame_send_message(version_response, sizeof(version_response), cid);
//}
//
//
// static void u2f_keyhandle_gen(const uint8_t *appId, uint8_t *nonce, uint8_t *privkey,
//                              uint8_t *mac)
//{
//    uint8_t hash[SHA256_DIGEST_LENGTH];
//    for (;;) {
//        hmac_sha256(appId, U2F_APPID_SIZE, memory_report_master_u2f(), 32, hash);
//        hmac_sha256(hash, SHA256_DIGEST_LENGTH, nonce, U2F_NONCE_LENGTH, privkey);
//        hmac_sha256(hash, SHA256_DIGEST_LENGTH, privkey, U2F_EC_KEY_SIZE, mac);
//
//        if (ecc_isValid(privkey, ECC_SECP256r1)) {
//            break;
//        }
//
//        memcpy(nonce, mac, U2F_NONCE_LENGTH);
//    }
//}
//
//
// static void u2f_device_register(const USB_APDU *a, const uint32_t cid)
//{
//    const U2F_REGISTER_REQ *req = (const U2F_REGISTER_REQ *)a->data;
//
//    if (APDU_LEN(*a) != sizeof(U2F_REGISTER_REQ)) {
//        u2f_send_error(U2F_SW_WRONG_LENGTH, cid);
//        return;
//    }
//
//    if (touch_button_press(DBB_TOUCH_TIMEOUT) != DBB_TOUCHED) {
//        u2f_send_error(U2F_SW_CONDITIONS_NOT_SATISFIED, cid);
//        return;
//
//    } else {
//
//        uint8_t privkey[U2F_EC_KEY_SIZE], nonce[U2F_NONCE_LENGTH];
//        uint8_t mac[SHA256_DIGEST_LENGTH], sig[64];
//        uint8_t data[sizeof(U2F_REGISTER_RESP) + 2];
//        U2F_REGISTER_SIG_STR sig_base;
//        U2F_REGISTER_RESP *resp = (U2F_REGISTER_RESP *)&data;
//        util_zero(data, sizeof(data));
//
//        if (random_bytes(nonce, sizeof(nonce), 0) == DBB_ERROR) {
//            u2f_send_error(U2F_SW_WRONG_DATA, cid);
//            return;
//        }
//
//        u2f_keyhandle_gen(req->appId, nonce, privkey, mac);
//
//        ecc_get_public_key65(privkey, (uint8_t *)&resp->pubKey, ECC_SECP256r1);
//
//        resp->registerId = U2F_REGISTER_ID;
//        resp->keyHandleLen = U2F_KEYHANDLE_LEN;
//
//        memcpy(resp->keyHandleCertSig, mac, sizeof(mac));
//        memcpy(resp->keyHandleCertSig + sizeof(mac), nonce, sizeof(nonce));
//        memcpy(resp->keyHandleCertSig + resp->keyHandleLen, U2F_ATT_CERT, sizeof(U2F_ATT_CERT));
//
//        // Add signature using attestation key
//        sig_base.reserved = 0;
//        memcpy(sig_base.appId, req->appId, U2F_APPID_SIZE);
//        memcpy(sig_base.challenge, req->challenge, U2F_NONCE_LENGTH);
//        memcpy(sig_base.keyHandle, &resp->keyHandleCertSig, U2F_KEYHANDLE_LEN);
//        memcpy(sig_base.pubKey, &resp->pubKey, U2F_EC_POINT_SIZE);
//
//        if (ecc_sign(U2F_ATT_PRIV_KEY, (uint8_t *)&sig_base, sizeof(sig_base), sig,
//                     NULL, ECC_SECP256r1)) {
//            u2f_send_error(U2F_SW_WRONG_DATA, cid);
//            return;
//        }
//
//        uint8_t *resp_sig = resp->keyHandleCertSig + resp->keyHandleLen + sizeof(U2F_ATT_CERT);
//
//        const uint8_t sig_len = ecc_sig_to_der(sig, resp_sig);
//
//        // Append success bytes
//        memcpy(resp->keyHandleCertSig + resp->keyHandleLen + sizeof(U2F_ATT_CERT) + sig_len,
//               "\x90\x00", 2);
//
//        int len = 1 /* registerId */ + U2F_EC_POINT_SIZE +
//                  1 /* keyhandleLen */ + resp->keyHandleLen +
//                  sizeof(U2F_ATT_CERT) + sig_len + 2;
//
//        usb_frame_send_message(data, len, cid);
//    }
//}
//
//
// static void u2f_device_hijack(const U2F_AUTHENTICATE_REQ *req, const uint32_t cid)
//{
//    static char hijack_cmd[COMMANDER_REPORT_SIZE] = {0};
//
//    const uint32_t ctr = memory_u2f_count_iter();
//    char empty_report[3 + U2F_CTR_SIZE] = {0};// 1-byte flag | 4-byte ctr | 2-byte status
//    char *report;
//    int report_len;
//
//    size_t kh_len = MIN(U2F_MAX_KH_SIZE - 2, strlens((const char *)req->keyHandle + 2));
//    uint8_t tot = req->keyHandle[0];
//    uint8_t cnt = req->keyHandle[1];
//    size_t idx = cnt * (U2F_MAX_KH_SIZE - 2);
//
//    if (idx + kh_len < sizeof(hijack_cmd)) {
//        memcpy(hijack_cmd + idx, req->keyHandle + 2, kh_len);
//        hijack_cmd[idx + kh_len] = '\0';
//    }
//
//    if (cnt + 1 < tot) {
//        // Need more data. Acknowledge by returning an empty report.
//        report = empty_report;
//        report_len = sizeof(empty_report);
//    } else {
//        screen_led_blink();
//        report = commander(hijack_cmd);
//        report_len = MIN(strlens(report) + sizeof(empty_report), COMMANDER_REPORT_SIZE);
//        memmove(report + 1 + U2F_CTR_SIZE, report, MIN(strlens(report),
//                COMMANDER_REPORT_SIZE - U2F_CTR_SIZE - 1));
//        memset(hijack_cmd, 0, sizeof(hijack_cmd));
//    }
//
//    report[0] = 0;// Flags
//    report[1] = (ctr >> 24) & 0xff;
//    report[2] = (ctr >> 16) & 0xff;
//    report[3] = (ctr >> 8) & 0xff;
//    report[4] = ctr & 0xff;
//
//    // Append success bytes so that response gets through U2F client code.
//    // Otherwise, the client will resend sign requests until timing out.
//    // Errors encoded in JSON-formatted report.
//    memcpy(report + report_len - 2, "\x90\x00", 2);
//    usb_frame_send_message((const uint8_t *)report, report_len, cid);
//}
//
//
// static void u2f_device_authenticate(const USB_APDU *a, const uint32_t cid)
//{
//    uint8_t privkey[U2F_EC_KEY_SIZE], nonce[U2F_NONCE_LENGTH], mac[SHA256_DIGEST_LENGTH],
//            sig[64], i;
//    const U2F_AUTHENTICATE_REQ *req = (const U2F_AUTHENTICATE_REQ *)a->data;
//    U2F_AUTHENTICATE_SIG_STR sig_base;
//
//    if (APDU_LEN(*a) < U2F_KEYHANDLE_LEN) { // actual size could vary
//        u2f_send_error(U2F_SW_WRONG_LENGTH, cid);
//        return;
//    }
//
//    for (i = 0; i < U2F_HIJACK_ORIGIN_TOTAL; i++) {
//        // As an alternative interface, hijack the U2F AUTH key handle data field.
//        // Slower but works in browsers for specified sites without requiring an extension.
//        if (MEMEQ(req->challenge, U2F_HIJACK_CODE[i], U2F_NONCE_LENGTH)) {
//            if (!(memory_report_ext_flags() & MEM_EXT_MASK_U2F_HIJACK)) {
//                // Abort U2F hijack commands if the U2F_hijack bit is not set (== disabled).
//                usb_frame_send_err_hid(cid, U2FHID_ERR_CHANNEL_BUSY);
//            } else {
//                u2f_device_hijack(req, cid);
//            }
//            return;
//        }
//    }
//
//    if (req->keyHandleLen != U2F_KEYHANDLE_LEN) {
//        u2f_send_error(U2F_SW_WRONG_DATA, cid);
//        return;
//    }
//
//    memcpy(nonce, req->keyHandle + sizeof(mac), sizeof(nonce));
//
//    u2f_keyhandle_gen(req->appId, nonce, privkey, mac);
//
//    if (!MEMEQ(req->keyHandle, mac, SHA256_DIGEST_LENGTH)) {
//        u2f_send_error(U2F_SW_WRONG_DATA, cid);
//        return;
//    }
//
//    if (a->p1 == U2F_AUTH_CHECK_ONLY) {
//        u2f_send_error(U2F_SW_CONDITIONS_NOT_SATISFIED, cid);
//        return;
//    }
//
//    if (a->p1 != U2F_AUTH_ENFORCE) {
//        u2f_send_error(U2F_SW_WRONG_DATA, cid);
//        return;
//    }
//
//    if (touch_button_press(DBB_TOUCH_TIMEOUT) != DBB_TOUCHED) {
//        u2f_send_error(U2F_SW_CONDITIONS_NOT_SATISFIED, cid);
//        return;
//
//    } else {
//        uint8_t buf[sizeof(U2F_AUTHENTICATE_RESP) + 2];
//        U2F_AUTHENTICATE_RESP *resp =
//            (U2F_AUTHENTICATE_RESP *)&buf;
//
//        const uint32_t ctr = memory_u2f_count_iter();
//        resp->flags = U2F_AUTH_FLAG_TUP;
//        resp->ctr[0] = (ctr >> 24) & 0xff;
//        resp->ctr[1] = (ctr >> 16) & 0xff;
//        resp->ctr[2] = (ctr >> 8) & 0xff;
//        resp->ctr[3] = ctr & 0xff;
//
//        // Sign
//        memcpy(sig_base.appId, req->appId, U2F_APPID_SIZE);
//        sig_base.flags = resp->flags;
//        memcpy(sig_base.ctr, resp->ctr, 4);
//        memcpy(sig_base.challenge, req->challenge, U2F_NONCE_LENGTH);
//
//        if (ecc_sign(privkey, (uint8_t *)&sig_base, sizeof(sig_base), sig, NULL, ECC_SECP256r1)) {
//            u2f_send_error(U2F_SW_WRONG_DATA, cid);
//            return;
//        }
//
//        const uint8_t sig_len = ecc_sig_to_der(sig, resp->sig);
//
//        // Append success bytes
//        memcpy(buf + sizeof(U2F_AUTHENTICATE_RESP) - U2F_MAX_EC_SIG_SIZE + sig_len, "\x90\x00",
//               2);
//
//        usb_frame_send_message(buf,
//                               sizeof(U2F_AUTHENTICATE_RESP) - U2F_MAX_EC_SIG_SIZE + sig_len + 2,
//                               cid);
//    }
//}
//
///**
// * Checks the channel ID when need and returns an error if it is unexpected.
// * Called by every U2F function except for U2FHID_INIT.
// */
// static void u2f_device_check_cid(const uint32_t cid)
//{
//    if (cid == U2FHID_CID_BROADCAST || cid == 0) {
//        usb_frame_send_err_hid(cid, U2FHID_ERR_INVALID_CID);
//        return;
//    }
//}
//
///**
// * Processes the U2F ping command.
// */
// static void u2f_device_ping(const Packet *packet)
//{
//    const uint8_t *buf = packet->data;
//    const uint32_t len = packet->len;
//    const uint32_t cid = packet->cid;
//    u2f_device_check_cid(cid);
//    usb_frame_send_cmd(U2FHID_PING, buf, len, cid);
//}
//
///**
// * Processes the U2F wink command.
// */
// static void u2f_device_wink(const Packet *packet)
//{
//    const uint32_t len = packet->len;
//    const uint32_t cid = packet->cid;
//    u2f_device_check_cid(cid);
//
//    if (len > 0) {
//        usb_frame_send_err_hid(cid, U2FHID_ERR_INVALID_LEN);
//        return;
//    }
//
//    screen_led_blink();
//
//    USB_FRAME f;
//    util_zero(&f, sizeof(f));
//    f.cid = cid;
//    f.init.cmd = U2FHID_WINK;
//    f.init.bcntl = 0;
//    usb_queue_reply_add(&f);
//}
//
//
// static void u2f_device_sync(const uint8_t *buf, uint32_t len)
//{
//    // TODO - implement
//    (void) buf;
//    (void) len;
//}
//
//
// static void u2f_device_lock(const uint8_t *buf, uint32_t len)
//{
//    // TODO - implement
//    (void) buf;
//    (void) len;
//}
//
///**
// * This command synchronizes a channel and optionally requests the device to
// * allocate a unique 32-bit channel identifier (CID) that can be used by the
// * requesting application during its lifetime.
// */
// static void u2f_device_init(const Packet *packet)
//{
//    const uint32_t cid = packet->cid;
//    const U2FHID_INIT_REQ *init_req = (const U2FHID_INIT_REQ *)&packet->data;
//    U2FHID_INIT_RESP resp;
//
//    if (cid == 0) {
//        usb_frame_send_err_hid(cid, U2FHID_ERR_INVALID_CID);
//        return;
//    }
//
//    uint8_t data[USB_DATA_MAX_LEN];
//    util_zero(data, sizeof(data));
//
//    memcpy(resp.nonce, init_req->nonce, sizeof(init_req->nonce));
//    resp.cid = cid == U2FHID_CID_BROADCAST ? next_cid() : cid;
//    resp.versionInterface = U2FHID_IF_VERSION;
//    resp.versionMajor = DIGITAL_BITBOX_VERSION_MAJOR;
//    resp.versionMinor = DIGITAL_BITBOX_VERSION_MINOR;
//    resp.versionBuild = DIGITAL_BITBOX_VERSION_PATCH;
//    resp.capFlags = U2FHID_CAPFLAG_WINK;
//    memcpy(data, &resp, U2FHID_INIT_RESP_SIZE);
//    usb_frame_send_cmd(U2FHID_INIT, data, U2FHID_INIT_RESP_SIZE, resp.cid);
//}
//
///**
// * Processes a U2F message. This can be a registration,
// * authentication or version command.
// */
// static void u2f_device_msg(const Packet *packet)
//{
//    const USB_APDU *a = (USB_APDU *) packet->data;
//    const uint32_t len = packet->len;
//    const uint32_t cid = packet->cid;
//
//    if ((APDU_LEN(*a) + sizeof(USB_APDU)) > len) {
//        return;
//    }
//
//    if (a->cla != 0) {
//        u2f_send_error(U2F_SW_CLA_NOT_SUPPORTED, cid);
//        return;
//    }
//
//    switch (a->ins) {
//        case U2F_REGISTER:
//            u2f_device_register(a, cid);
//            break;
//        case U2F_AUTHENTICATE:
//            u2f_device_authenticate(a, cid);
//            break;
//        case U2F_VERSION:
//            u2f_device_version(a, cid);
//            break;
//        default:
//            u2f_send_error(U2F_SW_INS_NOT_SUPPORTED, cid);
//    }
//}

/**
 * Set up the U2F commands.
 */
void u2f_device_setup(void)
{
    //    const CMD_Callback u2f_cmd_callbacks[] = {
    //        { U2FHID_PING, u2f_device_ping },
    //        { U2FHID_WINK, u2f_device_wink },
    //        { U2FHID_INIT, u2f_device_init },
    //        { U2FHID_MSG, u2f_device_msg },
    //    };

    //    usb_frame_register_cmds(u2f_cmd_callbacks, sizeof(u2f_cmd_callbacks));
    usb_processing_register_cmds(NULL, 0);
}
