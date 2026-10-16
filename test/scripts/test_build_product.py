# SPDX-License-Identifier: Apache-2.0

import os
from pathlib import Path
import subprocess
import sys
import unittest

from scripts.build_config import Config, image_info


ROOT = Path(__file__).resolve().parents[2]


class BitBox03BoardBuildTests(unittest.TestCase):
    def test_selected_board_reaches_each_cargo_build(self):
        for board, alias in (("dev-kit", "stm32u5a9j-dk"), ("testboard", "testboard")):
            for debug in ("yes", "no"):
                for image, package in (
                    ("firmware", "bitbox03-firmware"),
                    ("factorysetup", "bitbox03-factorysetup"),
                    ("bootloader-stage0", "bitbox03-boot0"),
                    ("bootloader-stage1", "bitbox03-boot1"),
                ):
                    with self.subTest(board=board, debug=debug, image=image):
                        result = subprocess.run(
                            [
                                sys.executable,
                                str(ROOT / "scripts/build_product.py"),
                                "--product",
                                "bitbox03",
                                "--board",
                                board,
                                "--debug",
                                debug,
                                image,
                            ],
                            env={**os.environ, "MAKEFLAGS": "n"},
                            capture_output=True,
                            text=True,
                            check=True,
                        )
                        suffix = "-release" if debug == "no" else ""
                        self.assertIn(f"cargo {package}-{alias}{suffix} ", result.stdout)
                        config = Config("bitbox03", board, "multi", "openocd", debug=debug)
                        info = image_info(config, image)
                        self.assertIn(str(ROOT / "build-bitbox03-cargo" / board), result.stdout)
                        self.assertIn(str(info.elf), result.stdout)

    def test_boards_have_distinct_artifacts_and_devices(self):
        devkit = Config("bitbox03", "dev-kit", "multi", "openocd")
        testboard = Config("bitbox03", "testboard", "multi", "openocd")
        self.assertEqual(devkit.chip_type, "STM32U5A9NJ")
        self.assertEqual(testboard.chip_type, "STM32U5G9ZJ")
        self.assertNotEqual(
            image_info(devkit, "firmware").elf, image_info(testboard, "firmware").elf
        )


if __name__ == "__main__":
    unittest.main()
