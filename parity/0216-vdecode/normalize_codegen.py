"""Compare codegen after removing relocation addresses, retaining local offsets."""
import difflib,hashlib,json,os,re
from pathlib import Path
A=Path(os.environ['DECODE_ART']);D=A/'codegen';rows={}
for p in D.glob('*-meshoptimizer-rs-codec-raw.asm'):
 lines=[]
 for line in p.read_text().splitlines():
  m=re.match(r'\s*[0-9a-f]+:\s+(.*)',line)
  if not m:continue
  s=m[1]
  s=re.sub(r'\b[0-9a-f]+ (<[^>]+>)',r'\1',s)
  s=re.sub(r'-?0x[0-9a-f]+\(%rip\)','<relocation>(%rip)',s)
  lines.append(s)
 text='\n'.join(lines)+'\n';p.with_suffix('.normalized').write_text(text)
 arm=p.name.split('-meshoptimizer')[0];rows[arm]=dict(lines=len(lines),sha256=hashlib.sha256(text.encode()).hexdigest())
base=(D/'base-meshoptimizer-rs-codec-raw.normalized').read_text()
def instructions(s):return '\n'.join(line.partition('#')[0].rstrip() for line in s.splitlines())+'\n'
base_instructions=instructions(base)
for arm in rows:
 text=(D/(arm+'-meshoptimizer-rs-codec-raw.normalized')).read_text()
 (D/(arm+'-raw.diff')).write_text(''.join(difflib.unified_diff(base.splitlines(True),text.splitlines(True),fromfile='base',tofile=arm)))
 rows[arm]['base_identical']=text==base
 code=instructions(text);(D/(arm+'-raw.instructions')).write_text(code)
 rows[arm]['instruction_sha256']=hashlib.sha256(code.encode()).hexdigest()
 rows[arm]['base_instruction_identity']=code==base_instructions
(D/'raw-comparison.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(rows,indent=2))
