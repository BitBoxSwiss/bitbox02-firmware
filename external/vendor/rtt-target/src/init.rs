/// rtt_init! implementation detail
#[macro_export]
#[doc(hidden)]
macro_rules! rtt_init_repeat {
    ({ $($code:tt)+ } { $($acc:tt)* }; $n:literal: { $($_:tt)* } $($tail:tt)*) => {
        $crate::rtt_init_repeat!({ $($code)* } { $($code)* $($acc)* }; $($tail)*)
    };
    ({ $($code:tt)+ } { $($acc:tt)* };) => {
        ($($acc)*)
    };
}

/// rtt_init! implementation detail
#[macro_export]
#[doc(hidden)]
macro_rules! rtt_init_channels {
    (
        $field:expr;
        $number:literal: {
            $(
                size: $size:expr
                $(, mode: $mode:path )?
                $(, name: $name:literal )?
                $(, section: $section:literal )?
                $(,)?
            )?
        }
        $($tail:tt)*
    ) => {
        $(
            let mut name: *const u8 = core::ptr::null();
            $( name = concat!($name, "\0").as_bytes().as_ptr(); )?

            let mut mode = $crate::ChannelMode::NoBlockSkip;
            $( mode = $mode; )?

            $field[$number].init(name, mode, {
                $( #[link_section = $section] )?
                static mut _RTT_CHANNEL_BUFFER: MaybeUninit<[u8; $size]> = MaybeUninit::uninit();
                _RTT_CHANNEL_BUFFER.as_mut_ptr()
            });
        )?

        $crate::rtt_init_channels!($field; $($tail)*);
    };
    ($field:expr;) => { };
}

/// rtt_init! implementation detail
#[macro_export]
#[doc(hidden)]
macro_rules! rtt_init_channels_are_reusable {
    (
        $field:expr;
        $number:literal: {
            $(
                size: $size:expr
                $(, mode: $mode:path )?
                $(, name: $name:literal )?
                $(, section: $section:literal )?
                $(,)?
            )?
        }
        $($tail:tt)*
    ) => {{
        let mut reusable = true;
        $(
            reusable = $field[$number].is_initialized_with_size($size);
        )?

        reusable && $crate::rtt_init_channels_are_reusable!($field; $($tail)*)
    }};
    ($field:expr;) => { true };
}

/// rtt_init! implementation detail
#[macro_export]
#[doc(hidden)]
macro_rules! rtt_init_wrappers {
    ($field:expr; $cons:path; { $($acc:tt)* }; $n:literal: { $($_:tt)* } $($tail:tt)*) => {
        $crate::rtt_init_wrappers!(
            $field;
            $cons;
            {
                $($acc)*
                $cons(&mut $field[$n] as *mut _),
            };
            $($tail)*)
    };
    ($field:expr; $cons:path; { $($acc:tt)* };) => {
        ($($acc)*)
    };
}

/// Initializes RTT with the specified channels. Channel numbers, buffer sizes and names can be
/// defined.
///
/// It is also possible to pre-allocate uninitialized channels to let crates accessing RTT via FFI
/// play nicely with crates using rtt-target directly.
///
/// The syntax looks as follows (note that commas are not allowed anywhere):
///
/// ```
/// let channels = rtt_init! {
///     up: {
///         0: { // channel number
///             size: 1024, // buffer size in bytes
///             mode: NoBlockSkip, // mode (optional, default: NoBlockSkip, see enum ChannelMode)
///             name: "Terminal", // name (optional, default: no name)
///             section: ".segger_term_buf" // Buffer linker section (optional, default: no section)
///         }
///         1: {
///             size: 32
///         }
///         2: { } // pre-allocated channel without buffer
///     }
///     down: {
///         0: {
///             size: 16,
///             name: "Terminal"
///         }
///     }
///     section_cb: ".segger_rtt" // Control block linker section (optional, default: no section)
///     reuse_if_initialized: true // Reuse an already valid control block (optional, default: false)
/// };
/// ```
///
/// The channel numbers must start from 0 and not skip any numbers, or otherwise odd things will
/// happen. The order of the channel parameters is fixed, but optional parameters can be left out.
///
/// If no size is given, then an RTT channel header entry will be allocated with its buffer pointer
/// set to `null`. This allows third-party FFI crates (e.g. [`rtos-trace`][rtos_trace]/
/// [`systemview-target`][systemview_target]) to identify and initialize the channel with its own
/// name and buffer from C code.
///
/// Note: Rust crates should rely on channels initialized through `rtt_init!`. That's why this crate
/// does not provide an API to initialize pre-allocated channels from Rust.
///
/// This macro should be called once within a function, preferably close to the start of your entry
/// point. The macro must only be called once - if it's called twice in the same program a duplicate
/// symbol error will occur.
///
/// At compile time the macro will statically reserve space for the RTT control block and the
/// channel buffers. At runtime the macro fills in the structures and prepares them for use.
///
/// If `reuse_if_initialized: true` is specified, the macro first checks if the control block is
/// already valid and matches the requested channel count. Configured channels with a buffer must
/// also already be initialized with the requested size. If all checks pass, the existing control
/// block is reused without resetting buffers or read/write pointers; otherwise the macro performs a
/// normal full initialization. This is useful when consecutive firmware stages intentionally share
/// the same RTT memory and must not desynchronize from an already attached host.
///
/// The macro returns a generate struct that contains the channels. The struct for the example above
/// would look as follows:
///
/// ```
/// struct Channels {
///     up: (UpChannel, UpChannel),
///     down: (DownChannel,),
/// }
/// ```
///
/// The channels can either be accessed by reference or moved out as needed. For example:
///
/// ```
/// use core::fmt::Write;
///
/// let channels = rtt_init! { ... };
/// let mut output = channels.up.0;
/// writeln!(output, "Hello, world!").ok();
/// ```
///
/// [rtos_trace]: https://docs.rs/rtos-trace
/// [systemview_target]: https://docs.rs/systemview-target/
#[macro_export]
macro_rules! rtt_init {
    {
        $(up: { $($up:tt)* } )?
        $(down: { $($down:tt)* } )?
        $(section_cb: $section_cb:literal )?
        $(reuse_if_initialized: $reuse_if_initialized:literal )?
    } => {{
        use core::mem::MaybeUninit;
        use core::ptr;
        use core::cell::Cell;
        use $crate::UpChannel;
        use $crate::DownChannel;
        use $crate::rtt::*;

        #[repr(C)]
        pub struct RttControlBlock {
            header: RttHeader,
            up_channels: [RttChannel; $crate::rtt_init_repeat!({ 1 + } { 0 }; $($($up)*)?)],
            down_channels: [RttChannel; $crate::rtt_init_repeat!({ 1 + } { 0 }; $($($down)*)?)],
        }

        #[used]
        #[no_mangle]
        #[export_name = "_SEGGER_RTT"]
        $( #[link_section = $section_cb] )?
        pub static mut CONTROL_BLOCK: MaybeUninit<RttControlBlock> = MaybeUninit::uninit();

        #[allow(unused)]
        #[export_name = "rtt_init_must_not_be_called_multiple_times"]
        fn rtt_init_must_not_be_called_multiple_times() { }

        use ::rtt_target::export::critical_section;

        static INITIALIZED: critical_section::Mutex<Cell<bool>> = critical_section::Mutex::new(Cell::new(false));
        critical_section::with(|cs| {
            if INITIALIZED.borrow(cs).get() {
                panic!("rtt_init! must not be called multiple times");
            }
            INITIALIZED.borrow(cs).set(true);
        });

        unsafe {
            let cb_ptr = CONTROL_BLOCK.as_mut_ptr();
            let reuse_if_initialized = false $(|| $reuse_if_initialized)?;

            let should_reuse = if reuse_if_initialized {
                let cb = &mut *cb_ptr;

                cb.header.is_valid(cb.up_channels.len(), cb.down_channels.len())
                    $(&& $crate::rtt_init_channels_are_reusable!(cb.up_channels; $($up)*))?
                    $(&& $crate::rtt_init_channels_are_reusable!(cb.down_channels; $($down)*))?
            } else {
                false
            };

            if !should_reuse {
                ptr::write_bytes(cb_ptr, 0, 1);

                let cb = &mut *cb_ptr;

                $( $crate::rtt_init_channels!(cb.up_channels; $($up)*); )?
                $( $crate::rtt_init_channels!(cb.down_channels; $($down)*); )?

                // The header is initialized last to make it less likely an unfinished control block is
                // detected by the host.

                cb.header.init(cb.up_channels.len(), cb.down_channels.len());
            }

            let cb = &mut *cb_ptr;

            pub struct Channels {
                $( pub up: $crate::rtt_init_repeat!({ UpChannel, } {}; $($up)*), )?
                $( pub down: $crate::rtt_init_repeat!({ DownChannel, } {}; $($down)*), )?
            }

            Channels {
                $( up: $crate::rtt_init_wrappers!(cb.up_channels; UpChannel::new; {}; $($up)*), )?
                $( down: $crate::rtt_init_wrappers!(cb.down_channels; DownChannel::new; {}; $($down)*), )?
            }
        }
    }};
}

/// Initializes RTT with default up/down channels.
///
/// The default channels are up channel 0 with a 1024 byte buffer and down channel 0 with a 16 byte
/// buffer. Both channels are called "Terminal". This macro is equivalent to:
///
/// ```
/// rtt_init! {
///     up: {
///         0: {
///             size: 1024,
///             name: "Terminal"
///         }
///     }
///     down: {
///         0: {
///             size: 16,
///             name: "Terminal"
///         }
///     }
/// };
/// ```
///
/// See [`rtt_init`] for more details.
#[macro_export]
macro_rules! rtt_init_default {
    () => {
        $crate::rtt_init! {
            up: {
                0: {
                    size: 1024,
                    name: "Terminal"
                }
            }
            down: {
                0: {
                    size: 16,
                    name: "Terminal"
                }
            }
        };
    };
}
