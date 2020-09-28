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
// to allow drop in replacement to set_password.h
#define SET_PASSWORD_MAX_PASSWORD_LENGTH INPUT_STRING_MAX_SIZE

/********************************** Create Instance **********************************/

/**
 * Creates a string input screen based on trinary input.
 * @param[in] title to show in the center before the user starts inputting.
 * @param[in] The user can only confirm words from the wordlist. The available keyboard letters
 * adjust automatically so only words from the wordlist can be entered. Must be sorted, with all
 * words being lowercase 'a-z'.
 * @param[in] wordlist_size number of words in the wordlist.
 * @param[in] confirm_cb The callback that is called when the user entered the string. Will be
 * called at most once.
 * @param[in] cancel_cb Called when the user cancels by hitting the back button.
 * @param[in] cancel_is_backbutton whether the cancel button should be rendered as a back button
 *            instead of as a cross.
 */
component_t* trinary_input_string_create_wordlist(
    const char* title,
    const char* const* wordlist,
    size_t wordlist_size,
    void (*confirm_cb)(const char* input, void* param),
    void* confirm_callback_param,
    void (*cancel_cb)(void* param),
    void* cancel_callback_param,
    bool cancel_is_backbutton);

component_t* trinary_input_string_create_password(
    const char* title,
    bool special_chars,
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
