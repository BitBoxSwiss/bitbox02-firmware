#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Exercise the BitBox03 USB HID echo firmware (requires the usb-echo feature)."""

import argparse

import hid


def main() -> None:
    """Send and verify complete, unnumbered 64-byte HID reports."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--reports", type=int, default=256)
    args = parser.parse_args()
    if args.reports <= 0:
        parser.error("--reports must be positive")

    devices = [
        device
        for device in hid.enumerate(0x03EB, 0x2403)
        if device["product_string"] == "BitBox03 USB echo" and device["interface_number"] in (0, -1)
    ]
    if len(devices) != 1:
        raise RuntimeError(f"Expected one BitBox03 USB echo device, found {len(devices)}")

    device = hid.device()
    try:
        device.open_path(devices[0]["path"])
        for index in range(args.reports):
            report = bytes((index + offset) % 256 for offset in range(64))
            # hidapi requires a zero report-ID prefix for an unnumbered report.
            if device.write(b"\x00" + report) != 65:
                raise RuntimeError(f"Short HID write for report {index}")
            received = bytes(device.read(64, timeout_ms=2000))
            if received != report:
                raise RuntimeError(f"Incorrect or missing response to report {index}")
    finally:
        device.close()
    print(f"Verified {args.reports} USB HID report round trips")


if __name__ == "__main__":
    main()
