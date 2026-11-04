#!/usr/bin/env python3
"""Prepare first-mesh foliage data from caller-owned glTF, buffer and alpha plane.

Usage: l5_inputs.py asset.gltf asset.bin alpha.raw width height output.input
The alpha plane is tightly packed u8, row zero matching UV v=0. Positions are
centered and normalized to extent 2. Input files are read as data only.
"""
import hashlib
import json
import pathlib
import struct
import sys

asset, buffer, alpha = map(pathlib.Path, sys.argv[1:4])
width, height = map(int, sys.argv[4:6])
output = pathlib.Path(sys.argv[6])
gltf = json.loads(asset.read_text())
data = buffer.read_bytes()


def read(index):
    accessor = gltf['accessors'][index]
    view = gltf['bufferViews'][accessor['bufferView']]
    count = {'SCALAR': 1, 'VEC2': 2, 'VEC3': 3}[accessor['type']]
    code = {5123: 'H', 5125: 'I', 5126: 'f'}[accessor['componentType']]
    stride = view.get('byteStride', count * struct.calcsize(code))
    offset = view.get('byteOffset', 0) + accessor.get('byteOffset', 0)
    return [struct.unpack_from('<' + code * count, data, offset + k * stride)
            for k in range(accessor['count'])]


primitive = gltf['meshes'][0]['primitives'][0]
positions = read(primitive['attributes']['POSITION'])
uvs = read(primitive['attributes']['TEXCOORD_0'])
indices = read(primitive['indices'])
lo = [min(v[k] for v in positions) for k in range(3)]
hi = [max(v[k] for v in positions) for k in range(3)]
scale = 2 / max(hi[k] - lo[k] for k in range(3))
positions = [[(v[k] - (lo[k] + hi[k]) * 0.5) * scale for k in range(3)] for v in positions]
plane = alpha.read_bytes()
assert len(plane) == width * height
out = bytearray(struct.pack('<IIII', len(positions), len(indices), width, height))
for v in positions:
    out.extend(struct.pack('<3f', *v))
for v in uvs:
    out.extend(struct.pack('<2f', *v))
for v in indices:
    out.extend(struct.pack('<I', *v))
out.extend(plane)
output.write_bytes(out)
print(json.dumps({'triangles': len(indices) // 3,
                  'sha256': {p.name: hashlib.sha256(p.read_bytes()).hexdigest()
                             for p in [asset, buffer, alpha, output]}}, sort_keys=True))
