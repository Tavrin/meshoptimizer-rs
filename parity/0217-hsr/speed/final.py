"""Final admitted checks after the paired campaign; preserves every exit code."""
import hashlib,json,os,subprocess,time
from pathlib import Path
root=Path.cwd();receipt=root/'parity/0217-hsr/speed';target=Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ['MOSS_HEAVY_GPU']=='0'
os.environ.update(CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',LC_ALL='C',GIT_OPTIONAL_LOCKS='0')
def run(name,args):
    start=time.monotonic();p=subprocess.run(args,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    (receipt/(name+'.log')).write_text(p.stdout.replace(str(root),'<worktree>').replace(str(target),'<target>'))
    with (receipt/'exits.tsv').open('a') as f:f.write(f'{name}\t{p.returncode}\t{time.monotonic()-start:.3f}\n')
    print(name,p.returncode,p.stdout[-1600:],flush=True)
    if p.returncode:raise SystemExit(p.returncode)
run('final-format',['cargo','fmt','--all'])
run('final-clippy-all',['cargo','clippy','--locked','--all-features','--all-targets','--','-D','warnings'])
run('final-clippy-none',['cargo','clippy','--locked','--no-default-features','--all-targets','--','-D','warnings'])
run('final-hidden-all',['cargo','test','--locked','--release','--all-features','--test','hidden','--','--nocapture'])
run('final-hidden-none',['cargo','test','--locked','--release','--no-default-features','--test','hidden','--','--nocapture'])
run('final-pinned-scaling',['python3',str(receipt/'pinned_scaling.py')])
run('final-amortized-scaling',['python3',str(receipt/'amortized_scaling.py')])
run('final-proof-binding',['python3',str(receipt/'finish.py')])
run('final-format-check',['cargo','fmt','--all','--','--check'])
run('final-whitespace',['git','--no-optional-locks','diff','--check'])
run('final-public-text',['bash','parity/tools/check_public_text.sh'])
print('PASS final gates; target cleanup waits until this admitted job exits',flush=True)
