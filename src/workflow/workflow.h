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

#ifndef _WORKFLOW_H_
#define _WORKFLOW_H_

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include <compiler_util.h>

/**
 * Typedef for a callback to be executed when a workflow is finishing.
 * This can be used by the parent workflow to obtain information about
 * the executed workflow's result.
 */
typedef void (*workflow_cb_t)(void*);

/**
 * Descriptor for a workflow to be run.
 */
typedef struct _workflow_t {
    /* Starts the workflow */
    void (*init)(struct _workflow_t*);
    /* Stops the workflow and destroys the related resources. */
    void (*cleanup)(struct _workflow_t*);
    /* Private data (depends on the workflow type) */
    void* data;
    /* Size of the allocated data. */
    size_t data_size;
    /* Function to get run at every cycle */
    void (*spin)(struct _workflow_t*);
} workflow_t;

#endif
