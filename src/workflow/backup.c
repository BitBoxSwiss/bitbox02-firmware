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
#include <generated/hww.pb.h>
#include <memory.h>
#include <sd.h>
#include <stdio.h>
#include <time.h>
#include <ui/components/confirm.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <workflow/backup.h>

#define MAX_EAST_UTC_OFFSET (50400) // 14 hours in seconds
#define MAX_WEST_UTC_OFFSET (-43200) // 12 hours in seconds

static uint32_t _unix_timestamp = 0;
static bool _done = false;
static bool _result = false;
static bool _is_done(void)
{
    return _done;
}

/**
 * Returns to waiting screen.
 */
static void _dismiss(void)
{
    ui_screen_stack_pop();
}

/**
 * Waits for the end of the confirm backup status screen.
 */
static void _confirm_backup(void)
{
    _done = true;
}

/**
 * Waits for user to confirm backup on screen.
 */
static void _confirm_backup_2(component_t* component)
{
    (void)component;
    _done = true;
}

/**
 * Creates a backup and does error handling.
 */
static void _backup(void)
{
    backup_error_t res = backup_create(_unix_timestamp);
    component_t* status_info = NULL;
    char msg[100];
    switch (res) {
    case BACKUP_OK:
        status_info = status_create("Backup created", true, STATUS_DEFAULT_DELAY, _dismiss);
        if (!memory_set_initialized()) {
            // The backup was created, so reporting an error here could have bad consequences like
            // replacing the sd card, not safely disposing of the old one.
            // The issue fixes itself after replugging and going through the backup process again.
        }
        _result = true;
        break;
    case BACKUP_ERR_SD_WRITE:
    case BACKUP_ERR_SD_LIST:
        if (!workflow_get_interface_functions()->sd_card_inserted()) {
            status_info = status_create(
                "Backup not created\nIs the SD card\ninserted?",
                false,
                STATUS_DEFAULT_DELAY,
                _dismiss);
        } else {
            snprintf(msg, sizeof(msg), "Backup not created\nPlease contact\nsupport (%d)", res);
            status_info = status_create(msg, false, STATUS_DEFAULT_DELAY, _dismiss);
        }
        break;
    default:
        snprintf(msg, sizeof(msg), "Backup not created\nPlease contact\nsupport (%d)", res);
        status_info = status_create(msg, false, STATUS_DEFAULT_DELAY, _dismiss);
        break;
    }
    if (status_info != NULL) {
        ui_screen_stack_switch(status_info);
    }
    _done = true;
}

static void _check_sd_card_inserted_and_do_backup(component_t* component)
{
    (void)component;
    if (!sd_card_inserted()) {
        component_t* screen = insert_sd_card_create(_backup);
        ui_screen_stack_switch(screen);
    } else {
        _backup();
    }
}

static void _confirm_time(component_t* component)
{
    (void)component;

    uint32_t seed_birthdate;
    memory_get_seed_birthdate(&seed_birthdate);
    if (seed_birthdate == 0) {
        if (!memory_set_seed_birthdate(_unix_timestamp)) {
            _result = false;
            return;
        }
    }

    _check_sd_card_inserted_and_do_backup(NULL);
}

static void _reject_time(component_t* component)
{
    (void)component;
    _dismiss();
    _done = true;
}

static void _set_time(const CreateBackupRequest* create_backup)
{
    _unix_timestamp = create_backup->timestamp;
    if (_unix_timestamp == 0) {
        _result = false;
        _done = true;
        return;
    }
    // TODO: Use utc_timestring for backup filename, human readable UTC time
    char utc_timestring[40] = {0};
    time_t timestamp = (time_t)_unix_timestamp;
    struct tm* utc_time = gmtime(&timestamp);
    strftime(utc_timestring, sizeof(utc_timestring), "%a %Y-%m-%d", utc_time);

    // Local time for confirming on screen
    time_t local_timestamp = timestamp + create_backup->timezone_offset;
    struct tm* local_time = localtime(&local_timestamp);
    static char local_timestring[100] = {0};
    strftime(local_timestring, sizeof(local_timestring), "%a %Y-%m-%d", local_time);

    ui_screen_stack_push(
        confirm_create("Is today?", local_timestring, _confirm_time, _reject_time));
}

bool workflow_backup_create(const CreateBackupRequest* create_backup)
{
    _result = false;
    _done = false;
    if (create_backup->timezone_offset < MAX_WEST_UTC_OFFSET ||
        create_backup->timezone_offset > MAX_EAST_UTC_OFFSET) {
        return false;
    }
    _set_time(create_backup);
    ui_screen_process(_is_done);
    return _result;
}

bool workflow_backup_check(char* id_out, bool silent)
{
    char backup_name[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    backup_error_t res = backup_check(id_out, backup_name);
    switch (res) {
    case BACKUP_ERR_SD_LIST:
    case BACKUP_ERR_SD_READ:
    case BACKUP_ERR_SD_WRITE:
        ui_screen_stack_push(status_create(
            "Could not read\nor write to the\nmicro SD card",
            false,
            STATUS_DEFAULT_DELAY,
            _dismiss));
        return false;
    case BACKUP_ERR_CHECK:
        if (!silent) {
            ui_screen_stack_push(
                status_create("Backup missing\nor invalid", false, STATUS_DEFAULT_DELAY, _dismiss));
        }
        return false;
    case BACKUP_OK:
        if (!silent) {
            _done = false;
            ui_screen_stack_push(status_create(
                "Backup valid\nConfirm details", true, STATUS_DEFAULT_DELAY, _confirm_backup));
            ui_screen_process(_is_done);
            _done = false;
            ui_screen_stack_switch(
                confirm_create_scrollable(backup_name, id_out, _confirm_backup_2, NULL));
            ui_screen_process(_is_done);
            ui_screen_stack_pop();
        }
        return true;
    default: {
        char err_msg[100];
        snprintf(err_msg, 100, "Could not check\nbackup (%d)", res);
        ui_screen_stack_push(status_create(err_msg, false, STATUS_DEFAULT_DELAY, _dismiss));
        return false;
    }
    }
}
