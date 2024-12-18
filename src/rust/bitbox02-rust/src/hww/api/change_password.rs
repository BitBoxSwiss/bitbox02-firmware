// Copyright 2024 Shift Crypto AG
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

use crate::workflow::{password, status, unlock};
use bitbox02::keystore;
use pb::response::Response;

pub async fn process() -> Result<Response, Error> {
    let old_password = password::enter("Old password", false, unlock::CanCancel::Yes).await?;

    if keystore::unlock(&old_password).is_err() {
        status::status("Wrong password", false).await;
        return Err(Error::Generic);
    }

    let new_password = password::enter_twice().await?;
    keystore::change_password(&old_password, &new_password).unwrap();

    Ok(Response::Success(pb::Success {}))
}
