#!/bin/bash
#
# Usage:
#   generate <registration.json>...  generate a vector set next to each registration
#   validate (<internalProjection.json> <responses.json>)...
#                                    validate IUT responses, writes validation.json next to them
#   check <registration.json>...     check registrations without generating

set -euo pipefail

# Exit code of the gen/val runner when the responses were validated and at least one test failed.
FAILED_VALIDATION=13
# See OrleansGatewayPort in /out/server/sharedappsettings.json.
GATEWAY_PORT=30000

genval() {
    (cd /out/genval && dotnet NIST.CVP.ACVTS.Generation.GenValApp.dll "$@")
}

# All crypto of the gen/val runner goes through a local Orleans silo. It logs nothing when it is
# ready, and clients that connect too early fail, so wait until its gateway accepts connections.
start_server() {
    (cd /out/server && exec dotnet NIST.CVP.ACVTS.Orleans.ServerHost.dll --console) >/tmp/server.log 2>&1 &
    local pid=$!
    for _ in $(seq 120); do
        if ! kill -0 "$pid" 2>/dev/null; then
            echo "the ACVP server exited during startup" >&2
            cat /tmp/server.log >&2
            exit 1
        fi
        if (exec 3<>"/dev/tcp/127.0.0.1/$GATEWAY_PORT") 2>/dev/null; then
            return
        fi
        sleep 1
    done
    echo "the ACVP server did not start" >&2
    cat /tmp/server.log >&2
    exit 1
}

cmd=${1:-}
shift || true
case "$cmd" in
generate)
    for reg in "$@"; do
        genval -c "$reg"
    done
    start_server
    for reg in "$@"; do
        echo "== $reg"
        genval -g "$reg"
    done
    ;;
validate)
    # Validate every vector set, even if some fail. A failed validation still writes
    # validation.json, so only other errors make the command fail.
    errors=0
    start_server
    while [ $# -ge 2 ]; do
        echo "== $2"
        status=0
        genval -n "$1" -b "$2" || status=$?
        if [ "$status" -ne 0 ] && [ "$status" -ne "$FAILED_VALIDATION" ]; then
            echo "gen/val failed with exit code $status: $2" >&2
            errors=1
        fi
        shift 2
    done
    exit "$errors"
    ;;
check)
    for reg in "$@"; do
        genval -c "$reg"
    done
    ;;
*)
    sed -n '3,8p' "$0" | sed 's/^# \{0,1\}//'
    exit 1
    ;;
esac
