// SPDX-License-Identifier: Apache-2.0

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
