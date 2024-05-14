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

#include "pukcc.h"
#include "CryptoLib_Hardware_pb.h"
#include "CryptoLib_Headers_pb.h"
#include "CryptoLib_JumpTable_pb.h"
#include "CryptoLib_typedef_pb.h"
#include "curve_p256.h"
#include <driver_init.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <util.h>

// TODO - function to derive public key from private key
// TODO - determistic K
// TODO - normalize signature (low-s)

#define PUKCC_PADDING 4u
#define PUKCC_PARAM_RAM_LEN (PUKCC_ECC_PARAM_LEN + PUKCC_PADDING)
#define PUKCC_BASE_CRYPTO_RAM 0x02011000u

// ******************************************************************************
// Memory mapping for ECDSA signature
// ******************************************************************************
#define BASE_ECDSA_MODULO PUKCC_BASE_CRYPTO_RAM
#define BASE_ECDSA_CNS (BASE_ECDSA_MODULO + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_POINT_A (BASE_ECDSA_CNS + PUKCC_PARAM_RAM_LEN + 8)
#define BASE_ECDSA_POINT_A_X (BASE_ECDSA_POINT_A)
#define BASE_ECDSA_POINT_A_Y (BASE_ECDSA_POINT_A_X + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_POINT_A_Z (BASE_ECDSA_POINT_A_Y + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_A (BASE_ECDSA_POINT_A_Z + PUKCC_PARAM_RAM_LEN)
#define BASE_PRIVATE_KEY (BASE_ECDSA_A + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_SCALAR (BASE_PRIVATE_KEY + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_ORDER (BASE_ECDSA_SCALAR + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_HASH (BASE_ECDSA_ORDER + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSA_WORKSPACE (BASE_ECDSA_HASH + PUKCC_PARAM_RAM_LEN)

// ******************************************************************************
// Memory mapping for ECDSA signature verification
// ******************************************************************************
#define BASE_ECDSAV_MODULO PUKCC_BASE_CRYPTO_RAM
#define BASE_ECDSAV_CNS (BASE_ECDSAV_MODULO + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_ORDER (BASE_ECDSAV_CNS + PUKCC_PARAM_RAM_LEN + 8)
#define BASE_ECDSAV_SIGNATURE (BASE_ECDSAV_ORDER + PUKCC_PARAM_RAM_LEN + 8)
#define BASE_ECDSAV_HASH (BASE_ECDSAV_SIGNATURE + 2 * PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_POINT_A (BASE_ECDSAV_HASH + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_POINT_A_X (BASE_ECDSAV_POINT_A)
#define BASE_ECDSAV_POINT_A_Y (BASE_ECDSAV_POINT_A_X + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_POINT_A_Z (BASE_ECDSAV_POINT_A_Y + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_PUBLIC_KEY (BASE_ECDSAV_POINT_A_Z + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_PUBLIC_KEY_X (BASE_ECDSAV_PUBLIC_KEY)
#define BASE_ECDSAV_PUBLIC_KEY_Y (BASE_ECDSAV_PUBLIC_KEY_X + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_PUBLIC_KEY_Z (BASE_ECDSAV_PUBLIC_KEY_Y + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_A (BASE_ECDSAV_PUBLIC_KEY_Z + PUKCC_PARAM_RAM_LEN)
#define BASE_ECDSAV_WORKSPACE (BASE_ECDSAV_A + PUKCC_PARAM_RAM_LEN)

// ******************************************************************************
// Memory mapping for ECC scalar multiplication
// ******************************************************************************
#define BASE_SCA_MUL_MODULO PUKCC_BASE_CRYPTO_RAM
#define BASE_SCA_MUL_CNS (BASE_SCA_MUL_MODULO + PUKCC_PARAM_RAM_LEN)
#define BASE_SCA_MUL_POINT_A (BASE_SCA_MUL_CNS + PUKCC_PARAM_RAM_LEN + 8)
#define BASE_SCA_MUL_POINT_A_X (BASE_SCA_MUL_POINT_A)
#define BASE_SCA_MUL_POINT_A_Y (BASE_SCA_MUL_POINT_A_X + PUKCC_PARAM_RAM_LEN)
#define BASE_SCA_MUL_POINT_A_Z (BASE_SCA_MUL_POINT_A_Y + PUKCC_PARAM_RAM_LEN)
#define BASE_SCA_MUL_A (BASE_SCA_MUL_POINT_A_Z + PUKCC_PARAM_RAM_LEN + 8)
#define BASE_SCA_MUL_SCALAR (BASE_SCA_MUL_A + PUKCC_PARAM_RAM_LEN)
#define BASE_SCA_MUL_ORDER (BASE_SCA_MUL_SCALAR + PUKCC_PARAM_RAM_LEN)
#define BASE_SCA_MUL_WORKSPACE (BASE_SCA_MUL_ORDER + PUKCC_PARAM_RAM_LEN)

// ******************************************************************************
// Memory mapping for conversion
// ******************************************************************************
#define BASE_CONV_MODULO PUKCC_BASE_CRYPTO_RAM
#define BASE_CONV_CNS (BASE_CONV_MODULO + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_POINT_A (BASE_CONV_CNS + PUKCC_PARAM_RAM_LEN + 4)
#define BASE_CONV_POINT_A_X (BASE_CONV_POINT_A)
#define BASE_CONV_POINT_A_Y (BASE_CONV_POINT_A_X + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_POINT_A_Z (BASE_CONV_POINT_A_Y + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_RANDOM (BASE_CONV_POINT_A_Z + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_A (BASE_CONV_RANDOM + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_B (BASE_CONV_A + PUKCC_PARAM_RAM_LEN)
#define BASE_CONV_WORKSPACE (BASE_CONV_B + PUKCC_PARAM_RAM_LEN)

COMPILER_ALIGNED(128)
static struct sha_context pukcc_sha256_context;
COMPILER_PACK_RESET()

// Copy bytes in SRAM to PUKCC RAM; change endianess
static void pukcc_memcopy(uint8_t* dest, const uint8_t* src)
{
    uint16_t cnt = 0;
    uint8_t* p_dest;
    p_dest = dest;

    while (cnt < PUKCC_ECC_PARAM_LEN) {
        *(p_dest++) = src[PUKCC_ECC_PARAM_LEN - cnt - 1];
        cnt++;
    }
    memset(p_dest, 0, PUKCC_PADDING);
}

#if 0
static void pukcc_init_params_ecdsa_generation(const uint8_t *private_key, const uint8_t *hash, const PUKCC_CURVE_256_X curve)
{
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_MODULO), curve.modulo_p);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_POINT_A_X), curve.base_x);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_POINT_A_Y), curve.base_y);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_POINT_A_Z), curve.one);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_A), curve.a);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_ORDER), curve.order);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_SCALAR), curve.test_k);// TODO - deterministic K
    pukcc_memcopy((uint8_t *)(BASE_PRIVATE_KEY), private_key);
    pukcc_memcopy((uint8_t *)(BASE_ECDSA_HASH), hash);
    PUKCL_ZpEcDsaGenerate(nu1ModBase)        = (nu1)BASE_ECDSA_MODULO;
    PUKCL_ZpEcDsaGenerate(nu1CnsBase)        = (nu1)BASE_ECDSA_CNS;
    PUKCL_ZpEcDsaGenerate(nu1PointABase)     = (nu1)BASE_ECDSA_POINT_A;
    PUKCL_ZpEcDsaGenerate(nu1PrivateKey)     = (nu1)BASE_PRIVATE_KEY;
    PUKCL_ZpEcDsaGenerate(nu1ScalarNumber)   = (nu1)BASE_ECDSA_SCALAR;
    PUKCL_ZpEcDsaGenerate(nu1OrderPointBase) = (nu1)BASE_ECDSA_ORDER;
    PUKCL_ZpEcDsaGenerate(nu1ABase)          = (nu1)BASE_ECDSA_A;
    PUKCL_ZpEcDsaGenerate(nu1Workspace)      = (nu1)BASE_ECDSA_WORKSPACE;
    PUKCL_ZpEcDsaGenerate(nu1HashBase)       = (nu1)BASE_ECDSA_HASH;
    PUKCL_ZpEcDsaGenerate(u2ModLength)       = PUKCC_ECC_PARAM_LEN;
    PUKCL_ZpEcDsaGenerate(u2ScalarLength)    = PUKCC_ECC_PARAM_LEN;
}
#endif

static void pukcc_init_params_ecdsa_verification(
    const uint8_t* signature,
    const uint8_t* hash,
    const uint8_t* public_key,
    const PUKCC_CURVE_256_X curve)
{
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_MODULO), curve.modulo_p);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_POINT_A_X), curve.base_x);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_POINT_A_Y), curve.base_y);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_POINT_A_Z), curve.one);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_A), curve.a);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_ORDER), curve.order);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_SIGNATURE), signature);
    pukcc_memcopy(
        (uint8_t*)(BASE_ECDSAV_SIGNATURE) + PUKCC_PARAM_RAM_LEN, signature + PUKCC_ECC_PARAM_LEN);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_PUBLIC_KEY_X), public_key);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_PUBLIC_KEY_Y), public_key + PUKCC_ECC_PARAM_LEN);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_PUBLIC_KEY_Z), curve.one);
    pukcc_memcopy((uint8_t*)(BASE_ECDSAV_HASH), hash);
    PUKCL_ZpEcDsaVerify(nu1ModBase) = (nu1)BASE_ECDSAV_MODULO;
    PUKCL_ZpEcDsaVerify(nu1CnsBase) = (nu1)BASE_ECDSAV_CNS;
    PUKCL_ZpEcDsaVerify(nu1PointABase) = (nu1)BASE_ECDSAV_POINT_A;
    PUKCL_ZpEcDsaVerify(nu1PointPublicKeyGen) = (nu1)BASE_ECDSAV_PUBLIC_KEY;
    PUKCL_ZpEcDsaVerify(nu1PointSignature) = (nu1)BASE_ECDSAV_SIGNATURE;
    PUKCL_ZpEcDsaVerify(nu1OrderPointBase) = (nu1)BASE_ECDSAV_ORDER;
    PUKCL_ZpEcDsaVerify(nu1ABase) = (nu1)BASE_ECDSAV_A;
    PUKCL_ZpEcDsaVerify(nu1Workspace) = (nu1)BASE_ECDSAV_WORKSPACE;
    PUKCL_ZpEcDsaVerify(nu1HashBase) = (nu1)BASE_ECDSAV_HASH;
    PUKCL_ZpEcDsaVerify(u2ModLength) = PUKCC_ECC_PARAM_LEN;
    PUKCL_ZpEcDsaVerify(u2ScalarLength) = PUKCC_ECC_PARAM_LEN;
}

// Self-test required before using PUKCC
static void pukcc_self_test(void)
{
    static bool self_test_run = false;
    if (!self_test_run) {
        while ((PUKCCSR & BIT_PUKCCSR_CLRRAM_BUSY) != 0);
        memset(&PUKCLParam, 0, sizeof(PUKCL_PARAM));
        pvPUKCLParam = &PUKCLParam;
        vPUKCL_Process(SelfTest, pvPUKCLParam);
        while (PUKCL(u2Status) != PUKCL_OK);
        while (pvPUKCLParam->P.PUKCL_SelfTest.u4Version != PUKCL_VERSION);
        while (pvPUKCLParam->P.PUKCL_SelfTest.u4CheckNum1 != 0x6E70DDD2);
        while (pvPUKCLParam->P.PUKCL_SelfTest.u4CheckNum2 != 0x25C8D64F);
        self_test_run = true;
    }
}

// Returns 0 on success
uint8_t pukcc_ecdsa_verify(
    const uint8_t* public_key,
    const uint8_t* signature,
    const uint8_t* message,
    uint32_t message_len,
    PUKCC_CURVE_256_X curve)
{
    uint8_t hash[SHA256_DIGEST_LENGTH];
    pukcc_self_test();
    pukcc_sha256_compute(message, message_len, hash);
    // pukcc_normalize_signature(signature);// TODO
    pukcc_init_params_ecdsa_verification(signature, hash, public_key, curve);
    vPUKCL_Process(ZpEcDsaVerifyFast, pvPUKCLParam);
    return PUKCL(u2Status);
}

// Returns 0 on success
int32_t pukcc_sha256_compute(const uint8_t* message, uint32_t message_len, uint8_t* hash)
{
    if (!hash) {
        return ERR_INVALID_DATA;
    }
    pukcc_self_test();
    return sha_sync_sha256_compute(
        &HASH_ALGORITHM_0,
        &pukcc_sha256_context,
        false, /* SHA256 mode */
        message,
        message_len,
        hash);
}
