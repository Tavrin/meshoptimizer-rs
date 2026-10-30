
<artifacts>/guard/driver-base:     file format elf64-x86-64


Disassembly of section .text:

000000000007ddf0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   7ddf0:	push   %rbp
   7ddf1:	push   %r15
   7ddf3:	push   %r14
   7ddf5:	push   %r13
   7ddf7:	push   %r12
   7ddf9:	push   %rbx
   7ddfa:	sub    $0x598,%rsp
   7de01:	mov    %rcx,%r15
   7de04:	mov    %rdx,%r14
   7de07:	mov    %rsi,%rbx
   7de0a:	mov    %rdi,%r12
   7de0d:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7de12:	cmp    $0x2,%al
   7de14:	jb     7de48 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   7de16:	cmp    $0x21,%rbx
   7de1a:	lea    0xd36f(%rip),%rax        # 8b190 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7de21:	lea    0x7618(%rip),%r9        # 85440 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7de28:	cmovb  %rax,%r9
   7de2c:	mov    $0x4,%edx
   7de31:	mov    %r12,%rdi
   7de34:	mov    %rbx,%rsi
   7de37:	mov    %r14,%rcx
   7de3a:	mov    %r15,%r8
   7de3d:	call   *%r9
   7de40:	cmp    $0xfe,%al
   7de42:	jne    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7de48:	mov    $0x7,%al
   7de4a:	test   %r15,%r15
   7de4d:	je     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7de53:	movzbl (%r14),%ebp
   7de57:	mov    %ebp,%ecx
   7de59:	and    $0xf0,%cl
   7de5c:	cmp    $0xa0,%cl
   7de5f:	jne    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7de65:	and    $0xf,%bpl
   7de69:	mov    $0x5,%al
   7de6b:	cmp    $0x1,%bpl
   7de6f:	ja     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7de75:	xor    %eax,%eax
   7de77:	test   %bpl,%bpl
   7de7a:	sete   %al
   7de7d:	lea    0x18(,%rax,8),%rcx
   7de85:	mov    $0x7,%al
   7de87:	cmp    %rcx,%r15
   7de8a:	jbe    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7de90:	cmp    $0x1,%bpl
   7de94:	mov    %r15,%rax
   7de97:	adc    $0x0,%rax
   7de9b:	lea    -0x1(%rax),%rdx
   7de9f:	mov    %rdx,0x70(%rsp)
   7dea4:	cmp    %r15,%rdx
   7dea7:	ja     7ea7b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   7dead:	mov    %rcx,0x88(%rsp)
   7deb5:	add    $0xfffffffffffffffb,%rax
   7deb9:	pxor   %xmm0,%xmm0
   7debd:	movdqu %xmm0,0x188(%rsp)
   7dec6:	movdqu %xmm0,0x17c(%rsp)
   7decf:	movdqu %xmm0,0x16c(%rsp)
   7ded8:	movdqu %xmm0,0x15c(%rsp)
   7dee1:	movdqu %xmm0,0x14c(%rsp)
   7deea:	movdqu %xmm0,0x13c(%rsp)
   7def3:	movdqu %xmm0,0x12c(%rsp)
   7defc:	movdqu %xmm0,0x11c(%rsp)
   7df05:	movdqu %xmm0,0x10c(%rsp)
   7df0e:	movdqu %xmm0,0xfc(%rsp)
   7df17:	movdqu %xmm0,0xec(%rsp)
   7df20:	movdqu %xmm0,0xdc(%rsp)
   7df29:	movdqu %xmm0,0xcc(%rsp)
   7df32:	movdqu %xmm0,0xbc(%rsp)
   7df3b:	movdqu %xmm0,0xac(%rsp)
   7df44:	movdqu %xmm0,0x9c(%rsp)
   7df4d:	mov    (%r14,%rax,1),%r13d
   7df51:	mov    %r13d,0x98(%rsp)
   7df59:	lea    0x198(%rsp),%rdi
   7df61:	mov    $0x400,%edx
   7df66:	xor    %esi,%esi
   7df68:	call   *0xcb022(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7df6e:	test   %rbx,%rbx
   7df71:	je     7df99 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   7df73:	mov    %r13d,0x5c(%rsp)
   7df78:	mov    $0x1,%r8d
   7df7e:	xor    %ecx,%ecx
   7df80:	mov    %r15,0x50(%rsp)
   7df85:	mov    %r14,0x28(%rsp)
   7df8a:	mov    %rbx,0x80(%rsp)
   7df92:	mov    %bpl,0x27(%rsp)
   7df97:	jmp    7e018 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   7df99:	mov    $0x1,%r8d
   7df9f:	sub    %r8,%r15
   7dfa2:	setae  %cl
   7dfa5:	cmp    0x88(%rsp),%r15
   7dfad:	sete   %al
   7dfb0:	and    %cl,%al
   7dfb2:	neg    %al
   7dfb4:	or     $0x7,%al
   7dfb6:	jmp    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7dfbb:	mov    $0x1,%ecx
   7dfc0:	mov    $0x1,%r8d
   7dfc6:	mov    %r13,%rdi
   7dfc9:	mov    %rbp,%rsi
   7dfcc:	mov    %r15,%rdx
   7dfcf:	call   8cfb0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   7dfd4:	mov    0x10(%r13),%rbp
   7dfd8:	mov    0x8(%r13),%rdi
   7dfdc:	add    %rbp,%rdi
   7dfdf:	lea    0x8(%r12),%rsi
   7dfe4:	mov    %r15,%rdx
   7dfe7:	call   *0xcaf93(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7dfed:	add    %r15,%rbp
   7dff0:	mov    %rbp,0x10(%r13)
   7dff4:	mov    0x78(%rsp),%rax
   7dff9:	mov    %rax,%rcx
   7dffc:	mov    0x80(%rsp),%rbx
   7e004:	cmp    %rbx,%rax
   7e007:	mov    0x50(%rsp),%r15
   7e00c:	movzbl 0x27(%rsp),%ebp
   7e011:	mov    0x48(%rsp),%r8
   7e016:	jae    7df9f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   7e018:	sub    %rcx,%rbx
   7e01b:	cmp    $0x100,%rbx
   7e022:	mov    $0x100,%r13d
   7e028:	cmovb  %rbx,%r13
   7e02c:	lea    0xf(%r13),%r9d
   7e030:	and    $0x3f0,%r9d
   7e037:	test   %bpl,%bpl
   7e03a:	mov    %r9,0x48(%rsp)
   7e03f:	mov    %rcx,0x90(%rsp)
   7e047:	mov    %rbx,0x60(%rsp)
   7e04c:	je     7e0dd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   7e052:	mov    $0x7,%al
   7e054:	cmp    %r15,%r8
   7e057:	jae    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e05d:	add    %r13,%rcx
   7e060:	mov    %rcx,0x78(%rsp)
   7e065:	lea    0x0(,%r13,4),%rbp
   7e06d:	mov    %rbp,0x408(%r12)
   7e075:	movzbl (%r14,%r8,1),%r14d
   7e07a:	inc    %r8
   7e07d:	mov    %r14d,%ecx
   7e080:	and    $0x3,%ecx
   7e083:	lea    0x0(,%r13,2),%rdx
   7e08b:	add    %r13,%rdx
   7e08e:	mov    %rdx,0x68(%rsp)
   7e093:	cmp    %r9,%r13
   7e096:	ja     7e7e9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   7e09c:	cmp    $0x2,%cl
   7e09f:	je     7e11d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   7e0a1:	movzbl %cl,%edx
   7e0a4:	cmp    $0x3,%edx
   7e0a7:	jne    7e135 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   7e0ad:	lea    (%r8,%r13,1),%rbx
   7e0b1:	cmp    %r15,%rbx
   7e0b4:	ja     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e0ba:	add    0x28(%rsp),%r8
   7e0bf:	lea    0x198(%rsp),%rdi
   7e0c7:	mov    %r8,%rsi
   7e0ca:	mov    %r13,%rdx
   7e0cd:	call   *0xcaead(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7e0d3:	mov    0x50(%rsp),%r15
   7e0d8:	jmp    7e17f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7e0dd:	lea    (%rcx,%r13,1),%rax
   7e0e1:	mov    %rax,0x78(%rsp)
   7e0e6:	lea    0x0(,%r13,4),%rbp
   7e0ee:	mov    %rbp,0x408(%r12)
   7e0f6:	xor    %r14d,%r14d
   7e0f9:	lea    0x0(,%r13,2),%rax
   7e101:	add    %r13,%rax
   7e104:	mov    %rax,0x68(%rsp)
   7e109:	lea    -0x6fa30(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e110:	mov    %rcx,%rax
   7e113:	cmp    %r9,%r13
   7e116:	jbe    7e14e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   7e118:	jmp    7e814 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   7e11d:	lea    0x198(%rsp),%rdi
   7e125:	xor    %esi,%esi
   7e127:	mov    %r13,%rdx
   7e12a:	mov    %r8,%rbx
   7e12d:	call   *0xcae5d(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7e133:	jmp    7e17f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7e135:	test   %cl,%cl
   7e137:	lea    -0x6f73e(%rip),%rcx        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e13e:	lea    -0x6f615(%rip),%rax        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e145:	cmove  %rax,%rcx
   7e149:	mov    0x48(%rsp),%r9
   7e14e:	mov    %rcx,(%rsp)
   7e152:	lea    0x30(%rsp),%rdi
   7e157:	mov    0x28(%rsp),%rsi
   7e15c:	mov    %r15,%rdx
   7e15f:	mov    %r8,%rcx
   7e162:	lea    0x198(%rsp),%r8
   7e16a:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e16f:	cmpb   $0x0,0x30(%rsp)
   7e174:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e17a:	mov    0x38(%rsp),%rbx
   7e17f:	mov    %r14d,%eax
   7e182:	shr    $0x2,%al
   7e185:	and    $0x3,%al
   7e187:	lea    (%rsp,%r13,1),%r8
   7e18b:	add    $0x198,%r8
   7e192:	cmp    $0x3,%al
   7e194:	je     7e1b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   7e196:	movzbl %al,%ecx
   7e199:	cmp    $0x2,%ecx
   7e19c:	jne    7e1e8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   7e19e:	mov    %r8,%rdi
   7e1a1:	xor    %esi,%esi
   7e1a3:	mov    %r13,%rdx
   7e1a6:	call   *0xcade4(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7e1ac:	jmp    7e23a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7e1b1:	mov    %rbx,%r15
   7e1b4:	mov    $0x7,%al
   7e1b6:	add    %r13,%r15
   7e1b9:	jb     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e1bf:	cmp    0x50(%rsp),%r15
   7e1c4:	ja     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e1ca:	add    0x28(%rsp),%rbx
   7e1cf:	mov    %r8,%rdi
   7e1d2:	mov    %rbx,%rsi
   7e1d5:	mov    %r13,%rdx
   7e1d8:	call   *0xcada2(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7e1de:	mov    %r15,%rbx
   7e1e1:	mov    0x50(%rsp),%r15
   7e1e6:	jmp    7e23a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7e1e8:	test   %al,%al
   7e1ea:	lea    -0x6f7f1(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e1f1:	lea    -0x6f6c8(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e1f8:	cmove  %rcx,%rax
   7e1fc:	cmpb   $0x0,0x27(%rsp)
   7e201:	lea    -0x6fb28(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e208:	cmove  %rcx,%rax
   7e20c:	mov    %rax,(%rsp)
   7e210:	lea    0x30(%rsp),%rdi
   7e215:	mov    0x28(%rsp),%rsi
   7e21a:	mov    %r15,%rdx
   7e21d:	mov    %rbx,%rcx
   7e220:	mov    0x48(%rsp),%r9
   7e225:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e22a:	cmpb   $0x0,0x30(%rsp)
   7e22f:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e235:	mov    0x38(%rsp),%rbx
   7e23a:	mov    %r14d,%eax
   7e23d:	shr    $0x4,%al
   7e240:	and    $0x3,%al
   7e242:	lea    (%rsp,%r13,2),%r8
   7e246:	add    $0x198,%r8
   7e24d:	cmp    $0x3,%al
   7e24f:	je     7e26c <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   7e251:	movzbl %al,%ecx
   7e254:	cmp    $0x2,%ecx
   7e257:	jne    7e2a3 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   7e259:	mov    %r8,%rdi
   7e25c:	xor    %esi,%esi
   7e25e:	mov    %r13,%rdx
   7e261:	call   *0xcad29(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7e267:	jmp    7e2f5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7e26c:	mov    %rbx,%r15
   7e26f:	mov    $0x7,%al
   7e271:	add    %r13,%r15
   7e274:	jb     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e27a:	cmp    0x50(%rsp),%r15
   7e27f:	ja     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e285:	add    0x28(%rsp),%rbx
   7e28a:	mov    %r8,%rdi
   7e28d:	mov    %rbx,%rsi
   7e290:	mov    %r13,%rdx
   7e293:	call   *0xcace7(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7e299:	mov    %r15,%rbx
   7e29c:	mov    0x50(%rsp),%r15
   7e2a1:	jmp    7e2f5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7e2a3:	test   %al,%al
   7e2a5:	lea    -0x6f8ac(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e2ac:	lea    -0x6f783(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e2b3:	cmove  %rcx,%rax
   7e2b7:	cmpb   $0x0,0x27(%rsp)
   7e2bc:	lea    -0x6fbe3(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e2c3:	cmove  %rcx,%rax
   7e2c7:	mov    %rax,(%rsp)
   7e2cb:	lea    0x30(%rsp),%rdi
   7e2d0:	mov    0x28(%rsp),%rsi
   7e2d5:	mov    %r15,%rdx
   7e2d8:	mov    %rbx,%rcx
   7e2db:	mov    0x48(%rsp),%r9
   7e2e0:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e2e5:	cmpb   $0x0,0x30(%rsp)
   7e2ea:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e2f0:	mov    0x38(%rsp),%rbx
   7e2f5:	shr    $0x6,%r14b
   7e2f9:	mov    0x68(%rsp),%rax
   7e2fe:	lea    (%rsp,%rax,1),%r8
   7e302:	add    $0x198,%r8
   7e309:	cmp    $0x3,%r14b
   7e30d:	je     7e32b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   7e30f:	movzbl %r14b,%eax
   7e313:	cmp    $0x2,%eax
   7e316:	jne    7e372 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   7e318:	mov    %r8,%rdi
   7e31b:	xor    %esi,%esi
   7e31d:	mov    %r13,%rdx
   7e320:	call   *0xcac6a(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7e326:	jmp    7e3c5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7e32b:	mov    %rbx,%rcx
   7e32e:	mov    $0x7,%al
   7e330:	add    %r13,%rcx
   7e333:	jb     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e339:	cmp    %r15,%rcx
   7e33c:	ja     7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e342:	mov    0x28(%rsp),%r14
   7e347:	add    %r14,%rbx
   7e34a:	mov    %r8,%rdi
   7e34d:	mov    %rbx,%rsi
   7e350:	mov    %r13,%rdx
   7e353:	mov    %rcx,%rbx
   7e356:	call   *0xcac24(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7e35c:	mov    0x50(%rsp),%r15
   7e361:	cmpb   $0x0,0x27(%rsp)
   7e366:	mov    %rbx,0x48(%rsp)
   7e36b:	jne    7e3d6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   7e36d:	jmp    7e3fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7e372:	test   %r14b,%r14b
   7e375:	lea    -0x6f97c(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e37c:	lea    -0x6f853(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e383:	cmove  %rcx,%rax
   7e387:	cmpb   $0x0,0x27(%rsp)
   7e38c:	lea    -0x6fcb3(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e393:	cmove  %rcx,%rax
   7e397:	mov    %rax,(%rsp)
   7e39b:	lea    0x30(%rsp),%rdi
   7e3a0:	mov    0x28(%rsp),%rsi
   7e3a5:	mov    %r15,%rdx
   7e3a8:	mov    %rbx,%rcx
   7e3ab:	mov    0x48(%rsp),%r9
   7e3b0:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e3b5:	cmpb   $0x0,0x30(%rsp)
   7e3ba:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e3c0:	mov    0x38(%rsp),%rbx
   7e3c5:	mov    0x28(%rsp),%r14
   7e3ca:	cmpb   $0x0,0x27(%rsp)
   7e3cf:	mov    %rbx,0x48(%rsp)
   7e3d4:	je     7e3fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7e3d6:	mov    0x70(%rsp),%rax
   7e3db:	cmp    %rax,%r15
   7e3de:	je     7eaa9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   7e3e4:	movzbl (%r14,%rax,1),%eax
   7e3e9:	mov    %eax,%ecx
   7e3eb:	and    $0x3,%ecx
   7e3ee:	lea    -0x6ce1d(%rip),%rdx        # 115d8 <libm::math::generic::sqrt::RSQRT_TAB+0x3e6>
   7e3f5:	movslq (%rdx,%rcx,4),%rcx
   7e3f9:	add    %rdx,%rcx
   7e3fc:	jmp    *%rcx
   7e3fe:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7e403:	test   %al,%al
   7e405:	je     7e6d4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   7e40b:	movd   0x5c(%rsp),%xmm0
   7e411:	pshufd $0x0,%xmm0,%xmm0
   7e416:	mov    %r13,%rcx
   7e419:	shr    $0x4,%rcx
   7e41d:	je     7e679 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   7e423:	lea    0x0(,%r13,2),%rdx
   7e42b:	lea    0x0(,%r13,2),%rsi
   7e433:	add    %r13,%rsi
   7e436:	mov    %r13,%r8
   7e439:	neg    %r8
   7e43c:	mov    %rdx,%r10
   7e43f:	neg    %r10
   7e442:	mov    %rsi,%r14
   7e445:	neg    %r14
   7e448:	lea    (%rsp,%rsi,1),%r15
   7e44c:	add    $0x198,%r15
   7e453:	shl    $0x4,%rcx
   7e457:	xor    %eax,%eax
   7e459:	mov    %rbp,%r9
   7e45c:	xor    %edi,%edi
   7e45e:	movdqa -0x6fd57(%rip),%xmm8        # e710 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   7e467:	movdqa -0x6fef0(%rip),%xmm9        # e580 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x910>
   7e470:	cmp    %rbp,%rdi
   7e473:	ja     7ea3a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   7e479:	mov    %r9,%r11
   7e47c:	sub    $0x10,%r11
   7e480:	jb     7e9b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7e486:	lea    (%rdi,%r13,1),%rbx
   7e48a:	cmp    %rbp,%rbx
   7e48d:	ja     7e9d9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   7e493:	lea    (%r8,%r9,1),%rbx
   7e497:	cmp    $0x10,%rbx
   7e49b:	jb     7e9b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7e4a1:	lea    (%rdx,%rdi,1),%rbx
   7e4a5:	cmp    %rbp,%rbx
   7e4a8:	ja     7e9f0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   7e4ae:	lea    (%r10,%r9,1),%rbx
   7e4b2:	cmp    $0x10,%rbx
   7e4b6:	jb     7e9b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7e4bc:	lea    (%rsi,%rdi,1),%rbx
   7e4c0:	cmp    %rbp,%rbx
   7e4c3:	ja     7ea1e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   7e4c9:	add    %r14,%r9
   7e4cc:	cmp    $0x10,%r9
   7e4d0:	jb     7e9b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7e4d6:	lea    0x3f(%rax),%r9
   7e4da:	cmp    %rbp,%r9
   7e4dd:	jae    7e9c2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   7e4e3:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   7e4ec:	lea    (%rsp,%r13,1),%r9
   7e4f0:	add    $0x198,%r9
   7e4f7:	movdqu (%r9,%rdi,1),%xmm4
   7e4fd:	lea    0x198(%rsp,%r13,2),%r9
   7e505:	movdqu (%r9,%rdi,1),%xmm5
   7e50b:	movdqa %xmm3,%xmm2
   7e50f:	punpckhbw %xmm4,%xmm2
   7e513:	movdqu (%r15,%rdi,1),%xmm6
   7e519:	movdqa %xmm5,%xmm7
   7e51d:	punpckhbw %xmm6,%xmm7
   7e521:	movdqa %xmm2,%xmm1
   7e525:	punpckhwd %xmm7,%xmm1
   7e529:	punpcklwd %xmm7,%xmm2
   7e52d:	punpcklbw %xmm4,%xmm3
   7e531:	punpcklbw %xmm6,%xmm5
   7e535:	movdqa %xmm3,%xmm4
   7e539:	punpckhwd %xmm5,%xmm4
   7e53d:	punpcklwd %xmm5,%xmm3
   7e541:	movdqa %xmm3,%xmm5
   7e545:	psrlw  $0x1,%xmm5
   7e54a:	pand   %xmm8,%xmm3
   7e54f:	pxor   %xmm6,%xmm6
   7e553:	psubb  %xmm3,%xmm6
   7e557:	pand   %xmm9,%xmm5
   7e55c:	pxor   %xmm6,%xmm5
   7e560:	movdqa %xmm5,%xmm3
   7e564:	pslldq $0x4,%xmm3
   7e569:	paddb  %xmm5,%xmm3
   7e56d:	movdqa %xmm3,%xmm5
   7e571:	pslldq $0x8,%xmm5
   7e576:	paddb  %xmm0,%xmm3
   7e57a:	paddb  %xmm5,%xmm3
   7e57e:	movdqu %xmm3,0x8(%r12,%rdi,4)
   7e585:	movdqa %xmm4,%xmm0
   7e589:	psrlw  $0x1,%xmm0
   7e58e:	pand   %xmm8,%xmm4
   7e593:	pxor   %xmm5,%xmm5
   7e597:	psubb  %xmm4,%xmm5
   7e59b:	pand   %xmm9,%xmm0
   7e5a0:	pxor   %xmm5,%xmm0
   7e5a4:	movdqa %xmm0,%xmm4
   7e5a8:	pslldq $0x4,%xmm4
   7e5ad:	paddb  %xmm0,%xmm4
   7e5b1:	movdqa %xmm4,%xmm0
   7e5b5:	pslldq $0x8,%xmm0
   7e5ba:	pshufd $0xff,%xmm3,%xmm3
   7e5bf:	paddb  %xmm4,%xmm0
   7e5c3:	paddb  %xmm3,%xmm0
   7e5c7:	movdqu %xmm0,0x18(%r12,%rdi,4)
   7e5ce:	movdqa %xmm2,%xmm3
   7e5d2:	psrlw  $0x1,%xmm3
   7e5d7:	pand   %xmm8,%xmm2
   7e5dc:	pxor   %xmm4,%xmm4
   7e5e0:	psubb  %xmm2,%xmm4
   7e5e4:	pand   %xmm9,%xmm3
   7e5e9:	pxor   %xmm4,%xmm3
   7e5ed:	movdqa %xmm3,%xmm2
   7e5f1:	pslldq $0x4,%xmm2
   7e5f6:	paddb  %xmm3,%xmm2
   7e5fa:	movdqa %xmm2,%xmm3
   7e5fe:	pslldq $0x8,%xmm3
   7e603:	pshufd $0xff,%xmm0,%xmm0
   7e608:	paddb  %xmm2,%xmm3
   7e60c:	paddb  %xmm0,%xmm3
   7e610:	movdqu %xmm3,0x28(%r12,%rdi,4)
   7e617:	movdqa %xmm1,%xmm0
   7e61b:	psrlw  $0x1,%xmm0
   7e620:	pand   %xmm8,%xmm1
   7e625:	pxor   %xmm2,%xmm2
   7e629:	psubb  %xmm1,%xmm2
   7e62d:	pand   %xmm9,%xmm0
   7e632:	pxor   %xmm2,%xmm0
   7e636:	movdqa %xmm0,%xmm1
   7e63a:	pslldq $0x4,%xmm1
   7e63f:	paddb  %xmm0,%xmm1
   7e643:	movdqa %xmm1,%xmm0
   7e647:	pslldq $0x8,%xmm0
   7e64c:	pshufd $0xff,%xmm3,%xmm2
   7e651:	paddb  %xmm1,%xmm0
   7e655:	paddb  %xmm2,%xmm0
   7e659:	movdqu %xmm0,0x38(%r12,%rdi,4)
   7e660:	add    $0x10,%rdi
   7e664:	pshufd $0xff,%xmm0,%xmm0
   7e669:	add    $0x40,%rax
   7e66d:	mov    %r11,%r9
   7e670:	cmp    %rdi,%rcx
   7e673:	jne    7e470 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   7e679:	mov    %r13d,%eax
   7e67c:	and    $0x1f0,%eax
   7e681:	cmp    %r13,%rax
   7e684:	mov    0x28(%rsp),%r14
   7e689:	je     7e788 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7e68f:	movdqa %xmm0,0x30(%rsp)
   7e695:	lea    0x0(,%rax,4),%edx
   7e69c:	mov    %rbp,%rcx
   7e69f:	sub    %rdx,%rcx
   7e6a2:	jb     7ea93 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   7e6a8:	lea    0x8(%r12),%rsi
   7e6ad:	add    %rsi,%rdx
   7e6b0:	mov    %rax,0x8(%rsp)
   7e6b5:	lea    0x30(%rsp),%rax
   7e6ba:	mov    %rax,(%rsp)
   7e6be:	mov    $0x4,%r9d
   7e6c4:	lea    0x198(%rsp),%rdi
   7e6cc:	mov    %r13,%r8
   7e6cf:	mov    %rbp,%rsi
   7e6d2:	jmp    7e705 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   7e6d4:	lea    0x98(%rsp),%rax
   7e6dc:	mov    %rax,(%rsp)
   7e6e0:	movq   $0x0,0x8(%rsp)
   7e6e9:	mov    $0x4,%r9d
   7e6ef:	lea    0x198(%rsp),%rdi
   7e6f7:	mov    %rbp,%rsi
   7e6fa:	lea    0x8(%r12),%rdx
   7e6ff:	mov    %r13,%r8
   7e702:	mov    %rbp,%rcx
   7e705:	call   7cc30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7e70a:	jmp    7e788 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7e70c:	shr    $0x4,%al
   7e70f:	neg    %al
   7e711:	and    $0x1f,%al
   7e713:	movzbl %al,%eax
   7e716:	lea    0x98(%rsp),%rcx
   7e71e:	mov    %rcx,(%rsp)
   7e722:	mov    %eax,0x8(%rsp)
   7e726:	movq   $0x0,0x10(%rsp)
   7e72f:	mov    $0x4,%r9d
   7e735:	lea    0x198(%rsp),%rdi
   7e73d:	mov    %rbp,%rsi
   7e740:	lea    0x8(%r12),%rdx
   7e745:	mov    %r13,%r8
   7e748:	mov    %rbp,%rcx
   7e74b:	call   7cfe0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   7e750:	jmp    7e788 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7e752:	lea    0x98(%rsp),%rax
   7e75a:	mov    %rax,(%rsp)
   7e75e:	movq   $0x0,0x8(%rsp)
   7e767:	mov    $0x4,%r9d
   7e76d:	lea    0x198(%rsp),%rdi
   7e775:	mov    %rbp,%rsi
   7e778:	lea    0x8(%r12),%rdx
   7e77d:	mov    %r13,%r8
   7e780:	mov    %rbp,%rcx
   7e783:	call   7ce20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7e788:	mov    0x90(%rsp),%rax
   7e790:	cmp    %rax,0x80(%rsp)
   7e798:	je     7ea64 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   7e79e:	mov    0x4(%r12,%rbp,1),%eax
   7e7a3:	mov    %eax,0x98(%rsp)
   7e7aa:	mov    0x408(%r12),%r15
   7e7b2:	cmp    $0x401,%r15
   7e7b9:	jae    7ea4d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   7e7bf:	mov    %eax,0x5c(%rsp)
   7e7c3:	mov    (%r12),%r13
   7e7c7:	mov    0x0(%r13),%rax
   7e7cb:	mov    0x10(%r13),%rbp
   7e7cf:	sub    %rbp,%rax
   7e7d2:	cmp    %rax,%r15
   7e7d5:	ja     7dfbb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   7e7db:	test   %r15,%r15
   7e7de:	jne    7dfd8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   7e7e4:	jmp    7dfed <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   7e7e9:	cmp    $0x2,%ecx
   7e7ec:	je     7ead1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e7f2:	cmp    $0x3,%ecx
   7e7f5:	je     7eaba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e7fb:	test   %cl,%cl
   7e7fd:	lea    -0x6fe04(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e804:	lea    -0x6fcdb(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e80b:	cmove  %rcx,%rax
   7e80f:	mov    0x48(%rsp),%r9
   7e814:	mov    %rax,(%rsp)
   7e818:	lea    0x30(%rsp),%rdi
   7e81d:	mov    0x28(%rsp),%rsi
   7e822:	mov    %r15,%rdx
   7e825:	mov    %r8,%rcx
   7e828:	lea    0x198(%rsp),%r8
   7e830:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e835:	cmpb   $0x0,0x30(%rsp)
   7e83a:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e840:	mov    %r14d,%eax
   7e843:	shr    $0x2,%al
   7e846:	and    $0x3,%al
   7e848:	cmp    $0x2,%al
   7e84a:	je     7ead1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e850:	movzbl %al,%ecx
   7e853:	cmp    $0x3,%ecx
   7e856:	je     7eaba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e85c:	mov    0x38(%rsp),%rcx
   7e861:	test   %al,%al
   7e863:	lea    -0x6fe6a(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e86a:	lea    -0x6fd41(%rip),%rdx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e871:	cmove  %rdx,%rax
   7e875:	cmpb   $0x0,0x27(%rsp)
   7e87a:	lea    0x198(%rsp,%r13,1),%r8
   7e882:	lea    -0x701a9(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e889:	cmove  %rdx,%rax
   7e88d:	mov    %rax,(%rsp)
   7e891:	lea    0x30(%rsp),%rdi
   7e896:	mov    0x28(%rsp),%rsi
   7e89b:	mov    %r15,%rdx
   7e89e:	mov    0x48(%rsp),%r9
   7e8a3:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e8a8:	cmpb   $0x0,0x30(%rsp)
   7e8ad:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e8b3:	mov    %r14d,%eax
   7e8b6:	shr    $0x4,%al
   7e8b9:	and    $0x3,%al
   7e8bb:	cmp    $0x2,%al
   7e8bd:	je     7ead1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e8c3:	movzbl %al,%ecx
   7e8c6:	cmp    $0x3,%ecx
   7e8c9:	je     7eaba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e8cf:	mov    0x38(%rsp),%rcx
   7e8d4:	test   %al,%al
   7e8d6:	lea    -0x6fedd(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e8dd:	lea    -0x6fdb4(%rip),%rdx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e8e4:	cmove  %rdx,%rax
   7e8e8:	cmpb   $0x0,0x27(%rsp)
   7e8ed:	lea    0x198(%rsp,%r13,2),%r8
   7e8f5:	lea    -0x7021c(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e8fc:	cmove  %rdx,%rax
   7e900:	mov    %rax,(%rsp)
   7e904:	lea    0x30(%rsp),%rdi
   7e909:	mov    0x28(%rsp),%rsi
   7e90e:	mov    %r15,%rdx
   7e911:	mov    0x48(%rsp),%r9
   7e916:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e91b:	cmpb   $0x0,0x30(%rsp)
   7e920:	jne    7e99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e922:	shr    $0x6,%r14b
   7e926:	cmp    $0x2,%r14b
   7e92a:	je     7ead1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e930:	movzbl %r14b,%eax
   7e934:	cmp    $0x3,%eax
   7e937:	je     7eaba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e93d:	mov    0x38(%rsp),%rcx
   7e942:	test   %r14b,%r14b
   7e945:	lea    -0x6ff4c(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e94c:	lea    -0x6fe23(%rip),%rdx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e953:	cmove  %rdx,%rax
   7e957:	cmpb   $0x0,0x27(%rsp)
   7e95c:	mov    0x68(%rsp),%rdx
   7e961:	lea    0x198(%rsp,%rdx,1),%r8
   7e969:	lea    -0x70290(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e970:	cmove  %rdx,%rax
   7e974:	mov    %rax,(%rsp)
   7e978:	lea    0x30(%rsp),%rdi
   7e97d:	mov    0x28(%rsp),%rsi
   7e982:	mov    %r15,%rdx
   7e985:	mov    0x48(%rsp),%r9
   7e98a:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7e98f:	cmpb   $0x0,0x30(%rsp)
   7e994:	je     7e3c0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   7e99a:	movzbl 0x31(%rsp),%eax
   7e99f:	add    $0x598,%rsp
   7e9a6:	pop    %rbx
   7e9a7:	pop    %r12
   7e9a9:	pop    %r13
   7e9ab:	pop    %r14
   7e9ad:	pop    %r15
   7e9af:	pop    %rbp
   7e9b0:	ret
   7e9b1:	lea    0xc5940(%rip),%rdi        # 1442f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x34c8>
   7e9b8:	call   *0xca772(%rip)        # 149130 <_DYNAMIC+0x3c8>
   7e9be:	mov    $0x7,%al
   7e9c0:	jmp    7e99f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e9c2:	lea    0x40(%rax),%rsi
   7e9c6:	lea    0xc4d5b(%rip),%rcx        # 143728 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28f8>
   7e9cd:	mov    %rax,%rdi
   7e9d0:	mov    %rbp,%rdx
   7e9d3:	call   *0xca6ff(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7e9d9:	mov    0x60(%rsp),%rcx
   7e9de:	cmp    $0x100,%rcx
   7e9e5:	mov    $0x100,%eax
   7e9ea:	cmovb  %rcx,%rax
   7e9ee:	jmp    7ea37 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   7e9f0:	mov    0x60(%rsp),%rcx
   7e9f5:	cmp    $0x100,%rcx
   7e9fc:	mov    $0x100,%eax
   7ea01:	cmovb  %rcx,%rax
   7ea05:	mov    %eax,%eax
   7ea07:	lea    (%rdi,%rax,2),%rdi
   7ea0b:	lea    0xc58ce(%rip),%rcx        # 1442e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x34b0>
   7ea12:	mov    %rbp,%rsi
   7ea15:	mov    %rbp,%rdx
   7ea18:	call   *0xca6ba(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ea1e:	mov    0x60(%rsp),%rcx
   7ea23:	cmp    $0x100,%rcx
   7ea2a:	mov    $0x100,%eax
   7ea2f:	cmovb  %rcx,%rax
   7ea33:	lea    (%rax,%rax,2),%rax
   7ea37:	add    %rax,%rdi
   7ea3a:	lea    0xc589f(%rip),%rcx        # 1442e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x34b0>
   7ea41:	mov    %rbp,%rsi
   7ea44:	mov    %rbp,%rdx
   7ea47:	call   *0xca68b(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ea4d:	lea    0xc896c(%rip),%rcx        # 1473c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6590>
   7ea54:	mov    $0x400,%edx
   7ea59:	xor    %edi,%edi
   7ea5b:	mov    %r15,%rsi
   7ea5e:	call   *0xca674(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ea64:	lea    -0x4(%rbp),%rdi
   7ea68:	lea    0xc4881(%rip),%rcx        # 1432f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   7ea6f:	mov    %rbp,%rsi
   7ea72:	mov    %rbp,%rdx
   7ea75:	call   *0xca65d(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ea7b:	lea    0xc4946(%rip),%rcx        # 1433c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2598>
   7ea82:	mov    0x70(%rsp),%rdi
   7ea87:	mov    %r15,%rsi
   7ea8a:	mov    %r15,%rdx
   7ea8d:	call   *0xca645(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ea93:	lea    0xc4c76(%rip),%rcx        # 143710 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28e0>
   7ea9a:	mov    %rdx,%rdi
   7ea9d:	mov    %rbp,%rsi
   7eaa0:	mov    %rbp,%rdx
   7eaa3:	call   *0xca62f(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7eaa9:	lea    0xc4870(%rip),%rdx        # 143320 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   7eab0:	xor    %edi,%edi
   7eab2:	xor    %esi,%esi
   7eab4:	call   *0xca4fe(%rip)        # 148fb8 <_DYNAMIC+0x250>
   7eaba:	lea    0xc48a7(%rip),%rcx        # 143368 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   7eac1:	xor    %edi,%edi
   7eac3:	mov    %r13,%rsi
   7eac6:	mov    0x48(%rsp),%rdx
   7eacb:	call   *0xca607(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ead1:	lea    0xc48a8(%rip),%rcx        # 143380 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   7ead8:	xor    %edi,%edi
   7eada:	mov    %r13,%rsi
   7eadd:	mov    0x48(%rsp),%rdx
   7eae2:	call   *0xca5f0(%rip)        # 1490d8 <_DYNAMIC+0x370>
