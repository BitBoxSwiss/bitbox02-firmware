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

#ifndef _TRINARY_INPUT_STRING_H_
#define _TRINARY_INPUT_STRING_H_

#include <ui/component.h>

#include <stddef.h>

// including null terminator
#define INPUT_STRING_MAX_SIZE 150

typedef struct {
    const char* title;
    // Restrict and autocomplete to this list of words. Set to NULL to allow arbitrary input.
    const char* const* wordlist;
    // If true, the user can enter numbers only.
    bool number_input;
    // Set to 0 if wordlist is NULL.
    size_t wordlist_size;
    // Mask the chars entered as `*`. For password input.
    bool hide;
    // Add special chars keyboard. Otherwise abc/ABC/012 only.
    bool special_chars;
    // Confirm via longtouch. If false, confirmation happens with a short tap.
    bool longtouch;
    // whether the cancel button should be rendered as a back button instead of as a cross.
    bool cancel_is_backbutton;
} trinary_input_string_params_t;

/********************************** Create Instance **********************************/

/**
 * Creates a string input screen based on trinary input.
 * @param[in] params See `trinary_input_string_params_t`.
 * @param[in] confirm_cb The callback that is called when the user entered the string. Will be
 * called at most once.
 * @param[in] cancel_cb Called when the user cancels by hitting the back button.
 */
component_t* trinary_input_string_create(
    const trinary_input_string_params_t* params,
    void (*confirm_cb)(const char* input, void* param),
    void* confirm_callback_param,
    void (*cancel_cb)(void* param),
    void* cancel_callback_param);

/**
 * Only applicable in wordlist-mode.
 * Sets the current word. The user can then accept it or delete characters.
 * The word must be in the wordlist as passed to `trinary_input_string_create_wordlist()`, otherwise
 * this function aborts.
 */
void trinary_input_string_set_input(component_t* trinary_input_string, const char* word);

#endif
