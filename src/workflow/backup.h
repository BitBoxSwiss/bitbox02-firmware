// Copyright 2019 Shift Cryptosecurity AG
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

#ifndef _WORKFLOW_BACKUP_H_
#define _WORKFLOW_BACKUP_H_

#include <hww.pb.h>
#include <stdbool.h>

/**
 * Starts the create backup workflow.
 * @return true if the backup was created successfully.
 */
bool workflow_backup_create(const CreateBackupRequest* create_backup);

#endif
