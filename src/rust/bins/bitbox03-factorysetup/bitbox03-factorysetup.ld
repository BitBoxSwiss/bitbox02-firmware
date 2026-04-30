INCLUDE memory.x
REGION_ALIAS("FLASH", FACTORYSETUP_TEXT);
REGION_ALIAS("RAM", FACTORYSETUP_DATA);
PROVIDE(_flash_start = ORIGIN(FACTORYSETUP_TEXT));

SECTIONS
{
  .segger_rtt (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt .segger_rtt.*));
    . = ALIGN(4);
  } > FACTORYSETUP_RTT

  .segger_rtt_buf (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt_buf .segger_rtt_buf.*));
    . = ALIGN(4);
  } > FACTORYSETUP_RTT
} INSERT BEFORE .vector_table;

INCLUDE bitbox03-common.ld

ASSERT(ADDR(.segger_rtt) == ORIGIN(FACTORYSETUP_RTT), "
ERROR(factorysetup): RTT control block must start immediately after BOOT_ARGS.");

ASSERT(SIZEOF(.segger_rtt) + SIZEOF(.segger_rtt_buf) <= LENGTH(FACTORYSETUP_RTT), "
ERROR(factorysetup): RTT sections do not fit into FACTORYSETUP_RTT.");
