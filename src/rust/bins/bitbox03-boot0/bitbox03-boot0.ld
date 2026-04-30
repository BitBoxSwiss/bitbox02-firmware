INCLUDE memory.x
REGION_ALIAS("FLASH", BOOT0_FLASH);

SECTIONS
{
  .segger_rtt (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt .segger_rtt.*));
    . = ALIGN(4);
  } > RAM :rtt

  .segger_rtt_buf (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt_buf .segger_rtt_buf.*));
    . = ALIGN(4);
  } > RAM :rtt
} INSERT BEFORE .data;

PROVIDE(_flash_start = ORIGIN(BOOT0_FLASH));

INCLUDE bitbox03-common.ld

ASSERT(ADDR(.segger_rtt) == ORIGIN(RAM), "
ERROR(boot0): RTT control block must start immediately after BOOT_ARGS.");

ASSERT(SIZEOF(.segger_rtt) + SIZEOF(.segger_rtt_buf) <= 4K, "
ERROR(boot0): RTT sections do not fit into the OpenOCD scan window.");
