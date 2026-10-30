"""Write the seed corpora under $FUZZ_SCRATCH/seeds/<target>.

  python3 seeds.py SEEDGEN_BIN
Differential targets start from the 174 meshes in seeds/diff (cropped real and
synthetic meshes in the harness input format, taken from the 0.2.15 fuzz
corpus), `bounds_diff` also from enlarged copies. Codec targets start from vertex
and index-sequence streams produced by the crate's encoders (seedgen). Repair
and parallel_clod seeds are small hand-built meshes in the targets' formats.
"""
import os
from pathlib import Path
import random
import shutil
import struct
import subprocess
import sys

S = Path(os.environ['FUZZ_SCRATCH'])
HERE = Path(__file__).resolve().parent
OUT = S / 'seeds'
DIFF = ['simplify', 'simplify_extra', 'meshlets', 'clod', 'remap', 'lod_chain', 'bounds_diff']


def write(target, name, data):
    d = OUT / target
    d.mkdir(parents=True, exist_ok=True)
    (d / name).write_bytes(bytes(data))


def diff():
    for f in sorted((HERE / 'seeds/diff').iterdir()):
        raw = f.read_bytes()
        for t in DIFF:
            write(t, f.name, raw)
        # remap: record size (q[8] // 3 % 8) and stride (q[8] % 3).
        if '-16-' in f.name or '-256-' in f.name:
            for q8 in range(0, 24, 3):
                for stride_class in (0, 1, 2):
                    q = bytearray(raw)
                    q[8] = q8 + stride_class
                    write('remap', f'{f.name}-size{q8 // 3}-s{stride_class}', q)
        # simplify: update family (q[7] = 3) with the options that reach the
        # sparse path (SPARSE = 2), at several targets and error limits.
        if '-16-' in f.name or '-64-' in f.name:
            for options in (2, 10, 34, 66, 3):
                for target, error in ((8, 5), (30, 0), (0, 5)):
                    q = bytearray(raw)
                    q[7], q[9], q[10], q[11], q[12], q[13] = 3, options, 0, target, 0, error
                    write('simplify', f'{f.name}-sparse{options}-t{target}-e{error}', q)
        # Enlarged bounds parameters: cluster index count q[8]*5, meshlet
        # vertices q[9], meshlet triangles q[10].
        for k, (a, b, c) in enumerate([(200, 100, 100), (60, 30, 20), (230, 250, 255)]):
            q = bytearray(raw)
            q[8], q[9], q[10] = a, b, c
            write('bounds_diff', f'{f.name}-big{k}', q)


def grid_mesh(w, h):
    """Indices of a w x h vertex grid, two triangles per cell."""
    ix = []
    for y in range(h - 1):
        for x in range(w - 1):
            a, b, c, d = y * w + x, y * w + x + 1, (y + 1) * w + x, (y + 1) * w + x + 1
            ix += [a, b, c, b, d, c]
    return ix


def repair():
    rng = random.Random(20261030)
    # weld_vertices: byte 0 is both the tolerance (/255) and the low byte of x0.
    for tol in [0, 2, 20, 120, 255]:
        for spacing in [0.0, 0.004, 0.05, 1.0]:
            pts = []
            for i in range(64):
                base = [(i % 4) * spacing, ((i // 4) % 4) * spacing, (i // 16) * spacing]
                if i % 5 == 0:
                    base = pts[i // 2][:] if pts else base  # exact duplicate
                if i % 7 == 0:
                    base = [v + 0.002 for v in base]  # near duplicate
                pts.append(base)
            raw = bytearray(b''.join(struct.pack('<3f', *p) for p in pts))
            raw[0] = tol
            write('weld_vertices', f'lattice-{tol}-{spacing}', raw)
    nan = struct.pack('<3f', float('nan'), 0.0, 1.0) + struct.pack('<3f', float('inf'), 1.0, 1.0) * 2
    write('weld_vertices', 'nonfinite', bytes([30]) + nan * 4)
    # orient_faces: indices are bytes % 32.
    quad = grid_mesh(5, 5)
    write('orient_faces', 'grid-consistent', quad)
    flipped = quad[:]
    for f in range(0, len(flipped) // 3, 3):
        flipped[f * 3 + 1], flipped[f * 3 + 2] = flipped[f * 3 + 2], flipped[f * 3 + 1]
    write('orient_faces', 'grid-flipped', flipped)
    write('orient_faces', 'tetra-flipped', [0, 1, 2, 0, 3, 1, 1, 3, 2, 0, 2, 3])
    write('orient_faces', 'tetra', [0, 2, 1, 0, 1, 3, 1, 2, 3, 0, 3, 2])
    write('orient_faces', 'octa-mixed', [0, 1, 2, 0, 2, 3, 0, 4, 3, 0, 1, 4, 5, 2, 1, 5, 3, 2, 5, 4, 3, 5, 1, 4])
    write('orient_faces', 'non-manifold', [0, 1, 2, 0, 1, 3, 0, 1, 4, 0, 1, 5, 6, 7, 8, 6, 7, 8])
    write('orient_faces', 'two-components', [0, 1, 2, 2, 1, 3, 10, 11, 12, 12, 11, 13, 20, 21, 22])
    for k in range(6):
        write('orient_faces', f'random-fan-{k}', [rng.randrange(0, 12) for _ in range(90)])
    # fill_holes: byte b is index b % 32 and coordinate b, so adding 32 * j to
    # an index byte keeps the mesh and moves the vertex.
    holes = {
        'tetra-open': [0, 1, 2, 0, 3, 1, 1, 3, 2],
        'octa-open': [0, 1, 2, 0, 2, 3, 0, 4, 3, 0, 1, 4, 5, 2, 1, 5, 3, 2, 5, 4, 3],
        'grid-hole': [i for t, i in zip(range(10 ** 6), grid_mesh(4, 4)) if not 24 <= t < 30],
        'grid-two-holes': [i for t, i in zip(range(10 ** 6), grid_mesh(5, 5)) if not (30 <= t < 36 or 60 <= t < 66)],
        'ring': [x for i in range(8) for x in (i, 8 + i, 8 + (i + 1) % 8, i, 8 + (i + 1) % 8, (i + 1) % 8)],
    }
    for name, ix in holes.items():
        for k in range(4):
            write('fill_holes', f'{name}-{k}', [b + 32 * rng.randrange(0, 7) for b in ix])
    write('fill_holes', 'plain', [b for ix in holes.values() for b in ix][:96])


def mode_variants():
    """The repair targets read a trailing mode byte (policy, limits, buffer size)."""
    for target in ('weld_vertices', 'orient_faces', 'fill_holes'):
        for f in sorted((OUT / target).iterdir()):
            raw = f.read_bytes()
            for m in (0x00, 0x01, 0x12, 0x13, 0xF3):
                write(target, f'{f.name}-m{m:02x}', raw + bytes([m]))


def parallel():
    rng = random.Random(20261031)
    # data: [n, flag, cluster, partition, flags, limit bytes, fail] then bytes
    # read both as little-endian f32 positions and as indices (% n).
    for k in range(48):
        n = rng.choice([8, 16, 24, 32, 48])
        head = [n - 1, rng.randrange(256) & ~1, rng.randrange(256), rng.randrange(256),
                rng.randrange(256), 0, 0, 0]
        body = []
        for _ in range(192):
            body.append(rng.randrange(256))
        # Every fourth byte is an exponent byte of a small float.
        for i in range(3, len(body), 4):
            body[i] = rng.choice([0x3E, 0x3F, 0x40, 0x41])
        write('parallel_clod', f'rand-{k}', head + body)
    n = 25
    grid = grid_mesh(5, 5)
    body = []
    for i in range(0, len(grid), 1):
        body.append(grid[i])
    write('parallel_clod', 'grid-ids', [n - 1, 0, 8, 1, 0, 0, 0, 0] + body)


def codec(seedgen):
    tmp = S / 'seedgen-out'
    if tmp.exists():
        shutil.rmtree(tmp)
    subprocess.run([seedgen, str(tmp)], check=True)
    for target in ['vertex', 'vertex_into', 'simd', 'sequence_stream']:
        d = OUT / target
        d.mkdir(parents=True, exist_ok=True)
        for f in sorted((tmp / target).iterdir()):
            shutil.copy(f, d / f.name)


if __name__ == '__main__':
    diff()
    repair()
    mode_variants()
    parallel()
    codec(sys.argv[1])
    for d in sorted(OUT.iterdir()):
        print(d.name, len(list(d.iterdir())))
