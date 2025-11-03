# Copyright 2018 Shift Cryptosecurity AG
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

# No linker for Mach-O supports the linker argument `--wrap`. Since we use
# that, unit tests will never work on macos. Use linux/arm64 in docker instead.

# We use FindPkgConfig instead of FindPackage because it finds libraries in
# both linux and macos
find_package(PkgConfig REQUIRED)

# Unit testing uses CMocka
pkg_check_modules(CMOCKA REQUIRED cmocka)

# u2f tests with hardware uses hidapi-hidraw
if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
  pkg_check_modules(HIDAPI REQUIRED hidapi)
else()
  pkg_check_modules(HIDAPI REQUIRED hidapi-hidraw)
endif()

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-parameter -Wno-missing-prototypes -Wno-missing-declarations -Wno-implicit-function-declaration -Wno-bad-function-cast")

add_library(mocks STATIC EXCLUDE_FROM_ALL
    framework/src/mock_gestures.c
    framework/src/mock_screen_stack.c
    framework/src/mock_qtouch.c
)
target_link_libraries(mocks PUBLIC c-unit-tests_rust_c ${CMOCKA_LDFLAGS})
target_include_directories(mocks
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/framework/include
    ${CMOCKA_INCLUDE_DIRS}
)

#-----------------------------------------------------------------------------
# Tests

set(TEST_LIST
   cleanup
   "-Wl,--wrap=util_cleanup_32"
   gestures
   ""
   random
   "-Wl,--wrap=rand,--wrap=rust_sha256"
   ui_components
   ""
   ui_util
   ""
   ui_component_gestures
   ""
   memory
   "-Wl,--wrap=memory_read_chunk_fake,--wrap=memory_write_chunk_fake,--wrap=rust_noise_generate_static_private_key,--wrap=memory_read_shared_bootdata_fake,--wrap=memory_write_to_address_fake,--wrap=random_32_bytes_mcu"
   memory_functional
   ""
   cipher
   "-Wl,--wrap=cipher_fake_iv"
   util
   ""
   ugui
   ""
)

if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
  message("No linker for Mach-O supports --wrap, will not generate unit-tests")
else()
  list(LENGTH TEST_LIST TEST_LIST_LEN)
  math(EXPR TEST_LIST_LEN ${TEST_LIST_LEN}-1)
  foreach(I RANGE 0 ${TEST_LIST_LEN} 2)
    math(EXPR I2 ${I}+1)
    list(GET TEST_LIST ${I} TEST_NAME)
    list(GET TEST_LIST ${I2} TEST_LINK_ARGS)
    set(EXE test_${TEST_NAME})
    add_executable(${EXE} test_${TEST_NAME}.c)
    # asan must be first library in linking order
    target_link_libraries(${EXE} PRIVATE
      $<$<BOOL:${SANITIZE_ADDRESS}>:-fsanitize=address>
      $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
      -Wl,--start-group
      c-unit-tests_rust_c
      mocks
      fatfs
      -Wl,--end-group
      secp256k1
      ${TEST_LINK_ARGS}
    )
    target_include_directories(${EXE} PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
    )
    add_test(NAME test_${TEST_NAME} COMMAND ${EXE})
  endforeach()
endif()

add_library(u2f-util
  STATIC
  u2f/sha2.c
  u2f/ecc.c
  u2f/uECC.c
  u2f/u2f_util_t.c
)
target_include_directories(u2f-util
  SYSTEM PUBLIC
  ${HIDAPI_INCLUDE_DIRS}
)
target_include_directories(u2f-util
  PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}
)

# NOTE: we only depend on include directories and definitions from c-unit-tests_rust_c
target_link_libraries(u2f-util PUBLIC ${HIDAPI_LDFLAGS} c-unit-tests_rust_c)

# These unit tests for U2F are special because they don't call any bitbox functions directly, instead they go through hid_read/write.
# They are copied from https://github.com/google/u2f-ref-code/tree/master/u2f-tests/HID
set(U2F_TESTS
   u2f_standard
   u2f_hid
)
list(LENGTH U2F_TESTS U2F_TESTS_LEN)
foreach(TEST_NAME ${U2F_TESTS})
  set(EXE test_${TEST_NAME})

  # This tests link to our code
  add_executable(${EXE} test_${TEST_NAME}.c framework/src/mock_hidapi.c)
  target_link_libraries(${EXE} PRIVATE
    $<$<BOOL:${SANITIZE_ADDRESS}>:-fsanitize=address>
    $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--start-group>
    c-unit-tests_rust_c
    fatfs
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--end-group>
    u2f-util
    secp256k1
  )
  add_test(NAME test_${TEST_NAME} COMMAND ${EXE})

  # Running these two tests require a hardware device connected and they link to the real libhidapi.
  set(EXE test_${TEST_NAME}_with_hardware)
  add_executable(${EXE} test_${TEST_NAME}.c)
  # asan must be first library in linking order
  target_link_libraries(${EXE} PRIVATE
    $<$<BOOL:${SANITIZE_ADDRESS}>:-fsanitize=address>
    $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
    u2f-util
  )
  target_compile_definitions(${EXE} PRIVATE "WITH_HARDWARE")
endforeach()
