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

#ifndef _WORKFLOW_STATUS_H_
#define _WORKFLOW_STATUS_H_

#include <stdbool.h>

/**
 * Blocking status. This will start a new workflow_status
 * and wait until it completes.
 *
 * @param msg See workflow_status.
 * @param status_success See workflow_status.
 */
void workflow_status_blocking(const char* msg, bool status_success);

#endif
