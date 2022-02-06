// Copyright 2022 Shift Crypto AG
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

#ifndef _APPS_BTC_UI_H
#define _APPS_BTC_UI_H

#include <stdbool.h>

#include <workflow/confirm.h>

typedef struct {
    bool (*verify_recipient)(const char* recipient, const char* amount);
    bool (*verify_total)(const char* total, const char* fee);
    bool (*confirm)(const confirm_params_t* params);
} app_btc_ui_t;

app_btc_ui_t* app_btc_ui(void);

#ifdef TESTING
void testing_app_btc_mock_ui(app_btc_ui_t mock);
#endif

#endif
