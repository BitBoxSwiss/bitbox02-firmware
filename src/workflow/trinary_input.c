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

#include "trinary_input.h"

#include "blocking.h"
#include "cancel.h"

#include <hardfault.h>
#include <ui/components/menu.h>
#include <ui/components/trinary_input_string.h>
#include <ui/screen_stack.h>
#include <util.h>

#include <stdio.h>

static char _word[WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1];
static workflow_trinary_input_result_t _cancel_reason;

static const char* _cancel_choices[] = {
    "Edit previous word",
    "Cancel restore",
};
#define CHOICE_DELETE 0
#define CHOICE_CANCEL 1

static void _confirm(const char* word, void* param)
{
    (void)param;
    int snprintf_result = snprintf(_word, sizeof(_word), "%s", word);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_word)) {
        Abort("length mismatch");
    }
    workflow_blocking_unblock();
}

static void _select(uint8_t choice_idx, void* param)
{
    (void)param;
    ui_screen_stack_pop();
    if (choice_idx == CHOICE_DELETE) {
        _cancel_reason = WORKFLOW_TRINARY_INPUT_RESULT_DELETE;
        workflow_cancel_force();
    } else if (choice_idx == CHOICE_CANCEL) {
        _cancel_reason = WORKFLOW_TRINARY_INPUT_RESULT_CANCEL;
        workflow_cancel();
    }
}

static void _pop(void* param)
{
    (void)param;
    ui_screen_stack_pop();
}

static void _cancel(void* param)
{
    size_t word_idx = *(size_t*)param;
    if (word_idx == 0) {
        _cancel_reason = WORKFLOW_TRINARY_INPUT_RESULT_CANCEL;
        workflow_cancel();
        return;
    }
    ui_screen_stack_push(menu_create(
        _cancel_choices,
        _select,
        NULL,
        sizeof(_cancel_choices) / sizeof(_cancel_choices[0]),
        "Choose",
        NULL,
        NULL,
        _pop,
        NULL,
        NULL));
}

workflow_trinary_input_result_t workflow_trinary_input_wordlist(
    size_t word_idx,
    const char* const* wordlist,
    size_t wordlist_size,
    const char* preset,
    char* word_out)
{
    char title[50] = {0};
    if (word_idx == 0) {
        snprintf(title, sizeof(title), "1st word");
    } else if (word_idx == 1) {
        snprintf(title, sizeof(title), "2nd word");
    } else if (word_idx == 2) {
        snprintf(title, sizeof(title), "3rd word");
    } else {
        snprintf(title, sizeof(title), "%dth word", (int)(word_idx + 1));
    }

    component_t* component = trinary_input_string_create_wordlist(
        title, wordlist, wordlist_size, _confirm, NULL, _cancel, &word_idx, word_idx > 0);
    if (preset != NULL) {
        trinary_input_string_set_input(component, preset);
    }
    if (!workflow_cancel_run("Restore", component)) {
        return _cancel_reason;
    }
    snprintf(word_out, WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1, "%s", _word);
    util_zero(_word, sizeof(_word));
    return WORKFLOW_TRINARY_INPUT_RESULT_OK;
}
