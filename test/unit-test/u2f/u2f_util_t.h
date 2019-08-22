// Copyright 2014 Google Inc. All rights reserved.
// Copyright 2017 Douglas J. Bakkum, Shift Devices AG
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef __U2F_UTIL_H_INCLUDED__
#define __U2F_UTIL_H_INCLUDED__

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>

#include "usb/u2f/u2f.h"
#include "usb/u2f/u2f_hid.h"
#include "usb/usb.h"
#include "usb/usb_frame.h"

#ifndef CONTINUOUS_INTEGRATION
#include <hidapi/hidapi.h>
#else
typedef void hid_device;
#endif

#define CHECK_HELPER(a, b, label, comp)                                      \
    __extension__({                                                          \
        __typeof__(a) _a = (a);                                              \
        __typeof__(b) _b = (b);                                              \
        if (_a comp _b) {                                                    \
            printf(                                                          \
                "\x1b[31mCHECK_" #label " fail at %s()[%d] %f (%08x) " #comp \
                " %f (%08x)\x1b[0m \n",                                      \
                __func__,                                                    \
                __LINE__,                                                    \
                _a * 1.0,                                                    \
                (uint32_t)_a,                                                \
                _b * 1.0,                                                    \
                (uint32_t)_b);                                               \
            abort();                                                         \
        }                                                                    \
    })

#define CHECK_EQ(a, b) CHECK_HELPER(a, b, EQ, !=)
#define CHECK_NE(a, b) CHECK_HELPER(a, b, NE, ==)
#define CHECK_GE(a, b) CHECK_HELPER(a, b, GE, <)
#define CHECK_GT(a, b) CHECK_HELPER(a, b, GT, <=)
#define CHECK_LT(a, b) CHECK_HELPER(a, b, LT, >=)
#define CHECK_LE(a, b) CHECK_HELPER(a, b, LE, >)

#define PASS(x)                                   \
    do {                                          \
        (x);                                      \
        printf("\x1b[32mPASS(" #x ")\x1b[0m \n"); \
    } while (0)
#define PRINT_INFO(...)       \
    do {                      \
        printf("\x1b[34m");   \
        printf(__VA_ARGS__);  \
        printf("\x1b[0m \n"); \
    } while (0)
#define PRINT_MESSAGE(...)            \
    do {                              \
        fprintf(stderr, __VA_ARGS__); \
        fflush(stderr);               \
    } while (0)

void U2Fob_testLiveDevice(uint8_t test);
uint8_t U2Fob_liveDeviceTesting(void);
float U2Fob_deltaTime(uint64_t* state);

struct U2Fob {
    hid_device* dev;
    char* path;
    uint32_t cid;
    uint8_t nonce[U2FHID_INIT_NONCE_SIZE];
};

struct U2Fob* U2Fob_create(void);
void U2Fob_destroy(struct U2Fob* device);
int U2Fob_open(struct U2Fob* device);
void U2Fob_close(struct U2Fob* device);
int U2Fob_reopen(struct U2Fob* device);
int U2Fob_init(struct U2Fob* device);
uint32_t U2Fob_getCid(struct U2Fob* device);
int U2Fob_sendHidFrame(struct U2Fob* device, USB_FRAME* out);
int U2Fob_receiveHidFrame(struct U2Fob* device, USB_FRAME* in, float timeoutSeconds);
int U2Fob_send(struct U2Fob* device, uint8_t cmd, const void* data, size_t size);
int U2Fob_recv(struct U2Fob* device, uint8_t* cmd, void* data, size_t size, float timeoutSeconds);

// Exchanges a pre-formatted APDU buffer with the device.
// returns
//   negative error
//   positive sw12, e.g. 0x9000, 0x6985 etc.
int U2Fob_exchange_apdu_buffer(
    struct U2Fob* device,
    void* data,
    size_t size,
    char* in,
    size_t* in_len);

// Formats an APDU with the given field values, and exchanges it
// with the device.
// returns
//   negative error
//   positive sw12, e.g. 0x9000, 0x6985 etc.
int U2Fob_apdu(
    struct U2Fob* device,
    uint8_t CLA,
    uint8_t INS,
    uint8_t P1,
    uint8_t P2,
    const char* out,
    size_t out_len,
    char* in,
    size_t* in_len);

bool getCertificate(U2F_REGISTER_RESP rsp, char* cert, size_t* cert_len);
bool getSignature(U2F_REGISTER_RESP rsp, char* sig, size_t* sig_len);
bool getSubjectPublicKey(const char* cert, size_t cert_len, char* pk, size_t* pk_len);
bool getCertSignature(const char* cert, size_t cert_len, char* sig, size_t* sig_len);
bool verifyCertificate(const char* pk, const char* cert);

#endif // __U2F_UTIL_H_INCLUDED__
