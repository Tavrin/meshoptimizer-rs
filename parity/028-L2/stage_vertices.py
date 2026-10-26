"""Invert r2 compaction to recover the pruned positions used by overdraw.

Moss prune_level only rescales kept components; unreferenced vertices in those
components share a welded position with an indexed vertex. Copy captured XYZ
bytes to that entire position class. No floating-point recomputation is used.
"""
import array, hashlib, json, os
from pathlib import Path
ART = Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1<<20),b''):h.update(b)
    return h.hexdigest()

def key(value):
    # welded_components uses (component + 0.0).to_bits(): fold signed zero.
    if b'\x00\x00\x00\x80' not in value:return value
    return b''.join(b'\0'*4 if value[i:i+4]==b'\0\0\0\x80' else value[i:i+4] for i in [0,4,8])

def recover(original, blobs, overdraw, packaged, stride):
    binding = dict(original=sha(original),indices=overdraw['output_indices'],vertices=packaged['compacted_vertex_bytes'],compacted_indices=packaged['compacted_index_bytes'],stride=stride)
    identity=hashlib.sha256(json.dumps(binding,sort_keys=True).encode()).hexdigest()
    folder=ART/'stage-vertices';folder.mkdir(exist_ok=True)
    output=folder/(identity+'.bin');receipt=folder/(identity+'.json')
    if receipt.exists():
        meta=json.loads(receipt.read_text());assert output.exists() and sha(output)==meta['sha256'];return output,meta
    assert not output.exists(), 'retain unexpected prior reconstruction'
    def read(desc):
        path=blobs/desc['$bytes'];assert path.stat().st_size==desc['length'] and sha(path)==desc['sha256'];return path.read_bytes()
    raw=original.read_bytes();n=len(raw)//stride
    compact=read(packaged['compacted_vertex_bytes'])
    globals=array.array('I');globals.frombytes(read(overdraw['output_indices']))
    indices=array.array('I');indices.frombytes(read(packaged['compacted_index_bytes']))
    assert len(globals)==len(indices)
    remap=array.array('I',[0xffffffff])*n
    for g,c in zip(globals,indices,strict=True):
        assert g<n and (c+1)*stride<=len(compact)
        if remap[g]!=0xffffffff:assert remap[g]==c
        remap[g]=c
    positions={}
    for g,c in enumerate(remap):
        if c==0xffffffff:continue
        old=raw[g*stride:g*stride+12];new=compact[c*stride:c*stride+12]
        assert raw[g*stride+12:(g+1)*stride]==compact[c*stride+12:(c+1)*stride]
        k=key(old)
        prior=positions.setdefault(k,new);assert prior==new, 'ambiguous welded position scale'
    del globals,indices,remap,compact
    scaled=bytearray(raw);changed=0
    for g in range(n):
        off=g*stride;old=raw[off:off+12];new=positions.get(key(old),old)
        if old!=new:scaled[off:off+12]=new;changed+=1
    digest=hashlib.sha256(scaled).hexdigest()
    temporary=output.with_suffix('.tmp');temporary.write_bytes(scaled);os.replace(temporary,output)
    meta=dict(binding=binding,sha256=digest,vertices=n,changed_vertices=changed,method='inverse captured index compaction; XYZ bytes propagated to signed-zero-normalized welded-position classes; all other bytes original')
    receipt.write_text(json.dumps(meta,indent=2)+'\n')
    print('recovered',output.name,changed,'changed vertices',flush=True)
    return output,meta
