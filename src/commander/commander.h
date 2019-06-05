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

#ifndef _COMMANDER_H_
#define _COMMANDER_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "hww.pb.h"

// error flags. From this, commander_error_t and the static
// commander_error_code, commander_error_message are produced.
#define COMMANDER_ERROR_TABLE                               \
    X(COMMANDER_OK, 0, "")                                  \
    X(COMMANDER_ERR_INVALID_INPUT, 101, "invalid input")    \
    X(COMMANDER_ERR_MEMORY, 102, "memory")                  \
    X(COMMANDER_ERR_GENERIC, 103, "generic error")          \
    X(COMMANDER_ERR_USER_ABORT, 104, "aborted by the user") \
    X(COMMANDER_ERR_INVALID_STATE, 105, "can't call this endpoint: wrong state")

#define X(a, b, c) a,
typedef enum { COMMANDER_ERROR_TABLE } commander_error_t;
#undef X

/**
 * Receives and processes a command.
 */
size_t commander(const uint8_t* input, size_t in_len, uint8_t* output, size_t max_out_len);

#endif
