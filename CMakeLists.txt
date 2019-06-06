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


cmake_minimum_required(VERSION 2.8.5)

if(BUILD_TYPE STREQUAL "firmware")
elseif(BUILD_TYPE STREQUAL "bootloader")
elseif(BUILD_TYPE STREQUAL "unit-test")
    set(BUILD_TYPE "unit-test")
elseif(BUILD_TYPE STREQUAL "device-test")
    set(BUILD_TYPE "device-test")
elseif(BUILD_TYPE STREQUAL "factory-setup")
    set(BUILD_TYPE "factory-setup")
elseif(BUILD_TYPE STREQUAL "boot")
    set(BUILD_TYPE "bootloader")
else()
    set(BUILD_ERR TRUE)
endif()

if(BUILD_ERR)
    message(FATAL_ERROR "Invalid build type: ${BUILD_TYPE}. Build with:
    cmake .. -DBUILD_TYPE=type
    # 'type' can be 'unit-test', 'device-test', 'factory-setup', 'firmware', or 'bootloader'
    ")
endif()

if(BUILD_TYPE STREQUAL "firmware" OR BUILD_TYPE STREQUAL "factory-setup")
  set(BUILD_TYPE_FIRMWARE_OR_SETUP YES)
else()
  set(BUILD_TYPE_FIRMWARE_OR_SETUP NO)
endif()

set(MYPROJECT ${BUILD_TYPE})
project(${MYPROJECT} C)


#-----------------------------------------------------------------------------
# Options for building

option(BUILD_COVERAGE "Compile with test coverage flags." OFF)
option(BUILD_VALGRIND "Compile with debug symbols." OFF)
option(BUILD_DOCUMENTATION "Build the Doxygen documentation." OFF)
option(CMAKE_VERBOSE_MAKEFILE "Verbose build." OFF)
option(BOOTLOADER_DEVDEVICE "Builds a bootloader for devdevices." OFF)
option(BOOTLOADER_PRODUCTION "Builds a production bootloader, enabling boot protection and the security bit." OFF)

if(NOT BUILD_TYPE STREQUAL "unit-test")
    set(TARGET_CODE "arm")
    set(ELF ${MYPROJECT}.elf)
    set(CMAKE_TOOLCHAIN_FILE arm.cmake)
    include(${CMAKE_TOOLCHAIN_FILE})
else()
    set(TARGET_CODE "x86")
endif()

if(BUILD_TYPE STREQUAL "unit-test")
    set(BUILD_COVERAGE "ON")
    set(BUILD_VALGRIND "ON")
endif()

execute_process(COMMAND ${CMAKE_C_COMPILER} -dumpversion OUTPUT_VARIABLE C_COMPILER_VERSION)


#-----------------------------------------------------------------------------
# Force out-of-source build

if(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR})
message(FATAL_ERROR "CMake generation is not allowed within the source directory!
Remove the CMakeCache.txt file and try again from another folder, e.g.:

   rm CMakeCache.txt
   mkdir build
   cd build
   cmake .. -DBUILD_TYPE=type
")
endif()


#-----------------------------------------------------------------------------
# Create version header file

find_package(PythonInterp 3.6 REQUIRED)

find_package(Git)
if(GIT_FOUND AND PYTHONINTERP_FOUND)
    # TODO: --check-gpg cannot be used in container. Add our pubkeys to image?
    # For firmware version
    execute_process(
        COMMAND ${PYTHON_EXECUTABLE} ./scripts/get_version firmware --check-semver --check-gpg
        RESULT_VARIABLE exit_code
        OUTPUT_VARIABLE FIRMWARE_VERSION
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    if(NOT exit_code EQUAL "0")
        set(FIRMWARE_VERSION "v0.0.0")
    endif()
    execute_process(
        COMMAND ${GIT_EXECUTABLE} rev-parse HEAD
        RESULT_VARIABLE exit_code
        OUTPUT_VARIABLE GIT_COMMIT_HASH
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    if(NOT exit_code EQUAL "0")
        set(GIT_COMMIT_HASH "000000")
    endif()
    execute_process(
        COMMAND ${GIT_EXECUTABLE} rev-parse --short HEAD
        RESULT_VARIABLE exit_code
        OUTPUT_VARIABLE GIT_COMMIT_HASH_SHORT
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    if(NOT exit_code EQUAL "0")
        set(GIT_COMMIT_HASH_SHORT "000000")
    endif()

    # For bootloader version
    execute_process(
        COMMAND ${PYTHON_EXECUTABLE} ./scripts/get_version bootloader --check-semver --check-gpg
        RESULT_VARIABLE exit_code
        OUTPUT_VARIABLE BOOTLOADER_VERSION_STRING
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    if(NOT exit_code EQUAL "0")
        set(BOOTLOADER_VERSION_STRING "v0.0.0")
    endif()

    # Firmware
    string(REPLACE "." ";" VERSION_LIST ${FIRMWARE_VERSION})
    list(GET VERSION_LIST 0 vMAJOR)
    list(GET VERSION_LIST 1 MINOR)
    list(GET VERSION_LIST 2 PATCH)
    string(REPLACE "v" "" MAJOR ${vMAJOR})

    # Bootloader
    STRING(REGEX REPLACE "-.*$" "" BOOTLOADER_VERSION_STRING_SHORT ${BOOTLOADER_VERSION_STRING})
    STRING(REGEX REPLACE "\\+.*$" "" BOOTLOADER_VERSION_STRING_SHORT ${BOOTLOADER_VERSION_STRING_SHORT})
    string(LENGTH ${BOOTLOADER_VERSION_STRING_SHORT} BOOTLOADER_VERSION_STRING_SHORT_LEN)
    string(REGEX REPLACE "(.)" "'\\1', 0, " BOOTLOADER_VERSION_STRING_SHORT_W16 ${BOOTLOADER_VERSION_STRING_SHORT})
else()
    set(FIRMWARE_VERSION "v0.0.0")
    set(GIT_COMMIT_HASH "000000")
    set(GIT_COMMIT_HASH_SHORT "000000")
endif()
STRING(REGEX REPLACE "-.*$" "" PATCH ${PATCH})
STRING(REGEX REPLACE "\\+.*$" "" PATCH ${PATCH})
set(VERSION_STRING "${FIRMWARE_VERSION}")
set(VERSION_STRING_SHORT "v${MAJOR}.${MINOR}.${PATCH}")
set(GITHASH_STRING "${GIT_COMMIT_HASH}")
set(GITHASH_SHORT_STRING "${GIT_COMMIT_HASH_SHORT}")
string(REGEX REPLACE "(.)" "'\\1', 0, " VERSION_STRING_SHORT_W16 ${VERSION_STRING_SHORT})
string(LENGTH ${GITHASH_STRING} GITHASH_STRING_LEN)
string(LENGTH ${GITHASH_SHORT_STRING} GITHASH_SHORT_STRING_LEN)
string(LENGTH ${VERSION_STRING_SHORT} VERSION_STRING_SHORT_LEN)
set(VERSION_STRING_MAJOR ${MAJOR})
set(VERSION_STRING_MINOR ${MINOR})
set(VERSION_STRING_PATCH ${PATCH})

configure_file(src/version.h.in src/version.h)
configure_file(src/bootloader/bootloader_version.h.in src/bootloader/bootloader_version.h)


#-----------------------------------------------------------------------------
# Source Definitions

if(BOOTLOADER_DEVDEVICE)
  add_definitions(-DBOOTLOADER_DEVDEVICE)
endif()

if(BOOTLOADER_PRODUCTION)
  add_definitions(-DBOOTLOADER_PRODUCTION)
endif()


if(BUILD_TYPE STREQUAL "unit-test")
    add_definitions(-DTESTING)
elseif(BUILD_TYPE STREQUAL "bootloader")
    add_definitions(-DBOOTLOADER)
elseif(BUILD_TYPE STREQUAL "factory-setup")
    add_definitions(-DFACTORYSETUP)
endif()



#-----------------------------------------------------------------------------
# Always generate compile_commands.json

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)


#-----------------------------------------------------------------------------
# Print system information and build options

message(STATUS "\n\n=============================================")
message(STATUS "            - General -")
message(STATUS "MCU version:            ${VERSION_STRING}")
message(STATUS "CMake version:          ${CMAKE_VERSION}")
message(STATUS "System:                 ${CMAKE_SYSTEM}")
message(STATUS "Processor:              ${CMAKE_SYSTEM_PROCESSOR}")
message(STATUS "             - Build -")
message(STATUS "Compiler version:       ${CMAKE_C_COMPILER_ID} ${C_COMPILER_VERSION}")
message(STATUS "Compiler:               ${CMAKE_C_COMPILER}")
message(STATUS "Linker:                 ${CMAKE_LINKER}")
message(STATUS "Archiver:               ${CMAKE_AR}")
message(STATUS "            - Options -")
message(STATUS "Build type:             ${MYPROJECT}")
message(STATUS "Hardware version:       ${HARDWARE}")
message(STATUS "Verbose:                ${CMAKE_VERBOSE_MAKEFILE}")
message(STATUS "Documentation:          ${BUILD_DOCUMENTATION}  (make doc)")
message(STATUS "Coverage flags:         ${BUILD_COVERAGE}")
message(STATUS "Debug symbols:          ${BUILD_VALGRIND}")
message(STATUS "\n=============================================\n\n")


#-----------------------------------------------------------------------------
# Get submodules

execute_process(COMMAND git "submodule" "update" "--init" "--recursive" WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})


#-----------------------------------------------------------------------------
# Collect all binaries into bin subdirectory

set(EXECUTABLE_OUTPUT_PATH ${CMAKE_CURRENT_BINARY_DIR}/bin)
set(LIBRARY_OUTPUT_PATH  ${CMAKE_CURRENT_BINARY_DIR}/lib)


#-----------------------------------------------------------------------------
# Compiler
# more specific CFLAGS might be added in the subdirectories src/ and test/.

if(BUILD_VALGRIND)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O0 -std=c99 -g3")
elseif(BUILD_TYPE STREQUAL "device-test")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Og -std=c99 -ggdb")
else()
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Os -std=c99")
endif()

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Werror -W -Wall -Wextra -Wpedantic -Wmissing-prototypes -Werror-implicit-function-declaration -Wpointer-arith -Wchar-subscripts -Wcomment -Wformat=2 -Wimplicit-int -Wmain -Wparentheses -Wsequence-point -Wreturn-type -Wswitch -Wtrigraphs -Wunused -Wuninitialized -Wunknown-pragmas -Wfloat-equal -Wundef -Wshadow -Wbad-function-cast -Wwrite-strings -Wsign-compare -Wmissing-declarations -Wformat -Wmissing-format-attribute -Wpacked -Wredundant-decls -Wnested-externs -Wmultichar -Wformat-nonliteral -Winit-self -Wformat-security -Wold-style-definition -Wmissing-include-dirs -Wswitch-default -Wattributes -Wdeprecated-declarations -Wcast-qual -Wstrict-prototypes -Wno-cast-function-type")

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -ffunction-sections -fdata-sections -fstack-protector-all")

if(NOT BUILD_TYPE STREQUAL "bootloader")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS}  -DPB_NO_PACKED_STRUCTS=1 -DPB_FIELD_16BIT=1")
endif()

if(NOT BUILD_TYPE STREQUAL "unit-test")
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -mthumb -fomit-frame-pointer -mlong-calls -mcpu=cortex-m4 -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -pipe -DNDEBUG -D__SAMD51J20A__")
endif()


#-----------------------------------------------------------------------------
# Test coverage flags

if(BUILD_COVERAGE)
    set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} --coverage")
    set(CMAKE_C_LINK_FLAGS "${CMAKE_C_LINK_FLAGS} --coverage")
endif()


#-----------------------------------------------------------------------------
# Build Documentation

if(BUILD_DOCUMENTATION)
    set(DOC_GRAPHS "YES" CACHE STRING "Create dependency graphs (needs graphviz)")
    set(DOC_FULLGRAPHS "NO" CACHE STRING "Create call/callee graphs (large)")

    find_program(DOT_PATH dot)

    if (DOT_PATH STREQUAL "DOT_PATH-NOTFOUND")
        message("Doxygen: graphviz not found - graphs disabled")
        set(DOC_GRAPHS "NO")
    endif()

    find_package(Doxygen)
    if(DOXYGEN_FOUND)
        configure_file("doc/Doxyfile.in" "Doxyfile" @ONLY)
        add_custom_target(doc
            ${DOXYGEN_EXECUTABLE} ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile
            WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
            COMMENT "Generating API documentation with Doxygen.." VERBATIM)
    endif()
endif()


#-----------------------------------------------------------------------------
# Build

message(STATUS "C Flags:          ${CMAKE_C_FLAGS}")

if(NOT BUILD_TYPE STREQUAL "bootloader")
    add_subdirectory(external)
endif()

if(BUILD_TYPE STREQUAL "device-test")
    add_subdirectory(test/device-test)
elseif(BUILD_TYPE STREQUAL "unit-test")
    # TODO: fix directory
    add_subdirectory(test/unit-test)
    enable_testing()
else()
    add_subdirectory(src)

    add_custom_command(
        OUTPUT ${MYPROJECT}.bin
        COMMAND ${CMAKE_SIZE} ${ELF}
        COMMAND ${CMAKE_OBJCOPY} -O binary ${ELF} ${MYPROJECT}.bin
        DEPENDS ${ELF}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/bin
        COMMENT "\nGenerating binary"
    )
    add_custom_target(binary_file ALL
        DEPENDS ${MYPROJECT}.bin ${ELF}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/bin
    )
    add_dependencies(binary_file ${ELF})

    #if(BUILD_TYPE STREQUAL "bootloader")
    #    add_custom_command(
    #        OUTPUT ${MYPROJECT}.pad.bin
    #        COMMAND python ../../py/pad_boot_binary.py ${MYPROJECT}.bin ${MYPROJECT}.pad.bin
    #        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/bin
    #        COMMENT "\nPadding binary"
    #    )
    #    add_custom_target(padded_file ALL
    #        DEPENDS ${MYPROJECT}.pad.bin
    #        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/bin
    #    )
    #    add_dependencies(padded_file binary_file)
    #endif()

endif()


#-----------------------------------------------------------------------------
# Clean

set(removefiles "bin/*.* lib/*.*")
set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES "${removefiles}")
