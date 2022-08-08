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

#ifndef _APPS_BTC_SIGN_H
#define _APPS_BTC_SIGN_H

#include "btc.h"

#include <stdbool.h>
#include <stdint.h>

#include <compiler_util.h>
#include <util.h>

#include <hww.pb.h>

USE_RESULT app_btc_result_t app_btc_sign_init(const BTCSignInitRequest* request);

USE_RESULT app_btc_result_t app_btc_sign_sighash_script(
    const BTCSignInputRequest* request,
    // at least MAX_PK_SCRIPT_SIZE + MAX_VARINT_SIZE bytes
    uint8_t* sighash_script,
    // in: size of the sighash_script buffer. out: resulting size of sighash_script.
    size_t* sighash_script_size);

USE_RESULT app_btc_result_t app_btc_sign_payload_at_keypath(
    const uint32_t* keypath,
    size_t keypath_len,
    const BTCScriptConfigWithKeypath* script_config_account,
    uint8_t* payload_bytes,
    size_t* payload_size);

USE_RESULT app_btc_result_t app_btc_sign_init_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_payload_at_keypath_wrapper(
    in_buffer_t requet_buf,
    const uint32_t* keypath,
    size_t keypath_len,
    uint8_t* payload_bytes,
    size_t* payload_size);
USE_RESULT app_btc_result_t app_btc_sign_sighash_script_wrapper(
    in_buffer_t request_buf,
    // at least MAX_PK_SCRIPT_SIZE + MAX_VARINT_SIZE bytes
    uint8_t* sighash_script,
    // in: size of the sighash_script buffer. out: resulting size of sighash_script.
    size_t* sighash_script_size);

void app_btc_sign_reset(void);

#endif
