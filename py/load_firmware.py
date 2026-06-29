#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Compatibility wrapper for bitbox02_cli.py firmware flashing."""

import sys
from typing import List, Sequence

from bitbox02_cli import main as bitbox02_cli_main


def _translate_args(argv: Sequence[str]) -> List[str]:
    global_args = []
    flash_args = []
    for arg in argv:
        if arg == "--debug":
            flash_args.append("--unsigned")
        elif arg == "--no-cache":
            global_args.append(arg)
        else:
            flash_args.append(arg)
    return global_args + ["firmware", "flash"] + flash_args


def main(argv: Sequence[str]) -> int:
    """Forward legacy load_firmware.py args to bitbox02_cli.py."""
    return bitbox02_cli_main(_translate_args(argv))


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
