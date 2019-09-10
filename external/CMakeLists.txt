include(ExternalProject)

if(CMAKE_CROSSCOMPILING)
  set(CONFIGURE_FLAGS
    --host=${CMAKE_SYSTEM_PROCESSOR}-none-eabi --build=${CMAKE_HOST_SYSTEM_PROCESSOR}-linux-gnu)
endif()

# Remove parameters to make build identical to older build
# TODO: Remove the below hack (Use CMAKE_C_FLAGS directly instead of MODIFIED_C_FLAGS)
string(REPLACE "-mfloat-abi=softfp" "" MODIFIED_C_FLAGS_TMP ${CMAKE_C_FLAGS})
string(REPLACE "-mfpu=fpv4-sp-d16" "" MODIFIED_C_FLAGS ${MODIFIED_C_FLAGS_TMP})

#-----------------------
# noise-c

set(NOISEC_CONFIGURE_FLAGS "--with-libsodium=no --with-openssl=no")
if(SANITIZE_ADDRESS)
  set(NOISEC_CFLAGS "-fsanitize=address")
endif()
if(SANITIZE_UNDEFINED)
  set(NOISEC_CFLAGS "${NOISEC_CFLAGS} -fsanitize=undefined")
endif()
set(NOISEC_CFLAGS "\
  ${NOISEC_CFLAGS} \
  ${MODIFIED_C_FLAGS} ${CMAKE_C_FLAGS_${CMAKE_BUILD_TYPE}} \
  -fno-strict-aliasing \
")
# Hide some warnings
set(NOISEC_CFLAGS "${NOISEC_CFLAGS} -Wformat=0 -Wno-implicit-fallthrough -Wno-undef -Wno-cast-qual -Wno-switch-default -Wno-packed -Wno-pedantic -Wno-missing-prototypes -Wno-unused-parameter -Wno-redundant-decls -Wno-missing-declarations -Wno-shadow")
if(CMAKE_CROSSCOMPILING)
  set(NOISEC_LDFLAGS "--specs=nosys.specs")
endif()
set(NOISEC_LDFLAGS "${NOISEC_LDFLAGS} ${CMAKE_C_LINK_FLAGS} -lc")

ExternalProject_Add(noise-c
  PREFIX          ${CMAKE_CURRENT_BINARY_DIR}/noise-c
  STEP_TARGETS    build-noise-c
  SOURCE_DIR      ${CMAKE_CURRENT_SOURCE_DIR}/noise-c
  CONFIGURE_COMMAND autoreconf --install --force ${CMAKE_CURRENT_SOURCE_DIR}/noise-c
  COMMAND ${CMAKE_COMMAND} -E env env
  COMMAND         ${CMAKE_COMMAND} -E env
                  "CFLAGS=${NOISEC_CFLAGS}"
                  "LDFLAGS=${NOISEC_LDFLAGS}"
                  ${CMAKE_CURRENT_SOURCE_DIR}/noise-c/configure
                  ${CONFIGURE_FLAGS}
                  ${NOISEC_CONFIGURE_FLAGS}
  BUILD_COMMAND   ${CMAKE_MAKE_PROGRAM} -C src/protocol
  INSTALL_COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}
  COMMAND         ${CMAKE_COMMAND} -E copy
                  ${CMAKE_CURRENT_BINARY_DIR}/noise-c/src/noise-c-build/src/protocol/libnoiseprotocol.a
                  ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libnoiseprotocol.a
)

add_library(noiseprotocol STATIC IMPORTED GLOBAL)
set_property(TARGET noiseprotocol
  PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libnoiseprotocol.a)

#----------------------
# wally-core

set(LIBWALLY_CONFIGURE_FLAGS --enable-static --disable-shared --disable-tests)
if(SANITIZE_ADDRESS)
  set(LIBWALLY_CFLAGS "-fsanitize=address")
endif()
if(SANITIZE_UNDEFINED)
  set(LIBWALLY_CFLAGS "${LIBWALLY_CFLAGS} -fsanitize=undefined")
endif()
set(LIBWALLY_CFLAGS  "\
  ${LIBWALLY_CFLAGS} \
  ${MODIFIED_C_FLAGS} ${CMAKE_C_FLAGS_${CMAKE_BUILD_TYPE}} \
  -DUSE_BASIC_CONFIG=1 -DUSE_ECMULT_STATIC_PRECOMPUTATION=1 \
  -fno-strict-aliasing \
")

# Hide some warnings
set(LIBWALLY_CFLAGS "${LIBWALLY_CFLAGS} -Wno-cast-qual -Wno-cast-align \
  -Wno-missing-prototypes -Wno-redundant-decls \
  -Wno-switch-default -Wno-missing-declarations \
  -Wno-array-bounds \
")
if(CMAKE_CROSSCOMPILING)
  set(LIBWALLY_LDFLAGS --specs=nosys.specs)
endif()
set(LIBWALLY_LDFLAGS "${LIBWALLY_LDFLAGS} ${CMAKE_C_LINK_FLAGS}")

ExternalProject_Add(libwally-core
  PREFIX          ${CMAKE_CURRENT_BINARY_DIR}/libwally-core
  STEP_TARGETS   build-libwally
  SOURCE_DIR      ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core
  CONFIGURE_COMMAND autoreconf --install --force ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core
  COMMAND         ${CMAKE_COMMAND} -E env
                  "CFLAGS=${LIBWALLY_CFLAGS}"
                  "LDFLAGS=${LIBWALLY_LDFLAGS}"
                  ${CMAKE_CURRENT_SOURCE_DIR}/libwally-core/configure
                  ${CONFIGURE_FLAGS}
                  ${LIBWALLY_CONFIGURE_FLAGS}
  # TODO: Add darwin hack to libwally repo
  BUILD_COMMAND   ${CMAKE_MAKE_PROGRAM}
  INSTALL_COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}
  COMMAND         ${CMAKE_COMMAND} -E copy
                  ${CMAKE_CURRENT_BINARY_DIR}/libwally-core/src/libwally-core-build/src/.libs/libwallycore.a
                  ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libwallycore.a
  COMMAND         ${CMAKE_COMMAND} -E copy
                  ${CMAKE_CURRENT_BINARY_DIR}/libwally-core/src/libwally-core-build/src/secp256k1/.libs/libsecp256k1.a
                  ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libsecp256k1.a
)

add_library(wallycore STATIC IMPORTED GLOBAL)
set_property(TARGET wallycore
  PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libwallycore.a)

add_library(secp256k1 STATIC IMPORTED GLOBAL)
set_property(TARGET secp256k1
  PROPERTY IMPORTED_LOCATION ${CMAKE_ARCHIVE_OUTPUT_DIRECTORY}/libsecp256k1.a)

add_library(CMSIS INTERFACE)
target_include_directories(CMSIS INTERFACE CMSIS/Include)

add_library(samd51a-ds samd51a-ds/gcc/system_samd51.c samd51a-ds/gcc/gcc/startup_samd51.c)
target_include_directories(samd51a-ds PUBLIC samd51a-ds/include)
target_link_libraries(samd51a-ds PUBLIC CMSIS)
