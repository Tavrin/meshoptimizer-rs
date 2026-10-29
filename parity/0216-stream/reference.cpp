// Unpublished line-oriented index-sequence oracle against the pinned reference.
#include "meshoptimizer.h"
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>
static unsigned nibble(char c) { return c <= '9' ? c - '0' : c - 'a' + 10; }
int main() {
    size_t count;
    std::string hex;
    while (std::cin >> count >> hex) {
        std::vector<unsigned char> source;
        if (hex != "-")
            for (size_t i = 0; i < hex.size(); i += 2)
                source.push_back((nibble(hex[i]) << 4) | nibble(hex[i + 1]));
        std::vector<uint32_t> output(count);
        int status = meshopt_decodeIndexSequence(output.data(), count, 4, source.data(), source.size());
        std::cout << status << ' ';
        if (status == 0) {
            const char* digits = "0123456789abcdef";
            for (uint32_t v : output)
                for (unsigned shift = 0; shift < 32; shift += 8) {
                    unsigned byte = (v >> shift) & 255;
                    std::cout << digits[byte >> 4] << digits[byte & 15];
                }
        }
        std::cout << std::endl;
    }
    return std::cin.eof() ? 0 : 1;
}
