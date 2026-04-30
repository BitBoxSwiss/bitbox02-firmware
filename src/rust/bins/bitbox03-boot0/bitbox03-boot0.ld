INCLUDE memory.x
REGION_ALIAS("IMMUTABLE", IMMUTABLE_DATA);
REGION_ALIAS("FLASH", BOOT0_FLASH);

SECTIONS
{
  .immutable_data ORIGIN(IMMUTABLE) :
  {
    . = ALIGN(4);
    KEEP(*(.immutable_data .immutable_data.*));
    . = ALIGN(4);
  } > IMMUTABLE
} INSERT BEFORE .vector_table;

PROVIDE(_flash_start = ORIGIN(BOOT0_FLASH));

INCLUDE bitbox03-common.ld

ASSERT(ADDR(.immutable_data) == ORIGIN(IMMUTABLE), "
ERROR(cortex-m-rt): Immutable data must start at the beginning of the immutable flash page.");

ASSERT(SIZEOF(.immutable_data) == LENGTH(IMMUTABLE), "
ERROR(cortex-m-rt): Immutable data must fill the immutable flash page.");
