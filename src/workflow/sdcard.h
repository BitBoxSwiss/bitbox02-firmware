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

#include "generated/hww.pb.h"

#ifndef _SDCARD_H_
#define _SDCARD_H_

/**
 * Handles the API call to insert or remove the SD card.
 * Checks whether SD card is inserted and blocks until it is inserted or
 * removed, depending on the API call.
 */
void sdcard_handle(const InsertRemoveSDCardRequest* insert_remove_sdcard);

#endif
