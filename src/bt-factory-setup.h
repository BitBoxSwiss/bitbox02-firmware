#ifndef BT_FACTORY_SETUP_H
#define BT_FACTORY_SETUP_H

#include <stddef.h>
#include <stdint.h>

extern const uint8_t _binary_bitbox_da14531_factory_setup_bin_start;
extern const uint8_t _binary_bitbox_da14531_factory_setup_bin_end;
extern const uint8_t _binary_bitbox_da14531_factory_setup_bin_size;

const uint8_t* ble_factory_setup_start = &_binary_bitbox_da14531_factory_setup_bin_start;
const uint8_t* ble_factory_setup_end = &_binary_bitbox_da14531_factory_setup_bin_end;
const size_t ble_factory_setup_size = (size_t)&_binary_bitbox_da14531_factory_setup_bin_size;
#endif
