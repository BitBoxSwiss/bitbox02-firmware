#include "bootloader_firmware_jump.h"

#include <driver_init.h>
#include <pukcc/curve_p256.h>
#include <screen.h>

#include "bootloader_graphics.h"
#include "bootloader_hash.h"
#include "mpu_regions.h"

const uint8_t _empty_sig[BOOT_SIG_LEN] = {0};

static void _maybe_show_hash(void)
{
    const boot_data_t* data = (const boot_data_t*)FLASH_BOOTDATA_START;
    if (!data->fields.show_firmware_hash) {
        return;
    }
    uint8_t hash[SHA256_DIGEST_LENGTH];
    bootloader_hash_firmware(data, hash);
    bootloader_graphics_render_hash("FIRMWARE", hash);
}

static void _render_bootloader_finished_marker(void)
{
    // We render the same as the firmware renders as a splash screen so it is not visually
    // distracting.
    screen_splash();
    // There will be a visible delay anyway before the firmware gets a chance to clear this, but
    // add a small delay anyway just in case.
    delay_ms(30);
}

__attribute__((noreturn)) static void _bin_exec(void* l_code_addr)
{
    __asm__(
        "mov   r1, r0        \n"
        "ldr   r0, [r1, #4]  \n"
        "ldr   sp, [r1]      \n"
        "blx   r0");
    (void)l_code_addr;
    __builtin_unreachable();
}

void bootloader_firmware_jump_exec(void)
{
    _render_bootloader_finished_marker();

    int i;

#if (FLASH_APP_START & 0x007F)
#error "app start address not aligned"
#else
    void* app_start_addr = (void*)FLASH_APP_START;
#endif

    rand_sync_disable(&RAND_0);

    // Update MPU settings for firmware mode
    mpu_regions_firmware_init();

    __disable_irq();
    for (i = 0; i < 8; i++) {
        NVIC->ICER[i] = 0xFFFFFFFF;
    }
    for (i = 0; i < 8; i++) {
        NVIC->ICPR[i] = 0xFFFFFFFF;
    }
    __DSB();
    __ISB();
    SCB->VTOR = ((uint32_t)app_start_addr & SCB_VTOR_TBLOFF_Msk);
    __DSB();
    __ISB();
    __enable_irq();
    _bin_exec(app_start_addr);
}

/*
 * Check the if the signatures of the firmware hash are valid.
 * If jump = true and M signatures are valid, jump to the firmware app.
 */
secbool_u32 bootloader_firmware_jump_verified(const boot_data_t* data, secbool_u32 jump)
{
    if (jump) {
        _maybe_show_hash();
    }

    for (uint32_t i = 0; i < (uint32_t)FLASH_APP_PAGE_NUM; i += FLASH_REGION_PAGE_NUM) {
        flash_lock(&FLASH_0, FLASH_APP_START + i * FLASH_PAGE_SIZE, FLASH_REGION_PAGE_NUM);
    }

    // Verify the firmware, signed by the signing keys
    if (bootloader_hash_pubkeys_verified(data) != sectrue_u32) {
        return secfalse_u32;
    }

    volatile uint8_t valid = 0;

    uint8_t hash[SHA256_DIGEST_LENGTH];
    bootloader_hash_firmware(data, hash);
    for (uint8_t i = 0; i < BOOT_NUM_FIRMWARE_SIGNING_KEYS; i++) {
        const uint8_t* pubkey = (const uint8_t*)&data->fields.signing_pubkeys + i * BOOT_PUBKEY_LEN;
        const uint8_t* sig = (const uint8_t*)&data->fields.firmware_signatures + i * BOOT_SIG_LEN;
        uint8_t random[SHA256_DIGEST_LENGTH];
        if (bootloader_hash_is_empty_sig(sig)) {
            continue;
        }
        rand_sync_read_buf8(&RAND_0, random, sizeof(random));
        // First is sanity check
        if (pukcc_ecdsa_verify(pubkey, sig, random, sizeof(random), curve_p256) != 0) {
            if (pukcc_ecdsa_verify(pubkey, sig, hash, sizeof(hash), curve_p256) == 0) {
                valid++;
                continue;
            }
        }
        return secfalse_u32;
    }

    if (valid >= BOOT_FIRMWARE_SIG_M) {
        if (jump == sectrue_u32) {
            bootloader_firmware_jump_exec(); /* no return */
        }
        return sectrue_u32;
    }
    return secfalse_u32;
}
