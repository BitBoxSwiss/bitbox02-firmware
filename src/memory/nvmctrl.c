// SPDX-License-Identifier: Apache-2.0

#include "nvmctrl.h"

#include <driver_init.h>

void nvmctrl_exec_cmd(uint16_t cmd)
{
    /* Wait until the NVM is ready to accept a new command. */
    while (NVMCTRL->STATUS.bit.READY == 0);
    NVMCTRL->ADDR.reg = (uint32_t)NVMCTRL_USER;
    NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMDEX_KEY | cmd;
    while (NVMCTRL->STATUS.bit.READY == 0);
}
