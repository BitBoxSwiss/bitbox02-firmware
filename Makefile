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

# Directory for building for "build" machine according to gcc convention
build-build:
	mkdir -p build-build
	cd build-build && cmake .. -DCOVERAGE=ON -DSANITIZE_ADDRESS=ON -DSANITIZE_UNDEFINED=ON

firmware: | build
# Generate python bindings for protobuf for test scripts
	$(MAKE) -C py
	$(MAKE) -C build firmware.elf
bootloader: | build
	$(MAKE) -C build bootloader.elf
bootloader-devdevice: | build
	$(MAKE) -C build bootloader-development.elf
bootloader-production: | build
	$(MAKE) -C build bootloader-production.elf
factory-setup: | build
	$(MAKE) -C build factory-setup.elf
docs: | build-build
	$(MAKE) -C build-build doc
unit-test: | build-build
	$(MAKE) -C build-build
device-test:
	mkdir -p build; cd build; cmake -DCMAKE_TOOLCHAIN_FILE=arm.cmake .. -DBUILD_TYPE=device-test -DMAIN-SOURCE:STRING=src/test_button_tap.c && $(MAKE)
# Must compile tests before running them
run-unit-tests: | build-build
	$(MAKE) -C build-build test
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
jlink-flash-bootloader: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/bootloader-development.jlink
jlink-flash-firmware: | build
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./build/scripts/firmware.jlink
dockerinit:
	docker build --pull --force-rm -t shiftcrypto/firmware_v2 .
dockerdev:
	./scripts/dockerdev.sh
dockerrun:
	docker-compose run -w /firmware_v2 firmware_v2
generate-atecc608-config:
	${MAKE} -C tools/go/src/atecc608a run
ci:
	./.ci/ci
clean:
	rm -rf build build-build
