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

#ifndef _SHOW_LOGO_H_
#define _SHOW_LOGO_H_

#include <ui/component.h>

/**
 * Creates an show_logo screen and registers a done callback.
 * @param[in] done_callback The callback that is called when the show_logo has been selected.
 * @param[in] timeout Time to display the logo in units of screen refresh counts
 */
component_t* show_logo_create(void);

#endif
