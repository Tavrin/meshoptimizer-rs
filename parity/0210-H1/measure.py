
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib,json,os,statistics,subprocess,struct,select,math
S=Path(_public_path('<scratch>/meshopt-0210-H1'));O=S/'meshopt-028-L2';OUT=S/'samples';OUT.mkdir(exist_ok=True)
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
PERF='/usr/lib/linux-tools/6.17.0-22-generic/perf'
ARMS=['base','candidate','M7'];FAMILIES=['cache','cache-fifo']
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def save(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def line(f):
 assert select.select([f],[],[],180)[0], 'protocol timeout'
 return f.readline().strip('\x00\n\r ')
cases=[]
for op in FAMILIES:
 for name in ['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']:
  src=O/'inputs'/f'{name}.bin';path=src
  if op=='simplify-scale':
   raw=src.read_bytes();n=int.from_bytes(raw[:4],'little');packed=raw[8:8+n*12]
   padded=b''.join(packed[j:j+12]+bytes(52) for j in range(0,len(packed),12))
   path=S/f'{name}-scale64.bin'
   path.write_bytes(raw+struct.pack('<III',0x52504c59,64,len(padded))+padded+struct.pack('<IfII',0,0.5,0,struct.unpack('<I',struct.pack('<f',1.05))[0]))
  cases.append(dict(id=f'frozen-{op}-{name}',kind='frozen',family=op,source=str(path),source_sha256=sha(path),args=[op,str(path)],iterations=10000 if name.startswith('tiny') else 4 if name.startswith('million') else 100,multiplicity=1))
# Keep real-fixture metadata distinct; overdraw family summary includes all eight frozen cases.
for trace in json.loads((O/'stages.json').read_text()):
 original=O/f"chain-{trace['case']}.tsv";text=original.read_text().replace(_public_path('<scratch>/meshopt-028-L2/'),str(O)+'/')
 manifest=S/original.name;manifest.write_text(text);lines=text.splitlines();unique={}
 for idx,call in enumerate(trace['calls']):
  if call['operation'] not in ['cache']:continue
  key=(call['operation'],call['vertex_sha256'],call['input_sha256'],tuple(lines[idx+1].split('\t')[2:6]))
  if key in unique:
   unique[key]['multiplicity']+=1;unique[key]['call_indices'].append(idx);continue
  vp=Path(call['vertex_path'].replace(_public_path('<scratch>/meshopt-028-L2/'),str(O)+'/'));assert sha(vp)==call['vertex_sha256']
  if call['input_path']!='-':assert sha(call['input_path'])==call['input_sha256']
  row=dict(id=f"chain-{trace['case'][:4]}-{idx}-{call['operation']}",kind='chain',chain=trace['case'],family=call['operation'],source=str(manifest),source_sha256=sha(manifest),args=['attr-stage',str(manifest)],index=idx,iterations=4,multiplicity=1,call_indices=[idx],expected=call['output_sha256'],vertex_sha256=call['vertex_sha256'],index_sha256=call['input_sha256']);unique[key]=row;cases.append(row)
save(S/'cases.json',cases)
roots={'base':S/'base','candidate':Path(_public_path('<repo>'))}
source={arm:{str(p.relative_to(root)):sha(p) for p in (root/'src').rglob('*.rs')} for arm,root in roots.items()}
for arm in ['base','candidate']:
 build=json.loads((S/f'build-{arm}-binding.json').read_text())
 assert build['sources']==source[arm],(arm,'source differs from built binary')
 assert build['binary_sha256']==sha(S/f'driver-{arm}'),(arm,'binary changed')
save(S/'binding.json',dict(sources=source,binaries={a:sha(S/f'driver-{a}') for a in ARMS},registration=sha(S/'REGISTRATION.md'),cases_sha256=sha(S/'cases.json'),admission=os.environ['MOSS_HEAVY_ACTIVE'],perf=subprocess.check_output([PERF,'--version'],text=True).strip()))
rows=[];summary=[];exits=[]
for case in cases:
 baseline=None
 for rep in range(3):
  order=ARMS[rep:]+ARMS[:rep]
  for arm in order:
   stem=OUT/f"{case['id']}-{rep}-{arm}";assert not stem.with_suffix('.json').exists(),str(stem)
   cr,cw=os.pipe();ar,aw=os.pipe();output=stem.with_suffix('.bin')
   args=case['args']+[str(output)]+([str(case['index'])] if 'index' in case else [])
   cmd=[PERF,'stat','-e','instructions:u','-x',',','-o',str(stem.with_suffix('.perf')),'--delay=-1',f'--control=fd:{cr},{aw}','--','taskset','-c','9',str(S/f'driver-{arm}'),*args]
   with stem.with_suffix('.stderr').open('w') as err:
    p=subprocess.Popen(cmd,pass_fds=(cr,aw),stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=err,text=True,bufsize=1)
    os.close(cr);os.close(aw)
    try:
     ready=line(p.stdout);assert ready=='READY',(case['id'],arm,ready,stem.with_suffix('.stderr').read_text())
     digest=sha(output)
     if arm=='base' and baseline is None:baseline=digest
     if arm!='M7':assert digest==baseline,(case['id'],arm,'output mismatch',digest,baseline)
     if arm!='M7' and 'expected' in case:assert digest==case['expected'],(case['id'],arm,'historical mismatch',digest,case['expected'])
     with os.fdopen(cw,'w',buffering=1) as ctl,os.fdopen(ar,'r',buffering=1) as ack:
      ctl.write('enable\n');assert line(ack)=='ack'
      p.stdin.write(f"count {case['iterations']}\n");p.stdin.flush();assert line(p.stdout)=='COUNTED'
      ctl.write('disable\n');assert line(ack)=='ack'
     p.stdin.write('stop\n');p.stdin.flush();p.communicate(timeout=180);assert p.returncode==0
    except:
     p.kill();p.wait();raise
   exits.append(dict(case=case['id'],arm=arm,rep=rep,exit=p.returncode))
   fields=next(l.split(',') for l in stem.with_suffix('.perf').read_text().splitlines() if ',instructions:u,' in l);n=int(fields[0]);assert float(fields[4])>=99.0,fields
   r=dict(case=case['id'],family=case['family'],arm=arm,rep=rep,iterations=case['iterations'],instructions=n,per_call=n/case['iterations'],output_sha256=digest,cpp_equal=digest==baseline,exit=p.returncode,counter_running_percent=float(fields[4]));rows.append(r);save(stem.with_suffix('.json'),r);save(S/'raw.json',rows);save(S/'measurement-exits.json',exits)
 med={a:statistics.median(r['per_call'] for r in rows if r['case']==case['id'] and r['arm']==a) for a in ARMS};ratio={a:med[a]/med['M7'] for a in ARMS[:-1]};gap=med['base']-med['M7'];shares={a:(med['base']-med[a])/gap if gap else None for a in ['candidate']};summary.append(dict(**case,median=med,ratio=ratio,gap_shares=shares));save(S/'summary.json',summary)
 print(case['id'],{a:round(v,5) for a,v in ratio.items()},flush=True)
assert source=={arm:{str(p.relative_to(root)):sha(p) for p in (root/'src').rglob('*.rs')} for arm,root in roots.items()}
print('COMPLETE',len(cases),len(rows),flush=True)
