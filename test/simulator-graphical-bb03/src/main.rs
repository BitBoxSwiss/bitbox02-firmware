// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, Paint, Path, renderer::OpenGl};
use image::{DynamicImage, Rgba, RgbaImage};

use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroU32;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc,
    mpsc::TryRecvError,
};
use std::task::Poll::Ready;
use std::thread;
use std::time::Duration;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
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

use bitbox_hal::{Hal, Ui};

use bitbox03::BitBox03;
use bitbox03::io::touchscreen::{TouchScreen, TouchScreenEvent};

// Explicitly link library for its C exports
extern crate bitbox02_rust_c;
use bitbox_lvgl as lvgl;
use lvgl::LvDisplayRenderMode;

const UI_REFRESH_PERIOD_MS: u64 = 5;

static BG: &[u8] = include_bytes!("../bg.png");

const MARGIN: usize = 20;
const PADDING_TOP: usize = 50;
const PADDING_BOTTOM: usize = 95;
const PADDING_LEFT_RIGHT: usize = 50;
// TODO put size in product crate
const SCREEN_WIDTH: usize = 480;
const SCREEN_HEIGHT: usize = 800;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const WINDOW_LOGICAL_WIDTH_ORIGINAL: usize = SCREEN_WIDTH + 2 * MARGIN + 2 * PADDING_LEFT_RIGHT;
const WINDOW_LOGICAL_HEIGHT_ORIGINAL: usize =
    SCREEN_HEIGHT + 2 * MARGIN + PADDING_TOP + PADDING_BOTTOM;

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
        let screen_id = canvas
            .create_image(
                ImageSource::try_from(&front_buffer).unwrap(),
                ImageFlags::NEAREST,
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

static ACCEPTING_CONNECTIONS: AtomicBool = AtomicBool::new(false);

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

fn init_hww(_bitbox: &mut BitBox03, preseed: bool) -> bool {
    // BitBox02 simulation initialization
    //bitbox02::usb_processing::init();
    info!("USB setup success");

    //bitbox02::hww::setup();
    info!("HWW setup success");

    //if !bitbox02::sd::format() {
    //    error!("ERROR, sd card setup failed");
    //    return false;
    //}

    info!("Sd card setup: success");

    //bitbox02::testing::mock_memory();
    //bitbox02::memory::fake_nova();
    info!("Memory setup: success");

    if preseed {
        //let mnemonic = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
        //let seed = bitbox02_rust::bip39::mnemonic_to_seed(&mnemonic).unwrap();
        //let mut hal = bitbox02::hal::BitBox02Hal::take().unwrap();
        //bitbox02_rust::keystore::encrypt_and_store_seed(&mut hal, &seed, "").unwrap();
        //bitbox.memory().set_initialized().unwrap();
    }

    //bitbox02::smarteeprom::bb02_config();
    //bitbox02::smarteeprom::init();

    true
}

struct App {
    bitbox: BitBox03,
    framebuffer: Option<NonNull<FrameBuffer>>,
    touchscreen: Option<TouchScreen>,
    window: Option<Rc<Window>>,
    surface: Option<Surface<WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
    canvas: Option<Canvas<OpenGl>>,
    bg: Option<ImageId>,
    touch_active: bool,
    touch_pos: Option<(i32, i32)>,
    outbound_in: Option<mpsc::Sender<[u8; 64]>>,
    inbound_out: Option<mpsc::Receiver<[u8; 64]>>,
    startup_task: Option<util::bb02_async::Task<'static, ()>>,
    counter: usize,
}

impl App {
    fn new(bitbox: BitBox03) -> App {
        App {
            bitbox,
            framebuffer: Default::default(),
            touchscreen: Default::default(),
            window: Default::default(),
            surface: Default::default(),
            gl_context: Default::default(),
            canvas: Default::default(),
            bg: Default::default(),
            touch_active: false,
            touch_pos: None,
            outbound_in: Default::default(),
            inbound_out: Default::default(),
            startup_task: Default::default(),
            counter: 0,
        }
    }
}

fn window_to_logical(window: &Window, position: PhysicalPosition<f64>) -> (i32, i32) {
    let window_size = window.inner_size();
    let width_scale_factor = window_size.width as f32 / WINDOW_LOGICAL_WIDTH_ORIGINAL as f32;
    let height_scale_factor = window_size.height as f32 / WINDOW_LOGICAL_HEIGHT_ORIGINAL as f32;
    (
        (position.x as f32 / width_scale_factor) as i32,
        (position.y as f32 / height_scale_factor) as i32,
    )
}

impl App {
    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: Option<String>,
    ) -> Result<WindowId, Box<dyn Error>> {
        lvgl::system::init();
        lvgl::log::register_print_cb(|level, buf| {
            if let Ok(s) = buf.to_str() {
                let s = s.trim();
                match level as u32 {
                    lvgl::ffi::LV_LOG_LEVEL_TRACE => tracing::trace!("{}", s),
                    lvgl::ffi::LV_LOG_LEVEL_INFO => tracing::info!("{}", s),
                    lvgl::ffi::LV_LOG_LEVEL_WARN => tracing::warn!("{}", s),
                    lvgl::ffi::LV_LOG_LEVEL_ERROR => tracing::error!("{}", s),
                    lvgl::ffi::LV_LOG_LEVEL_USER => tracing::info!("USER: {}", s),
                    lvgl::ffi::LV_LOG_LEVEL_NONE => tracing::info!("NONE: {}", s),
                    _ => tracing::error!("invalid log level!"),
                }
            }
        });
        lvgl::tick::set_cb(Some(get_current_time_ms));

        let buf1: &'static mut [Argb; SCREEN_PIXELS] =
            Box::leak(Box::new([Argb([0, 0, 0, 0]); SCREEN_PIXELS]));
        let buf2: &'static mut [Argb; SCREEN_PIXELS] =
            Box::leak(Box::new([Argb([0, 0, 0, 0]); SCREEN_PIXELS]));
        let buf1_ptr = NonNull::new(buf1.as_mut_ptr()).unwrap();
        let buf2_ptr = NonNull::new(buf2.as_mut_ptr()).unwrap();
        let disp = lvgl::LvDisplay::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .expect("create display");
        disp.set_buffers(
            buf1.as_mut_slice(),
            Some(buf2.as_mut_slice()),
            LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_DIRECT,
        )
        .expect("display set buffers");
        disp.set_flush_cb(my_flush_cb);
        //lv_display_set_color_format(&disp, bitbox_lvgl::LvColorFormat::LV_COLOR_FORMAT_RGB888);

        let width = WINDOW_LOGICAL_WIDTH_ORIGINAL as u32;
        let height = WINDOW_LOGICAL_HEIGHT_ORIGINAL as u32;
        let w_attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(width / 2, height / 2))
            .with_title("Graphical BitBox03 Simulator");

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

        let framebuffer = Box::leak(Box::new(FrameBuffer::new(
            &mut canvas,
            buf1_ptr,
            buf2_ptr,
            window.clone(),
        )));
        let framebuffer_ptr = NonNull::from(&mut *framebuffer);
        disp.set_user_data(Some(framebuffer));

        self.bitbox.init(disp);

        let touchscreen = TouchScreen::new();

        self.touchscreen.replace(touchscreen);
        self.framebuffer.replace(framebuffer_ptr);
        self.window.replace(window);
        self.surface.replace(surface);
        self.gl_context.replace(gl_context);
        self.canvas.replace(canvas);
        self.bg.replace(bg_id);
        info!("window created");
        Ok(window_id)
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

#[derive(Debug)]
enum UserEvent {
    WakeUp,
    NewConnection(mpsc::Sender<[u8; 64]>, mpsc::Receiver<[u8; 64]>),
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

impl ApplicationHandler<UserEvent> for App {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface
                    && let Some(canvas) = &mut self.canvas
                    && let Some(gl_context) = &mut self.gl_context
                    && let Some(bg_id) = &mut self.bg
                    && let Some(window) = &mut self.window
                    && let Some(mut framebuffer_ptr) = self.framebuffer
                {
                    let framebuffer = unsafe { framebuffer_ptr.as_mut() };
                    canvas
                        .update_image(
                            framebuffer.screen_id,
                            ImageSource::try_from(&framebuffer.front_buffer).unwrap(),
                            0,
                            0,
                        )
                        .unwrap();

                    let dpi_factor = window.scale_factor() as f32;
                    let window_size = window.inner_size();
                    let width_stretch_factor =
                        window_size.width as f32 / WINDOW_LOGICAL_WIDTH_ORIGINAL as f32;
                    let height_stretch_factor =
                        window_size.height as f32 / WINDOW_LOGICAL_HEIGHT_ORIGINAL as f32;
                    canvas.set_size(window_size.width, window_size.height, dpi_factor);

                    let mut bg = Path::new();
                    bg.rect(
                        0f32,
                        0f32,
                        window_size.width as f32,
                        window_size.height as f32,
                    );
                    canvas.fill_path(
                        &bg,
                        &Paint::color(femtovg::Color::rgba(0xf4, 0xf2, 0xee, 0xff)),
                    );

                    let device_x = MARGIN as f32 * width_stretch_factor;
                    let device_y = MARGIN as f32 * height_stretch_factor;
                    let device_width =
                        (2 * PADDING_LEFT_RIGHT + SCREEN_WIDTH) as f32 * width_stretch_factor;
                    let device_height = (PADDING_TOP + SCREEN_HEIGHT + PADDING_BOTTOM) as f32
                        * height_stretch_factor;

                    let mut device_path = Path::new();
                    device_path.rect(device_x, device_y, device_width, device_height);
                    canvas.fill_path(
                        &device_path,
                        &Paint::image(
                            *bg_id,
                            device_x,
                            device_y,
                            device_width,
                            device_height,
                            0f32,
                            1f32,
                        ),
                    );

                    let screen_x = (MARGIN + PADDING_LEFT_RIGHT) as f32 * width_stretch_factor;
                    let screen_y = (MARGIN + PADDING_TOP) as f32 * height_stretch_factor;
                    let screen_width = SCREEN_WIDTH as f32 * width_stretch_factor;
                    let screen_height = SCREEN_HEIGHT as f32 * height_stretch_factor;

                    let mut frame_path = Path::new();
                    frame_path.rect(screen_x, screen_y, screen_width, screen_height);
                    canvas.fill_path(
                        &frame_path,
                        &Paint::color(femtovg::Color::rgba(0x12, 0x14, 0x18, 0xff)),
                    );

                    framebuffer.present(canvas, screen_x, screen_y, screen_width, screen_height);

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
                let Some(window) = &self.window else {
                    return;
                };
                let (x, y) = window_to_logical(window, position);
                if let Some((x, y)) = screen_coord(x, y) {
                    self.touch_pos = Some((x, y));
                    if self.touch_active {
                        debug!("drag x={x}, y={y}");
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

    // Since the firmware code is very non-threadsafe all firmware code is called from user events
    // in the main thread.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {
                self.counter += 1;
                if self.counter == 5000 {
                    info!("test switch to logo (pop)");
                    self.bitbox.ui().switch_to_logo();
                }
                // Read data from TCP client
                let mut inbound_out = self.inbound_out.take();
                let mut disconnected = false;
                if let Some(inbound_out) = &mut inbound_out {
                    loop {
                        match inbound_out.try_recv() {
                            Ok(_data) => {
                                //bitbox02::usb_packet::process_from_report(&data);
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
                //loop {
                //    if let Some(data) = bitbox02::queue::pull_hww() {
                //        if let Some(outbound_in) = &mut self.outbound_in {
                //            if outbound_in.send(data).is_err() {
                //                info!("writer thread died and closed channel");
                //                let _ = self.outbound_in.take();
                //            }
                //        }
                //    } else {
                //        break;
                //    }
                //}
                // Business logic
                bitbox02_rust::async_usb::spin();
                //bitbox02::usb_processing::process_hww();
                //bitbox02::screen::process();
                lvgl::timer::handler();

                if let Some(ref mut task) = self.startup_task {
                    if let Ready(_startup) = util::bb02_async::spin(task) {
                        ACCEPTING_CONNECTIONS.store(true, Ordering::Relaxed);
                        self.startup_task = None;
                    }
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
        if self.window.is_some() {
            return;
        }
        self.create_window(event_loop, None)
            .expect("failed to create initial window");
        //self.startup_task = Some(Box::pin(bitbox02::hal::system::BitBox02System::startup()));
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

    info!("Change log level with environment variable. e.g. RUST_LOG=debug");

    let mut bitbox = BitBox03::new();

    let args = Args::parse();

    if !init_hww(&mut bitbox, args.preseed) {
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
                        counter.store(0, Ordering::Relaxed);
                        error!("Error with stream");
                        continue;
                    };
                    let Ok(stream_clone) = stream.try_clone() else {
                        counter.store(0, Ordering::Relaxed);
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
                    info!("Busy, won't accept new clients");
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
                std::thread::sleep(Duration::from_millis(UI_REFRESH_PERIOD_MS));
            }
        }
    });

    let mut app = App::new(bitbox);
    event_loop.run_app(&mut app)?;

    Ok(())
}
