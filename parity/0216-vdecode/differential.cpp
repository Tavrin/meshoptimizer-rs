#include "meshoptimizer.h"
#include <cstdint>
#include <iostream>
#include <vector>
static uint32_t word(const unsigned char* p) { return uint32_t(p[0]) | uint32_t(p[1])<<8 | uint32_t(p[2])<<16 | uint32_t(p[3])<<24; }
static void put(uint32_t v) { for(int j=0;j<4;j++) std::cout.put(char(v>>(j*8))); }
int main() {
    unsigned char h[4];
    while(std::cin.read(reinterpret_cast<char*>(h),4)) {
        std::vector<unsigned char> in(word(h));
        if(in.size()<12 || !std::cin.read(reinterpret_cast<char*>(in.data()),in.size())) return 2;
        size_t count=word(in.data()), stride=word(in.data()+4), len=word(in.data()+8);
        if(len!=in.size()-12 || count>4096 || stride==0 || stride>256 || stride%4) return 3;
        std::vector<unsigned char> out(count*stride+7,0xcc);
        int r=meshopt_decodeVertexBuffer(out.data(),count,stride,in.data()+12,len);
        put(uint32_t(out.size()+4));put(r==0?0:1);
        std::cout.write(reinterpret_cast<const char*>(out.data()),out.size());std::cout.flush();
    }
}
