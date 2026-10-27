"""Render the independently audited instruction and exact-observable results."""
from pathlib import Path
import json
import os

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
LANE = ROOT/'parity/0213-simplify3'
r = json.loads((ART/'results.json').read_text())
a = json.loads((ART/'audit.json').read_text())
assert a['exit']==0 and not r['regression_fail']
exits = dict(s.split('\t') for s in (ART/'gate-exits.tsv').read_text().splitlines())
s = ['# Simplify third pass\n\n',
     'Instruction counts only. Fresh v0.2.12, candidate and scalar M7/C++ 1.3 arms; median of three for 48 cases. Real-chain ratios use sums of six stage medians; corpus summaries use equal-case geometric means. No wall-performance claim.\n\n',
     'Gate applied verbatim: PASS if candidate/M7 <=1.15 on all three real chains; PARTIAL if candidate/base <=0.90 on all three; NO-GAIN otherwise. Any case candidate/base >1.01 is FAIL before aggregation. At most two campaigns; no implementation edits after a campaign result except reversions.\n\n',
     'Attribution and technique reasoning: [ATTRIBUTION.md](ATTRIBUTION.md) and [DECISIONS.md](DECISIONS.md).\n\n',
     '| Family | Cases/stages | candidate/base | candidate/M7 | Verdict |\n',
     '|---|---:|---:|---:|---|\n']
for f in r['families']:
    s.append(f"| {f['family']} | {f['cases']} | {f['candidate_base']:.6f} | {f['candidate_M7']:.6f} | {f['verdict']} |\n")
s += [f"\nOverall: **{r['verdict']}**. Worst individual candidate/base ratio: {max(c['candidate_base'] for c in r['cells']):.9f}.\n\n",
      '| Family / case | base instructions | candidate | M7 | candidate/base | candidate/M7 |\n',
      '|---|---:|---:|---:|---:|---:|\n']
for c in r['cells']:
    m = c['medians']
    s.append(f"| {c['family']} / {c['case']} | {m['base']:.0f} | {m['candidate']:.0f} | {m['M7']:.0f} | {c['candidate_base']:.6f} | {c['candidate_M7']:.6f} |\n")
s += ['\nExit codes: '+', '.join(f'{k}={v}' for k,v in sorted(exits.items()))+'; three arm builds=0; 432 instruction samples=0; instruction gate=0; evidence audit=0.\n\n',
      f"Exact v0.2.12 comparisons: {a['observations']['std']:,} paired observations under std and {a['observations']['no-std']:,} under no_std. Result/error bits, complete outputs and unused tails, partial writes, mutable positions/attributes and both Usage fields match. Coverage includes cold/warm workspaces, all 512 option masks, all six simplify families, every small-mesh work limit 0..success+1, both retention modes, hash collisions and batch boundaries, high valence, duplicate/degenerate faces, welded wedges, wide attributes, byte limits, endian byte views, sparse mappings and finite overflow/special values. Pinned C++ differential: {a['cpp_comparisons']} comparisons. Exact fuzz: {a['fuzz_seeds']:,} seeds, {a['fuzz_cpu_seconds']:.6f} child CPU-seconds, zero mismatches. CPU accounting is a fuzz work budget only.\n\n",
      'Protocol: unchanged release resident-loop harness, perf 6.17 instructions:u, CPU9, acknowledged counter enable/disable, 100% counter running for every sample. Scalar Rust opt3/thin-LTO/one codegen unit; scalar C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls, validation and serialization are outside counters. Destination allocation, required copies, scratch acquisition and destruction are inside. Every build and measurement uses shared admission, GPU disabled, 4 GB declaration.\n\n',
      'Raw inputs, frozen sources, binaries, annotated profiles, samples, outputs, commands and admissions remain in `<scratch>/meshopt-0213-simplify3/`. Portable bindings and compact instruction/proof records are retained beside this report. No unsafe, API/version, SIMD, threading or other-family changes.\n']
(LANE/'REPORT.md').write_text(''.join(s))
