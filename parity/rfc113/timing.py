#!/usr/bin/env python3
"""One final matrix with owner-authorized adaptive sampling and pair-boundary admission."""
import datetime,hashlib,json,math,os,statistics,subprocess,sys,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];ART=Path(os.environ.get('MESHOPT_RFC113_ART_DIR','/mnt/linux-extra/meshopt-artifacts/clodrec'));TARGET=Path(os.environ.get('CARGO_TARGET_DIR','/mnt/linux-extra/moss-cargo-targets/codex-meshopt-clodrec'))
ART.mkdir(parents=True,exist_ok=True)
os.environ['CARGO_TARGET_DIR']=str(TARGET);sys.path.insert(0,str(ROOT/'parity/rfc113'))
import run as clod
HEAD=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip();LEASE=Path.home()/'Documents/automation_game/assets_toolings/Moss/scripts/gpu-lease.sh'
POLICY='owner-adaptive: 5 to 20 stage-1 pairs, only borderline cases receive D146 stage 2; no load threshold; no GPU holder or scoreboard measurement'
T={4:2.776445,5:2.570582,6:2.446912,7:2.364624,8:2.306004,9:2.262157,10:2.228139,11:2.200985,12:2.178813,13:2.160369,14:2.144787,15:2.131450,16:2.119905,17:2.109816,18:2.100922,19:2.093024,29:2.045230}
profile=None;case=None
LOG=ART/'timing-early-admission.jsonl'
def record(event,**fields):
 row=dict(event=event,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commit=HEAD,profile=profile,case=case,policy=POLICY,**fields)
 with LOG.open('a') as f:f.write(json.dumps(row)+'\n')
 return row

def proc(pid):
 p=Path('/proc')/str(pid)
 try:return dict(pid=pid,name=(p/'comm').read_text().strip(),cmdline=(p/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace'),children=[int(v) for v in (p/'task'/str(pid)/'children').read_text().split()])
 except OSError:return None

def gate():
 lease=subprocess.run([str(LEASE),'status'],capture_output=True,text=True)
 units=subprocess.run(['systemctl','--user','list-units','moss-scoreboard-*','--state=active','--no-legend','--plain'],capture_output=True,text=True);details=[]
 clear=units.returncode==0
 for line in units.stdout.splitlines():
  if not line.strip():continue
  unit=line.split()[0];pid=subprocess.run(['systemctl','--user','show',unit,'--property=MainPID','--value'],capture_output=True,text=True)
  parent=proc(int(pid.stdout.strip())) if pid.returncode==0 and pid.stdout.strip().isdigit() else None
  children=[proc(p) for p in parent['children']] if parent else []
  waiting=bool(parent and len(children)==1 and children[0] and children[0]['name']=='sleep' and not children[0]['children'])
  details.append(dict(unit=unit,main=parent,children=children,waiting=waiting));clear=clear and waiting
 admitted=lease.returncode==0 and 'GPU lease: FREE' in lease.stdout and clear
 record('admission',admitted=admitted,lease=lease.stdout.strip(),lease_returncode=lease.returncode,scoreboard_units=details,load_average=os.getloadavg())
 return admitted

def idle_wait(seconds=300):
 until=time.monotonic()+seconds
 while time.monotonic()<until:
  time.sleep(min(50,until-time.monotonic()));print('HEARTBEAT waiting with drivers closed',flush=True)

def choose_core():
 before=clod.cpu_snapshot();time.sleep(1);after=clod.cpu_snapshot();allowed=os.sched_getaffinity(0);groups={}
 for cpu in after:
  p=Path(f'/sys/devices/system/cpu/cpu{cpu}/topology');key=(int((p/'physical_package_id').read_text()),int((p/'core_id').read_text()));groups.setdefault(key,[]).append(cpu)
 excluded={int(v) for v in os.environ.get('MESHOPT_RFC113_EXCLUDE_CORES','0,1').split(',') if v}
 candidates={key:cpus for key,cpus in groups.items() if set(cpus)&allowed and not set(cpus)&excluded}
 util={c:clod.utilization(before,after,c) for c in after};key=min(candidates,key=lambda k:(max(util[c] for c in candidates[k]),sum(util[c] for c in candidates[k]),k));core=min(set(candidates[key])&allowed,key=lambda c:(util[c],c))
 selection=dict(core=core,physical_package=key[0],physical_core=key[1],siblings=candidates[key],pre_run_utilization=util[core],sibling_utilization={str(c):util[c] for c in candidates[key]},candidate_physical_utilization={str(k):{str(c):util[c] for c in v} for k,v in candidates.items()})
 record('core_selection',**selection);return selection

def interval(samples):
 logs=[math.log(p['rust']/p['cpp']) for p in samples];n=len(logs);mean=statistics.mean(logs);sd=statistics.stdev(logs);half=T[n-1]*sd/math.sqrt(n)
 lower,upper=math.exp(mean-half),math.exp(mean+half)
 return dict(pairs=n,log_mean=mean,log_sample_sd=sd,lower=lower,upper=upper,nominal_confidence=0.95,verdict='PASS' if upper<=1.5 else 'FAIL' if lower>1.5 else 'BORDERLINE')

if __name__ == '__main__':
 import argparse
 parser=argparse.ArgumentParser()
 parser.add_argument('--case',action='append',default=[])
 parser.add_argument('--profile',action='append',choices=('consumer','release'))
 parser.add_argument('--pairs',type=int,help='fixed diagnostic pair count; final defaults to adaptive 5-20')
 args=parser.parse_args()
 if args.pairs is not None and args.pairs < 5:parser.error('--pairs must be at least 5')
 sources={str(p):clod.sha(p) for p in [ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'parity/rfc113/Cargo.toml',ROOT/'parity/rfc113/Cargo.lock',*sorted((ROOT/'src').rglob('*.rs')),*sorted((ROOT/'parity/rfc113').glob('*.py')),ROOT/'parity/rfc113/main.rs',ROOT/'parity/rfc113/reference.cpp',ROOT/'parity/rfc113/bridge.cpp',clod.VENDOR/'clusterlod.h',*sorted((clod.VENDOR/'src').glob('*.cpp'))]}
 base={'identity':{'hardware':clod.platform.machine(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cxx':subprocess.check_output(['c++','--version'],text=True),'cpp_flags':'-O3 -DNDEBUG -DMESHOPTIMIZER_NO_SIMD -fno-fast-math -ffp-contract=off','moss_cpp_flags':'-O3 -DNDEBUG -fPIC -ffunction-sections -fdata-sections -m64'}}
 record('method_start',diagnostic_pairs=args.pairs)
 for profile in (args.profile or ('consumer','release')):
  built_paths,built_sources=clod.build(profile)
  assert all(clod.sha(p)==h for p,h in built_sources.items())
  assert all(clod.sha(p)==h for p,h in sources.items())
  paths={'cpp':TARGET/'rfc113-cpp','moss_cpp':TARGET/'rfc113-moss-cpp','rust':TARGET/profile/'rfc113-rust'};binary_hashes={k:clod.sha(v) for k,v in paths.items()}
  profile_load=os.getloadavg();records=[];all_bursts=[]
  for mesh in sorted(clod.MESHES.glob('*.mesh')):
   for stride in ([32,48,64,72,88] if mesh.stem in ('pyramid','sponza_lionhead') else [48]):
    case=f'{mesh.stem}:{stride}'
    if args.case and case not in args.case and mesh.stem not in args.case:continue
    payload,meta=clod.prepare(mesh,stride)
    if stride==32:
     records.append(dict(case=case,**meta,mismatch={'field':'invalid S2 protect mask bit 8'},times_s={side:[] for side in paths},median_ratio=None,moss_ratio=None));continue
    pool=None;burst=None;bursts=[];selection=None;stage2_active=False;stage2_fixed_selection=None
    def close_pool():
     global pool,burst
     if pool:
      for d in pool.values():d.close()
      end=clod.cpu_snapshot();burst['load_after']=os.getloadavg();burst['duration_s']=time.monotonic()-burst.pop('_start');snap=burst.pop('_snapshot');burst['sibling_utilization']={str(c):clod.utilization(snap,end,c) for c in burst['selection']['siblings']};bursts.append(burst);all_bursts.append(burst);record('core_release',burst=burst);pool=None;burst=None
    def ready():
     global pool,burst,selection,stage2_fixed_selection
     if burst and time.monotonic()-burst['_start']>=840:
      close_pool();record('burst_cooldown');idle_wait(60)
     while not gate():
      close_pool();print('PAUSED',profile,case,'drivers closed',flush=True);idle_wait()
     if pool is None:
      selection=stage2_fixed_selection if stage2_active and stage2_fixed_selection is not None else choose_core()
      if stage2_active:stage2_fixed_selection=selection
      pool={k:clod.Driver(v,selection['core']) for k,v in paths.items()};burst=dict(selection=selection,load_before=os.getloadavg(),_start=time.monotonic(),_snapshot=clod.cpu_snapshot())
    try:
     ready();outputs={side:pool[side].run(b'R11B'+payload[4:])[0] for side in paths};cpp=outputs['cpp'];mismatch=clod.first_mismatch(cpp,outputs['moss_cpp'],outputs['rust'],f'python3 parity/rfc113/run.py --case {case} --pairs 0')
     if mismatch:raise RuntimeError(f'parity failed: {case}: {mismatch}')
     pairs=[];stage2=[];timed_outputs={};decision=None
     def pair(index,stage):
      ready();record('pair_start',stage=stage,pair=index+1,core=selection['core']);row={}
      for side in (('cpp','moss_cpp','rust') if index%2==0 else ('rust','moss_cpp','cpp')):
       output,seconds=pool[side].run(b'R11T'+payload[4:]);row[side]=seconds
       if side in timed_outputs and timed_outputs[side]!=output:raise RuntimeError('nondeterministic timing output')
       timed_outputs[side]=output
      assert timed_outputs['cpp']==timed_outputs['moss_cpp']==timed_outputs['rust']
      record('pair_complete',stage=stage,pair=index+1,core=selection['core'],times_s=row);return row
     if stride==48:
      for index in range(args.pairs or 20):
       pairs.append(pair(index,1))
       if len(pairs)>=5:
        decision=interval(pairs);record('stage1_look',decision=decision)
        if args.pairs is None and decision['verdict']!='BORDERLINE':break
      if args.pairs is None and decision['verdict']=='BORDERLINE':
       close_pool();stage2_active=True;record('stage2_start',reason='stage-1 interval still overlaps 1.5 at 20 pairs')
       for index in range(30):stage2.append(pair(index,2))
       stage2_decision=interval(stage2)
       if stage2_decision['verdict']=='BORDERLINE':stage2_decision['verdict']='INCONCLUSIVE'
      else:stage2_decision=None
     else:stage2_decision=None
     close_pool();times={side:[p[side] for p in pairs] for side in paths}
     ratio=statistics.median(times['rust'])/statistics.median(times['cpp']) if pairs else None;moss_ratio=statistics.median(times['rust'])/statistics.median(times['moss_cpp']) if pairs else None
     records.append(dict(case=case,mesh_sha256=clod.sha(mesh),**meta,cpp_sha256=hashlib.sha256(cpp).hexdigest(),moss_cpp_sha256=hashlib.sha256(outputs['moss_cpp']).hexdigest(),moss_cpp_matches_scalar=True,rust_sha256=hashlib.sha256(outputs['rust']).hexdigest(),bytes=len(cpp),group_count=clod.struct.unpack_from('<I',cpp,32)[0],dag_depth=clod.struct.unpack_from('<I',cpp,40)[0]-1,mismatch=None,times_s=times,median_ratio=ratio,moss_ratio=moss_ratio,early_stopping=decision,stage2_pairs=stage2,stage2_decision=stage2_decision,measurement_bursts=bursts))
     print(profile,case,'MATCH','pairs',len(pairs),'ratio',ratio,'decision',decision,flush=True)
    finally:close_pool()
  total={side:sum(statistics.median(r['times_s'][side]) for r in records if r['times_s'][side]) for side in paths}
  identity=dict(base['identity'],sources={**built_sources,**sources},executables=binary_hashes,commit=HEAD,core='per burst; see cases[].measurement_bursts',core_selection=all_bursts[0]['selection'],load_before=profile_load,load_after=os.getloadavg(),core_utilization=sum(b['sibling_utilization'][str(b['selection']['core'])]*b['duration_s'] for b in all_bursts)/sum(b['duration_s'] for b in all_bursts),rust_profile=('consumer opt-level=3 thin LTO codegen-units=1' if profile=='consumer' else 'Cargo release defaults: opt-level=3, 16 codegen units, no LTO'),admission_policy=POLICY,sampling_method='stage 1: start 5, inspect nominal paired-log 95% t interval after each pair, stop when wholly within/over 1.5, cap 20; only unresolved case receives 30 fresh D146 pairs; aggregate retains stage-1 medians',burst_limit_s=840)
  assert sources=={p:clod.sha(p) for p in sources};assert binary_hashes=={side:clod.sha(path) for side,path in paths.items()}
  result=dict(schema='rfc113-clod/3',cases=records,timing=dict(cpp_sum_medians_s=total['cpp'],moss_cpp_sum_medians_s=total['moss_cpp'],rust_sum_medians_s=total['rust'],ratio=total['rust']/total['cpp'],moss_ratio=total['rust']/total['moss_cpp'],bar='1.2x aggregate, 1.5x per mesh'),identity=identity)
  (ART/('diagnostic-'+profile+'.json' if args.case or args.pairs else ('result.json' if profile=='consumer' else 'result-defaults.json'))).write_text(json.dumps(result,indent=2)+'\n');record('profile_complete');print('PROFILE_COMPLETE',profile,flush=True)
 print('TIMING_COMPLETE',flush=True)
