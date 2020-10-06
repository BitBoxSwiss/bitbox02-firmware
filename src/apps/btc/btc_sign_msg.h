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

#ifndef _APPS_BTC_SIGN_MSG_H
#define _APPS_BTC_SIGN_MSG_H

#include "btc.h"

#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

#include <hww.pb.h>

USE_RESULT app_btc_result_t app_btc_sign_msg(
    BTCCoin coin,
    const BTCScriptConfig* script_config,
    const uint32_t* keypath,
    size_t keypath_len,
    const uint8_t* msg,
    size_t msg_size,
    uint8_t* signature_out);

#endif
