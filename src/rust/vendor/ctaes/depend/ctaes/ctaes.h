 /*********************************************************************
 * Copyright (c) 2016 Pieter Wuille                                   *
 * Distributed under the MIT software license, see the accompanying   *
 * file COPYING or https://opensource.org/licenses/mit-license.php.   *
 **********************************************************************/

#ifndef CTAES_H
#define CTAES_H

#include <stdint.h>
#include <stdlib.h>

typedef struct {
    uint16_t slice[8];
} AES_state;

typedef struct {
    AES_state rk[11];
} AES128_ctx;

typedef struct {
    AES_state rk[13];
} AES192_ctx;

typedef struct {
    AES_state rk[15];
} AES256_ctx;

typedef struct {
    AES128_ctx ctx;
    uint8_t iv[16]; /* iv is updated after each use */
} AES128_CBC_ctx;

typedef struct {
    AES192_ctx ctx;
    uint8_t iv[16]; /* iv is updated after each use */
} AES192_CBC_ctx;

typedef struct {
    AES256_ctx ctx;
    uint8_t iv[16]; /* iv is updated after each use */
} AES256_CBC_ctx;

void AES128_init(AES128_ctx* ctx, const unsigned char* key16);
void AES128_encrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
void AES128_decrypt(const AES128_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);

void AES192_init(AES192_ctx* ctx, const unsigned char* key24);
void AES192_encrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
void AES192_decrypt(const AES192_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);

void AES256_init(AES256_ctx* ctx, const unsigned char* key32);
void AES256_encrypt(const AES256_ctx* ctx, size_t blocks, unsigned char* cipher16, const unsigned char* plain16);
void AES256_decrypt(const AES256_ctx* ctx, size_t blocks, unsigned char* plain16, const unsigned char* cipher16);

void AES128_CBC_init(AES128_CBC_ctx* ctx, const unsigned char* key16, const uint8_t* iv);
void AES128_CBC_encrypt(AES128_CBC_ctx* ctx, size_t blocks, unsigned char* encrypted, const unsigned char* plain);
void AES128_CBC_decrypt(AES128_CBC_ctx* ctx, size_t blocks, unsigned char* plain, const unsigned char *encrypted);

void AES192_CBC_init(AES192_CBC_ctx* ctx, const unsigned char* key16, const uint8_t* iv);
void AES192_CBC_encrypt(AES192_CBC_ctx* ctx, size_t blocks, unsigned char* encrypted, const unsigned char* plain);
void AES192_CBC_decrypt(AES192_CBC_ctx* ctx, size_t blocks, unsigned char* plain, const unsigned char *encrypted);

void AES256_CBC_init(AES256_CBC_ctx* ctx, const unsigned char* key16, const uint8_t* iv);
void AES256_CBC_encrypt(AES256_CBC_ctx* ctx, size_t blocks, unsigned char* encrypted, const unsigned char* plain);
void AES256_CBC_decrypt(AES256_CBC_ctx* ctx, size_t blocks, unsigned char* plain, const unsigned char *encrypted);

#endif /* CTAES_H */
