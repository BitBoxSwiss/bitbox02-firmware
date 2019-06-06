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


firmware:
	$(MAKE) -C py/bitbox02
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=firmware && $(MAKE)
bootloader:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=bootloader && $(MAKE)
bootloader-devdevice:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=bootloader -DBOOTLOADER_DEVDEVICE=ON && $(MAKE)
bootloader-production:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=bootloader -DBOOTLOADER_PRODUCTION=ON && $(MAKE)
factory-setup:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=factory-setup && $(MAKE)
docs:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=firmware -DBUILD_DOCUMENTATION=ON && $(MAKE) doc
unit-test:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=unit-test && $(MAKE)
device-test:
	mkdir -p build; cd build; cmake .. -DBUILD_TYPE=device-test -DMAIN-SOURCE:STRING=src/test_button_tap.c && $(MAKE)
run-unit-tests:
	$(MAKE) unit-test
# TODO: wrap all unit tests in one script
	cd build; $(MAKE) test;
	cd build; lcov --quiet --capture --directory . --output-file raw_coverage.info; lcov --remove raw_coverage.info --output-file coverage.info '*/test/*' '*/external/*' '*/src/drivers/*'; genhtml --quiet coverage.info --output-directory coverage_html
	echo "coverage report in build/coverage_html/index.html"
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
jlink-flash-bootloader:
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/bootloader.jlink
jlink-flash-firmware:
	JLinkExe -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -CommanderScript ./scripts/firmware.jlink
dockerinit:
	docker build --pull --force-rm  -t shiftcrypto/firmware_v2 .
dockerdev:
	./scripts/dockerdev.sh
dockerrun:
	docker-compose run -w /firmware_v2 firmware_v2
generate-atecc608-config:
	cd tools/go/src/atecc608a/ && $(MAKE) run
clean:
	rm -rf build/*
