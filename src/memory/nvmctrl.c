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

#include "nvmctrl.h"

#include <driver_init.h>

void nvmctrl_exec_cmd(uint16_t cmd)
{
    /* Wait until the NVM is ready to accept a new command. */
    while (NVMCTRL->STATUS.bit.READY == 0)
        ;
    NVMCTRL->ADDR.reg = (uint32_t)NVMCTRL_USER;
    NVMCTRL->CTRLB.reg = NVMCTRL_CTRLB_CMDEX_KEY | cmd;
    while (NVMCTRL->STATUS.bit.READY == 0)
        ;
}
