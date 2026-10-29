"""Refresh line references after formatting; arguments are extracted from each block."""
import re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
module=ROOT/'src/codec/simd'
lines=['# SIMD safety audit','','This module is private. Its only unsafe operations are calls authorized by an','ISA token and fixed-array vector loads/stores, including spatial box sweeps. All value intrinsics run inside','a matching target_feature function. No transmute, raw-pointer arithmetic,','MaybeUninit, get_unchecked, static mut, heap allocation or thread-local kernel','state is used. Scalar parsing validates the layout and the complete 24-byte','vertex group before dispatch. Meshlet windows are checked 16-byte arrays.','Loads/stores permit byte alignment. Tails are staged in initialized stack','arrays and copied with checked slices. Stack scratch stays below 16 KiB','(vertex planes 1024 bytes; filter batching under 2 KiB). Immutable const','shuffle tables require no initialization. Detection is cached in AtomicU8;','only dispatch.rs constructs tokens. Optional diagnostic ceilings cannot exceed','detected capabilities. With std ceilings are thread-local and unwind-safe.','Without std diagnostic ceilings are global: concurrency can only lower them,','so outputs and instruction safety remain unchanged.','','Filters use the scalar arithmetic order, IEEE sqrt/div, sign-biased truncation,','and no FMA. Oct checks zero length before conversion; integer Oct/Quat fields','bound every conversion in i32. Color validates conversion range or falls back','to the checked scalar kernel before writing that batch. Exp uses scalar bit','construction and multiplication. The scalar reuse cache remains available.','','| Block (source line) | Kind and safety argument |','|---|---|']
lines[-2:-2]=['The NEON full vertex path uses checked 24-byte group windows (or a', 'checked 96-byte window for four groups), initialized 1024-byte plane scratch', 'and the canonical per-channel commit order. Zip operations transpose four', 'planes into records. Byte/halfword wrapping sums or word XOR prefixes shift', 'by one then two records and add/XOR the previous record. Variable NEON shifts', 'implement exact 32-bit rotation, including rotation zero. Only complete', '16-record groups use vector reconstruction; the canonical scalar tail uses', 'the last reconstructed record. No output overstore or input overread is used.', 'Native arm64 CI must execute forced scalar/NEON comparisons, including errors,', 'partial writes and destination tails; local cross-compilation is not identity', 'or performance evidence.', '']
count=0
# Preserve the hand-merged audit order: spatial kernels follow codec kernels.
for file in sorted(module.glob('*.rs'),key=lambda p:(p.stem=='spatial',p.name)):
 src=file.read_text().splitlines()
 for i,line in enumerate(src):
  if re.search(r'\bunsafe\s*\{',line):
   prior=[];j=i-1
   while j>=0 and src[j].lstrip().startswith('//'):
    prior.append(src[j].strip().removeprefix('// ').removeprefix('SAFETY: '));j-=1
   argument=' '.join(reversed(prior));assert argument
   kind='fixed-array memory access' if '_mm_load' in line or '_mm_store' in line or 'vld1' in line or 'vst1' in line or 'v128_load' in line or 'v128_store' in line else 'token-authorized target_feature call'
   if file.stem=='spatial':
    # Retain the expanded, hand-audited spatial arguments while deriving
    # locations and kinds from the source, just as for the codec inventory.
    if kind=='fixed-array memory access':
     assert argument=='a contains four initialized floats; unaligned load reads exactly 16 bytes.'
     isa='SSE loadu' if '_mm_load' in line else 'NEON load' if 'vld1' in line else 'wasm load'
     argument=f'four initialized floats supply exactly 16 readable bytes; {isa} permits unaligned addresses and retains no pointer.'
    else:
     argument={
      'Baseline proves sse2; the kernel uses checked indexing and array-backed loads.':'Baseline proves SSE2; the sweep uses checked indices and fixed-array loads only.',
      'Baseline proves neon; the kernel uses checked indexing and array-backed loads.':'Baseline proves AArch64 NEON; the sweep uses checked indices and fixed-array loads only.',
     }[argument]
   lines.append(f'| `{file.relative_to(ROOT)}:{i+1}` | {kind}: {argument} |');count+=1
lines+=['',f'Inventory: **{count} unsafe blocks**, one module-level allowance. The token-aware','boundary gate checks code tokens and exact file/line inventory independently','for repository sources and the published package. The unsafe-free build is','checked with --no-default-features and RUSTFLAGS=-Funsafe_code.','','Miri execution and intrinsic coverage, per-target parity, ASan differential','fuzzing budgets, exhaustive filter/sqrt records and performance qualifications','are recorded in parity/SIMD_RESULTS.md. Finite tests are not a proof of UB','absence. AArch64 NEON is not Miri-covered; its execution/fuzzing requires','native CI. A level without its release safety/parity record must not ship.']
lines+=['','Spatial sweeps dispatch once per axis and keep independent forward/reverse', 'accumulators in registers. SSE min/max select their second operand on equality;', 'NEON and wasm use strict comparison and bit selection to preserve exactly that', 'choice, including signed zero. Area uses (x*y + y*z) + z*x without FMA or a', 'horizontal reassociation. Lane 3 never reaches an area. Empty and short sweeps', 'need no padding outside the four-float record, heap scratch or unchecked access.', 'Work charges and fallible operations remain outside the infallible sweep.', 'Spatial qualification and the native NEON execution limitation are recorded in', 'parity/0216-spatial/PROOFS.md.']
(module/'SAFETY.md').write_text('\n'.join(lines)+'\n')
