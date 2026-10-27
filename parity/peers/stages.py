"""Preserve all eighteen existing real-chain simplify targets/errors as a supplement."""
import hashlib,json,os,struct
from pathlib import Path
import numpy as np
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
art=Path(os.environ['PEERS_ART']);source=Path(os.environ['PEERS_STAGES'])
metadata=[]
for s in json.loads(source.read_text()):
    name=s['case'];dst=art/'inputs'/(name+'.bin')
    if not dst.exists():
        p=np.memmap(s['vertex_blob'],dtype='<f4',mode='r').reshape(-1,s['stride']//4)[:,:3].copy()
        indices=Path(s['index_blob']).read_bytes();n=len(p);ni=len(indices)//4
        with dst.open('wb') as f:
            f.write(struct.pack('<II',n,ni));f.write(p.tobytes());f.write(indices);f.write(bytes(28));f.write(struct.pack('<II',s['target'],s['error_bits']))
        del p,indices
    with dst.open('rb') as f:
        n,ni=struct.unpack('<II',f.read(8));f.seek(0);h=hashlib.file_digest(f,'sha256').hexdigest()
    metadata.append(dict(case=name,vertices=n,indices=ni,target=s['target'],error_bits=s['error_bits'],options=s['options'],sha256=h,source_vertex_sha256=Path(s['vertex_blob']).stem,source_index_sha256=Path(s['index_blob']).stem,projection='Packed XYZ; retained simplify target, error, options=0.'))
    assert s['options']==0
    print(name,'prepared',flush=True)
(art/'stage-inputs.json').write_text(json.dumps(metadata,indent=2)+'\n')
