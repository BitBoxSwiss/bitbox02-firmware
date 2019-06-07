#!/bin/bash -e

# Check style

./.ci/check-style
make -C py
./.ci/check-pep8

# Bootloader variants
make -j8 bootloader
make -j8 bootloader-devdevice
make -j8 bootloader-production

# Firmware
make -j8 firmware
make -j8 factory-setup

# Unit tests
make -j8 run-unit-tests

# Build device tests
make -j8 device-test
