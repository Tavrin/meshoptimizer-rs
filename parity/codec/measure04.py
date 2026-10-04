#!/usr/bin/env python3
"""0.4 performance against the RFC 6.1 raw scalar codec bar, per function family.

Rust (built with an explicit consumer profile) is timed against scalar-strict
C++ 1.3 on identical inputs; optimized SIMD C++ is reported alongside. The bar
is unchanged: per family and API, geometric mean Rust/scalar time ratio <= 1.25
and no case > 1.50. Validation, required copies and output allocation are
timed on both sides; input generation, conversion and I/O are not.
"""
import argparse
import json
import math
import os
from pathlib import Path
import random
import statistics
import struct
import subprocess
import time
from measure import ROOT, TARGET, ART, ENV, FLAGS, Driver, request, sha, build_cpp, measure
from runner import sources as sources02
import runner04

PROFILES = {
    'moss': {'CARGO_PROFILE_RELEASE_LTO': 'thin', 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1',
             'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0'},
    'default': {'CARGO_PROFILE_RELEASE_LTO': 'false', 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '16',
                'CARGO_PROFILE_RELEASE_OPT_LEVEL': '3', 'CARGO_PROFILE_RELEASE_DEBUG': '0'},
}
FAMILIES = ['vertex_encode', 'index_encode', 'sequence_encode', 'oct_encode', 'quat_encode', 'exp_encode',
            'color_encode', 'color_decode', 'meshlet_encode', 'meshlet_decode', 'meshlet_decode_raw', 'bounds']


def build_rust(profile):
    directory = TARGET / ('bench-' + profile)
    env = dict(ENV, CARGO_TARGET_DIR=str(directory), **PROFILES[profile])
    subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
                    str(ROOT / 'parity/codec/Cargo.toml'), '--bin', 'codec-driver'], env=env, check=True)
    return directory / 'release/codec-driver'


def stream(count, stride):
    """Real-like attributes: smooth float positions, quantized pairs, normals, counters."""
    out = bytearray()
    for i in range(count):
        t = i / count
        for w in range(stride // 4):
            kind = w % 4
            if kind == 0:
                out += struct.pack('<f', math.sin(t * 40 + w) * 3.5)
            elif kind == 1:
                out += struct.pack('<2H', int((math.sin(t * 25) + 1) * 30000), (i * 7) & 0xffff)
            elif kind == 2:
                out += struct.pack('<4b', int(math.cos(t * 31) * 127), int(math.sin(t * 31) * 127), 127, 0)
            else:
                out += struct.pack('<I', i // 3)
    return bytes(out)


def grid(triangles):
    width = 256
    tris = runner04.grid_triangles(random.Random(1), width, triangles // (2 * width) + 1)
    return tris[:triangles * 3]


def u32s(values):
    return struct.pack(f'<{len(values)}I', *values)


def f32s(values):
    return struct.pack(f'<{len(values)}f', *values)


def corpus(cpp):
    rng = random.Random(20261004)
    cases = []

    def add(family, name, b):
        cases.append((family + '/' + name, b))
    for count, label in [(17, 'tiny'), (4097, 'resident'), (262145, 'streaming')]:
        for stride in ([12, 16, 32] if label == 'resident' else [16]):
            data = stream(count, stride)
            configs = [(0, 2), (1, 0), (1, 1), (1, 2), (1, 3)] if label == 'resident' else [(0, 2), (1, 2), (1, 3)] if label == 'streaming' else [(1, 2)]
            for version, level in configs:
                add('vertex_encode', f'{label}-s{stride}-v{version}-l{level}', request(11, count, stride, data, version=version, level=level))
    for count in [255, 257]:
        add('vertex_encode', f'boundary-{count}-s32-v1-l2', request(11, count, 32, stream(count, 32), version=1, level=2))
    for triangles, label in [(5, 'tiny'), (4096, 'resident'), (1048576, 'millions')]:
        tris = grid(triangles)
        for version in [0, 1]:
            add('index_encode', f'grid-{label}-v{version}', request(12, len(tris), 4, u32s(tris), version=version))
    add('index_encode', 'random-resident-v1', request(12, 4096 * 3, 4, u32s([rng.randrange(3000) for _ in range(4096 * 3)]), version=1))
    for name, values in [('tiny', [0, 1, 51, 2, 49, 1000]), ('sequential-65536', list(range(65536))),
                         ('random-4096', [rng.randrange(1 << 20) for _ in range(4096)]),
                         ('grid-millions', grid(1048576))]:
        for version in ([0, 1] if name == 'sequential-65536' else [1]):
            add('sequence_encode', f'{name}-v{version}', request(13, len(values), 4, u32s(values), version=version))
    for count, label in [(4097, 'resident'), (262145, 'streaming')]:
        normals = [x for i in range(count) for x in runner04.unit(rng, 3) + [1.0]]
        quats = [x for i in range(count) for x in runner04.unit(rng, 4)]
        colors = [rng.random() for _ in range(count * 4)]
        values = [rng.uniform(-100, 100) for _ in range(count * 3)]
        for stride, bits in [(4, 8), (8, 12)]:
            add('oct_encode', f'{label}-s{stride}-b{bits}', request(14, count, stride, f32s(normals), level=bits))
            add('color_encode', f'{label}-s{stride}-b{bits}', request(17, count, stride, f32s(colors), level=bits))
            status, encoded, _ = cpp.call(request(17, count, stride, f32s(colors), level=bits))
            assert status == 0
            add('color_decode', f'{label}-s{stride}-b{bits}', request(18, count, stride, encoded))
        add('quat_encode', f'{label}-s8-b12', request(15, count, 8, f32s(quats), level=12))
        for mode in ([0, 1, 2, 3] if label == 'resident' else [1]):
            add('exp_encode', f'{label}-s12-b15-mode{mode}', request(16, count, 12, f32s(values), mode=mode, level=15))
    meshlets = {}
    for name, vc, tc in [('small', 3, 1), ('typical', 64, 124), ('max', 256, 256)]:
        tris = grid(512)
        local = {}
        triangles = []
        for v in tris:
            if len(triangles) == tc * 3:
                break
            if v not in local and len(local) == vc:
                triangles = triangles[:len(triangles) - len(triangles) % 3]
                break
            local.setdefault(v, len(local))
            triangles.append(local[v])
        triangles = triangles[:len(triangles) // 3 * 3]
        triangles += [0, 1, 2] * (tc - len(triangles) // 3)
        vertices = sorted(rng.sample(range(1 << 20), vc))
        payload = u32s(vertices) + bytes(triangles)
        add('meshlet_encode', name, request(19, vc, 0, payload, version=tc))
        status, encoded, _ = cpp.call(request(19, vc, 0, payload, version=tc))
        assert status == 0
        meshlets[name] = (vc, tc, encoded)
    for name, (vc, tc, encoded) in meshlets.items():
        for vs, ts in ([(2, 3), (2, 4), (4, 3), (4, 4)] if name == 'typical' else [(4, 3)]):
            add('meshlet_decode', f'{name}-v{vs}-t{ts}', request(20, vc, vs, encoded, version=tc, level=ts))
        add('meshlet_decode_raw', name, request(21, vc, 4, encoded, version=tc, level=4))
    add('bounds', 'vertex', request(22, 1 << 20, 16, b''))
    add('bounds', 'index', request(23, 3 << 20, 4, struct.pack('<Q', 1 << 20)))
    add('bounds', 'sequence', request(24, 1 << 20, 4, struct.pack('<Q', 1 << 20)))
    add('bounds', 'meshlet', request(25, 64, 0, b'', level=124))
    return cases


def inputs():
    """Generate once per artifact directory; both profiles time identical bytes."""
    manifest_path = ART / 'benchmark04-inputs.json'
    if manifest_path.exists():
        cases = []
        for entry in json.loads(manifest_path.read_text()):
            if sha(entry['path']) != entry['sha256']:
                raise ValueError('benchmark input changed: ' + entry['case'])
            cases.append((entry['case'], Path(entry['path']).read_bytes()))
        return cases, sha(manifest_path)
    directory = ART / 'benchmark04-inputs'
    directory.mkdir(parents=True, exist_ok=True)
    cpp = Driver(build_cpp(True))
    cases = corpus(cpp)
    cpp.close()
    manifest = []
    for name, b in cases:
        p = directory / (name.replace('/', '--') + '.input')
        p.write_bytes(b)
        manifest.append({'case': name, 'path': str(p), 'sha256': sha(p)})
    manifest_path.write_text(json.dumps(manifest, indent=2))
    return cases, sha(manifest_path)


def verdict(rows, families):
    out = {}
    for family in families:
        ratios = [r['rust_scalar_time_ratio'] for r in rows if r['case'].startswith(family + '/')]
        if not ratios:
            raise ValueError('family without cases: ' + family)
        gm = statistics.geometric_mean(ratios)
        out[family] = {'cases': len(ratios), 'geometric_mean_rust_scalar_time_ratio': gm,
                       'maximum_rust_scalar_time_ratio': max(ratios), 'pass': gm <= 1.25 and max(ratios) <= 1.50}
    return out


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--phase', default='0.4')
    p.add_argument('--consumer-profile', choices=sorted(PROFILES), required=True)
    p.add_argument('--enforce', action='store_true')
    args = p.parse_args()
    if args.phase != '0.4':
        raise ValueError('unsupported phase')
    profile = args.consumer_profile
    record_path = ART / f'benchmark-0.4-{profile}.json'
    if record_path.exists():
        raise SystemExit('immutable record exists; select a fresh MESHOPT_ARTIFACTS')
    scalar, simd = build_cpp(True), build_cpp()
    rust = build_rust(profile)
    cases, manifest_sha = inputs()
    before = runner04.sources()
    binaries = {'rust': rust, 'scalar': scalar, 'simd': simd}
    identities = {k: sha(v) for k, v in binaries.items()}
    # Correctness of every timed output before timing: Rust bytes equal scalar C++.
    check = {k: Driver(v) for k, v in binaries.items() if k != 'simd'}
    for name, b in cases:
        for api in [0, 128]:
            req = bytearray(b)
            struct.pack_into('<I', req, 16, struct.unpack_from('<I', b, 16)[0] | api)
            c, r = check['scalar'].call(bytes(req)), check['rust'].call(bytes(req))
            if c[0] != 0 or r[:2] != c[:2]:
                raise AssertionError(('benchmark output differs', name, api))
    for d in check.values():
        d.close()
    started = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
    load_before = os.getloadavg()
    allocating = measure(cases, binaries, target_seconds=0.04, adaptive=True)
    caller = []
    into = []
    for name, b in cases:
        if name.startswith('bounds/'):
            continue
        req = bytearray(b)
        struct.pack_into('<I', req, 16, struct.unpack_from('<I', b, 16)[0] | 128)
        into.append((name, bytes(req)))
    caller = measure(into, binaries, target_seconds=0.04, adaptive=True)
    for rows in [allocating, caller]:
        for row in rows:
            t = row['statistics']
            row['rust_scalar_time_ratio'] = t['rust']['median'] / t['scalar']['median']
            row['rust_simd_time_ratio'] = t['rust']['median'] / t['simd']['median']
    # Bounds have one form; their allocating verdict stands for both APIs.
    verdicts = {'allocating': verdict(allocating, FAMILIES),
                'caller_buffer': verdict(caller, [f for f in FAMILIES if f != 'bounds'])}
    verdicts['caller_buffer']['bounds'] = verdicts['allocating']['bounds']
    families = {f: all(verdicts[api][f]['pass'] for api in verdicts) for f in FAMILIES}
    if before != runner04.sources() or identities != {k: sha(v) for k, v in binaries.items()}:
        raise ValueError('sources or executables changed during measurement')
    record = {'phase': '0.4', 'consumer_profile': profile, 'effective_profile_overrides': PROFILES[profile],
              'started_utc': started, 'finished_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
              'load_before': load_before, 'load_after': os.getloadavg(),
              'cpu': int(os.environ.get('MESHOPT_BENCH_CPU', min(os.sched_getaffinity(0)))),
              'bar': 'per family and API: geometric mean Rust/scalar-C++ time ratio <= 1.25, maximum <= 1.50 (RFC 6.1 raw scalar codecs)',
              'protocol': 'one warm-up; >= 12 paired samples targeting 40 ms, rotating backend order, extended to 60 when CV > 15% or the 1.50 gate is within the 95% interval',
              'input_manifest_sha256': manifest_sha, 'sources': before, 'executable_sha256': identities,
              'cxx_scalar_flags': FLAGS + ['-DMESHOPTIMIZER_NO_SIMD'], 'cxx_simd_flags': FLAGS,
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
              'allocating': allocating, 'caller_buffer': caller, 'verdicts': verdicts, 'family_pass': families}
    record_path.write_text(json.dumps(record, indent=2))
    print('VERDICT', profile, json.dumps(families), flush=True)
    if args.enforce and not all(families.values()):
        raise SystemExit(1)


if __name__ == '__main__':
    main()
