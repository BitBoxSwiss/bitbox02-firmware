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

#include "pairing.h"

#include <hardfault.h>
#include <ui/fonts/monogram_5X9.h>
#include <workflow/confirm.h>

#include <base32.h>

#include <stdio.h>

bool workflow_pairing_create(const uint8_t* hash)
{
    char base32[60] = {0};
    int count = base32_encode(hash, 32, (uint8_t*)base32, sizeof(base32));
    if (count < 20) {
        Abort("unexpected base32 size");
    }
    char base32_formatted[100];
    snprintf(
        base32_formatted,
        sizeof(base32_formatted),
        "%.5s %.5s\n%.5s %.5s",
        base32,
        base32 + 5,
        base32 + 10,
        base32 + 15);

    const confirm_params_t params = {
        .title = "Pairing code",
        .body = base32_formatted,
        .font = &font_monogram_5X9,
    };
    return workflow_confirm_blocking(&params);
}
