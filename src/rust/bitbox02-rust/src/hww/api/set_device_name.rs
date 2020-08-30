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
use super::Error;

use pb::response::Response;

use crate::workflow::confirm;
use bitbox02::ui::UI;

pub async fn process<M: bitbox02::memory::Memory, U: UI>(
    pb::SetDeviceNameRequest { name }: &pb::SetDeviceNameRequest,
) -> Result<Response, Error> {
    let params = confirm::Params {
        title: "Name",
        body: &name,
        scrollable: true,
        ..Default::default()
    };

    if !confirm::confirm::<U>(&params).await {
        return Err(Error::COMMANDER_ERR_USER_ABORT);
    }

    if M::set_device_name(&name).is_err() {
        return Err(Error::COMMANDER_ERR_MEMORY);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::prelude::v1::*;

    use crate::bb02_async::block_on;
    use alloc::rc::Rc;
    use core::cell::RefCell;
    use lazy_static::lazy_static;
    use std::sync::{Arc, Mutex};

    #[test]
    pub fn test_set_device_name() {
        static EXPECTED_NAME: &str = "foo";

        lazy_static! {
            static ref MEM_RESULT: Arc<Mutex<Result<(), ()>>> = Arc::new(Mutex::new(Ok(())));
            static ref USER_RESULT: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        }

        enum MockMemory {}
        enum MockUI {}

        impl bitbox02::memory::Memory for MockMemory {
            fn set_device_name(name: &str) -> Result<(), ()> {
                assert_eq!(name, EXPECTED_NAME);
                *MEM_RESULT.lock().unwrap()
            }
        }

        impl bitbox02::ui::UI for MockUI {
            fn confirm_create<'a, F>(
                params: &bitbox02::ui::ConfirmParams,
                mut result_callback: F,
            ) -> bitbox02::ui::Component<'a, Self>
            where
                F: FnMut(bool) + 'a,
            {
                assert_eq!(params.body, EXPECTED_NAME);
                assert!(params.scrollable);
                result_callback(*USER_RESULT.lock().unwrap());
                bitbox02::ui::Component::new_for_test()
            }
        }

        let set_device_name = || {
            block_on::<MockUI, _, _>(process::<MockMemory, MockUI>(&pb::SetDeviceNameRequest {
                name: EXPECTED_NAME.into(),
            }))
        };

        *USER_RESULT.lock().unwrap() = true;
        *MEM_RESULT.lock().unwrap() = Ok(());
        assert_eq!(set_device_name(), Ok(Response::Success(pb::Success {})));

        *USER_RESULT.lock().unwrap() = true;
        *MEM_RESULT.lock().unwrap() = Err(());
        assert_eq!(set_device_name(), Err(Error::COMMANDER_ERR_MEMORY));

        *USER_RESULT.lock().unwrap() = false;
        assert_eq!(set_device_name(), Err(Error::COMMANDER_ERR_USER_ABORT));
    }
}
