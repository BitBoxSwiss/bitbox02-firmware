// SPDX-License-Identifier: Apache-2.0

//! A demo screen showing all the navigation icon buttons, for visual/interaction testing in the
//! simulator. The four nav buttons (Back / Next / Confirm / Cancel) just demonstrate their
//! press-invert feedback and take no action; the top-right corner close button dismisses the demo.

use bitbox_lvgl::{
    self as lvgl, LabelExt, LvLabel, LvLabelLongMode, LvObj, LvObjFlag, LvOpacityLevel, ObjExt,
};
use util::futures::completion::Responder;

use super::nav_button::{NavIcon, build_close_button, build_nav_button};

pub fn build_demo_screen(responder: Responder<()>) -> LvObj {
    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_row(40, 0);

    let title = LvLabel::new(&screen).unwrap();
    title.set_width(380);
    title.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title.set_text("Navigation buttons").unwrap();
    title.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title.set_style_text_font(
        lvgl::fonts::INTER_BOLD_48,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    // The four nav buttons in a row. They only demonstrate the press feedback (no callback), so the
    // demo stays open until the corner close button is tapped.
    let row = LvObj::with_parent(&screen).unwrap();
    row.set_width(380);
    row.set_height(82);
    row.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    row.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    row.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    row.set_style_flex_cross_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_CENTER, 0);
    row.set_style_pad_top(0, 0);
    row.set_style_pad_bottom(0, 0);
    row.set_style_pad_left(0, 0);
    row.set_style_pad_right(0, 0);
    row.set_style_border_width(0, 0);
    row.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    for icon in [
        NavIcon::Back,
        NavIcon::Next,
        NavIcon::Confirm,
        NavIcon::Cancel,
    ] {
        build_nav_button(&row, icon);
    }

    // A second row below: the corner-close (cancel2) glyph, for preview only. Clear its floating
    // flag so it sits in the column flow (centred) instead of the corner, and wire no action.
    build_close_button(&screen).remove_flag(LvObjFlag::LV_OBJ_FLAG_FLOATING);

    let hint = LvLabel::new(&screen).unwrap();
    hint.set_width(380);
    hint.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    hint.set_text("Tap to preview. Close with the X.").unwrap();
    hint.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    hint.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    // The corner close button is the only way out: it resolves the screen's responder.
    let close = build_close_button(&screen);
    close
        .add_click_cb(move || responder.resolve(()))
        .expect("failed to register close callback");

    screen
}
