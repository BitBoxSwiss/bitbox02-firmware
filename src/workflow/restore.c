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

#include <workflow/restore.h>

#include <backup.h>
#include <hardfault.h>
#include <keystore.h>
#include <memory.h>
#include <restore.h>
#include <screen.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <workflow/password.h>
#include <workflow/unlock.h>
#include <workflow/workflow.h>

#ifndef TESTING
#include <hal_delay.h>
#endif

static BackupData _backup_data;
static char _restored_name[MEMORY_DEVICE_NAME_MAX_LEN];

// true after the workflow has finished.
static bool _result = false;

static bool _restore(const char* password)
{
    bool res = restore_seed(&_backup_data, password);
    component_t* status;
    if (res) {
        res = memory_set_initialized();
    }
    if (res) {
        _result = true;
        status = status_create("Success", true, 50, NULL);
        uint8_t remaining_attempts;
        if (keystore_unlock(password, &remaining_attempts) != KEYSTORE_OK) {
            // This should/can never happen, but let's check anyway.
            Abort("Unexpected error during restore: unlock failed.");
        }
        if (!memory_set_device_name(_restored_name)) {
            /* Ignore errors for now */
        }
    } else {
        _result = false;
        status = status_create("Could not\nrestore backup", false, STATUS_DEFAULT_DELAY, NULL);
    }
    ui_screen_render_component(status);
    ui_util_component_cleanup(status);
#ifndef TESTING
    delay_ms(2000);
#endif
    if (res) {
        // Unlock after restore.
        workflow_unlock_enter_done(password);
    }
    return res;
}

bool workflow_restore_backup(const RestoreBackupRequest* restore_request)
{
    _result = false;
    Backup backup;
    restore_error_t res = restore_from_directory(restore_request->id, &backup, &_backup_data);
    memcpy(_restored_name, backup.backup_v1.content.metadata.name, sizeof(_restored_name));
    if (res != RESTORE_OK) {
        ui_screen_stack_switch(
            status_create("Could not\nrestore backup", false, STATUS_DEFAULT_DELAY, NULL));
        return false;
    }
    // blocking call until password is entered and backup is either restored or restoring failed.
    password_set(_restore);
    return _result;
}

bool workflow_list_backups(ListBackupsResponse* backups)
{
    restore_error_t res = restore_list_backups(backups);
    if (res != RESTORE_OK) {
        return false;
    }
    return true;
}
