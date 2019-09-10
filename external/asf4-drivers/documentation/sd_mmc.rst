SD/MMC/SDIO
===========

The SD/MMC stack supports SD/MMC memory card and SDIO card.

The stack should be initialized with right HAL instance. Then sd_mmc_check is
be used to detect and mount the card. When the card is ready, the block
read/write functions are used to access the data on card. Note that the
read/write functions split the read/write process to init/start/wait_end, so
that user has chance to control the whole data process to remove overheads.
Note if used MCI OS driver version,the operation for SD/MMC should be in os task.

Features
--------

* Card initialization
* Card detection and protection detection
* Card type and capacity check
* Card mount/unmount
* Card read/write

Dependencies
------------

* Peripheral that supports multimedia card interface (MCI)

Limitations
-----------

* No SPI interface support yet
* Support only one peripheral instance
