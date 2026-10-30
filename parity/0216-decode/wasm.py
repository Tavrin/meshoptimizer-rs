"""Executed Node comparison against upstream's pinned shipped SIMD wasm."""
import base64,hashlib,json,os,statistics,struct,subprocess,sys
from pathlib import Path
art=Path(os.environ['DECODE_ART']); root=Path.cwd(); ref=Path(os.environ['MESHOPT_REFERENCE'])
tag="default-" if len(sys.argv)>1 else ""
simd=art/("wasm-default-simd.wasm" if tag else "wasm-base.wasm");scalar=art/("wasm-default-scalar.wasm" if tag else "wasm-scalar.wasm")
rows=[]
proc=subprocess.Popen(['taskset','-c','9','node',str(root/'parity/0216-decode/wasm_bench.mjs'),str(simd),str(scalar),str(ref/'js/meshopt_decoder.mjs')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
for x in json.loads((art/'inputs.json').read_text()):
    b=(art/'inputs'/(x['case']+'.bin')).read_bytes(); n,ni=struct.unpack_from('<II',b); at=8+n*12+ni*4
    for op,count,stride in [(1,n,12),(2,ni,4)]:
        size=struct.unpack_from('<I',b,at)[0]; at+=4; source=b[at:at+size]; at+=size
        request=b'MC02'+struct.pack('<10I',op,count,stride,0,0,1,2,0,1,len(source))+source
        raw=[]
        for rep in range(3):
            q=dict(input=base64.b64encode(request).decode(),into=0,pair=rep,iterations=1000 if n<100 else 20 if n<10000 else 3)
            proc.stdin.write(json.dumps(q)+'\n');proc.stdin.flush();r=json.loads(proc.stdout.readline());raw.append(r)
        med={k:statistics.median(r['times'][k] for r in raw) for k in ['rust','cpp','scalar']}
        rows.append(dict(case=x['case'],op=op,raw=raw,median_seconds=med,ratio=med['rust']/med['cpp']))
        print('wasm',x['case'],op,rows[-1]['ratio'],flush=True)
proc.stdin.close();assert proc.wait()==0
(art/("wasm-"+tag+"baseline.json")).write_text(json.dumps(dict(rows=rows,reference_js_sha256=hashlib.sha256((ref/'js/meshopt_decoder.mjs').read_bytes()).hexdigest(),rust_simd_sha256=hashlib.sha256(simd.read_bytes()).hexdigest(),method='Node v22, median of 3, wall time disclosure; includes equal source/output copies and allocating output; no instruction-performance claim'),indent=2)+'\n')
