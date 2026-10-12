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
#include <ui/fonts/password_12.h>
#include <ui/fonts/password_9.h>
#include <ui/fonts/regular_fonts.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#include "fake_component.h"
#include "mock_qtouch.h"

static UG_GUI gui;
static uint8_t pixels[64][128];
static size_t vertically_clipped_pixels;
static size_t horizontally_clipped_pixels;

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    if (color != C_BLACK && (y < 0 || y >= 64)) {
        vertically_clipped_pixels++;
    }
    if (color != C_BLACK && (x < 0 || x >= 128)) {
        horizontally_clipped_pixels++;
    }
    if (x >= 0 && x < 128 && y >= 0 && y < 64) {
        pixels[y][x] = color != C_BLACK;
    }
}

static int _setup(void** state)
{
    (void)state;
    UG_Init(&gui, _set_pixel, &font_regular_11, 128, 64);
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
    assert_true(label_fits_width("Test", &font_regular_11, 128));
    assert_true(label_fits_width("11111111111111111 BNB", &font_regular_9, 128));
    assert_false(
        label_fits_width("This label is much wider than the screen", &font_regular_9, 128));

    component_t* mock_component = fake_component_create();

    component_t* label = label_create("Test", NULL, CENTER, mock_component);
    assert_non_null(label);
    assert_ui_component_functions(label);
    label->f->cleanup(label);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_label_update_fitting(void** state)
{
    (void)state;
    component_t* parent = fake_component_create();
    const char* long_text = "A translated label\nthat is much wider than the screen";
    component_t* label = label_create_offset("Short", NULL, CENTER_TOP, 0, 44, parent);
    assert_int_equal(label->position.top, 44);
    assert_int_equal(label->sub_components.amount, 0);

    for (size_t i = 0; i < 3; i++) {
        label_update(label, long_text);
        assert_int_equal(label->position.top, 44);
        assert_int_equal(label->dimension.height, font_regular_11.line_height);
        assert_int_equal(label->sub_components.amount, 1);
        assert_false(label->sub_components.sub_components[0]->disabled);
        label->f->render(label);
        const event_t event = {
            .id = EVENT_SLIDE, .data = {.source = bottom_slider, .velocity = -1000}};
        for (size_t frame = 0; frame < 100; frame++) {
            label->f->on_event(&event, label);
        }

        label_update(label, "A\nB");
        assert_int_equal(label->dimension.height, font_regular_11.line_height);
        assert_true(label->sub_components.sub_components[0]->disabled);

        label_update(label, "Short");
        component_t* expected = label_create_offset("Short", NULL, CENTER_TOP, 0, 44, parent);
        memset(pixels, 0, sizeof(pixels));
        label->f->render(label);
        uint8_t rendered[64][128];
        memcpy(rendered, pixels, sizeof(rendered));
        memset(pixels, 0, sizeof(pixels));
        expected->f->render(expected);
        assert_memory_equal(rendered, pixels, sizeof(pixels));
        expected->f->cleanup(expected);
    }
    label->f->cleanup(label);
    label = label_create_offset("A\nB", NULL, CENTER, 0, 20, parent);
    assert_int_equal(label->dimension.height, font_regular_11.line_height);
    assert_true(label->position.top + label->dimension.height <= 64);
    label->f->cleanup(label);
    parent->f->cleanup(parent);
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
        {.title = "Unknown\ncontract",
         .body = "You will see the raw\ntransaction data.",
         .accept_is_nextarrow = true},
        {.title = "Unknown\ncontract",
         .body = "Only proceed if you\nunderstand the data.",
         .accept_is_nextarrow = true},
        {.title = "Unknown\ncontract",
         .body = "Signing a contract\nwith large data.",
         .accept_is_nextarrow = true},
        {.title = "Unknown\ncontract",
         .body = "Only proceed if you\nunderstand the risks.",
         .accept_is_nextarrow = true},
        {.title = "Two\nlines", .body = "ÄÖÜ\ngj"},
        {.title = "Message (21/23)", .body = "field value"},
        {.title = "Bitcoin\nat\nm/45'", .body = "address"},
        {.title = "Sign message\ndata (hex)", .body = "abcdef", .display_size = 3},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(*cases); i++) {
        component_t* confirm = confirm_create(&cases[i], confirm_callback, NULL);
        _assert_vertical_bounds(confirm);
        component_t* title = confirm->sub_components.sub_components[cases[i].display_size ? 1 : 0];
        memset(pixels, 0, sizeof(pixels));
        title->f->render(title);
        for (size_t y = 0; y < font_regular_11.line_height; y++) {
            for (size_t x = 0; x < 128; x++) {
                if (x < 20 || x >= 108) {
                    assert_int_equal(pixels[y][x], 0);
                }
            }
        }
        vertically_clipped_pixels = 0;
        confirm->f->render(confirm);
        assert_int_equal(vertically_clipped_pixels, 0);
        confirm->f->cleanup(confirm);
    }
}

static void test_ui_components_confirm_scroll_hint(void** state)
{
    (void)state;
    const confirm_params_t params = {
        .title = "Bitcoin\nat\nm/45'",
        .body = "bc1qexampleaddresslongenoughtoscrollacrossthescreen",
    };
    component_t* confirm = confirm_create(&params, confirm_callback, NULL);
    _assert_vertical_bounds(confirm);
    for (size_t frame = 0; frame < 512; frame++) {
        memset(pixels, 0, sizeof(pixels));
        confirm->f->render(confirm);
        size_t hint_pixels = 0;
        for (size_t x = 0; x < 128; x++) {
            hint_pixels += pixels[63][x];
        }
        assert_int_equal(hint_pixels, 9);
    }
    confirm->f->cleanup(confirm);
}

static void test_ui_components_confirm_overflow(void** state)
{
    (void)state;
    const confirm_params_t cases[] = {
        {.title = "Warning", .body = "Unknown network\nwith chain ID:\n18446744073709551615"},
        {.title = "Cardano", .body = "Can be mined until\nslot 431999 in\nepoch 42700796467128"},
        {.title = "ADA testnet", .body = "Fee\n18446744073709.551615 TADA", .longtouch = true},
        {.title = "High fee",
         .body = "Fee is 1844674407370955161.5%\nof the send amount.\nProceed?",
         .longtouch = true},
        {.title = "Two\nlines", .body = "ÄÖÜ\ngj\nthird line"},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(*cases); i++) {
        component_t* confirm = confirm_create(&cases[i], confirm_callback, NULL);
        _assert_vertical_bounds(confirm);
        component_t* container = confirm->sub_components.sub_components[1];
        component_t* body = container->sub_components.sub_components[0];
        char flattened[256];
        assert_true(strlen(cases[i].body) < sizeof(flattened));
        strcpy(flattened, cases[i].body);
        for (char* p = flattened; *p; p++) {
            if (*p == '\n') {
                *p = ' ';
            }
        }
        UG_S16 width, height;
        UG_FontSelect(&font_regular_11);
        UG_MeasureStringNoBreak(&width, &height, flattened);
        assert_int_equal(body->dimension.width, width);
        bool scrolls = width > 128;
        assert_int_equal(body->dimension.height, height);
        for (size_t end = 0; end < 2; end++) {
            memset(pixels, 0, sizeof(pixels));
            vertically_clipped_pixels = 0;
            body->f->render(body);
            assert_int_equal(vertically_clipped_pixels, 0);
            uint8_t rendered[64][128];
            memcpy(rendered, pixels, sizeof(rendered));
            memset(pixels, 0, sizeof(pixels));
            UG_FontSelect(&font_regular_11);
            int x = scrolls ? (end ? 128 - 2 * (width / 2) - 25 : 21) : 64 - width / 2;
            UG_PutStringNoBreak(x, body->position.top, flattened);
            for (int y = body->position.top; y < body->position.top + height; y++) {
                assert_memory_equal(rendered[y], pixels[y], 128);
            }
            const event_t event = {
                .id = EVENT_SLIDE, .data = {.source = bottom_slider, .velocity = -1000}};
            for (size_t frame = 0; frame < 1000; frame++) {
                body->f->on_event(&event, body);
            }
        }
        confirm->f->cleanup(confirm);
    }
}

static void test_ui_components_status_layout(void** state)
{
    (void)state;
    const char* messages[] = {
        "Success",
        "Wrong password",
        "Recovery words\ninvalid",
        "Backup not created\nPlease contact\nsupport (SDWrite)",
        "Error\nMaxAttemptsExceeded",
        "keystore unlock failed\nSecureChip(-2147483648)",
        "Could not\nrestore backup\nSecureChip(-2147483648)",
        "Index: 2147483647",
    };
    for (size_t i = 0; i < sizeof(messages) / sizeof(*messages); i++) {
        for (size_t success = 0; success < 2; success++) {
            component_t* status = status_create(messages[i], success);
            _assert_vertical_bounds(status);
            const component_t* label = status->sub_components.sub_components[0];
            assert_true(
                label->position.left + label->dimension.width <= 100 ||
                label->position.top + label->dimension.height <= 52);
            vertically_clipped_pixels = 0;
            horizontally_clipped_pixels = 0;
            status->f->render(status);
            assert_int_equal(vertically_clipped_pixels, 0);
            if (label->sub_components.amount == 0) {
                assert_int_equal(horizontally_clipped_pixels, 0);
            } else {
                assert_false(label->sub_components.sub_components[0]->disabled);
                assert_int_equal(label->dimension.height, font_regular_9.line_height);
                for (size_t frame = 0; frame < 512; frame++) {
                    memset(pixels, 0, sizeof(pixels));
                    status->f->render(status);
                    size_t hint_pixels = 0;
                    for (size_t x = 0; x < 128; x++) {
                        hint_pixels += pixels[63][x];
                    }
                    assert_int_equal(hint_pixels, 9);
                }
            }
            status->f->cleanup(status);
        }
    }
}

static void _skip_callback(component_t* component)
{
    (void)component;
}

static void test_ui_components_u2f_prompt_layout(void** state)
{
    (void)state;
    component_t* info =
        info_centered_create("Initialize with\nBitBoxApp to use U2F", _skip_callback);
    _assert_vertical_bounds(info);
    const component_t* label = info->sub_components.sub_components[0];
    assert_true(label->position.left >= 0);
    assert_true(label->position.left + label->dimension.width <= 128);
    _assert_above(label, info->sub_components.sub_components[1]);
    info->f->cleanup(info);
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
        cmocka_unit_test(test_ui_components_label_update_fitting),
        cmocka_unit_test(test_ui_components_right_arrow),
        cmocka_unit_test(test_ui_components_left_arrow),
        cmocka_unit_test(test_ui_components_image),
        cmocka_unit_test(test_ui_components_info_centered),
        cmocka_unit_test(test_ui_components_keyboard_switch),
        cmocka_unit_test(test_ui_components_status),
        cmocka_unit_test(test_ui_components_confirm),
        cmocka_unit_test(test_ui_components_confirm_layout),
        cmocka_unit_test(test_ui_components_confirm_scroll_hint),
        cmocka_unit_test(test_ui_components_confirm_overflow),
        cmocka_unit_test(test_ui_components_status_layout),
        cmocka_unit_test(test_ui_components_u2f_prompt_layout),
        cmocka_unit_test(test_ui_components_transaction_layout),
        cmocka_unit_test(test_ui_components_keyboard_input_above_rows)};

    return cmocka_run_group_tests(tests, _setup, NULL);
}
