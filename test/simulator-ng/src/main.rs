use sdl2::event::{Event, EventSender};
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
    mpsc,
};
use std::thread;
use std::time::{Duration, Instant};

// "use" these dependencies to link in functions
extern crate bitbox02_rust_c;
extern crate bitbox_aes;

#[derive(Debug)]
struct NewConnectionEvent {
    /// Sending half of channel for sending to connected client
    sender: mpsc::Sender<[u8; 64]>,
}

#[derive(Debug)]
struct HWWDataEvent {
    /// New data from client, to be forwarded to HWW
    data: [u8; 64],
}

/// When reading thread detects an end of stream, send disconnect event to kill writer thread
#[derive(Debug)]
struct DisconnectEvent {}

static BG: &[u8; 325362] = include_bytes!("../bg.png");

use bitbox02::bindings::{SCREEN_HEIGHT, SCREEN_WIDTH, UG_COLOR};

const MARGIN: u32 = 10;
const PADDING_TOP_BOTTOM: u32 = 22;
const PADDING_LEFT: u32 = 60;
const PADDING_RIGHT: u32 = 35;

pub fn handle_stream_reader(mut stream: TcpStream, es: EventSender, counter: Arc<AtomicU32>) {
    let mut buf = [0; 1024];
    while let Ok(len) = stream.read(&mut buf) {
        if len == 0 {
            // Client disconnected
            break;
        }
        for i in 0..len / 64 {
            let mut event = HWWDataEvent { data: [0; 64] };
            event.data.copy_from_slice(&buf[64 * i..64 * (i + 1)]);
            es.push_custom_event(event).unwrap();
        }
    }
    es.push_custom_event(DisconnectEvent {}).unwrap();
    println!("Reader: Disconnected");
    counter.fetch_sub(1, Ordering::SeqCst);
}

pub fn handle_stream_writer(
    mut stream: TcpStream,
    counter: Arc<AtomicU32>,
    outbound: mpsc::Receiver<[u8; 64]>,
) {
    for item in outbound.iter() {
        if stream.write_all(&item[..]).is_err() {
            // Client disconnected while writing
            break;
        }
    }
    println!("Writer: Disconnected");
    counter.fetch_sub(1, Ordering::SeqCst);
}

pub fn network_task(es: EventSender, evs: mpsc::Receiver<EventSender>) {
    let listener = TcpListener::bind("0.0.0.0:15423").unwrap();

    // Use this counter to ensure that we only connect to a single client.
    let counter = Arc::new(AtomicU32::new(0));
    for stream in listener.incoming() {
        if counter.fetch_add(2, Ordering::SeqCst) == 0 {
            let (sender, receiver) = mpsc::channel();
            let event = NewConnectionEvent { sender };
            es.push_custom_event(event).unwrap();
            let stream = stream.unwrap();
            thread::spawn({
                let stream = stream.try_clone().unwrap();
                let es = evs.recv().unwrap();
                let counter = counter.clone();
                move || handle_stream_reader(stream, es, counter)
            });
            thread::spawn({
                let counter = counter.clone();
                move || handle_stream_writer(stream, counter, receiver)
            });
        } else {
            println!("Busy, won't accept new clients");
            counter.fetch_sub(2, Ordering::SeqCst);
        }
    }
}

static mut DISPLAY_BUF: [u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize] =
    [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

// Unsafe, do not use this while you are mutating DISPLAY_BUF...
unsafe fn display_buf_as_u8_slice() -> &'static [u8] {
    let ptr: *const u32 = unsafe { &raw const DISPLAY_BUF[0] };
    let len = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize * size_of::<u32>();
    unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }
}

unsafe extern "C" fn pixel_fn(x: i16, y: i16, c: UG_COLOR) {
    let ptr: *mut _ = unsafe { &raw mut DISPLAY_BUF[0] };
    let c = c as u32;
    let offset = ((y * SCREEN_WIDTH as i16) + x) as usize;

    // White pixels are OPAQUE, Black pixels are completely transparent.
    if c != 0 {
        unsafe { *ptr.add(offset) = (c << 8) | (c << 16) | (c << 24) | 0xff }
    } else {
        unsafe { *ptr.add(offset) = 0 }
    }
}

unsafe extern "C" fn clear_fn() {
    let mut ptr: *mut _ = unsafe { &raw mut DISPLAY_BUF[0] };
    for _ in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
        unsafe { *ptr = 0 }
        ptr = unsafe { ptr.add(1) };
    }
}

unsafe extern "C" fn mirror_fn(_: bool) {}

fn init_hww() -> bool {
    unsafe { bitbox02::bindings::screen_init(Some(pixel_fn), Some(mirror_fn), Some(clear_fn)) };
    unsafe { bitbox02::bindings::screen_splash() };

    // BitBox02 simulation initialization
    unsafe { bitbox02::bindings::usb_processing_init() };
    println!("USB setup success");

    unsafe { bitbox02::bindings::hww_setup() };
    println!("HWW setup success");

    let sd_success = unsafe { bitbox02::bindings::sd_format() };
    if !sd_success {
        eprintln!("ERROR, sd card setup failed");
        return false;
    }

    println!("Sd card setup: success");

    bitbox02::testing::mock_memory();
    println!("Memory setup: success");

    unsafe { bitbox02::bindings::smarteeprom_bb02_config() };
    unsafe { bitbox02::bindings::bitbox02_smarteeprom_init() };
    true
}

unsafe extern "C" {
    fn rust_workflow_spin();
    fn rust_async_usb_spin();
}

struct Slider {
    active: bool,
    position: u16,
    pinch: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Slider {
            active: false,
            position: 0,
            pinch: false,
        }
    }
}

pub fn handle_mouse_motion(sliders: &mut (Slider, Slider), x: i32, y: i32, xrel: i32) {
    let slider_top = &mut sliders.0;
    let slider_bottom = &mut sliders.1;

    let slider_data = bitbox02::bindings::gestures_slider_data_t {
        diff: xrel as i16,
        position: (x as u16 - (MARGIN + PADDING_LEFT) as u16) * 2,
        velocity: 0,
    };
    if y < MARGIN as i32 / 2 {
        if slider_top.active {
            println!("top tap");
            slider_top.active = false;
            let event = bitbox02::bindings::event_t {
                data: &slider_data as *const _ as *const _,
                id: bitbox02::bindings::event_types::EVENT_TOP_SHORT_TAP as u8,
            };
            unsafe { bitbox02::bindings::emit_event(&event) };
        }
    } else if y < MARGIN as i32 {
        slider_top.active = true;
    } else if y > (MARGIN + PADDING_TOP_BOTTOM + SCREEN_HEIGHT / 3) as i32
        && y < (MARGIN + PADDING_TOP_BOTTOM + SCREEN_HEIGHT * 2 / 3) as i32
    {
        if x > (MARGIN + PADDING_LEFT + SCREEN_WIDTH + PADDING_RIGHT) as i32 {
            slider_top.pinch = true;
        }
    } else if y > (MARGIN + 2 * PADDING_TOP_BOTTOM + 64 + MARGIN / 2) as i32 {
        if slider_bottom.active {
            println!("bottom tap");
            slider_bottom.active = false;
            let event = bitbox02::bindings::event_t {
                data: &slider_data as *const _ as *const _,
                id: bitbox02::bindings::event_types::EVENT_BOTTOM_SHORT_TAP as u8,
            };
            unsafe { bitbox02::bindings::emit_event(&event) };
        }
    } else if y > (MARGIN + 2 * PADDING_TOP_BOTTOM + 64) as i32 {
        slider_bottom.active = true;
    } else {
        slider_top.pinch = false;
    }
}

pub fn main() -> Result<(), i32> {
    let sdl_context = sdl2::init().unwrap();

    if !init_hww() {
        return Err(1);
    }

    let ev = sdl_context.event().unwrap();
    let (evs_sender, evs_receiver) = mpsc::channel();
    ev.register_custom_event::<NewConnectionEvent>().unwrap();
    ev.register_custom_event::<HWWDataEvent>().unwrap();
    ev.register_custom_event::<DisconnectEvent>().unwrap();
    let es = ev.event_sender();
    thread::spawn(|| network_task(es, evs_receiver));

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "BitBox02 simulator",
            SCREEN_WIDTH + 2 * MARGIN + PADDING_LEFT + PADDING_RIGHT,
            SCREEN_HEIGHT + 2 * MARGIN + 2 * PADDING_TOP_BOTTOM,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let tc = canvas.texture_creator();
    let bg = tc.load_texture_bytes(BG).unwrap();
    let mut screen_content = tc
        .create_texture_static(Some(PixelFormatEnum::RGBA8888), SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();
    screen_content.set_blend_mode(BlendMode::Blend);

    let content_area = Rect::new(
        (MARGIN + PADDING_LEFT) as i32,
        (MARGIN + PADDING_TOP_BOTTOM) as i32,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );
    let bg_area = Rect::new(
        MARGIN as i32,
        MARGIN as i32,
        SCREEN_WIDTH + PADDING_LEFT + PADDING_RIGHT,
        SCREEN_HEIGHT + 2 * PADDING_TOP_BOTTOM,
    );
    let slider_top_bg = Rect::new(
        (MARGIN + PADDING_LEFT) as i32,
        (MARGIN / 2) as i32,
        SCREEN_WIDTH,
        MARGIN / 2,
    );
    let slider_bottom_bg = Rect::new(
        (MARGIN + PADDING_LEFT) as i32,
        (MARGIN + 2 * PADDING_TOP_BOTTOM + SCREEN_HEIGHT) as i32,
        SCREEN_WIDTH,
        MARGIN / 2,
    );
    let pinch_bg = Rect::new(
        (MARGIN + PADDING_LEFT + 128 + PADDING_RIGHT) as i32,
        (MARGIN + PADDING_TOP_BOTTOM + SCREEN_HEIGHT / 3) as i32,
        MARGIN / 2,
        SCREEN_HEIGHT / 3,
    );

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_blit = Instant::now();
    let mut current_client_outbound = None;
    let mut sliders = (Slider::default(), Slider::default());
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, xrel, .. } => {
                    handle_mouse_motion(&mut sliders, x, y, xrel);
                }
                Event::User { .. } => {
                    if let Some(e) = event.as_user_event_type::<NewConnectionEvent>() {
                        println!("New connection {e:?}");
                        current_client_outbound = Some(e.sender);
                        evs_sender.send(ev.event_sender()).unwrap();
                    } else if let Some(e) = event.as_user_event_type::<HWWDataEvent>() {
                        unsafe {
                            bitbox02::bindings::usb_packet_process(
                                e.data.as_ptr() as *const bitbox02::bindings::USB_FRAME
                            )
                        };
                    } else if let Some(_) = event.as_user_event_type::<DisconnectEvent>() {
                        current_client_outbound = None;
                    }
                }
                _ => {}
            }
        }

        // Send data from HWW to TCP Client
        loop {
            let hww_data =
                unsafe { bitbox02::bindings::queue_pull(bitbox02::bindings::queue_hww_queue()) };
            if hww_data != std::ptr::null() {
                if let Some(sender) = &current_client_outbound {
                    let mut data: [u8; 64] = [0; 64];
                    let slice: &[u8] = unsafe { std::slice::from_raw_parts(hww_data, 64) };
                    data.copy_from_slice(&slice[..]);
                    if sender.send(data).is_err() {
                        println!("writer thread died and closed channel");
                        current_client_outbound = None;
                    }
                }
            } else {
                break;
            }
        }

        unsafe { rust_workflow_spin() }
        bitbox02_rust::async_usb::spin();
        unsafe { rust_async_usb_spin() }
        unsafe {
            bitbox02::bindings::usb_processing_process(bitbox02::bindings::usb_processing_hww())
        }

        // Draw with 60hz
        let now = Instant::now();
        if now.duration_since(last_blit) > Duration::from_micros(16667) {
            // Special handling of pinch. Emit one event per frame
            if sliders.0.pinch {
                let slider_data = bitbox02::bindings::gestures_slider_data_t {
                    diff: 0,
                    position: bitbox02::bindings::SLIDER_POSITION_TWO_THIRD as u16 + 1,
                    velocity: 0,
                };
                let mut event = bitbox02::bindings::event_t {
                    data: &slider_data as *const _ as *const _,
                    id: bitbox02::bindings::event_types::EVENT_TOP_CONTINUOUS_TAP as u8,
                };
                unsafe { bitbox02::bindings::emit_event(&event) };
                event.id = bitbox02::bindings::event_types::EVENT_BOTTOM_CONTINUOUS_TAP as u8;
                unsafe { bitbox02::bindings::emit_event(&event) };
            }

            unsafe { bitbox02::bindings::screen_process() }
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0, 150, 150));
            last_blit = now;

            canvas
                .copy(&bg, None, bg_area)
                .expect("Couldn't copy to canvas");
            canvas
                .fill_rects(&[slider_top_bg, slider_bottom_bg, pinch_bg])
                .unwrap();

            screen_content
                .update(
                    None,
                    unsafe { display_buf_as_u8_slice() },
                    SCREEN_WIDTH as usize * size_of::<u32>(),
                )
                .unwrap();
            canvas.copy(&screen_content, None, content_area).unwrap();

            canvas.present();
        }
    }
    Ok(())
}
