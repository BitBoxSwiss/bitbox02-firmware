use alloc::vec;
use alloc::vec::Vec;
use bitbox_hal as hal;
use bitbox_lvgl::{
    self as lvgl, LabelExt, LvAlign, LvDisplay, LvHandle, LvLabel, LvObj, LvOpacityLevel, LvPart,
    LvSpangroup, ObjExt, SpangroupExt,
};
use tracing::info;

const LOGO: &[u8] = include_bytes!("../splash.png");

pub struct BitBox03Ui {
    display: Option<LvDisplay>,
    stack: Vec<LvHandle>,
}

pub struct BitBox03UiProgress;

pub struct BitBox03UiEmpty;

impl hal::ui::Progress for BitBox03UiProgress {
    fn set(&mut self, _progress: f32) {
        todo!()
    }
}

impl hal::ui::Empty for BitBox03UiEmpty {}

impl hal::ui::Ui for BitBox03Ui {
    type Progress = BitBox03UiProgress;

    type Empty = BitBox03UiEmpty;

    async fn confirm(
        &mut self,
        _params: &bitbox_hal::ui::ConfirmParams<'_>,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn verify_recipient(
        &mut self,
        _recipient: &str,
        _amount: &str,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn verify_total_fee(
        &mut self,
        _total: &str,
        _fee: &str,
        _longtouch: bool,
    ) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn unlock_animation(&mut self) {
        todo!()
    }

    async fn status(&mut self, _title: &str, _status_success: bool) {
        todo!()
    }

    fn print_screen(&mut self, _duration: core::time::Duration, _msg: &str) {
        todo!()
    }

    fn switch_to_logo(&mut self) {
        self.pop();
    }

    fn reset(&mut self) {
        todo!()
    }

    fn progress_create(&mut self, _title: &str) -> Self::Progress {
        todo!()
    }

    fn empty_create(&mut self) -> Self::Empty {
        todo!()
    }

    async fn enter_string(
        &mut self,
        _params: &bitbox_hal::ui::EnterStringParams<'_>,
        _can_cancel: bitbox_hal::ui::CanCancel,
        _preset: &str,
    ) -> Result<zeroize::Zeroizing<alloc::string::String>, bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn insert_sdcard(&mut self) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn menu(
        &mut self,
        _words: &[&str],
        _title: Option<&str>,
    ) -> Result<u8, bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn trinary_choice(
        &mut self,
        _message: &str,
        _label_left: Option<&str>,
        _label_middle: Option<&str>,
        _label_right: Option<&str>,
    ) -> bitbox_hal::ui::TrinaryChoice {
        todo!()
    }

    async fn show_mnemonic(&mut self, _words: &[&str]) -> Result<(), bitbox_hal::ui::UserAbort> {
        todo!()
    }

    async fn quiz_mnemonic_word(
        &mut self,
        _choices: &[&str],
        _title: &str,
    ) -> Result<u8, bitbox_hal::ui::UserAbort> {
        todo!()
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

impl BitBox03Ui {
    pub fn new() -> BitBox03Ui {
        BitBox03Ui {
            display: None,
            stack: vec![],
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
            current.delete();
        }
    }

    pub fn push(&mut self, screen: LvObj) {
        if let Some(display) = &self.display {
            let current = display.screen_active().expect("No active screen?!");
            self.stack.push(current);
            display.screen_load(screen);
        }
    }
}
