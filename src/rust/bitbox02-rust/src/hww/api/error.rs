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

use crate::pb;

use crate::workflow::unlock::UnlockError;

extern crate alloc;
use alloc::string::String;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
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

// See the unit tests below for usage patterns.
#[derive(Debug, PartialEq)]
pub struct Error {
    // Ideally we want this to be `Option<String>` so the messages can
    // be dynamic and contain runtime info, but that quickly leads to
    // uncontrolled binary bloat. For now we only do static error
    // context strings.
    pub msg: Option<&'static str>,
    pub kind: ErrorKind,
}

impl Error {
    pub fn err<E: core::convert::Into<ErrorKind>>(e: E) -> Error {
        Error {
            msg: None,
            kind: e.into(),
        }
    }
    pub fn err_generic<E>(_e: E) -> Error {
        Error {
            msg: None,
            kind: ErrorKind::Generic,
        }
    }

    pub fn err_memory<E>(_e: E) -> Error {
        Error {
            msg: None,
            kind: ErrorKind::Memory,
        }
    }

    pub fn err_invalid_input<E>(_e: E) -> Error {
        Error {
            msg: None,
            kind: ErrorKind::InvalidInput,
        }
    }

    pub fn err_noise_encrypt<E>(_e: E) -> Error {
        Error {
            msg: None,
            kind: ErrorKind::NoiseEncrypt,
        }
    }

    pub fn err_noise_decrypt<E>(_e: E) -> Error {
        Error {
            msg: None,
            kind: ErrorKind::NoiseDecrypt,
        }
    }
}

pub trait Context<A> {
    fn context(self, msg: &'static str) -> Result<A, Error>;
}

impl<A> Context<A> for Result<A, Error> {
    fn context(self, msg: &'static str) -> Result<A, Error> {
        self.map_err(|e| Error {
            msg: Some(msg),
            kind: e.kind,
        })
    }
}

impl core::convert::From<()> for ErrorKind {
    fn from(_error: ()) -> Self {
        ErrorKind::Generic
    }
}

impl core::convert::From<bitbox02::memory::Error> for ErrorKind {
    fn from(_error: bitbox02::memory::Error) -> Self {
        ErrorKind::Memory
    }
}

impl core::convert::From<crate::workflow::cancel::Error> for ErrorKind {
    fn from(_error: crate::workflow::cancel::Error) -> Self {
        ErrorKind::UserAbort
    }
}

impl core::convert::From<crate::workflow::confirm::UserAbort> for ErrorKind {
    fn from(_error: crate::workflow::confirm::UserAbort) -> Self {
        ErrorKind::UserAbort
    }
}

impl core::convert::From<crate::workflow::transaction::UserAbort> for ErrorKind {
    fn from(_error: crate::workflow::transaction::UserAbort) -> Self {
        ErrorKind::UserAbort
    }
}

impl core::convert::From<crate::workflow::verify_message::Error> for ErrorKind {
    fn from(error: crate::workflow::verify_message::Error) -> Self {
        match error {
            crate::workflow::verify_message::Error::InvalidInput => ErrorKind::InvalidInput,
            crate::workflow::verify_message::Error::UserAbort => ErrorKind::UserAbort,
        }
    }
}

impl core::convert::From<UnlockError> for ErrorKind {
    fn from(error: UnlockError) -> Self {
        match error {
            UnlockError::UserAbort => ErrorKind::UserAbort,
            UnlockError::IncorrectPassword | UnlockError::Generic => ErrorKind::Generic,
        }
    }
}

impl<A: core::convert::Into<ErrorKind>> core::convert::From<A> for Error {
    fn from(error: A) -> Self {
        Error {
            msg: None,
            kind: error.into(),
        }
    }
}

use pb::response::Response;

/// Creates an Error response. Corresponds to commander.c:_report_error().
pub fn make_error(err: Error) -> Response {
    fn format_err(kind_str: &str, msg: Option<&str>) -> String {
        match msg {
            Some(msg) => format!("{}: {}", kind_str, msg),
            None => kind_str.into(),
        }
    }

    use ErrorKind::*;
    let err = match err.kind {
        InvalidInput => pb::Error {
            code: 101,
            message: format_err("invalid input", err.msg),
        },
        Memory => pb::Error {
            code: 102,
            message: format_err("memory", err.msg),
        },
        Generic => pb::Error {
            code: 103,
            message: format_err("generic error", err.msg),
        },
        UserAbort => pb::Error {
            code: 104,
            message: format_err("aborted by the user", err.msg),
        },
        InvalidState => pb::Error {
            code: 105,
            message: format_err("can't call this endpoint: wrong state", err.msg),
        },
        Disabled => pb::Error {
            code: 106,
            message: format_err("function disabled", err.msg),
        },
        Duplicate => pb::Error {
            code: 107,
            message: format_err("duplicate entry", err.msg),
        },
        NoiseEncrypt => pb::Error {
            code: 108,
            message: format_err("noise encryption failed", err.msg),
        },
        NoiseDecrypt => pb::Error {
            code: 109,
            message: format_err("noise decryption failed", err.msg),
        },
    };
    Response::Error(err)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn err_generic() -> Result<(), ()> {
        Err(())
    }

    fn converted_err_generic() -> Result<(), Error> {
        Ok(err_generic()?)
    }

    fn err_generic_with_context() -> Result<(), Error> {
        Ok(err_generic()
            .map_err(Error::err)
            .context("generic context")?)
    }

    fn err_replaced_context() -> Result<(), Error> {
        Ok(err_generic_with_context().context("new generic context")?)
    }

    fn err_adhoc_kind() -> Result<(), Error> {
        Ok(err_generic()
            .map_err(Error::err_invalid_input)
            .context("invalid input context")?)
    }

    fn err_user_abort() -> Result<(), crate::workflow::confirm::UserAbort> {
        Err(crate::workflow::confirm::UserAbort {})
    }

    fn converted_err_user_abort() -> Result<(), Error> {
        Ok(err_user_abort()?)
    }

    fn err_user_abort_with_context() -> Result<(), Error> {
        Ok(err_user_abort()
            .map_err(Error::err)
            .context("user abort context")?)
    }

    #[test]
    pub fn test_errors() {
        assert_eq!(
            err_generic().map_err(Error::err),
            Err(Error {
                msg: None,
                kind: ErrorKind::Generic
            })
        );

        assert_eq!(
            converted_err_generic(),
            Err(Error {
                msg: None,
                kind: ErrorKind::Generic
            })
        );

        assert_eq!(
            err_generic_with_context(),
            Err(Error {
                msg: Some("generic context".into()),
                kind: ErrorKind::Generic
            })
        );

        assert_eq!(
            err_replaced_context(),
            Err(Error {
                msg: Some("new generic context".into()),
                kind: ErrorKind::Generic
            })
        );

        assert_eq!(
            err_adhoc_kind(),
            Err(Error {
                msg: Some("invalid input context".into()),
                kind: ErrorKind::InvalidInput
            })
        );

        assert_eq!(
            converted_err_user_abort(),
            Err(Error {
                msg: None,
                kind: ErrorKind::UserAbort
            })
        );

        assert_eq!(
            err_user_abort_with_context(),
            Err(Error {
                msg: Some("user abort context"),
                kind: ErrorKind::UserAbort
            })
        );
    }

    #[test]
    pub fn test_make_error() {
        assert_eq!(
            make_error(converted_err_user_abort().unwrap_err()),
            Response::Error(pb::Error {
                code: 104,
                message: "aborted by the user".into(),
            })
        );

        assert_eq!(
            make_error(err_user_abort_with_context().unwrap_err()),
            Response::Error(pb::Error {
                code: 104,
                message: "aborted by the user: user abort context".into(),
            })
        );
    }
}
