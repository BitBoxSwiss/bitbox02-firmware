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
    ${DBB-FIRMWARE-PROTOBUF-SOURCES}
    ${DBB-FIRMWARE-UI-SOURCES}
    ${CRYPTOAUTHLIB-SOURCES}
    ${CTAES-SOURCES}
    ${ETHEREUM-SOURCES}
    ${FIRMWARE-DRIVER-SOURCES}
    ${DRIVER-SOURCES}
    src/framework/test_common.c
    ${MAIN-SOURCE}
)

set(INCLUDES
    include/
    ${FIRMWARE-INCLUDES}
    ${INCLUDES}
    ${CMAKE_BINARY_DIR}/src
)

# In our test cases, we disregard the bootloader since we flash with JSEGGER JLinkExe
set(CMAKE_LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/test/device-test/device-test.ld")

set(CMAKE_C_LINK_FLAGS "-mthumb -Wl,-Map=\"../../bin/${MYPROJECT}.map\" --specs=nosys.specs --specs=nano.specs -Wl,--start-group -u _printf_float -lm  -Wl,--end-group -Wl,--gc-sections -mcpu=cortex-m4 -T\"${CMAKE_LINKER_SCRIPT}\"")

message(STATUS "C link flags:     ${CMAKE_C_LINK_FLAGS}\n")
include_directories(SYSTEM ${SYSTEMINCLUDES})
include_directories(${INCLUDES})
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/cryptoauthlib/lib)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/noise-c/include)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/noise-c/src)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/ctaes)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/sha3)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/libwally-core/include)
include_directories(SYSTEM ${CMAKE_SOURCE_DIR}/external/libwally-core/src/secp256k1/include)

add_library(qtouchlib_a STATIC IMPORTED)
set_property(TARGET qtouchlib_a PROPERTY
            IMPORTED_LOCATION
            ${CMAKE_SOURCE_DIR}/src/drivers/qtouch/lib/gcc/libqtm_acq_samd51_0x000f.a
            )
add_library(qtouchlib_b STATIC IMPORTED)
set_property(TARGET qtouchlib_b PROPERTY
            IMPORTED_LOCATION
            ${CMAKE_SOURCE_DIR}/src/drivers/qtouch/lib/gcc/libqtm_binding_layer_cm4_0x0005.a
            )
add_library(qtouchlib_t STATIC IMPORTED)
set_property(TARGET qtouchlib_t PROPERTY
            IMPORTED_LOCATION
            ${CMAKE_SOURCE_DIR}/src/drivers/qtouch/lib/gcc/libqtm_touch_key_cm4_0x0002.a
            )

link_directories(${CMAKE_SOURCE_DIR}/src/drivers/qtouch/lib/gcc)

link_directories(${CMAKE_SOURCE_DIR}/src/drivers/lib/ssp)

add_executable(${ELF} ${SOURCES})

add_dependencies(${ELF} noise-c)
target_link_libraries(${ELF} "${CMAKE_SOURCE_DIR}/external/noise-c-${TARGET_CODE}-build/libnoiseprotocol.a")
add_dependencies(${ELF} libwally-core)
target_link_libraries(${ELF} "${CMAKE_SOURCE_DIR}/external/libwally-core-${TARGET_CODE}-build/.libs/libwallycore.a")
target_link_libraries(${ELF} "${CMAKE_SOURCE_DIR}/external/libwally-core-${TARGET_CODE}-build/secp256k1/.libs/libsecp256k1.a")

target_link_libraries(${ELF} qtouchlib_a qtouchlib_b qtouchlib_t)
