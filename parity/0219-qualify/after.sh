#!/usr/bin/env bash
set -u
# Caller already owns the required CPU admission.
python3 parity/0219-qualify/supplement.py > "$QUALIFY_LOGS/supplement.out" 2>&1
python3 parity/0219-qualify/bench.py > "$QUALIFY_LOGS/bench.out" 2>&1
