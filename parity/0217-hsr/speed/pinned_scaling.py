"""Final pinned scaling disclosure, retaining the earlier unpinned observations."""
import hashlib,json,os,shutil,statistics,subprocess
from pathlib import Path
receipt=Path.cwd()/'parity/0217-hsr/speed';scratch=Path(os.environ['CARGO_TARGET_DIR'])/'speed'
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
cpu=json.loads((receipt/'performance.json').read_text())['cpu'];assert cpu in os.sched_getaffinity(0);rows=[]
for rep in range(3):
    for n in [32768,262144] if rep!=1 else [262144,32768]:
        out=scratch/('pinned-scale-'+str(n)+'.out')
        p=subprocess.run(['taskset','-c',str(cpu),str(scratch/'after'),'hidden',str(scratch/('scale-'+str(n)+'.bin')),str(out)],input='1\nstop\n',text=True,capture_output=True)
        (receipt/('pinned-scaling-'+str(n)+'-'+str(rep)+'.log')).write_text(p.stdout+p.stderr)
        assert p.returncode==0,p.stderr
        lines=p.stdout.splitlines();report=json.loads(lines[0].split(' ',2)[2]);elapsed=float(next(line.split()[1] for line in lines if line.startswith('COUNTED')))
        rows.append(dict(n=n,rep=rep,seconds=elapsed,work=report['work'],output_sha256=hashlib.sha256(out.read_bytes()).hexdigest()))
summary={}
for n in [32768,262144]:
    group=[r for r in rows if r['n']==n];assert len({r['output_sha256'] for r in group})==1
    summary[str(n)]=dict(seconds=statistics.median(r['seconds'] for r in group),work=group[0]['work'])
summary['work_ratio']=summary['262144']['work']/summary['32768']['work']
summary['wall_ratio']=summary['262144']['seconds']/summary['32768']['seconds']
assert summary['work_ratio']<=10,summary
shutil.copy2(receipt/'scaling.json',receipt/'scaling-unpinned.json')
(receipt/'scaling.json').write_text(json.dumps(dict(summary=summary,rows=rows,cpu=cpu,single_thread=True,pinned=True),indent=2)+'\n')
print('PASS pinned scaling',summary)
