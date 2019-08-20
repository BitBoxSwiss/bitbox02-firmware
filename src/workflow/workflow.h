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

#ifndef _WORKFLOW_H_
#define _WORKFLOW_H_

#include <stdbool.h>
#include <stdint.h>

typedef enum { WORKFLOW_STATE_CHOOSE_ORIENTATION } workflow_state_t;

/**
 * Pushes a confirm string on the screen a with a "Dismiss" button, to show data
 * on the screen for the user to verify.
 */
void workflow_confirm_dismiss(const char* title, const char* body);

/**
 * Invokes a workflow based on the input.
 */
void workflow_change_state(workflow_state_t state);

/**
 * Switches to either the initialization or the unlock state depending on if the
 * device is initialized or not.
 */
void workflow_start(void);

typedef struct {
    bool (*const get_bip39_mnemonic)(char** mnemonic_out);
    bool (*const sd_card_inserted)(void);
    bool (*const get_bip39_word)(uint16_t idx, char** word_out);
} workflow_interface_functions_t;

/**
 * Set the functions that we will use to perform certain operations.
 * This is meant improve modularity and make unit testing easier.
 */
void workflow_set_interface_functions(workflow_interface_functions_t* ifs);
/**
 * @return The previously set workflow functions.
 */
workflow_interface_functions_t* workflow_get_interface_functions(void);

#endif
