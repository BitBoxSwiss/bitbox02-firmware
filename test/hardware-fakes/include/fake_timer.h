// SPDX-License-Identifier: Apache-2.0

#ifndef FAKE_TIMER_H
#define FAKE_TIMER_H

#include <hal_timer.h>

extern struct timer_descriptor TIMER_0;
void fake_timer_fire(void);

#endif
