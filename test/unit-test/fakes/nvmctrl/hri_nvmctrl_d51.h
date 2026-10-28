// SPDX-License-Identifier: Apache-2.0

#ifndef TEST_HRI_NVMCTRL_D51_H
#define TEST_HRI_NVMCTRL_D51_H

#include <stdbool.h>
#include <stdint.h>

#include <driver_init.h>

#define NVMCTRL_INTFLAG_DONE (1U << 0)
#define NVMCTRL_INTFLAG_ADDRE (1U << 1)
#define NVMCTRL_INTFLAG_PROGE (1U << 2)
#define NVMCTRL_INTFLAG_LOCKE (1U << 3)
#define NVMCTRL_INTFLAG_ECCSE (1U << 4)
#define NVMCTRL_INTFLAG_ECCDE (1U << 5)
#define NVMCTRL_INTFLAG_NVME (1U << 6)

#define NVMCTRL_ECCERR_TYPEL_Msk (3UL << 28)
#define NVMCTRL_ECCERR_TYPEH_Msk (3UL << 30)
#define NVMCTRL_ECCERR_TYPEL_NONE (0UL << 28)
#define NVMCTRL_ECCERR_TYPEL_SINGLE (1UL << 28)
#define NVMCTRL_ECCERR_TYPEL_DUAL (2UL << 28)
#define NVMCTRL_ECCERR_TYPEH_NONE (0UL << 30)
#define NVMCTRL_ECCERR_TYPEH_SINGLE (1UL << 30)
#define NVMCTRL_ECCERR_TYPEH_DUAL (2UL << 30)

static inline bool hri_nvmctrl_get_STATUS_READY_bit(void* const hw)
{
    fake_nvmctrl_t* nvmctrl = hw;
    nvmctrl->INTFLAG.reg |= nvmctrl->flags_on_ready;
    nvmctrl->ECCERR.reg |= nvmctrl->eccerr_on_ready;
    if (nvmctrl->on_ready != NULL) {
        nvmctrl->on_ready();
    }
    return nvmctrl->STATUS.bit.READY != 0;
}

static inline uint16_t hri_nvmctrl_read_INTFLAG_reg(const void* const hw)
{
    return ((const fake_nvmctrl_t*)hw)->INTFLAG.reg;
}

static inline void hri_nvmctrl_clear_INTFLAG_reg(void* const hw, uint16_t mask)
{
    fake_nvmctrl_t* nvmctrl = hw;
    nvmctrl->INTFLAG.reg &= (uint16_t)~mask;
}

static inline uint32_t hri_nvmctrl_read_ECCERR_reg(void* const hw)
{
    fake_nvmctrl_t* nvmctrl = hw;
    const uint32_t result = nvmctrl->ECCERR.reg;
    nvmctrl->ECCERR.reg = 0;
    nvmctrl->INTFLAG.reg &= (uint16_t)~(NVMCTRL_INTFLAG_ECCSE | NVMCTRL_INTFLAG_ECCDE);
    nvmctrl->eccerr_reads++;
    return result;
}

#endif // TEST_HRI_NVMCTRL_D51_H
