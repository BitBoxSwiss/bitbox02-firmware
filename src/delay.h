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

#ifndef DELAY_H
#define DELAY_H
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
    size_t id;
} delay_t;

typedef void (*delay_callback_t)(void*);

// Creates a non-blocking delay. Check with delay_is_elapsed if it has elapsed.
// Limited to 10 concurrent delays, will abort if it fails to allocate one
void delay_init_ms(delay_t* self, uint32_t ms, delay_callback_t cb, void* user_data);

// returns true if time has passed. After it has returned true once it must not be called again
bool delay_is_elapsed(const delay_t* self);

// Cancel delay if you don't intend to check it until it elapses
void delay_cancel(const delay_t* self);
#endif
