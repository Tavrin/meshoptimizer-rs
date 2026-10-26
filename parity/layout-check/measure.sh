#!/bin/bash
set -eu
exec python3 /mnt/linux-extra/moss-scratch/meshopt-validate-once/layout-check/measure.py "$@"
