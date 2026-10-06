import re, sys
from pathlib import Path
from check_boundary import ROOT, MODULE, code
files=[ROOT/p for p in sys.stdin.read().splitlines() if p.endswith('.rs')]
assert files
allows=[]
for p in files:
    if not p.exists(): continue # Cargo's generated manifest does not contain Rust.
    s=code(p.read_text())
    assert not re.search(r'\bunsafe\b',s) or p.is_relative_to(MODULE),p
    for m in re.finditer(r'\b(?:allow|expect)\s*\([^)]*\bunsafe_code\b[^)]*\)',s,re.S):
        assert p.is_relative_to(MODULE) and m[0].startswith('allow'),p
        allows.append(p)
assert allows==[MODULE/'mod.rs'],allows
print('PASS: published package contains exactly one unsafe-code allowance')
