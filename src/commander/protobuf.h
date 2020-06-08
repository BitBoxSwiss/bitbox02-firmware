// Copyright 2020 Shift Cryptos AG
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

#ifndef _COMMANDER_PROTOBUF_H_
#define _COMMANDER_PROTOBUF_H_

#include <compiler_util.h>
#include <util.h>

#include <hww.pb.h>

#include <stdbool.h>

/**
 * Parses an api protobuf request.
 */
USE_RESULT bool protobuf_decode(const in_buffer_t* in_buf, Request* request_out);

/**
 * Serializes an api protobuf response. Aborts() on error (e.g. if the output buffer is too short).
 */
void protobuf_encode(buffer_t* buf_out, const Response* response);

#endif
