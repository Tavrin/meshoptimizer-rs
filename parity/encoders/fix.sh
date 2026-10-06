#!/usr/bin/env bash
set -euo pipefail
python3 parity/encoders/run.py build "$1"
python3 parity/encoders/run.py parity "$1"
python3 parity/encoders/run.py counters "$1"
