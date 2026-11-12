# BitBox python scripts

This directory contains scripts to talk to the BitBox device directly via the command line
 (e.g. `send_message.py`, `load_firmware.py`).

## Setup

These instructions require Python 3.10 or newer, pip, and venv (a lightweight "virtual
environment"). Inside the virtual environment, `python` and `pip` refer to Python 3.

All commands below assume you are in the `py/bitbox02` directory.

### Requirements

- Python >= 3.10
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

### Manual screen walkthrough

Run the long interactive UI test with a disposable test device:

```bash
python ./send_message.py --test-ui
```

For the simulator, use `--test-ui --simulator` (and optionally `--simulator-port`).
Install `py/requirements.txt`, including `bech32` and `rlp`, first.

The walkthrough exercises startup and pairing, passwords, backups and recovery words,
Bitcoin addresses and transaction warnings, message display variants, BIP85, BitBoxSync,
passphrases, resets and recovery. Multi-coin firmware also exercises Litecoin, Ethereum,
EIP712, payment memos/swaps, Cardano and U2F. Bootloader screens are inspected manually
at the end. Payment-request fixtures deliberately have invalid signatures: inspect the
memo and swap screens, then expect the invalid-payment-request error.

Each case has instructions and run/skip controls. Afterwards, enter `p` for pass, `f` for
fail, `r` to repeat, or `s` to skip. Enter `q` at these prompts or press Ctrl-C to stop.
The final summary records your observations; it does not automatically verify the display.
Exit status is 0 when all applicable cases pass, 1 for failures/skips, and 130 when stopped.

The test changes device settings, registers test accounts, creates backups and offers factory
resets. Each reset requires typing `RESET` and confirming on the device. Keep test microSD
cards (one with a backup and one empty) and record the recovery words when prompted. Follow
reconnection instructions after resets and passphrase changes. Transactions are never broadcast.

Use a fresh test wallet to see account-registration screens that would otherwise be skipped
by the firmware. Hardware faults, successful validation of service-signed payment requests,
and firmware installation need separate testing. Simulator runs cannot verify physical
microSD behavior, U2F or the bootloader.


## Flash the firmware.bin

Use the following script to flash the firmware.bin onto the BitBox.
The script prompts to enter the bootloader when necessary and confirms the detected firmware and
device types before flashing.

```bash
python ./load_firmware.py ./firmware.signed.bin
```

Signed firmware is detected by its header; all other input is treated as raw unsigned firmware.
The file name is not used. A recognized but malformed signed-firmware container is rejected.

The supported combinations are:

| Firmware input | Production device | Development device |
| --- | --- | --- |
| Signed | Flash firmware and signature data; all errors are fatal | Flash firmware, attempt signature data, warn if the signature data is rejected, and reboot anyway |
| Raw unsigned | Warn that firmware verification will fail, then flash without signature data | Flash firmware without signature data |

On production devices the bootloader only accepts newer signed firmware versions and
[prevents downgrades](https://bitbox.swiss/bitbox02/security-features/#secure-bootloader). On a
production device, unsigned firmware cannot boot. A signed firmware for a different product or
edition is allowed after a warning, but installing its signature data is expected to fail.

Every flash requires confirmation. Use `-y` or `--yes` to skip the prompt for non-interactive use:

```bash
python ./load_firmware.py --yes ./firmware.bin
```

The deprecated `--debug` option is accepted for backwards compatibility but has no effect.

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
