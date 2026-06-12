# SPDX-License-Identifier: Apache-2.0

if(PYTHONINTERP_FOUND)
  # This template flashes with an offset
  execute_process(
    COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-firmware.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${FIRMWARE_TARGET}.bin -o ${FIRMWARE_TARGET}.jlink
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )

  # This template flashes without an offset
  execute_process(
    COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-bootloader.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${BOOTLOADER_TARGET}.bin -o ${BOOTLOADER_TARGET}.jlink
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  )
endif()
