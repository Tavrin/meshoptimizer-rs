#!/usr/bin/env python3
"""Validate every final arm before resuming; no elapsed samples."""
import run as lane
from peer_validation import exp_separate_zero_fields
import struct,json,hashlib
A=lane.ART;ds={k:lane.Driver(A/k) for k in ['rust-before-fair','rust-after','cpp-scalar','cpp','crate']};rows=[]
for row in json.loads((A/'inputs.json').read_text()):
 for api in [0,128]:
  b=bytearray(open(row['path'],'rb').read());struct.pack_into('<I',b,16,struct.unpack_from('<I',b,16)[0]|api)
  out={k:d.call(b)[:2] for k,d in ds.items()};assert out['rust-before-fair']==out['rust-after']==out['cpp-scalar']==out['cpp']
  status,peer=out['crate'];assert status==0;cpp=out['cpp'][1];op,n,s,mode=struct.unpack_from('<IIII',b,4);bits=struct.unpack_from('<I',b,28)[0]
  kind='exact';differences=0
  if op==11:
   assert ds['cpp'].call(lane.request(1,n,s,peer))[:2]==(0,bytes(b[44:]));kind='lossless vertex cross-decode' if peer!=cpp else 'exact'
  elif op==16 and mode&127==0:
   differences=exp_separate_zero_fields(b[44:],bits,cpp,peer);kind='0.25 zero exponent' if differences else 'exact'
  else:assert peer==cpp,row['name']
  rows.append({'case':row['name'],'api':'caller' if api else 'allocating','validation':kind,'zero_exponent_differences':differences,'cpp_output_sha256':hashlib.sha256(cpp).hexdigest(),'crate_output_sha256':hashlib.sha256(peer).hexdigest()})
for d in ds.values():d.close()
lane.save('peer-validation',{'rows':rows,'binaries':{k:lane.sha(A/k) for k in ds},'mismatches':0,'timing_samples':0})
print('all final arms validated',len(rows),'rows',flush=True)
