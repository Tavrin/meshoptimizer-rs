"""Preserve one malformed-input failure-buffer counterexample."""
import hashlib,importlib.util,json,os,struct
from pathlib import Path
R=Path(__file__).resolve().parents[2];A=Path(__file__).resolve().parent;T=Path(os.environ['CARGO_TARGET_DIR'])
os.environ['DECODE_ART']=str(A/'decode-all')
spec=importlib.util.spec_from_file_location('decoder',R/'parity/0216-decode/differential.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
src=m.stream(1,8,0,0,0,1);data=struct.pack('<III',1,8,100)+src[:100]
cmds={'rust':[T/'bin/decode-repro-rust','scalar'],'cpp':[T/'bin/decode-repro-cpp']};outputs={}
for name,cmd in cmds.items():
 d=m.Driver(cmd)
 try:outputs[name]=d.call(data)
 finally:d.close()
assert outputs['rust'][:4]==outputs['cpp'][:4] and int.from_bytes(outputs['rust'][:4],'little')!=0
assert outputs['rust']!=outputs['cpp']
row=dict(case=dict(n=1,stride=8,v=0,ch=0,ctrl=0,sel=1,length=100),input_hex=data.hex(),outputs={k:v.hex() for k,v in outputs.items()},binary_sha256={k:hashlib.sha256(Path(v[0]).read_bytes()).hexdigest() for k,v in cmds.items()})
(A/'cpp-failure-counterexample.json').write_text(json.dumps(row,indent=2)+'\n');print(json.dumps(row))
