# SPDX-License-Identifier: Apache-2.0

# This makefile is used as a command runner and not for tracking dependencies between recipies

UNAME_S := $(shell uname -s)

.DEFAULT_GOAL := firmware
SANITIZE ?= ON
RUST_WORKSPACE_MANIFEST := src/rust/Cargo.toml
BITBOX02_FIRMWARE_CARGO_CONFIG := src/rust/bins/bitbox02-firmware/.cargo/config.toml
BITBOX02_FIRMWARE_CARGO_ARGS := --release --manifest-path $(RUST_WORKSPACE_MANIFEST) -p bitbox02-firmware --config $(BITBOX02_FIRMWARE_CARGO_CONFIG)
BITBOX02_FIRMWARE_TARGET_DIR := src/rust/target/thumbv7em-none-eabi/release
BITBOX02_FIRMWARE_ELF := $(BITBOX02_FIRMWARE_TARGET_DIR)/bitbox02-firmware
BITBOX02_BOOTLOADER_CARGO_CONFIG := src/rust/bins/bitbox02-bootloader/.cargo/config.toml
BITBOX02_BOOTLOADER_CARGO_ARGS := --manifest-path $(RUST_WORKSPACE_MANIFEST) -p bitbox02-bootloader --config $(BITBOX02_BOOTLOADER_CARGO_CONFIG)
BITBOX02_BOOTLOADER_TARGET_DIR_RELEASE := src/rust/target/thumbv7em-none-eabi/release
BITBOX02_BOOTLOADER_TARGET_DIR_DEBUG := src/rust/target/thumbv7em-none-eabi/debug

define build_bootloader_release
	mkdir -p build/bin build/scripts
	./scripts/dev_exec.sh cargo build $(BITBOX02_BOOTLOADER_CARGO_ARGS) --release --features $(2)
	cp $(BITBOX02_BOOTLOADER_TARGET_DIR_RELEASE)/bitbox02-bootloader build/bin/$(1).elf
	cp $(BITBOX02_BOOTLOADER_TARGET_DIR_RELEASE)/$(1).map build/bin/$(1).map
	arm-none-eabi-size build/bin/$(1).elf
	arm-none-eabi-objcopy -O binary build/bin/$(1).elf build/bin/$(1).bin
	python3 scripts/expand_template scripts/template-bootloader.jlink file=build/bin/$(1).bin -o build/scripts/$(1).jlink
endef

define build_bootloader_debug
	mkdir -p build/bin build/scripts
	./scripts/dev_exec.sh cargo build $(BITBOX02_BOOTLOADER_CARGO_ARGS) --features $(2)
	cp $(BITBOX02_BOOTLOADER_TARGET_DIR_DEBUG)/bitbox02-bootloader build/bin/$(1).elf
	cp $(BITBOX02_BOOTLOADER_TARGET_DIR_DEBUG)/$(1).map build/bin/$(1).map
	arm-none-eabi-size build/bin/$(1).elf
	arm-none-eabi-objcopy -O binary build/bin/$(1).elf build/bin/$(1).bin
	python3 scripts/expand_template scripts/template-bootloader.jlink file=build/bin/$(1).bin -o build/scripts/$(1).jlink
endef

bootstrap:
	git submodule update --init --recursive

build/Makefile:
	mkdir -p build
	cd build && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake ..
	$(MAKE) -C py/bitbox02

build-debug/Makefile:
	mkdir -p build-debug
	cd build-debug && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DCMAKE_BUILD_TYPE=DEBUG ..
	$(MAKE) -C py/bitbox02

build-build/Makefile:
	mkdir -p build-build
	cd build-build && cmake .. -DCOVERAGE=ON -DSANITIZE_ADDRESS=$(SANITIZE) -DSANITIZE_UNDEFINED=$(SANITIZE)
	$(MAKE) -C py/bitbox02

# ubsan/asan not supported with simulators and rust unit tests
build-build-noasan/Makefile:
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

firmware:
	mkdir -p build/bin build/scripts
	./scripts/dev_exec.sh cargo build $(BITBOX02_FIRMWARE_CARGO_ARGS)
	cp $(BITBOX02_FIRMWARE_ELF) build/bin/firmware.elf
	cp $(BITBOX02_FIRMWARE_TARGET_DIR)/firmware.map build/bin/firmware.map
	arm-none-eabi-size build/bin/firmware.elf
	arm-none-eabi-objcopy -O binary build/bin/firmware.elf build/bin/firmware.bin
	python3 scripts/expand_template scripts/template-firmware.jlink file=build/bin/firmware.bin -o build/scripts/firmware.jlink
firmware-btc:
	mkdir -p build/bin build/scripts
	./scripts/dev_exec.sh cargo build $(BITBOX02_FIRMWARE_CARGO_ARGS) --no-default-features --features btc-only
	cp $(BITBOX02_FIRMWARE_ELF) build/bin/firmware-btc.elf
	cp $(BITBOX02_FIRMWARE_TARGET_DIR)/firmware-btc.map build/bin/firmware-btc.map
	arm-none-eabi-size build/bin/firmware-btc.elf
	arm-none-eabi-objcopy -O binary build/bin/firmware-btc.elf build/bin/firmware-btc.bin
	python3 scripts/expand_template scripts/template-firmware.jlink file=build/bin/firmware-btc.bin -o build/scripts/firmware-btc.jlink
firmware-debug: | build-debug
	$(MAKE) -C build-debug firmware.elf

bootloader:
	$(call build_bootloader_release,bb02-bl-multi,target-bb02-bl-multi)
bootloader-development:
	$(call build_bootloader_release,bb02-bl-multi-development,target-bb02-bl-multi-development)
bootloader-development-locked:
	$(call build_bootloader_release,bb02-bl-multi-development-locked,target-bb02-bl-multi-development-locked)
bootloader-production:
	$(call build_bootloader_release,bb02-bl-multi-production,target-bb02-bl-multi-production)
bootloader-debug:
	$(call build_bootloader_debug,bb02-bl-multi-development,target-bb02-bl-multi-development)

bootloader-btc:
	$(call build_bootloader_release,bb02-bl-btconly,target-bb02-bl-btconly)
bootloader-btc-development:
	$(call build_bootloader_release,bb02-bl-btconly-development,target-bb02-bl-btconly-development)
bootloader-btc-production:
	$(call build_bootloader_release,bb02-bl-btconly-production,target-bb02-bl-btconly-production)

bootloader-plus:
	$(call build_bootloader_release,bb02p-bl-multi,target-bb02p-bl-multi)
bootloader-plus-development:
	$(call build_bootloader_release,bb02p-bl-multi-development,target-bb02p-bl-multi-development)
bootloader-plus-production:
	$(call build_bootloader_release,bb02p-bl-multi-production,target-bb02p-bl-multi-production)
bootloader-plus-debug:
	$(call build_bootloader_debug,bb02p-bl-multi-development,target-bb02p-bl-multi-development)

bootloader-plus-btc:
	$(call build_bootloader_release,bb02p-bl-btconly,target-bb02p-bl-btconly)
bootloader-plus-btc-development:
	$(call build_bootloader_release,bb02p-bl-btconly-development,target-bb02p-bl-btconly-development)
bootloader-plus-btc-production:
	$(call build_bootloader_release,bb02p-bl-btconly-production,target-bb02p-bl-btconly-production)

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
	CTEST_OUTPUT_ON_FAILURE=1 $(MAKE) -C build-build test
run-rust-unit-tests: | build-build-noasan
	${MAKE} -C build-build-noasan rust-test
run-rust-clippy: | build-build-noasan
	${MAKE} -C build-build-noasan rust-clippy
# Must run tests before creating coverage report
coverage: | build-build
	${MAKE} -C build-build coverage
#./build/bin/test_ui_component_gestures;
run-valgrind-on-unit-tests:
	$(MAKE) unit-test
	bash -ec 'for exe in build-build/bin/test_*; do  valgrind --leak-check=yes --track-origins=yes --error-exitcode=1 --exit-on-first-error=yes $$exe; done'
flash-dev-firmware: | firmware
	./py/load_firmware.py build/bin/firmware.bin --debug
jlink-flash-bootloader-development: | bootloader-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi-development.jlink
jlink-flash-bootloader-plus-development: | bootloader-plus-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02p-bl-multi-development.jlink
jlink-flash-bootloader-btc-plus-development: | bootloader-plus-btc-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02p-bl-btconly-development.jlink
jlink-flash-bootloader-development-locked: | bootloader-development-locked
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi-development-locked.jlink
jlink-flash-bootloader: | bootloader
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi.jlink
jlink-flash-bootloader-btc-development: | bootloader-btc-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-btconly-development.jlink
jlink-flash-bootloader-btc: | bootloader-btc
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-btconly.jlink
jlink-flash-firmware: | firmware
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | firmware-btc
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-btc.jlink
jlink-flash-factory-setup: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/factory-setup.jlink
jlink-flash-firmware-debug: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-debug/scripts/firmware.jlink
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
jlink-erase-firmware-quick:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/erase-firmware-quick.jlink
jlink-gdb-server:
	JLinkGDBServer -nogui -if SWD -device ATSAMD51J20 -speed 4000
rtt-client:
	telnet localhost 19021
run-debug:
	arm-none-eabi-gdb -x scripts/jlink.gdb build-debug/bin/firmware.elf
run-bootloader:
	arm-none-eabi-gdb -x scripts/jlink-bootloader.gdb build/bin/bb02p-bl-multi-development.elf
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
	(cd external; ./vendor-rust.sh)
