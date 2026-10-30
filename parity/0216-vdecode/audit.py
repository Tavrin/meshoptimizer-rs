"""Refresh unsafe inventory while preserving the existing safety explanations."""
from pathlib import Path
import re,subprocess
R=Path.cwd();p=R/'src/codec/simd/SAFETY.md'
base=subprocess.check_output(['git','show','HEAD:src/codec/simd/SAFETY.md'],text=True)
subprocess.run(['python3','parity/simd/update_audit.py'],check=True)
generated=p.read_text()
key=lambda s:re.search(r'`([^`]+)`',s).group(1)
original={key(s):s for s in base.splitlines() if s.startswith('| `')}
current={key(s):s for s in generated.splitlines() if s.startswith('| `')}
rows=[s for k,s in original.items() if k in current]
rows += [s for k,s in current.items() if k not in original]
head,rest=base.split('|---|---|\n',1);_,tail=rest.split('\nInventory:',1)
text=head+'|---|---|\n'+'\n'.join(rows)+'\n\nInventory:'+tail
text=re.sub(r'Inventory: \*\*\d+ unsafe blocks\*\*',f'Inventory: **{len(rows)} unsafe blocks**',text)
p.write_text(text)
