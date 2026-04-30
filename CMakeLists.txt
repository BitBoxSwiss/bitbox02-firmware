# SPDX-License-Identifier: Apache-2.0

# Copyright (c) 2015-2016 Lucas Betschart, Douglas J. Bakkum

cmake_minimum_required(VERSION 3.10)

# Set a default build type if none was specified
set(default_build_type "RelWithDebInfo")

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to '${default_build_type}' as none was specified.")
  set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE STRING "Choose the type of build." FORCE)
  # We don't use the builtin MinSizeRel because Release always optimizes for size.
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release" "RelWithDebInfo")
endif()

# TODO: Using ${CMAKE_C_FLAGS_${CMAKE_BUILD_TYPE}} does not work the first time due to this.
string(TOUPPER ${CMAKE_BUILD_TYPE} CMAKE_BUILD_TYPE)

if(
  NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG" AND
  NOT CMAKE_BUILD_TYPE STREQUAL "RELEASE" AND
  NOT CMAKE_BUILD_TYPE STREQUAL "RELWITHDEBINFO"
)
  message(FATAL_ERROR "Invalid CMAKE_BUILD_TYPE ${CMAKE_BUILD_TYPE}")
endif()

find_program(SCCACHE_PROGRAM sccache)
find_program(CCACHE_PROGRAM ccache)
if(SCCACHE_PROGRAM)
  set(CMAKE_C_COMPILER_LAUNCHER "${SCCACHE_PROGRAM}")
elseif(CCACHE_PROGRAM)
  set(CMAKE_C_COMPILER_LAUNCHER "${CCACHE_PROGRAM}")
endif()

# This is ignored on platforms other than darwin. By default rust compiles for
# 10.7 which doesn't link for us.
set(CMAKE_OSX_DEPLOYMENT_TARGET "11" CACHE STRING "Minimum OS X deployment version")

project(bitbox02 C)

# nosys is set in arm.cmake so that `project(c)` above works. Remove it since it interferes with compile options
if(CMAKE_CROSSCOMPILING)
  string(REPLACE "--specs=nosys.specs" "" CMAKE_EXE_LINKER_FLAGS ${CMAKE_EXE_LINKER_FLAGS})
endif()

# Where to find custom cmake modules
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake/modules)

#-----------------------------------------------------------------------------
# Options for building

option(COVERAGE "Compile with test coverage flags." OFF)
option(SANITIZE_ADDRESS "Compile with asan." OFF)
option(SANITIZE_UNDEFINED "Compile with ubsan." OFF)
option(CMAKE_VERBOSE_MAKEFILE "Verbose build." OFF)

# Generate compile_command.json (for tidy and other tools)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

#-----------------------------------------------------------------------------
# Force out-of-source build

if(${CMAKE_CURRENT_SOURCE_DIR} STREQUAL ${CMAKE_CURRENT_BINARY_DIR})
message(FATAL_ERROR
  "CMake generation is not allowed within the source directory!\n"
  "Remove the CMakeCache.txt file and try again from another folder, e.g.:\n\n"
  "  rm CMakeCache.txt\n"
  "  mkdir build\n"
  "  cd build\n"
  "  cmake ..\n"
)
endif()

# Figure out what target our compiler builds for
execute_process(
  COMMAND ${CMAKE_C_COMPILER} -dumpmachine
  OUTPUT_VARIABLE TARGET_TRIPLE
  OUTPUT_STRIP_TRAILING_WHITESPACE
)

# Finds objcopy and other utilities for us
include(CMakeFindBinUtils)
if(CMAKE_CROSSCOMPILING)
  find_program(CMAKE_SIZE "${TARGET_TRIPLE}-size")
  message(STATUS "size: ${CMAKE_SIZE} ${TARGET_TRIPLE}-size")
endif()


#-----------------------------------------------------------------------------
# Create version header file
#
find_package(PythonInterp 3.6 REQUIRED)
set(VERSION_MANIFEST ${CMAKE_SOURCE_DIR}/versions.json)
set(VERSION_GENERATOR ${CMAKE_SOURCE_DIR}/scripts/generate_version_headers.py)
set(VERSION_TEMPLATE ${CMAKE_SOURCE_DIR}/src/version.h.tmpl)
set(BOOTLOADER_VERSION_TEMPLATE ${CMAKE_SOURCE_DIR}/src/bootloader/bootloader_version.h.tmpl)
set(GENERATED_VERSION_HEADERS_DIR ${CMAKE_BINARY_DIR}/src)
set(GENERATED_VERSION_CMAKE ${CMAKE_BINARY_DIR}/generated_versions.cmake)
set_property(
  DIRECTORY
  APPEND
  PROPERTY CMAKE_CONFIGURE_DEPENDS
    ${VERSION_MANIFEST}
    ${VERSION_GENERATOR}
    ${VERSION_TEMPLATE}
    ${BOOTLOADER_VERSION_TEMPLATE}
)
execute_process(
  COMMAND
    ${PYTHON_EXECUTABLE}
    ${VERSION_GENERATOR}
    generate
    --repo-root ${CMAKE_SOURCE_DIR}
    --output-dir ${GENERATED_VERSION_HEADERS_DIR}
    --cmake-vars-out ${GENERATED_VERSION_CMAKE}
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  RESULT_VARIABLE exit_code
  ERROR_VARIABLE version_generator_error
)
if(NOT exit_code EQUAL "0")
  message(FATAL_ERROR "generate_version_headers.py failed:\n${version_generator_error}")
endif()
include(${GENERATED_VERSION_CMAKE})
if(BOOTLOADER_VERSION_HAS_METADATA)
  add_definitions("-DBOOTLOADER_VERSION_HAS_METADATA")
endif()

#-----------------------------------------------------------------------------
# Set the default compiler options (Only set global options that truly are necessary for all files)

# Flags that are passed to the C compiler when `cc` builds c code in build.rs scripts.
set(CARGO_C_FLAGS "")

string(APPEND CMAKE_C_FLAGS " -std=c11 -pipe")
if(CMAKE_CROSSCOMPILING)
  set(CMAKE_C_FLAGS "\
    ${CMAKE_C_FLAGS} -mcpu=cortex-m4 -mthumb \
    -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -fomit-frame-pointer -D__SAMD51J20A__ \
    "
  )
  # Allow gc-sections at linking stage. (TODO: move to targets that use that linking option)
  string(APPEND CMAKE_C_FLAGS " -ffunction-sections -fdata-sections")
  set(CMAKE_C_LINK_FLAGS "\
    ${CMAKE_C_LINK_FLAGS} -mcpu=cortex-m4 -mthumb  -Wl,--gc-sections \
    "
  )

  string(APPEND CARGO_C_FLAGS "\
    -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -fomit-frame-pointer -D__SAMD51J20A")
endif()

# Optimize for size by default
set(CMAKE_C_FLAGS_RELEASE "-Os -DNDEBUG")
# (-ggdb) Allow gdb extensions if available
# Optimize debug build for size, optimizing for debug takes to much space.
set(CMAKE_C_FLAGS_DEBUG "-Os -ggdb")
set(CMAKE_C_FLAGS_RELWITHDEBINFO "-Os -ggdb -DNDEBUG")

if(DEFINED ENV{RUSTFLAGS})
    set(RUSTFLAGS "$ENV{RUSTFLAGS}")
else()
    set(RUSTFLAGS "")
endif()

# Use libsecp256k1 that we link ourselves.
# See https://github.com/rust-bitcoin/rust-secp256k1/tree/7c8270a8506e31731e540fab7ee1abde1f48314e/secp256k1-sys#linking-to-external-symbols
# This is replicated in .cargo/config.toml because RUSTFLAGS here override the rustflags there.
string(APPEND RUSTFLAGS " --cfg=rust_secp_no_symbol_renaming")

#-----------------------------------------------------------------------------
# Print system information and build options

execute_process(
  COMMAND ${CMAKE_C_COMPILER} -dumpversion
  OUTPUT_VARIABLE C_COMPILER_VERSION
  OUTPUT_STRIP_TRAILING_WHITESPACE
)

message(STATUS "\n\n=============================================")
message(STATUS "            - General -")
message(STATUS "Firmware version:       ${FIRMWARE_VERSION_FULL}")
message(STATUS "Firmware v. (git):      ${GIT_FIRMWARE_VERSION_STRING}")
message(STATUS "Bootloader version:     ${BOOTLOADER_VERSION_FULL}")
message(STATUS "Bootloader v. (git):    ${GIT_BOOTLOADER_VERSION_STRING}")
message(STATUS "CMake version:          ${CMAKE_VERSION}")
message(STATUS "System:                 ${CMAKE_SYSTEM}")
message(STATUS "Processor:              ${CMAKE_SYSTEM_PROCESSOR}")
message(STATUS "             - Build -")
message(STATUS "Compiler version:       ${CMAKE_C_COMPILER_ID} ${C_COMPILER_VERSION}")
message(STATUS "Compiler:               ${CMAKE_C_COMPILER}")
message(STATUS "Target triple:          ${TARGET_TRIPLE}")
message(STATUS "Compiler cache:         ${CMAKE_C_COMPILER_LAUNCHER}")
message(STATUS "Linker:                 ${CMAKE_LINKER}")
message(STATUS "Archiver:               ${CMAKE_AR}")
message(STATUS "Default CFLAGS:         ${CMAKE_C_FLAGS} ${CMAKE_C_FLAGS_${CMAKE_BUILD_TYPE}}")
message(STATUS "Default RUSTFLAGS:      ${RUSTFLAGS}")
message(STATUS "Default EXE LDFLAGS:    ${CMAKE_EXE_LINKER_FLAGS}")
message(STATUS "            - Options -")
message(STATUS "Build type:             ${CMAKE_BUILD_TYPE}")
message(STATUS "Hardware version:       ${HARDWARE}")
message(STATUS "Verbose:                ${CMAKE_VERBOSE_MAKEFILE}")
message(STATUS "Coverage flags:         ${COVERAGE}")
message(STATUS "\n=============================================\n\n")

#-----------------------------------------------------------------------------
# Collect all binaries into bin/lib subdirectory

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)


#-----------------------------------------------------------------------------
# Compiler
# more specific CFLAGS might be added in the subdirectories src/ and test/.

# Compile all code with additional warnings by default and treat warnings as errors
# Excerpt from GCC manual to avoid manually enabling warnings
#    -Wall:
#     -Waddress -Warray-bounds=1 (only with -O2) -Wbool-compare -Wbool-operation -Wc++11-compat  -Wc++14-compat
#     -Wcatch-value (C++ and Objective-C++ only) -Wchar-subscripts -Wcomment -Wduplicate-decl-specifier (C and
#     Objective-C only) -Wenum-compare (in C/ObjC; this is on by default in C++) -Wformat -Wint-in-bool-context
#     -Wimplicit (C and Objective-C only) -Wimplicit-int (C and Objective-C only) -Wimplicit-function-declaration (C
#     and Objective-C only) -Winit-self (only for C++) -Wlogical-not-parentheses -Wmain (only for C/ObjC and unless
#     -ffreestanding) -Wmaybe-uninitialized -Wmemset-elt-size -Wmemset-transposed-args -Wmisleading-indentation (only
#     for C/C++) -Wmissing-attributes -Wmissing-braces (only for C/ObjC) -Wmultistatement-macros -Wnarrowing (only for
#     C++) -Wnonnull -Wnonnull-compare -Wopenmp-simd -Wparentheses -Wpointer-sign -Wreorder -Wrestrict -Wreturn-type
#     -Wsequence-point -Wsign-compare (only in C++) -Wsizeof-pointer-div -Wsizeof-pointer-memaccess -Wstrict-aliasing
#     -Wstrict-overflow=1 -Wstringop-truncation -Wswitch -Wtautological-compare -Wtrigraphs -Wuninitialized
#     -Wunknown-pragmas -Wunused-function -Wunused-label -Wunused-value -Wunused-variable -Wvolatile-register-var
#
#    -Wextra:
#     -Wclobbered -Wcast-function-type -Wempty-body -Wignored-qualifiers -Wimplicit-fallthrough=3
#     -Wmissing-field-initializers -Wmissing-parameter-type (C only) -Wold-style-declaration (C only) -Woverride-init
#     -Wsign-compare (C only) -Wtype-limits -Wuninitialized -Wshift-negative-value (in C++03 and in C99 and newer)
#     -Wunused-parameter (only with -Wunused or -Wall) -Wunused-but-set-parameter (only with -Wunused or -Wall)
#
#    -Wformat=2:
#     Enable -Wformat plus additional format checks.  Currently equivalent to -Wformat -Wformat-nonliteral
#     -Wformat-security -Wformat-y2k.

string(APPEND CMAKE_C_FLAGS " -Wall -Wextra -Wpedantic")
string(APPEND CMAKE_C_FLAGS " -Wmissing-prototypes -Werror-implicit-function-declaration ")
string(APPEND CMAKE_C_FLAGS " -Wpointer-arith -Wunused -Wfloat-equal -Wshadow -Wbad-function-cast")
string(APPEND CMAKE_C_FLAGS " -Wformat=2 -Wformat-signedness -Wwrite-strings ")
string(APPEND CMAKE_C_FLAGS " -Wmissing-declarations -Wmissing-format-attribute -Wpacked ")
string(APPEND CMAKE_C_FLAGS " -Wredundant-decls -Wnested-externs -Wmultichar -Winit-self ")
string(APPEND CMAKE_C_FLAGS " -Wold-style-definition -Wswitch-default -Wattributes ")
string(APPEND CMAKE_C_FLAGS " -Wdeprecated-declarations -Wcast-qual -Wstrict-prototypes")
string(APPEND CMAKE_C_FLAGS " -Wundef -Wmissing-include-dirs")

# Strict alignment checks for pointer casts (GCC only).
if(CMAKE_C_COMPILER_ID STREQUAL "GNU")
  string(APPEND CMAKE_C_FLAGS " -Wcast-align=strict")
elseif(CMAKE_C_COMPILER_ID MATCHES "Clang")
  string(APPEND CMAKE_C_FLAGS " -Wcast-align")
endif()

# Disable builtin warning
string(APPEND CMAKE_C_FLAGS " -Wno-cast-function-type")

# Enable stack protection on release builds
if(NOT CMAKE_BUILD_TYPE STREQUAL "DEBUG")
  string(APPEND CMAKE_C_FLAGS " -fstack-protector-strong")
  string(APPEND CARGO_C_FLAGS " -fstack-protector-strong")
  if(CMAKE_CROSSCOMPILING)
    # Path to empty dummy libssp and libssp_shared. '-llibssp -llibssp_shared' is automatically added
    # with '-fstack-protector-all', but we don't need them as we have our own custom
    # `__stack_chk_fail`. See https://wiki.osdev.org/Stack_Smashing_Protector.
    set(CMAKE_C_LINK_FLAGS "${CMAKE_C_LINK_FLAGS} -L${CMAKE_CURRENT_SOURCE_DIR}/external/lib/ssp")
  endif()
endif()

# For `struct timespec` and `strdup`
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -D_XOPEN_SOURCE=600")
string(APPEND CARGO_C_FLAGS " -D_XOPEN_SOURCE=600")

string(LENGTH "${CMAKE_SOURCE_DIR}/" SOURCE_PATH_SIZE)
add_definitions("-DSOURCE_PATH_SIZE=${SOURCE_PATH_SIZE}")

if(SANITIZE_ADDRESS)
    string(APPEND CMAKE_C_FLAGS " -fsanitize=address")
    string(APPEND CARGO_C_FLAGS " -fsanitize=address")
endif()
if(SANITIZE_UNDEFINED)
    string(APPEND CMAKE_C_FLAGS " -fsanitize=undefined")
    string(APPEND CARGO_C_FLAGS " -fsanitize=undefined")
endif()

#-----------------------------------------------------------------------------
# Build

add_subdirectory(external)
add_subdirectory(src)
add_subdirectory(scripts)

#-----------------------------------------------------------------------------
# Build Documentation


set(DOC_GRAPHS "YES" CACHE STRING "Create dependency graphs (needs graphviz)")
set(DOC_FULLGRAPHS "NO" CACHE STRING "Create call/callee graphs (large)")

find_program(DOT_PATH dot)

if(DOT_PATH STREQUAL "DOT_PATH-NOTFOUND")
  message("Doxygen: graphviz not found - graphs disabled")
  set(DOC_GRAPHS "NO")
endif()

add_custom_target(doc)
find_package(Doxygen)
if(DOXYGEN_FOUND)
  configure_file("doc/Doxyfile.in" "Doxyfile" @ONLY)
  add_custom_target(doxygen-docs
    ${DOXYGEN_EXECUTABLE} ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "Generating API documentation with Doxygen.." VERBATIM)
  add_dependencies(doc doxygen-docs)
else()
  message("Doxygen: doxygen not found - docs disabled")
endif()

if(CMAKE_CROSSCOMPILING)
  add_dependencies(doc rust-docs)
else()
  include(CTest)
  add_subdirectory(test/unit-test)
  add_subdirectory(test/simulator)
  add_subdirectory(test/simulator-graphical)
  add_subdirectory(test/simulator-graphical-bb03)
  if(COVERAGE)
    find_program(GCOVR gcovr)
    if(NOT GCOVR STREQUAL "GCOVR-NOTFOUND")
      # __dummy is used to always force regeneration of coverage file (There is no reasonable way to track generation of gcda files...)
      add_custom_command(
        OUTPUT gcovr/coverage.html __dummy
        COMMAND ${CMAKE_COMMAND} -E make_directory gcovr
        COMMAND ${GCOVR} --gcov-executable gcov-10 --delete --html-details -o gcovr/coverage.html -r ${CMAKE_SOURCE_DIR} -f ${CMAKE_SOURCE_DIR}/src
      )
      add_custom_target(
        coverage
        COMMAND ${CMAKE_COMMAND} -E echo Coverage at file:///${CMAKE_BINARY_DIR}/gcovr/coverage.html
        DEPENDS gcovr/coverage.html
      )
    endif()
    find_program(LCOV lcov)
    find_program(GENHTML genhtml)
    if(NOT LCOV STREQUAL "LOCV-NOTFOUND" AND NOT GENHTML STREQUAL "GENHTML-NOTFOUND")
      add_custom_command(
        OUTPUT lcovr/raw_coverage.info lcovr/coverage.info lcovr/index.html __dummy
        COMMAND ${CMAKE_COMMAND} -E make_directory lcovr
        COMMAND ${LCOV} --capture --directory ${CMAKE_BINARY_DIR} --output-file lcovr/raw_coverage.info
        COMMAND ${LCOV} --remove lcovr/raw_coverage.info --output-file lcovr/coverage.info '*/test/*' '*/external/*' '*/src/drivers/*' '/usr/include/*' '*/tools/*'
        COMMAND ${GENHTML} lcovr/coverage.info --output-directory lcovr
      )
      add_custom_target(
        coverage-lcovr
        COMMAND ${CMAKE_COMMAND} -E echo Coverage at file:///${CMAKE_BINARY_DIR}/lcovr/index.html
        DEPENDS lcovr/index.html
      )
    endif()
  endif()
endif()

#-----------------------------------------------------------------------------
# Clean

set(removefiles "bin/*.* lib/*.*")
set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES "${removefiles}")
