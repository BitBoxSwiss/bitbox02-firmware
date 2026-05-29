// SPDX-License-Identifier: Apache-2.0

#ifndef _PROGRESS_H_
#define _PROGRESS_H_

#include <ui/component.h>

/**
 * Creates an progress bar component.
 */
component_t* progress_create(const char* title);

/**
 * Set the progress as an exact fraction.
 * @param[in] denominator must be non-zero.
 */
void progress_set_fraction(component_t* component, uint32_t numerator, uint32_t denominator);

#endif
