# SPDX-License-Identifier: Apache-2.0

# scripts/probe.py supplies the image's $vectors address.

# Start RTT only after the application has initialized its channels.
define rtt_start
    monitor rtt setup 0x20000200 0x1000 "SEGGER RTT"
    monitor rtt start
    monitor rtt server start 19021 0
    monitor rtt server start 19022 1
end

target extended-remote :3333
monitor reset init
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
