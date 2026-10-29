"""Retain raw disassembly and function sizes for controlled executable arms."""
import hashlib,json,os,re,subprocess
from pathlib import Path
A=Path(os.environ['DECODE_ART']);out=A/'codegen';out.mkdir(exist_ok=True)
rows={}
for arm in os.environ.get('DECODE_ARMS','base control unisolated candidate').split():
 p=A/('driver-'+arm)
 symbols=subprocess.check_output(['nm','-S','-C',str(p)],text=True)
 selected=[]
 for line in symbols.splitlines():
  if re.search(r'codec::(raw$|allocate$|vertex::decode)|meshopt_peers::run$',line):
   addr,size,kind,name=line.split(None,3);addr=int(addr,16);size=int(size,16)
   asm=subprocess.check_output(['objdump','-d','-C','--no-show-raw-insn',f'--start-address={addr}',f'--stop-address={addr+size}',str(p)],text=True)
   namefile=re.sub('[^a-zA-Z0-9]+','-',name).strip('-')
   (out/(arm+'-'+namefile+'.asm')).write_text(asm)
   selected.append(dict(symbol=name,bytes=size,address=addr,disassembly_sha256=hashlib.sha256(asm.encode()).hexdigest()))
 rows[arm]=selected
(out/'functions.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(rows,indent=2))
