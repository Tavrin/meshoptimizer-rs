#!/usr/bin/env bash
set -euo pipefail
python3 parity/encoders/build_fair.py
python3 parity/encoders/run.py build after
python3 parity/encoders/run.py parity before-fair
python3 parity/encoders/run.py parity after
python3 parity/encoders/run.py counters after
