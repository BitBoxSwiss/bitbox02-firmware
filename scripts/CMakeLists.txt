if(PYTHONINTERP_FOUND)
  # This template flashes with an offset
  foreach(target firmware firmware-semihosting factory-setup)
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-firmware.jlink file=build/bin/${target}.bin -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()

  # This template flashes without an offset
  foreach(target bootloader bootloader-development bootloader-semihosting bootloader-production bootloader-btc bootloader-btc-development bootloader-btc-production)
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-bootloader.jlink file=build/bin/${target}.bin -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()
endif()
