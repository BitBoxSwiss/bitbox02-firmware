# SPDX-License-Identifier: Apache-2.0

"""Abstraction for the transport layer used for transmitting U2F messages."""

from .communication import PhysicalLayer, TransportLayer
from .bitbox_api_protocol import (
    BitBoxNoiseConfig,
    BitBoxCommonAPI,
    Bitbox02Exception,
    UserAbortException,
    AttestationException,
    FirmwareVersionOutdatedException,
    LibraryVersionOutdatedException,
    ERR_DUPLICATE_ENTRY,
    ERR_GENERIC,
    ERR_USER_ABORT,
    HARDENED,
)
