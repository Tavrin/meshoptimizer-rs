from pathlib import Path
import subprocess,json,os,sys
A=Path(__file__).parent;case,op=sys.argv[1:];out=A/'instructions';out.mkdir(exist_ok=True)
perf='/usr/lib/linux-hwe-6.17-tools-6.17.0-22/perf';counts={}
for arm in ['C','B']:
 reps=100 if case=='million-smooth' else 10000
 path=out/f'{case}-{op}-{arm}.stat';cr,cw=os.pipe();ar,aw=os.pipe()
 r=subprocess.run([perf,'stat','-x',';','-e','instructions:u','-D','-1','--control',f'fd:{cr},{aw}','-o',str(path),'--','taskset','-c','11',str(A/f'probe-{arm}'),op,str(A.parent/'inputs'/f'{case}.bin'),str(reps)],env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=180)
 for fd in [cr,cw,ar,aw]:os.close(fd)
 assert r.returncode==0,r.stderr
 counts[arm]=next(int(s.split(';')[0]) for s in path.read_text().splitlines() if ';instructions:u;' in s)/reps
(out/f'{case}-{op}.json').write_text(json.dumps(dict(case=case,operation=op+'-bounds',calls=reps,counts=counts,ratio=counts['C']/counts['B']),indent=2))
