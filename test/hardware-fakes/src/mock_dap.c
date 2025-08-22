// Copyright 2025 Shift Crypto AG
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

#ifndef MOCK_DAP_H
#define MOCK_DAP_H
#include <stdbool.h>
#include <stdint.h>

void dap_init(void);
void dap_connect(void);
void dap_disconnect(void);
uint32_t dap_read_word(uint32_t addr);
uint16_t dap_read_hword(uint32_t addr);
void dap_write_word(uint32_t addr, uint32_t data);
void dap_write_hword(uint32_t addr, uint16_t data);
void dap_reset_link(void);
uint32_t dap_read_idcode(void);
bool dap_target_prepare(int32_t timeout);

// Target operations
void dap_target_select(void);
void dap_target_deselect(void);
void dap_target_erase(void);
void dap_target_lock(void);
void dap_target_erase_row(uint32_t addr);
#endif
