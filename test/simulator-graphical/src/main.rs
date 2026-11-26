// SPDX-License-Identifier: Apache-2.0

mod backend;
#[cfg(test)]
mod tests;
use backend::{BG, Backend, GEOMETRY, NAME, SimulatorHal};

use clap::Parser;
use femtovg::{Canvas, ImageFlags, ImageId, ImageSource, renderer::OpenGl};

use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc,
    mpsc::TryRecvError,
};
use std::task::Poll::Ready;
use std::thread;
use std::time::{Duration, Instant};

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

use bitbox_hal::{Hal, Memory, system::System};

use util::bb02_async::block_on;

const UI_REFRESH_PERIOD_MS: u64 = 5;
const EDITION: &str = if cfg!(feature = "edition-btc-only") {
    "btc-only"
} else {
    "multi"
};
const PROFILE: &str = if cfg!(debug_assertions) {
    "debug"
} else {
    "release"
};
static ACCEPTING_CONNECTIONS: AtomicBool = AtomicBool::new(false);

pub fn handle_stream_reader(
    mut stream: TcpStream,
    inbound_in: mpsc::Sender<[u8; 64]>,
    counter: Arc<AtomicU32>,
) {
    let mut report = [0; 64];
    while stream.read_exact(&mut report).is_ok() {
        if inbound_in.send(report).is_err() {
            break;
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

fn init_hww(
    bitbox: &mut SimulatorHal,
    args: &Args,
) -> bitbox02_rust::hww::transport::HwwTransport<SimulatorHal> {
    block_on(bitbox02_rust::secp256k1::init(bitbox)).unwrap();

    if args.preseed {
        let mnemonic = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
        let seed = bitbox02_rust::bip39::mnemonic_to_seed(mnemonic).unwrap();
        block_on(bitbox02_rust::keystore::encrypt_and_store_seed(
            bitbox, &seed, "",
        ))
        .unwrap();
        bitbox.memory().set_initialized().unwrap();

        if args.unlock {
            // Storing the seed retains it but does not unlock BIP39; the keystore counts as
            // locked (and the first client connection prompts for the password) until the BIP39
            // seed is retained too.
            block_on(bitbox02_rust::keystore::unlock_bip39(
                &mut bitbox02_rust::keystore::KeystoreHalImpl::from_hal(bitbox),
                &seed,
                "",
                async || {},
            ))
            .unwrap();
        }
    }

    if args.passphrase {
        bitbox
            .memory()
            .set_mnemonic_passphrase_enabled(true)
            .unwrap();
    }

    if !args.unlock {
        bitbox02_rust::keystore::lock();
    }

    bitbox02_rust::hww::transport::hww_transport::<SimulatorHal>()
}

struct App {
    bitbox: SimulatorHal,
    backend: Backend,
    window: Option<Rc<Window>>,
    surface: Option<Surface<WindowSurface>>,
    gl_context: Option<PossiblyCurrentContext>,
    canvas: Option<Canvas<OpenGl>>,
    bg: Option<ImageId>,
    outbound_in: Option<mpsc::Sender<[u8; 64]>>,
    inbound_out: Option<mpsc::Receiver<[u8; 64]>>,
    startup_task: Option<util::bb02_async::Task<'static, ()>>,
    transport: Option<bitbox02_rust::hww::transport::HwwTransport<SimulatorHal>>,
    started_at: Instant,
}

impl App {
    fn new(bitbox: SimulatorHal) -> App {
        App {
            bitbox,
            backend: Backend::default(),
            window: Default::default(),
            surface: Default::default(),
            gl_context: Default::default(),
            canvas: Default::default(),
            bg: Default::default(),
            outbound_in: Default::default(),
            inbound_out: Default::default(),
            startup_task: Default::default(),
            transport: Default::default(),
            started_at: Instant::now(),
        }
    }
}

impl App {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> Result<WindowId, Box<dyn Error>> {
        let (width, height) = GEOMETRY.window_size();
        let w_attr = Window::default_attributes()
            .with_inner_size(LogicalSize::new(
                (width as f64 * GEOMETRY.initial_scale) as u32,
                (height as f64 * GEOMETRY.initial_scale) as u32,
            ))
            .with_title(format!("Graphical {NAME} Simulator ({EDITION})"));

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

        self.backend
            .init_display(&mut canvas, window.clone(), &mut self.bitbox);
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

impl ApplicationHandler<UserEvent> for App {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface
                    && let Some(canvas) = &mut self.canvas
                    && let Some(gl_context) = &mut self.gl_context
                    && let Some(bg_id) = self.bg
                    && let Some(window) = &self.window
                {
                    let size = window.inner_size();
                    if size.width == 0 || size.height == 0 {
                        return;
                    }
                    canvas.set_size(size.width, size.height, window.scale_factor() as f32);
                    GEOMETRY.render_background(canvas, bg_id, size);
                    self.backend.render(canvas, GEOMETRY.screen_rect(size));
                    canvas.flush_to_surface(&());
                    surface.swap_buffers(gl_context).unwrap();
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                if let Some(surface) = &mut self.surface
                    && let Some(gl_context) = &mut self.gl_context
                    && let Some(width) = NonZeroU32::new(width)
                    && let Some(height) = NonZeroU32::new(height)
                {
                    surface.resize(gl_context, width, height);
                }
            }
            event => {
                if let Some(window) = &self.window {
                    self.backend.input(window, event);
                }
            }
        }
    }

    // Since the firmware code is very non-threadsafe all firmware code is called from user events
    // in the main thread.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {
                let now_ms = self.started_at.elapsed().as_millis() as u64;
                // Read data from TCP client
                let mut inbound_out = self.inbound_out.take();
                let mut disconnected = false;
                if let Some(inbound_out) = &mut inbound_out {
                    loop {
                        match inbound_out.try_recv() {
                            Ok(data) => {
                                if let Some(transport) = self.transport.as_mut() {
                                    transport.handle_report(&data, now_ms);
                                }
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
                if let Some(transport) = self.transport.as_mut() {
                    transport.tick(now_ms);
                }
                // Send data to TCP Client
                loop {
                    if let Some(data) = self
                        .transport
                        .as_mut()
                        .and_then(|transport| transport.pull_report())
                    {
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
                if let Some(canvas) = &mut self.canvas
                    && let Some(window) = &self.window
                {
                    self.backend.poll(canvas, window);
                }

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
        self.create_window(event_loop)
            .expect("failed to create initial window");
        self.startup_task = Some(Box::pin(<SimulatorHal as Hal>::System::startup()));
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pre seed the simulated bitbox with empty password and the following bip39 seed phrase "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide"
    #[arg(long)]
    preseed: bool,

    /// Enable the BIP39 mnemonic passphrase setting, so the unlock/setup/restore workflows prompt
    /// for an optional passphrase.
    #[arg(long)]
    passphrase: bool,

    /// Start with the keystore unlocked (requires --preseed). By default the simulator starts
    /// locked and the first client connection triggers the on-device unlock workflow.
    #[arg(long, requires = "preseed")]
    unlock: bool,

    /// TCP port to listen on for the host connection.
    #[arg(long, default_value_t = 15423)]
    port: u16,
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

    info!("{NAME} {EDITION} simulator ({PROFILE})");
    let mut bitbox = Backend::init_hal()?;
    let transport = init_hww(&mut bitbox, &args);
    let mut app = App::new(bitbox);
    app.transport = Some(transport);
    let event_loop = EventLoop::<UserEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    // "Accept incoming connections" thread
    thread::spawn({
        let el_proxy = event_loop.create_proxy();
        let Ok(listener) = TcpListener::bind(("0.0.0.0", args.port)) else {
            return Err("Failed to bind to address!".into());
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

    event_loop.run_app(&mut app)?;

    Ok(())
}
