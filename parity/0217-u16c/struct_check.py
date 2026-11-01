"""Check that legacy struct definitions, attributes and fields survive unchanged."""
from pathlib import Path
import re,subprocess,json
rows=[]
for p in Path('src').rglob('*.rs'):
 s=p.read_text()
 if 'Generic' not in s:continue
 old=subprocess.run(['git','show','b89cc4fe03efe346e4695594f1ca4e4688a8c8f1:'+str(p)],capture_output=True,text=True)
 if old.returncode:continue
 for m in re.finditer(r'((?:#\[[^\n]+\]\n)*)pub struct (\w+)\b([^{}]*)\{([^}]+)\}',old.stdout):
  name=m[2]
  if not re.search(r'pub struct '+name+r'Generic\b',s):continue
  cur=re.search(r'((?:#\[[^\n]+\]\n)*)pub struct '+name+r'\b([^{}]*)\{([^}]+)\}',s)
  assert cur,name
  strip=lambda text:re.sub(r'\s+','',re.sub(r'///[^\n]*','',text))
  assert strip(m[1]+m[3]+m[4])==strip(cur[1]+cur[2]+cur[3]),name
  rows.append(dict(file=str(p),name=name))
assert len(rows)==23,len(rows)
Path('parity/0217-u16c/structs.json').write_text(json.dumps(rows,indent=2)+'\n')
print(len(rows),'unchanged concrete legacy struct definitions')
