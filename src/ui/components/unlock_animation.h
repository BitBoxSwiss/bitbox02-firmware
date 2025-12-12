// SPDX-License-Identifier: Apache-2.0

#ifndef _UNLOCK_ANIMATION_H_
#define _UNLOCK_ANIMATION_H_

#include <ui/component.h>

component_t* unlock_animation_create(void (*on_done)(void*), void* on_done_param);

#endif
