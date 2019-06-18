include(${CMAKE_ROOT}/Modules/ExternalProject.cmake)
if(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/noise-c-${TARGET_CODE}-build/libnoiseprotocol.a)
    set(NOISEC_CONFIGURE ${CMAKE_CURRENT_SOURCE_DIR}/noise-c-config/configure_${TARGET_CODE}.sh)
    file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/noise-c-${TARGET_CODE}-build/)
    ExternalProject_Add(noise-c
        GIT_REPOSITORY https://github.com/shiftdevices/noise-c.git
        GIT_TAG        firmware_v2
        PREFIX         ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_CODE}/noise-c
        STEP_TARGETS   build-noise-c
        SOURCE_DIR     ${CMAKE_CURRENT_SOURCE_DIR}/noise-c
        CONFIGURE_COMMAND ${NOISEC_CONFIGURE}
        BUILD_COMMAND make -C src/protocol
        BUILD_IN_SOURCE 1
        INSTALL_COMMAND cmake -E copy ${CMAKE_CURRENT_SOURCE_DIR}/noise-c/src/protocol/libnoiseprotocol.a ${CMAKE_CURRENT_SOURCE_DIR}/noise-c-${TARGET_CODE}-build/libnoiseprotocol.a
  )
else()
    add_custom_target(noise-c COMMAND echo "noise-c already built")
endif()

if(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core-${TARGET_CODE}-build/.libs/)
    set(LIBWALLY_CONFIGURE ${CMAKE_CURRENT_SOURCE_DIR}/libwally-config/configure_${TARGET_CODE}.sh)

    file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core-${TARGET_CODE}-build/)
    ExternalProject_Add(libwally-core
      GIT_REPOSITORY https://github.com/shiftdevices/libwally-core.git
      GIT_TAG        bitbox02-firmware
      PREFIX         ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_CODE}/libwally-core
      STEP_TARGETS   build-libwally
      SOURCE_DIR     ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core
      CONFIGURE_COMMAND ${LIBWALLY_CONFIGURE}
      BUILD_COMMAND  make
      INSTALL_COMMAND cmake -E copy_directory ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_CODE}/libwally-core/src/libwally-core-build/src/ ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core-${TARGET_CODE}-build/
      )
else()
    add_custom_target(libwally-core COMMAND echo "libwally already built")
endif()
