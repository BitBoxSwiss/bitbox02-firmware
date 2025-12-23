# SPDX-License-Identifier: Apache-2.0

"""Library to interact with a BitBox02 device."""

from __future__ import print_function
import sys

__version__ = "7.1.0"

if sys.version_info.major != 3 or sys.version_info.minor < 6:
    print(
        "Python version is {}.{}, but 3.6+ is required by this script.".format(
            sys.version_info.major, sys.version_info.minor
        ),
        file=sys.stderr,
    )
    sys.exit(1)

try:
    import hid

    hid.device  # pylint: disable=pointless-statement
except AttributeError:
    print(
        "Unable to reference hid.device; maybe hid package is masking "
        "hidapi? Try:\n\t$ pip3 uninstall hid",
        file=sys.stderr,
    )
    sys.exit(1)

# pylint: disable=wrong-import-position
from .bitbox02 import (
    btc_sign_needs_prevtxs,
    Backup,
    BitBox02,
    BTCInputType,
    BTCOutputExternal,
    BTCOutputInternal,
    BTCOutputType,
    BTCPrevTxInputType,
    BTCPrevTxOutputType,
    DuplicateEntryException,
    hww,
    btc,
    cardano,
    common,
    eth,
    system,
)
from .bootloader import Bootloader
