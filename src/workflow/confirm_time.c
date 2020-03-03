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

#include "confirm_time.h"
#include <time.h>
#include <workflow/confirm.h>

bool workflow_confirm_time(uint32_t timestamp, int32_t timezone_offset, bool date_only)
{
    if (timestamp == 0) {
        return false;
    }
    // Local time for confirming on screen
    time_t local_timestamp = timestamp + timezone_offset;
    struct tm* local_time = localtime(&local_timestamp);
    static char local_timestring[100] = {0};
    const char* title;
    if (date_only) {
        title = "Is today?";
        strftime(local_timestring, sizeof(local_timestring), "%a %Y-%m-%d", local_time);
    } else {
        title = "Is now?";
        strftime(local_timestring, sizeof(local_timestring), "%a %Y-%m-%d\n%H:%M:%S", local_time);
    }

    const confirm_params_t params = {
        .title = title,
        .body = local_timestring,
    };
    return workflow_confirm_blocking(&params);
}
