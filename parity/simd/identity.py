#!/usr/bin/env python3
"""Execute every local ISA ceiling against qualified Linux output archives."""
import argparse, hashlib, json, struct, subprocess, zipfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('corpus',type=Path);p.add_argument('binary',type=Path);args=p.parse_args()
binary=args.binary.resolve();levels=subprocess.check_output([binary,'levels'],text=True).split()
counts={}
for level in levels:
    proc=subprocess.Popen([binary,level],stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    def call(data):
        proc.stdin.write(struct.pack('<I',len(data))+data);proc.stdin.flush()
        n=struct.unpack('<I',proc.stdout.read(4))[0];b=bytearray()
        while len(b)<n:
            part=proc.stdout.read(n-len(b));assert part;b+=part
        assert b[:4]==b'MD02';status,size,samples=struct.unpack_from('<iII',b,4);assert samples==0
        return status,b[16:16+size]
    for record_path in args.corpus.glob('*.json'):
        record=json.loads(record_path.read_text());archive=record_path.with_suffix('.zip')
        if 'cases' not in record or not archive.exists():continue
        assert hashlib.sha256(archive.read_bytes()).hexdigest()==record['archive_sha256']
        with zipfile.ZipFile(archive) as z:
            for entry in record['cases']:
                data=z.read(entry['case']+'.input');expected=z.read(entry['case']+'.output')
                assert hashlib.sha256(data).hexdigest()==entry['input_sha256']
                status,out=call(data);assert status==entry['status'],(level,entry['case'],'status')
                if status==0:assert out==expected,(level,entry['case'],'bytes')
                counts[level]=counts.get(level,0)+1
    proc.stdin.close();assert proc.wait()==0
assert counts
result={'passed':True,'counts':counts,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True)}
(args.corpus/'identity.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
