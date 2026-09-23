# SPDX-License-Identifier: Apache-2.0

# Command aliases, not a dependency graph. Keep setup explicit in recipes.
.DEFAULT_GOAL := firmware
.NOTPARALLEL:
SANITIZE ?= ON

ifneq ($(filter config,$(MAKECMDGOALS)),)
ifneq ($(words $(MAKECMDGOALS)),1)
$(error Run make config separately so subsequent commands use the saved settings)
endif
else
-include config.mk
endif
PRODUCT ?= bitbox02
BOARD ?= $(if $(filter bitbox03,$(PRODUCT)),dev-kit,$(PRODUCT))
EDITION ?= multi
PROBE_SOFTWARE ?= $(if $(filter bitbox03,$(PRODUCT)),openocd,jlink)
SWD_SPEED ?= 4000
CONFIG_OPTIONS = --product "$(PRODUCT)" --board "$(BOARD)" --edition "$(EDITION)" --probe-software "$(PROBE_SOFTWARE)" --swd-speed "$(SWD_SPEED)"

config:
	python3 scripts/build_config.py $(CONFIG_ARGS)

bootstrap:
	git submodule update --init --recursive
	./scripts/bootstrap-cargo-config

IMAGES := firmware factorysetup bootloader-stage0 bootloader-stage1
FLASH_IMAGES := $(IMAGES)
ifeq ($(PRODUCT),bitbox03)
FLASH_IMAGES := $(filter-out factorysetup,$(FLASH_IMAGES))
endif
BUILD_TARGETS := $(IMAGES) firmware-debug firmware-release factorysetup-debug factorysetup-release \
	bootloader-stage0-production bootloader-stage0-development \
	bootloader-stage1-production bootloader-stage1-development \
	bootloader-upgrade-assets bootloader-upgrade-assets-development docs rust-docs
BUILD_TARGETS += $(foreach product,bitbox02 bitbox02nova,$(foreach edition,multi btconly,\
	firmware-blupgrade-$(product)-$(edition) firmware-blupgrade-$(product)-$(edition)-development))
BUILD_TARGETS += $(foreach image,boot0 boot1 firmware factorysetup,\
	bitbox03-$(image)-debug bitbox03-$(image)-release)

$(BUILD_TARGETS):
	+python3 scripts/build_product.py $(CONFIG_OPTIONS) $@

$(addprefix flash-,$(FLASH_IMAGES)):
	python3 scripts/probe.py $(CONFIG_OPTIONS) flash $(patsubst flash-%,%,$@)
$(addprefix run-,$(IMAGES)):
	python3 scripts/probe.py $(CONFIG_OPTIONS) run $(patsubst run-%,%,$@)
debug-server:
	python3 scripts/probe.py $(CONFIG_OPTIONS) server

build-build:
	./scripts/bootstrap-cargo-config
	test -f build-build/Makefile || cmake -S . -B build-build -DSANITIZE_ADDRESS=$(SANITIZE) -DSANITIZE_UNDEFINED=$(SANITIZE)
	$(MAKE) -C py/bitbox02

# ubsan/asan not supported with simulators and rust unit tests
build-build-noasan:
	./scripts/bootstrap-cargo-config
	test -f build-build-noasan/Makefile || cmake -S . -B build-build-noasan -DSANITIZE_ADDRESS=OFF -DSANITIZE_UNDEFINED=OFF
	$(MAKE) -C py/bitbox02

simulator:
	$(MAKE) build-build-noasan
	$(MAKE) -C build-build-noasan simulator
simulator-graphical:
	$(MAKE) build-build-noasan
	$(MAKE) -C build-build-noasan simulator-graphical
simulator-graphical-bb03:
	$(MAKE) build-build-noasan
	$(MAKE) -C build-build-noasan simulator-graphical-bb03
run-simulator:
	$(MAKE) simulator
	./build-build-noasan/bin/simulator
unit-test:
	$(MAKE) build-build
	$(MAKE) -C build-build
# Must compile C tests before running them
run-unit-tests:
	$(MAKE) build-build
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
run-rust-clippy:
	$(MAKE) build-build-noasan
	${MAKE} -C build-build-noasan rust-clippy
run-valgrind-on-unit-tests:
	$(MAKE) unit-test
	bash -ec 'for exe in build-build/bin/test_*; do  valgrind --leak-check=yes --track-origins=yes --error-exitcode=1 --exit-on-first-error=yes $$exe; done'
flash-dev-firmware: FIRMWARE_BIN = build-$(PRODUCT)-cmake-relwithdebinfo/bin/firmware$(if $(filter btc-only,$(EDITION)),-btc).bin
flash-dev-firmware:
	@case "$(PRODUCT)" in bitbox02|bitbox02nova) ;; *) echo "flash-dev-firmware is only supported on BitBox02/Nova" >&2; exit 1 ;; esac
	@case "$(EDITION)" in multi|btc-only) ;; *) echo "Invalid edition: $(EDITION)" >&2; exit 1 ;; esac
	@test -f "$(FIRMWARE_BIN)" || { echo "Image not found: $(FIRMWARE_BIN). Build it first with: make firmware" >&2; exit 1; }
	python3 py/load_firmware.py "$(FIRMWARE_BIN)" --yes

# Per-product development stage0/stage1 J-Link wrappers flash already-built images.
jlink-flash-bootloader-stage0-bitbox02-btconly-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/bootloader-stage0-bitbox02-btconly-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02-btconly-development.jlink || { echo "Missing image; run: make bootloader-stage0 PRODUCT=bitbox02 BOARD=bitbox02 EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02-btconly-development.jlink
jlink-flash-bootloader-stage0-bitbox02-multi-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/bootloader-stage0-bitbox02-multi-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02-multi-development.jlink || { echo "Missing image; run: make bootloader-stage0 PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02-multi-development.jlink
jlink-flash-bootloader-stage0-bitbox02nova-btconly-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/bootloader-stage0-bitbox02nova-btconly-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02nova-btconly-development.jlink || { echo "Missing image; run: make bootloader-stage0 PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02nova-btconly-development.jlink
jlink-flash-bootloader-stage0-bitbox02nova-multi-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/bootloader-stage0-bitbox02nova-multi-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02nova-multi-development.jlink || { echo "Missing image; run: make bootloader-stage0 PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage0-bitbox02nova-multi-development.jlink
jlink-flash-bootloader-stage1-bitbox02-btconly-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/bootloader-stage1-bitbox02-btconly-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02-btconly-development.jlink || { echo "Missing image; run: make bootloader-stage1 PRODUCT=bitbox02 BOARD=bitbox02 EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02-btconly-development.jlink
jlink-flash-bootloader-stage1-bitbox02-multi-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/bootloader-stage1-bitbox02-multi-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02-multi-development.jlink || { echo "Missing image; run: make bootloader-stage1 PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02-multi-development.jlink
jlink-flash-bootloader-stage1-bitbox02nova-btconly-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/bootloader-stage1-bitbox02nova-btconly-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02nova-btconly-development.jlink || { echo "Missing image; run: make bootloader-stage1 PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02nova-btconly-development.jlink
jlink-flash-bootloader-stage1-bitbox02nova-multi-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/bootloader-stage1-bitbox02nova-multi-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02nova-multi-development.jlink || { echo "Missing image; run: make bootloader-stage1 PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/bootloader-stage1-bitbox02nova-multi-development.jlink

jlink-flash-firmware:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/firmware.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware.jlink || { echo "Missing image; run: make firmware PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware.jlink
jlink-flash-firmware-btc:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/firmware-btc.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-btc.jlink || { echo "Missing image; run: make firmware PRODUCT=bitbox02 BOARD=bitbox02 EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-btc.jlink
jlink-flash-factory-setup:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/factory-setup.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/factory-setup.jlink || { echo "Missing image; run: make factorysetup PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/factory-setup.jlink
jlink-flash-firmware-debug:
	@test -f ./build-bitbox02-cmake-debug/bin/firmware.elf && test -f ./build-bitbox02-cmake-debug/scripts/firmware.jlink || { echo "Missing image; run: make firmware-debug PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-debug/scripts/firmware.jlink

jlink-flash-firmware-blupgrade-bitbox02-btconly-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/firmware-blupgrade-bitbox02-btconly-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02-btconly-development.jlink || { echo "Missing image; run: make firmware-blupgrade-bitbox02-btconly-development PRODUCT=bitbox02 BOARD=bitbox02 EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02-btconly-development.jlink
jlink-flash-firmware-blupgrade-bitbox02-multi-development:
	@test -f ./build-bitbox02-cmake-relwithdebinfo/bin/firmware-blupgrade-bitbox02-multi-development.elf && test -f ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02-multi-development.jlink || { echo "Missing image; run: make firmware-blupgrade-bitbox02-multi-development PRODUCT=bitbox02 BOARD=bitbox02 EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02-multi-development.jlink
jlink-flash-firmware-blupgrade-bitbox02nova-btconly-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/firmware-blupgrade-bitbox02nova-btconly-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02nova-btconly-development.jlink || { echo "Missing image; run: make firmware-blupgrade-bitbox02nova-btconly-development PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=btc-only" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02nova-btconly-development.jlink
jlink-flash-firmware-blupgrade-bitbox02nova-multi-development:
	@test -f ./build-bitbox02nova-cmake-relwithdebinfo/bin/firmware-blupgrade-bitbox02nova-multi-development.elf && test -f ./build-bitbox02nova-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02nova-multi-development.jlink || { echo "Missing image; run: make firmware-blupgrade-bitbox02nova-multi-development PRODUCT=bitbox02nova BOARD=bitbox02nova EDITION=multi" >&2; exit 1; }
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./build-bitbox02nova-cmake-relwithdebinfo/scripts/firmware-blupgrade-bitbox02nova-multi-development.jlink

jlink-flash-set-new-screen:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/set-new-screen.jlink
jlink-flash-set-original-screen:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/set-original-screen.jlink
jlink-flash-reset-version:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/reset-version.jlink
jlink-flash-set-securechip-optiga:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/set-securechip-optiga.jlink
jlink-flash-set-bb02plus:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/set-bb02plus.jlink
jlink-flash-bb02-set-factory-randomness:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/bb02-set-factory-randomness.jlink
jlink-erase-firmware-quick:
	JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -autoconnect 1 -CommanderScript ./scripts/erase-firmware-quick.jlink
jlink-gdb-server:
	JLinkGDBServer -nogui -if SWD -device ATSAMD51J20 -speed "$(SWD_SPEED)" -port 2331 -RTTTelnetPort 19021 -vd
rtt-client:
	telnet localhost 19021
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
prepare-tidy:
	+python3 scripts/build_product.py $(CONFIG_OPTIONS) prepare-tidy
	$(MAKE) build-build
	$(MAKE) -C build-build rust-cbindgen
clean:
	rm -rf build build-debug build-build build-build-noasan build-bitbox02-cmake-* build-bitbox02nova-cmake-* build-bitbox03-cargo src/rust/target

# When you vendor rust libs avoid duplicates
vendor-rust-deps:
	./external/vendor-rust.sh

# Mark all command aliases phony, including generated image/variant names.
.PHONY: $(BUILD_TARGETS) $(addprefix flash-,$(FLASH_IMAGES)) $(addprefix run-,$(IMAGES))
.PHONY: config bootstrap debug-server \
	build-build build-build-noasan simulator simulator-graphical simulator-graphical-bb03 \
	run-simulator unit-test run-unit-tests run-rust-unit-tests run-rust-clippy \
	run-valgrind-on-unit-tests flash-dev-firmware \
	jlink-flash-bootloader-stage0-bitbox02-btconly-development \
	jlink-flash-bootloader-stage0-bitbox02-multi-development \
	jlink-flash-bootloader-stage0-bitbox02nova-btconly-development \
	jlink-flash-bootloader-stage0-bitbox02nova-multi-development \
	jlink-flash-bootloader-stage1-bitbox02-btconly-development \
	jlink-flash-bootloader-stage1-bitbox02-multi-development \
	jlink-flash-bootloader-stage1-bitbox02nova-btconly-development \
	jlink-flash-bootloader-stage1-bitbox02nova-multi-development jlink-flash-firmware \
	jlink-flash-firmware-btc jlink-flash-factory-setup jlink-flash-firmware-debug \
	jlink-flash-firmware-blupgrade-bitbox02-btconly-development \
	jlink-flash-firmware-blupgrade-bitbox02-multi-development \
	jlink-flash-firmware-blupgrade-bitbox02nova-btconly-development \
	jlink-flash-firmware-blupgrade-bitbox02nova-multi-development jlink-flash-set-new-screen \
	jlink-flash-set-original-screen jlink-flash-reset-version jlink-flash-set-securechip-optiga \
	jlink-flash-set-bb02plus jlink-flash-bb02-set-factory-randomness jlink-erase-firmware-quick \
	jlink-gdb-server rtt-client dockerinit dockerpull dockerdev dockerrel generate-protobufs \
	generate-atecc608-config ci prepare-tidy clean vendor-rust-deps
