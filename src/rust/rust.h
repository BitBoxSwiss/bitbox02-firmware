#include <platform/platform_config.h>

#if PLATFORM_BITBOXBASE == 1 && !defined(BOOTLOADER)
#include "rust/bitboxbase_rust.h"
#endif

#if PLATFORM_BITBOX02 == 1 && !defined(BOOTLOADER)
#include "rust/bitbox02_rust.h"
#endif

#if defined(BOOTLOADER)
#include "rust/bootloader_rust.h"
#endif
