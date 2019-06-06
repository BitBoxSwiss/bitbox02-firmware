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

#include <stdio.h>
#include <string.h>

#include <hardfault.h>
#include <ui/components/ui_components.h>
#include <ui/ui_util.h>

#include "show_mnemonic.h"

/**
 * Component data.
 */
typedef struct {
    void (*confirm_mnemonic)(void);
    component_t* continue_button;
} show_mnemonic_data_t;

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop,
};

/**
 * Creates a screen that allows to scroll through the mnemonic words.
 * @param[in] wordlist The mnemonic sentence, split into words.
 * @param[in] length The amount of words in the mnemonic sentence.
 * @param[in] confirm_mnemonic A callback that lets the user confirm the words he/she wrote down.
 */
component_t* show_mnemonic_create(
    const char** wordlist,
    uint8_t length,
    void (*confirm_mnemonic)(void))
{
    show_mnemonic_data_t* data = malloc(sizeof(show_mnemonic_data_t));
    if (!data) {
        Abort("Error: malloc show_mnemonic data");
    }
    memset(data, 0, sizeof(show_mnemonic_data_t));

    data->confirm_mnemonic = confirm_mnemonic;

    component_t* show_mnemonic = malloc(sizeof(component_t));
    if (!show_mnemonic) {
        Abort("Error: malloc show_mnemonic");
    }
    memset(show_mnemonic, 0, sizeof(component_t));
    show_mnemonic->f = &_component_functions;

    show_mnemonic->data = data;
    show_mnemonic->dimension.width = SCREEN_WIDTH;
    show_mnemonic->dimension.height = SCREEN_HEIGHT;

    ui_util_add_sub_component(
        show_mnemonic,
        scroll_through_all_variants_create(
            wordlist, NULL, length, true, confirm_mnemonic, show_mnemonic));

    return show_mnemonic;
}
