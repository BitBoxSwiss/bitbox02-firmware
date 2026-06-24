#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

import argparse
import hashlib
import struct
from pathlib import Path

import ecdsa


STAGE1_HEADER_MAGIC = 0x31534242
STAGE0_DESCRIPTOR_MAGIC = 0x30534242
STAGE1_HEADER_FORMAT_VERSION = 1
STAGE1_HEADER_LEN = 1024
STAGE1_HEADER_ALIGNMENT = 1024
BOOTLOADER_UPGRADE_STAGE0_LEN = 0x2000
STAGE0_DESCRIPTOR_LEN = 12
STAGE0_DESCRIPTOR_OFFSET = BOOTLOADER_UPGRADE_STAGE0_LEN - STAGE0_DESCRIPTOR_LEN
STAGE0_DESCRIPTOR_FORMAT = "<HHII"
STAGE1_VECTOR_OFFSET = 0x400
STAGE1_MAX_LEN = 0xBFE0
BOOTLOADER_UPGRADE_PAYLOAD_LEN = 0xC000
STAGE1_ROOT_KEY_COUNT = 3
STAGE1_SIGNATURE_THRESHOLD = 2
STAGE1_SIGNATURE_LEN = 64
STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN = 37
STAGE0_DESCRIPTOR_FLAG_DEVELOPMENT = 1 << 0
STAGE1_HEADER_FLAG_DEVELOPMENT = 1 << 0
STAGE1_HEADER_ALLOWED_FLAGS = STAGE1_HEADER_FLAG_DEVELOPMENT
STAGE1_HEADER_RESERVED_LEN = 768
STAGE1_HEADER_PREFIX_FORMAT = (
    f"<IIHHIQHB{STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN}s{STAGE1_HEADER_RESERVED_LEN}s"
)
STAGE1_HEADER_SIGNED_LEN = struct.calcsize(STAGE1_HEADER_PREFIX_FORMAT)
STAGE1_HEADER_SIGNATURES_LEN = STAGE1_ROOT_KEY_COUNT * STAGE1_SIGNATURE_LEN
PRODUCT_IDS = {
    "bitbox-multi": 1,
    "bitbox-btconly": 2,
    "bitbox-plus-multi": 3,
    "bitbox-plus-btconly": 4,
}
# Keep in sync with src/bootloader_upgrade/stage1_pubkeys.c.
STAGE1_ROOT_PUBKEYS = (
    bytes.fromhex(
        "3a2d538f0e6db286287f5dfbf3046c2b436ead5f0153b0becb4561956016220e"
        "750e49a7a4ba412ecace07f286c0b34f6a0eb2d952e396a3ebabda4355d8e677"
    ),
    bytes.fromhex(
        "499370daa90cb008804237c62c7db4cb54eefed0430a3dcde7de57a61ae64ad3"
        "bb163a031ab2cc5647aa74e261c023effede98e64bbe58b019fb4f7180f6872f"
    ),
    bytes.fromhex(
        "4861aeb6b10526b73e97c6807918e9de8b99d498844c544cf22a6449a2120cf2"
        "9011f7eecc147f56f64dfae32e963bebd3408ee5120cd87123cf4db96e936c04"
    ),
)
STAGE1_ROOT_VERIFYING_KEYS = [
    ecdsa.VerifyingKey.from_string(pubkey, curve=ecdsa.NIST256p) for pubkey in STAGE1_ROOT_PUBKEYS
]

assert STAGE1_HEADER_SIGNED_LEN == 832
assert STAGE1_HEADER_SIGNED_LEN + STAGE1_HEADER_SIGNATURES_LEN == STAGE1_HEADER_LEN
assert STAGE1_VECTOR_OFFSET == 1024
assert STAGE1_VECTOR_OFFSET == STAGE1_HEADER_LEN
assert struct.calcsize(STAGE0_DESCRIPTOR_FORMAT) == STAGE0_DESCRIPTOR_LEN
assert len(STAGE1_ROOT_VERIFYING_KEYS) == STAGE1_ROOT_KEY_COUNT
assert all(len(pubkey) == 64 for pubkey in STAGE1_ROOT_PUBKEYS)


def _parse_product_id(value: str) -> int:
    if value in PRODUCT_IDS:
        return PRODUCT_IDS[value]
    product_id = int(value, 0)
    if product_id < 0 or product_id > 0xFFFF:
        raise argparse.ArgumentTypeError("product id must fit in uint16_t")
    return product_id


def _decode_stage1_marketing_version(
    stage1_marketing_version_len: int, stage1_marketing_version_field: bytes
) -> str:
    if (
        stage1_marketing_version_len == 0
        or stage1_marketing_version_len > STAGE1_HEADER_STAGE1_MARKETING_VERSION_MAX_LEN
    ):
        raise RuntimeError("invalid stage1 marketing version length")
    version = stage1_marketing_version_field[:stage1_marketing_version_len]
    padding = stage1_marketing_version_field[stage1_marketing_version_len:]
    if any(padding):
        raise RuntimeError("stage1 marketing version padding is not zero")
    if any(ch < 0x21 or ch > 0x7E for ch in version):
        raise RuntimeError("stage1 marketing version contains non-printable bytes")
    return version.decode("ascii")


def _unpack_header(header: bytes) -> dict:
    if len(header) != STAGE1_HEADER_LEN:
        raise RuntimeError("invalid header length")
    values = struct.unpack(STAGE1_HEADER_PREFIX_FORMAT, header[:STAGE1_HEADER_SIGNED_LEN])
    sigs = []
    for i in range(STAGE1_ROOT_KEY_COUNT):
        start = STAGE1_HEADER_SIGNED_LEN + i * STAGE1_SIGNATURE_LEN
        sigs.append(header[start : start + STAGE1_SIGNATURE_LEN])
    return {
        "prefix": header[:STAGE1_HEADER_SIGNED_LEN],
        "magic": values[0],
        "flags": values[1],
        "header_version": values[2],
        "product_id": values[3],
        "header_len": values[4],
        "image_len": values[5],
        "monotonic_version": values[6],
        "stage1_marketing_version_len": values[7],
        "stage1_marketing_version_field": values[8],
        "stage1_marketing_version": _decode_stage1_marketing_version(values[7], values[8]),
        "reserved": values[9],
        "signatures": sigs,
    }


def _pack_prefix(header: dict) -> bytes:
    return struct.pack(
        STAGE1_HEADER_PREFIX_FORMAT,
        STAGE1_HEADER_MAGIC,
        header["flags"],
        STAGE1_HEADER_FORMAT_VERSION,
        header["product_id"],
        header["header_len"],
        header["image_len"],
        header["monotonic_version"],
        header["stage1_marketing_version_len"],
        header["stage1_marketing_version_field"],
        header["reserved"],
    )


def _pack_header(prefix: bytes, sigs: list[bytes]) -> bytes:
    if len(prefix) != STAGE1_HEADER_SIGNED_LEN:
        raise RuntimeError("invalid signed header prefix length")
    if len(sigs) != STAGE1_ROOT_KEY_COUNT or any(len(sig) != STAGE1_SIGNATURE_LEN for sig in sigs):
        raise RuntimeError("invalid header signatures")
    return prefix + b"".join(sigs)


def _stage1_signed_digest(image: bytes) -> bytes:
    header = _unpack_header(image[:STAGE1_HEADER_LEN])
    if len(image) <= header["header_len"]:
        raise RuntimeError("stage1 image does not contain a vector table")
    signed_header_len = header["header_len"] - STAGE1_HEADER_SIGNATURES_LEN
    hasher = hashlib.sha256()
    hasher.update(image[:signed_header_len])
    hasher.update(image[header["header_len"] :])
    return hasher.digest()


def _signatures_are_zero(header: dict) -> bool:
    return not any(byte for signature in header["signatures"] for byte in signature)


def _verify_header_signatures(header: dict, image: bytes) -> None:
    digest = _stage1_signed_digest(image)
    valid = 0
    for verifying_key, signature in zip(STAGE1_ROOT_VERIFYING_KEYS, header["signatures"]):
        try:
            if verifying_key.verify(
                signature,
                digest,
                hashfunc=hashlib.sha256,
                sigdecode=ecdsa.util.sigdecode_string,
            ):
                valid += 1
        except ecdsa.BadSignatureError:
            pass
    if valid < STAGE1_SIGNATURE_THRESHOLD:
        raise RuntimeError("stage1 header signatures do not verify")


def _validate_fixed_fields(header: dict, expected_product_id: int | None = None) -> None:
    if header["magic"] != STAGE1_HEADER_MAGIC:
        raise RuntimeError("invalid header magic")
    if header["header_version"] != STAGE1_HEADER_FORMAT_VERSION:
        raise RuntimeError("invalid header version")
    if header["product_id"] not in PRODUCT_IDS.values():
        raise RuntimeError("invalid product id")
    if expected_product_id is not None and header["product_id"] != expected_product_id:
        raise RuntimeError("product id mismatch")
    if header["flags"] & ~STAGE1_HEADER_ALLOWED_FLAGS:
        raise RuntimeError("invalid stage1 header flags")
    if header["header_len"] != STAGE1_HEADER_LEN:
        raise RuntimeError("invalid stage1 header length")
    if header["header_len"] % STAGE1_HEADER_ALIGNMENT != 0:
        raise RuntimeError("invalid stage1 header alignment")
    if any(header["reserved"]):
        raise RuntimeError("reserved header bytes are not zero")


def _validate_raw_stage1(image: bytes) -> dict:
    header = _unpack_header(image[:STAGE1_HEADER_LEN])
    _validate_fixed_fields(header)
    if header["image_len"] != 0:
        raise RuntimeError("raw stage1 image length field is not zero")
    if not _signatures_are_zero(header):
        raise RuntimeError("raw stage1 signatures are not zero")
    if len(image) <= header["header_len"] or len(image) > STAGE1_MAX_LEN:
        raise RuntimeError("raw stage1 image length is invalid")
    return header


def _validate_complete_stage1(
    image: bytes,
    expected_product_id: int | None,
    require_signatures: bool,
) -> dict:
    header = _unpack_header(image[:STAGE1_HEADER_LEN])
    _validate_fixed_fields(header, expected_product_id)
    if header["image_len"] != len(image):
        raise RuntimeError("image length does not match header")
    if header["image_len"] <= header["header_len"] or header["image_len"] > STAGE1_MAX_LEN:
        raise RuntimeError("stage1 image length is invalid")
    if require_signatures:
        _verify_header_signatures(header, image)
    elif not _signatures_are_zero(header):
        raise RuntimeError("unsigned stage1 signatures are not zero")
    return header


def _write_if_changed(path: Path, data: bytes) -> None:
    if path.exists() and path.read_bytes() == data:
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)


def prepare_stage1_unsigned(args) -> None:
    image = Path(args.raw_bin).read_bytes()
    header = _validate_raw_stage1(image)
    header["image_len"] = len(image)
    prefix = _pack_prefix(header)
    unsigned_stage1 = (
        _pack_header(prefix, [b"\x00" * STAGE1_SIGNATURE_LEN] * STAGE1_ROOT_KEY_COUNT)
        + image[STAGE1_HEADER_LEN:]
    )
    _validate_complete_stage1(unsigned_stage1, None, require_signatures=False)
    _write_if_changed(Path(args.unsigned_bin), unsigned_stage1)


def _stage1_expected_flags(development: bool) -> int:
    return STAGE1_HEADER_FLAG_DEVELOPMENT if development else 0


def _stage0_expected_flags(development: bool) -> int:
    return STAGE0_DESCRIPTOR_FLAG_DEVELOPMENT if development else 0


def _update_payload(signed_stage1: bytes, product_id: int, development: bool) -> bytes:
    header = _validate_complete_stage1(signed_stage1, product_id, require_signatures=True)
    if header["flags"] != _stage1_expected_flags(development):
        raise RuntimeError("unexpected stage1 update payload flags")
    if len(signed_stage1) > BOOTLOADER_UPGRADE_PAYLOAD_LEN:
        raise RuntimeError(
            f"stage1 update payload is {len(signed_stage1)} bytes, max is {BOOTLOADER_UPGRADE_PAYLOAD_LEN}"
        )
    return signed_stage1 + b"\xff" * (BOOTLOADER_UPGRADE_PAYLOAD_LEN - len(signed_stage1))


def create_stage1_fw_embedding(args) -> None:
    payload = _update_payload(Path(args.signed_bin).read_bytes(), args.product_id, args.development)
    _write_if_changed(Path(args.out_bin), payload)


def _validate_stage0(stage0: bytes, product_id: int, development: bool) -> bytes:
    if len(stage0) > BOOTLOADER_UPGRADE_STAGE0_LEN:
        raise RuntimeError(
            f"stage0 image is {len(stage0)} bytes, max is {BOOTLOADER_UPGRADE_STAGE0_LEN}"
        )
    stage0 = stage0 + b"\xff" * (BOOTLOADER_UPGRADE_STAGE0_LEN - len(stage0))
    _descriptor_version, descriptor_product_id, flags, magic = struct.unpack_from(
        STAGE0_DESCRIPTOR_FORMAT, stage0, STAGE0_DESCRIPTOR_OFFSET
    )
    if magic != STAGE0_DESCRIPTOR_MAGIC:
        raise RuntimeError("invalid stage0 descriptor magic")
    if descriptor_product_id != product_id:
        raise RuntimeError("stage0 descriptor product id mismatch")
    if flags != _stage0_expected_flags(development):
        raise RuntimeError("stage0 descriptor flags mismatch")
    return stage0


def create_stage0_fw_embedding(args) -> None:
    stage0 = _validate_stage0(
        Path(args.stage0_bin).read_bytes(),
        args.product_id,
        args.development,
    )
    _write_if_changed(Path(args.out_bin), stage0)


def main() -> None:
    parser = argparse.ArgumentParser(
        description=(
            "Prepare and validate stage0/stage1 bootloader-upgrade binaries. "
            "The firmware embedding commands create binary inputs for CMake/objcopy; "
            "they do not modify or link firmware images themselves."
        )
    )
    subparsers = parser.add_subparsers()

    prepare_parser = subparsers.add_parser(
        "prepare-stage1-unsigned",
        help="create an unsigned stage1 image from a raw linked stage1 binary",
        description=(
            "Validate the raw stage1 binary produced by objcopy, fill the stage1 "
            "image_len header field with the actual image length, keep the signature "
            "array zeroed, and write the canonical unsigned stage1 image that is "
            "ready to be signed."
        ),
    )
    prepare_parser.add_argument("--raw-bin", required=True)
    prepare_parser.add_argument("--unsigned-bin", required=True)
    prepare_parser.set_defaults(func=prepare_stage1_unsigned)

    update_parser = subparsers.add_parser(
        "create-stage1-fw-embedding",
        help="create the stage1 update payload consumed by blupgrade firmware",
        description=(
            "Validate a signed stage1 image for the requested product and production "
            "or development mode, then pad it with erased-flash bytes to the fixed "
            "bootloader update slot size. The output is an intermediate binary that "
            "CMake converts into a firmware object file to be linked into the upgrader firmware."
        ),
    )
    update_parser.add_argument("--signed-bin", required=True)
    update_parser.add_argument("--out-bin", required=True)
    update_parser.add_argument("--product-id", type=_parse_product_id, required=True)
    update_parser.add_argument("--development", action="store_true")
    update_parser.set_defaults(func=create_stage1_fw_embedding)

    embed_parser = subparsers.add_parser(
        "create-stage0-fw-embedding",
        help="create the stage0 image blob consumed by blupgrade firmware",
        description=(
            "Validate a prebuilt stage0 image descriptor for the requested product "
            "and production or development mode, then pad the image to the fixed "
            "stage0 size. The output is an intermediate binary that CMake converts "
            "into a firmware object file to be linked into the upgrader firmware."
        ),
    )
    embed_parser.add_argument("--stage0-bin", required=True)
    embed_parser.add_argument("--out-bin", required=True)
    embed_parser.add_argument("--product-id", type=_parse_product_id, required=True)
    embed_parser.add_argument("--development", action="store_true")
    embed_parser.set_defaults(func=create_stage0_fw_embedding)

    args = parser.parse_args()
    if not hasattr(args, "func"):
        parser.error("missing command")
    args.func(args)


if __name__ == "__main__":
    main()
