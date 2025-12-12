// SPDX-License-Identifier: Apache-2.0

#ifndef __NVMCTRL_H
#define __NVMCTRL_H

#include <stdint.h>

/**
 * Writes a command to the NVM controller, and
 * waits for it to be completed.
 *
 * @param[in] cmd Command the NVM controller must execute.
 */
void nvmctrl_exec_cmd(uint16_t cmd);

#endif // __NVMCTRL_H
