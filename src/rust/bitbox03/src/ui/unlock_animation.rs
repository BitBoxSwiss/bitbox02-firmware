// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::cell::RefCell;
use core::task::{Poll, Waker};

use bitbox_lvgl::{self as lvgl, LottieExt, LvAlign, LvLottie, LvObj, LvOpacityLevel, ObjExt};

use super::BitBox03Ui;

const UNLOCK_ANIMATION: &[u8] = include_bytes!("../../Unlock.json");
const UNLOCK_ANIMATION_SIZE: u32 = 144;

struct SharedState {
    waker: Option<Waker>,
    result: bool,
}

pub struct UnlockAnimation {
    lottie: LvLottie,
    shared_state: Box<RefCell<SharedState>>,
}

pub struct UnlockAnimationScreen {
    pub screen: LvObj,
    pub handle: UnlockAnimation,
}

impl UnlockAnimation {
    pub async fn play<Timer>(self, ui: &mut BitBox03Ui<Timer>) {
        self.lottie.resume();

        let UnlockAnimation {
            lottie: _lottie,
            shared_state,
        } = self;

        core::future::poll_fn(move |cx| {
            let mut shared_state = shared_state.borrow_mut();
            if shared_state.result {
                Poll::Ready(())
            } else {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
        .await;

        ui.pop();
    }
}

pub(super) fn build_unlock_animation() -> UnlockAnimationScreen {
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: false,
    }));
    let shared_state_cb = shared_state.as_ref() as *const RefCell<SharedState>;

    let screen = LvObj::new().unwrap();
    screen.set_style_bg_color(lvgl::color::black(), 0);
    screen.set_style_bg_opa(LvOpacityLevel::LV_OPA_COVER as u8, 0);
    screen.set_style_text_color(lvgl::color::white(), 0);

    let lottie = LvLottie::new(&screen, UNLOCK_ANIMATION_SIZE, UNLOCK_ANIMATION_SIZE).unwrap();
    lottie.set_src_data(UNLOCK_ANIMATION);
    lottie.set_repeat_count(0);
    lottie.set_completed_cb(move || {
        let shared_state = unsafe { &*shared_state_cb };
        let mut shared_state = shared_state.borrow_mut();
        if !shared_state.result {
            shared_state.result = true;
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    });
    lottie.pause();
    lottie.align(LvAlign::LV_ALIGN_CENTER, 0, 0);

    UnlockAnimationScreen {
        screen,
        handle: UnlockAnimation {
            lottie,
            shared_state,
        },
    }
}
