# SPDX-License-Identifier: Apache-2.0

"""Tests for scripts/create_release.py."""

from __future__ import annotations

import contextlib
import hashlib
import importlib.util
import io
import struct
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
SCRIPTS_DIR = REPOSITORY_ROOT / "scripts"
sys.path.insert(0, str(SCRIPTS_DIR))
MODULE_PATH = SCRIPTS_DIR / "create_release.py"
SPEC = importlib.util.spec_from_file_location("create_release_under_test", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"Could not load {MODULE_PATH}")
CREATE_RELEASE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = CREATE_RELEASE
SPEC.loader.exec_module(CREATE_RELEASE)
FIRMWARE_FORMAT = CREATE_RELEASE.firmware_format

ASSERTION_MODULE_PATH = SCRIPTS_DIR / "create_release_assertions.py"
ASSERTION_SPEC = importlib.util.spec_from_file_location(
    "create_release_assertions_under_test", ASSERTION_MODULE_PATH
)
if ASSERTION_SPEC is None or ASSERTION_SPEC.loader is None:
    raise RuntimeError(f"Could not load {ASSERTION_MODULE_PATH}")
CREATE_ASSERTIONS = importlib.util.module_from_spec(ASSERTION_SPEC)
sys.modules[ASSERTION_SPEC.name] = CREATE_ASSERTIONS
ASSERTION_SPEC.loader.exec_module(CREATE_ASSERTIONS)


def completed_process(
    returncode: int = 0, stdout: str = "", stderr: str = ""
) -> subprocess.CompletedProcess[str]:
    """Create a gh-like subprocess result."""

    return subprocess.CompletedProcess(
        args=["gh"], returncode=returncode, stdout=stdout, stderr=stderr
    )


def write_signed_firmware(
    path: Path,
    product: CREATE_RELEASE.ReleaseProduct,
    monotonic_version: int,
    firmware: bytes = b"firmware",
) -> bytes:
    """Write a minimally populated signed-firmware container."""

    sigdata = bytearray(FIRMWARE_FORMAT.SIGDATA_LEN)
    struct.pack_into("<I", sigdata, FIRMWARE_FORMAT.SIGNING_PUBKEYS_DATA_LEN, monotonic_version)
    path.write_bytes(product.firmware_product.magic + sigdata + firmware)
    return bytes(sigdata)


class VersionAndChangelogTests(unittest.TestCase):
    """Test release input normalization."""

    def test_normalize_version(self) -> None:
        self.assertEqual(CREATE_RELEASE.normalize_version("9.26.6"), "v9.26.6")
        self.assertEqual(CREATE_RELEASE.normalize_version("v9.26.6"), "v9.26.6")

    def test_normalize_version_rejects_invalid_values(self) -> None:
        for value in ("", "V9.26.6", "v9.26", "v9.26.6-rc1"):
            with self.subTest(value=value), self.assertRaises(CREATE_RELEASE.ReleaseError):
                CREATE_RELEASE.normalize_version(value)

    def test_extract_changelog_entries(self) -> None:
        changelog = """# Changelog

## Firmware

### [Unreleased]
- Later change

### v9.26.6
- First release change
- Second release change

### v9.26.5
- Previous change
"""
        self.assertEqual(
            CREATE_RELEASE.extract_changelog_entries(changelog, "v9.26.6"),
            "- First release change\n- Second release change",
        )

    def test_extract_changelog_entries_requires_one_nonempty_section(self) -> None:
        for changelog in ("### v9.26.5\n- Old", "### v9.26.6\n\n### v9.26.5"):
            with self.subTest(changelog=changelog), self.assertRaises(CREATE_RELEASE.ReleaseError):
                CREATE_RELEASE.extract_changelog_entries(changelog, "v9.26.6")


class AssetTests(unittest.TestCase):
    """Test asset discovery and signed-firmware parsing."""

    def test_shared_product_values(self) -> None:
        self.assertEqual(
            [(product.product_id, product.magic.hex()) for product in FIRMWARE_FORMAT.PRODUCTS],
            [
                (1, "653f362b"),
                (2, "11233b0b"),
                (3, "5b648ceb"),
                (4, "48714774"),
            ],
        )

    def test_discover_assets_allows_any_firmware_subset(self) -> None:
        version = "v9.26.6"
        with tempfile.TemporaryDirectory() as temp_dir:
            directory = Path(temp_dir)
            simulator = directory / CREATE_RELEASE.SIMULATOR_ASSET_PATTERN.format(version=version)
            simulator.touch()
            selected_products = (CREATE_RELEASE.PRODUCTS[0], CREATE_RELEASE.PRODUCTS[3])
            for product in selected_products:
                (directory / product.asset_name(version)).touch()
            (directory / "unrelated-file").touch()

            actual_simulator, firmware_assets = CREATE_RELEASE.discover_assets(directory, version)

            self.assertEqual(actual_simulator, simulator)
            self.assertEqual(
                firmware_assets,
                [
                    (product, directory / product.asset_name(version))
                    for product in selected_products
                ],
            )

    def test_discover_assets_allows_simulator_only(self) -> None:
        version = "v9.26.6"
        with tempfile.TemporaryDirectory() as temp_dir:
            directory = Path(temp_dir)
            simulator = directory / CREATE_RELEASE.SIMULATOR_ASSET_PATTERN.format(version=version)
            simulator.touch()

            actual_simulator, firmware_assets = CREATE_RELEASE.discover_assets(directory, version)

            self.assertEqual(actual_simulator, simulator)
            self.assertEqual(firmware_assets, [])

    def test_discover_assets_requires_simulator(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir, self.assertRaises(
            CREATE_RELEASE.ReleaseError
        ):
            CREATE_RELEASE.discover_assets(Path(temp_dir), "v9.26.6")

    def test_calculate_sighash_for_every_product(self) -> None:
        firmware = b"test firmware payload"
        monotonic_version = 53
        with tempfile.TemporaryDirectory() as temp_dir:
            for product in CREATE_RELEASE.PRODUCTS:
                with self.subTest(product=product.label):
                    path = Path(temp_dir) / product.asset_name("v9.26.6")
                    sigdata = write_signed_firmware(path, product, monotonic_version, firmware)
                    version_bytes = sigdata[
                        FIRMWARE_FORMAT.SIGNING_PUBKEYS_DATA_LEN : FIRMWARE_FORMAT.SIGNING_PUBKEYS_DATA_LEN
                        + FIRMWARE_FORMAT.VERSION_LEN
                    ]
                    padded = firmware + b"\xff" * (
                        FIRMWARE_FORMAT.MAX_FIRMWARE_SIZE - len(firmware)
                    )
                    expected = hashlib.sha256(
                        struct.pack("<H", product.firmware_product.product_id)
                        + version_bytes
                        + padded
                    ).hexdigest()

                    self.assertEqual(
                        CREATE_RELEASE.calculate_firmware_sighash(path, product), expected
                    )

    def test_calculate_sighash_rejects_malformed_firmware(self) -> None:
        product = CREATE_RELEASE.PRODUCTS[0]
        with tempfile.TemporaryDirectory() as temp_dir:
            directory = Path(temp_dir)
            path = directory / product.asset_name("v9.26.6")

            path.write_bytes(b"too short")
            with self.assertRaisesRegex(CREATE_RELEASE.ReleaseError, "too small"):
                CREATE_RELEASE.calculate_firmware_sighash(path, product)

            write_signed_firmware(path, CREATE_RELEASE.PRODUCTS[1], 50)
            with self.assertRaisesRegex(CREATE_RELEASE.ReleaseError, "has magic"):
                CREATE_RELEASE.calculate_firmware_sighash(path, product)

            write_signed_firmware(path, product, 50, b"x" * (FIRMWARE_FORMAT.MAX_FIRMWARE_SIZE + 1))
            with self.assertRaisesRegex(CREATE_RELEASE.ReleaseError, "exceeding the maximum"):
                CREATE_RELEASE.calculate_firmware_sighash(path, product)

    def test_assertion_hash_uses_shared_parser(self) -> None:
        product = CREATE_RELEASE.PRODUCTS[1]
        firmware = b"unsigned firmware"
        with tempfile.TemporaryDirectory() as temp_dir:
            path = Path(temp_dir) / product.asset_name("v9.26.6")
            write_signed_firmware(path, product, 53, firmware)
            signed_firmware = path.read_bytes()

        self.assertEqual(
            CREATE_ASSERTIONS.extract_unsigned_firmware_hash(
                signed_firmware, product.firmware_product.magic, path.name
            ),
            hashlib.sha256(firmware).hexdigest(),
        )
        with self.assertRaisesRegex(RuntimeError, "unexpected magic"):
            CREATE_ASSERTIONS.extract_unsigned_firmware_hash(
                signed_firmware, FIRMWARE_FORMAT.BITBOX02_BTCONLY.magic, path.name
            )


class ReleaseNotesTests(unittest.TestCase):
    """Test the release-note template."""

    def test_render_release_notes_with_arbitrary_firmware_subset(self) -> None:
        selected_products = (CREATE_RELEASE.PRODUCTS[0], CREATE_RELEASE.PRODUCTS[3])
        firmware_assets = [
            CREATE_RELEASE.FirmwareAsset(product, Path(product.asset_name("v9.26.6")), "ab" * 32)
            for product in selected_products
        ]

        notes = CREATE_RELEASE.render_release_notes(
            "v9.26.6", "- First change\n- Second change", firmware_assets
        )

        self.assertTrue(notes.startswith("**Release notes:**\n\n- First change\n- Second change"))
        self.assertIn("**Note:**", notes)
        self.assertIn("**Simulator:**", notes)
        self.assertIn("**Verify this release:**", notes)
        self.assertIn("BitBox02 Bitcoin-only: `" + "ab" * 32 + "`", notes)
        self.assertIn("BitBox02 Nova Multi: `" + "ab" * 32 + "`", notes)
        self.assertNotIn("- BitBox02 Multi:", notes)
        self.assertNotIn("- BitBox02 Nova Bitcoin-only:", notes)
        self.assertIn("/tree/master/releases/firmware-v9.26.6", notes)
        self.assertIn("/tree/firmware/v9.26.6/releases#verify-assertions-by-the-community", notes)
        self.assertIn(
            "/tree/firmware/v9.26.6/releases"
            "#verify-the-hash-as-shown-by-the-bitbox02-at-startup",
            notes,
        )

    def test_render_simulator_only_release_notes(self) -> None:
        notes = CREATE_RELEASE.render_release_notes("v9.26.6", "- Simulator fix", [])

        self.assertEqual(
            notes,
            "**Release notes:**\n\n- Simulator fix\n\n"
            "**Simulator:**\n\n"
            "This release contains a simulator executable. Its primary use case is integration "
            "testing.\n",
        )


class GithubWorkflowTests(unittest.TestCase):
    """Test GitHub preflight and mutation boundaries."""

    @mock.patch.object(CREATE_RELEASE, "run_gh")
    def test_verify_remote_tag_reports_missing_tag(self, run_gh: mock.Mock) -> None:
        run_gh.return_value = completed_process(returncode=1, stderr="gh: Not Found (HTTP 404)")

        with self.assertRaisesRegex(CREATE_RELEASE.ReleaseError, "does not exist"):
            CREATE_RELEASE.verify_remote_tag("firmware/v9.26.6")

        self.assertIn("firmware%2Fv9.26.6", run_gh.call_args.args[0][1])

    @mock.patch.object(CREATE_RELEASE, "run_gh")
    def test_create_draft_release_uses_existing_tag_and_draft(self, run_gh: mock.Mock) -> None:
        run_gh.return_value = completed_process(stdout="https://example.test/release\n")
        assets = [Path("simulator"), Path("firmware.bin")]

        url = CREATE_RELEASE.create_draft_release(
            "v9.26.6", "firmware/v9.26.6", "release notes\n", assets
        )

        self.assertEqual(url, "https://example.test/release")
        args = run_gh.call_args.args[0]
        self.assertEqual(args[:3], ["release", "create", "firmware/v9.26.6"])
        self.assertIn("--draft", args)
        self.assertIn("--verify-tag", args)
        self.assertEqual(run_gh.call_args.kwargs["input_text"], "release notes\n")

    @mock.patch.object(CREATE_RELEASE, "create_draft_release")
    @mock.patch.object(CREATE_RELEASE, "confirm_release")
    @mock.patch.object(CREATE_RELEASE, "fetch_tagged_changelog")
    @mock.patch.object(CREATE_RELEASE, "ensure_release_absent")
    @mock.patch.object(CREATE_RELEASE, "verify_remote_tag")
    def test_dry_run_validates_and_previews_without_prompt_or_mutation(
        self,
        verify_remote_tag: mock.Mock,
        ensure_release_absent: mock.Mock,
        fetch_tagged_changelog: mock.Mock,
        confirm_release: mock.Mock,
        create_draft_release: mock.Mock,
    ) -> None:
        fetch_tagged_changelog.return_value = "### v9.26.6\n- Release change\n"
        with tempfile.TemporaryDirectory() as temp_dir:
            directory = Path(temp_dir)
            simulator = directory / CREATE_RELEASE.SIMULATOR_ASSET_PATTERN.format(version="v9.26.6")
            simulator.touch()
            product = CREATE_RELEASE.PRODUCTS[2]
            firmware = directory / product.asset_name("v9.26.6")
            write_signed_firmware(firmware, product, 50)
            output = io.StringIO()

            with contextlib.redirect_stdout(output):
                result = CREATE_RELEASE.main(["9.26.6", str(directory), "--dry-run"])

        self.assertEqual(result, 0)
        verify_remote_tag.assert_called_once_with("firmware/v9.26.6")
        ensure_release_absent.assert_called_once_with("firmware/v9.26.6")
        fetch_tagged_changelog.assert_called_once_with("firmware/v9.26.6")
        confirm_release.assert_not_called()
        create_draft_release.assert_not_called()
        self.assertIn("BitBox02 Nova Bitcoin-only", output.getvalue())
        self.assertIn("Dry run complete; no release was created.", output.getvalue())

    @mock.patch("builtins.input", side_effect=EOFError)
    def test_confirmation_defaults_to_no_on_eof(self, _input: mock.Mock) -> None:
        self.assertFalse(CREATE_RELEASE.confirm_release())


if __name__ == "__main__":
    unittest.main()
