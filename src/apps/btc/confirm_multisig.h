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

#ifndef _APPS_BTC_CONFIRM_MULTISIG_H_
#define _APPS_BTC_CONFIRM_MULTISIG_H_

#include <btc.pb.h>
#include <compiler_util.h>
#include <stdbool.h>

/**
 * Confirms a multisig setup with the user.
 * Verified are:
 * - coin
 * - multisig type (m-of-n)
 * - name given by the user
 * - if verify_xpubs, all xpubs (formatted as Zpubs on mainnet; Vpubs on testnet).
 * @param[in] title the title shown in each confirmation screen
 * @param[in] coin coin to be confirmed
 * @param[in] name User given name of the multisig account.
 * @param[in] multisig multisig details
 * @param[in] verify_xpubs if true, all cosigner xpubs are verified.
 * @param[in] xpub_type: if AUTO_ELECTRUM, will automatically format xpubs as `Zpub/Vpub`,
 * `Ypub/UPub` depending on the script type, to match Electrum's formatting. If AUTO_XPUB_TPUB,
 * format as xpub (mainnets) or tpub (testnets). Only applies if `verify_xpubs` is true.
 * @return true if the user accepts all confirmation screens, false otherwise.
 */
USE_RESULT bool apps_btc_confirm_multisig(
    const char* title,
    BTCCoin coin,
    const char* name,
    const BTCScriptConfig_Multisig* multisig,
    bool verify_xpubs,
    BTCRegisterScriptConfigRequest_XPubType xpub_type);

#endif
