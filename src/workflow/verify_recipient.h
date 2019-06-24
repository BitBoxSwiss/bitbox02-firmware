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

#ifndef _WORKFLOW_VERIFY_RECIPIENT_H_
#define _WORKFLOW_VERIFY_RECIPIENT_H_

#include <stdbool.h>

/**
 * Shows a confirmation dialog to the user to confirm that an amount is being
 * sent to recipient. This call blocks until the user confirms or rejects.
 * @param[in] recipient recipient address/identifier/etc.
 * @param[in] amount formatted amount including unit, e.g. " 123.12345678 BTC"
 * @return true if the user confirms, false if the user rejects.
 */
bool workflow_verify_recipient(const char* recipient, const char* amount);

#endif
