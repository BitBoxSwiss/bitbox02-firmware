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

use crate::hal::Sd;
use crate::hal::Ui;

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    &pb::InsertRemoveSdCardRequest { action }: &pb::InsertRemoveSdCardRequest,
) -> Result<Response, Error> {
    let inserted = hal.sd().sdcard_inserted();
    match SdCardAction::try_from(action) {
        Ok(SdCardAction::InsertCard) => {}
        _ => return Ok(Response::Success(pb::Success {})),
    };
    if inserted {
        return Ok(Response::Success(pb::Success {}));
    }
    hal.ui().insert_sdcard().await?;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use alloc::boxed::Box;

    #[test]
    pub fn test_reset() {
        // already inserted.
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::InsertRemoveSdCardRequest {
                    action: SdCardAction::InsertCard as _,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );

        // already removed.
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(false);
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::InsertRemoveSdCardRequest {
                    action: SdCardAction::RemoveCard as _,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );

        // insert
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(false);
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::InsertRemoveSdCardRequest {
                    action: SdCardAction::InsertCard as _,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );

        // remove
        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::InsertRemoveSdCardRequest {
                    action: SdCardAction::RemoveCard as _,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );
    }
}
