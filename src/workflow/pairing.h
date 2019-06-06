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

#ifndef _WORKFLOW_PAIRING_H_
#define _WORKFLOW_PAIRING_H_

#include <stdbool.h>
#include <stdint.h>

/**
 * Starts the pairing workflow.
 * @param[in] hash Pairing hash to display and compare with the connected app. 32 bytes.
 * @return true if the pairing was confirmed, false if it was rejected.
 */
bool workflow_pairing_create(const uint8_t* hash);

#endif
