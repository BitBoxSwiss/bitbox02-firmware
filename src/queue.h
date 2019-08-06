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

#define QUEUE_ERR_NONE 0
#define QUEUE_ERR_FULL -1

struct queue;

/**
 * Append the given data to the queue.
 * Returns QUEUE_ERR_NONE if the data was added and QUEUE_ERR_FULL if the buffer was full.
 * data must be USB_REPORT_SIZE large
 */
int32_t queue_push(struct queue* ctx, const uint8_t* data);

/**
 * Return the first data that was added to the queue.
 * Returns NULL if empty
 */
const uint8_t* queue_pull(struct queue* ctx);

/**
 * Clear the queue.
 */
void queue_clear(struct queue* ctx);

/**
 * Peek at the tip of the queue. Returns NULL if queue is empty.
 */
const uint8_t* queue_peek(struct queue* ctx);

/**
 * Get a pointer to the hww queue
 */
struct queue* queue_hww_queue(void);

/**
 * Get a pointer ot the u2f queue
 */
struct queue* queue_u2f_queue(void);

#endif
