//! The default, minimal renderer, and some helper functions.

use core::{cmp, fmt};

/// For some reason, there seems to be no existing method to easily render
/// fmt::Arguments in a pre-allocated byte array.
///
/// That is what this does.
pub fn render_arguments<'a>(buf: &'a mut [u8], args: fmt::Arguments) -> &'a [u8] {
    let mut writer = WriteTo::new(buf);
    core::fmt::write(&mut writer, args).ok();
    writer.endl();
    let used = writer.used;
    &buf[..used]
}

/// Render record, based on feature flags.
pub fn render_record<'a>(buf: &'a mut [u8], record: &log::Record) -> &'a [u8] {
    render_arguments(buf, *record.args())
}

// I don't get it, why isn't this implemented already?
struct WriteTo<'a> {
    buffer: &'a mut [u8],
    // on write error (i.e. not enough space in buffer) this grows beyond
    // `buffer.len()`.
    used: usize,
}

impl<'a> WriteTo<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        WriteTo { buffer, used: 0 }
    }

    pub fn endl(&mut self) {
        if self.used < self.buffer.len() {
            self.buffer[self.used] = b'\n';
            self.used += 1;
        }
    }
}

impl<'a> core::fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.used > self.buffer.len() {
            return Err(fmt::Error);
        }
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();
        let write_num = cmp::min(raw_s.len(), remaining_buf.len());
        remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
        self.used += write_num;
        if write_num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

use crate::Renderer;

#[derive(Clone, Copy)]
/// Renders just the `record.args()`.
pub struct DefaultRenderer {}

/// The default, minimal renderer.
pub fn default() -> &'static DefaultRenderer {
    static RENDERER: DefaultRenderer = DefaultRenderer {};
    &RENDERER
}

impl Renderer for DefaultRenderer {
    fn render<'a>(&self, buf: &'a mut [u8], record: &log::Record) -> &'a [u8] {
        render_arguments(buf, *record.args())
    }
}

unsafe impl Send for DefaultRenderer {}
unsafe impl Sync for DefaultRenderer {}

#[derive(Clone, Copy)]
/// Renders the `record.args()`, prefixed by level, target, and file, line if they are some.
pub struct RipgrepRenderer {}

impl Renderer for RipgrepRenderer {
    fn render<'a>(&self, buf: &'a mut [u8], record: &log::Record) -> &'a [u8] {
        match (record.file(), record.line()) {
            (Some(file), Some(line)) => render_arguments(
                buf,
                format_args!(
                    "{}|{}|{}:{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    line,
                    record.args()
                ),
            ),
            (Some(file), None) => render_arguments(
                buf,
                format_args!(
                    "{}|{}|{}: {}",
                    record.level(),
                    record.target(),
                    file,
                    record.args()
                ),
            ),
            _ => render_arguments(
                buf,
                format_args!("{}|{}: {}", record.level(), record.target(), record.args()),
            ),
        }
    }
}
