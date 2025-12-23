// SPDX-License-Identifier: Apache-2.0

#ifndef _HID_HWW_H_
#define _HID_HWW_H_

#include "hid.h"

/**
 * Initializes a HWW HID interface.
 * @param[in] callback The callback that is called upon status update (enabling/disabling or the
 * endpoints).
 */
int32_t hid_hww_init(void (*callback)(void));

/**
 * Deinitializes the HWW HID interface.
 */
int32_t hid_hww_deinit(void);

/**
 * Checks whether the HWW HID interface was enabled.
 */
bool hid_hww_is_enabled(void);

/**
 * Returns the version of the HWW interface.
 */
uint32_t hid_hww_get_version(void);

/**
 * Returns the endpoint for the given direction.
 * @param[in] dir The direction of the endpoint:
 *            dir == 1: outgoing (host -> BitBox)
 *            dir == 0: incoming (BitBox -> host)
 */
uint8_t hid_hww_get_ep(uint8_t dir);

/**
 * Registers a callback for a given transfer type.
 * @param[in] trans_type The transfer type for which the callback should be registered,
 *            which can be READ, WRITE or SET_REPORT.
 * @param[in] func The function that is registered as a callback.
 */
int32_t hid_hww_register_callback(enum hid_trans_type trans_type, FUNC_PTR func);

/**
 * Registers the HID HWW read and write callbacks and start listening for data.
 */
void hid_hww_setup(void);

/**
 * Send out data
 * returns true if data sent out, false if busy (need retry)
 */
bool hid_hww_write_poll(const uint8_t* data);

/**
 * Read data
 *
 * data must fit 64 bytes data.
 * Returns true if there is valid data in the buffer. data is invalidated when this is called again.
 * Returns false if USB subsystem was not ready to receive or there is a request in flight and data
 * is not.
 */
bool hid_hww_read(uint8_t* data);

#endif
