// SPDX-License-Identifier: Apache-2.0

#ifndef _STAGE0_INSTALL_H_
#define _STAGE0_INSTALL_H_

#include "util.h"

// Keep this decision independent of the earlier boot-path checks under LTO.
// NOLINTNEXTLINE(clang-diagnostic-unknown-attributes)
static secbool_u32 __attribute__((noinline, noclone)) stage0_install_auth(
    secbool_u32 update_ok,
    secbool_u32 installed_ok,
    secbool_u32 rechecked_installed_ok,
    uint16_t installed_version,
    uint16_t update_version)
{
    if (update_ok != sectrue_u32 || installed_ok != rechecked_installed_ok) {
        return secfalse_u32;
    }
    if (installed_ok == sectrue_u32) {
        return update_version >= installed_version ? sectrue_u32 : secfalse_u32;
    }
    // Recovery is allowed only when both checks returned a canonical failure.
    return installed_ok == secfalse_u32 ? sectrue_u32 : secfalse_u32;
}

#endif
