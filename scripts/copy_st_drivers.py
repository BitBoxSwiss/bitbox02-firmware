#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Import ST-generated files from an STM32Cube project into the repository layout.

The script takes two inputs:
- the source STM32Cube project directory
- the target board name

It assumes the source project uses the usual Cube layout with `Core/` and
`Drivers/` directories. If present, USBX project files under `USBX/` and the
shared USBX middleware under `Middlewares/ST/usbx` are imported as well. The
imported output is split into:
- `external/ST/<board>/Inc` and `external/ST/<board>/Src` for
  board-specific application code copied from `Core/Inc` and `Core/Src`
- `external/ST/Drivers` for shared vendor code copied from `Drivers/CMSIS`
  and any `Drivers/*_HAL_Driver` directories found in the source project
- `external/ST/USBX` for shared USBX application and target code copied from
  `USBX/App` and `USBX/Target`
- `external/ST/Middlewares` for shared vendor middleware copied from
  `Middlewares/ST/usbx`

During import, the script also rewrites the Cube entrypoint so it matches the
firmware naming used here:
- `main.c` becomes `board.c`
- `main.h` becomes `board.h`
- `main()` becomes `board_init()`

The current repository layout is still in flux, so this script intentionally
implements only the minimal transformation needed for repeated imports while
keeping shared drivers separate from per-board code.
"""

import argparse
import re
import shutil
import sys
import tempfile
from pathlib import Path


BOARD_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")
REQUIRED_DIRECTORIES = (
    Path("Core/Inc"),
    Path("Core/Src"),
    Path("Drivers/CMSIS"),
)
OPTIONAL_SHARED_DIRECTORIES = (
    Path("USBX/App"),
    Path("USBX/Target"),
)
OPTIONAL_MIDDLEWARE_DIRECTORIES = (Path("Middlewares/ST/usbx"),)
RESERVED_BOARD_NAMES = frozenset(("Core", "Drivers"))
# Interrupt routines are implemented in Rust in this repository, so the
# Cube-generated *_it sources must not be imported.
BOARD_FILE_EXCLUDES = ("*_it.c", "*_it.h")


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def parse_args():
    parser = argparse.ArgumentParser(
        description=(
            "Copy the ST-generated board/Core tree into external/ST/<board> and "
            "shared drivers into external/ST/Drivers."
        ),
    )
    parser.add_argument(
        "source",
        type=Path,
        help="Path to the STM32Cube project directory to import from.",
    )
    parser.add_argument(
        "board",
        help="Board name used under external/ST/<board>.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print the copy plan without modifying the repository.",
    )
    return parser.parse_args()


def repo_root():
    return Path(__file__).resolve().parent.parent


def validate_board(board):
    if not BOARD_RE.fullmatch(board):
        raise ValueError(
            "board must match {} (letters, digits, '.', '_' and '-', no slashes)".format(
                BOARD_RE.pattern
            )
        )
    if board in RESERVED_BOARD_NAMES:
        raise ValueError("board name '{}' is reserved".format(board))


def hal_driver_directories(source):
    drivers_dir = source / "Drivers"
    if not drivers_dir.is_dir():
        return []
    return sorted(
        path.relative_to(source)
        for path in drivers_dir.iterdir()
        if path.is_dir() and path.name.endswith("_HAL_Driver")
    )


def collect_copy_directories(source):
    missing = [path for path in REQUIRED_DIRECTORIES if not (source / path).is_dir()]
    if missing:
        raise FileNotFoundError(
            "source project is missing required directories:\n{}".format(
                "\n".join("  - {}".format(path) for path in missing)
            )
        )

    hal_dirs = hal_driver_directories(source)
    if not hal_dirs:
        raise FileNotFoundError("source project is missing Drivers/*_HAL_Driver directories")

    board_dirs = [Path("Core/Inc"), Path("Core/Src")]
    driver_dirs = [Path("Drivers/CMSIS"), *hal_dirs]
    shared_dirs = [
        directory for directory in OPTIONAL_SHARED_DIRECTORIES if (source / directory).is_dir()
    ]
    middleware_dirs = [
        directory for directory in OPTIONAL_MIDDLEWARE_DIRECTORIES if (source / directory).is_dir()
    ]
    return board_dirs, driver_dirs, shared_dirs, middleware_dirs


def print_plan(
    source,
    board_destination,
    driver_destination,
    shared_destination,
    middleware_destination,
    board_dirs,
    driver_dirs,
    shared_dirs,
    middleware_dirs,
):
    print("Source:      {}".format(source))
    print("Board:       {}".format(board_destination))
    print("Drivers:     {}".format(driver_destination))
    print("Shared:      {}".format(shared_destination))
    print("Middlewares: {}".format(middleware_destination))
    print("Board directories:")
    for directory in board_dirs:
        print("  - {}".format(directory))
    print("Driver directories:")
    for directory in driver_dirs:
        print("  - {}".format(directory))
    if shared_dirs:
        print("Shared directories:")
        for directory in shared_dirs:
            print("  - {}".format(directory))
    if middleware_dirs:
        print("Middleware directories:")
        for directory in middleware_dirs:
            print("  - {}".format(directory))


def rewrite_file(path, replacements):
    content = path.read_text(encoding="utf-8")
    for old, new in replacements:
        content = content.replace(old, new)
    path.write_text(content, encoding="utf-8")


def rewrite_board_init_signature(path):
    content = path.read_text(encoding="utf-8")
    content = re.sub(r"\bmain\s*\(", "board_init(", content)
    path.write_text(content, encoding="utf-8")


def rewrite_board_entrypoint(board_dir):
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
    rewrite_board_init_signature(board_h)
    rewrite_board_init_signature(board_c)


def rewrite_shared_usbx_headers(shared_root):
    usbx_app_dir = shared_root / "USBX" / "App"
    if not usbx_app_dir.is_dir():
        return

    for path in usbx_app_dir.rglob("*"):
        if path.suffix not in (".c", ".h"):
            continue
        rewrite_file(path, [('"main.h"', '"board.h"')])


def remove_excluded_board_files(board_dir):
    for pattern in BOARD_FILE_EXCLUDES:
        for path in board_dir.rglob(pattern):
            path.unlink()


def copy_board_directory(source, temp_root, board_dirs):
    board_dir = temp_root / "board"
    for directory in board_dirs:
        src = source / directory
        if directory.parts[0] == "Core":
            dst = board_dir / directory.relative_to("Core")
        else:
            dst = board_dir / directory
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    rewrite_board_entrypoint(board_dir)
    remove_excluded_board_files(board_dir)
    return board_dir


def copy_driver_directory(source, temp_root, driver_dirs):
    drivers_dir = temp_root / "Drivers"
    for directory in driver_dirs:
        src = source / directory
        dst = temp_root / directory
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    return drivers_dir


def copy_shared_directory(source, temp_root, shared_dirs):
    shared_root = temp_root / "shared"
    for directory in shared_dirs:
        src = source / directory
        dst = shared_root / directory
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    rewrite_shared_usbx_headers(shared_root)
    return shared_root


def copy_middleware_directory(source, temp_root, middleware_dirs):
    middlewares_dir = temp_root / "Middlewares"
    for directory in middleware_dirs:
        src = source / directory
        dst = temp_root / directory
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(src, dst)
    return middlewares_dir


def replace_directory(source, destination):
    if destination.exists():
        shutil.rmtree(destination)
    source.rename(destination)


def copy_directories(source, st_root, board, board_dirs, driver_dirs, shared_dirs, middleware_dirs):
    temp_root = Path(tempfile.mkdtemp(prefix=".copy-st-drivers-", dir=st_root))
    try:
        temp_board_dir = copy_board_directory(source, temp_root, board_dirs)
        temp_drivers_dir = copy_driver_directory(source, temp_root, driver_dirs)
        temp_shared_dir = None
        if shared_dirs:
            temp_shared_dir = copy_shared_directory(source, temp_root, shared_dirs)
        temp_middlewares_dir = None
        if middleware_dirs:
            temp_middlewares_dir = copy_middleware_directory(source, temp_root, middleware_dirs)

        replace_directory(temp_board_dir, st_root / board)
        replace_directory(temp_drivers_dir, st_root / "Drivers")
        if temp_shared_dir is not None:
            replace_directory(temp_shared_dir / "USBX", st_root / "USBX")
        if temp_middlewares_dir is not None:
            replace_directory(temp_middlewares_dir, st_root / "Middlewares")
    except Exception:
        shutil.rmtree(temp_root, ignore_errors=True)
        raise
    shutil.rmtree(temp_root, ignore_errors=True)


def main():
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
        board_dirs, driver_dirs, shared_dirs, middleware_dirs = collect_copy_directories(source)
    except FileNotFoundError as exc:
        eprint("error:", exc)
        return 2

    board_destination = st_root / args.board
    driver_destination = st_root / "Drivers"
    shared_destination = st_root / "USBX"
    middleware_destination = st_root / "Middlewares"
    print_plan(
        source,
        board_destination,
        driver_destination,
        shared_destination,
        middleware_destination,
        board_dirs,
        driver_dirs,
        shared_dirs,
        middleware_dirs,
    )

    if args.dry_run:
        return 0

    copy_directories(
        source,
        st_root,
        args.board,
        board_dirs,
        driver_dirs,
        shared_dirs,
        middleware_dirs,
    )
    print(
        "Imported ST project files into {}, {}, {} and {}".format(
            board_destination,
            driver_destination,
            shared_destination,
            middleware_destination,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
