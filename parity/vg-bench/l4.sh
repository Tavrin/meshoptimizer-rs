#!/usr/bin/env bash
# Run inside the caller's admitted CPU job, on a shared machine, with no quiet wait.
# Usage: bash parity/vg-bench/l4.sh <corpus-dir> <evidence-dir>
set -euo pipefail
corpus="${1:?corpus directory}"; evidence="${2:?evidence directory}"
bin="${CARGO_TARGET_DIR:?set the admitted target directory}/release/vg-bench"
mkdir -p "$evidence"
rustc -Vv > "$evidence/toolchain.txt"
lscpu > "$evidence/cpu.txt"
taskset -pc $$ > "$evidence/affinity.txt"
python3 parity/vg-bench/l4_inputs.py "$corpus" "$evidence"
for mesh in bunny dead_tree_trunk_02 namaqualand_boulder_02 coast_rocks_01 dragon; do
    input="$corpus/$mesh.gltf"
    if [[ "$mesh" == bunny || "$mesh" == dragon ]]; then input="$corpus/$mesh.ply"; fi
    "$bin" prep "$input" "$evidence/$mesh.vgm" > "$evidence/$mesh.prep.jsonl"
    for repeat in 1 2 3 4 5; do
        builders=(ours ours-v2)
        if (( repeat % 2 == 0 )); then builders=(ours-v2 ours); fi
        for threads in 1 8; do
            cores=2
            if (( threads == 8 )); then cores=2-9; fi
            for builder in "${builders[@]}"; do
                taskset -c "$cores" /usr/bin/time -v -o "$evidence/$mesh.$builder.$threads.$repeat.time" \
                    "$bin" build "$builder" "$evidence/$mesh.vgm" --threads "$threads" \
                    > "$evidence/$mesh.$builder.$threads.$repeat.jsonl"
            done
        done
    done
    for policy in baseline v2; do
        extra=()
        if [[ "$policy" == v2 ]]; then extra=(--set v2=1); fi
        taskset -c 2-9 "$bin" eval ours "$evidence/$mesh.vgm" --dirs 64 --jobs 8 --samples 200000 "${extra[@]}" \
            > "$evidence/$mesh.$policy.cuts.jsonl"
    done
done
taskset -c 2-9 "$bin" materials "$corpus/flighthelmet.gltf" > "$evidence/materials.jsonl"
python3 parity/vg-bench/l4_report.py "$evidence" > "$evidence/RESULTS.md"
