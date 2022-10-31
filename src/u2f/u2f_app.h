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

#ifndef _U2F_APP_H_
#define _U2F_APP_H_

#include <compiler_util.h>

#include <stdbool.h>
#include <stdint.h>

#include <util.h>

/** Size of application IDs */
#define U2F_APPID_SIZE 32

enum u2f_app_confirm_t {
    U2F_APP_NONE,
    U2F_APP_REGISTER,
    U2F_APP_AUTHENTICATE,
};

/**
 * User confirm auth/registration for a website given by the U2F app ID.
 *
 * FUTURE: transform this into a workflow.
 *
 * @param[in] type show registration or authentication screen.
 * @param[in] app_id U2F app ID to identify the website.
 * @param[out] result true if the user accepts, false for rejection.
 * @return Ready if result is ready, NotReady otherwise
 */
void u2f_app_confirm_start(enum u2f_app_confirm_t type, const uint8_t* app_id);

/**
 * Polls an outstanding confirmation for completion.
 *
 * @param type  Type of confirmation. This is for sanity checks: it must match
 *              the actual type of confirmation that is outstanding.
 * @param app_id U2F AppId. This is for sanity checks: it must match
 *              the actual AppId that is being confirmed.
 */
async_op_result_t u2f_app_confirm_retry(enum u2f_app_confirm_t type, const uint8_t* app_id);

/**
 * Clears any outstanding confirmation. Can only be called
 * if there is a confirmation outstanding. Returns back immediately
 * after aborting all open workflows related to this U2F confirm.
 */
void u2f_app_confirm_abort(void);

#endif
