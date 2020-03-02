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

#ifndef _WORKFLOW_ASYNC_H
#define _WORKFLOW_ASYNC_H
#include <stdbool.h>
#include <stdint.h>

/**
 * Returntype for functions that are async
 */
enum workflow_async_ready {
    WORKFLOW_ASYNC_READY,
    WORKFLOW_ASYNC_NOT_READY,
};

#endif
