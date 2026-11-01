"""Count packed-group escape patterns in retained C++ vertex streams."""
import collections, json, os, pathlib, struct
counts=collections.Counter()
for case in json.loads(pathlib.Path(os.environ['DECODE_MANIFEST']).read_text()):
    b=(pathlib.Path(os.environ['DECODE_INPUTS'])/(case['case']+'.bin')).read_bytes()
    n,ni=struct.unpack_from('<II',b); at=8+n*12+ni*4
    size=struct.unpack_from('<I',b,at)[0]; src=b[at+4:at+4+size]
    version=src[0]&15; stride=12; pos=1
    for offset in range(0,n,256):
        block=min(n-offset,256); groups=(block+15)//16
        controls=bytes(stride//4)
        if version: controls=src[pos:pos+stride//4]; pos+=stride//4
        for k in range(0,stride,4):
            for j in range(4):
                ctrl=(controls[k//4]>>(j*2))&3
                if ctrl==3: pos+=block; continue
                if ctrl==2: continue
                headers=src[pos:pos+(groups+3)//4]; pos+=len(headers)
                for g in range(groups):
                    h=(headers[g//4]>>((g%4)*2))&3
                    bits=([0,2,4,8] if version==0 else [0,1,2,4] if ctrl==0 else [1,2,4,8])[h]
                    if bits in (0,8): counts['zero' if bits==0 else 'literal']+=1; pos+=16 if bits==8 else 0; continue
                    packed=src[pos:pos+bits*2]; sentinel=(1<<bits)-1
                    escapes=sum(((byte>>shift)&sentinel)==sentinel for byte in packed for shift in range(0,8,bits))
                    counts['packed-none' if escapes==0 else 'packed-all' if escapes==16 else 'packed-mixed']+=1
                    pos+=bits*2+escapes
    tail=stride+(stride//4 if version else 0)
    assert len(src)-pos==max(tail,24 if version else 32),(case['case'],pos,len(src))
out=pathlib.Path('parity/0217-decoders/receipts/group-profile.json')
out.write_text(json.dumps(counts,indent=2)+'\n'); print(counts)
