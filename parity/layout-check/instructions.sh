#!/bin/bash
set -eu
exec python3 "${SCRATCH:?}"/meshopt-validate-once/layout-check/instructions.py "$@"
