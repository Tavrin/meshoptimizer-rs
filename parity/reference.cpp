// Unpublished scalar-strict driver. Upstream sources are linked unchanged.
#include "meshoptimizer.h"
#include <cfenv>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <iterator>
#include <limits>
#include <stdexcept>
#include <vector>
static uint32_t read(const std::vector<unsigned char>& in, size_t p) {
    if (p > in.size() || in.size()-p < 4) throw std::runtime_error("short message");
    return uint32_t(in[p]) | uint32_t(in[p+1])<<8 | uint32_t(in[p+2])<<16 | uint32_t(in[p+3])<<24;
}
static float as_float(uint32_t bits) { float v; std::memcpy(&v,&bits,4); return v; }
static uint32_t bits(float value) { uint32_t b; std::memcpy(&b,&value,4); return b; }
static void write(uint32_t value) { for (int i=0;i<4;++i) std::cout.put(char(value>>(i*8))); }
int main() {
 try {
    if (std::fegetround()!=FE_TONEAREST) throw std::runtime_error("rounding environment");
    volatile float subnormal=std::numeric_limits<float>::denorm_min();
    volatile float two=2.f;
    if (bits(subnormal*two)!=2) throw std::runtime_error("gradual underflow required");
    std::vector<unsigned char> in;
    char byte;
    while (std::cin.get(byte)) { if(in.size() >= 128*1024*1024) throw std::runtime_error("message too large"); in.push_back(static_cast<unsigned char>(byte)); }
    if(in.size()<28 || std::memcmp(in.data(),"MO01",4)) throw std::runtime_error("bad protocol version");
    uint32_t op=read(in,4), vc=read(in,8), ic=read(in,12), mode=read(in,20), samples=read(in,24);
    float threshold=as_float(read(in,16));
    if(op<1 || op>6 || mode>1 || samples>100 || (mode && samples<1) || (!mode && samples)) throw std::runtime_error("unsupported mode");
    uint64_t length=28+uint64_t(vc)*12+uint64_t(ic)*4;
    if((op<=3 && length!=in.size()) || length>in.size()) throw std::runtime_error("invalid message length");
    std::vector<float> positions(size_t(vc)*3);
    for(size_t i=0;i<positions.size();++i) positions[i]=as_float(read(in,28+i*4));
    std::vector<unsigned int> indices(ic);
    for(size_t i=0;i<ic;++i) indices[i]=read(in,28+size_t(vc)*12+i*4);
    uint32_t target=0, options=0, ac=0; float error=0;
    std::vector<float> weights, attributes; std::vector<unsigned char> flags;
    if(op>=4) {
       target=read(in,length); error=as_float(read(in,length+4)); options=read(in,length+8); ac=read(in,length+12);
       if(ac>32 || in.size()!=length+16+uint64_t(ac)*4+uint64_t(vc)*ac*4+uint64_t(vc)*4) throw std::runtime_error("invalid simplifier message");
       for(size_t k=0;k<ac;++k) weights.push_back(as_float(read(in,length+16+k*4)));
       for(size_t k=0;k<size_t(vc)*ac;++k) attributes.push_back(as_float(read(in,length+16+ac*4+k*4)));
       for(size_t k=0;k<vc;++k) { auto f=read(in,length+16+ac*4+size_t(vc)*ac*4+k*4); if(f>7) throw std::runtime_error("invalid flags"); flags.push_back(static_cast<unsigned char>(f)); }
    }
    auto operation=[&]() {
       // Scale has no topology input; do not charge unrelated index validation/allocation.
       if(op==6) {
          for(float v:positions) if(!std::isfinite(v)) throw std::runtime_error("invalid geometry");
          return std::vector<unsigned int>{bits(meshopt_simplifyScale(positions.data(),vc,12))};
       }
       if(op!=3) {
         if(ic%3) throw std::runtime_error("invalid topology");
         for(auto i:indices) if(i>=vc) throw std::runtime_error("invalid index");
         if(op==2) { if(!std::isfinite(threshold)) throw std::runtime_error("invalid threshold"); for(auto p:positions) if(!std::isfinite(p)) throw std::runtime_error("invalid geometry"); }
       }
       std::vector<unsigned int> out(ic);
       if(op==1) meshopt_optimizeVertexCache(out.data(),indices.data(),ic,vc);
       if(op==2) meshopt_optimizeOverdraw(out.data(),indices.data(),ic,positions.data(),vc,12,threshold);
       if(op==3) { if(vc || mode) throw std::runtime_error("invalid math probe"); for(size_t i=0;i<ic;++i) out[i]=bits(std::sqrt(as_float(indices[i]))); }
       if(op>=4) {
          if(target>ic || !std::isfinite(error) || error<0 || options & ~(1|4|16|32|64)) throw std::runtime_error("invalid settings");
          for(float v:positions) if(!std::isfinite(v)) throw std::runtime_error("invalid geometry");
          for(float w:weights) if(!std::isfinite(w) || w<0) throw std::runtime_error("invalid weight");
          for(float v:attributes) if(!std::isfinite(v)) throw std::runtime_error("invalid attribute");
          if(op==6) return std::vector<unsigned int>{bits(meshopt_simplifyScale(positions.data(),vc,12))};
          float result_error=0;
          size_t count=op==4 ? meshopt_simplify(out.data(),indices.data(),ic,positions.data(),vc,12,target,error,options,&result_error) : meshopt_simplifyWithAttributes(out.data(),indices.data(),ic,positions.data(),vc,12,attributes.data(),ac*4,weights.data(),ac,flags.data(),target,error,options,&result_error);
          out.resize(count); out.insert(out.begin(),bits(result_error));
       }
       return out;
    };
    auto out=operation();
    std::vector<double> times;
    for(uint32_t s=0;s<samples;++s) {
       auto start=std::chrono::steady_clock::now();
       auto result=operation();
       auto end=std::chrono::steady_clock::now();
       if(result!=out) throw std::runtime_error("non-deterministic output");
       times.push_back(std::chrono::duration<double>(end-start).count());
    }
    std::cout.write("MR01",4); write(0); write(uint32_t(out.size())); write(samples);
    for(auto i:out) write(i);
    for(double t:times) { uint64_t b; std::memcpy(&b,&t,8); for(int i=0;i<8;++i) std::cout.put(char(b>>(i*8))); }
    return std::cout.good()?0:1;
 } catch(const std::exception& e) { std::cerr<<e.what()<<'\n'; return 1; }
}
