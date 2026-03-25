/**
 * \file
 *
 * \brief Linker script for running in internal FLASH on the SAMD51J20A
 *
 * Copyright (c) 2017 Microchip Technology Inc.
 *
 * \asf_license_start
 *
 * \page License
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the Licence at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an AS IS BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * \asf_license_stop
 *
 */


OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
SEARCH_DIR(.)

/* Memory Spaces Definitions */
MEMORY
{
  rom      (rx)  : ORIGIN = 0x00010000, LENGTH = 0x000D8000
  ram      (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00040000
  bkupram  (rwx) : ORIGIN = 0x47000000, LENGTH = 0x00002000
  qspi     (rwx) : ORIGIN = 0x04000000, LENGTH = 0x01000000
}

/* The stack size used by the application. */
STACK_SIZE = DEFINED(STACK_SIZE) ? STACK_SIZE : DEFINED(__stack_size__) ? __stack_size__ : 0x10000;
HEAP_SIZE = DEFINED(HEAP_SIZE) ? HEAP_SIZE : DEFINED(__heap_size__) ? __heap_size__ : 0x18000;

/* Section Definitions */
SECTIONS
{
    .text :
    {
        . = ALIGN(4);
        _sfixed = .;
        KEEP(*(.vectors .vectors.*))
        *(.text .text.* .gnu.linkonce.t.*)
        *(.glue_7t) *(.glue_7)
        *(.rodata .rodata* .gnu.linkonce.r.*)
        *(.ARM.extab* .gnu.linkonce.armextab.*)

        . = ALIGN(4);
        KEEP(*(.init))
        . = ALIGN(4);
        __preinit_array_start = .;
        KEEP (*(.preinit_array))
        __preinit_array_end = .;

        . = ALIGN(4);
        __init_array_start = .;
        KEEP (*(SORT(.init_array.*)))
        KEEP (*(.init_array))
        __init_array_end = .;

        . = ALIGN(4);
        KEEP (*crtbegin.o(.ctors))
        KEEP (*(EXCLUDE_FILE (*crtend.o) .ctors))
        KEEP (*(SORT(.ctors.*)))
        KEEP (*crtend.o(.ctors))

        . = ALIGN(4);
        KEEP(*(.fini))

        . = ALIGN(4);
        __fini_array_start = .;
        KEEP (*(.fini_array))
        KEEP (*(SORT(.fini_array.*)))
        __fini_array_end = .;

        KEEP (*crtbegin.o(.dtors))
        KEEP (*(EXCLUDE_FILE (*crtend.o) .dtors))
        KEEP (*(SORT(.dtors.*)))
        KEEP (*crtend.o(.dtors))

        . = ALIGN(4);
        _efixed = .;
    } > rom

    PROVIDE_HIDDEN (__exidx_start = .);
    .ARM.exidx :
    {
      *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    } > rom
    PROVIDE_HIDDEN (__exidx_end = .);

    . = ALIGN(4);
    _etext = .;

    .rtt (NOLOAD) :
    {
        . = ALIGN(4);
        _srtt = .;
        *(.segger_rtt);
        *(.segger_rtt_buf);
        _ertt = .;
    } > ram

    .relocate :
    {
        . = ALIGN(4);
        _srelocate = .;
        *(.ramfunc .ramfunc.*);
        *(.data .data.*);
        . = ALIGN(4);
        _erelocate = .;
    } > ram AT> rom

    .bkupram (NOLOAD):
    {
        . = ALIGN(8);
        _sbkupram = .;
        *(.bkupram .bkupram.*);
        . = ALIGN(8);
        _ebkupram = .;
    } > bkupram

    .qspi (NOLOAD):
    {
        . = ALIGN(8);
        _sqspi = .;
        *(.qspi .qspi.*);
        . = ALIGN(8);
        _eqspi = .;
    } > qspi

    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        _sbss = . ;
        _szero = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = . ;
        _ezero = .;
    } > ram

    .stack (NOLOAD):
    {
        . = ALIGN(8);
        _sstack = .;
        . = . + STACK_SIZE;
        . = ALIGN(8);
        _estack = .;
    } > ram

    . = ALIGN(4);
    _end = . ;
    PROVIDE(end = _end);

    .heap (NOLOAD):
    {
        . = ALIGN(8);
        _heap_start = .;
        . = . + HEAP_SIZE;
        . = ALIGN(8);
        _heap_end = .;
    } > ram
}
