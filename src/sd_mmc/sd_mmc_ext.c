// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
