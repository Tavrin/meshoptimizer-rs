# Exact-comparison domain and current coverage

The requested comparison is not complete. The earlier finite driver copied into
this directory is an executable subset, not the specification's enumeration.
It crosses 512 all-feature simplify option masks (128 in no-default) with eight represented position layouts,
three attribute layouts, widths 0/1/3/32, and two selected flag patterns;
its loops cover work limits for those selected cases. It does not cover every
supported stride, offset, per-vertex flag and attribute configuration, or every
storage boundary. The meshlet and cluster-LOD drivers likewise use selected
settings, layouts and limits. No observation count from these subsets is an
exhaustive proof size.

The input domain must include packed, interleaved float, and both byte orders;
mapped positions; each supported position and attribute width, stride and
absolute offset; every valid vertex-flag/lock vector; each accepted option mask
and combination; all caller-buffer and allocation APIs; and, for each case,
work limits 0 through successful work plus one and all byte/storage boundaries.
Cluster LOD adds its complete configuration, attributes, locks, callback records,
position writes and result bits. Compare full sentinel-filled caller buffers,
result/error and work/storage usage for both feature sets.

The spec does not bound the supplied buffer length or absolute offset. In
`src/input.rs`, float strides range through 64, byte strides through 256, and
valid offsets are bounded by the caller's buffer length rather than by a fixed
layout constant. For example, even a one-vertex view with stride 3 accepts
many offsets in a sufficiently large buffer. Thus a fixed finite cross-product
cannot be reported as exhaustive without an additional domain bound or an
independent proof covering arbitrary offsets and buffers. The latter would not
itself be the requested literal execution of every combination. No reduction of
the registered proof requirement is assumed here.
