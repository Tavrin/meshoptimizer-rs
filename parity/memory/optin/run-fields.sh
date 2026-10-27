#!/bin/bash
set -euo pipefail
s="${SCRATCH:?}"/meshopt-memory/optin
mkdir "$s/bin-initial-probes"
cp "$s/bin/probe-A" "$s/bin/probe-B" "$s/bin-initial-probes/"
cp "$s/BINARIES.sha256" "$s/BINARIES-initial.sha256"
bash "$s/build-probes.sh"
exec python3 "$s/fields.py"
