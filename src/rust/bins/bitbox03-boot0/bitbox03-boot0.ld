INCLUDE memory.x
REGION_ALIAS("FLASH", BOOT0_FLASH);

PROVIDE(_image_payload_start = ORIGIN(BOOT0_FLASH));

INCLUDE bitbox03-common.ld
