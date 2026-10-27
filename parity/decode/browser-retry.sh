#!/bin/bash
set -euo pipefail
s="${SCRATCH:?}"/meshopt-decode
test ! -e "$s/browser/data/chromium-timing.json"
cat /proc/loadavg /proc/pressure/{cpu,io,memory} > "$s/host-before-browser-retry.txt"
node "$s/browser/drive-retry.mjs"
cat /proc/loadavg /proc/pressure/{cpu,io,memory} > "$s/host-after-timing.txt"
python3 parity/decode/analyze.py
