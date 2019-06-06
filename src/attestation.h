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

#ifndef _ATTESTATION_H_
#define _ATTESTATION_H_

#include <stdbool.h>
#include <stdint.h>

#include "generated/hww.pb.h"

bool attestation_perform(const uint8_t* host_challenge, PerformAttestationResponse* result_out);

#endif
