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
 * Create a centered label with a checkmark for success or a cross for failure.
 * @param msg Message to print
 * @param status_success true/false if screen should indicate success / failure
 */

void workflow_status_create(const char* msg, bool status_success);

#endif
