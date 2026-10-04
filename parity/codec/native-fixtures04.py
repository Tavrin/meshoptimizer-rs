#!/usr/bin/env python3
"""Export every upstream native invocation of a 0.4 operation, oracle unedited.

Encoders (with the caller's exact capacity and the global version in force),
bounds, filter encoders, Color decoding and the meshlet codec are captured as
harness requests before forwarding to the real function. The upstream test
bodies and their assertions run unchanged.
"""
import re
import sys
from pathlib import Path

reference, out, destination = map(Path, sys.argv[1:])
source = (reference / 'demo/tests.cpp').read_text()
source = source.replace('#include "../src/meshoptimizer.h"', '#include "meshoptimizer.h"')
source = source[:source.index('void runTests(')]
targets = [
    ('meshopt_encodeVertexBufferBound', 'capVertexBound'),
    ('meshopt_encodeVertexBufferLevel', 'capVertexLevel'),
    ('meshopt_encodeVertexBuffer', 'capVertex'),
    ('meshopt_encodeVertexVersion', 'capVertexVersion'),
    ('meshopt_encodeIndexBufferBound', 'capIndexBound'),
    ('meshopt_encodeIndexBuffer', 'capIndex'),
    ('meshopt_encodeIndexSequenceBound', 'capSequenceBound'),
    ('meshopt_encodeIndexSequence', 'capSequence'),
    ('meshopt_encodeIndexVersion', 'capIndexVersion'),
    ('meshopt_encodeFilterOct', 'capOct'),
    ('meshopt_encodeFilterQuat', 'capQuat'),
    ('meshopt_encodeFilterExp', 'capExp'),
    ('meshopt_encodeFilterColor', 'capColor'),
    ('meshopt_decodeFilterColor', 'capDecodeColor'),
    ('meshopt_encodeMeshletBound', 'capMeshletBound'),
    ('meshopt_encodeMeshlet', 'capMeshlet'),
    ('meshopt_decodeMeshletRaw', 'capDecodeMeshletRaw'),
    ('meshopt_decodeMeshlet', 'capDecodeMeshlet'),
]
# Functions whose bodies call a 0.4 operation, in file order.
bodies = re.split(r'\n(?=static void \w+\(\)\n)', source)
functions = []
for body in bodies:
    m = re.match(r'static void (\w+)\(\)\n', body)
    if m and any(name + '(' in body for name, _ in targets):
        functions.append(m.group(1))
for name, wrapper in targets:
    source = re.sub(r'\b' + name + r'\(', wrapper + '(', source)
header = r"""
#include "meshoptimizer.h"
#include <cstdint>
#include <cstring>
#include <fstream>
#include <string>
#include <vector>
static std::string dir, fixture;
static int sequence = 0, gVertexVersion = 1, gIndexVersion = 1;
static void put(std::vector<unsigned char>& b, uint32_t v) { for (int i = 0; i < 4; ++i) b.push_back(static_cast<unsigned char>(v >> (8 * i))); }
static void capture(uint32_t op, size_t n, size_t s, uint32_t mode, size_t filter, uint32_t version, uint32_t level, const void* p, size_t length) {
    std::vector<unsigned char> b = {'M', 'C', '0', '2'};
    for (uint32_t v : {op, uint32_t(n), uint32_t(s), mode, uint32_t(filter), version, level, 0u, 1u, uint32_t(length)}) put(b, v);
    if (length) { const unsigned char* q = static_cast<const unsigned char*>(p); b.insert(b.end(), q, q + length); }
    std::ofstream f(dir + "/native04-" + fixture + "-" + std::to_string(sequence++) + ".input", std::ios::binary);
    f.write(reinterpret_cast<char*>(b.data()), b.size());
}
static void capVertexVersion(int v) { gVertexVersion = v; meshopt_encodeVertexVersion(v); }
static void capIndexVersion(int v) { gIndexVersion = v; meshopt_encodeIndexVersion(v); }
static size_t capVertexBound(size_t n, size_t s) { capture(22, n, s, 0, 0, 0, 0, nullptr, 0); return meshopt_encodeVertexBufferBound(n, s); }
static size_t capVertexLevel(unsigned char* b, size_t bs, const void* v, size_t n, size_t s, int level, int version = -1) {
    capture(11, n, s, 0, bs + 1, uint32_t(version < 0 ? gVertexVersion : version), uint32_t(level), v, n * s);
    return meshopt_encodeVertexBufferLevel(b, bs, v, n, s, level, version);
}
static size_t capVertex(unsigned char* b, size_t bs, const void* v, size_t n, size_t s) { return capVertexLevel(b, bs, v, n, s, 2, -1); }
static void capBound(uint32_t op, size_t n, size_t vertices) { uint64_t v = vertices; capture(op, n, 4, 0, 0, 0, 0, &v, 8); }
static size_t capIndexBound(size_t n, size_t v) { capBound(23, n, v); return meshopt_encodeIndexBufferBound(n, v); }
static size_t capSequenceBound(size_t n, size_t v) { capBound(24, n, v); return meshopt_encodeIndexSequenceBound(n, v); }
static size_t capIndex(unsigned char* b, size_t bs, const unsigned int* idx, size_t n) {
    capture(12, n, 4, 0, bs + 1, uint32_t(gIndexVersion), 2, idx, n * 4); return meshopt_encodeIndexBuffer(b, bs, idx, n);
}
template <typename T> static size_t capIndex(unsigned char* b, size_t bs, const T* idx, size_t n) { std::vector<unsigned int> u(idx, idx + n); return capIndex(b, bs, u.data(), n); }
static size_t capSequence(unsigned char* b, size_t bs, const unsigned int* idx, size_t n) {
    capture(13, n, 4, 0, bs + 1, uint32_t(gIndexVersion), 2, idx, n * 4); return meshopt_encodeIndexSequence(b, bs, idx, n);
}
template <typename T> static size_t capSequence(unsigned char* b, size_t bs, const T* idx, size_t n) { std::vector<unsigned int> u(idx, idx + n); return capSequence(b, bs, u.data(), n); }
static void capOct(void* d, size_t n, size_t s, int bits, const float* p) { capture(14, n, s, 0, 0, 0, uint32_t(bits), p, n * 16); meshopt_encodeFilterOct(d, n, s, bits, p); }
static void capQuat(void* d, size_t n, size_t s, int bits, const float* p) { capture(15, n, s, 0, 0, 0, uint32_t(bits), p, n * 16); meshopt_encodeFilterQuat(d, n, s, bits, p); }
static void capExp(void* d, size_t n, size_t s, int bits, const float* p, meshopt_EncodeExpMode mode) {
    // Inputs may alias the destination upstream; capture before encoding.
    capture(16, n, s, uint32_t(mode), 0, 0, uint32_t(bits), p, n * s); meshopt_encodeFilterExp(d, n, s, bits, p, mode);
}
static void capColor(void* d, size_t n, size_t s, int bits, const float* p) { capture(17, n, s, 0, 0, 0, uint32_t(bits), p, n * 16); meshopt_encodeFilterColor(d, n, s, bits, p); }
static void capDecodeColor(void* p, size_t n, size_t s) { capture(18, n, s, 0, 0, 0, 0, p, n * s); meshopt_decodeFilterColor(p, n, s); }
static size_t capMeshletBound(size_t v, size_t t) { capture(25, v, 0, 0, 0, 0, uint32_t(t), nullptr, 0); return meshopt_encodeMeshletBound(v, t); }
static size_t capMeshlet(unsigned char* b, size_t bs, const unsigned int* v, size_t vc, const unsigned char* t, size_t tc) {
    std::vector<unsigned char> data(vc * 4 + tc * 3);
    if (vc) std::memcpy(data.data(), v, vc * 4);
    if (tc) std::memcpy(data.data() + vc * 4, t, tc * 3);
    capture(19, vc, 0, 0, bs + 1, uint32_t(tc), 0, data.data(), data.size());
    return meshopt_encodeMeshlet(b, bs, v, vc, t, tc);
}
static int capDecodeMeshlet(void* v, size_t vc, size_t vs, void* t, size_t tc, size_t ts, const unsigned char* p, size_t l) {
    capture(20, vc, vs, 0, 0, uint32_t(tc), uint32_t(ts), p, l); return meshopt_decodeMeshlet(v, vc, vs, t, tc, ts, p, l);
}
template <typename V, typename T> static int capDecodeMeshlet(V* v, size_t vc, T* t, size_t tc, const unsigned char* p, size_t l) {
    return capDecodeMeshlet(v, vc, sizeof(V), t, tc, sizeof(T) == 1 ? 3 : 4, p, l);
}
static int capDecodeMeshletRaw(unsigned int* v, size_t vc, unsigned int* t, size_t tc, const unsigned char* p, size_t l) {
    capture(21, vc, 4, 0, 0, uint32_t(tc), 4, p, l); return meshopt_decodeMeshletRaw(v, vc, t, tc, p, l);
}
"""
footer = 'int main(int argc,char** argv){if(argc!=2)return 1;dir=argv[1];\n'
for fn in functions:
    if fn.startswith(('decodeVertex', 'encodeVertex')):
        # The upstream runner executes these for both global vertex versions.
        for v in [0, 1]:
            footer += f'capVertexVersion({v});fixture="{fn}-v{v}";{fn}();\n'
        footer += 'capVertexVersion(1);\n'
    else:
        footer += f'fixture="{fn}";{fn}();\n'
footer += 'return 0;}\n'
out.write_text(header + source + footer)
print('native 0.4 fixture functions', len(functions), ' '.join(functions))
