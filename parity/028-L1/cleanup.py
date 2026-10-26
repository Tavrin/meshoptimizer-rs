#!/usr/bin/env python3
"""Retain evidence, then delete only the specified lane Cargo target."""
import shutil,json
from control import ART,TARGET,ROOT,LANE,sha,save
assert TARGET==__import__('pathlib').Path('/mnt/linux-extra/moss-cargo-targets/meshopt-L1')
assert not TARGET.is_symlink()
assert json.loads((ART/'campaign-complete.json').read_text())['exit']==0
binaries={p.name:sha(p)for p in ART.iterdir()if p.is_file()and(p.name.startswith(('driver-','proof-','rfc-','p03-'))or p.name=='division-fuzz')and p.stat().st_mode&0o111}
assert all(k in binaries for k in ['driver-final-ours','driver-final-base','driver-final-M7','driver-final-X','proof-ours','proof-base','rfc-ours','rfc-cpp','p03-cpp','p03-ours','division-fuzz'])
save(ART/'RETAINED-BINARIES.json',binaries)
if TARGET.exists():shutil.rmtree(TARGET)
assert not TARGET.exists()
assert all(sha(ART/name)==h for name,h in binaries.items())
save(ART/'CLEANUP.json',{'exit':0,'deleted':str(TARGET),'absent':not TARGET.exists(),'retained_binaries':len(binaries)})
