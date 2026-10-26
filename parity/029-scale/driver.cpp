// Pinned scalar C++ 1.3; matching nonallocating scale-stage boundary.
#include "meshoptimizer.h"
#include <cstdio>
#include <cstring>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
int main(int argc, char** argv) {
    if (argc != 3) return 2;
    std::ifstream input(argv[1], std::ios::binary | std::ios::ate);
    size_t size = size_t(input.tellg()); input.seekg(0);
    std::vector<float> data((size + 3) / 4);
    input.read(reinterpret_cast<char*>(data.data()), size);
    size_t stride = std::stoull(argv[2]), n = size / stride;
    if (!input || size != n * stride) return 3;
    float result = meshopt_simplifyScale(data.data(), n, stride);
    unsigned bits; std::memcpy(&bits, &result, 4);
    std::cout << "READY " << bits << std::endl;
    std::string line;
    while (std::getline(std::cin, line) && line != "stop") {
        size_t count = std::stoull(line);
        for (size_t i=0; i<count; ++i) {
            float v = meshopt_simplifyScale(data.data(), n, stride);
            asm volatile("" : : "g"(v) : "memory");
        }
        std::cout << "COUNTED" << std::endl;
    }
}
