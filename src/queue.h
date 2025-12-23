// SPDX-License-Identifier: Apache-2.0

#ifndef _QUEUE_H
#define _QUEUE_H

#include <stdint.h>
#include <string.h>

typedef enum {
    QUEUE_ERR_NONE = 0,
    QUEUE_ERR_FULL = 1,
} queue_error_t;

struct queue;

/**
 * Append the given data to the queue.
 * Returns QUEUE_ERR_NONE if the data was added and QUEUE_ERR_FULL if the buffer was full.
 * data must be USB_REPORT_SIZE large
 */
queue_error_t queue_push(struct queue* ctx, const uint8_t* data);

/**
 * Return the first data that was added to the queue.
 * Returns NULL if empty
 */
const uint8_t* queue_pull(struct queue* ctx);

/**
 * Initializes this queue object.
 * The queue will handle elements with the given size.
 *
 * @param[size] Size of each element. This will decide how many
 * bytes each push/pull operation will consume. This must be
 * a submultiple of QUEUE_SIZE.
 */
void queue_init(struct queue* ctx, size_t item_size);

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
