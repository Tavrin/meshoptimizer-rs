// Standalone invocation of the existing pinned C++ differential, no timers.
#include "meshoptimizer.h"
#include <cstdint>
#include <fstream>
#include <iterator>
#include <vector>
#include <cstring>
#include <cassert>
template<class T> std::vector<T> read(const char* path) {
    std::ifstream input(path, std::ios::binary | std::ios::ate);
    assert(input.good());
    size_t size = size_t(input.tellg());
    assert(size % sizeof(T) == 0);
    input.seekg(0);
    std::vector<T> result(size / sizeof(T));
    input.read(reinterpret_cast<char*>(result.data()), size);
    assert(input.good());
    return result;
}
int main(int argc, char** argv) {
    assert(argc == 4);
    auto p = read<float>(argv[1]);
    auto i = read<unsigned>(argv[2]);
    std::vector<unsigned> out(i.size());
    meshopt_optimizeOverdraw(out.data(), i.data(), i.size(), p.data(), p.size()/16, 64, 1.05f);
    std::ofstream output(argv[3], std::ios::binary);
    output.write(reinterpret_cast<const char*>(out.data()), out.size()*4);
    return output.good() ? 0 : 1;
}
