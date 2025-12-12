// SPDX-License-Identifier: Apache-2.0

#ifndef _SCREENSAVER_H_
#define _SCREENSAVER_H_

#include <ui/component.h>

component_t* screensaver_create(void);

/**
 * Resets the animation so that the logo starts to scroll in from left, out of screen.
 * The vertical position is unchanged, it starts where it left off.
 */
void screensaver_reset(component_t* component);

#endif
