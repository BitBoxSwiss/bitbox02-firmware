// SPDX-License-Identifier: Apache-2.0

#ifndef _BOOTLOADER_H_
#define _BOOTLOADER_H_

#include <platform_config.h>
#include <stdbool.h>
#include <stdint.h>

void bootloader_jump(void);

/**
 * Renders the default bootloader screen
 */
void bootloader_render_default_screen(void);

#if PLATFORM_BITBOX02PLUS
extern bool bootloader_pairing_request;
extern uint8_t bootloader_pairing_code_bytes[4];

/**
 * Renders a BLE pairing confirmations screen. Use the `confirmed` argument to display the
 * "user has confirmed on bitbox" version.
 *
 * confirmed - If false renders Yes/No icons at top, otherwise renders "See app".
 */
void bootloader_render_ble_confirm_screen(bool confirmed);
#endif

#endif
