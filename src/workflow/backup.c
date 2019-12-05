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

#include "workflow.h"
#include <backup.h>
#include <hww.pb.h>
#include <memory/memory.h>
#include <sd.h>
#include <stdio.h>
#include <time.h>
#include <ui/components/confirm.h>
#include <workflow/backup.h>
#include <workflow/confirm.h>
#include <workflow/confirm_time.h>
#include <workflow/password.h>
#include <workflow/sdcard.h>
#include <workflow/status.h>

#define MAX_EAST_UTC_OFFSET (50400) // 14 hours in seconds
#define MAX_WEST_UTC_OFFSET (-43200) // 12 hours in seconds

/**
 * Creates a backup and does error handling.
 */
static bool _backup(uint32_t backup_create_timestamp, uint32_t seed_birthdate_timestamp)
{
    backup_error_t res = backup_create(backup_create_timestamp, seed_birthdate_timestamp);
    switch (res) {
    case BACKUP_OK:
        if (!memory_set_initialized()) {
            // The backup was created, so reporting an error here could have bad consequences like
            // replacing the sd card, not safely disposing of the old one.
            // The issue fixes itself after replugging and going through the backup process again.
        }
        workflow_status_create("Backup created", true);
        return true;
    case BACKUP_ERR_SD_WRITE:
    case BACKUP_ERR_SD_LIST:
        if (!sd_card_inserted()) {
            workflow_status_create("Backup not created\nIs the SD card\ninserted?", false);
        } else {
            char msg[100];
            snprintf(
                msg,
                sizeof(msg),
                "Backup not created\nPlease contact\nsupport (%s)",
                backup_error_str(res));
            workflow_status_create(msg, false);
        }
        return false;
    default: {
        char msg[100];
        snprintf(
            msg,
            sizeof(msg),
            "Backup not created\nPlease contact\nsupport (%s)",
            backup_error_str(res));
        workflow_status_create(msg, false);
        return false;
    }
    }
}

bool workflow_backup_create(const CreateBackupRequest* request)
{
    if (request->timezone_offset < MAX_WEST_UTC_OFFSET ||
        request->timezone_offset > MAX_EAST_UTC_OFFSET) {
        return false;
    }

    // Wait for sd card.
    const InsertRemoveSDCardRequest sd = {
        .action = InsertRemoveSDCardRequest_SDCardAction_INSERT_CARD,
    };
    sdcard_handle(&sd);

    if (memory_is_initialized()) {
        if (!password_check()) {
            return false;
        }
    }

    uint32_t seed_birthdate = 0;
    if (!memory_is_initialized()) {
        if (!workflow_confirm_time(request->timestamp, request->timezone_offset, true)) {
            return false;
        }

        seed_birthdate = request->timestamp;
        if (!memory_set_seed_birthdate(seed_birthdate)) {
            return false;
        }
    } else {
        // If adding new backup after initialized, we do not know the seed bithdate.
        // If re-creating it, we use the already existing one.

        uint32_t existing_seed_birthdate;
        char backup_id[256];
        backup_error_t res = backup_check(backup_id, NULL, &existing_seed_birthdate);
        if (res == BACKUP_OK) {
            seed_birthdate = existing_seed_birthdate;
        }
    }
    return _backup(request->timestamp, seed_birthdate);
}

bool workflow_backup_check(char* id_out, bool silent)
{
    char backup_name[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    backup_error_t res = backup_check(id_out, backup_name, NULL);
    switch (res) {
    case BACKUP_ERR_SD_LIST:
    case BACKUP_ERR_SD_READ:
    case BACKUP_ERR_SD_WRITE:
        workflow_status_create("Could not read\nor write to the\nmicro SD card", false);
        return false;
    case BACKUP_ERR_CHECK:
        if (!silent) {
            workflow_status_create("Backup missing\nor invalid", false);
        }
        return false;
    case BACKUP_OK:
        if (!silent) {
            const confirm_params_t params_name = {
                .title = "Name?",
                .body = backup_name,
                .scrollable = true,
            };
            if (!workflow_confirm_blocking(&params_name)) {
                // TODO Change to error "User Abort"
                return false;
            }
            const confirm_params_t params_id = {
                .title = "ID?",
                .body = id_out,
                .scrollable = true,
            };
            if (!workflow_confirm_blocking(&params_id)) {
                // TODO Change to error "User Abort"
                return false;
            }
            workflow_status_create("Backup valid", true);
        }
        return true;
    default: {
        char err_msg[100];
        snprintf(err_msg, 100, "Could not check\nbackup (%s)", backup_error_str(res));
        workflow_status_create(err_msg, false);
        return false;
    }
    }
}
