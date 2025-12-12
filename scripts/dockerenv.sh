#!/bin/bash -e
# SPDX-License-Identifier: Apache-2.0

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

if [ -n "$CONTAINER_RUNTIME" ]; then
  RUNTIME="$CONTAINER_RUNTIME"
elif command -v podman &>/dev/null; then
  RUNTIME=podman
else
  RUNTIME=docker
fi

if [ "$1" = "release" ] ; then
    MOUNT_DIR=/bb02
    CONTAINER_NAME_SUFFIX=rel
else
    MOUNT_DIR="$DIR/.."
    CONTAINER_NAME_SUFFIX=dev
fi

CONTAINER_IMAGE=shiftcrypto/firmware_v2
CONTAINER_VERSION=${CONTAINER_VERSION:-$(cat .containerversion)}
PROJECT_NAME="$(basename "$(realpath "$DIR/..")")"
CONTAINER_NAME="$PROJECT_NAME-$CONTAINER_NAME_SUFFIX"

dockerdev () {
    local repo_path="$DIR/.."

    if ! $RUNTIME images --filter "reference=${CONTAINER_IMAGE}" --format table | grep -q "${CONTAINER_IMAGE} *${CONTAINER_VERSION}"; then
        echo "No '${CONTAINER_IMAGE}:${CONTAINER_VERSION}' ${RUNTIME} image found! Maybe you need to run
              '${RUNTIME} pull ${CONTAINER_IMAGE}:${CONTAINER_VERSION}'?" >&2
        exit 1
    fi

    USERFLAG=""
    if [ "$RUNTIME" = "docker" ] ; then
        # Only needed for docker - see the comment below.
        USERFLAG="--user=dockeruser"
    fi

    # If already running, enter the container.
    if $RUNTIME ps --filter "name=^${CONTAINER_NAME}$" | grep -q "$CONTAINER_NAME"; then
        id_running=$(${RUNTIME} inspect ${CONTAINER_NAME} | jq -r '.[0].Image')
        id_wanted=$(${RUNTIME} inspect ${CONTAINER_IMAGE}:${CONTAINER_VERSION} | jq -r '.[0].Id')
        # If requested version is same as running version, enter container
        if ! [ $id_wanted == $id_running ] ; then
            echo "Currently running container is not the same version as the requested version"
            echo "Requested version ${CONTAINER_IMAGE}:${CONTAINER_VERSION} ($id_wanted)"
            echo "Current version $(${RUNTIME} inspect ${CONTAINER_NAME} | jq -r '.[0].Config.Image') ($id_running)"
            exit 1
        else
            $RUNTIME exec $USERFLAG --workdir="$MOUNT_DIR" -it "$CONTAINER_NAME" bash
            return
        fi
    fi

    if $RUNTIME ps --all --filter "name=^${CONTAINER_NAME}$" | grep -q "$CONTAINER_NAME"; then
        $RUNTIME rm "$CONTAINER_NAME"
    fi

    # * `SYS_PTRACE` is needed to run address sanitizer
    # * `-p 15423` publishes the default port the simulator listens on
    $RUNTIME run \
           --detach \
           --interactive --tty \
           --name="$CONTAINER_NAME" \
           -v "$repo_path":"$MOUNT_DIR" \
           --cap-add SYS_PTRACE \
           ${CONTAINER_IMAGE}:${CONTAINER_VERSION} bash

    if [ "$RUNTIME" = "docker" ] ; then
        # Use same user/group id as on the host, so that files are not created as root in the
        # mounted volume. Only needed for Docker. On rootless podman, the host user maps to the
        # container root user.
        # If group already exists, don't create it
        if ! $RUNTIME exec -it "$CONTAINER_NAME" getent group "$(id -g)" > /dev/null ; then
            $RUNTIME exec -it "$CONTAINER_NAME" groupadd -o -g "$(id -g)" dockergroup
        fi
        $RUNTIME exec -it "$CONTAINER_NAME" useradd -u "$(id -u)" -m -g "$(id -g)" dockeruser
    fi

    # Call a second time to enter the container.
    dockerdev
}

if test "$1" == "stop"; then
    if $RUNTIME ps -a | grep -q "$CONTAINER_NAME"; then
        $RUNTIME stop "$CONTAINER_NAME"
    fi
else
    dockerdev
fi
