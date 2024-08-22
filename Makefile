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

.DEFAULT_GOAL := firmware
SANITIZE ?= ON
simulator: SANITIZE = OFF

bootstrap:
	git submodule update --init --recursive

build/Makefile:
	mkdir -p build
	cd build && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake ..
	$(MAKE) -C py/bitbox02

build-build/Makefile:
	mkdir -p build-build
	cd build-build && cmake .. -DCOVERAGE=ON -DSANITIZE_ADDRESS=$(SANITIZE) -DSANITIZE_UNDEFINED=$(SANITIZE)
	$(MAKE) -C py/bitbox02

# Should only be used for rust unit tests since we didn't add support to
# address santizers when they link code compiled with gcc.
build-build-rust-unit-tests/Makefile:
	mkdir -p build-build-rust-unit-tests
	cd build-build-rust-unit-tests && cmake .. -DCOVERAGE=OFF -DSANITIZE_ADDRESS=OFF -DSANITIZE_UNDEFINED=OFF
	$(MAKE) -C py/bitbox02

build-semihosting/Makefile:
	mkdir -p build-semihosting
	cd build-semihosting && cmake .. -DCMAKE_TOOLCHAIN_FILE=arm.cmake -DSEMIHOSTING=ON
	${MAKE} -C py/bitbox02

# Directory for building for "host" machine according to gcc convention
build: build/Makefile

# Directory for building for "build" machine according to gcc convention
build-build: build-build/Makefile

# Directory for building for "build" machine according to gcc convention
# Should only be used for rust unit tests since we didn't add support to
# address santizers when they link code compiled with gcc.
build-build-rust-unit-tests: build-build-rust-unit-tests/Makefile

# Directory for building for "host" machine but with semihosting enbled
build-semihosting: build-semihosting/Makefile

firmware: | build
# Generate python bindings for protobuf for test scripts
	$(MAKE) -C build firmware.elf
firmware-semihosting: | build-semihosting
	$(MAKE) -C build-semihosting firmware.elf
firmware-btc: | build
	$(MAKE) -C build firmware-btc.elf
bootloader: | build
	$(MAKE) -C build bootloader.elf
bootloader-semihosting: | build-semihosting
	$(MAKE) -C build-semihosting bootloader-development.elf
bootloader-development: | build
	$(MAKE) -C build bootloader-development.elf
bootloader-development-locked: | build
	$(MAKE) -C build bootloader-development-locked.elf
bootloader-production: | build
	$(MAKE) -C build bootloader-production.elf
bootloader-btc: | build
	$(MAKE) -C build bootloader-btc.elf
bootloader-btc-development: | build
	$(MAKE) -C build bootloader-btc-development.elf
bootloader-btc-production: | build
	$(MAKE) -C build bootloader-btc-production.elf
factory-setup: | build
	$(MAKE) -C build factory-setup.elf
docs: | build
	$(MAKE) -C build doc
rust-docs: | build
	$(MAKE) -C build rust-docs
simulator: | build-build
	$(MAKE) -C build-build simulator
run-simulator: | simulator
	./build-build/bin/simulator
unit-test: | build-build
	$(MAKE) -C build-build
# Must compile C tests before running them
run-unit-tests: | build-build
	CTEST_OUTPUT_ON_FAILURE=1 $(MAKE) -C build-build test
run-rust-unit-tests: | build-build-rust-unit-tests
	${MAKE} -C build-build-rust-unit-tests rust-test
run-rust-clippy: | build-build-rust-unit-tests
	${MAKE} -C build-build-rust-unit-tests rust-clippy
# Must run tests before creating coverage report
coverage: | build-build
	${MAKE} -C build-build coverage
#./build/bin/test_ui_component_gestures;
run-valgrind-on-unit-tests:
	$(MAKE) unit-test
	bash -c 'find build-build/bin/ -name "test_*" -exec valgrind --leak-check=yes --track-origins=yes {} \;'
flash-dev-firmware:
	./py/load_firmware.py build/bin/firmware.bin --debug
jlink-flash-bootloader-development: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-development.jlink
jlink-flash-bootloader-semihosting: | build-semihosting
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-semihosting/scripts/bootloader-development.jlink
jlink-flash-bootloader-development-locked: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-development-locked.jlink
jlink-flash-bootloader: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader.jlink
jlink-flash-bootloader-btc-development: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-btc-development.jlink
jlink-flash-bootloader-btc: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-btc.jlink
jlink-flash-firmware: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-btc.jlink
jlink-flash-factory-setup: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/factory-setup.jlink
jlink-flash-firmware-semihosting: | build-semihosting
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build-semihosting/scripts/firmware.jlink
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
	make -C build rust-cbindgen
	make -C build-build rust-cbindgen
clean:
	rm -rf build build-build build-semihosting build-build-rust-unit-tests
