# Connect to jlink gdb server
target extended-remote :2331

# It seems more reliable to reset the chip before loading the new firmware. It
# is also how they do it in the example in the wiki:
# https://kb.segger.com/J-Link_GDB_Server#Console

# Reset the CPU
monitor reset

# load the firmware into ROM
load

#break Reset_Handler
#break HardFault_Handler
#break NMI_Handler
#break MemManage_Handler

# start running
# change `continue` to `stepi` to stop execution at the start if you want to set breakpoints etc.
continue
