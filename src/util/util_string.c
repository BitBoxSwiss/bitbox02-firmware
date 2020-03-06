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

#include "util_string.h"

#include <hardfault.h>
#include <rust/rust.h>

#include <string.h>

bool util_string_validate_name(const char* name, size_t max_len)
{
    if (name == NULL) {
        Abort("util_string_validate_name");
    }
    const size_t len = strlen(name);
    if (len == 0 || len > max_len) {
        return false;
    }
    if (!rust_util_all_ascii(rust_util_cstr(name))) {
        return false;
    }
    if (name[0] == ' ') {
        return false;
    }
    if (name[len - 1] == ' ') {
        return false;
    }
    return true;
}
