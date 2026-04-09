// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_CONFIRM_SWAP_H
#define _UI_CONFIRM_SWAP_H

#include "ui/component.h"

/**
 * Creates a swap confirm screen.
 * @param[in] title centered title shown in the title bar.
 * @param[in] from source amount/value shown above the arrow.
 * @param[in] to destination amount/value shown below the arrow.
 * @param[in] callback The callback triggered when the user accepts or rejects. Is called at most
 * once.
 * @param[in] user_data Passed to `callback`.
 */
component_t* confirm_swap_create(
    const char* title,
    const char* from,
    const char* to,
    void (*callback)(bool accepted, void* user_data),
    void* user_data);

#endif
