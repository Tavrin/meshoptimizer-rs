#!/usr/bin/env python3
"""Export every applicable native decoder invocation without editing the oracle."""
import sys,re
from pathlib import Path
reference,out,destination=map(Path,sys.argv[1:])
source=(reference/'demo/tests.cpp').read_text()
source=source.replace('#include "../src/meshoptimizer.h"','#include "meshoptimizer.h"')
# Keep the upstream fixture bodies and assertions; adapt only the invocation
# names to capture exact requests before forwarding to the real decoder.
functions=re.findall(r'static void ((?:decode(?:Index|Vertex|Version|Filter(?:Oct|Quat|Exp))|encode(?:Index|Vertex|Filter(?:Oct|Quat|Exp)))\w*)\(\)',source)
main=source.index('void runTests(')
source=source[:main]
for fn,wrapper in [('meshopt_decodeVertexBuffer','capVertex'),('meshopt_decodeIndexBuffer','capIndex'),('meshopt_decodeIndexSequence','capSequence'),('meshopt_decodeVertexVersion','capVertexVersion'),('meshopt_decodeIndexVersion','capIndexVersion'),('meshopt_decodeFilterOct','capOct'),('meshopt_decodeFilterQuat','capQuat'),('meshopt_decodeFilterExp','capExp')]:
    source=source.replace(fn+'(',wrapper+'(')
header=r"""
#include "meshoptimizer.h"
#include <fstream>
#include <string>
#include <vector>
#include <cstdint>
static std::string dir,fixture;static int sequence=0;
static void put(std::vector<unsigned char>& b,uint32_t v){for(int i=0;i<4;++i)b.push_back(v>>(8*i));}
static void capture(int op,size_t n,size_t s,const void* p,size_t length){
 std::vector<unsigned char>b={'M','C','0','2'};
 for(uint32_t v:{uint32_t(op),uint32_t(n),uint32_t(s),0u,0u,0u,2u,0u,1u,uint32_t(length)})put(b,v);
 if(length){const unsigned char* q=static_cast<const unsigned char*>(p);b.insert(b.end(),q,q+length);}
 std::ofstream f(dir+"/native-"+fixture+"-"+std::to_string(sequence++)+".input",std::ios::binary);f.write(reinterpret_cast<char*>(b.data()),b.size());
}
static int capVertex(void* d,size_t n,size_t s,const unsigned char* p,size_t l){capture(1,n,s,p,l);return meshopt_decodeVertexBuffer(d,n,s,p,l);}
static int capIndex(void* d,size_t n,size_t s,const unsigned char* p,size_t l){capture(2,n,s,p,l);return meshopt_decodeIndexBuffer(d,n,s,p,l);}
static int capSequence(void* d,size_t n,size_t s,const unsigned char* p,size_t l){capture(3,n,s,p,l);return meshopt_decodeIndexSequence(d,n,s,p,l);}
template<typename T> static int capIndex(T* d,size_t n,const unsigned char* p,size_t l){return capIndex(d,n,sizeof(T),p,l);}
template<typename T> static int capSequence(T* d,size_t n,const unsigned char* p,size_t l){return capSequence(d,n,sizeof(T),p,l);}
static int capVertexVersion(const unsigned char* p,size_t l){capture(8,0,4,p,l);return meshopt_decodeVertexVersion(p,l);}
static int capIndexVersion(const unsigned char* p,size_t l){capture(9,0,4,p,l);return meshopt_decodeIndexVersion(p,l);}
static void capOct(void* p,size_t n,size_t s){capture(4,n,s,p,n*s);meshopt_decodeFilterOct(p,n,s);}
static void capQuat(void* p,size_t n,size_t s){capture(5,n,s,p,n*s);meshopt_decodeFilterQuat(p,n,s);}
static void capExp(void* p,size_t n,size_t s){capture(6,n,s,p,n*s);meshopt_decodeFilterExp(p,n,s);}
"""
footer='int main(int argc,char** argv){if(argc!=2)return 1;dir=argv[1];\n'
for fn in functions:
    if fn.startswith(('decodeVertex','encodeVertex')):
        for v in [0,1]:footer+=f'meshopt_encodeVertexVersion({v});fixture="{fn}-v{v}";{fn}();\n'
    else:footer+=f'fixture="{fn}";{fn}();\n'
footer+='return 0;}\n'
out.write_text(header+source+footer)
print('native fixture functions',len(functions))
