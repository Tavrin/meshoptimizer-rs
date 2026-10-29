"""Bind final implementation, tests and proof recipes; reject source drift."""
import hashlib,json,sys
from pathlib import Path
root=Path(__file__).resolve().parents[2];lane=Path(__file__).resolve().parent
files=[*root.glob('src/**/*.rs'),*root.glob('tests/*.rs'),root/'Cargo.toml',root/'Cargo.lock',root/'src/codec/simd/SAFETY.md',root/'parity/0215-toward-cpp/geometry-exact.rs',root/'parity/p03/src/lib.rs',root/'parity/p03/src/main.rs',root/'parity/p03/Cargo.toml',root/'parity/p03/Cargo.lock',root/'parity/p03/reference.cpp',root/'parity/p03/runner.py',root/'parity/wasm.cjs',root/'parity/check-reference.sh',root/'parity/check-unsafe-boundary.sh',root/'parity/simd/check_boundary.py',root/'parity/simd/check_package.py',root/'parity/tools/check_public_text.sh',*[p for p in lane.iterdir() if p.suffix in ['.rs','.py','.sh']]]
binding={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(files))}
out=Path(sys.argv[1])
if len(sys.argv)>2:
 assert json.loads(out.read_text())==binding,'final proof source drift'
 print('PASS: final proof source binding unchanged')
else:
 assert not out.exists(),'refuse overwrite of a proof binding'
 out.write_text(json.dumps(binding,indent=2)+'\n')
 print('Bound',len(binding),'source/test/recipe files')
