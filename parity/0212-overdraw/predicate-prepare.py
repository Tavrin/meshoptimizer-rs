import os
"""Two safe equivalent finite predicates; compare after the A/B/AB/C campaign."""
from pathlib import Path
import json,hashlib,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH'])
for arm in ['D','E']:
    d=A/f'variant-{arm}'
    if not d.exists():
        shutil.copytree(A/'variant-B',d)
        p=d/'src/overdraw.rs';s=p.read_text()
        old='!p.iter().all(|x| x.is_finite())'
        replacement='!(p[0].is_finite() & p[1].is_finite() & p[2].is_finite())' if arm=='D' else '(p[0].to_bits() & 0x7fff_ffff).max(p[1].to_bits() & 0x7fff_ffff).max(p[2].to_bits() & 0x7fff_ffff) >= 0x7f80_0000'
        assert old in s;s=s.replace(old,replacement);p.write_text(s)
        shutil.copytree(A/'harness-B',A/f'harness-{arm}')
        p=A/f'harness-{arm}/Cargo.toml';p.write_text(p.read_text().replace('variant-B',f'variant-{arm}'))
    (A/f'{arm}.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'predicate-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for arm in ['D','E'] for p in (A/f'variant-{arm}/src').rglob('*.rs')},indent=2)+'\n')
