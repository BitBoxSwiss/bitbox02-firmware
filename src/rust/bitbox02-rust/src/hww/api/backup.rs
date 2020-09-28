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

use crate::workflow::{confirm, status};
use bitbox02::backup;

pub async fn check(
    &pb::CheckBackupRequest { silent }: &pb::CheckBackupRequest,
) -> Result<Response, Error> {
    if !bitbox02::sdcard_inserted() {
        return Err(Error::COMMANDER_ERR_INVALID_INPUT);
    }
    match backup::check() {
        Ok(backup::CheckData { id, name, .. }) => {
            if !silent {
                let params = confirm::Params {
                    title: "Name?",
                    body: &name,
                    scrollable: true,
                    ..Default::default()
                };

                if !confirm::confirm(&params).await {
                    return Err(Error::COMMANDER_ERR_GENERIC);
                }

                let params = confirm::Params {
                    title: "ID?",
                    body: &id,
                    scrollable: true,
                    ..Default::default()
                };

                if !confirm::confirm(&params).await {
                    return Err(Error::COMMANDER_ERR_GENERIC);
                }

                status::status("Backup valid", true).await;
            }
            Ok(Response::CheckBackup(pb::CheckBackupResponse { id }))
        }
        Err(backup::Error::BACKUP_ERR_CHECK) => {
            if !silent {
                status::status("Backup missing\nor invalid", false).await;
            }
            Err(Error::COMMANDER_ERR_GENERIC)
        }
        Err(err) => {
            let msg = format!("Could not check\nbackup\n{:?}", err).replace("BACKUP_ERR_", "");
            status::status(&msg, false).await;
            Err(Error::COMMANDER_ERR_GENERIC)
        }
    }
}
