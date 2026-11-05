#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("You may not enable both `defmt` and `log` features.");

#[cfg(feature = "log")]
#[macro_use]
mod log {
    macro_rules! fmc_log {
        (trace, $($arg:expr),*) => { log::trace!($($arg),*); };
    }
}

#[cfg(feature = "defmt")]
#[macro_use]
mod log {
    macro_rules! fmc_log {
        (trace, $($arg:expr),*) => { ::defmt::trace!($($arg),*); };
    }
}

#[cfg(all(not(feature = "log"), not(feature = "defmt")))]
#[macro_use]
mod log {
    macro_rules! fmc_log {
        ($level:ident, $($arg:expr),*) => { $( let _ = $arg; )* }
    }
}

macro_rules! fmc_trace {
    ($($arg:expr),*) => (fmc_log!(trace, $($arg),*));
}
