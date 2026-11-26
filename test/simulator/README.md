# Simulator

Build with `make simulator`, then run `build-build-noasan/bin/simulator`.
It starts with the graphical device UI and no seed. Pass `--nogui` for headless
mode, which automatically approves prompts and uses empty passwords and the
fixed recovery phrase documented in [BUILD.md](../../BUILD.md#debugging-using-the-simulator).
Pass `--preseed` to load that phrase at startup in either mode. `--port` defaults
to 15423.

Both modes log confirmation, transaction, text-input, and status screens to stdout
using the same format. This logging is enabled only in simulator builds.

## Signals

SIGINT and SIGTERM stop the simulator. SIGHUP disconnects the current client;
the process and wallet state survive, and another client can connect. SIGHUP
without a client has no effect.

Reset deletes the wallet and closes the client connection in both modes. The
simulator stays running and accepts a new connection with a fresh session.
`--preseed` applies only at startup, so reconnecting after reset leaves the wallet
unseeded.

## Compatibility tests

After building, run from the repository root with the Python dependencies installed:

```sh
PYTHONPATH=py/bitbox02 python test/simulator/test_simulator.py -v
cargo test --manifest-path test/simulator/Cargo.toml -- --test-threads 1
```

The integration tests remove display-server environment variables. They exercise
real Noise sessions, mnemonic workflows, signing, payment-request identities,
reset, signals, TCP framing, and reconnection. `--binary PATH` tests a packaged or
alternate executable. The transcript fixtures were captured from the C simulator;
only the random pairing code and selected port are normalized.
Terminal transcripts are checked on both platforms; the redirected-output fixture
uses Linux's C stdio buffering.
Reset is checked separately because it now disconnects instead of returning on the
existing session.

`--legacy` skips scenarios the C executable does not support. `--record` explicitly
updates the transcript fixtures when testing a reference executable.

The Linux release executable retains the filename reported by `--version`:
`bitbox02-multi-<firmware-version>-simulator1.0.0-linux-amd64`. It includes both modes.
The separate BitBox03 graphical simulator is unchanged.
