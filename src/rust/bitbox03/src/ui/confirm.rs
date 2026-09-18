// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use bitbox_hal::ui::{ConfirmParams, MAX_CONFIRM_BODY_SIZE, UserAbort};
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvLabel, LvLabelLongMode, LvObj, LvObjFlag, LvOpacityLevel,
    LvZeroizingLabel, ObjExt,
};
use util::futures::completion::Responder;
use zeroize::Zeroizing;

use super::nav_button::{NavIcon, build_close_button, build_nav_button};
use super::slide_to_confirm::build_slide_to_confirm;

fn truncate_body(body: &str) -> Zeroizing<Vec<u8>> {
    if body.len() <= MAX_CONFIRM_BODY_SIZE {
        return Zeroizing::new(body.as_bytes().to_vec());
    }

    let mut end = MAX_CONFIRM_BODY_SIZE;
    while !body.is_char_boundary(end) {
        end -= 1;
    }
    // Reserve the ellipsis too: growing a secret buffer could leave an unwiped allocation.
    let mut truncated = Zeroizing::new(Vec::with_capacity(end + 3));
    truncated.extend_from_slice(&body.as_bytes()[..end]);
    truncated.extend_from_slice(b"...");
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
    // Same bottom padding as the entry screens, so the navigation buttons sit at the same
    // height across a workflow's screens.
    screen.set_style_pad_bottom(32, 0);
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

    let body_text = truncate_body(params.body);
    // Confirmation bodies can contain passphrases. Keep the text in a fixed zeroizing buffer
    // borrowed by LVGL, so deleting the screen also wipes it when confirmation is cancelled.
    let body = LvZeroizingLabel::new(&body_container, body_text.len()).unwrap();
    body.set_width(380);
    // Labels wrap by default; retain that mode for scrolling long confirmation contents.
    body.set_text(core::str::from_utf8(body_text.as_slice()).unwrap())
        .unwrap();
    drop(body_text);
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
    use super::super::{BitBox03Ui, test_util};
    use super::*;
    use alloc::{boxed::Box, rc::Rc, string::String};
    use core::cell::{Cell, RefCell};
    use core::future::Future;
    use core::task::{Context, Poll, Waker};
    use lvgl::{LvDisplay, LvEventCode, class};

    #[test]
    fn test_truncate_body() {
        assert!(truncate_body("").is_empty());
        assert_eq!(truncate_body("passphrase").as_slice(), b"passphrase");
        let exact = "a".repeat(MAX_CONFIRM_BODY_SIZE);
        assert_eq!(truncate_body(&exact).as_slice(), exact.as_bytes());

        let overlong = "a".repeat(MAX_CONFIRM_BODY_SIZE + 1);
        let mut expected = String::from(&overlong[..MAX_CONFIRM_BODY_SIZE]);
        expected.push_str("...");
        assert_eq!(truncate_body(&overlong).as_slice(), expected.as_bytes());

        let mut utf8 = "a".repeat(MAX_CONFIRM_BODY_SIZE - 1);
        utf8.push('€');
        let truncated = truncate_body(&utf8);
        assert_eq!(truncated.len(), MAX_CONFIRM_BODY_SIZE + 2);
        assert_eq!(
            truncated.as_slice(),
            [&utf8.as_bytes()[..MAX_CONFIRM_BODY_SIZE - 1], b"..."].concat()
        );
        assert!(core::str::from_utf8(truncated.as_slice()).is_ok());
    }

    #[async_test::test]
    async fn test_build_confirm_screen_teardown() {
        let _lock = test_util::lock_and_init();
        let mut ui = BitBox03Ui::<()>::new();
        ui.display = Some(LvDisplay::new(480, 800).unwrap());

        for longtouch in [false, true] {
            for outcome in [Some(Ok(())), Some(Err(UserAbort)), None] {
                let cleared = Rc::new(Cell::new(false));
                let responder = RefCell::new(None);
                let params = ConfirmParams {
                    title: "Confirm passphrase",
                    body: "secret passphrase",
                    longtouch,
                    ..Default::default()
                };
                let mut result = Box::pin(ui.with_result_screen(|screen_responder| {
                    responder.replace(Some(screen_responder.clone()));
                    let screen = build_confirm_screen(&params, screen_responder);
                    let container = screen.child(1).unwrap();
                    let body = container
                        .child(0)
                        .unwrap()
                        .try_downcast::<class::LabelTag>()
                        .unwrap();
                    assert_eq!(body.get_text().unwrap().to_str().unwrap(), params.body);
                    assert_eq!(
                        body.get_long_mode(),
                        LvLabelLongMode::LV_LABEL_LONG_MODE_WRAP
                    );
                    let body_on_delete = container
                        .child(0)
                        .unwrap()
                        .try_downcast::<class::LabelTag>()
                        .unwrap();
                    let cleared = Rc::clone(&cleared);
                    body.add_event_cb(LvEventCode::LV_EVENT_DELETE, move || {
                        // The zeroizing label detaches its buffer before later delete callbacks.
                        assert!(body_on_delete.get_text().unwrap().to_bytes().is_empty());
                        cleared.set(true);
                    })
                    .unwrap();
                    screen
                }));
                assert!(
                    result
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                assert!(!cleared.get());
                if let Some(outcome) = outcome {
                    let accepted = outcome.is_ok();
                    responder.borrow().as_ref().unwrap().resolve(outcome);
                    // Completion is ready: poll without suspending while holding the LVGL lock.
                    assert_eq!(
                        result
                            .as_mut()
                            .poll(&mut Context::from_waker(Waker::noop()))
                            .map(|outcome| outcome.is_ok()),
                        Poll::Ready(accepted)
                    );
                }
                drop(result);
                assert!(cleared.get());
                assert!(ui.stack.is_empty());
            }
        }
    }
}
