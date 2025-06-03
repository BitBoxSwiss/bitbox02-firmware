// Copyright 2025 Shift Crypto AG
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

#ifndef DA14531_HANDLER_H
#define DA14531_HANDLER_H

#include "da14531_protocol.h"
#include <utils_ringbuffer.h>

extern volatile const uint8_t* da14531_handler_current_product;
extern volatile uint16_t da14531_handler_current_product_len;

void da14531_handler(struct da14531_protocol_frame* frame, struct ringbuffer* queue);

#endif
