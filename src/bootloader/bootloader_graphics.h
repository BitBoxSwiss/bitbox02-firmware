#ifndef __BOOTLOADER_GRAPHICS_H
#define __BOOTLOADER_GRAPHICS_H

#include <stdint.h>

void bootloader_graphics_render_hash(const char* title, const uint8_t* hash);

void bootloader_graphics_load_logo(void);

void bootloader_graphics_render_default_screen(void);

void bootloader_graphics_render_message(const char* message, int duration);

#endif // __BOOTLOADER_GRAPHICS_H
