// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_SCREEN_PROCESS_H_
#define _UI_SCREEN_PROCESS_H_

#include "component.h"
#include <stdbool.h>

void ui_screen_render_component(component_t* component);

/*
 * Select which activity we should process next
 * This returns the default screen if there nothing else to process.
 * If the screensaver is active, this returns the screensaver.
 */
component_t* screen_process_get_top_component(void);

/**
 * Wraps `waiting_switch_to_logo()` for the waiting screen.
 */
void screen_process_waiting_switch_to_logo(void);

/**
 * Wraps `waiting_switch_to_lockscreen()` for the waiting screen.
 */
void screen_process_waiting_switch_to_lockscreen(void);

/**
 * Runs the UI once.
 *
 * This function will update the screen (if needed)
 * and process gesture-related events.
 */
void screen_process(void);

/**
 * The screen is refreshed every SCREEN_FRAME_RATE event loops cycles.
 *
 * In the simulator screen_process is called with a fixed frame rate so there a rate dividor isn't
 * necessary.
 */
#if defined(TESTING)
    #define SCREEN_FRAME_RATE 1
#else
    #define SCREEN_FRAME_RATE 30
#endif

#endif
