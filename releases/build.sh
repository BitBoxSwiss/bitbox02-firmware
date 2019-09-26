#!/bin/bash -e

# To maintainers: keep this script straight forward to follow and
# review.

# delete previous clone
rm -rf temp

# Copying repo at the specified version into `./temp`
git clone --depth 1 --branch $1 --recurse-submodules https://github.com/digitalbitbox/bitbox02-firmware temp

cd temp;

# Build the Docker image (this can take a while):
docker build --pull --force-rm --no-cache -t bitbox02-firmware .

# Build the firmware. The inlined python package install can be
# removed after v4.1.0, but is necessary for v4.1.0 as it is missing
# in the Dockerfile of that release.
docker run -it --rm --volume `pwd`:/bb02 bitbox02-firmware bash -c "apt-get update && apt-get install -y python && cd /bb02 && $2"

echo "firmware.bin created at:"
echo `pwd`/build/bin/firmware.bin
echo "or"
echo `pwd`/build/bin/firmware-btc.bin
