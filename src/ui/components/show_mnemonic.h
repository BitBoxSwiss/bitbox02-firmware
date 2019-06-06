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

#ifndef _SHOW_MNEMONIC_H_
#define _SHOW_MNEMONIC_H_

#include <ui/component.h>

/**
 * Creates a screen that allows to scroll through the mnemonic words.
 * @param[in] wordlist The mnemonic sentence, split into words.
 * @param[in] length The amount of words in the mnemonic sentence.
 * @param[in] confirm_mnemonic A callback that lets the user confirm the words he/she wrote down.
 */
component_t* show_mnemonic_create(
    const char** wordlist,
    uint8_t length,
    void (*confirm_mnemonic)(void));

#endif
