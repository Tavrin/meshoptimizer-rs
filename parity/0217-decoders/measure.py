"""Interleave before/after against pinned C++ wasm with byte assertions."""
import base64, hashlib, json, os, pathlib, statistics, struct, subprocess
root = pathlib.Path.cwd()
art = root/'parity/0217-decoders/receipts'
inputs = pathlib.Path(os.environ['DECODE_INPUTS'])
manifest = json.loads(pathlib.Path(os.environ['DECODE_MANIFEST']).read_text())
reference = pathlib.Path(os.environ['MESHOPT_REFERENCE'])/'js/meshopt_decoder.mjs'
p = subprocess.Popen(['taskset', '-c', str(min(os.sched_getaffinity(0))), 'node', str(root/'parity/0217-decoders/bench.mjs'), str(art/'after.wasm'), str(art/'before.wasm'), str(reference)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
rows = []
for case in manifest:
    b = (inputs/(case['case']+'.bin')).read_bytes()
    assert hashlib.sha256(b).hexdigest() == case['sha256']
    n, ni = struct.unpack_from('<II', b); at = 8+n*12+ni*4
    for op, count, stride in [(1,n,12),(2,ni,4)]:
        size = struct.unpack_from('<I', b, at)[0]; at += 4
        source = b[at:at+size]; at += size
        request = b'MC02'+struct.pack('<10I',op,count,stride,0,0,1,2,0,1,size)+source
        # Calibrate every arm to at least 150 ms per timed batch. The short
        # initial screen had timer-quantized zero samples and is not a verdict.
        iterations=128 if n<10000 else 2
        while True:
            q=dict(input=base64.b64encode(request).decode(),into=0,pair=0,iterations=iterations)
            p.stdin.write(json.dumps(q)+'\n');p.stdin.flush()
            trial=json.loads(p.stdout.readline())
            fastest=min(trial['times'].values())
            if fastest>0 and fastest*iterations>=.025:break
            iterations*=4
        iterations=max(iterations,min(1000000,int(.150/fastest)+1))
        raw=[]
        for rep in range(9):
            q=dict(input=base64.b64encode(request).decode(),into=0,pair=rep,iterations=iterations)
            p.stdin.write(json.dumps(q)+'\n'); p.stdin.flush()
            r=json.loads(p.stdout.readline())
            if rep>=2: raw.append(r)
        med={k:statistics.median(r['times'][k] for r in raw) for k in ['rust','cpp','scalar']}
        rows.append(dict(case=case['case'],op=op,iterations=iterations,raw=raw,median_seconds={'after':med['rust'],'before':med['scalar'],'cpp':med['cpp']}, before_cpp=med['scalar']/med['cpp'],after_cpp=med['rust']/med['cpp'],after_before=med['rust']/med['scalar']))
        print(case['case'],op,rows[-1]['before_cpp'],rows[-1]['after_cpp'],flush=True)
p.stdin.close(); assert p.wait()==0
(art/'wasm.json').write_text(json.dumps(dict(rows=rows,method='Node v22 allocating API, equal copies; hrtime.bigint, calibrated 150 ms fastest-arm batch target, 2 warmup rounds, 7 rotated rounds; wall medians, seconds per call; scalar label in reused harness denotes SIMD before',sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [art/'before.wasm',art/'after.wasm',reference]}),indent=2)+'\n')
