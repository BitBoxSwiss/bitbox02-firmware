# Copyright 2024 Shift Cryptosecurity AG
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
# Build bitbox lib to use in tests

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-parameter -Wno-missing-prototypes -Wno-missing-declarations -Wno-implicit-function-declaration -Wno-bad-function-cast")

set(DBB-FILTERED-SOURCES
  ${DBB-FIRMWARE-SOURCES}
  ${DBB-FIRMWARE-UI-SOURCES}
  ${FIRMWARE-U2F-SOURCES}
  ${DBB-FIRMWARE-USB-SOURCES}
  )

set(IGNORE_SOURCES
    "src/screen.c"
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

message("FILTERED SOURCES: ${DBB-FILTERED-SOURCES}")

add_library(sd-mock-simulator
  STATIC
  framework/mock_diskio.c
)
target_include_directories(
  sd-mock-simulator
  PUBLIC
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)


# We create a CMake "object library" to track all the compiled sources so that
# they can be reused between a normal library and the manually crafted "merged"
# library.
# See https://cmake.org/cmake/help/v3.10/command/add_library.html#object-libraries
add_library(bitbox_objects-simulator
  OBJECT
  ${DBB-FILTERED-SOURCES}
  ${CTAES-SOURCES}
  ${ETHEREUM-SOURCES}
  framework/mock_cipher.c
  framework/mock_memory.c
  framework/mock_spi_mem.c
  framework/mock_screen.c
  framework/mock_smarteeprom.c
  framework/mock_securechip.c
)

add_library(bitbox-simulator
  STATIC
  $<TARGET_OBJECTS:bitbox_objects-simulator>
)

# Here we create the "merged" library, which starts with the c-lib created by
# cargo in the "bitbox02-rust-c" project. That project produces a c-lib without
# mangled symbols so that it is easier to use from C. We then extend that
# library with all the object files declared in "bitbox_objects-simulator", all the code
# we want to be able to call from tests.
# By linking to "bitbox_merged-simulator" in the rust unit tests we get access to all our
# code. (That linkage is done by cargo using a "build.rs" script in the
# bitbox02 rust project.)
add_custom_command(
  OUTPUT ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged-simulator.a
  DEPENDS c-unit-tests_rust_c bitbox_objects-simulator $<TARGET_OBJECTS:bitbox_objects-simulator>
  COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_PROPERTY:c-unit-tests_rust_c,IMPORTED_LOCATION> ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged-simulator.a
  COMMAND ar q ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged-simulator.a $<TARGET_OBJECTS:bitbox_objects-simulator>
  VERBATIM
  COMMAND_EXPAND_LISTS
)

add_custom_target(bitbox_merged-simulator DEPENDS ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libbitbox_merged-simulator.a)

target_include_directories(
  bitbox_objects-simulator
  SYSTEM PUBLIC
  ${CMAKE_SOURCE_DIR}/external
  ${CMAKE_SOURCE_DIR}/external/asf4-drivers/hal/utils/include
  ${CMAKE_SOURCE_DIR}/external/ctaes
  ${CMAKE_SOURCE_DIR}/external/fatfs/source
  $<TARGET_PROPERTY:wallycore,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:secp256k1,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)
target_include_directories(
  bitbox_objects-simulator
  PUBLIC
  ${INCLUDES}
  ${CMAKE_CURRENT_SOURCE_DIR}/../unit-test/framework/includes
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_BINARY_DIR}/src
)

target_include_directories(
  bitbox-simulator
  SYSTEM PUBLIC
  ${CMAKE_SOURCE_DIR}/external
  ${CMAKE_SOURCE_DIR}/external/asf4-drivers/hal/utils/include
  ${CMAKE_SOURCE_DIR}/external/ctaes
  ${CMAKE_SOURCE_DIR}/external/fatfs/source
  $<TARGET_PROPERTY:wallycore,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:secp256k1,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)

target_include_directories(
  bitbox-simulator
  PUBLIC
  ${INCLUDES}
  ${CMAKE_CURRENT_SOURCE_DIR}/../unit-test/framework/includes
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_BINARY_DIR}/src
)

add_dependencies(bitbox_objects-simulator
  rust-cbindgen
  wallycore
  secp256k1
  ctaes
  fatfs
  sd-mock-simulator
)
target_link_libraries(bitbox-simulator PRIVATE ${LIBBITBOX02_RUST} "-lm")

# _UNIT_TEST_ is used by ASF4 to not cross compile

target_compile_definitions(bitbox_objects-simulator PUBLIC "PRODUCT_BITBOX_MULTI=1" "APP_BTC=1" "APP_LTC=1" "APP_U2F=1" "APP_ETH=1")
target_compile_definitions(bitbox_objects-simulator PUBLIC TESTING _UNIT_TEST_)

# Since wallycore is an external projects we need to specify the dependency
add_dependencies(bitbox_objects-simulator libwally-core)

target_compile_definitions(bitbox-simulator PUBLIC "PRODUCT_BITBOX_MULTI=1" "APP_BTC=1" "APP_LTC=1" "APP_U2F=1" "APP_ETH=1")
target_compile_definitions(bitbox-simulator PUBLIC TESTING _UNIT_TEST_)

target_link_libraries(bitbox-simulator
  PUBLIC
    secp256k1
  PRIVATE
    wallycore
    ctaes
    fatfs
    sd-mock-simulator
)

if(SANITIZE_ADDRESS)
  target_compile_options(bitbox_objects-simulator PUBLIC "-fsanitize=address")
  target_compile_options(bitbox-simulator PUBLIC "-fsanitize=address")
endif()
if(SANTIZE_UNDEFINED)
  target_compile_options(bitbox_objects-simulator PUBLIC "-fsanitize=undefined")
  target_compile_options(bitbox-simulator PUBLIC "-fsanitize=undefined")
endif()

#-----------------------------------------------------------------------------
# Simulator

add_executable(simulator simulator.c framework/eh_personality.c)
# asan must be first library in linking order
target_link_libraries(simulator PRIVATE
  $<$<BOOL:${SANITIZE_ADDRESS}>:asan>
  $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
  $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--start-group>
  c-unit-tests_rust_c
  bitbox-simulator
  $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--end-group>
  ""
)

add_custom_command(
  TARGET simulator POST_BUILD
  COMMAND ${CMAKE_STRIP} simulator
  WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
  COMMENT "\nStripping simulator binary"
)
