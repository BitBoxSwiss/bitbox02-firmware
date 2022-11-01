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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sha2.h"
#include "u2f/ecc.h"

#include "u2f/uECC.h"

static void random_32_bytes(uint8_t* buf)
{
    if (buf == NULL) {
        return;
    }
    uint8_t random[32] = {0};
    for (uint32_t i = 0; i < sizeof(random); i++) {
        random[i] = rand();
    }
    for (size_t i = 0; i < sizeof(random); i++) {
        buf[i] ^= random[i];
    }
}

#ifndef ECC_USE_SECP256K1_LIB
/* link the bitcoin ECC wrapper to uECC if secp256k1 is not available */
struct ecc_wrapper bitcoin_ecc = {
    ecc_context_init,
    ecc_context_destroy,
    ecc_sign_digest,
    ecc_sign,
    ecc_sign_double,
    ecc_verify,
    ecc_generate_private_key,
    ecc_isValid,
    ecc_get_public_key65,
    ecc_get_public_key33,
    ecc_ecdh,
    ecc_recover_public_key};
#endif

static int ecc_rng_function(uint8_t* r, unsigned l)
{
    uint8_t buf[32];
    random_32_bytes(buf);
    if (l <= 32) {
        memcpy(r, buf, l);
    } else {
        return 0;
    }
    return 1;
}

void ecc_context_init(void)
{
    uECC_RNG_Function rng_function = ecc_rng_function;
    uECC_set_rng(rng_function);
}

void ecc_context_destroy(void)
{
    // pass
}

static uECC_Curve ecc_curve_from_id(ecc_curve_id curve)
{
    if (curve == ECC_SECP256r1) {
        return uECC_secp256r1();
    }
    return uECC_secp256k1();
}

int ecc_sign_digest(
    const uint8_t* private_key,
    const uint8_t* data,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve)
{
    (void)recid; // not implemented in uECC
    uint8_t tmp[32 + 32 + 64];
    SHA256_HashContext ctx = {
        .uECC = {
            .init_hash = &init_SHA256,
            .update_hash = &update_SHA256,
            .finish_hash = &finish_SHA256,
            .block_size = 64,
            .result_size = 32,
            .tmp = tmp}};
    if (uECC_sign_deterministic(
            private_key, data, SHA256_DIGEST_LENGTH, &ctx.uECC, sig, ecc_curve_from_id(curve))) {
        uECC_normalize_signature(sig, ecc_curve_from_id(curve));
        return 0;
    } else {
        return 1; // error
    }
}

int ecc_sign(
    const uint8_t* private_key,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve)
{
    uint8_t hash[SHA256_DIGEST_LENGTH];
    sha256_Raw(msg, msg_len, hash);
    return ecc_sign_digest(private_key, hash, sig, recid, curve);
}

int ecc_sign_double(
    const uint8_t* privateKey,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t* sig,
    uint8_t* recid,
    ecc_curve_id curve)
{
    uint8_t hash[SHA256_DIGEST_LENGTH];
    sha256_Raw(msg, msg_len, hash);
    sha256_Raw(hash, SHA256_DIGEST_LENGTH, hash);
    return ecc_sign_digest(privateKey, hash, sig, recid, curve);
}

static int ecc_read_pubkey(const uint8_t* publicKey, uint8_t* public_key_64, ecc_curve_id curve)
{
    if (publicKey[0] == 0x04) {
        memcpy(public_key_64, publicKey + 1, 64);
        return 1;
    } else if (publicKey[0] == 0x02 || publicKey[0] == 0x03) { // compute missing y coords
        uECC_decompress(publicKey, public_key_64, ecc_curve_from_id(curve));
        return 1;
    }
    // error
    return 0;
}

int ecc_verify_digest(
    const uint8_t* public_key,
    const uint8_t* hash,
    const uint8_t* sig,
    ecc_curve_id curve)
{
    // Do not force normalization of the signature. Otherwise will break bootloader
    // verification of previous firmware blobs.
    return !uECC_verify(public_key, hash, SHA256_DIGEST_LENGTH, sig, ecc_curve_from_id(curve));
}

int ecc_verify(
    const uint8_t* public_key,
    const uint8_t* signature,
    const uint8_t* msg,
    uint32_t msg_len,
    ecc_curve_id curve)
{
    uint8_t public_key_64[64];
    uint8_t hash[SHA256_DIGEST_LENGTH];
    sha256_Raw(msg, msg_len, hash);
    ecc_read_pubkey(public_key, public_key_64, curve);
    return ecc_verify_digest(public_key_64, hash, signature, curve);
}

int ecc_generate_private_key(
    uint8_t* private_child,
    const uint8_t* private_master,
    const uint8_t* z,
    ecc_curve_id curve)
{
    uECC_generate_private_key(private_child, private_master, z, ecc_curve_from_id(curve));
    return ecc_isValid(private_child, curve);
}

int ecc_isValid(uint8_t* private_key, ecc_curve_id curve)
{
    return uECC_isValid(private_key, ecc_curve_from_id(curve));
}

void ecc_get_public_key65(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve)
{
    uint8_t* p = public_key;
    p[0] = 0x04;
    uECC_compute_public_key(private_key, p + 1, ecc_curve_from_id(curve));
}

void ecc_get_public_key33(const uint8_t* private_key, uint8_t* public_key, ecc_curve_id curve)
{
    uint8_t public_key_long[64];
    uECC_compute_public_key(private_key, public_key_long, ecc_curve_from_id(curve));
    uECC_compress(public_key_long, public_key, ecc_curve_from_id(curve));
}

int ecc_ecdh(
    const uint8_t* pair_pubkey,
    const uint8_t* rand_privkey,
    uint8_t* ecdh_secret,
    ecc_curve_id curve)
{
    uint8_t public_key[64];
    uECC_decompress(pair_pubkey, public_key, ecc_curve_from_id(curve));
    if (uECC_shared_secret(public_key, rand_privkey, ecdh_secret, ecc_curve_from_id(curve))) {
        sha256_Raw(ecdh_secret, 32, ecdh_secret);
        sha256_Raw(ecdh_secret, 32, ecdh_secret);
        return 0;
    } else {
        return 1;
    }
}

int ecc_recover_public_key(
    const uint8_t* sig,
    const uint8_t* msg,
    uint32_t msg_len,
    uint8_t recid,
    uint8_t* pubkey_65,
    ecc_curve_id curve)
{
    /* not implemented for uECC */
    return 1;
}

int ecc_sig_to_der(const uint8_t* sig, uint8_t* der)
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

static int trim_to_32_bytes(const uint8_t* src, int src_len, uint8_t* dst)
{
    int dst_offset;
    while (*src == '\0' && src_len > 0) {
        src++;
        src_len--;
    }
    if (src_len > 32 || src_len < 1) {
        return 1;
    }
    dst_offset = 32 - src_len;
    memset(dst, 0, dst_offset);
    memcpy(dst + dst_offset, src, src_len);
    return 0;
}

int ecc_der_to_sig(const uint8_t* der, int der_len, uint8_t* sig_64)
{
    /*
     * Structure is:
     *   0x30 0xNN  SEQUENCE + s_length
     *   0x02 0xNN  INTEGER + r_length
     *   0xAA 0xBB  ..   r_length bytes of "r" (offset 4)
     *   0x02 0xNN  INTEGER + s_length
     *   0xMM 0xNN  ..   s_length bytes of "s" (offset 6 + r_len)
     */
    int seq_len;
    uint8_t r_bytes[32];
    uint8_t s_bytes[32];
    int r_len;
    int s_len;

    memset(r_bytes, 0, sizeof(r_bytes));
    memset(s_bytes, 0, sizeof(s_bytes));

    /*
     * Must have at least:
     * 2 bytes sequence header and length
     * 2 bytes R integer header and length
     * 1 byte of R
     * 2 bytes S integer header and length
     * 1 byte of S
     *
     * 8 bytes total
     */
    if (der_len < 8 || der[0] != 0x30 || der[2] != 0x02) {
        return 1;
    }

    seq_len = der[1];
    if ((seq_len <= 0) || (seq_len + 2 != der_len)) {
        return 1;
    }

    r_len = der[3];
    /*
     * Must have at least:
     * 2 bytes for R header and length
     * 2 bytes S integer header and length
     * 1 byte of S
     */
    if ((r_len < 1) || (r_len > seq_len - 5) || (der[4 + r_len] != 0x02)) {
        return 1;
    }
    s_len = der[5 + r_len];

    /**
     * Must have:
     * 2 bytes for R header and length
     * r_len bytes for R
     * 2 bytes S integer header and length
     */
    if ((s_len < 1) || (s_len != seq_len - 4 - r_len)) {
        return 1;
    }

    /*
     * ASN.1 encoded integers are zero-padded for positive integers. Make sure we have
     * a correctly-sized buffer and that the resulting integer isn't too large.
     */
    if (trim_to_32_bytes(&der[4], r_len, sig_64) ||
        trim_to_32_bytes(&der[6 + r_len], s_len, sig_64 + 32)) {
        return 1;
    }

    return 0;
}
