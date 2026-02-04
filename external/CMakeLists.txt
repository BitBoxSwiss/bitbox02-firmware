include(ExternalProject)

# Tell `add_library()` to default to STATIC.
# Needed because secp256k1-zkp/CMakeLists.txt sets it to ON.
set(BUILD_SHARED_LIBS OFF)

if(CMAKE_CROSSCOMPILING)
  set(CONFIGURE_FLAGS
    --host=${CMAKE_SYSTEM_PROCESSOR}-none-eabi
    --build=${CMAKE_HOST_SYSTEM_PROCESSOR}-linux-gnu
    "$<$<BOOL:${CMAKE_C_COMPILER_LAUNCHER}>:CC=${CMAKE_C_COMPILER_LAUNCHER} ${CMAKE_C_COMPILER}>"
  )
endif()

# Remove parameters to make build identical to older build
# TODO: Remove the below hack (Use CMAKE_C_FLAGS directly instead of MODIFIED_C_FLAGS)
string(REPLACE "-mfloat-abi=softfp" "" MODIFIED_C_FLAGS_TMP ${CMAKE_C_FLAGS})
string(REPLACE "-mfpu=fpv4-sp-d16" "" MODIFIED_C_FLAGS ${MODIFIED_C_FLAGS_TMP})

if(CMAKE_CROSSCOMPILING)
  # Cortex Microcontroller Software Interface Standard
  add_library(CMSIS INTERFACE)
  target_include_directories(CMSIS SYSTEM INTERFACE CMSIS/Include)

  # Atmel SAMD51 Series Device Support
  add_library(samd51a-ds STATIC
    samd51a-ds/gcc/system_samd51.c
    samd51a-ds/gcc/gcc/startup_samd51.c
  )
  target_include_directories(samd51a-ds SYSTEM
    PUBLIC
      samd51a-ds/include
  )
  target_link_libraries(samd51a-ds
    PUBLIC
      CMSIS
  )

  # Advanced Software Framework Drivers
  # Only drivers needed for bootloader (minimal set)
  add_library(asf4-drivers-min STATIC
    asf4-drivers/hal/utils/src/utils_syscalls.c
    asf4-drivers/hal/utils/src/utils_list.c
    asf4-drivers/hal/src/hal_atomic.c
    asf4-drivers/hal/src/hal_gpio.c
    asf4-drivers/hal/src/hal_init.c
    asf4-drivers/hal/src/hal_delay.c
    asf4-drivers/hal/src/hal_timer.c
    asf4-drivers/hal/src/hal_usb_device.c
    asf4-drivers/hal/src/hal_rand_sync.c
    asf4-drivers/hal/src/hal_flash.c
    asf4-drivers/hal/src/hal_pac.c
    asf4-drivers/hal/src/hal_io.c
    asf4-drivers/hal/src/hal_sha_sync.c
    asf4-drivers/hpl/systick/hpl_systick.c
    asf4-drivers/hal/src/hal_usart_async.c
    asf4-drivers/hal/utils/src/utils_ringbuffer.c
    asf4-drivers/hpl/gclk/hpl_gclk.c
    asf4-drivers/hpl/oscctrl/hpl_oscctrl.c
    asf4-drivers/hpl/mclk/hpl_mclk.c
    asf4-drivers/hpl/osc32kctrl/hpl_osc32kctrl.c
    asf4-drivers/hpl/core/hpl_init.c
    asf4-drivers/hpl/core/hpl_core_m4.c
    asf4-drivers/hpl/spi/spi_lite.c
    asf4-drivers/hpl/usb/hpl_usb.c
    asf4-drivers/hpl/rtc/hpl_rtc.c
    asf4-drivers/hpl/sercom/hpl_sercom.c
    asf4-drivers/hpl/trng/hpl_trng.c
    asf4-drivers/hpl/nvmctrl/hpl_nvmctrl.c
    asf4-drivers/hpl/icm/hpl_icm.c
    asf4-drivers/hpl/pac/hpl_pac.c
    asf4-drivers/usb/usb_protocol.c
    asf4-drivers/usb/device/usbdc.c
    )

    target_compile_options(asf4-drivers-min PRIVATE -Wno-cast-qual -Wno-unused-parameter -Wno-missing-prototypes -Wno-missing-declarations -Wno-bad-function-cast -Wno-strict-prototypes -Wno-old-style-definition -Wno-cast-align)

    if (CMAKE_BUILD_TYPE STREQUAL "DEBUG")
        target_compile_definitions(asf4-drivers-min PUBLIC DEBUG USE_SIMPLE_ASSERT)
    endif()

  target_link_libraries(asf4-drivers-min samd51a-ds)
  set_property(TARGET asf4-drivers-min PROPERTY INTERFACE_LINK_LIBRARIES "")

  target_include_directories(asf4-drivers-min SYSTEM
    PUBLIC
      asf4-drivers
      asf4-drivers/Config
      asf4-drivers/hal/include
      asf4-drivers/hal/include
      asf4-drivers/hal/utils/include
      asf4-drivers/hpl/core
      asf4-drivers/hpl/gclk
      asf4-drivers/hpl/pm
      asf4-drivers/hpl/port
      asf4-drivers/hpl/pukcc
      asf4-drivers/hpl/rtc
      asf4-drivers/hpl/spi
      asf4-drivers/hri
      asf4-drivers/qtouch
      asf4-drivers/qtouch/include
      asf4-drivers/sd_mmc
      asf4-drivers/usb
      asf4-drivers/usb/class
      asf4-drivers/usb/class/hid
      asf4-drivers/usb/device
  )

  # Advanced Software Framework Drivers
  # All drivers
  add_library(asf4-drivers STATIC
    asf4-drivers/hal/src/hal_mci_sync.c
    asf4-drivers/hal/src/hal_i2c_m_sync.c
    asf4-drivers/hpl/sdhc/hpl_sdhc.c
    asf4-drivers/hpl/sercom/hpl_sercom.c
    asf4-drivers/sd_mmc/sd_mmc.c
    asf4-drivers/diskio/sdmmc_diskio.c
  )

  target_compile_options(asf4-drivers PRIVATE -Wno-cast-qual -Wno-unused-parameter -Wno-switch-default -Wno-bad-function-cast -Wno-implicit-fallthrough -Wno-cast-align)

  target_link_libraries(asf4-drivers
    PRIVATE
      samd51a-ds
    PUBLIC
      asf4-drivers-min
  )

  target_include_directories(asf4-drivers SYSTEM
    PUBLIC
      asf4-drivers/diskio
  )
set_property(TARGET asf4-drivers PROPERTY INTERFACE_LINK_LIBRARIES "")

  find_library(QTOUCHLIB_A qtm_acq_samd51_0x000f PATHS asf4-drivers/qtouch/lib/gcc NO_CMAKE_FIND_ROOT_PATH)
  find_library(QTOUCHLIB_B qtm_binding_layer_cm4_0x0005 PATHS asf4-drivers/qtouch/lib/gcc NO_CMAKE_FIND_ROOT_PATH)
  find_library(QTOUCHLIB_T qtm_touch_key_cm4_0x0002 PATHS asf4-drivers/qtouch/lib/gcc NO_CMAKE_FIND_ROOT_PATH)

  add_library(cryptoauthlib
    cryptoauthlib/lib/atca_cfgs.c
    cryptoauthlib/lib/atca_command.c
    cryptoauthlib/lib/atca_device.c
    cryptoauthlib/lib/atca_iface.c
    cryptoauthlib/lib/hal/atca_hal.c
    cryptoauthlib/lib/hal/hal_timer_start.c
    cryptoauthlib/lib/atca_basic.c
    cryptoauthlib/lib/atca_debug.c
    cryptoauthlib/lib/calib/calib_basic.c
    cryptoauthlib/lib/calib/calib_command.c
    cryptoauthlib/lib/calib/calib_execution.c
    cryptoauthlib/lib/calib/calib_counter.c
    cryptoauthlib/lib/calib/calib_gendig.c
    cryptoauthlib/lib/calib/calib_nonce.c
    cryptoauthlib/lib/calib/calib_checkmac.c
    cryptoauthlib/lib/calib/calib_info.c
    cryptoauthlib/lib/calib/calib_derivekey.c
    cryptoauthlib/lib/calib/calib_random.c
    cryptoauthlib/lib/calib/calib_selftest.c
    cryptoauthlib/lib/calib/calib_read.c
    cryptoauthlib/lib/calib/calib_privwrite.c
    cryptoauthlib/lib/calib/calib_verify.c
    cryptoauthlib/lib/calib/calib_write.c
    cryptoauthlib/lib/calib/calib_updateextra.c
    cryptoauthlib/lib/calib/calib_lock.c
    cryptoauthlib/lib/calib/calib_kdf.c
    cryptoauthlib/lib/calib/calib_genkey.c
    cryptoauthlib/lib/calib/calib_sign.c
    cryptoauthlib/lib/host/atca_host.c
    cryptoauthlib/lib/crypto/hashes/sha2_routines.c
    cryptoauthlib/lib/crypto/atca_crypto_sw_sha2.c
  )
  target_link_libraries(cryptoauthlib samd51a-ds asf4-drivers-min)
  set_property(TARGET cryptoauthlib PROPERTY INTERFACE_LINK_LIBRARIES "")
  target_include_directories(cryptoauthlib SYSTEM PUBLIC
    cryptoauthlib/lib
    ${CMAKE_CURRENT_SOURCE_DIR} # for the BitBox02-custom "atca_config.h"
    )
  target_compile_options(cryptoauthlib PRIVATE
    -Wno-pedantic -Wno-incompatible-pointer-types -Wno-unused-parameter -Wno-unused-variable -Wno-cast-qual
    -Wno-switch-default -Wno-format-nonliteral -Wno-missing-prototypes -Wno-missing-declarations
  )
  add_library(embedded-swd
    embedded-swd/dap.c
    embedded-swd/dap_target.c
  )
  target_include_directories(embedded-swd SYSTEM PUBLIC
    embedded-swd
  )
  target_link_libraries(embedded-swd
    PRIVATE
      CMSIS
  )
  target_link_libraries(embedded-swd
    PUBLIC
      samd51a-ds
  )

endif() # CMAKE_CROSSCOMPILING

# fatfs must to be linked together with a diskio middleware:
# - in the firmware, it is linked with asf4-drivers, which contains sdmmc_diskio.c (writing to actual microSD cards)
# - in unit tests, it is linked with a lib containing mock_diskio.c (volume mounted in RAM).
add_library(fatfs STATIC
  fatfs/source/ff.c
  fatfs/source/ffunicode.c
  )
set_property(TARGET fatfs PROPERTY INTERFACE_LINK_LIBRARIES "")
target_include_directories(fatfs SYSTEM PUBLIC fatfs/source)
target_compile_options(fatfs PRIVATE -Wno-switch-default)

# optiga-trust-m
# The below build instructions are loosely based on optiga-trust-m/tests/linux_i2c.cmake
aux_source_directory(optiga-trust-m/src/cmd SRC_CMD_FILES)
aux_source_directory(optiga-trust-m/src/common SRC_COMMON_FILES)
aux_source_directory(optiga-trust-m/external/mbedtls/library MBEDTLS_FILES)
aux_source_directory(optiga-trust-m/src/comms/ifx_i2c SRC_COMMS_IFX_I2C_FILES)
aux_source_directory(optiga-trust-m/src/crypt SRC_CRYPT_FILES)
aux_source_directory(optiga-trust-m/src/util SRC_UTIL_FILES)
set(PAL_FILES optiga-trust-m/extras/pal/pal_crypt_mbedtls.c)
add_library(optiga EXCLUDE_FROM_ALL
  ${SRC_CMD_FILES}
  ${SRC_COMMON_FILES}
  ${SRC_COMMS_IFX_I2C_FILES}
  optiga-trust-m/src/comms/optiga_comms_ifx_i2c.c
  ${SRC_CRYPT_FILES}
  ${SRC_UTIL_FILES}
  ${PAL_FILES}
  ${MBEDTLS_FILES}
)
target_compile_definitions(optiga PRIVATE MBEDTLS_USER_CONFIG_FILE="mbedtls_config.h")
# Ignore warnings in external lib.
target_compile_options(optiga PRIVATE "-w")
target_compile_definitions(optiga PRIVATE OPTIGA_LIB_EXTERNAL="optiga_config.h")
target_include_directories(optiga SYSTEM PUBLIC
  optiga-trust-m/config
  optiga-trust-m/include
  optiga-trust-m/include/cmd
  optiga-trust-m/include/common
  optiga-trust-m/include/ifx_i2c
  optiga-trust-m/include/pal
  optiga-trust-m/include/comms
  optiga-trust-m/external/mbedtls/include
  ${CMAKE_CURRENT_SOURCE_DIR} # for the BitBox-custom "optiga_config.h"
)
