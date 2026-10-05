"""Refresh line references after formatting; arguments are extracted from each block."""
import re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
module=ROOT/'src/codec/simd'
lines=['# SIMD safety audit','','This module is private. Its only unsafe operations are calls authorized by an','ISA token and fixed-array vector loads/stores. All value intrinsics run inside','a matching target_feature function. No transmute, raw-pointer arithmetic,','MaybeUninit, get_unchecked, static mut, heap allocation or thread-local kernel','state is used. Scalar parsing validates the layout and the complete 24-byte','vertex group before dispatch. Meshlet windows are checked 16-byte arrays.','Loads/stores permit byte alignment. Tails are staged in initialized stack','arrays and copied with checked slices. Stack scratch stays below 16 KiB','(vertex planes 1024 bytes; filter batching under 2 KiB). Immutable const','shuffle tables require no initialization. Detection is cached in AtomicU8;','only dispatch.rs constructs tokens. Optional diagnostic ceilings cannot exceed','detected capabilities. With std ceilings are thread-local and unwind-safe.','Without std diagnostic ceilings are global: concurrency can only lower them,','so outputs and instruction safety remain unchanged.','','Filters use the scalar arithmetic order, IEEE sqrt/div, sign-biased truncation,','and no FMA. Oct checks zero length before conversion; integer Oct/Quat fields','bound every conversion in i32. Color validates conversion range or falls back','to the checked scalar kernel before writing that batch. Exp uses scalar bit','construction and multiplication. The scalar reuse cache remains available.','','| Block (source line) | Kind and safety argument |','|---|---|']
count=0
for file in sorted(module.glob('*.rs')):
 src=file.read_text().splitlines()
 for i,line in enumerate(src):
  if re.search(r'\bunsafe\s*\{',line):
   prior=[];j=i-1
   while j>=0 and src[j].lstrip().startswith('//'):
    prior.append(src[j].strip().removeprefix('// ').removeprefix('SAFETY: '));j-=1
   argument=' '.join(reversed(prior));assert argument
   kind='fixed-array memory access' if '_mm_load' in line or '_mm_store' in line or 'vld1' in line or 'vst1' in line or 'v128_load' in line or 'v128_store' in line else 'token-authorized target_feature call'
   lines.append(f'| `{file.relative_to(ROOT)}:{i+1}` | {kind}: {argument} |');count+=1
lines+=['',f'Inventory: **{count} unsafe blocks**, one module-level allowance. The token-aware','boundary gate checks code tokens and exact file/line inventory independently','for repository sources and the published package. The unsafe-free build is','checked with --no-default-features and RUSTFLAGS=-Funsafe_code.','','Miri execution and intrinsic coverage, per-target parity, ASan differential','fuzzing budgets, exhaustive filter/sqrt records and performance qualifications','are recorded in parity/SIMD_RESULTS.md. Finite tests are not a proof of UB','absence. AArch64 NEON is not Miri-covered; its execution/fuzzing requires','native CI. A level without its release safety/parity record must not ship.']
(module/'SAFETY.md').write_text('\n'.join(lines)+'\n')
