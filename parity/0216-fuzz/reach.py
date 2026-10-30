"""Reachability of the 0.2.16 changes from each fuzz target's seed corpus.

  reach.py [--bin-dir DIR] TARGET...
Reads the `cargo fuzz coverage` profile of each target (fuzz.py cover), exports
llvm-cov JSON for the 0.2.16 source files that target is meant to reach, and
lists every function that has a region on a line added or changed since v0.2.15
(git diff -U0 v0.2.15..HEAD) with its region coverage (regions executed /
regions) and entry count, merged over generic instantiations. A summary line
per target gives the changed executable lines hit.
"""
import json
import os
from pathlib import Path
import re
import subprocess
import sys

S = Path(os.environ['FUZZ_SCRATCH'])
ROOT = Path(__file__).resolve().parents[2]
LLVM = Path.home() / '.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin'
DIFF_TARGETS = {'simplify', 'simplify_extra', 'meshlets', 'clod', 'remap', 'lod_chain', 'bounds_diff'}
FILES = {
    'weld_vertices': ['src/repair.rs'], 'orient_faces': ['src/repair.rs'], 'fill_holes': ['src/repair.rs'],
    'parallel_clod': ['src/clusterlod/parallel.rs', 'src/clusterlod/mod.rs'],
    'sequence_stream': ['src/codec/sequence_stream.rs', 'src/codec/mod.rs'],
    'vertex': ['src/codec/vertex.rs', 'src/codec/simd/x86.rs', 'src/codec/simd/mod.rs', 'src/codec/mod.rs'],
    'vertex_into': ['src/codec/vertex.rs', 'src/codec/simd/x86.rs', 'src/codec/simd/mod.rs', 'src/codec/mod.rs'],
    'simd': ['src/codec/vertex.rs', 'src/codec/simd/x86.rs', 'src/codec/simd/mod.rs', 'src/codec/mod.rs'],
    'remap': ['src/remap.rs'],
    'simplify': ['src/simplify.rs', 'src/simplify_extra.rs'], 'simplify_extra': ['src/simplify_extra.rs', 'src/simplify.rs'],
    'meshlets': ['src/meshlet_spatial.rs', 'src/meshlet_util.rs', 'src/codec/simd/spatial.rs', 'src/codec/simd/bounds.rs'],
    'bounds_diff': ['src/meshlet_util.rs', 'src/codec/simd/bounds.rs'],
    'clod': ['src/clusterlod/mod.rs', 'src/meshlet_util.rs'], 'lod_chain': ['src/clusterlod/mod.rs'],
}
GROUP_PROFILE = {
    **{t: S / 'fuzz/crate' for t in DIFF_TARGETS},
    **{t: ROOT / 'parity/0216-repair/fuzz' for t in ['weld_vertices', 'orient_faces', 'fill_holes']},
    **{t: ROOT / 'fuzz/codec' for t in ['vertex', 'vertex_into', 'simd', 'sequence_stream']},
    'parallel_clod': ROOT / 'fuzz/parallel-clod',
}


def changed_lines(rel):
    out = subprocess.run(['git', 'diff', '-U0', 'v0.2.15..HEAD', '--', rel], capture_output=True, text=True,
                         cwd=ROOT).stdout
    lines = set()
    for m in re.finditer(r'^@@ -\S+ \+(\d+)(?:,(\d+))? @@', out, re.M):
        start, count = int(m[1]), int(m[2]) if m[2] is not None else 1
        lines.update(range(start, start + count))
    return lines


def demangle(names):
    p = subprocess.run(['c++filt', '-s', 'rust'], input='\n'.join(names), capture_output=True, text=True)
    return [re.sub(r'\[[0-9a-f]{16}\]', '', n) for n in p.stdout.splitlines()]


def analyse(target, bin_dir):
    exe = bin_dir / target
    prof = GROUP_PROFILE[target] / 'coverage' / target / 'coverage.profdata'
    root = S / 'fuzz/candidate' if target in DIFF_TARGETS else ROOT
    sources = [str(root / f) for f in FILES[target]]
    raw = subprocess.run([str(LLVM / 'llvm-cov'), 'export', '-instr-profile', str(prof), str(exe), *sources],
                         capture_output=True, text=True, check=True).stdout
    data = json.loads(raw)['data'][0]
    changed = {str(root / f): changed_lines(f) for f in FILES[target]}
    table = {}
    per_line = {}
    lcov = subprocess.run([str(LLVM / 'llvm-cov'), 'export', '-format=lcov', '-instr-profile', str(prof),
                           str(exe), *sources], capture_output=True, text=True, check=True).stdout
    current = None
    for row in lcov.splitlines():
        if row.startswith('SF:'):
            current = row[3:]
        elif row.startswith('DA:') and current in changed:
            line, count = map(int, row[3:].split(',')[:2])
            if line in changed[current]:
                per_line[(current, line)] = per_line.get((current, line), 0) + count
    for fn in data['functions']:
        for r in fn['regions']:
            ls, _, le, _, count, file_id, _, kind = r
            name = fn['filenames'][file_id]
            if name not in changed or kind != 0:
                continue
            span = set(range(ls, le + 1))
            if span & changed[name]:
                key = (name, fn['name'])
                t = table.setdefault(key, [0, 0, fn['count']])
                t[1] += 1
                t[0] += count > 0
    rows = []
    mangled = sorted({k[1] for k in table})
    pretty = dict(zip(mangled, demangle(mangled)))
    merged = {}
    for (name, mangled_name), (hit, total, entry) in table.items():
        key = (Path(name).relative_to(root).as_posix(), pretty[mangled_name])
        m = merged.setdefault(key, [0, 0, 0])
        m[0] += hit
        m[1] += total
        m[2] += entry
    for (file, fn), (hit, total, entry) in sorted(merged.items()):
        rows.append(dict(file=file, function=fn, regions_hit=hit, regions=total, entry_count=entry))
    lines_hit = sum(1 for v in per_line.values() if v)
    return dict(target=target, changed_executable_lines=len(per_line), changed_lines_hit=lines_hit,
                functions=rows)


if __name__ == '__main__':
    args = sys.argv[1:]
    bin_dir = Path(os.environ['CARGO_TARGET_DIR']) / 'cov/x86_64-unknown-linux-gnu/release'
    if args[:1] == ['--bin-dir']:
        bin_dir, args = Path(args[1]), args[2:]
    out = S / 'reach'
    out.mkdir(exist_ok=True)
    for target in args:
        res = analyse(target, bin_dir)
        (out / f'{target}.json').write_text(json.dumps(res, indent=1) + '\n')
        print(f"{target}: changed executable lines hit {res['changed_lines_hit']}/{res['changed_executable_lines']}, "
              f"{len(res['functions'])} functions touching changed lines", flush=True)
