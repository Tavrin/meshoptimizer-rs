"""Adapt a scratch copy of the campaign peer consumer ($QUALIFY_PEERS_SOURCE).

Usage: prepare.py PEERS_COPY_DIR. The consumer is the S6 campaign driver
(all peer-table ops plus update, prune, points, flex and spatial; sha256
74d821a3... in parity/0215-simplify-cpp/binding.json). This applies the
basics lane's adapter (parity/0215-basics/prepare_driver.py: remap ops, SIMD
off for meshoptimizer-rs and optimesh) unchanged except for one anchor: in
this driver the RawMesh serialization arm follows the Updated arm on the same
line, so the anchor drops its leading indentation.
"""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[3]
source = (root / 'parity/0215-basics/prepare_driver.py').read_text()
assert source.count("'   Self::") == 2
source = source.replace("'   Self::", "'Self::")
sys.argv = [sys.argv[0], sys.argv[1]]
exec(compile(source, 'prepare_driver.py', 'exec'), {'__name__': '__main__'})
