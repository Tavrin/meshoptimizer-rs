#!/usr/bin/env python3
"""Evidence audit, independent of report verdict formatting."""
import json,pathlib,math,os,subprocess,hashlib,statistics
import peer_screen as p
A=p.ART;fail=[]
def check(condition,message):
 if not condition:fail.append(message)
p.source_check()
safety=json.loads((A/'safety-reuse.json').read_text());check(not safety['prior_receipt_mismatches']and not safety['live_source_mismatches']and safety['tree_exact_match'],'reused safety mismatch')
old=A.parent/'campaign-20261025T150110Z'
for rel,h in json.loads((old/'SHA256SUMS.json').read_text()).items():check(p.sha(old/rel)==h,'prior campaign modified: '+rel)
build_receipts=[json.loads(q.read_text())for q in A.rglob('build-*.json')if q.name!='wasm-builds.json']
first_build=min(x['started']for x in build_receipts if 'started'in x)
birth=int(subprocess.check_output(['stat','-c','%W',str(A/'SAFETY.md')],text=True).strip())
check(0<birth<=first_build,'safety file did not exist before first build')
result=json.loads((A/'results.json').read_text());check(result['floor_complete'],'A/A floor incomplete');check(len(result['floor_families'])==3,'single A/A family set missing')
if result['floor_families']:check(result['F']==max(abs(r['gm']-1)for r in result['floor_families']),'F formula')
checks=json.loads((A/'correctness.json').read_text());labels={tuple(r['key']):r for r in checks};samples=0
for path in A.glob('timing-*.json'):
 if '.pending.'in path.name:continue
 _,profile,mode,arm=path.stem.split('-');rows=json.loads(path.read_text())
 for r in rows:
  if not r.get('completed'):continue
  s=r['samples'];samples+=len(s)*2;check(len(s)in[5,20],f'{path.name}: pair count')
  first=p.m.interval([x['ratio']for x in s[:5]]);check(len(s)==(20 if first[0]<=1<=first[1]else 5),f'{path.name}: stopping rule')
  for k,x in enumerate(s):
   check(x['pair']==k and x['order']==(['O',arm]if k%2==0 else[arm,'O']),f'{path.name}: order')
   check(x['ratio']==x['ns'][arm]/x['ns']['O'],f'{path.name}: ratio')
   for b in ['O','O'if arm=='AA'else arm]:
    label=labels[(profile,mode,r['case'],r['operation'],b)];check(label['status']in['identical','different but valid'],'invalid timed');check(label['checked_utc']<x['utc'],'correctness after timing')
  for b,h in r['binary_sha256'].items():check(p.sha(A/f'cmp-driver-{profile}-{b}-{mode}')==h,'changed binary')
  check(p.sha(p.input_path(r['case'],r['operation']))==r['input_sha256'],'changed input')
F=result['F']
if F is not None:
 for r in result['records']:
  if not r.get('equivalent'):expected='NOT EQUIVALENT'
  elif r['ci95'][1]<1-F-0.02:expected='PEER FASTER'
  elif r['ci95'][0]>1+F+0.02:expected='OURS FASTER'
  else:expected='EQUAL WITHIN NOISE'
  check(r['verdict']==expected,'verdict differs from registration')
for profile in ['defaults','moss']:
 root=A/f'browser-{profile}';bp=root/'data/chromium-bytes.json';tp=root/'data/chromium-timing.json'
 if not tp.exists():continue
 data=json.loads(tp.read_text());check(bp.exists(),'browser samples without correctness phase')
 streams=json.loads((root/'data/streams.json').read_text())
 for stream in streams:check(p.sha(root/stream['reference_file'])==stream['reference_sha256'],'changed scalar stream reference')
 for row in data['rows']:
  for arm,x in row['peers'].items():
   check(row['bytes']['O']=='identical' and row['bytes'][arm]=='identical','invalid browser output timed')
   ss=x['samples'];check(len(ss)in[5,20],'browser pair count');ci=p.m.interval([v['ratio']for v in ss[:5]])
   check(len(ss)==(20 if ci[0]<=1<=ci[1] else 5),'browser stopping rule')
   for k,v in enumerate(ss):
    check(v['order']==(['O',arm]if k%2==0 else[arm,'O']),'browser alternating order')
    check(v['ratio']==v['ns'][arm]/v['ns']['O'],'browser ratio')
    check(bp.stat().st_mtime<v['utc'],'browser correctness after sample')
# Registration must exist and include the amendment before every retained sample.
registered=(A/'REGISTRATION.md').stat().st_mtime
for path in A.glob('timing-*.json'):
 if '.pending.'in path.name:continue
 for row in json.loads(path.read_text()):
  for sample in row['samples']:check(registered<sample['utc'],'registration postdates sample')
check(not subprocess.check_output(['git','diff','775ff8cab4fb2ba01966acd1b277aa7019eb8937','--','src/','README.md'],text=True),'library source or README changed')
aa=A/'aa-source'
for path in (p.ROOT/'src').rglob('*.rs'):check(p.sha(path)==p.sha(aa/path.relative_to(p.ROOT)),'A/A source differs')
registration=(A/'REGISTRATION.md').read_text();check('Run 2 amendment registered' in registration,'amendment missing')
# Birth time is retained separately from later append-only confirmations.
stat=subprocess.check_output(['stat','-c','%w %y',str(A/'SAFETY.md')],text=True)
summary={'passed':not fail,'failures':fail,'native_measured_arm_samples':samples,'registration_sha256':p.sha(A/'REGISTRATION.md'),'safety_sha256':p.sha(A/'SAFETY.md'),'safety_birth_and_modified':stat.strip(),'unmeasured_families':len(result['unmeasured'])}
(A/'audit.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
raise SystemExit(0 if not fail else 1)
