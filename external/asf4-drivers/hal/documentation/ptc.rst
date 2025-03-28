PTC Driver
==========

PTC - Peripheral Touch Controller.
The purpose of PTC is to acquire signals to detect touch on capacitive
sensors. The external capacitive touch sensor is typically formed on a
PCB, and the sensor electrodes are connected to the analog front end of
the PTC through the I/O pins in the device. The PTC supports both self-
and mutual-capacitance sensors.

In mutual-capacitance mode, sensing is done using capacitive touch 
matrices in various X-Y configurations, including indium tin oxide (ITO)
sensor grids. The PTC requires one pin per X-line and one pin per Y-line.

In self-capacitance mode, the PTC requires only one pin (Y-line) for each
touch sensor.
