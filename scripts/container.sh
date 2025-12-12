#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

# This script wraps podman or docker or the specified container
# runtime.

if [ -n "$CONTAINER_RUNTIME" ]; then
  RUNTIME="$CONTAINER_RUNTIME"
elif command -v podman &>/dev/null; then
  RUNTIME=podman
else
  RUNTIME=docker
fi

$RUNTIME $@
