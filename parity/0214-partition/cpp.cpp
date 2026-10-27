#include "meshoptimizer.h"
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <vector>

static uint32_t word() { uint32_t x; if (fread(&x, 4, 1, stdin) != 1) std::abort(); return x; }
int main() {
    const uint32_t nv = word(), ni = word(), nc = word(), target = word(), spatial = word();
    std::vector<float> positions(nv * 3);
    std::vector<uint32_t> indices(ni), counts(nc), output(nc);
    if (fread(positions.data(), 4, positions.size(), stdin) != positions.size()) return 2;
    if (fread(indices.data(), 4, indices.size(), stdin) != indices.size()) return 2;
    if (fread(counts.data(), 4, counts.size(), stdin) != counts.size()) return 2;
    const uint32_t result = static_cast<uint32_t>(meshopt_partitionClusters(output.data(), indices.data(), indices.size(), counts.data(), counts.size(), spatial ? positions.data() : nullptr, nv, 12, target));
    if (fwrite(&result, 4, 1, stdout) != 1 || fwrite(output.data(), 4, output.size(), stdout) != output.size()) return 3;
    return 0;
}
