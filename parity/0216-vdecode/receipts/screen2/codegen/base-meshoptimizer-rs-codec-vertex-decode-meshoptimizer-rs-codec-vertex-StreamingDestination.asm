
<artifacts>/screen2/driver-base:     file format elf64-x86-64


Disassembly of section .text:

0000000000073dd0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73dd0:	push   %rbp
   73dd1:	push   %r15
   73dd3:	push   %r14
   73dd5:	push   %r13
   73dd7:	push   %r12
   73dd9:	push   %rbx
   73dda:	sub    $0x598,%rsp
   73de1:	mov    %rcx,%r15
   73de4:	mov    %rdx,%r14
   73de7:	mov    %rsi,%rbx
   73dea:	mov    %rdi,%r12
   73ded:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73df2:	cmp    $0x2,%al
   73df4:	jb     73e28 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   73df6:	cmp    $0x21,%rbx
   73dfa:	lea    0xcedf(%rip),%rax        # 80ce0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73e01:	lea    0x7188(%rip),%r9        # 7af90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73e08:	cmovb  %rax,%r9
   73e0c:	mov    $0x4,%edx
   73e11:	mov    %r12,%rdi
   73e14:	mov    %rbx,%rsi
   73e17:	mov    %r14,%rcx
   73e1a:	mov    %r15,%r8
   73e1d:	call   *%r9
   73e20:	cmp    $0xfe,%al
   73e22:	jne    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73e28:	mov    $0x7,%al
   73e2a:	test   %r15,%r15
   73e2d:	je     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73e33:	movzbl (%r14),%ebp
   73e37:	mov    %ebp,%ecx
   73e39:	and    $0xf0,%cl
   73e3c:	cmp    $0xa0,%cl
   73e3f:	jne    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73e45:	and    $0xf,%bpl
   73e49:	mov    $0x5,%al
   73e4b:	cmp    $0x1,%bpl
   73e4f:	ja     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73e55:	xor    %eax,%eax
   73e57:	test   %bpl,%bpl
   73e5a:	sete   %al
   73e5d:	lea    0x18(,%rax,8),%rcx
   73e65:	mov    $0x7,%al
   73e67:	cmp    %rcx,%r15
   73e6a:	jbe    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73e70:	cmp    $0x1,%bpl
   73e74:	mov    %r15,%rax
   73e77:	adc    $0x0,%rax
   73e7b:	lea    -0x1(%rax),%rdx
   73e7f:	mov    %rdx,0x70(%rsp)
   73e84:	cmp    %r15,%rdx
   73e87:	ja     74a5b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   73e8d:	mov    %rcx,0x88(%rsp)
   73e95:	add    $0xfffffffffffffffb,%rax
   73e99:	pxor   %xmm0,%xmm0
   73e9d:	movdqu %xmm0,0x188(%rsp)
   73ea6:	movdqu %xmm0,0x17c(%rsp)
   73eaf:	movdqu %xmm0,0x16c(%rsp)
   73eb8:	movdqu %xmm0,0x15c(%rsp)
   73ec1:	movdqu %xmm0,0x14c(%rsp)
   73eca:	movdqu %xmm0,0x13c(%rsp)
   73ed3:	movdqu %xmm0,0x12c(%rsp)
   73edc:	movdqu %xmm0,0x11c(%rsp)
   73ee5:	movdqu %xmm0,0x10c(%rsp)
   73eee:	movdqu %xmm0,0xfc(%rsp)
   73ef7:	movdqu %xmm0,0xec(%rsp)
   73f00:	movdqu %xmm0,0xdc(%rsp)
   73f09:	movdqu %xmm0,0xcc(%rsp)
   73f12:	movdqu %xmm0,0xbc(%rsp)
   73f1b:	movdqu %xmm0,0xac(%rsp)
   73f24:	movdqu %xmm0,0x9c(%rsp)
   73f2d:	mov    (%r14,%rax,1),%r13d
   73f31:	mov    %r13d,0x98(%rsp)
   73f39:	lea    0x198(%rsp),%rdi
   73f41:	mov    $0x400,%edx
   73f46:	xor    %esi,%esi
   73f48:	call   *0xb2782(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   73f4e:	test   %rbx,%rbx
   73f51:	je     73f79 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   73f53:	mov    %r13d,0x5c(%rsp)
   73f58:	mov    $0x1,%r8d
   73f5e:	xor    %ecx,%ecx
   73f60:	mov    %r15,0x50(%rsp)
   73f65:	mov    %r14,0x28(%rsp)
   73f6a:	mov    %rbx,0x80(%rsp)
   73f72:	mov    %bpl,0x27(%rsp)
   73f77:	jmp    73ff8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   73f79:	mov    $0x1,%r8d
   73f7f:	sub    %r8,%r15
   73f82:	setae  %cl
   73f85:	cmp    0x88(%rsp),%r15
   73f8d:	sete   %al
   73f90:	and    %cl,%al
   73f92:	neg    %al
   73f94:	or     $0x7,%al
   73f96:	jmp    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73f9b:	mov    $0x1,%ecx
   73fa0:	mov    $0x1,%r8d
   73fa6:	mov    %r13,%rdi
   73fa9:	mov    %rbp,%rsi
   73fac:	mov    %r15,%rdx
   73faf:	call   82b00 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73fb4:	mov    0x10(%r13),%rbp
   73fb8:	mov    0x8(%r13),%rdi
   73fbc:	add    %rbp,%rdi
   73fbf:	lea    0x8(%r12),%rsi
   73fc4:	mov    %r15,%rdx
   73fc7:	call   *0xb26f3(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   73fcd:	add    %r15,%rbp
   73fd0:	mov    %rbp,0x10(%r13)
   73fd4:	mov    0x78(%rsp),%rax
   73fd9:	mov    %rax,%rcx
   73fdc:	mov    0x80(%rsp),%rbx
   73fe4:	cmp    %rbx,%rax
   73fe7:	mov    0x50(%rsp),%r15
   73fec:	movzbl 0x27(%rsp),%ebp
   73ff1:	mov    0x48(%rsp),%r8
   73ff6:	jae    73f7f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   73ff8:	sub    %rcx,%rbx
   73ffb:	cmp    $0x100,%rbx
   74002:	mov    $0x100,%r13d
   74008:	cmovb  %rbx,%r13
   7400c:	lea    0xf(%r13),%r9d
   74010:	and    $0x3f0,%r9d
   74017:	test   %bpl,%bpl
   7401a:	mov    %r9,0x48(%rsp)
   7401f:	mov    %rcx,0x90(%rsp)
   74027:	mov    %rbx,0x60(%rsp)
   7402c:	je     740bd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   74032:	mov    $0x7,%al
   74034:	cmp    %r15,%r8
   74037:	jae    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7403d:	add    %r13,%rcx
   74040:	mov    %rcx,0x78(%rsp)
   74045:	lea    0x0(,%r13,4),%rbp
   7404d:	mov    %rbp,0x408(%r12)
   74055:	movzbl (%r14,%r8,1),%r14d
   7405a:	inc    %r8
   7405d:	mov    %r14d,%ecx
   74060:	and    $0x3,%ecx
   74063:	lea    0x0(,%r13,2),%rdx
   7406b:	add    %r13,%rdx
   7406e:	mov    %rdx,0x68(%rsp)
   74073:	cmp    %r9,%r13
   74076:	ja     747c9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   7407c:	cmp    $0x2,%cl
   7407f:	je     740fd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   74081:	movzbl %cl,%edx
   74084:	cmp    $0x3,%edx
   74087:	jne    74115 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   7408d:	lea    (%r8,%r13,1),%rbx
   74091:	cmp    %r15,%rbx
   74094:	ja     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7409a:	add    0x28(%rsp),%r8
   7409f:	lea    0x198(%rsp),%rdi
   740a7:	mov    %r8,%rsi
   740aa:	mov    %r13,%rdx
   740ad:	call   *0xb260d(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   740b3:	mov    0x50(%rsp),%r15
   740b8:	jmp    7415f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   740bd:	lea    (%rcx,%r13,1),%rax
   740c1:	mov    %rax,0x78(%rsp)
   740c6:	lea    0x0(,%r13,4),%rbp
   740ce:	mov    %rbp,0x408(%r12)
   740d6:	xor    %r14d,%r14d
   740d9:	lea    0x0(,%r13,2),%rax
   740e1:	add    %r13,%rax
   740e4:	mov    %rax,0x68(%rsp)
   740e9:	lea    -0x66850(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   740f0:	mov    %rcx,%rax
   740f3:	cmp    %r9,%r13
   740f6:	jbe    7412e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   740f8:	jmp    747f4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   740fd:	lea    0x198(%rsp),%rdi
   74105:	xor    %esi,%esi
   74107:	mov    %r13,%rdx
   7410a:	mov    %r8,%rbx
   7410d:	call   *0xb25bd(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   74113:	jmp    7415f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   74115:	test   %cl,%cl
   74117:	lea    -0x6657e(%rip),%rcx        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7411e:	lea    -0x66465(%rip),%rax        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74125:	cmove  %rax,%rcx
   74129:	mov    0x48(%rsp),%r9
   7412e:	mov    %rcx,(%rsp)
   74132:	lea    0x30(%rsp),%rdi
   74137:	mov    0x28(%rsp),%rsi
   7413c:	mov    %r15,%rdx
   7413f:	mov    %r8,%rcx
   74142:	lea    0x198(%rsp),%r8
   7414a:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   7414f:	cmpb   $0x0,0x30(%rsp)
   74154:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7415a:	mov    0x38(%rsp),%rbx
   7415f:	mov    %r14d,%eax
   74162:	shr    $0x2,%al
   74165:	and    $0x3,%al
   74167:	lea    (%rsp,%r13,1),%r8
   7416b:	add    $0x198,%r8
   74172:	cmp    $0x3,%al
   74174:	je     74191 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   74176:	movzbl %al,%ecx
   74179:	cmp    $0x2,%ecx
   7417c:	jne    741c8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   7417e:	mov    %r8,%rdi
   74181:	xor    %esi,%esi
   74183:	mov    %r13,%rdx
   74186:	call   *0xb2544(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   7418c:	jmp    7421a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   74191:	mov    %rbx,%r15
   74194:	mov    $0x7,%al
   74196:	add    %r13,%r15
   74199:	jb     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7419f:	cmp    0x50(%rsp),%r15
   741a4:	ja     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   741aa:	add    0x28(%rsp),%rbx
   741af:	mov    %r8,%rdi
   741b2:	mov    %rbx,%rsi
   741b5:	mov    %r13,%rdx
   741b8:	call   *0xb2502(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   741be:	mov    %r15,%rbx
   741c1:	mov    0x50(%rsp),%r15
   741c6:	jmp    7421a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   741c8:	test   %al,%al
   741ca:	lea    -0x66631(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   741d1:	lea    -0x66518(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   741d8:	cmove  %rcx,%rax
   741dc:	cmpb   $0x0,0x27(%rsp)
   741e1:	lea    -0x66948(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   741e8:	cmove  %rcx,%rax
   741ec:	mov    %rax,(%rsp)
   741f0:	lea    0x30(%rsp),%rdi
   741f5:	mov    0x28(%rsp),%rsi
   741fa:	mov    %r15,%rdx
   741fd:	mov    %rbx,%rcx
   74200:	mov    0x48(%rsp),%r9
   74205:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   7420a:	cmpb   $0x0,0x30(%rsp)
   7420f:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74215:	mov    0x38(%rsp),%rbx
   7421a:	mov    %r14d,%eax
   7421d:	shr    $0x4,%al
   74220:	and    $0x3,%al
   74222:	lea    (%rsp,%r13,2),%r8
   74226:	add    $0x198,%r8
   7422d:	cmp    $0x3,%al
   7422f:	je     7424c <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   74231:	movzbl %al,%ecx
   74234:	cmp    $0x2,%ecx
   74237:	jne    74283 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   74239:	mov    %r8,%rdi
   7423c:	xor    %esi,%esi
   7423e:	mov    %r13,%rdx
   74241:	call   *0xb2489(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   74247:	jmp    742d5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7424c:	mov    %rbx,%r15
   7424f:	mov    $0x7,%al
   74251:	add    %r13,%r15
   74254:	jb     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7425a:	cmp    0x50(%rsp),%r15
   7425f:	ja     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   74265:	add    0x28(%rsp),%rbx
   7426a:	mov    %r8,%rdi
   7426d:	mov    %rbx,%rsi
   74270:	mov    %r13,%rdx
   74273:	call   *0xb2447(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   74279:	mov    %r15,%rbx
   7427c:	mov    0x50(%rsp),%r15
   74281:	jmp    742d5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   74283:	test   %al,%al
   74285:	lea    -0x666ec(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7428c:	lea    -0x665d3(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74293:	cmove  %rcx,%rax
   74297:	cmpb   $0x0,0x27(%rsp)
   7429c:	lea    -0x66a03(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   742a3:	cmove  %rcx,%rax
   742a7:	mov    %rax,(%rsp)
   742ab:	lea    0x30(%rsp),%rdi
   742b0:	mov    0x28(%rsp),%rsi
   742b5:	mov    %r15,%rdx
   742b8:	mov    %rbx,%rcx
   742bb:	mov    0x48(%rsp),%r9
   742c0:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   742c5:	cmpb   $0x0,0x30(%rsp)
   742ca:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   742d0:	mov    0x38(%rsp),%rbx
   742d5:	shr    $0x6,%r14b
   742d9:	mov    0x68(%rsp),%rax
   742de:	lea    (%rsp,%rax,1),%r8
   742e2:	add    $0x198,%r8
   742e9:	cmp    $0x3,%r14b
   742ed:	je     7430b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   742ef:	movzbl %r14b,%eax
   742f3:	cmp    $0x2,%eax
   742f6:	jne    74352 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   742f8:	mov    %r8,%rdi
   742fb:	xor    %esi,%esi
   742fd:	mov    %r13,%rdx
   74300:	call   *0xb23ca(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   74306:	jmp    743a5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7430b:	mov    %rbx,%rcx
   7430e:	mov    $0x7,%al
   74310:	add    %r13,%rcx
   74313:	jb     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   74319:	cmp    %r15,%rcx
   7431c:	ja     7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   74322:	mov    0x28(%rsp),%r14
   74327:	add    %r14,%rbx
   7432a:	mov    %r8,%rdi
   7432d:	mov    %rbx,%rsi
   74330:	mov    %r13,%rdx
   74333:	mov    %rcx,%rbx
   74336:	call   *0xb2384(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   7433c:	mov    0x50(%rsp),%r15
   74341:	cmpb   $0x0,0x27(%rsp)
   74346:	mov    %rbx,0x48(%rsp)
   7434b:	jne    743b6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   7434d:	jmp    743de <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   74352:	test   %r14b,%r14b
   74355:	lea    -0x667bc(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7435c:	lea    -0x666a3(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74363:	cmove  %rcx,%rax
   74367:	cmpb   $0x0,0x27(%rsp)
   7436c:	lea    -0x66ad3(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74373:	cmove  %rcx,%rax
   74377:	mov    %rax,(%rsp)
   7437b:	lea    0x30(%rsp),%rdi
   74380:	mov    0x28(%rsp),%rsi
   74385:	mov    %r15,%rdx
   74388:	mov    %rbx,%rcx
   7438b:	mov    0x48(%rsp),%r9
   74390:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   74395:	cmpb   $0x0,0x30(%rsp)
   7439a:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   743a0:	mov    0x38(%rsp),%rbx
   743a5:	mov    0x28(%rsp),%r14
   743aa:	cmpb   $0x0,0x27(%rsp)
   743af:	mov    %rbx,0x48(%rsp)
   743b4:	je     743de <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   743b6:	mov    0x70(%rsp),%rax
   743bb:	cmp    %rax,%r15
   743be:	je     74a89 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   743c4:	movzbl (%r14,%rax,1),%eax
   743c9:	mov    %eax,%ecx
   743cb:	and    $0x3,%ecx
   743ce:	lea    -0x63ca5(%rip),%rdx        # 10730 <libm::math::generic::sqrt::RSQRT_TAB+0x3d2>
   743d5:	movslq (%rdx,%rcx,4),%rcx
   743d9:	add    %rdx,%rcx
   743dc:	jmp    *%rcx
   743de:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   743e3:	test   %al,%al
   743e5:	je     746b4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   743eb:	movd   0x5c(%rsp),%xmm0
   743f1:	pshufd $0x0,%xmm0,%xmm0
   743f6:	mov    %r13,%rcx
   743f9:	shr    $0x4,%rcx
   743fd:	je     74659 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   74403:	lea    0x0(,%r13,2),%rdx
   7440b:	lea    0x0(,%r13,2),%rsi
   74413:	add    %r13,%rsi
   74416:	mov    %r13,%r8
   74419:	neg    %r8
   7441c:	mov    %rdx,%r10
   7441f:	neg    %r10
   74422:	mov    %rsi,%r14
   74425:	neg    %r14
   74428:	lea    (%rsp,%rsi,1),%r15
   7442c:	add    $0x198,%r15
   74433:	shl    $0x4,%rcx
   74437:	xor    %eax,%eax
   74439:	mov    %rbp,%r9
   7443c:	xor    %edi,%edi
   7443e:	movdqa -0x66b77(%rip),%xmm8        # d8d0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   74447:	movdqa -0x66d10(%rip),%xmm9        # d740 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x8fe>
   74450:	cmp    %rbp,%rdi
   74453:	ja     74a1a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   74459:	mov    %r9,%r11
   7445c:	sub    $0x10,%r11
   74460:	jb     74991 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   74466:	lea    (%rdi,%r13,1),%rbx
   7446a:	cmp    %rbp,%rbx
   7446d:	ja     749b9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   74473:	lea    (%r8,%r9,1),%rbx
   74477:	cmp    $0x10,%rbx
   7447b:	jb     74991 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   74481:	lea    (%rdx,%rdi,1),%rbx
   74485:	cmp    %rbp,%rbx
   74488:	ja     749d0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   7448e:	lea    (%r10,%r9,1),%rbx
   74492:	cmp    $0x10,%rbx
   74496:	jb     74991 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7449c:	lea    (%rsi,%rdi,1),%rbx
   744a0:	cmp    %rbp,%rbx
   744a3:	ja     749fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   744a9:	add    %r14,%r9
   744ac:	cmp    $0x10,%r9
   744b0:	jb     74991 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   744b6:	lea    0x3f(%rax),%r9
   744ba:	cmp    %rbp,%r9
   744bd:	jae    749a2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   744c3:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   744cc:	lea    (%rsp,%r13,1),%r9
   744d0:	add    $0x198,%r9
   744d7:	movdqu (%r9,%rdi,1),%xmm4
   744dd:	lea    0x198(%rsp,%r13,2),%r9
   744e5:	movdqu (%r9,%rdi,1),%xmm5
   744eb:	movdqa %xmm3,%xmm2
   744ef:	punpckhbw %xmm4,%xmm2
   744f3:	movdqu (%r15,%rdi,1),%xmm6
   744f9:	movdqa %xmm5,%xmm7
   744fd:	punpckhbw %xmm6,%xmm7
   74501:	movdqa %xmm2,%xmm1
   74505:	punpckhwd %xmm7,%xmm1
   74509:	punpcklwd %xmm7,%xmm2
   7450d:	punpcklbw %xmm4,%xmm3
   74511:	punpcklbw %xmm6,%xmm5
   74515:	movdqa %xmm3,%xmm4
   74519:	punpckhwd %xmm5,%xmm4
   7451d:	punpcklwd %xmm5,%xmm3
   74521:	movdqa %xmm3,%xmm5
   74525:	psrlw  $0x1,%xmm5
   7452a:	pand   %xmm8,%xmm3
   7452f:	pxor   %xmm6,%xmm6
   74533:	psubb  %xmm3,%xmm6
   74537:	pand   %xmm9,%xmm5
   7453c:	pxor   %xmm6,%xmm5
   74540:	movdqa %xmm5,%xmm3
   74544:	pslldq $0x4,%xmm3
   74549:	paddb  %xmm5,%xmm3
   7454d:	movdqa %xmm3,%xmm5
   74551:	pslldq $0x8,%xmm5
   74556:	paddb  %xmm0,%xmm3
   7455a:	paddb  %xmm5,%xmm3
   7455e:	movdqu %xmm3,0x8(%r12,%rdi,4)
   74565:	movdqa %xmm4,%xmm0
   74569:	psrlw  $0x1,%xmm0
   7456e:	pand   %xmm8,%xmm4
   74573:	pxor   %xmm5,%xmm5
   74577:	psubb  %xmm4,%xmm5
   7457b:	pand   %xmm9,%xmm0
   74580:	pxor   %xmm5,%xmm0
   74584:	movdqa %xmm0,%xmm4
   74588:	pslldq $0x4,%xmm4
   7458d:	paddb  %xmm0,%xmm4
   74591:	movdqa %xmm4,%xmm0
   74595:	pslldq $0x8,%xmm0
   7459a:	pshufd $0xff,%xmm3,%xmm3
   7459f:	paddb  %xmm4,%xmm0
   745a3:	paddb  %xmm3,%xmm0
   745a7:	movdqu %xmm0,0x18(%r12,%rdi,4)
   745ae:	movdqa %xmm2,%xmm3
   745b2:	psrlw  $0x1,%xmm3
   745b7:	pand   %xmm8,%xmm2
   745bc:	pxor   %xmm4,%xmm4
   745c0:	psubb  %xmm2,%xmm4
   745c4:	pand   %xmm9,%xmm3
   745c9:	pxor   %xmm4,%xmm3
   745cd:	movdqa %xmm3,%xmm2
   745d1:	pslldq $0x4,%xmm2
   745d6:	paddb  %xmm3,%xmm2
   745da:	movdqa %xmm2,%xmm3
   745de:	pslldq $0x8,%xmm3
   745e3:	pshufd $0xff,%xmm0,%xmm0
   745e8:	paddb  %xmm2,%xmm3
   745ec:	paddb  %xmm0,%xmm3
   745f0:	movdqu %xmm3,0x28(%r12,%rdi,4)
   745f7:	movdqa %xmm1,%xmm0
   745fb:	psrlw  $0x1,%xmm0
   74600:	pand   %xmm8,%xmm1
   74605:	pxor   %xmm2,%xmm2
   74609:	psubb  %xmm1,%xmm2
   7460d:	pand   %xmm9,%xmm0
   74612:	pxor   %xmm2,%xmm0
   74616:	movdqa %xmm0,%xmm1
   7461a:	pslldq $0x4,%xmm1
   7461f:	paddb  %xmm0,%xmm1
   74623:	movdqa %xmm1,%xmm0
   74627:	pslldq $0x8,%xmm0
   7462c:	pshufd $0xff,%xmm3,%xmm2
   74631:	paddb  %xmm1,%xmm0
   74635:	paddb  %xmm2,%xmm0
   74639:	movdqu %xmm0,0x38(%r12,%rdi,4)
   74640:	add    $0x10,%rdi
   74644:	pshufd $0xff,%xmm0,%xmm0
   74649:	add    $0x40,%rax
   7464d:	mov    %r11,%r9
   74650:	cmp    %rdi,%rcx
   74653:	jne    74450 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   74659:	mov    %r13d,%eax
   7465c:	and    $0x1f0,%eax
   74661:	cmp    %r13,%rax
   74664:	mov    0x28(%rsp),%r14
   74669:	je     74768 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7466f:	movdqa %xmm0,0x30(%rsp)
   74675:	lea    0x0(,%rax,4),%edx
   7467c:	mov    %rbp,%rcx
   7467f:	sub    %rdx,%rcx
   74682:	jb     74a73 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   74688:	lea    0x8(%r12),%rsi
   7468d:	add    %rsi,%rdx
   74690:	mov    %rax,0x8(%rsp)
   74695:	lea    0x30(%rsp),%rax
   7469a:	mov    %rax,(%rsp)
   7469e:	mov    $0x4,%r9d
   746a4:	lea    0x198(%rsp),%rdi
   746ac:	mov    %r13,%r8
   746af:	mov    %rbp,%rsi
   746b2:	jmp    746e5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   746b4:	lea    0x98(%rsp),%rax
   746bc:	mov    %rax,(%rsp)
   746c0:	movq   $0x0,0x8(%rsp)
   746c9:	mov    $0x4,%r9d
   746cf:	lea    0x198(%rsp),%rdi
   746d7:	mov    %rbp,%rsi
   746da:	lea    0x8(%r12),%rdx
   746df:	mov    %r13,%r8
   746e2:	mov    %rbp,%rcx
   746e5:	call   72c10 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   746ea:	jmp    74768 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   746ec:	shr    $0x4,%al
   746ef:	neg    %al
   746f1:	and    $0x1f,%al
   746f3:	movzbl %al,%eax
   746f6:	lea    0x98(%rsp),%rcx
   746fe:	mov    %rcx,(%rsp)
   74702:	mov    %eax,0x8(%rsp)
   74706:	movq   $0x0,0x10(%rsp)
   7470f:	mov    $0x4,%r9d
   74715:	lea    0x198(%rsp),%rdi
   7471d:	mov    %rbp,%rsi
   74720:	lea    0x8(%r12),%rdx
   74725:	mov    %r13,%r8
   74728:	mov    %rbp,%rcx
   7472b:	call   72fc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   74730:	jmp    74768 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   74732:	lea    0x98(%rsp),%rax
   7473a:	mov    %rax,(%rsp)
   7473e:	movq   $0x0,0x8(%rsp)
   74747:	mov    $0x4,%r9d
   7474d:	lea    0x198(%rsp),%rdi
   74755:	mov    %rbp,%rsi
   74758:	lea    0x8(%r12),%rdx
   7475d:	mov    %r13,%r8
   74760:	mov    %rbp,%rcx
   74763:	call   72e00 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   74768:	mov    0x90(%rsp),%rax
   74770:	cmp    %rax,0x80(%rsp)
   74778:	je     74a44 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   7477e:	mov    0x4(%r12,%rbp,1),%eax
   74783:	mov    %eax,0x98(%rsp)
   7478a:	mov    0x408(%r12),%r15
   74792:	cmp    $0x401,%r15
   74799:	jae    74a2d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   7479f:	mov    %eax,0x5c(%rsp)
   747a3:	mov    (%r12),%r13
   747a7:	mov    0x0(%r13),%rax
   747ab:	mov    0x10(%r13),%rbp
   747af:	sub    %rbp,%rax
   747b2:	cmp    %rax,%r15
   747b5:	ja     73f9b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   747bb:	test   %r15,%r15
   747be:	jne    73fb8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   747c4:	jmp    73fcd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   747c9:	cmp    $0x2,%ecx
   747cc:	je     74ab1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   747d2:	cmp    $0x3,%ecx
   747d5:	je     74a9a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   747db:	test   %cl,%cl
   747dd:	lea    -0x66c44(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   747e4:	lea    -0x66b2b(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   747eb:	cmove  %rcx,%rax
   747ef:	mov    0x48(%rsp),%r9
   747f4:	mov    %rax,(%rsp)
   747f8:	lea    0x30(%rsp),%rdi
   747fd:	mov    0x28(%rsp),%rsi
   74802:	mov    %r15,%rdx
   74805:	mov    %r8,%rcx
   74808:	lea    0x198(%rsp),%r8
   74810:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   74815:	cmpb   $0x0,0x30(%rsp)
   7481a:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74820:	mov    %r14d,%eax
   74823:	shr    $0x2,%al
   74826:	and    $0x3,%al
   74828:	cmp    $0x2,%al
   7482a:	je     74ab1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   74830:	movzbl %al,%ecx
   74833:	cmp    $0x3,%ecx
   74836:	je     74a9a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7483c:	mov    0x38(%rsp),%rcx
   74841:	test   %al,%al
   74843:	lea    -0x66caa(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7484a:	lea    -0x66b91(%rip),%rdx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74851:	cmove  %rdx,%rax
   74855:	cmpb   $0x0,0x27(%rsp)
   7485a:	lea    0x198(%rsp,%r13,1),%r8
   74862:	lea    -0x66fc9(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74869:	cmove  %rdx,%rax
   7486d:	mov    %rax,(%rsp)
   74871:	lea    0x30(%rsp),%rdi
   74876:	mov    0x28(%rsp),%rsi
   7487b:	mov    %r15,%rdx
   7487e:	mov    0x48(%rsp),%r9
   74883:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   74888:	cmpb   $0x0,0x30(%rsp)
   7488d:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74893:	mov    %r14d,%eax
   74896:	shr    $0x4,%al
   74899:	and    $0x3,%al
   7489b:	cmp    $0x2,%al
   7489d:	je     74ab1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   748a3:	movzbl %al,%ecx
   748a6:	cmp    $0x3,%ecx
   748a9:	je     74a9a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   748af:	mov    0x38(%rsp),%rcx
   748b4:	test   %al,%al
   748b6:	lea    -0x66d1d(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   748bd:	lea    -0x66c04(%rip),%rdx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   748c4:	cmove  %rdx,%rax
   748c8:	cmpb   $0x0,0x27(%rsp)
   748cd:	lea    0x198(%rsp,%r13,2),%r8
   748d5:	lea    -0x6703c(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   748dc:	cmove  %rdx,%rax
   748e0:	mov    %rax,(%rsp)
   748e4:	lea    0x30(%rsp),%rdi
   748e9:	mov    0x28(%rsp),%rsi
   748ee:	mov    %r15,%rdx
   748f1:	mov    0x48(%rsp),%r9
   748f6:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   748fb:	cmpb   $0x0,0x30(%rsp)
   74900:	jne    7497a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74902:	shr    $0x6,%r14b
   74906:	cmp    $0x2,%r14b
   7490a:	je     74ab1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   74910:	movzbl %r14b,%eax
   74914:	cmp    $0x3,%eax
   74917:	je     74a9a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7491d:	mov    0x38(%rsp),%rcx
   74922:	test   %r14b,%r14b
   74925:	lea    -0x66d8c(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7492c:	lea    -0x66c73(%rip),%rdx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74933:	cmove  %rdx,%rax
   74937:	cmpb   $0x0,0x27(%rsp)
   7493c:	mov    0x68(%rsp),%rdx
   74941:	lea    0x198(%rsp,%rdx,1),%r8
   74949:	lea    -0x670b0(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74950:	cmove  %rdx,%rax
   74954:	mov    %rax,(%rsp)
   74958:	lea    0x30(%rsp),%rdi
   7495d:	mov    0x28(%rsp),%rsi
   74962:	mov    %r15,%rdx
   74965:	mov    0x48(%rsp),%r9
   7496a:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   7496f:	cmpb   $0x0,0x30(%rsp)
   74974:	je     743a0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   7497a:	movzbl 0x31(%rsp),%eax
   7497f:	add    $0x598,%rsp
   74986:	pop    %rbx
   74987:	pop    %r12
   74989:	pop    %r13
   7498b:	pop    %r14
   7498d:	pop    %r15
   7498f:	pop    %rbp
   74990:	ret
   74991:	lea    0xad600(%rip),%rdi        # 121f98 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2e00>
   74998:	call   *0xb1ed2(%rip)        # 126870 <_DYNAMIC+0x3c8>
   7499e:	mov    $0x7,%al
   749a0:	jmp    7497f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   749a2:	lea    0x40(%rax),%rsi
   749a6:	lea    0xacdf3(%rip),%rcx        # 1217a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2608>
   749ad:	mov    %rax,%rdi
   749b0:	mov    %rbp,%rdx
   749b3:	call   *0xb1e5f(%rip)        # 126818 <_DYNAMIC+0x370>
   749b9:	mov    0x60(%rsp),%rcx
   749be:	cmp    $0x100,%rcx
   749c5:	mov    $0x100,%eax
   749ca:	cmovb  %rcx,%rax
   749ce:	jmp    74a17 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   749d0:	mov    0x60(%rsp),%rcx
   749d5:	cmp    $0x100,%rcx
   749dc:	mov    $0x100,%eax
   749e1:	cmovb  %rcx,%rax
   749e5:	mov    %eax,%eax
   749e7:	lea    (%rdi,%rax,2),%rdi
   749eb:	lea    0xad58e(%rip),%rcx        # 121f80 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2de8>
   749f2:	mov    %rbp,%rsi
   749f5:	mov    %rbp,%rdx
   749f8:	call   *0xb1e1a(%rip)        # 126818 <_DYNAMIC+0x370>
   749fe:	mov    0x60(%rsp),%rcx
   74a03:	cmp    $0x100,%rcx
   74a0a:	mov    $0x100,%eax
   74a0f:	cmovb  %rcx,%rax
   74a13:	lea    (%rax,%rax,2),%rax
   74a17:	add    %rax,%rdi
   74a1a:	lea    0xad55f(%rip),%rcx        # 121f80 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2de8>
   74a21:	mov    %rbp,%rsi
   74a24:	mov    %rbp,%rdx
   74a27:	call   *0xb1deb(%rip)        # 126818 <_DYNAMIC+0x370>
   74a2d:	lea    0xb01d4(%rip),%rcx        # 124c08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a70>
   74a34:	mov    $0x400,%edx
   74a39:	xor    %edi,%edi
   74a3b:	mov    %r15,%rsi
   74a3e:	call   *0xb1dd4(%rip)        # 126818 <_DYNAMIC+0x370>
   74a44:	lea    -0x4(%rbp),%rdi
   74a48:	lea    0xac919(%rip),%rcx        # 121368 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   74a4f:	mov    %rbp,%rsi
   74a52:	mov    %rbp,%rdx
   74a55:	call   *0xb1dbd(%rip)        # 126818 <_DYNAMIC+0x370>
   74a5b:	lea    0xac9de(%rip),%rcx        # 121440 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x22a8>
   74a62:	mov    0x70(%rsp),%rdi
   74a67:	mov    %r15,%rsi
   74a6a:	mov    %r15,%rdx
   74a6d:	call   *0xb1da5(%rip)        # 126818 <_DYNAMIC+0x370>
   74a73:	lea    0xacd0e(%rip),%rcx        # 121788 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25f0>
   74a7a:	mov    %rdx,%rdi
   74a7d:	mov    %rbp,%rsi
   74a80:	mov    %rbp,%rdx
   74a83:	call   *0xb1d8f(%rip)        # 126818 <_DYNAMIC+0x370>
   74a89:	lea    0xac908(%rip),%rdx        # 121398 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   74a90:	xor    %edi,%edi
   74a92:	xor    %esi,%esi
   74a94:	call   *0xb1c5e(%rip)        # 1266f8 <_DYNAMIC+0x250>
   74a9a:	lea    0xac93f(%rip),%rcx        # 1213e0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   74aa1:	xor    %edi,%edi
   74aa3:	mov    %r13,%rsi
   74aa6:	mov    0x48(%rsp),%rdx
   74aab:	call   *0xb1d67(%rip)        # 126818 <_DYNAMIC+0x370>
   74ab1:	lea    0xac940(%rip),%rcx        # 1213f8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   74ab8:	xor    %edi,%edi
   74aba:	mov    %r13,%rsi
   74abd:	mov    0x48(%rsp),%rdx
   74ac2:	call   *0xb1d50(%rip)        # 126818 <_DYNAMIC+0x370>
