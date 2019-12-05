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

#ifndef _WORKFLOW_CONFIRM_H_
#define _WORKFLOW_CONFIRM_H_

#include "async.h"
#include <stdbool.h>
#include <stdint.h>
#include <ui/components/confirm.h>
#include <ui/ugui/ugui.h>

#include "workflow.h"

/**
 * Confirm something with the user.
 * @param[in] params see confirm_params_t for details.
 * @return true if the user accepted, false if the user rejected.
 */
workflow_t* workflow_confirm(
    const confirm_params_t* params,
    void (*callback)(bool, void*),
    void* callback_param);

/**
 * Confirm something with the user asynchronously.
 *
 * Call this function with the same parameters until it returns WORKFLOW_ASYNC_READY. Then and only
 * then the `result` argument is set to a valid value.
 *
 * @param[in] title shown at the top
 * @param[in] body shown in the center
 * @param[in] font if not NULL will use the specified font for the body
 * @param[in] accept_only if true, the user can only confirm, not reject.
 * @param[in] longtouch if true, require the hold gesture to confirm instead of tap.
 * @param[out] result true if the user accepted, false if the user rejected.
 * @return WORKFLOW_ASYNC_NOT_READY if user has not repsonded yet, WORKFLOW_ASYNC_READY if user has
 * responded.
 */
enum workflow_async_ready workflow_confirm_async(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool accept_only,
    bool* result);

/**
 * Confirm something with the user.
 * Block until the user has either confirmed or rejected.
 *
 * @param[in] params see confirm_params_t for details.
 * @return true if the user accepted, false if the user rejected.
 */
bool workflow_confirm_blocking(const confirm_params_t* params);

/**
 * Confirm something with the user using longtouch. Blocks until the user either confirms or
 * cancels.
 * @param[in] title title
 * @param[in] body body
 * @param[in] font if not NULL will use the specified font for the body
 * @param[out] cancel_forced_out if the function returns false, this param is true if the workflow
 * was forcibly unblocked.
 * @return true if the user confirmed, false if they rejected.
 */
bool workflow_confirm_scrollable_longtouch_blocking(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool* cancel_forced_out);
#endif
