#!/usr/bin/env python3
"""Capture unmodified applicable upstream native fixture inputs before mutation."""
from pathlib import Path
import re
import sys
ref, destination = map(Path, sys.argv[1:])
source = (ref / 'demo/tests.cpp').read_text()
names = ['simplify','simplifyStuck','simplifyFlip','simplifyScale','simplifyDegenerate','simplifyLockBorder','simplifyAttr','simplifyLockFlags','simplifyLockFlagsSeam','simplifyErrorAbsolute','simplifySeam','simplifySeamFake','simplifySeamAttr']
preamble = r'''
#include "meshoptimizer.h"
#include <cassert>
#include <cmath>
#include <cstring>
#include <cstdint>
#include <fstream>
#include <string>
static std::string root, label;
static unsigned serial=0;
static void word(std::ofstream& f, uint32_t v) { for(int i=0;i<4;++i) f.put(char(v>>(i*8))); }
static void number(std::ofstream& f,float v) {uint32_t b;memcpy(&b,&v,4);word(f,b);}
static void capture(unsigned op,const unsigned* indices,size_t ic,const float* p,size_t vc,size_t ps,const float* a,size_t as,const float* weights,size_t ac,const unsigned char* flags,size_t target,float error,unsigned options) {
 std::ofstream f(root+"/native-"+label+"-"+std::to_string(serial++)+".input",std::ios::binary);
 f.write("MO01",4);word(f,op);word(f,uint32_t(vc));word(f,uint32_t(ic));number(f,1.05f);word(f,0);word(f,0);
 for(size_t i=0;i<vc;++i) for(size_t j=0;j<3;++j) number(f,p[i*(ps/4)+j]);
 for(size_t i=0;i<ic;++i) word(f,indices[i]);
 word(f,uint32_t(target));number(f,error);word(f,options);word(f,uint32_t(ac));
 for(size_t j=0;j<ac;++j) number(f,weights[j]);
 for(size_t i=0;i<vc;++i) for(size_t j=0;j<ac;++j) number(f,a[i*(as/4)+j]);
 for(size_t i=0;i<vc;++i) word(f,flags?flags[i]:0);
}
static size_t capturedSimplify(unsigned* out,const unsigned* indices,size_t ic,const float* p,size_t vc,size_t ps,size_t target,float error,unsigned options=0,float* result=nullptr) {
 capture(4,indices,ic,p,vc,ps,nullptr,0,nullptr,0,nullptr,target,error,options);
 return meshopt_simplify(out,indices,ic,p,vc,ps,target,error,options,result);
}
static size_t capturedAttributes(unsigned* out,const unsigned* indices,size_t ic,const float* p,size_t vc,size_t ps,const float* a,size_t as,const float* weights,size_t ac,const unsigned char* flags,size_t target,float error,unsigned options=0,float* result=nullptr) {
 capture(5,indices,ic,p,vc,ps,a,as,weights,ac,flags,target,error,options);
 return meshopt_simplifyWithAttributes(out,indices,ic,p,vc,ps,a,as,weights,ac,flags,target,error,options,result);
}
static float capturedScale(const float* p,size_t vc,size_t ps) {capture(6,nullptr,0,p,vc,ps,nullptr,0,nullptr,0,nullptr,0,0,0);return meshopt_simplifyScale(p,vc,ps);}
'''
bodies = []
for name in names:
    match = re.search(r'static void '+name+r'\([^)]*\)\s*\{', source)
    start = match.start(); end = match.end(); depth = 1
    while depth:
        depth += (source[end] == '{') - (source[end] == '}'); end += 1
    body = source[start:end].replace('meshopt_simplifyWithAttributes(', 'capturedAttributes(').replace('meshopt_simplifyScale(', 'capturedScale(').replace('meshopt_simplify(', 'capturedSimplify(')
    bodies.append(body)
main = 'int main(int argc,char** argv) { if(argc!=2) return 1;root=argv[1];\n'
for name in names:
    main += f'label="{name}";{name}(' + ('false' if name == 'simplifyAttr' else '') + ');\n'
    if name == 'simplifyAttr': main += 'label="simplifyAttr-zero-weight";simplifyAttr(true);\n'
main += '}\n'
destination.write_text(preamble+'\n'.join(bodies)+main)
