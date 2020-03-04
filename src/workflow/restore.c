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
#include <memory/memory.h>
#include <restore.h>
#include <securechip/securechip.h>
#include <workflow/confirm_time.h>
#include <workflow/password.h>
#include <workflow/status.h>
#include <workflow/unlock.h>
#include <workflow/workflow.h>

bool workflow_restore_backup(const RestoreBackupRequest* restore_request)
{
    Backup __attribute__((__cleanup__(backup_cleanup_backup))) backup;
    BackupData __attribute__((__cleanup__(backup_cleanup_backup_data))) backup_data;

    if (restore_from_directory(restore_request->id, &backup, &backup_data) != RESTORE_OK) {
        workflow_status_blocking("Could not\nrestore backup", false);
        return false;
    }

    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    if (!password_set(password)) {
        return false;
    }

    if (!restore_seed(&backup_data, password)) {
        workflow_status_blocking("Could not\nrestore backup", false);
        return false;
    }
#if APP_U2F == 1
    if (!workflow_confirm_time(
            restore_request->timestamp, restore_request->timezone_offset, false)) {
        return false;
    }
    if (!securechip_u2f_counter_set(restore_request->timestamp)) {
        // ignore error
    }
#endif
    if (!memory_set_initialized()) {
        return false;
    }
    uint8_t remaining_attempts;
    if (keystore_unlock(password, &remaining_attempts) != KEYSTORE_OK) {
        // This should/can never happen, but let's check anyway.
        Abort("workflow_restore_backup: unlock failed");
    }
    if (!memory_set_device_name(backup.backup_v1.content.metadata.name)) {
        /* Ignore errors for now */
    }
    return workflow_unlock_bip39();
}

bool workflow_list_backups(ListBackupsResponse* backups)
{
    restore_error_t res = restore_list_backups(backups);
    if (res != RESTORE_OK) {
        return false;
    }
    return true;
}
