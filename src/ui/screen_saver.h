// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_SCREEN_SAVER_H_
#define _UI_SCREEN_SAVER_H_

#include "component.h"

/**
 * If the screensaver is active, returns a static screen saver component.
 * If the screensaver is not active, returns NULL.
 */
component_t* screen_saver_get(void);

/**
 * Should be called from the mainloop. Handles the timing of the screensaver.
 * After a certain time, the screen saver becomes active.
 * Call `screen_saver_reset()` to reset the screensaver and the timer.
 */
void screen_saver_process(void);

/**
 * Call this to remove an active screensaver, or to reset the timer.
 */
void screen_saver_reset(void);

/**
 * disables the screensaver until enabled with `screen_saver_enable`.
 */
void screen_saver_disable(void);

/**
 * re-enables the screensaver.
 */
void screen_saver_enable(void);

#endif
