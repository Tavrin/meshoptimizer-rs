#!/usr/bin/env python3
"""Seeded exact differential sweep against the pinned scalar C++ library."""
import ctypes as c
import base64
import gzip
import hashlib
import json
import os
import pathlib
import shutil
import struct
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
REF = pathlib.Path(os.environ.get('MESHOPT_REFERENCE', pathlib.Path.home() / 'dev/refs/meshoptimizer'))
ART = pathlib.Path(os.environ.get('MESHOPT_ARTIFACTS', '/mnt/linux-extra/meshopt-artifacts/p05'))
ART.mkdir(parents=True, exist_ok=True)
CPP = ART / 'libmeshopt-p05.so'
RUST = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', '/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p05')) / 'release/meshopt-p05-parity'
FAMILIES = ['stripify', 'stripify_bound', 'unstripify', 'unstripify_bound', 'vertex_cache', 'vertex_fetch', 'overdraw', 'coverage', 'omm_measure', 'omm_rasterize', 'omm_entry_size', 'omm_compact', 'tangents', 'normals', 'remesh']
U = c.c_uint; I = c.c_int; F = c.c_float; B = c.c_ubyte; Z = c.c_size_t
PU = c.POINTER(U); PI = c.POINTER(I); PF = c.POINTER(F); PB = c.POINTER(B)


def rust_identity():
    paths = [*ROOT.glob('src/**/*.rs'), *(ROOT / 'parity/p05/src').glob('*.rs'),
             ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', ROOT / 'parity/p05/Cargo.toml',
             ROOT / 'parity/p05/Cargo.lock', ROOT / 'parity/p05/sweep.py', ROOT / 'parity/wasm.cjs']
    return {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in sorted(paths)}


def build():
    source = REF / 'src'
    command = ['g++', '-std=c++11', '-shared', '-fPIC', '-O3', '-fno-fast-math', '-ffp-contract=off', '-DMESHOPTIMIZER_NO_SIMD', '-I', str(source), *map(str, sorted(source.glob('*.cpp'))), '-o', str(CPP)]
    subprocess.run(command, check=True)


class Cache(c.Structure):
    _fields_ = [('transformed', U), ('warps', U), ('acmr', F), ('atvr', F)]


class Fetch(c.Structure):
    _fields_ = [('bytes', U), ('overfetch', F)]


class Overdraw(c.Structure):
    _fields_ = [('covered', U), ('shaded', U), ('overdraw', F)]


class Coverage(c.Structure):
    _fields_ = [('coverage', F * 3), ('extent', F)]


def bind(lib, name, result, *args):
    function = getattr(lib, 'meshopt_' + name)
    function.restype = result
    function.argtypes = args
    return function


def api(lib):
    return {
        'strip': bind(lib, 'stripify', Z, PU, PU, Z, Z, U),
        'strip_bound': bind(lib, 'stripifyBound', Z, Z),
        'unstrip': bind(lib, 'unstripify', Z, PU, PU, Z, U),
        'unstrip_bound': bind(lib, 'unstripifyBound', Z, Z),
        'cache': bind(lib, 'analyzeVertexCache', Cache, PU, Z, Z, U, U, U),
        'fetch': bind(lib, 'analyzeVertexFetch', Fetch, PU, Z, Z, Z),
        'overdraw': bind(lib, 'analyzeOverdraw', Overdraw, PU, Z, PF, Z, Z),
        'coverage': bind(lib, 'analyzeCoverage', Coverage, PU, Z, PF, Z, Z),
        'measure': bind(lib, 'opacityMapMeasure', Z, PB, PU, PI, PU, Z, PF, Z, Z, U, U, I, F),
        'raster': bind(lib, 'opacityMapRasterize', None, PB, I, I, PF, PF, PF, PB, Z, Z, U, U),
        'entry': bind(lib, 'opacityMapEntrySize', Z, I, I),
        'compact': bind(lib, 'opacityMapCompact', Z, PB, Z, PB, PU, Z, PI, Z, I),
        'tangents': bind(lib, 'generateTangents', None, PF, PU, Z, PF, Z, Z, PF, Z, PF, Z, U),
        'normals': bind(lib, 'generateNormals', None, PF, PU, Z, PF, Z, Z, F, F),
        'remesh': bind(lib, 'remesh', Z, PF, Z, PU, Z, PF, Z, Z, I, U),
    }


class Rng:
    def __init__(self, seed): self.value = seed + 1
    def next(self):
        x = self.value
        x ^= (x << 13) & 0xffffffff
        x ^= x >> 17
        x ^= (x << 5) & 0xffffffff
        self.value = x & 0xffffffff
        return self.value


def fixture(seed, count_override=None, triangles_override=None, style=0):
    rng = Rng(seed)
    count = 3 + rng.next() % 6
    triangles = 1 + rng.next() % 5
    if count_override is not None: count = count_override
    if triangles_override is not None: triangles = triangles_override
    positions, normals, uvs = [], [], []
    for _ in range(count):
        positions.extend((rng.next() % 17 - 8) / 8 for _ in range(3))
        normals.extend((0., 0., 1.))
        uvs.extend((rng.next() % 17 - 4) / 8 for _ in range(2))
    indices = [rng.next() % count for _ in range(triangles * 3)]
    texture = bytes(rng.next() & 255 for _ in range(64))
    if style == 1:  # connected grid
        side = max(2, __import__('math').isqrt(count))
        for i in range(count):
            x, y = i % side, i // side
            positions[i * 3:i * 3 + 3] = [x / 16, y / 16, 0.]
            uvs[i * 2:i * 2 + 2] = [x / 256, y / 256]
        cells = (side - 1) * (side - 1)
        for t in range(triangles):
            cell = (t // 2) % cells
            a = (cell // (side - 1)) * side + cell % (side - 1)
            indices[t * 3:t * 3 + 3] = [a, a + 1, a + side] if t & 1 == 0 else [a + 1, a + side + 1, a + side]
    elif style == 2:  # repeated positions with independent vertex IDs
        base = max(3, count // 4)
        for i in range(base, count):
            j = i % base
            positions[i * 3:i * 3 + 3] = positions[j * 3:j * 3 + 3]
            uvs[i * 2:i * 2 + 2] = uvs[j * 2:j * 2 + 2]
    elif style == 3:  # half the records never referenced
        used = max(3, count // 2)
        indices = [(v % used) * 2 for v in indices]
    elif style == 4:  # independent triangles; no shared vertex IDs
        if count < triangles * 3:
            raise ValueError('disconnected fixture needs three vertices per triangle')
        indices = list(range(triangles * 3))
    elif style != 0:
        raise ValueError(style)
    packed = struct.pack('<II', count, len(indices)) + struct.pack('<%sf' % len(positions), *positions) + struct.pack('<%sf' % len(normals), *normals) + struct.pack('<%sf' % len(uvs), *uvs) + struct.pack('<%sI' % len(indices), *indices) + texture
    h = 14695981039346656037
    for byte in packed: h = ((h ^ byte) * 1099511628211) & 0xffffffffffffffff
    return count, positions, normals, uvs, indices, texture, h


def pack_float(values): return struct.pack('<%sf' % len(values), *values)
def pack_uint(values): return struct.pack('<%sI' % len(values), *values)


def expected(functions, family, seed, data):
    count, positions, normals, uvs, indices, texture, _ = data
    m = len(indices)
    pos = (F * len(positions))(*positions)
    norm = (F * len(normals))(*normals)
    uv = (F * len(uvs))(*uvs)
    ind = (U * m)(*indices)
    tex = (B * 64).from_buffer_copy(texture)
    restart = 0 if seed & 1 == 0 else 65535
    if family in ('stripify', 'unstripify', 'unstripify_bound'):
        strip = (U * functions['strip_bound'](m))()
        n = functions['strip'](strip, ind, m, count, restart)
        if family == 'stripify': return pack_uint(strip[:n])
        if family == 'unstripify_bound': return struct.pack('<Q', functions['unstrip_bound'](n))
        result = (U * functions['unstrip_bound'](n))()
        used = functions['unstrip'](result, strip, n, restart)
        return pack_uint(result[:used])
    if family == 'stripify_bound': return struct.pack('<Q', functions['strip_bound'](m))
    if family == 'vertex_cache':
        value = functions['cache'](ind, m, count, 3 + seed % 24, 0 if seed & 1 == 0 else 16, seed % 4)
        return struct.pack('<IIff', value.transformed, value.warps, value.acmr, value.atvr)
    if family == 'vertex_fetch':
        value = functions['fetch'](ind, m, count, 4 + seed % 64)
        return struct.pack('<If', value.bytes, value.overfetch)
    if family == 'overdraw':
        value = functions['overdraw'](ind, m, pos, count, 12)
        return struct.pack('<IIf', value.covered, value.shaded, value.overdraw)
    if family == 'coverage':
        value = functions['coverage'](ind, m, pos, count, 12)
        return pack_float([*value.coverage, value.extent])
    if family == 'omm_measure':
        levels = (B * (m // 3))(); sources = (U * (m // 3))(); omm = (I * (m // 3))()
        used = functions['measure'](levels, sources, omm, ind, m, uv, count, 8, 8, 8, seed % 5, 0. if seed & 1 == 0 else 2.)
        return struct.pack('<I', used) + bytes(levels[:used]) + pack_uint(sources[:used]) + struct.pack('<%si' % (m // 3), *omm)
    if family == 'omm_rasterize':
        level = seed % 4; states = 2 if seed & 1 == 0 else 4
        result = (B * functions['entry'](level, states))()
        a, b, d = [c.cast(c.byref(uv, 8 * indices[i]), PF) for i in range(3)]
        functions['raster'](result, level, states, a, b, d, tex, 1, 8, 8, 8)
        return bytes(result)
    if family == 'omm_entry_size': return struct.pack('<Q', functions['entry'](seed % 13, 2 if seed & 1 == 0 else 4))
    if family == 'omm_compact':
        states = 2 if seed & 1 == 0 else 4
        levels = (B * 4)(0, 1, 2, 3)
        chunks = [bytes(texture[(i * 13 + j) % 64] for j in range(functions['entry'](i, states))) for i in range(4)]
        offsets = (U * 4)()
        offset = 0
        for i, chunk in enumerate(chunks): offsets[i] = offset; offset += len(chunk)
        original = b''.join(chunks)
        blob = (B * len(original)).from_buffer_copy(original)
        omm = (I * 6)(0, 1, 2, 3, 0, 2)
        used = functions['compact'](blob, len(blob), levels, offsets, 4, omm, 6, states)
        size = offsets[used - 1] + functions['entry'](levels[used - 1], states) if used else 0
        return struct.pack('<QQ', used, size) + bytes(blob) + bytes(levels) + pack_uint(offsets) + struct.pack('<6i', *omm)
    if family == 'tangents':
        result = (F * (m * 4))()
        functions['tangents'](result, ind, m, pos, count, 12, norm, 12, uv, 8, seed % 4)
        return pack_float(result)
    if family == 'normals':
        result = (F * (m * 3))()
        functions['normals'](result, ind, m, pos, count, 12, [0.5, 1., 2., 3.][seed % 4], (seed % 4) * 0.5)
        return pack_float(result)
    if family == 'remesh':
        resolution = 4 + seed % 5; options = seed % 4
        bound = functions['remesh'](None, 0, ind, m, pos, count, 12, resolution, options)
        result = (F * (bound * 9))()
        used = functions['remesh'](result, bound, ind, m, pos, count, 12, resolution, options)
        return struct.pack('<QQ', bound, used) + pack_float(result[:used * 9])
    raise ValueError(family)


def main():
    if sys.argv[1:] == ['--wasm-identity']:
        wasm_identity()
        return
    no_default = sys.argv[1:] == ['--no-default-identity']
    if sys.argv[1:] and not no_default:
        raise SystemExit('usage: sweep.py [--wasm-identity|--no-default-identity]')
    label = 'no-default-0.5' if no_default else 'sweep-0.5'
    identity = rust_identity()
    command = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', 'parity/p05/Cargo.toml']
    if no_default:
        command.append('--no-default-features')
    subprocess.run(command, cwd=ROOT, check=True)
    build()
    functions = api(c.CDLL(str(CPP)))
    process = subprocess.Popen([str(RUST)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
    detail = ART / f'{label}.jsonl.gz'
    counts = {}
    mismatches = []
    with gzip.open(detail, 'wt', compresslevel=6) as log:
        for family in FAMILIES:
            for seed in range(2000):
                data = fixture(seed)
                process.stdin.write(f'{family} {seed}\n'); process.stdin.flush()
                line = process.stdout.readline().rstrip('\n')
                if not line: raise RuntimeError('Rust runner exited early')
                input_hash, actual = line.split(' ', 1)
                reference = expected(functions, family, seed, data).hex()
                equal = input_hash == f'{data[-1]:016x}' and actual == reference
                record = {'family': family, 'seed': seed, 'input_hash': input_hash, 'expected': reference, 'actual': actual, 'equal': equal}
                log.write(json.dumps(record, separators=(',', ':')) + '\n')
                if not equal and len(mismatches) < 20: mismatches.append({'family': family, 'seed': seed, 'expected': reference[:128], 'actual': actual[:128], 'input_ok': input_hash == f'{data[-1]:016x}'})
                counts[family] = counts.get(family, 0) + 1
            print(family, counts[family], 'mismatch', sum(m['family'] == family for m in mismatches), flush=True)
    process.stdin.close(); process.wait()
    if rust_identity() != identity:
        raise RuntimeError('source changed during differential sweep')
    shutil.copyfile(RUST, ART / f'{label}-rust')
    shutil.copyfile(CPP, ART / f'{label}-cpp.so')
    summary = {'schema': 'meshopt-p05-sweep/1', 'phase': '0.5', 'upstream': '4c203430ca565cb59a468a91922c76c208169536', 'seed_range': [0, 1999], 'counts': counts, 'mismatches': mismatches, 'passed': not mismatches and process.returncode == 0, 'detail_artifact': detail.name, 'detail_sha256': hashlib.sha256(detail.read_bytes()).hexdigest(), 'rust_sha256': hashlib.sha256(RUST.read_bytes()).hexdigest(), 'cpp_sha256': hashlib.sha256(CPP.read_bytes()).hexdigest()}
    summary['source_sha256'] = identity
    (ROOT / f'parity/results/{label}.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({'passed': summary['passed'], 'counts': counts, 'mismatches': mismatches}, indent=2))
    raise SystemExit(0 if summary['passed'] else 1)


def wasm_identity():
    source = ART / 'sweep-0.5.jsonl.gz'
    if not source.is_file(): raise SystemExit('run the native sweep first')
    identity = rust_identity()
    native = json.loads((ROOT / 'parity/results/sweep-0.5.json').read_text())
    if native.get('source_sha256') != identity:
        raise SystemExit('rebuild and run the native sweep before WASM identity')
    subprocess.run(['cargo', 'build', '--offline', '--locked', '--release', '--lib',
                    '--target', 'wasm32-unknown-unknown', '--manifest-path', 'parity/p05/Cargo.toml'],
                   cwd=ROOT, check=True)
    wasm = pathlib.Path(os.environ.get('CARGO_TARGET_DIR', '/mnt/linux-extra/moss-cargo-targets/codex-meshopt-p05')) / 'wasm32-unknown-unknown/release/meshopt_p05_parity.wasm'
    process = subprocess.Popen(['node', str(ROOT / 'parity/wasm.cjs'), str(wasm)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1)
    detail = ART / 'wasm-0.5.jsonl.gz'
    counts = {}
    mismatches = []
    with gzip.open(source, 'rt') as records, gzip.open(detail, 'wt', compresslevel=6) as log:
        for line in records:
            case = json.loads(line)
            family, seed = case['family'], case['seed']
            request = base64.b64encode(f'{family} {seed}'.encode()).decode()
            process.stdin.write(request + '\n'); process.stdin.flush()
            reply = process.stdout.readline().strip()
            if not reply: raise RuntimeError(f'WASM runner exited at {family} {seed}')
            result = base64.b64decode(reply)
            input_hash = int.from_bytes(result[:8], 'little')
            actual = result[8:].hex()
            equal = input_hash == int(case['input_hash'], 16) and actual == case['actual']
            log.write(json.dumps({'family': family, 'seed': seed, 'input_hash': f'{input_hash:016x}', 'output': actual, 'equal': equal}, separators=(',', ':')) + '\n')
            if not equal and len(mismatches) < 20: mismatches.append({'family': family, 'seed': seed, 'input_ok': input_hash == int(case['input_hash'], 16), 'expected': case['actual'][:128], 'wasm': actual[:128]})
            counts[family] = counts.get(family, 0) + 1
            if seed == 1999: print('wasm', family, counts[family], flush=True)
    process.stdin.close(); process.wait()
    if rust_identity() != identity:
        raise RuntimeError('source changed during WASM identity')
    shutil.copyfile(wasm, ART / 'wasm-0.5.wasm')
    summary = {'schema': 'meshopt-p05-wasm/1', 'phase': '0.5', 'counts': counts, 'mismatches': mismatches, 'passed': not mismatches and process.returncode == 0, 'detail_artifact': detail.name, 'detail_sha256': hashlib.sha256(detail.read_bytes()).hexdigest(), 'wasm_sha256': hashlib.sha256(wasm.read_bytes()).hexdigest()}
    summary['source_sha256'] = identity
    (ROOT / 'parity/results/wasm-0.5.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({'passed': summary['passed'], 'mismatches': mismatches}, indent=2))
    raise SystemExit(0 if summary['passed'] else 1)


if __name__ == '__main__': main()
