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

#include "common_main.h"
#include "screen.h"
#include <driver_init.h>
#include <usb/class/hid/hww/hid_hww.h>
#include <usb/class/hid/u2f/hid_u2f.h>
#include <usb/usb_processing.h>

// common test functions
#include "test_common.h"

// uint32_t __stack_chk_guard = 0;

static void process_hww_cmd_cb(const Packet* in_packet, Packet* out_packet, size_t max_out_len);
static void process_u2f_ping_cmd_cb(
    const Packet* in_packet,
    Packet* out_packet,
    size_t max_out_len);

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
    usb_processing_register_cmds(
        usb_processing_hww(), hww_cmd_callbacks, NUM_REGISTERED_HWW_COMMANDS);

    hid_hww_setup();
}

/* ==== U2F ==== */

static void usb_u2f_endpoint_available(void)
{
    if (!hid_u2f_is_enabled()) {
        screen_print_debug("U2F interface disabled", 1000);
        return;
    };
    usb_processing_register_cmds(
        usb_processing_u2f(), u2f_cmd_callbacks, NUM_REGISTERED_U2F_COMMANDS);

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

    memcpy(out_packet->data_addr, in_packet->data_addr, 64);
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

    memcpy(out_packet->data_addr, in_packet->data_addr, 64);
    out_packet->len = in_packet->len;
}

static struct test_usb_metadata hww_metadata;
static struct test_usb_metadata u2f_metadata;

int main(void)
{
    system_init();
    //__stack_chk_guard = common_stack_chk_guard();

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
