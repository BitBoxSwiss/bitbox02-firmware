INCLUDE memory.x
REGION_ALIAS("HEADER", BOOT1_HEADER);
REGION_ALIAS("FLASH", BOOT1_FLASH);

SECTIONS
{
  .image_header ORIGIN(HEADER) :
  {
    . = ALIGN(4);
    KEEP(*(.image_header .image_header.*));
    . = ALIGN(4);
  } > HEADER
} INSERT BEFORE .vector_table;

SECTIONS
{
  .segger_rtt (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt .segger_rtt.*));
    . = ALIGN(4);
  } > RAM

  .segger_rtt_buf (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    KEEP(*(.segger_rtt_buf .segger_rtt_buf.*));
    . = ALIGN(4);
  } > RAM
} INSERT BEFORE .data;

PROVIDE(_flash_start = ORIGIN(HEADER) + LENGTH(HEADER));

INCLUDE bitbox03-common.ld

ASSERT(ADDR(.image_header) == ORIGIN(HEADER), "
ERROR(cortex-m-rt): The image header must start at the beginning of the HEADER memory.");

ASSERT(SIZEOF(.image_header) <= LENGTH(HEADER), "
ERROR(cortex-m-rt): The image header does not fit in the HEADER memory.");

ASSERT(ADDR(.vector_table) == ORIGIN(HEADER) + LENGTH(HEADER), "
ERROR(cortex-m-rt): The vector table must be placed immediately after the image header.");

ASSERT(ADDR(.segger_rtt) == ORIGIN(RAM), "
ERROR(boot1): RTT control block must start immediately after BOOT_ARGS.");

ASSERT(SIZEOF(.segger_rtt) + SIZEOF(.segger_rtt_buf) <= 4K, "
ERROR(boot1): RTT sections do not fit into the OpenOCD scan window.");
