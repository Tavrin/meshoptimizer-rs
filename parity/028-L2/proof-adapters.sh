#!/usr/bin/env bash
set -euo pipefail
cd "${REPO:?}"
: "${MOSS_HEAVY_ACTIVE:?CPU admission required}"
export MESHOPT_STAGES_ONLY=1
python3 parity/028-L2/adapter-proof.py "${SCRATCH:?}"/meshopt-028-L2/driver-candidate-4 "${SCRATCH:?}"/meshopt-028-L2/driver-base-3 "${SCRATCH:?}"/meshopt-028-L2/driver-M7-3 "${SCRATCH:?}"/meshopt-028-L2/driver-X-3

python3 parity/028-L2/compose-proof.py
# A real nested admission inherits this scope's 4 GiB declaration. It marks
# the wall phase as measurement work without releasing and reacquiring RAM.
MOSS_HEAVY_MEASUREMENT=1 bash parity/028-L2/admit.sh bash parity/028-L2/wall.sh
