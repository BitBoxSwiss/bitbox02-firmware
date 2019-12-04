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

#include "restore_from_mnemonic.h"

#include "blocking.h"
#include "confirm.h"
#include "password.h"
#include "status.h"
#include "trinary_input.h"
#include "unlock.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory/memory.h>
#include <securechip/securechip.h>
#include <ui/component.h>
#include <ui/components/trinary_choice.h>
#include <ui/components/trinary_input_string.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>
#include <util.h>
#include <workflow/confirm_time.h>

#include <stdio.h>
#include <string.h>

#define WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS 24

static trinary_choice_t _number_of_words_choice;
static void _number_of_words_picked(component_t* trinary_choice, trinary_choice_t choice)
{
    (void)trinary_choice;
    _number_of_words_choice = choice;
    workflow_blocking_unblock();
}

/**
 * Workflow to pick how many words.
 * @param[out] number_of_words_out 12, 18 or 24.
 */
static bool _pick_number_of_words(uint8_t* number_of_words_out)
{
    ui_screen_stack_push(
        trinary_choice_create("How many words?", "12", "18", "24", _number_of_words_picked, NULL));
    bool result = workflow_blocking_block();
    ui_screen_stack_pop();
    switch (_number_of_words_choice) {
    case TRINARY_CHOICE_LEFT:
        *number_of_words_out = 12;
        break;
    case TRINARY_CHOICE_MIDDLE:
        *number_of_words_out = 18;
        break;
    case TRINARY_CHOICE_RIGHT:
        *number_of_words_out = 24;
        break;
    default:
        Abort("restore_from_mnemonic: unreachable");
    }
    return result;
}

static void _cleanup_wordlist(char*** wordlist)
{
    for (size_t i = 0; i < BIP39_WORDLIST_LEN; i++) {
        if ((*wordlist)[i] != NULL) {
            free((*wordlist)[i]);
            (*wordlist)[i] = NULL;
        }
    }
}

static void _set_title(uint8_t word_idx, char* title_out, size_t title_out_len)
{
    if (word_idx == 0) {
        snprintf(title_out, title_out_len, "Enter 1st word");
    } else if (word_idx == 1) {
        snprintf(title_out, title_out_len, "Enter 2nd word");
    } else if (word_idx == 2) {
        snprintf(title_out, title_out_len, "Enter 3rd word");
    } else {
        snprintf(title_out, title_out_len, "Enter %dth word", (int)(word_idx + 1));
    }
}

static bool _get_mnemonic(char* mnemonic_out)
{
    char* wordlist[BIP39_WORDLIST_LEN] = {0};
    char** __attribute__((__cleanup__(_cleanup_wordlist))) __attribute__((unused)) wordlist_clean =
        wordlist;
    for (size_t i = 0; i < BIP39_WORDLIST_LEN; i++) {
        if (!keystore_get_bip39_word(i, &wordlist[i])) {
            return false;
        }
    }

    uint8_t num_words;
    if (!_pick_number_of_words(&num_words)) {
        return false;
    }
    char num_words_success_msg[20];
    snprintf(num_words_success_msg, sizeof(num_words_success_msg), "Enter %d words", num_words);
    workflow_status_create(num_words_success_msg, true);

    for (uint8_t word_idx = 0; word_idx < num_words; word_idx++) {
        char word[WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1] = {0};
        char title[50] = {0};
        _set_title(word_idx, title, sizeof(title));
        if (!workflow_trinary_input_wordlist(
                title, (const char* const*)wordlist, BIP39_WORDLIST_LEN, word)) {
            return false;
        }
        if (word_idx != 0) {
            strcat(mnemonic_out, " "); // NOLINT (gcc and clang cannot agree on best practice here)
        }
        strncat(mnemonic_out, word, WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH);
    }
    return true;
}

bool workflow_restore_from_mnemonic(const RestoreFromMnemonicRequest* request)
{
    // same as: MAX_WORD_LENGTH * MAX_WORDS + (MAX_WORDS - 1) + 1
    // (chars per word without null terminator) * (max words) + (spaces between words) + (null
    // terminator)
    char mnemonic
        [(WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1) * WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS] =
            {0};
    UTIL_CLEANUP_STR(mnemonic);
    if (!_get_mnemonic(mnemonic)) {
        return false;
    }
    uint8_t seed[32];
    UTIL_CLEANUP_32(seed);
    size_t seed_len = 0;
    if (!keystore_bip39_mnemonic_to_seed(mnemonic, seed, &seed_len)) {
        workflow_status_create("Invalid mnemonic", false);
        return false;
    }

    workflow_status_create("Mnemonic valid", true);

    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    // If entering password fails (repeat password does not match the first), we don't want to abort
    // the process immediately. We break out only if the user confirms.
    while (true) {
        if (!password_set(password)) {
            if (!workflow_confirm("", "Passwords\ndo not match.\nTry again?", false, false)) {
                return false;
            }
            continue;
        }
        break;
    }
    if (!keystore_encrypt_and_store_seed(seed, seed_len, password)) {
        workflow_status_create("Could not\nrestore backup", false);
        return false;
    }
#if APP_U2F == 1
    if (!workflow_confirm_time(request->timestamp, request->timezone_offset, false)) {
        return false;
    }
    if (!securechip_u2f_counter_set(request->timestamp)) {
        // ignore error
    }
#else
    (void)request;
#endif
    if (!memory_set_initialized()) {
        return false;
    }
    uint8_t remaining_attempts;
    if (keystore_unlock(password, &remaining_attempts) != KEYSTORE_OK) {
        // This should/can never happen, but let's check anyway.
        Abort("workflow_restore_from_mnemonic: unlock failed");
    }
    return workflow_unlock_bip39();
}
