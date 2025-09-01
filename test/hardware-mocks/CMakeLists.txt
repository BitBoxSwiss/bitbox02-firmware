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

set(HARDWARE-MOCKS-SOURCES
  src/mock_cipher.c
  src/mock_diskio.c
  src/mock_memory.c
  src/mock_screen.c
  src/mock_securechip.c
  src/mock_smarteeprom.c
  src/mock_spi_mem.c
  src/mock_component.c
  src/mock_delay.c
  src/mock_qtouch.c
)

add_library(hardware-mocks
  STATIC
  ${HARDWARE-MOCKS-SOURCES}
)

add_dependencies(hardware-mocks rust-cbindgen)

target_compile_definitions(hardware-mocks PUBLIC
    TESTING
    _UNIT_TEST_
    PRODUCT_BITBOX_MULTI=1
    APP_BTC=1
    APP_LTC=1
    APP_U2F=1
    APP_ETH=1
)

target_include_directories(
  hardware-mocks
  PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
target_include_directories(
  hardware-mocks
  PRIVATE
  ${INCLUDES}
  ${CMAKE_BINARY_DIR}/src
  $<TARGET_PROPERTY:fatfs,INTERFACE_INCLUDE_DIRECTORIES>
  $<TARGET_PROPERTY:wallycore,INTERFACE_INCLUDE_DIRECTORIES>
)

target_link_libraries(hardware-mocks PUBLIC wallycore)

if(SANITIZE_ADDRESS)
  target_compile_options(hardware-mocks PUBLIC "-fsanitize=address")
endif()
if(SANTIZE_UNDEFINED)
  target_compile_options(hardware-mocks PUBLIC "-fsanitize=undefined")
endif()
