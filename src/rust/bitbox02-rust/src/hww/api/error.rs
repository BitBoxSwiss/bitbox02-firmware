// Copyright 2020-2024 Shift Crypto AG
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

use crate::pb;

use crate::workflow::unlock::UnlockError;

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
    NoiseEncrypt,
    NoiseDecrypt,
}

impl core::convert::From<()> for Error {
    fn from(_error: ()) -> Self {
        Error::Generic
    }
}

impl core::convert::From<bitbox02::memory::Error> for Error {
    fn from(_error: bitbox02::memory::Error) -> Self {
        Error::Memory
    }
}

impl core::convert::From<crate::workflow::cancel::Error> for Error {
    fn from(_error: crate::workflow::cancel::Error) -> Self {
        Error::UserAbort
    }
}

impl core::convert::From<crate::workflow::password::EnterTwiceError> for Error {
    fn from(error: crate::workflow::password::EnterTwiceError) -> Self {
        match error {
            crate::workflow::password::EnterTwiceError::DoNotMatch => {
                // For backwards compatibility.
                Error::Generic
            }
            crate::workflow::password::EnterTwiceError::Cancelled => {
                // Added in v9.13.0.
                Error::UserAbort
            }
        }
    }
}

impl core::convert::From<crate::workflow::confirm::UserAbort> for Error {
    fn from(_error: crate::workflow::confirm::UserAbort) -> Self {
        Error::UserAbort
    }
}

impl core::convert::From<crate::workflow::transaction::UserAbort> for Error {
    fn from(_error: crate::workflow::transaction::UserAbort) -> Self {
        Error::UserAbort
    }
}

impl core::convert::From<crate::workflow::sdcard::UserAbort> for Error {
    fn from(_error: crate::workflow::sdcard::UserAbort) -> Self {
        Error::UserAbort
    }
}

impl core::convert::From<crate::workflow::verify_message::Error> for Error {
    fn from(error: crate::workflow::verify_message::Error) -> Self {
        match error {
            crate::workflow::verify_message::Error::InvalidInput => Error::InvalidInput,
            crate::workflow::verify_message::Error::UserAbort => Error::UserAbort,
        }
    }
}

#[cfg(feature = "app-cardano")]
impl core::convert::From<crate::hww::api::cardano::keypath::Error> for Error {
    fn from(_error: crate::hww::api::cardano::keypath::Error) -> Self {
        Error::InvalidInput
    }
}

#[cfg(feature = "app-cardano")]
impl<E> core::convert::From<minicbor::encode::Error<E>> for Error {
    fn from(_error: minicbor::encode::Error<E>) -> Self {
        Error::Generic
    }
}

impl core::convert::From<UnlockError> for Error {
    fn from(error: UnlockError) -> Self {
        match error {
            UnlockError::UserAbort => Error::UserAbort,
            UnlockError::IncorrectPassword | UnlockError::Generic => Error::Generic,
        }
    }
}

impl core::convert::From<prost::UnknownEnumValue> for Error {
    fn from(_error: prost::UnknownEnumValue) -> Self {
        Error::InvalidInput
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
        NoiseEncrypt => pb::Error {
            code: 108,
            message: "noise encryption failed".into(),
        },
        NoiseDecrypt => pb::Error {
            code: 109,
            message: "noise decryption failed".into(),
        },
    };
    Response::Error(err)
}
