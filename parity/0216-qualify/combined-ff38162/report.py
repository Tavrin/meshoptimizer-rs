"""Assemble the release report from the completed, retained campaign only.

Invoke through CPU admission; no build, benchmark or proof is repeated.
"""
import json
import re
from pathlib import Path

A = Path(__file__).resolve().parent
Q = A.parent


def load(name):
    return json.loads((A / name).read_text())


def main():
    r = load('release-results.json')
    assert not r['regressions'], r['regressions']
    assert r['campaign_samples'] == 3504 and r['samples'] == 3594
    assert load('final-source.json')['unchanged']
    jobs = [json.loads(s) for s in (A / 'jobs.jsonl').read_text().splitlines()]
    last = {j['stage']: j for j in jobs}
    required = ['prepare', 'proof_group', 'differential_group',
                'measure_inputs corpus', 'measure_inputs real-214e',
                'measure_inputs real-a082', 'measure_inputs real-b6fd',
                'disclosure_group']
    assert all(last[s]['exit'] == 0 for s in required)
    exits = [json.loads(s) for s in (A / 'exits.jsonl').read_text().splitlines()]
    assert all(e['exit'] == 0 for e in exits)
    wall = load('wall-results.json')
    cost = load('cost-results.json')
    worst = max((x['ratios'][k], x['case'], x['op'], k)
                for x in r['cells'] for k in ['scalar_base', 'default_base'])
    proof = (A / 'PROOF.md').read_text().split('\n', 1)[1]
    diff = (A / 'DIFFERENTIALS.md').read_text().split('\n', 1)[1]
    table = (Q / 'TABLE.md').read_text().split('\n', 1)[1]
    # The section files live one directory below the consolidated report.
    def links(text):
        return re.sub(r'\]\(([^)]+\.md)\)',
                      lambda m: '](combined-ff38162/' + m[1] + ')', text)
    sections = [
        '# SPEC-0216-qualify — 0.2.16 combined train qualification',
        '',
        '**PASS for the specified combined qualification. No lane reverted.**',
        '',
        'Candidate `ff38162d6de2c6c543a45536733442939100801b`; base v0.2.15 '
        '`1ecf69abe9f92c17e13dc38fa97be659aee44582`. Production source, '
        'Cargo manifest and lockfile are unchanged. This report uses only the '
        'current [combined campaign](combined-ff38162/REGISTRATION.md). '
        'The earlier, different-baseline qualification is preserved in '
        '[REPORT-pre-train.md](REPORT-pre-train.md); its receipts and broader '
        'proof requirements are not substituted for this specification.',
        '',
        '## Exact merged-code equivalence',
        links(proof),
        '## C++ differentials and arm64 rule',
        links(diff),
        '## Release measurements',
        table,
        f'Maximum individual base ratio: **{worst[0]:.9f}**, '
        f'`{worst[1]}` / `{worst[2]}` / `{worst[3]}`. '
        'No case exceeds 1.01. All 336 base/train and 336 matched-mode '
        'train/C++ output comparisons are equal. Ratios describe these '
        'registered inputs and owned-call workloads; no wall-time speedup '
        'is inferred from instruction counts.',
        '',
        '## Parallel cluster LOD wall disclosure',
        '',
        'Five interleaved AB/BA repetitions on each real mesh; medians in '
        'seconds. One and four workers use one and four distinct physical '
        'cores respectively. All 30 serialized callback outputs are equal '
        'to each other per mesh and to default serial cluster LOD. '
        'The clock surrounds the owned call, including output destruction, '
        'and excludes loading and serialization.',
        '',
        '| Input | 1 worker, seconds | 4 workers, seconds | Speedup |',
        '|---|---:|---:|---:|',
    ]
    sections += [f"| {x['case']} | {x['one_worker_seconds']:.6f} | "
                 f"{x['four_worker_seconds']:.6f} | {x['speedup']:.3f}x |"
                 for x in wall]
    sections += [
        '',
        '[Raw wall samples](combined-ff38162/wall.jsonl) and '
        '[medians](combined-ff38162/wall-results.json).',
        '',
        '## Streaming and repair cost disclosure',
        '',
        'Corpus only, default build, median of three instructions:u samples. '
        'The repair ratios compare distinct calls with sanitize EMPTY on '
        'the same raw mesh: positions-only zero-tolerance weld, face '
        'orientation, and fill with a 16-edge maximum. Streaming decodes '
        'version-1 index sequences in 4096-byte chunks into owned u32 output; '
        'one-shot produces owned packed 32-bit bytes. Setup asserts equal '
        'decoded bytes. Allocation and destruction are counted; encoding, '
        'loading, validation of equal outputs and serialization are outside '
        'counters. These calls provide different functionality.',
        '',
        '| Corpus input | weld / sanitize | orient / sanitize | fill / sanitize | streaming / one-shot |',
        '|---|---:|---:|---:|---:|',
    ]
    for x in cost['cases']:
        c = x['repair_vs_sanitize']
        sections.append(f"| {x['case']} | {c['weld']:.3f} | {c['orient']:.3f} | "
                        f"{c['fill']:.3f} | {x['stream_vs_one_shot']:.3f} |")
    g = cost['corpus_gm']
    sections += [
        f"| Corpus GM | {g['weld']:.3f} | {g['orient']:.3f} | {g['fill']:.3f} | {g['stream']:.3f} |",
        '',
        '[Per-case instruction medians](combined-ff38162/cost-results.json).',
        '',
        '## Gates, retained attempts and limits',
        '',
        '| Gate | Exit |',
        '|---|---:|',
        '| All-feature and no-default exact lane drivers | 0 each |',
        '| x86 C++ differentials, including forced SIMD | 0 each |',
        '| LOD corpus identity, both input sets and feature modes | 0 each |',
        '| Parallel identity: original grid and 1/4/16 workers, both modes | 0 each |',
        '| Cargo tests and doctests, both feature modes | 0 each |',
        '| Clippy all targets, both feature modes | 0 each |',
        '| Formatting; package/unsafe boundary | 0 each |',
        '| Instruction regression gate | 0; no ratio above 1.01 |',
        '',
        'Final source/reference, public-text and whitespace checks, retained '
        'counter-file verification and binary/input bindings are recorded in '
        '[final-verification.json](combined-ff38162/final-verification.json) '
        'and [exits.jsonl](combined-ff38162/exits.jsonl). '
        'Disposable-target removal is recorded in '
        '[cleanup-final.json](combined-ff38162/cleanup-final.json).',
        '',
        'The full staged whitespace audit exits 2 because six raw Cargo '
        'test/identity logs end with an extra blank line. Their recorded '
        'bytes and hashes are preserved. The staged audit excluding raw '
        'logs and the final public-text check exit 0; see '
        '[staged-review.json](combined-ff38162/staged-review.json).',
        '',
        'Every build and execution entered CPU-only admission with declaration '
        '4. Pre-run sizes and actual child/supervisor CPU are retained in '
        '[sizes.jsonl](combined-ff38162/sizes.jsonl), '
        '[job-sizing.jsonl](combined-ff38162/job-sizing.jsonl) and '
        '[jobs.jsonl](combined-ff38162/jobs.jsonl). Builds pause below the '
        'registered 100 GiB free-space floor. '
        f"The largest completed campaign/proof job used {max(j.get('total_cpu_seconds', j['child_cpu_seconds']) for j in jobs):.6f} "
        'CPU seconds, below the 7200-second maximum.',
        '',
        'The interrupted orchestration was resumed by waiting for its live '
        'processes; no duplicate campaign was launched and no queued or '
        'running job was canceled. Its counts were reused unchanged. '
        'The initial preparation attempt exited 1 before any build or '
        'comparison because retained old input links were broken; the '
        'hash-verified replacement input copy and successful retry are '
        'disclosed in [attempts.json](combined-ff38162/attempts.json). '
        'That infrastructure attempt is retained and is not a failed exact '
        'or measurement gate.',
        '',
        'No fuzzers ran in this qualification. Fuzzing is a separate lane '
        'and is not verified here. Arm64 is supported by the source-identical '
        'Linux/Apple CI receipts, not local execution. Malformed C++ '
        'destination prefixes and upstream C++ NEON ISA divergences remain '
        'explicit disclosures above; no C++ failed-buffer byte-identity '
        'claim is made. Exact counts describe the original finite lane '
        'drivers rather than an exhaustive public-input universe.',
        '',
    ]
    (Q / 'REPORT.md').write_text('\n'.join(sections))
    print('Release report assembled from completed retained receipts')


if __name__ == '__main__':
    main()
