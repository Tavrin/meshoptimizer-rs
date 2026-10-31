from support import *
import sys
binary=build('iterate',ROOT,['std','simd'])
cpu=min(os.sched_getaffinity(0)); rows=[]
for scene in ['breakfast_room','conference','living_room']:
    out=SCRATCH/('iterate-'+scene+'.out')
    log=run('iterate-'+scene,['taskset','-c',cpu,binary,SCENES/(scene+'.bin'),out],input='measured\nstop\n')
    samples=[json.loads(line) for line in log.splitlines() if line.startswith('{')]
    rows.append(dict(scene=scene,samples=samples))
    expected=SCENES/(scene+'-default-0.out')
    assert out.read_bytes()==expected.read_bytes(),scene
    print('IDENTICAL',scene,sha(out),flush=True)

(RECEIPT/'iterate.json').write_text(json.dumps(rows,indent=2)+'\n')
