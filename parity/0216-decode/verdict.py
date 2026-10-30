"""Apply the fixed gate; on rejection retain the measured unchanged-x86 control."""
import hashlib,json,math,os,shutil,statistics,subprocess
from pathlib import Path
root=Path.cwd();art=Path(os.environ['DECODE_ART']);r=json.loads((art/'results.json').read_text());rows=r['rows']
shutil.copy2(art/'results.json',art/'tuning-results.json');shutil.copy2(root/'parity/0216-decode/TABLE.md',root/'parity/0216-decode/TUNING.md')
g=math.exp(statistics.mean(math.log(x['candidate_base_ratio']) for x in rows if x['op']=='vertex-decode' and x['case'].startswith('real-')))
worst=max(x['candidate_base_ratio'] for x in rows)
control=max(x['instructions']['control']/x['instructions']['base'] for x in rows)
passed=g<=.97 and worst<=1.01 and control<=1.01
selected='candidate' if passed else 'control'
if not passed:
    p=root/'src/codec/simd/x86.rs';s=p.read_text().replace('        (true, 12) => vertex_kernel::<128, 12, D>,\n        (false, 12) => vertex_kernel::<1024, 12, D>,\n','');p.write_text(s)
binding=json.loads((art/'build-binding.json').read_text())
for name,digest in binding['sources'][selected].items():
    if name.startswith('src/'):
        assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest,(selected,name)
record=dict(selected_arm=selected,tuning_verdict='SHIPS' if passed else 'REJECTED',real_vertex_geomean=g,worst_case=worst,control_worst_case=control,gate='geomean <=0.97, every decode case <=1.01; unchanged-x86 control <=1.01')
(art.parent/'final-verdict.json').write_text(json.dumps(record,indent=2)+'\n');print(record,flush=True)
subprocess.run(['python3','parity/0216-decode/results.py'],env=dict(os.environ,DECODE_FINAL_ARM=selected),check=True)
