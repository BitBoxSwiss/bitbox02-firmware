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
# Build bitbox lib to use in tests

include(${CMAKE_SOURCE_DIR}/src/CMakeLists.txt)
message(STATUS "includes: ${INCLUDES}")

set(FILTERED_SOURCES "")
foreach(file IN ITEMS ${DBB-FIRMWARE-UI-SOURCES})
    if(NOT ${file} MATCHES "src/screen.c$" AND NOT ${file} MATCHES "ugui.c$")
    set(FILTERED_SOURCES
        ${FILTERED_SOURCES}
        ${file}
    )
    endif()
endforeach(file)
set(DBB-FIRMWARE-UI-SOURCES ${FILTERED_SOURCES})

set(CMAKE_C_LINK_FLAGS "${CMAKE_C_LINK_FLAGS} -lm")

message(STATUS "C link flags:     ${CMAKE_C_LINK_FLAGS}\n")
include_directories(
  SYSTEM
  ${SYSTEMINCLUDES}
  ${CMAKE_SOURCE_DIR}/external
  ${CMAKE_SOURCE_DIR}/external/noise-c/include
  ${CMAKE_SOURCE_DIR}/external/noise-c/src
  ${CMAKE_SOURCE_DIR}/external/ctaes
  ${CMAKE_SOURCE_DIR}/external/sha3
  ${CMAKE_SOURCE_DIR}/external/libwally-core/include
  ${CMAKE_SOURCE_DIR}/external/libwally-core/src/secp256k1/include
)
include_directories(
  ${INCLUDES}
  ${CMAKE_SOURCE_DIR}/test/unit-test/framework/includes
  ${CMAKE_BINARY_DIR}/src
)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/tools/nanopb/extra)
find_package(Nanopb REQUIRED)
include_directories(${NANOPB_INCLUDE_DIRS})

add_library(bitbox
    STATIC
    ${DBB-FIRMWARE-SOURCES}
    ${DBB-FIRMWARE-UI-SOURCES}
    ${DBB-FIRMWARE-PROTOBUF-SOURCES}
    ${DBB-FIRMWARE-USB-SOURCES}
    ${CTAES-SOURCES}
    ${ETHEREUM-SOURCES}
    framework/mock_screen.c
    framework/mock_memory.c
    framework/mock_qtouch.c
    framework/mock_gestures.c
    framework/mock_ugui.c
    framework/mock_component.c
    framework/mock_ff.c
    framework/mock_sd.c
    framework/mock_securechip.c
)

add_dependencies(bitbox noise-c)
add_dependencies(bitbox libwally-core)

target_link_libraries(bitbox "${CMAKE_SOURCE_DIR}/external/noise-c-${TARGET_CODE}-build/libnoiseprotocol.a")
target_link_libraries(bitbox "${CMAKE_SOURCE_DIR}/external/libwally-core-${TARGET_CODE}-build/.libs/libwallycore.a")
target_link_libraries(bitbox "${CMAKE_SOURCE_DIR}/external/libwally-core-${TARGET_CODE}-build/secp256k1/.libs/libsecp256k1.a")

#-----------------------------------------------------------------------------
# Tests

enable_testing()

# Build tests_unit
add_executable(test_simple test_simple.c)
target_link_libraries(test_simple bitbox cmocka)
add_test(NAME test_simple COMMAND test_simple)

add_executable(test_cleanup test_cleanup.c)
target_link_libraries(test_cleanup bitbox cmocka "-Wl,--wrap=util_cleanup_32")
add_test(NAME test_cleanup COMMAND test_cleanup)

add_executable(test_hww test_hww.c)
target_link_libraries(test_hww bitbox cmocka "-Wl,--wrap=random_32_bytes,--wrap=workflow_confirm_dismiss")
add_test(NAME test_hww COMMAND test_hww)

add_executable(test_keystore test_keystore.c)
target_link_libraries(test_keystore bitbox cmocka "-Wl,--wrap=secp256k1_ecdsa_sign_recoverable,--wrap=memory_is_initialized,--wrap=memory_is_seeded,--wrap=memory_get_failed_unlock_attempts,--wrap=memory_reset_failed_unlock_attempts,--wrap=memory_increment_failed_unlock_attempts,--wrap=memory_set_encrypted_seed_and_hmac,--wrap=memory_get_encrypted_seed_and_hmac,--wrap=reset_reset,--wrap=salt_hash_data,--wrap=cipher_aes_hmac_encrypt")
add_test(NAME test_keystore COMMAND test_keystore)

add_executable(test_gestures test_gestures.c)
target_link_libraries(test_gestures bitbox cmocka)
add_test(NAME test_gestures COMMAND test_gestures)

add_executable(test_random test_random.c)
target_link_libraries(test_random bitbox cmocka "-Wl,--wrap=rand,--wrap=wally_sha256")
add_test(NAME test_random COMMAND test_random)

add_executable(test_app_btc test_app_btc.c)
target_link_libraries(test_app_btc bitbox cmocka "-Wl,--wrap=keystore_get_xpub,--wrap=btc_common_is_valid_keypath,--wrap=btc_common_encode_xpub,--wrap=btc_common_outputhash_from_pubkeyhash")
add_test(NAME test_app_btc COMMAND test_app_btc)

add_executable(test_app_btc_common test_app_btc_common.c)
target_link_libraries(test_app_btc_common bitbox cmocka)
add_test(NAME test_app_btc_common COMMAND test_app_btc_common)

add_executable(test_sd test_sd.c)
target_link_libraries(test_sd bitbox cmocka)
add_test(NAME test_sd COMMAND test_sd)

add_executable(test_ui_components test_ui_components.c)
target_link_libraries(test_ui_components bitbox cmocka)
add_test(NAME test_ui_components COMMAND test_ui_components)

add_executable(test_ui_util test_ui_util.c)
target_link_libraries(test_ui_util bitbox cmocka)
add_test(NAME test_ui_util COMMAND test_ui_util)

add_executable(test_ui_component_gestures test_ui_component_gestures.c)
target_link_libraries(test_ui_component_gestures bitbox cmocka)
add_test(NAME test_ui_component_gestures COMMAND test_ui_component_gestures)

add_executable(test_btc_sign test_btc_sign.c)
target_link_libraries(test_btc_sign bitbox cmocka "-Wl,--wrap=workflow_verify_recipient,--wrap=workflow_verify_total,--wrap=btc_common_format_amount")
add_test(NAME test_btc_sign COMMAND test_btc_sign)

add_executable(test_memory test_memory.c)
target_link_libraries(test_memory bitbox cmocka "-Wl,--wrap=memory_read_chunk_mock,--wrap=memory_write_chunk_mock,--wrap=bb_noise_generate_static_private_key,--wrap=memory_read_shared_bootdata_mock,--wrap=memory_write_to_address_mock")
add_test(NAME test_memory COMMAND test_memory)

add_executable(test_salt test_salt.c)
target_link_libraries(test_salt bitbox cmocka "-Wl,--wrap=memory_get_salt_root")
add_test(NAME test_salt COMMAND test_salt)

add_executable(test_backup test_backup.c)
target_link_libraries(test_backup bitbox cmocka "-Wl,--wrap=keystore_copy_seed,--wrap=memory_get_seed_birthdate,--wrap=sd_write_bin,--wrap=sd_load_bin,--wrap=sd_list_subdir,--wrap=sd_erase_file_in_subdir,--wrap=sd_file_rename,--wrap=memory_get_device_name")
add_test(NAME test_backup COMMAND test_backup)

add_executable(test_restore test_restore.c)
target_link_libraries(test_restore bitbox cmocka "-Wl,--wrap=keystore_copy_seed,--wrap=keystore_encrypt_and_store_seed,--wrap=memory_get_seed_birthdate,--wrap=sd_write_bin,--wrap=sd_load_bin,--wrap=sd_list_subdir,--wrap=sd_list,--wrap=sd_erase_file_in_subdir,--wrap=sd_file_rename,--wrap=memory_get_device_name,--wrap=memory_set_device_name")
add_test(NAME test_restore COMMAND test_restore)

add_executable(test_cipher test_cipher.c)
target_link_libraries(test_cipher bitbox cmocka "-Wl,--wrap=random_32_bytes")
add_test(NAME test_cipher COMMAND test_cipher)

add_executable(test_sha3 test_sha3.c)
target_link_libraries(test_sha3 bitbox cmocka)
add_test(NAME test_sha3 COMMAND test_sha3)

add_executable(test_app_eth test_app_eth.c)
target_link_libraries(test_app_eth bitbox cmocka "-Wl,--wrap=keystore_secp256k1_pubkey,--wrap=eth_common_is_valid_keypath")
add_test(NAME test_app_eth COMMAND test_app_eth)

add_executable(test_app_eth_common test_app_eth_common.c)
target_link_libraries(test_app_eth_common bitbox cmocka)
add_test(NAME test_app_eth_common COMMAND test_app_eth_common)
