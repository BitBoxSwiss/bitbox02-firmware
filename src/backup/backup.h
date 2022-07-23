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

#ifndef _BACKUP_BACKUP_H_
#define _BACKUP_BACKUP_H_

#include "backup_common.h"

#include <backup.pb.h>

#include <stddef.h>
#include <stdint.h>

/**
 * Encodes the backup as protobuf.
 * @return the number of bytes written, or 0 if encoding failed.
 */
size_t backup_encode(const Backup* backup, uint32_t max_size, uint8_t* output);

backup_error_t backup_create(uint32_t backup_create_timestamp, uint32_t seed_birthdate_timestamp);

#endif
