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


#-----------------------------------------------------------------------------
# Source files

#include(${CMAKE_SOURCE_DIR}/src/CMakeLists.txt)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/tools/nanopb/extra)
find_package(Nanopb REQUIRED)
include_directories(${NANOPB_INCLUDE_DIRS})

set(SOURCES
  ${DBB-FIRMWARE-SOURCES}
  ${DBB-FIRMWARE-USB-SOURCES}
  ${DBB-FIRMWARE-UI-SOURCES}
  ${FIRMWARE-DRIVER-SOURCES}
  ${DRIVER-SOURCES}
  ${DBB-FIRMWARE-PROTOBUF-SOURCES}
  ${CRYPTOAUTHLIB-SOURCES}
  ${CTAES-SOURCES}
  ${ETHEREUM-SOURCES}
  ${FIRMWARE-U2F-SOURCES}
  ${CMAKE_SOURCE_DIR}/src/common_main.c
  ${CMAKE_CURRENT_SOURCE_DIR}/src/framework/test_common.c
)

# CMake is a bit retarted and doesn't keep some properties between directories...
set_property(SOURCE ${CMAKE_BINARY_DIR}/src/backup.pb.c PROPERTY GENERATED 1)
set_property(SOURCE ${CMAKE_BINARY_DIR}/src/hww.pb.c PROPERTY GENERATED 1)

set(INCLUDES
    include/
    ${FIRMWARE-INCLUDES}
    ${INCLUDES}
    ${CMAKE_BINARY_DIR}/src
)

add_library(bitbox
  STATIC
  ${SOURCES}
)

target_link_libraries(bitbox PUBLIC asf4-drivers)

target_include_directories(bitbox PUBLIC ${INCLUDES})
target_include_directories(bitbox SYSTEM PUBLIC ${SYSTEMINCLUDES})
target_include_directories(bitbox SYSTEM PUBLIC ${NANOPB_INCLUDE_DIRS})
target_compile_definitions(bitbox PUBLIC "PB_NO_PACKED_STRUCTS=1" "PB_FIELD_16BIT=1")
target_compile_definitions(bitbox PUBLIC APP_BTC APP_LTC APP_U2F APP_ETH)
add_dependencies(bitbox generate-hww generate-backup)
target_include_directories(bitbox SYSTEM PRIVATE ${CMAKE_SOURCE_DIR}/external)
target_include_directories(bitbox SYSTEM PRIVATE ${CMAKE_SOURCE_DIR}/external/ctaes)
target_include_directories(bitbox SYSTEM PRIVATE ${CMAKE_SOURCE_DIR}/external/sha3)
# enable KECCAK in sha3
target_compile_definitions(bitbox PUBLIC "USE_KECCAK")
# needed to find version.h
target_include_directories(bitbox SYSTEM PUBLIC ${CMAKE_BINARY_DIR}/src)

target_link_libraries(bitbox PRIVATE noiseprotocol secp256k1 wallycore cryptoauthlib)

# Always enable semihosting for the test firmwares
target_compile_definitions(bitbox PUBLIC SEMIHOSTING)
target_link_libraries(bitbox PUBLIC --specs=nano.specs --specs=rdimon.specs)
target_compile_options(bitbox PUBLIC --specs=nano.specs --specs=rdimon.specs)

set(TEST_LIST
  all_variants_menu
  backup
  bip39
  button_tap
  confirm_bip39
  entry_screen
  password
  scroll_menu
  scroll_menu_2
  simple_slide
  slide_info
  ssp
  tap_menu
  trace
  usb_cmd_process
  usb_composed_ep_in
  usb_composed_ep_out
  usb_d_init
  usb_hww_ep_in
  usb_hww_ep_out
  usb_u2f_ep_in
  usb_u2f_ep_out
  usbdc_init
  write_sd
)

add_custom_target(device-tests)

foreach(name ${TEST_LIST})
  set(elf fw_test_${name}.elf)
  add_executable(${elf} ${CMAKE_CURRENT_SOURCE_DIR}/src/test_${name}.c)

  target_link_libraries(${elf} PRIVATE "-Wl,-Map=\"${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/test_${name}.map\" -T\"${CMAKE_CURRENT_SOURCE_DIR}/device-test.ld\"")
  target_link_libraries(${elf} PRIVATE -Wl,--defsym=STACK_SIZE=${STACK_SIZE} -Wl,-defsym=HEAP_SIZE=${HEAP_SIZE})

  # Since noiseprotocol and wallycore are external projects we need to specify the dependency
  add_dependencies(${elf} noise-c)
  add_dependencies(${elf} libwally-core)
  # We must link against libc before libbitbox because malloc depends on _sbrk
  # We must mark "exception_table" as used, otherwise it won't be linked in since it is in a static library
  target_link_libraries(${elf} PRIVATE c bitbox asf4-drivers asf4-drivers-min -Wl,-u,exception_table)
  target_link_libraries(${elf} PRIVATE noiseprotocol)
  target_link_libraries(${elf} PRIVATE wallycore secp256k1)
  target_link_libraries(${elf} PRIVATE ${QTOUCHLIB_A} ${QTOUCHLIB_B} ${QTOUCHLIB_T})

  add_custom_command(
    TARGET ${elf} POST_BUILD
    COMMAND ${CMAKE_SIZE} ${elf}
    COMMAND ${CMAKE_OBJCOPY} -O binary ${elf} fw_test_${name}.bin
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nGenerating binary fw_test_${name}.bin"
  )
  if(PYTHONINTERP_FOUND)
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_SOURCE_DIR}/scripts/expand_template ${CMAKE_SOURCE_DIR}/scripts/template-firmware.jlink file=build/bin/fw_test_${name}.bin -o fw_test_${name}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  add_dependencies(device-tests ${elf})
  endif()
endforeach()
