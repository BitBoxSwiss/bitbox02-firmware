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

#include "random.h"
#include "screen.h"
#include "util.h"
#include <drivers/driver_init.h>
#include <drivers/usb/class/hid/hww/hid_hww.h>
#include <drivers/usb/class/hid/u2f/hid_u2f.h>
#include <string.h>
#include <usb/usb.h>
#include <usb/usb_frame.h>
#include <usb/usb_packet.h>

// common test functions
#include "test_common.h"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

static void process_hww_cmd_cb(
    const Packet* in_packet,
    Packet* out_packet,
    const size_t max_out_len);
static void process_u2f_ping_cmd_cb(
    const Packet* in_packet,
    Packet* out_packet,
    const size_t max_out_len);

#define NUM_REGISTERED_U2F_COMMANDS 1
#define NUM_REGISTERED_HWW_COMMANDS 1
static const CMD_Callback u2f_cmd_callbacks[NUM_REGISTERED_U2F_COMMANDS] = {
    {0x80 + 0x01, process_u2f_ping_cmd_cb}};
static const CMD_Callback hww_cmd_callbacks[NUM_REGISTERED_HWW_COMMANDS] = {
    {0x80 + 0x40 + 0x01, process_hww_cmd_cb}};

static void usb_hww_endpoint_available(void)
{
    if (!hid_hww_is_enabled()) {
        screen_print_debug("HWW interface disabled", 1000);
        return;
    };
    usb_packet_register_cmds(hww_cmd_callbacks, NUM_REGISTERED_HWW_COMMANDS);

    hid_hww_setup();
}

/* ==== U2F ==== */

static void usb_u2f_endpoint_available(void)
{
    if (!hid_u2f_is_enabled()) {
        screen_print_debug("U2F interface disabled", 1000);
        return;
    };
    usb_packet_register_cmds(u2f_cmd_callbacks, NUM_REGISTERED_U2F_COMMANDS);

    hid_u2f_setup();
}

static void process_u2f_ping_cmd_cb(
    const Packet* in_packet,
    Packet* out_packet,
    const size_t max_out_len)
{
    (void)max_out_len;
    char msg[100];
    snprintf(msg, sizeof(msg), "U2F ping command: %.81s", data_to_string(in_packet));
    screen_print_debug(msg, 1000);

    out_packet->data_addr = in_packet->data_addr;
    out_packet->len = in_packet->len;
}

static void process_hww_cmd_cb(
    const Packet* in_packet,
    Packet* out_packet,
    const size_t max_out_len)
{
    (void)max_out_len;
    char msg[100];
    snprintf(msg, sizeof(msg), "hww command: %.86s", data_to_string(in_packet));
    screen_print_debug(msg, 1000);

    out_packet->data_addr = in_packet->data_addr;
    out_packet->len = in_packet->len;
}

static struct test_usb_metadata hww_metadata;
static struct test_usb_metadata u2f_metadata;

int main(void)
{
    system_init();
    uint8_t random[RANDOM_NUM_SIZE];
    random_32_bytes_mcu(random);
    __stack_chk_guard = ((uint32_t*)random)[0];

    screen_init();

    hww_metadata.endpoint_available_cb = usb_hww_endpoint_available;
    u2f_metadata.endpoint_available_cb = usb_u2f_endpoint_available;

    uint8_t err = test_usb_init(&hww_metadata, &u2f_metadata);
    switch (err) {
    case ERR_USBDC_FAILED:
        screen_print_debug("USB device initialization failed", 1000);
        break;
    case ERR_HID_FAILED:
        screen_print_debug("USB HID initialization failed", 1000);
        break;
    case ERR_NONE:
    default:
        break;
    }

    while (1) {
    }
}
