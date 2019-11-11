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

#include "securechip/securechip.h"
#include <driver_init.h>
#include <hardfault.h>
#include <qtouch.h>
#include <random.h>
#include <screen.h>
#include <string.h>
#include <util.h>
#include <wally_core.h>

#include <backup.h>
#include <keystore.h>
#include <memory/memory.h>
#include <restore.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

static securechip_interface_functions_t _securechip_interface_functions = {
    .get_auth_key = memory_get_authorization_key,
    .get_io_protection_key = memory_get_io_protection_key,
    .random_32_bytes = random_32_bytes,
};

static memory_interface_functions_t _memory_interface_functions = {
    // Use random_32_bytes_mcu over random_32_bytes as the latter mixes in
    // randomness from the securechip, which is initialized only later.
    .random_32_bytes = random_32_bytes_mcu,
};

static void _wally_patched_bzero(void* ptr, size_t len)
{
    util_zero(ptr, len);
}

static bool _setup_wally(void)
{
    static struct wally_operations _ops = {0};
    if (wally_get_operations(&_ops) != WALLY_OK) {
        return false;
    }
    _ops.bzero_fn = _wally_patched_bzero;
    return wally_set_operations(&_ops) == WALLY_OK;
}

static void _test_backup(time_t timestamp, time_t birthdate)
{
    backup_error_t res = backup_create(timestamp, birthdate);
    switch (res) {
    case BACKUP_OK:
        screen_print_debug("backup OK", 1000);
        break;
    case BACKUP_SEED_INACCESSIBLE:
        screen_print_debug("backup failed: seed inaccessible", 1000);
        break;
    case BACKUP_ERR_ENCODE:
        screen_print_debug("backup failed: encoding failed", 1000);
        break;
    case BACKUP_ERR_SD_LIST:
        screen_print_debug("backup failed: sd list failed", 1000);
        break;
    case BACKUP_ERR_SD_WRITE:
        screen_print_debug("backup failed: sd write failed", 1000);
        break;
    case BACKUP_ERR_CHECK:
        screen_print_debug("backup check failed", 1000);
        break;
    default:
        screen_sprintf_debug(1000, "Unexpected result: %d", res);
        break;
    }
}

static time_t _timestamp = 1552553498;

static void _test_list_backups(char id[256])
{
    ListBackupsResponse backups;
    restore_error_t res = restore_list_backups(&backups);
    if (res != RESTORE_OK) {
        screen_print_debug("list backups failed", 1000);
    } else {
        for (int i = 0; i < backups.info_count; i++) {
            screen_sprintf_debug(
                1000, "%d: %s %d", (i + 1), backups.info[i].id, backups.info[i].timestamp);
            if (backups.info[i].timestamp == _timestamp) {
                memcpy(id, backups.info[i].id, 256);
            }
        }
    }
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();
    if (!memory_setup(&_memory_interface_functions)) {
        Abort("memory_setup failed");
    }
    if (!_setup_wally()) {
        Abort("_setup_wally failed");
    }
    // securechip_setup must come after memory_setup, so the io/auth keys to be
    // used are already initialized.
    if (!securechip_setup(&_securechip_interface_functions)) {
        Abort("securechip_setup failed");
    }

    screen_print_debug("Creating initial backup...", 1000);

    if (!keystore_create_and_store_seed("device-test", "host-entropy")) {
        Abort("Failed to create keystore");
    }
    uint8_t remaining_attempts;
    if (keystore_unlock("device-test", &remaining_attempts) != KEYSTORE_OK) {
        Abort("Failed to unlock keystore");
    }
    _test_backup(_timestamp, _timestamp);

    screen_print_debug("Creating another backup...", 1000);

    _timestamp = _timestamp + 24 * 3600;
    _test_backup(_timestamp, _timestamp);
    // after the test, the SD card should only contain 3 files under the sub-directory for the seed
    char id[256];
    _test_list_backups(id);

    Backup backup;
    BackupData backup_data;
    if (restore_from_directory(id, &backup, &backup_data) != RESTORE_OK) {
        Abort("Failed to restore backup");
    }
    if (!restore_seed(&backup_data, "device-test")) {
        Abort("Failed to restore seed");
    }
    screen_print_debug("Restore OK", 1000);
}

#pragma GCC diagnostic pop
