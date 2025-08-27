/***********************************************************************
 * Copyright (c) 2013-2015 Pieter Wuille                               *
 * Distributed under the MIT software license, see the accompanying    *
 * file COPYING or https://www.opensource.org/licenses/mit-license.php.*
 ***********************************************************************/

/* This is a C project. It should not be compiled with a C++ compiler,
 * and we error out if we detect one.
 *
 * We still want to be able to test the project with a C++ compiler
 * because it is still good to know if this will lead to real trouble, so
 * there is a possibility to override the check. But be warned that
 * compiling with a C++ compiler is not supported. */
#if defined(__cplusplus) && !defined(SECP256K1_CPLUSPLUS_TEST_OVERRIDE)
#error Trying to compile a C project with a C++ compiler.
#endif

#define SECP256K1_BUILD

#include "../include/secp256k1.h"
#include "../include/secp256k1_preallocated.h"

#include "assumptions.h"
#include "checkmem.h"
#include "util.h"

#include "field_impl.h"
#include "scalar_impl.h"
#include "group_impl.h"
#include "ecmult_impl.h"
#include "ecmult_const_impl.h"
#include "ecmult_gen_impl.h"
#include "ecdsa_impl.h"
#include "eckey_impl.h"
#include "hash_impl.h"
#include "int128_impl.h"
#include "scratch_impl.h"
#include "selftest.h"

#ifdef SECP256K1_NO_BUILD
# error "secp256k1.h processed without SECP256K1_BUILD defined while building secp256k1.c"
#endif

#define ARG_CHECK(cond) do { \
    if (EXPECT(!(cond), 0)) { \
        rustsecp256k1_v0_10_0_callback_call(&ctx->illegal_callback, #cond); \
        return 0; \
    } \
} while(0)

#define ARG_CHECK_VOID(cond) do { \
    if (EXPECT(!(cond), 0)) { \
        rustsecp256k1_v0_10_0_callback_call(&ctx->illegal_callback, #cond); \
        return; \
    } \
} while(0)

/* Note that whenever you change the context struct, you must also change the
 * context_eq function. */
struct rustsecp256k1_v0_10_0_context_struct {
    rustsecp256k1_v0_10_0_ecmult_gen_context ecmult_gen_ctx;
    rustsecp256k1_v0_10_0_callback illegal_callback;
    rustsecp256k1_v0_10_0_callback error_callback;
    int declassify;
};

static const rustsecp256k1_v0_10_0_context rustsecp256k1_v0_10_0_context_static_ = {
    { 0 },
    { rustsecp256k1_v0_10_0_default_illegal_callback_fn, 0 },
    { rustsecp256k1_v0_10_0_default_error_callback_fn, 0 },
    0
};
const rustsecp256k1_v0_10_0_context *rustsecp256k1_v0_10_0_context_static = &rustsecp256k1_v0_10_0_context_static_;
const rustsecp256k1_v0_10_0_context *rustsecp256k1_v0_10_0_context_no_precomp = &rustsecp256k1_v0_10_0_context_static_;

/* Helper function that determines if a context is proper, i.e., is not the static context or a copy thereof.
 *
 * This is intended for "context" functions such as rustsecp256k1_v0_10_0_context_clone. Function which need specific
 * features of a context should still check for these features directly. For example, a function that needs
 * ecmult_gen should directly check for the existence of the ecmult_gen context. */
static int rustsecp256k1_v0_10_0_context_is_proper(const rustsecp256k1_v0_10_0_context* ctx) {
    return rustsecp256k1_v0_10_0_ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx);
}

void rustsecp256k1_v0_10_0_selftest(void) {
    if (!rustsecp256k1_v0_10_0_selftest_passes()) {
        rustsecp256k1_v0_10_0_callback_call(&default_error_callback, "self test failed");
    }
}

size_t rustsecp256k1_v0_10_0_context_preallocated_size(unsigned int flags) {
    size_t ret = sizeof(rustsecp256k1_v0_10_0_context);
    /* A return value of 0 is reserved as an indicator for errors when we call this function internally. */
    VERIFY_CHECK(ret != 0);

    if (EXPECT((flags & SECP256K1_FLAGS_TYPE_MASK) != SECP256K1_FLAGS_TYPE_CONTEXT, 0)) {
            rustsecp256k1_v0_10_0_callback_call(&default_illegal_callback,
                                    "Invalid flags");
            return 0;
    }

    if (EXPECT(!SECP256K1_CHECKMEM_RUNNING() && (flags & SECP256K1_FLAGS_BIT_CONTEXT_DECLASSIFY), 0)) {
            rustsecp256k1_v0_10_0_callback_call(&default_illegal_callback,
                                    "Declassify flag requires running with memory checking");
            return 0;
    }

    return ret;
}

size_t rustsecp256k1_v0_10_0_context_preallocated_clone_size(const rustsecp256k1_v0_10_0_context* ctx) {
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(rustsecp256k1_v0_10_0_context_is_proper(ctx));
    return sizeof(rustsecp256k1_v0_10_0_context);
}

rustsecp256k1_v0_10_0_context* rustsecp256k1_v0_10_0_context_preallocated_create(void* prealloc, unsigned int flags) {
    size_t prealloc_size;
    rustsecp256k1_v0_10_0_context* ret;

    rustsecp256k1_v0_10_0_selftest();

    prealloc_size = rustsecp256k1_v0_10_0_context_preallocated_size(flags);
    if (prealloc_size == 0) {
        return NULL;
    }
    VERIFY_CHECK(prealloc != NULL);
    ret = (rustsecp256k1_v0_10_0_context*)prealloc;
    ret->illegal_callback = default_illegal_callback;
    ret->error_callback = default_error_callback;

    /* Flags have been checked by rustsecp256k1_v0_10_0_context_preallocated_size. */
    VERIFY_CHECK((flags & SECP256K1_FLAGS_TYPE_MASK) == SECP256K1_FLAGS_TYPE_CONTEXT);
    rustsecp256k1_v0_10_0_ecmult_gen_context_build(&ret->ecmult_gen_ctx);
    ret->declassify = !!(flags & SECP256K1_FLAGS_BIT_CONTEXT_DECLASSIFY);

    return ret;
}

rustsecp256k1_v0_10_0_context* rustsecp256k1_v0_10_0_context_preallocated_clone(const rustsecp256k1_v0_10_0_context* ctx, void* prealloc) {
    rustsecp256k1_v0_10_0_context* ret;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(prealloc != NULL);
    ARG_CHECK(rustsecp256k1_v0_10_0_context_is_proper(ctx));

    ret = (rustsecp256k1_v0_10_0_context*)prealloc;
    *ret = *ctx;
    return ret;
}

void rustsecp256k1_v0_10_0_context_preallocated_destroy(rustsecp256k1_v0_10_0_context* ctx) {
    ARG_CHECK_VOID(ctx == NULL || rustsecp256k1_v0_10_0_context_is_proper(ctx));

    /* Defined as noop */
    if (ctx == NULL) {
        return;
    }

    rustsecp256k1_v0_10_0_ecmult_gen_context_clear(&ctx->ecmult_gen_ctx);
}

void rustsecp256k1_v0_10_0_context_set_illegal_callback(rustsecp256k1_v0_10_0_context* ctx, void (*fun)(const char* message, void* data), const void* data) {
    /* We compare pointers instead of checking rustsecp256k1_v0_10_0_context_is_proper() here
       because setting callbacks is allowed on *copies* of the static context:
       it's harmless and makes testing easier. */
    ARG_CHECK_VOID(ctx != rustsecp256k1_v0_10_0_context_static);
    if (fun == NULL) {
        fun = rustsecp256k1_v0_10_0_default_illegal_callback_fn;
    }
    ctx->illegal_callback.fn = fun;
    ctx->illegal_callback.data = data;
}

void rustsecp256k1_v0_10_0_context_set_error_callback(rustsecp256k1_v0_10_0_context* ctx, void (*fun)(const char* message, void* data), const void* data) {
    /* We compare pointers instead of checking rustsecp256k1_v0_10_0_context_is_proper() here
       because setting callbacks is allowed on *copies* of the static context:
       it's harmless and makes testing easier. */
    ARG_CHECK_VOID(ctx != rustsecp256k1_v0_10_0_context_static);
    if (fun == NULL) {
        fun = rustsecp256k1_v0_10_0_default_error_callback_fn;
    }
    ctx->error_callback.fn = fun;
    ctx->error_callback.data = data;
}

/* Mark memory as no-longer-secret for the purpose of analysing constant-time behaviour
 *  of the software.
 */
static SECP256K1_INLINE void rustsecp256k1_v0_10_0_declassify(const rustsecp256k1_v0_10_0_context* ctx, const void *p, size_t len) {
    if (EXPECT(ctx->declassify, 0)) SECP256K1_CHECKMEM_DEFINE(p, len);
}

static int rustsecp256k1_v0_10_0_pubkey_load(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_ge* ge, const rustsecp256k1_v0_10_0_pubkey* pubkey) {
    if (sizeof(rustsecp256k1_v0_10_0_ge_storage) == 64) {
        /* When the rustsecp256k1_v0_10_0_ge_storage type is exactly 64 byte, use its
         * representation inside rustsecp256k1_v0_10_0_pubkey, as conversion is very fast.
         * Note that rustsecp256k1_v0_10_0_pubkey_save must use the same representation. */
        rustsecp256k1_v0_10_0_ge_storage s;
        memcpy(&s, &pubkey->data[0], sizeof(s));
        rustsecp256k1_v0_10_0_ge_from_storage(ge, &s);
    } else {
        /* Otherwise, fall back to 32-byte big endian for X and Y. */
        rustsecp256k1_v0_10_0_fe x, y;
        ARG_CHECK(rustsecp256k1_v0_10_0_fe_set_b32_limit(&x, pubkey->data));
        ARG_CHECK(rustsecp256k1_v0_10_0_fe_set_b32_limit(&y, pubkey->data + 32));
        rustsecp256k1_v0_10_0_ge_set_xy(ge, &x, &y);
    }
    ARG_CHECK(!rustsecp256k1_v0_10_0_fe_is_zero(&ge->x));
    return 1;
}

static void rustsecp256k1_v0_10_0_pubkey_save(rustsecp256k1_v0_10_0_pubkey* pubkey, rustsecp256k1_v0_10_0_ge* ge) {
    if (sizeof(rustsecp256k1_v0_10_0_ge_storage) == 64) {
        rustsecp256k1_v0_10_0_ge_storage s;
        rustsecp256k1_v0_10_0_ge_to_storage(&s, ge);
        memcpy(&pubkey->data[0], &s, sizeof(s));
    } else {
        VERIFY_CHECK(!rustsecp256k1_v0_10_0_ge_is_infinity(ge));
        rustsecp256k1_v0_10_0_fe_normalize_var(&ge->x);
        rustsecp256k1_v0_10_0_fe_normalize_var(&ge->y);
        rustsecp256k1_v0_10_0_fe_get_b32(pubkey->data, &ge->x);
        rustsecp256k1_v0_10_0_fe_get_b32(pubkey->data + 32, &ge->y);
    }
}

int rustsecp256k1_v0_10_0_ec_pubkey_parse(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey* pubkey, const unsigned char *input, size_t inputlen) {
    rustsecp256k1_v0_10_0_ge Q;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubkey != NULL);
    memset(pubkey, 0, sizeof(*pubkey));
    ARG_CHECK(input != NULL);
    if (!rustsecp256k1_v0_10_0_eckey_pubkey_parse(&Q, input, inputlen)) {
        return 0;
    }
    if (!rustsecp256k1_v0_10_0_ge_is_in_correct_subgroup(&Q)) {
        return 0;
    }
    rustsecp256k1_v0_10_0_pubkey_save(pubkey, &Q);
    rustsecp256k1_v0_10_0_ge_clear(&Q);
    return 1;
}

int rustsecp256k1_v0_10_0_ec_pubkey_serialize(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *output, size_t *outputlen, const rustsecp256k1_v0_10_0_pubkey* pubkey, unsigned int flags) {
    rustsecp256k1_v0_10_0_ge Q;
    size_t len;
    int ret = 0;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(outputlen != NULL);
    ARG_CHECK(*outputlen >= ((flags & SECP256K1_FLAGS_BIT_COMPRESSION) ? 33u : 65u));
    len = *outputlen;
    *outputlen = 0;
    ARG_CHECK(output != NULL);
    memset(output, 0, len);
    ARG_CHECK(pubkey != NULL);
    ARG_CHECK((flags & SECP256K1_FLAGS_TYPE_MASK) == SECP256K1_FLAGS_TYPE_COMPRESSION);
    if (rustsecp256k1_v0_10_0_pubkey_load(ctx, &Q, pubkey)) {
        ret = rustsecp256k1_v0_10_0_eckey_pubkey_serialize(&Q, output, &len, flags & SECP256K1_FLAGS_BIT_COMPRESSION);
        if (ret) {
            *outputlen = len;
        }
    }
    return ret;
}

int rustsecp256k1_v0_10_0_ec_pubkey_cmp(const rustsecp256k1_v0_10_0_context* ctx, const rustsecp256k1_v0_10_0_pubkey* pubkey0, const rustsecp256k1_v0_10_0_pubkey* pubkey1) {
    unsigned char out[2][33];
    const rustsecp256k1_v0_10_0_pubkey* pk[2];
    int i;

    VERIFY_CHECK(ctx != NULL);
    pk[0] = pubkey0; pk[1] = pubkey1;
    for (i = 0; i < 2; i++) {
        size_t out_size = sizeof(out[i]);
        /* If the public key is NULL or invalid, ec_pubkey_serialize will call
         * the illegal_callback and return 0. In that case we will serialize the
         * key as all zeros which is less than any valid public key. This
         * results in consistent comparisons even if NULL or invalid pubkeys are
         * involved and prevents edge cases such as sorting algorithms that use
         * this function and do not terminate as a result. */
        if (!rustsecp256k1_v0_10_0_ec_pubkey_serialize(ctx, out[i], &out_size, pk[i], SECP256K1_EC_COMPRESSED)) {
            /* Note that ec_pubkey_serialize should already set the output to
             * zero in that case, but it's not guaranteed by the API, we can't
             * test it and writing a VERIFY_CHECK is more complex than
             * explicitly memsetting (again). */
            memset(out[i], 0, sizeof(out[i]));
        }
    }
    return rustsecp256k1_v0_10_0_memcmp_var(out[0], out[1], sizeof(out[0]));
}

static void rustsecp256k1_v0_10_0_ecdsa_signature_load(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_scalar* r, rustsecp256k1_v0_10_0_scalar* s, const rustsecp256k1_v0_10_0_ecdsa_signature* sig) {
    (void)ctx;
    if (sizeof(rustsecp256k1_v0_10_0_scalar) == 32) {
        /* When the rustsecp256k1_v0_10_0_scalar type is exactly 32 byte, use its
         * representation inside rustsecp256k1_v0_10_0_ecdsa_signature, as conversion is very fast.
         * Note that rustsecp256k1_v0_10_0_ecdsa_signature_save must use the same representation. */
        memcpy(r, &sig->data[0], 32);
        memcpy(s, &sig->data[32], 32);
    } else {
        rustsecp256k1_v0_10_0_scalar_set_b32(r, &sig->data[0], NULL);
        rustsecp256k1_v0_10_0_scalar_set_b32(s, &sig->data[32], NULL);
    }
}

static void rustsecp256k1_v0_10_0_ecdsa_signature_save(rustsecp256k1_v0_10_0_ecdsa_signature* sig, const rustsecp256k1_v0_10_0_scalar* r, const rustsecp256k1_v0_10_0_scalar* s) {
    if (sizeof(rustsecp256k1_v0_10_0_scalar) == 32) {
        memcpy(&sig->data[0], r, 32);
        memcpy(&sig->data[32], s, 32);
    } else {
        rustsecp256k1_v0_10_0_scalar_get_b32(&sig->data[0], r);
        rustsecp256k1_v0_10_0_scalar_get_b32(&sig->data[32], s);
    }
}

int rustsecp256k1_v0_10_0_ecdsa_signature_parse_der(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_ecdsa_signature* sig, const unsigned char *input, size_t inputlen) {
    rustsecp256k1_v0_10_0_scalar r, s;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(sig != NULL);
    ARG_CHECK(input != NULL);

    if (rustsecp256k1_v0_10_0_ecdsa_sig_parse(&r, &s, input, inputlen)) {
        rustsecp256k1_v0_10_0_ecdsa_signature_save(sig, &r, &s);
        return 1;
    } else {
        memset(sig, 0, sizeof(*sig));
        return 0;
    }
}

int rustsecp256k1_v0_10_0_ecdsa_signature_parse_compact(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_ecdsa_signature* sig, const unsigned char *input64) {
    rustsecp256k1_v0_10_0_scalar r, s;
    int ret = 1;
    int overflow = 0;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(sig != NULL);
    ARG_CHECK(input64 != NULL);

    rustsecp256k1_v0_10_0_scalar_set_b32(&r, &input64[0], &overflow);
    ret &= !overflow;
    rustsecp256k1_v0_10_0_scalar_set_b32(&s, &input64[32], &overflow);
    ret &= !overflow;
    if (ret) {
        rustsecp256k1_v0_10_0_ecdsa_signature_save(sig, &r, &s);
    } else {
        memset(sig, 0, sizeof(*sig));
    }
    return ret;
}

int rustsecp256k1_v0_10_0_ecdsa_signature_serialize_der(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *output, size_t *outputlen, const rustsecp256k1_v0_10_0_ecdsa_signature* sig) {
    rustsecp256k1_v0_10_0_scalar r, s;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(output != NULL);
    ARG_CHECK(outputlen != NULL);
    ARG_CHECK(sig != NULL);

    rustsecp256k1_v0_10_0_ecdsa_signature_load(ctx, &r, &s, sig);
    return rustsecp256k1_v0_10_0_ecdsa_sig_serialize(output, outputlen, &r, &s);
}

int rustsecp256k1_v0_10_0_ecdsa_signature_serialize_compact(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *output64, const rustsecp256k1_v0_10_0_ecdsa_signature* sig) {
    rustsecp256k1_v0_10_0_scalar r, s;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(output64 != NULL);
    ARG_CHECK(sig != NULL);

    rustsecp256k1_v0_10_0_ecdsa_signature_load(ctx, &r, &s, sig);
    rustsecp256k1_v0_10_0_scalar_get_b32(&output64[0], &r);
    rustsecp256k1_v0_10_0_scalar_get_b32(&output64[32], &s);
    return 1;
}

int rustsecp256k1_v0_10_0_ecdsa_signature_normalize(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_ecdsa_signature *sigout, const rustsecp256k1_v0_10_0_ecdsa_signature *sigin) {
    rustsecp256k1_v0_10_0_scalar r, s;
    int ret = 0;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(sigin != NULL);

    rustsecp256k1_v0_10_0_ecdsa_signature_load(ctx, &r, &s, sigin);
    ret = rustsecp256k1_v0_10_0_scalar_is_high(&s);
    if (sigout != NULL) {
        if (ret) {
            rustsecp256k1_v0_10_0_scalar_negate(&s, &s);
        }
        rustsecp256k1_v0_10_0_ecdsa_signature_save(sigout, &r, &s);
    }

    return ret;
}

int rustsecp256k1_v0_10_0_ecdsa_verify(const rustsecp256k1_v0_10_0_context* ctx, const rustsecp256k1_v0_10_0_ecdsa_signature *sig, const unsigned char *msghash32, const rustsecp256k1_v0_10_0_pubkey *pubkey) {
    rustsecp256k1_v0_10_0_ge q;
    rustsecp256k1_v0_10_0_scalar r, s;
    rustsecp256k1_v0_10_0_scalar m;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(msghash32 != NULL);
    ARG_CHECK(sig != NULL);
    ARG_CHECK(pubkey != NULL);

    rustsecp256k1_v0_10_0_scalar_set_b32(&m, msghash32, NULL);
    rustsecp256k1_v0_10_0_ecdsa_signature_load(ctx, &r, &s, sig);
    return (!rustsecp256k1_v0_10_0_scalar_is_high(&s) &&
            rustsecp256k1_v0_10_0_pubkey_load(ctx, &q, pubkey) &&
            rustsecp256k1_v0_10_0_ecdsa_sig_verify(&r, &s, &q, &m));
}

static SECP256K1_INLINE void buffer_append(unsigned char *buf, unsigned int *offset, const void *data, unsigned int len) {
    memcpy(buf + *offset, data, len);
    *offset += len;
}

static int nonce_function_rfc6979(unsigned char *nonce32, const unsigned char *msg32, const unsigned char *key32, const unsigned char *algo16, void *data, unsigned int counter) {
   unsigned char keydata[112];
   unsigned int offset = 0;
   rustsecp256k1_v0_10_0_rfc6979_hmac_sha256 rng;
   unsigned int i;
   rustsecp256k1_v0_10_0_scalar msg;
   unsigned char msgmod32[32];
   rustsecp256k1_v0_10_0_scalar_set_b32(&msg, msg32, NULL);
   rustsecp256k1_v0_10_0_scalar_get_b32(msgmod32, &msg);
   /* We feed a byte array to the PRNG as input, consisting of:
    * - the private key (32 bytes) and reduced message (32 bytes), see RFC 6979 3.2d.
    * - optionally 32 extra bytes of data, see RFC 6979 3.6 Additional Data.
    * - optionally 16 extra bytes with the algorithm name.
    * Because the arguments have distinct fixed lengths it is not possible for
    *  different argument mixtures to emulate each other and result in the same
    *  nonces.
    */
   buffer_append(keydata, &offset, key32, 32);
   buffer_append(keydata, &offset, msgmod32, 32);
   if (data != NULL) {
       buffer_append(keydata, &offset, data, 32);
   }
   if (algo16 != NULL) {
       buffer_append(keydata, &offset, algo16, 16);
   }
   rustsecp256k1_v0_10_0_rfc6979_hmac_sha256_initialize(&rng, keydata, offset);
   memset(keydata, 0, sizeof(keydata));
   for (i = 0; i <= counter; i++) {
       rustsecp256k1_v0_10_0_rfc6979_hmac_sha256_generate(&rng, nonce32, 32);
   }
   rustsecp256k1_v0_10_0_rfc6979_hmac_sha256_finalize(&rng);
   return 1;
}

const rustsecp256k1_v0_10_0_nonce_function rustsecp256k1_v0_10_0_nonce_function_rfc6979 = nonce_function_rfc6979;
const rustsecp256k1_v0_10_0_nonce_function rustsecp256k1_v0_10_0_nonce_function_default = nonce_function_rfc6979;

static int rustsecp256k1_v0_10_0_ecdsa_sign_inner(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_scalar* r, rustsecp256k1_v0_10_0_scalar* s, int* recid, const unsigned char *msg32, const unsigned char *seckey, rustsecp256k1_v0_10_0_nonce_function noncefp, const void* noncedata) {
    rustsecp256k1_v0_10_0_scalar sec, non, msg;
    int ret = 0;
    int is_sec_valid;
    unsigned char nonce32[32];
    unsigned int count = 0;
    /* Default initialization here is important so we won't pass uninit values to the cmov in the end */
    *r = rustsecp256k1_v0_10_0_scalar_zero;
    *s = rustsecp256k1_v0_10_0_scalar_zero;
    if (recid) {
        *recid = 0;
    }
    if (noncefp == NULL) {
        noncefp = rustsecp256k1_v0_10_0_nonce_function_default;
    }

    /* Fail if the secret key is invalid. */
    is_sec_valid = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&sec, seckey);
    rustsecp256k1_v0_10_0_scalar_cmov(&sec, &rustsecp256k1_v0_10_0_scalar_one, !is_sec_valid);
    rustsecp256k1_v0_10_0_scalar_set_b32(&msg, msg32, NULL);
    while (1) {
        int is_nonce_valid;
        ret = !!noncefp(nonce32, msg32, seckey, NULL, (void*)noncedata, count);
        if (!ret) {
            break;
        }
        is_nonce_valid = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&non, nonce32);
        /* The nonce is still secret here, but it being invalid is is less likely than 1:2^255. */
        rustsecp256k1_v0_10_0_declassify(ctx, &is_nonce_valid, sizeof(is_nonce_valid));
        if (is_nonce_valid) {
            ret = rustsecp256k1_v0_10_0_ecdsa_sig_sign(&ctx->ecmult_gen_ctx, r, s, &sec, &msg, &non, recid);
            /* The final signature is no longer a secret, nor is the fact that we were successful or not. */
            rustsecp256k1_v0_10_0_declassify(ctx, &ret, sizeof(ret));
            if (ret) {
                break;
            }
        }
        count++;
    }
    /* We don't want to declassify is_sec_valid and therefore the range of
     * seckey. As a result is_sec_valid is included in ret only after ret was
     * used as a branching variable. */
    ret &= is_sec_valid;
    memset(nonce32, 0, 32);
    rustsecp256k1_v0_10_0_scalar_clear(&msg);
    rustsecp256k1_v0_10_0_scalar_clear(&non);
    rustsecp256k1_v0_10_0_scalar_clear(&sec);
    rustsecp256k1_v0_10_0_scalar_cmov(r, &rustsecp256k1_v0_10_0_scalar_zero, !ret);
    rustsecp256k1_v0_10_0_scalar_cmov(s, &rustsecp256k1_v0_10_0_scalar_zero, !ret);
    if (recid) {
        const int zero = 0;
        rustsecp256k1_v0_10_0_int_cmov(recid, &zero, !ret);
    }
    return ret;
}

int rustsecp256k1_v0_10_0_ecdsa_sign(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_ecdsa_signature *signature, const unsigned char *msghash32, const unsigned char *seckey, rustsecp256k1_v0_10_0_nonce_function noncefp, const void* noncedata) {
    rustsecp256k1_v0_10_0_scalar r, s;
    int ret;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(rustsecp256k1_v0_10_0_ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
    ARG_CHECK(msghash32 != NULL);
    ARG_CHECK(signature != NULL);
    ARG_CHECK(seckey != NULL);

    ret = rustsecp256k1_v0_10_0_ecdsa_sign_inner(ctx, &r, &s, NULL, msghash32, seckey, noncefp, noncedata);
    rustsecp256k1_v0_10_0_ecdsa_signature_save(signature, &r, &s);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_seckey_verify(const rustsecp256k1_v0_10_0_context* ctx, const unsigned char *seckey) {
    rustsecp256k1_v0_10_0_scalar sec;
    int ret;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(seckey != NULL);

    ret = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&sec, seckey);
    rustsecp256k1_v0_10_0_scalar_clear(&sec);
    return ret;
}

static int rustsecp256k1_v0_10_0_ec_pubkey_create_helper(const rustsecp256k1_v0_10_0_ecmult_gen_context *ecmult_gen_ctx, rustsecp256k1_v0_10_0_scalar *seckey_scalar, rustsecp256k1_v0_10_0_ge *p, const unsigned char *seckey) {
    rustsecp256k1_v0_10_0_gej pj;
    int ret;

    ret = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(seckey_scalar, seckey);
    rustsecp256k1_v0_10_0_scalar_cmov(seckey_scalar, &rustsecp256k1_v0_10_0_scalar_one, !ret);

    rustsecp256k1_v0_10_0_ecmult_gen(ecmult_gen_ctx, &pj, seckey_scalar);
    rustsecp256k1_v0_10_0_ge_set_gej(p, &pj);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_pubkey_create(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey *pubkey, const unsigned char *seckey) {
    rustsecp256k1_v0_10_0_ge p;
    rustsecp256k1_v0_10_0_scalar seckey_scalar;
    int ret = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubkey != NULL);
    memset(pubkey, 0, sizeof(*pubkey));
    ARG_CHECK(rustsecp256k1_v0_10_0_ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx));
    ARG_CHECK(seckey != NULL);

    ret = rustsecp256k1_v0_10_0_ec_pubkey_create_helper(&ctx->ecmult_gen_ctx, &seckey_scalar, &p, seckey);
    rustsecp256k1_v0_10_0_pubkey_save(pubkey, &p);
    rustsecp256k1_v0_10_0_memczero(pubkey, sizeof(*pubkey), !ret);

    rustsecp256k1_v0_10_0_scalar_clear(&seckey_scalar);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_seckey_negate(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey) {
    rustsecp256k1_v0_10_0_scalar sec;
    int ret = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(seckey != NULL);

    ret = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&sec, seckey);
    rustsecp256k1_v0_10_0_scalar_cmov(&sec, &rustsecp256k1_v0_10_0_scalar_zero, !ret);
    rustsecp256k1_v0_10_0_scalar_negate(&sec, &sec);
    rustsecp256k1_v0_10_0_scalar_get_b32(seckey, &sec);

    rustsecp256k1_v0_10_0_scalar_clear(&sec);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_privkey_negate(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey) {
    return rustsecp256k1_v0_10_0_ec_seckey_negate(ctx, seckey);
}

int rustsecp256k1_v0_10_0_ec_pubkey_negate(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey *pubkey) {
    int ret = 0;
    rustsecp256k1_v0_10_0_ge p;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubkey != NULL);

    ret = rustsecp256k1_v0_10_0_pubkey_load(ctx, &p, pubkey);
    memset(pubkey, 0, sizeof(*pubkey));
    if (ret) {
        rustsecp256k1_v0_10_0_ge_neg(&p, &p);
        rustsecp256k1_v0_10_0_pubkey_save(pubkey, &p);
    }
    return ret;
}


static int rustsecp256k1_v0_10_0_ec_seckey_tweak_add_helper(rustsecp256k1_v0_10_0_scalar *sec, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_scalar term;
    int overflow = 0;
    int ret = 0;

    rustsecp256k1_v0_10_0_scalar_set_b32(&term, tweak32, &overflow);
    ret = (!overflow) & rustsecp256k1_v0_10_0_eckey_privkey_tweak_add(sec, &term);
    rustsecp256k1_v0_10_0_scalar_clear(&term);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_seckey_tweak_add(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_scalar sec;
    int ret = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(seckey != NULL);
    ARG_CHECK(tweak32 != NULL);

    ret = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&sec, seckey);
    ret &= rustsecp256k1_v0_10_0_ec_seckey_tweak_add_helper(&sec, tweak32);
    rustsecp256k1_v0_10_0_scalar_cmov(&sec, &rustsecp256k1_v0_10_0_scalar_zero, !ret);
    rustsecp256k1_v0_10_0_scalar_get_b32(seckey, &sec);

    rustsecp256k1_v0_10_0_scalar_clear(&sec);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_privkey_tweak_add(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey, const unsigned char *tweak32) {
    return rustsecp256k1_v0_10_0_ec_seckey_tweak_add(ctx, seckey, tweak32);
}

static int rustsecp256k1_v0_10_0_ec_pubkey_tweak_add_helper(rustsecp256k1_v0_10_0_ge *p, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_scalar term;
    int overflow = 0;
    rustsecp256k1_v0_10_0_scalar_set_b32(&term, tweak32, &overflow);
    return !overflow && rustsecp256k1_v0_10_0_eckey_pubkey_tweak_add(p, &term);
}

int rustsecp256k1_v0_10_0_ec_pubkey_tweak_add(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey *pubkey, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_ge p;
    int ret = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubkey != NULL);
    ARG_CHECK(tweak32 != NULL);

    ret = rustsecp256k1_v0_10_0_pubkey_load(ctx, &p, pubkey);
    memset(pubkey, 0, sizeof(*pubkey));
    ret = ret && rustsecp256k1_v0_10_0_ec_pubkey_tweak_add_helper(&p, tweak32);
    if (ret) {
        rustsecp256k1_v0_10_0_pubkey_save(pubkey, &p);
    }

    return ret;
}

int rustsecp256k1_v0_10_0_ec_seckey_tweak_mul(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_scalar factor;
    rustsecp256k1_v0_10_0_scalar sec;
    int ret = 0;
    int overflow = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(seckey != NULL);
    ARG_CHECK(tweak32 != NULL);

    rustsecp256k1_v0_10_0_scalar_set_b32(&factor, tweak32, &overflow);
    ret = rustsecp256k1_v0_10_0_scalar_set_b32_seckey(&sec, seckey);
    ret &= (!overflow) & rustsecp256k1_v0_10_0_eckey_privkey_tweak_mul(&sec, &factor);
    rustsecp256k1_v0_10_0_scalar_cmov(&sec, &rustsecp256k1_v0_10_0_scalar_zero, !ret);
    rustsecp256k1_v0_10_0_scalar_get_b32(seckey, &sec);

    rustsecp256k1_v0_10_0_scalar_clear(&sec);
    rustsecp256k1_v0_10_0_scalar_clear(&factor);
    return ret;
}

int rustsecp256k1_v0_10_0_ec_privkey_tweak_mul(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *seckey, const unsigned char *tweak32) {
    return rustsecp256k1_v0_10_0_ec_seckey_tweak_mul(ctx, seckey, tweak32);
}

int rustsecp256k1_v0_10_0_ec_pubkey_tweak_mul(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey *pubkey, const unsigned char *tweak32) {
    rustsecp256k1_v0_10_0_ge p;
    rustsecp256k1_v0_10_0_scalar factor;
    int ret = 0;
    int overflow = 0;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubkey != NULL);
    ARG_CHECK(tweak32 != NULL);

    rustsecp256k1_v0_10_0_scalar_set_b32(&factor, tweak32, &overflow);
    ret = !overflow && rustsecp256k1_v0_10_0_pubkey_load(ctx, &p, pubkey);
    memset(pubkey, 0, sizeof(*pubkey));
    if (ret) {
        if (rustsecp256k1_v0_10_0_eckey_pubkey_tweak_mul(&p, &factor)) {
            rustsecp256k1_v0_10_0_pubkey_save(pubkey, &p);
        } else {
            ret = 0;
        }
    }

    return ret;
}

int rustsecp256k1_v0_10_0_context_randomize(rustsecp256k1_v0_10_0_context* ctx, const unsigned char *seed32) {
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(rustsecp256k1_v0_10_0_context_is_proper(ctx));

    if (rustsecp256k1_v0_10_0_ecmult_gen_context_is_built(&ctx->ecmult_gen_ctx)) {
        rustsecp256k1_v0_10_0_ecmult_gen_blind(&ctx->ecmult_gen_ctx, seed32);
    }
    return 1;
}

int rustsecp256k1_v0_10_0_ec_pubkey_combine(const rustsecp256k1_v0_10_0_context* ctx, rustsecp256k1_v0_10_0_pubkey *pubnonce, const rustsecp256k1_v0_10_0_pubkey * const *pubnonces, size_t n) {
    size_t i;
    rustsecp256k1_v0_10_0_gej Qj;
    rustsecp256k1_v0_10_0_ge Q;

    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(pubnonce != NULL);
    memset(pubnonce, 0, sizeof(*pubnonce));
    ARG_CHECK(n >= 1);
    ARG_CHECK(pubnonces != NULL);

    rustsecp256k1_v0_10_0_gej_set_infinity(&Qj);

    for (i = 0; i < n; i++) {
        ARG_CHECK(pubnonces[i] != NULL);
        rustsecp256k1_v0_10_0_pubkey_load(ctx, &Q, pubnonces[i]);
        rustsecp256k1_v0_10_0_gej_add_ge(&Qj, &Qj, &Q);
    }
    if (rustsecp256k1_v0_10_0_gej_is_infinity(&Qj)) {
        return 0;
    }
    rustsecp256k1_v0_10_0_ge_set_gej(&Q, &Qj);
    rustsecp256k1_v0_10_0_pubkey_save(pubnonce, &Q);
    return 1;
}

int rustsecp256k1_v0_10_0_tagged_sha256(const rustsecp256k1_v0_10_0_context* ctx, unsigned char *hash32, const unsigned char *tag, size_t taglen, const unsigned char *msg, size_t msglen) {
    rustsecp256k1_v0_10_0_sha256 sha;
    VERIFY_CHECK(ctx != NULL);
    ARG_CHECK(hash32 != NULL);
    ARG_CHECK(tag != NULL);
    ARG_CHECK(msg != NULL);

    rustsecp256k1_v0_10_0_sha256_initialize_tagged(&sha, tag, taglen);
    rustsecp256k1_v0_10_0_sha256_write(&sha, msg, msglen);
    rustsecp256k1_v0_10_0_sha256_finalize(&sha, hash32);
    return 1;
}

#ifdef ENABLE_MODULE_ECDH
# include "modules/ecdh/main_impl.h"
#endif

#ifdef ENABLE_MODULE_RECOVERY
# include "modules/recovery/main_impl.h"
#endif

#ifdef ENABLE_MODULE_EXTRAKEYS
# include "modules/extrakeys/main_impl.h"
#endif

#ifdef ENABLE_MODULE_SCHNORRSIG
# include "modules/schnorrsig/main_impl.h"
#endif

#ifdef ENABLE_MODULE_ELLSWIFT
# include "modules/ellswift/main_impl.h"
#endif
