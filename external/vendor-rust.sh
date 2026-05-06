#!/usr/bin/env bash
#
# Script for vendoring our dependencies, including the deps of core/alloc.
#
# This script can be called from any directory. It runs Cargo from the
# <git-project-root>/src/rust directory so rustup loads rust-toolchain.toml, and places the
# dependencies in <git-project-root>/external/vendor.
#
# For some reason Cargo needs to find the dependencies of all rust std libs. Since "test" depends
# on all the other ones, we take the toml-file from it. This means that we vendor libs that we
# don't use in the end (like hashbrown and getopts).
#
# The invocation below depends on the fact that rust std libs "Cargo.lock" has been manually copied
# to be next to the Cargo.toml file in the test directory.
#
# Copying the Cargo.lock file in the rust sysroot image requires root permissions. Therefore it is
# done in the Dockerfile in our setup.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RUST_WORKSPACE_DIR="$REPO_ROOT/src/rust"
VENDOR_DIR="../../external/vendor"

cd "$RUST_WORKSPACE_DIR"

RUST_SYSROOT="$(rustc --print=sysroot)"

RUSTC_BOOTSTRAP=1 cargo vendor \
    --manifest-path Cargo.toml \
    --sync "$RUST_SYSROOT/lib/rustlib/src/rust/library/test/Cargo.toml" \
    "$VENDOR_DIR"
