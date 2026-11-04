# SPDX-License-Identifier: Apache-2.0

from dataclasses import replace
from pathlib import Path
import sys
import unittest
from unittest.mock import call, patch

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "scripts"))

import build_config
import build_product


class TestBuildProduct(unittest.TestCase):
    def test_cmake_configuration(self) -> None:
        for product in ("bitbox02", "bitbox02nova"):
            for edition in ("multi", "btc-only"):
                config = replace(build_config.defaults(product), edition=edition)
                with self.subTest(config=config), patch.object(build_product, "run") as run:
                    build_product.cmake(config, ["firmware.elf"])
                    command = next(
                        item.args[0] for item in run.call_args_list if item.args[0][0] == "cmake"
                    )
                    self.assertIn(f"-DPRODUCT={product}", command)
                    self.assertIn(f"-DEDITION={edition}", command)
                    self.assertEqual(
                        command[command.index("-B") + 1], str(build_config.build_dir(config))
                    )
                    self.assertEqual(
                        run.call_args,
                        call(["make", "-C", str(build_config.build_dir(config)), "firmware.elf"]),
                    )

    def test_build_image_firmware(self) -> None:
        for product in ("bitbox02", "bitbox02nova"):
            for edition, name in (("multi", "firmware"), ("btc-only", "firmware-btc")):
                for debug in ("no", "yes"):
                    config = replace(build_config.defaults(product), edition=edition, debug=debug)
                    with self.subTest(config=config), patch.object(
                        build_product, "cmake"
                    ) as cmake, patch.object(build_product, "run") as run:
                        build_product.build_image(config, "firmware")
                        cmake.assert_called_once_with(config, ["firmware.elf"])
                        destination = "build-debug" if debug == "yes" else "build"
                        edition_name = "btconly" if edition == "btc-only" else "multi"
                        self.assertEqual(
                            run.call_args,
                            call(
                                [
                                    "cp",
                                    str(build_config.build_dir(config) / "bin" / f"{name}.bin"),
                                    str(
                                        build_config.ROOT
                                        / destination
                                        / "bin"
                                        / f"firmware-{product}-{edition_name}.bin"
                                    ),
                                ]
                            ),
                        )

    def test_dispatch_named_upgrade(self) -> None:
        for product in ("bitbox02", "bitbox02nova"):
            for edition, name in (("multi", "multi"), ("btc-only", "btconly")):
                for suffix in ("", "-development"):
                    target = f"firmware-blupgrade-{product}-{name}{suffix}"
                    with self.subTest(target=target), patch.object(build_product, "cmake") as cmake:
                        build_product.dispatch(build_config.defaults(), target)
                        cmake.assert_called_once_with(
                            replace(build_config.defaults(product), edition=edition),
                            [f"firmware-blupgrade{suffix}.elf"],
                        )

    def test_dispatch_upgrade_assets(self) -> None:
        for suffix in ("", "-development"):
            target = f"bootloader-upgrade-assets{suffix}"
            with self.subTest(target=target), patch.object(build_product, "cmake") as cmake:
                build_product.dispatch(build_config.defaults(), target)
                calls = cmake.call_args_list
                self.assertEqual(len(calls), 4)
                self.assertEqual(
                    {(item.args[0].product, item.args[0].edition) for item in calls},
                    {
                        ("bitbox02", "multi"),
                        ("bitbox02", "btc-only"),
                        ("bitbox02nova", "multi"),
                        ("bitbox02nova", "btc-only"),
                    },
                )
                self.assertEqual(len({build_config.build_dir(item.args[0]) for item in calls}), 4)
                for item in calls:
                    self.assertEqual(item.args[1], [target])


if __name__ == "__main__":
    unittest.main()
