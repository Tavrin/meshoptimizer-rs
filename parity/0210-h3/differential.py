from pathlib import Path
import importlib.util, os, subprocess, json, hashlib, sys
R=Path(__file__).resolve().parents[2];S=Path('/mnt/linux-extra/moss-scratch/meshopt-0210-h3');REF=Path('/home/etienne/dev/refs/meshoptimizer');T=Path(os.environ['CARGO_TARGET_DIR'])
os.environ['MESHOPT_REFERENCE']=str(REF)
spec=importlib.util.spec_from_file_location('p03',R/'parity/p03/runner.py');p03=importlib.util.module_from_spec(spec);spec.loader.exec_module(p03)
subprocess.run([str(R/'parity/check-reference.sh')],check=True)
sources=[REF/f'src/{n}.cpp' for n in ['clusterizer','meshletutils','spatialorder','partition','allocator','simplifier','indexgenerator']]
subprocess.run(['c++',*p03.FLAGS,'-I',str(REF/'src'),'-I',str(REF/'demo'),str(R/'parity/p03/reference.cpp'),*map(str,sources),'-o',str(S/'cpp-p03')],check=True)
counts={}
for mode,flags in [('nostd',[]),('std',['--features','meshoptimizer-rs/std'])]:
 subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(R/'parity/p03/Cargo.toml'),*flags],check=True)
 dst=S/f'p03-{mode}';subprocess.run(['cp',str(T/'release/meshopt-p03-driver'),str(dst)],check=True)
 clients={n:p03.Client(n,p) for n,p in [('cpp',S/'cpp-p03'),('rust',dst)]}
 try:
  n=0
  for op in [4,12]:
   for seed in range(20261004,20263004):
    data=p03.case(op,seed)
    # Exact upstream output bytes; exclude the legacy driver's timer field.
    a=clients['cpp'].run(data)[0];b=clients['rust'].run(data)[0]
    if a!=b:
     (S/f'cpp-fail-{mode}-{op}-{seed}.input').write_bytes(data)
     raise AssertionError((mode,op,seed,len(a),len(b)))
    n+=1
  counts[mode]=n
 finally:
  for c in clients.values():c.close()
(S/'differential.json').write_text(json.dumps(dict(counts=counts,reference=subprocess.check_output(['git','-C',str(REF),'rev-parse','HEAD'],text=True).strip(),exit=0),indent=2)+'\n')
print(counts)
