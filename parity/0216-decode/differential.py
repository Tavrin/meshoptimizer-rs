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

def neon_escape_group(n,stride,src):
    """Locate a scalar-reachable 2-bit group with exactly sixteen escapes.

    Parse actual bytes, including mutated/truncated cases; generator parameters
    and an unaligned occurrence of ff ff ff ff are not evidence of this class.
    Return before the first scalar parse failure, just as the native decoder.
    """
    if not src or src[0] not in (0xa0,0xa1) or not 0<stride<=256 or stride%4:
        return None
    v=src[0]&1;tail=stride+(stride//4 if v else 0)
    if len(src)<1+max(tail,24 if v else 32):return None
    channels=src[len(src)-tail+stride:] if v else bytes(stride//4)
    pos=1;blocksize=min((8192//stride)&~15,256)
    for offset in range(0,n,blocksize):
        block=min(n-offset,blocksize);groups=(block+15)//16
        controls=src[pos:pos+stride//4] if v else bytes(stride//4)
        if v:
            if len(controls)!=stride//4:return None
            pos+=stride//4
        for k in range(stride//4):
            for j in range(4):
                ctrl=(controls[k]>>(2*j))&3
                if ctrl==2:continue
                if ctrl==3:
                    if pos+block>len(src):return None
                    pos+=block;continue
                headers=(groups+3)//4;header=pos
                if pos+headers>len(src):return None
                pos+=headers
                table=[0,2,4,8] if not v else [0,1,2,4] if ctrl==0 else [1,2,4,8]
                for group in range(groups):
                    if pos+24>len(src):return None
                    bits=table[(src[header+group//4]>>(2*(group%4)))&3]
                    if bits==2 and src[pos:pos+4]==b'\xff'*4:
                        return dict(offset=pos,block=offset,plane=k*4+j,group=group)
                    if bits==8:pos+=16
                    elif bits:
                        packed=src[pos:pos+bits*2];escape=(1<<bits)-1
                        escapes=sum(((byte>>shift)&escape)==escape for byte in packed for shift in range(0,8,bits))
                        pos+=bits*2+escapes
            if channels[k]&3==3:return None
    return None

def allowed_cpp_divergence(machine,n,stride,src):
    return machine in ('aarch64','arm64') and neon_escape_group(n,stride,src) is not None

def main():
    ds={l:Driver([target/'proof/release/differential',l]) for l in levels}
    for name in ['cpp-scalar','cpp-default']:ds[name]=Driver([target/(name+'-exact')])
    count=0; cpp_failure_differences=0; cpp_success_differences=0; first=[]
    # Only the explicitly classified upstream NEON escape-count wrap is allowed.
    isa_divergences=0; isa_allowed=0; isa_failing=0; isa_first=[]
    def cpp_arms(out,case,n,stride,src):
        nonlocal isa_divergences,isa_allowed,isa_failing
        s,d=out['cpp-scalar'],out['cpp-default']
        # Failure prefixes may differ by staging (counted below); only a
        # status split or differing successful bytes is an ISA divergence.
        if s[:4]==d[:4] and (int.from_bytes(s[:4],'little') or s==d):return ['cpp-scalar','cpp-default']
        isa_divergences+=1
        allowed=allowed_cpp_divergence(os.uname().machine,n,stride,src)
        if allowed:isa_allowed+=1
        else:isa_failing+=1
        if len(isa_first)<4 or (not allowed and not any(not x['allowed'] for x in isa_first)):
            isa_first.append(dict(case=case,allowed=allowed,group=neon_escape_group(n,stride,src),scalar_status=s[:4].hex(),default_status=d[:4].hex()))
        return ['cpp-scalar']
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
                                    for k in cpp_arms(out,[n,stride,v,ch,ctrl,sel,length],n,stride,src[:length]):
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
            for k in cpp_arms(out,['random',i],n,stride,src):
                assert out[k][:4]==expected[:4],('random',i,k,'status')
                if out[k]!=expected:
                    if int.from_bytes(expected[:4],'little'):cpp_failure_differences+=1
                    else:cpp_success_differences+=1
            count+=1
        assert cpp_success_differences==0
    finally:
        for d in ds.values():d.close()
    r=dict(cases=count,levels=levels,forced_level_differences=0,cpp_success_differences=cpp_success_differences,cpp_failure_write_differences=cpp_failure_differences,cpp_default_isa_divergences=isa_divergences,cpp_default_isa_allowed=isa_allowed,cpp_default_isa_failing=isa_failing,first=first,isa_first=isa_first)
    (art/('differential-no-std.json' if os.environ.get('DECODE_NO_STD') else 'differential.json')).write_text(json.dumps(r,indent=2)+'\n');print(r)

    assert isa_failing==0,isa_first

if __name__=="__main__": main()
