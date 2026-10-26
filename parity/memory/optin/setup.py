from pathlib import Path
import subprocess,tarfile,io,shutil,json,hashlib
s=Path(__file__).parent;old=s.parent;repo=Path('/home/etienne/dev/meshoptimizer-rs-wt/memory')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
base=subprocess.check_output(['git','rev-parse','v0.2.6^{}'],text=True).strip()
with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['git','archive',base]))) as ar:ar.extractall(s/'library-A',filter='data')
(s/'library-B').mkdir()
for name in ['Cargo.toml','Cargo.lock','src']:
 p=repo/name
 if p.is_dir():shutil.copytree(p,s/'library-B'/name)
 else:shutil.copy2(p,s/'library-B'/name)
(s/'SOURCE.json').write_text(json.dumps({'baseline':base,'candidate_parent':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'files':{a:{str(p.relative_to(s/('library-'+a))):sha(p) for p in (s/('library-'+a)).rglob('*') if p.is_file()} for a in ['A','B']}},indent=2)+'\n')
shutil.copy2(old/'spec027/corpus.toml',s/'corpus.toml')
for arm in ['A','B']:
 h=s/('harness-'+arm);shutil.copytree(old/'spec027/timed-harness',h)
 p=h/'Cargo.toml';t=p.read_text();import re
t=''
for arm in ['A','B']:
 h=s/('harness-'+arm);p=h/'Cargo.toml';t=p.read_text();t=re.sub(r'meshoptimizer-rs = \{ path = "[^"]+"',f'meshoptimizer-rs = {{ path = "{s}/library-{arm}"',t);p.write_text(t)
 if arm=='B':
  p=h/'src/backend.rs';t=p.read_text();t=t.replace('workspace: meshoptimizer_rs::Workspace::new(meshoptimizer_rs::Limits {','workspace: { let mut ws = meshoptimizer_rs::Workspace::new(meshoptimizer_rs::Limits {',1);t=t.replace('''                max_work: 1 << 34,
            }),''','''                max_work: 1 << 34,
            });
            if std::env::var_os("MESHOPT_LEAN").is_some() {ws.set_retention(meshoptimizer_rs::Retention::Lean);}
            ws },''',1);p.write_text(t)
  p=h/'src/main.rs';t=p.read_text().replace(' let mut ws=Workspace::default();',' let mut ws=Workspace::default();\n if std::env::var_os("MESHOPT_LEAN").is_some(){ws.set_retention(meshoptimizer_rs::Retention::Lean);}') ;p.write_text(t)
 # Pure capacities probe is separate from the perf/RSS build.
 lib=s/('probe-library-'+arm);shutil.copytree(s/('library-'+arm),lib)
 original=(old/'library-probe.patch').read_text();diag=original[original.index('+impl Workspace'):];diag='\n'.join(l[1:] for l in diag.splitlines() if l.startswith('+'))+'\n'
 p=lib/'src/workspace.rs';p.write_text(p.read_text()+'\n'+diag)
 ph=s/('probe-harness-'+arm);shutil.copytree(h,ph);p=ph/'Cargo.toml';p.write_text(p.read_text().replace(str(s/('library-'+arm)),str(lib)))
 p=ph/'src/backend.rs';p.write_text(p.read_text().replace('let detail=json!({"usage_bytes":','let detail=json!({"retained":self.workspace.diag(),"usage_bytes":'))
(s/'bin').mkdir()
