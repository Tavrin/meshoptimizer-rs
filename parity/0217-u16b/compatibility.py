import hashlib,json,subprocess
from pathlib import Path
root=Path.cwd();base="b89cc4fe03efe346e4695594f1ca4e4688a8c8f1"
files=subprocess.check_output(["git","ls-tree","-r","--name-only",base,"examples","tests"],text=True).splitlines()
rows=[]
for name in files:
 original=subprocess.check_output(["git","show",base+":"+name]);current=(root/name).read_bytes()
 assert original==current,name
 rows.append({"file":name,"sha256":hashlib.sha256(current).hexdigest()})
Path("parity/0217-u16b/compatibility.json").write_text(json.dumps({"baseline":base,"unchanged_files":rows},indent=2)+"\n")
print(len(rows),"unchanged baseline test/example files")
# Existing documentation examples must compile without inference edits too.
import re
records=[]
for name in subprocess.check_output(['git','ls-tree','-r','--name-only',base,'src','docs','README.md'],text=True).splitlines():
 if not name.endswith(('.rs','.md')):continue
 original=subprocess.check_output(['git','show',base+':'+name],text=True)
 current=(root/name).read_text()
 def blocks(source):
  if name.endswith('.rs'):
   source='\n'.join(re.sub(r'^\s*//[/!] ?', '',line) for line in source.splitlines() if re.match(r'^\s*//[/!]',line))
  return re.findall(r'(?ms)^```[^\n]*\n.*?^```',source)
 before,after=blocks(original),blocks(current)
 assert after[:len(before)]==before,name
 if before:records.append({'file':name,'original_blocks':len(before),'sha256':hashlib.sha256('\n'.join(before).encode()).hexdigest()})
Path('parity/0217-u16b/doctest-sources.json').write_text(json.dumps(records,indent=2)+'\n')
print(sum(row['original_blocks'] for row in records),'unchanged original documentation blocks')

from source_blocks import blocks
units=[]
for name in subprocess.check_output(['git','ls-tree','-r','--name-only',base,'src'],text=True).splitlines():
 if not name.endswith('.rs'):continue
 original=subprocess.check_output(['git','show',base+':'+name],text=True)
 current={n:b for n,_,_,b in blocks((root/name).read_text())}
 for n,_,_,block in blocks(original):
  assert current.get(n)==block,(name,n)
  units.append({'file':name,'module':n,'sha256':hashlib.sha256(block.encode()).hexdigest()})
Path('parity/0217-u16b/unit-test-sources.json').write_text(json.dumps(units,indent=2)+'\n')
print(len(units),'unchanged original inline test modules')
