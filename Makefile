# SPDX-License-Identifier: Apache-2.0

# This makefile is used as a command runner and not for tracking dependencies between recipies

UNAME_S := $(shell uname -s)

.DEFAULT_GOAL := firmware
SANITIZE ?= ON

bootstrap:
	git submodule update --init --recursive
	./scripts/bootstrap-cargo-config

build/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build
	cd build && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake ..
	$(MAKE) -C py/bitbox02

build-debug/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-debug
	cd build-debug && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DCMAKE_BUILD_TYPE=DEBUG ..
	$(MAKE) -C py/bitbox02

build-build/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-build
	cd build-build && cmake .. -DCOVERAGE=ON -DSANITIZE_ADDRESS=$(SANITIZE) -DSANITIZE_UNDEFINED=$(SANITIZE)
	$(MAKE) -C py/bitbox02

# ubsan/asan not supported with simulators and rust unit tests
build-build-noasan/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-build-noasan
	cd build-build-noasan && cmake .. -DCOVERAGE=OFF -DSANITIZE_ADDRESS=OFF -DSANITIZE_UNDEFINED=OFF
	$(MAKE) -C py/bitbox02

# Directory for building for "host" machine according to gcc convention
build: build/Makefile

# Directory for building debug build for "host" machine according to gcc convention
build-debug: build-debug/Makefile

# Directory for building for "build" machine according to gcc convention
build-build: build-build/Makefile

# Directory for building for "build" machine according to gcc convention
# Should only be used for rust unit tests since we didn't add support to
# address santizers when they link code compiled with gcc.
build-build-noasan: build-build-noasan/Makefile

firmware: | build
	$(MAKE) -C build firmware.elf
firmware-btc: | build
	$(MAKE) -C build firmware-btc.elf
firmware-debug: | build-debug
	$(MAKE) -C build-debug firmware.elf

firmware-blupgrade-bitbox02-btconly: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02-btconly.elf
firmware-blupgrade-bitbox02-multi: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02-multi.elf
firmware-blupgrade-bitbox02nova-btconly: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02nova-btconly.elf
firmware-blupgrade-bitbox02nova-multi: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02nova-multi.elf
firmware-blupgrade-bitbox02-btconly-development: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02-btconly-development.elf
firmware-blupgrade-bitbox02-multi-development: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02-multi-development.elf
firmware-blupgrade-bitbox02nova-btconly-development: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02nova-btconly-development.elf
firmware-blupgrade-bitbox02nova-multi-development: | build
	$(MAKE) -C build firmware-blupgrade-bitbox02nova-multi-development.elf

# Stage0 aggregate targets build all production/development variants.
bootloader-stage0: | build
	$(MAKE) -C build bootloader-stage0
bootloader-stage0-production: | build
	$(MAKE) -C build bootloader-stage0-production
bootloader-stage0-development: | build
	$(MAKE) -C build bootloader-stage0-development

# Per-product stage0 targets build their matching ELF/bin.
bootloader-stage0-bitbox02-btconly-development: | build
	$(MAKE) -C build bootloader-stage0-bitbox02-btconly-development.elf
bootloader-stage0-bitbox02-btconly-production: | build
	$(MAKE) -C build bootloader-stage0-bitbox02-btconly-production.elf
bootloader-stage0-bitbox02-multi-development: | build
	$(MAKE) -C build bootloader-stage0-bitbox02-multi-development.elf
bootloader-stage0-bitbox02-multi-production: | build
	$(MAKE) -C build bootloader-stage0-bitbox02-multi-production.elf
bootloader-stage0-bitbox02nova-btconly-development: | build
	$(MAKE) -C build bootloader-stage0-bitbox02nova-btconly-development.elf
bootloader-stage0-bitbox02nova-btconly-production: | build
	$(MAKE) -C build bootloader-stage0-bitbox02nova-btconly-production.elf
bootloader-stage0-bitbox02nova-multi-development: | build
	$(MAKE) -C build bootloader-stage0-bitbox02nova-multi-development.elf
bootloader-stage0-bitbox02nova-multi-production: | build
	$(MAKE) -C build bootloader-stage0-bitbox02nova-multi-production.elf

# Stage1 aggregate targets build all production/development variants.
# The per-product stage1 targets build their matching ELF/bin.
bootloader-stage1: | build
	$(MAKE) -C build bootloader-stage1
bootloader-stage1-production: | build
	$(MAKE) -C build bootloader-stage1-production
bootloader-stage1-development: | build
	$(MAKE) -C build bootloader-stage1-development
bootloader-stage1-bitbox02-btconly-development: | build
	$(MAKE) -C build bootloader-stage1-bitbox02-btconly-development.elf
bootloader-stage1-bitbox02-btconly-production: | build
	$(MAKE) -C build bootloader-stage1-bitbox02-btconly-production.elf
bootloader-stage1-bitbox02-multi-development: | build
	$(MAKE) -C build bootloader-stage1-bitbox02-multi-development.elf
bootloader-stage1-bitbox02-multi-production: | build
	$(MAKE) -C build bootloader-stage1-bitbox02-multi-production.elf
bootloader-stage1-bitbox02nova-btconly-development: | build
	$(MAKE) -C build bootloader-stage1-bitbox02nova-btconly-development.elf
bootloader-stage1-bitbox02nova-btconly-production: | build
	$(MAKE) -C build bootloader-stage1-bitbox02nova-btconly-production.elf
bootloader-stage1-bitbox02nova-multi-development: | build
	$(MAKE) -C build bootloader-stage1-bitbox02nova-multi-development.elf
bootloader-stage1-bitbox02nova-multi-production: | build
	$(MAKE) -C build bootloader-stage1-bitbox02nova-multi-production.elf

bootloader-upgrade-assets: | build
	$(MAKE) -C build bootloader-upgrade-assets
bootloader-upgrade-assets-development: | build
	$(MAKE) -C build bootloader-upgrade-assets-development

factory-setup: | build
	$(MAKE) -C build factory-setup.elf
factory-setup-debug: | build-debug
	$(MAKE) -C build-debug factory-setup.elf
docs: | build
	$(MAKE) -C build doc
rust-docs: | build
	$(MAKE) -C build rust-docs
simulator: | build-build-noasan
	$(MAKE) -C build-build-noasan simulator
simulator-graphical: | build-build-noasan
	$(MAKE) -C build-build-noasan simulator-graphical
simulator-graphical-bb03: | build-build-noasan
	$(MAKE) -C build-build-noasan simulator-graphical-bb03
run-simulator: | simulator
	./build-build-noasan/bin/simulator
unit-test: | build-build
	$(MAKE) -C build-build
# Must compile C tests before running them
run-unit-tests: | build-build
	if command -v setarch >/dev/null 2>&1 && setarch "$$(uname -m)" -R true >/dev/null 2>&1; then \
		CTEST_OUTPUT_ON_FAILURE=1 setarch "$$(uname -m)" -R $(MAKE) -C build-build test; \
	else \
		CTEST_OUTPUT_ON_FAILURE=1 $(MAKE) -C build-build test; \
	fi
# Only one test thread because of unsafe concurrent access to `SafeData`,
# `mock_sd()` and `mock_memory()`. Using mutexes instead leads to mutex
# poisoning and very messy output in case of a unit test failure.
run-rust-unit-tests:
	./scripts/bootstrap-cargo-config
	cargo test --manifest-path src/rust/Cargo.toml --all-features -- --test-threads 1
run-rust-clippy: | build-build-noasan
	${MAKE} -C build-build-noasan rust-clippy
# Must run tests before creating coverage report
coverage: | build-build
	${MAKE} -C build-build coverage
#./build/bin/test_ui_component_gestures;
run-valgrind-on-unit-tests:
	$(MAKE) unit-test
	bash -ec 'for exe in build-build/bin/test_*; do  valgrind --leak-check=yes --track-origins=yes --error-exitcode=1 --exit-on-first-error=yes $$exe; done'
flash-dev-firmware:
	./py/load_firmware.py build/bin/firmware.bin --debug

# Per-product development stage0/stage1 J-Link wrappers flash already-built images.
jlink-flash-bootloader-stage0-bitbox02-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage0-bitbox02-btconly-development.jlink
jlink-flash-bootloader-stage0-bitbox02-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage0-bitbox02-multi-development.jlink
jlink-flash-bootloader-stage0-bitbox02nova-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage0-bitbox02nova-btconly-development.jlink
jlink-flash-bootloader-stage0-bitbox02nova-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage0-bitbox02nova-multi-development.jlink
jlink-flash-bootloader-stage1-bitbox02-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage1-bitbox02-btconly-development.jlink
jlink-flash-bootloader-stage1-bitbox02-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage1-bitbox02-multi-development.jlink
jlink-flash-bootloader-stage1-bitbox02nova-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage1-bitbox02nova-btconly-development.jlink
jlink-flash-bootloader-stage1-bitbox02nova-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-stage1-bitbox02nova-multi-development.jlink

jlink-flash-firmware: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-btc.jlink
jlink-flash-factory-setup: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/factory-setup.jlink
jlink-flash-firmware-debug: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-debug/scripts/firmware.jlink

jlink-flash-firmware-blupgrade-bitbox02-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-blupgrade-bitbox02-btconly-development.jlink
jlink-flash-firmware-blupgrade-bitbox02-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-blupgrade-bitbox02-multi-development.jlink
jlink-flash-firmware-blupgrade-bitbox02nova-btconly-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-blupgrade-bitbox02nova-btconly-development.jlink
jlink-flash-firmware-blupgrade-bitbox02nova-multi-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-blupgrade-bitbox02nova-multi-development.jlink

jlink-flash-set-new-screen:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/set-new-screen.jlink
jlink-flash-set-original-screen:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/set-original-screen.jlink
jlink-flash-reset-version:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/reset-version.jlink
jlink-flash-set-securechip-optiga:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/set-securechip-optiga.jlink
jlink-flash-set-bb02plus:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/set-bb02plus.jlink
jlink-flash-bb02-set-factory-randomness:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/bb02-set-factory-randomness.jlink
jlink-erase-firmware-quick:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/erase-firmware-quick.jlink
jlink-gdb-server:
	JLinkGDBServer -nogui -if SWD -device ATSAMD51J20 -speed 4000
rtt-client:
	telnet localhost 19021
run-debug:
	arm-none-eabi-gdb -x scripts/jlink.gdb build-debug/bin/firmware.elf
run-bootloader-stage0:
	arm-none-eabi-gdb -x scripts/jlink-bootloader-stage0.gdb build/bin/bootloader-stage0-bitbox02-multi-development.elf
run-bootloader-stage1:
	arm-none-eabi-gdb -x scripts/jlink-bootloader-stage1.gdb build/bin/bootloader-stage1-bitbox02-multi-development.elf
run-factory-setup-debug:
	arm-none-eabi-gdb -x scripts/jlink.gdb build-debug/bin/factory-setup.elf
dockerinit:
	./scripts/container.sh build --pull -t shiftcrypto/firmware_v2:$(shell cat .containerversion) .
dockerpull:
	./scripts/container.sh pull shiftcrypto/firmware_v2:$(shell cat .containerversion)
dockerdev:
	./scripts/dockerenv.sh
dockerrel:
	./scripts/dockerenv.sh release
generate-protobufs:
	./scripts/generate-protobuf-rust.sh
generate-atecc608-config:
	cd tools/atecc608 && go run main.go
ci:
	./.ci/ci
prepare-tidy: | build build-build
	$(MAKE) -C build rust-cbindgen
	$(MAKE) -C build-build rust-cbindgen
clean:
	rm -rf build build-build build-debug build-build-noasan src/rust/target

# When you vendor rust libs avoid duplicates
vendor-rust-deps:
	./external/vendor-rust.sh
