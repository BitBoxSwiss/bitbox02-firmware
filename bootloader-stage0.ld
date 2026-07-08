OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
SEARCH_DIR(.)

STAGE0_IMAGE_ORIGIN = 0x00000000;
STAGE0_IMAGE_LEN = 0x00002000;
STAGE0_DESCRIPTOR_LEN = 0x0000000C;
STAGE0_DESCRIPTOR_ADDR = STAGE0_IMAGE_ORIGIN + STAGE0_IMAGE_LEN - STAGE0_DESCRIPTOR_LEN;

MEMORY
{
  rom (rx)  : ORIGIN = STAGE0_IMAGE_ORIGIN, LENGTH = STAGE0_IMAGE_LEN - STAGE0_DESCRIPTOR_LEN
  stage0_descriptor (rx) : ORIGIN = STAGE0_DESCRIPTOR_ADDR, LENGTH = STAGE0_DESCRIPTOR_LEN
  ram (rwx) : ORIGIN = 0x20000200, LENGTH = 0x0003FE00
}

STACK_SIZE = DEFINED(STACK_SIZE) ? STACK_SIZE : 0x1000;
HEAP_SIZE = DEFINED(HEAP_SIZE) ? HEAP_SIZE : 0x0;

SECTIONS
{
    .text :
    {
        . = ALIGN(4);
        _sfixed = .;
        __stage0_vectors_start = .;
        KEEP(*(.vectors .vectors.*))
        __stage0_vectors_end = .;
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
        KEEP(*(.fini))
        . = ALIGN(4);
        __fini_array_start = .;
        KEEP (*(.fini_array))
        KEEP (*(SORT(.fini_array.*)))
        __fini_array_end = .;
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

    .relocate :
    {
        . = ALIGN(4);
        _srelocate = .;
        *(.ramfunc .ramfunc.*);
        *(.data .data.*);
        . = ALIGN(4);
        _erelocate = .;
    } > ram AT> rom

    .stage0_descriptor ORIGIN(stage0_descriptor) :
    {
        . = ALIGN(4);
        KEEP(*(.stage0_descriptor))
    } > stage0_descriptor

    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        _sbss = .;
        _szero = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
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
    _end = .;

    .heap (NOLOAD):
    {
        . = ALIGN(8);
        _heap_start = .;
        . = . + HEAP_SIZE;
        . = ALIGN(8);
        _heap_end = .;
    } > ram
}

ASSERT(__stage0_vectors_start == STAGE0_IMAGE_ORIGIN, "stage0 vector table offset changed")
ASSERT(ADDR(.stage0_descriptor) == STAGE0_DESCRIPTOR_ADDR, "stage0 descriptor offset changed")
ASSERT(bb02_stage0_descriptor == STAGE0_DESCRIPTOR_ADDR, "stage0 descriptor symbol changed")
ASSERT(SIZEOF(.stage0_descriptor) == STAGE0_DESCRIPTOR_LEN, "stage0 descriptor size changed")
ASSERT(_etext <= STAGE0_DESCRIPTOR_ADDR, "stage0 text overlaps descriptor")
ASSERT(LOADADDR(.relocate) + SIZEOF(.relocate) <= STAGE0_DESCRIPTOR_ADDR, "stage0 data load overlaps descriptor")
ASSERT(ADDR(.stage0_descriptor) + SIZEOF(.stage0_descriptor) == STAGE0_IMAGE_ORIGIN + STAGE0_IMAGE_LEN, "stage0 descriptor end changed")
