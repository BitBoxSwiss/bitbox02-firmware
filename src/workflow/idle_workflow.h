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

#ifndef __IDLE_WORKFLOW_H
#define __IDLE_WORKFLOW_H

#include "workflow.h"

/**
 * Base "idle" workflow that runs when nothing else is being done on the device.
 * When started, it will show the logo for a while and then switch
 * to a screen saying "See the BitBox App".
 */
workflow_t* idle_workflow(void);

#endif // __IDLE_WORKFLOW_H
