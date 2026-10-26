#!/usr/bin/env python3
"""Export the sealed SPEC-03 epoch without replacing historical P03 results."""
import json
import os
from pathlib import Path
import statistics
import sys

import profiles as campaign

A = campaign.ART
R = campaign.ROOT
baseline = json.loads(Path(os.environ['MESHOPT_ENCODER_BASELINE']).read_text()) if os.environ.get('MESHOPT_ENCODER_BASELINE') else None
registration = json.loads((A / 'registration.json').read_text())
assert registration['source'] == campaign.inventory(), 'source epoch changed'


def read(name):
    return json.loads((A / name).read_text())


def gm(values):
    return statistics.geometric_mean(values)


result = {'scope': 'SPEC-03 allocating encoders, shared host, Cargo defaults and a downstream engine thin LTO',
          'evidence_directory': str(A), 'registration': registration, 'build': read('build.json'),
          'native': {}, 'historical_request': {}, 'peer': {}, 'parity': {}, 'evidence_sha256': {}}
lines = ['## SPEC-03 consumer-profile qualification', '',
         ('Before is the retained initial source epoch; after is the candidate epoch.' if baseline else 'No library implementation change was made in this initial source epoch.'),
         'Ratios are Rust/comparator elapsed time. Policy epochs and every sample are retained.', '',
         '| Profile | Family | Before → after GM / C++ 1.3 | Case CI envelope | Max upper95 | Verdict |',
         '|---|---|---:|---:|---:|---|']
accepted = True
for profile in campaign.PROFILES:
    timing = read(f'timing-{profile}.json')
    historical = read(f'reproduce-{profile}.json')
    assert timing['completed'] and historical['completed']
    assert all(row['completed'] for row in timing['rows'] + historical['rows'])
    result['native'][profile] = {'rows': timing['rows'], 'binaries': timing['binaries'], 'bursts': timing['bursts'], 'families': {}}
    result['historical_request'][profile] = {k: v for k, v in historical.items() if k != 'source'}
    for family in campaign.FAMILIES:
        rows = [r for r in timing['rows'] if r['case'].startswith(family + '-')]
        summary = {'cases': len(rows), 'resolved': sum(r['resolved'] for r in rows),
                   'geometric_mean_case_medians': gm([r['ratio'] for r in rows]),
                   'case_ci_envelope': [min(r['ci95'][0] for r in rows), max(r['ci95'][1] for r in rows)],
                   'maximum_case_median': max(r['ratio'] for r in rows)}
        summary['pass'] = all(r['resolved'] for r in rows) and summary['geometric_mean_case_medians'] <= 1.25 and summary['case_ci_envelope'][1] <= 1.5
        accepted &= summary['pass']
        result['native'][profile]['families'][family] = summary
        lo, hi = summary['case_ci_envelope']
        before = gm([r['before_ratio'] for r in rows]) if all('before_ratio' in r for r in rows) else summary['geometric_mean_case_medians']
        if all('before_ci95' in r for r in rows):
            summary['before_case_ci_envelope'] = [min(r['before_ci95'][0] for r in rows), max(r['before_ci95'][1] for r in rows)]
        summary['before_geometric_mean'] = before
        lines.append(f"| {profile} | {family} | {before:.3f} → {summary['geometric_mean_case_medians']:.3f} | {lo:.3f}–{hi:.3f} | {hi:.3f} | {'PASS' if summary['pass'] else 'FAIL'} |")
    parity = read(f'parity-{profile}.json')
    assert parity['mismatches'] == 0
    result['parity'][profile] = {'comparisons': len(parity['cases']), 'mismatches': parity['mismatches'], 'binaries': parity['binaries'], 'receipt_sha256': campaign.lane.sha(A / f'parity-{profile}.json')}
    peer = read(f'peer/measure-{profile}-timing.json')
    assert peer['completed'] and all(row['completed'] for row in peer['rows'])
    result['peer'][profile] = {'measurement': peer, 'families': {}}
    for operation in campaign.PEER_OPS:
        rows = [r for r in peer['rows'] if r['operation'] == operation]
        assert len(rows) == 5
        result['peer'][profile]['families'][operation] = {
            'geometric_mean_case_medians': gm([r['median_ratio'] for r in rows]),
            'case_ci_envelope': [min(r['ci95'][0] for r in rows), max(r['ci95'][1] for r in rows)],
            'byte_identical_cases': sum(r['byte_identical'] for r in rows)}
    # Match the v0/v1 cases as one family, rather than weighting vertex twice.
    families = {}
    for family in ('vertex', 'index', 'sequence', 'oct', 'quat', 'exp', 'color'):
        rows = [r for r in peer['rows'] if r['operation'].startswith(family + '-')]
        families[family] = {'geometric_mean_case_medians': gm([r['median_ratio'] for r in rows]),
                            'case_ci_envelope': [min(r['ci95'][0] for r in rows), max(r['ci95'][1] for r in rows)]}
        families[family]['pass'] = families[family]['geometric_mean_case_medians'] <= 1.0
    result['peer'][profile]['matched_families'] = families

lines += ['', 'The CI envelope is the minimum lower and maximum upper bound across cases;',
          'it is not a confidence interval for the family geometric mean.', '',
          '| Historical exact owned request | Ratio | Nominal 95% CI | A/A resolves |',
          '|---|---:|---:|---|']
for profile in campaign.PROFILES:
    row = result['historical_request'][profile]['rows'][0]
    lines.append(f"| {profile} | {row['ratio']:.3f} | {row['ci95'][0]:.3f}–{row['ci95'][1]:.3f} | {row['resolved']} |")
lines += ['', '| Profile | Shared family | GM / meshopt 0.6.2 | Case CI envelope | Target <=1.0 |',
          '|---|---|---:|---:|---|']
for profile in campaign.PROFILES:
    for family, summary in result['peer'][profile]['matched_families'].items():
        lo, hi = summary['case_ci_envelope']
        lines.append(f"| {profile} | {family} | {summary['geometric_mean_case_medians']:.3f} | {lo:.3f}–{hi:.3f} | {'PASS' if summary['pass'] else 'FAIL'} |")
if baseline:
    result['baseline_receipt_sha256'] = campaign.lane.sha(Path(os.environ['MESHOPT_ENCODER_BASELINE']))
result['native_encoder_acceptance'] = accepted
result['peer_acceptance'] = all(s['pass'] for p in result['peer'].values() for s in p['matched_families'].values())
result['owned_decode_acceptance'] = all(r['resolved'] and r['ci95'][1] <= 1.5 for p in result['historical_request'].values() for r in p['rows'])
for path in sorted(A.rglob('*')):
    if path.is_file() and path.suffix in ('.json', '.log', '.py') and path.name not in ('report.json',):
        result['evidence_sha256'][str(path.relative_to(A))] = campaign.lane.sha(path)
result['limitations'] = [
    'Nominal paired-log Student-t intervals, not simultaneous or sequentially adjusted confidence.',
    'Current compiler and released implementation: non-reproduction cannot establish the cause of historical gaps.',
    'Peer 0.6.2 bundles C++ 0.25; wrapper, version, allocation and compiler differences are included.',
    'Shared Linux x86-64 host; no quiet-machine wait, downstream runtime, browser, remote architecture or release qualification.',
]
campaign.save(A / 'report.json', result)
(A / 'report.md').write_text('\n'.join(lines) + '\n')
print('\n'.join(lines))
