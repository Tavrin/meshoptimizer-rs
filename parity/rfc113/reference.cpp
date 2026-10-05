#include <cstdint>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <vector>
#include <map>
#include <stdexcept>
#include "meshoptimizer.h"

extern "C" int moss_clod_build(const float*, uint32_t, uint32_t, const uint32_t*, uint32_t,
    const unsigned char*, void**, const unsigned char**, size_t*) noexcept;
extern "C" void moss_clod_free(void*) noexcept;

static uint32_t u32(const unsigned char* p) {
    return uint32_t(p[0]) | uint32_t(p[1]) << 8 | uint32_t(p[2]) << 16 | uint32_t(p[3]) << 24;
}
static void put(std::vector<unsigned char>& o, uint32_t x) {
    for (unsigned k=0;k<4;++k) o.push_back(static_cast<unsigned char>(x>>(k*8)));
}
static void append_bounds(std::vector<unsigned char>& o, const meshopt_Bounds& b) {
    for (float f:b.center) { uint32_t bits;std::memcpy(&bits,&f,4);put(o,bits); }
    for (float f:{b.radius,b.cone_apex[0],b.cone_apex[1],b.cone_apex[2],
                  b.cone_axis[0],b.cone_axis[1],b.cone_axis[2],b.cone_cutoff}) {
        uint32_t bits;std::memcpy(&bits,&f,4);put(o,bits);
    }
    for (signed char x:b.cone_axis_s8) o.push_back(static_cast<unsigned char>(x));
    o.push_back(static_cast<unsigned char>(b.cone_cutoff_s8));
}
static bool read_exact(void* p, size_t n) { return std::fread(p, 1, n, stdin) == n; }
int main() {
    for (;;) {
        unsigned char header[4];
        if (!read_exact(header, 4)) return std::feof(stdin) ? 0 : 1;
        size_t size = u32(header);
        if (size > 256u << 20 || size < 16) return 2;
        std::vector<unsigned char> input(size);
        if (!read_exact(input.data(), size)) return 2;
        bool bounds_mode=std::memcmp(input.data(),"R11B",4)==0;
        bool timing_mode=std::memcmp(input.data(),"R11T",4)==0;
        if (!bounds_mode && !timing_mode && std::memcmp(input.data(), "R113", 4)) return 2;
        uint32_t nv = u32(input.data()+4), ni = u32(input.data()+8), stride = u32(input.data()+12);
        if (nv > 4u << 20 || ni > 16u << 20 || stride < 32 || stride > 256 || stride % 4 || ni % 3 ||
            size_t(nv) * stride + size_t(ni) * 4 + nv + 16 != size) return 2;
        auto started=std::chrono::steady_clock::now();
        std::vector<float> vertices(size_t(nv) * stride / 4);
        std::vector<uint32_t> indices(ni);
        const unsigned char* p = input.data()+16;
        for (size_t i=0;i<vertices.size();++i,p+=4) { uint32_t bits=u32(p); std::memcpy(&vertices[i],&bits,4); }
        for (size_t i=0;i<indices.size();++i,p+=4) indices[i]=u32(p);
        void* handle=nullptr; const unsigned char* bytes=nullptr; size_t length=0;
        int status=moss_clod_build(vertices.data(),nv,stride,indices.data(),ni,p,&handle,&bytes,&length);
        uint64_t elapsed=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-started).count();
        if (status || !bytes || length > 256u << 20) { std::fprintf(stderr,"moss_clod_build status %d\n",status); moss_clod_free(handle); return 3; }
        std::vector<unsigned char> augmented;
        if (bounds_mode) {
            augmented.assign(bytes,bytes+length);
            size_t vertices_offset=48;
            size_t indices_offset=vertices_offset+size_t(u32(bytes+20))*16;
            size_t clusters_offset=indices_offset+size_t(u32(bytes+24))*4;
            size_t clusters=u32(bytes+28);
            put(augmented,0x32444e42);put(augmented,static_cast<uint32_t>(clusters));
            for (size_t ci=0;ci<clusters;++ci) {
                const unsigned char* c=bytes+clusters_offset+ci*32;
                size_t first=u32(c),count=u32(c+4);
                std::vector<unsigned int> source(count),local_vertices;
                std::vector<unsigned char> local_triangles;
                std::map<unsigned int,unsigned char> remap;
                for (size_t k=0;k<count;++k) {
                    uint32_t v=u32(bytes+indices_offset+(first+k)*4);
                    uint32_t src=u32(bytes+vertices_offset+size_t(v)*16);
                    source[k]=src;
                    auto it=remap.find(src);
                    if (it==remap.end()) {
                        unsigned char id=static_cast<unsigned char>(local_vertices.size());
                        remap.emplace(src,id);local_vertices.push_back(src);local_triangles.push_back(id);
                    } else local_triangles.push_back(it->second);
                }
                meshopt_Bounds cluster=meshopt_computeClusterBounds(source.data(),source.size(),vertices.data(),nv,stride);
                meshopt_Bounds meshlet=meshopt_computeMeshletBounds(local_vertices.data(),local_triangles.data(),count/3,vertices.data(),nv,stride);
                append_bounds(augmented,cluster);append_bounds(augmented,meshlet);
            }
            bytes=augmented.data();length=augmented.size();
        }
        if (timing_mode) {
            augmented.assign(bytes,bytes+length);
            for (unsigned k=0;k<8;++k) augmented.push_back(static_cast<unsigned char>(elapsed>>(k*8)));
            bytes=augmented.data();length=augmented.size();
        }
        unsigned char n[4]={static_cast<unsigned char>(length),static_cast<unsigned char>(length>>8),
            static_cast<unsigned char>(length>>16),static_cast<unsigned char>(length>>24)};
        bool ok=std::fwrite(n,1,4,stdout)==4 && std::fwrite(bytes,1,length,stdout)==length && std::fflush(stdout)==0;
        moss_clod_free(handle);
        if (!ok) return 4;
    }
}
