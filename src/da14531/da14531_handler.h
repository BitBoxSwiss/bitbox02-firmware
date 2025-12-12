// SPDX-License-Identifier: Apache-2.0

#ifndef DA14531_HANDLER_H
#define DA14531_HANDLER_H

#include "da14531_protocol.h"
#include <platform/platform_config.h>
#include <utils_ringbuffer.h>

extern const uint8_t* da14531_handler_current_product;
extern uint16_t da14531_handler_current_product_len;

#if FACTORYSETUP == 1
bool da14531_handler_bond_db_set(void);
#endif

void da14531_handler(struct da14531_protocol_frame* frame, struct ringbuffer* queue);

#endif
