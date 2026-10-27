"""Freeze packed projections of retained harness inputs; C++ produces decode fixtures."""
import hashlib,json,os,struct,subprocess
from pathlib import Path
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
art=Path(os.environ['PEERS_ART']); source=Path(os.environ['PEERS_INPUTS'])
inputs=art/'inputs';inputs.mkdir(exist_ok=True)
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
metadata=[]
for name in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth','real-214e','real-a082','real-b6fd']:
    src=source/(name+('-simplify.bin' if name.startswith('real-') else '.bin'))
    dst=inputs/(name+'.bin')
    if not dst.exists():
        with src.open('rb') as f:
            n,ni=struct.unpack('<II',f.read(8));prefix=struct.pack('<II',n,ni)+f.read(n*12+ni*4)
        dst.write_bytes(prefix+bytes(28))
        buffers=[]
        for op in ['vertex-encode','index-encode']:
            out=art/(name+'-'+op+'.fixture')
            result=subprocess.run([str(art/'driver-cpp'),op,str(dst),str(out)],input='stop\n',text=True,capture_output=True)
            assert result.returncode==0,result.stderr
            buffers.append(out.read_bytes())
        dst.write_bytes(prefix+b''.join(struct.pack('<I',len(b))+b for b in buffers+[b'']*5))
    with dst.open('rb') as f:n,ni=struct.unpack('<II',f.read(8))
    metadata.append(dict(case=name,vertices=n,indices=ni,sha256=sha(dst),source_sha256=sha(src),projection='packed XYZ and original indices; synthetic four attributes from existing harness'))
    print(name,n,ni,'prepared',flush=True)
(art/'inputs.json').write_text(json.dumps(metadata,indent=2)+'\n')
