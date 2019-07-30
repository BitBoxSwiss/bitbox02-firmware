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

#include "show_mnemonic.h"

#include "blocking.h"
#include "password.h"
#include "status.h"
#include "workflow.h"

#include <hardfault.h>
#include <random.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>

#define BIP39_NUM_WORDS 24
#define MAX_WORDLENGTH 20

#define NUM_RANDOM_WORDS 5
#define NUM_CONFIRM_SCREEN_WORDS (NUM_RANDOM_WORDS + 1)

// The
static char* _mnemonic;
static uint16_t _mnemonic_length;
static const char* _wordlist[BIP39_NUM_WORDS];

// The list of randomly selected, unique words + the word form the seed phrase at the
// _check_word_idx.
static const char* _confirm_wordlist[NUM_CONFIRM_SCREEN_WORDS];
// The position of the correct word in the random word list generated during the seed phrase
// checkup.
static uint8_t _current_correct_idx;
// During seed phrase confirmation, this index is used to go to the seed phrase.
static uint8_t _check_word_idx = 0;

static void _check_word(uint8_t selection);

static const char* _back_label = "Back to seed phrase";

static void _split_and_save_wordlist(uint8_t* length)
{
    char* next_word = strtok(_mnemonic, " ");

    int i = 0;
    while (next_word != NULL) {
        _wordlist[i] = next_word;
        next_word = strtok(NULL, " ");
        i++;
    }
    *length = i;
}

static bool _is_in_list(uint8_t number, const uint8_t list[], uint8_t length)
{
    for (int i = 0; i < length; i++) {
        if (number == list[i]) {
            return true;
        }
    }
    return false;
}

/**
 * Creates a list of unique, random words from the BIP39 wordlist and places the passed
 * word at a random position. The generated list of words is used to show it to the user
 * who must identify the word that was pased to the function. This functionality is used
 * to confirm that the user correctly copied his/her BIP39 seed phrase.
 *
 * @param[out] wordlist A list of randomly selected, unique words and the passed word.
 * @param[in] length The length of the wordlist that is generated.
 * @param[in] word The word that is put into the wordlist at a random position.
 */
static uint8_t _create_random_unique_words(const char** wordlist, uint8_t length, const char* word)
{
    uint8_t random_numbers[length - 1];
    uint8_t current_length = 0;
    uint8_t i = 0;
    uint8_t index_word = random_byte_mcu() % length;
    while (i < length) {
        if (i == index_word) {
            // add correct word at random location
            wordlist[i] = word;
            i++;
            continue;
        }
        uint8_t random_num_b1 = random_byte_mcu();
        uint8_t random_num_b2 = random_byte_mcu();
        uint16_t random_num = (random_num_b1 << 8 | random_num_b2) %
                              workflow_get_interface_functions()->get_bip39_wordlist_length();
        if (_is_in_list(random_num, random_numbers, current_length)) {
            // already chose that word, so select a different word
            continue;
        }
        char* picked_word;
        workflow_get_interface_functions()->get_bip39_word(random_num, &picked_word);
        wordlist[i] = picked_word;
        if (STREQ(wordlist[i], word)) {
            // if it's the same as the correct word, select a different word
            continue;
        }
        random_numbers[current_length] = random_num;
        current_length++;
        i++;
    }
    return index_word;
}

/**
 * Creates a list of random, unique words and displays them to the user. The
 * user must identify, which of the words was at the _check_word_idx of the
 * seed phrase.
 */
static void _confirm_mnemonic(void)
{
    if (_check_word_idx == BIP39_NUM_WORDS) {
        // If we're at the last screen, we pop the check-word screen for the last word and replace
        // the underlying BIP39 seed phrase screen. We can only replace it now, because during
        // word-checking, the user might still want to go back to the seed phrase.
        util_zero(_mnemonic, _mnemonic_length);
        free(_mnemonic);
        workflow_blocking_unblock();
        return;
    }
    _current_correct_idx = _create_random_unique_words(
        _confirm_wordlist, NUM_RANDOM_WORDS, _wordlist[_check_word_idx]);
    _confirm_wordlist[NUM_CONFIRM_SCREEN_WORDS - 1] = _back_label;
    component_t* confirm_mnemonic = confirm_mnemonic_create(
        _confirm_wordlist, NUM_CONFIRM_SCREEN_WORDS, _check_word_idx, _check_word);
    ui_screen_stack_push(confirm_mnemonic);
}

static void _check_word(uint8_t selection)
{
    if (selection == NUM_CONFIRM_SCREEN_WORDS - 1) {
        ui_screen_stack_pop();
    } else if (_current_correct_idx == selection) {
        _check_word_idx++;
        ui_screen_stack_pop();
        _confirm_mnemonic();
    } else {
        // TODO: indicate that the wrong word was selected
        screen_print_debug("incorrect word selected. Try again", 1000);
    }
}

bool workflow_show_mnemonic_create(void)
{
    if (!password_check()) {
        return false;
    }
    if (!workflow_get_interface_functions()->get_bip39_mnemonic(&_mnemonic)) {
        Abort("mnemonic create not possible");
    }
    // This field must be set before we tokenize the _mnemonic,
    // because we use the length when we zero the memory after confirmation.
    _mnemonic_length = strlens(_mnemonic);
    uint8_t length;
    _split_and_save_wordlist(&length);
    ui_screen_stack_push(
        scroll_through_all_variants_create(_wordlist, NULL, length, true, _confirm_mnemonic, NULL));
    bool unblock_result = workflow_blocking_block();
    ui_screen_stack_pop();
    workflow_status_create("Success", true);
    return unblock_result;
}
