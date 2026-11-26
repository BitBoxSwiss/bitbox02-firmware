// SPDX-License-Identifier: Apache-2.0

mod hal;

use super::Geometry;
use bitbox_lvgl::{self as lvgl, LvDisplayRenderMode};
use bitbox03::io::touchscreen::{TouchScreen, TouchScreenEvent};
use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, Paint, Path, renderer::OpenGl};
use image::{DynamicImage, Rgba, RgbaImage};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::LazyLock;
use tracing::debug;
use winit::{
    event::{ElementState, MouseButton, WindowEvent},
    window::Window,
};

pub type SimulatorHal = hal::BitBox03;
pub const NAME: &str = "BitBox03";
pub const BG: &[u8] = include_bytes!("bitbox03/bg.png");
pub const GEOMETRY: Geometry = Geometry {
    screen_width: SCREEN_WIDTH,
    screen_height: SCREEN_HEIGHT,
    padding_left: PADDING_LEFT_RIGHT,
    padding_right: PADDING_LEFT_RIGHT,
    padding_top: PADDING_TOP,
    padding_bottom: PADDING_BOTTOM,
    margin: MARGIN,
    initial_scale: 0.5,
    background: [0xf4, 0xf2, 0xee],
};

const MARGIN: usize = 20;
const PADDING_TOP: usize = 50;
const PADDING_BOTTOM: usize = 95;
const PADDING_LEFT_RIGHT: usize = 50;
// TODO put size in product crate
const SCREEN_WIDTH: usize = 480;
const SCREEN_HEIGHT: usize = 800;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
#[repr(C, align(4))]
#[derive(Debug, Copy, Clone)]
struct Argb(pub [u8; 4]);

impl Argb {
    // Will this work on big endian?
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([self.0[2], self.0[1], self.0[0], self.0[3]])
    }
}

// Simulator frame buffer
#[derive(Debug)]
struct FrameBuffer {
    /// Used with OpenGL (RGBA)
    front_buffer: DynamicImage,
    /// First leaked LVGL draw buffer (ARGB).
    buf1: NonNull<Argb>,
    /// Second leaked LVGL draw buffer (ARGB).
    buf2: NonNull<Argb>,
    /// Which buffer is active
    buf1_active: bool,
    /// Window used to request redraws after LVGL flushes.
    window: Rc<Window>,
    screen_id: ImageId,
}
impl FrameBuffer {
    pub fn new(
        canvas: &mut Canvas<OpenGl>,
        buf1: NonNull<Argb>,
        buf2: NonNull<Argb>,
        window: Rc<Window>,
    ) -> FrameBuffer {
        let front_buffer =
            DynamicImage::ImageRgba8(RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
        // Linear filtering: the screen is drawn scaled (the window opens at 50% and is freely
        // resizable), and NEAREST sampling at non-integer scales drops different source rows per
        // screen position — identical borders then render with visibly different thickness.
        let screen_id = canvas
            .create_image(
                ImageSource::try_from(&front_buffer).unwrap(),
                ImageFlags::empty(),
            )
            .unwrap();
        FrameBuffer {
            front_buffer,
            screen_id,
            buf1,
            buf2,
            buf1_active: true,
            window,
        }
    }

    pub fn present(&self, canvas: &mut Canvas<OpenGl>, x: f32, y: f32, width: f32, height: f32) {
        let mut path = Path::new();
        path.rect(x, y, width, height);
        canvas.fill_path(
            &path,
            &Paint::image(self.screen_id, x, y, width, height, 0f32, 1f32),
        );
    }
}

extern "C" fn get_current_time_ms() -> u32 {
    use std::time::Instant;
    static INIT: LazyLock<Instant> = LazyLock::new(|| Instant::now());
    INIT.elapsed().as_millis() as u32
}

// In "DIRECT" mode only the content of buf1/2 should be blitted to the front buffer.
fn my_flush_cb(display: lvgl::LvDisplay, _area: &lvgl::LvArea, _px_map: *mut u8) {
    let Some(fb_ptr) = display.get_user_data() else {
        return;
    };
    let fb = unsafe { &mut *fb_ptr.cast::<FrameBuffer>().as_ptr() };

    let buf_ptr = if fb.buf1_active {
        &mut fb.buf1
    } else {
        &mut fb.buf2
    };
    let buf = unsafe { core::slice::from_raw_parts(buf_ptr.as_ptr(), SCREEN_PIXELS) };

    if display.flush_is_last() {
        if let DynamicImage::ImageRgba8(ref mut image_buf) = fb.front_buffer {
            // Last chunk, send back buffer to front
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    let px = buf[y * SCREEN_WIDTH + x];
                    image_buf.put_pixel(x as u32, y as u32, px.to_rgba());
                }
            }
        };
        fb.buf1_active = !fb.buf1_active;
        fb.window.request_redraw();
    }
}

#[derive(Default)]
pub struct Backend {
    framebuffer: Option<NonNull<FrameBuffer>>,
    touchscreen: Option<TouchScreen>,
    touch_active: bool,
    touch_pos: Option<(i32, i32)>,
}

impl Backend {
    pub fn init_hal() -> Result<SimulatorHal, Box<dyn std::error::Error>> {
        Ok(SimulatorHal::new())
    }

    pub fn init_display(
        &mut self,
        canvas: &mut Canvas<OpenGl>,
        window: Rc<Window>,
        hal: &mut SimulatorHal,
    ) {
        lvgl::system::init();
        lvgl::log::register_print_cb(|level, buf| {
            if let Ok(s) = buf.to_str() {
                let s = s.trim();
                match level as u32 {
                    lvgl::log::LV_LOG_LEVEL_TRACE => tracing::trace!("{}", s),
                    lvgl::log::LV_LOG_LEVEL_INFO => tracing::info!("{}", s),
                    lvgl::log::LV_LOG_LEVEL_WARN => tracing::warn!("{}", s),
                    lvgl::log::LV_LOG_LEVEL_ERROR => tracing::error!("{}", s),
                    lvgl::log::LV_LOG_LEVEL_USER => tracing::info!("USER: {}", s),
                    lvgl::log::LV_LOG_LEVEL_NONE => tracing::info!("NONE: {}", s),
                    _ => tracing::error!("invalid log level!"),
                }
            }
        });
        lvgl::tick::set_cb(Some(get_current_time_ms));

        let buf1 = Box::leak(vec![Argb([0, 0, 0, 0]); SCREEN_PIXELS].into_boxed_slice());
        let buf2 = Box::leak(vec![Argb([0, 0, 0, 0]); SCREEN_PIXELS].into_boxed_slice());
        let buf1_ptr = NonNull::new(buf1.as_mut_ptr()).unwrap();
        let buf2_ptr = NonNull::new(buf2.as_mut_ptr()).unwrap();
        let disp = lvgl::LvDisplay::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .expect("create display");
        disp.set_buffers(
            buf1,
            Some(buf2),
            LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_DIRECT,
        )
        .expect("display set buffers");
        disp.set_flush_cb(my_flush_cb);

        let framebuffer = Box::leak(Box::new(FrameBuffer::new(
            canvas,
            buf1_ptr,
            buf2_ptr,
            window.clone(),
        )));
        let framebuffer_ptr = NonNull::from(&mut *framebuffer);
        disp.set_user_data(Some(framebuffer));

        hal.init(disp);

        let touchscreen = TouchScreen::new();

        self.touchscreen.replace(touchscreen);
        self.framebuffer.replace(framebuffer_ptr);
    }

    pub fn render(&mut self, canvas: &mut Canvas<OpenGl>, [x, y, width, height]: [f32; 4]) {
        let framebuffer = unsafe { self.framebuffer.unwrap().as_ref() };
        canvas
            .update_image(
                framebuffer.screen_id,
                ImageSource::try_from(&framebuffer.front_buffer).unwrap(),
                0,
                0,
            )
            .unwrap();
        let mut frame_path = Path::new();
        frame_path.rect(x, y, width, height);
        canvas.fill_path(
            &frame_path,
            &Paint::color(femtovg::Color::rgba(0x12, 0x14, 0x18, 0xff)),
        );
        framebuffer.present(canvas, x, y, width, height);
    }

    pub fn input(&mut self, window: &Window, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = GEOMETRY.window_to_logical(window, position);
                if let Some((x, y)) = screen_coord(x, y) {
                    self.touch_pos = Some((x, y));
                    if self.touch_active {
                        debug!("drag x={x}, y={y}");
                        // Stream the motion like a real touch controller does: drag-based UI
                        // (e.g. slide-to-confirm) needs continuous samples, not just the
                        // press/release end points.
                        if let Some(touchscreen) = &mut self.touchscreen {
                            touchscreen.push(TouchScreenEvent {
                                x,
                                y,
                                pressed: true,
                            });
                        }
                    }
                }
            }
            WindowEvent::CursorEntered { .. } => {
                debug!("cursor entered");
            }
            WindowEvent::CursorLeft { .. } => {
                debug!("cursor left");
                self.release_touch();
                self.touch_pos = None;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button != MouseButton::Left {
                    return;
                }

                match state {
                    ElementState::Pressed => {
                        if let Some((x, y)) = self.touch_pos
                            && let Some(touchscreen) = &mut self.touchscreen
                        {
                            touchscreen.push(TouchScreenEvent {
                                x,
                                y,
                                pressed: true,
                            });
                            debug!("pressed x={x}, y={y}");
                            self.touch_active = true;
                        }
                    }
                    ElementState::Released => self.release_touch(),
                }
            }
            _ => debug!("{event:?}"),
        }
    }

    pub fn poll(&mut self, _canvas: &mut Canvas<OpenGl>, _window: &Window) {
        lvgl::timer::handler();
    }

    fn release_touch(&mut self) {
        if !self.touch_active {
            return;
        }

        self.touch_active = false;
        if let Some((x, y)) = self.touch_pos
            && let Some(touchscreen) = &mut self.touchscreen
        {
            touchscreen.push(TouchScreenEvent {
                x,
                y,
                pressed: false,
            });
            debug!("released x={x}, y={y}");
        }
    }
}

pub fn screen_coord(x: i32, y: i32) -> Option<(i32, i32)> {
    let screen_left_boundary = (MARGIN + PADDING_LEFT_RIGHT) as i32;
    let screen_right_boundary = (MARGIN + PADDING_LEFT_RIGHT + SCREEN_WIDTH) as i32;
    if x < screen_left_boundary || x >= screen_right_boundary {
        return None;
    }
    let screen_top_boundary = (MARGIN + PADDING_TOP) as i32;
    let screen_bottom_boundary = (MARGIN + PADDING_TOP + SCREEN_HEIGHT) as i32;
    if y < screen_top_boundary || y >= screen_bottom_boundary {
        return None;
    }
    Some((x - screen_left_boundary, y - screen_top_boundary))
}
