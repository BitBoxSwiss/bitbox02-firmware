# SPDX-License-Identifier: Apache-2.0

# scripts/probe.py supplies the image's $vectors address.

# Reset before loading, as recommended by the SEGGER GDB Server examples.
target extended-remote :2331
monitor reset
monitor halt

load
compare-sections
# Rust images can select the Rust expression parser; these are C expressions.
set language c
set {unsigned int}0xe000ed08 = $vectors
set $sp = *(unsigned int*)$vectors
set $pc = *(unsigned int*)($vectors + 4)
set $xpsr = 0x01000000
set language auto

# Add breakpoints here, or comment out c to stop before the first instruction.
c
