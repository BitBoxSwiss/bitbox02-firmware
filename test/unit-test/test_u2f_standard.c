// Copyright 2014 Google Inc. All rights reserved.
// Copyright 2017-2018 Douglas J. Bakkum, Shift Devices AG
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// U2F register / sign compliance test.

//  The error codes
//    SW_NO_ERROR (0x9000): The command completed successfully without error.
//    SW_CONDITIONS_NOT_SATISFIED (0x6985): The request was rejected due to test-of-user-presence
//    being required. SW_WRONG_DATA (0x6A80): The request was rejected due to an invalid key handle.
//    SW_WRONG_LENGTH (0x6700): The length of the request was invalid.
//    SW_CLA_NOT_SUPPORTED (0x6E00): The Class byte of the request is not supported.
//    SW_INS_NOT_SUPPORTED (0x6D00): The Instruction of the request is not supported.

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "u2f/ecc.h"
#include "u2f/sha2.h"

#include "u2f.h"
#include "u2f/u2f_util_t.h"

static bool arg_hasButton = true; // fob has button
int U_TESTS_FAIL = 0;

struct U2Fob* device;

U2F_REGISTER_REQ regReq;
U2F_REGISTER_RESP regRsp;

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

#if defined(WITH_HARDWARE)
static void WaitForUserPresence(struct U2Fob* dev, bool hasButton)
{
    char msg[1];
    U2Fob_close(dev);
    if (hasButton) {
        PRINT_MESSAGE("Hit enter then CONFIRM on the device...");
    }
    if (scanf("%c", msg)) {
        (void)msg;
    }
    CHECK_EQ(0, U2Fob_reopen(dev));
    CHECK_EQ(0, U2Fob_init(dev));
}
#endif

static void test_Version(void)
{
    char rsp[4096];
    size_t rsp_len;
    int res = U2Fob_apdu(device, 0, U2F_VERSION, 0, 0, "", 0, rsp, &rsp_len);
    if (res == 0x9000) {
        CHECK_EQ(0, strncmp("U2F_V2", rsp, rsp_len));
        return;
    }

    // Non-ISO 7816-4 compliant U2F_VERSION "APDU" that includes Lc value 0,
    // for compatibility with older devices.
    uint8_t buf[4 + 3 + 2];
    buf[0] = 0; // CLA
    buf[1] = U2F_VERSION; // INS
    buf[2] = 0; // P1
    buf[3] = 0; // P2
    buf[4] = 0; // extended length
    buf[5] = 0; // Lc = 0 (Not ISO 7816-4 compliant)
    buf[6] = 0; // Lc = 0 (Not ISO 7816-4 compliant)
    buf[7] = 0; // Le = 0
    buf[8] = 0; // Le = 0
    CHECK_EQ(0x9000, U2Fob_exchange_apdu_buffer(device, buf, sizeof(buf), rsp, &rsp_len));
    CHECK_EQ(0, strncmp("U2F_V2", rsp, rsp_len));
}

static void test_UnknownINS(void)
{
    char rsp[4096];
    size_t rsp_len;
    CHECK_EQ(0x6D00, U2Fob_apdu(device, 0, 0 /* not U2F INS */, 0, 0, "", 0, rsp, &rsp_len));
    CHECK_EQ(rsp_len, (size_t)0);
}

static void test_BadCLA(void)
{
    char rsp[4096];
    size_t rsp_len;
    CHECK_EQ(
        0x6E00,
        U2Fob_apdu(device, 1 /* not U2F CLA, 0x00 */, U2F_VERSION, 0, 0, "abc", 3, rsp, &rsp_len));
    CHECK_EQ(rsp_len, (size_t)0);
}

static void test_WrongLength_U2F_VERSION(void)
{
    char rsp[4096];
    size_t rsp_len;
    // U2F_VERSION does not take any input.
    CHECK_EQ(0x6700, U2Fob_apdu(device, 0, U2F_VERSION, 0, 0, "abc", 3, rsp, &rsp_len));
    CHECK_EQ(rsp_len, (size_t)0);
}

static void test_WrongLength_U2F_REGISTER(void)
{
    char rsp[4096];
    size_t rsp_len;
    // U2F_REGISTER does expect input.
    CHECK_EQ(0x6700, U2Fob_apdu(device, 0, U2F_REGISTER, 0, 0, "abc", 3, rsp, &rsp_len));
    CHECK_EQ(rsp_len, (size_t)0);
}

static void test_Enroll(int expectedSW12, int printinfo)
{
    // pick random origin and challenge.
    for (size_t i = 0; i < sizeof(regReq.challenge); ++i) {
        regReq.challenge[i] = rand();
    }
    for (size_t i = 0; i < sizeof(regReq.appId); ++i) {
        regReq.appId[i] = rand();
    }

    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    char rsp[4096];
    size_t rsp_len;
    char regReq_c[U2F_NONCE_LENGTH + U2F_APPID_SIZE + 1];
    memset(regReq_c, 0, sizeof(regReq_c));
    memcpy(regReq_c, regReq.challenge, U2F_NONCE_LENGTH);
    memcpy(regReq_c + U2F_NONCE_LENGTH, regReq.appId, U2F_APPID_SIZE);
    CHECK_EQ(
        expectedSW12,
        U2Fob_apdu(
            device, 0, U2F_REGISTER, U2F_AUTH_ENFORCE, 0, regReq_c, sizeof(regReq), rsp, &rsp_len));

    if (expectedSW12 != 0x9000) {
        CHECK_EQ(rsp_len, (size_t)0);
        return;
    }

    CHECK_NE(rsp_len, (size_t)0);
    CHECK_LE(rsp_len, sizeof(U2F_REGISTER_RESP));

    memcpy(&regRsp, rsp, rsp_len);
    CHECK_EQ(regRsp.registerId, U2F_REGISTER_ID);
    CHECK_EQ(regRsp.pubKey.format, U2F_UNCOMPRESSED_POINT);

    if (printinfo) {
        PRINT_INFO("Enroll: %lu bytes in %fs", rsp_len, U2Fob_deltaTime(&t));
    }

    // Check crypto of enroll response.
    char cert[U2F_MAX_ATT_CERT_SIZE];
    size_t cert_len;
    CHECK_EQ(getCertificate(regRsp, cert, &cert_len), true);
    if (printinfo) {
        char buf[cert_len * 2 + 1];
        util_uint8_to_hex((uint8_t*)cert, cert_len, buf);
        PRINT_INFO("Certificate: %lu %s", cert_len, buf);
    }

    char pk[U2F_EC_POINT_SIZE];
    size_t pk_len;
    CHECK_EQ(getSubjectPublicKey(cert, cert_len, pk, &pk_len), true);
    if (printinfo) {
        char buf[pk_len * 2 + 1];
        util_uint8_to_hex((uint8_t*)pk, pk_len, buf);
        PRINT_INFO("Public key:  %lu %s", pk_len, buf);
    }
    CHECK_EQ(pk_len, (size_t)U2F_EC_POINT_SIZE);

    char sig[U2F_MAX_EC_SIG_SIZE];
    size_t sig_len;
    CHECK_EQ(getSignature(regRsp, sig, &sig_len), true);
    if (printinfo) {
        char buf[sig_len * 2 + 1];
        util_uint8_to_hex((uint8_t*)sig, sig_len, buf);
        PRINT_INFO("Signature:   %lu %s", sig_len, buf);
    }

    // Parse signature into two integers.
    uint8_t signature[64];
    CHECK_EQ(0, ecc_der_to_sig((uint8_t*)sig, sig_len, signature));

    // Compute hash.
    uint8_t hash[SHA256_BLOCK_LENGTH];
    uint8_t rfu = 0;
    SHA256_CTX ctx;
    sha256_Init(&ctx);
    sha256_Update(&ctx, &rfu, sizeof(rfu)); // 0x00
    sha256_Update(&ctx, regReq.appId, sizeof(regReq.appId)); // O
    sha256_Update(&ctx, regReq.challenge, sizeof(regReq.challenge)); // d
    sha256_Update(&ctx, regRsp.keyHandleCertSig, regRsp.keyHandleLen); // hk
    sha256_Update(&ctx, (uint8_t*)&regRsp.pubKey, sizeof(regRsp.pubKey)); // pk
    sha256_Final(hash, &ctx);

    // Verify signature.
    CHECK_EQ(0, ecc_verify_digest((uint8_t*)pk + 1, hash, signature, ECC_SECP256r1));
}

// returns ctr
// TODO: Test without hardware as well.
#if defined(WITH_HARDWARE)
static uint32_t test_Sign(int expectedSW12, bool checkOnly)
{
    U2F_AUTHENTICATE_REQ authReq;

    // pick random challenge and use registered appId.
    for (size_t i = 0; i < sizeof(authReq.challenge); ++i) {
        authReq.challenge[i] = rand();
    }
    memcpy(authReq.appId, regReq.appId, sizeof(authReq.appId));
    authReq.keyHandleLength = regRsp.keyHandleLen;
    memcpy(authReq.keyHandle, regRsp.keyHandleCertSig, authReq.keyHandleLength);

    uint64_t t = 0;
    U2Fob_deltaTime(&t);

    char rsp[4096];
    size_t rsp_len;
    char authReq_c[U2F_NONCE_LENGTH + U2F_APPID_SIZE + 1 + U2F_MAX_KH_SIZE + 1];
    memset(authReq_c, 0, sizeof(authReq_c));
    memcpy(authReq_c, authReq.challenge, U2F_NONCE_LENGTH);
    memcpy(authReq_c + U2F_NONCE_LENGTH, authReq.appId, U2F_APPID_SIZE);
    memcpy(authReq_c + U2F_NONCE_LENGTH + U2F_APPID_SIZE, &authReq.keyHandleLength, 1);
    memcpy(
        authReq_c + U2F_NONCE_LENGTH + U2F_APPID_SIZE + 1,
        authReq.keyHandle,
        authReq.keyHandleLength);

    CHECK_EQ(
        expectedSW12,
        U2Fob_apdu(
            device,
            0,
            U2F_AUTHENTICATE,
            checkOnly ? U2F_AUTH_CHECK_ONLY : U2F_AUTH_ENFORCE,
            0,
            authReq_c,
            U2F_NONCE_LENGTH + U2F_APPID_SIZE + 1 + authReq.keyHandleLength,
            rsp,
            &rsp_len));

    if (expectedSW12 != 0x9000) {
        CHECK_EQ(rsp_len, (size_t)0);
        return 0;
    }

    CHECK_NE(rsp_len, (size_t)0);
    CHECK_LE(rsp_len, sizeof(U2F_AUTHENTICATE_RESP));

    U2F_AUTHENTICATE_RESP resp;
    memcpy(&resp, rsp, rsp_len);

    CHECK_EQ(resp.flags, 0x01);

    PRINT_INFO("Sign: %lu bytes in %fs", rsp_len, U2Fob_deltaTime(&t));

    // Parse signature from authenticate response.
    uint8_t signature[64];
    CHECK_EQ(
        0, ecc_der_to_sig(resp.sig, rsp_len - sizeof(resp.flags) - sizeof(resp.ctr), signature));

    // Compute hash.
    uint8_t hash[SHA256_BLOCK_LENGTH];
    SHA256_CTX ctx;
    sha256_Init(&ctx);

    sha256_Update(&ctx, regReq.appId, sizeof(regReq.appId)); // O
    sha256_Update(&ctx, &resp.flags, sizeof(resp.flags)); // T
    sha256_Update(&ctx, (uint8_t*)&resp.ctr, sizeof(resp.ctr)); // CTR
    sha256_Update(&ctx, authReq.challenge, sizeof(authReq.challenge)); // d
    sha256_Final(hash, &ctx);

    // Verify signature.
    CHECK_EQ(0, ecc_verify_digest((uint8_t*)&regRsp.pubKey + 1, hash, signature, ECC_SECP256r1));

    return ((resp.ctr[0] << 24) + (resp.ctr[1] << 16) + (resp.ctr[2] << 8) + (resp.ctr[3]));
}
#endif

static void check_Compilation(void)
{
    // Couple of sanity checks.
    CHECK_EQ(sizeof(U2F_EC_POINT), (size_t)65);
    CHECK_EQ(sizeof(U2F_REGISTER_REQ), (size_t)64);
}

static void run_tests(void)
{
    // Start of tests
    //
    device = U2Fob_create();

    if (U2Fob_open(device) == 0) {
        CHECK_EQ(0, U2Fob_init(device));
        PASS(check_Compilation());
        PASS(test_Version());
        PASS(test_UnknownINS());
        PASS(test_WrongLength_U2F_VERSION());
        PASS(test_WrongLength_U2F_REGISTER());
        PASS(test_BadCLA());

        // Fob with button should need touch.
        if (arg_hasButton) {
            // Timeout
            PRINT_MESSAGE("PRESS abort or WAIT for device to timeout.\n");
            PASS(test_Enroll(0x6985, 1));
        }
        // TODO: Wrap lower level functions to run below tests without hardware
        // Since we are using securechip for signing, we need to emulate that in software.
#if defined(WITH_HARDWARE)
        WaitForUserPresence(device, arg_hasButton);
        PASS(test_Enroll(0x9000, 1));

        // Fob with button should have consumed touch.
        if (arg_hasButton) {
            // Timeout
            PRINT_MESSAGE("Press ABORT or wait for device to timeout.\n");
            PASS(test_Sign(0x6985, false));
        }

        // Sign with check only should not produce signature.
        PASS(test_Sign(0x6985, true));

        // Sign with wrong key handle.
        regRsp.keyHandleCertSig[0] ^= 0x55;
        PASS(test_Sign(0x6a80, false));
        regRsp.keyHandleCertSig[0] ^= 0x55;

        // Sign with wrong app id.
        regReq.appId[0] ^= 0xaa;
        PASS(test_Sign(0x6a80, false));
        regReq.appId[0] ^= 0xaa;

        // Sign with check only should not produce signature.
        WaitForUserPresence(device, arg_hasButton);
        PASS(test_Sign(0x6985, true));

        uint32_t ctr1;
        PASS(ctr1 = test_Sign(0x9000, false)); // < fails
        // Timeout
        PRINT_MESSAGE("Press ABORT or wait for device to timeout.\n");
        PASS(test_Sign(0x6985, false));

        WaitForUserPresence(device, arg_hasButton);
        uint32_t ctr2;
        PASS(ctr2 = test_Sign(0x9000, false));
        // Ctr should have incremented by 1.
        CHECK_EQ(ctr2, ctr1 + 1);
#endif
    } else {
        PRINT_MESSAGE("\n\nNot testing HID API. A device is not connected.\n\n");
        return;
    }

    U2Fob_destroy(device);
}

int main(void)
{
    // Live test of the HID API
    PRINT_MESSAGE("\n\nHID API Result:\n");
    run_tests();
    PRINT_MESSAGE("\nALL TESTS PASSED\n\n");
    return 0;
}
