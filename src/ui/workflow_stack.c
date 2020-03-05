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

#include <stddef.h>

#include "hardfault.h"
#include "workflow_stack.h"

#define UI_WORKFLOW_STACK_MAX_ELEMENTS 5

typedef struct {
    workflow_t* workflows[UI_WORKFLOW_STACK_MAX_ELEMENTS];
    int size;
} workflow_stack_t;

static workflow_stack_t _workflow_stack = {0};

workflow_t* workflow_stack_top(void)
{
    if (_workflow_stack.size > 0) {
        workflow_t* top = _workflow_stack.workflows[_workflow_stack.size - 1];
        return top;
    }
    return NULL;
}

static workflow_t* _workflow_stack_pop(void)
{
    if (_workflow_stack.size > 0) {
        workflow_t* top = _workflow_stack.workflows[_workflow_stack.size - 1];
        _workflow_stack.size--;
        return top;
    }
    Abort("Abort: tried to pop from empty workflow stack.");
}

static void _workflow_stack_push(workflow_t* workflow)
{
    if (_workflow_stack.size < UI_WORKFLOW_STACK_MAX_ELEMENTS) {
        _workflow_stack.workflows[_workflow_stack.size] = workflow;
        _workflow_stack.size++;
    } else {
        Abort("Abort: workflow_stack_push");
    }
}

void workflow_stack_start_workflow(workflow_t* workflow)
{
    _workflow_stack_push(workflow);
    workflow->init(workflow);
}

void workflow_stack_stop_workflow(void)
{
    workflow_t* workflow = _workflow_stack_pop();
    workflow_destroy(workflow);
}

void workflow_stack_clear(void)
{
    while (_workflow_stack.size > 0) {
        workflow_stack_stop_workflow();
    }
}
