message("Loading arm.cmake")

set(CMAKE_SYSTEM_NAME "Generic")

include(CMakeForceCompiler)

CMAKE_FORCE_C_COMPILER("arm-none-eabi-gcc" GNU)

set(CMAKE_AR "arm-none-eabi-ar" CACHE PATH "" FORCE)
set(CMAKE_RANLIB "arm-none-eabi-ranlib" CACHE PATH "" FORCE)
set(CMAKE_LINKER "arm-none-eabi-ld" CACHE PATH "" FORCE)
set(CMAKE_SIZE "arm-none-eabi-size")
set(CMAKE_OBJCOPY "arm-none-eabi-objcopy")

# Search for programs in the build host directories
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)

# For libraries and headers in the target directories
#set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
#set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# Avoid known bug in linux giving: 
#    arm-none-eabi-gcc: error: unrecognized command line option '-rdynamic'
set(CMAKE_SHARED_LIBRARY_LINK_C_FLAGS "")
set(CMAKE_SHARED_LIBRARY_LINK_CXX_FLAGS "")

