# Copyright (c) 2015-2016 Lucas Betschart, Douglas J. Bakkum
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

set(DBB-FIRMWARE-SOURCES
  ${CMAKE_SOURCE_DIR}/src/firmware_main_loop.c
  ${CMAKE_SOURCE_DIR}/src/commander/commander.c
  ${CMAKE_SOURCE_DIR}/src/commander/commander_btc.c
  ${CMAKE_SOURCE_DIR}/src/commander/commander_states.c
  ${CMAKE_SOURCE_DIR}/src/keystore.c
  ${CMAKE_SOURCE_DIR}/src/random.c
  ${CMAKE_SOURCE_DIR}/src/attestation.c
  ${CMAKE_SOURCE_DIR}/src/hardfault.c
  ${CMAKE_SOURCE_DIR}/src/util.c
  ${CMAKE_SOURCE_DIR}/src/sd.c
  ${CMAKE_SOURCE_DIR}/src/hww.c
  ${CMAKE_SOURCE_DIR}/src/memory.c
  ${CMAKE_SOURCE_DIR}/src/salt.c
  ${CMAKE_SOURCE_DIR}/src/i2c_ecc.c
  ${CMAKE_SOURCE_DIR}/src/touch/gestures.c
  ${CMAKE_SOURCE_DIR}/src/usb/noise.c
  ${CMAKE_SOURCE_DIR}/src/backup.c
  ${CMAKE_SOURCE_DIR}/src/restore.c
  ${CMAKE_SOURCE_DIR}/src/reset.c
  ${CMAKE_SOURCE_DIR}/src/cipher/cipher.c
  ${CMAKE_SOURCE_DIR}/src/workflow/blocking.c
  ${CMAKE_SOURCE_DIR}/src/workflow/cancel.c
  ${CMAKE_SOURCE_DIR}/src/workflow/workflow.c
  ${CMAKE_SOURCE_DIR}/src/workflow/trinary_input.c
  ${CMAKE_SOURCE_DIR}/src/workflow/confirm.c
  ${CMAKE_SOURCE_DIR}/src/workflow/confirm_time.c
  ${CMAKE_SOURCE_DIR}/src/workflow/status.c
  ${CMAKE_SOURCE_DIR}/src/workflow/password.c
  ${CMAKE_SOURCE_DIR}/src/workflow/password_enter.c
  ${CMAKE_SOURCE_DIR}/src/workflow/create_seed.c
  ${CMAKE_SOURCE_DIR}/src/workflow/unlock.c
  ${CMAKE_SOURCE_DIR}/src/workflow/show_mnemonic.c
  ${CMAKE_SOURCE_DIR}/src/workflow/restore_from_mnemonic.c
  ${CMAKE_SOURCE_DIR}/src/workflow/pairing.c
  ${CMAKE_SOURCE_DIR}/src/workflow/verify_pub.c
  ${CMAKE_SOURCE_DIR}/src/workflow/verify_recipient.c
  ${CMAKE_SOURCE_DIR}/src/workflow/verify_total.c
  ${CMAKE_SOURCE_DIR}/src/workflow/reboot.c
  ${CMAKE_SOURCE_DIR}/src/workflow/sdcard.c
  ${CMAKE_SOURCE_DIR}/src/workflow/backup.c
  ${CMAKE_SOURCE_DIR}/src/workflow/reset.c
  ${CMAKE_SOURCE_DIR}/src/workflow/restore.c
  ${CMAKE_SOURCE_DIR}/src/apps/btc/btc.c
  ${CMAKE_SOURCE_DIR}/src/apps/btc/btc_common.c
  ${CMAKE_SOURCE_DIR}/src/apps/btc/btc_params.c
  ${CMAKE_SOURCE_DIR}/src/apps/btc/btc_sign.c
  ${CMAKE_SOURCE_DIR}/src/apps/btc/btc_bip143.c
)
set(DBB-FIRMWARE-SOURCES ${DBB-FIRMWARE-SOURCES} PARENT_SCOPE)

set(DBB-FIRMWARE-USB-SOURCES
  ${CMAKE_SOURCE_DIR}/src/usb/usb.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_frame.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_packet.c
  ${CMAKE_SOURCE_DIR}/src/u2f/u2f_packet.c
  ${CMAKE_SOURCE_DIR}/src/usb/usb_processing.c
  ${CMAKE_SOURCE_DIR}/src/queue.c
)
set(DBB-FIRMWARE-USB-SOURCES ${DBB-FIRMWARE-USB-SOURCES} PARENT_SCOPE)

set(DBB-FIRMWARE-UI-SOURCES
    ${CMAKE_SOURCE_DIR}/src/screen.c
    ${CMAKE_SOURCE_DIR}/src/ui/ugui/ugui.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_9X9.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_11X12.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_13X14.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_15X16.c
    ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_17X18.c
    ${CMAKE_SOURCE_DIR}/src/ui/screen_stack.c
    ${CMAKE_SOURCE_DIR}/src/ui/screen_process.c
    ${CMAKE_SOURCE_DIR}/src/ui/event_handler.c
    ${CMAKE_SOURCE_DIR}/src/ui/ui_util.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_choice.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_input_char.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/trinary_input_string.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/show_logo.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/waiting.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/entry_screen.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/knight_rider.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/right_arrow.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/left_arrow.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/icon_button.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_gesture.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/label.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_button.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/keyboard_switch.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/orientation_arrows.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/info_centered.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/scroll_through_all_variants.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/status.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/image.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/button.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/insert_sd_card.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/remove_sd_card.c
    ${CMAKE_SOURCE_DIR}/src/ui/components/confirm_mnemonic.c
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
  ${CMAKE_SOURCE_DIR}/src/bootloader/mpu.c
  ${CMAKE_SOURCE_DIR}/src/ui/ugui/ugui.c
  ${CMAKE_SOURCE_DIR}/src/ui/fonts/font_a_9X9.c
  ${CMAKE_SOURCE_DIR}/src/screen.c
  ${CMAKE_SOURCE_DIR}/src/hardfault.c
  ${CMAKE_SOURCE_DIR}/src/ui/components/ui_images.c
)
set(DBB-BOOTLOADER-SOURCES ${DBB-BOOTLOADER-SOURCES} PARENT_SCOPE)

set(DRIVER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/peripherals_init.c
  ${CMAKE_SOURCE_DIR}/src/sd_mmc/sd_mmc_start.c
  ${CMAKE_SOURCE_DIR}/src/ui/oled/oled.c
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hid.c
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hww/hid_hww.c
)
set(DRIVER-SOURCES ${DRIVER-SOURCES} PARENT_SCOPE)

set(CRYPTOAUTHLIB-SOURCES
  ${CMAKE_SOURCE_DIR}/src/securechip/securechip.c
)
set(CRYPTOAUTHLIB-SOURCES ${CRYPTOAUTHLIB-SOURCES} PARENT_SCOPE)

set(ETHEREUM-SOURCES
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth.c
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth_common.c
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth_params.c
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth_sighash.c
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth_sign.c
  ${CMAKE_SOURCE_DIR}/src/apps/eth/eth_verify.c
  ${CMAKE_SOURCE_DIR}/src/commander/commander_eth.c
)
set(ETHEREUM-SOURCES ${ETHEREUM-SOURCES} PARENT_SCOPE)

set(FIRMWARE-DRIVER-SOURCES
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/u2f/hid_u2f.c
)
set(FIRMWARE-DRIVER-SOURCES ${FIRMWARE-DRIVER-SOURCES} PARENT_SCOPE)

set(FIRMWARE-U2F-SOURCES
  ${CMAKE_SOURCE_DIR}/src/u2f.c
  ${CMAKE_SOURCE_DIR}/src/u2f/u2f_app.c
)
set(FIRMWARE-U2F-SOURCES ${FIRMWARE-U2F-SOURCES} PARENT_SCOPE)

#-----------------------------------------------------------------------------
# Include directories

set(INCLUDES
  ${CMAKE_SOURCE_DIR}/src
  ${CMAKE_SOURCE_DIR}/src/usb/class
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/hww
  ${CMAKE_SOURCE_DIR}/src/usb/class/hid/u2f
)
set(INCLUDES ${INCLUDES} PARENT_SCOPE)

set(SYSTEMINCLUDES
  ${CMAKE_BINARY_DIR}/messages
)
set(SYSTEMINCLUDES ${SYSTEMINCLUDES} PARENT_SCOPE)

#-----------------------------------------------------------------------------
# Build embedded firmware


# TODO: Not needed when building bootlaoder..
find_package(Nanopb REQUIRED)

set(FIRMWARE-SOURCES
  ${DBB-FIRMWARE-SOURCES}
  ${DBB-FIRMWARE-USB-SOURCES}
  ${DBB-FIRMWARE-UI-SOURCES}
  ${FIRMWARE-DRIVER-SOURCES}
  ${DRIVER-SOURCES}
  ${DBB-FIRMWARE-PROTOBUF-SOURCES}
  ${CRYPTOAUTHLIB-SOURCES}
  ${ETHEREUM-SOURCES}
  ${CMAKE_SOURCE_DIR}/src/common_main.c
)

set(BOOTLOADER-SOURCES
  ${DBB-BOOTLOADER-SOURCES}
  ${DBB-FIRMWARE-USB-SOURCES}
  ${DRIVER-SOURCES}
)

set_source_files_properties(
  ${DBB-FIRMWARE-PROTOBUF-SOURCES}
  PROPERTIES GENERATED TRUE
)

if(CMAKE_CROSSCOMPILING)
  set(STACK_SIZE "0x10000" CACHE STRING "Specify stack size for bootloader/firmware")
  set(STACK_SIZE ${STACK_SIZE} PARENT_SCOPE)
  set(HEAP_SIZE "0x18000" CACHE STRING "Specify heap size for bootloader/firmware")
  set(HEAP_SIZE ${HEAP_SIZE} PARENT_SCOPE)

  set(BOOTLOADERS bootloader bootloader-development bootloader-development-locked bootloader-semihosting bootloader-production bootloader-btc bootloader-btc-development bootloader-btc-production)
  set(BOOTLOADERS ${BOOTLOADERS} PARENT_SCOPE)
  foreach(bootloader ${BOOTLOADERS})
    set(elf ${bootloader}.elf)
    add_executable(${elf} ${BOOTLOADER-SOURCES})
    target_link_libraries(${elf} PRIVATE c asf4-drivers-min -Wl,-u,exception_table)
    target_include_directories(${elf} PRIVATE ${INCLUDES})
    target_include_directories(${elf} SYSTEM PRIVATE ${SYSTEMINCLUDES})
    target_compile_definitions(${elf} PRIVATE BOOTLOADER)
    # needed to find version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    # needed to find bootloader_version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/bootloader)
    target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${bootloader}.map\" -T\"${CMAKE_SOURCE_DIR}/bootloader.ld\"")
    target_link_libraries(${elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})
    target_link_libraries(${elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})
    # Select the smaller version of libc called nano.
    target_compile_options(${elf} PRIVATE --specs=nano.specs)
    target_link_libraries(${elf} PRIVATE --specs=nano.specs)
    if(NOT bootloader STREQUAL "bootloader-semihosting")
      target_compile_options(${elf} PRIVATE --specs=nosys.specs)
      target_link_libraries(${elf} PRIVATE --specs=nosys.specs)
    endif()
  endforeach(bootloader)

  target_compile_definitions(bootloader-development.elf PRIVATE BOOTLOADER_DEVDEVICE)
  target_compile_definitions(bootloader-development-locked.elf PRIVATE BOOTLOADER_DEVDEVICE BOOTLOADER_PRODUCTION)
  set_property(TARGET bootloader-development-locked.elf PROPERTY EXCLUDE_FROM_ALL ON)

  # Select an implementation of the system calls that can communicate with the debugger
  target_compile_options(bootloader-semihosting.elf PRIVATE --specs=rdimon.specs)
  target_link_libraries(bootloader-semihosting.elf PRIVATE --specs=rdimon.specs)
  target_compile_definitions(bootloader-semihosting.elf PRIVATE BOOTLOADER_DEVDEVICE SEMIHOSTING)

  target_compile_definitions(bootloader-production.elf PRIVATE BOOTLOADER_PRODUCTION)
  set_property(TARGET bootloader-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  target_compile_definitions(bootloader-btc.elf PRIVATE BOOTLOADER_BTC_ONLY)
  target_compile_definitions(bootloader-btc-development.elf PRIVATE BOOTLOADER_DEVDEVICE BOOTLOADER_BTC_ONLY)
  target_compile_definitions(bootloader-btc-production.elf PRIVATE BOOTLOADER_PRODUCTION BOOTLOADER_BTC_ONLY)
  set_property(TARGET bootloader-btc-production.elf PROPERTY EXCLUDE_FROM_ALL ON)

  set(FIRMWARES firmware firmware-semihosting firmware-btc factory-setup)
  set(FIRMWARES ${FIRMWARES} PARENT_SCOPE)
  foreach(firmware ${FIRMWARES})
    set(elf ${firmware}.elf)
    add_executable(${elf} ${FIRMWARE-SOURCES})
    target_link_libraries(${elf}
      PRIVATE
        cryptoauthlib
        fatfs
        base32
        bignum # TODO: only eth
        sha3   # TODO: Only eth
        ctaes
        c
        samd51a-ds
        asf4-drivers-min
        asf4-drivers
        -Wl,-u,exception_table)
    add_dependencies(${elf} generate-protobufs)
    target_include_directories(${elf} PRIVATE ${INCLUDES})
    target_include_directories(${elf} SYSTEM PRIVATE ${SYSTEMINCLUDES})

    # Include and provide configuration for nanopb
    target_include_directories(${elf} SYSTEM PRIVATE ${NANOPB_INCLUDE_DIRS})
    target_compile_definitions(${elf} PRIVATE "PB_NO_PACKED_STRUCTS=1" "PB_FIELD_16BIT=1")

    # needed to find version.h
    target_include_directories(${elf} PRIVATE ${CMAKE_CURRENT_BINARY_DIR})
    target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${firmware}.map\" -T\"${CMAKE_SOURCE_DIR}/firmware.ld\"")
    target_link_libraries(${elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

    add_dependencies(${elf} noise-c)
    target_link_libraries(${elf} PRIVATE noiseprotocol)

    add_dependencies(${elf} libwally-core)
    target_link_libraries(${elf} PRIVATE wallycore secp256k1)
    target_link_libraries(${elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})
    # Must manually link against C so that malloc can find _sbrk

    # Select the smaller version of libc called nano.
    target_compile_options(${elf} PRIVATE --specs=nano.specs)
    target_link_libraries(${elf} PRIVATE --specs=nano.specs)
    if(NOT firmware STREQUAL "firmware-semihosting")
      target_compile_options(${elf} PRIVATE --specs=nosys.specs)
      target_link_libraries(${elf} PRIVATE --specs=nosys.specs)
    endif()
  endforeach(firmware)

  target_sources(firmware.elf PRIVATE firmware.c)
  target_compile_definitions(firmware.elf PRIVATE APP_BTC APP_LTC APP_ETH APP_U2F)
  target_sources(firmware.elf PRIVATE ${FIRMWARE-U2F-SOURCES})

  target_sources(firmware-semihosting.elf PRIVATE firmware.c)
  target_sources(firmware-semihosting.elf PRIVATE ${FIRMWARE-U2F-SOURCES})
  # Select an implementation of the system calls that can communicate with the debugger
  target_compile_options(firmware-semihosting.elf PRIVATE --specs=rdimon.specs)
  target_link_libraries(firmware-semihosting.elf PRIVATE --specs=rdimon.specs)
  target_compile_definitions(firmware-semihosting.elf PRIVATE APP_BTC APP_LTC APP_ETH APP_U2F)
  target_compile_definitions(firmware-semihosting.elf PRIVATE SEMIHOSTING)

  target_sources(firmware-btc.elf PRIVATE firmware.c)
  target_compile_definitions(firmware-btc.elf PRIVATE FIRMWARE_BTC_ONLY APP_BTC)

  target_sources(factory-setup.elf PRIVATE factorysetup.c)
  target_compile_definitions(factory-setup.elf PRIVATE FACTORYSETUP)

  foreach(name ${BOOTLOADERS} ${FIRMWARES})
    add_custom_command(
      TARGET ${name}.elf POST_BUILD
      COMMAND ${CMAKE_SIZE} ${name}.elf
      COMMAND ${CMAKE_OBJCOPY} -O binary ${name}.elf ${name}.bin
      WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
      COMMENT "\nGenerating binary ${name}.bin"
    )
    # Fail on warnings for our builds
    target_compile_options(${name}.elf PRIVATE "-Werror")
  endforeach()
endif()
