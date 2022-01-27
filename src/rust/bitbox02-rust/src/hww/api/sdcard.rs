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

use super::Error;
use crate::pb;
use crate::pb::insert_remove_sd_card_request::SdCardAction;

use pb::response::Response;

use crate::workflow::sdcard;

pub async fn process(
    &pb::InsertRemoveSdCardRequest { action }: &pb::InsertRemoveSdCardRequest,
) -> Result<Response, Error> {
    let inserted = bitbox02::sdcard_inserted();
    let action = match SdCardAction::from_i32(action) {
        Some(action) => action,
        None => return Ok(Response::Success(pb::Success {})),
    };
    // No action required, already inserted (INSERT request) or not inserted (REMOVE request)
    if (action == SdCardAction::InsertCard && inserted)
        || (action == SdCardAction::RemoveCard && !inserted)
    {
        return Ok(Response::Success(pb::Success {}));
    }
    sdcard::sdcard(action == SdCardAction::InsertCard).await;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, Data};
    use std::boxed::Box;

    #[test]
    pub fn test_reset() {
        // already inserted.
        mock(Data {
            sdcard_inserted: Some(true),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::InsertRemoveSdCardRequest {
                action: SdCardAction::InsertCard as _,
            })),
            Ok(Response::Success(pb::Success {}))
        );

        // already removed.
        mock(Data {
            sdcard_inserted: Some(false),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::InsertRemoveSdCardRequest {
                action: SdCardAction::RemoveCard as _,
            })),
            Ok(Response::Success(pb::Success {}))
        );

        // insert
        mock(Data {
            sdcard_inserted: Some(false),
            ui_sdcard_create_arg: Some(true),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::InsertRemoveSdCardRequest {
                action: SdCardAction::InsertCard as _,
            })),
            Ok(Response::Success(pb::Success {}))
        );

        // remove
        mock(Data {
            sdcard_inserted: Some(true),
            ui_sdcard_create_arg: Some(false),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::InsertRemoveSdCardRequest {
                action: SdCardAction::RemoveCard as _,
            })),
            Ok(Response::Success(pb::Success {}))
        );
    }
}
