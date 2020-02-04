#include "bootloader_chunks.h"

#include <driver_init.h>
#include <flags.h>

#include "bootloader_usb.h"

uint8_t bootloader_chunks_write_chunk(uint32_t address, const uint8_t* data)
{
    const uint32_t lock_size = FLASH_ERASE_PAGE_NUM * FLASH_PAGE_SIZE;
    if (flash_unlock(&FLASH_0, address & ~(lock_size - 1), FLASH_REGION_PAGE_NUM) !=
        FLASH_REGION_PAGE_NUM) {
        return OP_STATUS_ERR_UNLOCK;
    }
    // Erase is handled inside of flash_write
    if (flash_write(&FLASH_0, address, data, FLASH_BOOTDATA_LEN) != ERR_NONE) {
        return OP_STATUS_ERR_WRITE;
    }
    if (flash_lock(&FLASH_0, address & ~(lock_size - 1), FLASH_REGION_PAGE_NUM) !=
        FLASH_REGION_PAGE_NUM) {
        return OP_STATUS_ERR_LOCK;
    }
    return OP_STATUS_OK;
}
