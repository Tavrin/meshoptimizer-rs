#!/usr/bin/env python3
"""Export the shared runner fixtures for the LOD chain identity test."""
import array
import pathlib
import struct
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import runner

out = pathlib.Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
for i, (name, positions, indices, _) in enumerate(runner.fixtures()):
    if len(indices) < 12:
        continue
    with (out / f"{i:02}-{name}.bin").open("wb") as f:
        f.write(struct.pack("<II", len(positions) // 3, len(indices)))
        for values, code in ((positions, "f"), (indices, "I")):
            data = array.array(code, values)
            if sys.byteorder != "little":
                data.byteswap()
            f.write(data.tobytes())
