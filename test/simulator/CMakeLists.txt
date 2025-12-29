# SPDX-License-Identifier: Apache-2.0

add_executable(simulator EXCLUDE_FROM_ALL simulator.c)

# asan must be first library in linking order
target_link_libraries(simulator PRIVATE
    $<$<BOOL:${SANITIZE_ADDRESS}>:-fsanitize=address>
    $<$<BOOL:${SANITIZE_UNDEFINED}>:-fsanitize=undefined>
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--start-group>
    c-unit-tests_rust_c
    fatfs
    $<$<NOT:$<PLATFORM_ID:Darwin>>:-Wl,--end-group>
)

add_custom_command(
    TARGET simulator POST_BUILD
    COMMAND ${CMAKE_STRIP} simulator
    WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
    COMMENT "\nStripping simulator binary"
)
