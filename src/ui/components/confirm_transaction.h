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

#ifndef _UI_CONFIRM_TRANSACTION_H
#define _UI_CONFIRM_TRANSACTION_H

#include "ui/component.h"

/**
 * Creates a confirm screen.
 * @param[in] amount of coins to send, including the unit suffix.
 * @param[in] address to send coins
 * @param[in] callback The callback triggered when the user accepts or rejects. Is called at most
 * once.
 * @param[in] callback_param Passed to `callback`.
 */
component_t* confirm_transaction_address_create(
    const char* amount,
    const char* address,
    void (*callback)(bool accepted, void* param),
    void* callback_param);

/**
 * Creates a confirm screen.
 * @param[in] amount of coins to send, including the unit suffix.
 * @param[in] fee to send coins
 * @param[in] callback The callback triggered when the user accepts or rejects. Is called at most
 * once.
 * @param[in] callback_param Passed to `callback`.
 */
component_t* confirm_transaction_fee_create(
    const char* amount,
    const char* fee,
    void (*callback)(bool accepted, void* param),
    void* callback_param);

#endif
