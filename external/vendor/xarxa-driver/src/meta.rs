//! Per-packet metadata.
//!
//! A [`PacketMeta`] is a small set of attributes that travels together with a packet
//! all the way through the stack.
//!
//! Every field has a corresponding Cargo feature. When none are enabled `PacketMeta` is zero-sized.
//!
//! Use cases:
//!
//! * **Tagging.** [`PacketMeta::id`] (`packetmeta-id`) is an opaque number you
//!   can use to correlate packets between the driver and sockets.
//! * **Receive timestamping.** [`PacketMeta::timestamp`] (`packetmeta-timestamp`)
//!   contains the time the packet was received, measured by the driver's own clock.
//! * **Transmit timestamping.** [`PacketMeta::request_timestamp`] asks the driver to
//!   timestamp a packet as it goes out. The result comes back through
//!   [`Driver::poll_tx_timestamp`](crate::Driver::poll_tx_timestamp), as a
//!   [`TxTimestamp`] tagged with the packet's `id`.

/// A reading of a device's own clock.
///
/// This is *different* from the clock the stack is polled with. Do not mix
/// `Timestamp` values with readings of that clock.
///
/// This is typically a higher-precision clock in the MAC.
#[cfg(feature = "packetmeta-timestamp")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct Timestamp {
    /// Whole seconds.
    pub seconds: u32,
    /// Fraction of a second, in units of 0.25 nanoseconds.
    ///
    /// Always less than `4_000_000_000`, i.e. less than one whole second. The quarter
    /// nanosecond is the resolution of the sub-second counter in common PTP hardware.
    pub quarter_nanos: u32,
}

#[cfg(feature = "packetmeta-timestamp")]
impl Timestamp {
    /// Construct a timestamp from whole seconds and nanoseconds.
    pub const fn from_seconds_and_nanos(seconds: u32, nanos: u32) -> Self {
        Self {
            seconds,
            quarter_nanos: nanos << 2,
        }
    }

    /// The fraction of a second, in whole nanoseconds, rounded down.
    pub const fn nanos(&self) -> u32 {
        self.quarter_nanos >> 2
    }
}

/// Metadata associated with a packet.
///
/// This struct is `#[non_exhaustive]`. Start from [`Default`] and set what you
/// care about:
///
/// ```
/// let mut meta = xarxa_driver::PacketMeta::default();
/// # #[cfg(feature = "packetmeta-id")] {
/// meta.id = 15;
/// # }
/// ```
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
#[non_exhaustive]
pub struct PacketMeta {
    /// An opaque identifier for this packet.
    ///
    /// On received packets it is set by the driver. On packets to transmit it is set
    /// by the application and handed to the driver untouched. It is also used
    /// to correlates a transmit timestamp back to the packet that produced it, see
    /// [`Driver::poll_tx_timestamp`](crate::Driver::poll_tx_timestamp).
    ///
    /// Defaults to `0`.
    #[cfg(feature = "packetmeta-id")]
    pub id: u32,

    /// The time at which this packet was received, as measured by the device.
    ///
    /// `None` if the device did not timestamp this packet. Devices commonly timestamp
    /// only a subset of received packets, e.g. only PTP event messages.
    ///
    /// Meaningful on received packets only. It is ignored on packets to transmit.
    #[cfg(feature = "packetmeta-timestamp")]
    pub timestamp: Option<Timestamp>,

    /// Request that the device timestamp this packet as it is transmitted.
    ///
    /// The timestamp is reported back later, out of band, by
    /// [`Driver::poll_tx_timestamp`](crate::Driver::poll_tx_timestamp),
    /// tagged with this packet's [`id`](Self::id).
    ///
    /// Meaningful on packets to transmit only, ignored on received packets.
    ///
    /// Timestamping is opt-in per packet because hardware typically has only a handful
    /// of transmit timestamp slots: requesting one for every packet will cause most of
    /// them to be dropped.
    #[cfg(feature = "packetmeta-timestamp")]
    pub request_timestamp: bool,
}

/// The timestamp of a transmitted packet, reported by
/// [`Driver::poll_tx_timestamp`](crate::Driver::poll_tx_timestamp).
#[cfg(feature = "packetmeta-timestamp")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TxTimestamp {
    /// The [`PacketMeta::id`] of the packet this timestamp belongs to.
    pub id: u32,

    /// The time at which the packet was transmitted, as measured by the device.
    pub timestamp: Timestamp,
}
