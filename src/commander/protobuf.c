// Copyright 2020 Shift Crypto AG
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

#include "protobuf.h"

#include <hardfault.h>

#include <pb_decode.h>
#include <pb_encode.h>

bool protobuf_decode(const in_buffer_t* buf, Request* request_out)
{
    pb_istream_t in_stream = pb_istream_from_buffer(buf->data, buf->len);
    return pb_decode(&in_stream, Request_fields, request_out);
}

void protobuf_encode(buffer_t* buf_out, const Response* response)
{
    pb_ostream_t out_stream = pb_ostream_from_buffer(buf_out->data, buf_out->max_len);
    if (!pb_encode(&out_stream, Response_fields, response)) {
        Abort("Abort: pb_encode");
    }
    buf_out->len = out_stream.bytes_written;
}
