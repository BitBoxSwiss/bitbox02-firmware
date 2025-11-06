// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
