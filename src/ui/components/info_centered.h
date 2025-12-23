// SPDX-License-Identifier: Apache-2.0

#ifndef _INFO_CENTERED_H_
#define _INFO_CENTERED_H_

#include <ui/component.h>

/**
 * Creates an info centered screen with a given text and an optional skip callback.
 * @param[IN] text The text that is displayed in the middle of the screen.
 * @param[IN] skip_callback The optional callback. If specified, a "Skip"
 * button is shown on the bottom and the callback is called when the button is
 * pressed.
 */
component_t* info_centered_create(const char* text, void (*skip_callback)(component_t*));

#endif
