"""Account frozen MDAG 2 disk streams without decoding or rebuilding geometry.

Run inside the admitted CPU scope with VG_EVIDENCE and VG_CONTINUATION set.
This diagnostic expects the validated compressed artifacts from the campaign.
"""
import json
import os
from pathlib import Path
import struct


def account(path):
    data = path.read_bytes()
    u32 = lambda at: struct.unpack_from("<I", data, at)[0]
    assert data[:4] == b"MDAG" and u32(4) == 2
    chunks = {}
    for i in range(u32(12)):
        at = 64 + i * 16
        chunks[data[at:at + 4]] = (u32(at + 8), u32(at + 12))
    result = dict(metadata=u32(24), page_headers=0, descriptors=0,
                  stream_tables=0, positions=0, triangles=0,
                  extension_headers=0, extensions=0, padding=0)
    mesh = chunks[b"MESH"][0]
    result["dag_triangles"] = struct.unpack_from("<Q", data, mesh + 32)[0]
    page, size = chunks[b"PAGE"]
    end = u32(24)
    for at in range(page, page + size, 48):
        offset = struct.unpack_from("<Q", data, at)[0]
        length, n = u32(at + 8), u32(at + 20)
        result["padding"] += offset - end
        assert u32(offset) == 1 and u32(offset + 8) == n
        result["page_headers"] += 24  # disk codec/count plus memory header
        result["descriptors"] += 64 * n
        result["stream_tables"] += 4 * n
        table = offset + 24 + 64 * n
        cursor = table + 4 * n
        for c in range(n):
            lengths = u32(table + 4 * c)
            positions, triangles = lengths & 65535, lengths >> 16
            result["positions"] += positions
            result["triangles"] += triangles
            cursor += positions + triangles
            desc = offset + 24 + 64 * c
            if u32(desc + 36) != 0 or not (u32(desc + 8) & (1 << 18)):
                extension = u32(cursor + 4)
                result["extension_headers"] += 8
                result["extensions"] += extension
                cursor += 8 + extension
        assert cursor == offset + length
        end = offset + length
    result["padding"] += len(data) - end
    result["file_bytes"] = len(data)
    assert sum(v for k, v in result.items() if k not in ["dag_triangles", "file_bytes"]) == len(data)
    return result


old = Path(os.environ["VG_EVIDENCE"])
out = Path(os.environ["VG_CONTINUATION"])
rows = {}
for mesh in ["bunny", "dead_tree_trunk_02", "namaqualand_boulder_02", "coast_rocks_01", "dragon"]:
    rows[mesh] = {arm: account(old / "final" / f"{mesh}.{arm}.8T.bin")
                  for arm in ["baseline", "ours"]}
(out / "disk-accounting.json").write_text(json.dumps(rows, indent=2) + "\n")
