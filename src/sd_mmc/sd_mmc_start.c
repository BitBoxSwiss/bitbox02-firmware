#include <sd_mmc.h>
#include <sd_mmc/sd_mmc_start.h>

// Card Detect (CD) {pin, detection level}
static sd_mmc_detect_t _sd_access_card_detect[CONF_SD_MMC_MEM_CNT] = {
    {PIN_SD_CD, 0},
};

// Write Protect (WP) {pin, detection level}
static sd_mmc_detect_t _sd_access_write_protect[CONF_SD_MMC_MEM_CNT] = {
    {-1, 1},
};

void sd_mmc_start(void)
{
    sd_mmc_init(&MCI_0, _sd_access_card_detect, _sd_access_write_protect);
}
