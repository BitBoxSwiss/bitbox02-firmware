# Reproducible builds

The BitBox02 firmware [releases](https://github.com/BitBoxSwiss/bitbox02-firmware/releases/) are
tagged: `firmware/vX.Y.Z` for the Multi edition, and `firmware-btc-only/vX.Y.Z` for the Bitcoin-only
edition. The binaries are built from those tags in a reproducible manner, based on fixed versions of
all dependencies. We use Docker to fix those dependencies.

The unsigned firmware binaries are the same for both **BitBox02** and **BitBox02 Nova**.

*Note*: it is possible to reproduce the binaries without Docker by installing the correct
dependencies. The instructions below use Docker however, as it makes it a easier to get started.

## What is the purpose of this?

The purpose is twofold:

- It is much more technically involved to actually build the binaries from source, than it is to
  verify signatures using `gpg` on the command line or in a GUI. This gives users who are willing
  and eager to verify the option to verify our releases even if they are unable to build the
  binaries themselves.
- The firmware binaries are only practically reproducible for a limited amount of time after the
  release. In the future, the dependencies of our Dockerfile might have changed, or Docker itself
  might even become incompatible or obsolete. This historic record allows you to be reasonably sure
  that the released binary was created correctly, even if it becomes infeasible to re-build the
  binary.

## Verify assertions by the community

Inspect the `assertion.txt` file of the relevant subdir,
e.g. [firmware-v4.1.0/assertion.txt](firmware-v4.1.0/assertion.txt).

The assertion.txt is created by the maintainers of this repo with every release.

Verify a signature confirming its contents, for example:

```sh
cd firmware-v4.1.0/
# import any missing public keys
gpg --import pubkeys/benma.asc
gpg --verify assertion-benma.sig assertion.txt
```

A valid signature means that the signer confirms that they could reproduce the binary from the
stated version tag.

You can check that the released signed firmware (usually named `firmware.vX.Y.Z.signed.bin`)
contains the unsigned binary with the following script (please install the [BitBox02 Python Library](../BUILD.md#BitBox02-Python-library) first):

```sh
./describe_signed_firmware.py firmware.vX.Y.Z.signed.bin
```

## Verify the hash as shown by the BitBox02 at startup

When installing new firmware on the BitBox02, an option can be turned on to display the firmware
hash on the device screen when the BitBox02 is plugged in. The hash shown is a hash of the firmware
and the firmware version: `sha256d(<version><padded firmware>)`.

It can be verified with the same tool as above. For example:

```sh
$ ./describe_signed_firmware.py firmware.v9.0.0.signed.bin
The following information assumes the provided binary was signed correctly; the signatures are not being verified.
This is a Multi-edition firmware.
The hash of the unsigned firmware binary is (compare with reproducible build):
19f692a769b22abe889849d7c2987688c57489a3437e70f649baef825a6788c6
The monotonic firmware version is: 11
The hash of the firmware as verified/shown by the bootloader is:
aae5fd961e552cb40ee9ef0f0060b96fb8a4f13f89168560c156010337f34b6a
```

## Contribute your signature

We kindly ask you to independently build the firmware binaries we released, and verify that you get
the same result. Please open a PR to add your signed message confirming this. Signing a confirmation
like this *only* confirms that you got the same result. It *does not* endorse the contents or
quality of the binary itself.

### How to reproduce

Run `./build.sh <version tag> <make command>`, e.g.:

```sh
./build.sh firmware/v4.1.0 "make firmware"
# or Bitcoin-only:
./build.sh firmware-btc-only/v4.1.0 "make firmware-btc"
```

This script is very simple and you can review it or run all the steps inside manually.

When it successfully finishes, print the sha256 hash of the binary:

```sh
# linux
sha256sum temp/build/bin/firmware.bin         # multi edition firmware
sha256sum temp/build/bin/firmware-btc.bin     # bitcoin-only firmware
# macOS
shasum -a 256 temp/build/bin/firmware.bin     # multi edition firmware
shasum -a 256 temp/build/bin/firmware-btc.bin # bitcoin-only firmware
```

### Contributing your signature

Please inspect the `assertion.txt` in the relevant subfolder,
e.g. [firmware-v4.1.0/assertion.txt](firmware-v4.1.0/assertion.txt). If you agree to its contents
and verified that the sha256 hash therein matches the one you got, please sign the file using:

```sh
cd firmware/v4.1.0 # go to the relevant subfolder
gpg -o assertion-YOURNAME.sig --detach-sign assertion.txt
```

Open a PR adding your signature file to this folder. Also add your pgp pubkey to the
[./pubkeys](./pubkeys) folder:

```sh
gpg --export --armor YOUR_PGP_KEY_ID  > ./pubkeys/YOURNAME.asc
```
