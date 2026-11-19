// SPDX-License-Identifier: Apache-2.0

#ifndef TEST_NVMCTRL_DRIVER_INIT_H
#define TEST_NVMCTRL_DRIVER_INIT_H

#include <stdint.h>

#include <err_codes.h>

struct flash_descriptor {
    uint8_t unused;
};
extern struct flash_descriptor FLASH_0;

int32_t flash_unlock(struct flash_descriptor* flash, uint32_t address, uint32_t pages);
int32_t flash_lock(struct flash_descriptor* flash, uint32_t address, uint32_t pages);
int32_t flash_erase(struct flash_descriptor* flash, uint32_t address, uint32_t pages);
int32_t flash_append(struct flash_descriptor* flash, uint32_t address, uint8_t* data, uint32_t len);

typedef union {
    struct {
        uint16_t READY : 1;
        uint16_t : 15;
    } bit;
    uint16_t reg;
} fake_nvmctrl_status_t;

typedef struct {
    fake_nvmctrl_status_t STATUS;
    struct {
        uint32_t reg;
    } ADDR;
    struct {
        uint32_t reg;
    } CTRLB;
    struct {
        uint16_t reg;
    } INTFLAG;
    struct {
        uint32_t reg;
    } ECCERR;
    uint32_t eccerr_reads;
    uint16_t flags_on_ready;
    uint32_t eccerr_on_ready;
    void (*on_ready)(void);
} fake_nvmctrl_t;

extern fake_nvmctrl_t fake_nvmctrl;

#define NVMCTRL (&fake_nvmctrl)
#define NVMCTRL_USER 0x00804000U
#define NVMCTRL_CTRLB_CMDEX_KEY 0xA500U

#endif // TEST_NVMCTRL_DRIVER_INIT_H
