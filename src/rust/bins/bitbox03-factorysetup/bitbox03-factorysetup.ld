INCLUDE memory.x
REGION_ALIAS("FLASH", RAM_CODE);
PROVIDE(_flash_start = ORIGIN(FLASH));

INCLUDE bitbox03-common.ld
