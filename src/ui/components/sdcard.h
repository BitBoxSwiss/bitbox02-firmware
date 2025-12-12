// SPDX-License-Identifier: Apache-2.0

#ifndef _SD_CARD_H_
#define _SD_CARD_H_

#include <ui/component.h>

/**
 * Creates an insert/remove SD card screen.
 * @param[in] insert if true, the user is asked to insert the sdcard. Otherwise the user is asked to
 *            remove it.
 */
component_t* sdcard_create(void (*callback)(bool inserted, void* user_data), void* user_data);

#endif
