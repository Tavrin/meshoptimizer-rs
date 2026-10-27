import os
"""Render the final, audited instruction/proof record."""
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ['MESHOPT_SCRATCH'])
LANE = ROOT/'parity/0212-simplify2'
r = json.loads((ART/'results.json').read_text())
audit = json.loads((ART/'audit.json').read_text())
assert audit['exit'] == 0 and not r['regression_fail']
exits = dict(l.split('\t') for l in (ART/'gate-exits.tsv').read_text().splitlines())
s = ['# registered protocol simplify2\n',
     'Instruction counts only. Fresh v0.2.11 (40dbf9e), post-#39 (c396a697), candidate and scalar M7/C++ 1.3 arms; median of three for all 48 cases. Real-chain ratios use sums of six stage medians; corpus summaries use equal-case geometric means. No wall-performance claim.\n',
     '\nGate applied verbatim: real-chain PASS when candidate/v0.2.11 <=0.80; otherwise PARTIAL when candidate/post-#39 <=0.95; otherwise NO-GAIN. Any individual candidate/post-#39 >1.01 is FAIL before aggregation.\n',
     '\nTechniques (src/simplify.rs):\n']
lines = (ROOT/'src/simplify.rs').read_text().splitlines()
for needle, explanation in [
    ('fn bounds_impl<', 'Validated-only input finite-check elimination; scale retains validation and extent overflow is checked in both paths.'),
    ('fn adjacency<', 'Shared vertex-length remap/counter slices and bounded release accounting.'),
    ('for start in (0..count).step_by(64)', 'Bounded hash batches; exact visited-prefix settlement with checked fallback.'),
    ('let wedge = &mut self.wedge.0[..count]', 'Iterator over remap entries and a fixed-length wedge slice.'),
    ('let finite_scale =', 'Finite-reciprocal normalization proof; reciprocal overflow retains the original post-write guard.'),
    ('fn rank_attributes<', 'Batched attribute ranking bounded by complete wedge traversals.'),
    ('fn finite(self)', 'Non-short-circuit finite reductions preserve the predicate and error position.'),
    ('let normalize_work =', 'Bounded normalization and attribute input accounting; overflow remains checked on the unproven path.'),
    ('let quadric_work =', 'Bounded quadric accounting; folding keeps the checked path.'),
    ('let update_work =', 'Bounded update accounting; finite checks stay at their original position.'),
]:
    n = next(i for i,line in enumerate(lines,1) if needle in line)
    s.append(f'- src/simplify.rs:{n}: {explanation}\n')
s += ['\nBounds: adjacency n+2m; quadrics 2m+n plus m*(ac+1) when ac>0; normalization n*(1+weights.len()); updates n*(3+ac); a hash batch b<=64 charges at most b*(table.len()+1). Hash probes run over 0..table.len(), preceded by one vertex read charge. Quadric triangles charge m, regularization n, edges m and attributes (m/3)*(3+3ac). Updates charge n vertices, at most n*ac attributes and two n-entry loop passes. Option 128 disables quadric prepayment. Checked bound overflow or insufficient remaining work selects the original per-item path. Metered success/error settles only the actual visits, preserving failure prefixes.\n',
       '\nPositional rank retains the #39 kernel/accounting with a bitwise finite predicate; picker is unchanged. Attribute rank uses batches of at most 64 candidates, each bounded by 1+2*ac*(n+1): one candidate visit, 2ac base visits and at most two n-entry wedge traversals (or 2ac seam visits). Sqrt, public API, shared math, other families and unsafe boundaries are unchanged. For normalization, finite input validation and finite bounds imply each rounded coordinate difference lies in [0, extent]. A finite rounded reciprocal gives extent*scale <2, hence finite products. The guard is omitted only on this proven path; reciprocal overflow retains it at the original post-charge/post-write position. Independent exponent checks use bitwise reductions without removing any arithmetic guard. Quadric/rank/update guards likewise remain.\n',
       '\n| Family | Cases/stages | candidate/v0.2.11 | candidate/post-#39 | candidate/M7 | Verdict |\n',
       '|---|---:|---:|---:|---:|---|\n']
for f in r['families']:
    s.append(f"| {f['family']} | {f['cases']} | {f['candidate_base']:.6f} | {f['candidate_post39']:.6f} | {f['candidate_M7']:.6f} | {f['verdict']} |\n")
s += ['\n| Family / case | v0.2.11 instructions | post-#39 | candidate | M7 | candidate/v0.2.11 | candidate/post-#39 | candidate/M7 | Verdict |\n',
       '|---|---:|---:|---:|---:|---:|---:|---:|---|\n']
for c in r['cells']:
    m=c['medians']
    s.append(f"| {c['family']} / {c['case']} | {m['base']:.0f} | {m['post39']:.0f} | {m['candidate']:.0f} | {m['M7']:.0f} | {c['candidate_base']:.6f} | {c['candidate_post39']:.6f} | {c['candidate_M7']:.6f} | {c['verdict']} |\n")
s += ['\nExit codes: '+', '.join(f'{k}={v}' for k,v in sorted(exits.items()))+'; four arm builds=0; 576 stat samples=0; instruction gate=0; evidence audit=0.\n',
      f"\nExact v0.2.11 comparisons: {audit['observations']['std']:,} paired observations under std and {audit['observations']['no-std']:,} under no_std, including Result/error bits, full outputs and unused tails, partial writes, mutable positions/attributes and both Usage fields. Cold/warm workspaces, all 512 option masks, every small-mesh work limit 0..success+1, both retention modes, deliberate hash collisions, >64-vertex batches, high valence, duplicate/degenerate faces, welded wedges, 32 attributes, byte limits, little/big endian byte readers, sparse mappings and finite overflow/special values. Finite-reciprocal bound sweep: {audit['normalization_extent_patterns']:,} nonnegative finite extent bit patterns in each exact mode, zero violations. Pinned C++ differential: {audit['cpp_comparisons']} comparisons. Widened differential fuzz: {audit['fuzz_seeds']:,} seeds / {audit['fuzz_seeds']*64:,} generated cases, {audit['fuzz_cpu_seconds']:.6f} child CPU-seconds, zero mismatches. Fuzz CPU accounting is a work budget only.\n",
      '\nRetained attempt 1: all four arm builds and 576 samples exited 0, with all three real chains NO-GAIN. Its all-feature preflight exited 0. The queued proof admission was canceled before execution (143) to adopt environment-only tooling; no proof pass is attributed to it. A report-tool syntax preflight exited 1 and was repaired to 0 before qualification; the private repair receipt is retained. Final proof and measurement use the refined source and independent fresh samples.\n',
      '\nProtocol: attribution resident-loop harness, perf 6.17 instructions:u on CPU9, acknowledged counter enable/disable; all samples have 100% running. Scalar Rust opt3/thin-LTO/one codegen unit; scalar M7 C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls, validation and serialization are outside counters. Destination allocation, required copies, scratch acquisition and destruction are inside. Every build/measurement uses <job-runner> with PARITY_RUNNER_GPU=0 and a 4 GB declaration.\n',
      '\nFull inputs, frozen sources, binaries, raw counters/output buffers, commands, logs and admissions are retained in `<scratch>/meshopt-0212-simplify2/`; binding.json and instructions.tsv retain independently checked hashes and compact counts. No thresholds, exhaustion charge granularity or observable error/partial-write order changed. No wall timings, API/version changes, new unsafe or sqrt changes.\n']
(LANE/'REPORT.md').write_text(''.join(s))
