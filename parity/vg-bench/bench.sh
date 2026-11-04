#!/usr/bin/env bash
# Run the benchmark matrix for one mesh and append JSON lines to <work>/<mesh>.jsonl.
#
#   bash parity/vg-bench/bench.sh <corpus-dir> <work-dir> <mesh> [source-file]
#
# <mesh> names a file in the corpus directory (<mesh>.ply or <mesh>.gltf) unless a
# source file is given. Environment:
#   VG_BIN      path to the release binary (default: target/release/vg-bench from the cargo target dir)
#   VG_THREADS  worker count for the multi-thread rows (default 8)
#   VG_CORES_1  core list for the single-thread rows (default 2)
#   VG_CORES_N  core list for the multi-thread rows (default 2-9)
#   VG_REPEATS  interleaved repeats (default 5)
#   VG_DIRS     orbit directions for the cut evaluation (default 64)
#   VG_BEVY     set to 0 to skip the reference builder (default 1)
#   VG_EXTENT   longest side of the fitted mesh in metres (default 2.0)
set -euo pipefail

corpus="${1:?corpus dir}"; work="${2:?work dir}"; mesh="${3:?mesh name}"
bin="${VG_BIN:-${CARGO_TARGET_DIR:-target}/release/vg-bench}"
threads="${VG_THREADS:-8}"; cores1="${VG_CORES_1:-2}"; coresn="${VG_CORES_N:-2-9}"
repeats="${VG_REPEATS:-5}"; dirs="${VG_DIRS:-64}"; bevy="${VG_BEVY:-1}"
mkdir -p "$work"
src="${4:-}"
if [ -z "$src" ]; then
  if [ -f "$corpus/$mesh.ply" ]; then src="$corpus/$mesh.ply"; else src="$corpus/$mesh.gltf"; fi
fi
vgm="$work/$mesh.vgm"; out="$work/$mesh.jsonl"; : >"$out"
"$bin" prep "$src" "$vgm" --extent "${VG_EXTENT:-2.0}" | tee -a "$out"

builders=(ours); [ "$bevy" = 1 ] && builders+=(bevy)
timed() { # builder threads cores extra...
  local b="$1" t="$2" c="$3"; shift 3
  local log; log="$(mktemp)"
  local line
  line="$(taskset -c "$c" /usr/bin/time -v -o "$log" "$bin" build "$b" "$vgm" --threads "$t" "$@")"
  local rss; rss="$(awk -F': ' '/Maximum resident set size/{print $2}' "$log")"
  rm -f "$log"
  printf '%s,"rss_kib":%s,"repeat":%s}\n' "${line%\}}" "$rss" "$rep" | tee -a "$out"
}
for rep in $(seq 1 "$repeats"); do
  for b in "${builders[@]}"; do
    timed "$b" 1 "$cores1"
    if [ "$rep" = 1 ]; then
      timed "$b" "$threads" "$coresn" --out "$work/$mesh.$b.bin"
    else
      timed "$b" "$threads" "$coresn"
    fi
  done
done

"$bin" eval ours "$vgm" --dirs "$dirs" --jobs "$threads" | tee -a "$out"
if [ "$bevy" = 1 ]; then
  "$bin" eval bevy "$vgm" --artifact "$work/$mesh.bevy.bin" --dirs "$dirs" --jobs "$threads" | tee -a "$out"
fi
