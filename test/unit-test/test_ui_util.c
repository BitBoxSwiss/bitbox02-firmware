// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <cmocka.h>

#include <ui/ui_util.h>

#include "fake_component.h"

static void test_ui_util_position_center(void** state)
{
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

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
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 50;

    component_t* mock_component_2 = fake_component_create();

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
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

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
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_left_bottom(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 1);
    assert_int_equal(mock_component_2->position.top, 90);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_left_top(void** state)
{
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

    mock_component_2->dimension.width = 10;
    mock_component_2->dimension.height = 10;

    ui_util_position_left_top(mock_component_1, mock_component_2);

    assert_int_equal(mock_component_2->position.left, 1);
    assert_int_equal(mock_component_2->position.top, 0);

    mock_component_1->f->cleanup(mock_component_1);
    mock_component_2->f->cleanup(mock_component_2);
}

static void test_ui_util_position_right_bottom(void** state)
{
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

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
    component_t* mock_component_1 = fake_component_create();

    mock_component_1->dimension.width = 100;
    mock_component_1->dimension.height = 100;

    mock_component_1->position.left = 0;
    mock_component_1->position.top = 0;

    component_t* mock_component_2 = fake_component_create();

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
