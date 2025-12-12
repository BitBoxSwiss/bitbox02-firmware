// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_CONFIRM_TRANSACTION_H
#define _UI_CONFIRM_TRANSACTION_H

#include "ui/component.h"

/**
 * Creates a confirm screen.
 * @param[in] amount of coins to send, including the unit suffix.
 * @param[in] address to send coins
 * @param[in] callback The callback triggered when the user accepts or rejects. Is called at most
 * once.
 * @param[in] user_data Passed to `callback`.
 */
component_t* confirm_transaction_address_create(
    const char* amount,
    const char* address,
    void (*callback)(bool accepted, void* user_data),
    void* user_data);

/**
 * Creates a confirm screen.
 * @param[in] amount of coins to send, including the unit suffix.
 * @param[in] fee to send coins
 * @param[in] longtouch if the confirmation dialog should have a longtouch. Otherwise, the
 * next-arrow is shown.
 * @param[in] callback The callback triggered when the user accepts or rejects. Is called at most
 * once.
 * @param[in] user_data Passed to `callback`.
 */
component_t* confirm_transaction_fee_create(
    const char* amount,
    const char* fee,
    bool longtouch,
    void (*callback)(bool accepted, void* user_data),
    void* user_data);

#endif
