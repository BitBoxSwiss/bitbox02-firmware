// SPDX-License-Identifier: Apache-2.0

#include "firmware_installer.h"
#include "bootloader_upgrade.h"
#include "driver_init.h"
#include "firmware_installer_check.h"
#include "flags.h"
#include "hardfault.h"
#include "memory/memory_shared.h"
#include "memory/mpu.h"
#include "screen.h"
#ifndef BOOTLOADER_UPGRADE_DEVELOPMENT
    #include "bootloader/stage0/stage1_sigcheck.h"
    #include "stage1_pubkeys.h"
#endif
#include "system.h"
#include <hal_flash.h>
#include <sam.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

extern const uint8_t _binary_bootloader_upgrade_stage0_image_bin_start;
extern const uint8_t _binary_bootloader_upgrade_stage0_image_bin_size;

#define LEGACY_BOOTLOADER_SCAN_ADDR (FLASH_BOOT_START + 1U)
#define LEGACY_BOOTLOADER_SCAN_LEN (FLASH_BOOT_LEN - 32U - 1U)

_Static_assert(FLASH_BOOT_LEN > 33U, "legacy bootloader scan length underflows");

#ifdef BOOTLOADER_UPGRADE_DEVELOPMENT
    #define BB02_BOOTLOADER_UPGRADE_EXPECTED_STAGE1_FLAGS BB02_STAGE1_FLAG_DEVELOPMENT
#else
    #define BB02_BOOTLOADER_UPGRADE_EXPECTED_STAGE1_FLAGS 0U
#endif

static const uint8_t* _stage0_image(void)
{
    return &_binary_bootloader_upgrade_stage0_image_bin_start;
}

static size_t _stage0_image_len(void)
{
    return (size_t)&_binary_bootloader_upgrade_stage0_image_bin_size;
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wcast-qual"
static uint8_t* _stage0_image_for_flash_write(void)
{
    return (uint8_t*)_stage0_image();
}
#pragma GCC diagnostic pop

static bool _bytes_equal(const uint8_t* a, const uint8_t* b, size_t len)
{
    // TODO need to worry about NULL deref?
    for (size_t i = 0; i < len; i++) {
        // Stage0 is intentionally memory-mapped at address 0 on the target.
        if (a[i] != b[i]) { // NOLINT(clang-analyzer-core.NullDereference)
            return false;
        }
    }
    return true;
}

static bool _bytes_zero(const uint8_t* data, size_t len)
{
    for (size_t i = 0; i < len; i++) {
        if (data[i] != 0) {
            return false;
        }
    }
    return true;
}

static void _apply_saved_orientation(void)
{
    chunk_shared_t shared_data = {0};
    memory_read_shared_bootdata(&shared_data);
    if (shared_data.fields.upside_down) {
        screen_rotate();
    }
}

static bool _stage1_marketing_version_ok(const bb02_stage1_header_t* header)
{
    if (header->stage1_marketing_version_len == 0 ||
        header->stage1_marketing_version_len >
            BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN) {
        return false;
    }
    for (uint8_t i = 0; i < header->stage1_marketing_version_len; i++) {
        const uint8_t ch = header->stage1_marketing_version[i];
        if (ch < 0x21 || ch > 0x7e) {
            return false;
        }
    }
    return _bytes_zero(
        &header->stage1_marketing_version[header->stage1_marketing_version_len],
        BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN - header->stage1_marketing_version_len);
}

static bool _stage1_header_len_ok(uint32_t header_len)
{
    return header_len >= BB02_STAGE1_HEADER_LEN && header_len <= BB02_STAGE1_MAX_LEN &&
           (header_len % BB02_STAGE1_HEADER_ALIGNMENT) == 0;
}

static bool _stage1_flags_ok(const bb02_stage1_header_t* header)
{
#ifdef BOOTLOADER_UPGRADE_DEVELOPMENT
    return (header->flags & BB02_STAGE1_FLAG_DEVELOPMENT) != 0;
#else
    return (header->flags & BB02_STAGE1_FLAG_DEVELOPMENT) == 0;
#endif
}

static void _disable_mpu(void)
{
    __disable_irq();
    __DSB();
    __ISB();
    MPU->CTRL = MPU_DISABLE;
    __DSB();
    __ISB();
    __enable_irq();
}

static void _disable_bootprot(void)
{
    while (NVMCTRL->STATUS.bit.READY == 0) {
    }
    NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMD_SBPDIS | NVMCTRL_CTRLB_CMDEX_KEY;
    while (NVMCTRL->STATUS.bit.READY == 0) {
    }
}

static bool _stage0_is_installed(void)
{
    if (_stage0_image_len() != BB02_BOOTLOADER_UPGRADE_STAGE0_LEN) {
        AbortAutoenter("stage0 len");
    }
    return _bytes_equal(
        (const uint8_t*)BB02_BOOTLOADER_UPGRADE_STAGE0_ADDR, _stage0_image(), _stage0_image_len());
}

static bool _update_header_basic_ok(const bb02_stage1_header_t* header)
{
    const uint32_t header_len = (uint32_t)header->header_len;
    return header->magic == BB02_STAGE1_HEADER_MAGIC && _stage1_header_len_ok(header_len) &&
           header->image_len > header_len && header->product_id == BB02_STAGE1_PRODUCT_ID &&
           _stage1_flags_ok(header) && header->image_len <= BB02_STAGE1_MAX_LEN &&
           _stage1_marketing_version_ok(header);
}

static bool _stage1_update_ok(const bb02_stage1_header_t* update)
{
    if (!_update_header_basic_ok(update)) {
        return false;
    }
#ifdef BOOTLOADER_UPGRADE_DEVELOPMENT
    return true;
#else
    return stage1_sigcheck_image_ok(update, bb02_stage1_pubkeys) == sectrue_u32;
#endif
}

static void _flash_stage0(void)
{
    _disable_mpu();
    _disable_bootprot();
    if (flash_unlock(&FLASH_0, BB02_BOOTLOADER_UPGRADE_STAGE0_ADDR, FLASH_REGION_PAGE_NUM) !=
        FLASH_REGION_PAGE_NUM) {
        AbortAutoenter("unlock stage0");
    }
    if (flash_write(
            &FLASH_0,
            BB02_BOOTLOADER_UPGRADE_STAGE0_ADDR,
            _stage0_image_for_flash_write(),
            _stage0_image_len()) != ERR_NONE) {
        AbortAutoenter("write stage0");
    }
    if (!_stage0_is_installed()) {
        AbortAutoenter("verify stage0");
    }
}

void bootloader_upgrade_install_or_reboot(void)
{
    _apply_saved_orientation();

#ifndef BOOTLOADER_UPGRADE_DEVELOPMENT
    if (bootloader_upgrade_is_development_bootloader(
            (const bb02_stage0_descriptor_t*)BB02_STAGE0_DESCRIPTOR_ADDR,
            bb02_stage1_installed_header(),
            (const uint8_t*)LEGACY_BOOTLOADER_SCAN_ADDR,
            LEGACY_BOOTLOADER_SCAN_LEN)) {
        AbortAutoenter("Development bootloader");
    }
#endif

    const bb02_stage1_header_t* update = bb02_stage1_update_header();
    if (!_stage1_update_ok(update)) {
        AbortAutoenter("stage1 update");
    }
    if (!_stage0_is_installed()) {
        _flash_stage0();
        if (!_stage0_is_installed()) {
            AbortAutoenter("verify stage0");
        }
    }
    boot_bootloader_wait(screen_is_upside_down());
    AbortAutoenter("reboot");
}
