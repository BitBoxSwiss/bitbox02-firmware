// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
