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

use crate::workflow::{confirm, Workflows};

pub async fn reboot<W: Workflows>(
    workflows: &mut W,
    &pb::RebootRequest { purpose }: &pb::RebootRequest,
) -> Result<Response, Error> {
    workflows
        .confirm(&confirm::Params {
            title: "",
            body: match Purpose::try_from(purpose) {
                Ok(Purpose::Upgrade) => "Proceed to upgrade?",
                Ok(Purpose::Settings) => "Go to\nstartup settings?",
                // No error, if new client library sends a purpose that we don't understand,
                // we reboot anyway.
                Err(_) => "Reboot?",
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
    use crate::workflow::testing::{Screen, TestingWorkflows};
    use alloc::boxed::Box;

    #[test]
    pub fn test_reboot() {
        let reboot_called = std::panic::catch_unwind(|| {
            block_on(reboot(
                &mut TestingWorkflows::new(),
                &pb::RebootRequest {
                    purpose: Purpose::Upgrade as _,
                },
            ))
            .unwrap();
        });
        match reboot_called {
            Ok(()) => panic!("reboot was not called"),
            Err(msg) => assert_eq!(msg.downcast_ref::<&str>(), Some(&"reboot called")),
        }
    }

    #[test]
    pub fn test_reboot_aborted() {
        let mut mock_workflows = TestingWorkflows::new();
        mock_workflows.abort_nth(0);
        assert_eq!(
            block_on(reboot(
                &mut mock_workflows,
                &pb::RebootRequest {
                    purpose: Purpose::Upgrade as _
                }
            )),
            Err(Error::UserAbort),
        );
    }
}
