#!/usr/bin/env python3
"""Audit Rust code tokens, excluding nested comments and all string literals."""
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
MODULE = ROOT / 'src/codec/simd'
# These unpublished oracle drivers call the peer meshopt crate's C++ FFI.
# Neither is part of our library; the independent package audit stays strict.
PEER_FFI_DRIVERS = {ROOT / 'parity/compare/src/main.rs', ROOT / 'parity/encoders/src/main.rs'}

def code(source):
    chars = list(source); i = 0
    def blank(a, b):
        for n in range(a,b):
            if chars[n] != '\n': chars[n] = ' '
    while i < len(source):
        if source.startswith('//',i):
            end = source.find('\n',i); end = len(source) if end < 0 else end
            blank(i,end);i=end
        elif source.startswith('/*',i):
            start=i;i+=2;depth=1
            while depth and i<len(source):
                if source.startswith('/*',i):depth+=1;i+=2
                elif source.startswith('*/',i):depth-=1;i+=2
                else:i+=1
            assert depth==0,'unterminated comment'
            blank(start,i)
        else:
            raw=re.match(r'(?:br|cr|r)(#*)"',source[i:])
            if raw:
                end=source.find('"'+raw[1],i+len(raw[0]));assert end>=0
                end+=1+len(raw[1]);blank(i,end);i=end
            elif source[i]=='"':
                start=i;i+=1
                while i<len(source):
                    if source[i]=='\\':i+=2
                    elif source[i]=='"':i+=1;break
                    else:i+=1
                blank(start,i)
            elif source[i]=="'" and re.match(r"'(?:\\[^\n]|[^'\\\n])'",source[i:]):
                m=re.match(r"'(?:\\[^\n]|[^'\\\n])'",source[i:]);end=i+len(m[0]);blank(i,end);i=end
            else:i+=1
    return ''.join(chars)

def main():
    # git inventory plus new source files; never descend into build/dependency output.
    files=set(ROOT.glob('src/**/*.rs'))|set(ROOT.glob('tests/**/*.rs'))|set(ROOT.glob('parity/**/*.rs'))|set(ROOT.glob('fuzz/**/*.rs'))
    allows=[];blocks=[]
    for file in sorted(files):
        if file in PEER_FFI_DRIVERS:
            continue
        source=code(file.read_text());inside=file.is_relative_to(MODULE)
        for m in re.finditer(r'\bunsafe\b(?:\s*\{)?',source):
            assert inside,f'unsafe outside audited module: {file}'
            assert m[0].endswith('{'),f'only unsafe blocks permitted: {file}'
            line=source.count('\n',0,m.start())+1
            blocks.append(f'{file.relative_to(ROOT)}:{line}')
            prefix=file.read_text()[:m.start()].splitlines()[-6:]
            assert any('// SAFETY:' in l for l in prefix),f'missing safety comment: {file}:{line}'
        for m in re.finditer(r'\b(?:allow|expect)\s*\([^)]*\bunsafe_code\b[^)]*\)',source,re.S):
            assert inside and m[0].startswith('allow'),f'unsafe lint escape: {file}'
            allows.append(file)
    assert allows==[MODULE/'mod.rs'],allows
    audit=(MODULE/'SAFETY.md').read_text()
    recorded=re.findall(r'^\| `(src/codec/simd/[^`]+:\d+)` \|',audit,re.M)
    assert sorted(recorded)==sorted(blocks),'SAFETY.md block inventory differs from code tokens'
    print(f'PASS: one allow, {len(blocks)} individually documented blocks; no unsafe fn/impl')

if __name__=='__main__':main()
