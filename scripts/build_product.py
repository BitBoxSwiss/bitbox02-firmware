#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Dispatch command aliases to the existing CMake and Cargo targets."""

import argparse
import os
from pathlib import Path
import shlex
import subprocess
import sys

from build_config import (
    ROOT,
    IMAGES,
    Config,
    add_config_arguments,
    build_dir,
    defaults,
    from_args,
    image_info,
)


def run(command: list[str], cwd: Path = ROOT) -> None:
    print(shlex.join(command), flush=True)
    # GNU make puts short flags first, followed by a space and other options.
    # Preserve an empty first field so long options are not treated as short flags.
    flags = os.environ.get("MAKEFLAGS", "").split(" ", 1)[0]
    if "n" in flags:
        return
    # Keep GNU make's jobserver available to recursive backend builds.
    subprocess.run(command, cwd=cwd, check=True, close_fds=False)


def cmake(config: Config, profile: str, targets: list[str]) -> None:
    directory = build_dir(config, profile)
    run([str(ROOT / "scripts/bootstrap-cargo-config")])
    if not (directory / "Makefile").exists():
        run(
            [
                "cmake",
                "-S",
                str(ROOT),
                "-B",
                str(directory),
                "-DCMAKE_TOOLCHAIN_FILE=arm.cmake",
                f"-DCMAKE_BUILD_TYPE={profile.upper()}",
            ]
        )
        run(["make", "-C", str(ROOT / "py/bitbox02")])
    run(["make", "-C", str(directory), *targets])


def build_image(
    config: Config, image: str, profile: str | None = None, variant: str = "development"
) -> None:
    profile = profile or config.profile
    info = image_info(config, image, profile, variant=variant)
    if config.product != "bitbox03":
        cmake(config, profile, [f"{info.name}.elf"])
        return
    config.describe()
    run([str(ROOT / "scripts/bootstrap-cargo-config")])
    # Execute Cargo from src/rust so it loads the vendored dependency configuration.
    suffix = "-release" if profile == "release" else ""
    run(
        [
            "cargo",
            f"{info.name}-stm32u5a9j-dk{suffix}",
            *shlex.split(os.environ.get("CARGOFLAGS", "")),
            "--target-dir",
            str(build_dir(config)),
        ],
        ROOT / "src/rust",
    )
    if image in ("firmware", "bootloader-stage1"):
        run([sys.executable, "scripts/image_header.py", "finalize-elf", str(info.elf)])
    run(["arm-none-eabi-size", str(info.elf)])
    run(["arm-none-eabi-size", "-Ax", str(info.elf)])


def dispatch(config: Config, target: str) -> None:
    if target in IMAGES:
        build_image(config, target)
    elif target in (
        "firmware-debug",
        "firmware-release",
        "factorysetup-debug",
        "factorysetup-release",
        "bootloader-stage0-debug",
        "bootloader-stage0-release",
        "bootloader-stage1-debug",
        "bootloader-stage1-release",
    ):
        image, profile = target.rsplit("-", 1)
        build_image(config, image, profile)
    elif target.startswith("firmware-blupgrade-"):
        product = target.split("-")[2]
        cmake(defaults(product), "relwithdebinfo", [target + ".elf"])
    elif target.startswith(("bootloader-stage0-", "bootloader-stage1-")):
        image, variant = target.rsplit("-", 1)
        # These aggregates retain their original all-product/all-edition coverage.
        for product in ("bitbox02", "bitbox02nova"):
            for edition in ("multi", "btc-only"):
                base = defaults(product)
                build_image(
                    Config(product, base.board, edition, base.probe_software),
                    image,
                    variant=variant,
                )
    elif target in ("bootloader-upgrade-assets", "bootloader-upgrade-assets-development"):
        suffix = "-development" if target.endswith("-development") else ""
        for product in ("bitbox02", "bitbox02nova"):
            cmake(
                defaults(product),
                "relwithdebinfo",
                [
                    f"bootloader-upgrade-assets-{product}-{edition}{suffix}"
                    for edition in ("multi", "btconly")
                ],
            )
    elif target in ("docs", "rust-docs", "prepare-tidy"):
        cmake(
            defaults(),
            "relwithdebinfo",
            [{"docs": "doc", "rust-docs": "rust-docs", "prepare-tidy": "rust-cbindgen"}[target]],
        )
    else:
        raise ValueError(f"Unknown build target: {target}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    add_config_arguments(parser)
    parser.add_argument("target")
    args = parser.parse_args()
    try:
        dispatch(from_args(args), args.target)
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        print(str(exc), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
