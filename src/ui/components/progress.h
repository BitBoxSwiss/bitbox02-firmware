// SPDX-License-Identifier: Apache-2.0

#ifndef _PROGRESS_H_
#define _PROGRESS_H_

#include <ui/component.h>

/**
 * Creates an progress bar component.
 */
component_t* progress_create(const char* title);

/**
 * Set the progress.
 * @param[in] progress value must be in [0, 1].
 */
void progress_set(component_t* component, float progress);

#endif
