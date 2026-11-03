"""Write the 0.2.17 fuzz seed corpora under fuzz/seeds/<target>.

  python3 fuzz/seeds.py SEEDGEN_BIN

SEEDGEN_BIN is the release binary of fuzz/seedgen (valid encoded streams made with the crate's
own encoders, for the codec targets). Everything else is deterministic: small closed and open
meshes in the r0217 mesh layout (flags, vertex count - 1, triangle count, 3 bytes per vertex,
3 index bytes per triangle, then parameter bytes), the cluster-LOD DAG fixtures, and random
bytes for targets with free-form layouts.
"""
from pathlib import Path
import random
import shutil
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
OUT = HERE / 'seeds'
MESH_TARGETS = ['hidden', 'character', 'process', 'width_equiv', 'simplify_meter']


def write(target, name, data):
    d = OUT / target
    d.mkdir(parents=True, exist_ok=True)
    (d / name).write_bytes(bytes(data))


def q(v):
    return max(0, min(252, round(v * 16) + 128))


def pack(verts, tris, flags=0):
    out = [flags, len(verts) - 1, len(tris)]
    for v in verts:
        out += [q(c) for c in v]
    for t in tris:
        out += list(t)
    return out


def grid(w, h, z=0.0):
    verts = [(x, y, z * ((x + y) % 2)) for y in range(h) for x in range(w)]
    tris = []
    for y in range(h - 1):
        for x in range(w - 1):
            a, b, c, d = y * w + x, y * w + x + 1, (y + 1) * w + x, (y + 1) * w + x + 1
            tris += [(a, b, c), (b, d, c)]
    return verts, tris


def meshes():
    tetra = ([(0, 0, 0), (2, 0, 0), (0, 2, 0), (0, 0, 2)], [(0, 2, 1), (0, 1, 3), (1, 2, 3), (0, 3, 2)])
    cube_v = [(x, y, z) for z in (0, 2) for y in (0, 2) for x in (0, 2)]
    cube_t = [(0, 2, 3), (0, 3, 1), (4, 5, 7), (4, 7, 6), (0, 1, 5), (0, 5, 4),
              (2, 6, 7), (2, 7, 3), (0, 4, 6), (0, 6, 2), (1, 3, 7), (1, 7, 5)]
    # Cube with a smaller cube inside (fully occluded) and an open-top box.
    inner_v = [(x * 0.5 + 0.75, y * 0.5 + 0.75, z * 0.5 + 0.75) for x, y, z in [(a / 2, b / 2, c / 2) for a, b, c in cube_v]]
    nested = (cube_v + inner_v, cube_t + [tuple(i + 8 for i in t) for t in cube_t])
    open_box = (cube_v, cube_t[:8] + cube_t[10:])
    # Duplicate and back-to-back triangles.
    dup = (cube_v, cube_t + cube_t[:4] + [(c, b, a) for a, b, c in cube_t[:4]])
    # Lat-long sphere.
    sv, st = [(0, 0, 3)], []
    rings, seg = 4, 8
    import math
    for r in range(1, rings + 1):
        for s in range(seg):
            a, p = 2 * math.pi * s / seg, math.pi * r / (rings + 1)
            sv.append((3 * math.sin(p) * math.cos(a), 3 * math.sin(p) * math.sin(a), 3 * math.cos(p)))
    sv.append((0, 0, -3))
    for s in range(seg):
        st.append((0, 1 + s, 1 + (s + 1) % seg))
    for r in range(rings - 1):
        for s in range(seg):
            a, b = 1 + r * seg + s, 1 + r * seg + (s + 1) % seg
            st += [(a, a + seg, b), (b, a + seg, b + seg)]
    last = 1 + (rings - 1) * seg
    for s in range(seg):
        st.append((len(sv) - 1, last + (s + 1) % seg, last + s))
    return {
        'tetra': tetra, 'cube': (cube_v, cube_t), 'nested': nested, 'open-box': open_box,
        'dup': dup, 'sphere': (sv, st), 'grid4': grid(4, 4), 'grid8': grid(8, 8, 0.5),
        'grid12': grid(12, 12, 1.0), 'bad-index': (cube_v, cube_t[:6] + [(0, 1, 9), (3, 3, 3)]),
    }


def mesh_targets():
    rng = random.Random(20261102)
    for name, (v, t) in meshes().items():
        flags = 0x40 if name == 'bad-index' else 0
        base = pack(v, t, flags)
        for k in range(3):
            params = [rng.randrange(256) for _ in range(160 + 48 * len(v))]
            for target in MESH_TARGETS:
                write(target, f'{name}-{k}', base + params)
    # Non-finite coordinates, then a few small random meshes.
    flags_bad = [0x80, 5, 3] + [255, 128, 128] * 6 + [0, 1, 2, 3, 4, 5, 1, 2, 3]
    for target in MESH_TARGETS:
        write(target, 'nonfinite', flags_bad + [rng.randrange(256) for _ in range(80)])
        for k in range(4):
            write(target, f'random-{k}', [rng.randrange(256) for _ in range(400)])


def free_form():
    rng = random.Random(20261103)
    for k in range(24):
        write('lod_quality', f'rand-{k}', [rng.randrange(256) for _ in range(rng.choice([96, 200, 384]))])
        write('repair4', f'rand-{k}', [rng.randrange(256) for _ in range(rng.choice([24, 72, 96]))])
        write('build_meshlets_spatial', f'rand-{k}',
              [rng.choice([16, 40, 90]), rng.choice([20, 60, 120]), 0, 0, 0, 0, 0] + [rng.randrange(256) for _ in range(500)])
    for k in range(16):
        head = [2 * rng.randrange(128)] + [rng.randrange(256) for _ in range(14)]
        write('parallel_equiv', f'vertex-{k}', head + [rng.randrange(256) for _ in range(40)])
        write('parallel_equiv', f'spatial-{k}', [2 * rng.randrange(128) + 1] + [rng.randrange(256) for _ in range(14)])
    # parallel_clod: n - 1, flag, cluster, partition, flags, limits..., then bytes read as floats and ids.
    for k in range(32):
        n = rng.choice([8, 16, 24, 32, 48])
        body = [rng.randrange(256) for _ in range(192)]
        for i in range(3, len(body), 4):
            body[i] = rng.choice([0x3E, 0x3F, 0x40, 0x41])
        write('parallel_clod', f'rand-{k}', [n - 1, rng.randrange(256) & ~1, rng.randrange(256), rng.randrange(256), rng.randrange(256), 0, 0, 0] + body)
    for f in sorted((HERE / 'seeds/dag_format').iterdir()):
        write('dag_reader', f.name, f.read_bytes())


def codec(seedgen):
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run([seedgen, tmp], check=True)
        for target in ['vertex', 'vertex_into', 'simd', 'sequence_stream', 'buffer_stream', 'vertex_encode', 'vertex_encode_into']:
            d = OUT / target
            d.mkdir(parents=True, exist_ok=True)
            for f in sorted((Path(tmp) / target).iterdir()):
                shutil.copy(f, d / f.name)


if __name__ == '__main__':
    mesh_targets()
    free_form()
    codec(sys.argv[1])
    for d in sorted(OUT.iterdir()):
        print(d.name, len(list(d.iterdir())))
