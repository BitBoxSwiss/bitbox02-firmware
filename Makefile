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

build-btconly/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-btconly
	cd build-btconly && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_EDITION=btconly ..
	$(MAKE) -C py/bitbox02

build-nova/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-nova
	cd build-nova && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova ..
	$(MAKE) -C py/bitbox02

build-nova-btconly/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-nova-btconly
	cd build-nova-btconly && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova -DBITBOX02_EDITION=btconly ..
	$(MAKE) -C py/bitbox02

build-bootloader-development/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-development
	cd build-bootloader-development && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBOOTLOADER_DEVDEVICE=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-nova-development/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-nova-development
	cd build-bootloader-nova-development && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova -DBOOTLOADER_DEVDEVICE=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-btconly-development/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-btconly-development
	cd build-bootloader-btconly-development && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_EDITION=btconly -DBOOTLOADER_DEVDEVICE=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-nova-btconly-development/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-nova-btconly-development
	cd build-bootloader-nova-btconly-development && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova -DBITBOX02_EDITION=btconly -DBOOTLOADER_DEVDEVICE=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-locked/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-locked
	cd build-bootloader-locked && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBOOTLOADER_LOCKED=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-nova-locked/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-nova-locked
	cd build-bootloader-nova-locked && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova -DBOOTLOADER_LOCKED=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-btconly-locked/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-btconly-locked
	cd build-bootloader-btconly-locked && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_EDITION=btconly -DBOOTLOADER_LOCKED=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-nova-btconly-locked/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-nova-btconly-locked
	cd build-bootloader-nova-btconly-locked && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBITBOX02_PLATFORM=nova -DBITBOX02_EDITION=btconly -DBOOTLOADER_LOCKED=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-development-locked/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-development-locked
	cd build-bootloader-development-locked && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DBOOTLOADER_DEVDEVICE=ON -DBOOTLOADER_LOCKED=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-development-debug/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-development-debug
	cd build-bootloader-development-debug && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DCMAKE_BUILD_TYPE=DEBUG -DBOOTLOADER_DEVDEVICE=ON ..
	$(MAKE) -C py/bitbox02

build-bootloader-nova-development-debug/Makefile:
	./scripts/bootstrap-cargo-config
	mkdir -p build-bootloader-nova-development-debug
	cd build-bootloader-nova-development-debug && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DCMAKE_BUILD_TYPE=DEBUG -DBITBOX02_PLATFORM=nova -DBOOTLOADER_DEVDEVICE=ON ..
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

# Directory for building BTC-only firmware and bootloaders
build-btconly: build-btconly/Makefile

# Directory for building Nova bootloaders
build-nova: build-nova/Makefile

# Directory for building BTC-only Nova bootloaders
build-nova-btconly: build-nova-btconly/Makefile

# Directory for building development bootloaders
build-bootloader-development: build-bootloader-development/Makefile

# Directory for building Nova development bootloaders
build-bootloader-nova-development: build-bootloader-nova-development/Makefile

# Directory for building BTC-only development bootloaders
build-bootloader-btconly-development: build-bootloader-btconly-development/Makefile

# Directory for building BTC-only Nova development bootloaders
build-bootloader-nova-btconly-development: build-bootloader-nova-btconly-development/Makefile

# Directory for building locked bootloaders
build-bootloader-locked: build-bootloader-locked/Makefile

# Directory for building Nova locked bootloaders
build-bootloader-nova-locked: build-bootloader-nova-locked/Makefile

# Directory for building BTC-only locked bootloaders
build-bootloader-btconly-locked: build-bootloader-btconly-locked/Makefile

# Directory for building BTC-only Nova locked bootloaders
build-bootloader-nova-btconly-locked: build-bootloader-nova-btconly-locked/Makefile

# Directory for building development locked bootloaders
build-bootloader-development-locked: build-bootloader-development-locked/Makefile

# Directory for building development debug bootloaders
build-bootloader-development-debug: build-bootloader-development-debug/Makefile

# Directory for building Nova development debug bootloaders
build-bootloader-nova-development-debug: build-bootloader-nova-development-debug/Makefile

# Directory for building for "build" machine according to gcc convention
build-build: build-build/Makefile

# Directory for building for "build" machine according to gcc convention
# Should only be used for rust unit tests since we didn't add support to
# address santizers when they link code compiled with gcc.
build-build-noasan: build-build-noasan/Makefile

firmware: | build
	$(MAKE) -C build firmware.elf
firmware-btc: | build-btconly
	$(MAKE) -C build-btconly firmware.elf
firmware-debug: | build-debug
	$(MAKE) -C build-debug firmware.elf

bootloader: | build
	$(MAKE) -C build bootloader.elf
bootloader-development: | build-bootloader-development
	$(MAKE) -C build-bootloader-development bootloader.elf
bootloader-development-locked: | build-bootloader-development-locked
	$(MAKE) -C build-bootloader-development-locked bootloader.elf
bootloader-production: | build-bootloader-locked
	$(MAKE) -C build-bootloader-locked bootloader.elf
bootloader-debug: | build-bootloader-development-debug
	$(MAKE) -C build-bootloader-development-debug bootloader.elf

bootloader-btc: | build-btconly
	$(MAKE) -C build-btconly bootloader.elf
bootloader-btc-development: | build-bootloader-btconly-development
	$(MAKE) -C build-bootloader-btconly-development bootloader.elf
bootloader-btc-production: | build-bootloader-btconly-locked
	$(MAKE) -C build-bootloader-btconly-locked bootloader.elf

bootloader-plus: | build-nova
	$(MAKE) -C build-nova bootloader.elf
bootloader-plus-development: | build-bootloader-nova-development
	$(MAKE) -C build-bootloader-nova-development bootloader.elf
bootloader-plus-production: | build-bootloader-nova-locked
	$(MAKE) -C build-bootloader-nova-locked bootloader.elf
bootloader-plus-debug: | build-bootloader-nova-development-debug
	$(MAKE) -C build-bootloader-nova-development-debug bootloader.elf

bootloader-plus-btc: | build-nova-btconly
	$(MAKE) -C build-nova-btconly bootloader.elf
bootloader-plus-btc-development: | build-bootloader-nova-btconly-development
	$(MAKE) -C build-bootloader-nova-btconly-development bootloader.elf
bootloader-plus-btc-production: | build-bootloader-nova-btconly-locked
	$(MAKE) -C build-bootloader-nova-btconly-locked bootloader.elf

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
jlink-flash-bootloader-development: | build-bootloader-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-bootloader-development/scripts/bootloader.jlink
jlink-flash-bootloader-plus-development: | build-bootloader-nova-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-bootloader-nova-development/scripts/bootloader.jlink
jlink-flash-bootloader-btc-plus-development: | build-bootloader-nova-btconly-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-bootloader-nova-btconly-development/scripts/bootloader.jlink
jlink-flash-bootloader-development-locked: | build-bootloader-development-locked
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-bootloader-development-locked/scripts/bootloader.jlink
jlink-flash-bootloader: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader.jlink
jlink-flash-bootloader-btc-development: | build-bootloader-btconly-development
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-bootloader-btconly-development/scripts/bootloader.jlink
jlink-flash-bootloader-btc: | build-btconly
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-btconly/scripts/bootloader.jlink
jlink-flash-firmware: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | build-btconly
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-btconly/scripts/firmware.jlink
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
run-bootloader:
	arm-none-eabi-gdb -x scripts/jlink-bootloader.gdb build-bootloader-nova-development/bin/bootloader.elf
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
	rm -rf build build-btconly build-nova build-nova-btconly build-bootloader-development build-bootloader-nova-development build-bootloader-btconly-development build-bootloader-nova-btconly-development build-bootloader-locked build-bootloader-nova-locked build-bootloader-btconly-locked build-bootloader-nova-btconly-locked build-bootloader-development-locked build-bootloader-development-debug build-bootloader-nova-development-debug build-build build-debug build-build-noasan src/rust/target

# When you vendor rust libs avoid duplicates
vendor-rust-deps:
	./external/vendor-rust.sh
