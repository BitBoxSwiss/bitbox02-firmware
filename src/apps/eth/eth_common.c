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

#include "eth_common.h"

#include <hardfault.h>

#include <stdio.h>
#include <string.h>

void eth_common_format_amount(
    const bignum256* scalar,
    const char* unit,
    unsigned int decimals,
    char* out,
    size_t out_len)
{
    // Since scalar is at most 32 bytes, 100 chars is plenty of space for the output.
    const size_t min_out_len = 100;
    // Truncate the number at this many chars and append '...' if truncated.
    // Empirically found to fit on one line on the screen (including unit).
    const size_t truncate_len = 10;

    if (out == NULL || out_len < min_out_len || strlen(unit) > 10) {
        Abort("eth_common_format_amount");
    }
    char unit_with_space[strlen(unit) + 2];
    snprintf(unit_with_space, sizeof(unit_with_space), " %s", unit);
    bn_format(scalar, "", "", decimals, 0, false, out, out_len);
    if (strlen(out) > truncate_len) {
        snprintf(&out[truncate_len], out_len - truncate_len, "..%s", unit_with_space);
    } else {
        snprintf(&out[strlen(out)], out_len - strlen(out), "%s", unit_with_space);
    }
}
