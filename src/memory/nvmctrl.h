// SPDX-License-Identifier: Apache-2.0

#ifndef __NVMCTRL_H
#define __NVMCTRL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

/**
 * Writes a command to the NVM controller, and
 * waits for it to be completed.
 *
 * @param[in] cmd Command the NVM controller must execute.
 */
void nvmctrl_exec_cmd(uint16_t cmd);

/**
 * Clears NVM controller errors before a checked flash operation.
 *
 * This clears address, programming, lock, single- and dual-bit ECC, and
 * generic NVM errors.
 */
void nvmctrl_clear_errors(void);

/**
 * Waits for the NVM controller and checks errors from a flash operation.
 *
 * The checked error state is cleared before returning.
 * Corrected single-bit ECC errors are accepted; dual-bit/unexpected ECC types and other NVM
 * errors are rejected. Corrected data must still pass the caller's integrity checks.
 *
 * @return true if the controller became ready without an uncorrectable ECC or other NVM error.
 */
USE_RESULT bool nvmctrl_wait_ready_and_check_errors(void);

/** Copies flash through volatile reads, rejecting uncorrectable ECC and other NVM errors. */
USE_RESULT bool nvmctrl_read(const volatile uint8_t* flash, uint8_t* out, size_t len);

/**
 * Reads one erase block twice, rejecting NVM errors or differing reads.
 * out must hold 8192 bytes; its contents must not be used on failure.
 */
USE_RESULT bool nvmctrl_read_block_checked(uintptr_t address, uint8_t* out);

/**
 * Erases and writes one aligned erase block of main flash, checking unlock, erase, each
 * page write and relock for NVM/ECC errors. The caller must verify the complete readback and
 * abort on failure; the block may have been erased or partially written.
 */
USE_RESULT bool nvmctrl_write_block(uint32_t address, const uint8_t* data);

#endif // __NVMCTRL_H
