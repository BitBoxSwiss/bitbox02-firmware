#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
REPO_ROOT="$(realpath "$DIR/..")"
RUST_WORKSPACE_DIR="${REPO_ROOT}/src/rust"
LIBBITBOX02_RUST_SOURCE_DIR="${RUST_WORKSPACE_DIR}/bitbox02-rust-c"

OUTPUT_DIR="${1:?usage: generate_rust_header.sh <output_dir>}"
CARGO_BIN="${CARGO_BIN:-cargo}"
CBINDGEN_BIN="${CBINDGEN_BIN:-cbindgen}"

mkdir -p "${OUTPUT_DIR}"
OUTPUT_DIR="$(realpath "${OUTPUT_DIR}")"
METADATA_PATH="${OUTPUT_DIR}/rust-metadata.json"

(
    cd "${RUST_WORKSPACE_DIR}"
    "${CARGO_BIN}" metadata \
        --offline \
        --format-version 1 \
        --manifest-path "${LIBBITBOX02_RUST_SOURCE_DIR}/Cargo.toml" \
        > "${METADATA_PATH}"
    "${CBINDGEN_BIN}" \
        --quiet \
        --config "${RUST_WORKSPACE_DIR}/bitbox02-cbindgen.toml" \
        --output "${OUTPUT_DIR}/rust.h" \
        --profile release \
        --metadata "${METADATA_PATH}" \
        "${LIBBITBOX02_RUST_SOURCE_DIR}"
)
