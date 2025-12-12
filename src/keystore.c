// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include "hardfault.h"
#include "keystore.h"
#include "memory/bitbox02_smarteeprom.h"
#include "memory/memory.h"
#include "reset.h"
#include "salt.h"
#include "securechip/securechip.h"
#include "util.h"
#include <usb/usb_processing.h>

#include <rust/rust.h>
#include <secp256k1_ecdsa_s2c.h>

bool keystore_get_bip39_word_stack(uint16_t idx, char* word_out, size_t word_out_size)
{
    return rust_get_bip39_word(idx, rust_util_bytes_mut((uint8_t*)word_out, word_out_size));
}

bool keystore_secp256k1_nonce_commit(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_commitment,
    uint8_t* signer_commitment_out)
{
    secp256k1_ecdsa_s2c_opening signer_commitment;
    if (!secp256k1_ecdsa_anti_exfil_signer_commit(
            ctx, &signer_commitment, msg32, private_key, host_commitment)) {
        return false;
    }

    if (!secp256k1_ecdsa_s2c_opening_serialize(ctx, signer_commitment_out, &signer_commitment)) {
        return false;
    }
    return true;
}

bool keystore_secp256k1_sign(
    const secp256k1_context* ctx,
    const uint8_t* private_key,
    const uint8_t* msg32,
    const uint8_t* host_nonce32,
    uint8_t* sig_compact_out,
    int* recid_out)
{
    secp256k1_ecdsa_signature secp256k1_sig = {0};
    if (!secp256k1_anti_exfil_sign(
            ctx, &secp256k1_sig, msg32, private_key, host_nonce32, recid_out)) {
        return false;
    }
    if (!secp256k1_ecdsa_signature_serialize_compact(ctx, sig_compact_out, &secp256k1_sig)) {
        return false;
    }
    return true;
}
