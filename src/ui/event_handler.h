// SPDX-License-Identifier: Apache-2.0

#ifndef _EVENT_HANDLER_H_
#define _EVENT_HANDLER_H_

#include "event.h"

/**
 * Emits an event by passing it to the component at the top of the stack
 * and its subcomponents.
 */
void emit_event(const event_t* event);

#endif
