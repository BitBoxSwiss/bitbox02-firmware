// SPDX-License-Identifier: Apache-2.0

#include <delay.h>
#include <da14531/da14531.h>
#include <memory/bitbox02_smarteeprom.h>
#include <memory/memory.h>
#include <memory/memory_shared.h>
#include <memory/memory_spi.h>
#include <memory/smarteeprom.h>
#include <memory/spi_mem.h>
#include <random.h>
#include <reset.h>
#include <screen.h>
#include <sd.h>
#include <securechip/securechip.h>
#include <system.h>
#include <time.h>
#include <ui/components/confirm.h>
#include <ui/components/confirm_transaction.h>
#include <ui/components/empty.h>
#include <ui/components/label.h>
#include <ui/components/menu.h>
#include <ui/components/orientation_arrows.h>
#include <ui/components/progress.h>
#include <ui/components/sdcard.h>
#include <ui/components/status.h>
#include <ui/components/trinary_choice.h>
#include <ui/components/trinary_input_string.h>
#include <ui/components/unlock_animation.h>
#include <ui/fonts/font_a_11X10.h>
#include <ui/fonts/font_a_9X9.h>
#include <ui/fonts/monogram_5X9.h>
#include <ui/fonts/password_11X12.h>
#include <ui/oled/oled.h>
#include <ui/screen_process.h>
#include <ui/screen_saver.h>
#include <ui/screen_stack.h>
#include <ui/ugui/ugui.h>
#include <usb/usb.h>
#include <usb/usb_processing.h>
#include <util.h>
#include <utils_ringbuffer.h>

#if defined(TESTING)
    #include <fake_memory.h>
    #include <hww.h>
    #include <touch/gestures.h>
    #include <ui/event.h>
    #include <ui/event_handler.h>
    #include <usb/usb_packet.h>
    #include <usb/usb_processing.h>
#endif

#if !defined(TESTING)
    #include <hal_delay.h>
#else
void delay_us(const uint16_t us);
void delay_ms(const uint16_t ms);
#endif
