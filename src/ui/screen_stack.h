// SPDX-License-Identifier: Apache-2.0

#ifndef _UI_STACK_H_
#define _UI_STACK_H_

#include "component.h"
#include "event.h"

component_t* ui_screen_stack_top(void);
void ui_screen_stack_push(component_t* component);
/* pop component and defer cleanup to `void ui_screen_stack_cleanup()` */
void ui_screen_stack_pop(void);
/* pop component and immediately perform cleanup on it */
void ui_screen_stack_pop_and_clean(void);
void ui_screen_stack_pop_all(void);
/* clean up all popped components. */
void ui_screen_stack_cleanup(void);

#endif
