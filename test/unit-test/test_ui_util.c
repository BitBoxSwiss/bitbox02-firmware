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
#include <stdbool.h>
#include <stddef.h>
#include <cmocka.h>

#include <ui/ui_util.h>

#include "mock_component.h"

static void test_ui_util_position_center(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_center(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 45);
    assert_int_equal(mock_component_2->position.top, 45);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_center_top(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 50;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_center_top(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 45);
    assert_int_equal(mock_component_2->position.top, 50);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_center_bottom(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_center_bottom(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 45);
    assert_int_equal(mock_component_2->position.top, 90);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_left_bottom(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_left_bottom(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 0);
    assert_int_equal(mock_component_2->position.top, 90);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_left_top(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_left_top(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 0);
    assert_int_equal(mock_component_2->position.top, 0);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_right_bottom(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_right_bottom(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 90);
    assert_int_equal(mock_component_2->position.top, 90);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_right_top(void** state)
{
    component_t* mock_component_1 = mock_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = mock_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_right_top(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 90);
    assert_int_equal(mock_component_2->position.top, 0);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_ui_util_position_center),
        cmocka_unit_test(test_ui_util_position_center_top),
        cmocka_unit_test(test_ui_util_position_center_bottom),
        cmocka_unit_test(test_ui_util_position_left_bottom),
        cmocka_unit_test(test_ui_util_position_left_top),
        cmocka_unit_test(test_ui_util_position_right_bottom),
        cmocka_unit_test(test_ui_util_position_right_top),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
