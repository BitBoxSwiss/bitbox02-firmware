#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

config_dir=".cargo"
config_file="$config_dir/config.toml"

mkdir -p "$config_dir"
touch "$config_file"

if grep -q '^\[env\]$' "$config_file"; then
    echo "[env] section already exists in $config_file"
    exit 0
fi

sysroot="$(arm-none-eabi-gcc -print-sysroot)"

if [ -z "$sysroot" ]; then
    echo "arm-none-eabi-gcc did not return a sysroot" >&2
    exit 1
fi

cat >>"$config_file" <<EOF

[env]
"CC_thumbv8m.main-none-eabihf" = { value="clang-21"}
"CFLAGS_thumbv8m.main-none-eabihf" = {value ="-mcpu=cortex-m33 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard -flto --sysroot=$sysroot"}
"BINDGEN_EXTRA_CLANG_ARGS_thumbv8m.main_none_eabihf" = {value = "--sysroot=$sysroot"}
EOF

echo "Appended [env] section to $config_file"
