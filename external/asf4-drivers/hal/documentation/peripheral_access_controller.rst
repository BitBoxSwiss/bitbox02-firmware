The Peripheral Access Controller Driver
=======================================

The Peripheral Access Controller provides write protection for registers of the
peripherals.

User can use periph_lock to enable a selected peripheral's write-protection,
periph_unlock to disable the selected peripheral's write-protection.

If a peripheral is write-protected, and if a write access is performed, data
will not be written.

Features
--------

* Lock(enable write-protection)
* Unlock(disable write-protection)
* Get the write-protection state

Applications
------------

* Protect critical peripheral in case of modification

Dependencies
------------

Peripheral access controller/Write-protection hardware

Concurrency
-----------

N/A

Limitations
-----------

N/A

Known issues and workarounds
----------------------------

double write-protection or double unprotection may lead to an access error