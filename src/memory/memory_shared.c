// Copyright 2022 Shift Crypto AG
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

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <flags.h>

#include "memory_shared.h"

#ifdef TESTING
#include <mock_memory.h>
#endif

void memory_read_shared_bootdata(chunk_shared_t* chunk_out)
{
#ifdef TESTING
    memory_read_shared_bootdata_mock(chunk_out->bytes);
#else
    memcpy(chunk_out->bytes, (uint8_t*)(FLASH_SHARED_DATA_START), FLASH_SHARED_DATA_LEN);
#endif
}
