set(PROTO_FILES
  hww.proto
  backup.proto
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
string(REGEX REPLACE "\.proto" ".pb.c" OUTPUT_SOURCES "${PROTO_FILES}")
string(REGEX REPLACE "\.proto" ".pb.h" OUTPUT_HEADERS "${PROTO_FILES}")

# Create absolute paths to protobuf sources
foreach(i ${PROTO_FILES})
  list(APPEND PROTO_FILES_ABSOLUTE "${CMAKE_CURRENT_SOURCE_DIR}/${i}")
endforeach()

# Create absolute paths to generated sources
foreach(i ${OUTPUT_SOURCES})
  list(APPEND DBB-FIRMWARE-PROTOBUF-SOURCES "${CMAKE_CURRENT_BINARY_DIR}/${i}")
endforeach()
list(APPEND DBB-FIRMWARE-PROTOBUF-SOURCES
  ${CMAKE_SOURCE_DIR}/tools/nanopb/pb_common.c
  ${CMAKE_SOURCE_DIR}/tools/nanopb/pb_decode.c
  ${CMAKE_SOURCE_DIR}/tools/nanopb/pb_encode.c
)

# export the list of sources
set(DBB-FIRMWARE-PROTOBUF-SOURCES ${DBB-FIRMWARE-PROTOBUF-SOURCES} PARENT_SCOPE)

# .options files are optional
foreach(i ${PROTO_FILES})
  string(REGEX REPLACE "\.proto" ".options" tmp ${i})
  if(EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/${tmp})
    list(APPEND PROTO_OPTION_FILES ${tmp})
  endif()
endforeach()

find_program(PROST_BUILD prost-build)

add_custom_command(
  OUTPUT ${OUTPUT_SOURCES} ${OUTPUT_HEADERS}
  DEPENDS ${PROTO_FILES} ${PROTO_OPTION_FILES}
  COMMAND
  ${CMAKE_SOURCE_DIR}/tools/nanopb/generator/protoc
    --plugin=protoc-gen-nanopb=${CMAKE_SOURCE_DIR}/tools/nanopb/generator/protoc-gen-nanopb
    --proto_path=${CMAKE_CURRENT_SOURCE_DIR}
    '--nanopb_out=-I${CMAKE_CURRENT_SOURCE_DIR}:${CMAKE_CURRENT_BINARY_DIR}'
    ${PROTO_FILES_ABSOLUTE}
  # We build the Rust protobuf files here and put them straight into the crate.
  # This way, the crate can be compiled and tested without relying on cmake environment vars.
  # Using prost-build the normal way as part of build.rs does not work due to a cargo bug:
  # https://github.com/danburkert/prost/issues/344#issuecomment-650721245
  COMMAND
    ${PROST_BUILD} --messages-dir=${CMAKE_CURRENT_SOURCE_DIR} --out-dir=${CMAKE_SOURCE_DIR}/src/rust/bitbox02-rust/src/
)

add_custom_target(
  generate-protobufs
  DEPENDS ${OUTPUT_SOURCES} ${OUTPUT_HEADERS}
)
