#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Flash built images or load and continue them in an attached GDB session."""

import argparse
from dataclasses import replace
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile

from build_config import ROOT, IMAGES, Config, Image, add_config_arguments, from_args, image_info


def require_tool(tool: str) -> None:
    if shutil.which(tool) is None:
        raise ValueError(f"Required tool not found in PATH: {tool}")


def require_artifact(path: Path, target: str) -> None:
    if not path.is_file():
        raise ValueError(f"Image not found: {path}\nBuild it first with: make {target}")


def tcl_quote(path: Path) -> str:
    value = str(path)
    for char in ("\\", '"', "$", "[", "]"):
        value = value.replace(char, "\\" + char)
    return '"' + value + '"'


def openocd_command(config: Config) -> list[str]:
    filename = "openocd-bitbox03.cfg" if config.product == "bitbox03" else "openocd-bitbox02.cfg"
    return [
        "openocd",
        "-c",
        f"set PROBE_ADAPTER {config.probe_hardware}",
        "-f",
        str(ROOT / "scripts" / filename),
        "-c",
        f"adapter speed {config.swd_speed}",
        "-c",
        "gdb_port 3333",
    ]


def jlink_gdb_server(config: Config) -> list[str]:
    return [
        "JLinkGDBServer",
        "-nogui",
        "-if",
        "SWD",
        "-device",
        config.chip_type,
        "-speed",
        str(config.swd_speed),
        "-port",
        "2331",
        "-RTTTelnetPort",
        "19021",
        "-vd",
    ]


def jlink_exe(config: Config) -> list[str]:
    return [
        "JLinkExe",
        "-NoGui",
        "1",
        "-if",
        "SWD",
        "-device",
        config.chip_type,
        "-speed",
        str(config.swd_speed),
        "-autoconnect",
        "1",
        "-ExitOnError",
        "1",
    ]


def server_command(config: Config) -> list[str]:
    if config.probe_software == "openocd":
        return openocd_command(config)
    return jlink_gdb_server(config)


def openocd_flash_commands(info: Image) -> str:
    path = tcl_quote(info.elf)
    return f"program {path} verify reset exit"


def jlink_flash_commands(info: Image) -> str:
    # Commander paths are quoted to support workspaces containing spaces.
    path = str(info.elf)
    if any(char in path for char in ('"', "\n", "\r")):
        raise ValueError("Unsupported character in J-Link image path")
    return f'r\nh\nloadfile "{path}" 0 noreset\nr\ng\nq\n'


def execute(
    config: Config, action: str, image: str = "firmware", profile: str | None = None
) -> None:
    config.validate()
    if action == "server":
        command = server_command(config)
        require_tool(command[0])
        subprocess.run(command, check=True)
        return
    info = image_info(config, image, profile)
    if action == "flash" and info.ram:
        raise ValueError(f"{config.product} {image} runs from RAM; use make run-{image} instead")
    target = f"{image}-{profile}" if profile else f"{image} DEBUG={config.debug}"
    require_artifact(info.elf, target)
    if action == "flash":
        command = (
            openocd_command(config) if config.probe_software == "openocd" else jlink_exe(config)
        )
        require_tool(command[0])
        if config.probe_software == "openocd":
            subprocess.run([*command, "-c", openocd_flash_commands(info)], check=True)
        else:
            with tempfile.TemporaryDirectory(prefix="bitbox-probe-") as tmp:
                script = Path(tmp) / "flash.jlink"
                if info.elf.suffix != ".elf":
                    # Give extensionless Cargo artifacts an ELF suffix for Commander.
                    elf = Path(tmp) / "image.elf"
                    elf.symlink_to(info.elf.resolve())
                    info = replace(info, elf=elf)
                script.write_text(jlink_flash_commands(info), encoding="utf-8")
                subprocess.run([*command, "-CommanderScript", str(script)], check=True)
        return
    require_tool("arm-none-eabi-gdb")
    # Ctrl-C must interrupt the inferior without terminating the attached GDB.
    previous = signal.signal(signal.SIGINT, lambda _signum, _frame: None)
    try:
        subprocess.run(
            [
                "arm-none-eabi-gdb",
                "-q",
                str(info.elf),
                "-ex",
                f"set $vectors = {info.vectors:#x}",
                "-x",
                str(ROOT / "scripts" / f"{config.probe_software}.gdb"),
            ],
            check=True,
        )
    finally:
        signal.signal(signal.SIGINT, previous)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    add_config_arguments(parser)
    parser.add_argument("action", choices=("flash", "run", "server"))
    parser.add_argument("image", choices=IMAGES, nargs="?", default="firmware")
    parser.add_argument("--profile", choices=("debug", "release"))
    args = parser.parse_args()
    try:
        execute(from_args(args), args.action, args.image, args.profile)
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
