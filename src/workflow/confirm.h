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

#ifndef _WORKFLOW_CONFIRM_H_
#define _WORKFLOW_CONFIRM_H_

#include <stdbool.h>
#include <ui/components/confirm.h>

/**
 * Confirm something with the user.
 * Block until the user has either confirmed or rejected.
 *
 * @param[in] params see confirm_params_t for details.
 * @return true if the user accepted, false if the user rejected.
 */
bool workflow_confirm_blocking(const confirm_params_t* params);

#endif
