# SPDX-License-Identifier: Apache-2.0
"""Configuration persistence and build/probe routing without attached hardware."""

import argparse
from contextlib import redirect_stdout
from dataclasses import replace
import io
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

import build_config as config
import build_product as build
import probe


class ConfigurationTests(unittest.TestCase):
    def test_defaults_and_probe_mappings(self):
        for product in config.PRODUCTS:
            selected = config.defaults(product).validate()
            self.assertEqual(selected.edition, "multi")
            self.assertEqual(selected.swd_speed, 4000)
            self.assertEqual(selected.board, "dev-kit" if product == "bitbox03" else product)
            self.assertEqual(
                selected.probe_hardware, "stlink" if product == "bitbox03" else "jlink"
            )
        selected = replace(config.defaults("bitbox03"), board="testboard").validate()
        self.assertEqual(selected.probe_hardware, "jlink")
        self.assertEqual(selected.probe_software, "openocd")
        self.assertEqual(config.BOARDS["bitbox03"]["bitbox03"], ("jlink", False))

    def test_scripted_omissions_use_builtin_defaults(self):
        parser = argparse.ArgumentParser()
        config.add_config_arguments(parser)
        selected = config.from_args(parser.parse_args(["--product", "bitbox03"]))
        self.assertEqual(selected, config.defaults("bitbox03"))
        selected = config.from_args(parser.parse_args(["--edition", "btc-only"]))
        self.assertEqual(selected, replace(config.defaults(), edition="btc-only"))
        selected = config.from_args(parser.parse_args(["--swd-speed", "1000"]))
        self.assertEqual(selected, replace(config.defaults(), swd_speed=1000))

    def test_save_reload_and_invalid_input_preserves_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "config.mk"
            selected = replace(
                config.defaults("bitbox02nova"),
                edition="btc-only",
                probe_software="openocd",
                swd_speed=1000,
            )
            config.save_config(path, selected)
            self.assertEqual(
                config.read_config(path),
                {
                    "PRODUCT": "bitbox02nova",
                    "BOARD": "bitbox02nova",
                    "EDITION": "btc-only",
                    "PROBE_SOFTWARE": "openocd",
                    "SWD_SPEED": "1000",
                },
            )
            previous = path.read_bytes()
            for invalid in (
                replace(selected, product="other"),
                replace(selected, board="dev-kit"),
                replace(selected, edition="other"),
                replace(selected, probe_software="other"),
                replace(selected, swd_speed=0),
                replace(selected, swd_speed=-1),
                replace(config.defaults("bitbox03"), board="bitbox03"),
                replace(config.defaults("bitbox03"), probe_software="jlink"),
            ):
                with self.assertRaises(ValueError):
                    config.save_config(path, invalid)
                self.assertEqual(path.read_bytes(), previous)
            with patch("build_config.os.replace", side_effect=OSError("save failed")):
                with self.assertRaises(OSError):
                    config.save_config(path, config.defaults())
            self.assertEqual(path.read_bytes(), previous)
            self.assertEqual(list(path.parent.iterdir()), [path])

    def test_interactive_persistence_switching_and_single_choices(self):
        saved = {
            "PRODUCT": "bitbox02nova",
            "BOARD": "bitbox02nova",
            "EDITION": "btc-only",
            "PROBE_SOFTWARE": "jlink",
            "SWD_SPEED": "1000",
        }
        with patch("builtins.input", side_effect=["", "", "", ""]), redirect_stdout(io.StringIO()):
            self.assertEqual(
                config.interactive(saved),
                replace(config.defaults("bitbox02nova"), edition="btc-only", swd_speed=1000),
            )
        with patch("builtins.input", side_effect=["bitbox03", "", "", ""]), redirect_stdout(
            io.StringIO()
        ) as output:
            result = config.interactive(saved)
        self.assertEqual(
            result, replace(config.defaults("bitbox03"), edition="btc-only", swd_speed=1000)
        )
        self.assertNotIn("testboard: disabled", output.getvalue())
        self.assertIn("bitbox03: disabled", output.getvalue())

    def test_cancellation_and_bad_selection_preserve_config(self):
        with tempfile.TemporaryDirectory() as tmp, patch.object(config, "ROOT", Path(tmp)):
            path = Path(tmp) / "config.mk"
            config.save_config(path, config.defaults())
            previous = path.read_bytes()
            for failure in (EOFError, KeyboardInterrupt, ValueError("invalid choice")):
                with patch.object(sys, "argv", ["build_config.py"]), patch(
                    "builtins.input", side_effect=failure
                ):
                    self.assertEqual(config.main(), 1)
                self.assertEqual(path.read_bytes(), previous)


class RoutingTests(unittest.TestCase):
    def test_all_images_and_editions_have_flat_product_directories(self):
        for product in config.PRODUCTS:
            for edition in config.EDITIONS:
                selected = replace(config.defaults(product), edition=edition)
                for image in config.IMAGES:
                    info = config.image_info(selected, image)
                    self.assertEqual(
                        info.elf.parts[len(ROOT.parts)],
                        (
                            "build-bitbox03-cargo"
                            if product == "bitbox03"
                            else f"build-{product}-cmake-relwithdebinfo"
                        ),
                    )
                    if product != "bitbox03" and image.startswith("bootloader"):
                        self.assertIn(
                            f"{product}-{edition.replace('-', '')}-development", info.name
                        )
                    if image == "factorysetup":
                        self.assertEqual(
                            info, config.image_info(replace(selected, edition="multi"), image)
                        )

    def test_vector_addresses(self):
        for product in ("bitbox02", "bitbox02nova"):
            self.assertEqual(
                [
                    config.image_info(config.defaults(product), name).vectors
                    for name in config.IMAGES
                ],
                [0x10000, 0x10000, 0, 0x2400],
            )
        self.assertEqual(
            [
                config.image_info(config.defaults("bitbox03"), name).vectors
                for name in config.IMAGES
            ],
            [0x08052400, 0x20040000, 0x08002000, 0x08010400],
        )

    def test_cmake_backend_edition_and_named_product(self):
        with patch.object(build, "cmake") as backend:
            build.dispatch(replace(config.defaults("bitbox02nova"), edition="btc-only"), "firmware")
            self.assertEqual(backend.call_args.args[1:], ("relwithdebinfo", ["firmware-btc.elf"]))
            build.dispatch(config.defaults("bitbox03"), "firmware-blupgrade-bitbox02nova-multi")
            self.assertEqual(backend.call_args.args[0].product, "bitbox02nova")
            build.dispatch(config.defaults(), "firmware-debug")
            self.assertEqual(backend.call_args.args[1], "debug")

    def test_cmake_cache_reuse_and_probe_independence(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            with patch.object(build, "build_dir", return_value=directory), patch.object(
                build, "run"
            ) as run:
                build.cmake(config.defaults(), "relwithdebinfo", ["firmware.elf"])
                self.assertTrue(any(call.args[0][0] == "cmake" for call in run.call_args_list))
                (directory / "Makefile").touch()
                run.reset_mock()
                build.cmake(
                    replace(config.defaults(), probe_software="openocd"),
                    "relwithdebinfo",
                    ["firmware.elf"],
                )
                self.assertEqual(len(run.call_args_list), 2)
                self.assertFalse(any(call.args[0][0] == "cmake" for call in run.call_args_list))
        self.assertEqual(
            config.build_dir(config.defaults()),
            config.build_dir(replace(config.defaults(), probe_software="openocd")),
        )
        self.assertEqual(
            config.build_dir(config.defaults()),
            config.build_dir(replace(config.defaults(), swd_speed=1000)),
        )

    def test_cargo_alias_target_dir_finalization_and_size(self):
        for image in config.IMAGES:
            for profile in ("debug", "release"):
                with patch.object(build, "run") as run, redirect_stdout(io.StringIO()):
                    build.build_image(config.defaults("bitbox03"), image, profile)
                commands = [call.args[0] for call in run.call_args_list]
                cargo = commands[1]
                self.assertEqual(cargo[0], "cargo")
                self.assertEqual(cargo[-2:], ["--target-dir", str(ROOT / "build-bitbox03-cargo")])
                self.assertEqual(run.call_args_list[1].args[1], ROOT / "src/rust")
                self.assertEqual(
                    any("finalize-elf" in command for command in commands),
                    image in ("firmware", "bootloader-stage1"),
                )
                self.assertEqual(commands[-1][:2], ["arm-none-eabi-size", "-Ax"])

    def test_aggregate_routes_each_product_to_its_own_tree(self):
        with patch.object(build, "build_image") as run:
            build.dispatch(config.defaults("bitbox03"), "bootloader-stage1-production")
        self.assertEqual(
            [(call.args[0].product, call.args[0].edition) for call in run.call_args_list],
            [(p, e) for p in ("bitbox02", "bitbox02nova") for e in config.EDITIONS],
        )
        self.assertTrue(all(call.kwargs["variant"] == "production" for call in run.call_args_list))


class ProbeTests(unittest.TestCase):
    def test_swd_speed_reaches_server_and_flash_before_device_access(self):
        for product in config.PRODUCTS:
            for software in config.defaults(product).software_choices:
                for speed in (4000, 1000):
                    selected = replace(
                        config.defaults(product), probe_software=software, swd_speed=speed
                    )
                    with patch.object(Path, "is_file", return_value=True), patch.object(
                        shutil, "which", return_value="tool"
                    ), patch.object(probe.subprocess, "run") as run:
                        for action in ("server", "flash"):
                            probe.execute(selected, action)
                            command = run.call_args.args[0]
                            if software == "jlink":
                                self.assertEqual(command[command.index("-speed") + 1], str(speed))
                            else:
                                speed_index = command.index(f"adapter speed {speed}")
                                # Override the board file before any initialization/programming.
                                self.assertGreater(speed_index, command.index("-f") + 1)
                                if action == "flash":
                                    self.assertLess(speed_index, len(command) - 1)
                        run.reset_mock()
                        with self.assertRaisesRegex(ValueError, "SWD speed"):
                            probe.execute(replace(selected, swd_speed=0), "flash")
                        run.assert_not_called()

    def test_run_loads_then_initializes_then_continues_attached(self):
        for product in config.PRODUCTS:
            for software in config.defaults(product).software_choices:
                selected = replace(config.defaults(product), probe_software=software)
                for name in config.IMAGES:
                    info = config.image_info(selected, name)
                    commands = probe.gdb_commands(selected, info).splitlines()
                    load = commands.index("load")
                    self.assertTrue(all("reset" not in command for command in commands[load + 1 :]))
                    self.assertGreater(
                        commands.index(f"set {{unsigned int}}0xe000ed08 = {info.vectors:#x}"), load
                    )
                    self.assertEqual(commands[-1], "c")
                    self.assertFalse(any(command in ("quit", "detach") for command in commands))
                    port = "3333" if software == "openocd" else "2331"
                    self.assertIn(f"target extended-remote :{port}", commands)
                    (
                        self.assertIn(port, probe.server_command(selected))
                        if software == "jlink"
                        else self.assertIn("gdb_port 3333", probe.server_command(selected))
                    )

    def test_flash_verifies_and_runs(self):
        for name in config.IMAGES:
            info = config.image_info(config.defaults(), name)
            commands = probe.jlink_flash_commands(info)
            self.assertIn("verifybin", commands)
            self.assertTrue(commands.endswith("r\ng\nq\n"))
            commands = probe.openocd_flash_commands(info)
            self.assertIn("verify reset exit", commands)
            self.assertIn(str(info.binary), commands)
            if name == "bootloader-stage1":
                self.assertIn("0x2000", commands)
                self.assertNotIn(".raw.bin", commands)

    def test_factorysetup_ram_only(self):
        info = config.image_info(config.defaults("bitbox03"), "factorysetup")
        commands = probe.openocd_flash_commands(info).split("; ")
        load_index = next(i for i, cmd in enumerate(commands) if cmd.startswith("load_image"))
        self.assertTrue(commands[load_index + 1].startswith("verify_image"))
        self.assertTrue(all("reset" not in cmd for cmd in commands[load_index + 1 :]))
        self.assertFalse(any("program" in cmd or "flash" in cmd for cmd in commands))
        self.assertEqual(commands[-2:], ["resume", "shutdown"])
        self.assertIn("mww 0xe000ed08 0x20040000", commands)

    def test_server_adapter_and_rtt(self):
        segger = probe.server_command(config.defaults())
        self.assertIn("-vd", segger)
        self.assertIn("19021", segger)
        for product, filename in (
            ("bitbox02", "openocd-bitbox02.cfg"),
            ("bitbox02nova", "openocd-bitbox02.cfg"),
            ("bitbox03", "openocd-bitbox03.cfg"),
        ):
            selected = replace(config.defaults(product), probe_software="openocd")
            self.assertIn(str(ROOT / "scripts" / filename), probe.server_command(selected))
            self.assertIn(
                f"set PROBE_ADAPTER {selected.probe_hardware}", probe.server_command(selected)
            )
            commands = probe.gdb_commands(selected, config.image_info(selected, "firmware"))
            self.assertIn("rtt server start 19021 0", commands)
            self.assertIn("rtt server start 19022 1", commands)

    def test_bitbox03_openocd_adapter_is_selected_before_shared_config(self):
        for board, adapter in (("dev-kit", "stlink"), ("testboard", "jlink")):
            selected = replace(config.defaults("bitbox03"), board=board)
            with patch.object(shutil, "which", return_value="openocd"), patch.object(
                probe.subprocess, "run"
            ) as run:
                probe.execute(selected, "server")
            command = run.call_args.args[0]
            self.assertEqual(command[0], "openocd")
            self.assertIn(str(ROOT / "scripts/openocd-bitbox03.cfg"), command)
            self.assertLess(command.index(f"set PROBE_ADAPTER {adapter}"), command.index("-f"))

    def test_missing_images_tools_and_invalid_config_never_access_device(self):
        with patch.object(probe.subprocess, "run") as run:
            with patch.object(Path, "is_file", return_value=False):
                for action in ("flash", "run"):
                    with self.assertRaisesRegex(ValueError, "make bootloader-stage1"):
                        probe.execute(config.defaults(), action, "bootloader-stage1")
            with patch.object(Path, "is_file", return_value=True), patch.object(
                shutil, "which", return_value=None
            ):
                with self.assertRaisesRegex(ValueError, "Required tool"):
                    probe.execute(config.defaults(), "flash")
            with self.assertRaisesRegex(ValueError, "requires probe software"):
                probe.execute(
                    replace(config.defaults("bitbox03"), probe_software="jlink"), "server"
                )
            run.assert_not_called()

    def test_mocked_tool_invocations(self):
        selected = config.defaults("bitbox03")
        with patch.object(Path, "is_file", return_value=True), patch.object(
            shutil, "which", return_value="tool"
        ), patch.object(probe.subprocess, "run") as run:
            probe.execute(selected, "flash", "factorysetup")
            self.assertEqual(run.call_args.args[0][0], "openocd")
            self.assertIn("load_image", run.call_args.args[0][-1])
            probe.execute(config.defaults(), "flash")
            self.assertEqual(run.call_args.args[0][0], "JLinkExe")
            self.assertIn("-ExitOnError", run.call_args.args[0])
            probe.execute(selected, "run")
            self.assertEqual(run.call_args.args[0][0], "arm-none-eabi-gdb")
            self.assertNotIn("--batch", run.call_args.args[0])


class MakeTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        shutil.copy(ROOT / "Makefile", self.root)
        (self.root / "scripts").mkdir()
        for name in ("build_config.py", "build_product.py", "probe.py", "bitbox03_image_header.py"):
            shutil.copy(ROOT / "scripts" / name, self.root / "scripts")

    def make(self, *args, input_text=None):
        env = dict(os.environ)
        env.pop("MAKEFLAGS", None)
        env.pop("MFLAGS", None)
        return subprocess.run(
            ["make", *args],
            cwd=self.root,
            env=env,
            input=input_text,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )

    def test_scripted_config_defaults_switching_and_build_routing(self):
        self.assertEqual(self.make("config", "CONFIG_ARGS=--defaults").returncode, 0)
        self.assertIn("firmware.elf", self.make("-n", "firmware").stdout)
        self.assertEqual(
            self.make("config", "CONFIG_ARGS=--product bitbox03 --edition btc-only").returncode, 0
        )
        output = self.make("-n", "firmware").stdout
        self.assertIn("bitbox03-firmware-stm32u5a9j-dk", output)
        self.assertIn(str(self.root / "build-bitbox03-cargo"), output)
        self.assertIn("same stub", output)
        result = self.make("config", "CONFIG_ARGS=--product bitbox03 --board testboard")
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("Probe hardware: jlink; software: openocd", result.stdout)
        saved = (self.root / "config.mk").read_bytes()
        self.assertEqual(self.make("config", input_text="\n\n\n\n\n").returncode, 0)
        self.assertEqual((self.root / "config.mk").read_bytes(), saved)
        result = self.make("-n", "firmware")
        self.assertEqual(result.returncode, 0, result.stdout)
        self.assertIn("bitbox03-firmware-stm32u5a9j-dk", result.stdout)
        self.assertIn("testboard currently uses the dev-kit build target", result.stdout)
        for target in ("debug-server", "flash-firmware", "run-firmware"):
            result = self.make("-n", target)
            self.assertEqual(result.returncode, 0, result.stdout)
            self.assertIn('--board "testboard"', result.stdout)
            self.assertIn('--probe-software "openocd"', result.stdout)
        self.assertEqual(
            self.make("config", "CONFIG_ARGS=--product bitbox02nova --edition btc-only").returncode,
            0,
        )
        output = self.make("-n", "firmware").stdout
        self.assertIn("build-bitbox02nova-cmake-relwithdebinfo", output)
        self.assertIn("firmware-btc.elf", output)
        self.assertEqual(self.make("config", "CONFIG_ARGS=--defaults").returncode, 0)
        self.assertIn("build-bitbox02-cmake-relwithdebinfo", self.make("-n", "firmware").stdout)

    def test_swd_speed_defaults_persistence_and_override(self):
        # Existing configurations without SWD_SPEED still use the default.
        (self.root / "config.mk").write_text("PRODUCT = bitbox03\n")
        self.assertIn('--swd-speed "4000"', self.make("-n", "debug-server").stdout)
        self.assertEqual(
            self.make("config", "CONFIG_ARGS=--product bitbox03 --swd-speed 1000").returncode, 0
        )
        self.assertIn("SWD_SPEED = 1000", (self.root / "config.mk").read_text())
        self.assertIn('--swd-speed "1000"', self.make("-n", "debug-server").stdout)
        self.assertIn(
            '--swd-speed "2000"', self.make("-n", "debug-server", "SWD_SPEED=2000").stdout
        )
        self.assertIn('-speed "1000"', self.make("-n", "jlink-gdb-server").stdout)
        self.assertIn('--swd-speed "1000"', self.make("-n", "flash-bootloader-stage0").stdout)
        self.assertEqual(self.make("config", input_text="\n\n\n1500\n").returncode, 0)
        self.assertIn("SWD_SPEED = 1500", (self.root / "config.mk").read_text())
        saved = (self.root / "config.mk").read_bytes()
        for invalid in ("0", "-1", "fast"):
            self.assertNotEqual(
                self.make("config", f"CONFIG_ARGS=--swd-speed {invalid}").returncode, 0
            )
            self.assertEqual((self.root / "config.mk").read_bytes(), saved)
        self.assertNotEqual(self.make("config", input_text="\n\n\n0\n").returncode, 0)
        self.assertEqual((self.root / "config.mk").read_bytes(), saved)
        self.assertEqual(self.make("config", "CONFIG_ARGS=--defaults").returncode, 0)
        self.assertIn("SWD_SPEED = 4000", (self.root / "config.mk").read_text())

    def test_config_cannot_be_combined_and_invalid_preserves_saved_file(self):
        self.make("config", "CONFIG_ARGS=--defaults")
        saved = (self.root / "config.mk").read_bytes()
        for args in (
            ("config", "firmware"),
            ("firmware", "config"),
            ("config", "CONFIG_ARGS=--product bitbox03 --board bitbox03"),
            ("config", "CONFIG_ARGS=--product bitbox03 --probe-software jlink"),
        ):
            result = self.make(*args)
            self.assertNotEqual(result.returncode, 0, result.stdout)
            self.assertEqual((self.root / "config.mk").read_bytes(), saved)
        self.assertNotEqual(self.make("config", input_text="").returncode, 0)
        self.assertEqual((self.root / "config.mk").read_bytes(), saved)

    def test_make_flash_run_routing_and_removed_aliases(self):
        for image in config.IMAGES:
            for action in ("flash", "run"):
                result = self.make("-n", f"{action}-{image}")
                self.assertEqual(result.returncode, 0, result.stdout)
                self.assertIn(f"{action} {image}", result.stdout)
        for target in (
            "firmware-btc",
            "factory-setup",
            "bitbox03-firmware",
            "bootloader-stage0-bitbox02-multi-development",
            "flash-bitbox03-boot0-openocd",
            "flash-bitbox03-boot1-openocd",
            "run-debug",
            "run-factorysetup-debug",
            "run-factory-setup-debug",
        ):
            self.assertNotEqual(self.make("-n", target).returncode, 0)

    def test_clean_preserves_configuration(self):
        self.make("config", "CONFIG_ARGS=--defaults")
        for directory in (
            "build-bitbox02-cmake-debug",
            "build-bitbox02nova-cmake-relwithdebinfo",
            "build-bitbox03-cargo",
        ):
            (self.root / directory).mkdir()
        self.assertEqual(self.make("clean").returncode, 0)
        self.assertTrue((self.root / "config.mk").is_file())
        self.assertEqual(list(self.root.glob("build-*")), [])


@unittest.skipUnless(shutil.which("arm-none-eabi-as"), "ARM binutils unavailable")
class Stage1Tests(unittest.TestCase):
    def test_finalized_elf_matches_binary_and_preserves_symbols(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = Path(tmp)
            source = directory / "image.s"
            source.write_text(
                """.section .stage1_header,"a"
.word 0x31534242, 1
.short 1, 1
.word 1024
.quad 0
.space 1000
.section .vectors,"a"
.word 0x20010000, 0x2409
.section .text,"ax"
.global reset_handler
.thumb_func
reset_handler:
 b reset_handler
"""
            )
            linker = directory / "image.ld"
            linker.write_text(
                "SECTIONS { .stage1_header 0x2000 : { *(.stage1_header) } .vectors 0x2400 : { *(.vectors) } .text : { *(.text) } }"
            )
            elf = directory / "image.elf"
            obj = directory / "image.o"
            binary = directory / "image.bin"
            subprocess.run(
                ["arm-none-eabi-as", "-mcpu=cortex-m4", str(source), "-o", str(obj)], check=True
            )
            subprocess.run(
                ["arm-none-eabi-ld", "-T", str(linker), str(obj), "-o", str(elf)], check=True
            )
            subprocess.run(
                ["arm-none-eabi-objcopy", "-O", "binary", str(elf), str(binary)], check=True
            )
            original = elf.read_bytes()
            data = bytearray(binary.read_bytes())
            data[16:24] = len(data).to_bytes(8, "little")
            binary.write_bytes(data)
            info = config.Image("stage1", elf, binary, 0x2000, 0x2400)
            finalized = probe.finalized_stage1_elf(info)
            self.assertEqual(elf.read_bytes(), original)
            raw = directory / "roundtrip.bin"
            subprocess.run(
                ["arm-none-eabi-objcopy", "-O", "binary", str(finalized), str(raw)], check=True
            )
            self.assertEqual(raw.read_bytes(), binary.read_bytes())
            symbols = subprocess.check_output(["arm-none-eabi-nm", str(finalized)], text=True)
            self.assertIn("reset_handler", symbols)
            data[-1] ^= 1
            binary.write_bytes(data)
            with self.assertRaisesRegex(ValueError, "differ"):
                probe.finalized_stage1_elf(info)
            data[16:24] = bytes(8)
            binary.write_bytes(data)
            with self.assertRaisesRegex(ValueError, "not finalized"):
                probe.finalized_stage1_elf(info)


if __name__ == "__main__":
    unittest.main()
