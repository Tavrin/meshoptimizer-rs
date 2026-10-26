#!/usr/bin/env bash
set -euo pipefail
cd /home/etienne/dev/meshoptimizer-rs-wt/cook-chain
case "$1" in
 build) shift
 python3 parity/028-L2/build.py "$@"
 if [[ "$*" == 'base M7 X' ]]; then
   python3 parity/028-L2/instructions.py baseline /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-0 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-0 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-0
 fi
 if [[ "$1" == candidate ]]; then
   python3 parity/028-L2/instructions.py probe3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-2
   python3 parity/028-L2/prepare-stages.py
   python3 parity/028-L2/proof.py /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-2
 fi ;;


 final-build) shift
 python3 parity/028-L2/build.py base M7 X candidate
 python3 parity/028-L2/instructions.py final /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-2 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-2 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-2
 set +e
 python3 parity/028-L2/adapter-proof.py /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-2 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-2 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-2
 code=$?
 set -e
 printf '%s\n' "$code" > /mnt/linux-extra/moss-scratch/meshopt-028-L2/adapter-proof.exit
 MESHOPT_PROTOCOL_ONLY=1 python3 parity/028-L2/proof.py /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-3
 python3 parity/028-L2/strict-overdraw.py
 ;;
 stage-rebuild) shift
 python3 parity/028-L2/prepare-stages.py
 python3 parity/028-L2/build.py base M7 X candidate
 python3 parity/028-L2/instructions.py final-stage /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-4 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-3
 python3 parity/028-L2/adapter-proof.py /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-candidate-4 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-base-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-M7-3 /mnt/linux-extra/moss-scratch/meshopt-028-L2/driver-X-3
 ;;
 strict) shift; exec python3 parity/028-L2/strict-overdraw.py ;;
 instructions) shift; exec python3 parity/028-L2/instructions.py "$@" ;;
 verify) shift
 set +e
 bash parity/028-L2/gates.sh
 gates=$?
 bash parity/028-L2/fuzz.sh
 fuzz=$?
 printf 'gates\t%s\nfuzz\t%s\n' "$gates" "$fuzz" > /mnt/linux-extra/moss-scratch/meshopt-028-L2/verify-exits.tsv
 test "$gates" -eq 0 && test "$fuzz" -eq 0
 ;;
 campaign) shift; exec python3 parity/028-L2/campaign.py "$@" ;;
 proof) shift; exec python3 parity/028-L2/proof.py "$@" ;;
 gates) shift; exec bash parity/028-L2/gates.sh "$@" ;;
esac
