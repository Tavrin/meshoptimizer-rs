#!/usr/bin/env python3
"""SPEC-03 consumer-profile qualification, preserving the original P03 epochs.

Run build, prepare, reproduce, parity, timing and peer through moss-heavy.sh.
All evidence is append-only; a completed stream cannot be measured again.
See README.md for the admission commands and the registered stopping rule.
"""
import argparse
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import shutil
import statistics
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
ART = Path(os.environ.get('MESHOPT_ENCODER_ARTIFACTS', '/mnt/linux-extra/moss-scratch/meshopt-enc03'))
TARGET = Path('/mnt/linux-extra/moss-cargo-targets/codex-meshopt-enc03')
assert Path(os.environ['CARGO_TARGET_DIR']) == TARGET
os.environ['MESHOPT_ENCODER_ARTIFACTS'] = str(ART)
import run as lane

HISTORICAL = '13413c1e2c69f3de2f28a32005650fb46dfac1f92621cce393acefb3929040e9'
PROFILES = ('defaults', 'moss')
FAMILIES = ('vertex', 'index', 'sequence', 'oct', 'quat', 'exp')
PEER_OPS = ('vertex-encode', 'vertex-encode-v0', 'index-encode', 'sequence-encode',
            'oct-encode', 'quat-encode', 'exp-encode', 'color-encode')
T95 = (0, 0, 0, 0, 0, 2.776445, 2.570582, 2.446912, 2.364624, 2.306004,
       2.262157, 2.228139, 2.200985, 2.178813, 2.160369, 2.144787, 2.131450,
       2.119905, 2.109816, 2.100922, 2.093024)


def save(path, value):
    temp = path.with_suffix('.tmp')
    temp.write_text(json.dumps(value, separators=(',', ':')) + '\n')
    temp.replace(path)


def inventory():
    files = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock']
    files.extend(ROOT / 'parity' / name for name in ('runner.py', 'performance.py', 'quiet.py'))
    for directory in ('src', 'parity/codec', 'parity/encoders', 'parity/compare'):
        files.extend(p for p in (ROOT / directory).rglob('*')
                     if p.is_file() and p.suffix in ('.rs', '.py', '.cpp', '.toml', '.lock'))
    return {str(p.relative_to(ROOT)): lane.sha(p) for p in sorted(files)}


def interval(values):
    logs = [math.log(v) for v in values]
    center = statistics.mean(logs)
    radius = T95[min(len(logs), 20)] * statistics.stdev(logs) / math.sqrt(len(logs))
    return [math.exp(center - radius), math.exp(center + radius)]


def resolved(ci):
    return ci[0] >= .8 and ci[1] <= 1.25


def comparison_module():
    spec = importlib.util.spec_from_file_location('encoder_profile_peer', ROOT / 'parity/compare/measure.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.ART = ART / 'peer'
    return module


def prepare():
    assert not (ART / 'inputs.json').exists(), 'frozen inputs already exist'
    lane.prepare()
    # Rebuild the archived smooth/seamed/sparse/million inputs with the original
    # deterministic generator; validate their full container hashes against the
    # committed historical peer ledger before deriving P03 requests.
    peer = comparison_module()
    peer.ART.mkdir(exist_ok=True)
    for profile in PROFILES:
        for backend in ('ours', 'theirs'):
            shutil.copy2(ART / f'cmp-driver-{profile}-{backend}', peer.ART / f'cmp-driver-{profile}-{backend}')
    peer.prepare()
    old = json.loads((ROOT / 'parity/compare/results.json').read_text())
    original = {r['case']: r['input_sha256'] for r in old['timing']['defaults']['cases']
                if r['operation'] == 'vertex-encode'}
    rows = json.loads((ART / 'inputs.json').read_text())
    for entry in json.loads((peer.ART / 'inputs.json').read_text()):
        name = entry['name']
        assert entry['sha256'] == original[name], (name, 'historical geometry mismatch')
        if name == 'tiny-smooth':
            continue
        b = (peer.ART / 'inputs' / (name + '.bin')).read_bytes()
        n, ni = struct.unpack_from('<II', b)
        vertices = b[8:8 + n * 12]
        indices = b[8 + n * 12:8 + n * 12 + ni * 4]
        configs = [(0, 2), (1, 0), (1, 1), (1, 2), (1, 3)] if name != 'million-smooth' else [(0, 2), (1, 2)]
        cases = [(f'vertex-geometry-{name}-v{v}-l{l}', lane.request(11, n, 12, vertices, version=v, level=l)) for v, l in configs]
        cases += [(f'{family}-geometry-{name}-v{v}', lane.request(op, ni, 4, indices, version=v))
                  for op, family in ((12, 'index'), (13, 'sequence')) for v in (0, 1)]
        for label, request in cases:
            path = ART / 'inputs' / (label + '.input')
            assert not path.exists()
            path.write_bytes(request)
            rows.append({'name': label, 'path': str(path), 'sha256': lane.sha(path)})
    assert len(rows) == 112
    assert next(r['sha256'] for r in rows if r['name'] == 'vertex-v1-streaming-s4') == HISTORICAL
    save(ART / 'inputs.json', rows)
    save(ART / 'registration.json', {
        'source': inventory(), 'reference': '4c203430ca565cb59a468a91922c76c208169536',
        'profiles': {'defaults': {'lto': False, 'codegen_units': 16}, 'moss': {'lto': 'thin', 'codegen_units': 1}},
        'cpu': 26, 'pairs': [5, 256], 'target_seconds': .025,
        'family_gm_limit': 1.25, 'case_upper95_limit': 1.5, 'peer_family_gm_limit': 1.0,
        'aa_interval_bounds': [.8, 1.25], 'historical_request': HISTORICAL,
        'rule': 'Check after 5 complete pairs. If the interval is not wholly on one side of 1.5 or either A/A interval fails resolution, collect exactly 256 pairs with no intermediate confidence decisions. Retain every sample and unresolved case.',
        'calibration': 'one identical timed probe per Rust/C++ and duplicate process before iteration selection',
        'burst_seconds': 690, 'input_sha256': lane.sha(ART / 'inputs.json'),
    })
    # Seal the independently validated peer consumer for measure.py.
    save(peer.ART / 'measurement-source.json', {'files': inventory()})


def measure(profile, group):
    registration = json.loads((ART / 'registration.json').read_text())
    assert registration['source'] == inventory(), 'source epoch changed; start a new named epoch'
    assert registration['input_sha256'] == lane.sha(ART / 'inputs.json')
    build = json.loads((ART / 'build.json').read_text())
    assert build['source'] == {str(p.relative_to(ROOT)): lane.sha(p) for p in (ROOT / 'src').rglob('*.rs')}
    names = {'rust': f'codec-driver-{profile}', 'cpp': 'cpp', 'rust-aa': f'codec-driver-{profile}', 'cpp-aa': 'cpp'}
    before_binding = None
    if os.environ.get('MESHOPT_ENCODER_BASELINE_BINARIES'):
        before_art = Path(os.environ['MESHOPT_ENCODER_BASELINE_BINARIES'])
        before_build = json.loads((before_art / 'build.json').read_text())
        names['before'] = str(before_art / f'codec-driver-{profile}')
        assert lane.sha(Path(names['before'])) == before_build['binaries'][f'codec-driver-{profile}']
        before_binding = {'build_sha256': lane.sha(before_art / 'build.json'), 'source': before_build['source']}
    identities = {k: lane.sha(ART / v) for k, v in names.items()}
    assert all(build['binaries'][names[k]] == v for k, v in identities.items() if k != 'before')
    path = ART / f'{group}-{profile}.json'
    result = json.loads(path.read_text()) if path.exists() else {
        'source': registration['source'], 'binaries': identities, 'profile': profile,
        'cpu': 26, 'before_binding': before_binding, 'rows': [], 'completed': False, 'bursts': []}
    assert not result['completed'], 'completed stream is immutable'
    assert result['binaries'] == identities and result['source'] == inventory()
    assert all(r['completed'] and 'pending_pair' not in r for r in result['rows']), 'interrupted case retained; do not silently discard or repeat it'
    cases = json.loads((ART / 'inputs.json').read_text())
    cases = [r for r in cases if (r['name'] == 'vertex-v1-streaming-s4') == (group == 'reproduce')]
    ds = {k: lane.Driver(ART / v) for k, v in names.items()}
    scalar = lane.Driver(ART / 'cpp-scalar')
    started = time.monotonic()
    burst = {'admission': os.environ['MOSS_HEAVY_ACTIVE'], 'start': time.time()}
    result['bursts'].append(burst)
    try:
        for case in cases[len(result['rows']):]:
            b = Path(case['path']).read_bytes()
            assert hashlib.sha256(b).hexdigest() == case['sha256']
            expected = scalar.call(b)[:2]
            assert expected[0] == 0
            assert all(d.call(b)[:2] == expected for d in ds.values()), case['name']
            probe = bytearray(b)
            struct.pack_into('<II', probe, 32, 1, 1)
            # Match allocator/warmup history in every timed and A/A process.
            calibration = {k: d.call(probe)[2][0] for k, d in ds.items()}
            trial = calibration['cpp']
            iterations = max(1, min(1000000, math.ceil(.025 / max(trial, 1e-9))))
            struct.pack_into('<I', probe, 36, iterations)
            row = {'case': case['name'], 'input_sha256': case['sha256'], 'output_sha256': hashlib.sha256(expected[1]).hexdigest(),
                   'iterations': iterations, 'calibration_seconds': calibration, 'samples': [], 'completed': False}
            result['rows'].append(row)
            save(path, result)
            for pair in range(256):
                keys = list(ds)
                keys = keys[pair % len(keys):] + keys[:pair % len(keys)]
                if pair % 2:
                    keys.reverse()
                sample = {'order': keys, 'seconds': {}, 'load': os.getloadavg(), 'unix': time.time()}
                row['pending_pair'] = sample
                save(path, result)
                for key in keys:
                    status, output, seconds = ds[key].call(probe)
                    assert (status, output) == expected
                    sample['seconds'][key] = seconds[0]
                    save(path, result)
                row['samples'].append(sample)
                del row['pending_pair']
                if pair in (4, 255):
                    values = [s['seconds'] for s in row['samples']]
                    ratios = [v['rust'] / v['cpp'] for v in values]
                    ci = interval(ratios)
                    aa = interval([v['rust-aa'] / v['rust'] for v in values])
                    ca = interval([v['cpp-aa'] / v['cpp'] for v in values])
                    if (ci[1] <= 1.5 or ci[0] > 1.5) and resolved(aa) and resolved(ca):
                        break
            row.update(ratio=statistics.median(ratios), ci95=ci, rust_aa_ci95=aa, cpp_aa_ci95=ca,
                       resolved=resolved(aa) and resolved(ca), completed=True)
            if 'before' in ds:
                before_ratios = [v['before'] / v['cpp'] for v in values]
                row.update(before_ratio=statistics.median(before_ratios), before_ci95=interval(before_ratios))
            save(path, result)
            print(profile, group, case['name'], row['ratio'], ci, row['resolved'], flush=True)
            if time.monotonic() - started >= 690:
                break
        result['completed'] = len(result['rows']) == len(cases)
        burst['seconds'] = time.monotonic() - started
        assert registration['source'] == inventory()
        save(path, result)
    finally:
        for driver in [*ds.values(), scalar]:
            driver.close()


def parity(profile):
    # Reuse the complete P03 seeded and edge/layout matrix, with profile-bound
    # binaries; preserve each profile receipt separately.
    for source, dest in ((f'codec-driver-{profile}', f'rust-{profile}'), (f'encoder-crate-oracle-{profile}', 'crate')):
        shutil.copy2(ART / source, ART / dest)
    lane.parity(profile)


def peer(profile):
    comparison = comparison_module()
    os.environ['MESHOPT_COMPARE_OPERATIONS'] = ','.join(PEER_OPS)
    comparison.measure(profile, 'timing')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('prepare', 'reproduce', 'timing', 'parity', 'peer'))
    parser.add_argument('profile', nargs='?', choices=PROFILES)
    args = parser.parse_args()
    assert os.environ.get('MOSS_HEAVY_ACTIVE'), 'run through moss-heavy.sh'
    ART.mkdir(parents=True, exist_ok=True)
    if args.action == 'prepare':
        prepare()
    else:
        assert args.profile, 'profile required'
        if args.action in ('reproduce', 'timing'):
            measure(args.profile, args.action)
        else:
            globals()[args.action](args.profile)
