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
#include <array>
#include <fstream>
#include <cstdlib>
#include <unordered_map>
static bool tracking = true;
static size_t current_bytes = 0, peak_bytes = 0;
static std::unordered_map<void*, size_t> sizes;
static void* tracked_allocate(size_t n) {
    void* p = std::malloc(n);
    if (!p) throw std::bad_alloc();
    if (tracking) { sizes[p] = n; current_bytes += n; peak_bytes = std::max(peak_bytes, current_bytes); }
    return p;
}
static void tracked_free(void* p) {
    if (tracking) { current_bytes -= sizes.at(p); sizes.erase(p); }
    std::free(p);
}
struct Result { std::vector<unsigned int> output; size_t count; uint32_t error; bool has_error; };
static uint32_t read(const std::vector<unsigned char>& in, size_t p) {
    if (p > in.size() || in.size()-p < 4) throw std::runtime_error("short message");
    return uint32_t(in[p]) | uint32_t(in[p+1])<<8 | uint32_t(in[p+2])<<16 | uint32_t(in[p+3])<<24;
}
static float as_float(uint32_t bits) { float v; std::memcpy(&v,&bits,4); return v; }
static uint32_t bits(float value) { uint32_t b; std::memcpy(&b,&value,4); return b; }
static std::vector<char> encoded;
static void write(uint32_t value) { for (int i=0;i<4;++i) encoded.push_back(char(value>>(i*8))); }
int main(int argc, char** argv) {
 try {
    if (std::fegetround()!=FE_TONEAREST) throw std::runtime_error("rounding environment");
    volatile float subnormal=std::numeric_limits<float>::denorm_min();
    volatile float two=2.f;
    if (bits(subnormal*two)!=2) throw std::runtime_error("gradual underflow required");
    bool paired=argc==2;
    if(argc>2) throw std::runtime_error("invalid arguments");
    std::ifstream file;
    if(paired) { file.open(argv[1],std::ios::binary); if(!file) throw std::runtime_error("missing input"); }
    std::istream& source=paired ? static_cast<std::istream&>(file) : std::cin;
    std::vector<unsigned char> in;
    std::array<char, 65536> buffer;
    while (source) {
        source.read(buffer.data(), buffer.size());
        size_t n = size_t(source.gcount());
        if (in.size() + n > 128*1024*1024) throw std::runtime_error("message too large");
        in.insert(in.end(), buffer.data(), buffer.data() + n);
    }
    if (source.bad()) throw std::runtime_error("input failure");
    if(in.size()<28 || std::memcmp(in.data(),"MO01",4)) throw std::runtime_error("bad protocol version");
    uint32_t op=read(in,4), vc=read(in,8), ic=read(in,12), mode=read(in,20), samples=read(in,24);
    float threshold=as_float(read(in,16));
    if(op<1 || op>6 || mode>2 || samples>100 || (mode && samples<1) || (!mode && samples)) throw std::runtime_error("unsupported mode");
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
    meshopt_setAllocator(tracked_allocate, tracked_free);
    std::vector<unsigned int> destination(mode==2 ? ic : 0);
    auto operation=[&]() -> Result {
       // Scale has no topology input; do not charge unrelated index validation/allocation.
       if(op==6) {
          for(float v:positions) if(!std::isfinite(v)) throw std::runtime_error("invalid geometry");
          return {{}, 0, bits(meshopt_simplifyScale(positions.data(),vc,12)), true};
       }
       if(op!=3) {
         if(ic%3) throw std::runtime_error("invalid topology");
         for(auto i:indices) if(i>=vc) throw std::runtime_error("invalid index");
         if(op==2) { if(!std::isfinite(threshold)) throw std::runtime_error("invalid threshold"); for(auto p:positions) if(!std::isfinite(p)) throw std::runtime_error("invalid geometry"); }
       }
       Result result{{}, ic, 0, false};
       if(mode!=2) result.output.resize(ic);
       unsigned int* out = mode==2 ? destination.data() : result.output.data();
       if(op==1) meshopt_optimizeVertexCache(out,indices.data(),ic,vc);
       if(op==2) meshopt_optimizeOverdraw(out,indices.data(),ic,positions.data(),vc,12,threshold);
       if(op==3) { if(vc || mode) throw std::runtime_error("invalid math probe"); for(size_t i=0;i<ic;++i) out[i]=bits(std::sqrt(as_float(indices[i]))); }
       if(op>=4) {
          if(target>ic || !std::isfinite(error) || error<0 || options & ~(1|4|16|32|64)) throw std::runtime_error("invalid settings");
          for(float v:positions) if(!std::isfinite(v)) throw std::runtime_error("invalid geometry");
          for(float w:weights) if(!std::isfinite(w) || w<0) throw std::runtime_error("invalid weight");
          for(float v:attributes) if(!std::isfinite(v)) throw std::runtime_error("invalid attribute");
          if(op==6) return {{}, 0, bits(meshopt_simplifyScale(positions.data(),vc,12)), true};
          float result_error=0;
          size_t count=op==4 ? meshopt_simplify(out,indices.data(),ic,positions.data(),vc,12,target,error,options,&result_error) : meshopt_simplifyWithAttributes(out,indices.data(),ic,positions.data(),vc,12,attributes.data(),ac*4,weights.data(),ac,flags.data(),target,error,options,&result_error);
          result.count=count; result.error=bits(result_error); result.has_error=true;
       }
       return result;
    };
    auto out=operation(); // memory measurement and one warm-up
    size_t memory=peak_bytes + (mode!=2 && op!=6 ? size_t(ic)*4 : 0);
    tracking=false;
    auto expected_output = mode==2 ? destination : out.output;
    const size_t expected_count=out.count;
    const uint32_t expected_error=out.error;
    std::vector<double> times;
    if(paired) { if(!mode) throw std::runtime_error("paired mode requires timing"); std::cout.put('R'); std::cout.flush(); }
    for(uint32_t s=0;s<samples;++s) {
       if(paired) { char command; if(!std::cin.get(command)) throw std::runtime_error("short paired command"); if(command=='S') break; if(command!='R') throw std::runtime_error("invalid paired command"); }
       auto start=std::chrono::steady_clock::now();
       unsigned int repeats=ic<3000 ? 64 : 1;
       for(unsigned int j=0;j<repeats;++j) out=operation();
       auto end=std::chrono::steady_clock::now();
       if(out.count!=expected_count || out.error!=expected_error || (mode==2 ? destination : out.output)!=expected_output) throw std::runtime_error("non-deterministic output");
       double seconds=std::chrono::duration<double>(end-start).count()/repeats;
       times.push_back(seconds);
       if(paired) { uint64_t b; std::memcpy(&b,&seconds,8); char sample[8]; for(int i=0;i<8;++i) sample[i]=char(b>>(i*8)); std::cout.put('T'); std::cout.write(sample,8); std::cout.flush(); }
    }
    encoded.reserve(24 + (out.count + out.has_error)*4 + times.size()*8);
    encoded.insert(encoded.end(), {'M','R','0','1'}); write(0); write(uint32_t(out.count + out.has_error)); write(uint32_t(times.size()));
    if(out.has_error) write(out.error);
    for(size_t i=0;i<out.count;++i) write(mode==2 ? destination[i] : out.output[i]);
    for(double t:times) { uint64_t b; std::memcpy(&b,&t,8); for(int i=0;i<8;++i) encoded.push_back(char(b>>(i*8))); }
    if(mode) { write(uint32_t(memory)); write(uint32_t(memory>>32)); }
    std::cout.write(encoded.data(), std::streamsize(encoded.size()));
    return std::cout.good()?0:1;
 } catch(const std::exception& e) { std::cerr<<e.what()<<'\n'; return 1; }
}
