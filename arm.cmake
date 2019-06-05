set(CMAKE_SYSTEM_NAME "Generic")
set(CMAKE_SYSTEM_PROCESSOR "arm")

set(TOOLCHAIN_PREFIX "arm-none-eabi-")
set(CMAKE_C_COMPILER "/usr/local/bin/${TOOLCHAIN_PREFIX}gcc")

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

# Avoid linking error:
#    exit.c:(.text.exit+0x2c): undefined reference to `_exit'
set(CMAKE_EXE_LINKER_FLAGS "--specs=nosys.specs --specs=nano.specs" CACHE INTERNAL "")
