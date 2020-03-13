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
#include <ui/components/trinary_input_string.h>
#include <util.h>

#include <stdio.h>

static char _word[WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1];

static void _confirm(const char* word, void* param)
{
    (void)param;
    int snprintf_result = snprintf(_word, sizeof(_word), "%s", word);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_word)) {
        Abort("length mismatch");
    }
    workflow_blocking_unblock();
}

static void _cancel(void* param)
{
    (void)param;
    workflow_cancel();
}
bool workflow_trinary_input_wordlist(
    const char* title,
    const char* const* wordlist,
    size_t wordlist_size,
    char* word_out)
{
    if (!workflow_cancel_run(
            "Restore",
            trinary_input_string_create_wordlist(
                title, wordlist, wordlist_size, _confirm, NULL, _cancel, NULL))) {
        return false;
    }
    snprintf(word_out, WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1, "%s", _word);
    util_zero(_word, sizeof(_word));
    return true;
}
