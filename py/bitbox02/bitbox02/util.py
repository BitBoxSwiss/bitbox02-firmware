# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""Useful functions"""

import base58

from .bitbox02 import common


def parse_xpub(xpub: str) -> common.XPub:
    """
    Parse an xpub to a protobuf XPub.
    The version is stripped, so the xpub can be any format (xpub, ypub, etc.).
    """

    decoded = base58.b58decode_check(xpub)
    decoded = decoded[4:]
    depth, decoded = decoded[:1], decoded[1:]
    parent_fp, decoded = decoded[:4], decoded[4:]
    child_num, decoded = decoded[:4], decoded[4:]
    chain_code, decoded = decoded[:32], decoded[32:]
    pubkey, decoded = decoded[:33], decoded[33:]
    assert len(decoded) == 0
    return common.XPub(
        depth=depth,
        parent_fingerprint=parent_fp,
        child_num=int.from_bytes(child_num, "big"),
        chain_code=chain_code,
        public_key=pubkey,
    )
