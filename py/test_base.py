#!/usr/bin/python3
"""Basic test script"""

import time
import bitbox02


def main() -> None:
    """Main function"""
    device = bitbox02.get_any_bitbox02()

    def show_pair(_code: str) -> None:
        pass

    def att_check(_result: bool) -> None:
        pass

    bitbox = bitbox02.BitBox02(device, show_pair, att_check)
    # bitbox.display_base32(bytes([0xee] * 32))
    bitbox.display_status()
    time.sleep(10)
    bitbox.base_set_config("hejhopp")
    bitbox.display_status()


if __name__ == "__main__":
    main()
