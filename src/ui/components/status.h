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
 * @param[IN] callback The callback that is called after <delay> time. Will be called at most once.
 */
component_t* status_create(
    const char* text,
    bool status_success,
    void (*callback)(void* user_data),
    void* user_data);

#endif
