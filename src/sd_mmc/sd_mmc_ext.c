// SPDX-License-Identifier: Apache-2.0

/* Extensions to the ASF4 lib */

#include "sd_mmc_ext.h"
#include "driver_init.h"

static void _pause_clock(struct _mci_sync_device* const mci_dev)
{
    ASSERT(mci_dev && mci_dev->hw);

    if (hri_sdhc_get_CCR_SDCLKEN_bit(mci_dev->hw)) {
        hri_sdhc_clear_CCR_SDCLKEN_bit(mci_dev->hw);
    }
}
static void _resume_clock(struct _mci_sync_device* const mci_dev)
{
    ASSERT(mci_dev && mci_dev->hw);

    if (!hri_sdhc_get_CCR_SDCLKEN_bit(mci_dev->hw)) {
        hri_sdhc_set_CCR_SDCLKEN_bit(mci_dev->hw);
    }
}

void sd_mmc_pause_clock(void)
{
    _pause_clock(&MCI_0.device);
}

void sd_mmc_resume_clock(void)
{
    _resume_clock(&MCI_0.device);
}
