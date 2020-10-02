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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInput,
    Memory,
    Generic,
    UserAbort,
    InvalidState,
    Disabled,
    Duplicate,
}

impl core::convert::From<()> for Error {
    fn from(_error: ()) -> Self {
        Error::Generic
    }
}

use pb::response::Response;

/// Creates an Error response. Corresponds to commander.c:_report_error().
pub fn make_error(err: Error) -> Response {
    use Error::*;
    let err = match err {
        InvalidInput => pb::Error {
            code: 101,
            message: "invalid input".into(),
        },
        Memory => pb::Error {
            code: 102,
            message: "memory".into(),
        },
        Generic => pb::Error {
            code: 103,
            message: "generic error".into(),
        },
        UserAbort => pb::Error {
            code: 104,
            message: "aborted by the user".into(),
        },
        InvalidState => pb::Error {
            code: 105,
            message: "can't call this endpoint: wrong state".into(),
        },
        Disabled => pb::Error {
            code: 106,
            message: "function disabled".into(),
        },
        Duplicate => pb::Error {
            code: 107,
            message: "duplicate entry".into(),
        },
    };
    Response::Error(err)
}
