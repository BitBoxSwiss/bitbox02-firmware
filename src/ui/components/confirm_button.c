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

#include "confirm_button.h"
#include "../event.h"
#include "../event_handler.h"
#include "confirm_gesture.h"
#include "icon_button.h"

static void _confirm(component_t* confirm_button)
{
    (void)confirm_button;
    event_t event;
    event.id = EVENT_CONFIRM;
    emit_event(&event);
}

component_t* confirm_button_create(bool longtouch)
{
    if (longtouch) {
        return confirm_gesture_create(NULL);
    }
    return icon_button_create(top_slider, ICON_BUTTON_CHECK, _confirm);
}
