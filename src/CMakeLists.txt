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
  ${CMAKE_SOURCE_DIR}/src/i2c_ecc.c
  ${CMAKE_SOURCE_DIR}/src/touch/gestures.c
  ${CMAKE_SOURCE_DIR}/src/reset.c
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
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_swap.c
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
  ${CMAKE_SOURCE_DIR}/src/bootloader/bootloader_format.c
  ${CMAKE_SOURCE_DIR}/src/bootloader/startup.c
  ${CMAKE_SOURCE_DIR}/src/bootloader/mpu_regions.c
  ${CMAKE_SOURCE_DIR}/src/random.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_shared.c
  ${CMAKE_SOURCE_DIR}/src/memory/mpu.c
  ${CMAKE_SOURCE_DIR}/src/memory/nvmctrl.c
  ${CMAKE_SOURCE_DIR}/src/memory/spi_mem.c
  ${CMAKE_SOURCE_DIR}/src/memory/memory_spi.c
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
  string(JOIN " " RUSTFLAGS "${RUSTFLAGS}" "-Zunstable-options -Zfmt-debug=none -Cpanic=immediate-abort")
else()
  set(RUST_TARGET_ARCH_DIR .)
endif()
if(CMAKE_CROSSCOMPILING)
  set(CARGO_CC_FLAGS_ENV "CFLAGS_${RUST_TARGET_ARCH}=${CARGO_C_FLAGS} --sysroot=${CMAKE_SYSROOT}")
else()
  set(CARGO_CC_FLAGS_ENV "CFLAGS=${CARGO_C_FLAGS}")
endif()

set(RUST_BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR}/rust)
set(LIBBITBOX02_RUST_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust/bitbox02-rust-c)
set(LIBBITBOX02_RUST ${LIBBITBOX02_RUST} PARENT_SCOPE)

# Generate c-headers for the rust library
add_custom_target(rust-cbindgen
  COMMAND
    ${CMAKE_COMMAND} -E env
      CARGO_BIN=${CARGO}
      CBINDGEN_BIN=${CBINDGEN}
      bash
      ${CMAKE_CURRENT_SOURCE_DIR}/../scripts/generate_rust_header.sh
      ${CMAKE_CURRENT_BINARY_DIR}/rust
  WORKING_DIRECTORY
    ${CMAKE_CURRENT_SOURCE_DIR}/rust
  BYPRODUCTS
    ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
  COMMENT
    "Generating ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h"
)

# Test rust crates that contain business logic. Avoid testing crates that depend on hardware.
if(NOT CMAKE_CROSSCOMPILING)
  add_custom_target(rust-clippy
    COMMAND
      ${CMAKE_COMMAND} -E env
        CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
        ${CARGO} clippy
          $<$<BOOL:${CMAKE_VERBOSE_MAKEFILE}>:-v>
          --all-features
          --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/rust/Cargo.toml
          --target-dir ${RUST_BINARY_DIR}/clippy
          --release
          --tests --
            # enabled linters:
            -W clippy::use_debug
            -W clippy::large_futures
            # disabled linters:
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
  add_dependencies(rust-clippy rust-cbindgen)
endif()

function(add_rust_static_library type rust_features)
  set(rust_cargo_flags ${RUST_CARGO_FLAGS})
  # We build the rust standard libs so that everything from the rust runtime
  # is included (like eh_personality).
  if(type STREQUAL "c-unit-tests")
    list(APPEND rust_cargo_flags -Zbuild-std=std,panic_abort)
  endif()

  set(lib ${RUST_BINARY_DIR}/feature-${type}/${RUST_TARGET_ARCH_DIR}/${RUST_PROFILE}/libbitbox02_rust_c.a)
  # The dummy output is to always trigger rebuild (cargo is clever enough to
  # only rebuild if something changed).
  add_custom_command(
    OUTPUT ${lib} ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a dummy
    COMMAND
      ${CMAKE_COMMAND} -E env
      CMAKE_CURRENT_BINARY_DIR=${CMAKE_CURRENT_BINARY_DIR}
      RUSTFLAGS=${RUSTFLAGS}
      ${CARGO_CC_FLAGS_ENV}
      $<$<BOOL:${SCCACHE_PROGRAM}>:RUSTC_WRAPPER=${SCCACHE_PROGRAM}>
      RUSTC_BOOTSTRAP=1
      MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}
      LIB_TYPE=${type}
      ${CARGO}
        rustc
        $<$<BOOL:${CMAKE_VERBOSE_MAKEFILE}>:-vv>
        --offline
        $ENV{CARGOFLAGS}
        --features ${rust_features}$<$<OR:$<STREQUAL:${CMAKE_BUILD_TYPE},DEBUG>,$<BOOL:${FACTORY_SETUP}>>:,rtt>
        --manifest-path ${LIBBITBOX02_RUST_SOURCE_DIR}/Cargo.toml
        --target-dir ${RUST_BINARY_DIR}/feature-${type}
        ${rust_cargo_flags}
        ${RUST_TARGET_ARCH_ARG}
        --crate-type=staticlib
    COMMAND
      ${CMAKE_COMMAND} -E copy_if_different ${lib} ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a
    # DEPFILES are only supported with the Ninja build tool.
    #DEPFILE ${RUST_BINARY_DIR}/feature-${type}/${RUST_TARGET_ARCH_DIR}/${RUST_PROFILE}/libbitbox02_rust_c.d
    WORKING_DIRECTORY
      ${CMAKE_CURRENT_SOURCE_DIR}/rust
    COMMENT "Building Rust library lib${type}_rust_c.a"
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/rust/rust.h
  )
  add_custom_target(${type}-rust-target DEPENDS ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a)
  add_library(${type}_rust_c STATIC IMPORTED GLOBAL)
  add_dependencies(${type}_rust_c ${type}-rust-target)
  set_property(TARGET ${type}_rust_c PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a)
  # Add the include directory to find rust/rust.h.
  target_include_directories(${type}_rust_c INTERFACE ${CMAKE_CURRENT_BINARY_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/rust/fatfs-sys/depend/fatfs/source)
  if (NOT CMAKE_CROSSCOMPILING)
    # Needs to match the definitions in `bitbox02-sys/build.rs`.
    target_compile_definitions(${type}_rust_c INTERFACE
        TESTING
        _UNIT_TEST_
        PRODUCT_BITBOX_MULTI=1
        APP_U2F=1
    )
    # For tests all C-files are built into the rust lib.
    target_include_directories(${type}_rust_c INTERFACE
        ${INCLUDES}
        ${CMAKE_CURRENT_SOURCE_DIR}/../test/hardware-fakes/include
    )
  endif()
endfunction()

if(CMAKE_CROSSCOMPILING)
  if(FACTORY_SETUP)
    add_rust_static_library(${FIRMWARE_TARGET} "factory-setup,bitbox02-platform-${BITBOX02_PLATFORM}")
  else()
    add_rust_static_library(${FIRMWARE_TARGET} "firmware,bitbox02-platform-${BITBOX02_PLATFORM},bitbox02-edition-${BITBOX02_EDITION}")
  endif()

  add_rust_static_library(${BOOTLOADER_TARGET} "bootloader,bitbox02-platform-${BITBOX02_PLATFORM}")
else()
  add_rust_static_library(c-unit-tests "bitbox02-edition-multi,firmware,c-unit-testing")
endif()


if(CMAKE_CROSSCOMPILING)
  add_custom_target(rust-docs
    COMMAND
      ${CMAKE_COMMAND} -E env
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

  add_executable(${BOOTLOADER_TARGET}.elf ${BOOTLOADER-SOURCES} ${PLATFORM-BITBOX02-SOURCES})
  target_compile_options(${BOOTLOADER_TARGET}.elf PRIVATE -fno-lto)
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE c asf4-drivers-min samd51a-ds -Wl,-u,exception_table)
  target_include_directories(${BOOTLOADER_TARGET}.elf PRIVATE ${INCLUDES})
  target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE BOOTLOADER "APP_U2F=0")
  if(BOOTLOADER_LOCKED)
    target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE BOOTLOADER_PRODUCTION)
  endif()
  if(BOOTLOADER_DEVDEVICE)
    target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE BOOTLOADER_DEVDEVICE)
  endif()
  # needed to find version.h
  target_include_directories(${BOOTLOADER_TARGET}.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
  # needed to find bootloader_version.h
  target_include_directories(${BOOTLOADER_TARGET}.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/bootloader)
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${BOOTLOADER_TARGET}.map\" -T\"${CMAKE_SOURCE_DIR}/bootloader.ld\"")
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE ${BOOTLOADER_TARGET}_rust_c)

  # Select the smaller version of libc called nano.
  target_compile_options(${BOOTLOADER_TARGET}.elf PRIVATE --specs=nano.specs)
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE --specs=nano.specs)
  target_compile_options(${BOOTLOADER_TARGET}.elf PRIVATE --specs=nosys.specs)
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE --specs=nosys.specs)
  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE -Wl,--print-memory-usage)

  if(BITBOX02_PLATFORM STREQUAL "nova" OR BOOTLOADER_DEVDEVICE)
    target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})
    target_sources(${BOOTLOADER_TARGET}.elf PRIVATE ${QTOUCH-SOURCES})
  endif()

  if(BITBOX02_PLATFORM STREQUAL "nova")
    target_sources(${BOOTLOADER_TARGET}.elf PRIVATE ${PLATFORM-BITBOX02-PLUS-SOURCES})
    if(CMAKE_BUILD_TYPE STREQUAL "DEBUG")
      target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE embedded-swd)
    endif()
  endif()

  if(BITBOX02_EDITION STREQUAL "btconly")
    if(BITBOX02_PLATFORM STREQUAL "nova")
      target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE PRODUCT_BITBOX_PLUS_BTCONLY)
    else()
      target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE PRODUCT_BITBOX_BTCONLY)
    endif()
  else()
    if(BITBOX02_PLATFORM STREQUAL "nova")
      target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE PRODUCT_BITBOX_PLUS_MULTI)
    else()
      target_compile_definitions(${BOOTLOADER_TARGET}.elf PRIVATE PRODUCT_BITBOX_MULTI)
    endif()
  endif()

  set(firmware_elf ${FIRMWARE_TARGET}.elf)
  add_executable(${firmware_elf} ${FIRMWARE-SOURCES})
  # Static libraries are LTO-capable for firmware size, but only firmware images do the LTO link.
  if(FACTORY_SETUP)
    target_compile_options(${firmware_elf} PRIVATE -fno-lto)
    target_link_libraries(${firmware_elf} PRIVATE -fno-lto)
  else()
    target_compile_options(${firmware_elf} PRIVATE -flto -ffat-lto-objects)
    target_link_libraries(${firmware_elf} PRIVATE -flto)
  endif()
  # Must manually link against C so that malloc can find _sbrk
  target_link_libraries(${firmware_elf}
    PRIVATE
      optiga
      cryptoauthlib
      c
      ${FIRMWARE_TARGET}_rust_c
      samd51a-ds
      asf4-drivers-min
      asf4-drivers
      -Wl,-u,exception_table)
  if(FACTORY_SETUP OR CMAKE_BUILD_TYPE STREQUAL "DEBUG")
    target_link_libraries(${firmware_elf} PRIVATE embedded-swd)
  endif()
  target_include_directories(${firmware_elf} PRIVATE ${INCLUDES})

  # needed to find version.h
  target_include_directories(${firmware_elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
  target_link_libraries(${firmware_elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${FIRMWARE_TARGET}.map\" -T\"${CMAKE_SOURCE_DIR}/firmware.ld\"")
  target_link_libraries(${firmware_elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

  target_link_libraries(${firmware_elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})

  # Select the smaller version of libc called nano.
  target_compile_options(${firmware_elf} PRIVATE --specs=nano.specs)
  target_link_libraries(${firmware_elf} PRIVATE --specs=nano.specs)
  target_compile_options(${firmware_elf} PRIVATE --specs=nosys.specs)
  target_link_libraries(${firmware_elf} PRIVATE --specs=nosys.specs)

  # Optiga config must be defined both when compiling the optiga lib, and also when compiling our
  # sources. We can verify this include is indeed used by our sources - when you point it to a
  # non-existing file, compilation fails.
  # This definition is also added in external/CMakeLists.txt for the optiga lib.
  target_compile_definitions(${firmware_elf} PRIVATE OPTIGA_LIB_EXTERNAL="optiga_config.h")

  if(FACTORY_SETUP)
    target_sources(${firmware_elf} PRIVATE factorysetup.c)
    target_compile_definitions(${firmware_elf} PRIVATE PRODUCT_BITBOX02_FACTORYSETUP "APP_U2F=0")
    target_sources(${firmware_elf} PRIVATE ${PLATFORM-BITBOX02-SOURCES})
  else()
    target_sources(${firmware_elf} PRIVATE firmware.c)
  endif()

  if(NOT FACTORY_SETUP AND BITBOX02_EDITION STREQUAL "btconly")
    target_compile_definitions(${FIRMWARE_TARGET}.elf PRIVATE PRODUCT_BITBOX_BTCONLY "APP_U2F=0")
    target_sources(${FIRMWARE_TARGET}.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})
  elseif(NOT FACTORY_SETUP)
    target_compile_definitions(${FIRMWARE_TARGET}.elf PRIVATE PRODUCT_BITBOX_MULTI "APP_U2F=1")
    target_sources(${FIRMWARE_TARGET}.elf PRIVATE ${FIRMWARE-U2F-SOURCES} ${FIRMWARE-U2F-DRIVER-SOURCES} ${PLATFORM-BITBOX02-SOURCES})
  endif()

  if(FACTORY_SETUP)
    # Copy the binary file to the output directory because the symbol names in
    # the object file depend on the path to the binary file.
    set(DA14531_FIRMWARE_OBJECT ${CMAKE_CURRENT_BINARY_DIR}/bitbox-da14531-firmware.o)
    add_custom_command(
      OUTPUT ${DA14531_FIRMWARE_OBJECT}
      COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_SOURCE_DIR}/bitbox-da14531-firmware.bin ${CMAKE_CURRENT_BINARY_DIR}
      COMMAND ${CMAKE_OBJCOPY} -Ibinary -Oelf32-littlearm --rename-section .data=.rodata,alloc,load,readonly,data,contents bitbox-da14531-firmware.bin bitbox-da14531-firmware.o
      DEPENDS ${CMAKE_SOURCE_DIR}/bitbox-da14531-firmware.bin
      COMMENT "Convert da14531 firmware to object file using ${CMAKE_OBJCOPY}")
    set_source_files_properties(${DA14531_FIRMWARE_OBJECT} PROPERTIES GENERATED TRUE EXTERNAL_OBJECT TRUE)
    target_sources(${firmware_elf} PRIVATE ${DA14531_FIRMWARE_OBJECT})
  endif()

  add_custom_command(
    TARGET ${BOOTLOADER_TARGET}.elf POST_BUILD
    COMMAND ${CMAKE_SIZE} ${BOOTLOADER_TARGET}.elf
    COMMAND ${CMAKE_OBJCOPY} -O binary ${BOOTLOADER_TARGET}.elf ${BOOTLOADER_TARGET}.bin
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nGenerating binary ${BOOTLOADER_TARGET}.bin"
  )
  # Fail on warnings for our release builds
  if(NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG")
    target_compile_options(${BOOTLOADER_TARGET}.elf PRIVATE "-Werror")
  endif()

  target_link_libraries(${BOOTLOADER_TARGET}.elf PRIVATE -Wl,--print-memory-usage)

  add_custom_command(
    TARGET ${FIRMWARE_TARGET}.elf POST_BUILD
    COMMAND ${CMAKE_SIZE} ${FIRMWARE_TARGET}.elf
    COMMAND ${CMAKE_OBJCOPY} -O binary ${FIRMWARE_TARGET}.elf ${FIRMWARE_TARGET}.bin
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nGenerating binary ${FIRMWARE_TARGET}.bin"
  )
  # Fail on warnings for our release builds
  if(NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG")
    target_compile_options(${FIRMWARE_TARGET}.elf PRIVATE "-Werror")
  endif()

  target_link_libraries(${FIRMWARE_TARGET}.elf PRIVATE -Wl,--print-memory-usage)
endif()
