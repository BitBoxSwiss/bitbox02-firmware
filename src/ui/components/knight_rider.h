// SPDX-License-Identifier: Apache-2.0

#ifndef _KNIGHT_RIDER_H_
#define _KNIGHT_RIDER_H_

#include <screen.h>
#include <ui/component.h>

#include <stdint.h>

/********************************** Create Instance **********************************/

/**
 * Creates a Knight-Rider style moving bar component located at height vertical position.
 */
component_t* knight_rider_create(component_t* parent, uint8_t height);

#endif
