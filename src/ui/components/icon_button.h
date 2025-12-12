// SPDX-License-Identifier: Apache-2.0

#ifndef _ICON_BUTTON_H_
#define _ICON_BUTTON_H_

#include <screen.h>
#include <ui/component.h>

typedef enum {
    ICON_BUTTON_CHECK,
    ICON_BUTTON_CROSS,
    ICON_BUTTON_NEXT,
} icon_button_type_t;

/**
 * Creates an icon_button component on the right side of the screen.
 * @param[in] type which icon to display
 * @param[in] location whether to render the component on top or bottom (UPPER/LOWER slider)
 */
component_t* icon_button_create(
    slider_location_t location,
    icon_button_type_t type,
    void (*callback)(void* user_data),
    void* user_data);

#endif
