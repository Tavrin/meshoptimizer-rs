#!/usr/bin/env python3
"""One final paired stream, 5-20 pairs; symmetric A/A resolution."""
import sys,json,struct,math,statistics,time,os,hashlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent));import run as lane
A=lane.ART
from peer_validation import exp_separate_zero_fields
T95={5:2.776,6:2.571,7:2.447,8:2.365,9:2.306,10:2.262,11:2.228,12:2.201,13:2.179,14:2.160,15:2.145,16:2.131,17:2.120,18:2.110,19:2.101,20:2.093}
def interval(xs):
 logs=[math.log(x) for x in xs];mean=statistics.mean(logs);r=T95[len(xs)]*statistics.stdev(logs)/math.sqrt(len(xs));return [math.exp(mean-r),math.exp(mean+r)]
def main():
 assert os.environ.get('MOSS_HEAVY_ACTIVE')
 manifest=json.loads((A/'build-after.json').read_text());assert manifest['source']=={str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'src').rglob('*.rs')}
 consumer={str(p.relative_to(lane.ROOT)):lane.sha(p) for p in (lane.ROOT/'parity/codec').rglob('*') if p.is_file() and '__pycache__' not in p.parts}
 assert consumer==json.loads((A/'build-before-fair.json').read_text())['consumer']
 assert lane.sha(lane.ROOT/'parity/encoders/src/main.rs')==json.loads((A/'build-crate-fair.json').read_text())['consumer']
 policy={str(p.relative_to(lane.ROOT)):lane.sha(p) for p in [Path(__file__),Path(lane.__file__),lane.ROOT/'parity/encoders/reference.cpp',lane.ROOT/'parity/encoders/peer_validation.py']}
 inputs_hash=lane.sha(A/'inputs.json')
 path=A/'timing.json';identities={k:lane.sha(A/k) for k in ['rust-before-fair','rust-after','cpp','crate']};start=time.monotonic()
 assert identities['rust-after']==manifest['binary']
 assert identities['rust-before-fair']==json.loads((A/'build-before-fair.json').read_text())['binary']
 assert identities['crate']==json.loads((A/'build-crate-fair.json').read_text())['binary']
 if path.exists():
  result=json.loads(path.read_text());assert not result['completed'],'completed stream is immutable';assert result['binaries']==identities;assert result['consumer']==consumer and result['inputs_hash']==inputs_hash
  if result['policy']!=policy:
   amendment=json.loads((A/'timing-validation-amendment.json').read_text())
   assert amendment['old_policy']==result['policy'] and amendment['new_policy']==policy
   assert amendment['prefix_rows']==len(result['rows'])
   assert amendment['prefix_sha256']==hashlib.sha256(json.dumps(result['rows'],sort_keys=True,separators=(',',':')).encode()).hexdigest()
   assert amendment['binaries']==identities and amendment['inputs_hash']==inputs_hash and amendment['consumer']==consumer
   result['validation_amendments']=[{'receipt_sha256':lane.sha(A/'timing-validation-amendment.json'),'prefix_rows':amendment['prefix_rows'],'reason':amendment['reason']}]
   result['policy']=policy
 else:result={'binaries':identities,'source':manifest['source'],'consumer':consumer,'policy':policy,'inputs_hash':inputs_hash,'cpu':26,'target_seconds':.008,'resolution_rule':'Rust A/A and C++ A/A 95% interval inside [0.8,1.25]','rows':[],'completed':False,'bursts':[]}
 ds={k:lane.Driver(A/v) for k,v in {'before':'rust-before-fair','after':'rust-after','cpp':'cpp','crate':'crate','rust-aa':'rust-after','cpp-aa':'cpp'}.items()}
 cases=json.loads((A/'inputs.json').read_text());total=len(cases)*2
 for j in range(len(result['rows']),total):
  row=cases[j//2];api=j%2;b=bytearray(Path(row['path']).read_bytes());assert hashlib.sha256(b).hexdigest()==row['sha256']
  if api:struct.pack_into('<I',b,16,struct.unpack_from('<I',b,16)[0]|128)
  outputs={k:d.call(b)[:2] for k,d in ds.items()};assert outputs['before']==outputs['after']==outputs['cpp'],row['name'];assert outputs['after'][0]==0
  # The old crate is independently checked for byte equality or lossless decode.
  st,crateout=outputs['crate'];assert st==0
  op=struct.unpack_from('<I',b,4)[0];n,s=struct.unpack_from('<II',b,8)
  if op==11:
   dec=lane.request(1,n,s,crateout);assert ds['cpp'].call(dec)[:2]==(0,bytes(b[44:])),row['name']
  elif op==16 and struct.unpack_from('<I',b,16)[0]&127==0:
   exp_separate_zero_fields(b[44:],struct.unpack_from('<I',b,28)[0],outputs['cpp'][1],crateout)
  elif op!=1:assert crateout==outputs['cpp'][1],(row['name'],'crate bytes')
  else:assert crateout==outputs['cpp'][1]
  probe=bytearray(b);struct.pack_into('<II',probe,32,1,1);trial=ds['cpp'].call(probe)[2][0];it=max(1,min(1000000,math.ceil(.008/max(trial,1e-9))));struct.pack_into('<I',probe,36,it)
  samples={k:[] for k in ds};i=0
  while i<20:
   keys=list(ds);keys=keys[i%len(keys):]+keys[:i%len(keys)]
   if i%2:keys=keys[::-1]
   for k in keys:
    st,out,ts=ds[k].call(probe);assert (st,out)==outputs[k];samples[k].append(ts[0])
   i+=1
   if i>=5:
    ci=interval([x/y for x,y in zip(samples['after'],samples['cpp'])]);aa=interval([x/y for x,y in zip(samples['rust-aa'],samples['after'])]);ca=interval([x/y for x,y in zip(samples['cpp-aa'],samples['cpp'])])
    if (ci[1]<=1.5 or ci[0]>1.5) and aa[0]>=.8 and aa[1]<=1.25 and ca[0]>=.8 and ca[1]<=1.25:break
  ratios={k:[x/y for x,y in zip(samples[k],samples['cpp'])] for k in ['before','after']}
  cr={k:[x/y for x,y in zip(samples[k],samples['crate'])] for k in ['before','after']}
  out={'case':row['name'],'api':'caller' if api else 'allocating','pairs':i,'iterations':it,'samples':samples,'before_cpp':statistics.median(ratios['before']),'after_cpp':statistics.median(ratios['after']),'before_crate':statistics.median(cr['before']),'after_crate':statistics.median(cr['after']),'after_cpp_ci':interval(ratios['after']),'rust_aa_ci':aa,'cpp_aa_ci':ca,'resolvable':aa[0]>=.8 and aa[1]<=1.25 and ca[0]>=.8 and ca[1]<=1.25,'crate_equal':outputs['crate']==outputs['cpp'],'output_sha256':hashlib.sha256(outputs['cpp'][1]).hexdigest(),'load':os.getloadavg()}
  result['rows'].append(out);lane.save('timing',result);print(j+1,total,row['name'],out['api'],'ratio',round(out['after_cpp'],3),'resolves',out['resolvable'],flush=True)
  if time.monotonic()-start>690:break
 for d in ds.values():d.close()
 result['completed']=len(result['rows'])==total;result['bursts'].append({'seconds':time.monotonic()-start,'admission':os.environ['MOSS_HEAVY_ACTIVE'],'load':os.getloadavg()});lane.save('timing',result)
 if result['completed']:
  groups={}
  for api in ['allocating','caller']:
   for family in ['vertex','index','sequence','oct','quat','exp']:
    rows=[r for r in result['rows'] if r['api']==api and r['case'].startswith(family+'-') and r['case']!='vertex-v1-streaming-s4'];resolved=[r for r in rows if r['resolvable']]
    groups[api+'/'+family]={key:statistics.geometric_mean(r[key] for r in resolved) if resolved else None for key in ['before_cpp','after_cpp','before_crate','after_crate']}
    groups[api+'/'+family].update(cases=len(rows),resolved=len(resolved),maximum=max([r['after_cpp'] for r in resolved],default=None),max_upper95=max([r['after_cpp_ci'][1] for r in resolved],default=None),pass_bar=bool(resolved) and groups[api+'/'+family]['after_cpp']<=1.25 and all(r['after_cpp_ci'][1]<=1.5 for r in resolved))
  lane.save('timing-summary',groups)
 print('completed',result['completed'],flush=True)
if __name__=='__main__':main()
