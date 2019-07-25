/*

 The MIT License (MIT)

 Copyright (c) 2015-2016 Douglas J. Bakkum

 Permission is hereby granted, free of charge, to any person obtaining
 a copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation
 the rights to use, copy, modify, merge, publish, distribute, sublicense,
 and/or sell copies of the Software, and to permit persons to whom the
 Software is furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included
 in all copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
 OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 OTHER DEALINGS IN THE SOFTWARE.

*/

#ifndef _ECC_H_
#define _ECC_H_

#include <stdint.h>

typedef enum ecc_curve_id {
    ECC_SECP256k1,
    ECC_SECP256r1,
} ecc_curve_id;

/* ecc_wrapper for bitcoin use */
struct ecc_wrapper {
    void (*ecc_context_init)(void);
    void (*ecc_context_destroy)(void);
    int (*ecc_sign_digest)(
        const uint8_t* private_key,
        const uint8_t* data,
        uint8_t* sig,
        uint8_t* recid,
        ecc_curve_id curve);
    int (*ecc_sign)(
        const uint8_t* private_key,
        const uint8_t* msg,
        uint32_t msg_len,
        uint8_t* sig,
        uint8_t* recid,
        ecc_curve_id curve);
    int (*ecc_sign_double)(
        const uint8_t* privateKey,
        const uint8_t* msg,
        uint32_t msg_len,
        uint8_t* sig,
        uint8_t* recid,
        ecc_curve_id curve);
    int (*ecc_verify)(
        const uint8_t* public_key,
        const uint8_t* signature,
        const uint8_t* msg,
        uint32_t msg_len,
        ecc_curve_id curve);
    int (*ecc_generate_private_key)(
        uint8_t* private_child,
        const uint8_t* private_master,
        const uint8_t* z,
        ecc_curve_id curve);
    int (*ecc_isValid)(uint8_t* private_key, ecc_curve_id curve);
    void (
        *ecc_get_public_key65)(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve);
    void (
        *ecc_get_public_key33)(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve);
    int (*ecc_ecdh)(
        const uint8_t* pair_pubkey,
        const uint8_t* rand_privkey,
        uint8_t* ecdh_secret,
        ecc_curve_id curve);
    int (*ecc_recover_public_key)(
        const uint8_t* sig,
        const uint8_t* msg,
        uint32_t msg_len,
        uint8_t recid,
        uint8_t* pubkey_65,
        ecc_curve_id curve);
};

/* uECC direct wrapper */
void ecc_context_init(void);
void ecc_context_destroy(void);
int ecc_sign_digest(
    const uint8_t* private_key,
    const uint8_t* data,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve);
int ecc_sign(
    const uint8_t* private_key,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve);
int ecc_sign_double(
    const uint8_t* privateKey,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve);
int ecc_verify_digest(
    const uint8_t* public_key,
    const uint8_t* hash,
    const uint8_t* sig,
    ecc_curve_id curve);
int ecc_verify(
    const uint8_t* public_key,
    const uint8_t* signature,
    const uint8_t* msg,
    uint32_t msg_len,
    ecc_curve_id curve);
int ecc_generate_private_key(
    uint8_t* private_child,
    const uint8_t* private_master,
    const uint8_t* z,
    ecc_curve_id curve);
int ecc_isValid(uint8_t* private_key, ecc_curve_id curve);
void ecc_get_public_key65(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve);
void ecc_get_public_key33(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve);
int ecc_ecdh(
    const uint8_t* pair_pubkey,
    const uint8_t* rand_privkey,
    uint8_t* ecdh_secret,
    ecc_curve_id curve);
int ecc_sig_to_der(const uint8_t* sig, uint8_t* der);
int ecc_der_to_sig(const uint8_t* der, int der_len, uint8_t* sig);
int ecc_recover_public_key(
    const uint8_t* sig,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t recid,
    uint8_t* pubkey_65,
    ecc_curve_id curve);

/* bitcoin ecc wrapper that gets linked to secp256k1 if presen, otherwise to uECC */
extern struct ecc_wrapper bitcoin_ecc;

#endif
