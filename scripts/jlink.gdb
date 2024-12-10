# Connect to jlink gdb server
target extended-remote :2331

# load the firmware into ROM
load

# Reset the CPU
monitor reset

#break Reset_Handler
#break HardFault_Handler
#break NMI_Handler
#break MemManage_Handler

# start running
# change `continue` to `stepi` to stop execution at the start if you want to set breakpoints etc.
continue
