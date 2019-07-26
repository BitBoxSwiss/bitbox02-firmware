// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
