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

#include <setjmp.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <cmocka.h>

#include <FatFs/source/ff.h>
#include <assert_ff.h>

int f_putc(TCHAR c, FIL* fp)
{
    return 0;
}

TCHAR* f_gets(TCHAR* buff, int len, FIL* fp)
{
    return NULL;
}

FRESULT f_opendir(DIR* dp, const TCHAR* path)
{
    return FR_OK;
}

FRESULT f_closedir(DIR* dp)
{
    return FR_OK;
}

FRESULT f_readdir(DIR* dp, FILINFO* fno)
{
    return FR_OK;
}

FRESULT f_close(FIL* fp)
{
    return FR_OK;
}

FRESULT f_mkdir(const TCHAR* path)
{
    return FR_OK;
}

FRESULT f_unlink(const TCHAR* path)
{
    return FR_OK;
}

FRESULT f_open(FIL* fp, const TCHAR* path, BYTE mode)
{
    check_expected(path);
    return FR_OK;
}

void assert_will_mount_unmount(void)
{
    // mount
    expect_not_value(f_mount, fs, NULL);
    will_return(f_mount, FR_OK);
    // unmount
    expect_value(f_mount, fs, NULL);
    will_return(f_mount, FR_OK);
}

FRESULT f_mount(FATFS* fs, const TCHAR* path, BYTE opt)
{
    check_expected(fs);
    return mock();
}

int f_puts(const TCHAR* str, FIL* cp)
{
    check_expected(str);
    return (int)strlen(str);
}

FRESULT f_write(FIL* fp, const void* buff, UINT btw, UINT* bw)
{
    check_expected(buff);
    check_expected(bw);
    check_expected(btw);
    *bw = btw;
    return FR_OK;
}

FRESULT f_read(FIL* fp, void* buff, UINT btr, UINT* br)
{
    check_expected(fp);
    *br = btr;
    return FR_OK;
}

FRESULT f_rename(const TCHAR* path_old, const TCHAR* path_new)
{
    return FR_OK;
}
