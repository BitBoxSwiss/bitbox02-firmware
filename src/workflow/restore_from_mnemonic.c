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

#include "password.h"
#include "status.h"
#include "trinary_input.h"
#include "unlock.h"

#include <hardfault.h>
#include <keystore.h>
#include <memory.h>
#include <ui/components/trinary_input_string.h>
#include <util.h>

#include <stdio.h>
#include <string.h>

#define WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS 24

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

    const uint8_t num_words = 24;
    for (uint8_t word_idx = 0; word_idx < num_words; word_idx++) {
        char word[WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH] = {0};
        char title[50] = {0};
        _set_title(word_idx, title, sizeof(title));
        if (!workflow_trinary_input_wordlist(
                title, (const char* const*)wordlist, BIP39_WORDLIST_LEN, word)) {
            return false;
        }
        if (word_idx != 0) {
            strcat(mnemonic_out, " ");
        }
        strcat(mnemonic_out, word);
    }
    return true;
}

bool workflow_restore_from_mnemonic(void)
{
    // same as: (MAX_WORD_LENGTH-1) * MAX_WORDS + (MAX_WORDS-1) + 1
    // (chars per word without null terminator) * max_words + spaces between words + null terminator
    char mnemonic
        [WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH * WORKFLOW_RESTORE_FROM_MNEMONIC_MAX_WORDS] = {0};
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

    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH] = {0};
    UTIL_CLEANUP_STR(password);
    if (!password_set(password)) {
        return false;
    }
    if (!keystore_encrypt_and_store_seed(seed, seed_len, password)) {
        workflow_status_create("Could not\nrestore backup", false);
        return false;
    }
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
