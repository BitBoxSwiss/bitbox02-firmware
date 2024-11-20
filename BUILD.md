<img src="./doc/BB02_logo_github.svg" width="345px"/>

# Build BitBox02 firmware and bootloader

## Reporting issues

<!-- TODO: Write section on reporting issues -->

For security related issues please see [SECURITY.md](SECURITY.md).

## Development environment

There is a container image with all the build dependencies and there are some
`make` shortcuts to use it.

> [!TIP]
> It is highly recommended to use the container for development.

Accessing USB devices, like the flashing tool and the bitbox, is easier outside
of the container. So it is recommended to install the J-Link Software on your
development machine to follow the instructions below.

### Development Dependencies*

| Dependency | Version** |
| ---------- | -------- |
| [Arm GNU Toolchain](https://developer.arm.com/downloads/-/gnu-rm) | 8-2018-q4 |
| [HIDAPI](https://github.com/signal11/hidapi) | 0.11.2 |
| [cmake](https://cmake.org/download/) | 3.10 |
| [git](https://git-scm.com/downloads) | 2.34 |
| [Protobuf Compiler](https://github.com/protocolbuffers/protobuf/releases) | 21.2 |
| [Python Probobuf Runtime](https://github.com/protocolbuffers/protobuf/tree/master/python#installation) | 5.27.3 |
| [SEGGER J-Link Software and Documentation Pack](https://www.segger.com/downloads/jlink) | 6.34g |
| Graphviz | 2.42.2 |
| Doxygen | 1.9.1 |
| [cmocka](https://cmocka.org/files/1.1/) | 1.1.5 |

<sub>* See the complete list of dependences in the Dockerfile.</sub>

<sub>** The versions here are known to be working. Newer versions should
work.</sub>

### Setup containerized environment

Run the following commands to fetch the container image and run it:

```sh
make dockerpull
make dockerdev
```

`dockerpull` will use `docker pull` to fetch the current container image.
`dockerdev` will use `docker run` and `docker exec` to run a container in the
background and enter it. `dockerdev` will mount the project root using the same
path inside the container, which lets you use your preferred editor/IDE outside
the container.

> [!NOTE]
> The current development container is defined in
> [.containerversion](.containerversion). This is the version that is pulled
> with `dockerpull` and built with `dockerinit`.

> [!NOTE]
> `make dockerdev` will enter an already running container if it exists.

Run the following command to build the container:

```sh
make dockerinit
```

`dockerinit` is a shortcut to run `docker build`. Use this if you need to
permanently update the container image ([Dockerfile](Dockerfile)). Don't forget
to update the [container version file](.containerversion).

> [!TIP]
> For temporary changes you should enter the container running `docker exec`
> with user id 0.

### Setup development environment on macOS with brew

> [!CAUTION]
> Brew usually only supports the latest versions of software packages. It is
> not easy to get a working development environment using brew. Any
> discrepancies between your environment and the containerized environment may
> lead to CI build failures, since CI uses the container.

> [!IMPORTANT]
> If you use compiler versions different from CI you will not be able to
> reproducibly build the firmware. Different compilers typically lead to
> slightly different binary outputs.

Make sure you have [Homebrew](https://brew.sh) installed. Install the
dependencies with:

```sh
brew install hidapi cmake protobuf@21
brew install automake libtool
brew tap osx-cross/arm
brew install arm-gcc-bin
```

## Contributor instructions

### Check out the repository

#### 1. Fork the repository on github.

Go to [bitbox02-firmware](https://github.com/bitboxswiss/bitbox02-firmware) and fork the repository.

#### 2. Check out your fork

Run the following commands to check out your fork:

```sh
git clone --recurse-submodules git@github.com:<username>/bitbox02-firmware.git
cd bitbox02-firmware
```

> [!TIP]
> If you have already cloned the repository without the `--recurse-submodules`
> argument, run:
>
> ```sh
> git submodule update --init --recursive
> ```

> [!TIP]
> Add the original repo as a second remote so that you can sync the `master` branch.
> ```
> git remote add upstream https://github.com/bitboxswiss/bitbox02-firmware
> ```

### Build the firmware

Run the following commands to enter the container and build the firmware:

```sh
make dockerdev
make firmware
```

> [!TIP]
> If you have multiple cores you can speed up compilation by passing `-j<N>`, for example `-j8`.

### Build the bootloader

Run the following commands to enter the container and build the bootloader:

```sh
make dockerdev
make bootloader
```

> [!NOTE]
> To create a bootloader for a development or a production device, use `make
> bootloader-devdevice` or `make bootloader-production` respectively.

> [!NOTE]
> To run unsigned firmwares you need a development bootloader.

### Build the simulator

The Multi edition firmware can be built as a simulator for linux-amd64. To build it, run:

```sh
make simulator
```

### Flash instructions

#### Connect J-Link probe

Connect the J-Link probe to the debug pins on the BitBox02 prototype board. The
pinout of the board and the Arm JTAG/SWD 10-pin connector can be seen in the
table below.

| Signal | Bitbox02 # | Arm JTAG/SWD # |
| ------ | ---------- | -------------- |
| VCC    | 1          | 1              |
| CLK    | 2          | 4              |
| GND    | 3          | 3, 5           |
| DIO    | 4          | 2              |

See [bitbox schematics](doc/bb02_v2.10_schematics.pdf) and [Arm JTAG/SWD
interface](https://developer.arm.com/documentation/101636/0100/Debug-and-Trace/JTAG-SWD-Interface)

Plug **both** the J-Link probe and the BitBox02 into the computer using USB. A
USB hub can be used.

#### Flash bootloader using J-Link

Load the bootloader by JLink (requires `JLinkExe` in `$PATH`).

```sh
make jlink-flash-bootloader
```

> [!NOTE]
> To flash a bootloader for a development device
> `make jlink-flash-bootloader-development`.

#### Flash firmware using J-Link

Load the firmware by JLink:

```sh
make jlink-flash-firmware
```

#### Flash firmware using bootloader and python cli client

> [!TIP]
> This method does not require a J-Link probe while developing.

Install the [BitBox02 Python CLI client](#bitbox02-python-cli-client).

Load the firmware through the bootloader:

```sh
make flash-dev-firmware
```

### Build reference documentation (Doxygen)

```sh
make docs
```

To view the results, open `build/docs/html/index.html` in a web browser.

### Debugging

#### Debugging using the simulator

Run it with:

```sh
./build-build/bin/simulator
```

This launches a server simulating the firmware. The send_message tool can connect to it with:

    ./py/send_message.py --simulator

If you choose to create a wallet by restoring a mnemonic, the simulator will automatically use this
mnemonic:

    boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide


#### Debugging using the J-Link probe and GDB

The *debug firmware* enables pretty printing of panics over [RTT](https://www.segger.com/products/debug-probes/j-link/technology/about-real-time-transfer/).

Run the following commands to build the debug firmware.

```sh
make dockerdev
make firmware-debug
```

Run the following command to run the J-Link GDB Server.

```sh
make jlink-gdb-server
```

> [!IMPORTANT]
> The J-Link GDB Server must be left running in the background.

Run the following command to connect with telnet to the J-Link GDB Server to
see the RTT output.

```sh
make rtt-client
```

Run the following command to run GDB. GDB will connect to the J-Link GDB
server, flash the debug firmware and then start execution from the bootloader
(as if the device was just plugged in).

```sh
make run-debug
```

> [!TIP]
> After rebuilding the firmware, exit GDB and rerun `run-debug` to flash and reset the device.

> [!TIP]
> The initial set of GDB commands that are run are specified in the [gdb init
> script](./scripts/jlink.gdb). You may want to modify it if you are debugging
> something specific.

> [!TIP]
> In debug builds you can use the following functions to log:
> ```c
> util_log(fmt, args...)
> ```
> ```rust
> use ::util::log::log!(fmt, args...)
> ```
> in C you can also format with hex using `util_dbg_hex`:
> ```c
> uint8_t arr[] = {1,2};
> util_log("%s", util_dbg_hex(arr, sizeof(arr)));
> ```
> in rust you can format with hex using the built in hex formatter or the hex
> crate:
> ```rust
> let arr = [1, 2];
> log!("{:02x?}", arr)
> log!("{}", hex::encode(arr))
> ```

### Unit tests

CMocka [https://cmocka.org/](https://cmocka.org/) is used for mocking in the
unit tests. To compile the tests, the CMocka library needs to be installed on
your system. CMocka is available through most package managers, like *brew* and
*apt*.

> [!NOTE]
> If you compiled it yourself from souce, the library will, by default, be
> installed into **/usr/local/** directory instead of **/usr/**.
> If the library is not on the library path by default, you might need to export
> the following environment variable:
> ```sh
> export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib64/
> ```

Then you can run the tests by executing

```sh
make run-unit-tests # or make -C build-build test
```

Rust unit tests, if not invoked via `make run-rust-unit-tests`, must be run with
`-- --test-threads 1` due to unsafe concurrent access to `SafeData`, `mock_sd()` and `mock_memory()`.

### Coverage

gcovr or lcov/genhtml can be used to generate HTML coverage reports using the following commands:

```sh
make coverage # or make -C build-build coverage
```

```sh
make -C build-build coverage-lcovr
```

### SCCache / CCache

The build systems supports sccache/ccache, you just need to have it available
in your path. You can install it into your dev container with the following
commands:

```
docker exec -u 0 -it bitbox02-firmware-dev bash -c 'apt update && apt install -y libssl-dev && CARGO_HOME=/opt/cargo cargo install --locked sccache'
```

## BitBox02 Python Library

There is a Python api library in `py/bitbox02`.

### BitBox02 CLI client

Run `pip install -r py/requirements.txt` to install the deps (virtualenv recommended).

`make -C py/bitbox02` to generate the protobuf files.

To kick off some api calls:

```sh
./py/send_message.py
```
