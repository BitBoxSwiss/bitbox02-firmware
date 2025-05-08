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
#include "sd_mmc/sd_mmc_ext.h"
#include "sd_mmc/sd_mmc_start.h"
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
    FRESULT res = f_mount(&fs, "", 1);
    if (res == FR_DISK_ERR) {
#ifndef TESTING
        sd_mmc_start();
#endif
        res = f_mount(&fs, "", 1);
    }
    if (res != FR_OK) {
#ifndef TESTING
        sd_mmc_pause_clock();
#endif
        return false;
    }
    return true;
}

/**
 * Unmounts an SD card and pauses the bus clock.
 */
static void _unmount(void)
{
    f_unmount("");
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
    if (!strlens(fn) || data == NULL || !length || length > SD_MAX_FILE_SIZE) {
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
    UINT len = 0;
    FRESULT result = f_read(&file_object, buffer, f_size(&file_object), &len);
    // UINT and size_t can have different sizes but both are at least 4 bytes.
    *length_out = (size_t)len;
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
    list_out->files = (char**)calloc(allocated_files, sizeof(char*));
    if (list_out->files == NULL) {
        Abort("Error: alloc sd_list_subdir");
    }
    FILINFO fno;
    DIR dir;
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
        result = f_readdir(&dir, &fno);
        if (result != FR_OK || fno.fname[0] == 0) {
            break;
        }
        const char* pc_fn = fno.fname;
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
            new_list_out_files =
                (char**)realloc((void*)list_out->files, sizeof(char*) * allocated_files);
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
    free((void*)list->files);
    list->files = NULL;
}

bool sd_card_inserted(void)
{
#ifdef TESTING
    return true;
#else
    sd_mmc_err_t err = sd_mmc_check(0);
    sd_mmc_pause_clock();
    /* If initialization is ongoing, wait up to 1 second for it to initialize */
    if (err == SD_MMC_INIT_ONGOING) {
        for (int i = 0; i < 10; ++i) {
            delay_ms(100);
            err = sd_mmc_check(0);
            if (err != SD_MMC_INIT_ONGOING) {
                break;
            }
        }
    }
#if !defined(NDEBUG)
    switch (err) {
    case SD_MMC_OK:
        break;
    case SD_MMC_ERR_UNUSABLE:
        util_log("sd_mmc_check returned \"SD_MMC_ERR_UNUSABLE\"");
        break;
    case SD_MMC_INIT_ONGOING:
        util_log("sd_mmc_check returned \"SD_MMC_INIT_ONGOING\" after 10 retries");
        break;
    default:
        util_log("sd_mmc_check returned %d", err);
        break;
    }
#endif
    return err == SD_MMC_OK;
#endif
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

bool sd_erase_file_in_subdir(const char* fn, const char* subdir)
{
    if (!strlens(fn)) {
        return false;
    }
    if (!_mount()) {
        return false;
    }
    bool status = _delete_file(fn, subdir);
    _unmount();
    return status;
}

#ifdef TESTING
bool sd_format(void)
{
    const MKFS_PARM params = {
        .fmt = FM_FAT32,
        // Default values for the rest.
        .n_fat = 0,
        .align = 0,
        .n_root = 0,
        .au_size = 0,
    };
    uint8_t work[FF_MAX_SS] = {0};
    return f_mkfs("SD", &params, work, sizeof(work)) == FR_OK;
}
#endif
