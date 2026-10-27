"""Apply the pre-registered instruction gate verbatim; retain failed ratios."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import collections
import json
import math
import statistics

ROOT = Path(__file__).resolve().parents[2]
ART = Path(_public_path('<scratch>/meshopt-0212-simplify'))
rows = [json.loads(line) for line in (ART / 'counts.jsonl').read_text().splitlines()]
assert len(rows) == 432
for arm in ['base', 'candidate', 'M7']:
    receipt = json.loads((ART / f'build-{arm}.json').read_text())
    assert receipt['exit'] == 0
    assert all(row['binary_sha256'] == receipt['sha256'] for row in rows if row['arm'] == arm)
by_cell = collections.defaultdict(dict)
for row in rows:
    assert row['exit'] == 0 and row['validation'] and row['running_percent'] == 100.0
    key = (row['family'], row['case'])
    arm = by_cell[key].setdefault(row['arm'], {})
    assert row['rep'] not in arm
    arm[row['rep']] = row
cells = []
for (family, case), arms in sorted(by_cell.items()):
    assert set(arms) == {'base', 'candidate', 'M7'}
    med = {}
    for arm, repetitions in arms.items():
        assert set(repetitions) == {0, 1, 2}
        assert len({r['binary_sha256'] for r in repetitions.values()}) == 1
        assert len({r['output_sha256'] for r in repetitions.values()}) == 1
        med[arm] = statistics.median(r['per_call'] for r in repetitions.values())
    assert arms['base'][0]['output_sha256'] == arms['candidate'][0]['output_sha256']
    ratio = med['candidate'] / med['base']
    verdict = 'FAIL' if ratio > 1.01 else ('PASS' if ratio <= 0.85 else 'PARTIAL' if ratio <= 0.95 else 'NO-GAIN') if family.startswith('real-') else 'WITHIN-REGRESSION-LIMIT'
    cells.append(dict(family=family, case=case, medians=med, candidate_base=ratio,
                      candidate_M7=med['candidate'] / med['M7'], verdict=verdict))
assert len(cells) == 48
families = []
for family in sorted({c['family'] for c in cells}):
    group = [c for c in cells if c['family'] == family]
    if family.startswith('real-'):
        totals = {arm: sum(c['medians'][arm] for c in group) for arm in ['base', 'candidate', 'M7']}
        ratio, peer = totals['candidate'] / totals['base'], totals['candidate'] / totals['M7']
    else:
        ratio = math.exp(statistics.mean(math.log(c['candidate_base']) for c in group))
        peer = math.exp(statistics.mean(math.log(c['candidate_M7']) for c in group))
    verdict = 'FAIL' if any(c['candidate_base'] > 1.01 for c in group) else ('PASS' if ratio <= 0.85 else 'PARTIAL' if ratio <= 0.95 else 'NO-GAIN') if family.startswith('real-') else 'WITHIN-REGRESSION-LIMIT'
    families.append(dict(family=family, cases=len(group), candidate_base=ratio, candidate_M7=peer, verdict=verdict))
exits = dict(line.split('\t') for line in (ART / 'gate-exits.tsv').read_text().splitlines())
required = {'test-all', 'test-no-default', 'clippy', 'fmt', 'boundary', 'sqrt', 'exact-build-std', 'exact-std', 'exact-build-no-std', 'exact-no-std', 'cpp', 'fuzz'}
assert required <= exits.keys() and all(code == '0' for code in exits.values()), exits
# Filter matching zero tests is not proof of exhaustive identity.
assert '1 passed' in (ART / 'gate-sqrt.log').read_text()
report = ['# registered protocol simplify\n',
          'Fresh user-instruction counts only. Median of three per arm. Real chains use the sum of six stage medians; corpus summaries use the equal-case geometric mean. Per-case regression rejection is applied before aggregation.\n',
          'Techniques: simplify-local std sqrt for nonnegative finite operands, portable special-value/no_std fallback; std memo removal; shared-length picker/quadric slices; bounded prepaid picker with original low-budget fallback. All arithmetic finite guards remain.\n',
          'Source locations:\n']
source = (ROOT / 'src/simplify.rs').read_text().splitlines()
for needle in ['fn sqrt(value:', 'struct SqrtCache;', 'fn quadrics<M:', 'fn pick<M:', 'let nc = metered!', 'fn simplify_sqrt_exhaustive_identity']:
    line = next(i for i, text in enumerate(source, 1) if needle in text)
    report.append(f'- src/simplify.rs:{line}: `{needle}`\n')
extra = (ROOT / 'src/simplify_extra.rs').read_text().splitlines()
line = next(i for i, text in enumerate(extra, 1) if 'cache: SqrtCache,' in text)
report.append(f'- src/simplify_extra.rs:{line}: sloppy scratch uses the simplify-local cache adapter.\n')
report.append('\nPicker bound: each entered complete triangle charges exactly three corners, is entered once, and the capacity stop precedes entry. Visits are therefore at most the current index count. The covered path settles the exact tally; the insufficient-budget path retains the original checks and failure prefixes. There are no fallible arithmetic or reader callbacks in pick. See REGISTRATION.md and DECISIONS.md for the retained reasoning and worst-case sweep.\n')
report += ['\n| Family | Cases/stages | candidate/base | candidate/M7 | Verdict |\n', '|---|---:|---:|---:|---|\n']
for f in families:
    report.append(f"| {f['family']} | {f['cases']} | {f['candidate_base']:.6f} | {f['candidate_M7']:.6f} | {f['verdict']} |\n")
report += ['\n| Family / case | base instructions | candidate instructions | M7 instructions | candidate/base | candidate/M7 | Verdict |\n', '|---|---:|---:|---:|---:|---:|---|\n']
for c in cells:
    med = c['medians']
    report.append(f"| {c['family']} / {c['case']} | {med['base']:.0f} | {med['candidate']:.0f} | {med['M7']:.0f} | {c['candidate_base']:.6f} | {c['candidate_M7']:.6f} | {c['verdict']} |\n")
report += ['\nExit codes: ' + ', '.join(f'{name}={code}' for name, code in sorted(exits.items())) + '; three arm builds=0; 432 stat samples=0.\n',
           '\nProtocol: perf 6.17 instructions:u, CPU9, acknowledged resident-loop counter enable/disable, 100% running in every sample. Scalar opt3/thin-LTO/one-codegen-unit Rust arms; M7 uses scalar C++ 1.3 with NDEBUG and debug symbols. Loading, warm calls and output validation stay outside counters; destination allocation, required copies, scratch acquisition and destruction stay inside.\n']
fuzz = json.loads((ART / 'fuzz.json').read_text())
assert fuzz[-1]['cpu_seconds'] >= 300 and all(r['exit'] == 0 for r in fuzz)
observations = {}
for mode in ['std', 'no-std']:
    text = (ART / f'gate-exact-{mode}.log').read_text().strip()
    assert text.startswith('PASS observations=')
    observations[mode] = int(text.split('=')[1])
cpp = len(json.loads((ART / 'cpp-differential.json').read_text()))
report.append(f"\nProof: exact baseline comparisons passed {observations['std']:,} paired observations under std and {observations['no-std']:,} under no_std. They compare Result discriminants/error bits, output words including failure prefixes and unused tails, updated positions/attributes, and both Usage fields, with cold/warm calls. Coverage includes all 512 option masks, all six families, every small-mesh work limit 0..unrestricted-work+1, worst-case picker meshes in both retention modes, byte limits and invalid/non-finite/overflow/signed-zero/subnormal inputs. The exhaustive adapter/backend sweep passed all 4,294,967,296 operand patterns with zero mismatches. Pinned C++ differential: {cpp} comparisons. Widened exact differential fuzz: {len(fuzz):,} seeds / {len(fuzz)*64:,} generated cases / {fuzz[-1]['cpu_seconds']:.6f} child CPU-seconds, zero mismatches. CPU duration is a fuzz work budget, not a performance result.\n")
report.append('\nRetained failed attempts: launcher parse=2 before preparation; initial picker test compilation=101; second proof fmt=1 after test-all/test-no-default/clippy=0. The latter two attempts and their successful arm builds are retained in attempt1-test-compile/ and attempt2-test-format/. Repairs affect launcher/tests only. The final complete proof passed on the final source; every accepted instruction sample comes from that source. No failed attempt is relabeled as passing.\n')
report.append(_public_path('\nRaw counters, output buffers, commands, source/binary hashes and admission identities remain in `<scratch>/meshopt-0212-simplify/`. Compact per-case counts, results, proof exits and an independently checked hash binding are retained beside this report. No wall-time or production-performance claim is made. No new unsafe, API, version or shared-math changes.\n'))
failed = any(c['candidate_base'] > 1.01 for c in cells)
(ART / 'results.json').write_text(json.dumps(dict(cells=cells, families=families, exits=exits, regression_fail=failed), indent=2) + '\n')
(ROOT / 'parity/0212-simplify/REPORT.md').write_text(''.join(report))
if failed:
    raise SystemExit('FAIL: a case exceeds 1.01; fix/revert before any PR')
