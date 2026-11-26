# SPDX-License-Identifier: Apache-2.0

import itertools
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
PRODUCTS = ("bitbox02", "bitbox02nova", "bitbox03")
EDITIONS = ("multi", "btc-only")
DEBUG = ("no", "yes")


class TestSimulatorConfig(unittest.TestCase):
    def make_recipe(self, directory, *args, target="simulator-graphical", env=None):
        return subprocess.run(
            [
                "make",
                "--no-print-directory",
                "--dry-run",
                "-f",
                str(ROOT / "Makefile"),
                "MAKE=echo",
                target,
                *args,
            ],
            cwd=directory,
            env=env,
            text=True,
            capture_output=True,
            check=True,
        ).stdout

    def test_make_config_dispatch(self):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "config.mk"
            for product, edition, debug in itertools.product(PRODUCTS, EDITIONS, DEBUG):
                with self.subTest(product=product, edition=edition, debug=debug):
                    config.write_text(
                        f"PRODUCT = {product}\nEDITION = {edition}\nDEBUG = {debug}\n"
                        "BOARD = ignored\nPROBE_SOFTWARE = ignored\nLOCKED = ignored\n"
                    )
                    recipe = self.make_recipe(directory)
                    command = next(
                        line
                        for line in recipe.splitlines()
                        if line.lstrip().startswith("cargo build ")
                    )
                    argv = shlex.split(command)
                    self.assertEqual(
                        argv[argv.index("--features") + 1], f"product-{product},edition-{edition}"
                    )
                    self.assertIn("--no-default-features", argv)
                    self.assertEqual(argv.count("--locked"), 1)
                    self.assertEqual(
                        argv[argv.index("--profile") + 1], "dev" if debug == "yes" else "release"
                    )
                    self.assertNotIn("--target-dir", argv)
                    self.assertNotIn("--target", argv)
                    self.assertNotIn("ignored", command)
                    self.assertNotIn("test -f", recipe)
                    self.assertNotIn("cmake", recipe)
            recipe = self.make_recipe(directory, "PRODUCT=bitbox02", "EDITION=multi", "DEBUG=no")
            self.assertIn("--features product-bitbox02,edition-multi", recipe)
            self.assertIn("--profile release", recipe)
            config.unlink()
            env = dict(os.environ)
            for key in ("PRODUCT", "EDITION", "DEBUG"):
                env.pop(key, None)
            recipe = self.make_recipe(directory, env=env)
            self.assertIn("--features product-bitbox02,edition-multi", recipe)
            self.assertIn("--profile release", recipe)
            for debug in DEBUG:
                recipe = self.make_recipe(
                    directory,
                    f"DEBUG={debug}",
                    "SIMULATOR_ARGS=--preseed --unlock --port 15424",
                    target="run-simulator-graphical",
                )
                self.assertIn("echo simulator-graphical", recipe)
                profile = "debug" if debug == "yes" else "release"
                self.assertIn(
                    f"./test/simulator-graphical/target/{profile}/simulator-graphical "
                    "--preseed --unlock --port 15424",
                    recipe,
                )
            recipe = self.make_recipe(directory, target="run-simulator")
            self.assertIn("echo simulator", recipe.splitlines())
            self.assertIn("./build-build-noasan/bin/simulator", recipe.splitlines())
            self.assertNotIn("simulator-graphical", recipe)

    def test_make_cargo_flags(self):
        with tempfile.TemporaryDirectory() as directory:
            for cargo_flags in (
                "",
                "--offline",
                "--locked --offline",
                "--locked --locked --config 'net.retry = 2'",
            ):
                with self.subTest(cargo_flags=cargo_flags):
                    recipe = self.make_recipe(
                        directory, env=dict(os.environ, CARGOFLAGS=cargo_flags)
                    )
                    command = next(
                        line
                        for line in recipe.splitlines()
                        if line.lstrip().startswith("cargo build ")
                    )
                    argv = shlex.split(command)
                    self.assertEqual(argv.count("--locked"), 1)
                    self.assertEqual("--offline" in argv, "--offline" in cargo_flags)
                    if "--config" in cargo_flags:
                        self.assertEqual(argv[argv.index("--config") + 1], "net.retry = 2")


if __name__ == "__main__":
    unittest.main()
