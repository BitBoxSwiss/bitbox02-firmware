# SPDX-License-Identifier: Apache-2.0

"""
This file re-exports all symbols from the nested bitbox02 subpackage.

This is needed for compatibility with pip >= 25, which changed how editable installs
resolve imports. With pip 25, "from .bitbox02 import common" inside util.py resolves
to this outer __init__.py instead of the inner bitbox02 subpackage.
"""

# Re-export all symbols from the nested package
from .bitbox02 import *  # noqa: F401, F403

# Explicitly list all public exports for type checkers and documentation
from .bitbox02 import (
    __version__,
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
    Bootloader,
)
