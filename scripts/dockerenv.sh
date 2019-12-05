#!/bin/bash -e
# Copyright 2019 Shift Cryptosecurity AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

if [ "$1" = "release" ] ; then
    MOUNT_DIR=/bb02
    CONTAINER_NAME_SUFFIX=rel
else
    MOUNT_DIR="$DIR/.."
    CONTAINER_NAME_SUFFIX=dev
fi

CONTAINER_IMAGE=shiftcrypto/firmware_v2
PROJECT_NAME="$(basename "$(realpath "$DIR/..")")"
CONTAINER_NAME="$PROJECT_NAME-$CONTAINER_NAME_SUFFIX"

dockerdev () {
    local repo_path="$DIR/.."

    if ! docker images --filter "reference=${CONTAINER_IMAGE}" | grep -q "${CONTAINER_IMAGE}"; then
        echo "No '${CONTAINER_IMAGE}' docker image found! Maybe you need to run
              'docker build --pull -t ${CONTAINER_IMAGE} .'?" >&2
        exit 1
    fi

    # If already running, enter the container.
    if docker ps --filter "name=^${CONTAINER_NAME}$" | grep -q "$CONTAINER_NAME"; then
        docker exec --user=dockeruser --workdir="$MOUNT_DIR" -it "$CONTAINER_NAME" bash
        return
    fi

    if docker ps --all --filter "name=^${CONTAINER_NAME}$" | grep -q "$CONTAINER_NAME"; then
        docker rm "$CONTAINER_NAME"
    fi

    # SYS_PTRACE is needed to run address sanitizer
    docker run \
           --detach \
           --interactive --tty \
           --name="$CONTAINER_NAME" \
           -v "$repo_path":"$MOUNT_DIR" \
           --cap-add SYS_PTRACE \
           ${CONTAINER_IMAGE} bash

    # Use same user/group id as on the host, so that files are not created as root in the mounted
    # volume.
    docker exec -it "$CONTAINER_NAME" groupadd -g "$(id -g)" dockergroup
    docker exec -it "$CONTAINER_NAME" useradd -u "$(id -u)" -m -g dockergroup dockeruser

    # Call a second time to enter the container.
    dockerdev
}

if test "$1" == "stop"; then
    if docker ps -a | grep -q "$CONTAINER_NAME"; then
        docker stop "$CONTAINER_NAME"
    fi
else
    dockerdev
fi
