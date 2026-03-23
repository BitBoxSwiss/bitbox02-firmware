#!/usr/bin/env python3
"""
Shared version metadata and header generator.

This script serves two roles:
1. Generate version headers (and optional CMake vars) from a manifest.
2. Provide the legacy `scripts/get_version` behavior via an importable entrypoint.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import textwrap
from string import Template


SEMVER_RE = re.compile(
    r"""
    ^v
    (?:0|[1-9][0-9]*)
    \.
    (?:0|[1-9][0-9]*)
    \.
    (?:0|[1-9][0-9]*)
    (\-
        (?:0|[1-9A-Za-z-][0-9A-Za-z-]*)
        (\.(?:0|[1-9A-Za-z-][0-9A-Za-z-]*))*
    )?
    (\+
        [0-9A-Za-z-]+
        (\.[0-9A-Za-z-]+)*
    )?
    $
    """,
    re.VERBOSE,
)
RELEASE_VERSION_RE = re.compile(r"^v(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)$")

ZERO_GIT_COMMIT_HASH = "0000000000000000000000000000000000000000"
ZERO_GIT_COMMIT_HASH_SHORT = "0000000000"


def eprintln(*args, **kwargs):
    print(*args, **kwargs, file=sys.stderr)


def system(*args, **kwargs):
    res = subprocess.run(
        *args, stdout=subprocess.PIPE, stderr=subprocess.PIPE, encoding="utf-8", **kwargs
    )
    if res.returncode != 0:
        eprintln("Failed to run `{}`".format(args[0]))
        eprintln("stderr: {}".format(res.stderr))
    return res


def parse_tags(rows, only_signed, prefix):
    """Parses `git tag` rows and returns matching refnames."""
    rows = [row[4:] for row in rows if row.startswith("tag")]
    rows = [row[2:] for row in rows if row[0] == "Y" or not only_signed]
    if prefix is None:
        rows = [row for row in rows if "/" not in row]
    return rows


def git_list_cmd(prefix, extra_args=None):
    if extra_args is None:
        extra_args = []
    cmd = ["git", "tag", "--list", "--sort=taggerdate"]
    cmd += extra_args
    if prefix is None:
        cmd += [
            "--format=%(objecttype) %(if)%(contents:signature)%(then)Y%(else)N%(end) %(refname:strip=2)"
        ]
    else:
        cmd += [
            "--format=%(objecttype) %(if)%(contents:signature)%(then)Y%(else)N%(end) %(refname:strip=3)",
            "{}/{}".format(prefix, "*"),
        ]
    return cmd


def compute_tag_version(repo_root, prefix, check_gpg=False, verify=False, check_semver=False):
    git = shutil.which("git")
    if git is None:
        raise RuntimeError("Command `git` not found.")

    res = system(git_list_cmd(prefix, ["--points-at", "HEAD"]), cwd=repo_root)
    if res.returncode != 0:
        raise RuntimeError("Failed to list tags on HEAD")

    rows = res.stdout.strip().splitlines()
    tags = parse_tags(rows, check_gpg, prefix)

    if not tags:
        res = system(git_list_cmd(prefix), cwd=repo_root)
        if res.returncode != 0:
            raise RuntimeError("Failed to list repository tags")

        rows = res.stdout.strip().splitlines()
        tags = parse_tags(rows, check_gpg, prefix)

        if not tags:
            annotated = "signed" if check_gpg else "annotated"
            if prefix is not None:
                raise RuntimeError("No {} tags found with prefix {}".format(annotated, prefix))
            raise RuntimeError("No {} tags found without prefix".format(annotated))

        version = tags[-1]
        selected_tag = version if prefix is None else "{}/{}".format(prefix, version)

        if check_semver and not SEMVER_RE.match(version):
            raise RuntimeError("Invalid format of tag '{}'".format(selected_tag))

        if verify:
            res = system(["git", "tag", "-v", selected_tag], cwd=repo_root)
            if res.returncode != 0:
                raise RuntimeError("Failed to verify tag '{}'".format(selected_tag))

        res = system(
            ["git", "rev-list", "--count", "{}..HEAD".format(selected_tag)],
            cwd=repo_root,
        )
        if res.returncode != 0:
            raise RuntimeError("Failed to count commits since '{}'".format(selected_tag))

        try:
            count = int(res.stdout.strip())
        except ValueError:
            raise RuntimeError(
                "Failed to parse `git rev-list --count` as integer: {}".format(res.stdout.strip())
            )

        version += "+{}".format(count)
    else:
        version = tags[0]
        selected_tag = version if prefix is None else "{}/{}".format(prefix, version)

        if check_semver and not SEMVER_RE.match(version):
            raise RuntimeError("Invalid format of tag '{}'".format(selected_tag))

        if verify:
            res = system(["git", "tag", "-v", selected_tag], cwd=repo_root)
            if res.returncode != 0:
                raise RuntimeError("Failed to verify tag '{}'".format(selected_tag))

    res = system(["git", "status", "--porcelain"], cwd=repo_root)
    if res.returncode != 0:
        raise RuntimeError("Failed to read git status")

    if res.stdout.strip():
        if "+" in version:
            version += ".dirty"
        else:
            version += "+dirty"

    return version


def git_output(repo_root, args, default=None):
    git = shutil.which("git")
    if git is None:
        return default
    try:
        output = subprocess.run(
            [git] + list(args),
            cwd=repo_root,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            encoding="utf-8",
            check=False,
        )
    except OSError:
        return default
    if output.returncode != 0:
        return default
    value = output.stdout.strip()
    return value if value else default


def strict_tag_version(repo_root, prefix):
    return compute_tag_version(repo_root, prefix, check_gpg=True, verify=False, check_semver=True)


def load_manifest(manifest_path):
    with open(manifest_path, "r", encoding="utf-8") as infile:
        manifest = json.load(infile)
    for key in ("firmware", "bootloader"):
        value = manifest.get(key)
        if not isinstance(value, str) or not RELEASE_VERSION_RE.match(value):
            raise ValueError("Manifest entry '{}' must be a semver string like v1.2.3".format(key))
    return manifest


def build_version_info(base_version, tag_version, git_commit_hash_short):
    parts = base_version.split(".")
    if len(parts) != 3 or not parts[0].startswith("v"):
        raise ValueError("invalid version format: {}".format(base_version))
    full_version = base_version
    has_metadata = tag_version != base_version
    if has_metadata:
        full_version = "{}-pre+{}".format(base_version, git_commit_hash_short)
    return {
        "base": base_version,
        "full": full_version,
        "full_len": len(full_version),
        "full_w16": "".join("'{}', 0, ".format(ch) for ch in full_version),
        "major": parts[0][1:],
        "minor": parts[1],
        "patch": parts[2],
        "has_metadata": has_metadata,
    }


def render_template(template_path, substitutions):
    with open(template_path, "r", encoding="utf-8") as infile:
        template = Template(infile.read())
    return template.substitute(substitutions)


def write_file(path, contents):
    parent = os.path.dirname(path)
    if parent:
        os.makedirs(parent, exist_ok=True)
    with open(path, "w", encoding="utf-8") as outfile:
        outfile.write(contents)


def cmake_quote(value):
    return value.replace("\\", "\\\\").replace('"', '\\"')


def write_cmake_vars(
    path,
    firmware_info,
    bootloader_info,
    git_firmware_version_string,
    git_bootloader_version_string,
):
    contents = textwrap.dedent(
        """\
        set(GIT_FIRMWARE_VERSION_STRING "{git_firmware_version_string}")
        set(GIT_BOOTLOADER_VERSION_STRING "{git_bootloader_version_string}")
        set(FIRMWARE_VERSION_FULL "{firmware_version_full}")
        set(BOOTLOADER_VERSION_FULL "{bootloader_version_full}")
        set(BOOTLOADER_VERSION_HAS_METADATA {bootloader_has_metadata})
        """
    ).format(
        git_firmware_version_string=cmake_quote(git_firmware_version_string),
        git_bootloader_version_string=cmake_quote(git_bootloader_version_string),
        bootloader_version_full=cmake_quote(bootloader_info["full"]),
        firmware_version_full=cmake_quote(firmware_info["full"]),
        bootloader_has_metadata="TRUE" if bootloader_info["has_metadata"] else "FALSE",
    )
    write_file(path, contents)


def generate_headers(repo_root, output_dir, cmake_vars_out=None, manifest_path=None):
    if manifest_path is None:
        manifest_path = os.path.join(repo_root, "versions.json")
    manifest = load_manifest(manifest_path)

    git_commit_hash = git_output(repo_root, ["rev-parse", "HEAD"], ZERO_GIT_COMMIT_HASH)
    git_commit_hash_short = git_output(
        repo_root, ["rev-parse", "--short=10", "HEAD"], ZERO_GIT_COMMIT_HASH_SHORT
    )

    git_firmware_version_string = strict_tag_version(repo_root, "firmware")
    git_bootloader_version_string = strict_tag_version(repo_root, "bootloader")
    firmware_info = build_version_info(
        manifest["firmware"],
        git_firmware_version_string,
        git_commit_hash_short,
    )
    bootloader_info = build_version_info(
        manifest["bootloader"],
        git_bootloader_version_string,
        git_commit_hash_short,
    )

    substitutions = {
        "FIRMWARE_VERSION_FULL": firmware_info["full"],
        "FIRMWARE_VERSION": firmware_info["base"],
        "FIRMWARE_VERSION_FULL_LEN": str(firmware_info["full_len"]),
        "FIRMWARE_VERSION_FULL_W16": firmware_info["full_w16"],
        "FIRMWARE_VERSION_MAJOR": firmware_info["major"],
        "FIRMWARE_VERSION_MINOR": firmware_info["minor"],
        "FIRMWARE_VERSION_PATCH": firmware_info["patch"],
        "GIT_COMMIT_HASH": git_commit_hash,
        "GIT_COMMIT_HASH_SHORT": git_commit_hash_short,
        "BOOTLOADER_VERSION_FULL": bootloader_info["full"],
        "BOOTLOADER_VERSION_FULL_W16": bootloader_info["full_w16"],
        "BOOTLOADER_VERSION_FULL_LEN": str(bootloader_info["full_len"]),
    }

    write_file(
        os.path.join(output_dir, "version.h"),
        render_template(
            os.path.join(repo_root, "src", "version.h.tmpl"),
            substitutions,
        ),
    )
    write_file(
        os.path.join(output_dir, "bootloader", "bootloader_version.h"),
        render_template(
            os.path.join(repo_root, "src", "bootloader", "bootloader_version.h.tmpl"),
            substitutions,
        ),
    )
    if cmake_vars_out is not None:
        write_cmake_vars(
            cmake_vars_out,
            firmware_info,
            bootloader_info,
            git_firmware_version_string,
            git_bootloader_version_string,
        )


def main_generate(argv=None):
    parser = argparse.ArgumentParser(description="Generate version headers from versions.json")
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--cmake-vars-out")
    parser.add_argument("--manifest")
    args = parser.parse_args(argv)

    try:
        generate_headers(
            repo_root=os.path.abspath(args.repo_root),
            output_dir=os.path.abspath(args.output_dir),
            cmake_vars_out=(
                os.path.abspath(args.cmake_vars_out) if args.cmake_vars_out is not None else None
            ),
            manifest_path=(os.path.abspath(args.manifest) if args.manifest is not None else None),
        )
    except (RuntimeError, ValueError) as err:
        eprintln(err)
        return 1
    return 0


def main_get_version(argv=None):
    parser = argparse.ArgumentParser(
        description=textwrap.dedent(
            """
            %(prog)s is a tool for creating a version string out of annotated tags. If there isn't any
            tag on the current HEAD it will print the newest tag concatenated with the count of commits
            since that commit (i.e. vX.Y.Z-COUNT, similar to git-describe).

            If there are modified or untracked files in the repository it will append `+dirty` to the
            version.

            Optionally it is also possible to enforce that the tag is signed and return an error
            otherwise.

            Using `prefix` it supports "monorepo" style repositories, where many "components" live in
            the same repository with individual release schedules. Releases must then be tagged with
            <prefix>/vX.Y.Z. If `prefix` is used and there isn't any tag on HEAD it will count the
            commits since the newest tag with the correct prefix.

            `--list` can be used for debugging, it will print all tags in the repository with
            information about if the tags. The first column indicates if it is a lightweight (commit)
            tag or if it is an annotated tag (tag). The second column shows if it contains a signature
            (Y) or not (N). The third column is the ref/tag name.
            """
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("prefix", nargs="?", default=None)
    parser.add_argument(
        "--check-gpg", help="Require a gpg signature of chosen tag", action="store_true"
    )
    parser.add_argument("--verify", help="Verify gpg signature of chosen tag", action="store_true")
    parser.add_argument(
        "--check-semver",
        help="Require tag to follow `vX.Y.Z` naming scheme",
        action="store_true",
    )
    parser.add_argument("--list", help="List all tags", action="store_true")
    args = parser.parse_args(argv)

    git = shutil.which("git")
    if git is None:
        eprintln("Command `git` not found.")
        return 1

    if args.list:
        res = system(git_list_cmd(None))
        if res.returncode != 0:
            return res.returncode
        sys.stdout.write(res.stdout)
        return 0

    try:
        print(
            compute_tag_version(
                repo_root=os.getcwd(),
                prefix=args.prefix,
                check_gpg=args.check_gpg,
                verify=args.verify,
                check_semver=args.check_semver,
            )
        )
    except RuntimeError as err:
        eprintln(err)
        return 1
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description="Version metadata helpers")
    subparsers = parser.add_subparsers(dest="command")
    subparsers.add_parser("generate", help="Generate version headers and optional CMake vars")
    subparsers.add_parser("get-version", help="Print the version derived from git tags")
    args, remaining = parser.parse_known_args(argv)

    if args.command is None:
        parser.error("a command is required")

    if args.command == "generate":
        return main_generate(remaining)
    if args.command == "get-version":
        return main_get_version(remaining)
    parser.error("unknown command")


if __name__ == "__main__":
    sys.exit(main())
