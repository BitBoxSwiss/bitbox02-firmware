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

#ifndef _PASSWORD_H_
#define _PASSWORD_H_

#include <compiler_util.h>
#include <ui/components/trinary_input_string.h> // for INPUT_STRING_MAX_SIZE

#include <stdbool.h>

/**
 * Asks the user to set a password by entering it once and then confirming it.
 * @param[out] password_out must be of size INPUT_STRING_MAX_SIZE.
 * Use `UTIL_CLEANUP_STR` to destroy the password after use.
 * @return true if the the two entered passwords match. Returns false otherwise.
 */
USE_RESULT bool password_set(char* password_out);

#endif
