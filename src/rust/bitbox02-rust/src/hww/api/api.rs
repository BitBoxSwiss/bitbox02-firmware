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

use alloc::vec::Vec;

use super::pb;
use super::pb::request::Request;
use super::pb::response::Response;
use bitbox02::commander::Error;
use prost::Message;

/// Creates an Error response. Corresponds to commander.c:_report_error().
fn make_error(err: bitbox02::commander::Error) -> Response {
    use Error::*;
    let err = match err {
        COMMANDER_OK => panic!("can't call this function with COMMANDER_OK"),
        COMMANDER_ERR_INVALID_INPUT => pb::Error {
            code: 101,
            message: "invalid input".into(),
        },
        COMMANDER_ERR_MEMORY => pb::Error {
            code: 102,
            message: "memory".into(),
        },
        COMMANDER_ERR_GENERIC => pb::Error {
            code: 103,
            message: "generic error".into(),
        },
        COMMANDER_ERR_USER_ABORT => pb::Error {
            code: 104,
            message: "aborted by the user".into(),
        },
        COMMANDER_ERR_INVALID_STATE => pb::Error {
            code: 105,
            message: "can't call this endpoint: wrong state".into(),
        },
        COMMANDER_ERR_DISABLED => pb::Error {
            code: 106,
            message: "function disabled".into(),
        },
        COMMANDER_ERR_DUPLICATE => pb::Error {
            code: 107,
            message: "duplicate entry".into(),
        },
    };
    Response::Error(err)
}

/// Encodes a protobuf Response message.
fn encode(response: Response) -> Vec<u8> {
    let response = pb::Response {
        response: Some(response),
    };
    let mut out = Vec::<u8>::new();
    response.encode(&mut out).unwrap();
    out
}

async fn api_set_device_name(
    pb::SetDeviceNameRequest { name }: &pb::SetDeviceNameRequest,
) -> Response {
    use crate::workflow::confirm;
    let params = confirm::Params {
        title: "Name",
        body: &name,
        scrollable: true,
        ..Default::default()
    };

    if !confirm::confirm(&params).await {
        return make_error(Error::COMMANDER_ERR_USER_ABORT);
    }

    if bitbox02::memory::set_device_name(&name).is_err() {
        return make_error(Error::COMMANDER_ERR_MEMORY);
    }

    Response::Success(pb::Success {})
}

/// Handle a protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case
/// it should be handled by the C commander.
async fn process_api(request: &Request) -> Option<Response> {
    match request {
        Request::DeviceName(ref request) => Some(api_set_device_name(request).await),
        _ => None,
    }
}

/// Handle a protobuf api call.  API calls not handled by Rust are
/// handled by the C commander, which allows us to use Rust for new
/// api calls and port the old calls step by step.
///
/// `input` is a hww.proto Request message, protobuf encoded.
/// Returns a protobuf encoded hww.proto Response message.
pub async fn process(input: Vec<u8>) -> Vec<u8> {
    let request = match pb::Request::decode(&input[..]) {
        Ok(pb::Request {
            request: Some(request),
        }) => request,
        _ => return encode(make_error(Error::COMMANDER_ERR_INVALID_INPUT)),
    };
    match process_api(&request).await {
        Some(response) => encode(response),
        // Api call not handled in Rust -> handle it in C.
        _ => bitbox02::commander::commander(input),
    }
}
