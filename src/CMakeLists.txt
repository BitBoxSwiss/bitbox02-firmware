# SPDX-License-Identifier: Apache-2.0

# Copyright (c) 2015-2016 Lucas Betschart, Douglas J. Bakkum

set(DBB-FIRMWARE-SOURCES
  ${CMAKE_SOURCE_DIR}/src/delay.c
  ${CMAKE_SOURCE_DIR}/src/random.c
  ${CMAKE_SOURCE_DIR}/src/hardfault.c
  ${CMAKE_SOURCE_DIR}/src/util.c
  ${CMAKE_SOURCE_DIR}/src/sd.c
  ${CMAKE_SOURCE_DIR}/src/system.c
  ${CMAKE_SOURCE_DIR}/src/hww.c
  ${CMAKE_SOURCE_DIR}/src/memory/bitbox02_smarteeprom.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_shared.c
  ${CMAKE_SOURCE_DIR}/src/memory/mpu.c
  ${CMAKE_SOURCE_DIR}/src/memory/nvmctrl.c
  ${CMAKE_SOURCE_DIR}/src/memory/spi_mem.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_spi.c
  ${CMAKE_SOURCE_DIR}/src/memory/smarteeprom.c
  ${CMAKE_SOURCE_DIR}/src/salt.c
  ${CMAKE_SOURCE_DIR}/src/i2c_ecc.c
  ${CMAKE_SOURCE_DIR}/src/touch/gestures.c
  ${CMAKE_SOURCE_DIR}/src/reset.c
  ${CMAKE_SOURCE_DIR}/src/queue.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_processing.c
)
set(DBB-FIRMWARE-SOURCES ${DBB-FIRMWARE-SOURCES} PARENT_SCOPE)

set(DBB-FIRMWARE-USB-SOURCES
  ${CMAKE_SOURCE_DIR}/src/usb/usb.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_frame.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_packet.c
  ${CMAKE_SOURCE_DIR}/src/u2f/u2f_packet.c
)
set(DBB-FIRMWARE-USB-SOURCES ${DBB-FIRMWARE-USB-SOURCES} PARENT_SCOPE)

set(DBB-FIRMWARE-UI-SOURCES
    ${CMAKE_SOURCE_DIR}/src/screen.c
    ${CMAKE_SOURCE_DIR}/src/ui/graphics/graphics.c
    ${CMAKE_SOURCE_DIR}/src/ui/ugui/ugui.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_9X9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_11X10.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_11X12.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_13X14.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_15X16.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_17X18.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/monogram_5X9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/password_9X9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/password_11X12.c
    ${CMAKE_SOURCE_DIR}/src/ui/screen_saver.c
    ${CMAKE_SOURCE_DIR}/src/ui/screen_stack.c
    ${CMAKE_SOURCE_DIR}/src/ui/screen_process.c
    ${CMAKE_SOURCE_DIR}/src/ui/event_handler.c
    ${CMAKE_SOURCE_DIR}/src/ui/ui_util.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_choice.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_input_char.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_input_string.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/waiting.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/screensaver.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/knight_rider.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/right_arrow.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/left_arrow.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/icon_button.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_gesture.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/label.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/keyboard_switch.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/orientation_arrows.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/info_centered.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/lockscreen.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/unlock_animation.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/menu.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/status.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/image.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/button.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/empty.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/progress.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/sdcard.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/ui_images.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_transaction.c
)
set(DBB-FIRMWARE-UI-SOURCES ${DBB-FIRMWARE-UI-SOURCES} PARENT_SCOPE)

set(DBB-BOOTLOADER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/util.c
  ${CMAKE_SOURCE_DIR}/src/pukcc/curve_p256.c
  ${CMAKE_SOURCE_DIR}/src/pukcc/pukcc.c
  ${CMAKE_SOURCE_DIR}/src/bootloader/bootloader.c
  ${CMAKE_SOURCE_DIR}/src/bootloader/startup.c
  ${CMAKE_SOURCE_DIR}/src/bootloader/mpu_regions.c
  ${CMAKE_SOURCE_DIR}/src/random.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_shared.c
  ${CMAKE_SOURCE_DIR}/src/memory/mpu.c
  ${CMAKE_SOURCE_DIR}/src/memory/nvmctrl.c
  ${CMAKE_SOURCE_DIR}/src/memory/spi_mem.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_spi.c
  ${CMAKE_SOURCE_DIR}/src/queue.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_processing.c
  ${CMAKE_SOURCE_DIR}/src/ui/ugui/ugui.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_9X9.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_11X10.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/monogram_5X9.c
  ${CMAKE_SOURCE_DIR}/src/ui/graphics/graphics.c
  ${CMAKE_SOURCE_DIR}/src/screen.c
  ${CMAKE_SOURCE_DIR}/src/hardfault.c
  ${CMAKE_SOURCE_DIR}/src/ui/components/ui_images.c
)
set(DBB-BOOTLOADER-SOURCES ${DBB-BOOTLOADER-SOURCES} PARENT_SCOPE)

set(DRIVER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/platform/platform_init.c
  ${CMAKE_SOURCE_DIR}/src/platform/driver_init.c
  ${CMAKE_SOURCE_DIR}/src/ui/oled/oled.c
  ${CMAKE_SOURCE_DIR}/src/ui/oled/oled_writer.c
)
set(DRIVER-SOURCES ${DRIVER-SOURCES} PARENT_SCOPE)

set(QTOUCH-SOURCES
  ${CMAKE_SOURCE_DIR}/src/qtouch/qtouch.c
)
set(QTOUCH-SOURCES ${QTOUCH-SOURCES} PARENT_SCOPE)

# The additional files required for the plus platform
set(PLATFORM-BITBOX02-PLUS-SOURCES
  ${CMAKE_SOURCE_DIR}/src/da14531/da14531.c
  ${CMAKE_SOURCE_DIR}/src/da14531/da14531_protocol.c
  ${CMAKE_SOURCE_DIR}/src/da14531/da14531_handler.c
  ${CMAKE_SOURCE_DIR}/src/uart.c
)
set(PLATFORM-BITBOX02-PLUS-SOURCES ${PLATFORM-BITBOX02-PLUS-SOURCES} PARENT_SCOPE)

set(PLATFORM-BITBOX02-SOURCES
  ${CMAKE_SOURCE_DIR}/src/sd_mmc/sd_mmc_start.c
  ${CMAKE_SOURCE_DIR}/src/sd_mmc/sd_mmc_ext.c
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hid.c
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hww/hid_hww.c
  ${CMAKE_SOURCE_DIR}/src/ui/oled/sh1107.c
  ${CMAKE_SOURCE_DIR}/src/ui/oled/ssd1312.c
  ${DBB-FIRMWARE-USB-SOURCES}
)
set(PLATFORM-BITBOX02-SOURCES ${PLATFORM-BITBOX02-SOURCES} PARENT_SCOPE)

set(SECURECHIP-SOURCES
  ${CMAKE_SOURCE_DIR}/src/atecc/atecc.c
  ${CMAKE_SOURCE_DIR}/src/securechip/securechip.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_gpio.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_i2c.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_ifx_i2c_config.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_logger.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_os_datastore.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_os_event.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_os_lock.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_os_timer.c
  ${CMAKE_SOURCE_DIR}/src/optiga/pal/pal_os_memory.c
  ${CMAKE_SOURCE_DIR}/src/optiga/optiga_ops.c
  ${CMAKE_SOURCE_DIR}/src/optiga/optiga.c
)
set(SECURECHIP-SOURCES ${SECURECHIP-SOURCES} PARENT_SCOPE)

set(FIRMWARE-U2F-DRIVER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/u2f/hid_u2f.c
)
set(FIRMWARE-U2F-DRIVER-SOURCES ${FIRMWARE-U2F-DRIVER-SOURCES} PARENT_SCOPE)

set(FIRMWARE-U2F-SOURCES
  ${CMAKE_SOURCE_DIR}/src/u2f.c
  ${CMAKE_SOURCE_DIR}/src/u2f/u2f_app.c
)
set(FIRMWARE-U2F-SOURCES ${FIRMWARE-U2F-SOURCES} PARENT_SCOPE)

#-----------------------------------------------------------------------------
# Include directories

set(INCLUDES
  ${CMAKE_SOURCE_DIR}/src
  ${CMAKE_SOURCE_DIR}/src/ui/ugui
  ${CMAKE_SOURCE_DIR}/src/platform
  ${CMAKE_SOURCE_DIR}/src/qtouch
  ${CMAKE_SOURCE_DIR}/src/usb/class
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hww
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/u2f
)
set(INCLUDES ${INCLUDES} PARENT_SCOPE)

#-----------------------------------------------------------------------------
# Build embedded firmware

set(FIRMWARE-SOURCES
  ${DBB-FIRMWARE-SOURCES}
  ${DBB-FIRMWARE-UI-SOURCES}
  ${DRIVER-SOURCES}
  ${QTOUCH-SOURCES}
  ${SECURECHIP-SOURCES}
  ${PLATFORM-BITBOX02-PLUS-SOURCES}
  ${CMAKE_SOURCE_DIR}/src/common_main.c
)

set(BOOTLOADER-SOURCES
  ${DBB-BOOTLOADER-SOURCES}
  ${DRIVER-SOURCES}
)

# cbindgen is used to generate the c-headers from rust code.
find_program(CBINDGEN cbindgen)
# cargo is the rust build system and dependency manager
find_program(CARGO cargo)

if(CMAKE_BUILD_TYPE STREQUAL "DEBUG")
  set(RUST_PROFILE "debug")
else()
  set(RUST_PROFILE "release")
  set(RUST_CARGO_FLAGS "--release")
  # For binary reproducibility, strip path prefixes that can be different depending on environment (e.g. /home/<user>, etc.).
  string(JOIN " " RUSTFLAGS "${RUSTFLAGS}" "--remap-path-prefix=${CMAKE_CURRENT_SOURCE_DIR}/rust=src" "--remap-path-prefix=$ENV{HOME}=")
endif()

if(CMAKE_CROSSCOMPILING)
  set(RUST_TARGET_ARCH thumbv7em-none-eabi)
  set(RUST_TARGET_ARCH_DIR ${RUST_TARGET_ARCH})
  set(RUST_TARGET_ARCH_ARG --target ${RUST_TARGET_ARCH})
  set(RUST_CARGO_FLAGS ${RUST_CARGO_FLAGS} -Zbuild-std=core,alloc -Zbuild-std-features=optimize_for_size)
  string(JOIN " " RUSTFLAGS "${RUSTFLAGS}" "-Zunstable-options -Cpanic=immediate-abort")
else()
  set(RUST_TARGET_ARCH_DIR .)
endif()

set(RUST_BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR}/rust)
set(LIBBITBOX02_RUST_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust/bitbox02-rust-c)
set(LIBBITBOX02_RUST ${LIBBITBOX02_RUST} PARENT_SCOPE)

# Generate c-headers for the rust library
# Working dir must be set to the rust workspace so that cargo finds the
# configuration for vendoring sources
add_custom_target(rust-cbindgen
  # cbindgen can automatically create the metadata, but it does so without the `--offline` flag.
  # The workaround is to manually create and pass the metadata.
  COMMAND
    ${CARGO}
      metadata
      --offline
      --manifest-path ${LIBBITBOX02_RUST_SOURCE_DIR}/Cargo.toml
      > ${CMAKE_CURRENT_BINARY_DIR}/rust-metadata
  COMMAND
    ${CBINDGEN}
      --quiet
      --config ${CMAKE_CURRENT_SOURCE_DIR}/rust/bitbox02-cbindgen.toml
      --output ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
      --profile release
      --metadata ${CMAKE_CURRENT_BINARY_DIR}/rust-metadata
      ${LIBBITBOX02_RUST_SOURCE_DIR}
  WORKING_DIRECTORY
    ${CMAKE_CURRENT_SOURCE_DIR}/rust
  BYPRODUCTS
    ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
  COMMENT
    "Generating ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h"
)

# Test rust crates that contain business logic. Avoid testing crates that depend on hardware.
if(NOT CMAKE_CROSSCOMPILING)

  # Since we build with all features we need to use a separate build directory.
  # Otherwise we invalidate the result from the normal compilation that uses a
  # different set of features.
  add_custom_target(rust-test
    COMMAND
      ${CMAKE_COMMAND} -E env
        CMAKE_SYSROOT=${CMAKE_SYSROOT}
        CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
        FIRMWARE_VERSION_SHORT=${FIRMWARE_VERSION}
        # only one test thread because of unsafe concurrent access to `SafeData`, `mock_sd()` and `mock_memory()`. Using mutexes instead leads to mutex poisoning and very messy output in case of a unit test failure.
        ${CARGO}
          test
          $<$<BOOL:${CMAKE_VERBOSE_MAKEFILE}>:-v>
          --all-features
          --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/rust/Cargo.toml
          --target-dir ${RUST_BINARY_DIR}/all-features
          --
          --nocapture
          --test-threads 1
    WORKING_DIRECTORY
      ${CMAKE_CURRENT_SOURCE_DIR}/rust
    DEPENDS
      ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
    )
  add_dependencies(rust-test generate-protobufs)

  add_custom_target(rust-clippy
    COMMAND
      ${CMAKE_COMMAND} -E env
        CMAKE_SYSROOT=${CMAKE_SYSROOT}
        CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
        FIRMWARE_VERSION_SHORT=${FIRMWARE_VERSION}
        ${CARGO} clippy
          $<$<BOOL:${CMAKE_VERBOSE_MAKEFILE}>:-v>
          --all-features
          --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/rust/Cargo.toml
          --target-dir ${RUST_BINARY_DIR}/clippy
          --release
          --tests
          -- # disabled linters:
            -A clippy::large_enum_variant
            -A clippy::identity_op
            -A clippy::new_without_default
            -A clippy::single_match
            -A clippy::redundant_clone
            -A clippy::type_complexity
            -A clippy::len_without_is_empty
            -A clippy::result_unit_err
            -A clippy::enum_variant_names
            -A clippy::derive_partial_eq_without_eq
            -A clippy::forget_non_drop
            -A clippy::collapsible_if
    WORKING_DIRECTORY
      ${CMAKE_CURRENT_SOURCE_DIR}/rust
    )
  add_dependencies(rust-clippy rust-cbindgen generate-protobufs)
endif()

# If a bootloader that locks the bootloader is flashed the bootloader area is permanently read-only.
set(BOOTLOADERS-BITBOX02
  bb02-bl-multi # Runs signed firmware
  bb02-bl-multi-development # Runs signed/unsigned firmware and has bootloader menu
  bb02-bl-multi-development-locked # Runs signed/unsigned firmware, has bootloader menu and bootloader is locked
  bb02-bl-multi-production # Runs signed firmware and bootloader is locked
  bb02-bl-btconly # Runs signed (btc-only keys) firmware
  bb02-bl-btconly-development # Runs signed/unsigned firmware and has bootloader menu
  bb02-bl-btconly-production # Runs signed (btc-only keys) firmware and bootloader is locked
  )
set(BOOTLOADERS-BITBOX02PLUS
  bb02p-bl-multi # Runs signed firmware
  bb02p-bl-multi-development # Runs signed/unsigned firmware and has bootloader menu
  bb02p-bl-multi-development-locked # Runs signed/unsigned firmware, has bootloader menu and bootloader is locked
  bb02p-bl-multi-production # Runs signed firmware and bootloader is locked
  bb02p-bl-btconly # Runs signed (btc-only keys) firmware
  bb02p-bl-btconly-development # Runs signed/unsigned firmware and has bootloader menu
  bb02p-bl-btconly-production # Runs signed (btc-only keys) firmware and bootloader is locked
)
set(BOOTLOADERS
  ${BOOTLOADERS-BITBOX02}
  ${BOOTLOADERS-BITBOX02PLUS}
  )
set(BOOTLOADERS ${BOOTLOADERS} PARENT_SCOPE)

# Used to add QTouch to development bootloaders of bb02 bootloaders
set(DEVDEVICE-BOOTLOADERS
  bb02-bl-multi-development
  bb02-bl-multi-development-locked
  bb02-bl-btconly-development
)

set(FIRMWARES
  firmware # Firmware MULTI
  firmware-btc # Firmware BTC-ONLY
  factory-setup # Factory-setup
  )
set(FIRMWARES ${FIRMWARES} PARENT_SCOPE)

if(CMAKE_CROSSCOMPILING)
  set(RUST_LIBS ${FIRMWARES} ${BOOTLOADERS})
else()
  set(RUST_LIBS c-unit-tests)
endif()

#  Create static libraries from rust code. Either:
#  *  c-unit-tests_rust_c
#  or
#  *  firmware_rust_c
#  *  firmware-btc_rust_c
#  *  factory-setup_rust_c
foreach(type ${RUST_LIBS})
  # We build the rust standard libs so that everything from the rust runtime
  # is included (like eh_personality).
  if(type STREQUAL "c-unit-tests")
      set(RUST_CARGO_FLAGS ${RUST_CARGO_FLAGS} -Zbuild-std=std,panic_abort)
  endif()
  set(lib ${RUST_BINARY_DIR}/feature-${type}/${RUST_TARGET_ARCH_DIR}/${RUST_PROFILE}/libbitbox02_rust_c.a)
  # The dummy output is to always trigger rebuild (cargo is clever enough to
  # only rebuild if something changed)
  add_custom_command(
    OUTPUT ${lib} ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a dummy
    COMMAND
      ${CMAKE_COMMAND} -E env
      CMAKE_SYSROOT=${CMAKE_SYSROOT}
      CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
      RUSTFLAGS=${RUSTFLAGS}
      CFLAGS=${CARGO_C_FLAGS}
      FIRMWARE_VERSION_SHORT=${FIRMWARE_VERSION}
      $<$<BOOL:${SCCACHE_PROGRAM}>:RUSTC_WRAPPER=${SCCACHE_PROGRAM}>
      RUSTC_BOOTSTRAP=1
      MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}
      LIB_TYPE=${type}
      ${CARGO}
        rustc
        $<$<BOOL:${CMAKE_VERBOSE_MAKEFILE}>:-vv>
        --offline
        $ENV{CARGOFLAGS}
        --features target-${type}$<$<OR:$<STREQUAL:${CMAKE_BUILD_TYPE},DEBUG>,$<STREQUAL:${type},factory-setup>>:,rtt>
        --manifest-path ${LIBBITBOX02_RUST_SOURCE_DIR}/Cargo.toml
        --target-dir ${RUST_BINARY_DIR}/feature-${type}
        ${RUST_CARGO_FLAGS}
        ${RUST_TARGET_ARCH_ARG}
        --crate-type=staticlib
    COMMAND
      ${CMAKE_COMMAND} -E copy_if_different ${lib} ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a
    # DEPFILES are only supported with the Ninja build tool
    #DEPFILE ${RUST_BINARY_DIR}/feature-${type}/${RUST_TARGET_ARCH_DIR}/${RUST_PROFILE}/libbitbox02_rust_c.d
    WORKING_DIRECTORY
      ${CMAKE_CURRENT_SOURCE_DIR}/rust
    COMMENT "Building Rust library lib${type}_rust_c.a"
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
  )
  add_custom_target(${type}-rust-target DEPENDS ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a)
  add_library(${type}_rust_c STATIC IMPORTED GLOBAL)
  add_dependencies(${type}_rust_c ${type}-rust-target generate-protobufs)
  set_property(TARGET ${type}_rust_c PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a)
  # Add the include directory to find rust/rust.h
  target_include_directories(${type}_rust_c INTERFACE ${CMAKE_CURRENT_BINARY_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/rust/fatfs-sys/depend/fatfs/source)
  if (NOT CMAKE_CROSSCOMPILING)
    # Needs to match the definitions in `bitbox02-sys/build.rs`
    target_compile_definitions(${type}_rust_c INTERFACE
        TESTING
        _UNIT_TEST_
        PRODUCT_BITBOX_MULTI=1
        APP_BTC=1
        APP_LTC=1
        APP_U2F=1
        APP_ETH=1
    )
    # For tests all C-files are built into the rust lib.
    target_include_directories(${type}_rust_c INTERFACE
        ${INCLUDES}
        ${CMAKE_CURRENT_SOURCE_DIR}/../test/hardware-fakes/include
    )
  endif()
endforeach()


if(CMAKE_CROSSCOMPILING)
  add_custom_target(rust-docs
    COMMAND
      CMAKE_SYSROOT=${CMAKE_SYSROOT}
      ${CMAKE_COMMAND} -E env
      FIRMWARE_VERSION_SHORT=${FIRMWARE_VERSION}
      ${CARGO} doc --document-private-items --target-dir ${CMAKE_BINARY_DIR}/docs-rust --target thumbv7em-none-eabi
    COMMAND
      ${CMAKE_COMMAND} -E echo "See docs at file://${CMAKE_BINARY_DIR}/docs-rust/thumbv7em-none-eabi/doc/bitbox02_rust/index.html"
    WORKING_DIRECTORY
      ${CMAKE_CURRENT_SOURCE_DIR}/rust
  )

  set(STACK_SIZE "0x10000" CACHE STRING "Specify stack size for bootloader/firmware")
  set(STACK_SIZE ${STACK_SIZE} PARENT_SCOPE)
  set(HEAP_SIZE "0x18000" CACHE STRING "Specify heap size for bootloader/firmware")
  set(HEAP_SIZE ${HEAP_SIZE} PARENT_SCOPE)

  foreach(bootloader ${BOOTLOADERS})
    set(elf ${bootloader}.elf)
    add_executable(${elf} ${BOOTLOADER-SOURCES} ${PLATFORM-BITBOX02-SOURCES})
    target_link_libraries(${elf} PRIVATE c asf4-drivers-min samd51a-ds -Wl,-u,exception_table)
    target_include_directories(${elf} PRIVATE ${INCLUDES})
    target_compile_definitions(${elf} PRIVATE BOOTLOADER "APP_BTC=0" "APP_LTC=0" "APP_ETH=0" "APP_U2F=0")
    # needed to find version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    # needed to find bootloader_version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/bootloader)
    target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${bootloader}.map\" -T\"${CMAKE_SOURCE_DIR}/bootloader.ld\"")
    target_link_libraries(${elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

    target_link_libraries(${elf} PRIVATE ${bootloader}_rust_c)

    # Select the smaller version of libc called nano.
    target_compile_options(${elf} PRIVATE --specs=nano.specs)
    target_link_libraries(${elf} PRIVATE --specs=nano.specs)
    target_compile_options(${elf} PRIVATE --specs=nosys.specs)
    target_link_libraries(${elf} PRIVATE --specs=nosys.specs)
    target_link_libraries(${elf} PRIVATE -Wl,--print-memory-usage)
  endforeach(bootloader)

  foreach(bootloader ${DEVDEVICE-BOOTLOADERS} ${BOOTLOADERS-BITBOX02PLUS})
    set(elf ${bootloader}.elf)
    target_link_libraries(${elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})
    target_sources(${elf} PRIVATE ${QTOUCH-SOURCES})
  endforeach(bootloader)

  foreach(bootloader ${BOOTLOADERS-BITBOX02PLUS})
    set(elf ${bootloader}.elf)
    target_sources(${elf} PRIVATE ${PLATFORM-BITBOX02-PLUS-SOURCES})
    target_link_libraries(${bootloader}.elf PRIVATE embedded-swd)
  endforeach(bootloader)

  # BB02 definitions

  target_compile_definitions(bb02-bl-multi.elf PRIVATE PRODUCT_BITBOX_MULTI)
  target_compile_definitions(bb02-bl-multi-development.elf PRIVATE PRODUCT_BITBOX_MULTI BOOTLOADER_DEVDEVICE)
  target_compile_definitions(bb02-bl-multi-development-locked.elf PRIVATE PRODUCT_BITBOX_MULTI BOOTLOADER_DEVDEVICE BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02-bl-multi-development-locked.elf PROPERTY EXCLUDE_FROM_ALL ON)


  target_compile_definitions(bb02-bl-multi-production.elf PRIVATE PRODUCT_BITBOX_MULTI BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02-bl-multi-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  target_compile_definitions(bb02-bl-btconly.elf PRIVATE PRODUCT_BITBOX_BTCONLY)
  target_compile_definitions(bb02-bl-btconly-development.elf PRIVATE PRODUCT_BITBOX_BTCONLY BOOTLOADER_DEVDEVICE)
  target_compile_definitions(bb02-bl-btconly-production.elf PRIVATE PRODUCT_BITBOX_BTCONLY BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02-bl-btconly-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  # BB02PLUS definitions

  target_compile_definitions(bb02p-bl-multi.elf PRIVATE PRODUCT_BITBOX_PLUS_MULTI)
  target_compile_definitions(bb02p-bl-multi-development.elf PRIVATE PRODUCT_BITBOX_PLUS_MULTI BOOTLOADER_DEVDEVICE)
  target_compile_definitions(bb02p-bl-multi-development-locked.elf PRIVATE PRODUCT_BITBOX_PLUS_MULTI BOOTLOADER_DEVDEVICE BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02p-bl-multi-development-locked.elf PROPERTY EXCLUDE_FROM_ALL ON)


  target_compile_definitions(bb02p-bl-multi-production.elf PRIVATE PRODUCT_BITBOX_PLUS_MULTI BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02p-bl-multi-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  target_compile_definitions(bb02p-bl-btconly.elf PRIVATE PRODUCT_BITBOX_PLUS_BTCONLY)
  target_compile_definitions(bb02p-bl-btconly-development.elf PRIVATE PRODUCT_BITBOX_PLUS_BTCONLY BOOTLOADER_DEVDEVICE)
  target_compile_definitions(bb02p-bl-btconly-production.elf PRIVATE PRODUCT_BITBOX_PLUS_BTCONLY BOOTLOADER_PRODUCTION)
  set_property(TARGET bb02p-bl-btconly-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  foreach(firmware ${FIRMWARES})
    set(elf ${firmware}.elf)
    add_executable(${elf} ${FIRMWARE-SOURCES})
    # Must manually link against C so that malloc can find _sbrk
    target_link_libraries(${elf}
      PRIVATE
        optiga
        embedded-swd
        cryptoauthlib
        c
        ${firmware}_rust_c
        samd51a-ds
        asf4-drivers-min
        asf4-drivers
        -Wl,-u,exception_table)
    target_include_directories(${elf} PRIVATE ${INCLUDES})

    # needed to find version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${firmware}.map\" -T\"${CMAKE_SOURCE_DIR}/firmware.ld\"")
    target_link_libraries(${elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

    target_link_libraries(${elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})

    # Select the smaller version of libc called nano.
    target_compile_options(${elf} PRIVATE --specs=nano.specs)
    target_link_libraries(${elf} PRIVATE --specs=nano.specs)
    target_compile_options(${elf} PRIVATE --specs=nosys.specs)
    target_link_libraries(${elf} PRIVATE --specs=nosys.specs)

    # Optiga config must be defined both when compiling the optiga lib, and also when compiling our
    # sources. We can verify this include is indeed used by our sources - when you point it to a
    # non-existing file, compilation fails.
    # This definition is also added in external/CMakeLists.txt for the optiga lib.
    target_compile_definitions(${elf} PRIVATE OPTIGA_LIB_EXTERNAL="optiga_config.h")
  endforeach(firmware)

  target_sources(firmware.elf PRIVATE firmware.c)
  target_compile_definitions(firmware.elf PRIVATE PRODUCT_BITBOX_MULTI "APP_BTC=1" "APP_LTC=1" "APP_ETH=1" "APP_U2F=1")
  target_sources(firmware.elf PRIVATE ${FIRMWARE-U2F-SOURCES} ${FIRMWARE-U2F-DRIVER-SOURCES} ${PLATFORM-BITBOX02-SOURCES})

  target_sources(firmware-btc.elf PRIVATE firmware.c)
  target_compile_definitions(firmware-btc.elf PRIVATE PRODUCT_BITBOX_BTCONLY "APP_BTC=1" "APP_LTC=0" "APP_ETH=0" "APP_U2F=0")
  target_sources(firmware-btc.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})

  target_sources(factory-setup.elf PRIVATE factorysetup.c)
  target_compile_definitions(factory-setup.elf PRIVATE PRODUCT_BITBOX02_FACTORYSETUP "APP_BTC=0" "APP_LTC=0" "APP_ETH=0" "APP_U2F=0")
  target_sources(factory-setup.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})

  # Copy the binary file to the output directory because the symbol names in
  # the object file depend on the path to the binary file
  add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/bitbox-da14531-firmware.o
    COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/bitbox-da14531-firmware.bin ${CMAKE_CURRENT_BINARY_DIR}
    COMMAND ${CMAKE_OBJCOPY} -Ibinary -Oelf32-littlearm --rename-section .data=.rodata,alloc,load,readonly,data,contents bitbox-da14531-firmware.bin bitbox-da14531-firmware.o
    DEPENDS ${CMAKE_SOURCE_DIR}/bitbox-da14531-firmware.bin
    COMMENT "Convert da14531 firmware to object file using ${CMAKE_OBJCOPY}")

  add_custom_target(
    da14531-firmware-object
    DEPENDS bitbox-da14531-firmware.o
  )
  add_dependencies(factory-setup.elf da14531-firmware-object)

  target_link_libraries(factory-setup.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/bitbox-da14531-firmware.o)


  foreach(name ${BOOTLOADERS} ${FIRMWARES})
    add_custom_command(
      TARGET ${name}.elf POST_BUILD
      COMMAND ${CMAKE_SIZE} ${name}.elf
      COMMAND ${CMAKE_OBJCOPY} -O binary ${name}.elf ${name}.bin
      WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
      COMMENT "\nGenerating binary ${name}.bin"
    )
    # Fail on warnings for our release builds
    if(NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG")
      target_compile_options(${name}.elf PRIVATE "-Werror")
    endif()

    target_link_libraries(${name}.elf PRIVATE -Wl,--print-memory-usage)
  endforeach()
endif()
