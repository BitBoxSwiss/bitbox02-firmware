use bitbox_lvgl as lvgl;
use lvgl::{LvAlign, LvPart, ObjExt, SpinnerExt};

use tracing::info;

const SPLASH: &[u8] = include_bytes!("../../splash.png");

pub fn splash() {
    // Get the currently active screen.
    let scr = lvgl::display::screen_active().expect("get active screen");
    let color = lvgl::color::black();
    scr.set_style_bg_color(color, LvPart::LV_PART_MAIN as u32);

    let (header, mut splash) = png_decoder::decode(SPLASH).expect("valid png");
    // `png_decoder` returns RGBA, but LVGL ARGB8888 expects bytes as BGRA in memory.
    for px in splash.iter_mut() {
        px.swap(0, 2);
    }
    info!("loader {header:?}");
    let img = lvgl::LvCanvas::new(&scr, splash, header.width, header.height).expect("load png");
    img.align(LvAlign::LV_ALIGN_CENTER, 0, 0);

    let spinner = lvgl::LvSpinner::new(&scr).expect("create spinner");
    spinner.set_anim_params(3000 /* duration */, 240 /*angle*/);
    spinner.set_size(128, 128);
    spinner.align(LvAlign::LV_ALIGN_TOP_LEFT, 0, 0);

    let spinner = lvgl::LvSpinner::new(&scr).expect("create spinner");
    spinner.set_anim_params(1500 /* duration */, 240 /*angle*/);
    spinner.set_size(128, 128);
    spinner.align(LvAlign::LV_ALIGN_TOP_RIGHT, 0, 0);

    let spinner = lvgl::LvSpinner::new(&scr).expect("create spinner");
    spinner.set_anim_params(750 /* duration */, 240 /*angle*/);
    spinner.set_size(128, 128);
    spinner.align(LvAlign::LV_ALIGN_BOTTOM_LEFT, 0, 0);

    let spinner = lvgl::LvSpinner::new(&scr).expect("create spinner");
    spinner.set_anim_params(375 /* duration */, 240 /*angle*/);
    spinner.set_size(128, 128);
    spinner.align(LvAlign::LV_ALIGN_BOTTOM_RIGHT, 0, 0);
}
