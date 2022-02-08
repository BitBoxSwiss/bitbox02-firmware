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
#include <workflow/confirm.h>

#include <hww.pb.h>

USE_RESULT app_btc_result_t app_btc_sign_init(const BTCSignInitRequest* request);

USE_RESULT app_btc_result_t app_btc_sign_input_pass2(
    const BTCSignInputRequest* request,
    // 32 bytes
    const uint8_t* hash_prevouts,
    // 32 bytes
    const uint8_t* hash_sequence,
    // 64 bytes
    uint8_t* sig_out,
    // 33 bytes
    uint8_t* anti_klepto_signer_commitment_out);

USE_RESULT app_btc_result_t app_btc_sign_payload_at_change(
    const BTCSignOutputRequest* request,
    uint8_t* payload_bytes,
    size_t* payload_size);

USE_RESULT app_btc_result_t app_btc_sign_output(
    const BTCSignOutputRequest* request,
    bool last,
    BTCOutputType output_type,
    const uint8_t* payload_bytes,
    size_t payload_size);

USE_RESULT app_btc_result_t app_btc_sign_antiklepto(
    const AntiKleptoSignatureRequest* request,
    // 64 bytes
    uint8_t* sig_out);

USE_RESULT app_btc_result_t app_btc_sign_init_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_payload_at_change_wrapper(
    in_buffer_t requet_buf,
    uint8_t* payload_bytes,
    size_t* payload_size);
USE_RESULT app_btc_result_t app_btc_sign_output_wrapper(
    in_buffer_t request_buf,
    bool last,
    BTCOutputType output_type,
    const uint8_t* payload_bytes,
    size_t payload_size);
USE_RESULT app_btc_result_t app_btc_sign_input_pass2_wrapper(
    in_buffer_t request_buf,
    // 32 bytes
    const uint8_t* hash_prevouts,
    // 32 bytes
    const uint8_t* hash_sequence,
    // 64 bytes
    uint8_t* sig_out,
    // 33 bytes
    uint8_t* anti_klepto_signer_commitment_out);
USE_RESULT app_btc_result_t app_btc_sign_antiklepto_wrapper(
    in_buffer_t request_buf,
    // 64 bytes
    uint8_t* sig_out);

void app_btc_sign_reset(void);

#endif
