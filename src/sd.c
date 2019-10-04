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

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef TESTING
#include "driver_init.h"
#include "sd_mmc.h"
#endif

#include "flags.h"
#include "hardfault.h"
#include "screen.h"
#include "sd.h"
#include "util.h"

#include <ff.h>

// max number of files returned by sd_list/sd_list_subdir.
#define LIST_MAX 200u
#if LIST_MAX < 1
#error LIST_MAX must be at least 1
#endif

static const char* ROOTDIR = "0:/bitbox02";
FATFS fs;

/**
 * Gets the full directory for an optionally given sub-directory.
 * Also creates the sub-directory if it doesn't exist yet.
 * @param[in] dir The sub-directory in which the file should reside, or NULL, if none.
 * @param[out] full_dir The full directory for the given sub-directory.
 * @param[in] max_length The max. length of the full directory path.
 * @param[in] create_dir If true, creates the sub-directory.
 */
static bool _get_full_dir(const char* dir, char* full_dir, size_t max_length, bool create_dir)
{
    if (dir != NULL) {
        int snprintf_result = snprintf(full_dir, max_length, "%s/%s", ROOTDIR, dir);
        if (snprintf_result < 0 || snprintf_result >= (int)max_length) {
            return false;
        }
        if (create_dir) {
            f_mkdir(full_dir);
        }
    } else {
        int snprintf_result = snprintf(full_dir, max_length, "%s", ROOTDIR);
        if (snprintf_result < 0 || snprintf_result >= (int)max_length) {
            return false;
        }
    }
    return true;
}

/**
 * Gets the absolute path for a file with a given sub-directory and filename.
 * Also creates the sub-directory if it doesn't exist yet.
 * @param[in] dir The sub-directory in which the file should reside, or NULL, if the file should be
 * stored in the root directory.
 * @param[in] fn The file name.
 * @param[out] absolute_path The absolute path for the given file.
 * @param[in] max_length The max. length of the absolute path.
 * @param[in] create_dir If true, creates the sub-directory.
 */
static bool _get_absolute_path(
    const char* dir,
    const char* fn,
    char* absolute_path,
    size_t max_length,
    bool create_dir)
{
    char full_dir[514] = {0};
    _get_full_dir(dir, full_dir, sizeof(full_dir), create_dir);
    int snprintf_result = snprintf(absolute_path, max_length, "%s/%s", full_dir, fn);
    if (snprintf_result < 0 || snprintf_result >= (int)max_length) {
        return false;
    }
    return true;
}

/**
 * Checks if an SD card is inserted and, if so, mounts it.
 * Resumes the bus clock. If mounting fails, pauses the bus clock.
 *
 * @return true if successful, false otherwise.
 */
static bool _mount(void)
{
    if (!sd_card_inserted()) {
        return false;
    }
#ifndef TESTING
    sd_mmc_resume_clock();
#endif
    memset(&fs, 0, sizeof(FATFS));
    if (f_mount(&fs, "SD", 1) == FR_INVALID_DRIVE) {
#ifndef TESTING
        sd_mmc_pause_clock();
#endif
        return false;
    }
    return true;
}

/**
 * Unmunts an SD card and pauses the bus clock.
 */
static void _unmount(void)
{
    f_mount(NULL, "SD", 1);
#ifndef TESTING
    sd_mmc_pause_clock();
#endif
}

/**
 * Opens the file with the given filename.
 * The file object is returned via outgoing parameter.
 * Expects that the filesystem is already mounted.
 *
 * @param[in] fn The file name of the file that should be opened.
 * @param[in] dir The name of the directory that should be opened.
 * @param[in] mode The file mode: FA_CREATE_ALWAYS, FA_CREATE_NEW, FA_WRITE, FA_OPEN_EXISTING,
 * FA_READ.
 * @param[out] file_object The file pointer, pointing to the opened file.
 * @return true if opening the file is OK, false otherwise.
 */
static bool _open(const char* fn, const char* dir, uint8_t mode, FIL* file_object)
{
    if (!strlens(fn)) {
        return false;
    }
    f_mkdir(ROOTDIR);
    char filename[772] = {0};
    if (!_get_absolute_path(dir, fn, filename, sizeof(filename), true)) {
        return false;
    }
    if (f_open(file_object, (const char*)filename, mode) != FR_OK) {
        return false;
    }
    return true;
}

bool sd_write_bin(
    const char* fn,
    const char* dir,
    const uint8_t* data,
    const uint16_t length,
    bool replace)
{
    if (!length || length > SD_MAX_FILE_SIZE) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    FIL file_object;
    if (!_open(
            fn, dir, (replace == 1 ? FA_CREATE_ALWAYS : FA_CREATE_NEW) | FA_WRITE, &file_object)) {
        _unmount();
        return false;
    }
    unsigned int out_length;
    FRESULT result = f_write(&file_object, data, length, &out_length);
    f_close(&file_object);
    _unmount();
    return result == FR_OK;
}

bool sd_write(const char* fn, const char* dir, const char* text, bool replace)
{
    size_t text_len = strlens(text);
    if (!text_len || text_len > SD_MAX_FILE_SIZE || !strlens(fn)) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    FIL file_object;
    if (!_open(fn, dir, replace, &file_object)) {
        _unmount();
        return false;
    }

    if (f_puts(text, &file_object) == EOF) {
        f_close(&file_object);
        _unmount();
        return false;
    }

    f_close(&file_object);
    _unmount();
    return true;
}

bool sd_load(const char* fn, const char* dir, char* text_out)
{
    if (text_out == NULL || !strlens(fn)) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    char line[SD_MAX_FILE_SIZE + 1] = {0};
    FIL file_object;
    if (!_open(fn, dir, FA_OPEN_EXISTING | FA_READ, &file_object)) {
        _unmount();
        return false;
    }
    if (f_size(&file_object) > SD_MAX_FILE_SIZE) {
        f_close(&file_object);
        _unmount();
        return false;
    }
    unsigned text_p_index = 0;
    while (f_gets(line, sizeof(line), &file_object)) {
        int snprintf_result =
            snprintf(text_out + text_p_index, SD_MAX_FILE_SIZE + 1 - text_p_index, "%s", line);
        if (snprintf_result < 0 || snprintf_result >= (int)(SD_MAX_FILE_SIZE + 1 - text_p_index)) {
            _unmount();
            return false;
        }
        text_p_index += strlens(line);
        text_out[text_p_index] = '\0';
    }

    f_close(&file_object);
    _unmount();
    return true;
}

bool sd_load_bin(const char* fn, const char* dir, uint8_t* buffer, size_t* length_out)
{
    if (buffer == NULL || length_out == NULL || !strlens(fn)) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    FIL file_object;
    if (!_open(fn, dir, FA_OPEN_EXISTING | FA_READ, &file_object)) {
        _unmount();
        return false;
    }
    if (f_size(&file_object) > SD_MAX_FILE_SIZE) {
        f_close(&file_object);
        _unmount();
        return false;
    }
    FRESULT result = f_read(&file_object, buffer, f_size(&file_object), (unsigned int*)length_out);
    f_close(&file_object);
    _unmount();
    return result == FR_OK;
}

bool sd_list_subdir(sd_list_t* list_out, const char* subdir)
{
    if (list_out == NULL) {
        return false;
    }
    list_out->num_files = 0;
    size_t allocated_files = 16;
    list_out->files = (char**)calloc(sizeof(char*), allocated_files);
    if (list_out->files == NULL) {
        Abort("Error: alloc sd_list_subdir");
    }
    FILINFO fno;
    DIR dir;
#ifdef _USE_LFN
    char c_lfn[_MAX_LFN + 1];
    fno.lfname = c_lfn;
    fno.lfsize = sizeof(c_lfn);
#endif

    if (!_mount()) {
        return false;
    }

    char full_dir[514] = {0};
    if (!_get_full_dir(subdir, full_dir, sizeof(full_dir), false)) {
        _unmount();
        return false;
    }

    FRESULT result;
    result = f_opendir(&dir, full_dir);
    if (result == FR_NO_PATH) {
        _unmount();
        return true;
    }
    if (result != FR_OK) {
        _unmount();
        return false;
    }
    for (;;) {
        char* pc_fn;
        result = f_readdir(&dir, &fno);
        if (result != FR_OK || fno.fname[0] == 0) {
            break;
        }
#ifdef _USE_LFN
        pc_fn = *fno.lfname ? fno.lfname : fno.fname;
#else
        pc_fn = fno.fname;
#endif
        if (STREQ(pc_fn, ".") || STREQ(pc_fn, "..")) {
            continue;
        }
        char* fn_copy = strdup(pc_fn);
        if (fn_copy == NULL) {
            f_closedir(&dir);
            _unmount();
            return false;
        }
        list_out->files[list_out->num_files] = fn_copy;
        list_out->num_files++;
        if (list_out->num_files >= LIST_MAX) {
            break;
        }
        if (list_out->num_files == allocated_files) {
            char** new_list_out_files;
            allocated_files *= 2;
            new_list_out_files = (char**)realloc(list_out->files, sizeof(char*) * allocated_files);
            if (new_list_out_files == NULL) {
                sd_free_list(list_out);
                Abort("Error: realloc sd_list_subdir");
            }
            list_out->files = new_list_out_files;
        }
    }
    f_closedir(&dir);
    _unmount();
    return true;
}

bool sd_list(sd_list_t* list_out)
{
    return sd_list_subdir(list_out, NULL);
}

void sd_free_list(sd_list_t* list)
{
    if (list->files == NULL) {
        return;
    }
    for (size_t i = 0; i < list->num_files; i++) {
        util_zero(list->files[i], strlen(list->files[i]));
        free(list->files[i]);
    }
    free(list->files);
    list->files = NULL;
}

bool sd_card_inserted(void)
{
#ifdef TESTING
    return true;
#else
    sd_mmc_err_t err = sd_mmc_check(0);
    sd_mmc_pause_clock();
    return (err == SD_MMC_OK || err == SD_MMC_INIT_ONGOING);
#endif
}

bool sd_file_exists_subdir(const char* fn, const char* subdir, bool* exists_out)
{
    if (!strlens(fn) || exists_out == NULL) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    char file[772] = {0};
    if (!_get_absolute_path(subdir, fn, file, sizeof(file), false)) {
        _unmount();
        return false;
    }

    FIL file_object;
    FRESULT result = f_open(&file_object, (char const*)file, FA_OPEN_EXISTING | FA_READ);
    *exists_out = result == FR_OK;
    if (*exists_out) {
        f_close(&file_object);
    }
    _unmount();
    return true;
}

bool sd_file_exists(const char* fn, bool* exists_out)
{
    return sd_file_exists_subdir(fn, NULL, exists_out);
}

/**
 * Deletes the directory in the given sub-directory if no files are in the directory.
 * Expects that the filesystem is already mounted.
 */
static bool _delete_dir(const char* dir_name)
{
    char absolute_path[772] = {0};
    if (!_get_absolute_path(NULL, dir_name, absolute_path, sizeof(absolute_path), false)) {
        return false;
    }
    FRESULT res = f_unlink(absolute_path);
    if (res != FR_OK) {
        return false;
    }
    return true;
}
/**
 * Deletes the file in the given sub-directory.
 * Expects that the filesystem is already mounted.
 */
static bool _delete_file(const char* fn, const char* subdir)
{
    char file[772] = {0};
    if (!_get_absolute_path(subdir, fn, file, sizeof(file), false)) {
        return false;
    }
    FIL file_object;

    FRESULT result = f_open(&file_object, (char const*)file, FA_OPEN_EXISTING | FA_WRITE);
    if (result != FR_OK) {
        return false;
    }
    for (DWORD f_ps = 0; f_ps < file_object.obj.objsize; f_ps++) {
        f_putc('\xAC', &file_object); // overwrite data
    }
    if (f_close(&file_object) != FR_OK) {
        return false;
    }
    if (f_unlink(file) != FR_OK) {
        return false;
    }
    return true;
}

static bool _erase_in_subdir(const char* fn, const char* subdir, bool is_dir)
{
    if ((!strlens(fn) && !is_dir) || (!strlens(subdir) && is_dir)) {
        return false;
    }
    if (!_mount()) {
        return false;
    }
    bool status = false;
    if (is_dir) {
        status = _delete_dir(subdir);
    } else {
        status = _delete_file(fn, subdir);
    }
    _unmount();
    return status;
}

bool sd_erase_file_in_subdir(const char* fn, const char* subdir)
{
    return _erase_in_subdir(fn, subdir, false);
}

bool sd_erase_file(const char* fn)
{
    return _erase_in_subdir(fn, NULL, false);
}

bool sd_erase_dir(const char* directory_name)
{
    return _erase_in_subdir(NULL, directory_name, true);
}

bool sd_file_rename(const char* from, const char* to, const char* dir)
{
    if (!strlens(from) || !strlens(to)) {
        return false;
    }

    if (!_mount()) {
        return false;
    }

    char oldfile[772] = {0};
    if (!_get_absolute_path(dir, from, oldfile, sizeof(oldfile), false)) {
        _unmount();
        return false;
    }

    char newfile[772] = {0};
    if (!_get_absolute_path(dir, to, newfile, sizeof(newfile), false)) {
        _unmount();
        return false;
    }

    FRESULT result = f_rename(oldfile, newfile);
    _unmount();
    return result == FR_OK;
}
