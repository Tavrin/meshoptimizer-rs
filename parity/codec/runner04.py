#!/usr/bin/env python3
"""0.4 codec completion: fixtures, malformed inputs, seeded sweeps, cross-decoding
and executed WASM identity, plus the full 0.2 decoder regression.

Every case runs on scalar-strict C++ 1.3, native Rust and executed WASM Rust,
through both the allocating and caller-buffer APIs. Encoded bytes must match
exactly; encoder outputs are then decoded in both directions (Rust bytes by
C++, C++ bytes by Rust). C++ status -3 marks an input whose reference
behaviour is undefined; such cases are never compared, only retained as Rust
robustness cases with native/WASM identity.
"""
import argparse
import hashlib
import json
import math
import random
import struct
import subprocess
import zipfile
from measure import ROOT, REF, TARGET, ART, FLAGS, Driver, request, sha, cmd
import runner as r02

OLD_OPS = list(range(1, 8))
NEW_OPS = list(range(11, 26))
ENCODERS = {11, 12, 13, 14, 15, 16, 17, 19}
NATIVE04 = 563
JS04 = 19


def u32(b, i):
    return struct.unpack_from('<I', b, i)[0]


def sources():
    s = r02.sources()
    for p in [REF / 'js/meshopt_encoder.test.js', REF / 'js/meshopt_encoder.js', REF / 'js/meshopt_decoder.mjs']:
        s[str(p)] = sha(p)
    return dict(sorted(s.items()))


def fixtures04():
    directory = ART / 'fixtures04'
    directory.mkdir(exist_ok=True)
    cmd(['node', ROOT / 'parity/codec/js-fixtures04.mjs', REF, directory])
    generated = ART / 'native-fixtures04.cpp'
    cmd(['python3', ROOT / 'parity/codec/native-fixtures04.py', REF, generated, directory])
    binary = TARGET / 'codec-native-fixtures04'
    cmd(['c++', *[f for f in FLAGS if f != '-DNDEBUG'], '-DMESHOPTIMIZER_NO_SIMD', '-I', REF / 'src', generated,
         *sorted((REF / 'src').glob('*.cpp')), '-o', binary])
    cmd([binary, directory])
    cases = []
    for p in sorted(directory.glob('*.input')):
        e = p.with_suffix('.expected')
        cases.append((p.stem, p.read_bytes(), e.read_bytes() if e.exists() else None))
    native = sum(n.startswith('native04-') for n, _, _ in cases)
    js = sum(n.startswith('js04-') for n, _, _ in cases)
    if native != NATIVE04 or js != JS04:
        raise ValueError(f'missing or extra pinned 0.4 fixtures: native {native}, js {js}')
    return cases


def rotation(a, b):
    """True when each triangle of b is a cyclic rotation of the same triangle of a."""
    return len(a) == len(b) and all(
        tuple(b[i:i + 3]) in [tuple(a[i:i + 3]), (a[i + 1], a[i + 2], a[i]), (a[i + 2], a[i], a[i + 1])]
        for i in range(0, len(a), 3))


def cross_requests(op, b, encoded):
    count, stride, version = u32(b, 8), u32(b, 12), u32(b, 24)
    if op == 11:
        return [request(1, count, stride, encoded)]
    if op in (12, 13):
        return [request(op - 10, count, 4, encoded)]
    if op in (14, 15, 16):
        return [request(op - 10, count, stride, encoded)]
    if op == 17:
        return [request(18, count, stride, encoded)]
    return [request(20, count, 4, encoded, version=version, level=3),
            request(21, count, 4, encoded, version=version, level=4)]


def cross(op, b, encoded, cpp, rust):
    """Rust-encoded bytes through C++ decoders and C++-encoded bytes through
    Rust decoders (identical bytes, both directions recorded)."""
    hashes = []
    for req in cross_requests(op, b, encoded):
        c = cpp.call(req)
        r = rust.call(req)
        if c[0] == -3 or (c[0] == 0) != (r[0] == 0) or (c[0] == 0 and c[1] != r[1]):
            raise AssertionError(('cross-decoding differs', op))
        data = b[44:]
        count = u32(b, 8)
        # Sequence codes drop bit 31 of zigzag deltas; it is lossless only when
        # every index is below 2^30 (vertex streams are always lossless).
        lossless = op == 11 or (op == 13 and all(v < 1 << 30 for v in struct.unpack(f'<{count}I', data)))
        if lossless and (c[0] != 0 or c[1] != data):
            raise AssertionError(('lossless round trip failed', op))
        # Index 0xffffffff is upstream's empty-FIFO sentinel: such triangle
        # lists are not lossless in C++ either (D61); bytes still match exactly.
        triangles = struct.unpack(f'<{count}I', data) if op == 12 else ()
        if op == 12 and 0xffffffff not in triangles and (c[0] != 0 or not rotation(triangles, struct.unpack(f'<{count}I', c[1]))):
            raise AssertionError('triangle round trip is not a rotation')
        if op == 19:
            vertices = data[:count * 4]
            tc = u32(b, 24)
            triangles = list(data[count * 4:])
            if c[0] != 0 or c[1][:count * 4] != vertices:
                raise AssertionError('meshlet vertex round trip failed')
            out = c[1][count * 4:]
            decoded = list(out) if u32(req, 4) == 20 else [v for t in struct.unpack(f'<{tc}I', out) for v in (t & 255, t >> 8 & 255, t >> 16 & 255)]
            if not rotation(triangles, decoded):
                raise AssertionError('meshlet triangle round trip is not a rotation')
        hashes.append({'request_sha256': hashlib.sha256(req).hexdigest(), 'status': c[0],
                       'output_sha256': hashlib.sha256(c[1]).hexdigest()})
    return hashes


def compare04(cases, binaries, label, seed=None):
    before = sources()
    identities = {k: sha(v) for k, v in binaries.items()}
    cpp = Driver(binaries['scalar'])
    simd = Driver(binaries['simd'])
    rust = Driver(binaries['rust'])
    wasm = r02.Wasm(binaries['wasm'])
    archive = ART / (label + '.zip')
    records = []
    counts = {}
    conformance = 0
    undefined = 0
    color_simd_max = 0
    with zipfile.ZipFile(archive, 'w', compression=zipfile.ZIP_DEFLATED) as z:
        for name, b, expected in cases:
            op = u32(b, 4)
            c = cpp.call(b)
            r = rust.call(b)
            w = wasm.call(b)
            oracle = c[0] != -3
            if r[:2] != w[:2] or (oracle and ((c[0] == 0) != (r[0] == 0) or (c[0] == 0 and c[1] != r[1]))):
                (ART / 'failure.input').write_bytes(b)
                raise AssertionError((name, c[0], r[0], w[0], next((i for i, (x, y) in enumerate(zip(c[1], r[1])) if x != y), None)))
            if expected is not None and (c[0] != 0 or c[1] != expected):
                raise AssertionError((name, 'C++ differs from the shipped JS encoder output'))
            undefined += not oracle
            counts[op] = counts.get(op, 0) + 1
            if op not in (8, 9, 22, 23, 24, 25):
                into = bytearray(b)
                struct.pack_into('<I', into, 16, u32(b, 16) | 128)
                ri = rust.call(into)
                wi = wasm.call(into)
                if not ri[:2] == r[:2] == wi[:2]:
                    raise AssertionError((name, 'caller-buffer API differs'))
            filt = op - 3 if op in (4, 5, 6) else u32(b, 20) if op == 7 else 0
            if c[0] == 0 and filt:
                # 0.2 SIMD conformance: Exp exact, Oct/Quat within one decoded unit.
                s = simd.call(b)
                stride = u32(b, 12)
                if s[0] != 0:
                    raise AssertionError(name)
                if filt == 3:
                    if s[1] != r[1]:
                        raise AssertionError(name)
                else:
                    fmt = 'b' if filt == 1 and stride == 4 else 'h'
                    a = struct.unpack(f'<{len(s[1]) // struct.calcsize(fmt)}{fmt}', s[1])
                    v = struct.unpack(f'<{len(r[1]) // struct.calcsize(fmt)}{fmt}', r[1])
                    if max([abs(x - y) for x, y in zip(a, v)] + [0]) > 1:
                        raise AssertionError(name)
                conformance += 1
            if c[0] == 0 and op == 18:
                # Color has no assumed cross-ISA identity; record the SIMD distance only.
                s = simd.call(b)
                fmt = 'B' if u32(b, 12) == 4 else 'H'
                a = struct.unpack(f'<{len(s[1]) // struct.calcsize(fmt)}{fmt}', s[1])
                v = struct.unpack(f'<{len(r[1]) // struct.calcsize(fmt)}{fmt}', r[1])
                color_simd_max = max([color_simd_max] + [abs(x - y) for x, y in zip(a, v)])
            crossed = cross(op, b, r[1], cpp, rust) if c[0] == 0 and op in ENCODERS else []
            files = {}
            parts = [('input', b), ('cpp', c[1]), ('rust', r[1]), ('wasm', w[1])]
            if expected is not None:
                parts.append(('expected', expected))
            for key, data in parts:
                member = name + '.' + key
                z.writestr(member, data)
                files[key] = {'member': member, 'sha256': hashlib.sha256(data).hexdigest()}
            records.append({'case': name, 'operation': op, 'cpp_status': c[0], 'rust_status': r[0], 'wasm_status': w[0],
                            'classification': 'compared' if oracle else 'cpp-undefined', 'cross_decoding': crossed,
                            'files': files})
            if len(records) % 2000 == 0:
                print(label, len(records), 'zero mismatches', flush=True)
    for d in [cpp, simd, rust]:
        d.close()
    wasm.close()
    if before != sources() or identities != {k: sha(v) for k, v in binaries.items()}:
        raise ValueError('sources or executables changed during the run')
    record = {'phase': '0.4', 'seed': seed, 'counts': counts, 'mismatches': 0, 'cpp_undefined_cases': undefined,
              'cross_decoded_cases': sum(1 for x in records if x['cross_decoding']),
              'simd_filter_conformance_cases': conformance, 'color_simd_max_unit_difference': color_simd_max,
              'sources': before, 'executable_sha256': identities, 'archive': str(archive), 'archive_sha256': sha(archive),
              'cases': records, 'node': subprocess.check_output(['node', '--version'], text=True).strip(),
              'rustc': subprocess.check_output(['rustc', '-Vv'], text=True),
              'cxx': subprocess.check_output(['c++', '--version'], text=True),
              'cxx_scalar_flags': FLAGS + ['-DMESHOPTIMIZER_NO_SIMD'], 'cxx_simd_flags': FLAGS,
              'rust_flags': 'empty; generic target; release; fat LTO; one codegen unit; no_std core',
              'reference_revision': '4c203430ca565cb59a468a91922c76c208169536'}
    (ART / (label + '.json')).write_text(json.dumps(record, indent=2))
    print(label, len(records), 'PASS', counts, flush=True)


# ---------------------------------------------------------------- malformed

def with_field(b, offset, value):
    out = bytearray(b)
    struct.pack_into('<I', out, offset, value)
    return bytes(out)


def with_payload(b, payload):
    return bytes(b[:40]) + struct.pack('<I', len(payload)) + payload


def malformed04(fixtures, cpp):
    out = []
    seen = set()
    for name, b, _ in fixtures:
        op = u32(b, 4)
        status, encoded, _ = cpp.call(b)
        key = (op, b[8:16], b[24:32], b[44:])
        if key in seen or status != 0:
            continue
        seen.add(key)
        payload = b[44:]
        if op in (20, 21):
            vc, tc = u32(b, 8), u32(b, 24)
            for n in range(len(payload)):
                out.append((f'{name}-prefix-{n}', with_payload(b, payload[:n]), None))
                out.append((f'{name}-suffix-{n}', with_payload(b, payload[len(payload) - n:]), None))
            for i in range(len(payload)):
                for mask in (0x01, 0x80, 0xff):
                    corrupt = bytearray(payload)
                    corrupt[i] ^= mask
                    out.append((f'{name}-flip-{i}-{mask}', with_payload(b, bytes(corrupt)), None))
            for dv, dt in [(-1, 0), (1, 0), (0, -1), (0, 1), (257 - vc, 0), (0, 257 - tc)]:
                if vc + dv >= 0 and tc + dt >= 0:
                    out.append((f'{name}-counts-{dv}-{dt}', with_field(with_field(b, 8, vc + dv), 24, tc + dt), None))
            out.append((f'{name}-extra-byte', with_payload(b, payload + b'\0'), None))
        if op in (11, 12, 13, 19):
            # Every capacity around the encoded size; the reference needs slack.
            size = len(encoded)
            capacities = range(size + 33) if size <= 640 else [*range(65), *range(size - 64, size + 33)]
            for capacity in capacities:
                out.append((f'{name}-capacity-{capacity}', with_field(b, 20, capacity + 1), None))
        if op == 18:
            stride = u32(b, 12)
            for i in range(len(payload) // stride):
                zero = bytearray(payload)
                zero[i * stride + (3 if stride == 4 else 6):(i + 1) * stride] = bytes(1 if stride == 4 else 2)
                out.append((f'{name}-zero-alpha-{i}', with_payload(b, bytes(zero)), None))
                small = bytearray(payload)
                if stride == 8:
                    small[i * 8:i * 8 + 8] = struct.pack('<4H', 65535, 32767, 32768, 1)
                    out.append((f'{name}-overflow-{i}', with_payload(b, bytes(small)), None))
            out.append((f'{name}-short', with_payload(b, payload[:-1]), None))
    out += invalid_parameters()
    return out


def invalid_parameters():
    """Parameters outside each reference assertion: both sides must reject."""
    v = bytes(range(64))
    floats = struct.pack('<16f', *[0.5] * 16)
    cases = [
        ('vertex-level-10', request(11, 4, 16, v, version=1, level=10)),
        ('vertex-version-2', request(11, 4, 16, v, version=2, level=2)),
        ('vertex-stride-6', request(11, 4, 6, v[:24], version=1, level=2)),
        ('vertex-stride-0', request(11, 0, 0, b'', version=1, level=2)),
        ('vertex-length', request(11, 4, 16, v[:63], version=1, level=2)),
        ('index-not-triangles', request(12, 4, 4, struct.pack('<4I', 0, 1, 2, 3), version=1)),
        ('index-version-2', request(12, 3, 4, struct.pack('<3I', 0, 1, 2), version=2)),
        ('sequence-version-2', request(13, 3, 4, struct.pack('<3I', 0, 1, 2), version=2)),
        ('sequence-length', request(13, 3, 4, struct.pack('<2I', 0, 1), version=1)),
        ('oct-bits-1', request(14, 4, 8, floats, level=1)),
        ('oct-bits-17', request(14, 4, 8, floats, level=17)),
        ('oct-stride4-bits-9', request(14, 4, 4, floats, level=9)),
        ('oct-stride-12', request(14, 4, 12, floats, level=8)),
        ('quat-bits-3', request(15, 4, 8, floats, level=3)),
        ('quat-stride-4', request(15, 4, 4, floats, level=8)),
        ('exp-bits-0', request(16, 4, 16, floats, level=0)),
        ('exp-bits-25', request(16, 4, 16, floats, level=25)),
        ('exp-mode-4', request(16, 4, 16, floats, mode=4, level=8)),
        ('exp-stride-6', request(16, 4, 6, floats[:24], level=8)),
        ('exp-nan', request(16, 1, 4, struct.pack('<f', math.nan), level=8)),
        ('exp-inf', request(16, 1, 8, struct.pack('<2f', 1.0, math.inf), level=8, mode=2)),
        ('exp-bits1-2^127', request(16, 1, 8, struct.pack('<2f', 0.0, 2.0 ** 127), level=1, mode=1)),
        ('color-bits-9-stride-4', request(17, 4, 4, floats, level=9)),
        ('color-stride-12', request(17, 4, 12, floats, level=8)),
        ('color-decode-stride-12', request(18, 1, 12, bytes(12))),
        ('color-decode-zero', request(18, 1, 4, bytes(4))),
        ('meshlet-vertices-257', request(19, 257, 0, bytes(257 * 4), version=0)),
        ('meshlet-triangles-257', request(19, 0, 0, bytes(257 * 3), version=257)),
        ('meshlet-length', request(19, 1, 0, bytes(5), version=1)),
        ('meshlet-decode-vs-3', request(20, 1, 3, bytes(32), version=1, level=3)),
        ('meshlet-decode-ts-2', request(20, 1, 4, bytes(32), version=1, level=2)),
        ('meshlet-decode-257', request(20, 257, 4, bytes(400), version=1, level=3)),
        ('meshlet-raw-257', request(21, 1, 4, bytes(400), version=257, level=4)),
        ('bound-vertex-stride-6', request(22, 10, 6, b'')),
        ('bound-vertex-stride-0', request(22, 10, 0, b'')),
        ('bound-index-not-triangles', request(23, 4, 4, struct.pack('<Q', 10))),
        # Delta exactly 2^31: the reference's baseline negation is undefined.
        ('sequence-int-min-delta', request(13, 3, 4, struct.pack('<3I', 0x80000000, 0, 0x80000000), version=1)),
    ]
    return [('invalid-' + n, b, None) for n, b in cases]


# ---------------------------------------------------------------- generators

def block_size(stride):
    return min(256, (8192 // stride) & ~15)


def vertex_stream(rng, count, stride, kind):
    """Random, constant and real-like attribute streams."""
    if kind == 0:
        return rng.randbytes(count * stride)
    if kind == 1:
        return bytes(count * stride)
    if kind == 2:
        return rng.randbytes(stride) * count
    words = stride // 4
    types = [rng.randrange(6) for _ in range(words)]
    phase = [rng.uniform(0, 6.283) for _ in range(words)]
    out = bytearray()
    for i in range(count):
        t = i / max(count, 1)
        for w, kind_w in enumerate(types):
            if kind_w == 0:  # smooth float position/UV
                out += struct.pack('<f', math.sin(t * 17 + phase[w]) * 3.5 + i * 0.001)
            elif kind_w == 1:  # quantized u16 pair
                out += struct.pack('<2H', int((math.sin(t * 9 + phase[w]) + 1) * 30000) & 0xffff, (i * 7) & 0xffff)
            elif kind_w == 2:  # 8-bit normal-like
                out += struct.pack('<4b', int(math.cos(t * 13 + phase[w]) * 127), int(math.sin(t * 13) * 127), 127, 0)
            elif kind_w == 3:  # counters
                out += struct.pack('<I', (i * 3 + w) & 0xffffffff)
            elif kind_w == 4:  # sparse changes
                out += struct.pack('<I', (i // 37) * 0x01010101 & 0xffffffff)
            else:
                out += rng.randbytes(4)
    return bytes(out)


def grid_triangles(rng, width, height):
    tris = []
    for y in range(height):
        for x in range(width):
            a, b = y * (width + 1) + x, y * (width + 1) + x + 1
            c, d = a + width + 1, b + width + 1
            tris += [a, b, c, b, d, c]
    return tris


def index_triangles(rng, kind, triangles):
    if kind == 0:
        v = rng.choice([3, 16, 300, 70000])
        return [rng.randrange(v) for _ in range(triangles * 3)]
    if kind == 1:
        width = rng.randint(1, 40)
        t = grid_triangles(rng, width, triangles // (2 * width) + 1)[:triangles * 3]
        # Local corner rotations and swaps keep a cache-friendly order.
        for i in range(0, len(t), 3):
            if rng.random() < 0.3:
                t[i:i + 3] = [t[i + 1], t[i + 2], t[i]]
        return t
    if kind == 2:
        t = []
        n = 0
        for i in range(triangles):
            if rng.random() < 0.02:
                t += [0, 1, 2]
                n = 3
            else:
                t += [n, n + 1, n + 2] if rng.random() < 0.5 else [n - 1 if n else 0, n, n + 1]
                n += rng.choice([1, 1, 2, 3])
        return t
    if kind == 3:
        return [rng.choice([rng.randrange(1 << 32), 0xffffffff - rng.randrange(64), rng.randrange(64)]) for _ in range(triangles * 3)]
    if kind == 4:
        base = [rng.randrange(20) for _ in range(9)]
        return [base[rng.randrange(9)] for _ in range(triangles * 3)]
    t = grid_triangles(rng, 16, triangles // 32 + 1)[:triangles * 3]
    order = list(range(len(t) // 3))
    for i in range(len(order) - 1):
        if rng.random() < 0.2:
            j = min(len(order) - 1, i + rng.randrange(1, 8))
            order[i], order[j] = order[j], order[i]
    return [t[3 * o + k] for o in order for k in range(3)]


def sequence_values(rng, kind, count):
    if kind == 0:
        return [rng.randrange(rng.choice([2, 256, 70000, 1 << 32])) for _ in range(count)]
    if kind == 1:
        start = rng.randrange(1 << 20)
        return [start + i for i in range(count)]
    if kind == 2:
        out, v = [], rng.randrange(1000)
        for _ in range(count):
            v = max(0, v + rng.randint(-40, 40))
            out.append(v)
        return out
    if kind == 3:
        a, b = rng.randrange(100), rng.randrange(1 << 24)
        return [(a + i // 2) if i % 2 == 0 else (b + i // 2) for i in range(count)]
    if kind == 4:
        return [(0xffffffff - i * rng.randint(1, 1000)) & 0xffffffff for i in range(count)]
    return [rng.choice([0, 1, 29, 30, 31, 0x7fffffff, 0x80000001, 0xffffffff, rng.randrange(1 << 32)]) for _ in range(count)]


def sequence_defined(values):
    """Nudge any index whose baseline delta is exactly 2^31 (undefined in the
    reference's negation); such inputs are covered separately as robustness cases."""
    last, current = [0, 0], 0
    out = []
    for v in values:
        if (v - last[current]) & 0xffffffff == 0x80000000:
            v = (v + 1) & 0xffffffff
        d = (v - last[current]) & 0xffffffff
        cd = d - (1 << 32) if d >= 1 << 31 else d
        current ^= abs(cd) >= 30
        last[current] = v
        out.append(v)
    return out


SPECIAL = [0.0, -0.0, 1.0, -1.0, math.nan, math.inf, -math.inf, 1e-40, -1e-40, 3.0e38, 0.5]


def unit(rng, n):
    v = [rng.gauss(0, 1) for _ in range(n)]
    length = math.sqrt(sum(x * x for x in v)) or 1.0
    return [x / length for x in v]


def vector_values(rng, op, kind):
    if op == 14:
        if kind == 0:
            return unit(rng, 3) + [rng.choice([-1.0, 1.0, rng.uniform(-1, 1)])]
        if kind == 1:
            return [rng.choice([0.0, 1.0, -1.0, -0.0]) for _ in range(3)] + [1.0]
        if kind == 2:
            return [rng.uniform(-5, 5) for _ in range(4)]
        return [rng.choice(SPECIAL) for _ in range(4)]
    if op == 15:
        if kind == 0:
            return unit(rng, 4)
        if kind == 1:
            v = [rng.choice([0.5, -0.5, 0.0]) for _ in range(4)]
            return v
        if kind == 2:
            return [rng.uniform(-2, 2) for _ in range(4)]
        return [rng.choice(SPECIAL) for _ in range(4)]
    # 17: Color
    if kind == 0:
        return [rng.random() for _ in range(4)]
    if kind == 1:
        return [rng.choice([0.0, 1.0, 0.5]) for _ in range(4)]
    if kind == 2:
        return [rng.uniform(-0.5, 1.5) for _ in range(4)]
    return [rng.choice(SPECIAL) for _ in range(4)]


def exp_values(rng, n, kind, bits):
    if kind == 0:
        return [rng.choice([-1, 1]) * 10 ** rng.uniform(-30, 30) for _ in range(n)]
    if kind == 1:
        return [rng.uniform(-1000, 1000) for _ in range(n)]
    if kind == 2:
        return [rng.choice([0.0, -0.0, 1e-42, 2.0 ** -126, 2.0 ** rng.randint(-149, 126), float(rng.randint(-8, 8))]) for _ in range(n)]
    if kind == 3:
        top = 2.0 ** 126 * rng.uniform(1, 1.99)
        return [rng.choice([top, -top, 0.0, 1.0]) if bits > 1 else rng.choice([2.0 ** 126, 0.0, -1.0]) for _ in range(n)]
    return [rng.choice(SPECIAL + [rng.uniform(-1, 1)]) for _ in range(n)]


def capacity_edge(cpp, b, rng):
    status, encoded, _ = cpp.call(b)
    if status != 0:
        return b
    size = len(encoded)
    return with_field(b, 20, max(0, rng.choice([size - 1, size, size + 1, size + 15, size + 23, size + 24, size + 32, rng.randrange(size + 40)])) + 1)


def meshlet_data(rng, i):
    vc = rng.choice([0, 1, 3, 4, 5, 63, 64, 128, 255, 256, rng.randrange(257)])
    tc = rng.choice([0, 1, 2, 3, 124, 126, 255, 256, rng.randrange(257)])
    kind = i % 4
    if kind == 0 and vc:
        # Real-like: a grid patch with local indices in first-appearance order.
        t = grid_triangles(rng, rng.randint(1, 8), 16)[:tc * 3]
        local = {}
        tri = []
        for v in t:
            local.setdefault(v, len(local))
            tri.append(local[v] % 256)
        tri += [rng.randrange(max(vc, 1)) for _ in range(tc * 3 - len(tri))]
    elif kind == 1:
        tri = [rng.randrange(256) for _ in range(tc * 3)]
    elif kind == 2:
        tri = [rng.randrange(max(vc, 1)) for _ in range(tc * 3)]
    else:
        tri = [rng.choice([0, 1, 2, 6]) for _ in range(tc * 3)]
    style = rng.randrange(4)
    if style == 0:
        start = rng.randrange(1 << 20)
        vertices = sorted(start + rng.randrange(4 * vc + 1) for _ in range(vc))
    elif style == 1:
        vertices = [rng.randrange(1 << 32) for _ in range(vc)]
    elif style == 2:
        vertices = [rng.randrange(1 << 16) for _ in range(vc)]
    else:
        vertices = [rng.choice([0, 0xffffffff, 0x01000000, 0xffffff, 0x10000]) for _ in range(vc)]
    return vc, tc, struct.pack(f'<{vc}I', *vertices) + bytes(tri)


def generated04(cpp, seed, cases):
    rng = random.Random(seed)
    for op in NEW_OPS:
        for i in range(cases):
            name = f'seed-{seed}-op{op}-{i}'
            if op == 11:
                stride = rng.choice([4, 8, 12, 16, 20, 24, 32, 36, 48, 64, 128, 252, 256])
                block = block_size(stride)
                count = rng.choice([0, 1, 2, 15, 16, 17, block - 1, block, block + 1, 2 * block - 1, 2 * block, 2 * block + 1,
                                    3 * block + 5, rng.randrange(1, 3 * block + 1)])
                count = min(count, 262144 // stride)
                b = request(11, count, stride, vertex_stream(rng, count, stride, i % 6), version=(i // 10) % 2, level=i % 10)
                if i % 7 == 3:
                    b = capacity_edge(cpp, b, rng)
            elif op in (12, 13):
                count = rng.choice([0, 1, 2, 3, 15, 16, 17, 255, 256, 257, rng.randrange(3000)])
                if op == 12:
                    values = index_triangles(rng, i % 6, count)
                else:
                    values = sequence_defined(sequence_values(rng, i % 6, count))
                b = request(op, len(values), 4, struct.pack(f'<{len(values)}I', *values), version=(i // 6) % 2)
                if i % 7 == 3:
                    b = capacity_edge(cpp, b, rng)
            elif op in (14, 15, 17):
                count = rng.choice([0, 1, 2, 3, 16, 17, 255, rng.randrange(1500)])
                stride = 8 if op == 15 else rng.choice([4, 8])
                low = 4 if op == 15 else 2
                bits = rng.randint(low, 8 if stride == 4 else 16)
                values = [x for _ in range(count) for x in vector_values(rng, op, (i // 4) % 4 if i % 4 else 0)]
                b = request(op, count, stride, struct.pack(f'<{len(values)}f', *values), level=bits)
            elif op == 16:
                stride = rng.choice([4, 8, 12, 16, 32, 64, 128, 256])
                count = min(rng.choice([0, 1, 2, 3, 17, 255, rng.randrange(1500)]), 65536 // stride)
                bits = rng.choice([1, 2, 8, 12, 15, 16, 23, 24, rng.randint(1, 24)])
                kind = (i // 4) % 5 if i % 9 else 4
                values = exp_values(rng, count * stride // 4, kind, bits)
                b = request(16, count, stride, struct.pack(f'<{len(values)}f', *values), mode=i % 4, level=bits)
            elif op == 18:
                stride = rng.choice([4, 8])
                count = rng.choice([0, 1, 2, 3, 16, 17, 255, rng.randrange(1500)])
                if i % 3:
                    values = [x for _ in range(count) for x in vector_values(rng, 17, rng.randrange(4))]
                    bits = rng.randint(2, 8 if stride == 4 else 16)
                    status, payload, _ = cpp.call(request(17, count, stride, struct.pack(f'<{len(values)}f', *values), level=bits))
                    assert status == 0
                else:
                    payload = rng.randbytes(count * stride)
                b = request(18, count, stride, payload)
            elif op == 19:
                vc, tc, payload = meshlet_data(rng, i)
                b = request(19, vc, 0, payload, version=tc)
                if i % 7 == 3:
                    b = capacity_edge(cpp, b, rng)
            elif op in (20, 21):
                vc, tc, payload = meshlet_data(rng, i)
                status, encoded, _ = cpp.call(request(19, vc, 0, payload, version=tc))
                assert status == 0
                if i % 5 == 4 and encoded:
                    corrupt = bytearray(encoded)
                    corrupt[rng.randrange(len(corrupt))] ^= 1 << rng.randrange(8)
                    encoded = bytes(corrupt)
                if op == 20:
                    b = request(20, vc, rng.choice([2, 4]), encoded, version=tc, level=rng.choice([3, 4]))
                else:
                    b = request(21, vc, 4, encoded, version=tc, level=4)
            elif op == 22:
                stride = rng.choice([4, 8, 12, 16, 64, 252, 256, 6])
                b = request(22, rng.randrange(min(1 << 25, (128 << 20) // stride)), stride, b'')
            elif op in (23, 24):
                count = rng.randrange(1 << 24)
                count -= count % 3 if op == 23 and i % 50 else 0
                vertices = rng.choice([0, 1, 2, 3, 127, 128, 129, 1 << 16, (1 << 16) + 1, rng.randrange(1 << 32), 0xffffffff])
                b = request(op, count, 4, struct.pack('<Q', vertices))
            else:
                b = request(25, rng.randrange(1 << 20), 0, b'', level=rng.randrange(1 << 20))
            yield name, b, None


def main():
    p = argparse.ArgumentParser()
    p.add_argument('action', choices=['run', 'sweep'])
    p.add_argument('--phase', default='0.4')
    p.add_argument('--profile', default='scalar-strict')
    p.add_argument('--cases-per-family', type=int, default=2000)
    p.add_argument('--seed', type=int, default=20261004)
    args = p.parse_args()
    if args.phase != '0.4' or args.profile != 'scalar-strict' or args.cases_per_family < 2000:
        raise ValueError('unsupported phase/profile/count')
    binaries = r02.build()
    if args.action == 'run':
        old = r02.fixtures()
        new = fixtures04()
        compare04([(n, b, None) for n, b in old] + new, binaries, 'fixtures')
        cpp = Driver(binaries['scalar'])
        malformed = [(n, b, None) for n, b in r02.malformed(old)] + malformed04(new, cpp)
        cpp.close()
        compare04(malformed, binaries, 'malformed')
    else:
        cpp = Driver(binaries['scalar'])

        def cases():
            for n, b in r02.generated(cpp, args.seed, args.cases_per_family):
                yield n, b, None
            yield from generated04(cpp, args.seed, args.cases_per_family)
        compare04(cases(), binaries, 'sweep', args.seed)
        cpp.close()


if __name__ == '__main__':
    main()
