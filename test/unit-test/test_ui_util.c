// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <cmocka.h>

#include <ui/fonts/arial_fonts.h>
#include <ui/ui_util.h>

#include "fake_component.h"

static UG_GUI gui;
static UG_S16 last_x;
static UG_S16 last_y;
static uint8_t pixels_set;

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    (void)color;
    last_x = x;
    last_y = y;
    pixels_set++;
}

static void _reset_pixel_capture(void)
{
    last_x = 0;
    last_y = 0;
    pixels_set = 0;
}

static void _render_pixel(component_t* component)
{
    UG_DrawPixel(component->position.left + 2, component->position.top + 3, C_WHITE);
}

static const component_functions_t _pixel_component_functions = {
    .cleanup = NULL,
    .render = _render_pixel,
    .on_event = NULL,
};

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

static void test_ui_util_component_render_rotated_180(void** state)
{
    (void)state;
    UG_Init(&gui, _set_pixel, &font_font_a_11X10, 128, 64);
    component_t component = {
        .f = &_pixel_component_functions,
        .dimension = {.width = 10, .height = 8},
        .position = {.left = 5, .top = 7},
    };

    _reset_pixel_capture();
    ui_util_component_render_rotated_180(&component);

    assert_int_equal(pixels_set, 1);
    assert_int_equal(last_x, 12);
    assert_int_equal(last_y, 11);
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
        cmocka_unit_test(test_ui_util_component_render_rotated_180),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
