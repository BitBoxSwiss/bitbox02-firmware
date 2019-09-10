DISK IO
=======
Disk IO provides common disk access interfaces for different storage medias
It supports widely used disk access APIs like initialize/read/write/status etc.

Storage Media
-------------

The different storage media can be SD/MMC, USB, Flash etc.
Storage media has the dependency to the low level driver modules.


Dependencies
------------

::

         DISKIO
           |
           |
         SD/MMC
           |
           |
          MCI

* SD/MMC media accessor
* Peripheral that supports multimedia card interface (MCI)

API Example
------------

::

	dstatus_t status;
	dresult_t res;
	status = disk_initialize( 0 /*drv*/);
	if(0 == status){
		uint8_t buffer[10]="Sample Data";
		status = disk_status(0);
		switch(status){
		case 0:
			res = disk_write(0 /*drv*/, buffer, 7 /*Sector*/, 512 /*count*/);
			if(RES_OK == res){
				res = disk_read(0 /*drv*/, buffer, 7 /*Sector*/, 512 /*count*/);
				if(RES_OK == res)
					return 0;
			}
			break;
		case STA_NODISK:
			//Disk not preset, Note that FatFs does not refer this flag
			break;
		case STA_NOINIT:
			//Disk init failed
			break;
		case STA_PROTECT:
			//Disk is write protected
			break;
		}
	}
