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

#ifndef _WORKFLOW_BLOCKING_H_
#define _WORKFLOW_BLOCKING_H_

#include <compiler_util.h>

#include <stdbool.h>
#include <stdint.h>

/**
 * Start a blocking workflow. Call workflow_blocking_unblock() to unblock this call.
 * This function aborts if there is already a blocking workflow running.
 */
void workflow_blocking_block(void);

/**
 * Unblocks the workflow. Use this to terminate the worklow after the workflow finishes normally by
 * user interaction.
 */
void workflow_blocking_unblock(void);

#endif
