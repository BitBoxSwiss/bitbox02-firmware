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
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/regular_11.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/regular_12.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/regular_9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/lvgl_font_compat.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/monogram_16.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/password_9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/password_12.c
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
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/regular_9_bootloader.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/lvgl_font_compat.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/monogram_16_bootloader.c
  ${CMAKE_SOURCE_DIR}/src/ui/graphics/graphics.c
  ${CMAKE_SOURCE_DIR}/src/screen.c
  ${CMAKE_SOURCE_DIR}/src/hardfault.c
  ${CMAKE_SOURCE_DIR}/src/ui/components/ui_images.c
)
set(DBB-BOOTLOADER-SOURCES ${DBB-BOOTLOADER-SOURCES} PARENT_SCOPE)

set(DRIVER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/platform/platform_init.c
  ${CMAKE_SOURCE_DIR}/src/platform/driver_init.c
  ${CMAKE_SOURCE_DIR}/src/platform/sbrk.c
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
  ${CMAKE_SOURCE_DIR}/external/lvgl
)
set(INCLUDES ${INCLUDES} PARENT_SCOPE)

set(LVGL_DEFINITIONS
  LV_KCONFIG_IGNORE
  LV_LVGL_H_INCLUDE_SIMPLE
  LV_CONF_PATH="${CMAKE_SOURCE_DIR}/src/rust/bitbox-lvgl-sys/lv_conf.h"
  LV_CONF_INCLUDE_SIMPLE
)

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

# Referencing $(MAKE) marks Cargo recipes as recursive, preserving GNU make's
# jobserver pipes. JOB_SERVER_AWARE would require CMake 3.28 or newer.
if(CMAKE_GENERATOR MATCHES "Makefiles")
  set(CARGO_JOBSERVER_ENV "BITBOX_CARGO_MAKE=$(MAKE)")
endif()

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
          --workspace
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

set(FIRMWARES
  firmware
  factory-setup # Factory-setup
  )
set(FIRMWARES ${FIRMWARES} PARENT_SCOPE)

if(CMAKE_CROSSCOMPILING)
  # Product IDs must match bootloader/bootloader_product.h.
  set(is_plus FALSE)
  if(PRODUCT STREQUAL "bitbox02")
    set(product_prefix PRODUCT_BITBOX)
    set(product_id 1)
  elseif(PRODUCT STREQUAL "bitbox02nova")
    set(is_plus TRUE)
    set(product_prefix PRODUCT_BITBOX_PLUS)
    set(product_id 3)
  else()
    message(FATAL_ERROR "Invalid PRODUCT: ${PRODUCT}")
  endif()
  if(EDITION STREQUAL "multi")
    set(edition multi)
    set(firmware_name firmware)
    set(app_u2f 1)
  elseif(EDITION STREQUAL "btc-only")
    set(edition btconly)
    set(firmware_name firmware-btc)
    set(app_u2f 0)
    math(EXPR product_id "${product_id} + 1")
  else()
    message(FATAL_ERROR "Invalid EDITION: ${EDITION}")
  endif()
  string(TOUPPER ${edition} edition_upper)
  set(firmware_product_definition PRODUCT_BITBOX_${edition_upper})
  set(product_definition ${product_prefix}_${edition_upper})
  set(bootloader_definitions)
  set(header_flags 0)
  if(BOOTLOADER_DEVELOPMENT)
    set(header_flags 1)
    list(APPEND bootloader_definitions BOOTLOADER_DEVELOPMENT)
  elseif(NOT BOOTLOADER_LOCKED)
    message(FATAL_ERROR "BOOTLOADER_DEVELOPMENT=OFF requires BOOTLOADER_LOCKED=ON")
  endif()
  if(BOOTLOADER_LOCKED)
    list(APPEND bootloader_definitions BOOTLOADER_LOCKED)
  endif()

  set(RUST_LIBS ${FIRMWARES} bootloader-stage1)
else()
  set(RUST_LIBS c-unit-tests)
endif()

#  Create static libraries from rust code. Either:
#  *  c-unit-tests_rust_c
#  or
#  *  firmware_rust_c
#  *  factory-setup_rust_c
foreach(type ${RUST_LIBS})
  if(type STREQUAL "bootloader-stage1")
    set(rust_features bootloader,platform-bitbox02$<$<BOOL:${is_plus}>:plus>)
  elseif(type STREQUAL "firmware")
    set(rust_features firmware,platform-bitbox02,app-bitcoin)
    if(EDITION STREQUAL "multi")
      string(APPEND rust_features ",app-litecoin,app-ethereum,app-u2f,app-cardano")
    endif()
  else()
    set(rust_features target-${type})
  endif()
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
      ${CARGO_JOBSERVER_ENV}
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
        --features ${rust_features}$<$<OR:$<STREQUAL:${CMAKE_BUILD_TYPE},DEBUG>,$<STREQUAL:${type},factory-setup>>:,rtt>
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
  add_dependencies(${type}_rust_c ${type}-rust-target)
  set_property(TARGET ${type}_rust_c PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/lib${type}_rust_c.a)
  # Add the include directory to find rust/rust.h
  target_include_directories(${type}_rust_c INTERFACE ${CMAKE_CURRENT_BINARY_DIR} ${CMAKE_CURRENT_SOURCE_DIR}/rust/fatfs-sys/depend/fatfs/source)
  if (NOT CMAKE_CROSSCOMPILING)
    # Needs to match the definitions in `bitbox02-sys/build.rs`
    target_compile_definitions(${type}_rust_c INTERFACE
        TESTING
        _UNIT_TEST_
        PRODUCT_BITBOX_MULTI=1
        APP_U2F=1
        ${LVGL_DEFINITIONS}
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

  foreach(firmware ${FIRMWARES})
    set(elf ${firmware}.elf)
    if(firmware STREQUAL "firmware")
      set(output_name ${firmware_name})
    else()
      set(output_name ${firmware})
    endif()
    add_executable(${elf} ${FIRMWARE-SOURCES})
    set_target_properties(${elf} PROPERTIES OUTPUT_NAME ${output_name}.elf)
    # Static libraries are LTO-capable for firmware size, but only firmware images do the LTO link.
    if(firmware STREQUAL "factory-setup")
      target_compile_options(${elf} PRIVATE -fno-lto)
      target_link_libraries(${elf} PRIVATE -fno-lto)
    else()
      target_compile_options(${elf} PRIVATE -flto -ffat-lto-objects)
      target_link_libraries(${elf} PRIVATE -flto)
    endif()
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
    target_compile_definitions(${elf} PRIVATE ${LVGL_DEFINITIONS})

    # needed to find version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${output_name}.map\" -T\"${CMAKE_SOURCE_DIR}/firmware.ld\"")
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
  target_compile_definitions(firmware.elf PRIVATE ${firmware_product_definition} "APP_U2F=${app_u2f}")
  if(app_u2f)
    target_sources(firmware.elf PRIVATE ${FIRMWARE-U2F-SOURCES} ${FIRMWARE-U2F-DRIVER-SOURCES})
  endif()
  target_sources(firmware.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})

  target_sources(factory-setup.elf PRIVATE factorysetup.c)
  target_compile_definitions(factory-setup.elf PRIVATE PRODUCT_BITBOX02_FACTORYSETUP "APP_U2F=0")
  target_sources(factory-setup.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})

  set(BB02_BLUPD_BIN_DIR ${CMAKE_SOURCE_DIR}/src/bootloader_upgrade/bin)
  set(BB02_BLUPD_NAME ${PRODUCT}-${edition})

  set(header_dir ${CMAKE_CURRENT_BINARY_DIR}/bootloader/bootloader-stage1)
  set(header_manifest ${header_dir}/image_header.json)
  set(header_bin ${header_dir}/image_header.bin)
  set(header_obj ${header_dir}/image_header.o)
  configure_file(bootloader/image_header.json.in ${header_manifest} @ONLY)
  add_custom_command(
    OUTPUT ${header_bin} ${header_obj}
    COMMAND
      ${PYTHON_EXECUTABLE} ${CMAKE_SOURCE_DIR}/scripts/image_header.py
      render-header --manifest ${header_manifest} --output ${header_bin}
    COMMAND
      ${CMAKE_OBJCOPY} -I binary -O elf32-littlearm -B arm
      --set-section-alignment .data=4
      --rename-section .data=.stage1_header,alloc,load,readonly,data,contents
      ${header_bin} ${header_obj}
    DEPENDS ${header_manifest} ${CMAKE_SOURCE_DIR}/scripts/image_header.py
    COMMENT "Generating image header for bootloader-stage1"
    VERBATIM
  )
  set_source_files_properties(${header_obj} PROPERTIES GENERATED TRUE EXTERNAL_OBJECT TRUE)
  add_executable(bootloader-stage1.elf
    ${BOOTLOADER-SOURCES}
    ${PLATFORM-BITBOX02-SOURCES}
    ${QTOUCH-SOURCES}
    ${header_obj}
  )
  target_compile_options(bootloader-stage1.elf PRIVATE -fno-lto)
  target_link_libraries(bootloader-stage1.elf PRIVATE
    c
    asf4-drivers-min
    samd51a-ds
    ${QTOUCHLIB_A}
    ${QTOUCHLIB_B}
    ${QTOUCHLIB_T}
    -Wl,-u,exception_table
    "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/bootloader-stage1.map\" -T\"${CMAKE_SOURCE_DIR}/bootloader-stage1.ld\""
    -Wl,--defsym=STACK_SIZE=${STACK_SIZE}
    -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE}
    bootloader-stage1_rust_c
    --specs=nano.specs
    --specs=nosys.specs
    -Wl,--print-memory-usage
  )
  target_include_directories(bootloader-stage1.elf PRIVATE ${INCLUDES})
  target_include_directories(bootloader-stage1.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
  target_include_directories(bootloader-stage1.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/bootloader)
  target_compile_definitions(bootloader-stage1.elf PRIVATE
    BOOTLOADER
    ${bootloader_definitions}
    ${product_definition}
    "APP_U2F=0"
    ${LVGL_DEFINITIONS}
  )
  target_compile_options(bootloader-stage1.elf PRIVATE --specs=nano.specs --specs=nosys.specs)
  if(is_plus)
    target_sources(bootloader-stage1.elf PRIVATE ${PLATFORM-BITBOX02-PLUS-SOURCES})
    target_link_libraries(bootloader-stage1.elf PRIVATE embedded-swd)
  endif()
  add_custom_command(
    TARGET bootloader-stage1.elf POST_BUILD
    COMMAND ${CMAKE_SIZE} bootloader-stage1.elf
    COMMAND
      ${PYTHON_EXECUTABLE}
      ${CMAKE_SOURCE_DIR}/scripts/image_header.py
      finalize-elf --section .stage1_header bootloader-stage1.elf
    COMMAND ${CMAKE_OBJCOPY} -O binary bootloader-stage1.elf bootloader-stage1.bin
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nGenerating binary bootloader-stage1.bin"
  )
  set_property(TARGET bootloader-stage1.elf PROPERTY EXCLUDE_FROM_ALL ON)

  # Builds one stage0 ELF/bin for a product. Production stage0 includes stage1 signature
  # verification; development stage0 skips that path and shows a visible development marker.
  set(stage0_sources
    ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage0.c
    ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage0_descriptor.c
    ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage0_runtime.c
    ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage0_startup.c
    ${CMAKE_SOURCE_DIR}/src/platform/driver_init.c
    ${CMAKE_SOURCE_DIR}/src/ui/oled/oled.c
    ${CMAKE_SOURCE_DIR}/src/ui/oled/oled_writer.c
    ${CMAKE_SOURCE_DIR}/src/ui/oled/sh1107.c
    ${CMAKE_SOURCE_DIR}/src/ui/oled/ssd1312.c
  )
  set_source_files_properties(
    ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage0_startup.c
    PROPERTIES COMPILE_OPTIONS -fno-stack-protector
  )
  if(NOT BOOTLOADER_DEVELOPMENT)
    list(APPEND stage0_sources
      ${CMAKE_SOURCE_DIR}/src/bootloader_upgrade/stage1_pubkeys.c
      ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage1_sigcheck.c
      ${CMAKE_SOURCE_DIR}/src/pukcc/pukcc.c
      ${CMAKE_SOURCE_DIR}/src/pukcc/curve_p256.c
    )
  endif()

  add_executable(bootloader-stage0.elf ${stage0_sources})
  target_compile_options(bootloader-stage0.elf PRIVATE -flto -ffat-lto-objects)
  target_include_directories(bootloader-stage0.elf PRIVATE ${INCLUDES})
  target_include_directories(bootloader-stage0.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
  target_compile_definitions(bootloader-stage0.elf PRIVATE
    ${product_definition}
    BB02_STAGE1_TARGET_PRODUCT_ID=${product_id}
    ${bootloader_definitions}
  )
  target_link_libraries(bootloader-stage0.elf
    PRIVATE
      -flto
      -nostdlib
      samd51a-ds
      asf4-drivers-stage0
      -Wl,-u,exception_table
      "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/bootloader-stage0.map\" -T\"${CMAKE_SOURCE_DIR}/bootloader-stage0.ld\""
      -Wl,--defsym=STACK_SIZE=0x1000
      -Wl,-defsym=HEAP_SIZE=0
      -Wl,--print-memory-usage
  )
  set_property(TARGET bootloader-stage0.elf PROPERTY EXCLUDE_FROM_ALL ON)

  add_custom_command(
    TARGET bootloader-stage0.elf POST_BUILD
    COMMAND ${CMAKE_SIZE} bootloader-stage0.elf
    COMMAND ${CMAKE_OBJCOPY} --gap-fill 0xff -O binary bootloader-stage0.elf bootloader-stage0.bin
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nGenerating binary bootloader-stage0.bin"
  )

  # Creates the embedded prebuilt stage0 image object and stage1 update payload object used by one
  # bootloader-upgrade firmware target.
  function(add_bb02_blupgrade_assets
      suffix
      out_prefix
      stage0_bin
      stage1_signed_bin
      development)
    set(asset_dir ${CMAKE_CURRENT_BINARY_DIR}/bootloader_upgrade_${BB02_BLUPD_NAME}${suffix})
    set(stage0_image_bin ${asset_dir}/bootloader_upgrade_stage0_image.bin)
    set(stage0_image_obj ${asset_dir}/bootloader_upgrade_stage0_image.o)
    set(payload_bin ${asset_dir}/bootloader_upgrade_payload.bin)
    set(payload_obj ${asset_dir}/bootloader_upgrade_payload.o)
    set(development_arg)
    if(development)
      set(development_arg --development)
    endif()

    add_custom_command(
      OUTPUT ${stage0_image_bin}
      COMMAND ${CMAKE_COMMAND} -E make_directory ${asset_dir}
      COMMAND
        ${PYTHON_EXECUTABLE}
        ${CMAKE_SOURCE_DIR}/scripts/bootloader_update.py
        create-stage0-fw-embedding
        --stage0-bin ${stage0_bin}
        --out-bin ${stage0_image_bin}
        --product-id ${product_id}
        ${development_arg}
      DEPENDS ${stage0_bin} ${CMAKE_SOURCE_DIR}/scripts/bootloader_update.py
      COMMENT "Creating stage0 firmware embedding input from ${stage0_bin}"
    )

    add_custom_command(
      OUTPUT ${stage0_image_obj}
      COMMAND
        ${CMAKE_OBJCOPY}
        -Ibinary
        -Oelf32-littlearm
        --rename-section .data=.rodata,alloc,load,readonly,data,contents
        --set-section-alignment .rodata=512
        bootloader_upgrade_stage0_image.bin
        bootloader_upgrade_stage0_image.o
      DEPENDS ${stage0_image_bin}
      WORKING_DIRECTORY ${asset_dir}
      COMMENT "Convert stage0 upgrade image to object file using ${CMAKE_OBJCOPY}"
    )

    add_custom_command(
      OUTPUT ${payload_bin}
      COMMAND ${CMAKE_COMMAND} -E make_directory ${asset_dir}
      COMMAND
        ${PYTHON_EXECUTABLE}
        ${CMAKE_SOURCE_DIR}/scripts/bootloader_update.py
        create-stage1-fw-embedding
        --signed-bin ${stage1_signed_bin}
        --out-bin ${payload_bin}
        --product-id ${product_id}
        ${development_arg}
      DEPENDS ${stage1_signed_bin} ${CMAKE_SOURCE_DIR}/scripts/bootloader_update.py
      COMMENT "Creating stage1 firmware embedding input from ${stage1_signed_bin}"
    )

    add_custom_command(
      OUTPUT ${payload_obj}
      COMMAND
        ${CMAKE_OBJCOPY}
        -Ibinary
        -Oelf32-littlearm
        --rename-section .data=.bootloader_update,alloc,load,readonly,data,contents
        --set-section-alignment .bootloader_update=512
        bootloader_upgrade_payload.bin
        bootloader_upgrade_payload.o
      DEPENDS ${payload_bin}
      WORKING_DIRECTORY ${asset_dir}
      COMMENT "Convert stage1 upgrade payload to object file using ${CMAKE_OBJCOPY}"
    )
    set_source_files_properties(
      ${stage0_image_obj}
      ${payload_obj}
      PROPERTIES GENERATED TRUE EXTERNAL_OBJECT TRUE)
    add_custom_target(
      bootloader-upgrade-assets${suffix}
      DEPENDS
        ${stage0_image_bin}
        ${stage0_image_obj}
        ${payload_bin}
        ${payload_obj}
    )
    set(${out_prefix}_STAGE0_IMAGE_OBJ ${stage0_image_obj} PARENT_SCOPE)
    set(${out_prefix}_PAYLOAD_OBJ ${payload_obj} PARENT_SCOPE)
  endfunction()

  add_bb02_blupgrade_assets(
    ""
    BB02_BLUPD
    ${BB02_BLUPD_BIN_DIR}/bootloader-stage0-${BB02_BLUPD_NAME}-production.v3.bin
    ${BB02_BLUPD_BIN_DIR}/bootloader-stage1-${BB02_BLUPD_NAME}-production.v1.2.3.signed.bin
    FALSE)
  add_bb02_blupgrade_assets(
    "-development"
    BB02_BLUPD_DEVELOPMENT
    ${BB02_BLUPD_BIN_DIR}/bootloader-stage0-${BB02_BLUPD_NAME}-development.v3.bin
    ${BB02_BLUPD_BIN_DIR}/bootloader-stage1-${BB02_BLUPD_NAME}-development.v1.2.3.signed.bin
    TRUE)

  add_custom_target(bootloader-stage1 DEPENDS bootloader-stage1.elf)
  add_custom_target(bootloader-stage0 DEPENDS bootloader-stage0.elf)

  function(add_bb02_blupgrade_firmware suffix stage0_image_obj payload_obj development)
    set(target firmware-blupgrade${suffix})
    set(output_name firmware-blupgrade-${BB02_BLUPD_NAME}${suffix})
    add_executable(${target}.elf
      ${FIRMWARE-SOURCES}
      ${CMAKE_SOURCE_DIR}/src/firmware.c
      ${CMAKE_SOURCE_DIR}/src/bootloader_upgrade/firmware_installer_check.c
      ${CMAKE_SOURCE_DIR}/src/bootloader_upgrade/firmware_installer.c
      ${stage0_image_obj}
      ${payload_obj}
    )
    set_target_properties(${target}.elf PROPERTIES OUTPUT_NAME ${output_name}.elf)
    target_compile_options(${target}.elf PRIVATE -flto -ffat-lto-objects)
    target_link_libraries(${target}.elf PRIVATE
      optiga
      embedded-swd
      cryptoauthlib
      c
      firmware_rust_c
      samd51a-ds
      asf4-drivers-min
      asf4-drivers
      -Wl,-u,exception_table
      -flto
      "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${output_name}.map\" -T\"${CMAKE_SOURCE_DIR}/firmware-blupgrade.ld\""
      -Wl,--defsym=STACK_SIZE=${STACK_SIZE}
      -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE}
      ${QTOUCHLIB_A}
      ${QTOUCHLIB_B}
      ${QTOUCHLIB_T}
      --specs=nano.specs
      --specs=nosys.specs
      -Wl,--print-memory-usage
    )
    target_compile_options(${target}.elf PRIVATE --specs=nano.specs --specs=nosys.specs)
    target_include_directories(${target}.elf PRIVATE ${INCLUDES})
    target_include_directories(${target}.elf PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    set(blupgrade_compile_definitions BOOTLOADER_UPGRADE)
    if(development)
      list(APPEND blupgrade_compile_definitions BOOTLOADER_UPGRADE_DEVELOPMENT)
    endif()
    target_compile_definitions(${target}.elf PRIVATE
      ${firmware_product_definition}
      "APP_U2F=${app_u2f}"
      BB02_STAGE1_TARGET_PRODUCT_ID=${product_id}
      ${blupgrade_compile_definitions}
      ${LVGL_DEFINITIONS}
      OPTIGA_LIB_EXTERNAL="optiga_config.h"
    )
    target_sources(${target}.elf PRIVATE ${PLATFORM-BITBOX02-SOURCES})
    if(${app_u2f} EQUAL 1)
      target_sources(${target}.elf PRIVATE ${FIRMWARE-U2F-SOURCES} ${FIRMWARE-U2F-DRIVER-SOURCES})
    endif()
    if(NOT development)
      target_sources(${target}.elf PRIVATE
        ${CMAKE_SOURCE_DIR}/src/bootloader_upgrade/stage1_pubkeys.c
        ${CMAKE_SOURCE_DIR}/src/bootloader/stage0/stage1_sigcheck.c
        ${CMAKE_SOURCE_DIR}/src/pukcc/pukcc.c
        ${CMAKE_SOURCE_DIR}/src/pukcc/curve_p256.c
      )
    endif()
    add_custom_command(
      TARGET ${target}.elf POST_BUILD
      COMMAND ${CMAKE_SIZE} ${output_name}.elf
      COMMAND ${CMAKE_OBJCOPY} -O binary ${output_name}.elf ${output_name}.bin
      WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
      COMMENT "\nGenerating binary ${output_name}.bin"
    )
  endfunction()

  add_bb02_blupgrade_firmware(
    ""
    ${BB02_BLUPD_STAGE0_IMAGE_OBJ}
    ${BB02_BLUPD_PAYLOAD_OBJ}
    FALSE)
  add_bb02_blupgrade_firmware(
    "-development"
    ${BB02_BLUPD_DEVELOPMENT_STAGE0_IMAGE_OBJ}
    ${BB02_BLUPD_DEVELOPMENT_PAYLOAD_OBJ}
    TRUE)

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


  foreach(name ${FIRMWARES})
    if(name STREQUAL "firmware")
      set(output_name ${firmware_name})
    else()
      set(output_name ${name})
    endif()
    add_custom_command(
      TARGET ${name}.elf POST_BUILD
      COMMAND ${CMAKE_SIZE} ${output_name}.elf
      COMMAND ${CMAKE_OBJCOPY} -O binary ${output_name}.elf ${output_name}.bin
      WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
      COMMENT "\nGenerating binary ${output_name}.bin"
    )
    # Fail on warnings for our release builds
    if(NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG")
      target_compile_options(${name}.elf PRIVATE "-Werror")
    endif()

    target_link_libraries(${name}.elf PRIVATE -Wl,--print-memory-usage)
  endforeach()
endif()
