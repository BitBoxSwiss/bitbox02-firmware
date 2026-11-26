// SPDX-License-Identifier: Apache-2.0

use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, Paint, Path, renderer::OpenGl};
use image::{DynamicImage, GenericImage, Rgba, RgbaImage};

use std::collections::VecDeque;
use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

use crate::core::{GraphicalHal, SimulatorCore, TICK_INTERVAL};

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowId};

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;

use bitbox02::ui::ugui::UG_COLOR;
use tracing::{debug, info};

static BG: &[u8; 325362] = include_bytes!("../bg.png");

const MARGIN: usize = 20;
const PADDING_TOP_BOTTOM: usize = 22;
const PADDING_LEFT: usize = 60;
const PADDING_RIGHT: usize = 35;
const SCREEN_WIDTH: usize = bitbox02::screen::SCREEN_WIDTH as usize;
const SCREEN_HEIGHT: usize = bitbox02::screen::SCREEN_HEIGHT as usize;
const WINDOW_LOGICAL_WIDTH_ORIGINAL: usize =
    SCREEN_WIDTH + 2 * MARGIN + PADDING_LEFT + PADDING_RIGHT;
const WINDOW_LOGICAL_HEIGHT_ORIGINAL: usize = SCREEN_HEIGHT + 2 * MARGIN + 2 * PADDING_TOP_BOTTOM;

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

struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
    canvas: Option<Canvas<OpenGl>>,
    bg: Option<ImageId>,
    screen: Option<ImageId>,
    slider_top: Slider,
    slider_bottom: Slider,
    pinch: bool,
    mouse_last_x: i32,
    mouse_last_y: i32,
    core: SimulatorCore<GraphicalHal>,
    error: Option<std::io::Error>,
}

impl App {
    fn new(core: SimulatorCore<GraphicalHal>) -> App {
        App {
            window: Default::default(),
            surface: Default::default(),
            gl_context: Default::default(),
            canvas: Default::default(),
            bg: Default::default(),
            screen: Default::default(),
            slider_top: Default::default(),
            slider_bottom: Default::default(),
            pinch: false,
            mouse_last_x: 0,
            mouse_last_y: 0,
            core,
            error: None,
        }
    }
}

impl App {
    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: Option<String>,
    ) -> Result<WindowId, Box<dyn Error>> {
        let width = (WINDOW_LOGICAL_WIDTH_ORIGINAL) as u32;
        let height = (WINDOW_LOGICAL_HEIGHT_ORIGINAL) as u32;
        let w_attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title("Graphical BitBox02 Simulator");

        let (window, gl_config) = {
            let template = ConfigTemplateBuilder::new();
            let (window, gl_config) = DisplayBuilder::new()
                .with_window_attributes(Some(w_attr))
                .build(event_loop, template, |mut configs| configs.next().unwrap())?;
            (Rc::new(window.unwrap()), gl_config)
        };

        let raw_window_handle = Some(window.window_handle().unwrap().as_ref().clone());

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);
        let mut not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.window_handle().unwrap().as_ref().clone();
        let attrs = SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap();

        let renderer =
            unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast()) }
                .expect("Cannot create renderer");

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
        canvas.set_size(width, height, window.scale_factor() as f32);

        let window_id = window.id();
        info!("Created window {window_id:?}");
        let bg_orig = image::load_from_memory(BG).unwrap();
        debug!("image: {} {}", bg_orig.width(), bg_orig.height());

        let bg_id = canvas
            .create_image(
                ImageSource::try_from(&bg_orig).unwrap(),
                ImageFlags::NEAREST,
            )
            .unwrap();
        let screen_id = canvas
            .create_image(
                ImageSource::try_from(&*SCREEN_FB.lock().unwrap()).unwrap(),
                ImageFlags::NEAREST,
            )
            .unwrap();

        self.window.replace(window);
        self.surface.replace(surface);
        self.gl_context.replace(gl_context);
        self.canvas.replace(canvas);
        self.bg.replace(bg_id);
        self.screen.replace(screen_id);
        Ok(window_id)
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

#[derive(Debug)]
enum UserEvent {
    WakeUp,
}

impl ApplicationHandler<UserEvent> for App {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            WindowEvent::RedrawRequested => {
                //info!("{event:?}");
                if let Some(surface) = &mut self.surface
                    && let Some(canvas) = &mut self.canvas
                    && let Some(gl_context) = &mut self.gl_context
                    && let Some(bg_id) = &mut self.bg
                    && let Some(screen_id) = &mut self.screen
                    && let Some(window) = &mut self.window
                {
                    let dpi_factor = window.scale_factor() as f32;
                    let window_size = window.inner_size();
                    let width_stretch_factor =
                        window_size.width as f32 / WINDOW_LOGICAL_WIDTH_ORIGINAL as f32;
                    let height_stretch_factor =
                        window_size.height as f32 / WINDOW_LOGICAL_HEIGHT_ORIGINAL as f32;
                    canvas.set_size(window_size.width, window_size.height, dpi_factor);

                    // fill background
                    let mut bg = Path::new();
                    bg.rect(
                        0f32,
                        0f32,
                        window_size.width as f32,
                        window_size.height as f32,
                    );
                    canvas.fill_path(
                        &bg,
                        &Paint::color(femtovg::Color::rgba(0xf5, 0xf5, 0xf5, 0xff)),
                    );

                    let bitbox_x = MARGIN as f32 * width_stretch_factor;
                    let bitbox_y = MARGIN as f32 * height_stretch_factor;
                    let bitbox_width =
                        (PADDING_LEFT + SCREEN_WIDTH + PADDING_RIGHT) as f32 * width_stretch_factor;
                    let bitbox_height =
                        (2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT) as f32 * height_stretch_factor;

                    let mut bitbox_path = Path::new();
                    bitbox_path.rect(bitbox_x, bitbox_y, bitbox_width, bitbox_height);

                    canvas.fill_path(
                        &bitbox_path,
                        &Paint::image(
                            bg_id.clone(),
                            bitbox_x,
                            bitbox_y,
                            bitbox_width,
                            bitbox_height,
                            0f32,
                            1f32,
                        ),
                    );

                    let screen_x = (MARGIN + PADDING_LEFT) as f32 * width_stretch_factor;
                    let screen_y = (MARGIN + PADDING_TOP_BOTTOM) as f32 * height_stretch_factor;
                    let screen_width = SCREEN_WIDTH as f32 * width_stretch_factor;
                    let screen_height = SCREEN_HEIGHT as f32 * height_stretch_factor;
                    let mut screen_path = Path::new();
                    screen_path.rect(screen_x, screen_y, screen_width, screen_height);
                    let paint = if MIRROR.load(Ordering::Relaxed) {
                        Paint::image(
                            screen_id.clone(),
                            screen_x + screen_width,
                            screen_y + screen_height,
                            screen_width,
                            screen_height,
                            std::f32::consts::PI,
                            1f32,
                        )
                    } else {
                        Paint::image(
                            screen_id.clone(),
                            screen_x,
                            screen_y,
                            screen_width,
                            screen_height,
                            0f32,
                            1f32,
                        )
                    };
                    canvas.fill_path(&screen_path, &paint);

                    canvas.flush_to_surface(&());
                    surface.swap_buffers(gl_context).unwrap();
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                if let Some(surface) = &mut self.surface
                    && let Some(gl_context) = &mut self.gl_context
                {
                    surface.resize(
                        gl_context,
                        width.try_into().unwrap(),
                        height.try_into().unwrap(),
                    )
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                //debug!("{position:?}");
                let Some(window) = &mut self.window else {
                    return;
                };
                let (x, y) = (position.x, position.y);
                let window_size = window.inner_size();
                let width_scale_factor = (window_size.width) as f32
                    / (2 * MARGIN + PADDING_LEFT + SCREEN_WIDTH + PADDING_RIGHT) as f32;

                let height_scale_factor = (window_size.height) as f32
                    / (2 * MARGIN + 2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT) as f32;

                let (x, y) = (
                    (x as f32 / width_scale_factor) as i32,
                    (y as f32 / height_scale_factor) as i32,
                );
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

    // Since the firmware code is very non-threadsafe all firmware code is called from user events
    // in the main thread.
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {
                match self.core.tick() {
                    Ok(true) => {}
                    result => {
                        self.error = result.err();
                        event_loop.exit();
                        return;
                    }
                }
                bitbox02::screen::process(bitbox02_rust::async_usb::is_processing_request());

                if let Some(window) = &self.window
                    && let Some(canvas) = &mut self.canvas
                    && let Some(screen_id) = self.screen.clone()
                {
                    // TODO: We should only update texture and redraw in case screen actually changed.
                    // Update opengl texture from "screen_process"
                    let screen_fb = &*SCREEN_FB.lock().unwrap();
                    canvas
                        .update_image(screen_id, ImageSource::try_from(screen_fb).unwrap(), 0, 0)
                        .unwrap();

                    window.request_redraw();
                }
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop, None)
            .expect("failed to create initial window");
    }
}

pub fn run(args: &crate::Args) -> Result<(), Box<dyn Error>> {
    bitbox02::screen::init(pixel_fn, mirror_fn, clear_fn);
    bitbox02::screen::splash();
    let mut app = App::new(SimulatorCore::new(args)?);
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let proxy = event_loop.create_proxy();
    thread::spawn(move || {
        while proxy.send_event(UserEvent::WakeUp).is_ok() {
            thread::sleep(TICK_INTERVAL);
        }
    });
    event_loop.run_app(&mut app)?;
    if let Some(error) = app.error.take() {
        return Err(error.into());
    }
    Ok(())
}
