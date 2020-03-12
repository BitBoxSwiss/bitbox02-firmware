if(PYTHONINTERP_FOUND)
  # This template flashes with an offset
  foreach(target ${FIRMWARES})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-firmware.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.bin -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()

  # This template flashes without an offset
  foreach(target ${BOOTLOADERS})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-bootloader.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.bin -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()
endif()
