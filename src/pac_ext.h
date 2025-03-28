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

#ifndef PAC_EXT_H
#define PAC_EXT_H

#include <compiler.h>
#include <hpl_pac.h>
#include <utils_assert.h>

static inline int32_t periph_lock_hard(void* const module)
{
    ASSERT((((uint32_t)module) > (uint32_t)HPB0_ADDR));

    uint32_t peripheral;
    int32_t timeout = 1000;
    bool stat;

    peripheral = _pac_get_peripheral_id(module);

    hri_pac_write_WRCTRL_reg(PAC, PAC_WRCTRL_PERID(peripheral) | PAC_WRCTRL_KEY_SETLCK);

    do {
        _periph_get_lock_state(module, &stat);
    } while (!stat && timeout--);

    if (timeout < 0) {
        return ERR_TIMEOUT;
    }

    return ERR_NONE;
}

#endif
