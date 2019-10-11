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

bootstrap:
	git submodule update --init --recursive
# Directory for building for "host" machine according to gcc convention
build:
	mkdir -p build
	cd build && cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake ..
	$(MAKE) -C py/bitbox02

# Directory for building for "build" machine according to gcc convention
build-build:
	mkdir -p build-build
	cd build-build && cmake .. -DCOVERAGE=ON -DSANITIZE_ADDRESS=ON -DSANITIZE_UNDEFINED=ON
	$(MAKE) -C py/bitbox02

firmware: | build
# Generate python bindings for protobuf for test scripts
	$(MAKE) -C build firmware.elf
firmware-semihosting: | build
	$(MAKE) -C build firmware-semihosting.elf
firmware-btc: | build
	$(MAKE) -C build firmware-btc.elf
firmware-bitboxbase: | build
	$(MAKE) -C build firmware-bitboxbase.elf
bootloader: | build
	$(MAKE) -C build bootloader.elf
bootloader-devdevice: | build
	$(MAKE) -C build bootloader-development.elf
bootloader-devdevice-locked: | build
	$(MAKE) -C build bootloader-development-locked.elf
bootloader-production: | build
	$(MAKE) -C build bootloader-production.elf
bootloader-btc: | build
	$(MAKE) -C build bootloader-btc.elf
bootloader-btc-devdevice: | build
	$(MAKE) -C build bootloader-btc-development.elf
bootloader-btc-production: | build
	$(MAKE) -C build bootloader-btc-production.elf
bootloader-bitboxbase: | build
	$(MAKE) -C build bootloader-bitboxbase.elf
bootloader-bitboxbase-devdevice: | build
	$(MAKE) -C build bootloader-bitboxbase-development.elf
bootloader-bitboxbase-production: | build
	$(MAKE) -C build bootloader-bitboxbase-production.elf
factory-setup: | build
	$(MAKE) -C build factory-setup.elf
docs: | build
	$(MAKE) -C build doc
unit-test: | build-build
	$(MAKE) -C build-build
device-tests: | build
	${MAKE} -C build device-tests
# Must compile C tests before running them
run-unit-tests: | build build-build
	$(MAKE) -C build-build test
	${MAKE} -C build rust-test
# Must run tests before creating coverage report
coverage: | build-build
	${MAKE} -C build-build coverage
#./build/bin/test_ui_component_gestures;
run-valgrind-on-unit-tests:
	$(MAKE) unit-test
# TODO: wrap all unit tests in one script
	bash -c 'set -e; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_hww; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_gestures; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_random; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_app_btc; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_sd; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_ui_util; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_ui_components; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_cleanup; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_keystore; \
		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_backup;'
#		valgrind --leak-check=yes --track-origins=yes ./build/bin/test_ui_component_gestures;'
flash-dev-firmware:
	./py/load_firmware.py build/bin/firmware.bin debug
flash-dev-firmware-bitboxbase:
	./py/load_firmware.py build/bin/firmware-bitboxbase.bin debug
jlink-flash-bootloader: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-development.jlink
jlink-flash-bootloader-btc: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-btc-development.jlink
jlink-flash-firmware: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
jlink-flash-firmware-btc: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-btc.jlink
jlink-flash-firmware-semihosting: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-semihosting.jlink
jlink-flash-firmware-bitboxbase: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware-bitboxbase.jlink
dockerinit:
	docker build --pull --force-rm --no-cache -t shiftcrypto/firmware_v2 .
dockerdev:
	./scripts/dockerdev.sh
dockerrun:
	docker-compose run -w /firmware_v2 firmware_v2
generate-atecc608-config:
	${MAKE} -C tools/go/src/atecc608a run
ci:
	./.ci/ci
prepare-tidy: | build build-build
	make -C build generate-protobufs
	make -C build-build generate-protobufs
	make -C build rust-bindgen
clean:
	rm -rf build build-build
