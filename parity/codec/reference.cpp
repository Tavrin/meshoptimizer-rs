// Unpublished decoder/fixture oracle; pinned unmodified meshoptimizer 1.3.
#include "meshoptimizer.h"
#include <algorithm>
#include <chrono>
#include <cfenv>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <limits>
#include <vector>

static uint32_t word(const unsigned char* p) {
    return uint32_t(p[0]) | uint32_t(p[1])<<8 | uint32_t(p[2])<<16 | uint32_t(p[3])<<24;
}
static void put(std::vector<unsigned char>& b, uint32_t v) {
    for (int i=0; i<4; ++i) b.push_back(static_cast<unsigned char>(v>>(8*i)));
}
int main() {
    volatile float normal=std::numeric_limits<float>::min();
    if (std::fegetround()!=FE_TONEAREST || normal*0.5f==0.f) return 6;
    for (;;) {
        unsigned char size[4];
        if (!std::cin.read(reinterpret_cast<char*>(size),4)) return std::cin.eof()?0:1;
        size_t n=word(size);
        if (n<44 || n>128*1024*1024) return 2;
        std::vector<unsigned char> b(n);
        if (!std::cin.read(reinterpret_cast<char*>(b.data()),n) || std::memcmp(b.data(),"MC02",4)) return 3;
        uint32_t op=word(&b[4]), count=word(&b[8]), stride=word(&b[12]), mode=word(&b[16]), filter=word(&b[20]);
        uint32_t version=word(&b[24]), level=word(&b[28]), samples=word(&b[32]), iterations=word(&b[36]), length=word(&b[40]);
        if (length!=n-44 || count>32*1024*1024 || stride>256 || size_t(count)*stride>128*1024*1024 || samples>100 || !iterations || iterations>1000000) return 4;
        const unsigned char* src=&b[44];
        std::vector<unsigned char> result;
        int status=0;
        auto once=[&]() {
            status=0;
            if (op==8 || op==9) {
                int v=op==8?meshopt_decodeVertexVersion(src,length):meshopt_decodeIndexVersion(src,length);
                result.clear(); put(result,uint32_t(v)); return;
            }
            if (op>=11 && op<=16) {
                if (version>1 || (op==11 && level>9) || level>24 || !stride || !count) {status=-1; return;}
                if (op==11) {
                    result.resize(meshopt_encodeVertexBufferBound(count,stride));
                    size_t used=meshopt_encodeVertexBufferLevel(result.data(),result.size(),src,count,stride,level,version);
                    result.resize(used);
                } else if (op==12 || op==13) {
                    std::vector<unsigned int> indices(count);
                    for (size_t i=0;i<count;++i) indices[i]=word(src+i*4);
                    meshopt_encodeIndexVersion(version);
                    size_t bound=op==12?meshopt_encodeIndexBufferBound(count,0x100000000ull):meshopt_encodeIndexSequenceBound(count,0x100000000ull);
                    result.resize(bound);
                    size_t used=op==12?meshopt_encodeIndexBuffer(result.data(),bound,indices.data(),count):meshopt_encodeIndexSequence(result.data(),bound,indices.data(),count);
                    result.resize(used);
                } else {
                    result.resize(size_t(count)*stride);
                    std::vector<float> values(length/4);
                    std::memcpy(values.data(),src,length);
                    if(op==14) meshopt_encodeFilterOct(result.data(),count,stride,level,values.data());
                    if(op==15) meshopt_encodeFilterQuat(result.data(),count,stride,level,values.data());
                    if(op==16) meshopt_encodeFilterExp(result.data(),count,stride,level,values.data(),meshopt_EncodeExpSeparate);
                }
                return;
            }
            uint32_t raw=op==7?(mode&127)+1:op;
            if (!stride || (raw==1 && (stride%4 || stride>256)) || ((raw==2 || raw==3) && stride!=2 && stride!=4) || (raw==2 && count%3)) {status=-1;return;}
            if (op>=4 && op<=6 && length!=size_t(count)*stride) {status=-1;return;}
            if (mode&128) result.resize(size_t(count)*stride);
            else if (op>=4 && op<=6) result=std::vector<unsigned char>(src,src+length);
            else result=std::vector<unsigned char>(size_t(count)*stride,0);
            if(raw==1) status=meshopt_decodeVertexBuffer(result.data(),count,stride,src,length);
            if(raw==2) status=meshopt_decodeIndexBuffer(result.data(),count,stride,src,length);
            if(raw==3) status=meshopt_decodeIndexSequence(result.data(),count,stride,src,length);
            if(op>=4 && op<=6 && (mode&128)) std::copy(src,src+length,result.begin());
            uint32_t f=op==7?filter:(op>=4 && op<=6?op-3:0);
            if(status==0 && f) {
                if (f==1 && (stride==4 || stride==8)) meshopt_decodeFilterOct(result.data(),count,stride);
                else if (f==2 && stride==8) meshopt_decodeFilterQuat(result.data(),count,stride);
                else if (f==3 && stride%4==0) meshopt_decodeFilterExp(result.data(),count,stride);
                else status=-1;
            }
        };
        once();
        std::vector<double> times;
        for (uint32_t s=0;s<samples;++s) {
            auto start=std::chrono::steady_clock::now();
            for(uint32_t i=0;i<iterations;++i) once();
            times.push_back(std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()/iterations);
        }
        if(status) result.clear();
        std::vector<unsigned char> out={'M','D','0','2'};
        put(out,uint32_t(status)); put(out,uint32_t(result.size())); put(out,uint32_t(times.size()));
        out.insert(out.end(),result.begin(),result.end());
        for(double t:times) {uint64_t bits; std::memcpy(&bits,&t,8);for(int i=0;i<8;++i) out.push_back(static_cast<unsigned char>(bits>>(8*i)));}
        unsigned char os[4];for(int i=0;i<4;++i)os[i]=static_cast<unsigned char>(out.size()>>(8*i));
        std::cout.write(reinterpret_cast<char*>(os),4).write(reinterpret_cast<char*>(out.data()),out.size()).flush();
    }
}
