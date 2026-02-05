// SPDX-License-Identifier: Apache-2.0

#include "common_main.h"
#include "da14531/da14531_protocol.h"
#include "driver_init.h"
#include "hardfault.h"
#include "memory/bitbox02_smarteeprom.h"
#include "memory/memory_shared.h"
#include "platform/platform_config.h"
#include "platform_init.h"
#include "qtouch.h"
#include "screen.h"
#include "ui/screen_stack.h"
#include "usb/usb_processing.h"
#include <hww.h>
#include <memory/memory_spi.h>
#include <rust/rust.h>
#include <ui/oled/oled.h>

#if APP_U2F == 1
    #include <u2f.h>
#endif

uint32_t __stack_chk_guard = 0;

int main(void)
{
    init_mcu();
    system_init();
    platform_init();
    __stack_chk_guard = common_stack_chk_guard();
    screen_init(oled_set_pixel, oled_mirror, oled_clear_buffer);
    screen_splash();
    qtouch_init();
    common_main();
    bitbox02_smarteeprom_init();
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        da14531_protocol_init();
    }
    usb_processing_init();
    // Setup usb_processing handlers
    hww_setup();
#if APP_U2F == 1
    u2f_device_setup();
#endif
    rust_main_loop();
    return 0;
}
