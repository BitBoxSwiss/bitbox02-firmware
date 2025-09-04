# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This makefile is used as a command runner and not for tracking dependencies between recipies

UNAME_S := $(shell uname -s)

.DEFAULT_GOAL := firmware
# asan/ubsan is not supported on darwin, default to off
ifeq ($(UNAME_S),Darwin)
  SANITIZE ?= OFF
else
  SANITIZE ?= ON
endif

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

firmware: | build
	$(MAKE) -C build firmware.elf
firmware-btc: | build
	$(MAKE) -C build firmware-btc.elf
firmware-debug: | build-debug
	$(MAKE) -C build-debug firmware.elf

bootloader: | build
	$(MAKE) -C build bb02-bl-multi.elf
bootloader-development: | build
	$(MAKE) -C build bb02-bl-multi-development.elf
bootloader-development-locked: | build
	$(MAKE) -C build bb02-bl-multi-development-locked.elf
bootloader-production: | build
	$(MAKE) -C build bb02-bl-multi-production.elf
bootloader-debug: | build-debug
	$(MAKE) -C build-debug bb02-bl-multi-development.elf

bootloader-btc: | build
	$(MAKE) -C build bb02-bl-btconly.elf
bootloader-btc-development: | build
	$(MAKE) -C build bb02-bl-btconly-development.elf
bootloader-btc-production: | build
	$(MAKE) -C build bb02-bl-btconly-production.elf

bootloader-plus: | build
	$(MAKE) -C build bb02p-bl-multi.elf
bootloader-plus-development: | build
	$(MAKE) -C build bb02p-bl-multi-development.elf
bootloader-plus-production: | build
	$(MAKE) -C build bb02p-bl-multi-production.elf
bootloader-plus-debug: | build-debug
	$(MAKE) -C build-debug bb02p-bl-multi-development.elf

bootloader-plus-btc: | build
	$(MAKE) -C build bb02p-bl-btconly.elf
bootloader-plus-btc-development: | build
	$(MAKE) -C build bb02p-bl-btconly-development.elf
bootloader-plus-btc-production: | build
	$(MAKE) -C build bb02p-bl-btconly-production.elf

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
simulator-ng: | build-build-noasan
	$(MAKE) -C build-build-noasan simulator-ng
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
flash-dev-firmware:
	./py/load_firmware.py build/bin/firmware.bin --debug
jlink-flash-bootloader-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi-development.jlink
jlink-flash-bootloader-plus-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02p-bl-multi-development.jlink
jlink-flash-bootloader-btc-plus-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02p-bl-btconly-development.jlink
jlink-flash-bootloader-development-locked: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi-development-locked.jlink
jlink-flash-bootloader: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-multi.jlink
jlink-flash-bootloader-btc-development: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-btconly-development.jlink
jlink-flash-bootloader-btc: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bb02-bl-btc.jlink
jlink-flash-firmware: | build
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | build
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
	./scripts/container.sh build --pull --force-rm --no-cache -t shiftcrypto/firmware_v2:$(shell cat .containerversion) .
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
	rm -rf build build-build build-debug build-build-noasan

# When you vendor rust libs avoid duplicates
vendor-rust-deps:
	(cd external; ./vendor-rust.sh)
