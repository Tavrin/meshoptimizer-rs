"""Publish compact, portable evidence; private logs remain in configured scratch."""
from pathlib import Path
import hashlib,json,math,os,re,shutil
ROOT=Path(__file__).resolve().parents[2]
LANE=Path(__file__).parent
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
rows=json.loads((A/'final/results.json').read_text())
assert len(rows)==13
for file,destination in [('final/results.json','results.json'),('instructions.tsv','instructions.tsv'),('gate-exits.tsv','proof-exits.tsv')]:
    shutil.copy2(A/file,LANE/destination)
replacements=[(str(ROOT),'<repo>'),(str(A),'<scratch>/overdraw'),(os.environ['MESHOPT_RESCREEN_SCRATCH'],'<scratch>/rescreen'),(os.environ['MESHOPT_REFERENCE'],'<reference>'),(os.environ['CARGO_TARGET_DIR'],'<target-dir>'),(os.environ['MESHOPT_JOB_RUNNER'],'<job-runner>'),(str(Path(os.environ.get('CARGO_HOME',Path.home()/'.cargo'))),'<cargo-home>')]
def portable(s):
    for old,new in replacements:s=s.replace(old,new)
    assert not re.search(r'/(?:home|mnt)/',s),s[:300]
    return s
for f in ['binding.json','cleanup.json']:
    if (A/f).exists():(LANE/f).write_text(portable((A/f).read_text()))
summary=[]
for op in ['overdraw','analyze-overdraw']:
    cases=[r for r in rows if r['operation']==op and not r['case'].startswith('real-')]
    q={p:math.exp(sum(math.log(r['ratios']['candidate'][p]) for r in cases)/len(cases)) for p in ['base','M7','X']}
    summary.append((op,q,'WITHIN-REGRESSION-LIMIT'))
summary=[(r['case'],r['ratios']['candidate'],r['verdict']) for r in rows if r['case'].startswith('real-')]+summary
text='''# SPEC-0212 overdraw

Instruction counts only. Median of three per arm. Real inputs use their own
medians; corpus summaries are equal-case geometric means. The >1.01 per-case
regression rejection applies before aggregation. No wall-performance claim.

TECHNIQUES

| Family | candidate/base | candidate/M7 | candidate/X | Verdict |
|---|---:|---:|---:|---|
'''
for label,q,verdict in summary:text+=f"| {label} | {q['base']:.6f} | {q['M7']:.6f} | {q['X']:.6f} | {verdict} |\n"
text+='''
| Family / case | base instructions | candidate instructions | M7 instructions | X instructions | candidate/base | candidate/M7 | candidate/X | Verdict |
|---|---:|---:|---:|---:|---:|---:|---:|---|
'''
for r in rows:
    m=r['median_instructions'];q=r['ratios']['candidate']
    text+=f"| {r['operation']} / {r['case']} | "+' | '.join(f'{m[a]:.2f}' for a in ['base','candidate','M7','X'])+' | '+' | '.join(f'{q[p]:.6f}' for p in ['base','M7','X'])+f" | {r['verdict']} |\n"
text+='\nExit codes: '+', '.join(f'{name}={rc}' for name,rc in (line.split('\t') for line in (A/'gate-exits.tsv').read_text().splitlines()))+'.\n'
text+='''
Protocol: rescreen resident-loop driver, scalar opt3/thin-LTO/one codegen unit,
std without SIMD. M7 = scalar meshoptimizer 1.3 via meshopt 0.7.0; X = optimesh
1.1.0. perf 6.17 instructions:u, CPU 9, acknowledged counter enable/disable.
Input loading, warm call, output validation and serialization are outside
counts; required allocations, copies, API validation, scratch and destruction
are inside. Fixed repetitions, no elapsed calibration. All final samples and
their individual output buffers are retained in `<scratch>/overdraw/final/`.

Proof compares independently compiled v0.2.11 and candidate, std and no_std:
Result discriminants, complete output words including partial-write prefixes,
untouched tails, and Usage work/bytes. Cold and warm calls, all three public
entry points plus short buffers, frozen corpus and real inputs, every work
limit from zero to terminal+1 on small finite/nonfinite/overflow/subnormal/
signed-zero/disconnected/high-valence/duplicate/reversed/degenerate meshes,
byte limits, negative/zero/large/nonfinite thresholds, interleaved floats and
little/big-endian full/short byte records with unaligned offsets and varied
strides. No new global work reservation, API, unsafe, allocation-policy, sqrt-backend,
SIMD or other-family change.

Full raw counters, commands, input/output bytes, source archives, dependency
sources, manifests/locks, compiler identities, admission identities, exact
proof/fuzz logs and failed attempts remain in `<scratch>/overdraw/`. Public
binding.json replaces private locations with placeholders while retaining
hashes. REGISTRATION.md and DECISIONS.md record the frozen gates and choices.
The designated `<target-dir>` is deleted after successful proof and audit;
cleanup.json records absence. No merge, tag, publication or version bump.
'''

def line(file,term):
    return next(i for i,v in enumerate((ROOT/file).read_text().splitlines(),1) if term in v)
record_method='fn overdraw_record_blocks' if 'fn overdraw_record_blocks' in (ROOT/'src/input.rs').read_text() else 'pub(crate) fn scale_full_records'
techniques=[f"- src/input.rs:{line('src/input.rs',record_method)}: reuse fully padded fixed-size records for validation; original short/mapped/other-layout readers retained.",
            f"- src/overdraw/fast.rs:{line('src/overdraw/fast.rs','fn validate(')}: combine all three already-loaded finite classifications before the original single validation error."]
if 'fn scan_records' in (ROOT/'src/input.rs').read_text():
    techniques.append(f"- src/input.rs:{line('src/input.rs','fn scan_records')}: maximum-index proof shares the three full-record bounds checks, preserving per-triangle charges and callback order.")
if '*x *= inv_length;' in (ROOT/'src/overdraw/fast.rs').read_text():
    techniques.append(f"- src/overdraw/fast.rs:{line('src/overdraw/fast.rs','let inv_area =')}: remove only two unreachable reciprocal errors and three unreachable normalized-component errors; PROOF.md derives the finite bounds; conditional guard removal is listed separately below.")
if 'fn bounded_finite' in (ROOT/'src/overdraw/fast.rs').read_text():
    techniques.append(f"- src/overdraw/fast.rs:{line('src/overdraw/fast.rs','fn bounded_finite')}: validation admits coordinates of magnitude <=2^30 to a bounded scoring kernel; larger finite inputs retain the general guarded path. The accumulated-normal squared-norm check remains at its original point. PROOF.md derives the bounds.")
if 'work.target(threshold *' in (ROOT/'src/overdraw/fast.rs').read_text():
    techniques.append(f"- src/overdraw/fast.rs:{line('src/overdraw/fast.rs','impl ClusterMeter for ClusterPrepaid')}: omit the already-proven finite target guard on the existing prepaid path; retain the Work check and every original charge/reservation.")
if 'overdraw_record_blocks' in (ROOT/'src/input.rs').read_text():
    techniques.append(f"- src/input.rs:{line('src/input.rs','fn overdraw_record_blocks')}: validate eight full records with a bitwise reduction, then scan admitted groups without per-vertex finite branches. Unit scans preserve each tight-budget prefix; rejected groups, tails and other layouts retain checked visits. The exact group bound is eight visits, with no heap allocation.")
techniques.append("- src/overdraw.rs:1: retain the baseline packed implementation at its original module and reader path; delegate views to the optimized private fast module. No public API changes.")
entry=json.loads((A/'ENTRY.json').read_text())
text=text.replace('TECHNIQUES',f"Baseline: v0.2.11 `{entry['baseline']}`; candidate starts at post-#39 `{entry['head']}`.\n\n"+'\n'.join(techniques)+'\n\nNo new global work reservation or root backend. See PROOF.md for equivalence and DECISIONS.md for selection.')
exact=[]
for mode in ['std','no-std']:
    log=(A/f'gate-exact-{mode}.log').read_text()
    n=int(re.search(r'PASS: (\d+) exact observations',log).group(1))
    exact.append((mode,n))
fuzz=[json.loads(v) for v in (A/'fuzz.jsonl').read_text().splitlines()]
assert fuzz[-1]['cpu_seconds']>=300 and all(v['exit']==0 for v in fuzz)
cpp=json.loads((A/'cpp-differential.json').read_text())
text+='\nExact paired observations (each includes cold and warm calls): '+', '.join(f'{mode} {n:,}' for mode,n in exact)+f". C++ comparisons: {len(cpp)}. Widened fuzz: {len(fuzz)} seeds, {len(fuzz)*192:,} generated cases, {fuzz[-1]['cpu_seconds']:.6f} child CPU-seconds, zero mismatches. CPU duration is a fuzz work budget.\n"
text+='\nRejected first complete campaign: results=1; packed overdraw regressions of 1.014435581 (medium-seams), 1.017515090 (medium-smooth), 1.020038574 (million-smooth). Its passing proof does not clear that failure. The original packed validation path was restored; the complete build/proof/counter sequence was restarted. All original receipts remain under `<scratch>/overdraw/attempt1-packed-regression/`.\n'
text+='\nRejected second complete campaign: results=1, all five packed overdraw cases exceeded 1.01 (worst 1.034843). Its full proof and fresh 300.028914 CPU-second fuzz passed. Packed calls were isolated onto the unchanged private 0.2.11 kernel, and the complete qualification restarted again. Receipts remain under `<scratch>/overdraw/attempt2-packed-validation/`.\n'
text+='\nRejected third complete campaign: results=1, packed overdraw worst 1.056722 despite source-identical compatibility code. Its proof passed. Restored the baseline module and original input-reader path, isolated the optimized view reader, and restarted the full qualification after a separate packed counter precheck. Receipts: `<scratch>/overdraw/attempt3-packed-module/`.\n'
text+='\nFinal four-arm builds, 156 instruction samples, independent evidence audit and target cleanup exited 0. Every final sample has its own retained output buffer. Retained setup failure: missing scratch documentation, build 101; retained cancelled configuration-transition admissions: 143, no build or sample. Neither is a passing receipt.\n'
worst=max(r['ratios']['candidate']['base'] for r in rows)
assert worst<=1.01
text+=f"\nDecision: retain the qualified changes. Worst individual candidate/base {worst:.9f}; no case exceeds 1.01. The <=0.80 PASS threshold and <=0.95 PARTIAL threshold are applied verbatim to every real input.\n"
(LANE/'REPORT.md').write_text(portable(text))
print('report wrote',len(rows),'per-case instruction rows')
