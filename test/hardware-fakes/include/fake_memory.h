// SPDX-License-Identifier: Apache-2.0

#ifndef _FAKE_MEMORY_H_
#define _FAKE_MEMORY_H_

#include <stdbool.h>
#include <stdint.h>

#include <flags.h>

void fake_memory_factoryreset(void);
bool fake_memory_nova(void);
bool memory_write_to_address_fake(uint32_t base, uint32_t addr, const uint8_t* chunk);
bool memory_write_chunk_fake(uint32_t chunk_num, const uint8_t* chunk);
void memory_read_chunk_fake(uint32_t chunk_num, uint8_t* chunk_out);
// Size: `FLASH_SHARED_DATA_LEN`.
void memory_read_shared_bootdata_fake(uint8_t* chunk_out);
const uint8_t* fake_memory_get_salt_root(void);
void memory_bootloader_hash_fake(uint8_t* hash_out);
void memory_set_bootloader_hash_fake(const uint8_t* fake_hash);
#endif
