// SPDX-License-Identifier: Apache-2.0

use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicBool, Ordering},
};

use bitbox_hal::{Eeprom, Hal};
use bitbox02::ui::ugui::UG_COLOR;
use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, Paint, Path, renderer::OpenGl};
use image::{DynamicImage, GenericImage, Rgba, RgbaImage};
use tracing::{debug, info};
use winit::{event::WindowEvent, window::Window};

use super::Geometry;

// Explicitly link library for its C exports
extern crate bitbox02_rust_c;

pub type SimulatorHal = bitbox02::hal::BitBox02Hal<bitbox_platform_host::timer::HostTimer>;

pub const NAME: &str = if cfg!(feature = "product-bitbox02nova") {
    "BitBox02 Nova"
} else {
    "BitBox02"
};
pub const BG: &[u8] = include_bytes!("bitbox02/bg.png");
pub const GEOMETRY: Geometry = Geometry {
    screen_width: SCREEN_WIDTH,
    screen_height: SCREEN_HEIGHT,
    padding_left: PADDING_LEFT,
    padding_right: PADDING_RIGHT,
    padding_top: PADDING_TOP_BOTTOM,
    padding_bottom: PADDING_TOP_BOTTOM,
    margin: MARGIN,
    initial_scale: 1.0,
    background: [0xf5, 0xf5, 0xf5],
};

const MARGIN: usize = 20;
const PADDING_TOP_BOTTOM: usize = 22;
const PADDING_LEFT: usize = 60;
const PADDING_RIGHT: usize = 35;
const SCREEN_WIDTH: usize = bitbox02::screen::SCREEN_WIDTH as usize;
const SCREEN_HEIGHT: usize = bitbox02::screen::SCREEN_HEIGHT as usize;
/// Screen frame buffer
static SCREEN_FB: LazyLock<Mutex<DynamicImage>> = LazyLock::new(|| {
    Mutex::new(DynamicImage::ImageRgba8(RgbaImage::new(
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
    )))
});

static MIRROR: AtomicBool = AtomicBool::new(false);

fn pixel_fn(x: i16, y: i16, c: UG_COLOR) {
    if x < 0 || x >= SCREEN_WIDTH as i16 {
        return;
    }
    if y < 0 || y >= SCREEN_HEIGHT as i16 {
        return;
    }
    let x = x as u32;
    let y = y as u32;
    let mut screen = SCREEN_FB.lock().unwrap();

    if c != 0 {
        screen.put_pixel(x, y, Rgba([0xff, 0xff, 0xff, 0xff]));
    }
}

fn clear_fn() {
    let mut screen = SCREEN_FB.lock().unwrap();
    if let DynamicImage::ImageRgba8(rgba) = &mut *screen {
        for pixel in rgba.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

fn mirror_fn(_: bool) {
    MIRROR.fetch_xor(true, Ordering::Relaxed);
}

#[derive(Debug)]
struct Slider {
    active: bool,
    position: u16,
    position_start: u16,
    sliding: bool,
    velocity_history: VecDeque<i32>,
}

impl Default for Slider {
    fn default() -> Self {
        Slider {
            active: false,
            position: 0,
            position_start: 0,
            sliding: false,
            velocity_history: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
struct SliderUpdate {
    active: bool,
    position: u16,
}

impl SliderUpdate {
    fn new(active: bool, position: u16) -> SliderUpdate {
        SliderUpdate { active, position }
    }
}

fn emit_slider_event(slider: &mut Slider, updated: SliderUpdate, slider_source: u8) {
    let diff = updated.position as i16 - slider.position as i16;
    let updated_data = bitbox02::event::event_slider_data_t {
        source: slider_source,
        diff: diff,
        position: updated.position as u16,
        velocity: slider.velocity_history.iter().sum(),
    };
    if !updated.active && slider.active {
        debug!("tap {:?}", updated_data);
        let event = bitbox02::event::event_t {
            id: bitbox02::event::event_types::EVENT_SHORT_TAP as u8,
            data: updated_data,
        };
        bitbox02::event::emit_event(&event);
        debug!("slide released {:?}", updated_data);
        slider.sliding = false;
        slider.velocity_history.clear();
        let event = bitbox02::event::event_t {
            id: bitbox02::event::event_types::EVENT_SLIDE_RELEASED as u8,
            data: updated_data,
        };
        bitbox02::event::emit_event(&event);
    }
    if updated.active && !slider.active {
        debug!("cont {:?}", updated_data);
        let event = bitbox02::event::event_t {
            data: updated_data,
            id: bitbox02::event::event_types::EVENT_CONTINUOUS_TAP as u8,
        };
        bitbox02::event::emit_event(&event);
        slider.position_start = updated.position;
    }
    if updated.active
        && slider.active
        && (i32::abs(slider.position_start as i32 - updated.position as i32) > 10 || slider.sliding)
    {
        debug!("slide {:?}", updated_data);
        slider.velocity_history.push_back(diff as i32);
        if slider.velocity_history.len() > 30 {
            slider.velocity_history.pop_front();
        }
        slider.sliding = true;
        let event = bitbox02::event::event_t {
            id: bitbox02::event::event_types::EVENT_SLIDE as u8,
            data: updated_data,
        };
        bitbox02::event::emit_event(&event);
    }
    slider.active = updated.active;
    slider.position = updated.position;
}

#[derive(Default)]
pub struct Backend {
    screen: Option<ImageId>,
    slider_top: Slider,
    slider_bottom: Slider,
    pinch: bool,
    mouse_last_x: i32,
    mouse_last_y: i32,
}

impl Backend {
    pub fn init_hal() -> Result<SimulatorHal, Box<dyn std::error::Error>> {
        bitbox02::screen::init(pixel_fn, mirror_fn, clear_fn);
        bitbox02::screen::splash();
        if !bitbox02::sd::format() {
            return Err("SD card setup failed".into());
        }
        info!("Sd card setup: success");
        bitbox02::testing::mock_memory();
        if cfg!(feature = "product-bitbox02nova") {
            assert!(bitbox02::memory::fake_nova());
        } else {
            bitbox02::memory::fake_bitbox02();
        }
        info!("Memory setup: success");
        let mut hal = SimulatorHal::new();
        hal.eeprom().setup();
        hal.eeprom().init();
        Ok(hal)
    }

    pub fn init_display(
        &mut self,
        canvas: &mut Canvas<OpenGl>,
        _window: Rc<Window>,
        _hal: &mut SimulatorHal,
    ) {
        self.screen = Some(
            canvas
                .create_image(
                    ImageSource::try_from(&*SCREEN_FB.lock().unwrap()).unwrap(),
                    ImageFlags::NEAREST,
                )
                .unwrap(),
        );
    }

    pub fn render(
        &mut self,
        canvas: &mut Canvas<OpenGl>,
        [screen_x, screen_y, screen_width, screen_height]: [f32; 4],
    ) {
        let screen_id = self.screen.unwrap();
        let mut screen_path = Path::new();
        screen_path.rect(screen_x, screen_y, screen_width, screen_height);
        let paint = if MIRROR.load(Ordering::Relaxed) {
            Paint::image(
                screen_id,
                screen_x + screen_width,
                screen_y + screen_height,
                screen_width,
                screen_height,
                std::f32::consts::PI,
                1f32,
            )
        } else {
            Paint::image(
                screen_id,
                screen_x,
                screen_y,
                screen_width,
                screen_height,
                0f32,
                1f32,
            )
        };
        canvas.fill_path(&screen_path, &paint);
    }

    pub fn input(&mut self, window: &Window, event: WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                //debug!("{position:?}");
                let (x, y) = GEOMETRY.window_to_logical(window, position);
                let xrel = x - self.mouse_last_x;
                let yrel = y - self.mouse_last_y;
                // Ignore if mouse didn't move long enough
                if xrel == 0 && yrel == 0 {
                    return;
                }
                debug!("x={x}, y={y}, xrel={xrel}, yrel={yrel}");
                self.mouse_last_x = x;
                self.mouse_last_y = y;

                let slider_pos = (x - ((MARGIN + PADDING_LEFT / 2) as i32)) * 255
                    / (SCREEN_WIDTH + PADDING_LEFT / 2 + PADDING_RIGHT) as i32;
                debug!("slider_pos = {slider_pos}");
                if slider_pos >= 0 && slider_pos <= 255 {
                    // Check top slider hit box
                    if y >= MARGIN as i32 / 2 && y < MARGIN as i32 {
                        emit_slider_event(
                            &mut self.slider_top,
                            SliderUpdate::new(true, slider_pos as u16),
                            1,
                        );
                    }
                    // Check if moved out upwards
                    if y < MARGIN as i32 / 2 && self.slider_top.active {
                        let prev_pos = self.slider_top.position;
                        emit_slider_event(
                            &mut self.slider_top,
                            SliderUpdate::new(false, prev_pos),
                            1,
                        );
                    }

                    // Check bottom slider hit box
                    if y >= (MARGIN + 2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT) as i32
                        && y < (MARGIN + 2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT + MARGIN / 2) as i32
                    {
                        emit_slider_event(
                            &mut self.slider_bottom,
                            SliderUpdate::new(true, slider_pos as u16),
                            0,
                        );
                    }
                    // Check if moved out downwards
                    if y > (MARGIN + 2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT + MARGIN / 2) as i32
                        && self.slider_bottom.active
                    {
                        let prev_pos = self.slider_bottom.position;
                        emit_slider_event(
                            &mut self.slider_bottom,
                            SliderUpdate::new(false, prev_pos),
                            0,
                        );
                    }
                }

                // Check "pinch hit box"
                if x > (MARGIN + PADDING_LEFT + SCREEN_WIDTH + PADDING_RIGHT) as i32
                    && y > (MARGIN + PADDING_TOP_BOTTOM + SCREEN_HEIGHT / 3) as i32
                    && y < (MARGIN + PADDING_TOP_BOTTOM + SCREEN_HEIGHT * 2 / 3) as i32
                {
                    if !self.pinch {
                        self.pinch = true;
                        let prev_pos = self.slider_top.position;
                        emit_slider_event(
                            &mut self.slider_top,
                            SliderUpdate::new(false, prev_pos),
                            1,
                        );
                        let prev_pos = self.slider_bottom.position;
                        emit_slider_event(
                            &mut self.slider_bottom,
                            SliderUpdate::new(false, prev_pos),
                            0,
                        );
                        emit_slider_event(&mut self.slider_top, SliderUpdate::new(true, 255), 1);
                        emit_slider_event(&mut self.slider_bottom, SliderUpdate::new(true, 255), 0);
                    }
                } else if self.pinch {
                    self.pinch = false;
                    let prev_pos = self.slider_top.position;
                    emit_slider_event(&mut self.slider_top, SliderUpdate::new(false, prev_pos), 1);
                    let prev_pos = self.slider_bottom.position;
                    emit_slider_event(
                        &mut self.slider_bottom,
                        SliderUpdate::new(false, prev_pos),
                        0,
                    );
                }
            }

            _ => debug!("{event:?}"),
        }
    }

    pub fn poll(&mut self, canvas: &mut Canvas<OpenGl>, window: &Window) {
        bitbox02::screen::process(bitbox02_rust::async_usb::is_processing_request());
        // TODO: We should only update texture and redraw in case screen actually changed.
        // Update opengl texture from "screen_process"
        let screen_fb = &*SCREEN_FB.lock().unwrap();
        canvas
            .update_image(
                self.screen.unwrap(),
                ImageSource::try_from(screen_fb).unwrap(),
                0,
                0,
            )
            .unwrap();
        window.request_redraw();
    }
}
