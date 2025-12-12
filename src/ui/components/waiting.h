// SPDX-License-Identifier: Apache-2.0

#ifndef _WAITING_H_
#define _WAITING_H_

#include <ui/component.h>

/**
 * Creates a waiting screen. It starts out with the logo. Use `waiting_switch_to_lockscreen()` and
 * `waiting_switch_to_logo()` to change the display between logo and lockscreen.
 */
component_t* waiting_create(void);

/**
 * Switch the waiting screen to show the BitBox logo instead.
 */
void waiting_switch_to_logo(component_t* component);

/**
 * Switch the waiting screen to show the lockscreen instead (see lockscreen.c).
 */
void waiting_switch_to_lockscreen(component_t* component);

#endif
