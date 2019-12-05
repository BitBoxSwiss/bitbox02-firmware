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

#include "verify_pub.h"
#include "confirm.h"

#include <stddef.h>
#include <string.h>

bool workflow_verify_pub(const char* title, const char* pub)
{
    const int buf_len = 128;
    char buf[buf_len];
    if (strlen(title) + 1 < buf_len) {
        memset(buf, 0, buf_len);
        UG_WrapTitleString(title, buf, 55);
        title = buf;
    }
    const confirm_params_t params = {
        .title = title,
        .body = pub,
        .scrollable = true,
    };
    return workflow_confirm_blocking(&params);
}
