find_path(CMOCKA_INCLUDE_DIR NAMES cmocka.h PATHS ${CMOCKA_ROOT_DIR}/include)

find_library(CMOCKA_LIBRARY NAMES cmocka PATHS ${CMOCKA_ROOT_DIR}/include)

if (CMOCKA_LIBRARY)
  set(CMOCKA_LIBRARIES
      ${CMOCKA_LIBRARIES}
      ${CMOCKA_LIBRARY}
  )
endif (CMOCKA_LIBRARY)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(CMocka DEFAULT_MSG CMOCKA_LIBRARIES CMOCKA_INCLUDE_DIR)
