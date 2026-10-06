#!/usr/bin/env bash
set -euo pipefail
python3 "$(dirname "$0")/simd/check_boundary.py"
# Check the actual package inventory, independently of the repository scan.
cargo package --allow-dirty --no-verify --list | python3 "$(dirname "$0")/simd/check_package.py"
