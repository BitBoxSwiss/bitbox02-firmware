#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Create release assertion files for BitBox02 firmware releases."""

from __future__ import annotations

import argparse
import hashlib
import subprocess
import sys
from pathlib import Path
from urllib import error, request


REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
UPSTREAM_REPOSITORY_URL = "https://github.com/BitBoxSwiss/bitbox02-firmware"
UPSTREAM_GIT_URL = f"{UPSTREAM_REPOSITORY_URL}.git"

MAGIC_LEN = 4
MAGIC_MULTI = bytes.fromhex("653f362b")
MAGIC_BTCONLY = bytes.fromhex("11233b0b")

VERSION_FIELD_LEN = 4
NUM_ROOT_KEYS = 3
NUM_SIGNING_KEYS = 3
SIGNING_PUBKEYS_DATA_LEN = VERSION_FIELD_LEN + NUM_SIGNING_KEYS * 64 + NUM_ROOT_KEYS * 64
FIRMWARE_DATA_LEN = VERSION_FIELD_LEN + NUM_SIGNING_KEYS * 64
SIGDATA_LEN = SIGNING_PUBKEYS_DATA_LEN + FIRMWARE_DATA_LEN

PRODUCTS = (
    {
        "label": "BitBox02 Multi",
        "asset_name": "firmware-bitbox02-multi.{version}.signed.bin",
        "filename": "assertion-bitbox02-multi.txt",
        "expected_magic": MAGIC_MULTI,
    },
    {
        "label": "BitBox02 Bitcoin-only",
        "asset_name": "firmware-bitbox02-btconly.{version}.signed.bin",
        "filename": "assertion-bitbox02-btconly.txt",
        "expected_magic": MAGIC_BTCONLY,
    },
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Create releases/firmware-<version>/ with BitBox02 Multi and Bitcoin-only assertion "
            "text files."
        )
    )
    parser.add_argument("version", help="Release version in vX.Y.Z format, e.g. v9.26.1")
    return parser.parse_args()


def run_git_command(*args: str) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(
            ["git", *args],
            cwd=REPOSITORY_ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
    except FileNotFoundError as exc:
        raise RuntimeError("git is required but was not found in PATH") from exc


def resolve_commit_hash_local(tag_name: str) -> str | None:
    result = run_git_command("rev-list", "-n1", tag_name)
    if result.returncode != 0:
        return None
    commit_hash = result.stdout.strip()
    return commit_hash or None


def resolve_commit_hash_remote(tag_name: str) -> str:
    ref = f"refs/tags/{tag_name}"
    dereferenced_ref = f"{ref}^{{}}"
    result = run_git_command("ls-remote", "--tags", UPSTREAM_GIT_URL, ref, dereferenced_ref)
    if result.returncode != 0:
        stderr = result.stderr.strip()
        raise RuntimeError(f"Failed to resolve remote tag '{tag_name}': {stderr}")

    resolved_refs = {}
    for line in result.stdout.splitlines():
        commit_hash, _, ref_name = line.partition("\t")
        if not ref_name:
            continue
        resolved_refs[ref_name] = commit_hash

    if dereferenced_ref in resolved_refs:
        return resolved_refs[dereferenced_ref]
    if ref in resolved_refs:
        return resolved_refs[ref]

    raise RuntimeError(f"Remote tag '{tag_name}' was not found in {UPSTREAM_GIT_URL}")


def resolve_commit_hash(version: str) -> str:
    tag_name = f"firmware/{version}"
    local_commit_hash = resolve_commit_hash_local(tag_name)
    if local_commit_hash is not None:
        return local_commit_hash
    return resolve_commit_hash_remote(tag_name)


def download_signed_firmware(version: str, asset_name: str) -> bytes:
    url = f"{UPSTREAM_REPOSITORY_URL}/releases/download/firmware/{version}/{asset_name}"
    request_headers = {"User-Agent": "bitbox02-release-assertion-generator"}
    try:
        with request.urlopen(request.Request(url, headers=request_headers)) as response:
            return response.read()
    except error.HTTPError as exc:
        raise RuntimeError(
            f"Failed to download '{asset_name}' from {url}: HTTP {exc.code}"
        ) from exc
    except error.URLError as exc:
        raise RuntimeError(f"Failed to download '{asset_name}' from {url}: {exc.reason}") from exc


def extract_unsigned_firmware_hash(
    signed_firmware: bytes, expected_magic: bytes, asset_name: str
) -> str:
    if len(signed_firmware) < MAGIC_LEN + SIGDATA_LEN:
        raise RuntimeError(f"Downloaded asset '{asset_name}' is too small to be a signed firmware")

    actual_magic = signed_firmware[:MAGIC_LEN]
    if actual_magic != expected_magic:
        raise RuntimeError(
            f"Downloaded asset '{asset_name}' has unexpected magic {actual_magic.hex()}"
        )

    firmware = signed_firmware[MAGIC_LEN + SIGDATA_LEN :]
    return hashlib.sha256(firmware).hexdigest()


def render_assertion(version: str, product_label: str, commit_hash: str, sha256_hash: str) -> str:
    return (
        f"By signing this file, the signer confirms that the {product_label} firmware binary built "
        f"from:\n\n"
        f"git tag firmware/{version}\n"
        f"git commit hash {commit_hash}\n\n"
        "resulted, at the time of signing, in a firmware binary file with the following "
        "sha256sum:\n\n"
        f"{sha256_hash}\n"
    )


def main() -> int:
    args = parse_args()

    release_directory = REPOSITORY_ROOT / "releases" / f"firmware-{args.version}"
    if release_directory.exists():
        raise RuntimeError(f"Destination already exists: {release_directory}")

    print(f"Resolving commit hash for firmware/{args.version}...")
    commit_hash = resolve_commit_hash(args.version)
    rendered_assertions: list[tuple[str, str]] = []

    for product in PRODUCTS:
        asset_name = product["asset_name"].format(version=args.version)
        print(f"Downloading {asset_name}...")
        signed_firmware = download_signed_firmware(args.version, asset_name)
        sha256_hash = extract_unsigned_firmware_hash(
            signed_firmware, product["expected_magic"], asset_name
        )
        rendered_assertions.append(
            (
                product["filename"],
                render_assertion(args.version, product["label"], commit_hash, sha256_hash),
            )
        )

    release_directory.mkdir()

    for filename, contents in rendered_assertions:
        output_path = release_directory / filename
        output_path.write_text(contents, encoding="utf-8")
        print(f"Wrote {output_path.relative_to(REPOSITORY_ROOT)}")

    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except RuntimeError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        sys.exit(1)
