// Copyright 2020 Shift Cryptosecurity AG
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

#ifndef _HWW_API_H_
#define _HWW_API_H_

#include <util.h>

/**
 * Executes the HWW packet. This is the entry point of the HWW API. It handles a few bare OP codes,
 * and otherwise passes api processing to the commander (noise encrypted protobuf api messages).
 * @param[in] in_req The incoming HWW packet.
 * @param[in] out_rsp The outgoing HWW packet.
 */
void hww_api_process_packet(const in_buffer_t* in_req, buffer_t* out_rsp);

#endif
