#include "eth_sighash.h"

#include <hardfault.h>

#include <sha3.h>

// https://github.com/ethereum/wiki/wiki/RLP
// If ctx is NULL, we skip the hashing.
// If encoded_len_out is NULL, we skip counting the bytes.
// If encoded_len_out is not NULL, we add to it, so it has to be initialized to 0 before the first
// call.
static void _hash_header(
    sha3_ctx* ctx,
    uint8_t small_tag,
    uint8_t large_tag,
    pb_size_t len,
    uint32_t* encoded_len_out)
{
    if (sizeof(len) != 2) {
        Abort("_hash_header: unexpected size");
    }
    // According to the RLP spec., buffer headers are encoded differently for lengths below and
    // above 55 (for >55, length of length is encoded).
    if (len <= 55) {
        if (ctx != NULL) {
            uint8_t byte = small_tag + len;
            rhash_sha3_update(ctx, &byte, 1);
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 1;
        }
    } else if (len <= 0xff) {
        if (ctx != NULL) {
            uint8_t encoding[2] = {large_tag + 1, len};
            rhash_sha3_update(ctx, encoding, sizeof(encoding));
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 2;
        }
    } else {
        if (ctx != NULL) {
            uint8_t byte = large_tag + 2;
            rhash_sha3_update(ctx, &byte, 1);
            // big endian serialization of 2-bytes `len`.
            rhash_sha3_update(ctx, (const uint8_t*)&len + 1, 1);
            rhash_sha3_update(ctx, (const uint8_t*)&len, 1);
        }
        if (encoded_len_out != NULL) {
            *encoded_len_out += 3;
        }
    }
}

// https://github.com/ethereum/wiki/wiki/RLP
// If ctx is NULL, we skip the hashing.
// If encoded_len_out is NULL, we skip counting the bytes.
// If encoded_len_out is not NULL, we add to it, so it has to be initialized to 0 before the first
// call.
static void _hash_element(
    sha3_ctx* ctx,
    const uint8_t* bytes,
    pb_size_t len,
    uint32_t* encoded_len_out)
{
    if (sizeof(len) != 2) {
        Abort("_hash_element: unexpected size");
    }
    // hash header
    if (len != 1 || bytes[0] > 0x7f) {
        _hash_header(ctx, 0x80, 0xb7, len, encoded_len_out);
    }
    if (ctx != NULL) {
        // hash bytes
        rhash_sha3_update(ctx, bytes, len);
    }
    if (encoded_len_out != NULL) {
        *encoded_len_out += len;
    }
}

bool app_eth_sighash(const ETHSignRequest* request, uint8_t chain_id, uint8_t* sighash_out)
{
    // We hash [nonce, gas price, gas limit, recipient, value, data], RLP encoded.
    // The list length prefix is (0xc0 + length of the encoding of all elements).
    // 1) calculate length
    uint32_t encoded_length = 0;
    _hash_element(NULL, request->nonce.bytes, request->nonce.size, &encoded_length);
    _hash_element(NULL, request->gas_price.bytes, request->gas_price.size, &encoded_length);
    _hash_element(NULL, request->gas_limit.bytes, request->gas_limit.size, &encoded_length);
    _hash_element(NULL, request->recipient, sizeof(request->recipient), &encoded_length);
    _hash_element(NULL, request->value.bytes, request->value.size, &encoded_length);
    _hash_element(NULL, request->data.bytes, request->data.size, &encoded_length);
    encoded_length += 3; // EIP155 part, see below.
    if (encoded_length > 0xffff) {
        // Don't support bigger than this for now.
        return false;
    }
    // 2) hash len and encoded tx elements
    sha3_ctx ctx;
    rhash_sha3_256_init(&ctx);
    _hash_header(&ctx, 0xc0, 0xf7, (pb_size_t)encoded_length, NULL);
    _hash_element(&ctx, request->nonce.bytes, request->nonce.size, NULL);
    _hash_element(&ctx, request->gas_price.bytes, request->gas_price.size, NULL);
    _hash_element(&ctx, request->gas_limit.bytes, request->gas_limit.size, NULL);
    _hash_element(&ctx, request->recipient, sizeof(request->recipient), NULL);
    _hash_element(&ctx, request->value.bytes, request->value.size, NULL);
    _hash_element(&ctx, request->data.bytes, request->data.size, NULL);
    { // EIP155
        if (chain_id == 0 || chain_id > 0x7f) {
            Abort("chain id encoding error");
        }
        // encodes <chainID><0><0>
        uint8_t eip155_part[3] = {chain_id, 0x80, 0x80};
        rhash_sha3_update(&ctx, eip155_part, sizeof(eip155_part));
    }
    rhash_keccak_final(&ctx, sighash_out);
    return true;
}
