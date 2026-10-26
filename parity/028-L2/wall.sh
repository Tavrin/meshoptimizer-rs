#!/usr/bin/env bash
set -euo pipefail
cd /home/etienne/dev/meshoptimizer-rs-wt/cook-chain
: "${MOSS_HEAVY_ACTIVE:?CPU admission required}"
: "${MOSS_HEAVY_MEASUREMENT:?measurement admission required}"
exec python3 parity/028-L2/campaign.py /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-4 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-3
