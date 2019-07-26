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

#ifndef _APPS_ETH_VERIFY_H
#define _APPS_ETH_VERIFY_H

#include "eth_params.h"

#include <compiler_util.h>

#include <generated/hww.pb.h>

typedef enum {
    APP_ETH_SIGN_OK,
    APP_ETH_SIGN_ERR_UNKNOWN,
    APP_ETH_SIGN_ERR_INVALID_INPUT,
    APP_ETH_SIGN_ERR_USER_ABORT,
} app_eth_sign_error_t;

USE_RESULT app_eth_sign_error_t app_eth_verify_standard_transaction(
    const ETHSignRequest* request,
    const app_eth_coin_params_t* params);

USE_RESULT app_eth_sign_error_t app_eth_verify_erc20_transaction(
    const ETHSignRequest* request,
    const app_eth_coin_params_t* params);

#endif
