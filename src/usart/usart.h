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

#ifndef __USART_H_
#define __USART_H_

#include <stdbool.h>

/**
 * Start the USART interface.
 */
void usart_start(void);

/**
 * Stop the USART interfaces.
 */
void usart_stop(void);

/**
 * This function polls the USART rx status. If there is
 * data available, it will unframe it and buffer any
 * resulting packet for later processing.
 */
bool usart_receive(void);

#endif
