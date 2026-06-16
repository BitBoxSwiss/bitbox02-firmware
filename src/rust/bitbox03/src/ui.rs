use core::time::Duration;

use alloc::format;
use alloc::vec::Vec;
use bitbox_hal as hal;
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvDisplay, LvHandle, LvLabel, LvObj, LvOpacityLevel, LvPart,
    LvSpangroup, ObjExt, SpangroupExt,
};
use core::marker::PhantomData;
use tracing::info;
use util::futures::completion;

mod choice;
mod confirm;
mod enter_string;
mod menu;
mod status;

const LOGO: &[u8] = include_bytes!("../splash.png");

pub struct BitBox03Ui<Timer = crate::timer::BitBox03Timer> {
    display: Option<LvDisplay>,
    stack: Vec<LvHandle>,
    _timer: PhantomData<Timer>,
}

pub struct BitBox03UiProgress;

pub struct BitBox03UiEmpty;

struct ScreenGuard<'a, Timer> {
    ui: &'a mut BitBox03Ui<Timer>,
}

impl<Timer> Drop for ScreenGuard<'_, Timer> {
    fn drop(&mut self) {
        self.ui.pop();
    }
}

impl hal::ui::Progress for BitBox03UiProgress {
    fn set_fraction(&mut self, _numerator: u32, _denominator: u32) {}
}

impl hal::ui::Empty for BitBox03UiEmpty {}

impl<Timer: bitbox_hal::timer::Timer> hal::ui::Ui for BitBox03Ui<Timer> {
    type Progress = BitBox03UiProgress;

    type Empty = BitBox03UiEmpty;
    type UnlockAnimation = BitBox03UiEmpty;

    async fn confirm(
        &mut self,
        params: &bitbox_hal::ui::ConfirmParams<'_>,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        self.with_result_screen(|responder| confirm::build_confirm_screen(params, responder))
            .await
    }

    async fn confirm_swap(
        &mut self,
        title: &str,
        from: &str,
        to: &str,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        let body = format!("from\n{from}\n\nto\n{to}");
        self.confirm(&bitbox_hal::ui::ConfirmParams {
            title,
            body: &body,
            ..Default::default()
        })
        .await
    }

    async fn verify_recipient(
        &mut self,
        recipient: &str,
        amount: &str,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        let body = format!("{amount}\n\n{recipient}");
        self.confirm(&bitbox_hal::ui::ConfirmParams {
            title: "Send",
            body: &body,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await
    }

    async fn verify_total_fee(
        &mut self,
        total: &str,
        fee: &str,
        longtouch: bool,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        let body = format!("Total amount\n{total}\n\nFee\n{fee}");
        self.confirm(&bitbox_hal::ui::ConfirmParams {
            title: "Transaction",
            body: &body,
            longtouch,
            ..Default::default()
        })
        .await
    }

    async fn status(&mut self, title: &str, status_success: bool) {
        let screen = status::build_status_screen(title, status_success);
        let _screen = self.push_guard(screen);
        Timer::delay_for(Duration::from_millis(2000)).await;
    }

    fn print_screen(&mut self, _duration: core::time::Duration, _msg: &str) {
        todo!()
    }

    fn switch_to_logo(&mut self) {
        self.pop();
    }

    fn reset(&mut self) {
        while !self.stack.is_empty() {
            self.pop();
        }
    }

    fn progress_create(&mut self, _title: &str) -> Self::Progress {
        BitBox03UiProgress
    }

    fn empty_create(&mut self) -> Self::Empty {
        BitBox03UiEmpty
    }

    fn unlock_animation_create(&mut self) -> Self::UnlockAnimation {
        BitBox03UiEmpty
    }

    async fn unlock_animation_play(&mut self, _animation: Self::UnlockAnimation) {
        self.status("TODO\nunlock_animation", true).await
    }

    async fn enter_string(
        &mut self,
        params: &bitbox_hal::ui::EnterStringParams<'_>,
        can_cancel: bitbox_hal::ui::CanCancel,
        preset: &str,
    ) -> Result<zeroize::Zeroizing<alloc::string::String>, bitbox_hal::ui::UserAbort> {
        self.with_result_screen(|responder| {
            enter_string::build_enter_string_screen(params, can_cancel, preset, responder)
        })
        .await
    }

    async fn insert_sdcard(&mut self) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn menu(
        &mut self,
        words: &[&str],
        title: Option<&str>,
    ) -> Result<u8, bitbox_hal::ui::UserAbort> {
        match self.menu_impl(words, title, true, false, 0).await {
            menu::MenuResult::Selected(choice_idx) => Ok(choice_idx),
            menu::MenuResult::Cancel(_) => Err(bitbox_hal::ui::UserAbort),
            menu::MenuResult::Continue => panic!("unexpected menu continue"),
        }
    }

    async fn trinary_choice(
        &mut self,
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> bitbox_hal::ui::TrinaryChoice {
        self.with_result_screen(|responder| {
            choice::build_trinary_choice_screen(
                message,
                label_left,
                label_middle,
                label_right,
                responder,
            )
        })
        .await
    }

    async fn show_mnemonic(&mut self, words: &[&str]) -> Result<(), bitbox_hal::ui::UserAbort> {
        let mut index = 0usize;
        loop {
            match self.menu_impl(words, None, false, true, index).await {
                menu::MenuResult::Continue => return Ok(()),
                menu::MenuResult::Cancel(cancelled_index) => {
                    index = cancelled_index;
                    match menu::confirm_recovery_words_cancel(self).await {
                        Ok(()) => return Err(bitbox_hal::ui::UserAbort),
                        Err(bitbox_hal::ui::UserAbort) => {}
                    }
                }
                menu::MenuResult::Selected(_) => panic!("unexpected mnemonic word selection"),
            }
        }
    }

    async fn quiz_mnemonic_word(
        &mut self,
        choices: &[&str],
        title: &str,
    ) -> Result<u8, bitbox_hal::ui::UserAbort> {
        let mut index = 0usize;
        loop {
            match self
                .menu_impl(choices, Some(title), true, false, index)
                .await
            {
                menu::MenuResult::Selected(choice_idx) => return Ok(choice_idx),
                menu::MenuResult::Cancel(cancelled_index) => {
                    index = cancelled_index;
                    match menu::confirm_recovery_words_cancel(self).await {
                        Ok(()) => return Err(bitbox_hal::ui::UserAbort),
                        Err(bitbox_hal::ui::UserAbort) => {}
                    }
                }
                menu::MenuResult::Continue => panic!("unexpected mnemonic quiz continue"),
            }
        }
    }
}

/// Sets the bottom layer to the bitbox logo. NOTE: The bottom layer can only be seen if the active
/// layer is transparent.
fn set_background(display: &mut LvDisplay) {
    let scr = display.layer_bottom().expect("create screen");
    // By default the bottom layer is transparent, make it opaque
    scr.set_style_bg_opa(
        LvOpacityLevel::LV_OPA_COVER as u8,
        LvPart::LV_PART_MAIN as u32,
    );
    scr.set_style_bg_color(lvgl::color::black(), LvPart::LV_PART_MAIN as u32);

    let (header, mut logo) = png_decoder::decode(LOGO).expect("valid png");
    // `png_decoder` returns RGBA, but LVGL ARGB8888 expects bytes as BGRA in memory.
    for px in logo.iter_mut() {
        px.swap(0, 2);
    }
    info!("loader {header:?}");
    let img = lvgl::LvCanvas::new(&scr, logo, header.width, header.height).expect("load png");
    img.align(LvAlign::LV_ALIGN_CENTER, 0, 0);
}

impl<Timer> BitBox03Ui<Timer> {
    pub const fn new() -> BitBox03Ui<Timer> {
        BitBox03Ui {
            display: None,
            stack: Vec::new(),
            _timer: PhantomData,
        }
    }
    pub fn init(&mut self, mut display: LvDisplay) {
        // Make background of default active screen transparent so that bottom layer is visible
        display
            .screen_active()
            .expect("get active screen")
            .set_style_bg_opa(
                LvOpacityLevel::LV_OPA_TRANSP as u8,
                LvPart::LV_PART_MAIN as u32,
            );
        set_background(&mut display);
        self.display.replace(display);

        let screen = LvObj::new().unwrap();
        screen.set_layout(lvgl::LvLayout::LV_LAYOUT_FLEX);
        screen.set_flex_flow(lvgl::LvFlexFlow::LV_FLEX_FLOW_COLUMN);
        screen.set_style_bg_color(lvgl::color::black(), 0);
        screen.set_style_text_color(lvgl::color::white(), 0);
        screen.set_style_pad_top(40, 0);
        screen.set_style_pad_left(50, 0);

        let label = LvLabel::new(&screen).unwrap();
        label.set_text("Welcome!").unwrap();
        label.set_style_margin_bottom(60, 0);
        label.set_style_text_font(
            lvgl::fonts::INTER_REGULAR_48,
            lvgl::LvState::LV_STATE_DEFAULT as u32,
        );

        let spangroup = LvSpangroup::new(&screen).unwrap();
        spangroup.set_style_margin_bottom(60, 0);
        spangroup.set_width(380);
        spangroup.set_height(lvgl::ffi::LV_SIZE_CONTENT as i32);

        let span0 = spangroup.add_span().unwrap();
        span0
            .set_text("Let's get started\nsetting up your ")
            .unwrap();
        span0.set_style_text_font(lvgl::fonts::INTER_REGULAR_48);
        let span1 = spangroup.add_span().unwrap();
        span1.set_text("BitBox").unwrap();
        span1.set_style_text_font(lvgl::fonts::INTER_BOLD_48);
        let span2 = spangroup.add_span().unwrap();
        span2.set_text("03").unwrap();
        span2.set_style_text_font(lvgl::fonts::INTER_REGULAR_48);

        let label = LvLabel::new(&screen).unwrap();
        label.set_text("Download and open\nthe BitBoxApp.").unwrap();
        label.set_style_margin_bottom(60, 0);
        label.set_style_text_font(
            lvgl::fonts::INTER_REGULAR_32,
            lvgl::LvState::LV_STATE_DEFAULT as u32,
        );

        let label = LvLabel::new(&screen).unwrap();
        label.set_text("bitbox.swiss/start").unwrap();
        label.set_style_text_font(
            lvgl::fonts::INTER_BOLD_32,
            lvgl::LvState::LV_STATE_DEFAULT as u32,
        );
        self.push(screen);
    }

    pub fn pop(&mut self) {
        if let Some(display) = &self.display
            && let Some(screen) = self.stack.pop()
        {
            let current = display.screen_active().expect("no active screen?!");
            display.screen_load(screen);
            unsafe { current.delete() };
        }
    }

    fn push_guard(&mut self, screen: LvObj) -> ScreenGuard<'_, Timer> {
        self.push(screen);
        ScreenGuard { ui: self }
    }

    async fn with_result_screen<R, F>(&mut self, build_screen: F) -> R
    where
        F: FnOnce(completion::Responder<R>) -> LvObj,
    {
        let (responder, result) = completion::completion();
        let screen = build_screen(responder);
        let _screen = self.push_guard(screen);
        result.await
    }

    pub fn push(&mut self, screen: LvObj) {
        if let Some(display) = &self.display {
            let current = display.screen_active().expect("No active screen?!");
            self.stack.push(current);
            display.screen_load(screen);
        }
    }
}

impl<Timer: bitbox_hal::timer::Timer> BitBox03Ui<Timer> {
    async fn menu_impl(
        &mut self,
        words: &[&str],
        title: Option<&str>,
        select_word: bool,
        continue_on_last: bool,
        start_index: usize,
    ) -> menu::MenuResult {
        assert!(!words.is_empty(), "menu requires at least one word");
        let mut index = start_index.min(words.len() - 1);
        loop {
            let action = self
                .with_result_screen(|responder| {
                    menu::build_menu_screen(
                        words,
                        title,
                        index,
                        select_word,
                        continue_on_last,
                        responder,
                    )
                })
                .await;
            match action {
                menu::MenuAction::Previous => index = index.saturating_sub(1),
                menu::MenuAction::Next => {
                    if index + 1 < words.len() {
                        index += 1;
                    }
                }
                menu::MenuAction::Select => {
                    return menu::MenuResult::Selected(
                        index.try_into().expect("menu supports at most 256 items"),
                    );
                }
                menu::MenuAction::Continue => return menu::MenuResult::Continue,
                menu::MenuAction::Cancel => return menu::MenuResult::Cancel(index),
            }
        }
    }
}
