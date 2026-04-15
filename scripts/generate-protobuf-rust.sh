#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null && pwd)"
OUT_DIR="${ROOT_DIR}/src/rust/bitbox-proto/src/generated"
TMP_DIR="$(mktemp -d)"

cleanup() {
    rm -rf "${TMP_DIR}"
}

trap cleanup EXIT

mkdir -p "${OUT_DIR}"

prost-build-proto \
    "${ROOT_DIR}/messages" \
    "${TMP_DIR}"

rustfmt \
    --edition 2024 \
    "${ROOT_DIR}/src/rust/bitbox-proto/src/lib.rs" \
    "${TMP_DIR}/shiftcrypto.bitbox02.rs" \
    "${TMP_DIR}/shiftcrypto.bitbox02.backups.rs"

for filename in \
    "shiftcrypto.bitbox02.rs" \
    "shiftcrypto.bitbox02.backups.rs"
do
    if [[ ! -f "${OUT_DIR}/${filename}" ]] ||
        ! cmp -s "${TMP_DIR}/${filename}" "${OUT_DIR}/${filename}"; then
        cp "${TMP_DIR}/${filename}" "${OUT_DIR}/${filename}"
    fi
done
