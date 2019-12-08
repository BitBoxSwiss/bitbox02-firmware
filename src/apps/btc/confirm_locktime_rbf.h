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

#ifndef _APPS_BTC_CONFIRM_LOCKTIME_RBF_H_
#define _APPS_BTC_CONFIRM_LOCKTIME_RBF_H_

#include <util.h>

enum apps_btc_rbf_flag {
    CONFIRM_LOCKTIME_RBF_OFF,
    CONFIRM_LOCKTIME_RBF_ON,
    CONFIRM_LOCKTIME_RBF_DISABLED
};

/**
 * Shows a confirmation dialog to the user showing the locktime of the transaction
 * to be signed, if the locktime is > 0 and whether it is RBF. This call blocks until the user
 * confirms.
 * The function expects either locktime > 0 or CONFIRM_LOCKTIME_RBF_ON, providing neither will
 * result in unwanted behavior.
 * @param[in] locktime to be confirmed.
 * @param[in] rbf flag to display different text based on its setting.
 * @return true if the user confirms, false if the user rejects.
 */
bool apps_btc_confirm_locktime_rbf(uint32_t locktime, enum apps_btc_rbf_flag rbf);

#endif
