#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image="${1:-$repo_root/src/rust/target/thumbv8m.main-none-eabihf/debug/bitbox03-boot0}"
config="$repo_root/scripts/stm32u5a9j-dk.cfg"

if [[ ! -f "$image" ]]; then
    echo "image not found: $image" >&2
    exit 1
fi

exec openocd \
    -f "$config" \
    -c "program \"$image\" verify reset exit"
