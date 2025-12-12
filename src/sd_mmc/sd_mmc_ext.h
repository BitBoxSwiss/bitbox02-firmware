// SPDX-License-Identifier: Apache-2.0

#ifndef SD_MMC_EXT_H
#define SD_MMC_EXT_H

// Turn off the bus clock
void sd_mmc_pause_clock(void);

// Turn on the bus clock
void sd_mmc_resume_clock(void);

#endif
