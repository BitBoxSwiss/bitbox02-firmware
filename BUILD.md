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
- protobuf-compiler
- [protoc-gen-nanopb](https://jpa.kapsi.fi/nanopb/download/)
  - Choose appropriate linux/macosx/windows file which includes the binaries
  - Add generator-bin/ to your path
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

```
make dockerinit
make dockerdev
```

The docker container will not allow you to access the hosts USB devices by default which means that
it is necessary to flash the device in a terminal not running in docker.

### Install development environment on macOS

Make sure you have [Homebrew](https://brew.sh) installed.
Install the dependencies with:

```bash
brew install hidapi cmake protobuf
brew install automake libtool # for building some code in the external/ folder
brew tap osx-cross/arm
brew install arm-gcc-bin
```

Add the following directory in this repository to your `PATH` in `~/.bash_login`:

```bash
export PATH="$PATH:[…]/bitbox02-firmware/tools/nanopb/generator"
```

## Instructions

Connect the J-Link to the debug pins on the BitBox02 prototype board.

Plug in both the J-Link hardware and the BitBox02 device into USB ports on your computer or a hub connected to your computer.

Build the firmware:
```
git clone --recurse-submodules https://github.com/digitalbitbox/bitbox02-firmware && cd bitbox02-firmware
# or via ssh
git clone --recurse-submodules git@github.com:digitalbitbox/bitbox02-firmware.git && cd bitbox02-firmware
make firmware # requires a GNU ARM toolchain for cross-compiling
```

If you have already cloned the repository without the `--recurse-submodules` argument, run:
```
git submodule update --init --recursive
```

Build the bootloader:
```
make bootloader
```

(to create a bootloader for a devdevice or a production device, use `make bootloader-devdevice` or
`make bootloader-production` respectively).

Load the bootloader by JLink (requires JLinkExe in PATH).
```
make jlink-flash-bootloader
```

You need to install the [BitBox02 Python Library](py/README.md) before you can flash the built firmware.

Load the firmware by the bootloader (requires loading bootloader.bin by JLink, if not already loaded on the device):
```
make flash-dev-firmware
```

Load the firmware by JLink:
```
make jlink-flash-firmware
```

### Build reference documentation (Doxygen)

Dependencies:
```
brew install graphviz doxygen
```

Build:
```
make docs
```

To view the results, open `build/docs/html/index.html` in a web browser.

### BitBox02 Python library

There is a Python api library in `py/bitbox02`.

Run `pip install -r py/requirements.txt -r py/bitbox02/requirements.txt` to install the deps (virtualenv recommended).

`make -C py/bitbox02` to generate the protobuf files.

To kick off some api calls:

```
./py/send_message.py
```

### Unit tests

We are using CMocka (https://cmocka.org/) for unit tests. To run the tests, the CMocka library
needs to be installed on your system.

If you're on a Mac, you can use the brew command to install it:

```
brew install cmocka
```

Alternatively, you can get CMocka by cloning the git repository and following these instructions:

```
git clone git://git.cryptomilk.org/projects/cmocka.git
cd cmocka
mkdir build && cd build
cmake ..
make && sudo make install
```

By default, the library will be installed into /usr/local/lib64 directory under Linux x86\_64.
If the library is not on the library path by default, you might need to export the following environment variable:

```
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib64/
```

Then you can run the tests by executing

```
make run-unit-tests # or make -C build-build test
```

### Coverage

gcovr or lcov/genhtml can be used to generate HTML coverage reports using the following commands:

```
make coverage # or make -C build-build coverage
```

```
make -C build-build coverage-lcovr
```

### Device tests

If you have a developer device at hand you can run device tests on it. Device tests help to verify functionality on an actual device.
Feedback can be provided via the screen or via USB. They are especially useful to test low-level, driver-specific features.

Device tests replace the source file where the main function resides, but otherwise have access to all firmware functions.

#### Code and build structure

The python scripts for setting up and running the test can be found under `test/device-test`. Firmware test code can be found
under `test/device-test/src`. `test/device-test/src/common` contains common C code functions that many test cases might need.

The object and binary files are built into `test/device-test/build`.

#### How to set up a test

The `test/device-test/setup_test.py` script assists you in building the binary with a given test case, flashing the device
with the resulting `device-test.bin` that gets built into `test/device-test/build/bin` and resetting the device so that the test is started.

You can run `setup_test.py` as follows:

```
./test/device-test/setup_test.py -t test/device-test/src/startup_test.c
```

If you run it successfully, the device should print `Integration test` on the screen.

#### How to write a test

The test becomes more interesting as we add the ability to function in a python script.
Here is an example for a python test:

```
#!/usr/bin/env python

from setup_test import *
import sys
import time

# required to find py/dbb_utils.py
test_dir = os.path.dirname(os.path.realpath(__file__))
sys.path.insert(0, test_dir + '/../py/')

from dbb_utils import *

def main(argv):
    test = "src/test_usb_hww_ep_in.c"
    setup(argv, test)
    run_test(test)

def run_test(testfile):
    print("execute testcase for " + testfile)
    print("expecting 'Hi HWW!' as a reply")

    try:
        openSpecificHid(USB_HWW)
        time.sleep(5)
        reply = hid_send_and_read_plain('Hi HWW!', 5)

    except IOError as ex:
        print(ex)
    except(KeyboardInterrupt, SystemExit):
        print("Exiting code")
    dbb_hid.close()

if __name__ == "__main__":
    main(sys.argv[1:])
```

The test passes the C test file to `setup()`. Afterwards it executed `run_test()`.
In `run_test`, it opens the HID interface `HWW` and sends some data over USB.
