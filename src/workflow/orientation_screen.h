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

#ifndef __ORIENTATION_SCREEN_H
#define __ORIENTATION_SCREEN_H

#include "workflow.h"

/**
 * Workflow to select the screen orientation and start
 * the main bitbox activity afterwards.
 */
workflow_t* orientation_screen(void);

#endif // __ORIENTATION_SCREEN_H
