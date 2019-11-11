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

#include "mpu.h"

#ifndef TESTING

/** Order of includes here is important. */
#include <flags.h>

#include <core_cm4.h>

#include <stdint.h>

uint32_t mpu_region_size(uint32_t size)
{
    uint32_t regionSize = 32;
    uint32_t ret = 4;

    while (ret < 31) {
        if (size <= regionSize) {
            break;
        }
        ret++;
        regionSize <<= 1;
    }
    return (ret << MPU_RASR_SIZE_Pos);
}

void mpu_set_region(uint32_t rbar, uint32_t rasr)
{
    MPU->RBAR = rbar;
    MPU->RASR = rasr;
}

void mpu_disable_region(uint32_t region_number)
{
    MPU->RNR = region_number;
    MPU->RASR &= 0xfffffffe;
}

#endif
