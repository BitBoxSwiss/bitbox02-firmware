// SPDX-License-Identifier: Apache-2.0

//! Headless renderer for the slide-to-confirm transaction screen, for visual review through
//! LVGL's real software renderer (no GPU / windowing needed). Renders on the real 480×800
//! geometry.
//!
//! ```sh
//! # Render the start state, or a mid-drag state at a given slider value (0..=304):
//! cargo run -p bitbox03 --example render_slide_confirm -- /tmp/slide.bmp [value]
//! sips -s format png /tmp/slide.bmp --out /tmp/slide.png   # macOS; or ImageMagick `convert`
//! ```

use std::cell::{Cell, RefCell};
use std::io::Write;
use std::rc::Rc;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use bitbox_hal::ui::ConfirmParams;
use bitbox_lvgl::{self as lvgl, LvArea, LvDisplay, LvDisplayRenderMode, ObjExt};
use bitbox03::ui::confirm::build_confirm_screen;
use util::futures::completion;

const WIDTH: usize = 480;
const HEIGHT: usize = 800;

extern "C" fn now_ms() -> u32 {
    static START: LazyLock<Instant> = LazyLock::new(Instant::now);
    START.elapsed().as_millis() as u32
}

fn write_bmp(path: &str, bgr: &[u8]) -> std::io::Result<()> {
    let row_bytes = WIDTH * 3; // 480*3 = 1440, already 4-byte aligned.
    let pixel_bytes = row_bytes * HEIGHT;
    let file_size = 54 + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);

    // BITMAPFILEHEADER
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // reserved
    out.extend_from_slice(&54u32.to_le_bytes()); // pixel data offset

    // BITMAPINFOHEADER
    out.extend_from_slice(&40u32.to_le_bytes()); // header size
    out.extend_from_slice(&(WIDTH as i32).to_le_bytes());
    out.extend_from_slice(&(HEIGHT as i32).to_le_bytes()); // positive => bottom-up
    out.extend_from_slice(&1u16.to_le_bytes()); // planes
    out.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel
    out.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&2835i32.to_le_bytes()); // ~72 DPI
    out.extend_from_slice(&2835i32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // colors used
    out.extend_from_slice(&0u32.to_le_bytes()); // important colors

    // Pixel data, bottom-up.
    for y in (0..HEIGHT).rev() {
        let start = y * row_bytes;
        out.extend_from_slice(&bgr[start..start + row_bytes]);
    }

    let mut file = std::fs::File::create(path)?;
    file.write_all(&out)
}

fn main() {
    let out_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "slide_preview.bmp".to_string());
    let value: Option<i32> = std::env::args().nth(2).and_then(|s| s.parse().ok());

    lvgl::system::init();
    lvgl::tick::set_cb(Some(now_ms));

    let draw_buf: &'static mut [u32] = Box::leak(vec![0u32; WIDTH * HEIGHT].into_boxed_slice());
    let display = LvDisplay::new(WIDTH as i32, HEIGHT as i32).expect("create display");
    display
        .set_buffers(
            draw_buf,
            None,
            LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_PARTIAL,
        )
        .expect("set display buffers");

    let framebuffer = Rc::new(RefCell::new(vec![0u8; WIDTH * HEIGHT * 3]));
    let flushed = Rc::new(Cell::new(false));
    {
        let framebuffer = Rc::clone(&framebuffer);
        let flushed = Rc::clone(&flushed);
        display.set_flush_cb(move |display: LvDisplay, area: &LvArea, px_map: *mut u8| {
            let area_w = (area.x2 - area.x1 + 1) as usize;
            let area_h = (area.y2 - area.y1 + 1) as usize;
            let mut fb = framebuffer.borrow_mut();
            for row in 0..area_h {
                for col in 0..area_w {
                    let src = (row * area_w + col) * 4; // ARGB8888 in memory: B, G, R, A
                    let px = area.x1 as usize + col;
                    let py = area.y1 as usize + row;
                    if px >= WIDTH || py >= HEIGHT {
                        continue;
                    }
                    let dst = (py * WIDTH + px) * 3;
                    unsafe {
                        fb[dst] = *px_map.add(src); // B
                        fb[dst + 1] = *px_map.add(src + 1); // G
                        fb[dst + 2] = *px_map.add(src + 2); // R
                    }
                }
            }
            if display.flush_is_last() {
                flushed.set(true);
            }
        });
    }

    let (responder, _result) = completion::completion();
    let body = "Total amount\n0.005 BTC\n\nFee\n0.0001 BTC";
    let screen = build_confirm_screen(
        &ConfirmParams {
            title: "Transaction",
            body,
            longtouch: true,
            ..Default::default()
        },
        responder,
    );

    if let Some(value) = value {
        // Children in longtouch mode: title, body, close button, slide component; the slide
        // component is label + track. The slider is located among the track's children by widget
        // class so this does not depend on the marker count or child order.
        let slide = screen.child(3).expect("slide component");
        let track = slide.child(1).expect("slide track");
        let slider = {
            let mut index = 0;
            loop {
                let child = track.child(index).expect("track has a slider child");
                match child.try_downcast::<bitbox_lvgl::class::SliderTag>() {
                    Ok(slider) => break slider,
                    Err(_) => index += 1,
                }
            }
        };
        slider.set_value(value, false);
        unsafe {
            lvgl::ffi::lv_obj_send_event(
                slider.as_ptr(),
                lvgl::LvEventCode::LV_EVENT_VALUE_CHANGED,
                std::ptr::null_mut(),
            );
        }
    }

    display.screen_load(screen);

    // Pump the LVGL timer until the first full frame is flushed.
    for _ in 0..2000 {
        lvgl::timer::handler();
        if flushed.get() {
            break;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(flushed.get(), "LVGL never produced a frame");

    write_bmp(&out_path, &framebuffer.borrow()).expect("write bmp");
    match value {
        Some(value) => eprintln!("wrote {out_path} (value {value})"),
        None => eprintln!("wrote {out_path} (start state)"),
    }
}
