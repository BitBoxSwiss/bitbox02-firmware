// SPDX-License-Identifier: Apache-2.0

#ifndef _RIGHT_ARROW_H_
#define _RIGHT_ARROW_H_

#include <screen.h>
#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a right arrow component.
 * param[in] location whether the arrow should be rendered on top or bottom (UPPER/LOWER slider)
 */
component_t* right_arrow_create(
    slider_location_t location,
    component_t* parent,
    void (*callback)(void*),
    void* user_data);

#endif
