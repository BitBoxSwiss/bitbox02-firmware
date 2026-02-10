# SPDX-License-Identifier: Apache-2.0

#-----------------------------------------------------------------------------
# Simulator BitBox03
#

set(RUST_BINARY_DIR ${CMAKE_BINARY_DIR}/src/rust)

if(SANITIZE_ADDRESS OR SANITIZE_UNDEFINED)
    string(APPEND RUSTFLAGS " -Zexternal-clangrt -Cdefault-linker-libraries=y")
    # When building with address sanitizer it is important that llvm/clang is
    # the same version as rustcs llvm. On macos this can be achieved by
    # installing llvm from brew and then prepend the path to its binary dir to
    # PATH. By default rust uses `cc` as linker, but this will pick the system llvm.
    if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
        string(APPEND RUSTFLAGS " -Clinker=clang")
    endif()
endif()
if(SANITIZE_ADDRESS)
    string(APPEND RUSTFLAGS " -Clink-arg=-fsanitize=address -Zsanitizer=address")
endif()
if(SANITIZE_UNDEFINED)
    string(APPEND RUSTFLAGS " -Clink-arg=-fsanitize=undefined")
endif()

execute_process(COMMAND rustc --print host-tuple OUTPUT_VARIABLE LLVM_HOST_TUPLE OUTPUT_STRIP_TRAILING_WHITESPACE)

set(LV_BUILD_CONF_DIR ${CMAKE_CURRENT_SOURCE_DIR} CACHE PATH
    "Can be used to specify the include dir containing lv_conf.h, to be used in conjunction with LV_CONF_INCLUDE_SIMPLE"
)
set(CONFIG_LV_BUILD_EXAMPLES OFF)
set(CONFIG_LV_BUILD_DEMOS OFF)
add_subdirectory(${CMAKE_SOURCE_DIR}/external/lvgl lvgl)
# lvgl overrides our archive output directory :(. We override it back
set_target_properties(
    lvgl
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}
)
# ignore warnings in external lib
target_compile_options(lvgl PRIVATE "-w")

# Look for lvgl in cmake build directory
string(APPEND RUSTFLAGS " -L${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}")

# Supplying llvm host tuple is necessary for address sanitization and doesn't
# hurt during regular builds.
add_custom_command(
    OUTPUT dummy ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
    COMMAND
      ${CMAKE_COMMAND} -E env
      MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}
      RUSTFLAGS=${RUSTFLAGS}
      CFLAGS=${CARGO_C_FLAGS}
      RUSTC_BOOTSTRAP=1
      LV_CONF_PATH=${CMAKE_CURRENT_SOURCE_DIR}/lv_conf.h
      cargo build $ENV{CARGOFLAGS} --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${RUST_BINARY_DIR} --target=${LLVM_HOST_TUPLE} --release
    COMMAND
      ${CMAKE_COMMAND} -E copy
      ${RUST_BINARY_DIR}/${LLVM_HOST_TUPLE}/release/simulator-graphical-bb03
      ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
    DEPENDS rust-cbindgen lvgl
    WORKING_DIRECTORY ${RUST_BINARY_DIR}
)

add_custom_target(simulator-graphical-bb03
    DEPENDS ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
)
