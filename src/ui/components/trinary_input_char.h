// SPDX-License-Identifier: Apache-2.0

#ifndef _TRINARY_INPUT_CHAR_H_
#define _TRINARY_INPUT_CHAR_H_

#include <ui/component.h>
#include <ui/ugui/ugui.h>

#include <stdbool.h>

/**
 * Presents user input in form of three buttons at the bottom, with at most three taps needed to
 * select a character. Use `trinary_input_char_set_alphabet()` to set the current keyboard.
 * @param[in] character_chosen_cb will be called after a letter is chosen.
 * @param[in] parent parent component.
 */
component_t* trinary_input_char_create(
    void (*character_chosen_cb)(component_t*, char),
    component_t* parent);

/**
 * Set a new charset, e.g. when switching keyboards to lowercase/uppercase/numeric, or to restrict
 * the chars to facilitate entering dictionary words.
 * @param[in] alphabet_input are the available characters. Can be at most 26 chars (27
 * @param[in] horiz_space Horizontal space between characters in a group.
 * including null).
 */
void trinary_input_char_set_alphabet(
    component_t* component,
    const char* alphabet_input,
    UG_S16 horiz_space);

/**
 * @return whether the user user already tapped one of the buttons and has not yet chosen the
 * latter. Once a letter is chosen, return false again until the next tap.
 */
bool trinary_input_char_in_progress(component_t* component);

/**
 * Go back to the previous alphabet in the navigation stack.
 * @return true if successfully went back, false if navigation stack was empty.
 */
bool trinary_input_char_go_back(component_t* component);

/**
 * @return whether the alphabet provided is the empty set.
 */
bool trinary_input_char_alphabet_is_empty(component_t* component);

#endif
