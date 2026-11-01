"""Reuse pinned fixture generators and exact decoder assertions at every ceiling."""
import hashlib, json, os, pathlib, struct, sys
root=pathlib.Path.cwd(); sys.path.insert(0,str(root/'parity/codec'))
import measure as m
import runner as r
import runner04 as r4
art=root/'parity/0217-decoders/receipts'; m.ART.mkdir(exist_ok=True)
cpp_path=m.build_cpp(True)
cpp=m.Driver(cpp_path)
fixtures=r.fixtures()
extra=r4.fixtures04()
cases=fixtures+[(n,b) for n,b,_ in extra if struct.unpack_from('<I',b,4)[0] in [1,2,3,4,5,6,7,18,20,21]]
cases+=list(r.generated(cpp,20261101,32))
cases += [(n,b) for n,b in r.malformed(fixtures)]
cpp.close()
binary=m.TARGET/'release/codec-driver'
levels=__import__('subprocess').check_output([binary,'levels'],text=True).split()
drivers={level:__import__('types').SimpleNamespace() for level in levels}
for level in levels:
    d=m.Driver.__new__(m.Driver)
    d.p=__import__('subprocess').Popen([binary,level],stdin=__import__('subprocess').PIPE,stdout=__import__('subprocess').PIPE,env=m.ENV)
    drivers[level]=d
drivers['wasm']=r.Wasm(m.TARGET/'wasm32-unknown-unknown/release/meshopt_codec_parity.wasm')
drivers['cpp']=m.Driver(cpp_path)
counts={}; rows=[]
try:
    for name,b in cases:
        expected=drivers['scalar'].call(b)
        c=drivers['cpp'].call(b)
        if c[0]!=-3:
            assert (c[0]==0)==(expected[0]==0),(name,'status')
            if c[0]==0: assert c[1]==expected[1],(name,'C++ bytes')
        for level,d in drivers.items():
            if level=='cpp':continue
            actual=d.call(b)
            assert actual[0]==expected[0] and (actual[0]!=0 or actual[1]==expected[1]),(name,level)
            op=struct.unpack_from('<I',b,4)[0]
            if op not in [8,9,22,23,24,25]:
                into=bytearray(b);struct.pack_into('<I',into,16,struct.unpack_from('<I',b,16)[0]|128)
                a=d.call(into)
                assert a[0]==expected[0] and (a[0]!=0 or a[1]==expected[1]),(name,level,'into')
            counts[level]=counts.get(level,0)+1
        rows.append(dict(case=name,input_sha256=hashlib.sha256(b).hexdigest(),status=expected[0],output_sha256=hashlib.sha256(expected[1]).hexdigest()))
finally:
    for d in drivers.values():d.close()
(art/'identity.json').write_text(json.dumps(dict(counts=counts,mismatches=0,cases=rows),indent=2)+'\n')
print(counts)
