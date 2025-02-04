use crate::UpChannel;
use portable_atomic::{AtomicBool, Ordering};

static mut CHANNEL: Option<UpChannel> = None;

#[defmt::global_logger]
struct Logger;

/// Sets the channel to use for [`defmt`] macros.
pub fn set_defmt_channel(channel: UpChannel) {
    unsafe { CHANNEL = Some(channel) }
}

/// Global logger lock.
static TAKEN: AtomicBool = AtomicBool::new(false);
static mut CS_RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        // safety: Must be paired with corresponding call to release(), see below
        let restore = unsafe { critical_section::acquire() };

        if TAKEN.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        // no need for CAS because interrupts are disabled
        TAKEN.store(true, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe { CS_RESTORE = restore };

        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        unsafe {
            let encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
            encoder.start_frame(do_write)
        }
    }

    unsafe fn flush() {}

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        let encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
        encoder.end_frame(do_write);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        TAKEN.store(false, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        let restore = CS_RESTORE;

        // safety: Must be paired with corresponding call to acquire(), see above
        critical_section::release(restore);
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        let encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
        encoder.write(bytes, do_write);
    }
}

fn do_write(bytes: &[u8]) {
    unsafe {
        let channel = core::ptr::addr_of_mut!(CHANNEL);
        if let Some(Some(c)) = channel.as_mut() {
            c.write(bytes);
        }
    }
}

/// Initializes RTT with a single up channel and sets it as the defmt channel for the printing
/// macros.
///
/// The optional arguments specify the blocking mode (default: `NoBlockSkip`) and size of the buffer
/// in bytes (default: 1024). See [`rtt_init`] for more details.
///
/// [`rtt_init`]: crate::rtt_init
#[macro_export]
macro_rules! rtt_init_defmt {
    ($mode:path, $size:expr) => {{
        let channels = $crate::rtt_init! {
            up: {
                0: {
                    size: $size,
                    mode: $mode,
                    name: "defmt"
                }
            }
        };

        $crate::set_defmt_channel(channels.up.0);
    }};

    ($mode:path) => {
        $crate::rtt_init_defmt!($mode, 1024);
    };

    () => {{
        use $crate::ChannelMode::NoBlockSkip;
        $crate::rtt_init_defmt!(NoBlockSkip, 1024);
    }};
}
