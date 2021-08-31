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

#ifndef _SD_H_
#define _SD_H_

#include "compiler_util.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define SD_MAX_FILE_SIZE (1024) // 1kB, increase if needed

typedef struct {
    size_t num_files;
    char** files;
} sd_list_t;

// the caller must free list_out using sd_free_list(). list_out itself is not
// freed, as it could be passed as a stack var.
USE_RESULT bool sd_list(sd_list_t* list_out);
USE_RESULT bool sd_list_subdir(sd_list_t* list_out, const char* subdir);
void sd_free_list(sd_list_t* list);
USE_RESULT bool sd_card_inserted(void);
// returns false on error. If no error, exists_out will have the result.
USE_RESULT bool sd_file_exists(const char* fn, bool* exists_out);
USE_RESULT bool sd_file_exists_subdir(const char* fn, const char* subdir, bool* exists_out);
// returns true if the erase was successful.
USE_RESULT bool sd_erase_file(const char* fn);
USE_RESULT bool sd_erase_dir(const char* directory_name);
USE_RESULT bool sd_erase_file_in_subdir(const char* fn, const char* subdir);
USE_RESULT bool sd_file_rename(const char* from, const char* to, const char* dir);

/**
 * Reads binary data from SD card.
 * @param[in] fn The file name of the file that should be opened.
 * @param[in] dir The name of the directory, or NULL if file should be in the root dir.
 * @param[out] buffer The data read from the file.
 * @param[out] length_out The length of the data byte array.
 * @return true if mounting and opening the file is OK, false otherwise.
 */
USE_RESULT bool sd_load_bin(const char* fn, const char* dir, uint8_t* buffer, size_t* length_out);

/**
 * Writes binary data to SD card.
 * @param[in] fn The file name of the file that should be opened.
 * @param[in] dir The name of the directory, or NULL if file should be in the root dir.
 * @param[in] data The data that should be written into the file.
 * @param[in] length The length of the data byte array.
 * @param[in] replace Whether the file can be replaced.
 * @return true if mounting and opening the file is OK, false otherwise.
 */
USE_RESULT bool sd_write_bin(
    const char* fn,
    const char* dir,
    const uint8_t* data,
    uint16_t length,
    bool replace);

#endif
