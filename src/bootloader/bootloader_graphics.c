#include "bootloader_graphics.h"

#include <driver_init.h>
#include <screen.h>
#include <ui/components/ui_images.h>
#include <ui/ugui/ugui.h>
#include <util.h>

void bootloader_graphics_render_hash(const char* title, const uint8_t* hash)
{
    uint8_t seconds = 10;
    char message[16];
    char hash_hex[2 * SHA256_DIGEST_LENGTH + 1];
    util_uint8_to_hex(hash, SHA256_DIGEST_LENGTH, hash_hex);
    char scratch = 0;
    for (uint8_t i = 1; i <= seconds; i++) {
        snprintf(message, sizeof(message), "HASH  (%2ds)", seconds - i);
        UG_ClearBuffer();
        UG_PutString(0, SCREEN_HEIGHT - 9, message, false);
        UG_PutString(0, SCREEN_HEIGHT - 9 * 2, title, false);

        scratch = hash_hex[16];
        hash_hex[16] = 0;
        UG_PutString(0, 0, hash_hex, false);
        hash_hex[16] = scratch;

        scratch = hash_hex[32];
        hash_hex[32] = 0;
        UG_PutString(0, 9, &hash_hex[16], false);
        hash_hex[32] = scratch;

        scratch = hash_hex[48];
        hash_hex[48] = 0;
        UG_PutString(0, 18, &hash_hex[32], false);
        hash_hex[48] = scratch;

        UG_PutString(0, 27, &hash_hex[48], false);

        UG_SendBuffer();
        delay_ms(1000);
    }
    bootloader_graphics_render_default_screen();
}

void bootloader_graphics_load_logo(void)
{
    uint16_t x = 0;
    uint16_t y = 0;
    for (size_t i = 0; i < sizeof(IMAGE_BB2_LOGO); i++) {
        uint8_t b = IMAGE_BB2_LOGO[i];
        for (uint8_t j = 0; j < 8; j++) {
            if (b & 0x80) {
                UG_DrawPixel(x, y, C_WHITE);
            }
            b <<= 1;
            x++;
            if ((x % IMAGE_BB2_LOGO_W) == 0) {
                x = 0;
                y++;
            }
        }
    }
}

void bootloader_graphics_render_default_screen(void)
{
    UG_ClearBuffer();
    bootloader_graphics_load_logo();
    UG_PutString(1, SCREEN_HEIGHT - 9, "BOOTLOADER", false);
    UG_SendBuffer();
}

void bootloader_graphics_render_message(const char* message, int duration)
{
    char print[100];
    snprintf(print, sizeof(print), "%s", message);
    UG_ClearBuffer();
    UG_PutString(0, 0, print, false);
    UG_SendBuffer();
    delay_ms(duration);
}
