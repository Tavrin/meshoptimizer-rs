"""Refresh only the new geometry rows after formatting; the independent audit verifies them."""
from pathlib import Path
import re
root=Path(__file__).resolve().parents[2]
source=root/'src/codec/simd/bounds.rs'
audit=root/'src/codec/simd/SAFETY.md'
arguments={
 'arch::roots(': 'token-authorized target_feature call: Baseline proves SSE2, NEON or simd128 for the selected backend; the input and output are complete initialized eight-float arrays.',
 'arch::distances(': 'token-authorized target_feature call: Baseline proves the selected backend ISA; three complete initialized four-float coordinate arrays and an exclusive four-float result bound every access.',
 '_mm_loadu_ps(': 'fixed-array memory access: the initialized four-float array supplies exactly 16 readable bytes; loadu permits unaligned addresses and retains no pointer.',
 '_mm_storeu_ps(': 'fixed-array memory access: the exclusive four-float array supplies exactly 16 writable bytes; storeu permits unaligned addresses and retains no pointer.',
 'vld1q_f32(': 'fixed-array memory access: the initialized four-float array supplies exactly 16 readable bytes and natural float alignment; vld1q permits this alignment and retains no pointer.',
 'vst1q_f32(': 'fixed-array memory access: the exclusive four-float array supplies exactly 16 writable bytes and natural float alignment; vst1q permits this alignment and retains no pointer.',
 'v128_load(': 'fixed-array memory access: the initialized four-float array supplies exactly 16 readable bytes; v128_load permits unaligned addresses and retains no pointer.',
 'v128_store(': 'fixed-array memory access: the exclusive four-float array supplies exactly 16 writable bytes; v128_store permits unaligned addresses and retains no pointer.',
}
rows=[]
for line,text in enumerate(source.read_text().splitlines(),1):
 if 'unsafe {' not in text: continue
 matches=[v for k,v in arguments.items() if k in text]
 assert len(matches)==1,(line,text)
 rows.append(f'| `src/codec/simd/bounds.rs:{line}` | {matches[0]} |')
s='\n'.join(l for l in audit.read_text().splitlines() if not l.startswith('| `src/codec/simd/bounds.rs:'))+'\n'
i=s.index('\nInventory:')
s=s[:i].rstrip()+'\n'+'\n'.join(rows)+'\n'+s[i:]
count=len(re.findall(r'^\| `src/codec/simd/[^`]+:\d+` \|',s,re.M))
s=re.sub(r'Inventory: \*\*\d+ unsafe blocks\*\*',f'Inventory: **{count} unsafe blocks**',s)
audit.write_text(s)
print('Geometry inventory:',len(rows),'new blocks;',count,'total')
