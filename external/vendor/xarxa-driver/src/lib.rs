#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
//!
//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

pub mod config;

mod buf;
mod meta;

#[cfg(feature = "async")]
use core::task::Waker;

pub use buf::PacketBuf;
pub use meta::PacketMeta;
#[cfg(feature = "packetmeta-timestamp")]
pub use meta::{Timestamp, TxTimestamp};

/// Error returned by an operation the driver does not support.
#[cfg(feature = "async")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct NotSupported;

/// Link state of a network device.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum LinkState {
    /// The link is down. No frames can pass.
    Down,
    /// The link is up.
    Up,
}

/// Type of medium of a network device.
///
/// This is `#[non_exhaustive]` so that media can be added later without breaking
/// every driver.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Medium {
    /// Ethernet medium. Devices of this type send and receive Ethernet frames.
    Ethernet,

    /// IP medium. Devices of this type send and receive IP frames, without an
    /// Ethernet header. MAC addresses are not used.
    Ip,

    /// IEEE 802.15.4 medium. Devices of this type send and receive 802.15.4
    /// MAC frames carrying 6LoWPAN.
    ///
    /// [`Capabilities::max_transmission_unit`] is the whole MAC frame
    /// without the FCS: 125 for a 127-byte PHY frame with a 2-byte FCS.
    Ieee802154,
}

/// A hardware (link-layer) address, as reported by a driver.
///
/// This is `#[non_exhaustive]` so that address kinds can be added later without
/// breaking every driver.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HardwareAddress {
    /// An Ethernet (MAC) address.
    Ethernet([u8; 6]),
    /// No address, for devices that send and receive bare IP packets.
    Ip,
    /// An IEEE 802.15.4 extended (64-bit) address.
    Ieee802154([u8; 8]),
}

impl HardwareAddress {
    /// The medium this kind of address belongs to.
    pub const fn medium(&self) -> Medium {
        match self {
            HardwareAddress::Ethernet(_) => Medium::Ethernet,
            HardwareAddress::Ip => Medium::Ip,
            HardwareAddress::Ieee802154(_) => Medium::Ieee802154,
        }
    }
}

/// Checksum offload capabilities for a given protocol, per direction.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChecksumOffload {
    /// The device verifies the checksum of received packets.
    ///
    /// The stack then does not verify it in software.
    pub rx: bool,
    /// The device fills in the checksum of transmitted packets.
    ///
    /// The stack then writes the field as zero instead of computing it.
    pub tx: bool,
}

impl ChecksumOffload {
    /// No offload. The stack computes and verifies the checksum in software.
    pub const NONE: Self = Self { rx: false, tx: false };
    /// Offload in both directions.
    pub const BOTH: Self = Self { rx: true, tx: true };
}

/// Checksum offload capabilities per protocol direction.
///
/// The stack skips the offloaded work in software.
///
/// The default is no offload for every protocol: the stack computes and
/// verifies everything itself.
///
/// A checksum the stack does not compute is written as zero, so that a device
/// that fills it in finds a known value there.
///
/// The fields are the protocols with a checksum the stack handles. IGMP is not
/// among them: its checksum is always computed and verified in software.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ChecksumCapabilities {
    /// Offload for the IPv4 header checksum.
    pub ipv4: ChecksumOffload,
    /// Offload for the UDP checksum.
    pub udp: ChecksumOffload,
    /// Offload for the TCP checksum.
    pub tcp: ChecksumOffload,
    /// Offload for the ICMPv4 checksum.
    pub icmpv4: ChecksumOffload,
    /// Offload for the ICMPv6 checksum.
    pub icmpv6: ChecksumOffload,
}

impl ChecksumCapabilities {
    /// Every checksum offloaded in both directions.
    ///
    /// The stack computes and verifies nothing. Use this for devices where
    /// checksums don't matter, like loopback.
    pub fn all_offloaded() -> Self {
        ChecksumCapabilities {
            ipv4: ChecksumOffload::BOTH,
            udp: ChecksumOffload::BOTH,
            tcp: ChecksumOffload::BOTH,
            icmpv4: ChecksumOffload::BOTH,
            icmpv6: ChecksumOffload::BOTH,
        }
    }
}

/// A description of a device's capabilities.
///
/// This is `#[non_exhaustive]` so that capabilities can be added later without breaking
/// every driver. Drivers live outside this crate and so cannot use a struct expression,
/// they start from [`Default`] and overwrite the fields they care about:
///
/// ```
/// # use xarxa_driver::Capabilities;
/// let mut caps = Capabilities::default();
/// caps.max_transmission_unit = 1514;
/// // caps.medium = Medium::Ethernet; is the default
/// ```
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Capabilities {
    /// Medium of the device.
    pub medium: Medium,

    /// Maximum transmission unit.
    ///
    /// The network device is unable to send or receive frames larger than the value returned
    /// by this function.
    pub max_transmission_unit: usize,

    /// Checksum offload.
    ///
    /// Which checksums the device's hardware verifies or computes, so the
    /// stack skips them in software.
    pub checksum: ChecksumCapabilities,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            medium: Medium::Ethernet,
            max_transmission_unit: 1514,
            checksum: ChecksumCapabilities::default(),
        }
    }
}

/// A network device driver, sending and receiving raw network frames.
pub trait Driver {
    /// Get a description of the device's capabilities.
    fn capabilities(&self) -> Capabilities;

    /// Get the device's hardware address.
    ///
    /// The address kind must match the medium in [`capabilities`](Self::capabilities):
    /// an Ethernet address for [`Medium::Ethernet`], [`HardwareAddress::Ip`] for
    /// [`Medium::Ip`], an IEEE 802.15.4 extended address for [`Medium::Ieee802154`].
    ///
    /// The stack reads it once, when the driver is added to it. The stack has its
    /// own way to override the address after that.
    fn hardware_address(&self) -> HardwareAddress;

    /// Get the link state.
    ///
    /// Devices that cannot tell, or whose link is always up, return
    /// [`LinkState::Up`], which is the default implementation.
    fn link_state(&mut self) -> LinkState {
        LinkState::Up
    }

    /// Register a waker.
    ///
    /// The driver must wake it when:
    /// - a frame has been received, so [`receive`](Self::receive) may return `Some`,
    /// - there is room to transmit again, after [`can_transmit`](Self::can_transmit) returned `false`,
    /// - the link state changed, so [`link_state`](Self::link_state) may return something new.
    ///
    /// Only one waker is kept. Registering another replaces it. Wakes are
    /// allowed to be spurious.
    ///
    /// A registered waker is woken just one. The main loop must re-register it if
    /// it wants to be woken again.
    ///
    /// Drivers that cannot wake anything return `Err(NotSupported)`, which is the
    /// default implementation. Such a driver can only be polled, so a caller that
    /// needs to sleep until the driver has something new cannot use it.
    #[cfg(feature = "async")]
    fn register_waker(&mut self, waker: &Waker) -> Result<(), NotSupported> {
        let _ = waker;
        Err(NotSupported)
    }

    /// Poll for a received frame.
    ///
    /// Returns a buffer holding the received frame if one is available, transferring
    /// ownership of it to the caller.
    ///
    /// A driver that has per-packet metadata to report, such as an identifier or a
    /// receive timestamp, sets it on the buffer's [`PacketMeta`] here. It travels
    /// with the packet up to the socket that receives it.
    fn receive(&mut self) -> Option<PacketBuf>;

    /// Whether the device can transmit one frame right now.
    ///
    /// Devices typically have a transmit packet queue. This returns
    /// whether this queue has space to take one more frame.
    ///
    /// If this returns `true`, the next `transmit()` call must not fail.
    ///
    /// In devices where there's no queue so transmit always succeeds, this
    /// should always return `true`.
    fn can_transmit(&mut self) -> bool;

    /// Queue a frame for transmission, transferring ownership of the buffer to the driver.
    ///
    /// The driver holds the buffer until the hardware is done with it, then drops it.
    /// If the frame cannot be queued right now (device busy or queue full), the buffer
    /// is handed back in the `Err` variant.
    ///
    /// The buffer's [`PacketMeta`] is whatever the sending socket attached to the
    /// packet (default for packets the stack generates itself). A driver that
    /// supports transmit timestamping timestamps the frame if
    /// [`request_timestamp`](PacketMeta::request_timestamp) is set, and reports
    /// the result from [`poll_tx_timestamp`](Self::poll_tx_timestamp) tagged with the
    /// packet's [`id`](PacketMeta::id).
    fn transmit(&mut self, buf: PacketBuf) -> Result<(), PacketBuf>;

    /// Poll for the timestamp of an already-transmitted packet.
    ///
    /// Returns the transmit timestamp of a packet previously sent with
    /// [`PacketMeta::request_timestamp`] set, tagged with that packet's
    /// [`PacketMeta::id`], or `None` if no timestamp is available right now.
    ///
    /// Transmit timestamps are reported out of band, rather than on the packet like
    /// receive timestamps are, because a packet's transmit timestamp does not exist yet
    /// when [`transmit`](Self::transmit) returns: it has not gone out on the wire yet.
    ///
    /// Callers must be robust against all of the following:
    ///
    /// * Timestamps become available an arbitrary time after `transmit` returned, so
    ///   this should be polled repeatedly, not just once after sending.
    /// * Timestamps may be reported out of order with respect to transmission.
    /// * Timestamps may never arrive at all, e.g. because the hardware ran out of
    ///   timestamp slots. Never block waiting for a particular `id` to show up without
    ///   a timeout.
    ///
    /// Devices that do not support transmit timestamping always return `None`, which is
    /// the default implementation.
    #[cfg(feature = "packetmeta-timestamp")]
    fn poll_tx_timestamp(&mut self) -> Option<TxTimestamp> {
        None
    }

    /// Set the device's multicast hardware address filter.
    ///
    /// `addrs` is the full list of multicast MAC addresses to listen on. It
    /// replaces the previous one.
    ///
    /// A device with no multicast filter can ignore the calls, which
    /// is the default implementation.
    ///
    /// Only called for [`Medium::Ethernet`] devices. The list has no duplicates.
    ///
    /// It may be the same list as last time: the stack does not compare. If
    /// applying the filter is expensive, the driver should should keep the last
    /// list and skip if there were no changes.
    ///
    /// If the list does not fit the filter, receive the addresses anyway if
    /// possible, for example by turning the filter off or switching it to
    /// receive all multicast. Losing filter efficiency is fine, filtering out
    /// traffic the network stack wants is not.
    fn set_multicast_filter(&mut self, addrs: &[[u8; 6]]) {
        let _ = addrs;
    }
}

impl<T: Driver + ?Sized> Driver for &mut T {
    fn capabilities(&self) -> Capabilities {
        T::capabilities(self)
    }
    fn hardware_address(&self) -> HardwareAddress {
        T::hardware_address(self)
    }
    fn link_state(&mut self) -> LinkState {
        T::link_state(self)
    }
    #[cfg(feature = "async")]
    fn register_waker(&mut self, waker: &Waker) -> Result<(), NotSupported> {
        T::register_waker(self, waker)
    }
    fn receive(&mut self) -> Option<PacketBuf> {
        T::receive(self)
    }
    fn can_transmit(&mut self) -> bool {
        T::can_transmit(self)
    }
    fn transmit(&mut self, buf: PacketBuf) -> Result<(), PacketBuf> {
        T::transmit(self, buf)
    }
    #[cfg(feature = "packetmeta-timestamp")]
    fn poll_tx_timestamp(&mut self) -> Option<TxTimestamp> {
        T::poll_tx_timestamp(self)
    }
    fn set_multicast_filter(&mut self, addrs: &[[u8; 6]]) {
        T::set_multicast_filter(self, addrs)
    }
}
