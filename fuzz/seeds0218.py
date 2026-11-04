"""Write the 0.2.18 fuzz seed corpora under fuzz/seeds/<target> (deterministic).

  python3 fuzz/seeds0218.py

Layouts follow the readers in fuzz/r0218/fuzz_targets. Seeds are small valid cases that reach
the main paths: planar quads with blob masks, split cards, repeat wrap, vertex cost, bakes of a
quad / cube / crossed cards with each albedo source, and reconstructions of those atlases.
"""
from pathlib import Path
import random

OUT = Path(__file__).resolve().parent / 'seeds'


def write(target, name, data):
    d = OUT / target
    d.mkdir(parents=True, exist_ok=True)
    (d / name).write_bytes(bytes(b & 255 for b in data))


def q(v, scale=16, bias=128):
    return max(0, min(252, round(v * scale) + bias))


def trim(name, flags, nv, nt, w, h, mode, thr, maxv, pad, cost, stride, lim, mask, verts, idx):
    raw = [0] * 256
    for y in range(h):
        for x in range(w):
            raw[y * 16 + x] = mask(x, y)
    out = [flags, nv - 1, nt, w - 1, h - 1, mode, thr, maxv - 2, pad, cost, stride - 5, 255, 0] + raw
    for v in verts:
        out += list(v)
    out += idx
    write('trim_cards', name, out)


def vert(x, y, z, u, v):
    return (q(x), q(y), q(z), max(0, min(252, round(u * 176 + 40))), max(0, min(252, round(v * 176 + 40))))


def main():
    rng = random.Random(20261103)
    blob = lambda w, h: (lambda x, y: 255 if (x + 0.5 - w / 2) ** 2 + (y + 0.5 - h / 2) ** 2 < (min(w, h) * 0.35) ** 2 else 0)
    quad = [vert(-1, -1, 0, 0, 0), vert(1, -1, 0, 1, 0), vert(1, 1, 0, 1, 1), vert(-1, 1, 0, 0, 1)]
    qi = [0, 1, 2, 0, 2, 3]
    # Planar quad (uv affine in position), blob mask, several budgets / costs / wraps / paddings.
    for k, (maxv, cost, wrap, pad) in enumerate([(8, 0, 0, 128), (4, 0, 0, 0), (6, 3 << 4, 4, 128), (16, 5 << 4, 0, 255)]):
        trim(f'planar{k}', 0x10, 4, 2, 12, 12, 3 | wrap | cost, 128, maxv, pad, 100, 5, 0, blob(12, 12), quad, qi)
    # Non-affine card: a trapezoid with perspective-like uv forces the split path.
    trap = [vert(-1, -1, 0, 0, 0), vert(1, -1, 0, 1, 0), vert(0.5, 1, 0, 0.8, 1), vert(-0.5, 1, 0, 0.2, 1)]
    trim('trapezoid', 0, 4, 2, 16, 16, 3, 128, 8, 128, 0, 6, 0, blob(16, 16), trap, qi)
    # Two cards, raw noise mask, strided vertices.
    two = quad + [vert(2, -1, 0, 0, 0), vert(3, -1, 0, 1, 0), vert(3, 1, 0, 1, 1), vert(2, 1, 0, 0, 1)]
    trim('two_cards', 0x20, 8, 4, 8, 8, 0, 100, 8, 128, 0, 7, 0, lambda x, y: rng.randrange(256), two, [])
    trim('empty_mask', 0x10, 4, 2, 4, 4, 1, 200, 8, 128, 0, 5, 0, lambda x, y: 0, quad, qi)
    trim('full_mask', 0x10, 4, 2, 4, 4, 2, 200, 8, 128, 0, 5, 0, lambda x, y: 255, quad, qi)
    trim('invalid', 0x88, 4, 2, 4, 4, 3, 128, 8, 128, 0, 5, 0, blob(4, 4), [(255, 255, 255, 255, 255)] * 4, qi)

    # bake: flags, nv-1, nt, layout, vb, rb, sb, db, cutoff, cov, mode, lim(2), bc(4), then vertices/indices.
    def bake(name, flags, layout, views, res, spa, dil, mode, verts, idx, cutoff=128, cov=128):
        out = [flags, len(verts) - 1, len(idx) // 3, layout, views - 2, res - 1, spa - 1, dil, cutoff, cov, mode, 200, 255, 128, 128, 128, 40]
        for v in verts:
            out += [q(c) for c in v]
        out += idx
        write('bake', name, out)

    qp = [(-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0)]
    cube = [(x, y, z) for x in (-1, 1) for y in (-1, 1) for z in (-1, 1)]
    cube_i = [0, 1, 3, 0, 3, 2, 4, 6, 7, 4, 7, 5, 0, 4, 5, 0, 5, 1, 2, 3, 7, 2, 7, 6, 0, 2, 6, 0, 6, 4, 1, 5, 7, 1, 7, 3]
    cross = qp + [(0, -1, -1), (0, -1, 1), (0, 1, 1), (0, 1, -1)]
    cross_i = [0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]
    bake('quad_white', 0, 0, 3, 8, 2, 4, 0b0001, qp, [0, 1, 2, 0, 2, 3])
    bake('cube_colors_sphere', 0, 1, 4, 10, 2, 2, 0b0111, cube, cube_i)
    bake('cross_texture', 0, 0, 3, 12, 1, 3, 0b10110, cross, cross_i)
    bake('cube_normals_bounds', 0, 0, 2, 16, 3, 0, 0b10011, cube, cube_i)
    bake('invalid_params', 0x40, 0, 1, 5, 1, 0, 0b0, qp, [0, 1, 2])
    bake('nan_sampler', 0x84, 1, 3, 6, 1, 1, 0b01000, qp, [0, 1, 2, 0, 2, 3])

    # reconstruct: flags, nv-1, nt, layout, views-2, res-1, dil, then verts, idx, then params.
    def recon(name, flags, layout, views, res, dil, verts, idx, direction, steps, point, pow_b, odd, corrupt, arg):
        out = [flags, len(verts) - 1, len(idx) // 3, layout, views - 2, res - 1, dil]
        for v in verts:
            out += [q(c) for c in v]
        out += idx
        out += [(round(d * 64) + 128) & 255 for d in direction]
        out += [steps, round(point[0] * 255), round(point[1] * 255), pow_b, odd, corrupt, arg]
        write('reconstruct', name, out)

    recon('quad_front', 0, 1, 3, 16, 2, qp, [0, 1, 2, 0, 2, 3], (0, 0, 1), 3, (0.5, 0.5), 3, 6, 0, 0)
    recon('cross_top', 1, 0, 4, 12, 3, cross, cross_i, (0.2, 1, 0.1), 0, (0.4, 0.6), 2, 4, 0, 0)
    recon('cube_steps', 0, 1, 4, 10, 1, cube, cube_i, (0.5, 0.5, 0.7), 8, (0.3, 0.3), 4, 7, 0x21, 3)
    recon('corrupt_atlas', 0, 0, 3, 8, 2, qp, [0, 1, 2, 0, 2, 3], (0, 1, 0.2), 3, (0.5, 0.5), 1, 3, 0x45, 1)

    # layout: flags, n, direction (3 bytes), point (2 bytes), view, then bounds / camera bytes.
    def layout(name, flags, n, d, p, view, tail):
        write('layout', name, [flags, n] + [(round(x * 32) + 128) & 255 for x in d] + [round((x + 1) * 127.5) for x in p] + [view] + tail)

    tail = [100, 120, 140, 60, 4, 30, 200, 128, 90, 60, 30]
    layout('hemi_up', 0, 6, (0, 1, 0), (0, 0), 7, tail)
    layout('sphere_down', 1, 6, (0.1, -1, 0.3), (0.5, -0.5), 20, tail)
    layout('hemi_below', 0, 8, (1, -1, 0), (1, 1), 63, tail)
    layout('grid_min', 1, 0, (1, 0, 0), (-1, -1), 3, tail)
    layout('raw_direction', 4, 5, (0, 0, 0), (0, 0), 2, [0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 0, 0] + tail)

    for t, n in [('trim_cards', 'tiny_zero'), ('bake', 'tiny_zero'), ('reconstruct', 'tiny_zero'), ('layout', 'tiny_zero')]:
        write(t, n, [0] * 8)


main()
