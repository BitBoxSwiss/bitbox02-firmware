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

For automation and editor integrations, use `./scripts/dev_exec.sh <command>` as the project
entrypoint. It runs commands natively on the host by default and can be switched to container
execution with `BITBOX_FW_EXEC_MODE=docker`.

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
| Python | >= 3.10 |
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

Install [rustup](https://rustup.rs). The toolchain version is pinned by
`src/rust/rust-toolchain.toml`; rustup will download it automatically once you run `cargo` from
 within `src/rust` (or after setting a rustup override for the repository).

The build also invokes three tools that are not installed by the brew commands
above. Install them with the same versions as the container (see the
[Dockerfile](Dockerfile)):

```sh
cargo install cbindgen --version 0.29.2 --locked
cargo install bindgen-cli --version 0.72.1 --locked
cargo install --path tools/prost-build-proto --locked
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

### Configure and build

Inside the development container, run `make bootstrap` once to initialize submodules. From the
host, use `./scripts/dev_exec.sh make <target>` for builds (or set
`BITBOX_FW_EXEC_MODE=docker` to use the container toolchain).

```sh
make config
make firmware
make factorysetup
make bootloader-stage0
make bootloader-stage1
```

`make config` remembers selections in the git-ignored `config.mk`. It prompts for product, board,
edition (`multi` or `btc-only`), debug builds (`no` or `yes`), probe software, and SWD speed in kHz,
automatically selecting fields with one choice. `DEBUG` defaults to `no`; `SWD_SPEED` defaults to
`4000` for all products and both probe backends.
Probe hardware is fixed by the board:

| Product | Board | Probe | Software (default first) |
| --- | --- | --- | --- |
| `bitbox02` | `bitbox02` | J-Link | `jlink`, `openocd` |
| `bitbox02nova` | `bitbox02nova` | J-Link | `jlink`, `openocd` |
| `bitbox03` | `dev-kit` | ST-Link | `openocd` |
| `bitbox03` | `testboard` | J-Link | `openocd`, `jlink` |
| `bitbox03` | `bitbox03` (disabled) | J-Link | `jlink`, `openocd` |

BitBox03 supports `dev-kit` and `testboard` selections; the testboard defaults to OpenOCD with
the J-Link adapter and currently uses the dev-kit build target. The production board remains
disabled until its build support is implemented. Both BitBox03 editions currently build the same stub.

Without a configuration, defaults are BitBox02, multi edition, and J-Link software. Explicit
arguments run noninteractively; omitted fields use built-in defaults, independent of saved choices:

```sh
make config CONFIG_ARGS="--product bitbox02 --edition btc-only --probe-software openocd"
make config CONFIG_ARGS="--product bitbox03 --board dev-kit"
make config CONFIG_ARGS="--product bitbox03 --board testboard"
make config CONFIG_ARGS="--product bitbox03 --debug yes"
make config CONFIG_ARGS="--product bitbox03 --swd-speed 1000"
make config CONFIG_ARGS="--defaults"
```

SWD speed must be a positive integer. It is saved in `config.mk` and can also be overridden for a
command, for example `make debug-server SWD_SPEED=1000`.

The debug setting is also saved in `config.mk` and can be overridden per command with `DEBUG=yes`
or `DEBUG=no`. Build, flash, and run commands use the same configured profile.

Invalid input, EOF, or Ctrl-C preserves the previous file. Interactive product changes reset
incompatible saved choices. Run configuration separately from builds: `make config firmware` is
rejected because Make has already parsed the old settings. Configuration and compilation require
neither installed probe software nor attached hardware.

With `DEBUG=no`, BitBox02/Nova primary targets use `RelWithDebInfo` and BitBox03 uses Cargo release
builds. `DEBUG=yes` selects debug builds for all products, with their existing RTT features.
Firmware follows the selected edition; factorysetup is edition-independent; stage0/stage1 select
development images for the configured product and edition. Build directories are flat:

| Backend | Directory |
| --- | --- |
| BitBox02/Nova CMake | `build-<product>-cmake-<profile>` |
| BitBox03 Cargo | `build-bitbox03-cargo` |

For example, `build-bitbox02-cmake-relwithdebinfo/bin/firmware.elf`,
`build-bitbox02nova-cmake-debug/bin/firmware-btc.elf`, and
`build-bitbox03-cargo/thumbv8m.main-none-eabihf/debug/bitbox03-firmware`.
Cargo receives an absolute target directory; backend-managed contents keep their native layouts.
CMake Rust feature caches remain inside their product/profile directory. Images and editions share
those caches. Switching products and switching back reuses previous results; changing probe
software or SWD speed does not affect compilation. Configuration never cleans. `make clean` removes
build trees but preserves `config.mk`. Host tests and simulators still use `build-build` and
`build-build-noasan`.

`bootloader-stage{0,1}-{production,development}` builds all BitBox02/Nova editions, sequentially in
each product's directory. Production/development upgrade-firmware and upgrade-assets aliases are
retained. Named product commands always use that product's directory.

The old `firmware-btc`, `factory-setup`, per-product stage build shortcuts, and
`bitbox03-*` build shortcuts are replaced by configuration plus the primary targets above. Backend
targets and artifact basenames remain unchanged. Additional BitBox03 board/edition implementations
are deferred.

Pass `-j<N>` to speed up a build, for example `make -j8 firmware`. Top-level aliases run sequentially.

### Build the simulator

The Multi edition firmware can be built as a simulator for linux and macos. To
build it, run:

```sh
make simulator
```

### Build the graphical simulator

The Multi edition firmware can be built as a graphical simulator for linux and
macos. To build it, run:

```sh
make simulator-graphical
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

#### Flash or run a built image

Build first, then use the matching command. These commands consume existing ELF files and report
the build command if one is missing.

| Image | Program, verify, reset/run | Run in GDB |
| --- | --- | --- |
| Firmware | `make flash-firmware` | `make run-firmware` |
| Factorysetup | `make flash-factorysetup` (BitBox02/Nova only) | `make run-factorysetup` |
| Stage0 | `make flash-bootloader-stage0` | `make run-bootloader-stage0` |
| Stage1 | `make flash-bootloader-stage1` | `make run-bootloader-stage1` |

`run-<image>` selects the ELF built by `make <image>` with the same `DEBUG` setting and immediately
continues with GDB attached. For example, `make firmware DEBUG=yes` followed by
`make run-firmware DEBUG=yes` builds and runs the debug image.

The selected probe software chooses SEGGER Commander/GDB Server or OpenOCD. For BitBox02/Nova,
OpenOCD uses `scripts/openocd-bitbox02.cfg` with the J-Link adapter; install an OpenOCD build
containing the upstream
[`target/atsame5x.cfg`](https://raw.githubusercontent.com/openocd-org/openocd/master/tcl/target/atsame5x.cfg)
SAMD51 driver. BitBox03 uses the shared `scripts/openocd-bitbox03.cfg` with `target/stm32u5x.cfg`.
The product configuration supplies the board's adapter through `PROBE_ADAPTER`: ST-Link for the
dev-kit and J-Link for the testboard. Both share the reset and RTT work-area settings.

Start `make debug-server` in a separate terminal and leave it running in the foreground before
using a `run-*` command. SEGGER uses port 2331 with download verification enabled;
OpenOCD uses 3333. GDB resets/halts, loads the ELF, verifies sections, sets VTOR, SP and PC from the
image vector table, and issues `c`. It stays attached for Ctrl-C and breakpoints. The startup commands
live in [`scripts/openocd.gdb`](scripts/openocd.gdb) and [`scripts/jlink.gdb`](scripts/jlink.gdb).
Edit the script for your probe software to add breakpoints or comment out the final `c` to stop
at entry. The helper selects the script and supplies the image vector address each time.

Flashing and GDB use the built ELF directly for all products, including BitBox02/Nova stage1,
whose header is finalized during the build. Load addresses come from the ELF. J-Link Commander
uses `loadfile` with download verification. See the documentation for
[J-Link ELF loading](https://kb.segger.com/J-Link_Commander#LoadFile),
[GDB loading](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Target-Commands.html), and
[SEGGER download verification](https://kb.segger.com/J-Link_GDB_Server#-vd).

BitBox03 factorysetup is a RAM image and has no flash command. Use `make debug-server` followed by
`make run-factorysetup` with the matching `DEBUG` setting to load and verify it in RAM,
initialize the vector/entry state, and resume without resetting after loading. Flash commands
reset/run after programming.

#### Run J-Link Commander scripts

Run `.jlink` scripts directly with `JLinkExe` from the repository root. For example, to read the
shared memory area on BitBox02/Nova:

```sh
JLinkExe -NoGui 1 -if SWD -device ATSAMD51J20 -speed 4000 -autoconnect 1 -ExitOnError 1 \
    -CommanderScript scripts/print-memory-shared.jlink
```

Replace the script path with the required script, such as `scripts/set-new-screen.jlink`,
`scripts/reset-version.jlink`, or `scripts/bb02-set-factory-randomness.jlink`. These maintenance
scripts use BitBox02/Nova memory addresses. Run from the repository root so scripts can find
relative paths to data files.

`-speed` is the SWD speed in kHz. Direct `JLinkExe` commands do not read `config.mk`; specify the
device and speed explicitly. `ATSAMD51J20` is the device for both BitBox02 and BitBox02 Nova.

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

To view the results, open `build-bitbox02-cmake-relwithdebinfo/docs/html/index.html` in a web browser.

### Debugging

#### Debugging using the simulator

Run it with:

```sh
./build-build-noasan/bin/simulator
```

or the following if you built the graphical one:

```sh
./build-build-noasan/bin/simulator-graphical --preseed
```

This launches a server simulating the firmware. The send_message tool can
connect to it with:

    ./py/send_message.py --simulator

Both simulators can load the following seed:

    boring mistake dish oyster truth pigeon viable emerge sort crash wire
    portion cannon couple enact box walk height pull today solid off enable
    tide

The graphical simulator does it with the flag `--preseed`. The original
simulator loads it if you restore from mnemonic.


#### Debugging and RTT

Build the desired profile, start `make debug-server` in another terminal, then use the matching
run command. For example, use `make firmware` followed by `make run-firmware`, or
`make factorysetup DEBUG=yes` followed by `make run-factorysetup DEBUG=yes`.

Images with RTT enabled provide panic logging over
[RTT](https://www.segger.com/products/debug-probes/j-link/technology/about-real-time-transfer/).

Let the image run until its RTT channels have initialized. With OpenOCD, interrupt GDB, issue
`rtt_start`, and continue with `c`. This starts channel 0 on port 19021 and channel 1 (API traffic)
on port 19022. SEGGER provides RTT on port 19021. Connect with `make rtt-client`.
RTT availability follows existing build features; it is not enabled by selecting a probe backend.

After rebuilding, exit GDB and rerun the matching `run-*` command to reload the image.

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

### SCCache / CCache

The build systems supports sccache/ccache, you just need to have it available
in your path. You can install it into your dev container with the following
commands:

```
docker exec -u 0 -it bitbox02-firmware-dev bash -c 'apt update && apt install -y libssl-dev && CARGO_HOME=/opt/cargo cargo install --locked sccache'
```

## BitBox02 Python Library

There is a Python api library in `py/bitbox02`.

> [!IMPORTANT]
> The Python scripts require **Python ≥ 3.10**, and editable installs require **pip ≥ 25**.
> Older versions will fail.
> For setup and usage instructions, see [`py/README.md`](py/README.md).

### BitBox02 CLI client

Run `pip install -r py/requirements.txt` to install the deps (virtualenv recommended).

`make -C py/bitbox02` to generate the protobuf files.

To kick off some api calls:

```sh
./py/send_message.py
```
