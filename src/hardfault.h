// SPDX-License-Identifier: Apache-2.0

#ifndef _HARDFAULT_H_
#define _HARDFAULT_H_

#ifdef TESTING
    #include <stdio.h>
void Dummy_Handler(void);

void HardFault_Handler(void) __attribute__((weak));
#endif

// Abort is for manual calls to stop execution, providing a message for
// debugging.
__attribute__((noreturn)) void Abort(const char* msg);

// Abort is for manual calls to stop execution, providing a message for debugging. It also sets
// autoenter to true, making sure that the device boots into the bootloader after reconnecting it.
// This should be called for any Abort during firmware startup, so a firmware update can be
// applied. Otherwise, if there is an Abort() during startup, there would no way to reboot into the
// bootloader and the device would be bricked.
__attribute__((noreturn)) void AbortAutoenter(const char* msg);

#endif
