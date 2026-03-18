#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
MODE="${BITBOX_FW_EXEC_MODE:-host}"

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <command> [args...]" >&2
    exit 1
fi

case "$MODE" in
    docker)
        exec "$DIR/docker_exec.sh" "$@"
        ;;
    host)
        exec "$@"
        ;;
    *)
        echo "Unsupported BITBOX_FW_EXEC_MODE: '$MODE'." >&2
        echo "Use BITBOX_FW_EXEC_MODE=docker or BITBOX_FW_EXEC_MODE=host." >&2
        exit 1
        ;;
esac
