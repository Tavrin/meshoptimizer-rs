#include "meshoptimizer.h"
#include <chrono>
#include <vector>
#include <stdint.h>
#include <unordered_map>

static bool memory_tracking;
static size_t memory_live;
static size_t memory_peak;
static std::unordered_map<void*, size_t> memory_sizes;

static void* tracked_allocate(size_t size)
{
    void* result = ::operator new(size);
    if (memory_tracking)
    {
        memory_sizes[result] = size;
        memory_live += size;
        if (memory_live > memory_peak) memory_peak = memory_live;
    }
    return result;
}

static void tracked_deallocate(void* pointer)
{
    if (memory_tracking)
    {
        std::unordered_map<void*, size_t>::iterator it = memory_sizes.find(pointer);
        if (it != memory_sizes.end())
        {
            memory_live -= it->second;
            memory_sizes.erase(it);
        }
    }
    ::operator delete(pointer);
}

extern "C" void p05_install_memory() { meshopt_setAllocator(tracked_allocate, tracked_deallocate); }
extern "C" size_t p05_peak_memory() { return memory_peak; }
extern "C" void p05_track_memory(int enabled)
{
    memory_tracking = enabled != 0;
    memory_live = 0;
    memory_peak = 0;
    memory_sizes.clear();
}

static volatile unsigned int sink;

template <typename T> static inline void consume(T value)
{
    // Match Rust's black_box: retain each varying result without a volatile
    // read/modify/write cost in the timed loop.
    asm volatile("" : : "g"(value) : "memory");
}

template <typename T> static void commit_caller_storage(std::vector<T>& values)
{
    const size_t step = 4096 / sizeof(T) ? 4096 / sizeof(T) : 1;
    for (size_t i = 0; i < values.size(); i += step)
    {
        T zero = T();
        asm volatile("" : "+g"(zero) : : "memory");
        values[i] = zero;
    }
    if (!values.empty())
    {
        T zero = T();
        asm volatile("" : "+g"(zero) : : "memory");
        values.back() = zero;
    }
}

// Dispatch once before timing, matching the Rust monomorphized API loops.
template <typename Call> __attribute__((noinline)) static double timed_iterations(int iterations, Call call)
{
    auto start = std::chrono::steady_clock::now();
    for (int i = 0; i < iterations; ++i) call();
    auto end = std::chrono::steady_clock::now();
    return double(std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count()) / iterations;
}

extern "C" double p05_bench(int op, int caller, int iterations, unsigned seed, size_t vertex_count, size_t index_count,
    const unsigned int* indices, const float* positions, const float* normals, const float* uvs, const unsigned char* texture)
{
    // Match Rust: only the selected caller API's exact bound is resident.
    // Owned outputs and scratch are still allocated inside the API timer.
    std::vector<unsigned int> persistent;
    if (caller && op == 0) persistent.resize(meshopt_stripifyBound(index_count));
    std::vector<unsigned char> bytes;
    if (caller && op == 9) bytes.resize(meshopt_opacityMapEntrySize(seed % 4, (seed & 1) ? 4 : 2));
    std::vector<float> floats;
    if (caller && op == 12) floats.resize(index_count * 4);
    if (caller && op == 13) floats.resize(index_count * 3);
    unsigned restart = (seed & 1) ? 65535 : 0;
    std::vector<unsigned> strip;
    size_t strip_size = 0;
    if (op == 2) { strip.resize(meshopt_stripifyBound(index_count)); strip_size = meshopt_stripify(strip.data(), indices, index_count, vertex_count, restart); }
    const unsigned int* const strip_data = strip.data();
    if (caller && op == 2) persistent.resize(meshopt_unstripifyBound(strip_size));
    size_t remesh_bound = 0;
    if (op == 14 && caller) { remesh_bound = meshopt_remesh(NULL,0,indices,index_count,positions,vertex_count,12,4+seed%5,seed%4); floats.resize(remesh_bound*9); }
    if (caller)
    {
        if (op == 0 || op == 2) commit_caller_storage(persistent);
        else if (op == 9) commit_caller_storage(bytes);
        else if (op == 12 || op == 13 || op == 14) commit_caller_storage(floats);
    }
    if (memory_tracking) { memory_live = 0; memory_peak = 0; memory_sizes.clear(); }
    if (op == 1 || op == 3 || op == 10)
    {
        auto start = std::chrono::steady_clock::now();
        if (op == 1)
            for (int i = 0; i < iterations; ++i) consume(unsigned(meshopt_stripifyBound(index_count + size_t(i & 3) * 3)));
        else if (op == 3)
            for (int i = 0; i < iterations; ++i) consume(unsigned(meshopt_unstripifyBound(index_count + size_t(i & 3))));
        else
            for (int i = 0; i < iterations; ++i) consume(unsigned(meshopt_opacityMapEntrySize((seed + unsigned(i)) % 13, ((seed + unsigned(i)) & 1) ? 4 : 2)));
        auto end = std::chrono::steady_clock::now();
        return double(std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count()) / iterations;
    }
    switch (op)
    {
    case 0: {
        if (caller) return timed_iterations(iterations, [&]() { // stripify
             sink ^= unsigned(meshopt_stripify(persistent.data(), indices, index_count, vertex_count, restart)); consume(persistent.data());
        });
        return timed_iterations(iterations, [&]() { // stripify
             std::vector<unsigned> out(meshopt_stripifyBound(index_count)); sink ^= unsigned(meshopt_stripify(out.data(), indices, index_count, vertex_count, restart)); consume(out.data());
        });
    }
    case 2: {
        if (caller) return timed_iterations(iterations, [&]() { // unstripify includes strip preparation outside the timed routine in both drivers
             sink ^= unsigned(meshopt_unstripify(persistent.data(), strip_data, strip_size, restart)); consume(persistent.data());
        });
        return timed_iterations(iterations, [&]() { // unstripify includes strip preparation outside the timed routine in both drivers
             std::vector<unsigned> out(meshopt_unstripifyBound(strip_size)); sink ^= unsigned(meshopt_unstripify(out.data(), strip_data, strip_size, restart)); consume(out.data());
        });
    }
    case 4: return timed_iterations(iterations, [&]() { { auto s=meshopt_analyzeVertexCache(indices,index_count,vertex_count,3+seed%24,(seed&1)?16:0,seed%4); consume(&s);  } });
    case 5: return timed_iterations(iterations, [&]() { { auto s=meshopt_analyzeVertexFetch(indices,index_count,vertex_count,4+seed%64); consume(&s);  } });
    case 6: return timed_iterations(iterations, [&]() { { auto s=meshopt_analyzeOverdraw(indices,index_count,positions,vertex_count,12); consume(&s);  } });
    case 7: return timed_iterations(iterations, [&]() { { auto s=meshopt_analyzeCoverage(indices,index_count,positions,vertex_count,12); consume(&s);  } });
    case 8: return timed_iterations(iterations, [&]() { { std::vector<unsigned char> levels(index_count/3); std::vector<unsigned> sources(index_count/3); std::vector<int> omm(index_count/3);
            sink ^= unsigned(meshopt_opacityMapMeasure(levels.data(),sources.data(),omm.data(),indices,index_count,uvs,vertex_count,8,8,8,seed%5,(seed&1)?2.f:0.f)); consume(levels.data()); consume(sources.data()); consume(omm.data());  } });
    case 9: {
        if (caller) return timed_iterations(iterations, [&]() { int level=seed%4, states=(seed&1)?4:2;  meshopt_opacityMapRasterize(bytes.data(),level,states,uvs+indices[0]*2,uvs+indices[1]*2,uvs+indices[2]*2,texture,1,8,8,8); sink ^= bytes[0]; consume(bytes.data());
        });
        return timed_iterations(iterations, [&]() { int level=seed%4, states=(seed&1)?4:2;  std::vector<unsigned char> result(meshopt_opacityMapEntrySize(level,states)); meshopt_opacityMapRasterize(result.data(),level,states,uvs+indices[0]*2,uvs+indices[1]*2,uvs+indices[2]*2,texture,1,8,8,8); sink ^= result[0]; consume(result.data()); consume(result.size());
        });
    }
    case 11: return timed_iterations(iterations, [&]() { { int states=(seed&1)?4:2; unsigned char levels[4]={0,1,2,3}; unsigned offsets[4]={}; int omm[6]={0,1,2,3,0,2}; size_t size=0;
            for(int k=0;k<4;k++){ offsets[k]=unsigned(size); size+=meshopt_opacityMapEntrySize(k,states); }
            std::vector<unsigned char> data(size); for(int k=0;k<4;k++) for(size_t j=0;j<meshopt_opacityMapEntrySize(k,states);j++) data[offsets[k]+j]=texture[(k*13+j)%64];
            sink ^= unsigned(meshopt_opacityMapCompact(data.data(),data.size(),levels,offsets,4,omm,6,states)); consume(data.data()); consume(levels); consume(offsets); consume(omm);  } });
    case 12: {
        if (caller) return timed_iterations(iterations, [&]() {  meshopt_generateTangents(floats.data(),indices,index_count,positions,vertex_count,12,normals,12,uvs,8,seed%4); consume(floats.data());
        });
        return timed_iterations(iterations, [&]() {  std::vector<float> out(index_count*4); meshopt_generateTangents(out.data(),indices,index_count,positions,vertex_count,12,normals,12,uvs,8,seed%4); consume(out.data()); consume(out.size());
        });
    }
    case 13: {
        if (caller) return timed_iterations(iterations, [&]() {  meshopt_generateNormals(floats.data(),indices,index_count,positions,vertex_count,12,(seed%4==0)?0.5f:(seed%4==1)?1.f:(seed%4==2)?2.f:3.f,(seed%4)*0.5f); consume(floats.data());
        });
        return timed_iterations(iterations, [&]() {  std::vector<float> out(index_count*3); meshopt_generateNormals(out.data(),indices,index_count,positions,vertex_count,12,(seed%4==0)?0.5f:(seed%4==1)?1.f:(seed%4==2)?2.f:3.f,(seed%4)*0.5f); consume(out.data()); consume(out.size());
        });
    }
    case 14: {
        if (caller) return timed_iterations(iterations, [&]() { int resolution=4+seed%5; unsigned options=seed%4; size_t bound=caller?remesh_bound:meshopt_remesh(NULL,0,indices,index_count,positions,vertex_count,12,resolution,options);
             sink ^= unsigned(meshopt_remesh(floats.data(),bound,indices,index_count,positions,vertex_count,12,resolution,options)); consume(floats.data());
        });
        return timed_iterations(iterations, [&]() { int resolution=4+seed%5; unsigned options=seed%4; size_t bound=caller?remesh_bound:meshopt_remesh(NULL,0,indices,index_count,positions,vertex_count,12,resolution,options);
             std::vector<float> out(bound*9); sink ^= unsigned(meshopt_remesh(out.data(),bound,indices,index_count,positions,vertex_count,12,resolution,options)); consume(out.data()); consume(out.size());
        });
    }
    }
    return 0;
}
