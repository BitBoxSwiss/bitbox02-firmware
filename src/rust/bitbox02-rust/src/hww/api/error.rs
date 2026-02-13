// SPDX-License-Identifier: Apache-2.0

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

impl core::convert::From<crate::hal::memory::Error> for Error {
    fn from(error: crate::hal::memory::Error) -> Self {
        match error {
            crate::hal::memory::Error::InvalidInput => Error::InvalidInput,
            crate::hal::memory::Error::DuplicateName => Error::Duplicate,
            crate::hal::memory::Error::Unknown => Error::Memory,
            crate::hal::memory::Error::Full => Error::Generic,
        }
    }
}

impl core::convert::From<crate::workflow::cancel::Error> for Error {
    fn from(_error: crate::workflow::cancel::Error) -> Self {
        Error::UserAbort
    }
}

impl core::convert::From<crate::workflow::password::EnterError> for Error {
    fn from(error: crate::workflow::password::EnterError) -> Self {
        match error {
            crate::workflow::password::EnterError::Memory => Error::Memory,
            crate::workflow::password::EnterError::Cancelled => Error::UserAbort,
        }
    }
}

impl core::convert::From<crate::workflow::password::EnterTwiceError> for Error {
    fn from(error: crate::workflow::password::EnterTwiceError) -> Self {
        match error {
            crate::workflow::password::EnterTwiceError::DoNotMatch => {
                // For backwards compatibility.
                Error::Generic
            }
            crate::workflow::password::EnterTwiceError::EnterError(err) => err.into(),
        }
    }
}

impl core::convert::From<crate::hal::ui::UserAbort> for Error {
    fn from(_error: crate::hal::ui::UserAbort) -> Self {
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
            UnlockError::Memory => Error::Memory,
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
