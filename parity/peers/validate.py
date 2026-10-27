"""Validate outputs independently and retain exact first differences to C++ 1.3."""
import hashlib,json,math,os,struct,subprocess
from pathlib import Path
import numpy as np
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU']=='0'
art=Path(os.environ['PEERS_ART']);rows=[json.loads(s) for s in (art/'counts.jsonl').read_text().splitlines()]
assert len(rows)==840 and len({(r['case'],r['op'],r['arm'],r['rep']) for r in rows})==840
assert all(r['exit']==0 and r['running_percent']==100 for r in rows)
assert max(r['peak_rss_kib'] for r in rows)*1024<=4_000_000_000,'process RSS exceeds declaration'
def sha(b):return hashlib.sha256(b).hexdigest()
def canonical(a):
    a=a.reshape(-1,3);b=np.roll(a,-1,axis=1);c=np.roll(a,-2,axis=1)
    def less(a,b):return (a[:,0]<b[:,0])|((a[:,0]==b[:,0])&((a[:,1]<b[:,1])|((a[:,1]==b[:,1])&(a[:,2]<b[:,2]))))
    r=np.where(less(b,a)[:,None],b,a);r=np.where(less(c,r)[:,None],c,r)
    return r[np.lexsort((r[:,2],r[:,1],r[:,0]))]
def diff(a,b):
    same=a==b
    if same:return dict(identity='byte-identical',bytes=len(a),sha256=sha(a))
    n=min(len(a),len(b));ix=np.flatnonzero(np.frombuffer(a[:n],dtype='u1')!=np.frombuffer(b[:n],dtype='u1'))
    k=int(ix[0]) if len(ix) else n
    return dict(identity='differs',first_byte=k,actual_byte=a[k] if k<len(a) else None,cpp_byte=b[k] if k<len(b) else None,bytes=len(a),cpp_bytes=len(b),sha256=sha(a),cpp_sha256=sha(b))
identities=[];checks=[]
for meta in json.loads((art/'inputs.json').read_text()):
 case=meta['case'];path=art/'inputs'/(case+'.bin');raw=path.read_bytes();n,ni=struct.unpack_from('<II',raw);positions=raw[8:8+n*12];ix=np.frombuffer(raw,dtype='<u4',count=ni,offset=8+n*12).copy();del raw
 assert ix.size%3==0 and np.all(ix<n)
 triangles=canonical(ix)
 for op in ['simplify','simplify-attributes','simplify-sloppy','cache','overdraw','fetch','meshlets','vertex-encode','vertex-decode','index-encode','index-decode','cluster-lod']:
  arms=['ours','cpp'] if op=='cluster-lod' else ['ours','cpp','rust']
  cpp=(art/'outputs'/f'{case}-{op}-cpp-0.bin').read_bytes()
  for arm in arms:
   out=art/'outputs'/f'{case}-{op}-{arm}-0.bin';b=out.read_bytes()
   record=dict(case=case,op=op,arm=arm,**diff(b,cpp))
   if op=='meshlets':record['raw']=diff(Path(str(out)+'.raw').read_bytes(),(art/'outputs'/f'{case}-{op}-cpp-0.bin.raw').read_bytes())
   identities.append(record)
   if op in ['vertex-encode','index-encode']:
    # Cross-decode each encoder's exact stream with all three implementations.
    cross=art/'cross.bin';vb=b if op=='vertex-encode' else b'';ib=b if op=='index-encode' else b''
    cross.write_bytes(struct.pack('<II',n,ni)+positions+ix.tobytes()+b''.join(struct.pack('<I',len(x))+x for x in [vb,ib]+[b'']*5))
    decode=op.replace('encode','decode')
    for decoder in ['ours','cpp','rust']:
     dst=art/'cross-output.bin';r=subprocess.run([str(art/f'driver-{decoder}'),decode,str(cross),str(dst)],input='stop\n',text=True,capture_output=True);assert r.returncode==0,(case,op,arm,decoder,r.stderr)
     decoded=dst.read_bytes()
     if op=='vertex-encode':assert decoded==positions
     else:assert np.array_equal(canonical(np.frombuffer(decoded,dtype='<u4')),triangles)
   elif op=='vertex-decode':assert b==positions
   elif op in ['cache','overdraw','index-decode']:
    a=np.frombuffer(b,dtype='<u4');assert len(a)==ni and np.all(a<n);assert np.array_equal(canonical(a),triangles)
   elif op.startswith('simplify'):
    a=np.frombuffer(b[:-4],dtype='<u4');e=struct.unpack('<f',b[-4:])[0];assert len(a)%3==0 and len(a)<=ni and np.all(a<n) and math.isfinite(e) and e>=0
   elif op=='fetch':
    count=struct.unpack_from('<I',b)[0];a=np.frombuffer(b,dtype='<u4',count=count,offset=4);v=np.frombuffer(b[4+count*4:],dtype='u1').reshape(-1,12);p=np.frombuffer(positions,dtype='u1').reshape(-1,12);assert count==ni and np.all(a<len(v));assert np.array_equal(v[a],p[ix])
   elif op=='meshlets':
    count,nv=struct.unpack_from('<II',b);desc=np.frombuffer(b,dtype='<u4',count=count*4,offset=8).reshape(-1,4);v=np.frombuffer(b,dtype='<u4',count=nv,offset=8+count*16);t=np.frombuffer(b,dtype='u1',offset=8+count*16+nv*4);flat=[]
    for vo,to,vc,tc in desc:
     vo,to,vc,tc=map(int,[vo,to,vc,tc]);assert 0<vc<=64 and 0<tc<=124 and vo+vc<=nv and to+tc*3<=len(t);local=t[to:to+tc*3];assert np.all(local<vc);flat.append(v[vo+local.astype('u4')])
    a=np.concatenate(flat);assert len(a)==ni and np.all(a<n);assert np.array_equal(canonical(a),triangles)
   else:
    # Canonical cluster-LOD records: ensure valid levels/refinement IDs/bounds and triangles.
    a=np.frombuffer(b,dtype='<u4');c=0;gid=0;level0=[]
    while c<len(a):
     depth,num=map(int,a[c:c+2]);assert depth<100;c+=2;bounds=a[c:c+5].view('<f4');assert np.all(np.isfinite(bounds)) and np.all(bounds[3:]>=0);c+=5
     for _ in range(num):
      refined,vc,ic=map(int,a[c:c+3]);c+=3;assert refined==0xffffffff or refined<gid
      bounds=a[c:c+5].view('<f4');assert np.all(np.isfinite(bounds)) and np.all(bounds[3:]>=0);c+=5;idx=a[c:c+ic];c+=ic;assert ic%3==0 and np.all(idx<n) and len(np.unique(idx))==vc
      if depth==0:level0.append(idx)
     gid+=1
    assert c==len(a) and np.array_equal(canonical(np.concatenate(level0)),triangles)
   checks.append(dict(case=case,op=op,arm=arm,valid=True))
  print(case,op,'validated',flush=True)
(art/'identities.json').write_text(json.dumps(identities,indent=2)+'\n')
(art/'validation.json').write_text(json.dumps(dict(exit=0,checks=checks,encoder_cross_decodes=8*2*3*3),indent=2)+'\n')
for name in ['cross.bin','cross-output.bin']:
 p=art/name
 if p.exists():p.unlink()
print('PASS',len(checks),'outputs;',8*2*3*3,'cross decodes')
