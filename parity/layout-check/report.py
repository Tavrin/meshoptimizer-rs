
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import json,math,statistics,hashlib,subprocess,datetime,shutil
A=Path(__file__).parent;R=Path(_public_path('<repo>'));D=R/'parity/layout-check'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def gm(v):return math.exp(statistics.mean(math.log(x) for x in v))
assert (A/'COMPLETE.json').exists()
identity=json.loads((A/'MEASUREMENT-IDENTITY.json').read_text());assert all(sha(Path(p))==h for p,h in identity['files'].items())
ops=['meshlet-bounds','cluster-bounds'];cases=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']
rows=[json.loads(p.read_text()) for p in (A/'timing').glob('*.json') if json.loads(p.read_text()).get('completed')];assert len(rows)==30
for r in rows:
 assert len(r['samples']) in [5,20]
 assert len(r['samples'])==(20 if r['five_pair_ci'][0]<=1<=r['five_pair_ci'][1] else 5)
 assert abs(gm([x['ratio'] for x in r['samples']])-r['gm'])<1e-12
ins=[json.loads(p.read_text()) for p in (A/'instructions').glob('*.json')];assert len(ins)==10
walls={v:{op:gm([r['gm'] for r in rows if r['comparison']==v and r['operation']==op]) for op in ops} for v in ['AA','default','pinned']}
F=max(abs(x-1) for x in walls['AA'].values());T=math.ceil((1+F+.02)*1000)/1000
result=[]
for op in ops:
 i=gm([r['ratio'] for r in ins if r['operation']==op]);default=walls['default'][op];pinned=walls['pinned'][op]
 verdict='REGRESSION' if i>1.01 or (default>T and pinned>T) else 'LAYOUT' if i<=1.01 and default>T and pinned<=T else 'PASS'
 result.append(dict(family=op,instructions=i,default_wall=default,pinned_wall=pinned,verdict=verdict))
outcome='RELEASABLE in next 0.2.x together with the retained new-API result' if all(r['verdict'] in ['PASS','LAYOUT'] for r in result) else 'BOUNDS WAITS FOR NEW CODE'
analysis=dict(F=F,T=T,aa_family_gms=walls['AA'],families=result,outcome=outcome)
(A/'ANALYSIS.json').write_text(json.dumps(analysis,indent=2))
text='# Bounds layout check\n\n'+outcome+'.\n\n| Family | Instructions | Default wall | Pinned wall | Verdict |\n|---|---:|---:|---:|---|\n'
for r in result:text+=f"| {r['family']} | {r['instructions']:.6f} | {r['default_wall']:.6f} | {r['pinned_wall']:.6f} | {r['verdict']} |\n"
text+=f'\nF = {F:.9f}; T = {T:.3f} (ceil to 0.001 of 1 + F + 0.02). A/A family GMs: '+json.dumps(walls['AA'])+'.\n\n'
text+='Candidate a97301e; baseline 775ff8c. Registration and DECISIONS disclosure committed as 42f7374 before samples. Post-fix1 rule revision disclosed; fix1 FAIL remains historical. New API meshlet 1.243711 and cluster 0.993831 against M7 retained from fix1, not remeasured.\n\nCPU 11, five frozen cases, unchanged thin-LTO downstream consumer, alternating five pairs extended to twenty only when five-pair CI contains equality; 25 ms calibration. Case paired-ratio GMs feed equal-weight family GMs. A/A uses baseline archives Bprime/B in different paths in this session. Default flags empty; pinned flag `-C llvm-args=-align-all-functions=6`. Both pinned kernels in both arms are mod64=0; see ALIGNMENT.json and alignment-symbols.txt. Fixed-call instructions:u counters use fix1 controlled regions (10,000 calls, million-smooth 100). Byte equality checked before calibration in all 30 comparisons.\n\nPreflight encoded-flags override was caught before samples; preflight artifacts retained and pinned arms rebuilt with the same registered flag. See BUILD-PREFLIGHT.json. The first generic-Python admission was cancelled while queued, before payload/samples; it was resubmitted through a dedicated shell entry point without an admission override or declaration change (ADMISSION-PREFLIGHT.json). No time-value reruns. Infrastructure retry used: '+str((A/'INFRA_RETRY.json').exists())+'. Train-first checks and any PAUSE are in CAMPAIGN.jsonl; no waits on other meshopt lanes.\n\nRegistered decision applied verbatim: REGRESSION if instruction GM >1.01 or both wall GMs >T; LAYOUT if instructions <=1.01, default >T, pinned <=T; PASS otherwise.\n\nReceipts: '+str(A)+'; raw timing/calibration/load and instruction stat files retained here and under parity/layout-check. Source/binary/input epoch hashes in MEASUREMENT-IDENTITY.json. Full linker maps and executables retained in scratch. No library source changes, new API/fetch-remap remeasurement, release/tag/version action, GPU or production acceptance.\n'
(A/'REPORT.md').write_text(text)
for p in A.iterdir():
 if p.is_file() and p.suffix in ['.py','.sh','.json','.jsonl','.md','.txt','.log']:shutil.copy2(p,D/p.name)
for name in ['timing','instructions']:
 dest=D/name;dest.mkdir(exist_ok=True)
 for p in (A/name).iterdir():
  if p.suffix in ['.json','.stat']:shutil.copy2(p,dest/p.name)
print(json.dumps(analysis,indent=2))
