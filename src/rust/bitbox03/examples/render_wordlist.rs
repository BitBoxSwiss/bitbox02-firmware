// SPDX-License-Identifier: Apache-2.0

//! Headless renderer for the BIP39 wordlist (recovery word) entry screen, for visual review
//! through LVGL's real software renderer (no GPU / windowing needed). Renders on the real
//! 480×800 geometry.
//!
//! The optional second argument pre-enters a word prefix (via the screen's preset mechanism), so
//! the letter filtering and confirm gating are visible:
//!
//! ```sh
//! cargo run -p bitbox03 --example render_wordlist -- /tmp/wordlist.bmp          # empty entry
//! cargo run -p bitbox03 --example render_wordlist -- /tmp/wordlist.bmp ac       # mid-word
//! cargo run -p bitbox03 --example render_wordlist -- /tmp/wordlist.bmp action   # complete word
//! sips -s format png /tmp/wordlist.bmp --out /tmp/wordlist.png   # macOS; or ImageMagick
//! ```

use std::cell::{Cell, RefCell};
use std::io::Write;
use std::rc::Rc;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use bitbox_hal::ui::{CanCancel, EnterStringParams, WordlistEntryAbort};
use bitbox_lvgl::{self as lvgl, LvArea, LvDisplay, LvDisplayRenderMode};
use bitbox03::ui::enter_string::build_wordlist_screen;

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
        .unwrap_or_else(|| "wordlist_preview.bmp".to_string());
    let preset = std::env::args().nth(2).unwrap_or_default();

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
    let flushed = Rc::new(Cell::new(0u32));
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
                flushed.set(flushed.get() + 1);
            }
        });
    }

    // The full BIP39 wordlist, as the mnemonic workflow passes it for words 1..n-1.
    let wordlist: Vec<u16> = (0..2048).collect();
    let params = EnterStringParams {
        title: "4 of 24",
        wordlist: Some(&wordlist),
        ..Default::default()
    };
    let (responder, _result) = util::futures::completion::completion::<
        Result<zeroize::Zeroizing<String>, WordlistEntryAbort>,
    >();
    let screen = build_wordlist_screen(&params, CanCancel::Yes, &preset, responder);
    display.screen_load(screen);

    let pump_until = |frames: u32, deadline_ms: u32| {
        for _ in 0..deadline_ms / 2 {
            lvgl::timer::handler();
            if flushed.get() >= frames {
                break;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
        assert!(flushed.get() >= frames, "LVGL never produced a frame");
    };
    pump_until(1, 4000);
    // Settle any style transitions kicked off by the initial disabled/enabled state, then force
    // a full redraw and capture the settled frame.
    for _ in 0..150 {
        lvgl::timer::handler();
        std::thread::sleep(Duration::from_millis(2));
    }
    unsafe { lvgl::ffi::lv_obj_invalidate(lvgl::ffi::lv_screen_active()) };
    let already = flushed.get();
    pump_until(already + 1, 4000);

    write_bmp(&out_path, &framebuffer.borrow()).expect("write bmp");
    eprintln!("wrote {out_path} (preset: {preset:?})");
}
