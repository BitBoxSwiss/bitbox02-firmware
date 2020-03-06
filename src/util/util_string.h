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

#ifndef _UTIL_NAME_VALIDATE_H_
#define _UTIL_NAME_VALIDATE_H_

#include <compiler_util.h>
#include <stdbool.h>
#include <stddef.h>

/**
 * Validate a user given name. The name must be smaller or equal to `max_len` and larger than 0 in
 * size (without the null terminator), consist of printable ASCII characters only (and space), not
 * start or end with whitespace, and contain no whitespace other than space.
 * @return true if the name is valid, false if it is invalid.
 */
USE_RESULT bool util_string_validate_name(const char* name, size_t max_len);

#endif
