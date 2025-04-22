set(PROTO_FILES
  hww.proto
  backup.proto
  bluetooth.proto
  common.proto
  backup_commands.proto
  bitbox02_system.proto
  btc.proto
  cardano.proto
  eth.proto
  mnemonic.proto
  system.proto
  perform_attestation.proto
  keystore.proto
  antiklepto.proto
)

# Create absolute paths to protobuf sources
foreach(i ${PROTO_FILES})
  list(APPEND PROTO_FILES_ABSOLUTE "${CMAKE_CURRENT_SOURCE_DIR}/${i}")
endforeach()

find_program(PROST_BUILD prost-build-proto)

set(OUTPUT_FILES
  ${CMAKE_SOURCE_DIR}/src/rust/bitbox02-rust/src/shiftcrypto.bitbox02.rs
  ${CMAKE_SOURCE_DIR}/src/rust/bitbox02-rust/src/shiftcrypto.bitbox02.backups.rs)

add_custom_command(
  OUTPUT ${OUTPUT_FILES}
  DEPENDS ${PROTO_FILES}
  # We build the Rust protobuf files here and put them straight into the crate.
  # This way, the crate can be compiled and tested without relying on cmake environment vars.
  # Using prost-build the normal way as part of build.rs does not work due to a cargo bug:
  # https://github.com/danburkert/prost/issues/344#issuecomment-650721245
  COMMAND
    ${PROST_BUILD} ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_SOURCE_DIR}/src/rust/bitbox02-rust/src/
)

add_custom_target(
  generate-protobufs
  DEPENDS ${OUTPUT_FILES}
)
