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
#include <cmocka.h>

#include <workflow/blocking.h>

static void (*_unblock_func)(void) = NULL;
static bool _blocked = false;
void __wrap_workflow_blocking_block(void)
{
    assert_false(_blocked);
    _blocked = true;
    if (_unblock_func != NULL) {
        _unblock_func();
    }
}

void __wrap_workflow_blocking_unblock(void)
{
    assert_true(_blocked);
    _blocked = false;
}

bool mock_blocking_is_unblocked(void)
{
    return !_blocked;
}

void mock_blocking_set_unblock_func(void (*unblock_func)(void))
{
    _unblock_func = unblock_func;
}
