#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Product configuration and shared build/probe image metadata."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import os
from pathlib import Path
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent
PRODUCTS = ("bitbox02", "bitbox02nova", "bitbox03")
CHIP_TYPES = {
    "bitbox02": "ATSAMD51J20",
    "bitbox02nova": "ATSAMD51J20",
    "bitbox03": "STM32U5A9NJ",
}
EDITIONS = ("multi", "btc-only")
IMAGES = ("firmware", "factorysetup", "bootloader-stage0", "bootloader-stage1")
CARGO_TARGET = "thumbv8m.main-none-eabihf"
DEFAULT_SWD_SPEED = 4000
# Disabled boards retain their intended probe mapping here.
BOARDS = {
    "bitbox02": {"bitbox02": ("jlink", True)},
    "bitbox02nova": {"bitbox02nova": ("jlink", True)},
    "bitbox03": {
        "dev-kit": ("stlink", True),
        "testboard": ("jlink", True),
        "bitbox03": ("jlink", False),
    },
}


@dataclass(frozen=True)
class Config:
    product: str = "bitbox02"
    board: str = "bitbox02"
    edition: str = "multi"
    probe_software: str = "jlink"
    swd_speed: int = DEFAULT_SWD_SPEED

    @property
    def chip_type(self) -> str:
        return CHIP_TYPES[self.product]

    @property
    def probe_hardware(self) -> str:
        return BOARDS[self.product][self.board][0]

    @property
    def software_choices(self) -> tuple[str, ...]:
        return ("openocd",) if self.probe_hardware == "stlink" else ("jlink", "openocd")

    def validate(self) -> Config:
        if self.product not in BOARDS:
            raise ValueError(f"Unknown product: {self.product}")
        if self.board not in BOARDS[self.product]:
            raise ValueError(f"Invalid board {self.board} for {self.product}")
        if not BOARDS[self.product][self.board][1]:
            raise ValueError(f"Board {self.board} is disabled (not implemented)")
        if self.edition not in EDITIONS:
            raise ValueError(f"Invalid edition: {self.edition}")
        if self.probe_software not in self.software_choices:
            raise ValueError(
                f"{self.board} requires probe software: {', '.join(self.software_choices)}"
            )
        if self.swd_speed <= 0:
            raise ValueError("SWD speed must be a positive integer in kHz")
        return self

    def describe(self) -> None:
        print(f"Product: {self.product}; board: {self.board}; edition: {self.edition}")
        print(f"Probe hardware: {self.probe_hardware}; software: {self.probe_software}")
        print(f"SWD speed: {self.swd_speed} kHz")
        if self.product == "bitbox03":
            print("BitBox03: multi and btc-only currently build the same stub.")
            if self.board == "testboard":
                print("BitBox03 testboard currently uses the dev-kit build target.")


def defaults(product: str = "bitbox02") -> Config:
    if product not in PRODUCTS:
        raise ValueError(f"Unknown product: {product}")
    return Config(
        product,
        "dev-kit" if product == "bitbox03" else product,
        "multi",
        "openocd" if product == "bitbox03" else "jlink",
    )


def add_config_arguments(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--product", choices=PRODUCTS)
    parser.add_argument("--board")
    parser.add_argument("--edition", choices=EDITIONS)
    parser.add_argument("--probe-software", choices=("jlink", "openocd"))
    parser.add_argument("--swd-speed", type=int, help="SWD speed in kHz (default: 4000)")


def from_args(args: argparse.Namespace) -> Config:
    base = defaults(args.product or "bitbox02")
    return Config(
        base.product,
        args.board or base.board,
        args.edition or base.edition,
        args.probe_software or base.probe_software,
        args.swd_speed if args.swd_speed is not None else base.swd_speed,
    ).validate()


def read_config(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    result = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        key, sep, value = line.partition("=")
        if sep and key.strip() in ("PRODUCT", "BOARD", "EDITION", "PROBE_SOFTWARE", "SWD_SPEED"):
            result[key.strip()] = value.strip()
    return result


def choose(label: str, choices: tuple[str, ...], saved: str) -> str:
    default = saved if saved in choices else choices[0]
    if len(choices) == 1:
        print(f"{label}: {default}")
        return default
    answer = input(f"{label} ({', '.join(choices)}) [{default}]: ").strip()
    if answer and answer not in choices:
        raise ValueError(f"Invalid {label.lower()}: {answer}")
    return answer or default


def interactive(saved: dict[str, str]) -> Config:
    product = choose("Product", PRODUCTS, saved.get("PRODUCT", "bitbox02"))
    base = defaults(product)
    boards = BOARDS[product]
    for board, (probe, enabled) in boards.items():
        if not enabled:
            print(f"Board {board}: disabled (not implemented; {probe} probe)")
    board = choose(
        "Board", tuple(b for b in boards if boards[b][1]), saved.get("BOARD", base.board)
    )
    edition = choose("Edition", EDITIONS, saved.get("EDITION", base.edition))
    config = Config(product, board, edition, base.probe_software)
    print(f"Probe hardware: {config.probe_hardware} (derived from board)")
    software = choose(
        "Probe software", config.software_choices, saved.get("PROBE_SOFTWARE", base.probe_software)
    )
    saved_speed = saved.get("SWD_SPEED", str(base.swd_speed))
    speed = input(f"SWD speed in kHz [{saved_speed}]: ").strip() or saved_speed
    return Config(product, board, edition, software, int(speed)).validate()


def save_config(path: Path, config: Config) -> None:
    config.validate()
    # Replace only after all input has been validated, on the same filesystem.
    name = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w", dir=path.parent, prefix=".config-", delete=False, encoding="utf-8"
        ) as outfile:
            name = outfile.name
            outfile.write("# Generated by make config. Local configuration; do not commit.\n")
            for key in ("product", "board", "edition", "probe_software", "swd_speed"):
                outfile.write(f"{key.upper()} = {getattr(config, key)}\n")
        os.replace(name, path)
    finally:
        if name and os.path.exists(name):
            os.unlink(name)


def build_dir(config: Config, profile: str = "relwithdebinfo", root: Path = ROOT) -> Path:
    if config.product == "bitbox03":
        return root / "build-bitbox03-cargo"
    return root / f"build-{config.product}-cmake-{profile}"


@dataclass(frozen=True)
class Image:
    name: str
    elf: Path
    vectors: int
    ram: bool = False


def image_info(
    config: Config,
    image: str,
    profile: str | None = None,
    root: Path = ROOT,
    variant: str = "development",
) -> Image:
    config.validate()
    if image not in IMAGES:
        raise ValueError(f"Unknown image: {image}")
    if config.product == "bitbox03":
        name = {
            "bootloader-stage0": "bitbox03-boot0",
            "bootloader-stage1": "bitbox03-boot1",
            "firmware": "bitbox03-firmware",
            "factorysetup": "bitbox03-factorysetup",
        }[image]
        address = {
            "bootloader-stage0": 0x08002000,
            "bootloader-stage1": 0x08010000,
            "firmware": 0x08052000,
            "factorysetup": 0x20040000,
        }[image]
        vectors = address + (0x400 if image in ("firmware", "bootloader-stage1") else 0)
        elf = build_dir(config, root=root) / CARGO_TARGET / (profile or "debug") / name
        return Image(name, elf, vectors, image == "factorysetup")
    if image.startswith("bootloader-"):
        name = f"{image}-{config.product}-{config.edition.replace('-', '')}-{variant}"
        address = 0 if image == "bootloader-stage0" else 0x2000
    else:
        name = (
            "factory-setup"
            if image == "factorysetup"
            else ("firmware-btc" if config.edition == "btc-only" else "firmware")
        )
        address = 0x10000
    vectors = address + (0x400 if image == "bootloader-stage1" else 0)
    directory = build_dir(config, profile or "relwithdebinfo", root) / "bin"
    return Image(name, directory / f"{name}.elf", vectors)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    add_config_arguments(parser)
    parser.add_argument("--defaults", action="store_true")
    args = parser.parse_args()
    try:
        config = (
            from_args(args) if len(sys.argv) > 1 else interactive(read_config(ROOT / "config.mk"))
        )
        save_config(ROOT / "config.mk", config)
        config.describe()
    except (EOFError, KeyboardInterrupt):
        print("\nConfiguration cancelled; config.mk unchanged.", file=sys.stderr)
        return 1
    except (ValueError, OSError) as exc:
        print(f"Configuration not saved: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
