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

#ifndef _SCROLL_THROUGH_ALL_VARIANTS_H_
#define _SCROLL_THROUGH_ALL_VARIANTS_H_

#include <ui/component.h>

/**
 * Creates a scroll through list that renders the current word in the center and parts of the words
 * before and after on the left and right.
 * @param[in] words The words that are displayed on the screen, and through which you can slide
 * through.
 * @param[in] callback If specified, the callback will be called if the user selects a word. The
 * parameter is the index of the selected word.
 * @param[in] length The word list length.
 * @param[in] show_index If true, displays the index of the current word (starting at 1).
 * @param[in] continue_on_last If set, the left bottom button will update when the last word is
 * reached.
 * @param[in] parent The parent component.
 */
component_t* scroll_through_all_variants_create(
    const char** words,
    void (*callback)(uint8_t),
    const uint8_t length,
    bool show_index,
    void (*continue_on_last)(void),
    component_t* parent);

#endif
