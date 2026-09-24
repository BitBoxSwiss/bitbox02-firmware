# BitBox03 USB bring-up

The STM32U5A9J-DK firmware enumerates one vendor-defined HID interface using
`embassy-usb` and the Rust Synopsys OTG driver. It retains the existing HWW VID/PID
(`03eb:2403`), usage page (`ffff`), and unnumbered 64-byte input/output reports.
The product string is `BitBox03`. The integrated HS PHY operates at full
speed (12 Mbit/s), matching the descriptors and existing HWW transport. High-speed
enumeration needs additional descriptor support and is not enabled.

The firmware routes reports through `bitbox-u2fhid` to the existing HWW
handler with the BitBox03 HAL. HWW discovery and session reset are available;
wallet commands requiring unimplemented HAL services (including persistent
storage, randomness, and the secure chip) remain unsupported and can panic.
Discovery reports an uninitialized device until persistent storage is implemented.
U2F and bootloader updates are not connected.

## Build and test

From the repository root:

```sh
source ~/.venv/bin/activate
./scripts/dev_exec.sh make bitbox03-firmware-debug
```

Load the resulting firmware using the normal BitBox03 debug/flash workflow, then
connect the board's MCU USB-C port to the host. The ST-LINK port is a separate
USB device and is not the port under test. Check enumeration and HWW communication
after unplug/replug while keeping ST-LINK power connected, and
after host suspend/resume. Test both a high-speed connection and a full-speed
hub. Physical enumeration, transfer, and power-cycle checks require a board;
host unit tests exercise descriptors, report validation, message fragmentation,
backpressure, idle timeouts, and reconnect cleanup with a fake driver.

## Integration

- `bitbox-usb` owns the HID descriptors and exposes `HwwHid::ready`, `read`, and
  `write`. Reads return a complete `[u8; 64]` or an error; short reports are rejected.
  `HwwHid::run` connects reports to HWW framing, ticks its timeouts, and polls the
  pending wallet task even when USB I/O is waiting. Run `UsbDevice::run()`
  concurrently with report processing on the same thread.
- `bitbox-platform-stm32u5::usb` adapts Embassy's Synopsys driver to the existing
  PAC and interrupt table. USB has nine endpoint slots and a 1024-word FIFO. The
  OUT buffer has static storage so interrupts cannot reference freed memory.
- `bitbox-board-stm32u5a9j-dk::usb` sets PA11/PA12 to analog mode, enables the
  board's 16 MHz HSE crystal, and senses VBUS on PG1 through ADC4 channel 8. The
  board's 330 kOhm / 27 kOhm divider puts about 0.38 V on PG1 at 5 V VBUS, so a
  digital GPIO read cannot detect attachment. ADC4 continuously samples this
  voltage; its analog watchdog signals attachment above 4.0 V and removal below
  3.6 V, using the board's 1.8 V ADC reference. ADC4 is reserved for USB. The existing
  160 MHz MSI/PLL system clock and ST startup remain in place. USB remote wakeup
  is disabled.

The platform adapter reports power events directly from the ADC-based VBUS
detector and updates the controller's VBUS override to match. Attachment does
not wait for a controller session interrupt while the core is soft-disconnected.

Debug RTT output reports ADC startup, USB power/PHY/core initialization, the
VBUS state, and bus events. `Starting BitBox03 USB HID` only announces startup;
`USB: attached to host` means the device has enabled its connection. A subsequent
`Reset` event and device-address assignment show that the host has begun
enumeration. If enumeration fails, capture these messages from reset along with
the host's USB log (for example, `sudo dmesg -w` on Linux).

The Embassy USB stack and its coupled driver crates are pinned in
`src/rust/Cargo.toml` to an upstream revision with reset/reconnect fixes that
discard buffered data from the previous USB session and abort interrupted
transfers. The dependencies are vendored for the normal build workflow.

The firmware runs USB and HWW processing on `bitbox-executor`. A fixed 128 KiB
heap backs executor tasks and transport allocations. The platform HAL tick
provides elapsed milliseconds, extended across its 32-bit wrap for HWW timeouts.
USB reset, deconfiguration, and disconnect discard partial requests and pending
responses, cancel the wallet task, and reset the Noise/UI session. USB lifecycle
events are tracked even when the host reconfigures before the HWW task runs again.
