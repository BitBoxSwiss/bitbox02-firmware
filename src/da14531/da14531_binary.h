#ifndef DA14531_BINARY_H
#define DA14531_BINARY_H

#include <stddef.h>
#include <stdint.h>

extern const uint8_t _binary_bitbox_da14531_firmware_bin_start;
extern const uint8_t _binary_bitbox_da14531_firmware_bin_end;
extern const uint8_t _binary_bitbox_da14531_firmware_bin_size;

inline const uint8_t* da14531_firmware_start(void)
{
    return &_binary_bitbox_da14531_firmware_bin_start;
}

inline const uint8_t* da14531_firmware_end(void)
{
    return &_binary_bitbox_da14531_firmware_bin_end;
}

inline size_t da14531_firmware_size(void)
{
    return (size_t)&_binary_bitbox_da14531_firmware_bin_size;
}
#endif
