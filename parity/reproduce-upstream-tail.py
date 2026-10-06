#!/usr/bin/env python3
"""Reproduce pinned upstream's odd meshlet-tail scalar/SIMD status split."""
import hashlib
import struct
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent / 'simd'))
import qualify as q
b = bytes.fromhex('4d4330321400000005000000040000000000000000000000010000000400000000000000010000001b000000943fe890013fd1c493776900000000000000000000000000f2101c')
assert hashlib.sha256(b).hexdigest() == '1602803bd71de17b647bdfdf64f7f5e1e9230836796c257359ba1491c5c97512'
expected = '84f10ea2dbf10053d297030ad90332fba171139f972049575862fda7f5c777f7'
rows = []
for api in ['allocating', 'caller-buffer']:
    request = bytearray(b)
    if api == 'caller-buffer': struct.pack_into('<I', request, 16, 128)
    ds = q.drivers()
    try:
        for backend, driver in ds.items():
            status, out, _ = driver.call(request)
            digest = hashlib.sha256(out).hexdigest()
            assert status == (-3 if backend == 'cpp-simd' else 0), (api, backend, status)
            if status == 0: assert digest == expected, (api, backend)
            rows.append({'api':api,'backend':backend,'status':status,'output_bytes':len(out),'output_sha256':digest})
    finally:
        for driver in ds.values(): driver.close()
q.save(q.ART/'upstream-tail-reproduction.json', {'input_hex':b.hex(),'input_sha256':hashlib.sha256(b).hexdigest(),'sources':q.sources(),'binaries':{p.name:q.m.sha(p) for p in q.BIN.iterdir()},'rows':rows})
print('PASS: expected upstream status disagreement; all Rust levels match scalar bytes')
