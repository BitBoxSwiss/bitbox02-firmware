// SPDX-License-Identifier: Apache-2.0

#ifndef _ORIENTATION_ARROWS_H_
#define _ORIENTATION_ARROWS_H_

#include "ui/component.h"

#include <stdbool.h>

/**
 * Creates an orientation screen and registers a done callback.
 * @param[in] done_callback The callback that is called when the orientation has been selected.
 * @param[in] cb_param The user-defined parameter that will be passed into the callback when it's
 * invoked.
 */
component_t* orientation_arrows_create(void (*done_callback)(bool, void*), void* cb_param);

#endif
