// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use bitbox_hal::ui::{ConfirmParams, MAX_CONFIRM_BODY_SIZE, UserAbort};
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvLabel, LvLabelLongMode, LvObj, LvObjFlag, LvOpacityLevel, ObjExt,
};
use util::futures::completion::Responder;

use super::nav_button::{NavIcon, build_close_button, build_nav_button};
use super::slide_to_confirm::build_slide_to_confirm;

fn truncate_body(body: &str) -> String {
    if body.len() <= MAX_CONFIRM_BODY_SIZE {
        return String::from(body);
    }

    let mut end = MAX_CONFIRM_BODY_SIZE;
    while !body.is_char_boundary(end) {
        end -= 1;
    }
    let mut truncated = String::from(&body[..end]);
    truncated.push_str("...");
    truncated
}

pub fn build_confirm_screen(
    params: &ConfirmParams<'_>,
    responder: Responder<Result<(), UserAbort>>,
) -> LvObj {
    let screen = LvObj::new().unwrap();
    screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_text_color(lvgl::color::white(), 0);
    screen.set_style_pad_top(40, 0);
    screen.set_style_pad_right(50, 0);
    screen.set_style_pad_bottom(40, 0);
    screen.set_style_pad_left(50, 0);
    screen.set_style_pad_row(24, 0);

    let title = LvLabel::new(&screen).unwrap();
    title.set_width(380);
    title.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    title.set_text(params.title).unwrap();
    title.set_style_text_align(lvgl::LvTextAlign::LV_TEXT_ALIGN_CENTER, 0);
    title.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    // Keep navigation controls fixed while allowing long confirmation contents to be reviewed.
    let body_container = LvObj::with_parent(&screen).unwrap();
    body_container.set_width(380);
    body_container.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    body_container.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
    body_container.set_style_flex_grow(1, 0);
    body_container.set_style_pad_top(0, 0);
    body_container.set_style_pad_right(0, 0);
    body_container.set_style_pad_bottom(0, 0);
    body_container.set_style_pad_left(0, 0);
    body_container.set_style_border_width(0, 0);
    body_container.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);
    body_container.add_flag(LvObjFlag::LV_OBJ_FLAG_SCROLLABLE);

    let body = LvLabel::new(&body_container).unwrap();
    body.set_width(380);
    body.set_long_mode(LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP);
    body.set_text(&truncate_body(params.body)).unwrap();
    body.set_style_text_font(
        lvgl::fonts::INTER_REGULAR_32,
        lvgl::LvState::LV_STATE_DEFAULT as u32,
    );

    if params.longtouch {
        // High-stakes confirmation: the accept action is a slide gesture instead of a tap, and
        // cancel moves to the corner close button (the slide track occupies the bottom row).
        if !params.accept_only {
            let reject_responder = responder.clone();
            let close = build_close_button(&screen);
            close
                .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
                .expect("failed to register reject callback");
        }
        let slide = build_slide_to_confirm(&screen, move || responder.resolve(Ok(())));
        slide.set_style_margin_top(16, 0);
        return screen;
    }

    let actions = LvObj::with_parent(&screen).unwrap();
    actions.set_width(380);
    actions.set_height(82);
    actions.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
    actions.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_ROW);
    actions.set_style_flex_main_place(lvgl::LvFlexAlign::LV_FLEX_ALIGN_SPACE_BETWEEN, 0);
    actions.set_style_pad_top(0, 0);
    actions.set_style_pad_bottom(0, 0);
    actions.set_style_pad_left(0, 0);
    actions.set_style_pad_right(0, 0);
    actions.set_style_margin_top(16, 0);
    actions.set_style_border_width(0, 0);
    actions.set_style_bg_opa(LvOpacityLevel::LV_OPA_TRANSP as u8, 0);

    let reject_responder = responder.clone();
    let reject = build_nav_button(&actions, NavIcon::Cancel);
    reject
        .add_click_cb(move || reject_responder.resolve(Err(UserAbort)))
        .expect("failed to register reject callback");

    let accept = build_nav_button(&actions, NavIcon::Confirm);
    accept
        .add_click_cb(move || responder.resolve(Ok(())))
        .expect("failed to register accept callback");

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_body() {
        let exact = "a".repeat(MAX_CONFIRM_BODY_SIZE);
        assert_eq!(truncate_body(&exact), exact);

        let overlong = "a".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let mut expected = String::from(&overlong[..MAX_CONFIRM_BODY_SIZE]);
        expected.push_str("...");
        assert_eq!(truncate_body(&overlong), expected);

        let mut utf8 = "a".repeat(MAX_CONFIRM_BODY_SIZE - 1);
        utf8.push('€');
        let truncated = truncate_body(&utf8);
        assert_eq!(truncated.len(), MAX_CONFIRM_BODY_SIZE + 2);
        assert!(truncated.ends_with("..."));
    }
}
