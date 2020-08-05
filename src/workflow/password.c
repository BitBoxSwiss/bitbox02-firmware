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

#include "password.h"
#include <rust/rust.h>

bool password_set(char* password_out)
{
    return rust_workflow_password_enter_twice_blocking(
        rust_util_cstr_mut(password_out, SET_PASSWORD_MAX_PASSWORD_LENGTH));
}
