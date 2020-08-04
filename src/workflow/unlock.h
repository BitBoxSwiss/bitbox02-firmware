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

#ifndef _WORKFLOW_UNLOCK_H_
#define _WORKFLOW_UNLOCK_H_

#include <compiler_util.h>
#include <keystore.h>

/**
 * Tries to unlock the key store, showing appropriate error messages.
 * If the error is KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED (device reset), we return to workflow_start.
 */
USE_RESULT keystore_error_t workflow_unlock_and_handle_error(const char* password);

#endif
