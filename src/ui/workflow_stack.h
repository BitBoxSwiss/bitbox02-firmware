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

#ifndef _UI_WORKFLOW_STACK_H_
#define _UI_WORKFLOW_STACK_H_

#include "event.h"

#include <workflow/workflow.h>

workflow_t* workflow_stack_top(void);

void workflow_stack_start_workflow(workflow_t* workflow);

/**
 * Stops the currently active workflow.
 * This is only supposed to be called by the active workflow itself.
 */
void workflow_stack_stop_workflow(void);

/**
 * Forcefully closes the given workflow and any sub-workflow it has spawned.
 */
void workflow_stack_abort_workflow(workflow_t* wf);

void workflow_stack_clear(void);

#endif
