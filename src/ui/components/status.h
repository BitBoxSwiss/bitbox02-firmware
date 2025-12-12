// SPDX-License-Identifier: Apache-2.0

#ifndef _STATUS_H_
#define _STATUS_H_

#include <stdbool.h>
#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a status component with a given text. Calls the callback after delay.
 * @param[IN] text The text of the status screen.
 * @param[IN] status_success If true, indicates a success. Otherwise, false.
 */
component_t* status_create(const char* text, bool status_success);

#endif
