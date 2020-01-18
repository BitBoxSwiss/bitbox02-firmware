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

#ifndef _APPS_COMMON_BIP32_H
#define _APPS_COMMON_BIP32_H

#include <stdbool.h>

#include <common.pb.h>
#include <compiler_util.h>

#include <wally_bip32.h>

/**
 * Converts between an xpub provided via a protobuf message to a libwally xpub.
 */
USE_RESULT bool apps_common_bip32_xpub_from_protobuf(const XPub* xpub_in, struct ext_key* xpub_out);

#endif
