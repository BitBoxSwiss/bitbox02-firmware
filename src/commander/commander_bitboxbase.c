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

#include "commander_bitboxbase.h"
#include "commander.h"

#include "rust/bitbox02_rust.h"

#include "hww.pb.h"
#include <pb_decode.h>
#include <pb_encode.h>

static commander_error_t _api_display_base32(const BitBoxBaseDisplayBase32Request* request)
{
    if (!bitboxbase_workflow_display_base32(request->msg, sizeof(request->msg))) {
        return COMMANDER_ERR_GENERIC;
    }
    return COMMANDER_OK;
}

commander_error_t commander_bitboxbase(const BitBoxBaseRequest* request) {
    switch(request->which_request) {
    case BitBoxBaseRequest_display_base32_tag:
        return _api_display_base32(&(request->request.display_base32));
    default:
        return COMMANDER_ERR_GENERIC;
    }
}
