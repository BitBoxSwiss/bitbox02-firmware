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

use super::pb;

pub use bitbox02::commander::Error;

use pb::response::Response;

/// Creates an Error response. Corresponds to commander.c:_report_error().
pub fn make_error(err: bitbox02::commander::Error) -> Response {
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
