// Copyright 2020 Shift Crypto AG
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

#include "btc.h"

#include <compiler_util.h>
#include <hww.pb.h>
#include <stddef.h>

/**
 * When starting a signing workflow with the BTCSignInit request, this function is used to validate
 * the script configs in the request.
 *
 * For multisig, this means confirming with the user that they are spending from the correct
 * account.
 *
 * @param[in] coin we are spending
 * @param[in] script_config the script config used for all inputs and changes.
 * @param[in] keypath_account Account-level keypath.
 * @param[in] keypath_account_count number of elements in keypath_account.
 * @return See `app_btc_result_t`.
 */
USE_RESULT app_btc_result_t app_btc_sign_validate_init_script_configs(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath_account,
    size_t keypath_account_count);
