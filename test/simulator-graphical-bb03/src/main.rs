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

use bitbox02_rust::hal::{Hal, Memory};

// Explicitly link library for its C exports
extern crate bitbox02_rust_c;
use bitbox_lvgl::{
    LvAlign, LvDisplayRenderMode, LvIndevState, LvIndevType, LvPoint, lv_display_create,
    lv_display_set_buffers, lv_display_set_flush_cb, lv_display_set_user_data, lv_indev_create,
    lv_indev_set_read_cb, lv_indev_set_type, lv_indev_set_user_data, lv_init, lv_label_create,
    lv_label_set_text, lv_obj_align, lv_screen_active, lv_tick_set_cb, lv_timer_handler,
};

use std::collections::VecDeque;

const UI_REFRESH_PERIOD_MS: u64 = 5;

static BG: &[u8] = include_bytes!("../bg.png");

const MARGIN: usize = 20;
const PADDING_TOP: usize = 50;
const PADDING_BOTTOM: usize = 95;
const PADDING_LEFT_RIGHT: usize = 50;
// TODO put size in product crate
const SCREEN_WIDTH: usize = 480;
const SCREEN_HEIGHT: usize = 800;
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

// Simulator frame buffer
#[derive(Debug)]
struct FrameBuffer {
    buffer: DynamicImage,
    screen_id: ImageId,
}
impl FrameBuffer {
    pub fn new(canvas: &mut Canvas<OpenGl>) -> FrameBuffer {
        let buffer =
            DynamicImage::ImageRgba8(RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
        let screen_id = canvas
            .create_image(ImageSource::try_from(&buffer).unwrap(), ImageFlags::NEAREST)
            .unwrap();
        FrameBuffer { buffer, screen_id }
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

impl bitbox03::display::Display for FrameBuffer {
    type Error = ();

    fn size(&self) -> (u16, u16) {
        (480, 800)
    }

    fn clear(&mut self, _c: bitbox03::display::Rgb565) -> Result<(), Self::Error> {
        todo!()
    }
    fn fill_rect(&mut self, _r: bitbox03::display::Rect) -> Result<(), Self::Error> {
        todo!()
    }
    fn blit(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

static ACCEPTING_CONNECTIONS: AtomicBool = AtomicBool::new(false);

fn hw_lvgl() {
    /* Get the currently active screen */
    let scr = lv_screen_active().expect("get active screen");

    /* Create a label */
    let label = lv_label_create(&scr).expect("create label");

    /* Set the label text */
    lv_label_set_text(&label, "BitBox03\nHello, World!\nFrom LVGL").expect("label set text");

    /* Center it on the screen */
    lv_obj_align(&label, LvAlign::LV_ALIGN_CENTER, 0, 0);

    //let button = lv_button_create();
}

extern "C" fn get_current_time_ms() -> u32 {
    use std::time::Instant;
    static INIT: LazyLock<Instant> = LazyLock::new(|| Instant::now());
    INIT.elapsed().as_millis() as u32
}

extern "C" fn my_flush_cb(
    display: *mut bitbox_lvgl::ffi::lv_display_t,
    area: *const bitbox_lvgl::ffi::lv_area_t,
    px_map: *mut u8,
) {
    debug_assert!(!display.is_null());
    debug_assert!(!area.is_null());
    debug_assert!(!px_map.is_null());
    let area = unsafe { &*area };
    info!("flush {:?}", area);
    let fb_ptr = unsafe { bitbox_lvgl::ffi::lv_display_get_user_data(display) as *mut FrameBuffer };
    debug_assert!(fb_ptr != core::ptr::null_mut());
    let fb = unsafe { &mut *fb_ptr };
    let pxs = px_map as *mut u32;
    const STRIDE: i32 = 480;
    let offset = area.y1 * STRIDE;
    let len = (area.y2 - area.y1 + 1) * STRIDE;
    if let DynamicImage::ImageRgba8(ref mut image_buf) = fb.buffer {
        for (i, pixel) in image_buf
            .pixels_mut()
            .skip(offset as usize)
            .take(len as usize)
            .enumerate()
        {
            *pixel = Rgba(unsafe { (*pxs.add(i)).to_le_bytes() });
        }
    };

    /* IMPORTANT!!!
     * Inform LVGL that flushing is complete so buffer can be modified again. */
    unsafe { bitbox_lvgl::ffi::lv_display_flush_ready(display) };
}

extern "C" fn indev_read_cb(
    indev: *mut bitbox_lvgl::ffi::lv_indev_t,
    data: *mut bitbox_lvgl::ffi::lv_indev_data_t,
) {
    let ud_ptr = unsafe { bitbox_lvgl::ffi::lv_indev_get_user_data(indev) };
    debug_assert!(ud_ptr != core::ptr::null_mut());
    let ud = unsafe { &mut *(ud_ptr as *mut VecDeque<TouchScreenEvent>) };
    if let Some(next) = ud.pop_front() {
        info!("popped event");
        let data = unsafe { &mut *data };
        data.point = LvPoint {
            x: next.x,
            y: next.y,
        };
        data.state = if next.pressed {
            LvIndevState::LV_INDEV_STATE_PRESSED
        } else {
            LvIndevState::LV_INDEV_STATE_RELEASED
        };
        data.continue_reading = !ud.is_empty()
    }
}

fn init_hww(preseed: bool) -> bool {
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
    bitbox02::memory::fake_nova();
    info!("Memory setup: success");

    if preseed {
        let mnemonic = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
        let seed = bitbox02_rust::bip39::mnemonic_to_seed(&mnemonic).unwrap();
        let mut hal = bitbox02_rust::hal::BitBox02Hal::new();
        bitbox02_rust::keystore::encrypt_and_store_seed(&mut hal, &seed, "").unwrap();
        hal.memory().set_initialized().unwrap();
    }

    bitbox02::smarteeprom::bb02_config();
    bitbox02::smarteeprom::init();

    true
}

struct TouchScreenEvent {
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
}

//struct TouchScreen {
//    indev: LvIndev,
//}

//impl TouchScreen {
//    pub fn new(indev: LvIndev) -> TouchScreen {
//        TouchScreen { indev }
//    }
//}

struct App {
    framebuffer: Option<Box<FrameBuffer>>,
    //touchscreen: Option<TouchScreen>,
    touchscreen_events: Box<VecDeque<TouchScreenEvent>>,
    window: Option<Rc<Window>>,
    surface: Option<Surface<WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
    canvas: Option<Canvas<OpenGl>>,
    bg: Option<ImageId>,
    touch_active: bool,
    cursor_pos: (i32, i32),
    outbound_in: Option<mpsc::Sender<[u8; 64]>>,
    inbound_out: Option<mpsc::Receiver<[u8; 64]>>,
    orientation_task: Option<util::bb02_async::Task<'static, bool>>,
}

impl Default for App {
    fn default() -> App {
        App {
            framebuffer: Default::default(),
            //touchscreen: Default::default(),
            touchscreen_events: Default::default(),
            window: Default::default(),
            surface: Default::default(),
            gl_context: Default::default(),
            canvas: Default::default(),
            bg: Default::default(),
            touch_active: false,
            cursor_pos: (0, 0),
            outbound_in: Default::default(),
            inbound_out: Default::default(),
            orientation_task: Default::default(),
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
        lv_init();
        lv_tick_set_cb(Some(get_current_time_ms));

        // Make a buffer and give it to lvgl.
        // RGB565 (16 bits per pixel)
        let buf = Box::leak(Box::new([0; 480 * 800 / 10 * 4]));
        let disp = lv_display_create(480, 800).expect("create display");
        unsafe {
            lv_display_set_buffers(
                &disp,
                buf,
                None,
                LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_PARTIAL,
            )
            .expect("display set buffers");
        };
        lv_display_set_flush_cb(&disp, Some(my_flush_cb));
        hw_lvgl();

        let width = WINDOW_LOGICAL_WIDTH_ORIGINAL as u32;
        let height = WINDOW_LOGICAL_HEIGHT_ORIGINAL as u32;
        let w_attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(width, height))
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

        let mut framebuffer = Box::new(FrameBuffer::new(&mut canvas));
        unsafe {
            lv_display_set_user_data(
                &disp,
                NonNull::new(framebuffer.as_mut() as *mut _ as *mut _),
            )
        };

        let indev = lv_indev_create().expect("create input device");
        lv_indev_set_type(&indev, LvIndevType::LV_INDEV_TYPE_POINTER);
        lv_indev_set_read_cb(&indev, Some(indev_read_cb));

        unsafe {
            lv_indev_set_user_data(
                &indev,
                NonNull::new(self.touchscreen_events.as_mut() as *mut _ as *mut _),
            )
        };

        //let touchscreen = TouchScreen::new(indev);

        self.framebuffer.replace(framebuffer);
        //self.touchscreen.replace(touchscreen);
        self.window.replace(window);
        self.surface.replace(surface);
        self.gl_context.replace(gl_context);
        self.canvas.replace(canvas);
        self.bg.replace(bg_id);
        info!("window created");
        Ok(window_id)
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
                    && let Some(framebuffer) = &mut self.framebuffer
                {
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
                self.cursor_pos = (x, y);

                if !self.touch_active {
                    return;
                }

                if let Some((x, y)) = screen_coord(x, y) {
                    debug!("drag x={x}, y={y}");
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button != MouseButton::Left {
                    return;
                }
                let (x, y) = self.cursor_pos;

                match state {
                    ElementState::Pressed => {
                        if let Some((x, y)) = screen_coord(x, y) {
                            self.touchscreen_events.push_back(TouchScreenEvent {
                                x,
                                y,
                                pressed: true,
                            });
                            debug!("pressed x={x}, y={y}");
                            self.touch_active = true;
                        }
                    }
                    ElementState::Released => {
                        if !self.touch_active {
                            return;
                        }
                        self.touch_active = false;
                        if let Some((x, y)) = screen_coord(x, y) {
                            self.touchscreen_events.push_back(TouchScreenEvent {
                                x,
                                y,
                                pressed: false,
                            });
                            debug!("released x={x}, y={y}");
                        }
                    }
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
                                bitbox02::usb_packet::process_from_report(&data);
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
                bitbox02_rust::async_usb::spin();
                bitbox02::usb_processing::process_hww();
                //bitbox02::screen::process();
                lv_timer_handler();

                if let Some(ref mut task) = self.orientation_task {
                    if let Ready(_orientation) = util::bb02_async::spin(task) {
                        ACCEPTING_CONNECTIONS.store(true, Ordering::Relaxed);
                        self.orientation_task = None;
                    }
                }

                if let Some(window) = &self.window
                    && let Some(canvas) = &mut self.canvas
                    && let Some(framebuffer) = &mut self.framebuffer
                {
                    // TODO: We should only update texture and redraw in case screen actually changed.
                    // Update opengl texture from "screen_process"
                    canvas
                        .update_image(
                            framebuffer.screen_id,
                            ImageSource::try_from(&framebuffer.buffer).unwrap(),
                            0,
                            0,
                        )
                        .unwrap();

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

    info!("Change log level with environment variable. e.g. RUST_LOG=debug");

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

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
