# Start a JLink gdb server with the following command:
# $ JLinkGDBServer -if SWD -device ATSAMD51J20 -speed 4000 -ir
# See I/O using telnet:
# $ telnet localhost 2333
file ../../build/bin/firmware-semihosting.elf
target remote :2331
monitor semihosting enable
monitor reset
