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

// ATTR encoding codes 1..11. Per-cluster block at descriptor word 9:
// length, channels, words/vertex, vertices; then 8 words per channel
// (tag, encoding, source offset, zero, min.xy, scale.xy), then vertex words.
fn attribute_channel_count(page: u32, cluster: u32) -> u32 {
    let at = cluster_word(page, cluster, 9u);
    if (at == 0u) { return 0u; }
    return pages[page + at + 1u];
}

// IEEE binary16 to binary32 using only integer operations, including subnormals.
fn decode_half(word: u32) -> f32 {
    let sign = (word & 0x8000u) << 16u;
    let exponent = (word >> 10u) & 31u;
    var fraction = word & 1023u;
    if (exponent == 0u) {
        if (fraction == 0u) { return bitcast<f32>(sign); }
        var shift = 0u;
        loop {
            if ((fraction & 1024u) != 0u) { break; }
            fraction <<= 1u;
            shift++;
        }
        return bitcast<f32>(sign | ((113u - shift) << 23u) | ((fraction & 1023u) << 13u));
    }
    if (exponent == 31u) { return bitcast<f32>(sign | 0x7f800000u | (fraction << 13u)); }
    return bitcast<f32>(sign | ((exponent + 112u) << 23u) | (fraction << 13u));
}

fn decode_attribute(page: u32, cluster: u32, channel: u32, vertex: u32) -> vec4<f32> {
    let block = page + cluster_word(page, cluster, 9u);
    let channels = pages[block + 1u];
    let stride = pages[block + 2u];
    var offset = 0u;
    for (var c = 0u; c < channel; c++) {
        let code = pages[block + 4u + c * 8u + 1u];
        if (code >= 8u) { offset += code - 7u; }
        else if (code == 3u || code == 6u) { offset += 2u; }
        else { offset += 1u; }
    }
    let header = block + 4u + channel * 8u;
    let code = pages[header + 1u];
    let at = block + 4u + channels * 8u + vertex * stride + offset;
    let packed = pages[at];
    var result = vec4<f32>(0.0);
    if (code <= 3u) {
        let bits = select(16u, 12u, code == 2u);
        let x = bitcast<i32>(packed << (32u - bits)) >> (32u - bits);
        let y = bitcast<i32>((packed >> bits) << (32u - bits)) >> (32u - bits);
        let m = f32((1u << (bits - 1u)) - 1u);
        var n = vec3<f32>(f32(x) / m, f32(y) / m, 0.0);
        n.z = 1.0 - abs(n.x) - abs(n.y);
        if (n.z < 0.0) {
            let old = n.xy;
            n.x = (1.0 - abs(old.y)) * select(1.0, -1.0, old.x < 0.0);
            n.y = (1.0 - abs(old.x)) * select(1.0, -1.0, old.y < 0.0);
        }
        result = vec4<f32>(normalize(n), 0.0);
        if (code == 3u) { result.w = select(1.0, -1.0, pages[at + 1u] == 1u); }
    } else if (code == 4u) {
        let lo = vec2<f32>(bitcast<f32>(pages[header + 4u]), bitcast<f32>(pages[header + 5u]));
        let scale = vec2<f32>(bitcast<f32>(pages[header + 6u]), bitcast<f32>(pages[header + 7u]));
        let q = vec2<f32>(f32(packed & 65535u), f32(packed >> 16u)) / 65535.0;
        result = vec4<f32>(lo + scale * q, 0.0, 0.0);
    } else if (code == 5u || code == 6u) {
        result = vec4<f32>(decode_half(packed & 65535u), decode_half(packed >> 16u), 0.0, 0.0);
        if (code == 6u) { result.z = decode_half(pages[at + 1u] & 65535u); result.w = decode_half(pages[at + 1u] >> 16u); }
    } else if (code == 7u) {
        result = vec4<f32>(f32(packed & 255u), f32((packed >> 8u) & 255u), f32((packed >> 16u) & 255u), f32(packed >> 24u)) / 255.0;
    } else {
        for (var c = 0u; c < code - 7u; c++) { result[c] = bitcast<f32>(pages[at + c]); }
    }
    return result;
}

fn triangle_material(page: u32, cluster: u32, triangle: u32) -> u32 {
    let material = cluster_word(page, cluster, 15u);
    if ((cluster_word(page, cluster, 2u) & (1u << 18u)) != 0u) { return material; }
    let at = page + material;
    let count = pages[at];
    var slot = 0u;
    for (var k = 0u; k < count; k++) {
        let pair = pages[at + 1u + k];
        if ((pair >> 16u) > triangle) { break; }
        slot = pair & 65535u;
    }
    return slot;
}

// Attribute-vector replay: per cluster, vertex-major then channel-major vec4
// float bits, then one material slot word per triangle. Same bindings as replay_page.
@compute @workgroup_size(1)
fn replay_attributes() {
    var out = 0u;
    for (var c = 0u; c < page_cluster_count(replay.page); c++) {
        let channels = attribute_channel_count(replay.page, c);
        for (var v = 0u; v < cluster_vertex_count(replay.page, c); v++) {
            for (var k = 0u; k < channels; k++) {
                let value = bitcast<vec4<u32>>(decode_attribute(replay.page, c, k, v));
                decoded[out] = value.x;
                decoded[out + 1u] = value.y;
                decoded[out + 2u] = value.z;
                decoded[out + 3u] = value.w;
                out += 4u;
            }
        }
        for (var t = 0u; t < cluster_triangle_count(replay.page, c); t++) {
            decoded[out] = triangle_material(replay.page, c, t);
            out++;
        }
    }
}
