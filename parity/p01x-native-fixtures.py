"""Capture preprocessing inputs from pinned native fixture bodies before mutation."""
from pathlib import Path
import re
import sys
reference,destination=map(Path,sys.argv[1:])
source=(reference/'demo/tests.cpp').read_text()
names=['customAllocator','emptyMesh','remapCustom','simplifySloppyStuck','simplifySloppyLocks','simplifyPointsStuck','simplifySparse','simplifyPrune','simplifyPruneCleanup','simplifyPruneFunc','simplifyUpdate','simplifyUpdateLocked','simplifyFolds','filterTriangles','adjacency','tessellation','provoking','quantizeFloat','quantizeHalf','dequantizeHalf','computePositionExponent']
preamble=r'''
#include "meshoptimizer.h"
#include <cassert>
#include <cmath>
#include <cstring>
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <string>
#include <vector>
static std::string root,label;static unsigned serial=0;
static void word(std::ofstream& f,uint32_t v){for(int k=0;k<4;k++)f.put(char(v>>(k*8)));}
static uint32_t bits(float f){uint32_t b;memcpy(&b,&f,4);return b;}
static void capture(unsigned op,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,const void* keys,size_t size,size_t stride,const float* a,size_t as,const float* w,size_t ac,const unsigned char* flags,size_t target,unsigned p0,unsigned p1,unsigned options){
 std::ofstream f(root+"/p01x-native-"+label+"-"+std::to_string(serial++)+".input",std::ios::binary);f.write("MO02",4);
 unsigned sc=keys?1:0;
 for(auto v:{op,unsigned(vc),unsigned(ic),0u,0u,unsigned(size),unsigned(stride),sc,options,unsigned(target),p0,p1,unsigned(ac)})word(f,v);
 for(size_t i=0;i<vc;i++)for(int k=0;k<3;k++)word(f,p?bits(p[i*(ps/4)+k]):0);
 for(size_t i=0;i<ic;i++)word(f,ib?ib[i]:unsigned(i));
 if(keys)for(size_t i=0;i<vc;i++){f.write(static_cast<const char*>(keys)+i*stride,size);for(size_t k=size;k<stride;k++)f.put(0);}
 for(size_t k=0;k<ac;k++)word(f,bits(w[k]));
 for(size_t i=0;i<vc;i++)for(size_t k=0;k<ac;k++)word(f,bits(a[i*(as/4)+k]));
 for(size_t i=0;i<vc;i++)word(f,flags?flags[i]:0);
}
static size_t allocCount;static size_t freeCount;
template<typename T>static size_t capturedFetch(void* out,T* ib,size_t ic,const void* vb,size_t vc,size_t size){std::vector<unsigned> indices(ib,ib+ic);capture(22,indices.data(),ic,nullptr,vc,0,vb,size,size,nullptr,0,nullptr,0,nullptr,0,0,0,0);return meshopt_optimizeVertexFetch(out,ib,ic,vb,vc,size);}
static void capturedFifo(unsigned* out,const unsigned* ib,size_t ic,size_t vc,unsigned size){capture(8,ib,ic,nullptr,vc,0,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,size,0,0);meshopt_optimizeVertexCacheFifo(out,ib,ic,vc,size);}
static size_t capturedSloppy(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,const unsigned char* locks,size_t target,float error,float* re=nullptr){capture(24,ib,ic,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,locks,target,bits(error),locks?2:0,0);return meshopt_simplifySloppy(out,ib,ic,p,vc,ps,locks,target,error,re);}
static size_t capturedSloppy(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,size_t target,float error,float* re=nullptr){return capturedSloppy(out,ib,ic,p,vc,ps,nullptr,target,error,re);}
static size_t capturedPoints(unsigned* out,const float* p,size_t vc,size_t ps,const float* c,size_t cs,float weight,size_t target){assert(c==nullptr);capture(26,nullptr,0,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,target,bits(weight),0,0);return meshopt_simplifyPoints(out,p,vc,ps,c,cs,weight,target);}
static size_t capturedPrune(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,float error){capture(25,ib,ic,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,bits(error),0,0);return meshopt_simplifyPrune(out,ib,ic,p,vc,ps,error);}
static size_t capturedSimplify(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,size_t target,float error,unsigned options=0,float* re=nullptr){unsigned op=options&128?36:options&8?35:34;capture(op,ib,ic,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,target,bits(error),0,options);return meshopt_simplify(out,ib,ic,p,vc,ps,target,error,options,re);}
static size_t capturedAttributes(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,const float* a,size_t as,const float* w,size_t ac,const unsigned char* flags,size_t target,float error,unsigned options=0,float* re=nullptr){capture(34,ib,ic,p,vc,ps,nullptr,1,1,a,as,w,ac,flags,target,bits(error),0,options);return meshopt_simplifyWithAttributes(out,ib,ic,p,vc,ps,a,as,w,ac,flags,target,error,options,re);}
static size_t capturedUpdate(unsigned* ib,size_t ic,float* p,size_t vc,size_t ps,float* a,size_t as,const float* w,size_t ac,const unsigned char* flags,size_t target,float error,unsigned options=0,float* re=nullptr){capture(27,ib,ic,p,vc,ps,nullptr,1,1,a,as,w,ac,flags,target,bits(error),0,options);return meshopt_simplifyWithUpdate(ib,ic,p,vc,ps,a,as,w,ac,flags,target,error,options,re);}
static size_t capturedFilter(unsigned* out,const unsigned* ib,size_t ic,const void* v,size_t vc,size_t size,size_t stride){capture(14,ib,ic,nullptr,vc,0,v,size,stride,nullptr,0,nullptr,0,nullptr,0,0,0,0);return meshopt_filterIndexBuffer(out,ib,ic,v,vc,size,stride);}
static void capturedAdjacency(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps){capture(19,ib,ic,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,0,0);meshopt_generateAdjacencyIndexBuffer(out,ib,ic,p,vc,ps);}
static void capturedTessellation(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps){capture(20,ib,ic,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,0,0);meshopt_generateTessellationIndexBuffer(out,ib,ic,p,vc,ps);}
static size_t capturedProvoking(unsigned* out,unsigned* reorder,const unsigned* ib,size_t ic,size_t vc){capture(21,ib,ic,nullptr,vc,0,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,0,0);return meshopt_generateProvokingIndexBuffer(out,reorder,ib,ic,vc);}
static size_t capturedCustom(unsigned* out,const unsigned* ib,size_t ic,const float* p,size_t vc,size_t ps,int(*callback)(void*,unsigned,unsigned),void* context){unsigned p1=ib?0:1;if(callback)p1|=callback(context,0,0)?8:16;else p1|=4;capture(11,ib,ib?ic:0,p,vc,ps,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,p1,0);return meshopt_generateVertexRemapCustom(out,ib,ic,p,vc,ps,callback,context);}
static float capturedFloat(float v,int n){unsigned i=bits(v);capture(31,&i,1,nullptr,0,0,nullptr,1,1,nullptr,0,nullptr,0,nullptr,n,0,0,0);return meshopt_quantizeFloat(v,n);}
static unsigned short capturedHalf(float v){unsigned i=bits(v);capture(30,&i,1,nullptr,0,0,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,0,0);return meshopt_quantizeHalf(v);}
static float capturedDehalf(unsigned short v){unsigned i=v;capture(32,&i,1,nullptr,0,0,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,0,0,0);return meshopt_dequantizeHalf(v);}
static int capturedExponent(const float* minv,const float* maxv,int minexp,int maxbits){float p[6];memcpy(p,minv,12);memcpy(p+3,maxv,12);capture(33,nullptr,0,p,2,12,nullptr,1,1,nullptr,0,nullptr,0,nullptr,0,unsigned(minexp),unsigned(maxbits),0);return meshopt_computePositionExponent(minv,maxv,minexp,maxbits);}
'''
mapping={'optimizeVertexFetch':'Fetch','optimizeVertexCacheFifo':'Fifo','simplifySloppy':'Sloppy','simplifyPoints':'Points','simplifyPrune':'Prune','simplify':'Simplify','simplifyWithAttributes':'Attributes','simplifyWithUpdate':'Update','filterIndexBuffer':'Filter','generateAdjacencyIndexBuffer':'Adjacency','generateTessellationIndexBuffer':'Tessellation','generateProvokingIndexBuffer':'Provoking','generateVertexRemapCustom':'Custom','quantizeFloat':'Float','quantizeHalf':'Half','dequantizeHalf':'Dehalf','computePositionExponent':'Exponent'}
def body(name):
    match=re.search(r'static (?:void\*?|int) '+name+r'\([^)]*\)\s*\{',source)
    if not match:raise ValueError(f'missing fixture: {name}')
    start=match.start();end=match.end();depth=1
    while depth:depth+=(source[end]=='{')-(source[end]=='}');end+=1
    result=source[start:end]
    for old,new in mapping.items():result=result.replace('meshopt_'+old+'(','captured'+new+'(')
    return result
bodies=[body(n)for n in ['customAlloc','customFree','remapCustomFalse','remapCustomTrue','validatePositionExponent',*names]]
main='int main(int argc,char** argv){if(argc!=2)return 1;root=argv[1];\n'
for name in names:
    main+=f'label="{name}";{name}('+('0'if name=='simplifyUpdateLocked'else '')+');\n'
    if name=='simplifyUpdateLocked':main+='label="simplifyUpdateLockedSparse";simplifyUpdateLocked(meshopt_SimplifySparse);\n'
destination.write_text(preamble+'\n'.join(bodies)+main+'}\n')
