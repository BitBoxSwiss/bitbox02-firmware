#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Create a draft GitHub release for BitBox02 firmware."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from string import Template
from typing import Sequence
from urllib.parse import quote

import signed_firmware as firmware_format


REPOSITORY = "BitBoxSwiss/bitbox02-firmware"
REPOSITORY_URL = f"https://github.com/{REPOSITORY}"

VERSION_PATTERN = re.compile(r"v?(\d+\.\d+\.\d+)\Z")
SIMULATOR_ASSET_PATTERN = "bitbox02-multi-{version}-simulator1.0.0-linux-amd64"

FIRMWARE_RELEASE_TEMPLATE = Template(
    """**Release notes:**

$changelog_entries

**Note:**

When upgrading from a firmware with version v9.17.0 or below, you first must install and run [firmware v9.17.1](${repository_url}/releases/tag/firmware%2Fv9.17.1).

When upgrading from a firmware with version v9.26.1 or below, you first must install and run [firmware v9.26.2](${repository_url}/releases/tag/firmware%2Fv9.26.2).

The [BitBoxApp](https://bitbox.swiss/app/) automatically performs these steps.

**Simulator:**

This release contains a simulator executable. Its primary use case is integration testing.

**Verify this release:**

Please see the [instructions here](${repository_url}/tree/master/releases) on how to reproduce this binary.

Alternatively, [verify the signatures](${repository_url}/tree/master/releases/firmware-${version}) from the community to verify this build. See the instructions [here](${tagged_releases_url}#verify-assertions-by-the-community) on how to do so.

We [welcome your signature](${repository_url}/tree/master/releases#contribute-your-signature) confirming this build.

**Verify the hash shown by the BitBox02:**

The hash of the firmware as verified/shown by the BitBox02 at startup is:

$hash_lines

See this [documentation](${tagged_releases_url}#verify-the-hash-as-shown-by-the-bitbox02-at-startup) for more details about this hash."""
)

SIMULATOR_RELEASE_TEMPLATE = Template(
    """**Release notes:**

$changelog_entries

**Simulator:**

This release contains a simulator executable. Its primary use case is integration testing."""
)


class ReleaseError(RuntimeError):
    """An error that should be reported without a traceback."""


@dataclass(frozen=True)
class ReleaseProduct:
    """Properties of one firmware release product."""

    label: str
    asset_pattern: str
    firmware_product: firmware_format.Product

    def asset_name(self, version: str) -> str:
        """Return the release asset name for a normalized version."""

        return self.asset_pattern.format(version=version)


PRODUCTS = (
    ReleaseProduct(
        label="BitBox02 Bitcoin-only",
        asset_pattern="firmware-bitbox02-btconly.{version}.signed.bin",
        firmware_product=firmware_format.BITBOX02_BTCONLY,
    ),
    ReleaseProduct(
        label="BitBox02 Multi",
        asset_pattern="firmware-bitbox02-multi.{version}.signed.bin",
        firmware_product=firmware_format.BITBOX02_MULTI,
    ),
    ReleaseProduct(
        label="BitBox02 Nova Bitcoin-only",
        asset_pattern="firmware-bitbox02nova-btconly.{version}.signed.bin",
        firmware_product=firmware_format.BITBOX02_NOVA_BTCONLY,
    ),
    ReleaseProduct(
        label="BitBox02 Nova Multi",
        asset_pattern="firmware-bitbox02nova-multi.{version}.signed.bin",
        firmware_product=firmware_format.BITBOX02_NOVA_MULTI,
    ),
)


@dataclass(frozen=True)
class FirmwareAsset:
    """A firmware asset and its bootloader-visible hash."""

    product: ReleaseProduct
    path: Path
    sighash: str


def parse_args(argv: Sequence[str] | None = None) -> argparse.Namespace:
    """Parse command-line arguments."""

    parser = argparse.ArgumentParser(description="Create a draft BitBox02 firmware release")
    parser.add_argument("version", help="Release version, with or without a leading 'v'")
    parser.add_argument("input_dir", type=Path, help="Directory containing the release assets")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="validate and preview the release without prompting or creating it",
    )
    return parser.parse_args(argv)


def normalize_version(value: str) -> str:
    """Normalize X.Y.Z or vX.Y.Z to vX.Y.Z."""

    match = VERSION_PATTERN.fullmatch(value)
    if match is None:
        raise ReleaseError(f"Invalid version '{value}'; expected X.Y.Z or vX.Y.Z")
    return f"v{match.group(1)}"


def run_gh(args: Sequence[str], input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    """Run an authenticated GitHub CLI command."""

    try:
        return subprocess.run(
            ["gh", *args],
            check=False,
            capture_output=True,
            input=input_text,
            text=True,
        )
    except FileNotFoundError as exc:
        raise ReleaseError("gh is required but was not found in PATH") from exc


def command_error(action: str, result: subprocess.CompletedProcess[str]) -> ReleaseError:
    """Create a useful error from a failed gh invocation."""

    details = result.stderr.strip() or result.stdout.strip() or "unknown error"
    return ReleaseError(f"Failed to {action}: {details}")


def is_not_found(result: subprocess.CompletedProcess[str]) -> bool:
    """Return whether a gh API failure was an HTTP 404."""

    return "HTTP 404" in result.stderr


def verify_remote_tag(tag_name: str) -> None:
    """Require tag_name to already exist in the upstream GitHub repository."""

    encoded_tag = quote(tag_name, safe="")
    result = run_gh(["api", f"repos/{REPOSITORY}/git/ref/tags/{encoded_tag}"])
    if result.returncode == 0:
        return
    if is_not_found(result):
        raise ReleaseError(f"Remote tag '{tag_name}' does not exist in {REPOSITORY}")
    raise command_error(f"verify remote tag '{tag_name}'", result)


def ensure_release_absent(tag_name: str) -> None:
    """Require that no GitHub release already exists for tag_name."""

    encoded_tag = quote(tag_name, safe="")
    result = run_gh(["api", f"repos/{REPOSITORY}/releases/tags/{encoded_tag}"])
    if result.returncode == 0:
        raise ReleaseError(f"A GitHub release already exists for tag '{tag_name}'")
    if not is_not_found(result):
        raise command_error(f"check for an existing release for '{tag_name}'", result)


def fetch_tagged_changelog(tag_name: str) -> str:
    """Fetch CHANGELOG.md exactly as it appears at the remote tag."""

    encoded_tag = quote(tag_name, safe="")
    result = run_gh(
        [
            "api",
            "-H",
            "Accept: application/vnd.github.raw+json",
            f"repos/{REPOSITORY}/contents/CHANGELOG.md?ref={encoded_tag}",
        ]
    )
    if result.returncode != 0:
        raise command_error(f"fetch CHANGELOG.md at '{tag_name}'", result)
    return result.stdout


def extract_changelog_entries(changelog: str, version: str) -> str:
    """Extract the Markdown below the exact firmware version heading."""

    heading = f"### {version}"
    lines = changelog.splitlines()
    heading_indices = [index for index, line in enumerate(lines) if line.rstrip() == heading]
    if len(heading_indices) != 1:
        raise ReleaseError(
            f"Expected exactly one '{heading}' section in the tagged CHANGELOG.md, "
            f"found {len(heading_indices)}"
        )

    entries = []
    for line in lines[heading_indices[0] + 1 :]:
        if re.match(r"^#{1,3}\s", line):
            break
        entries.append(line)

    rendered = "\n".join(entries).strip()
    if not rendered:
        raise ReleaseError(f"The '{heading}' section in the tagged CHANGELOG.md is empty")
    return rendered


def discover_assets(
    input_dir: Path, version: str
) -> tuple[Path, list[tuple[ReleaseProduct, Path]]]:
    """Find the required simulator and any recognized firmware assets."""

    try:
        resolved_input_dir = input_dir.expanduser().resolve(strict=True)
    except FileNotFoundError as exc:
        raise ReleaseError(f"Input directory does not exist: {input_dir}") from exc
    if not resolved_input_dir.is_dir():
        raise ReleaseError(f"Input path is not a directory: {resolved_input_dir}")

    simulator = resolved_input_dir / SIMULATOR_ASSET_PATTERN.format(version=version)
    if not simulator.exists():
        raise ReleaseError(f"Required simulator asset is missing: {simulator}")
    if not simulator.is_file():
        raise ReleaseError(f"Simulator asset is not a regular file: {simulator}")

    firmware_assets = []
    for product in PRODUCTS:
        path = resolved_input_dir / product.asset_name(version)
        if not path.exists():
            continue
        if not path.is_file():
            raise ReleaseError(f"Firmware asset is not a regular file: {path}")
        firmware_assets.append((product, path))

    return simulator, firmware_assets


def calculate_firmware_sighash(path: Path, product: ReleaseProduct) -> str:
    """Calculate the firmware hash verified and shown by the bootloader."""

    try:
        signed_firmware = path.read_bytes()
    except OSError as exc:
        raise ReleaseError(f"Failed to read firmware asset '{path}': {exc}") from exc

    try:
        parsed = firmware_format.parse(signed_firmware)
    except ValueError as exc:
        raise ReleaseError(f"Invalid firmware asset '{path}': {exc}") from exc

    if parsed.product != product.firmware_product:
        magic = signed_firmware[: firmware_format.MAGIC_LEN]
        raise ReleaseError(
            f"Firmware asset '{path}' has magic {magic.hex()}, expected "
            f"{product.firmware_product.magic.hex()} for {product.label}"
        )
    return firmware_format.sighash(parsed).hex()


def render_release_notes(
    version: str, changelog_entries: str, firmware_assets: Sequence[FirmwareAsset]
) -> str:
    """Render release notes in the established firmware release format."""

    template_values = {"changelog_entries": changelog_entries.strip()}
    if not firmware_assets:
        return SIMULATOR_RELEASE_TEMPLATE.substitute(template_values).strip() + "\n"

    hash_lines = "\n".join(
        f"- {asset.product.label}: `{asset.sighash}`" for asset in firmware_assets
    )
    template_values.update(
        {
            "repository_url": REPOSITORY_URL,
            "version": version,
            "tagged_releases_url": f"{REPOSITORY_URL}/tree/firmware/{version}/releases",
            "hash_lines": hash_lines,
        }
    )
    return FIRMWARE_RELEASE_TEMPLATE.substitute(template_values).strip() + "\n"


def print_preview(
    version: str,
    tag_name: str,
    release_notes: str,
    simulator: Path,
    firmware_assets: Sequence[FirmwareAsset],
) -> None:
    """Display the complete pending GitHub mutation."""

    print("Action: create a draft GitHub release")
    print(f"Repository: {REPOSITORY}")
    print(f"Tag: {tag_name} (must already exist)")
    print(f"Title: {version}")
    print("Assets:")
    print(f"  - {simulator}")
    for asset in firmware_assets:
        print(f"  - {asset.path}")

    if firmware_assets:
        print("Firmware hashes shown by the device:")
        for asset in firmware_assets:
            print(f"  - {asset.product.label}: {asset.sighash}")
        print("Firmware signatures are not verified by this script.")

    print("\nRelease notes:\n")
    print(release_notes, end="")


def confirm_release() -> bool:
    """Ask for default-negative confirmation."""

    try:
        response = input("\nCreate this draft release? [y/N] ")
    except EOFError:
        return False
    return response.strip().lower() in {"y", "yes"}


def create_draft_release(
    version: str,
    tag_name: str,
    release_notes: str,
    asset_paths: Sequence[Path],
) -> str:
    """Create the draft release and upload all selected assets."""

    result = run_gh(
        [
            "release",
            "create",
            tag_name,
            *(str(path) for path in asset_paths),
            "--repo",
            REPOSITORY,
            "--draft",
            "--verify-tag",
            "--title",
            version,
            "--notes-file",
            "-",
        ],
        input_text=release_notes,
    )
    if result.returncode != 0:
        error = command_error("create the draft release", result)
        raise ReleaseError(
            f"{error}\nGitHub may have retained a partially created draft; inspect the "
            "repository before retrying."
        )
    return result.stdout.strip()


def main(argv: Sequence[str] | None = None) -> int:
    """Run the release creation workflow."""

    args = parse_args(argv)
    try:
        version = normalize_version(args.version)
        tag_name = f"firmware/{version}"
        simulator, discovered_firmware = discover_assets(args.input_dir, version)

        verify_remote_tag(tag_name)
        ensure_release_absent(tag_name)
        changelog = fetch_tagged_changelog(tag_name)
        changelog_entries = extract_changelog_entries(changelog, version)

        firmware_assets = [
            FirmwareAsset(
                product=product,
                path=path,
                sighash=calculate_firmware_sighash(path, product),
            )
            for product, path in discovered_firmware
        ]
        release_notes = render_release_notes(version, changelog_entries, firmware_assets)

        print_preview(version, tag_name, release_notes, simulator, firmware_assets)
        if args.dry_run:
            print("\nDry run complete; no release was created.")
            return 0
        if not confirm_release():
            print("Cancelled; no release was created.")
            return 0

        asset_paths = [simulator, *(asset.path for asset in firmware_assets)]
        release_url = create_draft_release(version, tag_name, release_notes, asset_paths)
        if release_url:
            print(f"Created draft release: {release_url}")
        else:
            print("Created draft release.")
        return 0
    except ReleaseError as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("\nCancelled; no release was created.", file=sys.stderr)
        return 130


if __name__ == "__main__":
    sys.exit(main())
