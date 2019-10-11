set(CMAKE_SYSTEM_NAME "Generic")
set(CMAKE_SYSTEM_PROCESSOR "arm")

set(TOOLCHAIN_PREFIX_PREFIX "arm-none-eabi")
set(TOOLCHAIN_PREFIX "${TOOLCHAIN_PREFIX_PREFIX}-")
set(CMAKE_C_COMPILER "/usr/local/bin/${TOOLCHAIN_PREFIX}gcc")
set(CMAKE_SYSROOT "/usr/local/${TOOLCHAIN_PREFIX_PREFIX}")

# Search for programs in the build host directories
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)

# For libraries and headers in the target directories
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# Avoid known bug in linux giving: 
#    arm-none-eabi-gcc: error: unrecognized command line option '-rdynamic'
# TODO: Is this needed?
set(CMAKE_SHARED_LIBRARY_LINK_C_FLAGS "")
set(CMAKE_SHARED_LIBRARY_LINK_CXX_FLAGS "")

# nosys selects an weak "no-op" implementation of the system commands. This allows CMake to link its test executables.
# weak means that if a symbol is provided, like _break, that will be used instead.
set(CMAKE_EXE_LINKER_FLAGS "--specs=nosys.specs" CACHE INTERNAL "")
