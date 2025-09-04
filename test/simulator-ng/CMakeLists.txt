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
# Simulator Next Generation
#

set(RUST_BINARY_DIR ${CMAKE_BINARY_DIR}/src/rust)

set(RUSTFLAGS -L${CMAKE_ARCHIVE_OUTPUT_DIRECTORY} -lstatic=fatfs -lstatic=secp256k1 --cfg=rust_secp_no_symbol_renaming)

add_custom_command(
    OUTPUT target/release/simulator-ng
    COMMAND
      ${CMAKE_COMMAND} -E env
      MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET}
      RUSTFLAGS="${RUSTFLAGS}"
      cargo build --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${RUST_BINARY_DIR}/fw --release
    COMMAND ${CMAKE_COMMAND} -E copy ${RUST_BINARY_DIR}/fw/release/simulator-ng ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/simulator-ng
    DEPENDS rust-cbindgen secp256k1 fatfs
    WORKING_DIRECTORY ${RUST_BINARY_DIR}
)

add_custom_target(simulator-ng
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/target/release/simulator-ng
)
