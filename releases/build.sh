#!/bin/bash -e

# To maintainers: keep this script straight forward to follow and
# review.

# delete previous clone
rm -rf temp

# Copying repo at the specified version into `./temp`
git clone --depth 1 --branch $1 --recurse-submodules https://github.com/digitalbitbox/bitbox02-firmware temp

cd temp;

# The shallow clone above doesn't fetch tags. Even if only building the firmware, the CMakeLists.txt
# fetches the bootloader version using `./scripts/get_version bootloader`, which requires a
# bootloader tag. The build scripts can be changed to only use the firmware tag that is needed,
# ignoring the others, but we fetch the tags here so that builds of previous releases continue to
# work.
git fetch --tags;

# Build the Docker image (this can take a while):
docker build --pull --force-rm --no-cache -t bitbox02-firmware .

# Build the firmware.
#
# For firmware versions v4.1.0 and older, you'll need to manually install `python` inside the Docker
# container, as it is missing in the Dockerfile of that release.
#
# The safe.directory config is so that git commands work. even though the repo folder mounted in
# Docker is owned by root, which can be different from the owner on the host.
docker run -it --rm --volume `pwd`:/bb02 bitbox02-firmware bash -c "git config --global --add safe.directory /bb02 && cd /bb02 && $2"

echo "firmware.bin created at:"
echo `pwd`/build/bin/firmware.bin
echo "or"
echo `pwd`/build/bin/firmware-btc.bin
