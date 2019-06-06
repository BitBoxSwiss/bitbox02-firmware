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

#ifndef _WORKFLOW_UNLOCK_H_
#define _WORKFLOW_UNLOCK_H_

/**
 * Unlocks the keystore and stretches the seed with
 * a BIP39 passphrase.
 */
void workflow_unlock_enter_done(const char* password);

void workflow_unlock(void);

#endif
