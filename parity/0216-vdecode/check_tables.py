"""Verify the relocated raw-dispatch jump table retains identical local targets."""
import json,os,re,struct
from pathlib import Path
A=Path(os.environ['DECODE_ART']);D=A/'codegen';functions=json.loads((D/'functions.json').read_text());out={}
for arm in ['base','control','candidate']:
 if arm not in functions:continue
 raw=next(x for x in functions[arm] if x['symbol']=='meshoptimizer_rs::codec::raw')
 binary=(A/('driver-'+arm)).read_bytes();assert binary[:6]==b'\x7fELF\x02\x01'
 offset=struct.unpack_from('<Q',binary,0x28)[0];size,count=struct.unpack_from('<HH',binary,0x3a)
 sections=[struct.unpack_from('<IIQQQQIIQQ',binary,offset+i*size) for i in range(count)]
 asm=(D/(arm+'-meshoptimizer-rs-codec-raw.asm')).read_text()
 addresses=[int(x,16) for x in re.findall(r'lea[^\n]+#\s+([0-9a-f]+) <libm::math::generic::sqrt::RSQRT_TAB\+',asm)]
 assert len(addresses)==1,(arm,addresses)
 targets=[]
 for address in addresses:
  section=next(s for s in sections if s[1]!=8 and s[3]<=address< s[3]+s[5])
  data_offset=section[4]+address-section[3]
  target=[address+x-raw['address'] for x in struct.unpack_from('<4i',binary,data_offset)]
  assert all(0<=x<raw['bytes'] for x in target),target
  targets.append(target)
 out[arm]=targets
assert all(x==out['base'] for x in out.values()),out
(D/'jump-tables.json').write_text(json.dumps(dict(local_targets=out,identity=True),indent=2)+'\n');print(out)
