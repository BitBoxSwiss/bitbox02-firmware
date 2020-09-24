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

#ifndef _WORKFLOW_TRINARY_INPUT_H_
#define _WORKFLOW_TRINARY_INPUT_H_

#include <compiler_util.h>

#include <stdbool.h>
#include <stddef.h>

// Excluding null terminator. 8 is the longest bip39 word.
#define WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH (8U)

typedef enum {
    // The user entered the word.
    WORKFLOW_TRINARY_INPUT_RESULT_OK,
    // The user cancelled the operation.
    WORKFLOW_TRINARY_INPUT_RESULT_CANCEL,
    // The user wants to go back to edit the previous word.
    WORKFLOW_TRINARY_INPUT_RESULT_DELETE,
} workflow_trinary_input_result_t;

/**
 * The length of word_out must be WORKFLOW_TRINARY_INPUT_MAX_WORD_LENGTH + 1
 */
USE_RESULT workflow_trinary_input_result_t workflow_trinary_input_wordlist(
    const char* title,
    const char* const* wordlist,
    size_t wordlist_size,
    char* word_out);

#endif
