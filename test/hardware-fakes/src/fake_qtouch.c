// SPDX-License-Identifier: Apache-2.0

// The real qtouch.h depends on hardware specific headers and cannot be used
#include <fake_qtouch.h>

#include <stdbool.h>

volatile bool measurement_done_touch = true;

void qtouch_process(void) {}

void qtouch_force_calibrate(void) {}
