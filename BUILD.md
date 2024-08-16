<img src="./doc/BB02_logo_github.svg" width="345px"/>

# Build BitBox02 firmware and bootloader

## Dependencies

- [HIDAPI](https://github.com/signal11/hidapi)
- [GNU ARM Embedded Toolchain](https://developer.arm.com/open-source/gnu-toolchain/gnu-rm/downloads)
- SEGGER J-Link software
  - [All packages and versions](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack)
    - Newer versions should work, but if not, go to "Older versions" and get version 6.34g
  - [OSX package](https://www.segger.com/downloads/jlink/JLink_MacOSX_V630d.pkg)
  - [Linux 64bit](https://www.segger.com/downloads/jlink/JLink_Linux_x86_64.tgz)
  - [others](https://www.segger.com/downloads/jlink/)
- cmake
- git
- Install the pre-built [protobuf python binary](https://github.com/protocolbuffers/protobuf/releases)
  - Then install the included [Python Protocol Buffers](https://github.com/protocolbuffers/protobuf/tree/master/python#installation) runtime library

## Reporting issues

<!-- TODO: Write section on reporting issues -->

For security related issues please see [SECURITY.md](SECURITY.md).

## Development environment

### Install development environment as a Docker container

The container will contain all tools needed to build the project but it is still necessary to get
the J-Link software to flash the bootloader.  Run the commands below to build the container and
execute a persistent one.

```sh
make dockerinit
make dockerdev
```

The docker container will not allow you to access the hosts USB devices by default which means that
it is necessary to flash the device in a terminal not running in docker.

> [!NOTE]
> Current development container is defined in the file `.containerversion`

### Install development environment on macOS

Make sure you have [Homebrew](https://brew.sh) installed.
Install the dependencies with:

```sh
brew install hidapi cmake protobuf
brew install automake libtool # for building some code in the external/ folder
brew tap osx-cross/arm
brew install arm-gcc-bin
```

## Instructions

Connect the J-Link to the debug pins on the BitBox02 prototype board.

Plug in both the J-Link hardware and the BitBox02 device into USB ports on your computer or a hub connected to your computer.

Build the firmware:

```sh
git clone --recurse-submodules https://github.com/digitalbitbox/bitbox02-firmware && cd bitbox02-firmware
# or via ssh
git clone --recurse-submodules git@github.com:digitalbitbox/bitbox02-firmware.git && cd bitbox02-firmware
make firmware # requires a GNU ARM toolchain for cross-compiling
```

If you have already cloned the repository without the `--recurse-submodules` argument, run:

```sh
git submodule update --init --recursive
```

Build the bootloader:

```sh
make bootloader
```

(to create a bootloader for a devdevice or a production device, use `make bootloader-devdevice` or
`make bootloader-production` respectively).

Load the bootloader by JLink (requires JLinkExe in PATH).

```sh
make jlink-flash-bootloader
```

You need to install the [BitBox02 Python Library](#BitBox02-Python-library) before you can flash the built firmware.

Load the firmware by the bootloader (requires loading bootloader.bin by JLink, if not already loaded on the device):

```sh
make flash-dev-firmware
```

Load the firmware by JLink:

```sh
make jlink-flash-firmware
```

### Build reference documentation (Doxygen)

Dependencies:

```sh
brew install graphviz doxygen
```

Build:

```sh
make docs
```

To view the results, open `build/docs/html/index.html` in a web browser.

### BitBox02 Python library

There is a Python api library in `py/bitbox02`.

Run `pip install -r py/requirements.txt` to install the deps (virtualenv recommended).

`make -C py/bitbox02` to generate the protobuf files.

To kick off some api calls:

```sh
./py/send_message.py
```

### Unit tests

We are using CMocka [https://cmocka.org/](https://cmocka.org/) for unit tests. To run the tests, the CMocka library
needs to be installed on your system.

If you're on a Mac, you can use the brew command to install it:

```sh
brew install cmocka
```

Alternatively, you can get CMocka by cloning the git repository and following these instructions:

```sh
git clone git://git.cryptomilk.org/projects/cmocka.git
cd cmocka
mkdir build && cd build
cmake ..
make && sudo make install
```

By default, the library will be installed into /usr/local/lib64 directory under Linux x86\_64.
If the library is not on the library path by default, you might need to export the following environment variable:

```sh
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib64/
```

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

The build systems supports sccache/ccache, you just need to have it available in your path. You can
install it into your dev container with the following commands:

```
docker exec -u 0 -it bitbox02-firmware-dev bash -c 'apt update && apt install -y libssl-dev && CARGO_HOME=/opt/cargo cargo install --locked sccache'
```
