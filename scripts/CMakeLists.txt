# SPDX-License-Identifier: Apache-2.0

if(PYTHONINTERP_FOUND)
  # ELF files supply the load addresses for all image types.
  foreach(target ${FIRMWARES})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-firmware.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.elf -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()
  foreach(target ${BB02_BLUPD_FIRMWARE_TARGETS})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-firmware.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.elf -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()

  foreach(target ${BB02_STAGE0_TARGETS})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-stage0.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.elf -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()

  foreach(target ${BB02_STAGE1_TARGETS})
    execute_process(
      COMMAND ${PYTHON_EXECUTABLE} ${CMAKE_CURRENT_SOURCE_DIR}/expand_template ${CMAKE_CURRENT_SOURCE_DIR}/template-stage1.jlink file=${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/${target}.elf -o ${target}.jlink
      WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    )
  endforeach()
endif()
