// SPDX-License-Identifier: Apache-2.0

#include "firmware_installer_check.h"
#include "bootloader_upgrade.h"
#include <string.h>

_Static_assert(
    BB02_STAGE0_DESCRIPTOR_ADDR >= BB02_BOOTLOADER_UPGRADE_STAGE0_ADDR,
    "stage0 descriptor must be inside the bootloader image");

#define LEGACY_DEV_DEVICE_MARKER "DEV DEVICE"
#define LEGACY_DEV_DEVICE_MARKER_LEN (sizeof(LEGACY_DEV_DEVICE_MARKER) - 1)
#define LEGACY_NOT_FOR_VALUE_MARKER "NOT FOR VALUE"
#define LEGACY_NOT_FOR_VALUE_MARKER_LEN (sizeof(LEGACY_NOT_FOR_VALUE_MARKER) - 1)

static bool _matches_at(
    const uint8_t* haystack,
    size_t haystack_len,
    size_t pos,
    const char* needle,
    size_t needle_len)
{
    if (needle_len == 0 || pos > haystack_len || needle_len > haystack_len - pos) {
        return false;
    }
    return memcmp(&haystack[pos], needle, needle_len) == 0;
}

bool bootloader_upgrade_has_legacy_development_markers(const uint8_t* bootloader, size_t len)
{
    bool has_dev_device = false;
    bool has_not_for_value = false;
    if (bootloader == NULL) {
        return false;
    }
    for (size_t i = 0; i < len && (!has_dev_device || !has_not_for_value); i++) {
        if (!has_dev_device &&
            _matches_at(
                bootloader, len, i, LEGACY_DEV_DEVICE_MARKER, LEGACY_DEV_DEVICE_MARKER_LEN)) {
            has_dev_device = true;
        }
        if (!has_not_for_value &&
            _matches_at(
                bootloader, len, i, LEGACY_NOT_FOR_VALUE_MARKER, LEGACY_NOT_FOR_VALUE_MARKER_LEN)) {
            has_not_for_value = true;
        }
    }
    return has_dev_device && has_not_for_value;
}

static bool _read_stage0_descriptor(
    const bb02_stage0_descriptor_t* descriptor,
    bb02_stage0_descriptor_t* descriptor_out)
{
    if (descriptor == NULL || descriptor_out == NULL) {
        return false;
    }
    memcpy(descriptor_out, descriptor, sizeof(*descriptor_out));
    return descriptor_out->magic == BB02_STAGE0_DESCRIPTOR_MAGIC &&
           descriptor_out->stage0_version == BB02_STAGE0_IMAGE_VERSION &&
           descriptor_out->product_id == BB02_STAGE1_PRODUCT_ID;
}

static bool _stage1_header_len_ok(uint32_t header_len)
{
    return header_len >= BB02_STAGE1_HEADER_LEN && header_len <= BB02_STAGE1_MAX_LEN &&
           (header_len % BB02_STAGE1_HEADER_ALIGNMENT) == 0;
}

static bool _read_stage1_header(
    const bb02_stage1_header_t* header,
    bb02_stage1_header_t* header_out)
{
    if (header == NULL || header_out == NULL) {
        return false;
    }
    memcpy(header_out, header, sizeof(*header_out));
    const uint32_t header_len = header_out->header_len;
    return header_out->magic == BB02_STAGE1_HEADER_MAGIC &&
           header_out->product_id == BB02_STAGE1_PRODUCT_ID && _stage1_header_len_ok(header_len) &&
           header_out->image_len > header_len && header_out->image_len <= BB02_STAGE1_MAX_LEN;
}

bool bootloader_upgrade_is_development_bootloader(
    const bb02_stage0_descriptor_t* stage0_descriptor,
    const bb02_stage1_header_t* stage1_header,
    const uint8_t* legacy_bootloader,
    size_t legacy_bootloader_len)
{
    bb02_stage0_descriptor_t descriptor;
    if (_read_stage0_descriptor(stage0_descriptor, &descriptor)) {
        bb02_stage1_header_t header;
        return (descriptor.flags & BB02_STAGE0_FLAG_DEVELOPMENT) != 0 ||
               (_read_stage1_header(stage1_header, &header) &&
                (header.flags & BB02_STAGE1_FLAG_DEVELOPMENT) != 0);
    }
    return bootloader_upgrade_has_legacy_development_markers(
        legacy_bootloader, legacy_bootloader_len);
}
