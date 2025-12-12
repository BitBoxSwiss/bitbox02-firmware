// SPDX-License-Identifier: Apache-2.0

#ifndef _HID_U2F_H_
#define _HID_U2F_H_

#include "hid.h"

/**
 * Initializes a HWW HID interface.
 * @param[in] callback The callback that is called upon status update (enabling/disabling or the
 * endpoints).
 */
int32_t hid_u2f_init(void (*callback)(void));

/**
 * Deinitializes the HWW HID interface.
 */
int32_t hid_u2f_deinit(void);

/**
 * Checks whether the HWW HID interface was enabled.
 */
bool hid_u2f_is_enabled(void);

/**
 * Returns the version of the HWW interface.
 */
uint32_t hid_u2f_get_version(void);

/**
 * Returns the endpoint for the given direction.
 * @param[in] dir The direction of the endpoint:
 *            dir == 1: outgoing (host -> BitBox)
 *            dir == 0: incoming (BitBox -> host)
 */
uint8_t hid_u2f_get_ep(uint8_t dir);

/**
 * Registers a callback for a given transfer type.
 * @param[in] trans_type The transfer type for which the callback should be registered,
 *            which can be READ, WRITE or SET_REPORT.
 * @param[in] func The function that is registered as a callback.
 */
int32_t hid_u2f_register_callback(enum hid_trans_type trans_type, FUNC_PTR func);

/**
 * Registers the HID U2F read and write callbacks and start listening for data.
 */
void hid_u2f_setup(void);

bool hid_u2f_write_poll(const uint8_t* data);
bool hid_u2f_read(uint8_t* data);

#endif
