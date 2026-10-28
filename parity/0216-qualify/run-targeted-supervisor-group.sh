#!/usr/bin/env bash
set -euo pipefail
scratch=${MESHOPT_SCRATCH:?}
mode=${QUAL_MODE:?}
start=${QUAL_START:?}
end=${QUAL_END:?}
[[ "$mode" == all || "$mode" == none ]] || exit 2
(( start >= 1 && end <= 32 && start < end && end - start <= 8 )) || exit 2
out="$scratch/proof/0216d-supervisor-${mode}-${start}-${end}"
if [[ -d "$out" ]]; then
    [[ -f "$out/exits.txt" ]] || { echo 'existing incomplete group' >&2; exit 2; }
    for ((shard=start; shard<end; shard++)); do
        rg -qx "$shard 0" "$out/exits.txt" || { echo 'existing group is not complete' >&2; exit 2; }
        rg -q '^PASS observations=' "$out/shard-$shard.log" || exit 2
    done
    echo "reusing completed group $mode $start $end"
    exit 0
fi
mkdir -p "$out"
bin="$scratch/proof/0216d-supervisor-shard-00-pilot/binary-$mode"
sha256sum "$bin" > "$out/binary.sha256"
sha256sum "$0" > "$out/supervisor.sha256"
for ((batch=start; batch<end; batch+=4)); do
    avail=$(df -Pk "$scratch" | awk 'NR==2 {print $4}')
    (( avail >= 104857600 )) || exit 75
    pids=(); shards=()
    for ((shard=batch; shard<end && shard<batch+4; shard++)); do
        (ulimit -t 600; QUAL_SHARD="$shard" /usr/bin/time -p "$bin") > "$out/shard-$shard.log" 2>&1 &
        pids+=("$!"); shards+=("$shard")
    done
    failed=0
    for ((i=0; i<${#pids[@]}; i++)); do
        set +e
        wait "${pids[$i]}"; rc=$?
        set -e
        printf '%s %s\n' "${shards[$i]}" "$rc" >> "$out/exits.txt"
        if (( rc != 0 && failed == 0 )); then failed=$rc; fi
    done
    (( failed == 0 )) || exit "$failed"
done
