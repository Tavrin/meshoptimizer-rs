from pathlib import Path
import json
A=Path(__file__).parent;rows=[];raw=[]
for arm in ['B','C']:
 for variant in ['default','pinned']:
  lines=(A/f'{arm}-{variant}.map').read_text().splitlines()
  for kernel in ['compute_meshlet_bounds','compute_cluster_bounds']:
   matches=[l for l in lines if l.strip().endswith('meshoptimizer_rs::meshlet_util::'+kernel)]
   assert len(matches)==1,(arm,variant,kernel,matches)
   line=matches[0];addr=int(line.split()[0],16);raw.append(f'{arm}-{variant}: {line}')
   rows.append(dict(arm=arm,variant=variant,kernel=kernel,address=hex(addr),mod64=addr%64))
   if variant=='pinned':assert addr%64==0,rows[-1]
(A/'alignment-symbols.txt').write_text('\n'.join(raw)+'\n')
(A/'ALIGNMENT.json').write_text(json.dumps(dict(flag='-C llvm-args=-align-all-functions=6',rows=rows,passed=True),indent=2))
