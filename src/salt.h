// SPDX-License-Identifier: Apache-2.0

#ifndef _SALT_H_
#define _SALT_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * Creates sha256(<salt root><purpose><data>), where <salt root> is a persisted salt (static until
 * device reset).
 * @param[in] data data to salt and hash.
 * @param[in] purpose a string which is part of the hash, to put a namespace on the use.
 * @param[out] hash_out must be 32 bytes.
 * @return false if the salt root could not be retrieved.
 */
bool salt_hash_data(const uint8_t* data, size_t data_len, const char* purpose, uint8_t* hash_out);

#endif
