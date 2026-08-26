#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Import ST-generated files from an STM32U5 STM32Cube project into the repository
layout.

The script takes two inputs:
- the source STM32Cube project directory
- the target board name

The source project must contain `Core/Inc/main.h`, `Core/Src/main.c`,
`Core/Inc/stm32u5xx_hal_conf.h`, `Drivers/CMSIS`, and
`Drivers/STM32U5xx_HAL_Driver`. `main.c` must contain exactly one static
declaration and definition of `SystemPower_Config()`. The imported output is
split into:
- `external/ST/<board>/Inc` and `external/ST/<board>/Src` for
  board-specific application code copied from `Core/Inc` and `Core/Src`, except
  for `stm32u5xx_hal_conf.h` and `*_it.c`/`*_it.h` interrupt files
- `external/ST/Common/Inc` for common project configuration copied from
  `Core/Inc/stm32u5xx_hal_conf.h`
- `external/ST/Drivers` for shared vendor code copied from `Drivers/CMSIS`
  and `Drivers/STM32U5xx_HAL_Driver`

During import, the script also rewrites the Cube board files so they match the
firmware naming used here:
- `main.c` becomes `board.c`
- `main.h` becomes `board.h`
- C includes and the `main.h` include guard are updated for the new name
- `SystemPower_Config()` is made non-static
Text file line endings are normalized to LF.

Existing `external/ST/<board>`, `external/ST/Common`, and
`external/ST/Drivers` directories are replaced entirely rather than merged.

The script intentionally performs only the transformations required by this
repository while keeping shared drivers separate from per-board code.
"""

import argparse
import re
import shutil
import sys
import tempfile
from collections.abc import Sequence
from pathlib import Path
from typing import Any


BOARD_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")
HAL_CONF = Path("Core/Inc/stm32u5xx_hal_conf.h")
HAL_DRIVER = Path("Drivers/STM32U5xx_HAL_Driver")
REQUIRED_DIRECTORIES = (
    Path("Core/Inc"),
    Path("Core/Src"),
    Path("Drivers/CMSIS"),
    HAL_DRIVER,
)
REQUIRED_FILES = (HAL_CONF,)
RESERVED_BOARD_NAMES = frozenset(("Core", "Common", "Drivers"))
# Interrupt routines are implemented in Rust in this repository, so the
# Cube-generated *_it sources must not be imported.
BOARD_FILE_EXCLUDES = ("*_it.c", "*_it.h")


def eprint(*args: object, **kwargs: Any) -> None:
    print(*args, file=sys.stderr, **kwargs)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Import an STM32U5 STM32Cube project, replacing external/ST/<board>, "
            "external/ST/Common and external/ST/Drivers."
        ),
    )
    parser.add_argument(
        "source",
        type=Path,
        help="Path to the STM32U5 STM32Cube project directory to import from.",
    )
    parser.add_argument(
        "board",
        help="Board name used under external/ST/<board>.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print the replacement plan without modifying the repository.",
    )
    return parser.parse_args()


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent


def validate_board(board: str) -> None:
    if not BOARD_RE.fullmatch(board):
        raise ValueError(
            "board must match {} (letters, digits, '.', '_' and '-', no slashes)".format(
                BOARD_RE.pattern
            )
        )
    if board in RESERVED_BOARD_NAMES:
        raise ValueError("board name '{}' is reserved".format(board))


def collect_copy_directories(source: Path) -> tuple[list[Path], list[Path]]:
    missing = [path for path in REQUIRED_DIRECTORIES if not (source / path).is_dir()]
    if missing:
        raise FileNotFoundError(
            "source project is missing required directories:\n{}".format(
                "\n".join("  - {}".format(path) for path in missing)
            )
        )

    missing = [path for path in REQUIRED_FILES if not (source / path).is_file()]
    if missing:
        raise FileNotFoundError(
            "source project is missing required files:\n{}".format(
                "\n".join("  - {}".format(path) for path in missing)
            )
        )

    board_dirs = [Path("Core/Inc"), Path("Core/Src")]
    driver_dirs = [Path("Drivers/CMSIS"), HAL_DRIVER]
    return board_dirs, driver_dirs


def print_plan(
    source: Path,
    board_destination: Path,
    common_destination: Path,
    driver_destination: Path,
    board_dirs: Sequence[Path],
    driver_dirs: Sequence[Path],
) -> None:
    print("Source:      {}".format(source))
    print("Board:       {}".format(board_destination))
    print("Common:      {}".format(common_destination))
    print("Drivers:     {}".format(driver_destination))
    print("Board directories:")
    for directory in board_dirs:
        print("  - {}".format(directory))
    print("Common files:")
    print("  - {}".format(HAL_CONF))
    print("Driver directories:")
    for directory in driver_dirs:
        print("  - {}".format(directory))


def rewrite_file(path: Path, replacements: Sequence[tuple[str, str]]) -> None:
    content = path.read_text(encoding="utf-8")
    for old, new in replacements:
        content = content.replace(old, new)
    path.write_text(content, encoding="utf-8")


def rewrite_file_regex_once(path: Path, replacements: Sequence[tuple[str, str]]) -> None:
    content = path.read_text(encoding="utf-8")
    for pattern, replacement in replacements:
        content, count = re.subn(pattern, replacement, content)
        if count != 1:
            raise ValueError(
                "expected exactly one match for {!r} in {}, found {}".format(
                    pattern,
                    path,
                    count,
                )
            )
    path.write_text(content, encoding="utf-8")


def rewrite_board_entrypoint(board_dir: Path) -> None:
    board_inc_dir = board_dir / "Inc"
    board_src_dir = board_dir / "Src"

    main_h = board_inc_dir / "main.h"
    main_c = board_src_dir / "main.c"
    board_h = board_inc_dir / "board.h"
    board_c = board_src_dir / "board.c"

    if main_h.exists():
        main_h.rename(board_h)
    if main_c.exists():
        main_c.rename(board_c)

    if not board_h.is_file() or not board_c.is_file():
        raise FileNotFoundError(
            "expected Core/Inc/main.h and Core/Src/main.c in the imported project"
        )

    for path in board_dir.rglob("*"):
        if path.suffix not in (".c", ".h"):
            continue
        rewrite_file(path, [('"main.h"', '"board.h"')])

    rewrite_file(
        board_h,
        [
            ("main.c", "board.c"),
            ("main.h", "board.h"),
            ("__MAIN_H", "__BOARD_H"),
        ],
    )
    rewrite_file(
        board_c,
        [
            ("main.c", "board.c"),
            ("main.h", "board.h"),
        ],
    )
    rewrite_file_regex_once(
        board_c,
        [
            (
                r"static\s+void\s+SystemPower_Config\s*\(\s*void\s*\)\s*;",
                "void SystemPower_Config(void);",
            ),
            (
                r"static\s+void\s+SystemPower_Config\s*\(\s*void\s*\)\s*(?=\{)",
                "void SystemPower_Config(void)\n",
            ),
        ],
    )


def normalize_line_endings(root: Path) -> None:
    for path in root.rglob("*"):
        if not path.is_file() or path.is_symlink():
            continue

        content = path.read_bytes()
        if b"\0" in content:
            continue

        normalized = content.replace(b"\r\n", b"\n").replace(b"\r", b"\n")
        if normalized != content:
            path.write_bytes(normalized)


def remove_excluded_board_files(board_dir: Path) -> None:
    for pattern in BOARD_FILE_EXCLUDES:
        for path in board_dir.rglob(pattern):
            path.unlink()


def remove_common_board_files(board_dir: Path) -> None:
    hal_conf = board_dir / "Inc" / HAL_CONF.name
    if hal_conf.exists():
        hal_conf.unlink()


def copy_common_directory(source: Path, temp_root: Path) -> Path:
    src = source / HAL_CONF
    if not src.is_file():
        raise FileNotFoundError("source project is missing {}".format(HAL_CONF))

    common_dir = temp_root / "Common"
    dst = common_dir / "Inc" / HAL_CONF.name
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)
    return common_dir


def copy_board_directory(source: Path, temp_root: Path, board_dirs: Sequence[Path]) -> Path:
    board_dir = temp_root / "board"
    for directory in board_dirs:
        src = source / directory
        dst = board_dir / directory.relative_to("Core")
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    rewrite_board_entrypoint(board_dir)
    remove_common_board_files(board_dir)
    remove_excluded_board_files(board_dir)
    return board_dir


def copy_driver_directory(source: Path, temp_root: Path, driver_dirs: Sequence[Path]) -> Path:
    drivers_dir = temp_root / "Drivers"
    for directory in driver_dirs:
        src = source / directory
        dst = temp_root / directory
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    return drivers_dir


def replace_directory(source: Path, destination: Path) -> None:
    if destination.exists():
        shutil.rmtree(destination)
    source.rename(destination)


def copy_directories(
    source: Path,
    st_root: Path,
    board: str,
    board_dirs: Sequence[Path],
    driver_dirs: Sequence[Path],
) -> None:
    temp_root = Path(tempfile.mkdtemp(prefix=".copy-st-drivers-", dir=st_root))
    try:
        temp_common_dir = copy_common_directory(source, temp_root)
        temp_board_dir = copy_board_directory(source, temp_root, board_dirs)
        temp_drivers_dir = copy_driver_directory(source, temp_root, driver_dirs)

        normalize_line_endings(temp_root)

        replace_directory(temp_board_dir, st_root / board)
        replace_directory(temp_common_dir, st_root / "Common")
        replace_directory(temp_drivers_dir, st_root / "Drivers")
    except Exception:
        shutil.rmtree(temp_root, ignore_errors=True)
        raise
    shutil.rmtree(temp_root, ignore_errors=True)


def main() -> int:
    args = parse_args()
    try:
        validate_board(args.board)
    except ValueError as exc:
        eprint("error:", exc)
        return 2

    source = args.source.expanduser().resolve()
    if not source.is_dir():
        eprint("error: source directory not found:", source)
        return 2

    st_root = repo_root() / "external" / "ST"
    if not st_root.is_dir():
        eprint("error: expected repository directory missing:", st_root)
        return 2

    try:
        board_dirs, driver_dirs = collect_copy_directories(source)
    except FileNotFoundError as exc:
        eprint("error:", exc)
        return 2

    board_destination = st_root / args.board
    common_destination = st_root / "Common"
    driver_destination = st_root / "Drivers"
    print_plan(
        source,
        board_destination,
        common_destination,
        driver_destination,
        board_dirs,
        driver_dirs,
    )

    if args.dry_run:
        return 0

    copy_directories(
        source,
        st_root,
        args.board,
        board_dirs,
        driver_dirs,
    )
    print(
        "Imported ST project files into {}, {} and {}".format(
            board_destination,
            common_destination,
            driver_destination,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
