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

USE_RESULT app_btc_result_t
app_btc_sign_init(const BTCSignInitRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_prevtx_init(const BTCPrevTxInitRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_prevtx_input(const BTCPrevTxInputRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_prevtx_output(const BTCPrevTxOutputRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_input(const BTCSignInputRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_output(const BTCSignOutputRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t
app_btc_sign_antiklepto(const AntiKleptoSignatureRequest* request, BTCSignNextResponse* next_out);

USE_RESULT app_btc_result_t app_btc_sign_init_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_input_pass1_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_prevtx_init_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_prevtx_input_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_prevtx_output_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_output_wrapper(in_buffer_t request_buf);
USE_RESULT app_btc_result_t app_btc_sign_input_pass2_wrapper(
    in_buffer_t request_buf,
    // 64 bytes
    uint8_t* sig_out,
    // 33 bytes
    uint8_t* anti_klepto_signer_commitment_out);
USE_RESULT app_btc_result_t app_btc_sign_antiklepto_wrapper(
    in_buffer_t request_buf,
    // 64 bytes
    uint8_t* sig_out);

typedef struct {
    bool (*verify_recipient)(const char* recipient, const char* amount);
    bool (*verify_total)(const char* total, const char* fee);
    void (*status)(const char* msg, bool status_success);
    bool (*confirm)(const confirm_params_t* params);
} app_btc_ui_t;

#ifdef TESTING
void testing_app_btc_mock_ui(app_btc_ui_t mock);
#endif

#endif
