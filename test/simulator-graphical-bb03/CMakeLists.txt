# SPDX-License-Identifier: Apache-2.0

#-----------------------------------------------------------------------------
# Simulator BitBox03
#

set(RUST_BINARY_DIR ${CMAKE_BINARY_DIR}/src/rust)

# Use libsecp256k1 that we link ourselves.
# See https://github.com/rust-bitcoin/rust-secp256k1/tree/7c8270a8506e31731e540fab7ee1abde1f48314e/secp256k1-sys#linking-to-external-symbols
# This is replicated in .cargo/config.toml because RUSTFLAGS here override the rustflags there.
set(RUSTFLAGS "-L${CMAKE_ARCHIVE_OUTPUT_DIRECTORY} -lstatic=fatfs --cfg=rust_secp_no_symbol_renaming")

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
    set(CFLAGS "-fsanitize=address")
endif()
if(SANITIZE_UNDEFINED)
    string(APPEND RUSTFLAGS " -Clink-arg=-fsanitize=undefined")
    string(APPEND CFLAGS " -fsanitize=undefined")
endif()

execute_process(COMMAND rustc --print host-tuple OUTPUT_VARIABLE LLVM_HOST_TUPLE OUTPUT_STRIP_TRAILING_WHITESPACE)

# Supplying llvm host tuple is necessary for address sanitization and doesn't
# hurt during regular builds.
add_custom_command(
    OUTPUT dummy ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
    COMMAND
      ${CMAKE_COMMAND} -E env
      MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}
      RUSTFLAGS=${RUSTFLAGS}
      CFLAGS=${CFLAGS}
      RUSTC_BOOTSTRAP=1
      cargo build $ENV{CARGOFLAGS} --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${RUST_BINARY_DIR} --target=${LLVM_HOST_TUPLE} --release
    COMMAND
      ${CMAKE_COMMAND} -E copy
      ${RUST_BINARY_DIR}/${LLVM_HOST_TUPLE}/release/simulator-graphical-bb03
      ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
    DEPENDS rust-cbindgen fatfs
    WORKING_DIRECTORY ${RUST_BINARY_DIR}
)

add_custom_target(simulator-graphical-bb03
    DEPENDS ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-graphical-bb03
)
