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

#include "commander_states.h"

#include "hardfault.h"
#include "hww.pb.h"
#include "memory.h"

// If non-zero (set), only this api endpoint can be called next.
static commander_states_endpoint_id _force_next = 0;

// api commands the host can invoke regardless of which of the three main states we are in.
// Exception applies if one of the api endpoints forces subsequent calls, see
// `commander_states_force_next()`.
static commander_states_endpoint_id _commands_anytime[] = {
    Request_device_info_tag,
    Request_reboot_tag,
    Request_device_name_tag,
    Request_device_language_tag,
    Request_check_sdcard_tag,
    Request_insert_remove_sdcard_tag,
    Request_list_backups_tag,
    Request_bitboxbase_tag,
};

// api commands the host can invoke when the device is uninitialized.
static commander_states_endpoint_id _commands_uninitialized[] = {
    Request_set_password_tag,
    Request_restore_backup_tag,
    Request_restore_from_mnemonic_tag,
};

// api commands the host can invoke when the device is seeded.
static commander_states_endpoint_id _commands_seeded[] = {
    Request_create_backup_tag,
    Request_set_password_tag,
    Request_restore_backup_tag,
    Request_restore_from_mnemonic_tag,
};

// api commands the host can invoke when the device is initialized.
static commander_states_endpoint_id _commands_initialized[] = {
    Request_random_number_tag,
    Request_btc_pub_tag,
    Request_btc_sign_init_tag,
    Request_check_backup_tag,
    Request_create_backup_tag,
    Request_show_mnemonic_tag,
    Request_set_mnemonic_passphrase_enabled_tag,
    Request_eth_tag,
    Request_reset_tag,
};

commander_states_state_t commander_states_state(void)
{
    if (memory_is_initialized()) {
        return COMMANDER_STATES_INITIALIZED;
    }
    if (memory_is_seeded()) {
        return COMMANDER_STATES_SEEDED;
    }
    return COMMANDER_STATES_UNINITIALIZED;
}

void commander_states_force_next(commander_states_endpoint_id id)
{
    _force_next = id;
}

void commander_states_clear_force_next(void)
{
    _force_next = 0;
}

bool commander_states_can_call(commander_states_endpoint_id id)
{
    if (_force_next != 0) {
        return _force_next == id;
    }
    for (size_t i = 0; i < sizeof(_commands_anytime) / sizeof(_commands_anytime[0]); i++) {
        if (_commands_anytime[i] == id) {
            return true;
        }
    }
    commander_states_endpoint_id* ok_list = NULL;
    size_t ok_list_len = 0;
    switch (commander_states_state()) {
    case COMMANDER_STATES_UNINITIALIZED:
        ok_list = _commands_uninitialized;
        ok_list_len = sizeof(_commands_uninitialized) / sizeof(_commands_uninitialized[0]);
        break;
    case COMMANDER_STATES_SEEDED:
        ok_list = _commands_seeded;
        ok_list_len = sizeof(_commands_seeded) / sizeof(_commands_seeded[0]);
        break;
    case COMMANDER_STATES_INITIALIZED:
        ok_list = _commands_initialized;
        ok_list_len = sizeof(_commands_initialized) / sizeof(_commands_initialized[0]);
        break;
    default:
        Abort("invalid state");
    }
    for (size_t i = 0; i < ok_list_len; i++) {
        if (ok_list[i] == id) {
            return true;
        }
    }
    return false;
}
