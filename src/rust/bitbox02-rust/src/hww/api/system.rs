// Copyright 2021 Shift Crypto AG
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

use super::Error;
use crate::pb;

use pb::reboot_request::Purpose;
use pb::response::Response;

use crate::workflow::confirm;

pub async fn reboot(&pb::RebootRequest { purpose }: &pb::RebootRequest) -> Result<Response, Error> {
    confirm::confirm(&confirm::Params {
        title: "",
        body: match Purpose::from_i32(purpose) {
            Some(Purpose::Upgrade) => "Proceed to upgrade?",
            Some(Purpose::Settings) => "Go to\nstartup settings?",
            // No error, if new client library sends a purpose that we don't understand,
            // we reboot anyway.
            None => "Reboot?",
        },
        ..Default::default()
    })
    .await?;
    bitbox02::reboot()
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hww::api::error::ErrorKind;
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;

    #[test]
    pub fn test_reboot() {
        let _guard = MUTEX.lock().unwrap();

        mock(Data {
            ui_confirm_create: Some(Box::new(|_| true)),
            ..Default::default()
        });
        let reboot_called = std::panic::catch_unwind(|| {
            block_on(reboot(&pb::RebootRequest {
                purpose: Purpose::Upgrade as _,
            }))
            .unwrap();
        });
        match reboot_called {
            Ok(()) => panic!("reboot was not called"),
            Err(msg) => assert_eq!(msg.downcast_ref::<&str>(), Some(&"reboot called")),
        }
    }

    #[test]
    pub fn test_reboot_aborted() {
        let _guard = MUTEX.lock().unwrap();

        mock(Data {
            ui_confirm_create: Some(Box::new(|_| false)),
            ..Default::default()
        });
        assert_eq!(
            block_on(reboot(&pb::RebootRequest {
                purpose: Purpose::Upgrade as _
            }))
            .unwrap_err()
            .kind,
            ErrorKind::UserAbort,
        );
    }
}
