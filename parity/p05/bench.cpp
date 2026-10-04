#include "meshoptimizer.h"
#include <chrono>
#include <vector>
#include <stdint.h>

static volatile unsigned int sink;

extern "C" double p05_bench(int op, int caller, int iterations, unsigned seed, size_t vertex_count, size_t index_count,
    const unsigned int* indices, const float* positions, const float* normals, const float* uvs, const unsigned char* texture)
{
    std::vector<unsigned int> persistent(meshopt_stripifyBound(index_count) + meshopt_unstripifyBound(index_count * 5 / 3) + 16);
    std::vector<unsigned char> bytes(1 << 16);
    std::vector<float> floats(index_count * 4 + 16);
    unsigned restart = (seed & 1) ? 65535 : 0;
    std::vector<unsigned> strip;
    size_t strip_size = 0;
    if (op == 2) { strip.resize(meshopt_stripifyBound(index_count)); strip_size = meshopt_stripify(strip.data(), indices, index_count, vertex_count, restart); }
    size_t remesh_bound = 0;
    if (op == 14 && caller) { remesh_bound = meshopt_remesh(NULL,0,indices,index_count,positions,vertex_count,12,4+seed%5,seed%4); floats.resize(remesh_bound*9); }
    auto start = std::chrono::steady_clock::now();
    for (int i = 0; i < iterations; ++i)
    {
        switch (op)
        {
        case 0: { // stripify
            if (caller) sink ^= unsigned(meshopt_stripify(persistent.data(), indices, index_count, vertex_count, restart));
            else { std::vector<unsigned> out(meshopt_stripifyBound(index_count)); sink ^= unsigned(meshopt_stripify(out.data(), indices, index_count, vertex_count, restart)); }
            break;
        }
        case 1: sink ^= unsigned(meshopt_stripifyBound(index_count)); break;
        case 2: { // unstripify includes strip preparation outside the timed routine in both drivers
            if (caller) sink ^= unsigned(meshopt_unstripify(persistent.data(), strip.data(), strip_size, restart));
            else { std::vector<unsigned> out(meshopt_unstripifyBound(strip_size)); sink ^= unsigned(meshopt_unstripify(out.data(), strip.data(), strip_size, restart)); }
            break;
        }
        case 3: sink ^= unsigned(meshopt_unstripifyBound(index_count)); break;
        case 4: { auto s=meshopt_analyzeVertexCache(indices,index_count,vertex_count,3+seed%24,(seed&1)?16:0,seed%4); sink ^= s.vertices_transformed; break; }
        case 5: { auto s=meshopt_analyzeVertexFetch(indices,index_count,vertex_count,4+seed%64); sink ^= s.bytes_fetched; break; }
        case 6: { auto s=meshopt_analyzeOverdraw(indices,index_count,positions,vertex_count,12); sink ^= s.pixels_shaded; break; }
        case 7: { auto s=meshopt_analyzeCoverage(indices,index_count,positions,vertex_count,12); sink ^= unsigned(s.coverage[0]*65536); break; }
        case 8: { std::vector<unsigned char> levels(index_count/3); std::vector<unsigned> sources(index_count/3); std::vector<int> omm(index_count/3);
            sink ^= unsigned(meshopt_opacityMapMeasure(levels.data(),sources.data(),omm.data(),indices,index_count,uvs,vertex_count,8,8,8,seed%5,(seed&1)?2.f:0.f)); break; }
        case 9: { int level=seed%4, states=(seed&1)?4:2; if(caller) { meshopt_opacityMapRasterize(bytes.data(),level,states,uvs+indices[0]*2,uvs+indices[1]*2,uvs+indices[2]*2,texture,1,8,8,8); sink ^= bytes[0]; }
            else { std::vector<unsigned char> result(meshopt_opacityMapEntrySize(level,states)); meshopt_opacityMapRasterize(result.data(),level,states,uvs+indices[0]*2,uvs+indices[1]*2,uvs+indices[2]*2,texture,1,8,8,8); sink ^= result[0]; } break; }
        case 10: sink ^= unsigned(meshopt_opacityMapEntrySize(seed%13,(seed&1)?4:2)); break;
        case 11: { int states=(seed&1)?4:2; unsigned char levels[4]={0,1,2,3}; unsigned offsets[4]={}; int omm[6]={0,1,2,3,0,2}; size_t size=0;
            for(int k=0;k<4;k++){ offsets[k]=unsigned(size); size+=meshopt_opacityMapEntrySize(k,states); }
            std::vector<unsigned char> data(size); for(int k=0;k<4;k++) for(size_t j=0;j<meshopt_opacityMapEntrySize(k,states);j++) data[offsets[k]+j]=texture[(k*13+j)%64];
            sink ^= unsigned(meshopt_opacityMapCompact(data.data(),data.size(),levels,offsets,4,omm,6,states)); break; }
        case 12: { if(caller) { meshopt_generateTangents(floats.data(),indices,index_count,positions,vertex_count,12,normals,12,uvs,8,seed%4); sink ^= unsigned(floats[0]*1000); }
            else { std::vector<float> out(index_count*4); meshopt_generateTangents(out.data(),indices,index_count,positions,vertex_count,12,normals,12,uvs,8,seed%4); sink ^= unsigned(out[0]*1000); } break; }
        case 13: { if(caller) { meshopt_generateNormals(floats.data(),indices,index_count,positions,vertex_count,12,(seed%4==0)?0.5f:(seed%4==1)?1.f:(seed%4==2)?2.f:3.f,(seed%4)*0.5f); sink ^= unsigned(floats[0]*1000); }
            else { std::vector<float> out(index_count*3); meshopt_generateNormals(out.data(),indices,index_count,positions,vertex_count,12,(seed%4==0)?0.5f:(seed%4==1)?1.f:(seed%4==2)?2.f:3.f,(seed%4)*0.5f); sink ^= unsigned(out[0]*1000); } break; }
        case 14: { int resolution=4+seed%5; unsigned options=seed%4; size_t bound=caller?remesh_bound:meshopt_remesh(NULL,0,indices,index_count,positions,vertex_count,12,resolution,options);
            if(caller) { sink ^= unsigned(meshopt_remesh(floats.data(),bound,indices,index_count,positions,vertex_count,12,resolution,options)); }
            else { std::vector<float> out(bound*9); sink ^= unsigned(meshopt_remesh(out.data(),bound,indices,index_count,positions,vertex_count,12,resolution,options)); } break; }
        }
    }
    auto end = std::chrono::steady_clock::now();
    return double(std::chrono::duration_cast<std::chrono::nanoseconds>(end-start).count())/iterations;
}
