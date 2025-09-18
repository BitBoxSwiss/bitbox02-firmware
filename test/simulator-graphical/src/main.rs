// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use clap::Parser;
use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, Paint, Path, renderer::OpenGl};
use image::{DynamicImage, GenericImage, Rgba, RgbaImage};

use std::collections::VecDeque;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::{
    Arc, LazyLock, Mutex,
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc,
    mpsc::TryRecvError,
};
use std::task::Poll::Ready;
use std::thread;
use std::time::Duration;

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

use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};

use bitbox02::ui::ugui::UG_COLOR;

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

pub fn handle_stream_reader(
    mut stream: TcpStream,
    inbound_in: mpsc::Sender<[u8; 64]>,
    counter: Arc<AtomicU32>,
) {
    let mut buf = [0; 1024];
    while let Ok(len) = stream.read(&mut buf) {
        if len == 0 {
            // Client disconnected
            break;
        }
        for i in 0..len / 64 {
            let mut data: [u8; 64] = [0; 64];
            //debug!("rx [{}]", hex::encode(&data[..]));
            data.copy_from_slice(&buf[64 * i..64 * (i + 1)]);
            inbound_in.send(data).unwrap();
        }
    }
    drop(inbound_in);
    info!("TCP Stream: Disconnected read");
    counter.fetch_sub(1, Ordering::SeqCst);
}

pub fn handle_stream_writer(
    mut stream: TcpStream,
    outbound_out: mpsc::Receiver<[u8; 64]>,
    counter: Arc<AtomicU32>,
) {
    for item in outbound_out.iter() {
        //debug!("tx [{}]", hex::encode(item));
        if stream.write_all(&item[..]).is_err() {
            // Client disconnected while writing
            break;
        }
    }
    drop(outbound_out);
    info!("TCP Stream: Disconnected write");
    counter.fetch_sub(1, Ordering::SeqCst);
}

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

fn mirror_fn(_: bool) {
    MIRROR.fetch_xor(true, Ordering::Relaxed);
}

static ACCEPTING_CONNECTIONS: AtomicBool = AtomicBool::new(false);

fn init_hww(preseed: bool) -> bool {
    bitbox02::screen::init(pixel_fn, mirror_fn);
    bitbox02::canvas::init();
    bitbox02::screen::splash();

    // BitBox02 simulation initialization
    bitbox02::usb_processing::init();
    info!("USB setup success");

    bitbox02::hww::setup();
    info!("HWW setup success");

    if !bitbox02::sd::format() {
        error!("ERROR, sd card setup failed");
        return false;
    }

    info!("Sd card setup: success");

    bitbox02::testing::mock_memory();
    info!("Memory setup: success");

    if preseed {
        let mnemonic = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
        let seed = bitbox02_rust::bip39::mnemonic_to_seed(&mnemonic).unwrap();
        bitbox02_rust::keystore::encrypt_and_store_seed(
            &mut bitbox02_rust::hal::BitBox02Hal::new(),
            &seed,
            "",
        )
        .unwrap();
        bitbox02::memory::set_initialized().unwrap();
    }

    bitbox02::smarteeprom::bb02_config();
    bitbox02::smarteeprom::init();

    true
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
    outbound_in: Option<mpsc::Sender<[u8; 64]>>,
    inbound_out: Option<mpsc::Receiver<[u8; 64]>>,
    orientation_task: Option<util::bb02_async::Task<'static, bool>>,
}

impl Default for App {
    fn default() -> App {
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
            outbound_in: Default::default(),
            inbound_out: Default::default(),
            orientation_task: Default::default(),
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

        canvas.save();
        canvas.reset();
        canvas.restore();

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
    NewConnection(mpsc::Sender<[u8; 64]>, mpsc::Receiver<[u8; 64]>),
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
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {
                // Read data from TCP client
                let mut inbound_out = self.inbound_out.take();
                let mut disconnected = false;
                if let Some(inbound_out) = &mut inbound_out {
                    loop {
                        match inbound_out.try_recv() {
                            Ok(data) => {
                                bitbox02::usb_packet::process(&data);
                            }
                            Err(TryRecvError::Disconnected) => {
                                // Drop the outbound channel
                                let _ = self.outbound_in.take();
                                disconnected = true;
                                break;
                            }
                            Err(TryRecvError::Empty) => {
                                break;
                            }
                        }
                    }
                }
                if !disconnected {
                    self.inbound_out = inbound_out;
                }
                // Send data to TCP Client
                loop {
                    if let Some(data) = bitbox02::queue::pull_hww() {
                        if let Some(outbound_in) = &mut self.outbound_in {
                            if outbound_in.send(data).is_err() {
                                info!("writer thread died and closed channel");
                                let _ = self.outbound_in.take();
                            }
                        }
                    } else {
                        break;
                    }
                }
                // Business logic
                unsafe { bitbox02_rust_c::workflow::rust_workflow_spin() }
                bitbox02_rust::async_usb::spin();
                bitbox02::usb_processing::process_hww();
                bitbox02::screen::process();

                if let Some(ref mut task) = self.orientation_task {
                    if let Ready(_orientation) = util::bb02_async::spin(task) {
                        ACCEPTING_CONNECTIONS.store(true, Ordering::Relaxed);
                        self.orientation_task = None;
                    }
                }

                if let Some(window) = &self.window
                    && let Some(canvas) = &mut self.canvas
                    && let Some(screen_id) = self.screen.clone()
                {
                    // TODO: We should only update texture and redraw in case screen actually changed.
                    // Update opengl texture from "screen_process"
                    {
                        let screen_fb = &*SCREEN_FB.lock().unwrap();

                        // Blit screen
                        canvas
                            .update_image(
                                screen_id,
                                ImageSource::try_from(screen_fb).unwrap(),
                                0,
                                0,
                            )
                            .unwrap();
                    }
                    {
                        let screen_fb = &mut *SCREEN_FB.lock().unwrap();

                        // Clear screen
                        if let DynamicImage::ImageRgba8(rgba) = &mut *screen_fb {
                            for pixel in rgba.pixels_mut() {
                                *pixel = Rgba([0, 0, 0, 0]);
                            }
                        }
                    }

                    window.request_redraw();
                }
            }
            UserEvent::NewConnection(outbound_in, inbound_out) => {
                self.outbound_in.replace(outbound_in);
                self.inbound_out.replace(inbound_out);
                info!("Accepted connection")
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop, None)
            .expect("failed to create initial window");
        self.orientation_task = Some(Box::pin(
            bitbox02_rust::workflow::orientation_screen::orientation_screen(),
        ));
    }
}

#[derive(Debug)]
struct AppError(String);

impl Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AppError {
    pub fn new(msg: &str) -> AppError {
        AppError(msg.into())
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pre seed the simulated bitbox with empty password and the following bip39 seed phrase "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide"
    #[arg(long)]
    preseed: bool,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Enable debug output with environment variable RUST_LOG=debug
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    if !init_hww(args.preseed) {
        return Err(Box::new(AppError::new("Failed to init hww")));
    }
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    // "Accept incoming connections" thread
    thread::spawn({
        let el_proxy = event_loop.create_proxy();
        let Ok(listener) = TcpListener::bind("0.0.0.0:15423") else {
            return Err(Box::new(AppError::new("Failed to bind to address!")));
        };
        move || {
            // Use this counter to ensure that we only connect to a single client.
            let counter = Arc::new(AtomicU32::new(0));

            for stream in listener.incoming() {
                if ACCEPTING_CONNECTIONS.load(Ordering::Relaxed)
                    && counter.compare_exchange(0, 2, Ordering::Acquire, Ordering::Relaxed) == Ok(0)
                {
                    let Ok(stream) = stream else {
                        error!("Error with stream");
                        continue;
                    };
                    let Ok(stream_clone) = stream.try_clone() else {
                        error!("Error with cloning stream");
                        continue;
                    };
                    // Channel for communicating from HWW to client
                    let (outbound_in, outbound_out) = mpsc::channel();
                    // Channel for communicating from Client to HWW
                    let (inbound_in, inbound_out) = mpsc::channel();
                    // Inform the main event loop about the new connection
                    if el_proxy
                        .send_event(UserEvent::NewConnection(outbound_in, inbound_out))
                        .is_err()
                    {
                        // Event loop has quit
                        return;
                    }
                    thread::spawn({
                        let counter = Arc::clone(&counter);
                        move || handle_stream_reader(stream_clone, inbound_in, counter)
                    });
                    thread::spawn({
                        let counter = Arc::clone(&counter);
                        move || handle_stream_writer(stream, outbound_out, counter)
                    });
                } else {
                    info!("Busy, won't accept new clients",);
                }
            }
        }
    });

    // Wake up main event loop
    thread::spawn({
        let el_proxy = event_loop.create_proxy();
        move || {
            loop {
                if el_proxy.send_event(UserEvent::WakeUp).is_err() {
                    // Event loop has quit
                    return;
                }
                std::thread::sleep(Duration::from_micros(5000));
            }
        }
    });

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
