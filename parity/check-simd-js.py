#!/usr/bin/env python3
"""Check the actual shipped upstream JS SIMD decoder on the timing corpus."""
import base64, hashlib, json, os, struct, subprocess, zipfile
from pathlib import Path
art=Path(os.environ['MESHOPT_ARTIFACTS'])
identity=json.loads((art/'wasm-upstream-identity.json').read_text())
reference=Path(identity['path'])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(reference)==identity['sha256_before']
script=r'''
import readline from 'node:readline';
import {pathToFileURL} from 'node:url';
const {MeshoptDecoder:d}=await import(pathToFileURL(process.argv[1]));await d.ready;
if(!d.supported)throw Error('upstream decoder unsupported');
for await(const line of readline.createInterface({input:process.stdin})){
 const b=Buffer.from(JSON.parse(line),'base64'),op=b.readUInt32LE(4),n=b.readUInt32LE(8),s=b.readUInt32LE(12),mode=b.readUInt32LE(16)&127,f=b.readUInt32LE(20),src=b.subarray(44);
 const out=new Uint8Array(n*s+17);out.fill(0xcc);
 if(op===1)d.decodeVertexBuffer(out,n,s,src);
 else if(op===2)d.decodeIndexBuffer(out,n,s,src);
 else if(op===3)d.decodeIndexSequence(out,n,s,src);
 else d.decodeGltfBuffer(out,n,s,src,['ATTRIBUTES','TRIANGLES','INDICES'][mode],['NONE','OCTAHEDRAL','QUATERNION','EXPONENTIAL'][f]);
 if(!out.subarray(n*s).every(v=>v===0xcc))throw Error('destination tail changed');
 process.stdout.write(JSON.stringify(Buffer.from(out.subarray(0,n*s)).toString('base64'))+'\n');
}
'''
p=subprocess.Popen(['node','--input-type=module','-e',script,str(reference)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
record={'script_sha256':sha(Path(__file__)),'upstream_sha256':identity['sha256_before'],'node':subprocess.check_output(['node','--version'],text=True).strip(),'archive_sha256':sha(art/'benchmark-identity.zip'),'cases':[]}
output_archive=art/'wasm-upstream-conformance.zip'
with zipfile.ZipFile(art/'benchmark-identity.zip') as archive, zipfile.ZipFile(output_archive,'w',compression=zipfile.ZIP_DEFLATED) as outputs:
 for c in json.loads((art/'benchmark-identity.json').read_text())['cases']:
  b=archive.read(c['case']+'.input');op=struct.unpack_from('<I',b,4)[0]
  if op not in [1,2,3,7]:continue
  expected=archive.read(c['case']+'.output')
  p.stdin.write(json.dumps(base64.b64encode(b).decode())+'\n');p.stdin.flush()
  actual=base64.b64decode(json.loads(p.stdout.readline()))
  assert len(actual)==len(expected),c['case']
  f=struct.unpack_from('<I',b,20)[0] if op==7 else 0
  distance=0
  if f in [1,2]:
   stride=struct.unpack_from('<I',b,12)[0];fmt='b' if f==1 and stride==4 else 'h'
   a=struct.unpack('<'+fmt*(len(actual)//struct.calcsize(fmt)),actual);e=struct.unpack('<'+fmt*(len(expected)//struct.calcsize(fmt)),expected)
   distance=max([abs(x-y) for x,y in zip(a,e)]+[0]);assert distance<=1,c['case']
  else:assert actual==expected,c['case']
  outputs.writestr(c['case']+'.output',actual)
  record['cases'].append({'case':c['case'],'output_sha256':hashlib.sha256(actual).hexdigest(),'max_unit_difference':distance,'tail_preserved':True})
p.stdin.close();assert p.wait()==0 and sha(reference)==record['upstream_sha256']
record['complete']=True;record['mismatches']=0
record['output_archive_sha256']=sha(output_archive)
(art/'wasm-upstream-conformance.json').write_text(json.dumps(record,indent=2)+'\n')
print(len(record['cases']),'shipped JS SIMD cases conform; tails preserved')
