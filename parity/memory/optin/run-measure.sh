#!/bin/bash
set -euo pipefail
bash /mnt/linux-extra/moss-scratch/meshopt-memory/optin/build-probes.sh
exec python3 /mnt/linux-extra/moss-scratch/meshopt-memory/optin/measure.py
