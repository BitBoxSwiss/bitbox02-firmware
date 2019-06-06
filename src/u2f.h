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

#ifndef _U2F_DEVICE_H_
#define _U2F_DEVICE_H_

#include "usb/u2f/u2f.h"
#include "usb/usb.h"
#include <stdbool.h>
#include <stdint.h>

#define U2F_HIJACK_ORIGIN_TOTAL 2

extern const uint8_t U2F_HIJACK_CODE[U2F_HIJACK_ORIGIN_TOTAL][U2F_NONCE_LENGTH];

void u2f_device_timeout(void);

void u2f_device_setup(void);

#endif
