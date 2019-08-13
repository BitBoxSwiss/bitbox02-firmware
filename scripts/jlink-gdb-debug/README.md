# Debugging with JLinkGDBServer

If the firmware is compiled with semihosting support, the input and output is redirected to the
developers computer. See instructions below.

## Compile and flash firmware with semihosting support

In the root directory of the project run:

```
make firmware-semihosting
make jlink-flash-firmware-semihosting
```


## Configure GDB

If you want to use the `.gdbinit` helper script you need to allow it by creating `~/.gdbinit` with
the following content:

```
set auto-load safe-path /
```

## Run JLinkGDBServer

Start the JLink GDB server with the following command:

```
JLinkGDBServer -if SWD -device ATSAMD51J20 -speed 4000 -ir
```

## Run GDB (with arm support)

The GDB init file, `.gdbinit`, contains the following commands:

```
file ../../build/bin/firmware-semihosting.elf
target remote :2331
monitor semihosting enable
monitor reset
```

First it will load the symbols from `firmware-semihosting.elf` then it will connect to the JLink GDB Server.
After that it will enable "semihosting" to redirect IO to the JLink GDB Server. Finally it will reset the
device.

Change your current working directory to the directory with the `.gdbinit` file and run `gdb`.

```
cd <project-dir>/scripts/jlink-gdb-debug
gdb-multiarch
```

## See input and output

In a new terminal, run `telnet localhost 2333` to connect to the IO of the device through the GDB
Server.

```
$ telnet localhost 2333
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
SEGGER J-Link GDB Server V6.44h - Terminal output channel
```


## Start debugging

To start debugging run the command `continue` in GDB.
