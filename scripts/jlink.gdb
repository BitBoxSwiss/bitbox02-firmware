# Connect to jlink gdb server
target extended-remote :2331

# It seems more reliable to reset the chip before loading the new firmware. It
# is also how they do it in the example in the wiki:
# https://kb.segger.com/J-Link_GDB_Server#Console

# Reset the CPU
monitor reset

# load the firmware into ROM
load

# Set VTOR (Vector Table Offset Register) to where the firmware is located
set *(uint32_t*)0xE000ED08=0x10000
# Set stack pointer to initial stack pointer according to exception table.
set $sp = *(uint32_t*)0x10000
# Set the program counter to the reset handler (second item in exception table)
set $pc = *(uint32_t*)0x10004

#break Reset_Handler
#break HardFault_Handler
#break NMI_Handler
#break MemManage_Handler

# start running
# change `continue` to `stepi` to stop execution at the start if you want to set breakpoints etc.
continue
