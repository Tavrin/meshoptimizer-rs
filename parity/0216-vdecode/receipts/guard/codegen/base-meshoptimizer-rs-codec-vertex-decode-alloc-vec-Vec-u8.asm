
<artifacts>/guard/driver-base:     file format elf64-x86-64


Disassembly of section .text:

000000000007d150 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   7d150:	push   %rbp
   7d151:	push   %r15
   7d153:	push   %r14
   7d155:	push   %r13
   7d157:	push   %r12
   7d159:	push   %rbx
   7d15a:	sub    $0x5f8,%rsp
   7d161:	mov    %r8,%r15
   7d164:	mov    %rcx,%r12
   7d167:	mov    %rdx,%r14
   7d16a:	mov    %rsi,%r13
   7d16d:	mov    %rdi,%rbp
   7d170:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7d175:	cmp    $0x2,%al
   7d177:	jb     7d1dd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8d>
   7d179:	cmp    $0x20,%r14
   7d17d:	lea    0x5fcc(%rip),%rax        # 83150 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   7d184:	lea    0x4a55(%rip),%rcx        # 81be0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   7d18b:	cmove  %rax,%rcx
   7d18f:	lea    0xb83a(%rip),%rax        # 889d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   7d196:	lea    0x9ec3(%rip),%rdx        # 87060 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   7d19d:	cmove  %rax,%rdx
   7d1a1:	cmp    $0x4,%r14
   7d1a5:	lea    0x73d4(%rip),%rax        # 84580 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   7d1ac:	cmovne %rcx,%rax
   7d1b0:	lea    0xcff9(%rip),%r9        # 8a1b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   7d1b7:	cmovne %rdx,%r9
   7d1bb:	cmp    $0x21,%r13
   7d1bf:	cmovae %rax,%r9
   7d1c3:	mov    %rbp,%rdi
   7d1c6:	mov    %r13,%rsi
   7d1c9:	mov    %r14,%rdx
   7d1cc:	mov    %r12,%rcx
   7d1cf:	mov    %r15,%r8
   7d1d2:	call   *%r9
   7d1d5:	cmp    $0xfe,%al
   7d1d7:	jne    7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d1dd:	mov    $0x7,%al
   7d1df:	test   %r15,%r15
   7d1e2:	je     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d1e8:	movzbl (%r12),%edx
   7d1ed:	mov    %edx,%ecx
   7d1ef:	and    $0xf0,%cl
   7d1f2:	cmp    $0xa0,%cl
   7d1f5:	jne    7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d1fb:	and    $0xf,%dl
   7d1fe:	mov    $0x5,%al
   7d200:	mov    %dl,0x27(%rsp)
   7d204:	cmp    $0x1,%dl
   7d207:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d20d:	cmpb   $0x0,0x27(%rsp)
   7d212:	je     7d23b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xeb>
   7d214:	mov    %r14,%rcx
   7d217:	shr    $0x2,%rcx
   7d21b:	add    %r14,%rcx
   7d21e:	cmp    $0x19,%rcx
   7d222:	mov    $0x18,%ebx
   7d227:	cmovae %rcx,%rbx
   7d22b:	lea    0x1(%rbx),%rdx
   7d22f:	mov    $0x7,%al
   7d231:	cmp    %rdx,%r15
   7d234:	jae    7d25a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x10a>
   7d236:	jmp    7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d23b:	cmp    $0x21,%r14
   7d23f:	mov    $0x20,%ebx
   7d244:	cmovae %r14,%rbx
   7d248:	mov    %r14,%rcx
   7d24b:	lea    0x1(%rbx),%rdx
   7d24f:	mov    $0x7,%al
   7d251:	cmp    %rdx,%r15
   7d254:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d25a:	mov    %r15,%rsi
   7d25d:	sub    %rcx,%rsi
   7d260:	lea    (%rsi,%r14,1),%rax
   7d264:	mov    %r15,%rcx
   7d267:	mov    %rax,0x70(%rsp)
   7d26c:	sub    %rax,%rcx
   7d26f:	mov    %rcx,0xa8(%rsp)
   7d277:	jb     7dd54 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc04>
   7d27d:	xorps  %xmm0,%xmm0
   7d280:	movaps %xmm0,0x1e0(%rsp)
   7d288:	movaps %xmm0,0x1d0(%rsp)
   7d290:	movaps %xmm0,0x1c0(%rsp)
   7d298:	movaps %xmm0,0x1b0(%rsp)
   7d2a0:	movaps %xmm0,0x1a0(%rsp)
   7d2a8:	movaps %xmm0,0x190(%rsp)
   7d2b0:	movaps %xmm0,0x180(%rsp)
   7d2b8:	movaps %xmm0,0x170(%rsp)
   7d2c0:	movaps %xmm0,0x160(%rsp)
   7d2c8:	movaps %xmm0,0x150(%rsp)
   7d2d0:	movaps %xmm0,0x140(%rsp)
   7d2d8:	movaps %xmm0,0x130(%rsp)
   7d2e0:	movaps %xmm0,0x120(%rsp)
   7d2e8:	movaps %xmm0,0x110(%rsp)
   7d2f0:	movaps %xmm0,0x100(%rsp)
   7d2f8:	movaps %xmm0,0xf0(%rsp)
   7d300:	cmp    $0x101,%r14
   7d307:	jae    7dd10 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbc0>
   7d30d:	lea    (%rsi,%r14,1),%rax
   7d311:	cmp    %rsi,%rax
   7d314:	jb     7dd27 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd7>
   7d31a:	add    %r12,%rsi
   7d31d:	lea    0xf0(%rsp),%rdi
   7d325:	mov    %r14,%rdx
   7d328:	call   *0xcbc52(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7d32e:	test   %r14,%r14
   7d331:	je     7ddac <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc5c>
   7d337:	mov    %rbx,0xd8(%rsp)
   7d33f:	mov    $0x2000,%ax
   7d343:	xor    %edx,%edx
   7d345:	div    %r14w
   7d349:	and    $0x3ff0,%eax
   7d34e:	cmp    $0x100,%eax
   7d353:	mov    $0x100,%ecx
   7d358:	cmovb  %eax,%ecx
   7d35b:	mov    %rcx,0xe0(%rsp)
   7d363:	lea    0x1f8(%rsp),%rdi
   7d36b:	mov    $0x400,%edx
   7d370:	xor    %esi,%esi
   7d372:	call   *0xcbc18(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d378:	test   %r13,%r13
   7d37b:	je     7dcad <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5d>
   7d381:	add    %r12,0x70(%rsp)
   7d386:	mov    %r14,%rax
   7d389:	shr    $0x2,%rax
   7d38d:	mov    %rax,0xa0(%rsp)
   7d395:	lea    0x3(%r14),%rax
   7d399:	shr    $0x2,%rax
   7d39d:	mov    %rax,0xe8(%rsp)
   7d3a5:	mov    $0x1,%ebx
   7d3aa:	xor    %eax,%eax
   7d3ac:	mov    %rbp,0x68(%rsp)
   7d3b1:	mov    %r15,0x30(%rsp)
   7d3b6:	mov    %r12,0x38(%rsp)
   7d3bb:	mov    %r13,0xc0(%rsp)
   7d3c3:	mov    %r13,%rdx
   7d3c6:	mov    %rax,0x58(%rsp)
   7d3cb:	sub    %rax,%rdx
   7d3ce:	mov    0xe0(%rsp),%rax
   7d3d6:	cmp    %rax,%rdx
   7d3d9:	cmovae %rax,%rdx
   7d3dd:	mov    %rdx,0x28(%rsp)
   7d3e2:	cmpb   $0x0,0x27(%rsp)
   7d3e7:	je     7d42c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2dc>
   7d3e9:	mov    %rbx,%rcx
   7d3ec:	mov    $0x7,%al
   7d3ee:	add    0xa0(%rsp),%rcx
   7d3f6:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d3fc:	cmp    %r15,%rcx
   7d3ff:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d405:	add    0x38(%rsp),%rbx
   7d40a:	mov    0xa0(%rsp),%rax
   7d412:	mov    %rax,0x98(%rsp)
   7d41a:	mov    %rbx,0xb0(%rsp)
   7d422:	mov    %rcx,%rbx
   7d425:	mov    0x68(%rsp),%rbp
   7d42a:	jmp    7d445 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2f5>
   7d42c:	movq   $0x0,0x98(%rsp)
   7d438:	mov    $0x1,%eax
   7d43d:	mov    %rax,0xb0(%rsp)
   7d445:	mov    0x58(%rsp),%rax
   7d44a:	mov    0x28(%rsp),%rcx
   7d44f:	lea    (%rcx,%rax,1),%r15
   7d453:	mov    %r15,0xd0(%rsp)
   7d45b:	imul   %r14,%r15
   7d45f:	mov    0x10(%rbp),%r13
   7d463:	mov    %rbp,%rax
   7d466:	mov    %r15,%rbp
   7d469:	mov    %r15,%rdx
   7d46c:	sub    %r13,%rbp
   7d46f:	jbe    7d4b9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   7d471:	mov    (%rax),%rax
   7d474:	sub    %r13,%rax
   7d477:	cmp    %rax,%rbp
   7d47a:	ja     7dc85 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb35>
   7d480:	mov    0x68(%rsp),%rax
   7d485:	mov    0x8(%rax),%r12
   7d489:	lea    (%r12,%r13,1),%rdi
   7d48d:	cmp    $0x2,%rbp
   7d491:	jb     7d4ab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x35b>
   7d493:	lea    -0x1(%rbp),%rdx
   7d497:	xor    %esi,%esi
   7d499:	call   *0xcbaf1(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d49f:	add    %rbp,%r13
   7d4a2:	dec    %r13
   7d4a5:	add    %r13,%r12
   7d4a8:	mov    %r12,%rdi
   7d4ab:	movb   $0x0,(%rdi)
   7d4ae:	inc    %r13
   7d4b1:	mov    %r13,%rdx
   7d4b4:	mov    0x68(%rsp),%rax
   7d4b9:	mov    0x58(%rsp),%rcx
   7d4be:	imul   %r14,%rcx
   7d4c2:	mov    %rdx,0x10(%rax)
   7d4c6:	mov    %rcx,0x58(%rsp)
   7d4cb:	cmp    %rcx,%r15
   7d4ce:	jb     7dd3f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   7d4d4:	cmp    %rdx,%r15
   7d4d7:	ja     7dd3f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   7d4dd:	mov    0x28(%rsp),%rdx
   7d4e2:	lea    0xf(%rdx),%ecx
   7d4e5:	and    $0x3f0,%ecx
   7d4eb:	mov    %rcx,0x50(%rsp)
   7d4f0:	mov    %rdx,%rcx
   7d4f3:	imul   %r14,%rcx
   7d4f7:	mov    %rcx,0xb8(%rsp)
   7d4ff:	mov    0x58(%rsp),%rcx
   7d504:	add    0x8(%rax),%rcx
   7d508:	mov    %rcx,0x58(%rsp)
   7d50d:	lea    -0x1(%rdx),%rax
   7d511:	imul   %r14,%rax
   7d515:	mov    %rax,0xc8(%rsp)
   7d51d:	add    $0x4,%rax
   7d521:	mov    %rax,0x60(%rsp)
   7d526:	lea    0x0(,%rdx,4),%rax
   7d52e:	mov    %rax,0x78(%rsp)
   7d533:	lea    0x1f8(%rsp,%rdx,1),%rax
   7d53b:	mov    %rax,0x90(%rsp)
   7d543:	lea    0x1f8(%rsp,%rdx,2),%rax
   7d54b:	mov    %rax,0x88(%rsp)
   7d553:	lea    (%rdx,%rdx,2),%rax
   7d557:	add    %rsp,%rax
   7d55a:	add    $0x1f8,%rax
   7d560:	mov    %rax,0x80(%rsp)
   7d568:	xor    %r13d,%r13d
   7d56b:	xor    %ebp,%ebp
   7d56d:	movzbl 0x27(%rsp),%r15d
   7d573:	jmp    7d5cf <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x47f>
   7d575:	lea    (%rsp,%rbp,1),%rcx
   7d579:	add    $0xf0,%rcx
   7d580:	shr    $0x4,%al
   7d583:	neg    %al
   7d585:	and    $0x1f,%al
   7d587:	movzbl %al,%eax
   7d58a:	mov    %rcx,(%rsp)
   7d58e:	mov    %eax,0x8(%rsp)
   7d592:	movq   $0x0,0x10(%rsp)
   7d59b:	lea    0x1f8(%rsp),%rdi
   7d5a3:	mov    0x78(%rsp),%rsi
   7d5a8:	mov    0x60(%rsp),%rcx
   7d5ad:	mov    0x28(%rsp),%r8
   7d5b2:	mov    %r14,%r9
   7d5b5:	call   7cfe0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   7d5ba:	inc    %r13
   7d5bd:	add    $0x4,%rbp
   7d5c1:	cmp    %r13,0xe8(%rsp)
   7d5c9:	je     7dc2b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xadb>
   7d5cf:	test   %r15b,%r15b
   7d5d2:	je     7d5f1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a1>
   7d5d4:	cmp    0x98(%rsp),%r13
   7d5dc:	jae    7dd7c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2c>
   7d5e2:	mov    0xb0(%rsp),%rax
   7d5ea:	movzbl (%rax,%r13,1),%r12d
   7d5ef:	jmp    7d5f4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a4>
   7d5f1:	xor    %r12d,%r12d
   7d5f4:	mov    %r12d,%eax
   7d5f7:	and    $0x3,%al
   7d5f9:	movzbl %al,%ecx
   7d5fc:	mov    0x50(%rsp),%rdx
   7d601:	cmp    %rdx,0x28(%rsp)
   7d606:	ja     7da78 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x928>
   7d60c:	cmp    $0x2,%al
   7d60e:	je     7d64d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4fd>
   7d610:	cmp    $0x3,%ecx
   7d613:	jne    7d667 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x517>
   7d615:	mov    %rbx,%r15
   7d618:	mov    $0x7,%al
   7d61a:	add    0x28(%rsp),%r15
   7d61f:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d625:	cmp    0x30(%rsp),%r15
   7d62a:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d630:	add    0x38(%rsp),%rbx
   7d635:	lea    0x1f8(%rsp),%rdi
   7d63d:	mov    %rbx,%rsi
   7d640:	mov    0x28(%rsp),%rdx
   7d645:	call   *0xcb935(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7d64b:	jmp    7d6c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   7d64d:	lea    0x1f8(%rsp),%rdi
   7d655:	xor    %esi,%esi
   7d657:	mov    0x28(%rsp),%rdx
   7d65c:	call   *0xcb92e(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d662:	mov    %rbx,%r15
   7d665:	jmp    7d6c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   7d667:	test   %al,%al
   7d669:	lea    -0x6ec70(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d670:	lea    -0x6eb47(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d677:	cmove  %rcx,%rax
   7d67b:	test   %r15b,%r15b
   7d67e:	lea    -0x6efa5(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d685:	cmove  %rcx,%rax
   7d689:	mov    %rax,(%rsp)
   7d68d:	lea    0x40(%rsp),%rdi
   7d692:	mov    0x38(%rsp),%rsi
   7d697:	mov    0x30(%rsp),%rdx
   7d69c:	mov    %rbx,%rcx
   7d69f:	lea    0x1f8(%rsp),%r8
   7d6a7:	mov    0x50(%rsp),%r9
   7d6ac:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7d6b1:	cmpb   $0x0,0x40(%rsp)
   7d6b6:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7d6bc:	mov    0x48(%rsp),%r15
   7d6c1:	mov    %r12d,%eax
   7d6c4:	shr    $0x2,%al
   7d6c7:	and    $0x3,%al
   7d6c9:	cmp    $0x3,%al
   7d6cb:	je     7d6ef <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x59f>
   7d6cd:	movzbl %al,%ecx
   7d6d0:	cmp    $0x2,%ecx
   7d6d3:	jne    7d72a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5da>
   7d6d5:	mov    0x90(%rsp),%rdi
   7d6dd:	xor    %esi,%esi
   7d6df:	mov    0x28(%rsp),%rdx
   7d6e4:	call   *0xcb8a6(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d6ea:	jmp    7d786 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   7d6ef:	mov    %r15,%rbx
   7d6f2:	mov    $0x7,%al
   7d6f4:	add    0x28(%rsp),%rbx
   7d6f9:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d6ff:	cmp    0x30(%rsp),%rbx
   7d704:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d70a:	add    0x38(%rsp),%r15
   7d70f:	mov    0x90(%rsp),%rdi
   7d717:	mov    %r15,%rsi
   7d71a:	mov    0x28(%rsp),%rdx
   7d71f:	call   *0xcb85b(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7d725:	mov    %rbx,%r15
   7d728:	jmp    7d786 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   7d72a:	test   %al,%al
   7d72c:	lea    -0x6ed33(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d733:	lea    -0x6ec0a(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d73a:	cmove  %rcx,%rax
   7d73e:	cmpb   $0x0,0x27(%rsp)
   7d743:	lea    -0x6f06a(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d74a:	cmove  %rcx,%rax
   7d74e:	mov    %rax,(%rsp)
   7d752:	lea    0x40(%rsp),%rdi
   7d757:	mov    0x38(%rsp),%rsi
   7d75c:	mov    0x30(%rsp),%rdx
   7d761:	mov    %r15,%rcx
   7d764:	mov    0x90(%rsp),%r8
   7d76c:	mov    0x50(%rsp),%r9
   7d771:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7d776:	cmpb   $0x0,0x40(%rsp)
   7d77b:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7d781:	mov    0x48(%rsp),%r15
   7d786:	mov    %r12d,%eax
   7d789:	shr    $0x4,%al
   7d78c:	and    $0x3,%al
   7d78e:	cmp    $0x3,%al
   7d790:	je     7d7c2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x672>
   7d792:	movzbl %al,%ecx
   7d795:	cmp    $0x2,%ecx
   7d798:	jne    7d84a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6fa>
   7d79e:	mov    0x88(%rsp),%rdi
   7d7a6:	xor    %esi,%esi
   7d7a8:	mov    0x28(%rsp),%rdx
   7d7ad:	call   *0xcb7dd(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d7b3:	shr    $0x6,%r12b
   7d7b7:	cmp    $0x3,%r12b
   7d7bb:	jne    7d809 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   7d7bd:	jmp    7d8b4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   7d7c2:	mov    %r15,%rbx
   7d7c5:	mov    $0x7,%al
   7d7c7:	add    0x28(%rsp),%rbx
   7d7cc:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d7d2:	cmp    0x30(%rsp),%rbx
   7d7d7:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d7dd:	add    0x38(%rsp),%r15
   7d7e2:	mov    0x88(%rsp),%rdi
   7d7ea:	mov    %r15,%rsi
   7d7ed:	mov    0x28(%rsp),%rdx
   7d7f2:	call   *0xcb788(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7d7f8:	mov    %rbx,%r15
   7d7fb:	shr    $0x6,%r12b
   7d7ff:	cmp    $0x3,%r12b
   7d803:	je     7d8b4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   7d809:	movzbl %r12b,%eax
   7d80d:	cmp    $0x2,%eax
   7d810:	jne    7d904 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7b4>
   7d816:	mov    0x80(%rsp),%rdi
   7d81e:	xor    %esi,%esi
   7d820:	mov    0x28(%rsp),%rdx
   7d825:	call   *0xcb765(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   7d82b:	mov    %r15,%rbx
   7d82e:	movzbl 0x27(%rsp),%r15d
   7d834:	test   %r15b,%r15b
   7d837:	mov    0xb8(%rsp),%rcx
   7d83f:	jne    7d974 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   7d845:	jmp    7d8fd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   7d84a:	test   %al,%al
   7d84c:	lea    -0x6ee53(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d853:	lea    -0x6ed2a(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d85a:	cmove  %rcx,%rax
   7d85e:	cmpb   $0x0,0x27(%rsp)
   7d863:	lea    -0x6f18a(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d86a:	cmove  %rcx,%rax
   7d86e:	mov    %rax,(%rsp)
   7d872:	lea    0x40(%rsp),%rdi
   7d877:	mov    0x38(%rsp),%rsi
   7d87c:	mov    0x30(%rsp),%rdx
   7d881:	mov    %r15,%rcx
   7d884:	mov    0x88(%rsp),%r8
   7d88c:	mov    0x50(%rsp),%r9
   7d891:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7d896:	cmpb   $0x0,0x40(%rsp)
   7d89b:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7d8a1:	mov    0x48(%rsp),%r15
   7d8a6:	shr    $0x6,%r12b
   7d8aa:	cmp    $0x3,%r12b
   7d8ae:	jne    7d809 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   7d8b4:	mov    %r15,%rbx
   7d8b7:	mov    $0x7,%al
   7d8b9:	add    0x28(%rsp),%rbx
   7d8be:	jb     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d8c4:	cmp    0x30(%rsp),%rbx
   7d8c9:	ja     7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7d8cf:	add    0x38(%rsp),%r15
   7d8d4:	mov    0x80(%rsp),%rdi
   7d8dc:	mov    %r15,%rsi
   7d8df:	mov    0x28(%rsp),%rdx
   7d8e4:	call   *0xcb696(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7d8ea:	movzbl 0x27(%rsp),%r15d
   7d8f0:	test   %r15b,%r15b
   7d8f3:	mov    0xb8(%rsp),%rcx
   7d8fb:	jne    7d974 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   7d8fd:	xor    %eax,%eax
   7d8ff:	jmp    7d98c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x83c>
   7d904:	test   %r12b,%r12b
   7d907:	lea    -0x6ef0e(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d90e:	lea    -0x6ede5(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d915:	cmove  %rcx,%rax
   7d919:	cmpb   $0x0,0x27(%rsp)
   7d91e:	lea    -0x6f245(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d925:	cmove  %rcx,%rax
   7d929:	mov    %rax,(%rsp)
   7d92d:	lea    0x40(%rsp),%rdi
   7d932:	mov    0x38(%rsp),%rsi
   7d937:	mov    0x30(%rsp),%rdx
   7d93c:	mov    %r15,%rcx
   7d93f:	mov    0x80(%rsp),%r8
   7d947:	mov    0x50(%rsp),%r9
   7d94c:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7d951:	cmpb   $0x0,0x40(%rsp)
   7d956:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7d95c:	mov    0x48(%rsp),%rbx
   7d961:	movzbl 0x27(%rsp),%r15d
   7d967:	test   %r15b,%r15b
   7d96a:	mov    0xb8(%rsp),%rcx
   7d972:	je     7d8fd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   7d974:	cmp    0xa8(%rsp),%r13
   7d97c:	jae    7dd94 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc44>
   7d982:	mov    0x70(%rsp),%rax
   7d987:	movzbl (%rax,%r13,1),%eax
   7d98c:	mov    %rcx,%rdx
   7d98f:	sub    %rbp,%rdx
   7d992:	jb     7dcfa <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbaa>
   7d998:	cmp    %rdx,0x60(%rsp)
   7d99d:	ja     7dce6 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb96>
   7d9a3:	mov    0x58(%rsp),%rcx
   7d9a8:	lea    (%rcx,%rbp,1),%rdx
   7d9ac:	mov    %eax,%ecx
   7d9ae:	and    $0x3,%cl
   7d9b1:	movzbl %cl,%ecx
   7d9b4:	lea    -0x6c3f3(%rip),%rsi        # 115c8 <libm::math::generic::sqrt::RSQRT_TAB+0x3d6>
   7d9bb:	movslq (%rsi,%rcx,4),%rcx
   7d9bf:	add    %rsi,%rcx
   7d9c2:	jmp    *%rcx
   7d9c4:	mov    %rdx,%r15
   7d9c7:	lea    (%rsp,%rbp,1),%r12
   7d9cb:	add    $0xf0,%r12
   7d9d2:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7d9d7:	test   %al,%al
   7d9d9:	je     7da3e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8ee>
   7d9db:	mov    %r12,(%rsp)
   7d9df:	lea    0x1f8(%rsp),%rdi
   7d9e7:	mov    0x78(%rsp),%rsi
   7d9ec:	mov    %r15,%rdx
   7d9ef:	mov    0x60(%rsp),%rcx
   7d9f4:	mov    0x28(%rsp),%r8
   7d9f9:	mov    %r14,%r9
   7d9fc:	call   801d0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   7da01:	jmp    7da6d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91d>
   7da03:	lea    (%rsp,%rbp,1),%rax
   7da07:	add    $0xf0,%rax
   7da0d:	mov    %rax,(%rsp)
   7da11:	movq   $0x0,0x8(%rsp)
   7da1a:	lea    0x1f8(%rsp),%rdi
   7da22:	mov    0x78(%rsp),%rsi
   7da27:	mov    0x60(%rsp),%rcx
   7da2c:	mov    0x28(%rsp),%r8
   7da31:	mov    %r14,%r9
   7da34:	call   7ce20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7da39:	jmp    7d5ba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   7da3e:	mov    %r12,(%rsp)
   7da42:	movq   $0x0,0x8(%rsp)
   7da4b:	lea    0x1f8(%rsp),%rdi
   7da53:	mov    0x78(%rsp),%rsi
   7da58:	mov    %r15,%rdx
   7da5b:	mov    0x60(%rsp),%rcx
   7da60:	mov    0x28(%rsp),%r8
   7da65:	mov    %r14,%r9
   7da68:	call   7cc30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7da6d:	movzbl 0x27(%rsp),%r15d
   7da73:	jmp    7d5ba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   7da78:	cmp    $0x2,%al
   7da7a:	je     7ddd2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   7da80:	cmp    $0x3,%ecx
   7da83:	je     7ddb9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   7da89:	test   %al,%al
   7da8b:	lea    -0x6f092(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7da92:	lea    -0x6ef69(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7da99:	cmove  %rcx,%rax
   7da9d:	test   %r15b,%r15b
   7daa0:	lea    -0x6f3c7(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7daa7:	cmove  %rcx,%rax
   7daab:	mov    %rax,(%rsp)
   7daaf:	lea    0x40(%rsp),%rdi
   7dab4:	mov    0x38(%rsp),%rsi
   7dab9:	mov    0x30(%rsp),%rdx
   7dabe:	mov    %rbx,%rcx
   7dac1:	lea    0x1f8(%rsp),%r8
   7dac9:	mov    0x50(%rsp),%r9
   7dace:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7dad3:	cmpb   $0x0,0x40(%rsp)
   7dad8:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7dade:	mov    %r12d,%eax
   7dae1:	shr    $0x2,%al
   7dae4:	and    $0x3,%al
   7dae6:	cmp    $0x2,%al
   7dae8:	movzbl 0x27(%rsp),%edx
   7daed:	je     7ddd2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   7daf3:	movzbl %al,%ecx
   7daf6:	cmp    $0x3,%ecx
   7daf9:	je     7ddb9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   7daff:	test   %al,%al
   7db01:	lea    -0x6f108(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7db08:	lea    -0x6efdf(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7db0f:	cmove  %rcx,%rax
   7db13:	mov    0x48(%rsp),%rcx
   7db18:	test   %dl,%dl
   7db1a:	lea    -0x6f441(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7db21:	cmove  %rdx,%rax
   7db25:	mov    %rax,(%rsp)
   7db29:	lea    0x40(%rsp),%rdi
   7db2e:	mov    0x38(%rsp),%rsi
   7db33:	mov    0x30(%rsp),%rdx
   7db38:	mov    0x90(%rsp),%r8
   7db40:	mov    0x50(%rsp),%r9
   7db45:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7db4a:	cmpb   $0x0,0x40(%rsp)
   7db4f:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7db55:	mov    %r12d,%eax
   7db58:	shr    $0x4,%al
   7db5b:	and    $0x3,%al
   7db5d:	cmp    $0x2,%al
   7db5f:	movzbl 0x27(%rsp),%edx
   7db64:	je     7ddd2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   7db6a:	movzbl %al,%ecx
   7db6d:	cmp    $0x3,%ecx
   7db70:	je     7ddb9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   7db76:	test   %al,%al
   7db78:	lea    -0x6f17f(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7db7f:	lea    -0x6f056(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7db86:	cmove  %rcx,%rax
   7db8a:	mov    0x48(%rsp),%rcx
   7db8f:	test   %dl,%dl
   7db91:	lea    -0x6f4b8(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7db98:	cmove  %rdx,%rax
   7db9c:	mov    %rax,(%rsp)
   7dba0:	lea    0x40(%rsp),%rdi
   7dba5:	mov    0x38(%rsp),%rsi
   7dbaa:	mov    0x30(%rsp),%rdx
   7dbaf:	mov    0x88(%rsp),%r8
   7dbb7:	mov    0x50(%rsp),%r9
   7dbbc:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   7dbc1:	cmpb   $0x0,0x40(%rsp)
   7dbc6:	jne    7dccb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7dbcc:	shr    $0x6,%r12b
   7dbd0:	cmp    $0x2,%r12b
   7dbd4:	movzbl 0x27(%rsp),%edx
   7dbd9:	je     7ddd2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   7dbdf:	movzbl %r12b,%eax
   7dbe3:	cmp    $0x3,%eax
   7dbe6:	je     7ddb9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   7dbec:	test   %r12b,%r12b
   7dbef:	lea    -0x6f1f6(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7dbf6:	lea    -0x6f0cd(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dbfd:	cmove  %rcx,%rax
   7dc01:	mov    0x48(%rsp),%rcx
   7dc06:	test   %dl,%dl
   7dc08:	lea    -0x6f52f(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7dc0f:	cmove  %rdx,%rax
   7dc13:	mov    %rax,(%rsp)
   7dc17:	lea    0x40(%rsp),%rdi
   7dc1c:	mov    0x38(%rsp),%rsi
   7dc21:	mov    0x30(%rsp),%rdx
   7dc26:	jmp    7d93f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ef>
   7dc2b:	mov    0xb8(%rsp),%rdx
   7dc33:	mov    0xc8(%rsp),%rdi
   7dc3b:	cmp    %rdx,%rdi
   7dc3e:	ja     7dd6c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc1c>
   7dc44:	mov    0x58(%rsp),%rsi
   7dc49:	add    %rdi,%rsi
   7dc4c:	lea    0xf0(%rsp),%rdi
   7dc54:	mov    %r14,%rdx
   7dc57:	call   *0xcb323(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   7dc5d:	mov    0xd0(%rsp),%rcx
   7dc65:	mov    %rcx,%rax
   7dc68:	mov    0xc0(%rsp),%r13
   7dc70:	cmp    %r13,%rcx
   7dc73:	mov    0x30(%rsp),%r15
   7dc78:	mov    0x68(%rsp),%rbp
   7dc7d:	jb     7d3c3 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x273>
   7dc83:	jmp    7dcb2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb62>
   7dc85:	mov    $0x1,%ecx
   7dc8a:	mov    $0x1,%r8d
   7dc90:	mov    0x68(%rsp),%r12
   7dc95:	mov    %r12,%rdi
   7dc98:	mov    %r13,%rsi
   7dc9b:	mov    %rbp,%rdx
   7dc9e:	call   8cfb0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   7dca3:	mov    0x10(%r12),%r13
   7dca8:	jmp    7d480 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x330>
   7dcad:	mov    $0x1,%ebx
   7dcb2:	sub    %rbx,%r15
   7dcb5:	setae  %cl
   7dcb8:	cmp    0xd8(%rsp),%r15
   7dcc0:	sete   %al
   7dcc3:	and    %cl,%al
   7dcc5:	neg    %al
   7dcc7:	or     $0x7,%al
   7dcc9:	jmp    7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7dccb:	movzbl 0x41(%rsp),%eax
   7dcd0:	add    $0x5f8,%rsp
   7dcd7:	pop    %rbx
   7dcd8:	pop    %r12
   7dcda:	pop    %r13
   7dcdc:	pop    %r14
   7dcde:	pop    %r15
   7dce0:	pop    %rbp
   7dce1:	ret
   7dce2:	mov    $0x7,%al
   7dce4:	jmp    7dcd0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7dce6:	lea    0xc564b(%rip),%rcx        # 143338 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   7dced:	xor    %edi,%edi
   7dcef:	mov    0x60(%rsp),%rsi
   7dcf4:	call   *0xcb3de(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dcfa:	mov    %rcx,%rdx
   7dcfd:	lea    0xc564c(%rip),%rcx        # 143350 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   7dd04:	mov    %rbp,%rdi
   7dd07:	mov    %rdx,%rsi
   7dd0a:	call   *0xcb3c8(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd10:	lea    0xc5699(%rip),%rcx        # 1433b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   7dd17:	mov    $0x100,%edx
   7dd1c:	xor    %edi,%edi
   7dd1e:	mov    %r14,%rsi
   7dd21:	call   *0xcb3b1(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd27:	lea    0xc566a(%rip),%rcx        # 143398 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   7dd2e:	mov    %rsi,%rdi
   7dd31:	mov    0x70(%rsp),%rsi
   7dd36:	mov    %r15,%rdx
   7dd39:	call   *0xcb399(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd3f:	lea    0xc96da(%rip),%rcx        # 147420 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x65f0>
   7dd46:	mov    0x58(%rsp),%rdi
   7dd4b:	mov    %r15,%rsi
   7dd4e:	call   *0xcb384(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd54:	lea    0xc566d(%rip),%rcx        # 1433c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2598>
   7dd5b:	mov    0x70(%rsp),%rdi
   7dd60:	mov    %r15,%rsi
   7dd63:	mov    %r15,%rdx
   7dd66:	call   *0xcb36c(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd6c:	lea    0xc557d(%rip),%rcx        # 1432f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   7dd73:	mov    %rdx,%rsi
   7dd76:	call   *0xcb35c(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7dd7c:	lea    0xc5585(%rip),%rdx        # 143308 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   7dd83:	mov    %r13,%rdi
   7dd86:	mov    0x98(%rsp),%rsi
   7dd8e:	call   *0xcb224(%rip)        # 148fb8 <_DYNAMIC+0x250>
   7dd94:	lea    0xc5585(%rip),%rdx        # 143320 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   7dd9b:	mov    %r13,%rdi
   7dd9e:	mov    0xa8(%rsp),%rsi
   7dda6:	call   *0xcb20c(%rip)        # 148fb8 <_DYNAMIC+0x250>
   7ddac:	lea    0xc5525(%rip),%rdi        # 1432d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   7ddb3:	call   *0xcb3e7(%rip)        # 1491a0 <_DYNAMIC+0x438>
   7ddb9:	lea    0xc55a8(%rip),%rcx        # 143368 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   7ddc0:	xor    %edi,%edi
   7ddc2:	mov    0x28(%rsp),%rsi
   7ddc7:	mov    0x50(%rsp),%rdx
   7ddcc:	call   *0xcb306(%rip)        # 1490d8 <_DYNAMIC+0x370>
   7ddd2:	lea    0xc55a7(%rip),%rcx        # 143380 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   7ddd9:	xor    %edi,%edi
   7dddb:	mov    0x28(%rsp),%rsi
   7dde0:	mov    0x50(%rsp),%rdx
   7dde5:	call   *0xcb2ed(%rip)        # 1490d8 <_DYNAMIC+0x370>
