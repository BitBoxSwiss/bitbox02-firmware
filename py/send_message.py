#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Compatibility wrapper for bitbox02_cli.py."""

import sys
from typing import Sequence

from bitbox02_cli import main as bitbox02_cli_main


def main(argv: Sequence[str]) -> int:
    """Forward to bitbox02_cli.py, preserving the old no-argument menu."""
    if not argv:
        return bitbox02_cli_main(["--interactive"])
    return bitbox02_cli_main(argv)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
