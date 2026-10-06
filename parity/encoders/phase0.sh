#!/usr/bin/env bash
set -euo pipefail
python3 parity/encoders/run.py prepare
python3 parity/encoders/run.py parity before
python3 parity/encoders/run.py counters before
python3 parity/encoders/knockouts.py
python3 parity/encoders/run.py parity bounds
python3 parity/encoders/run.py parity validation
python3 parity/encoders/run.py counters bounds
python3 parity/encoders/run.py counters validation
