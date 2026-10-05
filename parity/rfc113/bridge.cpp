// Moss-owned C ABI. Vendor sources remain unmodified (MIT, see vendor README).
#include <array>
#include <map>
#include <vector>
#include <cstring>
#include <cstdint>
#include <stdexcept>
#include <algorithm>
#include "meshoptimizer.h"
#define CLUSTERLOD_IMPLEMENTATION
#include "clusterlod.h"

struct OutCluster
{
	uint32_t first_index, index_count, group;
	int32_t refined;
	float center[3], radius;
};

struct OutGroup
{
	float center[3], radius, error;
	int32_t depth;
	uint32_t first_cluster, cluster_count;
};

struct OutNode
{
	float center[3], radius, error;
	int32_t group;
	uint32_t child_offset, child_count;
};

struct Builder
{
	std::vector<float>* positions; // mutable copy (dilation moves vertices in place)
	uint32_t pos_stride_floats;
	std::vector<uint32_t> vsrc;   // per output vertex: source vertex index
	std::vector<float> vpos;      // per output vertex: position snapshot
	std::vector<uint32_t> indices; // absolute into output vertices
	std::vector<OutCluster> clusters;
	std::vector<OutGroup> groups;
	std::vector<clodGroup> clod_groups;
	std::map<std::array<uint32_t, 4>, uint32_t> dedupe;
};

static int onGroup(void* context, clodGroup group, const clodCluster* clusters, size_t cluster_count)
{
	Builder& b = *(Builder*)context;
	if (b.groups.size() >= (1u << 20) || b.clusters.size() + cluster_count > (1u << 20))
        throw std::length_error("DAG record ceiling");
    uint32_t gid = uint32_t(b.groups.size());
	OutGroup g = {};
	memcpy(g.center, group.simplified.center, 12);
	g.radius = group.simplified.radius;
	g.error = group.simplified.error;
	g.depth = group.depth;
	g.first_cluster = uint32_t(b.clusters.size());
	g.cluster_count = uint32_t(cluster_count);
	b.groups.push_back(g);
	b.clod_groups.push_back(group);
	std::vector<unsigned int> local_vertices(256);
	std::vector<unsigned char> local_triangles;
	for (size_t i = 0; i < cluster_count; ++i)
	{
		const clodCluster& c = clusters[i];
		local_triangles.resize(c.index_count);
		local_vertices.resize(c.index_count);
		size_t unique = clodLocalIndices(local_vertices.data(), local_triangles.data(), c.indices, c.index_count);
		// Output vertices are deduplicated by (source vertex, position snapshot): without
		// dilation every position is the source's, so the cluster mesh can reuse the base
		// vertex buffer; with dilation only moved vertices get a second copy.
		uint32_t remap[256];
		for (size_t v = 0; v < unique; ++v)
		{
			uint32_t src = local_vertices[v];
			const float* p = &(*b.positions)[size_t(src) * b.pos_stride_floats];
			uint32_t bits[3];
			memcpy(bits, p, 12);
			std::array<uint32_t, 4> key = {src, bits[0], bits[1], bits[2]};
			auto it = b.dedupe.find(key);
			if (it != b.dedupe.end() && b.vsrc[it->second] == src && !memcmp(&b.vpos[size_t(it->second) * 3], p, 12))
			{
				remap[v] = it->second;
				continue;
			}
			if (b.vsrc.size() >= (4u << 20)) throw std::length_error("DAG vertex ceiling");
            uint32_t id = uint32_t(b.vsrc.size());
			b.vsrc.push_back(src);
			b.vpos.insert(b.vpos.end(), p, p + 3);
			if (it == b.dedupe.end())
				b.dedupe.emplace(key, id);
			remap[v] = id;
		}
		OutCluster oc = {};
		oc.first_index = uint32_t(b.indices.size());
		oc.index_count = uint32_t(c.index_count);
		oc.group = gid;
		oc.refined = c.refined;
		memcpy(oc.center, c.bounds.center, 12);
		oc.radius = c.bounds.radius;
		for (size_t k = 0; k < c.index_count; ++k)
			{
                if (b.indices.size() >= (16u << 20)) throw std::length_error("DAG index ceiling");
                b.indices.push_back(remap[local_triangles[k]]);
            }
		b.clusters.push_back(oc);
	}
	return int(gid);
}


extern "C" {
// Inputs are owned/aligned Rust arrays, validated before this boundary. The
// returned vector stays opaque and is always released by moss_clod_free.
int moss_clod_build(const float* vertices, uint32_t vertex_count, uint32_t stride,
    const uint32_t* indices, uint32_t index_count, const unsigned char* locks,
    void** handle, const unsigned char** bytes, size_t* size) noexcept {
    *handle = nullptr; *bytes = nullptr; *size = 0;
    try {
        std::vector<float> positions(vertices, vertices + size_t(vertex_count) * stride / 4);
        float weights[3] = {0.5f, 0.5f, 0.5f};
        clodMesh mesh = {};
        mesh.indices = indices; mesh.index_count = index_count; mesh.vertex_count = vertex_count;
        mesh.vertex_positions = positions.data(); mesh.vertex_positions_stride = stride;
        mesh.vertex_attributes = vertices + 3; mesh.vertex_attributes_stride = stride;
        mesh.vertex_lock = locks; mesh.attribute_weights = weights; mesh.attribute_count = 3;
        mesh.attribute_protect_mask = (1u << 9) - 1;
        if (stride == 64) mesh.attribute_protect_mask |= (1u << 9) | (1u << 10) | (1u << 11) | (1u << 12);
        clodConfig config = clodDefaultConfig(128);
        // Alpha dilation/coverage admission is not supplied by this static candidate builder.
        config.simplify_dilate_borders = false;
        Builder b; b.positions = &positions; b.pos_stride_floats = stride / 4;
        clodBuild(config, mesh, &b, onGroup);
        int max_depth = 0;
        for (const auto& g : b.groups) max_depth = std::max(max_depth, g.depth);
        size_t levels = size_t(max_depth) + 1;
        size_t bound = clodBuildHierarchyBound(b.groups.size(), 8, levels);
        if (levels > 64 || bound > (1u << 20)) return 2;
        std::vector<clodNode> nodes(bound);
        nodes.resize(clodBuildHierarchy(nodes.data(), b.clod_groups.data(), b.groups.size(), 8, levels));
        uint32_t depth = 0;
        std::vector<std::pair<uint32_t, uint32_t>> stack;
        for (size_t i = 0; i < levels; ++i) stack.push_back({uint32_t(i), 1});
        while (!stack.empty()) {
            auto item = stack.back(); stack.pop_back();
            depth = std::max(depth, item.second);
            if (depth > 32 || item.first >= nodes.size()) return 2;
            const auto& n = nodes[item.first];
            if (n.group < 0) for (uint32_t i = 0; i < n.child_count; ++i)
                stack.push_back({n.child_offset + i, item.second + 1});
        }
        std::vector<unsigned char> out;
        auto w32 = [&](uint32_t v) { for (unsigned i = 0; i < 4; ++i) out.push_back(static_cast<unsigned char>(v >> (i * 8))); };
        auto wf = [&](float v) { uint32_t bits; memcpy(&bits, &v, 4); w32(bits); };
        w32(0x444c434d); w32(1); w32(vertex_count); w32(index_count); w32(2);
        w32(uint32_t(b.vsrc.size())); w32(uint32_t(b.indices.size())); w32(uint32_t(b.clusters.size()));
        w32(uint32_t(b.groups.size())); w32(uint32_t(nodes.size())); w32(uint32_t(levels)); w32(depth);
        for (size_t i = 0; i < b.vsrc.size(); ++i) {
            w32(b.vsrc[i]); for (unsigned k = 0; k < 3; ++k) wf(b.vpos[i * 3 + k]);
        }
        for (auto i : b.indices) w32(i);
        for (const auto& c : b.clusters) {
            w32(c.first_index); w32(c.index_count); w32(c.group); w32(uint32_t(c.refined));
            for (auto p : c.center) wf(p); wf(c.radius);
        }
        for (const auto& g : b.groups) {
            for (auto p : g.center) wf(p); wf(g.radius); wf(g.error); w32(uint32_t(g.depth));
            w32(g.first_cluster); w32(g.cluster_count);
        }
        for (const auto& n : nodes) {
            for (auto p : n.bounds.center) wf(p); wf(n.bounds.radius); wf(n.bounds.error);
            w32(uint32_t(n.group)); w32(n.child_offset); w32(n.child_count);
        }
        if (out.size() > (256u << 20)) return 2;
        auto* owned = new std::vector<unsigned char>(std::move(out));
        *bytes = owned->data(); *size = owned->size(); *handle = owned;
        return 0;
    } catch (...) { return 1; }
}
void moss_clod_free(void* handle) noexcept {
    delete static_cast<std::vector<unsigned char>*>(handle);
}
}
