# Contract-preserving search bounds

## KD-tree search work

`src/meshlet.rs:518` and `src/meshlet.rs:540`: a search visits each reachable
KD node at most once. A leaf occupies one array element per stored triangle;
its additional visit charge is its number of elements. The sum of leaf charges
is at most the array length, and node-entry charges are at most the array
length. Therefore twice the allocated node-array length bounds all search
charges, including empty nodes. Overflow or insufficient remaining work
selects the original checked path. On the covered path each charge still occurs
at the original point; failure in a checked leaf scan retains its prefix charge.
The bound is checked anew for every search, without consuming work.

## Moderate leaf distance root

`src/meshlet.rs:559` and `src/meshlet.rs:636`: the moderate path is
selected only after position validation (`src/processing.rs:217`) bounds each absolute
component by 1e8. Cluster LOD passes that validated envelope to the child
builder (`src/clusterlod/mod.rs:867`) and disables it after dilation
(`src/clusterlod/mod.rs:967`). Triangle centers are averages of three such
coordinates, and search points are derived from those centers or their bounded
meshlet accumulations.
The settings validation (`src/meshlet.rs:70`) allows at most 512 triangles
per meshlet. With that limit, each rounded accumulated center
component is below 5.13e10 in magnitude; the scale is at most one. Each
difference component is below 5.15e10, so the rounded sum of its three
squares is below 8e21, far below f32's maximum. Sums of squares are
nonnegative. For those operands the native
std root agrees bit-for-bit with the pinned root (the 0.2.10 exhaustive
2^32-pattern root experiment); no_std retains the pinned memoized backend.
The unchecked moderate arithmetic and its validation charge are unchanged.

## Cluster LOD simplifier roots

`src/clusterlod/support/math.rs:19`: an unsigned IEEE-754 bit pattern at most
`0x7f7fffff` has sign zero and a non-all-ones exponent. It is finite and
nonnegative, including positive zero and subnormals. The retained exhaustive
0.2.10 comparison found zero native-versus-pinned mismatches in that range.
Negative zero, negative values, infinities and NaNs retain the original
memoized backend, as do all operands in no_std. This guard does not change
validation, work charges or storage accounting.
