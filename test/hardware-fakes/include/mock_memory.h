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

#ifndef _MOCK_MEMORY_H_
#define _MOCK_MEMORY_H_

#include <stdbool.h>
#include <stdint.h>

#include <flags.h>

void mock_memory_factoryreset(void);
bool memory_write_to_address_mock(uint32_t base, uint32_t addr, const uint8_t* chunk);
bool memory_write_chunk_mock(uint32_t chunk_num, const uint8_t* chunk);
void memory_read_chunk_mock(uint32_t chunk_num, uint8_t* chunk_out);
// Size: `FLASH_SHARED_DATA_LEN`.
void memory_read_shared_bootdata_mock(uint8_t* chunk_out);
void mock_memory_set_salt_root(const uint8_t* salt_root);
void memory_bootloader_hash_mock(uint8_t* hash_out);
void memory_set_bootloader_hash_mock(const uint8_t* mock_hash);
#endif
