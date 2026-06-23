INCLUDE memory.x
REGION_ALIAS("FLASH", BOOT1_FLASH);

IMAGE_HEADER_LEN = 1024;

SECTIONS
{
  .image_header ORIGIN(FLASH) :
  {
    . = ALIGN(4);
    KEEP(*(.image_header .image_header.*));
    . = ALIGN(4);
  } > FLASH :flash
} INSERT BEFORE .vector_table;

PROVIDE(_flash_start = ORIGIN(FLASH) + IMAGE_HEADER_LEN);

INCLUDE bitbox03-common.ld

ASSERT(ADDR(.image_header) == ORIGIN(FLASH), "
ERROR(cortex-m-rt): The image header must start at the beginning of the FLASH memory.");

ASSERT(SIZEOF(.image_header) <= IMAGE_HEADER_LEN, "
ERROR(cortex-m-rt): The image header does not fit in the reserved header space.");

ASSERT(ADDR(.vector_table) == ORIGIN(FLASH) + IMAGE_HEADER_LEN, "
ERROR(cortex-m-rt): The vector table must be placed immediately after the image header.");
