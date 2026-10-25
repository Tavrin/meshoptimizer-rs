from pathlib import Path
import subprocess,re,struct,json,hashlib,sys,zipfile,os
import capstone as cs
from capstone.x86_const import X86_OP_IMM,X86_OP_MEM,X86_REG_RIP
s=Path(__file__).parent;arm=sys.argv[1] if len(sys.argv)>1 else 'C'
records=sorted(s.glob(f'{arm}-attempt-*/record.json'));d=json.loads(records[-1].read_text());assert d['completed']
os.sched_setaffinity(0,{8});results={};frozen=json.loads((s.parent/'r2/frozen-final-1/benchmark.json').read_text())
# Parse symbols and ELF load segments. Mask only relocation-dependent RIP and external branch displacements.
def binary(a):
 p=s/f'driver-{a}';data=p.read_bytes();phoff=struct.unpack_from('<Q',data,32)[0];ents,num=struct.unpack_from('<HH',data,54);segments=[]
 for k in range(num):
  typ,flags,off,addr,_,filesz,memsz,align=struct.unpack_from('<IIQQQQQQ',data,phoff+k*ents)
  if typ==1:segments.append((addr,off,filesz))
 def read(addr,n):
  for x,o,z in segments:
   if x<=addr and addr+n<=x+z:return data[o+addr-x:o+addr-x+n]
  raise ValueError(hex(addr))
 raw=subprocess.check_output(['nm','-SC',str(p)],text=True);(s/f'{a}-layout-symbols.txt').write_text(raw);syms={}
 for line in raw.splitlines():
  m=re.match(r'([0-9a-f]+) ([0-9a-f]+) ([tT]) (.*)',line)
  if m:syms[m[4]]=(int(m[1],16),int(m[2],16))
 def masked(name):
  if name not in syms:return None
  start,n=syms[name];raw=read(start,n);out=bytearray(raw);md=cs.Cs(cs.CS_ARCH_X86,cs.CS_MODE_64);md.detail=True;refs=[];decoded=0
  for ins in md.disasm(raw,start):
   decoded+=ins.size
   for op in ins.operands:
    if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
     at=ins.address-start+ins.disp_offset;out[at:at+ins.disp_size]=bytes(ins.disp_size);refs.append({'type':'RIP','target':hex(ins.address+ins.size+op.mem.disp)})
    if op.type==X86_OP_IMM and (ins.group(cs.CS_GRP_CALL) or ins.group(cs.CS_GRP_JUMP)) and not start<=op.imm<start+n:
     at=ins.address-start+ins.imm_offset;out[at:at+ins.imm_size]=bytes(ins.imm_size);refs.append({'type':'external-branch','target':hex(op.imm),'symbols':[k for k,(a,z) in syms.items() if a==op.imm]})
  return {'address':hex(start),'address_mod64':start%64,'size':n,'decoded_bytes':decoded,'raw_sha256':hashlib.sha256(raw).hexdigest(),'masked_sha256':hashlib.sha256(out).hexdigest(),'references':refs}
 return masked
maskers={a:binary(a) for a in [arm,'B']}
with zipfile.ZipFile(s.parent/'r2/frozen-final-1-artifacts/benchmark-buffers.zip') as z:
 for name in d['flagged']:
  key=hashlib.sha256(name.encode()).hexdigest();dest=s/'layout'/arm/key;dest.mkdir(parents=True,exist_ok=False);path=dest/'case.input';path.write_bytes(z.read(frozen['workloads'][name]['input']['member']))
  hot={};profiles={}
  for a in [arm,'B']:
   # Exactly one perf record per arm. Resident workload: one warmup and 99 diagnostic triggers.
   command=['/usr/lib/linux-tools/6.17.0-22-generic/perf','record','-e','cycles:u','-c','10000','-o',str(dest/f'{a}.data'),'--','taskset','-c','8',str(s/f'driver-{a}'),str(path)]
   with (dest/f'{a}.stderr').open('w') as err:
    p=subprocess.run(command,input=b'R'*99+b'S',stdout=subprocess.PIPE,stderr=err,timeout=600,check=True)
   (dest/f'{a}.output').write_bytes(p.stdout)
   report=subprocess.check_output(['/usr/lib/linux-tools/6.17.0-22-generic/perf','report','-i',str(dest/f'{a}.data'),'--stdio','--no-children','--no-demangle','--sort','symbol','--percent-limit','0','--stdio-color','never'],text=True);(dest/f'{a}-report.txt').write_text(report)
   hot[a]=[]
   for line in report.splitlines():
    m=re.match(r'\s*([0-9.]+)%\s+\[[^]]+\]\s+(.+)',line)
    if m and float(m[1])>=10:hot[a].append({'percent':float(m[1]),'raw_symbol':m[2].strip(),'symbol':subprocess.check_output(['c++filt',m[2].strip()],text=True).strip()})
   profiles[a]={'hot':hot[a],'binary_sha256':hashlib.sha256((s/f'driver-{a}').read_bytes()).hexdigest()}
  names=sorted({x['symbol'] for a in hot for x in hot[a]});comparisons=[]
  for symbol in names:
   b,c=maskers['B'](symbol),maskers[arm](symbol);identical=bool(b and c and b['decoded_bytes']==b['size'] and c['decoded_bytes']==c['size'] and b['size']==c['size'] and b['masked_sha256']==c['masked_sha256'])
   comparisons.append({'symbol':symbol,'B':b,arm:c,'relocation_masked_identical':identical})
  layout=bool(names) and all(x['relocation_masked_identical'] for x in comparisons) and any(x['B']['address']!=x[arm]['address'] for x in comparisons)
  results[name]={'LAYOUT':layout,'profiles':profiles,'hot_symbol_union':names,'comparisons':comparisons,'directory':str(dest)}
(s/f'{arm}-layout.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({n:v['LAYOUT'] for n,v in results.items()},indent=2))
