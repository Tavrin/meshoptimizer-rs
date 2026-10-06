#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <vector>

extern "C" double p05_bench(int, int, int, unsigned, size_t, size_t, const unsigned*, const float*, const float*, const float*, const unsigned char*);

template <typename T> static void read(FILE* file, std::vector<T>& values)
{
    if (fread(values.data(), sizeof(T), values.size(), file) != values.size()) std::abort();
}

int main(int argc, char** argv)
{
    if (argc != 6) return 2;
    FILE* file = fopen(argv[5], "rb");
    if (!file) return 3;
    uint32_t count, index_count;
    if (fread(&count, sizeof(count), 1, file) != 1 || fread(&index_count, sizeof(index_count), 1, file) != 1) return 4;
    std::vector<float> positions(size_t(count) * 3), normals(size_t(count) * 3), uvs(size_t(count) * 2);
    std::vector<unsigned> indices(index_count);
    std::vector<unsigned char> texture(64);
    read(file, positions); read(file, normals); read(file, uvs); read(file, indices); read(file, texture);
    fclose(file);
    double ns = p05_bench(atoi(argv[1]), atoi(argv[2]), atoi(argv[3]), unsigned(atoi(argv[4])), count, index_count,
                          indices.data(), positions.data(), normals.data(), uvs.data(), texture.data());
    printf("%.6f\n", ns);
    return 0;
}
