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
void fake_memory_set_salt_root(const uint8_t* salt_root);
const uint8_t* fake_memory_get_salt_root(void);
void memory_bootloader_hash_fake(uint8_t* hash_out);
void memory_set_bootloader_hash_fake(const uint8_t* fake_hash);
#endif
