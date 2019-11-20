find_program(PROTOC_GEN_NANOPB protoc-gen-nanopb)

set(PROTO_FILES
  hww.proto
  backup.proto
  common.proto
  backup_commands.proto
  bitbox02_system.proto
  bitboxbase.proto
  btc.proto
  eth.proto
  mnemonic.proto
  random_number.proto
  system.proto
  perform_attestation.proto
)
string(REGEX REPLACE "\.proto" ".pb.c" OUTPUT_SOURCES "${PROTO_FILES}")
string(REGEX REPLACE "\.proto" ".pb.h" OUTPUT_HEADERS "${PROTO_FILES}")

if (PROTOC_GEN_NANOPB STREQUAL "PROTOC_GEN_NANOPB-NOTFOUND")
  message(FATAL_ERROR
    "Could not find 'protoc-gen-nanopb'.\n"
    "Please install it from here: https://jpa.kapsi.fi/nanopb/download/\n"
    "And add generator-bin/ to your path.")
endif()

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

add_custom_command(
  OUTPUT ${OUTPUT_SOURCES} ${OUTPUT_HEADERS}
  DEPENDS ${PROTO_FILES} ${PROTO_OPTION_FILES}
  COMMAND
  ${PROTOC} "--proto_path=${CMAKE_CURRENT_SOURCE_DIR}" "--nanopb_out=-I${CMAKE_CURRENT_SOURCE_DIR}:${CMAKE_CURRENT_BINARY_DIR}" ${PROTO_FILES_ABSOLUTE}
)

add_custom_target(
  generate-protobufs
  DEPENDS ${OUTPUT_SOURCES} ${OUTPUT_HEADERS}
)

set(PB_DOCS_DIR ${CMAKE_BINARY_DIR}/docs-pb)

if(NOT PROTOC_GEN_DOC STREQUAL "PROTOC_GEN_DOC-NOTFOUND")
  add_custom_command(
    OUTPUT ${PB_DOCS_DIR}/index.html
    COMMAND ${CMAKE_COMMAND} -E make_directory ${PB_DOCS_DIR}
    COMMAND ${PROTOC} "--proto_path=${CMAKE_CURRENT_SOURCE_DIR}" "--doc_out=${PB_DOCS_DIR}" ${PROTO_FILES_ABSOLUTE}
    COMMAND ${CMAKE_COMMAND} -E echo Find the protobuf docs here ${PB_DOCS_DIR}/index.html
    DEPENDS ${PROTO_FILES}
  )
  add_custom_target(
    protobuf-docs
    DEPENDS ${PB_DOCS_DIR}/index.html
  )
endif()
