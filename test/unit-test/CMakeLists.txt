# Copyright 2018 Shift Cryptosecurity AG
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

# No linker for Mach-O supports the linker argument `--wrap`. Since we use
# that, unit tests will never work on macos. Use linux/arm64 in docker instead.

# We use FindPkgConfig instead of FindPackage because it finds libraries in
# both linux and macos
find_package(PkgConfig REQUIRED)

# Unit testing uses CMocka
pkg_check_modules(CMOCKA REQUIRED cmocka)
# u2f tests with hardware uses hidapi-hidraw
if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
  pkg_check_modules(HIDAPI REQUIRED hidapi)
else()
  pkg_check_modules(HIDAPI REQUIRED hidapi-hidraw)
endif()

#-----------------------------------------------------------------------------
# Build bitbox lib to use in tests

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-parameter -Wno-missing-prototypes -Wno-missing-declarations -Wno-implicit-function-declaration -Wno-bad-function-cast")

set(DBB-FILTERED-SOURCES
  ${DBB-FIRMWARE-SOURCES}
  ${DBB-FIRMWARE-UI-SOURCES}
  ${FIRMWARE-U2F-SOURCES}
  ${DBB-FIRMWARE-USB-SOURCES}
  ${PLATFORM-BITBOX02-PLUS-SOURCES}
  ${CMAKE_SOURCE_DIR}/external/asf4-drivers/hal/utils/src/utils_ringbuffer.c
  )

set(IGNORE_SOURCES
    "src/screen.c"
    "src/firmware_main_loop.c"
    "src/memory/nvmctrl.c"
    "src/memory/smarteeprom.c"
    "src/memory/mpu.c"
    "src/memory/spi_mem.c"
    "src/uart.c"
    )

# Exclude some files which depends on the hardware.
foreach(SOURCEFILE ${IGNORE_SOURCES})
    list(FILTER DBB-FILTERED-SOURCES EXCLUDE REGEX ".*/${SOURCEFILE}$")
endforeach()

add_library(sd-mock
  STATIC
  framework/mock_diskio.c
)
target_include_directories(
  sd-mock
  PUBLIC
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)


# We create a CMake "object library" to track all the compiled sources so that
# they can be reused between a normal library and the manually crafted "merged"
# library.
# See https://cmake.org/cmake/help/v3.10/command/add_library.html#object-libraries
add_library(bitbox_objects
  OBJECT
  ${DBB-FILTERED-SOURCES}
  ${ETHEREUM-SOURCES}
  framework/mock_cipher.c
  framework/mock_screen.c
  framework/mock_screen_stack.c
  framework/mock_memory.c
  framework/mock_spi_mem.c
  framework/mock_qtouch.c
  framework/mock_gestures.c
  framework/mock_component.c
  framework/mock_smarteeprom.c
  framework/mock_securechip.c
  framework/mock_delay.c
)

add_library(bitbox
  STATIC
  $<TARGET_OBJECTS:bitbox_objects>
)

# Here we create the "merged" library, which starts with the c-lib created by
# cargo in the "bitbox02-rust-c" project. That project produces a c-lib without
# mangled symbols so that it is easier to use from C. We then extend that
# library with all the object files declared in "bitbox_objects", all the code
# we want to be able to call from tests.
# By linking to "bitbox_merged" in the rust unit tests we get access to all our
# code. (That linkage is done by cargo using a "build.rs" script in the
# bitbox02 rust project.)
add_custom_command(
  OUTPUT ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged.a
  DEPENDS c-unit-tests_rust_c bitbox_objects $<TARGET_OBJECTS:bitbox_objects>
  COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_PROPERTY:c-unit-tests_rust_c,IMPORTED_LOCATION> ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged.a
  COMMAND ar q ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged.a $<TARGET_OBJECTS:bitbox_objects>
  VERBATIM
  COMMAND_EXPAND_LISTS
)

add_custom_target(bitbox_merged DEPENDS ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged.a)

target_include_directories(
  bitbox_objects
  SYSTEM PUBLIC
  ${CMAKE_SOURCE_DIR}/external
  ${CMAKE_SOURCE_DIR}/external/asf4-drivers/hal/utils/include
  ${CMAKE_SOURCE_DIR}/external/fatfs/source
  $<TARGET_PROPERTY:wallycore,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:secp256k1,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)
target_include_directories(
  bitbox_objects
  PUBLIC
  ${INCLUDES}
  ${CMAKE_CURRENT_SOURCE_DIR}/framework/includes
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_BINARY_DIR}/src
  ${CMOCKA_INCLUDE_DIRS}
)

target_include_directories(
  bitbox
  SYSTEM PUBLIC
  ${CMAKE_SOURCE_DIR}/external
  ${CMAKE_SOURCE_DIR}/external/asf4-drivers/hal/utils/include
  ${CMAKE_SOURCE_DIR}/external/fatfs/source
  $<TARGET_PROPERTY:wallycore,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:secp256k1,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)

target_include_directories(
  bitbox
  PUBLIC
  ${INCLUDES}
  ${CMAKE_CURRENT_SOURCE_DIR}/framework/includes
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_BINARY_DIR}/src
)

add_dependencies(bitbox_objects
  rust-cbindgen
  wallycore
  secp256k1
  fatfs
  sd-mock
)
target_link_libraries(bitbox PRIVATE ${LIBBITBOX02_RUST} "-lm")

# _UNIT_TEST_ is used by ASF4 to not cross compile

target_compile_definitions(bitbox_objects PUBLIC "PRODUCT_BITBOX_MULTI=1" "APP_BTC=1" "APP_LTC=1" "APP_U2F=1" "APP_ETH=1")
target_compile_definitions(bitbox_objects PUBLIC TESTING _UNIT_TEST_)

# Since wallycore is an external projects we need to specify the dependency
add_dependencies(bitbox_objects libwally-core)

target_compile_definitions(bitbox PUBLIC "PRODUCT_BITBOX_MULTI=1" "APP_BTC=1" "APP_LTC=1" "APP_U2F=1" "APP_ETH=1")
target_compile_definitions(bitbox PUBLIC TESTING _UNIT_TEST_)

target_link_libraries(bitbox
  PUBLIC
    secp256k1
    ${CMOCKA_LDFLAGS}
  PRIVATE
    wallycore
    fatfs
    sd-mock
)

if(SANITIZE_ADDRESS)
  target_compile_options(bitbox_objects PUBLIC "-fsanitize=address")
  target_compile_options(bitbox PUBLIC "-fsanitize=address")
endif()
if(SANTIZE_UNDEFINED)
  target_compile_options(bitbox_objects PUBLIC "-fsanitize=undefined")
  target_compile_options(bitbox PUBLIC "-fsanitize=undefined")
endif()
if(COVERAGE)
  target_link_libraries(bitbox PUBLIC "--coverage")
  target_compile_options(bitbox_objects PUBLIC "--coverage")
  target_compile_options(bitbox PUBLIC "--coverage")
endif()

add_library(u2f-util
  STATIC
  u2f/sha2.c
  u2f/ecc.c
  u2f/uECC.c
  u2f/u2f_util_t.c
)
target_include_directories(
  u2f-util
  SYSTEM PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${HIDAPI_INCLUDE_DIRS}
)
target_include_directories(
  u2f-util
  PUBLIC
  ${INCLUDES}
  ${CMAKE_BINARY_DIR}/src
)
target_compile_definitions(u2f-util PUBLIC "TESTING" _UNIT_TEST_ PRODUCT_BITBOX_MULTI "APP_U2F=1" "APP_BTC=1" "APP_LTC=1" "APP_ETH=1")
target_compile_definitions(u2f-util PUBLIC "USE_KECCAK")

target_link_libraries(u2f-util PUBLIC ${HIDAPI_LDFLAGS})


#-----------------------------------------------------------------------------
# Tests

set(TEST_LIST
   cleanup
   "-Wl,--wrap=util_cleanup_32"
   keystore_antiklepto
   ""
   gestures
   ""
   random
   "-Wl,--wrap=rand,--wrap=rust_sha256"
   ui_components
   ""
   ui_util
   ""
   ui_component_gestures
   ""
   memory
   "-Wl,--wrap=memory_read_chunk_mock,--wrap=memory_write_chunk_mock,--wrap=rust_noise_generate_static_private_key,--wrap=memory_read_shared_bootdata_mock,--wrap=memory_write_to_address_mock,--wrap=random_32_bytes_mcu"
   memory_functional
   ""
   salt
   "-Wl,--wrap=memory_get_salt_root"
   cipher
   "-Wl,--wrap=cipher_mock_iv"
   util
   ""
   ugui
   ""
)

if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
  message("No linker for Mach-O supports --wrap, will not generate unit-tests")
else()
  list(LENGTH TEST_LIST TEST_LIST_LEN)
  math(EXPR TEST_LIST_LEN ${TEST_LIST_LEN}-1)
  foreach(I RANGE 0 ${TEST_LIST_LEN} 2)
    math(EXPR I2 ${I}+1)
    list(GET TEST_LIST ${I} TEST_NAME)
    list(GET TEST_LIST ${I2} TEST_LINK_ARGS)
    set(EXE test_${TEST_NAME})
    add_executable(${EXE} test_${TEST_NAME}.c framework/eh_personality.c)
    # asan must be first library in linking order
    target_link_libraries(${EXE} PRIVATE
      $<$<BOOL:${SANITIZE_ADDRESS}>:asan>
      $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
      -Wl,--start-group
      c-unit-tests_rust_c
      bitbox
      -Wl,--end-group
      ${TEST_LINK_ARGS}
    )
    if(NOT ${TEST_NAME} STREQUAL "simulator")
        add_test(NAME test_${TEST_NAME} COMMAND ${EXE})
      endif()
  endforeach()
endif()


# These unit tests for U2F are special because they don't call any bitbox functions directly, instead they go through hid_read/write.
# They are copied from https://github.com/google/u2f-ref-code/tree/master/u2f-tests/HID
set(U2F_TESTS
   u2f_standard
   u2f_hid
)
list(LENGTH U2F_TESTS U2F_TESTS_LEN)
foreach(TEST_NAME ${U2F_TESTS})
  set(EXE test_${TEST_NAME})

  # This tests link to our code
  add_executable(${EXE} test_${TEST_NAME}.c framework/mock_hidapi.c framework/eh_personality.c)
  target_link_libraries(${EXE} PRIVATE
    $<$<BOOL:${SANITIZE_ADDRESS}>:asan>
    $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--start-group>
    c-unit-tests_rust_c
    bitbox
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--end-group>
    u2f-util
  )
  target_compile_definitions(${EXE} PRIVATE "TESTING")
  add_test(NAME test_${TEST_NAME} COMMAND ${EXE})

  # Running these two tests require a hardware device connected and they link to the real libhidapi.
  set(EXE test_${TEST_NAME}_with_hardware)
  add_executable(${EXE} test_${TEST_NAME}.c)
  # asan must be first library in linking order
  target_link_libraries(${EXE} PRIVATE
    $<$<BOOL:${SANITIZE_ADDRESS}>:asan>
    $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
    u2f-util
  )
  target_compile_definitions(${EXE} PRIVATE "TESTING" "WITH_HARDWARE")
endforeach()
