# SPDX-License-Identifier: Apache-2.0

import argparse
import contextlib
from dataclasses import replace
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from scripts import build_config


class TestBuildConfig(unittest.TestCase):
    def test_bootloader_modes(self) -> None:
        modes = (
            ("no", "yes", "development"),
            ("yes", "no", "production"),
            ("yes", "yes", "development-locked"),
        )
        for product in ("bitbox02", "bitbox02nova"):
            for edition in build_config.EDITIONS:
                for debug in ("no", "yes"):
                    for locked, development, variant in modes:
                        with self.subTest(
                            product=product,
                            edition=edition,
                            debug=debug,
                            locked=locked,
                            development=development,
                        ):
                            config = replace(
                                build_config.defaults(product),
                                edition=edition,
                                debug=debug,
                                locked=locked,
                                development=development,
                            )
                            for stage, vectors in ((0, 0), (1, 0x2400)):
                                info = build_config.image_info(config, f"bootloader-stage{stage}")
                                expected = (
                                    f"bootloader-stage{stage}-{product}-"
                                    f"{edition.replace('-', '')}-{variant}"
                                )
                                self.assertEqual(info.name, expected)
                                self.assertEqual(info.elf.name, expected + ".elf")
                                self.assertEqual(info.vectors, vectors)
                                profile = "debug" if debug == "yes" else "relwithdebinfo"
                                self.assertEqual(
                                    info.elf.parent.parent.name,
                                    f"build-{product}-cmake-{profile}",
                                )

    def test_config_flags_roundtrip(self) -> None:
        parser = argparse.ArgumentParser()
        build_config.add_config_arguments(parser)
        defaults = build_config.from_args(parser.parse_args([]))
        self.assertEqual((defaults.locked, defaults.development), ("no", "yes"))
        for locked, development in (("no", "yes"), ("yes", "no"), ("yes", "yes")):
            with self.subTest(locked=locked, development=development):
                config = build_config.from_args(
                    parser.parse_args(["--locked", locked, "--development", development])
                )
                with tempfile.TemporaryDirectory() as directory:
                    path = Path(directory) / "config.mk"
                    build_config.save_config(path, config)
                    saved = build_config.read_config(path)
                self.assertEqual(saved["LOCKED"], locked)
                self.assertEqual(saved["DEVELOPMENT"], development)
                with patch("builtins.input", return_value=""), contextlib.redirect_stdout(
                    io.StringIO()
                ):
                    self.assertEqual(build_config.interactive(saved), config)

    def test_unlocked_non_development_rejected(self) -> None:
        parser = argparse.ArgumentParser()
        build_config.add_config_arguments(parser)
        for product in ("bitbox02", "bitbox02nova"):
            with self.subTest(product=product):
                args = parser.parse_args(
                    ["--product", product, "--locked", "no", "--development", "no"]
                )
                with self.assertRaisesRegex(ValueError, "DEVELOPMENT=no requires LOCKED=yes"):
                    build_config.from_args(args)
                config = replace(build_config.defaults(product), development="no")
                for image in build_config.IMAGES:
                    with self.assertRaisesRegex(ValueError, "DEVELOPMENT=no requires LOCKED=yes"):
                        build_config.image_info(config, image)

    def test_invalid_flags_preserve_saved_config(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "config.mk"
            build_config.save_config(path, build_config.Config())
            previous = path.read_bytes()
            for config in (
                replace(build_config.Config(), locked="invalid"),
                replace(build_config.Config(), development="invalid"),
                replace(build_config.Config(), development="no"),
            ):
                with self.subTest(config=config), self.assertRaises(ValueError):
                    build_config.save_config(path, config)
                self.assertEqual(path.read_bytes(), previous)

    def test_interactive_unlock_requires_development(self) -> None:
        saved = {"LOCKED": "yes", "DEVELOPMENT": "no"}
        with patch(
            "builtins.input", side_effect=["", "", "", "no", "", ""]
        ), contextlib.redirect_stdout(io.StringIO()):
            config = build_config.interactive(saved)
        self.assertEqual((config.locked, config.development), ("no", "yes"))

    def test_bitbox03_rejects_unimplemented_modes(self) -> None:
        base = build_config.defaults("bitbox03")
        base.validate()
        for config in (replace(base, locked="yes"), replace(base, development="no")):
            with self.assertRaisesRegex(ValueError, "not implemented for BitBox03"):
                config.validate()

    def test_interactive_product_change_resets_unsupported_modes(self) -> None:
        saved = {"LOCKED": "yes", "DEVELOPMENT": "no"}
        with patch(
            "builtins.input", side_effect=["bitbox03", "", "", "", ""]
        ), contextlib.redirect_stdout(io.StringIO()):
            config = build_config.interactive(saved)
        self.assertEqual(config, build_config.defaults("bitbox03"))


if __name__ == "__main__":
    unittest.main()
