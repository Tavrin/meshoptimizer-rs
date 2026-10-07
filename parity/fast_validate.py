#!/usr/bin/env python3
"""Compare complete fast matrices with same-revision full records, fail closed."""
import argparse
from datetime import datetime
import hashlib
import json
import math
from pathlib import Path
import statistics
import struct
import subprocess

if __package__:
    from .fast_qualify import ART, ROOT, sha, write, family_verdict, fs
else:
    from fast_qualify import ART, ROOT, sha, write, family_verdict, fs

REQUIRED = {('main', '0.1', 'moss'), ('main', '0.1', 'default'),
            ('main', '0.2', 'default'), ('main', '0.4', 'moss'),
            ('main', '0.4', 'default'), ('main', '0.3', 'consumer'),
            ('main', '0.3', 'release-defaults'),
            ('phase/0.1.x', '0.1.x', 'moss'), ('phase/0.1.x', '0.1.x', 'default'),
            ('phase/0.1.x', '0.1', 'crate')}
HISTORICAL_01X_WRAPPERS = {'parity/benchmark.sh', 'parity/fuzz.sh',
                           'parity/report.sh', 'parity/run.sh', 'parity/sweep.sh'}
P02_MEASURE_BRIDGE = {
    'full': '05847a18a5305411fbabf0772c1ec96f93679ff63883a2a674d21e5822877465',
    'fast': '3e13e73e3fc520f60a60bf9f25f30288d305f9af542a330d515242e16164d36a',
}


def source_hashes(source, root=None):
    """Compare library files despite different read-only checkout path prefixes."""
    if 'source_sha256' in source:
        source = source['source_sha256']['rust_and_harness']
    if 'sources' in source:
        source = source['sources']
    result = {}
    for name, digest in source.items():
        if root and name.startswith(root.rstrip('/') + '/'):
            tail = name[len(root.rstrip('/'))+1:]
        elif '/meshopt-wt/' in name:
            tail = name.split('/meshopt-wt/', 1)[1]
            tail = tail.split('/', 1)[1] if '/' in tail else ''
        elif '/meshoptimizer-rs/' in name:
            tail = name.split('/meshoptimizer-rs/', 1)[1]
        else:
            tail = name
        if tail.startswith('src/'):
            result[tail] = digest
    return result


def canonical_snapshot(snapshot, root=None):
    """Normalize checkout prefixes while retaining oracle and dependency files."""
    result = {}
    for name, digest in snapshot.items():
        if root and name.startswith(root.rstrip('/') + '/'):
            name = name[len(root.rstrip('/')) + 1:]
        elif '/meshopt-wt/' in name:
            name = name.split('/meshopt-wt/', 1)[1].split('/', 1)[1]
        result[name] = digest
    return result


def full_rows(full, phase):
    if phase in ('0.1', '0.1.x'):
        rows = {name: {'ratio': row.get('rust_cpp_ratio', row['paired_ratio_stats']['median']),
                       'memory_ratio': row['memory_ratio'] if row['memory_ratio'] is not None else math.inf,
                       'input_sha256': row['input']['sha256'],
                       'samples': row['paired_ratio_stats']['raw_ratios']}
                for name, row in full['workloads'].items()}
        families = {name: row['passed'] for name, row in full['families'].items()}
        wall = (full['finished_unix'] - full['started_unix']
                if 'finished_unix' in full and 'started_unix' in full else full.get('wall_seconds'))
        identity = full['identities']
    elif phase == '0.3':
        rows = {}
        for row in full['rows']:
            if row['baseline'] != 'cpp':
                continue
            rows[row['family'] + '/' + row['case']] = {
                'ratio': row['ratio'], 'memory_ratio': row['memory_ratio'],
                'input_sha256': row['input_sha256'],
                'samples': [s['ratio'] for s in row['samples']]}
        families = {name: row['pass'] for name, row in full['verdicts']['cpp'].items()}
        wall = full['elapsed']
        identity = full['identity']
    elif phase == '0.4':
        rows = {}
        for api in ('allocating', 'caller_buffer'):
            for row in full[api]:
                name = row['case']
                family, label = name.split('/', 1)
                rust = row['raw_seconds']['rust']
                cpp = row['raw_seconds']['scalar']
                rows[f'{family}/{api}/{label}'] = {
                    'ratio': row['rust_scalar_time_ratio'], 'memory_ratio': 1.0,
                    'samples': [a/b for a, b in zip(rust, cpp, strict=True)]}
        families = {f'{family}/{api}': value['pass'] for api, per_api in full['verdicts'].items()
                    for family, value in per_api.items() if family != 'bounds' or api == 'allocating'}
        wall = (datetime.fromisoformat(full['finished_utc'].replace('Z', '+00:00')) -
                datetime.fromisoformat(full['started_utc'].replace('Z', '+00:00'))).total_seconds()
        identity = {'sources': full['sources']}
    elif phase == '0.2':
        rows = {}
        for api in ('allocating', 'caller_buffer'):
            for row in full[api]:
                name = row['case']
                family = name.split('-')[0]
                rows[f'{family}/{api}/{name}'] = {
                    'ratio': row['rust_scalar_time_ratio'], 'memory_ratio': 1.0,
                    'minimum_pass': row.get('bar_pass'),
                    'minimum': row.get('minimum_bytes_second'),
                    'decoded_bytes': row['decoded_bytes'],
                    'rust_samples': row['raw_seconds']['rust'],
                    'samples': [a/b for a, b in zip(row['raw_seconds']['rust'],
                                                     row['raw_seconds']['scalar'], strict=True)]}
        families = {api: value['minimum_bar_pass'] and value['raw_scalar_bar']['pass']
                    for api, value in full['verdicts'].items()}
        wall = full.get('wall_seconds')
        identity = {'sources': full['sources']}
    else:
        raise ValueError('unsupported phase')
    return rows, families, wall, identity


def full_case_pass(name, row, phase):
    if phase != '0.2':
        return row['ratio'] <= 1.5 and row['memory_ratio'] <= 1.25
    registered = row['minimum_pass']
    raw = name.split('/')[0] in ('vertex', 'index')
    return (registered is not False) and (not raw or row['ratio'] <= 1.5)


def within_full_noise(name, row, phase, current=None):
    """Every gate responsible for a disagreement must lie within its own noise."""
    if current is not None and phase != '0.2' and ((row['memory_ratio'] <= 1.25) !=
                                                 (current['memory_ratio'] <= 1.25)):
        return False  # Requested heap bytes are deterministic, not timing noise.
    tests = []
    if phase == '0.2' and row['minimum'] is not None:
        rust = row['rust_samples']
        q = statistics.quantiles(rust, n=4)
        threshold = row['decoded_bytes']/row['minimum']
        changed = (current is None or (statistics.median(rust) <= threshold) !=
                   (statistics.median(current['samples_seconds']['rust']) <= threshold))
        if changed:
            tests.append(q[0] <= threshold <= q[2])
    if phase != '0.2' or name.split('/')[0] in ('vertex', 'index'):
        if current is None or (row['ratio'] <= 1.5) != (current['ratio'] <= 1.5):
            q = statistics.quantiles(row['samples'], n=4)
            tests.append(q[0] <= 1.5 <= q[2])
    return bool(tests) and all(tests)


def cv(values):
    return statistics.stdev(values)/statistics.mean(values)


def read_record(location):
    if location.startswith('tar:'):
        archive, marker, member = location[4:].partition('!')
        if not marker or not member:
            raise ValueError('tar record must be tar:/path/archive.tar!member')
        data = subprocess.check_output(['tar', '-xOf', archive, member])
    else:
        data = Path(location).read_bytes()
    return json.loads(data), hashlib.sha256(data).hexdigest()


def verify_codec_inputs(item, fast_rows):
    """Bind codec case bytes to the archived base corpus and API transform."""
    if item['phase'] not in ('0.2', '0.4'):
        return None
    location = item['full_input_manifest']
    manifest, digest = read_record(location)
    for entry in manifest:
        name = entry['case']
        if item['phase'] == '0.2':
            case = f"{name.split('-')[0]}/allocating/{name}"
        else:
            family, label = name.split('/', 1)
            case = f'{family}/allocating/{label}'
        if fast_rows[case]['input_sha256'] != entry['sha256']:
            raise ValueError('archived corpus input differs: ' + case)
    for case, row in fast_rows.items():
        data = Path(row['input']).read_bytes()
        if hashlib.sha256(data).hexdigest() != row['input_sha256']:
            raise ValueError('fast case input changed: ' + case)
        if '/allocating/' not in case:
            continue
        caller = case.replace('/allocating/', '/caller_buffer/', 1)
        if caller not in fast_rows:
            continue
        request = bytearray(data)
        flags = struct.unpack_from('<I', request, 16)[0]
        struct.pack_into('<I', request, 16, flags | 128)
        if hashlib.sha256(request).hexdigest() != fast_rows[caller]['input_sha256']:
            raise ValueError('caller-buffer input is not the full-method transform: ' + caller)
    return digest


def verify_fast_record(record):
    """Recompute stored decisions; duplicate or partial rows never cover a matrix."""
    rows = record['rows']
    names = [row['case'] for row in rows]
    maximum = record['sequential_error']['maximum_pairs']
    if not rows or len(set(names)) != len(names) or not 20 <= maximum <= 90:
        raise ValueError('empty, duplicate or invalid fast matrix')
    if record.get('bar') != {'family_geometric_mean': 1.25, 'case_maximum': 1.5, 'memory': 1.25}:
        raise ValueError('fast matrix changed the registered bars')
    alpha = .05 / len(rows)
    if record['sequential_error'].get('per_case_alpha') != alpha:
        raise ValueError('fast matrix has a different error budget')
    phase = record['phase']
    for row in rows:
        rust, scalar = (row['samples_seconds'][key] for key in ('rust', 'scalar'))
        if not 12 <= len(rust) <= maximum:
            raise ValueError('partial fast timing stream: ' + row['case'])
        decision = fs.stop(rust, scalar, alpha / 2 if phase == '0.2' else alpha,
                           paired=phase in ('0.1', '0.1.x', '0.3'), maximum=maximum)
        if phase == '0.2':
            decision = fs.codec_stop(decision, rust, alpha / 2,
                                     row['registered_minimum'], row['decoded_bytes'],
                                     row['raw_scalar'], maximum_pairs=maximum)
            passed = ((not row['raw_scalar'] or decision['estimate'] <= 1.5) and
                      (row['registered_minimum'] is None or
                       row['decoded_bytes']/statistics.median(rust) >= row['registered_minimum']))
        else:
            passed = decision['estimate'] <= 1.5 and row['memory_ratio'] <= 1.25
        if (not decision['stopped'] or row['sequential'] != decision or
                row['ratio'] != decision['estimate'] or row['case_pass'] != passed):
            raise ValueError('fast decision differs from raw timings: ' + row['case'])
    families = {name: family_verdict([row for row in rows if row['family'] == name], phase)
                for name in {row['family'] for row in rows}}
    if record['families'] != families:
        raise ValueError('fast family decisions differ from raw timings')


def compare(item):
    fast_path = Path(item['fast'])
    fast, fast_digest = read_record(item['fast'])
    full, full_digest = read_record(item['full'])
    phase = item['phase']
    verify_fast_record(fast)
    if (fast.get('phase') != phase or fast.get('profile') != item['profile'] or
            not fast.get('complete') or fast.get('smoke') or not fast.get('full') or
            not fast.get('source_unchanged') or not fast.get('binary_unchanged') or
            any(row.get('reused') for row in fast.get('rows', []))):
        raise ValueError('incomplete or mismatched fast matrix: ' + str(fast_path))
    if (fast.get('harness_sha256') != sha(ROOT / 'parity/fast_qualify.py') or
            fast.get('statistics_sha256') != sha(ROOT / 'parity/fast_stats.py')):
        raise ValueError('fast matrix used a different harness or stopping rule: ' + str(fast_path))
    if (full.get('completed', full.get('complete', True)) is not True or
            full.get('identities_unchanged', full.get('source_unchanged', True)) is not True or
            full.get('phase', phase) != phase or
            full.get('consumer_profile', full.get('rust_profile', full.get('profile', item['profile']))) !=
            ('defaults' if phase == '0.1.x' and item['profile'] == 'default' else item['profile'])):
        raise ValueError('incomplete or mismatched full matrix: ' + item['full'])
    if phase in ('0.1', '0.1.x'):
        overrides = full.get('profile_overrides', full['identities'].get('effective_profile_overrides', {}))
        if overrides != fast['identity'].get('effective_profile_overrides', {}):
            raise ValueError('fast and full 0.1 build profiles differ: ' + item['full'])
    if phase == '0.4' and full['effective_profile_overrides'] != fast['identity']['profile']:
        raise ValueError('fast and full 0.4 build profiles differ: ' + item['full'])
    old_rows, old_families, old_wall, old_identity = full_rows(full, phase)
    if phase == '0.3' and (not old_identity.get('dependencies') or
                           old_identity['dependencies'] != fast['identity'].get('dependencies')):
        raise ValueError('0.3 full dependency bytes are missing or differ; use a fresh matched full run')
    if item.get('full_wall_seconds') is not None:
        old_wall = item['full_wall_seconds']
    if old_wall is not None and (not math.isfinite(old_wall) or old_wall <= 0):
        raise ValueError('invalid full-method wall duration')
    if not math.isfinite(fast['wall_seconds']) or fast['wall_seconds'] <= 0:
        raise ValueError('invalid fast-method wall duration')
    fast_rows = {row['case']: row for row in fast['rows']}
    fast_families = {name: value['pass'] for name, value in fast['families'].items()}
    if set(fast_rows) != set(old_rows) or set(fast_families) != set(old_families):
        raise ValueError('case or family inventory differs: ' + str(fast_path))
    input_manifest_digest = verify_codec_inputs(item, fast_rows)
    if phase == '0.4' and full['input_manifest_sha256'] != input_manifest_digest:
        raise ValueError('0.4 full record does not name the archived input manifest')
    if phase == '0.2':
        registered = json.loads((ROOT / 'parity/P02_RESULTS.json').read_text())
        baseline, baseline_digest = read_record(item['registered_baseline'])
        if (baseline_digest != registered['baseline_sha256'] or
                full['baseline_sha256'] != baseline_digest or
                full['registered_bar_sha256'] != registered['registered_bar_sha256'] or
                baseline['input_manifest_sha256'] != input_manifest_digest):
            raise ValueError('0.2 registered baseline, bar or corpus identity differs')
    fast_sources = source_hashes(fast['identity'], fast.get('source_root'))
    full_root = item.get('full_source_root', item.get('source_root'))
    old_sources = source_hashes(old_identity, full_root)
    if not fast_sources or fast_sources != old_sources:
        raise ValueError('library source revision differs: ' + str(fast_path))
    if phase in ('0.1', '0.1.x'):
        newer = fast['identity']['source_sha256']
        older = full['identities']['source_sha256']
        different = {name for name, digest in older['rust_and_harness'].items()
                     if newer['rust_and_harness'].get(name) != digest}
        allowed = HISTORICAL_01X_WRAPPERS if item['revision'] == 'phase/0.1.x' else set()
        if (newer['upstream'] != older['upstream'] or
                newer['dependencies'] != older['dependencies'] or
                different - allowed or
                any(name not in older['rust_and_harness'] and
                    Path(name).name not in {'fast_qualify.py', 'fast_stats.py', 'fast_validate.py',
                                           'test_fast_stats.py', 'test_fast_validate.py', 'test_fast_qualify.py',
                                           'prepare_fast_validation.py', 'run_fast_validation.py'}
                    for name in newer['rust_and_harness'])):
            raise ValueError('0.1 full source, oracle or dependency snapshot differs')
    else:
        newer = canonical_snapshot(fast['identity']['sources'], fast.get('source_root'))
        older = canonical_snapshot(old_identity['sources'], full_root)
        differences = {name for name in newer.keys() | older.keys()
                       if newer.get(name) != older.get(name)}
        # The 0.2 source export moves the original candidate case builder into
        # candidate_cases(), so the historical measure.py hash cannot match.
        # Every case byte and all other source/dependency hashes must match.
        allowed = {'parity/codec/measure.py'} if phase == '0.2' else set()
        if differences - allowed:
            raise ValueError('full source, oracle or dependency snapshot differs: ' +
                             ', '.join(sorted(differences - allowed)))
        if phase == '0.2' and 'parity/codec/measure.py' in differences and (older.get('parity/codec/measure.py') != P02_MEASURE_BRIDGE['full'] or
                               newer.get('parity/codec/measure.py') != P02_MEASURE_BRIDGE['fast']):
            raise ValueError('unreviewed 0.2 measure.py source bridge')
    disagreements = []
    noise = []
    for name, current in fast_rows.items():
        old = old_rows[name]
        if phase == '0.2' and current['registered_minimum'] != old['minimum']:
            raise ValueError('fast case changed the registered decoder floor: ' + name)
        if old.get('input_sha256') and current['input_sha256'] != old['input_sha256']:
            raise ValueError('case input differs from full record: ' + name)
        if bool(current['case_pass']) != full_case_pass(name, old, phase):
            disagreements.append({'case': name, 'fast_ratio': current['ratio'],
                                  'full_ratio': old['ratio'],
                                  'fast_case_pass': current['case_pass'],
                                  'full_case_pass': full_case_pass(name, old, phase),
                                  'full_noise_covers_bar': within_full_noise(name, old, phase, current),
                                  'fast_throughput_bytes_second': (current['decoded_bytes']/statistics.median(current['samples_seconds']['rust']) if phase == '0.2' else None),
                                  'full_throughput_bytes_second': (old['decoded_bytes']/statistics.median(old['rust_samples']) if phase == '0.2' else None),
                                  'registered_minimum_bytes_second': old.get('minimum'),
                                  'fast_pairs': len(current['samples_seconds']['rust']),
                                  'full_pairs': len(old['samples'])})
        noise.append((cv(old['samples']),
                      cv([a/b for a, b in zip(current['samples_seconds']['rust'],
                                               current['samples_seconds']['scalar'], strict=True)])))
    family_differences = [name for name in fast_families if fast_families[name] != old_families[name]]
    family_measurements = {name: {'fast': fast['families'][name],
                                  'full': full.get('families', {}).get(name) or
                                  full.get('verdicts', {}).get('cpp', {}).get(name) or
                                  (full.get('verdicts', {}).get(name.rsplit('/', 1)[1], {}).get(name.rsplit('/', 1)[0]) if '/' in name else None) or
                                  full.get('verdicts', {}).get(name)}
                           for name in family_differences}
    return {'revision': item['revision'], 'phase': phase, 'profile': item['profile'],
            'fast_record': str(fast_path), 'full_record': item['full'],
            'fast_sha256': fast_digest, 'full_sha256': full_digest,
            'full_input_manifest_sha256': input_manifest_digest,
            'cases': len(fast_rows), 'families': len(fast_families),
            'case_agreements': len(fast_rows)-len(disagreements),
            'family_agreements': len(fast_families)-len(family_differences),
            'disagreements': disagreements, 'family_disagreements': family_differences,
            'family_measurements': family_measurements,
            'fast_wall_seconds': fast['wall_seconds'], 'full_wall_seconds': old_wall,
            'full_wall_provenance': item.get('full_wall_provenance', 'record timestamps'),
            'median_full_pair_cv': statistics.median(pair[0] for pair in noise),
            'median_fast_pair_cv': statistics.median(pair[1] for pair in noise)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('manifest', type=Path, help='JSON with comparisons and smoke pairs')
    args = parser.parse_args()
    (ART / 'validation-receipt.json').unlink(missing_ok=True)
    manifest = json.loads(args.manifest.read_text())
    results = []
    errors = []
    for item in manifest['comparisons']:
        try:
            results.append(compare(item))
        except (OSError, KeyError, ValueError, subprocess.CalledProcessError) as error:
            errors.append(f"{item['revision']} {item['phase']}/{item['profile']}: {error}")
    covered = {(r['revision'], r['phase'], r['profile']) for r in results}
    missing = sorted(REQUIRED - covered)
    total_cases = sum(r['cases'] for r in results)
    total_families = sum(r['families'] for r in results)
    case_disagreements = [d for r in results for d in r['disagreements']]
    family_disagreements = [d for r in results for d in r['family_disagreements']]
    pair_limits = {json.loads(Path(r['fast_record']).read_text())['sequential_error']['maximum_pairs']
                   for r in results}
    if len(pair_limits) != 1:
        errors.append('all validation matrices must use the same maximum-pair limit')
    noise_pairs = []
    for item in manifest.get('smoke_noise', []):
        try:
            a, b = (json.loads(Path(item[key]).read_text()) for key in ('single', 'parallel'))
            verify_fast_record(a)
            verify_fast_record(b)
        except (OSError, KeyError, ValueError) as error:
            errors.append('invalid smoke record: ' + str(error))
            continue
        if not a.get('smoke') or not b.get('smoke'):
            errors.append('noise comparison requires two labelled smoke records')
            continue
        if (any(not record.get('complete') or not record.get('source_unchanged') or
                not record.get('binary_unchanged') or record.get('release_grade')
                for record in (a, b)) or
                len(a.get('selection', {}).get('selected', [])) != 1 or
                len(b.get('selection', {}).get('selected', [])) < 2):
            errors.append('noise comparison requires complete single-core and parallel smoke runs')
            continue
        if any(record.get('harness_sha256') != sha(ROOT / 'parity/fast_qualify.py') or
               record.get('statistics_sha256') != sha(ROOT / 'parity/fast_stats.py')
               for record in (a, b)):
            errors.append('noise comparison used a different harness or stopping rule')
            continue
        left = {row['case']: row for row in a['rows']}
        right = {row['case']: row for row in b['rows']}
        if (not left or len(left) != len(a['rows']) or len(right) != len(b['rows']) or
                left.keys() != right.keys() or a['identity'] != b['identity'] or
                a.get('phase') != b.get('phase') or a.get('profile') != b.get('profile') or
                any(left[name]['input_sha256'] != right[name]['input_sha256'] for name in left)):
            errors.append('smoke case/source identity differs')
            continue
        noise_pairs.append({'single': item['single'], 'parallel': item['parallel'],
                            'cases': len(left),
                            'single_cv': statistics.median(cv([x/y for x, y in zip(row['samples_seconds']['rust'], row['samples_seconds']['scalar'], strict=True)]) for row in left.values()),
                            'parallel_cv': statistics.median(cv([x/y for x, y in zip(row['samples_seconds']['rust'], row['samples_seconds']['scalar'], strict=True)]) for row in right.values())})
    accepted = (not missing and not errors and bool(noise_pairs) and
                not family_disagreements and
                all(d['full_noise_covers_bar'] for d in case_disagreements) and
                all(r['full_wall_seconds'] is not None for r in results))
    lines = ['# Fast qualification validation', '',
             f"**Verdict: {'ACCEPTED' if accepted else 'NOT RELEASE-GRADE'}**.", '',
             f'Case agreement: {total_cases-len(case_disagreements)}/{total_cases}; family agreement: {total_families-len(family_disagreements)}/{total_families}.', '',
             '| Revision | Phase/profile | Cases agreeing | Families agreeing | Fast wall | Full wall | Full/fast | Full median pair CV | Fast median pair CV |',
             '|---|---|---:|---:|---:|---:|---:|---:|---:|']
    for r in results:
        full_wall = r['full_wall_seconds']
        full_label = f'{full_wall:.1f}s' if full_wall is not None else 'unavailable'
        speedup = f"{full_wall/r['fast_wall_seconds']:.2f}x" if full_wall is not None else 'unavailable'
        lines.append(f"| {r['revision']} | {r['phase']}/{r['profile']} | {r['case_agreements']}/{r['cases']} | {r['family_agreements']}/{r['families']} | {r['fast_wall_seconds']:.1f}s | {full_label} | {speedup} | {r['median_full_pair_cv']:.3f} | {r['median_fast_pair_cv']:.3f} |")
    lines += ['', 'Full wall provenance:']
    for r in results:
        lines.append(f"- {r['revision']} {r['phase']}/{r['profile']}: {r['full_wall_provenance']}.")
    lines += ['', '## Every disagreement', '']
    if not case_disagreements and not family_disagreements:
        lines.append('None in the compared records.')
    for r in results:
        for d in r['disagreements']:
            lines.append(f"- {r['revision']} {r['phase']}/{r['profile']} {d['case']}: fast {d['fast_ratio']:.6f} ({d['fast_pairs']} pairs, {'PASS' if d['fast_case_pass'] else 'FAIL'}); full {d['full_ratio']:.6f} ({d['full_pairs']} pairs, {'PASS' if d['full_case_pass'] else 'FAIL'}); full IQR covers bar: {d['full_noise_covers_bar']}.")
            if d['fast_throughput_bytes_second'] is not None:
                lines.append(f"  Registered floor {d['registered_minimum_bytes_second']} B/s; fast {d['fast_throughput_bytes_second']:.3f} B/s, full {d['full_throughput_bytes_second']:.3f} B/s.")
        for family in r['family_disagreements']:
            lines.append(f"- Family: {r['revision']} {r['phase']}/{r['profile']} {family}: fast {json.dumps(r['family_measurements'][family]['fast'], sort_keys=True)}; full {json.dumps(r['family_measurements'][family]['full'], sort_keys=True)}.")
    lines += ['', '## Parallel versus single-core noise', '']
    if noise_pairs:
        for pair in noise_pairs:
            lines.append(f"- {pair['cases']} matching smoke cases: median paired-ratio CV {pair['single_cv']:.3f} single-core versus {pair['parallel_cv']:.3f} parallel. Records: `{pair['single']}` and `{pair['parallel']}`. Smoke is not a qualification.")
    else:
        lines.append('No matching smoke pair available.')
    lines += ['', '## Missing evidence and errors', '']
    lines += [f'- {item}' for item in missing]
    lines += [f'- {item}' for item in errors]
    if not missing and not errors:
        lines.append('None.')
    lines += ['', 'Accepted only with all required same-source comparisons, zero family disagreements, case disagreements within the full record\'s own noise, reported full/fast wall times, and a same-source single-core/parallel noise comparison. The registered bars remain unchanged.', '']
    (ROOT / 'parity/FAST_QUALIFY_VALIDATION.md').write_text('\n'.join(lines))
    write(ART / 'validation-results.json', {'accepted': accepted, 'comparisons': results,
                                           'missing': missing, 'errors': errors,
                                           'noise': noise_pairs})
    if accepted:
        write(ART / 'validation-receipt.json',
              {'accepted': True, 'harness_sha256': sha(ROOT / 'parity/fast_qualify.py'),
               'statistics_sha256': sha(ROOT / 'parity/fast_stats.py'),
               'validator_sha256': sha(ROOT / 'parity/fast_validate.py'),
               'maximum_pairs': next(iter(pair_limits)),
               'validated_records_sha256': [r['fast_sha256'] for r in results]})
    else:
        (ART / 'validation-receipt.json').unlink(missing_ok=True)
    print('ACCEPTED' if accepted else 'NOT RELEASE-GRADE')
    raise SystemExit(0 if accepted else 1)


if __name__ == '__main__':
    main()
