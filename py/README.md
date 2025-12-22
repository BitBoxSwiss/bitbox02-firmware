# BitBox python scripts

This directory contains scripts to talk to the BitBox device directly via the command line
 (e.g. `send_message.py`, `load_firmware.py`).

## Setup

These instructions use Python 3, pip, and venv (a lightweight "virtual environment").
Inside the virtual environment, `python` and `pip` refer to Python 3.

All commands below assume you are in the `py/bitbox02` directory.

### Requirements

- python
- pip >= 25

Editable installs (`pip install -e …`) are only supported with pip 25 or newer.  
Older pip versions will fail due to changes in how editable installs are handled (PEP 660).

### Installing dependencies

Install the required Python dependencies listed in `requirements.txt`:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

## Activate the virtual environment

If you open a new shell, remember to re-activate the virtual environment,
to use the scripts and communicate with the BitBox.

```
source .venv/bin/activate
```

You can deactivate a virtual environment by typing `deactivate` in your shell.


## Communicate with the BitBox

This assumes that the firmware was installed on the BitBox device. To flash the
firmware manually read the next section or install with the official BitBoxApp.

Connect your BitBox and "tap this side".

List and execute all available commands by running:

```bash
python ./send_message.py
```

This command will list what commands are currently possible, depending on which
mode the device currently is, i.e "Bootloader mode" accepts different commands.
From here you can execute any command the BitBox accepts.

```
What would you like to do?
- (1) List device info
- (2) Change device name
- (3) Get root fingerprint
- (4) Retrieve zpub of first account
- (5) Retrieve multiple xpubs
- (6) …
```

When connecting the first time to an initialized but unpaired BitBox, the device
will prompt to unlock and continue to compare and confirm the Noise pairing key.
This is a one-time action.


## Flash the firmware.bin

Use the following script to flash the firmware.bin onto the BitBox.
The script will prompt to enter the bootloader on the device before flashing.

Production devices only accept `./firmware.signed.bin` signed by BitBox.

```bash
python ./load_firmware.py ./firmware.signed.bin
```

Please note:
On production devices the bootloader only accepts newer signed
firmware versions and
[prevents downgrades](https://bitbox.swiss/bitbox02/security-features/#secure-bootloader).

On dev-devices use the `--debug` flag to flash unsigned `./firmware.bin`.

```bash
python ./load_firmware.py --debug ./firmware.bin
```

Contributors that don't have a dev-devices please refer to the
[simulator](https://github.com/BitBoxSwiss/bitbox02-firmware?tab=readme-ov-file#simulator).

For building the BitBox firmware please refer to the
[reproduce the firmware](https://github.com/BitBoxSwiss/bitbox02-firmware/tree/master/releases#reproducible-builds) documentation.


## Development

To work on the library or scripts, install them in editable mode.
Editable installs are only needed if you want to modify the scripts or library code.

```bash
pip install -e ./bitbox02
```

For developing the Python sources, almost all of it can be done on the host easily.
To regenerate protobufs it is recommended to use the Docker container.
Read more about dockerized setup in [BUILD.md](../BUILD.md).
