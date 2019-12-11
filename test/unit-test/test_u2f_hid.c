// Copyright 2014 Google Inc. All rights reserved.
// Copyright 2017 Douglas J. Bakkum, Shift Devices AG
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Basic U2F HID framing compliance test.

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#ifdef __linux__
#include <linux/hidraw.h>
#include <sys/ioctl.h>
#endif

#include "random.h"
#include "u2f/ecc.h"
#include "util.h"

#include "u2f.h"
#include "u2f/u2f_util_t.h"
#include "usb/u2f/u2f_hid.h"
#include "usb/usb.h"

struct U2Fob* device;

#define SEND(f) CHECK_EQ(0, U2Fob_sendHidFrame(device, &(f)))
#define RECV(f, t) CHECK_EQ(0, U2Fob_receiveHidFrame(device, &(f), t))

// Initialize a frame with |len| random payload, or data.
static void initFrame(USB_FRAME* f, uint32_t cid, uint8_t cmd, size_t len, const void* data)
{
    memset(f, 0, sizeof(USB_FRAME));
    f->cid = cid;
    f->init.cmd = cmd | U2FHID_TYPE_INIT;
    f->init.bcnth = (uint8_t)(len >> 8);
    f->init.bcntl = (uint8_t)len;
    for (size_t i = 0; i < MIN(len, sizeof(f->init.data)); ++i) {
        f->init.data[i] = data ? ((const uint8_t*)data)[i] : (rand() & 255);
    }
}

// Return true if frame r is error frame for expected error.
static bool isError(const USB_FRAME r, int error)
{
    // printf("%02x %d %d %d\n", r.init.cmd, U2FHID_MSG_LEN(r), r.init.data[0], error);
    return r.init.cmd == U2FHID_ERROR && U2FHID_MSG_LEN(r) == 1 && r.init.data[0] == error;
}

// Test basic INIT.
// Returns basic capabilities field.
static uint8_t test_BasicInit(void)
{
    USB_FRAME f, r;
    initFrame(&f, U2FHID_CID_BROADCAST, U2FHID_INIT, U2FHID_INIT_NONCE_SIZE, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(r.init.cmd, U2FHID_INIT);
    CHECK_EQ(U2FHID_MSG_LEN(r), U2FHID_INIT_RESP_SIZE);
    CHECK_EQ(memcmp(&f.init.data[0], &r.init.data[0], U2FHID_INIT_NONCE_SIZE), 0);
    CHECK_EQ(r.init.data[12], U2FHID_IF_VERSION);
    return r.init.data[16];
}

// Test we have a working (single frame) echo.
static void test_Echo(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 8, NULL);

    U2Fob_deltaTime(&t);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    // Expect echo somewhat quickly.
    CHECK_LT(U2Fob_deltaTime(&t), .1);

    // Check echoed content matches.
    CHECK_EQ(U2FHID_PING, r.init.cmd);
    CHECK_EQ(U2FHID_MSG_LEN(f), U2FHID_MSG_LEN(r));
    CHECK_EQ(0, memcmp(f.init.data, r.init.data, U2FHID_MSG_LEN(f)));
}

// Test we can echo message larger than a single frame.
#define TESTSIZE 1024
static void test_LongEcho(void)
{
    uint8_t challenge[TESTSIZE];
    uint8_t response[TESTSIZE];
    uint8_t cmd = U2FHID_PING;

    for (size_t i = 0; i < sizeof(challenge); ++i) {
        challenge[i] = rand();
    }

    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    CHECK_EQ(0, U2Fob_send(device, cmd, challenge, sizeof(challenge)));

    float sent = U2Fob_deltaTime(&t);

    CHECK_EQ(sizeof(response), (size_t)U2Fob_recv(device, &cmd, response, sizeof(response), 2.0));

    float received = U2Fob_deltaTime(&t);

    CHECK_EQ(cmd, U2FHID_PING);
    CHECK_EQ(0, memcmp(challenge, response, sizeof(challenge)));

    PRINT_INFO("test_LongEcho() - SENT %f, RECV %f", sent, received);

    // Expected transfer times for 2ms bInterval.
    // We do not want fobs to be too slow or too agressive.
    CHECK_GE(sent, .020);
    CHECK_GE(received, .020);
    CHECK_LE(sent, .090);
    CHECK_LE(received, .090);
}

// Execute WINK, if implemented.
// Visually inspect fob for compliance.
static void test_OptionalWink(void)
{
    USB_FRAME f, r;
    uint8_t caps = test_BasicInit();

    initFrame(&f, U2Fob_getCid(device), U2FHID_WINK, 0, NULL);

    SEND(f);
    RECV(r, 5.0);
    CHECK_EQ(f.cid, r.cid);

    if (caps & U2FHID_CAPFLAG_WINK) {
        CHECK_EQ(f.init.cmd, r.init.cmd);
        CHECK_EQ(U2FHID_MSG_LEN(r), 0);
    } else {
        CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CMD), true);
    }
}

// Test max data size limit enforcement.
// We try echo 7610 bytes.
// Device should pre-empt communications with error reply.
static void test_Limits(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 7610, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_LEN), true);
}

// Check there are no frames pending for this cid.
// Poll for a frame with short timeout.
// Make sure none got received and timeout time passed.
static void test_Idle(float timeOut)
{
    USB_FRAME r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    U2Fob_deltaTime(&t);
    CHECK_EQ(-U2FHID_ERR_MSG_TIMEOUT, U2Fob_receiveHidFrame(device, &r, timeOut));
    CHECK_GE(U2Fob_deltaTime(&t), .2);
    CHECK_LE(U2Fob_deltaTime(&t), .5);
}

// Check we get a timeout error frame if not sending U2FHID_TYPE_CONT frames
// for a message that spans multiple frames.
// Device should timeout at ~.5 seconds.
static void test_Timeout(void)
{
    USB_FRAME f, r;
    float measuredTimeout;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL);

    U2Fob_deltaTime(&t);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_MSG_TIMEOUT), true);

    measuredTimeout = U2Fob_deltaTime(&t);
    CHECK_GE(measuredTimeout, .4); // needs to be at least 0.4 seconds
    CHECK_LE(measuredTimeout, 1.0); // but at most 1.0 seconds
}

// Test LOCK functionality, if implemented.
static void test_Lock(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);
    uint8_t caps = test_BasicInit();

    // Check whether lock is supported using an unlock command.
    initFrame(&f, U2Fob_getCid(device), U2FHID_LOCK, 1, "\x00");
    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    if (!(caps & U2FHID_CAPFLAG_LOCK)) {
        // Make sure CAPFLAG reflects behavior.
        CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CMD), true);
        return;
    }

    // Lock channel for 3 seconds.
    initFrame(&f, U2Fob_getCid(device), U2FHID_LOCK, 1, "\x03");

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(f.init.cmd, r.init.cmd);
    CHECK_EQ(0, U2FHID_MSG_LEN(r));

    // Rattle lock, checking for BUSY.
    int count = 0;
    do {
        // The requested channel timeout (3 seconds) resets
        // after every message, so we only send a couple of
        // messages down the channel in this loop. Otherwise
        // the lock would never expire.
        if (++count < 2) {
            test_Echo();
        }
        sleep(1);
        initFrame(&f, U2Fob_getCid(device) ^ 1, U2FHID_PING, 1, NULL);

        SEND(f);
        RECV(r, 1.0);
        CHECK_EQ(f.cid, r.cid);

        if (r.init.cmd == U2FHID_ERROR) {
            // We only expect BUSY here.
            CHECK_EQ(isError(r, U2FHID_ERR_CHANNEL_BUSY), true);
        }
    } while (r.init.cmd == U2FHID_ERROR);

    CHECK_GE(U2Fob_deltaTime(&t), 2.5);
}

// Check we get abort if we send U2FHID_TYPE_INIT when U2FHID_TYPE_CONT is expected.
static void test_NotCont(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL); // Note 99 > frame.

    SEND(f);

    SEND(f); // Send frame again, i.e. another U2FHID_TYPE_INIT frame.
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_LT(U2Fob_deltaTime(&t), .1); // Expect fail reply quickly.
    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_SEQ), true);

    // Check there are no further messages.
    CHECK_EQ(-U2FHID_ERR_MSG_TIMEOUT, U2Fob_receiveHidFrame(device, &r, 0.6F));
}

// Check we get a error when sending wrong sequence in continuation frame.
static void test_WrongSeq(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL);

    SEND(f);

    f.cont.seq = 1 | U2FHID_TYPE_CONT; // Send wrong SEQ, 0 is expected.

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_LT(U2Fob_deltaTime(&t), .1); // Expect fail reply quickly.
    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_SEQ), true);

    // Check there are no further messages.
    CHECK_EQ(-U2FHID_ERR_MSG_TIMEOUT, U2Fob_receiveHidFrame(device, &r, 0.6F));
}

// Check we hear nothing if we send a random CONT frame.
// TODO: isn't the correct behavior to return an error frame?
static void test_NotFirst(void)
{
    USB_FRAME f, r;

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 8, NULL);
    f.cont.seq = 0 | U2FHID_TYPE_CONT; // Make continuation packet.

    SEND(f);
    int res = U2Fob_receiveHidFrame(device, &r, 1.0F);
    CHECK_EQ(-U2FHID_ERR_MSG_TIMEOUT, res);
}

// Check we get a BUSY if device is waiting for CONT on other channel.
static void test_Busy(void)
{
    USB_FRAME f, r;
    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL);

    SEND(f);

    f.cid ^= 1; // Flip channel.

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_LT(U2Fob_deltaTime(&t), .1); // Expect busy reply quickly.
    CHECK_EQ(isError(r, U2FHID_ERR_CHANNEL_BUSY), true);

    f.cid ^= 1; // Flip back.

    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_MSG_TIMEOUT), true);

    CHECK_GE(U2Fob_deltaTime(&t), .45); // Expect T/O msg only after timeout.
}

// Test INIT self aborts wait for CONT frame
static void test_InitSelfAborts(void)
{
    USB_FRAME f, r;

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL);
    SEND(f);

    initFrame(&f, U2Fob_getCid(device), U2FHID_INIT, U2FHID_INIT_NONCE_SIZE, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(r.init.cmd, U2FHID_INIT);
    CHECK_GE(U2FHID_MSG_LEN(r), U2FHID_MSG_LEN(f));
    CHECK_EQ(memcmp(&f.init.data[0], &r.init.data[0], U2FHID_INIT_NONCE_SIZE), 0);

    test_NotFirst();
}

// Test INIT other does not abort wait for CONT.
static void test_InitOther(void)
{
    USB_FRAME f, f2, r;

    initFrame(&f, U2Fob_getCid(device), U2FHID_PING, 99, NULL);
    SEND(f);

    initFrame(&f2, U2Fob_getCid(device) ^ 1, U2FHID_INIT, U2FHID_INIT_NONCE_SIZE, NULL);

    SEND(f2);
    RECV(r, 1.0);
    CHECK_EQ(f2.cid, r.cid);

    // Expect sync reply for requester
    CHECK_EQ(r.init.cmd, U2FHID_INIT);
    CHECK_GE(U2FHID_MSG_LEN(r), U2FHID_MSG_LEN(f2));
    CHECK_EQ(memcmp(&f2.init.data[0], &r.init.data[0], U2FHID_INIT_NONCE_SIZE), 0);

    // Expect error frame after timeout on first channel.
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_MSG_TIMEOUT), true);
}

// static void wait_Idle(void)
//{
//    USB_FRAME r;
//
//    while (-U2FHID_ERR_MSG_TIMEOUT != U2Fob_receiveHidFrame(device, &r, .2f)) {
//    }
//}

static void test_LeadingZero(void)
{
    USB_FRAME f, r;
    initFrame(&f, 0x100, U2FHID_PING, 10, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(r.cid, f.cid);

    CHECK_EQ(r.init.cmd, U2FHID_PING);
    CHECK_EQ(U2FHID_MSG_LEN(f), U2FHID_MSG_LEN(r));
}

static void test_InitOnNonBroadcastEchoesCID(void)
{
    USB_FRAME f, r;
    size_t cs = U2FHID_INIT_NONCE_SIZE;

    initFrame(&f, 0xdeadbeef, U2FHID_INIT, cs, NULL); // Use non-broadcast cid

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(r.cid, f.cid);

    CHECK_EQ(r.init.cmd, U2FHID_INIT);
    CHECK_EQ(U2FHID_MSG_LEN(r), U2FHID_INIT_RESP_SIZE);
    CHECK_EQ(0, memcmp(f.init.data, r.init.data, cs));

    uint32_t cid = (r.init.data[cs + 0] << 24) | (r.init.data[cs + 1] << 16) |
                   (r.init.data[cs + 2] << 8) | (r.init.data[cs + 3] << 0);

    CHECK_EQ(cid, 0xdeadbeef);
}

static uint32_t test_Init(bool check)
{
    USB_FRAME f, r;
    size_t cs = U2FHID_INIT_NONCE_SIZE;

    initFrame(&f, -1, U2FHID_INIT, cs, NULL); // -1 is broadcast channel

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(r.cid, f.cid);

    // expect init reply
    CHECK_EQ(r.init.cmd, U2FHID_INIT);

    CHECK_EQ(U2FHID_MSG_LEN(r), U2FHID_INIT_RESP_SIZE);

    // Check echo of challenge
    CHECK_EQ(0, memcmp(f.init.data, r.init.data, cs));

    uint32_t cid = (r.init.data[cs + 0] << 0) | (r.init.data[cs + 1] << 8) |
                   (r.init.data[cs + 2] << 16) | (r.init.data[cs + 3] << 24);

    if (check) {
        // Check that another INIT yields a distinct cid.
        CHECK_NE(test_Init(false), cid);
    }

    return cid;
}

static void test_InitUnderLock(void)
{
    USB_FRAME f, r;
    uint8_t caps = test_BasicInit();

    // Check whether lock is supported, using an unlock command.
    initFrame(&f, U2Fob_getCid(device), U2FHID_LOCK, 1, "\x00"); // unlock

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    if (!(caps & U2FHID_CAPFLAG_LOCK)) {
        // Make sure CAPFLAG reflects behavior.
        CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CMD), true);
        return;
    }

    initFrame(&f, U2Fob_getCid(device), U2FHID_LOCK, 1, "\x03"); // 3 seconds

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(f.init.cmd, r.init.cmd);
    CHECK_EQ(0, U2FHID_MSG_LEN(r));

    // We have a lock. CMD_INIT should work whilst another holds lock.

    test_Init(false);
    test_InitOnNonBroadcastEchoesCID();

    // Unlock.
    initFrame(&f, U2Fob_getCid(device), U2FHID_LOCK, 1, "\x00");

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(f.init.cmd, r.init.cmd);
    CHECK_EQ(0, U2FHID_MSG_LEN(r));
}

static void test_Unknown(uint8_t cmd)
{
    USB_FRAME f, r;

    initFrame(&f, U2Fob_getCid(device), cmd, 0, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CMD), true);
}

static void test_OnlyInitOnBroadcast(void)
{
    USB_FRAME f, r;

    initFrame(&f, -1, U2FHID_PING, U2FHID_INIT_NONCE_SIZE, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CID), true);
}

static void test_NothingOnChannel0(void)
{
    USB_FRAME f, r;

    initFrame(&f, 0, U2FHID_INIT, U2FHID_INIT_NONCE_SIZE, NULL);

    SEND(f);
    RECV(r, 1.0);
    CHECK_EQ(f.cid, r.cid);

    CHECK_EQ(isError(r, U2FHID_ERR_INVALID_CID), true);
}

static void test_Descriptor(void)
{
#if defined(WITH_HARDWARE) && defined(__linux__)
    struct hidraw_report_descriptor rpt_desc;
    int res, desc_size;
    // hidapi hides internal struct.
    // Use inside knowledge to cast and get fd we need.
    // Is not compatible with hidapi-libusb because of internals
    int fd = *(int*)(device->dev);

    memset(&rpt_desc, 0x0, sizeof(rpt_desc));
    res = ioctl(fd, HIDIOCGRDESCSIZE, &desc_size);
    CHECK_GE(res, 0);

    rpt_desc.size = desc_size;
    res = ioctl(fd, HIDIOCGRDESC, &rpt_desc);
    CHECK_GE(res, 0);
    CHECK_GE(desc_size, 4);

    // Should start with Usage Page 0xf1d0, Usage 0x01
    CHECK_EQ(0, memcmp(rpt_desc.value, "\x06\xd0\xf1\x09\x01", 5));

    // TODO: check for 0x20 and 0x21 endpoints.
#endif
}

static void run_tests(void)
{
    // Start of tests
    //
    device = U2Fob_create();

    if (U2Fob_open(device) == 0) {
        PASS(test_Idle(0.3));
        PASS(test_Init(true));
        // Now that we have INIT, get a proper cid for device.
        CHECK_EQ(U2Fob_init(device), 0);
        PASS(test_BasicInit());
        PASS(test_Unknown(U2FHID_SYNC));
        PASS(test_InitOnNonBroadcastEchoesCID());
        PASS(test_InitUnderLock());
        PASS(test_InitSelfAborts());
        PASS(test_OptionalWink());
        PASS(test_Lock());
        PASS(test_Echo());
        PASS(test_LongEcho());
        PASS(test_WrongSeq());
        PASS(test_NotCont());
        PASS(test_NotFirst());
        PASS(test_Limits());
        PASS(test_InitOther());
        PASS(test_Timeout());
        PASS(test_Busy());
        PASS(test_Descriptor());
        PASS(test_LeadingZero());
        PASS(test_Idle(2.0));
        PASS(test_NothingOnChannel0());
        PASS(test_OnlyInitOnBroadcast());
    } else {
        printf("\n\nNot testing HID API. A device is not connected.\n\n");
        return;
    }

    U2Fob_destroy(device);
}

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    printf("\n\nError: stack smashing detected!\n\n");
    abort();
}

int main(void)
{
    // Live test of the HID API
    printf("\n\nHID API Result:\n");
    run_tests();

    printf("\nALL TESTS PASSED\n\n");
    return 0;
}
