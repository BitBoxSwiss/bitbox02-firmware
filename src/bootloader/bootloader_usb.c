#include "bootloader_usb.h"

#include <driver_init.h>
#include <flags.h>
#include <screen.h>
#include <ui/ugui/ugui.h>

#if PLATFORM_BITBOXBASE == 1
#include <usart/usart.h>
#include <usart/usart_frame.h>
#elif PLATFORM_BITBOX02 == 1
#include <usb/usb.h>
#include <usb/usb_frame.h>
#include <usb/usb_packet.h>
#endif

#include <usb/usb_processing.h>
#include <util.h>

#include "bootloader_chunks.h"
#include "bootloader_data.h"
#include "bootloader_firmware_jump.h"
#include "bootloader_graphics.h"
#include "bootloader_hash.h"

#define BOOT_OP_LEN 2u // 1 byte op code and 1 byte parameter
#define BOOTLOADER_CMD (HID_VENDOR_FIRST + 0x03) // Hardware wallet command

// Bootloader API command op codes
//
// OP_ERASE - Receives the total number of firmware chunks to write,
// then pads any left over FLASH that would be empty by 0xFFs;
// FLASH areas that will contain firmware are erased when writing the
// firmware chunk to FLASH.
#define OP_ERASE ((uint8_t)'e') /* 0x65 */
// OP_REBOOT - Reboot the MCU and clear the auto_enter flag.
#define OP_REBOOT ((uint8_t)'r') /* 0x72 */
// OP_WRITE - Write a firmware chunk; the binary must be streamed in chunks.
// The command must include the chunk number in order to be written to the
// correct FLASH location.
#define OP_WRITE_FIRMWARE_CHUNK ((uint8_t)'w') /* 0x77 */
// OP_WRITE_SIG_DATA - Write the firmware's signature data, which is used
// for firmware verification.
#define OP_WRITE_SIG_DATA ((uint8_t)'s') /* 0x73 */
// OP_VERSIONS - Return firmware and signature data version numbers to the
// computer. Note: The bootloader version is specified in the USB descriptor.
#define OP_VERSIONS ((uint8_t)'v') /* 0x76 */
// OP_HASHES - Display firmware and signature data hashes on the screen
// and return the values to the computer.
#define OP_HASHES ((uint8_t)'h') /* 0x68 */
// OP_SCREEN_ROTATE flips the screen orientation.
#define OP_SCREEN_ROTATE ((uint8_t)'f') /* 0x66 */
// OP_SET_SHOW_FIRMWARE_HASH - Enable or disable the flag to automatically show the firmware hash.
#define OP_SET_SHOW_FIRMWARE_HASH ((uint8_t)'H') /* 0x4A */

static uint8_t _loading_ready = 0;
static uint8_t _firmware_num_chunks = 0;

static void _load_arrow(int x, int y, int height)
{
    int width = height * 2 - 1;
    for (int h = 0; h < height; h++) {
        for (int w = (width + 1) / 2 - h; w < (width + 1) / 2 + h + 1; w++) {
            UG_DrawPixel(x + w, y + h, C_WHITE);
        }
    }
}

static void _load_progress_bar(float progress)
{
    const uint16_t bar_height = 5;
    UG_FillFrame(0, SCREEN_HEIGHT - bar_height, SCREEN_WIDTH * progress, SCREEN_HEIGHT, C_WHITE);
}

static size_t _report_status(uint8_t status, uint8_t* output)
{
    output[1] = status;
    return BOOT_OP_LEN;
}

static void _render_progress(float progress)
{
    UG_ClearBuffer();
    bootloader_graphics_load_logo();
    if (progress > 0) {
        char label[5] = {0};
        snprintf(label, sizeof(label), "%2d%%", (int)(100 * progress));
        UG_PutString(0, SCREEN_HEIGHT - 9 * 2, label, false);
        _load_progress_bar(progress);
    } else {
        _load_arrow(0, SCREEN_HEIGHT - 16, 10);
    }
    UG_PutString(SCREEN_WIDTH / 2 - 3, SCREEN_HEIGHT - 9 * 2, "UPGRADING", false);
    UG_SendBuffer();
}

static size_t _api_write_chunk(const uint8_t* buf, uint8_t chunknum, uint8_t* output)
{
    if (!_loading_ready) {
        return _report_status(OP_STATUS_ERR_LOAD_FLAG, output);
    }
    _loading_ready = 0;

    if (BOOT_OP_LEN + FIRMWARE_CHUNK_LEN > USB_DATA_MAX_LEN) {
        return _report_status(OP_STATUS_ERR_MACRO, output);
    }

    // The second is redundant, as _firmware_num_chunks <=
    // FIRMWARE_MAX_NUM_CHUNKS.
    if (chunknum > _firmware_num_chunks - 1 || chunknum > FIRMWARE_MAX_NUM_CHUNKS - 1) {
        return _report_status(OP_STATUS_ERR_LEN, output);
    }

    if (MEMEQ(
            (const void*)(FLASH_APP_START + (chunknum * FIRMWARE_CHUNK_LEN)),
            buf,
            FIRMWARE_CHUNK_LEN)) {
        _loading_ready = 1;
        return _report_status(OP_STATUS_OK, output);
    }

    // Erase is handled inside of flash_write
    if (flash_write(
            &FLASH_0,
            FLASH_APP_START + (chunknum * FIRMWARE_CHUNK_LEN),
            (const uint8_t*)buf,
            FIRMWARE_CHUNK_LEN) != ERR_NONE) {
        return _report_status(OP_STATUS_ERR_WRITE, output);
    }

    if (!MEMEQ(
            (const void*)(FLASH_APP_START + (chunknum * FIRMWARE_CHUNK_LEN)),
            buf,
            FIRMWARE_CHUNK_LEN)) {
        return _report_status(OP_STATUS_ERR_CHECK, output);
    }

    size_t len = _report_status(OP_STATUS_OK, output);
    _loading_ready = 1;
    return len;
}

/**
 * This function erases only the padding bytes, if not already erased. Other
 * bytes get erased and written when writing the firmware chunks.
 *
 * The number of chunks is put into RAM in order to show the correct
 * progress in the next step flashing the firmware.
 */
static size_t _api_firmware_erase(uint8_t firmware_num_chunks, uint8_t* output)
{
    if (firmware_num_chunks > FIRMWARE_MAX_NUM_CHUNKS - 1) {
        return _report_status(OP_STATUS_ERR_LEN, output);
    }
    if (firmware_num_chunks > 0) {
        _render_progress(0);
    }
    _loading_ready = 0;
    for (uint32_t i = 0; i < (uint32_t)FLASH_APP_PAGE_NUM; i += FLASH_REGION_PAGE_NUM) {
        if (flash_unlock(&FLASH_0, FLASH_APP_START + i * FLASH_PAGE_SIZE, FLASH_REGION_PAGE_NUM) !=
            FLASH_REGION_PAGE_NUM) {
            return _report_status(OP_STATUS_ERR_UNLOCK, output);
        }
    }
    uint8_t empty_page[FLASH_PAGE_SIZE];
    memset(empty_page, 0xff, sizeof(empty_page));
    uint16_t firmware_num_pages = firmware_num_chunks * FIRMWARE_CHUNK_LEN / FLASH_PAGE_SIZE;
    for (uint32_t i = firmware_num_pages; i < (uint32_t)FLASH_APP_PAGE_NUM;
         i += FLASH_ERASE_PAGE_NUM) {
        const uint32_t addr = FLASH_APP_START + i * FLASH_PAGE_SIZE;
        if (MEMEQ((const void*)addr, empty_page, sizeof(empty_page))) {
            continue;
        }
        if (flash_erase(&FLASH_0, addr, FLASH_ERASE_PAGE_NUM) != ERR_NONE) {
            return _report_status(OP_STATUS_ERR_ERASE, output);
        }
        if (!MEMEQ((const void*)addr, empty_page, sizeof(empty_page))) {
            return _report_status(OP_STATUS_ERR_CHECK, output);
        }
    }
    if (firmware_num_chunks > 0) {
        _firmware_num_chunks = firmware_num_chunks;
        _loading_ready = 1;
    }
    return _report_status(OP_STATUS_OK, output);
}

static inline version_t _parse_version(const uint8_t* start)
{
    // 4 byte little endian
    return *(const version_t*)start;
}

/*
 * input: firmware version | i signatures of the double hash of the [version | firmware app]
 */
static uint8_t _set_firmware_data(boot_data_t* data, const uint8_t* input)
{
    version_t version_new = _parse_version(input);
    // Downgrade prevention
    if (version_new < data->fields.firmware_version) {
        return OP_STATUS_ERR_VERSION;
    }

    data->fields.firmware_version = version_new;

    memcpy(
        &data->fields.firmware_signatures,
        input + sizeof(version_t),
        BOOT_SIG_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS);

    if (bootloader_firmware_jump_verified(data, secfalse_u32) != sectrue_u32) {
        return OP_STATUS_ERR;
    }
    return OP_STATUS_OK;
}

/*
 * input:  signing pubkeys version | i signing pubkeys | j signatures of the double hash of [version
 * | signing pubkeys]
 */
static uint8_t _set_signing_pubkey_data(boot_data_t* data, const uint8_t* input)
{
    version_t version_new = _parse_version(input);
    // Downgrade prevention
    if (version_new < data->fields.signing_pubkeys_version) {
        return OP_STATUS_ERR_VERSION;
    }

    data->fields.signing_pubkeys_version = version_new;

    const uint8_t* signing_pubkeys_addr = input + sizeof(version_t);
    memcpy(
        (uint8_t*)&data->fields.signing_pubkeys,
        signing_pubkeys_addr,
        sizeof(data->fields.signing_pubkeys));

    memcpy(
        &data->fields.root_signatures_of_signing_pubkeys,
        signing_pubkeys_addr + BOOT_NUM_FIRMWARE_SIGNING_KEYS * BOOT_PUBKEY_LEN,
        sizeof(data->fields.root_signatures_of_signing_pubkeys));

    if (bootloader_hash_pubkeys_verified(data) != sectrue_u32) {
        return OP_STATUS_ERR;
    }
    return OP_STATUS_OK;
}

static size_t _api_write_sig_data(const uint8_t* input, uint8_t* output)
{
    boot_data_t data;
    memcpy(data.bytes, (uint8_t*)(FLASH_BOOTDATA_START), FLASH_BOOTDATA_LEN);
    uint8_t status = _set_signing_pubkey_data(&data, input);
    if (status != OP_STATUS_OK) {
        return _report_status(status, output);
    }
    status = _set_firmware_data(
        &data,
        input + sizeof(version_t) + BOOT_PUBKEY_LEN * BOOT_NUM_FIRMWARE_SIGNING_KEYS +
            BOOT_SIG_LEN * BOOT_NUM_ROOT_SIGNING_KEYS);
    if (status != OP_STATUS_OK) {
        return _report_status(status, output);
    }
    return _report_status(bootloader_chunks_write_chunk(FLASH_BOOTDATA_START, data.bytes), output);
}

/*
 * output filled with double hash of firmware app | double hash of signing key data
 */
static size_t _api_get_hashes(const uint8_t* input, uint8_t* output)
{
    const boot_data_t* data = (const boot_data_t*)FLASH_BOOTDATA_START;
    uint8_t hash[SHA256_DIGEST_LENGTH];
    bootloader_hash_firmware(data, hash);
    memcpy(output + BOOT_OP_LEN, hash, SHA256_DIGEST_LENGTH);

    if (input[0]) {
        bootloader_graphics_render_hash("FIRMWARE", hash);
    }

    bootloader_hash_signing_keys(data, hash);
    memcpy(output + BOOT_OP_LEN + SHA256_DIGEST_LENGTH, hash, SHA256_DIGEST_LENGTH);

    if (input[1]) {
        bootloader_graphics_render_hash("SIGKEYS", hash);
    }

    size_t len = _report_status(OP_STATUS_OK, output);
    return len + SHA256_DIGEST_LENGTH * 2;
}

/*
 * input: one byte: 0 - disable, 1 - enable, >1: no effect.
 * output: one byte: 0 - disabled, 1: enabled.
 */
static size_t _api_set_show_firmware_hash(const uint8_t* input, uint8_t* output)
{
    boot_data_t data;
    memcpy(data.bytes, (uint8_t*)(FLASH_BOOTDATA_START), FLASH_BOOTDATA_LEN);
    uint8_t result = OP_STATUS_OK;
    uint8_t value = input[0];
    if (value == 0 || value == 1) {
        if (data.fields.show_firmware_hash != value) {
            data.fields.show_firmware_hash = value;
            result = bootloader_chunks_write_chunk(FLASH_BOOTDATA_START, data.bytes);
        }
    }
    output[BOOT_OP_LEN] = data.fields.show_firmware_hash;
    return _report_status(result, output) + 1;
}

/*
 * output filled with bootloader version | firmware version | signing pubkeys version
 */
static size_t _api_versions(uint8_t* output)
{
    const boot_data_t* data = (const boot_data_t*)FLASH_BOOTDATA_START;
    memcpy(output + BOOT_OP_LEN, (const uint8_t*)&data->fields.firmware_version, sizeof(version_t));
    memcpy(
        output + BOOT_OP_LEN + sizeof(version_t),
        (const uint8_t*)&data->fields.signing_pubkeys_version,
        sizeof(version_t));
    _report_status(OP_STATUS_OK, output);
    return BOOT_OP_LEN + sizeof(version_t) * 3;
}

static void _api_reboot(void)
{
    shared_data_t shared_data;
    memcpy(shared_data.bytes, (uint8_t*)(FLASH_SHARED_DATA_START), FLASH_SHARED_DATA_LEN);
    if (shared_data.fields.auto_enter == sectrue_u8) {
        shared_data.fields.auto_enter = secfalse_u8;
        bootloader_chunks_write_chunk(FLASH_SHARED_DATA_START, shared_data.bytes);
    }
    _reset_mcu();
}

static size_t _api_screen_rotate(uint8_t* output)
{
#if PLATFORM_BITBOX02 == 1
    if (_loading_ready) {
        return _report_status(OP_STATUS_ERR_LOAD_FLAG, output);
    }
    screen_rotate();
    bootloader_graphics_render_default_screen();
    return _report_status(OP_STATUS_OK, output);
#elif PLATFORM_BITBOXBASE == 1
    return _report_status(OP_STATUS_ERR_INVALID_CMD, output);
#endif
}

static size_t _api_command(const uint8_t* input, uint8_t* output, const size_t max_out_len)
{
    memset(output, 0, max_out_len);
    output[0] = input[0]; // OP_CODE
    size_t len = 1;

    switch (output[0]) {
    case OP_VERSIONS:
        len = _api_versions(output);
        break;

    case OP_HASHES:
        len = _api_get_hashes(input + 1, output);
        break;

    case OP_SET_SHOW_FIRMWARE_HASH:
        len = _api_set_show_firmware_hash(input + 1, output);
        break;

    case OP_ERASE:
        len = _api_firmware_erase(input[1], output);
        break;

    case OP_REBOOT:
        _api_reboot();
        break;

    case OP_WRITE_FIRMWARE_CHUNK: {
        uint8_t chunk_num = input[1];
        len = _api_write_chunk(input + 2, chunk_num, output);
        if (output[1] != OP_STATUS_OK) {
            bootloader_graphics_render_default_screen();
        } else {
            _render_progress((float)chunk_num / (float)(_firmware_num_chunks - 1));
        }
        break;
    }
    case OP_WRITE_SIG_DATA:
        len = _api_write_sig_data(input + 1, output);
        break;
    case OP_SCREEN_ROTATE:
        len = _api_screen_rotate(output);
        break;
    default:
        len = _report_status(OP_STATUS_ERR_INVALID_CMD, output);
        _loading_ready = 0;
        char msg[100];
        snprintf(msg, 100, "Command: %u unknown", input[0]);
        bootloader_graphics_render_message(msg, 1000);
        break;
    }

    return len;
}

static void _api_msg(const Packet* in_packet, Packet* out_packet, const size_t max_out_len)
{
    size_t len = _api_command(in_packet->data_addr, out_packet->data_addr, max_out_len);
    out_packet->len = len;
}

static void _api_setup(void)
{
    const CMD_Callback cmd_callbacks[] = {{BOOTLOADER_CMD, _api_msg}};

    usb_processing_register_cmds(usb_processing_hww(), cmd_callbacks, 1);
}

bool bootloader_usb_start(void)
{
#if PLATFORM_BITBOX02 == 1
    return usb_start(_api_setup) == ERR_NONE;
#elif PLATFORM_BITBOXBASE == 1
    usart_start();
    _api_setup();
    return true;
#endif
}
