// Reference decoder for the page memory form (docs/PAGE_FORMAT.md).
//
// Only 32-bit loads, shifts, masks and bitcasts: no 8- or 16-bit storage, no
// 64-bit integers or atomics, no subgroup operations. `page` is the word index of
// the page header inside the bound page pool; every offset in the page is in
// words from that header.

const PAGE_HEADER_WORDS: u32 = 4u;
const CLUSTER_WORDS: u32 = 16u;

@group(0) @binding(0) var<storage, read> pages: array<u32>;

fn page_cluster_count(page: u32) -> u32 {
    return pages[page];
}

fn cluster_word(page: u32, cluster: u32, w: u32) -> u32 {
    return pages[page + PAGE_HEADER_WORDS + cluster * CLUSTER_WORDS + w];
}

fn cluster_vertex_count(page: u32, cluster: u32) -> u32 {
    return cluster_word(page, cluster, 2u) & 0x1ffu;
}

fn cluster_triangle_count(page: u32, cluster: u32) -> u32 {
    return (cluster_word(page, cluster, 2u) >> 9u) & 0x1ffu;
}

// `count` bits (at most 24) at bit address `bit`, least significant bit first.
fn read_bits(page: u32, bit: u32, count: u32) -> u32 {
    if (count == 0u) {
        return 0u;
    }
    let at = page + (bit >> 5u);
    let shift = bit & 31u;
    var value = pages[at] >> shift;
    if (shift + count > 32u) {
        value = value | (pages[at + 1u] << (32u - shift));
    }
    return value & (0xffffffffu >> (32u - count));
}

// Grid step 2^exponent for exponents -126..127, built exactly from its bits.
fn grid_step(exponent: i32) -> f32 {
    return bitcast<f32>(u32(exponent + 127) << 23u);
}

fn decode_position(page: u32, cluster: u32, vertex: u32, exponent: i32) -> vec3<f32> {
    let bits = cluster_word(page, cluster, 4u);
    let bx = bits & 31u;
    let by = (bits >> 5u) & 31u;
    let bz = (bits >> 10u) & 31u;
    let start = (cluster_word(page, cluster, 3u) << 5u) + vertex * (bx + by + bz);
    let delta = vec3<u32>(
        read_bits(page, start, bx),
        read_bits(page, start + bx, by),
        read_bits(page, start + bx + by, bz),
    );
    let base = vec3<i32>(
        bitcast<i32>(cluster_word(page, cluster, 5u)),
        bitcast<i32>(cluster_word(page, cluster, 6u)),
        bitcast<i32>(cluster_word(page, cluster, 7u)),
    );
    // |base + delta| < 2^24, so the conversion to f32 is exact; so is the scaling.
    return vec3<f32>(base + bitcast<vec3<i32>>(delta)) * grid_step(exponent);
}

fn decode_triangle(page: u32, cluster: u32, triangle: u32) -> vec3<u32> {
    let packed = read_bits(page, (cluster_word(page, cluster, 8u) << 5u) + triangle * 24u, 24u);
    return vec3<u32>(packed & 0xffu, (packed >> 8u) & 0xffu, packed >> 16u);
}

// Tight sphere: center xyz, radius w.
fn cluster_sphere(page: u32, cluster: u32) -> vec4<f32> {
    return bitcast<vec4<f32>>(vec4<u32>(
        cluster_word(page, cluster, 10u),
        cluster_word(page, cluster, 11u),
        cluster_word(page, cluster, 12u),
        cluster_word(page, cluster, 13u),
    ));
}

// Normal cone: axis xyz and cutoff w, each a signed byte divided by 127.
// Only meaningful when descriptor word 2 bit 19 is set.
fn cluster_cone(page: u32, cluster: u32) -> vec4<f32> {
    let w = cluster_word(page, cluster, 14u);
    let s = vec4<i32>(
        bitcast<i32>(w << 24u) >> 24u,
        bitcast<i32>(w << 16u) >> 24u,
        bitcast<i32>(w << 8u) >> 24u,
        bitcast<i32>(w) >> 24u,
    );
    return vec4<f32>(s) / 127.0;
}

// Replay entry point for the test vectors in docs/page_vectors.txt: for each
// cluster in order, the position bits of every vertex (x, y, z), then the three
// local indices of every triangle.
struct Replay {
    page: u32,
    exponent: i32,
}

@group(0) @binding(1) var<storage, read_write> decoded: array<u32>;
@group(0) @binding(2) var<uniform> replay: Replay;

@compute @workgroup_size(1)
fn replay_page() {
    var out = 0u;
    let clusters = page_cluster_count(replay.page);
    for (var c = 0u; c < clusters; c++) {
        let nv = cluster_vertex_count(replay.page, c);
        for (var v = 0u; v < nv; v++) {
            let p = bitcast<vec3<u32>>(decode_position(replay.page, c, v, replay.exponent));
            decoded[out] = p.x;
            decoded[out + 1u] = p.y;
            decoded[out + 2u] = p.z;
            out += 3u;
        }
        let nt = cluster_triangle_count(replay.page, c);
        for (var t = 0u; t < nt; t++) {
            let i = decode_triangle(replay.page, c, t);
            decoded[out] = i.x;
            decoded[out + 1u] = i.y;
            decoded[out + 2u] = i.z;
            out += 3u;
        }
    }
}
