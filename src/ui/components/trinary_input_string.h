// SPDX-License-Identifier: Apache-2.0

#ifndef _TRINARY_INPUT_STRING_H_
#define _TRINARY_INPUT_STRING_H_

#include <ui/component.h>

#include <stddef.h>

// including null terminator
#define INPUT_STRING_MAX_SIZE 150

typedef struct {
    const char* title;
    // Restrict and autocomplete to this list of BIP39 words. The elements are indices into the
    // BIP39 English wordlist. Set to NULL to allow arbitrary input.
    uint16_t const* wordlist;
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
    // start with the digits keyboard instead of the lowercase keyboard.
    bool default_to_digits;
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
    void (*confirm_cb)(const char* input, void* confirm_user_data),
    void* confirm_user_data,
    void (*cancel_cb)(void* cancel_user_data),
    void* cancel_user_data);

/**
 * Only applicable in wordlist-mode.
 * Sets the current word. The user can then accept it or delete characters.
 * The word must be in the wordlist as passed to `trinary_input_string_create_wordlist()`, otherwise
 * this function aborts.
 */
void trinary_input_string_set_input(component_t* trinary_input_string, const char* word);

#endif
