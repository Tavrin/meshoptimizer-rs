// Unpublished decoder/fixture oracle; pinned unmodified meshoptimizer 1.3.
#include "meshoptimizer.h"
#include <algorithm>
#include <chrono>
#include <cfenv>
#include <cstdint>
#include <cmath>
#include <cstring>
#include <iostream>
#include <limits>
#include <vector>

static uint32_t word(const unsigned char* p) {
    return uint32_t(p[0]) | uint32_t(p[1])<<8 | uint32_t(p[2])<<16 | uint32_t(p[3])<<24;
}
// Color decoding converts float(c) * (max / alpha_scale) + 0.5f to int; the
// conversion is undefined for a zero alpha word or out-of-range 16-bit values.
static bool colorDefined(const unsigned char* p, size_t count, size_t stride) {
    for (size_t i=0;i<count;++i) {
        const unsigned char* e=p+i*stride;
        int c[4];
        for (int k=0;k<4;++k) c[k]=stride==4?e[k]:int(e[2*k]|(e[2*k+1]<<8));
        int y=c[0];
        int co=stride==4?int(static_cast<signed char>(c[1])):int(static_cast<short>(c[1]));
        int cg=stride==4?int(static_cast<signed char>(c[2])):int(static_cast<short>(c[2]));
        int as=c[3]; as|=as>>1; as|=as>>2; as|=as>>4; as|=as>>8;
        int a=((c[3]<<1)&as)|(c[3]&1);
        if (!as) return false;
        float ss=(stride==4?255.f:65535.f)/float(as);
        for (int v:{y+co-cg,y+cg,y-co-cg,a}) {
            float f=float(v)*ss+0.5f;
            if (!(f>=-2147483648.f && f<2147483648.f)) return false;
        }
    }
    return true;
}
// Reference assertions and undefined-conversion domains of operations 11-21:
// -1 rejects parameters (as Rust does), -3 marks undefined behaviour (no oracle).
static int prepare04(uint32_t op, uint32_t count, uint32_t stride, uint32_t mode, uint32_t version, uint32_t level,
                     const unsigned char* src, size_t length, std::vector<unsigned int>& indices, std::vector<float>& values) {
    if (op==11 && (version>1 || level>9 || !stride || stride%4 || length!=size_t(count)*stride)) return -1;
    if (op==12 || op==13) {
        if (version>1 || length!=size_t(count)*4 || (op==12 && count%3)) return -1;
        indices.resize(count);
        for (size_t i=0;i<count;++i) indices[i]=word(src+i*4);
        if (op==13) {
            // Baseline selection negates int(index - last); INT_MIN is undefined.
            unsigned int last[2]={0,0}; int current=0;
            for (unsigned int index:indices) {
                if (index-last[current]==0x80000000u) return -3;
                int cd=int(index-last[current]); current^=((cd<0?-cd:cd)>=30); last[current]=index;
            }
        }
    }
    if (op>=14 && op<=17) {
        bool octlike=op==14 || op==17;
        if (octlike && ((stride!=4 && stride!=8) || level<2 || level>16 || (stride==4 && level>8))) return -1;
        if (op==15 && (stride!=8 || level<4 || level>16)) return -1;
        if (op==16 && (!stride || stride%4 || level<1 || level>24 || (mode&127)>3)) return -1;
        if (length!=size_t(count)*(op==16?stride:16)) return -1;
        values.resize(length/4);
        if (length) std::memcpy(values.data(),src,length);
        // Exp converts v * 2^-e to int: non-finite input, or a one-bit mantissa
        // with exponent 128, is undefined; Rust reports NumericalFailure there.
        if (op==16) for (float v:values) if (!std::isfinite(v) || (level==1 && std::fabs(v)>=0x1p127f)) return -1;
    }
    if (op==18 && ((stride!=4 && stride!=8) || length!=size_t(count)*stride || !colorDefined(src,count,stride))) return -1;
    if (op==19) {
        if (count>256 || version>256 || length!=size_t(count)*4+size_t(version)*3) return -1;
        indices.resize(count);
        for (size_t i=0;i<count;++i) indices[i]=word(src+i*4);
    }
    if (op==20 && (count>256 || version>256 || (stride!=2 && stride!=4) || (level!=3 && level!=4))) return -1;
    if (op==21 && (count>256 || version>256)) return -1;
    return 0;
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
        std::vector<unsigned char> result, meshletv, meshlett;
        std::vector<unsigned int> indices;
        std::vector<float> values;
        size_t bound=0;
        bool boundResult=false, prepared=false, meshletResult=false;
        int prestatus=0;
        size_t meshletBytes[2]={0,0};
        int status=0;
        auto once=[&]() {
            status=0; boundResult=false; meshletResult=false;
            if (op==8 || op==9) {
                int v=op==8?meshopt_decodeVertexVersion(src,length):meshopt_decodeIndexVersion(src,length);
                result.clear(); put(result,uint32_t(v)); return;
            }
            if (op>=11) {
                // 0.4 operations. Parameters outside each reference assertion,
                // and inputs whose float-to-int conversion is undefined, are
                // refused before the call (status -1) instead of executed.
                size_t capacity=filter?size_t(filter)-1:0;
                // Validation and input conversion run once, before timing,
                // as the Rust driver's do; timed calls include output
                // allocation (fresh unless the caller-buffer bit is set).
                if (!prepared) {prepared=true; prestatus=prepare04(op,count,stride,mode,version,level,src,length,indices,values);}
                if (prestatus) {status=prestatus; return;}
                auto output=[&](size_t n) {if (mode&128) result.resize(n); else result=std::vector<unsigned char>(n);};
                if (op==11) {
                    output(filter?capacity:meshopt_encodeVertexBufferBound(count,stride));
                    size_t used=meshopt_encodeVertexBufferLevel(result.data(),result.size(),src,count,stride,level,version);
                    result.resize(used); status=used?0:-2;
                } else if (op==12 || op==13) {
                    meshopt_encodeIndexVersion(version);
                    size_t bound=op==12?meshopt_encodeIndexBufferBound(count,0x100000000ull):meshopt_encodeIndexSequenceBound(count,0x100000000ull);
                    output(filter?capacity:bound);
                    size_t used=op==12?meshopt_encodeIndexBuffer(result.data(),result.size(),indices.data(),count):meshopt_encodeIndexSequence(result.data(),result.size(),indices.data(),count);
                    result.resize(used); status=used?0:-2;
                } else if (op>=14 && op<=17) {
                    output(size_t(count)*stride);
                    if(op==14) meshopt_encodeFilterOct(result.data(),count,stride,level,values.data());
                    if(op==15) meshopt_encodeFilterQuat(result.data(),count,stride,level,values.data());
                    if(op==16) meshopt_encodeFilterExp(result.data(),count,stride,level,values.data(),meshopt_EncodeExpMode(mode&127));
                    if(op==17) meshopt_encodeFilterColor(result.data(),count,stride,level,values.data());
                } else if (op==18) {
                    if (mode&128) result.assign(src,src+length); else result=std::vector<unsigned char>(src,src+length);
                    meshopt_decodeFilterColor(result.data(),count,stride);
                } else if (op==19) {
                    output(filter?capacity:meshopt_encodeMeshletBound(count,version));
                    size_t used=meshopt_encodeMeshlet(result.data(),result.size(),count?indices.data():nullptr,count,src+size_t(count)*4,version);
                    result.resize(used); status=used?0:-2;
                } else if (op==20 || op==21) {
                    size_t vs=op==21?4:stride, ts=op==21?4:level;
                    // Upstream requires 4-byte aligned destinations (16 for raw); only counted bytes are compared.
                    size_t vb=size_t(count)*vs, tb=size_t(version)*ts;
                    if (mode&128) {meshletv.resize((vb+15)/16*16+16); meshlett.resize((tb+15)/16*16+16);}
                    else {meshletv=std::vector<unsigned char>((vb+15)/16*16+16); meshlett=std::vector<unsigned char>((tb+15)/16*16+16);}
                    status=op==20?meshopt_decodeMeshlet(count?meshletv.data():nullptr,count,vs,meshlett.data(),version,ts,src,length)
                                 :meshopt_decodeMeshletRaw(reinterpret_cast<unsigned int*>(meshletv.data()),count,reinterpret_cast<unsigned int*>(meshlett.data()),version,src,length);
                    meshletResult=status==0; meshletBytes[0]=vb; meshletBytes[1]=tb;
                } else if (op>=22 && op<=25) {
                    if (op==22 && (!stride || stride%4)) {status=-1; return;}
                    if (op==23 && count%3) {status=-1; return;}
                    if ((op==23 || op==24) && length!=8) {status=-1; return;}
                    uint64_t vertices=0; if (op==23 || op==24) std::memcpy(&vertices,src,8);
                    if ((op==23 || op==24) && vertices>0x100000000ull) {status=-1; return;}
                    auto boundOf=[&](size_t n){return op==22?meshopt_encodeVertexBufferBound(n,stride):op==23?meshopt_encodeIndexBufferBound(n,size_t(vertices)):op==24?meshopt_encodeIndexSequenceBound(n,size_t(vertices)):meshopt_encodeMeshletBound(n,level);};
                    bound=boundOf(count);
                    // 255 more bounds (count + 3i) amortize per-request overhead, as in the Rust driver.
                    size_t sink=0;
                    for (size_t i=1;i<256;++i) sink^=boundOf(size_t(count)+3*i);
                    asm volatile(""::"r"(sink));
                    boundResult=true;
                } else status=-1;
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
        if(!status && meshletResult) {
            result.assign(meshletv.begin(),meshletv.begin()+meshletBytes[0]);
            result.insert(result.end(),meshlett.begin(),meshlett.begin()+meshletBytes[1]);
        }
        if(status) result.clear();
        else if(boundResult) {result.clear(); put(result,uint32_t(uint64_t(bound))); put(result,uint32_t(uint64_t(bound)>>32));}
        std::vector<unsigned char> out={'M','D','0','2'};
        put(out,uint32_t(status)); put(out,uint32_t(result.size())); put(out,uint32_t(times.size()));
        out.insert(out.end(),result.begin(),result.end());
        for(double t:times) {uint64_t bits; std::memcpy(&bits,&t,8);for(int i=0;i<8;++i) out.push_back(static_cast<unsigned char>(bits>>(8*i)));}
        unsigned char os[4];for(int i=0;i<4;++i)os[i]=static_cast<unsigned char>(out.size()>>(8*i));
        std::cout.write(reinterpret_cast<char*>(os),4).write(reinterpret_cast<char*>(out.data()),out.size()).flush();
    }
}
