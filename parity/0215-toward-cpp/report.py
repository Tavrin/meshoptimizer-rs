"""Export portable evidence and the report from a completed admitted run."""
import hashlib
import json
import os
from pathlib import Path
import shutil

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_GPU') == '0'
root = Path(__file__).resolve().parents[2]
lane = Path(__file__).resolve().parent
art = Path(os.environ['PEERS_ART'])
proof = Path(os.environ['MESHOPT_LANE_SCRATCH'])
campaign = art / os.environ.get('MESHOPT_CAMPAIGN', 'campaign-1')
result = json.loads((campaign / 'results.json').read_text())
assert not result['regressions']
assert json.loads((campaign / 'measurement-exit.json').read_text())['exit'] == 0
exits = [line.split('\t') for line in (proof / 'proof-exits.tsv').read_text().splitlines()]
assert all(code == '0' for _, code in exits)
assert len(exits) == 27, exits

def sha(p):
    with p.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

binding = json.loads((proof / 'proof-source-binding.json').read_text())
for key, digest in binding.items():
    if key.startswith('candidate/'):
        assert sha(root / key.removeprefix('candidate/')) == digest
for name in ['counts.jsonl', 'results.json', 'campaign.json', 'measurement-exit.json']:
    shutil.copy2(campaign / name, lane / name)
for name in ['inputs.json', 'build-binding.json']:
    shutil.copy2(art / name, lane / name)
shutil.copy2(proof / 'proof-source-binding.json', lane / 'proof-source-binding.json')
receipts = dict(steps={name: int(code) for name, code in exits},
                final_helpers={p.name: sha(p) for p in lane.iterdir() if p.suffix in ['.py', '.rs', '.sh']},
                logs={p.name: sha(p) for p in proof.glob('proof-*.log')},
                binaries={p.name: sha(p) for p in proof.iterdir() if p.is_file() and os.access(p, os.X_OK)},
                fuzz=json.loads((proof / 'fuzz-exit.json').read_text()),
                cpp=json.loads((art / 'cpp-differential/result.json').read_text()),
                exact={p.name: p.read_text().strip() for p in proof.glob('proof-exact-*.log') if 'build' not in p.name},
                geometry={p.name: p.read_text().splitlines() for p in proof.glob('proof-geometry-exact-*.log')})
(lane / 'proof-results.json').write_text(json.dumps(receipts, indent=2) + '\n')
lines = [
    '# Toward C++: 0.2.15 instruction and contract proof', '',
    'Baseline: v0.2.14. One registered four-arm instruction-only campaign,',
    'median of three per cell, five frozen corpus meshes and three real meshes.',
    'All accepted counters ran 100%; LC_ALL=C. No wall-time claim.', '',
    '## Attribution and techniques', '',
    'The retained 0214 contract-cost experiment attributes 39.5% of the cache',
    'gap to work accounting; for meshlets W/R explain 55.5%/29.8%, and for',
    'cluster LOD 41.8%/25.7%. These historical interventions are attribution,',
    'not savings measured for this candidate. Optimesh 1.1.0 cache, clusterizer',
    'and vertex-codec sources were read as MIT-licensed data; none was copied.', '',
    'The candidate batches infallible covered cache score visits, uses guarded',
    'native std roots in greedy/flex meshlets and evaluates exact scores instead',
    'of root brackets, and meters four private cluster-LOD simplifier phases',
    'under checked work bounds. PROOFS.md records file:line derivations and',
    'REVIEW.md the completing agent\'s read-only adversarial pass. No finite',
    'check, public API, unsafe code or SIMD implementation was added or removed.',
    'The inherited stride-12 SIMD specialization was removed as out of scope;',
    'vertex decoding has no retained source change.', '',
    'Shared-helper changes: only the private cluster-LOD snapshot\'s',
    '`support/workspace.rs` (Meter/Tally/dispatch) and `support/simplify.rs`',
    '(adjacency/pick/update/filter, local table/vector borrows). Main shared',
    'workspace/math/processing and other function families are unchanged.', '',
    '## Three-real-mesh geometric means', '',
    '| Function | Candidate/base | Candidate/optimesh | Base/C++ | Candidate/C++ | Verdict |',
    '|---|---:|---:|---:|---:|---|']
for op, v in result['ops'].items():
    s = {f: x['real'] for f, x in v['summary'].items()}
    opt = f"{s['candidate_optimesh']:.4f}" if s['candidate_optimesh'] is not None else 'N/A'
    lines.append(f"| {op} | {s['candidate_base']:.4f} | {opt} | {s['base_cpp']:.4f} | {s['candidate_cpp']:.4f} | {v['verdict']} |")
lines += ['', 'The context-specific C++ substitution in SPEC-0215-S5 and the inherited',
          'registration is applied: SHIPS requires candidate/base <=0.97 and',
          'base/C++ minus candidate/C++ >=0.05. IMPROVES requires only the former.',
          'The later generic optimesh formula conflicts with that explicit',
          'substitution; candidate/optimesh is still reported transparently.',
          'Every case must also be <=1.01 versus base.', '',
          '## Per-case medians of instructions per call', '',
          '| Case | Op | Base | Candidate | C++ | Optimesh | Candidate/base |',
          '|---|---|---:|---:|---:|---:|---:|']
maximum = 0.
for op, v in result['ops'].items():
    for name, cell in v['cases'].items():
        m = cell['medians']
        opt = f"{m['rust']:.0f}" if 'rust' in m else 'N/A'
        lines.append(f"| {name} | {op} | {m['base']:.0f} | {m['candidate']:.0f} | {m['cpp']:.0f} | {opt} | {cell['candidate_base']:.6f} |")
        maximum = max(maximum, cell['candidate_base'])
lines += ['', f'Maximum candidate/base: {maximum:.8f}. No case exceeds 1.01.',
          'All base/candidate and candidate/C++ canonical hashes match in every',
          'cell; the complete per-arm hashes and optimesh outputs are retained',
          'in results.json and counts.jsonl.', '',
          '## Proof receipts and boundaries', '',
          'All final proof steps below exited 0. The first proof attempt failed',
          'with exit 101 because the added geometry driver requested unsupported',
          'LOD defaults of size 2. A subsequent supervisor exited 2 before any',
          'proof step because its binding helper had been placed at the wrong',
          'location. The next attempt was deliberately terminated after the',
          'source binding changed during a proof-script restart fix. These',
          'failed/interrupted logs remain in scratch; the corrected supervisor',
          'was rerun in a fresh proof directory. Library code was unchanged',
          'through these harness repairs. Its 27 proof steps completed with',
          'exit 0, then the combined supervisor exited 1 when the peer build',
          'snapshot omitted the documentation included by lib.rs (Cargo exit',
          '101). The snapshot helper now includes and hashes those documents;',
          'the failed build log remains retained and no measurement sample',
          'had been taken. Proof-source-binding.json retains the helper version',
          'executed during proof; the final build helper differs only in that',
          'snapshot-document repair. The campaign is resumed separately.', '',
          '| Command / step | Exit |', '|---|---:|']
for name, code in exits:
    lines.append(f'| {name} | {code} |')
lines += ['', 'Exact cache/decode comparisons:']
for text in receipts['exact'].values():
    lines.append('- ' + text)
lines += ['', 'Connected geometry work sweeps:']
for name, values in receipts['geometry'].items():
    lines.append('- ' + name + ': ' + values[-1])
fuzz = receipts['fuzz']
lines += ['', f"Differential fuzz: {fuzz['cases']} cases, {fuzz['cpu_seconds']:.3f} child CPU seconds, exit 0.",
          'The generator now reaches >=256-face candidate lists and tiny/large',
          'finite coordinates, and includes attributes, locks and both LOD defaults.',
          'Exact drivers compare full caller buffers, results and Usage, callback',
          'records and position bits at every work limit through success+1 on the',
          'selected meshes and settings. Repeated/high-valence cache inputs and',
          'connected/repeated/coincident geometry exercise the work-bound paths.',
          'The inherited exact driver adds disconnected broad KD searches.', '',
          'These are finite boundary domains, supplemented by the written proof,',
          'rather than enumeration of all floating values, strides, numeric',
          'settings, option products or allocation failures. The native-root',
          'argument follows the baseline release disposition; its exhaustive',
          'root experiment was not rerun. Runtime verification is for this host',
          'only: other architectures, wasm and no_std performance were not',
          'measured. No independent review, release, merge, tag or publication',
          'is claimed. The change is submitted as a draft.', '',
          '## Reproduction and retention', '',
          'README.md gives the admitted build/proof/measurement sequence.',
          'REGISTRATION.md fixes arms and gates; source and binary bindings,',
          'input hashes, every accepted instruction sample and exact-step',
          'receipts are committed alongside this report. Raw outputs, perf',
          'records, profiles, private inputs, executables and admission logs',
          'remain in scratch. The dedicated target cleanup is recorded in',
          'cleanup.json after all lane jobs exit.']
(lane / 'REPORT.md').write_text('\n'.join(lines) + '\n')
print('portable receipts and REPORT.md exported')
