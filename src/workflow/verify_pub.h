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

#ifndef _WORKFLOW_VERIFY_PUB_H_
#define _WORKFLOW_VERIFY_PUB_H_

/**
 * Shows a confirmation dialog to the user to confirm an xpub/address.
 * This call blocks until the user dismisses the screen.
 * @param[in] title, e.g. "BTC Account #1", "BTC Address #10", etc.
 * @param[in] pub the xpub/address/...
 */
void workflow_verify_pub(const char* title, const char* pub);

#endif
