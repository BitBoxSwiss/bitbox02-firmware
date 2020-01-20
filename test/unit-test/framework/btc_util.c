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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <cmocka.h>

#include <btc_util.h>
#include <string.h>
#include <wally_bip32.h>

XPub btc_util_parse_xpub(const char* base58)
{
    struct ext_key xpub = {0};
    assert_int_equal(bip32_key_from_base58(base58, &xpub), WALLY_OK);

    XPub xpub_out = {0};
    xpub_out.depth[0] = xpub.depth;
    memcpy(xpub_out.parent_fingerprint, xpub.parent160, 20);
    xpub_out.child_num = xpub.child_num;
    memcpy(xpub_out.chain_code, xpub.chain_code, 32);
    memcpy(xpub_out.public_key, xpub.pub_key, 33);
    return xpub_out;
}
