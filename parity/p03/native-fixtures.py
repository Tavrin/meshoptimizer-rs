#!/usr/bin/env python3
"""Capture unchanged applicable upstream fixture bodies, preserving assertions."""
import re,sys
from pathlib import Path
reference=Path(sys.argv[1]);dest=Path(sys.argv[2])
text=(reference/'demo/tests.cpp').read_text()
names=['clusterBoundsDegenerate','sphereBounds','meshletsEmpty','meshletsDense','meshletsSparse','meshletsFlex','meshletsMax','extractMeshlet','meshletsSpatial','meshletsSpatialDeep','partitionBasic','partitionSpatial','partitionSpatialMerge']
head=r'''
#include "meshoptimizer.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <string.h>
#include <vector>
#include <cstdio>
#include <string>
#include <cstdint>
static const char* fixture;
static unsigned counter=0;
static std::string directory;
static void word(std::vector<unsigned char>&o,unsigned v){for(int k=0;k<4;k++)o.push_back(v>>(k*8));}
static void real(std::vector<unsigned char>&o,float f){unsigned v;memcpy(&v,&f,4);word(o,v);}
static void capture(unsigned op,const float*p,size_t nv,size_t stride,const unsigned*idx,size_t ni,size_t mv=64,size_t mint=16,size_t mt=64,float weight=0,float split=0,unsigned target=4,const unsigned*counts=nullptr,size_t nc=0,const float*radii=nullptr,size_t rstride=0){
 std::vector<unsigned char>o{'M','O','0','3'};word(o,op);word(o,nv);word(o,ni);word(o,mv);word(o,mint);word(o,mt);real(o,weight);real(o,split);word(o,target);word(o,nc);word(o,0);
 for(size_t i=0;i<nv;i++)for(size_t k=0;k<3;k++)real(o,p?p[i*(stride/4)+k]:0.f);
 for(size_t i=0;i<ni;i++)word(o,idx?idx[i]:0);for(size_t i=0;i<nc;i++)word(o,counts[i]);for(size_t i=0;i<nv;i++)real(o,radii?radii[i*(rstride/4)]:0.f);
 std::string filename=directory+"/native-"+fixture+"-"+std::to_string(counter++)+".input";FILE*f=fopen(filename.c_str(),"wb");assert(f);assert(fwrite(o.data(),1,o.size(),f)==o.size());fclose(f);
}
static meshopt_Bounds cap_cluster(const unsigned*i,size_t ni,const float*p,size_t nv,size_t st){capture(6,p,nv,st,i,ni);return meshopt_computeClusterBounds(i,ni,p,nv,st);}
static meshopt_Bounds cap_sphere(const float*p,size_t nv,size_t st,const float*r,size_t rs){capture(8,p,nv,st,nullptr,0,64,16,64,0,0,r?1:0,nullptr,0,r,rs);return meshopt_computeSphereBounds(p,nv,st,r,rs);}
static size_t cap_build(meshopt_Meshlet*m,unsigned*v,unsigned char*t,const unsigned*i,size_t ni,const float*p,size_t nv,size_t st,size_t mv,size_t mt,float w){capture(1,p,nv,st,i,ni,mv,mt,mt,w);return meshopt_buildMeshlets(m,v,t,i,ni,p,nv,st,mv,mt,w);}
static size_t cap_bound(size_t ni,size_t mv,size_t mt){capture(5,nullptr,0,0,nullptr,ni,mv,mt,mt);return meshopt_buildMeshletsBound(ni,mv,mt);}
static size_t cap_flex(meshopt_Meshlet*m,unsigned*v,unsigned char*t,const unsigned*i,size_t ni,const float*p,size_t nv,size_t st,size_t mv,size_t mint,size_t mt,float w,float sf){capture(3,p,nv,st,i,ni,mv,mint,mt,w,sf);return meshopt_buildMeshletsFlex(m,v,t,i,ni,p,nv,st,mv,mint,mt,w,sf);}
static size_t cap_spatial(meshopt_Meshlet*m,unsigned*v,unsigned char*t,const unsigned*i,size_t ni,const float*p,size_t nv,size_t st,size_t mv,size_t mint,size_t mt,float w){capture(4,p,nv,st,i,ni,mv,mint,mt,w);return meshopt_buildMeshletsSpatial(m,v,t,i,ni,p,nv,st,mv,mint,mt,w);}
static size_t cap_extract(unsigned*v,unsigned char*t,const unsigned*i,size_t ni){capture(11,nullptr,0,0,i,ni);return meshopt_extractMeshletIndices(v,t,i,ni);}
static void cap_optimize(unsigned*v,unsigned char*t,size_t nt,size_t nv){std::vector<unsigned>idx(nt*3);for(size_t i=0;i<nt*3;i++)idx[i]=v[t[i]];capture(9,nullptr,0,0,idx.data(),nt*3);meshopt_optimizeMeshlet(v,t,nt,nv);}
static size_t cap_partition(unsigned*d,const unsigned*i,size_t ni,const unsigned*counts,size_t nc,const float*p,size_t nv,size_t st,size_t target){capture(12,p,nv,st,i,ni,64,16,64,0,0,target|(p?0:0x80000000),counts,nc);return meshopt_partitionClusters(d,i,ni,counts,nc,p,nv,st,target);}
'''
replace={'computeClusterBounds':'cluster','computeSphereBounds':'sphere','buildMeshlets':'build','buildMeshletsBound':'bound','buildMeshletsFlex':'flex','buildMeshletsSpatial':'spatial','extractMeshletIndices':'extract','optimizeMeshlet':'optimize','partitionClusters':'partition'}
functions=[]
for name in names:
 match=re.search(r'static void '+name+r'\(\)\s*\{',text);assert match,name
 start=match.start();at=match.end();depth=1
 while depth:
  depth+=(text[at]=='{')-(text[at]=='}');at+=1
 body=text[start:at]
 body=re.sub(r'meshopt_(\w+)\(',lambda m:'cap_'+replace[m[1]]+'(' if m[1] in replace else m[0],body)
 functions.append(body)
main='\nint main(int argc,char**argv){assert(argc==2);directory=argv[1];\n'+''.join(f'fixture="{n}";{n}();\n' for n in names)+'return 0;}\n'
dest.write_text(head+'\n'.join(functions)+main)
