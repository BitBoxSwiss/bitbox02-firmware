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

#ifndef _QUEUE_H
#define _QUEUE_H

#include <stdint.h>
#include <string.h>

#define ERR_NONE 0
#define ERR_QUEUE_FULL 7

/**
 * Append the given data to the queue.
 * Returns ERR_NONE if the data was added and ERR_QUEUE_FULL if the buffer was full.
 */
uint8_t queue_push(const uint8_t* data);

/**
 * Return the first data that was added to the queue.
 */
uint8_t* queue_pull(void);

/**
 * Clear the queue.
 */
void queue_clear(void);

#endif
