// Copyright 2021 Shift Crypto AG
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

#include <stdint.h>
#include <string.h>

// Must be included before diskio.h, see http://elm-chan.org/fsw/ff/bd/?show=3626
#include <ff.h>

#include <diskio.h>

// This file is the disk middleware for use in tests, replacing sdmmc_diskio.c used in the firmware
// to write to microSD cards.
//
// It writes and reads from RAM instead of to a card or disk so that unit tests can exercise all
// sd-card related functionality.

// 100mb with 512 bytes per sector
#define SECTOR_SIZE 512
#define SECTOR_COUNT 204800
static uint8_t _data[SECTOR_SIZE * SECTOR_COUNT] = {0};

DSTATUS disk_initialize(uint8_t drv)
{
    (void)drv;
    return 0;
}

DSTATUS disk_status(uint8_t drv)
{
    (void)drv;
    return 0;
}

DRESULT disk_read(BYTE pdrv, BYTE* buff, LBA_t sector, UINT count)
{
    (void)pdrv;
    for (UINT i = 0; i < count; i++) {
        memcpy(&buff[SECTOR_SIZE * i], &_data[SECTOR_SIZE * sector + SECTOR_SIZE * i], SECTOR_SIZE);
    }
    return RES_OK;
}

DRESULT disk_write(BYTE pdrv, const BYTE* buff, LBA_t sector, UINT count)
{
    for (UINT i = 0; i < count; i++) {
        memcpy(&_data[SECTOR_SIZE * sector + SECTOR_SIZE * i], &buff[SECTOR_SIZE * i], SECTOR_SIZE);
    }
    return RES_OK;
}

DRESULT disk_ioctl(BYTE pdrv, BYTE cmd, void* buff)
{
    (void)pdrv;
    switch (cmd) {
    case CTRL_SYNC:
        // Already synced (RAM).
        return RES_OK;
    case GET_BLOCK_SIZE:
        *(DWORD*)buff = 1;
        return RES_OK;
    case GET_SECTOR_SIZE:
        *(LBA_t*)buff = SECTOR_SIZE;
        return RES_OK;
    case GET_SECTOR_COUNT:
        *(LBA_t*)buff = SECTOR_COUNT;
        return RES_OK;
    default:
        return RES_PARERR;
    }
}
