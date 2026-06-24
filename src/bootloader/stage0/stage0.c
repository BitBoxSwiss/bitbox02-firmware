// SPDX-License-Identifier: Apache-2.0

#include "bootloader/boot_args.h"
#include "bootloader_upgrade/bootloader_upgrade.h"
#ifndef BB02_STAGE0_DEVELOPMENT
    #include "bootloader_upgrade/stage1_pubkeys.h"
#endif
#include "driver_init.h"
#include "memory/memory_shared.h"
#include "pac_ext.h"
#include "stage0_flash.h"
#include "ui/oled/oled.h"
#include "util.h"
#ifndef BB02_STAGE0_DEVELOPMENT
    #include "pukcc/curve_p256.h"
    #include "pukcc/pukcc.h"
    #include "stage1_sigcheck.h"
#endif
#include <err_codes.h>
#include <hal_flash.h>
#include <hal_sha_sync.h>
#include <hpl_cmcc_config.h>
#include <hpl_dmac_config.h>
#include <hpl_port_config.h>
#include <hpl_reset.h>
#include <hri_nvmctrl_d51.h>
#include <sam.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#if CONF_DMAC_ENABLE
    #error "stage0 _init_chip() must not initialize DMAC"
#endif

#if CONF_CMCC_ENABLE
    #error "stage0 _init_chip() must not initialize CMCC"
#endif

#if CONF_PORT_EVCTRL_PORT_0 || CONF_PORT_EVCTRL_PORT_1 || CONF_PORT_EVCTRL_PORT_2 || \
    CONF_PORT_EVCTRL_PORT_3
    #error "stage0 _init_chip() must not initialize PORT events"
#endif

#define OLED_WIDTH (128)
#define OLED_HEIGHT (64)
#define NVMCTRL_STAGE0_ERROR_FLAGS \
    (NVMCTRL_INTFLAG_ADDRE | NVMCTRL_INTFLAG_PROGE | NVMCTRL_INTFLAG_LOCKE | NVMCTRL_INTFLAG_NVME)
static bool _oled_initialized = false;

__attribute__((aligned(128))) static struct sha_context _sha_context;

// GCC LTO needs externally_visible; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void __attribute__((noreturn, used, externally_visible)) __stack_chk_fail(void);

static void _oled_render_error(void);

static void __attribute__((noreturn)) _halt(void)
{
    _oled_render_error();
    while (1) {
        __WFI();
    }
}

static void __attribute__((noreturn)) _reset(void)
{
    _reset_mcu();
    _halt();
}

// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
uintptr_t __attribute__((used, externally_visible)) __stack_chk_guard = 0;

// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
void __attribute__((noreturn, used, externally_visible)) __stack_chk_fail(void)
{
    _halt();
}

static secbool_u32 _memeq(const uint8_t* a, const uint8_t* b, size_t len)
{
    uint8_t diff = 0;
    for (size_t i = 0; i < len; i++) {
        diff |= a[i] ^ b[i];
    }
    if (diff == 0) {
        return sectrue_u32;
    }
    return secfalse_u32;
}

static const chunk_shared_t* _shared_data(void)
{
    return (const chunk_shared_t*)FLASH_SHARED_DATA_START;
}

uint8_t memory_get_screen_type(void)
{
    const uint8_t screen_type = _shared_data()->fields.screen_type;
    if (screen_type == MEMORY_SCREEN_TYPE_SSD1312) {
        return MEMORY_SCREEN_TYPE_SSD1312;
    }
    return MEMORY_SCREEN_TYPE_SH1107;
}

static bool _oled_upside_down(void)
{
    const bool flash_upside_down = _shared_data()->fields.upside_down != 0;
    if (boot_args_is_valid()) {
        return boot_args_is_upside_down();
    }
    return flash_upside_down;
}

#ifdef BB02_STAGE0_DEVELOPMENT
static void _oled_development_cross_overlay(void)
{
    for (int16_t x = 0; x < OLED_WIDTH; x++) {
        const int16_t y = (int16_t)(((int32_t)x * (OLED_HEIGHT - 1)) / (OLED_WIDTH - 1));
        oled_set_pixel(x, y, true);
        oled_set_pixel(x, y + 1, true);
        oled_set_pixel(x, (OLED_HEIGHT - 1) - y, true);
        oled_set_pixel(x, (OLED_HEIGHT - 2) - y, true);
    }
}

static void _oled_development_cross(void)
{
    oled_clear_buffer();
    _oled_development_cross_overlay();
    oled_send_buffer();
}
#endif

static void _oled_progress(uint8_t done, uint8_t total)
{
    oled_clear_buffer();
    const int16_t width = 100;
    const int16_t height = 10;
    const int16_t border_width = 1;
    const int16_t x0 = (int16_t)((OLED_WIDTH - width) / 2);
    const int16_t y0 = (int16_t)((OLED_HEIGHT - height) / 2);
    const int16_t fill = (int16_t)(((uint32_t)(width - 2 * border_width) * done) / total);
    for (int16_t x = x0; x < x0 + width; x++) {
        oled_set_pixel(x, y0, true);
        oled_set_pixel(x, y0 + height - 1, true);
    }
    for (int16_t y = y0; y < y0 + height; y++) {
        oled_set_pixel(x0, y, true);
        oled_set_pixel(x0 + width - 1, y, true);
    }
    const int16_t y_fill_end = (int16_t)(y0 + height - border_width);
    const int16_t x_fill_end = (int16_t)(x0 + border_width + fill);
    for (int16_t y = (int16_t)(y0 + border_width); y < y_fill_end; y++) {
        for (int16_t x = (int16_t)(x0 + border_width); x < x_fill_end; x++) {
            oled_set_pixel(x, y, true);
        }
    }
#ifdef BB02_STAGE0_DEVELOPMENT
    _oled_development_cross_overlay();
#endif
    oled_send_buffer();
}

static void _oled_init(void)
{
    oled_init();
    oled_mirror(_oled_upside_down());
    _oled_initialized = true;
}

static void _oled_draw_error_pixel(int16_t x, int16_t y)
{
    const int16_t scale = 2;
    for (int16_t dy = 0; dy < scale; dy++) {
        for (int16_t dx = 0; dx < scale; dx++) {
            oled_set_pixel((int16_t)(x + dx), (int16_t)(y + dy), true);
        }
    }
}

// Render a centered fixed "Error" bitmap. The framebuffer is cleared first,
// then only the bitmap's set pixels are drawn, scaled up 2x.
static void _oled_render_error(void)
{
    static const uint32_t error_bitmap[7] = {
        0x1f000000U,
        0x10000000U,
        0x10596396U,
        0x1e659459U,
        0x10410450U,
        0x10410450U,
        0x1f410390U,
    };
    const int16_t bitmap_width = 29;
    const int16_t bitmap_height = 7;
    const int16_t scale = 2;
    const int16_t text_width = (int16_t)(bitmap_width * scale);
    const int16_t text_height = (int16_t)(bitmap_height * scale);
    const int16_t x0 = (int16_t)((OLED_WIDTH - text_width) / 2);
    const int16_t y0 = (int16_t)((OLED_HEIGHT - text_height) / 2);

    if (!_oled_initialized) {
        _oled_init();
    }
    oled_clear_buffer();
    for (int16_t row = 0; row < bitmap_height; row++) {
        const uint32_t bits = error_bitmap[row];
        for (int16_t col = 0; col < bitmap_width; col++) {
            if ((bits & (1UL << (bitmap_width - 1 - col))) == 0) {
                continue;
            }
            _oled_draw_error_pixel((int16_t)(x0 + col * scale), (int16_t)(y0 + row * scale));
        }
    }
    oled_send_buffer();
}

static void _nvm_wait(void)
{
    while (!hri_nvmctrl_get_STATUS_READY_bit(NVMCTRL)) {
    }
}

static void _nvm_clear_errors(void)
{
    hri_nvmctrl_clear_INTFLAG_reg(NVMCTRL, NVMCTRL_STAGE0_ERROR_FLAGS);
}

static void _nvm_check_errors(void)
{
    if ((hri_nvmctrl_read_INTFLAG_reg(NVMCTRL) & NVMCTRL_STAGE0_ERROR_FLAGS) != 0) {
        _halt();
    }
}

#ifndef BB02_STAGE0_DEVELOPMENT
static void _lock_debug_access(void)
{
    // Hard lock the Device Service Unit, i.e., set the PAC write-protection bit,
    // which can only be cleared by a hardware reset. Because the DSU is soft
    // locked by default on reset, an unlock is required before a hard lock.
    periph_unlock(DSU);
    periph_lock_hard(DSU);

    // Set the security bit to disable hardware debug access if not already set.
    // The security bit is persistent and erased on chip erase. A chip erase is
    // disabled by hard locking the DSU.
    if (!DSU->STATUSB.bit.PROT) {
        _nvm_wait();
        do {
            NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMD_SSB | NVMCTRL_CTRLB_CMDEX_KEY;
            while (NVMCTRL->INTFLAG.bit.DONE == 0 || NVMCTRL->STATUS.bit.READY == 0) {
            }
        } while (NVMCTRL->INTFLAG.bit.PROGE); // Program Error flag
        // Software reset the NVMCTRL peripheral to have correct NVM state output
        NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMD_SWRST | NVMCTRL_CTRLB_CMDEX_KEY;
        _nvm_wait();
    }
}
#endif

// GCC needs noclone; clang-tidy parses with Clang and does not support it.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
static void __attribute__((noinline, noclone)) _require_auth(const volatile secbool_u32* auth)
{
    if (*auth != sectrue_u32) {
        _halt();
    }
}

static void _flash_unlock_region(uint32_t addr, const volatile secbool_u32* install_auth)
{
    const uint32_t region_addr = addr & ~(STAGE0_FLASH_REGION_SIZE_BYTES - 1U);
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_clear_errors();
    if (flash_unlock(&FLASH_0, region_addr, FLASH_REGION_PAGE_NUM) != FLASH_REGION_PAGE_NUM) {
        _halt();
    }
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_check_errors();
}

static void _nvm_disable_bootprot(const volatile secbool_u32* install_auth)
{
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_clear_errors();
    hri_nvmctrl_write_CTRLB_reg(NVMCTRL, NVMCTRL_CTRLB_CMDEX_KEY | NVMCTRL_CTRLB_CMD_SBPDIS);
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_check_errors();
}

static void _flash_erase_block(uint32_t addr, const volatile secbool_u32* install_auth)
{
    if (stage0_flash_block_addr_ok(addr) != sectrue_u32) {
        _halt();
    }
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_clear_errors();
    if (flash_erase(&FLASH_0, addr, STAGE0_FLASH_BLOCK_SIZE_BYTES / STAGE0_FLASH_PAGE_SIZE_BYTES) !=
        ERR_NONE) {
        _halt();
    }
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_check_errors();
}

static void _flash_write_page(
    uint32_t addr,
    uint32_t* page_words,
    const volatile secbool_u32* install_auth)
{
    if (stage0_flash_page_addr_ok(addr) != sectrue_u32) {
        _halt();
    }
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_clear_errors();
    if (flash_append(&FLASH_0, addr, (uint8_t*)page_words, STAGE0_FLASH_PAGE_SIZE_BYTES) !=
        ERR_NONE) {
        _halt();
    }
    _require_auth(install_auth);
    _nvm_wait();
    _nvm_check_errors();
}

static secbool_u32 _flash_block_erased(uint32_t addr)
{
    const uint32_t* words = (const uint32_t*)addr;
    for (uint32_t i = 0; i < STAGE0_FLASH_BLOCK_SIZE_BYTES / sizeof(uint32_t); i++) {
        if (words[i] != UINT32_MAX) {
            return secfalse_u32;
        }
    }
    return sectrue_u32;
}

static secbool_u32 _factory_random_backup_valid(
    uint8_t factory_random_out[BB02_STAGE1_FACTORY_RANDOM_LEN])
{
    return stage0_factory_random_backup_valid(
        (const stage0_factory_random_backup_t*)BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR,
        factory_random_out);
}

static void _factory_random_backup_create(
    const uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN],
    const volatile secbool_u32* install_auth)
{
    if (_flash_block_erased(BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR) != sectrue_u32) {
        _flash_erase_block(BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR, install_auth);
    }

    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    stage0_factory_random_backup_make_data_page(page, factory_random);
    _flash_write_page(BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR, page, install_auth);

    stage0_factory_random_backup_make_commit_page(page);
    _flash_write_page(BB02_STAGE1_FACTORY_RANDOM_BACKUP_ADDR, page, install_auth);

    uint8_t verify[BB02_STAGE1_FACTORY_RANDOM_LEN];
    if (_factory_random_backup_valid(verify) != sectrue_u32 ||
        _memeq(verify, factory_random, sizeof(verify)) != sectrue_u32) {
        _halt();
    }
}

static void _factory_random_load_or_create_backup(
    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN],
    secbool_u32 installed_header_ok,
    const volatile secbool_u32* install_auth)
{
    uint8_t current[BB02_STAGE1_FACTORY_RANDOM_LEN];
    memcpy(current, (const void*)BB02_STAGE1_FACTORY_RANDOM_ADDR, BB02_STAGE1_FACTORY_RANDOM_LEN);

    uint8_t backup[BB02_STAGE1_FACTORY_RANDOM_LEN];
    const secbool_u32 backup_valid = _factory_random_backup_valid(backup);
    const secbool_u32 current_matches_backup =
        backup_valid == sectrue_u32 ? _memeq(current, backup, sizeof(current)) : secfalse_u32;

    if (stage0_factory_random_source(installed_header_ok, backup_valid, current_matches_backup) ==
        STAGE0_FACTORY_RANDOM_SOURCE_BACKUP) {
        memcpy(factory_random, backup, sizeof(backup));
        return;
    }

    memcpy(factory_random, current, sizeof(current));
    _factory_random_backup_create(factory_random, install_auth);
}

static secbool_u32 _stage1_marketing_version_len_ok(const bb02_stage1_header_t* header)
{
    if (header->stage1_marketing_version_len >
        BB02_STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN) {
        return secfalse_u32;
    }
    return sectrue_u32;
}

static secbool_u32 _stage1_header_len_ok(uint32_t header_len)
{
    if (header_len < BB02_STAGE1_HEADER_LEN || header_len > BB02_STAGE1_MAX_LEN) {
        return secfalse_u32;
    }
    if ((header_len % BB02_STAGE1_HEADER_ALIGNMENT) != 0) {
        return secfalse_u32;
    }
    return sectrue_u32;
}

static uint32_t _stage1_vector_addr(const bb02_stage1_header_t* header)
{
    return BB02_STAGE1_ADDR + (uint32_t)header->header_len;
}

static secbool_u32 _stage1_flags_ok(const bb02_stage1_header_t* header)
{
#ifndef BB02_STAGE0_DEVELOPMENT
    if ((header->flags & BB02_STAGE1_FLAG_DEVELOPMENT) != 0) {
        return secfalse_u32;
    }
#else
    if ((header->flags & BB02_STAGE1_FLAG_DEVELOPMENT) == 0) {
        return secfalse_u32;
    }
#endif
    return sectrue_u32;
}

static secbool_u32 _header_basic_ok(const bb02_stage1_header_t* header)
{
    const uint32_t header_len = (uint32_t)header->header_len;
    if (header->magic != BB02_STAGE1_HEADER_MAGIC ||
        _stage1_header_len_ok(header_len) != sectrue_u32 || header->image_len <= header_len ||
        header->image_len > BB02_STAGE1_MAX_LEN || header->product_id != BB02_STAGE1_PRODUCT_ID ||
        _stage1_flags_ok(header) != sectrue_u32 ||
        _stage1_marketing_version_len_ok(header) != sectrue_u32) {
        return secfalse_u32;
    }
    return sectrue_u32;
}

static void _stage1_hash(const bb02_stage1_header_t* header, uint8_t hash_out[32])
{
    const uint32_t image_len = (uint32_t)header->image_len;
    sha_sync_sha256_start(&HASH_ALGORITHM_0, &_sha_context, false);
    sha_sync_sha256_update(&HASH_ALGORITHM_0, (const uint8_t*)header, image_len);
    sha_sync_sha256_finish(&HASH_ALGORITHM_0, hash_out);
}

#ifndef BB02_STAGE0_DEVELOPMENT
static secbool_u32 _header_signatures_ok(const bb02_stage1_header_t* header)
{
    return stage1_sigcheck_image_ok(header, bb02_stage1_pubkeys);
}
#endif

static secbool_u32 _stage1_image_ok(const bb02_stage1_header_t* header, uint8_t hash_out[32])
{
    if (_header_basic_ok(header) != sectrue_u32) {
        return secfalse_u32;
    }

    _stage1_hash(header, hash_out);
#ifdef BB02_STAGE0_DEVELOPMENT
    return sectrue_u32;
#else
    return _header_signatures_ok(header);
#endif
}

static void _install_stage1(
    const bb02_stage1_header_t* update,
    secbool_u32 installed_header_ok,
    const volatile secbool_u32* install_auth)
{
    uint8_t factory_random[BB02_STAGE1_FACTORY_RANDOM_LEN];
    const uint32_t header_len = (uint32_t)update->header_len;
    const uint32_t image_len = (uint32_t)update->image_len;
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];

    _nvm_disable_bootprot(install_auth);
    _flash_unlock_region(BB02_STAGE1_FACTORY_RANDOM_BACKUP_BLOCK_ADDR, install_auth);
    _factory_random_load_or_create_backup(factory_random, installed_header_ok, install_auth);
    _flash_unlock_region(0x00000000U, install_auth);
    _flash_unlock_region(0x00008000U, install_auth);

    for (uint32_t block = BB02_STAGE1_ADDR; block < BB02_STAGE1_FACTORY_RANDOM_ADDR;
         block += STAGE0_FLASH_BLOCK_SIZE_BYTES) {
        _flash_erase_block(block, install_auth);
        if (block <= STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR &&
            block + STAGE0_FLASH_BLOCK_SIZE_BYTES > STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR) {
            stage0_flash_make_stage1_page(
                page,
                (const uint8_t*)update,
                image_len,
                STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR,
                factory_random);
            _flash_write_page(STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR, page, install_auth);
            // Check that the write succeeded.
            if (_memeq(
                    (const uint8_t*)BB02_STAGE1_FACTORY_RANDOM_ADDR,
                    factory_random,
                    BB02_STAGE1_FACTORY_RANDOM_LEN) != sectrue_u32) {
                _halt();
            }
        }
        for (uint32_t page_offset = 0; page_offset < STAGE0_FLASH_BLOCK_SIZE_BYTES;
             page_offset += STAGE0_FLASH_PAGE_SIZE_BYTES) {
            const uint32_t dst_addr = block + page_offset;
            if ((dst_addr >= BB02_STAGE1_HEADER_ADDR &&
                 dst_addr < BB02_STAGE1_HEADER_ADDR + header_len) ||
                dst_addr == STAGE0_STAGE1_FACTORY_RANDOM_PAGE_ADDR) {
                continue;
            }
            stage0_flash_make_stage1_page(
                page, (const uint8_t*)update, image_len, dst_addr, factory_random);
            _flash_write_page(dst_addr, page, install_auth);
        }
        _oled_progress(
            (uint8_t)((block - BB02_STAGE1_ADDR) / STAGE0_FLASH_BLOCK_SIZE_BYTES + 1), 6);
    }
    for (uint32_t header_offset = 0; header_offset < header_len;
         header_offset += STAGE0_FLASH_PAGE_SIZE_BYTES) {
        const uint32_t dst_addr = BB02_STAGE1_HEADER_ADDR + header_offset;
        stage0_flash_make_stage1_page(page, (const uint8_t*)update, image_len, dst_addr, NULL);
        _flash_write_page(dst_addr, page, install_auth);
    }
}

static void _invalidate_stage1_header(const volatile secbool_u32* install_auth)
{
    uint32_t page[STAGE0_FLASH_PAGE_WORDS];
    stage0_flash_make_invalid_header_page(page);
    _flash_write_page(BB02_STAGE1_HEADER_ADDR, page, install_auth);
}

static secbool_u32 _valid_stage1_vector_table(const bb02_stage1_header_t* stage1)
{
    const uint32_t header_len = (uint32_t)stage1->header_len;
    if (_stage1_header_len_ok(header_len) != sectrue_u32 || stage1->image_len <= header_len ||
        stage1->image_len > BB02_STAGE1_MAX_LEN) {
        return secfalse_u32;
    }
    const uint32_t vector_addr = _stage1_vector_addr(stage1);
    const uint32_t* vectors = (const uint32_t*)vector_addr;
    const uint32_t sp = vectors[0];
    const uint32_t pc = vectors[1];
    const uint32_t pc_addr = pc & ~1U;
    const uint32_t image_end = BB02_STAGE1_ADDR + (uint32_t)stage1->image_len;
    if (sp < BOOT_ARGS_ADDR + BOOT_ARGS_LEN || sp > 0x20040000U || (pc & 1U) == 0 ||
        pc_addr < vector_addr || pc_addr >= image_end) {
        return secfalse_u32;
    }
    return sectrue_u32;
}

static void _stage1_exec(const void* l_code_addr) __attribute__((noreturn));
static void _stage1_exec(const void* l_code_addr)
{
    __asm__ volatile(
        "ldr   r1, [%[vectors], #4]  \n"
        "ldr   sp, [%[vectors]]      \n"
        "blx   r1                    \n"
        :
        : [vectors] "r"(l_code_addr)
        : "r1", "memory");
    __builtin_unreachable();
}

static void _boot_stage1(const bb02_stage1_header_t* stage1, const volatile secbool_u32* boot_auth)
{
    _require_auth(boot_auth);
    if (_valid_stage1_vector_table(stage1) != sectrue_u32) {
        _halt();
    }
    const uint32_t vector_addr = _stage1_vector_addr(stage1);
    const void* vectors = (const void*)vector_addr;
    _require_auth(boot_auth);
    stage0_deinit();
    __disable_irq();
    for (uint32_t i = 0; i < 8; i++) {
        NVIC->ICER[i] = 0xFFFFFFFF;
    }
    for (uint32_t i = 0; i < 8; i++) {
        NVIC->ICPR[i] = 0xFFFFFFFF;
    }
    __DSB();
    __ISB();
    SCB->VTOR = vector_addr;
    __DSB();
    __ISB();
    _stage1_exec(vectors);
}

static void __attribute__((noreturn, noinline)) _stage0_main(void)
{
#ifdef BB02_STAGE0_DEVELOPMENT
    _oled_init();
    _oled_development_cross();
#else
    _lock_debug_access();
#endif

    uint8_t installed_hash[32];
    uint8_t update_hash[32];
    const bb02_stage1_header_t* installed = bb02_stage1_installed_header();
    const bb02_stage1_header_t* update = bb02_stage1_update_header();

    const secbool_u32 installed_header_ok = _header_basic_ok(installed);
    const secbool_u32 installed_ok = installed_header_ok == sectrue_u32
                                         ? _stage1_image_ok(installed, installed_hash)
                                         : secfalse_u32;
    const secbool_u32 update_ok = _stage1_image_ok(update, update_hash);
    volatile secbool_u32 install_auth = update_ok;
    volatile secbool_u32 boot_auth = installed_ok;

    if (update_ok == sectrue_u32) {
        if (installed_ok == sectrue_u32) {
            if (_memeq(installed_hash, update_hash, sizeof(installed_hash)) == sectrue_u32) {
                _boot_stage1(installed, &boot_auth);
            }
            if (update->monotonic_version < installed->monotonic_version) {
                _boot_stage1(installed, &boot_auth);
            }
        }
#ifndef BB02_STAGE0_DEVELOPMENT
        _oled_init();
#endif
        _oled_progress(0, 6);
        _install_stage1(update, installed_header_ok, &install_auth);
        installed = bb02_stage1_installed_header();
        if (_stage1_image_ok(installed, installed_hash) == sectrue_u32) {
            _oled_progress(6, 6);
            _reset();
        }
        _invalidate_stage1_header(&install_auth);
        _halt();
    }

    if (installed_ok == sectrue_u32) {
        _boot_stage1(installed, &boot_auth);
    }

    _halt();
}

int __attribute__((noreturn, noinline, no_stack_protector)) main(void)
{
    init_mcu();
    stage0_init();
    __stack_chk_guard = rand_sync_read32(&RAND_0);
    _stage0_main();
}
