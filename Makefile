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
DEBUG ?= no
PROBE_SOFTWARE ?= $(if $(filter bitbox03,$(PRODUCT)),openocd,jlink)
SWD_SPEED ?= 4000
CONFIG_OPTIONS = --product "$(PRODUCT)" --board "$(BOARD)" --edition "$(EDITION)" --debug "$(DEBUG)" --probe-software "$(PROBE_SOFTWARE)" --swd-speed "$(SWD_SPEED)"

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
BUILD_TARGETS := $(IMAGES) \
	bootloader-stage0-production bootloader-stage0-development \
	bootloader-stage1-production bootloader-stage1-development \
	bootloader-upgrade-assets bootloader-upgrade-assets-development docs rust-docs
BUILD_TARGETS += $(foreach product,bitbox02 bitbox02nova,$(foreach edition,multi btconly,\
	firmware-blupgrade-$(product)-$(edition) firmware-blupgrade-$(product)-$(edition)-development))

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
flash-dev-firmware: FIRMWARE_BIN = build-$(PRODUCT)-cmake-$(if $(filter yes,$(DEBUG)),debug,relwithdebinfo)/bin/firmware$(if $(filter btc-only,$(EDITION)),-btc).bin
flash-dev-firmware:
	@case "$(PRODUCT)" in bitbox02|bitbox02nova) ;; *) echo "flash-dev-firmware is only supported on BitBox02/Nova" >&2; exit 1 ;; esac
	@case "$(EDITION)" in multi|btc-only) ;; *) echo "Invalid edition: $(EDITION)" >&2; exit 1 ;; esac
	@case "$(DEBUG)" in yes|no) ;; *) echo "Invalid debug setting: $(DEBUG)" >&2; exit 1 ;; esac
	@test -f "$(FIRMWARE_BIN)" || { echo "Image not found: $(FIRMWARE_BIN). Build it first with: make firmware DEBUG=$(DEBUG)" >&2; exit 1; }
	python3 py/load_firmware.py "$(FIRMWARE_BIN)" --yes

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
	rtt-client dockerinit dockerpull dockerdev dockerrel generate-protobufs \
	generate-atecc608-config ci prepare-tidy clean vendor-rust-deps
