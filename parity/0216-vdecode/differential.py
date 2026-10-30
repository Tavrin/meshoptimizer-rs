"""All local ISA ceilings versus scalar and C++ (including failure writes)."""
import hashlib,json,os,struct,subprocess,random
from pathlib import Path
art=Path(os.environ['DECODE_ART']); target=Path(os.environ['CARGO_TARGET_DIR']); ref=Path(os.environ['MESHOPT_REFERENCE'])
levels=['scalar','sse2','ssse3','sse41'] if os.uname().machine=='x86_64' else ['scalar','neon']
class Driver:
    def __init__(self,cmd):self.p=subprocess.Popen(list(map(str,cmd)),stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    def call(self,b):
        self.p.stdin.write(struct.pack('<I',len(b))+b);self.p.stdin.flush(); n=struct.unpack('<I',self.p.stdout.read(4))[0];out=bytearray()
        while len(out)<n:out+=self.p.stdout.read(n-len(out))
        return bytes(out)
    def close(self):self.p.stdin.close();assert self.p.wait()==0

def stream(n,stride,v,ch,ctrl,sel):
    b=bytearray([0xa0|v]);blocksize=min((8192//stride)&~15,256)
    for offset in range(0,n,blocksize):
        block=min(n-offset,blocksize);groups=(block+15)//16
        if v:b+=bytes([ctrl*85])*(stride//4)
        for _ in range(stride):
            if v and ctrl==2:continue
            if v and ctrl==3:b+=bytes((i*73+offset*3+17)&255 for i in range(block));continue
            b+=bytes([sel*85])*((groups+3)//4)
            bits=([0,2,4,8] if not v else [0,1,2,4] if ctrl==0 else [1,2,4,8])[sel]
            for _ in range(groups):
                if bits==8:b+=bytes([0x93])*16
                elif bits:b+=bytes([255])*(bits*2)+bytes((i*71+19)&255 for i in range(16))
    tail=stride+(stride//4 if v else 0);padded=max(tail,24 if v else 32)
    b+=bytes(padded-tail)+bytes((i*37+251)&255 for i in range(stride))
    if v:b+=bytes([ch])*(stride//4)
    return b

def main():
    ds={l:Driver([target/'proof/release/differential',l]) for l in levels}
    for name in ['cpp-scalar','cpp-default']:ds[name]=Driver([target/(name+'-exact')])
    count=0; cpp_failure_differences=0; cpp_success_differences=0; first=[]
    try:
        for n in list(range(34))+list(range(64,81))+list(range(256,273)):
            for stride in [4,8,12,32,48,256]:
                for v in [0,1]:
                    for ch in ([0] if not v else [0,1,3]+[r*16+2 for r in range(16)]):
                        for ctrl in (range(4) if v else [0]):
                            for sel in (range(4) if ctrl<2 else [0]):
                                src=stream(n,stride,v,ch,ctrl,sel)
                                for length in [len(src),len(src)-1,len(src)//2,0]:
                                    b=struct.pack('<III',n,stride,length)+src[:length]
                                    out={k:d.call(b) for k,d in ds.items()};expected=out['scalar']
                                    for level in levels:assert out[level]==expected,(n,stride,v,ch,ctrl,sel,length,level)
                                    for k in ['cpp-scalar','cpp-default']:
                                        # Both success/failure status and entire destination are observed.
                                        assert out[k][:4]==expected[:4],(n,stride,v,ch,ctrl,sel,length,k,'status')
                                        if out[k]!=expected:
                                            failed=bool(int.from_bytes(expected[:4],'little'))
                                            if failed:cpp_failure_differences+=1
                                            else:cpp_success_differences+=1
                                            if len(first)<4:first.append(dict(n=n,stride=stride,v=v,ch=ch,ctrl=ctrl,sel=sel,length=length,arm=k,rust_status=expected[:4].hex(),cpp_status=out[k][:4].hex()))
                                    count+=1
        rng=random.Random(20261029)
        for i in range(2000):
            n=rng.randrange(1025);stride=rng.choice([4,8,12,16,32,48,64,256]);v=i%2
            src=stream(n,stride,v,[0,1,2,0xf2][i%4],i%4,i%4)
            if i%3==0:src[rng.randrange(len(src))]^=rng.randrange(1,256)
            if i%5==0:src=src[:rng.randrange(len(src))]
            b=struct.pack('<III',n,stride,len(src))+src
            out={k:d.call(b) for k,d in ds.items()};expected=out['scalar']
            for level in levels:assert out[level]==expected,('random',i,level)
            for k in ['cpp-scalar','cpp-default']:
                assert out[k][:4]==expected[:4],('random',i,k,'status')
                if out[k]!=expected:
                    if int.from_bytes(expected[:4],'little'):cpp_failure_differences+=1
                    else:cpp_success_differences+=1
            count+=1
        assert cpp_success_differences==0
    finally:
        for d in ds.values():d.close()
    r=dict(cases=count,levels=levels,forced_level_differences=0,cpp_success_differences=cpp_success_differences,cpp_failure_write_differences=cpp_failure_differences,first=first)
    (art/('differential-no-std.json' if os.environ.get('DECODE_NO_STD') else 'differential.json')).write_text(json.dumps(r,indent=2)+'\n');print(r)

if __name__=="__main__": main()
