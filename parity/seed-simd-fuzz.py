#!/usr/bin/env python3
"""Seed every differential entry point from hash-checked qualified archives."""
import hashlib,json,os,struct,zipfile
from pathlib import Path
art=Path(os.environ['MESHOPT_ARTIFACTS'])
names={1:['vertex','vertex_into'],2:['triangles','triangles_into'],3:['sequence','sequence_into'],4:['oct'],5:['quat'],6:['exp'],7:['view','view_into'],8:['vertex_version'],9:['index_version'],18:['color'],20:['meshlet_decode','meshlet_decode_into'],21:['meshlet_raw','meshlet_raw_into']}
selectors={name:i for i,name in enumerate(['vertex','vertex_into','triangles','triangles_into','sequence','sequence_into','oct','quat','exp','vertex_version','index_version','view','view_into','color','meshlet_decode','meshlet_decode_into','meshlet_raw','meshlet_raw_into'])}
counts={}
for record in art.glob('*.json'):
    archive=record.with_suffix('.zip');meta=json.loads(record.read_text())
    if 'cases' not in meta or not archive.exists():continue
    assert hashlib.sha256(archive.read_bytes()).hexdigest()==meta['archive_sha256']
    with zipfile.ZipFile(archive) as z:
        for case in meta['cases']:
            if case['status']!=0:continue
            b=z.read(case['case']+'.input');assert hashlib.sha256(b).hexdigest()==case['input_sha256']
            op,count,stride,mode,filter,version,level=struct.unpack_from('<7I',b,4)
            if op not in names or count*stride>65000 or stride>256 or len(b)>65000:continue
            if op in [20,21]:
                if count>255 or (op==21 and version>255):continue
                count,mode,filter=(int(version==256)|(int(stride==4)<<1)|(int(level==4)<<2) if op==20 else 0),count,version&255
            seed=struct.pack('<IHBBB',count,stride,mode,filter,255)+b[44:]
            for name in names[op]:
                for target,data in [(name,seed),('simd',bytes([selectors[name]])+seed)]:
                    directory=art/'simd-fuzz'/target/'corpus';directory.mkdir(parents=True,exist_ok=True)
                    (directory/hashlib.sha256(data).hexdigest()).write_bytes(data);counts[target]=counts.get(target,0)+1
assert counts,'No qualified seed archive supplied'
print(counts)
