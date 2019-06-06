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

#ifndef _RANDOM_H_
#define _RANDOM_H_

#include <stdint.h>
#include <stdlib.h>

#define RANDOM_NUM_SIZE ((uint8_t)32)

// random_32_bytes_mcu generates 32 random bytes using the mcu trng and xors it into buf.
void random_32_bytes_mcu(uint8_t* buf);
// random_32_bytes generates 32 random bytes (a combination of mcu trng and secure chip trng).
void random_32_bytes(uint8_t* buf);

/**
 * Return single random byte.
 */
uint8_t random_byte_mcu(void);

#endif
