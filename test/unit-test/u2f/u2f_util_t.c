// Copyright 2014 Google Inc. All rights reserved.
// Copyright 2017 Douglas J. Bakkum, Shift Devices AG
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#include <time.h>

#include <arpa/inet.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "u2f_util_t.h"
#include "usb/usb.h"

#define MIN(a, b)                                       \
    __extension__({                                     \
        _Pragma("GCC diagnostic push");                 \
        _Pragma("GCC diagnostic ignored \"-Wshadow\""); \
        __typeof__(a) _a = (a);                         \
        __typeof__(b) _b = (b);                         \
        _Pragma("GCC diagnostic pop");                  \
        _a > _b ? _b : _a;                              \
    })
#define MAX(a, b)                                       \
    __extension__({                                     \
        _Pragma("GCC diagnostic push");                 \
        _Pragma("GCC diagnostic ignored \"-Wshadow\""); \
        __typeof__(a) _a = (a);                         \
        __typeof__(b) _b = (b);                         \
        _Pragma("GCC diagnostic pop");                  \
        _a > _b ? _a : _b;                              \
    })

static void util_uint8_to_hex(const uint8_t* in_bin, const size_t in_len, char* out)
{
    static char digits[] = "0123456789abcdef";
    size_t i;
    for (i = 0; i < in_len; i++) {
        out[i * 2] = digits[(in_bin[i] >> 4) & 0xF];
        out[i * 2 + 1] = digits[in_bin[i] & 0xF];
    }
    out[in_len * 2] = '\0';
}

#ifdef __APPLE__
    #ifndef CLOCK_MONOTONIC
        // Implement something compatible w/ linux clock_gettime()
        #include <mach/mach_time.h>
        #define CLOCK_MONOTONIC 0
static void clock_gettime(int which, struct timespec* ts)
{
    static mach_timebase_info_data_t __clock_gettime_inf;
    uint64_t now, nano;

    now = mach_absolute_time();
    if (0 == __clock_gettime_inf.denom) {
        mach_timebase_info(&__clock_gettime_inf);
    }

    nano = now * __clock_gettime_inf.numer / __clock_gettime_inf.denom;
    ts->tv_sec = nano * 1e-9;
    ts->tv_nsec = nano - (ts->tv_sec * 1e9);
}
    #endif
#endif // __APPLE__

float U2Fob_deltaTime(uint64_t* state)
{
    uint64_t now, delta;
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    now = (uint64_t)(ts.tv_sec * 1e9 + ts.tv_nsec);
    delta = *state ? now - *state : 0;
    *state = now;
    return (float)(delta / 1.0e9);
}

struct U2Fob* U2Fob_create(void)
{
    struct U2Fob* f = NULL;
    if (hid_init() == 0) {
        f = (struct U2Fob*)malloc(sizeof(struct U2Fob));
        memset(f, 0, sizeof(struct U2Fob));
        f->cid = -1;
    }
    return f;
}

void U2Fob_destroy(struct U2Fob* device)
{
    if (device) {
        U2Fob_close(device);
        free(device);
    }
    hid_exit();
}

uint32_t U2Fob_getCid(struct U2Fob* device)
{
    return device->cid;
}

static char* U2Fob_path(void)
{
    // Enumerate and print the HID devices on the system
    static char path[1024];
    memset(path, 0, sizeof(path));
    struct hid_device_info *devs, *cur_dev;
    hid_init();
    devs = hid_enumerate(0x0, 0x0);
    cur_dev = devs;
    while (cur_dev) {
        if (cur_dev->vendor_id == 0x03eb && cur_dev->product_id == 0x2403) {
            if (cur_dev->interface_number == 1 || cur_dev->usage_page == 0xf1d0) {
                // hidapi is not consistent across platforms
                // usage_page works on Windows/Mac; interface_number works on Linux
                PRINT_INFO("Device Found");
                PRINT_INFO("  VID PID:      %04hx %04hx", cur_dev->vendor_id, cur_dev->product_id);
                PRINT_INFO(
                    "  Page/Usage:   0x%x/0x%x (%d/%d)",
                    cur_dev->usage_page,
                    cur_dev->usage,
                    cur_dev->usage_page,
                    cur_dev->usage);
                PRINT_INFO("  Manufacturer: %ls", cur_dev->manufacturer_string);
                PRINT_INFO("  Product:      %ls", cur_dev->product_string);
                PRINT_INFO("  Device path:  %s", cur_dev->path);
                snprintf(path, sizeof(path), "%s", cur_dev->path);
                break;
            }
        }
        cur_dev = cur_dev->next;
    }
    hid_free_enumeration(devs);
    hid_exit();
    return path;
}

int U2Fob_open(struct U2Fob* device)
{
    U2Fob_close(device);
    device->path = U2Fob_path();
    device->dev = hid_open_path(device->path);
    return device->dev != NULL ? -U2FHID_ERR_NONE : -U2FHID_ERR_OTHER;
}

void U2Fob_close(struct U2Fob* device)
{
    if (device->dev) {
        hid_close(device->dev);
        device->dev = NULL;
    }
}

int U2Fob_reopen(struct U2Fob* device)
{
    U2Fob_close(device);
    device->dev = hid_open_path(device->path);
    return device->dev != NULL ? -U2FHID_ERR_NONE : -U2FHID_ERR_OTHER;
}

int U2Fob_sendHidFrame(struct U2Fob* device, USB_FRAME* f)
{
    uint8_t d[sizeof(USB_FRAME) + 1];
    int res = 0;

    d[0] = 0; // un-numbered report
    f->cid = htonl(f->cid); // cid is in network order on the wire
    memcpy(d + 1, f, sizeof(USB_FRAME));
    f->cid = ntohl(f->cid);

    if (!device->dev) {
        return -U2FHID_ERR_OTHER;
    }
    res = hid_write(device->dev, d, sizeof(d));
    // printf("%d\n", res);

    if (res == sizeof(d)) {
        return 0;
    }

    return -U2FHID_ERR_OTHER;
}

int U2Fob_receiveHidFrame(struct U2Fob* device, USB_FRAME* r, float to)
{
    if (to <= 0.0) {
        return -U2FHID_ERR_MSG_TIMEOUT;
    }

    if (!device->dev) {
        return -U2FHID_ERR_OTHER;
    }
    memset((int8_t*)r, 0xEE, sizeof(USB_FRAME));

    int res = 0;
    res = hid_read_timeout(device->dev, (uint8_t*)r, sizeof(USB_FRAME), (int)(to * 1000));

    if (res == sizeof(USB_FRAME)) {
        r->cid = ntohl(r->cid);
        return 0;
    }

    if (res == -1) {
        return -U2FHID_ERR_OTHER;
    }

    return -U2FHID_ERR_MSG_TIMEOUT;
}

int U2Fob_init(struct U2Fob* device)
{
    int res;
    USB_FRAME challenge;

    for (size_t i = 0; i < sizeof(device->nonce); ++i) {
        device->nonce[i] ^= (rand() >> 3);
    }

    challenge.cid = device->cid;
    challenge.init.cmd = U2FHID_INIT | U2FHID_TYPE_INIT;
    challenge.init.bcnth = 0;
    challenge.init.bcntl = U2FHID_INIT_NONCE_SIZE;
    memcpy(challenge.init.data, device->nonce, U2FHID_INIT_NONCE_SIZE);

    res = U2Fob_sendHidFrame(device, &challenge);
    if (res != 0) {
        printf("err other %" PRIu32 "\n", device->cid);
        return res;
    }

    for (;;) {
        USB_FRAME response;
        res = U2Fob_receiveHidFrame(device, &response, 2.0);

        if (res == -U2FHID_ERR_MSG_TIMEOUT) {
            printf("err timeout\n");
            return res;
        }
        if (res == -U2FHID_ERR_OTHER) {
            printf("err other\n");
            return res;
        }

        if (response.cid != challenge.cid) {
            continue;
        }
        if (response.init.cmd != challenge.init.cmd) {
            continue;
        }
        if (U2FHID_MSG_LEN(response) != U2FHID_INIT_RESP_SIZE) {
            continue;
        }
        if (memcmp(response.init.data, challenge.init.data, U2FHID_INIT_NONCE_SIZE)) {
            continue;
        }

        device->cid = (response.init.data[8] << 24) | (response.init.data[9] << 16) |
                      (response.init.data[10] << 8) | (response.init.data[11] << 0);

        break;
    }

    return 0;
}

int U2Fob_send(struct U2Fob* device, uint8_t cmd, const void* data, size_t size)
{
    USB_FRAME frame;
    int res;
    size_t frameLen;
    uint8_t seq = 0;
    const uint8_t* pData = (const uint8_t*)data;

    frame.cid = device->cid;
    frame.init.cmd = U2FHID_TYPE_INIT | cmd;
    frame.init.bcnth = (size >> 8) & 255;
    frame.init.bcntl = (size & 255);

    frameLen = MIN(size, sizeof(frame.init.data));
    memset(frame.init.data, 0xEE, sizeof(frame.init.data));
    memcpy(frame.init.data, pData, frameLen);

    do {
        res = U2Fob_sendHidFrame(device, &frame);
        if (res != 0) {
            return res;
        }

        size -= frameLen;
        pData += frameLen;

        frame.cont.seq = seq++;
        frameLen = MIN(size, sizeof(frame.cont.data));
        memset(frame.cont.data, 0xEE, sizeof(frame.cont.data));
        memcpy(frame.cont.data, pData, frameLen);
    } while (size);

    return 0;
}

int U2Fob_recv(struct U2Fob* device, uint8_t* cmd, void* data, size_t max, float timeout)
{
    USB_FRAME frame;
    int res, result;
    size_t totalLen, frameLen;
    uint8_t seq = 0;
    uint8_t* pData = (uint8_t*)data;
    uint64_t timeTracker = 0;

    U2Fob_deltaTime(&timeTracker);

    do {
        res = U2Fob_receiveHidFrame(device, &frame, timeout);
        if (res != 0) {
            return res;
        }

        timeout -= U2Fob_deltaTime(&timeTracker);
    } while (frame.cid != device->cid || U2FHID_FRAME_TYPE(frame) != U2FHID_TYPE_INIT);

    if (frame.init.cmd == U2FHID_ERROR) {
        return -frame.init.data[0];
    }

    *cmd = frame.init.cmd;

    totalLen = MIN((uint16_t)max, U2FHID_MSG_LEN(frame));
    frameLen = MIN(sizeof(frame.init.data), totalLen);

    result = totalLen;

    memcpy(pData, frame.init.data, frameLen);
    totalLen -= frameLen;
    pData += frameLen;

    while (totalLen) {
        res = U2Fob_receiveHidFrame(device, &frame, timeout);
        if (res != 0) {
            return res;
        }

        timeout -= U2Fob_deltaTime(&timeTracker);

        if (frame.cid != device->cid) {
            continue;
        }
        if (U2FHID_FRAME_TYPE(frame) != U2FHID_TYPE_CONT) {
            return -U2FHID_ERR_INVALID_SEQ;
        }
        if (U2FHID_FRAME_SEQ(frame) != seq++) {
            return -U2FHID_ERR_INVALID_SEQ;
        }

        frameLen = MIN(sizeof(frame.cont.data), totalLen);

        memcpy(pData, frame.cont.data, frameLen);
        totalLen -= frameLen;
        pData += frameLen;
    }

    return result;
}

int U2Fob_exchange_apdu_buffer(
    struct U2Fob* device,
    void* data,
    size_t size,
    char* in,
    size_t* in_len)
{
    uint8_t cmd = U2FHID_MSG;

    int res = U2Fob_send(device, cmd, data, size);
    if (res != 0) {
        return res;
    }

    uint8_t buf[4096];
    memset(buf, 0xEE, sizeof(buf));
    res = U2Fob_recv(device, &cmd, buf, sizeof(buf), 20.0);
    if (res < 0) {
        return res;
    }

    if (cmd != U2FHID_MSG) {
        return -U2FHID_ERR_OTHER;
    }

    uint16_t sw12;

    if (res < 2) {
        return -U2FHID_ERR_OTHER;
    }
    sw12 = (buf[res - 2] << 8) | buf[res - 1];
    res -= 2;

    memcpy(in, buf, res);
    *in_len = res;
    return sw12;
}

int U2Fob_apdu(
    struct U2Fob* device,
    uint8_t CLA,
    uint8_t INS,
    uint8_t P1,
    uint8_t P2,
    const char* out,
    size_t out_len,
    char* in,
    size_t* in_len)
{
    uint8_t buf[4096];
    size_t nc = out_len ? (3 + out_len) : 0;

    // Construct outgoing message.
    memset(buf, 0xEE, sizeof(buf));
    buf[0] = CLA;
    buf[1] = INS;
    buf[2] = P1;
    buf[3] = P2;

    uint8_t offs = 4;

    // Encode lc.
    if (nc) {
        buf[offs++] = 0; // extended length
        buf[offs++] = (out_len >> 8) & 255;
        buf[offs++] = (out_len & 255);
        memcpy(buf + offs, out, out_len);
        offs += out_len;
    }

    // Encode le.
    if (!nc) {
        // When there are no data sent, an extra 0 is necessary prior to Le.
        buf[offs++] = 0;
    }
    buf[offs++] = 0;
    buf[offs++] = 0;

    return U2Fob_exchange_apdu_buffer(device, buf, offs, in, in_len);
}

bool getCertificate(const U2F_REGISTER_RESP rsp, char* cert, size_t* cert_len)
{
    size_t hkLen = rsp.keyHandleLen;

    CHECK_GE(hkLen, (size_t)64);
    CHECK_LT(hkLen, sizeof(rsp.keyHandleCertSig));

    size_t certOff = hkLen;
    size_t certLen = sizeof(rsp.keyHandleCertSig) - certOff;
    const uint8_t* p = &rsp.keyHandleCertSig[certOff];

    CHECK_GE(certLen, (size_t)4);
    CHECK_EQ(p[0], 0x30);

    CHECK_GE(p[1], 0x81);
    CHECK_LE(p[1], 0x82);

    size_t seqLen;
    size_t headerLen;
    if (p[1] == 0x81) {
        seqLen = p[2];
        headerLen = 3;
    } else if (p[1] == 0x82) {
        seqLen = p[2] * 256 + p[3];
        headerLen = 4;
    } else {
        // FAIL
        abort();
    }

    CHECK_LE(seqLen, certLen - headerLen);

    memcpy(cert, p, seqLen + headerLen);
    *cert_len = seqLen + headerLen;
    return true;
}

bool getSignature(const U2F_REGISTER_RESP rsp, char* sig, size_t* sig_len)
{
    char cert[1028];
    size_t cert_len;
    CHECK_NE(false, getCertificate(rsp, cert, &cert_len));

    size_t sigOff = rsp.keyHandleLen + cert_len;
    CHECK_LE(sigOff, sizeof(rsp.keyHandleCertSig));

    size_t sigLen = sizeof(rsp.keyHandleCertSig) - sigOff;
    const uint8_t* p = &rsp.keyHandleCertSig[sigOff];

    CHECK_GE(sigLen, (size_t)2);
    CHECK_EQ(p[0], 0x30);

    size_t seqLen = p[1];
    CHECK_LE(seqLen, sigLen - 2);

    memcpy(sig, p, seqLen + 2);
    *sig_len = seqLen + 2;
    return true;
}

bool getSubjectPublicKey(const char* cert, size_t cert_len, char* pk, size_t* pk_len)
{
    CHECK_GE(cert_len, (size_t)U2F_EC_POINT_SIZE);

    // Explicitly search for ASN.1 lead-in sequence of p256-ecdsa public key.
    const char asn1[] = "3059301306072a8648ce3d020106082a8648ce3d030107034200";

    char cert_c[cert_len * 2 + 1];
    char* cert_c_p = cert_c;
    util_uint8_to_hex((const uint8_t*)cert, cert_len, cert_c);
    // memcpy(cert_c, utils_uint8_to_hex((const uint8_t *)cert, cert_len), cert_len * 2);

    char* pkStart = strstr(cert_c, asn1);
    CHECK_EQ(!pkStart, 0);

    size_t off = (pkStart - cert_c_p) / 2;
    CHECK_NE(off, (size_t)0);

    off += sizeof(asn1) / 2;
    CHECK_LE(off, cert_len - U2F_EC_POINT_SIZE);

    memcpy(pk, cert + off, U2F_EC_POINT_SIZE);
    *pk_len = U2F_EC_POINT_SIZE;
    return true;
}

bool getCertSignature(const char* cert, size_t cert_len, char* sig, size_t* sig_len)
{
    // Explicitly search ASN.1 lead-in sequence of p256-ecdsa signature.
    const char asn1[] = "300906072a8648ce3d040103";
    char cert_c[cert_len * 2 + 1];
    char* cert_c_p = cert_c;
    util_uint8_to_hex((const uint8_t*)cert, cert_len, cert_c);
    // memcpy(cert_c, utils_uint8_to_hex((const uint8_t *)cert, cert_len), cert_len * 2);

    char* pkStart = strstr(cert_c, asn1);
    CHECK_EQ(!pkStart, 0);

    size_t off = (pkStart - cert_c_p) / 2;
    CHECK_NE(off, (size_t)0);

    off += sizeof(asn1) / 2;
    CHECK_LE(off, cert_len - 8);

    size_t bitStringLen = cert[off] & 255;
    CHECK_EQ(bitStringLen, cert_len - off - 1);
    CHECK_EQ(cert[off + 1], 0);

    memcpy(sig, cert + off + 2, cert_len - off - 2);
    *sig_len = cert_len - off - 2;
    return true;
}

bool verifyCertificate(const char* pk, const char* cert)
{
    CHECK_EQ(true, false); // not yet implemented
    return true;
}
