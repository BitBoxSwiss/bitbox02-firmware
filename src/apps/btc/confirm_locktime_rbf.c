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

#include "confirm_locktime_rbf.h"

#include <workflow/confirm.h>
#include <workflow/status.h>

bool apps_btc_confirm_locktime_rbf(uint32_t locktime, enum apps_btc_rbf_flag rbf)
{
    char formatted_locktime_rbf[100] = {0};
    const char* locktime_text = "Locktime on block:";
    const char* rbf_text;
    if (rbf == CONFIRM_LOCKTIME_RBF_ON) {
        rbf_text = "Transaction is RBF";
    } else if (rbf == CONFIRM_LOCKTIME_RBF_OFF) {
        rbf_text = "Transaction is not RBF";
    } else {
        rbf_text = "";
    }
    snprintf(
        formatted_locktime_rbf,
        sizeof formatted_locktime_rbf,
        "%s\n%lu\n%s",
        locktime_text,
        (unsigned long)locktime,
        rbf_text);

    const confirm_params_t params = {
        .title = "",
        .body = formatted_locktime_rbf,
    };
    bool result = workflow_confirm_blocking(&params);
    if (!result) {
        workflow_status_create("Transaction\ncanceled", false);
    }
    return result;
}
