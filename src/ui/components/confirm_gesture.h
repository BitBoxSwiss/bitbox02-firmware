// SPDX-License-Identifier: Apache-2.0

#ifndef _CONFIRM_GESTURE_
#define _CONFIRM_GESTURE_

#include <stdbool.h>
#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a confirm_gesture component on the top slider.
 * @param[in] parent The parent component.
 */
component_t* confirm_gesture_create(void (*callback)(void* user_data), void* user_data);

bool confirm_gesture_is_active(component_t* component);

#endif
