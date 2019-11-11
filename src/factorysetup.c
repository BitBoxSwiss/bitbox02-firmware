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

#include "common_main.h"
#include "driver_init.h"
#include "flags.h"
#include "hardfault.h"
#include "memory/memory.h"
#include "platform_init.h"
#include "screen.h"
#include "securechip/securechip.h"
#include "ui/screen_process.h"
#include "usart/usart.h"
#include "usb/usb.h"
#include "usb/usb_packet.h"
#include "usb/usb_processing.h"
#include <secp256k1.h>
#include <wally_crypto.h>

#define FACTORYSETUP_CMD (HID_VENDOR_FIRST + 0x02) // factory setup commands

// 65 bytes uncompressed secp256k1 root attestation pubkey.
// batch #0
#define ROOT_PUBKEY_SIZE 65
static uint8_t _root_pubkey_bytes[6][ROOT_PUBKEY_SIZE] = {
    {
        0x04, 0x07, 0x4f, 0xf1, 0x27, 0x3b, 0x36, 0xc2, 0x4e, 0x80, 0xfe, 0x3d, 0x59,
        0xe0, 0xe8, 0x97, 0xa8, 0x17, 0x32, 0xd3, 0xf8, 0xe9, 0xcd, 0x07, 0xe1, 0x7e,
        0x9f, 0xc0, 0x63, 0x19, 0xcd, 0x16, 0xb2, 0x5c, 0xf7, 0x42, 0x55, 0x67, 0x44,
        0x77, 0xb3, 0xac, 0x9c, 0xba, 0xc2, 0xd1, 0x2f, 0x0d, 0xc2, 0x7a, 0x66, 0x26,
        0x81, 0xfc, 0xbc, 0x12, 0x95, 0x5b, 0x0b, 0xcc, 0xdc, 0xbb, 0xdc, 0xfd, 0x01,
    },
    {
        0x04, 0x4c, 0x53, 0xa8, 0x4f, 0x41, 0xfa, 0x73, 0x01, 0xb3, 0x78, 0xbb, 0x3c,
        0x26, 0x0f, 0xc9, 0xb2, 0xff, 0x1c, 0xbe, 0xa7, 0xa7, 0x81, 0x81, 0x27, 0x9a,
        0x85, 0x66, 0x79, 0x7a, 0x73, 0x6f, 0x12, 0xce, 0xa2, 0x5f, 0xa2, 0xb1, 0xc2,
        0x7a, 0x84, 0x43, 0x92, 0xfe, 0x9b, 0x37, 0x54, 0x7d, 0xc6, 0xfb, 0xd0, 0x0a,
        0x26, 0x76, 0xb8, 0x16, 0xe7, 0xd2, 0xd3, 0x56, 0x2b, 0xe2, 0xa0, 0xcb, 0xbd,
    },
    {
        0x04, 0xe9, 0xc8, 0xdc, 0x92, 0x97, 0x96, 0xaa, 0xc6, 0x5a, 0xf5, 0x08, 0x4e,
        0xb5, 0x4d, 0xc1, 0xee, 0x48, 0x2d, 0x5e, 0x0b, 0x5c, 0x58, 0xe2, 0xc9, 0x3f,
        0x24, 0x3c, 0x5b, 0x70, 0xb2, 0x15, 0x23, 0x32, 0x4b, 0xdb, 0x78, 0xd7, 0x39,
        0x53, 0x17, 0xda, 0x16, 0x5e, 0xf1, 0x13, 0x88, 0x26, 0xc3, 0xca, 0x3c, 0x91,
        0xca, 0x95, 0xe6, 0xf4, 0x90, 0xc3, 0x40, 0xcf, 0x55, 0x08, 0xa4, 0xa3, 0xec,
    },
    {
        0x04, 0xc2, 0xfb, 0x05, 0x88, 0x9b, 0x9d, 0xff, 0x5a, 0x9f, 0xb2, 0x2a, 0x59,
        0xee, 0x1d, 0x16, 0xbf, 0xc2, 0x86, 0x3f, 0x04, 0x00, 0xdd, 0xcb, 0x69, 0x56,
        0x6e, 0x2a, 0xbe, 0x8a, 0x15, 0xfa, 0x0b, 0xa1, 0x24, 0x02, 0x54, 0xca, 0x45,
        0xaa, 0x31, 0x0d, 0x17, 0x0e, 0x72, 0x4e, 0x13, 0x10, 0xce, 0x5f, 0x61, 0x1c,
        0xad, 0xa7, 0x6c, 0x12, 0xe3, 0xc2, 0x4a, 0x92, 0x6a, 0x39, 0x0c, 0xa4, 0xbe,
    },
    {
        0x04, 0xc4, 0xe8, 0x2d, 0x6d, 0x1b, 0x91, 0xe7, 0x85, 0x3e, 0xba, 0x96, 0xa8,
        0x71, 0xad, 0x31, 0xfc, 0x62, 0x62, 0x0b, 0x82, 0x6b, 0x0b, 0x8a, 0xcf, 0x81,
        0x5c, 0x03, 0xde, 0x31, 0xb7, 0x92, 0xa9, 0x8e, 0x05, 0xbb, 0x34, 0xd3, 0xb9,
        0xe0, 0xdf, 0x10, 0x40, 0xea, 0xc4, 0x85, 0xf0, 0x3f, 0xf8, 0xbb, 0xbf, 0x7a,
        0x85, 0x7e, 0xf1, 0xcf, 0x2a, 0x49, 0xa6, 0x0a, 0xc0, 0x84, 0xef, 0xb8, 0x8f,
    },
    {
        0x04, 0x05, 0x26, 0xf5, 0xb8, 0x34, 0x8a, 0x8d, 0x55, 0xe7, 0xb1, 0xca, 0xc0,
        0x43, 0xce, 0x98, 0xc5, 0x5b, 0xbd, 0xb3, 0x31, 0x1b, 0x4d, 0x1b, 0xb2, 0xd6,
        0x54, 0x28, 0x1e, 0xdf, 0x8a, 0xeb, 0x21, 0xf0, 0x18, 0xfb, 0x02, 0x7a, 0x6b,
        0x08, 0xe4, 0xdd, 0xc6, 0x2c, 0x91, 0x9e, 0x64, 0x86, 0x90, 0x72, 0x2d, 0x00,
        0xc6, 0xf5, 0x4c, 0x66, 0x8c, 0x9b, 0xd8, 0x22, 0x4a, 0x1d, 0x82, 0x42, 0x3a,
    },
};

uint32_t __stack_chk_guard = 0;

typedef enum {
    OP_BOOTLOADER_HASH = 'b',
    OP_GENKEY = 'g',
    OP_SET_CERTIFICATE = 'c',
    OP_SC_ROLLKEYS = 'k',
    OP_REBOOT = 'r',
} op_code_t;

typedef enum {
    ERR_OK,
    ERR_INVALID_INPUT,
    ERR_FAILED,
    ERR_UNKNOWN_COMMAND,
} error_code_t;

/**
 * Computes the hash which is signed by the root key.
 * @param[in] attestation_device_pubkey 64 bytes P-256 pubkey.
 * @param[out] sighash_out must be 32 bytes
 */
static void _attestation_sighash(const uint8_t* attestation_device_pubkey, uint8_t* sighash_out)
{
    uint8_t msg[32 + 64];
    memory_bootloader_hash(msg);
    memcpy(msg + 32, attestation_device_pubkey, 64);
    if (wally_sha256(msg, sizeof(msg), sighash_out, SHA256_LEN) != WALLY_OK) {
        Abort("wally_sha256 failed here");
    }
}

static void _api_msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    (void)max_out_len;
    const uint8_t* input = in_packet->data_addr;
    uint8_t* output = out_packet->data_addr;
    output[0] = input[0]; // OP_CODE
    error_code_t result = ERR_OK;
    size_t out_len = 2;
    size_t in_len = in_packet->len;
    switch (input[0]) {
    case OP_BOOTLOADER_HASH:
        memory_bootloader_hash(output + 2);
        out_len = 2 + 32;
        break;
    case OP_GENKEY: {
        screen_print_debug("generating pubkey...", 0);
        uint8_t pubkey[64];
        if (!securechip_gen_attestation_key(pubkey)) {
            screen_print_debug("generating pubkey\nfailed", 0);
            result = ERR_FAILED;
            break;
        }
        if (!memory_set_attestation_device_pubkey(pubkey)) {
            screen_print_debug("setting pubkey\nfailed", 0);
            result = ERR_FAILED;
            break;
        }
        memcpy(output + 2, pubkey, 64);
        out_len = 2 + 64;
        break;
    }
    case OP_SET_CERTIFICATE:
        if (in_len != 1 + 64 + 64 + 32) {
            result = ERR_INVALID_INPUT;
            break;
        }
        const uint8_t* attestation_device_pubkey = input + 1;
        const size_t pubkey_size = 64;
        const uint8_t* certificate = input + 1 + pubkey_size;
        const size_t certificate_size = 64;
        const uint8_t* root_pubkey_identifier = input + 1 + pubkey_size + certificate_size;

        // Verify sig
        secp256k1_context* ctx = wally_get_secp_context();
        secp256k1_ecdsa_signature sig = {0};
        if (!secp256k1_ecdsa_signature_parse_compact(ctx, &sig, certificate)) {
            result = ERR_INVALID_INPUT;
            break;
        }
        uint8_t msg32[SHA256_LEN] = {0};
        _attestation_sighash(attestation_device_pubkey, msg32);
        bool matches_a_root_pubkey = false;
        for (size_t pubkey_idx = 0; pubkey_idx < sizeof(_root_pubkey_bytes) / ROOT_PUBKEY_SIZE;
             pubkey_idx++) {
            secp256k1_pubkey pubkey;
            if (!secp256k1_ec_pubkey_parse(
                    wally_get_secp_context(),
                    &pubkey,
                    _root_pubkey_bytes[pubkey_idx],
                    ROOT_PUBKEY_SIZE)) {
                Abort("Invalid root pubkey");
            }

            if (secp256k1_ecdsa_verify(ctx, &sig, msg32, &pubkey)) {
                matches_a_root_pubkey = true;
                break;
            }
        }
        if (!matches_a_root_pubkey) {
            screen_print_debug("setting certificate\nfailed: sig", 0);
            result = ERR_INVALID_INPUT;
            break;
        }

        screen_print_debug("setting certificate...", 0);
        if (!memory_set_attestation_certificate(
                attestation_device_pubkey, certificate, root_pubkey_identifier)) {
            screen_print_debug("setting certificate\nfailed: write", 0);
            result = ERR_FAILED;
            break;
        }
        screen_print_debug("DONE", 0);
        break;
    case OP_SC_ROLLKEYS:
        if (!securechip_update_keys()) {
            screen_print_debug("rollkeys: failed", 0);
            result = ERR_FAILED;
            break;
        }
        screen_print_debug("rollkeys: success", 100);
        if (!securechip_u2f_counter_set(0)) {
            screen_print_debug("reset u2f counter", 0);
            result = ERR_FAILED;
            break;
        }
        screen_print_debug("reset u2f counter: success", 0);
        break;
    case OP_REBOOT:
        _reset_mcu();
        break;
    default:
        screen_sprintf_debug(1000, "unknown command: 0x%x", input[0]);
        result = ERR_UNKNOWN_COMMAND;
        break;
    }
    output[1] = result; // error code
    out_packet->len = out_len;
}

static void _api_setup(void)
{
    const CMD_Callback cmd_callbacks[] = {{FACTORYSETUP_CMD, _api_msg}};
    usb_processing_register_cmds(
        usb_processing_hww(), cmd_callbacks, sizeof(cmd_callbacks) / sizeof(CMD_Callback));
    screen_print_debug("READY", 0);
}

int main(void)
{
    init_mcu();
    system_init();
    platform_init();
    __stack_chk_guard = common_stack_chk_guard();
    screen_init();
    screen_splash();
    common_main();
    {
        // Set to re-enter bootloader again, otherwise we are stuck with this
        // firmware forever.
        auto_enter_t auto_enter = {
            .value = sectrue_u8,
        };
        upside_down_t upside_down = {
            .value = false,
        };
        if (!memory_bootloader_set_flags(auto_enter, upside_down)) {
            // Not much we can do here.
        }
    }
#if PLATFORM_BITBOX02 == 1
    usb_start(_api_setup);
#elif PLATFORM_BITBOXBASE == 1
    usart_start();
    _api_setup();
#endif

    while (1) {
        screen_process();
#if PLATFORM_BITBOXBASE == 1
        usart_receive();
#endif
        usb_processing_process(usb_processing_hww());
    }
}
