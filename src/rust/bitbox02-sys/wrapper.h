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

#include <bitboxbase.pb.h>
#include <bitboxbase/bitboxbase_screensaver.h>
#include <bitboxbase/bitboxbase_watchdog.h>
#include <commander/commander.h>
#include <platform/bitboxbase/leds.h>
#include <screen.h>
#include <ui/components/label.h>
#include <ui/fonts/font_a_9X9.h>
#include <ui/fonts/monogram_5X9.h>
#include <ui/oled/oled.h>
#include <ui/screen_stack.h>
#include <ui/ugui/ugui.h>
#include <wally_crypto.h>
#include <workflow/confirm.h>

#if !defined(TESTING)
#include <hal_delay.h>
#else
void delay_us(const uint16_t us);
void delay_ms(const uint16_t ms);
#endif
