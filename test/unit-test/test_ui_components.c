// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>
#include <cmocka.h>

#include <touch/gestures.h>
#include <ui/components/confirm.h>
#include <ui/components/confirm_swap.h>
#include <ui/components/confirm_transaction.h>
#include <ui/components/image.h>
#include <ui/components/info_centered.h>
#include <ui/components/keyboard_switch.h>
#include <ui/components/label.h>
#include <ui/components/left_arrow.h>
#include <ui/components/right_arrow.h>
#include <ui/components/status.h>
#include <ui/components/trinary_input_char.h>
#include <ui/components/trinary_input_string.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/fonts/password_12.h>
#include <ui/fonts/password_9.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#include "fake_component.h"
#include "mock_qtouch.h"

static UG_GUI gui;
static uint8_t pixels[64][128];
static size_t vertically_clipped_pixels;

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    if (color != C_BLACK && (y < 0 || y >= 64)) {
        vertically_clipped_pixels++;
    }
    if (x >= 0 && x < 128 && y >= 0 && y < 64) {
        pixels[y][x] = color != C_BLACK;
    }
}

static int _setup(void** state)
{
    (void)state;
    UG_Init(&gui, _set_pixel, &font_arial_11, 128, 64);
    return 0;
}

static void _cb(void* user_data)
{
    (void)user_data;
}

static void _ks_cb(keyboard_mode_t mode, void* user_data)
{
    (void)mode;
    (void)user_data;
}

static void assert_ui_component_functions(component_t* component)
{
    assert_non_null(component->f->render);
    assert_non_null(component->f->cleanup);
}

static void test_ui_components_label(void** state)
{
    assert_true(label_fits_width("Test", &font_arial_11, 128));
    assert_true(label_fits_width("11111111111111111 BNB", &font_arial_9, 128));
    assert_false(label_fits_width("This label is much wider than the screen", &font_arial_9, 128));

    component_t* mock_component = fake_component_create();

    component_t* label = label_create("Test", NULL, CENTER, mock_component);
    assert_non_null(label);
    assert_ui_component_functions(label);
    label->f->cleanup(label);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_right_arrow(void** state)
{
    component_t* mock_component = fake_component_create();

    component_t* right_arrow = right_arrow_create(top_slider, mock_component, _cb, NULL);
    assert_non_null(right_arrow);
    assert_ui_component_functions(right_arrow);
    right_arrow->f->cleanup(right_arrow);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_left_arrow(void** state)
{
    component_t* mock_component = fake_component_create();

    component_t* left_arrow = left_arrow_create(top_slider, mock_component, _cb, NULL);
    assert_non_null(left_arrow);
    assert_ui_component_functions(left_arrow);
    left_arrow->f->cleanup(left_arrow);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_image(void** state)
{
    const unsigned char logo_bytes[] = {
        0x00, 0xc0, 0x3f, 0xff, 0x80, 0x00, 0x60, 0x3f, 0xff, 0xc0, 0x00, 0x78, 0x00, 0x00, 0x60,
        0x00, 0xff, 0x00, 0x00, 0x30, 0x00, 0x7f, 0x80, 0x00, 0x18, 0x00, 0xff, 0xf0, 0x00, 0x0c,
        0x00, 0x7f, 0xf8, 0x00, 0x06, 0x00, 0x7f, 0xfe, 0x00, 0x03, 0x00, 0xff, 0xff, 0xc0, 0x01,
        0x80, 0x7f, 0xff, 0xe0, 0x00, 0xc0, 0x00, 0x30, 0x00, 0x00, 0x60, 0x00, 0x18, 0x00, 0x00,
        0x30, 0x00, 0x0c, 0x00, 0x00, 0x18, 0x00, 0x06, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00, 0x00,
        0x0e, 0x00, 0x01, 0x80, 0x07, 0xff, 0xff, 0x00, 0xc0, 0x03, 0xff, 0xff, 0x80, 0x60, 0x00,
        0x7f, 0xff, 0x00, 0x30, 0x00, 0x1f, 0xff, 0x00, 0x18, 0x00, 0x07, 0xff, 0x80, 0x0c, 0x00,
        0x01, 0xff, 0x00, 0x06, 0x00, 0x00, 0xff, 0x80, 0x03, 0x00, 0x00, 0x1f, 0x00, 0x01, 0xff,
        0xfc, 0x03, 0x00, 0x00, 0xff, 0xfe, 0x01, 0x80, 0x00};

    component_t* mock_component = fake_component_create();

    component_t* image =
        image_create(logo_bytes, sizeof(logo_bytes), 41, 25, CENTER, mock_component);
    assert_non_null(image);
    assert_ui_component_functions(image);
    image->f->cleanup(image);

    mock_component->f->cleanup(mock_component);
}

static void confirm_callback(bool result, void* param)
{
    (void)param;
    (void)result;
}

static void test_ui_components_confirm(void** state)
{
    const confirm_params_t params = {
        .title = "Is the Code correct?",
        .body = "CODE",
        .font = &font_monogram_16,
    };
    component_t* confirm = confirm_create(&params, confirm_callback, NULL);
    assert_non_null(confirm);
    assert_ui_component_functions(confirm);
    confirm->f->cleanup(confirm);
}

static void test_ui_components_info_centered(void** state)
{
    component_t* info_centered = info_centered_create("Some info", NULL);
    assert_non_null(info_centered);
    assert_ui_component_functions(info_centered);
    info_centered->f->cleanup(info_centered);
}

static void test_ui_components_keyboard_switch(void** state)
{
    component_t* mock_component = fake_component_create();

    component_t* keyboard_switch =
        keyboard_switch_create(true, false, mock_component, _ks_cb, NULL);
    assert_non_null(keyboard_switch);
    assert_ui_component_functions(keyboard_switch);
    keyboard_switch->f->cleanup(keyboard_switch);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_status(void** state)
{
    component_t* status = status_create("Password created", true);
    assert_non_null(status);
    assert_ui_component_functions(status);
    status->f->cleanup(status);
}

static void _assert_vertical_bounds(const component_t* component)
{
    assert_true(component->position.top >= 0);
    assert_true(component->position.top + component->dimension.height <= 64);
    for (size_t i = 0; i < component->sub_components.amount; i++) {
        const component_t* child = component->sub_components.sub_components[i];
        _assert_vertical_bounds(child);
        if (child->dimension.height != 0) {
            assert_true(child->position.top >= component->position.top);
            assert_true(
                child->position.top + child->dimension.height <=
                component->position.top + component->dimension.height);
        }
    }
}

static void _assert_above(const component_t* top, const component_t* bottom)
{
    assert_true(top->position.top + top->dimension.height <= bottom->position.top);
}

static void test_ui_components_confirm_layout(void** state)
{
    (void)state;
    const confirm_params_t cases[] = {
        {.title = "Warning", .body = "The next value is\ntoo large to display\nin full"},
        {.title = "WARNING", .body = "Your password has\nunder 4 characters.\nContinue?"},
        {.title = "WARNING", .body = "LAST attempt!\nWrong password\nresets the device."},
        {.title = "Two\nlines", .body = "ÄÖÜ\ngj"},
        {.title = "Bitcoin\nat\nm/45'", .body = "address", .scrollable = true},
        {.title = "Sign message\ndata (hex)",
         .body = "abcdef",
         .scrollable = true,
         .display_size = 3},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(*cases); i++) {
        component_t* confirm = confirm_create(&cases[i], confirm_callback, NULL);
        _assert_vertical_bounds(confirm);
        confirm->f->cleanup(confirm);
    }
}

static void test_ui_components_transaction_layout(void** state)
{
    (void)state;
    const char* amounts[] = {"1.234 BTC", "11111111111111111 BNB"};
    for (size_t i = 0; i < sizeof(amounts) / sizeof(*amounts); i++) {
        component_t* fee =
            confirm_transaction_fee_create(amounts[i], "0.01 BTC", false, confirm_callback, NULL);
        _assert_vertical_bounds(fee);
        component_t** labels = fee->sub_components.sub_components;
        _assert_above(labels[4], labels[5]); // Total, amount
        _assert_above(labels[5], labels[2]); // amount, Fee
        _assert_above(labels[2], labels[3]); // Fee, fee value
        fee->f->cleanup(fee);

        component_t* address =
            confirm_transaction_address_create(amounts[i], "bc1qexample", confirm_callback, NULL);
        _assert_vertical_bounds(address);
        labels = address->sub_components.sub_components;
        assert_true(labels[3]->position.top + labels[3]->dimension.height <= 34);
        assert_true(labels[2]->position.top >= 40); // Below the arrow.
        address->f->cleanup(address);

        component_t* swap =
            confirm_swap_create("Swap", amounts[i], amounts[i], confirm_callback, NULL);
        _assert_vertical_bounds(swap);
        labels = swap->sub_components.sub_components;
        _assert_above(labels[2], labels[3]);
        assert_true(labels[3]->position.top + labels[3]->dimension.height <= 34);
        assert_true(labels[4]->position.top >= 40);
        swap->f->cleanup(swap);
    }
}

static void test_ui_components_keyboard_input_above_rows(void** state)
{
    (void)state;
    const trinary_input_string_params_t params = {.title = "Password", .special_chars = true};
    component_t* input = trinary_input_string_create(&params, NULL, NULL, NULL, NULL);
    component_t* keyboard = input->sub_components.sub_components[input->sub_components.amount - 1];
    const event_t event = {
        .id = EVENT_SHORT_TAP, .data = {.source = bottom_slider, .position = MAX_SLIDER_POS}};
    const char* text = "gyj";
    for (size_t i = 0; i < strlen(text); i++) {
        const char alphabet[] = {text[i], '\0'};
        trinary_input_char_set_alphabet(keyboard, alphabet, 1);
        keyboard->f->on_event(&event, keyboard);
    }
    // Exercise both font sizes, checking every animation frame for clipped descenders.
    const char* alphabets[] = {
        "abcdefghijklmnopqrstuvwxyz", "abcdefghi", "abcdefghijklmnopqrstuvwxyz"};
    for (size_t i = 0; i < sizeof(alphabets) / sizeof(*alphabets); i++) {
        trinary_input_char_set_alphabet(keyboard, alphabets[i], 1);
        for (size_t frame = 0; frame < 180; frame++) {
            memset(pixels, 0, sizeof(pixels));
            vertically_clipped_pixels = 0;
            input->f->render(input);
            assert_int_equal(vertically_clipped_pixels, 0);
        }
    }
    uint8_t rendered[64][128];
    memcpy(rendered, pixels, sizeof(rendered));
    memset(pixels, 0, sizeof(pixels));
    UG_FontSelect(&font_password_12);
    UG_S16 width, height;
    UG_MeasureStringNoBreak(&width, &height, text);
    UG_PutStringNoBreak(5, 0, text);

    const int keyboard_top = 64 - 2 * font_password_9.line_height - 1;
    bool found = false;
    for (int top = 0; top + height <= keyboard_top; top++) {
        bool matches = true;
        for (int y = 0; y < height; y++) {
            if (memcmp(&rendered[top + y][5], &pixels[y][5], width) != 0) {
                matches = false;
                break;
            }
        }
        found |= matches;
    }
    assert_true(found);
    input->f->cleanup(input);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_ui_components_label),
        cmocka_unit_test(test_ui_components_right_arrow),
        cmocka_unit_test(test_ui_components_left_arrow),
        cmocka_unit_test(test_ui_components_image),
        cmocka_unit_test(test_ui_components_info_centered),
        cmocka_unit_test(test_ui_components_keyboard_switch),
        cmocka_unit_test(test_ui_components_status),
        cmocka_unit_test(test_ui_components_confirm),
        cmocka_unit_test(test_ui_components_confirm_layout),
        cmocka_unit_test(test_ui_components_transaction_layout),
        cmocka_unit_test(test_ui_components_keyboard_input_above_rows)};

    return cmocka_run_group_tests(tests, _setup, NULL);
}
