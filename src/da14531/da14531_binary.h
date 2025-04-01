#ifndef DA14531_BINARY_H
#define DA14531_BINARY_H

#include <stddef.h>
#include <stdint.h>

extern const uint8_t _binary_bitbox_da14531_firmware_bin_start;
extern const uint8_t _binary_bitbox_da14531_firmware_bin_end;
extern const uint8_t _binary_bitbox_da14531_firmware_bin_size;

const uint8_t* da14531_firmware_start = &_binary_bitbox_da14531_firmware_bin_start;
const uint8_t* da14531_firmware_end = &_binary_bitbox_da14531_firmware_bin_end;
const size_t da14531_firmware_size = (size_t)&_binary_bitbox_da14531_firmware_bin_size;
#endif
