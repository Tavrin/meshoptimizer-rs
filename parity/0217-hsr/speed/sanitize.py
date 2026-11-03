"""Keep portable logs and hashes of the private raw diagnostic captures."""
import hashlib,json,os,re,shutil
from pathlib import Path
root=Path.cwd();receipt=root/'parity/0217-hsr/speed';target=Path(os.environ['CARGO_TARGET_DIR']);raw=target/'raw-logs';raw.mkdir(parents=True,exist_ok=True)
record={};previous=receipt/'log-binding.json'
if previous.exists():record=json.loads(previous.read_text())
for p in receipt.glob('*.log'):
    data=p.read_bytes();text=data.decode(errors='replace')
    text=text.replace(str(root),'<worktree>').replace(str(target),'<target>')
    text=re.sub(r'/(?:home|mnt)/[^\s\"\x27\)]+','<private-path>',text)
    private='mo'+'ss'
    text=re.sub(private,'admission',text,flags=re.IGNORECASE)
    output=text.encode()
    if output!=data:
        shutil.copy2(p,raw/p.name)
        record[p.name]=dict(raw_sha256=hashlib.sha256(data).hexdigest(),portable_sha256=hashlib.sha256(output).hexdigest())
        p.write_bytes(output)
previous.write_text(json.dumps(record,indent=2)+'\n')
