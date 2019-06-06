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

#include <assert_sd.h>
#include <sd.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <util.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-prototypes"
#pragma GCC diagnostic ignored "-Wunused-function"

typedef struct {
    char name[256];
    char data[SD_MAX_FILE_SIZE];
    uint16_t length;
} file_content_t;

typedef struct {
    char name[256];
    file_content_t files[100];
    uint8_t count;
} dir_content_t;

static dir_content_t _directories[100] = {0};
static uint8_t _dir_count = 0;

bool __wrap_sd_write_bin(
    const char* fn,
    const char* dn,
    const uint8_t* data,
    const uint16_t length,
    bool replace)
{
    if (!length || length > SD_MAX_FILE_SIZE) {
        return false;
    }

    dir_content_t* dir = NULL;
    file_content_t* file_content = NULL;
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, dn) == 0) {
            dir = &_directories[i];
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                if (strcmp(_directories[i].files[j].name, fn) == 0) {
                    if (!replace) {
                        return false;
                    }
                    file_content = &_directories[i].files[j];
                }
            }
        }
    }
    if (dir == NULL) {
        // mkdir
        dir = &_directories[_dir_count++];
        if (snprintf(dir->name, 256, "%s", dn) >= 256) {
            return false;
        }
    }
    if (file_content == NULL) {
        // create file
        file_content = &dir->files[dir->count++];
        if (snprintf(file_content->name, 256, "%s", fn) >= 256) {
            return false;
        }
    }

    file_content->length = length;
    memset(file_content->data, 0, sizeof(file_content->data));
    memcpy(file_content->data, data, file_content->length);
    return true;
}

bool __wrap_sd_load_bin(const char* fn, const char* dn, uint8_t* buffer, uint32_t* length_out)
{
    if (buffer == NULL || length_out == NULL) {
        return false;
    }
    bool found = false;
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, dn) == 0) {
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                file_content_t* file_content = &_directories[i].files[j];
                if (strcmp(file_content->name, fn) == 0) {
                    memcpy(buffer, file_content->data, file_content->length);
                    *length_out = file_content->length;
                    found = true;
                }
            }
        }
    }
    return found;
}

bool __wrap_sd_list_subdir(sd_list_t* list_out, const char* subdir)
{
    if (list_out == NULL) {
        return false;
    }
    list_out->num_files = 0;
    size_t allocated_files = 16;
    list_out->files = (char**)calloc(sizeof(char*), allocated_files);
    if (list_out->files == NULL) {
        return false;
    }
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, subdir) == 0) {
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                char* fn_copy = strdup(_directories[i].files[j].name);
                list_out->files[j] = fn_copy;
            }
            list_out->num_files = _directories[i].count;
        }
    }
    return true;
}

bool __wrap_sd_list(sd_list_t* list_out)
{
    if (list_out == NULL) {
        return false;
    }
    list_out->num_files = 0;
    size_t allocated_files = 16;
    list_out->files = (char**)calloc(sizeof(char*), allocated_files);
    if (list_out->files == NULL) {
        return false;
    }
    for (uint8_t i = 0; i < _dir_count; i++) {
        char* dn_copy = strdup(_directories[i].name);
        list_out->files[i] = dn_copy;
        list_out->num_files++;
    }
    return true;
}

bool __wrap_sd_erase_file_in_subdir(const char* fn, const char* subdir)
{
    bool removed = false;
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, subdir) == 0) {
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                if (removed) {
                    memcpy(
                        &_directories[i].files[j - 1],
                        &_directories[i].files[j],
                        sizeof(file_content_t));
                }
                if (strcmp(_directories[i].files[j].name, fn) == 0) {
                    removed = true;
                }
            }
            if (removed) {
                _directories[i].count--;
            }
        }
    }
    return removed;
}

bool __wrap_sd_file_rename(const char* from, const char* to, const char* dir)
{
    if (!strlens(from) || !strlens(to) || !sd_card_inserted()) {
        return false;
    }
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, dir) == 0) {
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                if (strcmp(_directories[i].files[j].name, from) == 0) {
                    snprintf(_directories[i].files[j].name, 256, "%s", to);
                    return true;
                }
            }
            return false;
        }
    }
    return false;
}

bool dir_exists(const char* dir)
{
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, dir) == 0) {
            return true;
        }
    }
    return false;
}

bool file_exists(const char* file_name, const char* dir)
{
    for (uint8_t i = 0; i < _dir_count; i++) {
        if (strcmp(_directories[i].name, dir) == 0) {
            for (uint8_t j = 0; j < _directories[i].count; j++) {
                if (strcmp(_directories[i].files[j].name, file_name) == 0) {
                    return true;
                }
            }
        }
    }
    return false;
}

void reset_sd(void)
{
    _dir_count = 0;
    memset(_directories, 0, sizeof(_directories));
}

#pragma GCC diagnostic pop
