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

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-parameter -Wno-missing-prototypes -Wno-missing-declarations -Wno-implicit-function-declaration -Wno-bad-function-cast")

set(HARDWARE-FAKES-SOURCES
  src/fake_cipher.c
  src/fake_diskio.c
  src/fake_memory.c
  src/fake_screen.c
  src/fake_securechip.c
  src/fake_smarteeprom.c
  src/fake_spi_mem.c
  src/fake_component.c
  src/fake_delay.c
  src/fake_qtouch.c
)

add_library(hardware-fakes
  STATIC
  ${HARDWARE-FAKES-SOURCES}
)

add_dependencies(hardware-fakes rust-cbindgen)

target_compile_definitions(hardware-fakes PUBLIC
    TESTING
    _UNIT_TEST_
    PRODUCT_BITBOX_MULTI=1
    APP_BTC=1
    APP_LTC=1
    APP_U2F=1
    APP_ETH=1
)

target_include_directories(
  hardware-fakes
  PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
target_include_directories(
  hardware-fakes
  PRIVATE
  ${INCLUDES}
  ${CMAKE_BINARY_DIR}/src
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
)

if(SANITIZE_ADDRESS)
  target_compile_options(hardware-fakes PUBLIC "-fsanitize=address")
endif()
if(SANTIZE_UNDEFINED)
  target_compile_options(hardware-fakes PUBLIC "-fsanitize=undefined")
endif()
