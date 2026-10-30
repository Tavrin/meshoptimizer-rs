
<artifacts>/guard/driver-control:     file format elf64-x86-64


Disassembly of section .text:

000000000007d550 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   7d550:	push   %rbp
   7d551:	push   %r15
   7d553:	push   %r14
   7d555:	push   %r13
   7d557:	push   %r12
   7d559:	push   %rbx
   7d55a:	sub    $0x598,%rsp
   7d561:	mov    %rcx,%r15
   7d564:	mov    %rdx,%r14
   7d567:	mov    %rsi,%rbx
   7d56a:	mov    %rdi,%r12
   7d56d:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7d572:	cmp    $0x2,%al
   7d574:	jb     7d5a8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   7d576:	cmp    $0x21,%rbx
   7d57a:	lea    0xd36f(%rip),%rax        # 8a8f0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7d581:	lea    0x7618(%rip),%r9        # 84ba0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7d588:	cmovb  %rax,%r9
   7d58c:	mov    $0x4,%edx
   7d591:	mov    %r12,%rdi
   7d594:	mov    %rbx,%rsi
   7d597:	mov    %r14,%rcx
   7d59a:	mov    %r15,%r8
   7d59d:	call   *%r9
   7d5a0:	cmp    $0xfe,%al
   7d5a2:	jne    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d5a8:	mov    $0x7,%al
   7d5aa:	test   %r15,%r15
   7d5ad:	je     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d5b3:	movzbl (%r14),%ebp
   7d5b7:	mov    %ebp,%ecx
   7d5b9:	and    $0xf0,%cl
   7d5bc:	cmp    $0xa0,%cl
   7d5bf:	jne    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d5c5:	and    $0xf,%bpl
   7d5c9:	mov    $0x5,%al
   7d5cb:	cmp    $0x1,%bpl
   7d5cf:	ja     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d5d5:	xor    %eax,%eax
   7d5d7:	test   %bpl,%bpl
   7d5da:	sete   %al
   7d5dd:	lea    0x18(,%rax,8),%rcx
   7d5e5:	mov    $0x7,%al
   7d5e7:	cmp    %rcx,%r15
   7d5ea:	jbe    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d5f0:	cmp    $0x1,%bpl
   7d5f4:	mov    %r15,%rax
   7d5f7:	adc    $0x0,%rax
   7d5fb:	lea    -0x1(%rax),%rdx
   7d5ff:	mov    %rdx,0x70(%rsp)
   7d604:	cmp    %r15,%rdx
   7d607:	ja     7e1db <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   7d60d:	mov    %rcx,0x88(%rsp)
   7d615:	add    $0xfffffffffffffffb,%rax
   7d619:	pxor   %xmm0,%xmm0
   7d61d:	movdqu %xmm0,0x188(%rsp)
   7d626:	movdqu %xmm0,0x17c(%rsp)
   7d62f:	movdqu %xmm0,0x16c(%rsp)
   7d638:	movdqu %xmm0,0x15c(%rsp)
   7d641:	movdqu %xmm0,0x14c(%rsp)
   7d64a:	movdqu %xmm0,0x13c(%rsp)
   7d653:	movdqu %xmm0,0x12c(%rsp)
   7d65c:	movdqu %xmm0,0x11c(%rsp)
   7d665:	movdqu %xmm0,0x10c(%rsp)
   7d66e:	movdqu %xmm0,0xfc(%rsp)
   7d677:	movdqu %xmm0,0xec(%rsp)
   7d680:	movdqu %xmm0,0xdc(%rsp)
   7d689:	movdqu %xmm0,0xcc(%rsp)
   7d692:	movdqu %xmm0,0xbc(%rsp)
   7d69b:	movdqu %xmm0,0xac(%rsp)
   7d6a4:	movdqu %xmm0,0x9c(%rsp)
   7d6ad:	mov    (%r14,%rax,1),%r13d
   7d6b1:	mov    %r13d,0x98(%rsp)
   7d6b9:	lea    0x198(%rsp),%rdi
   7d6c1:	mov    $0x400,%edx
   7d6c6:	xor    %esi,%esi
   7d6c8:	call   *0xccb9a(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   7d6ce:	test   %rbx,%rbx
   7d6d1:	je     7d6f9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   7d6d3:	mov    %r13d,0x5c(%rsp)
   7d6d8:	mov    $0x1,%r8d
   7d6de:	xor    %ecx,%ecx
   7d6e0:	mov    %r15,0x50(%rsp)
   7d6e5:	mov    %r14,0x28(%rsp)
   7d6ea:	mov    %rbx,0x80(%rsp)
   7d6f2:	mov    %bpl,0x27(%rsp)
   7d6f7:	jmp    7d778 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   7d6f9:	mov    $0x1,%r8d
   7d6ff:	sub    %r8,%r15
   7d702:	setae  %cl
   7d705:	cmp    0x88(%rsp),%r15
   7d70d:	sete   %al
   7d710:	and    %cl,%al
   7d712:	neg    %al
   7d714:	or     $0x7,%al
   7d716:	jmp    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d71b:	mov    $0x1,%ecx
   7d720:	mov    $0x1,%r8d
   7d726:	mov    %r13,%rdi
   7d729:	mov    %rbp,%rsi
   7d72c:	mov    %r15,%rdx
   7d72f:	call   8c710 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   7d734:	mov    0x10(%r13),%rbp
   7d738:	mov    0x8(%r13),%rdi
   7d73c:	add    %rbp,%rdi
   7d73f:	lea    0x8(%r12),%rsi
   7d744:	mov    %r15,%rdx
   7d747:	call   *0xccb0b(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   7d74d:	add    %r15,%rbp
   7d750:	mov    %rbp,0x10(%r13)
   7d754:	mov    0x78(%rsp),%rax
   7d759:	mov    %rax,%rcx
   7d75c:	mov    0x80(%rsp),%rbx
   7d764:	cmp    %rbx,%rax
   7d767:	mov    0x50(%rsp),%r15
   7d76c:	movzbl 0x27(%rsp),%ebp
   7d771:	mov    0x48(%rsp),%r8
   7d776:	jae    7d6ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   7d778:	sub    %rcx,%rbx
   7d77b:	cmp    $0x100,%rbx
   7d782:	mov    $0x100,%r13d
   7d788:	cmovb  %rbx,%r13
   7d78c:	lea    0xf(%r13),%r9d
   7d790:	and    $0x3f0,%r9d
   7d797:	test   %bpl,%bpl
   7d79a:	mov    %r9,0x48(%rsp)
   7d79f:	mov    %rcx,0x90(%rsp)
   7d7a7:	mov    %rbx,0x60(%rsp)
   7d7ac:	je     7d83d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   7d7b2:	mov    $0x7,%al
   7d7b4:	cmp    %r15,%r8
   7d7b7:	jae    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d7bd:	add    %r13,%rcx
   7d7c0:	mov    %rcx,0x78(%rsp)
   7d7c5:	lea    0x0(,%r13,4),%rbp
   7d7cd:	mov    %rbp,0x408(%r12)
   7d7d5:	movzbl (%r14,%r8,1),%r14d
   7d7da:	inc    %r8
   7d7dd:	mov    %r14d,%ecx
   7d7e0:	and    $0x3,%ecx
   7d7e3:	lea    0x0(,%r13,2),%rdx
   7d7eb:	add    %r13,%rdx
   7d7ee:	mov    %rdx,0x68(%rsp)
   7d7f3:	cmp    %r9,%r13
   7d7f6:	ja     7df49 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   7d7fc:	cmp    $0x2,%cl
   7d7ff:	je     7d87d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   7d801:	movzbl %cl,%edx
   7d804:	cmp    $0x3,%edx
   7d807:	jne    7d895 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   7d80d:	lea    (%r8,%r13,1),%rbx
   7d811:	cmp    %r15,%rbx
   7d814:	ja     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d81a:	add    0x28(%rsp),%r8
   7d81f:	lea    0x198(%rsp),%rdi
   7d827:	mov    %r8,%rsi
   7d82a:	mov    %r13,%rdx
   7d82d:	call   *0xcca25(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   7d833:	mov    0x50(%rsp),%r15
   7d838:	jmp    7d8df <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7d83d:	lea    (%rcx,%r13,1),%rax
   7d841:	mov    %rax,0x78(%rsp)
   7d846:	lea    0x0(,%r13,4),%rbp
   7d84e:	mov    %rbp,0x408(%r12)
   7d856:	xor    %r14d,%r14d
   7d859:	lea    0x0(,%r13,2),%rax
   7d861:	add    %r13,%rax
   7d864:	mov    %rax,0x68(%rsp)
   7d869:	lea    -0x6f0d0(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d870:	mov    %rcx,%rax
   7d873:	cmp    %r9,%r13
   7d876:	jbe    7d8ae <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   7d878:	jmp    7df74 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   7d87d:	lea    0x198(%rsp),%rdi
   7d885:	xor    %esi,%esi
   7d887:	mov    %r13,%rdx
   7d88a:	mov    %r8,%rbx
   7d88d:	call   *0xcc9d5(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   7d893:	jmp    7d8df <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7d895:	test   %cl,%cl
   7d897:	lea    -0x6edde(%rip),%rcx        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d89e:	lea    -0x6ecb5(%rip),%rax        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d8a5:	cmove  %rax,%rcx
   7d8a9:	mov    0x48(%rsp),%r9
   7d8ae:	mov    %rcx,(%rsp)
   7d8b2:	lea    0x30(%rsp),%rdi
   7d8b7:	mov    0x28(%rsp),%rsi
   7d8bc:	mov    %r15,%rdx
   7d8bf:	mov    %r8,%rcx
   7d8c2:	lea    0x198(%rsp),%r8
   7d8ca:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7d8cf:	cmpb   $0x0,0x30(%rsp)
   7d8d4:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7d8da:	mov    0x38(%rsp),%rbx
   7d8df:	mov    %r14d,%eax
   7d8e2:	shr    $0x2,%al
   7d8e5:	and    $0x3,%al
   7d8e7:	lea    (%rsp,%r13,1),%r8
   7d8eb:	add    $0x198,%r8
   7d8f2:	cmp    $0x3,%al
   7d8f4:	je     7d911 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   7d8f6:	movzbl %al,%ecx
   7d8f9:	cmp    $0x2,%ecx
   7d8fc:	jne    7d948 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   7d8fe:	mov    %r8,%rdi
   7d901:	xor    %esi,%esi
   7d903:	mov    %r13,%rdx
   7d906:	call   *0xcc95c(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   7d90c:	jmp    7d99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7d911:	mov    %rbx,%r15
   7d914:	mov    $0x7,%al
   7d916:	add    %r13,%r15
   7d919:	jb     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d91f:	cmp    0x50(%rsp),%r15
   7d924:	ja     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d92a:	add    0x28(%rsp),%rbx
   7d92f:	mov    %r8,%rdi
   7d932:	mov    %rbx,%rsi
   7d935:	mov    %r13,%rdx
   7d938:	call   *0xcc91a(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   7d93e:	mov    %r15,%rbx
   7d941:	mov    0x50(%rsp),%r15
   7d946:	jmp    7d99a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7d948:	test   %al,%al
   7d94a:	lea    -0x6ee91(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7d951:	lea    -0x6ed68(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7d958:	cmove  %rcx,%rax
   7d95c:	cmpb   $0x0,0x27(%rsp)
   7d961:	lea    -0x6f1c8(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7d968:	cmove  %rcx,%rax
   7d96c:	mov    %rax,(%rsp)
   7d970:	lea    0x30(%rsp),%rdi
   7d975:	mov    0x28(%rsp),%rsi
   7d97a:	mov    %r15,%rdx
   7d97d:	mov    %rbx,%rcx
   7d980:	mov    0x48(%rsp),%r9
   7d985:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7d98a:	cmpb   $0x0,0x30(%rsp)
   7d98f:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7d995:	mov    0x38(%rsp),%rbx
   7d99a:	mov    %r14d,%eax
   7d99d:	shr    $0x4,%al
   7d9a0:	and    $0x3,%al
   7d9a2:	lea    (%rsp,%r13,2),%r8
   7d9a6:	add    $0x198,%r8
   7d9ad:	cmp    $0x3,%al
   7d9af:	je     7d9cc <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   7d9b1:	movzbl %al,%ecx
   7d9b4:	cmp    $0x2,%ecx
   7d9b7:	jne    7da03 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   7d9b9:	mov    %r8,%rdi
   7d9bc:	xor    %esi,%esi
   7d9be:	mov    %r13,%rdx
   7d9c1:	call   *0xcc8a1(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   7d9c7:	jmp    7da55 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7d9cc:	mov    %rbx,%r15
   7d9cf:	mov    $0x7,%al
   7d9d1:	add    %r13,%r15
   7d9d4:	jb     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d9da:	cmp    0x50(%rsp),%r15
   7d9df:	ja     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d9e5:	add    0x28(%rsp),%rbx
   7d9ea:	mov    %r8,%rdi
   7d9ed:	mov    %rbx,%rsi
   7d9f0:	mov    %r13,%rdx
   7d9f3:	call   *0xcc85f(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   7d9f9:	mov    %r15,%rbx
   7d9fc:	mov    0x50(%rsp),%r15
   7da01:	jmp    7da55 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7da03:	test   %al,%al
   7da05:	lea    -0x6ef4c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7da0c:	lea    -0x6ee23(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7da13:	cmove  %rcx,%rax
   7da17:	cmpb   $0x0,0x27(%rsp)
   7da1c:	lea    -0x6f283(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7da23:	cmove  %rcx,%rax
   7da27:	mov    %rax,(%rsp)
   7da2b:	lea    0x30(%rsp),%rdi
   7da30:	mov    0x28(%rsp),%rsi
   7da35:	mov    %r15,%rdx
   7da38:	mov    %rbx,%rcx
   7da3b:	mov    0x48(%rsp),%r9
   7da40:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7da45:	cmpb   $0x0,0x30(%rsp)
   7da4a:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7da50:	mov    0x38(%rsp),%rbx
   7da55:	shr    $0x6,%r14b
   7da59:	mov    0x68(%rsp),%rax
   7da5e:	lea    (%rsp,%rax,1),%r8
   7da62:	add    $0x198,%r8
   7da69:	cmp    $0x3,%r14b
   7da6d:	je     7da8b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   7da6f:	movzbl %r14b,%eax
   7da73:	cmp    $0x2,%eax
   7da76:	jne    7dad2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   7da78:	mov    %r8,%rdi
   7da7b:	xor    %esi,%esi
   7da7d:	mov    %r13,%rdx
   7da80:	call   *0xcc7e2(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   7da86:	jmp    7db25 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7da8b:	mov    %rbx,%rcx
   7da8e:	mov    $0x7,%al
   7da90:	add    %r13,%rcx
   7da93:	jb     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7da99:	cmp    %r15,%rcx
   7da9c:	ja     7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7daa2:	mov    0x28(%rsp),%r14
   7daa7:	add    %r14,%rbx
   7daaa:	mov    %r8,%rdi
   7daad:	mov    %rbx,%rsi
   7dab0:	mov    %r13,%rdx
   7dab3:	mov    %rcx,%rbx
   7dab6:	call   *0xcc79c(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   7dabc:	mov    0x50(%rsp),%r15
   7dac1:	cmpb   $0x0,0x27(%rsp)
   7dac6:	mov    %rbx,0x48(%rsp)
   7dacb:	jne    7db36 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   7dacd:	jmp    7db5e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7dad2:	test   %r14b,%r14b
   7dad5:	lea    -0x6f01c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7dadc:	lea    -0x6eef3(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dae3:	cmove  %rcx,%rax
   7dae7:	cmpb   $0x0,0x27(%rsp)
   7daec:	lea    -0x6f353(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7daf3:	cmove  %rcx,%rax
   7daf7:	mov    %rax,(%rsp)
   7dafb:	lea    0x30(%rsp),%rdi
   7db00:	mov    0x28(%rsp),%rsi
   7db05:	mov    %r15,%rdx
   7db08:	mov    %rbx,%rcx
   7db0b:	mov    0x48(%rsp),%r9
   7db10:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7db15:	cmpb   $0x0,0x30(%rsp)
   7db1a:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7db20:	mov    0x38(%rsp),%rbx
   7db25:	mov    0x28(%rsp),%r14
   7db2a:	cmpb   $0x0,0x27(%rsp)
   7db2f:	mov    %rbx,0x48(%rsp)
   7db34:	je     7db5e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7db36:	mov    0x70(%rsp),%rax
   7db3b:	cmp    %rax,%r15
   7db3e:	je     7e209 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   7db44:	movzbl (%r14,%rax,1),%eax
   7db49:	mov    %eax,%ecx
   7db4b:	and    $0x3,%ecx
   7db4e:	lea    -0x6c4d5(%rip),%rdx        # 11680 <libm::math::generic::sqrt::RSQRT_TAB+0x3d6>
   7db55:	movslq (%rdx,%rcx,4),%rcx
   7db59:	add    %rdx,%rcx
   7db5c:	jmp    *%rcx
   7db5e:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7db63:	test   %al,%al
   7db65:	je     7de34 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   7db6b:	movd   0x5c(%rsp),%xmm0
   7db71:	pshufd $0x0,%xmm0,%xmm0
   7db76:	mov    %r13,%rcx
   7db79:	shr    $0x4,%rcx
   7db7d:	je     7ddd9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   7db83:	lea    0x0(,%r13,2),%rdx
   7db8b:	lea    0x0(,%r13,2),%rsi
   7db93:	add    %r13,%rsi
   7db96:	mov    %r13,%r8
   7db99:	neg    %r8
   7db9c:	mov    %rdx,%r10
   7db9f:	neg    %r10
   7dba2:	mov    %rsi,%r14
   7dba5:	neg    %r14
   7dba8:	lea    (%rsp,%rsi,1),%r15
   7dbac:	add    $0x198,%r15
   7dbb3:	shl    $0x4,%rcx
   7dbb7:	xor    %eax,%eax
   7dbb9:	mov    %rbp,%r9
   7dbbc:	xor    %edi,%edi
   7dbbe:	movdqa -0x6f3f7(%rip),%xmm8        # e7d0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   7dbc7:	movdqa -0x6f590(%rip),%xmm9        # e640 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x913>
   7dbd0:	cmp    %rbp,%rdi
   7dbd3:	ja     7e19a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   7dbd9:	mov    %r9,%r11
   7dbdc:	sub    $0x10,%r11
   7dbe0:	jb     7e111 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7dbe6:	lea    (%rdi,%r13,1),%rbx
   7dbea:	cmp    %rbp,%rbx
   7dbed:	ja     7e139 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   7dbf3:	lea    (%r8,%r9,1),%rbx
   7dbf7:	cmp    $0x10,%rbx
   7dbfb:	jb     7e111 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7dc01:	lea    (%rdx,%rdi,1),%rbx
   7dc05:	cmp    %rbp,%rbx
   7dc08:	ja     7e150 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   7dc0e:	lea    (%r10,%r9,1),%rbx
   7dc12:	cmp    $0x10,%rbx
   7dc16:	jb     7e111 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7dc1c:	lea    (%rsi,%rdi,1),%rbx
   7dc20:	cmp    %rbp,%rbx
   7dc23:	ja     7e17e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   7dc29:	add    %r14,%r9
   7dc2c:	cmp    $0x10,%r9
   7dc30:	jb     7e111 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7dc36:	lea    0x3f(%rax),%r9
   7dc3a:	cmp    %rbp,%r9
   7dc3d:	jae    7e122 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   7dc43:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   7dc4c:	lea    (%rsp,%r13,1),%r9
   7dc50:	add    $0x198,%r9
   7dc57:	movdqu (%r9,%rdi,1),%xmm4
   7dc5d:	lea    0x198(%rsp,%r13,2),%r9
   7dc65:	movdqu (%r9,%rdi,1),%xmm5
   7dc6b:	movdqa %xmm3,%xmm2
   7dc6f:	punpckhbw %xmm4,%xmm2
   7dc73:	movdqu (%r15,%rdi,1),%xmm6
   7dc79:	movdqa %xmm5,%xmm7
   7dc7d:	punpckhbw %xmm6,%xmm7
   7dc81:	movdqa %xmm2,%xmm1
   7dc85:	punpckhwd %xmm7,%xmm1
   7dc89:	punpcklwd %xmm7,%xmm2
   7dc8d:	punpcklbw %xmm4,%xmm3
   7dc91:	punpcklbw %xmm6,%xmm5
   7dc95:	movdqa %xmm3,%xmm4
   7dc99:	punpckhwd %xmm5,%xmm4
   7dc9d:	punpcklwd %xmm5,%xmm3
   7dca1:	movdqa %xmm3,%xmm5
   7dca5:	psrlw  $0x1,%xmm5
   7dcaa:	pand   %xmm8,%xmm3
   7dcaf:	pxor   %xmm6,%xmm6
   7dcb3:	psubb  %xmm3,%xmm6
   7dcb7:	pand   %xmm9,%xmm5
   7dcbc:	pxor   %xmm6,%xmm5
   7dcc0:	movdqa %xmm5,%xmm3
   7dcc4:	pslldq $0x4,%xmm3
   7dcc9:	paddb  %xmm5,%xmm3
   7dccd:	movdqa %xmm3,%xmm5
   7dcd1:	pslldq $0x8,%xmm5
   7dcd6:	paddb  %xmm0,%xmm3
   7dcda:	paddb  %xmm5,%xmm3
   7dcde:	movdqu %xmm3,0x8(%r12,%rdi,4)
   7dce5:	movdqa %xmm4,%xmm0
   7dce9:	psrlw  $0x1,%xmm0
   7dcee:	pand   %xmm8,%xmm4
   7dcf3:	pxor   %xmm5,%xmm5
   7dcf7:	psubb  %xmm4,%xmm5
   7dcfb:	pand   %xmm9,%xmm0
   7dd00:	pxor   %xmm5,%xmm0
   7dd04:	movdqa %xmm0,%xmm4
   7dd08:	pslldq $0x4,%xmm4
   7dd0d:	paddb  %xmm0,%xmm4
   7dd11:	movdqa %xmm4,%xmm0
   7dd15:	pslldq $0x8,%xmm0
   7dd1a:	pshufd $0xff,%xmm3,%xmm3
   7dd1f:	paddb  %xmm4,%xmm0
   7dd23:	paddb  %xmm3,%xmm0
   7dd27:	movdqu %xmm0,0x18(%r12,%rdi,4)
   7dd2e:	movdqa %xmm2,%xmm3
   7dd32:	psrlw  $0x1,%xmm3
   7dd37:	pand   %xmm8,%xmm2
   7dd3c:	pxor   %xmm4,%xmm4
   7dd40:	psubb  %xmm2,%xmm4
   7dd44:	pand   %xmm9,%xmm3
   7dd49:	pxor   %xmm4,%xmm3
   7dd4d:	movdqa %xmm3,%xmm2
   7dd51:	pslldq $0x4,%xmm2
   7dd56:	paddb  %xmm3,%xmm2
   7dd5a:	movdqa %xmm2,%xmm3
   7dd5e:	pslldq $0x8,%xmm3
   7dd63:	pshufd $0xff,%xmm0,%xmm0
   7dd68:	paddb  %xmm2,%xmm3
   7dd6c:	paddb  %xmm0,%xmm3
   7dd70:	movdqu %xmm3,0x28(%r12,%rdi,4)
   7dd77:	movdqa %xmm1,%xmm0
   7dd7b:	psrlw  $0x1,%xmm0
   7dd80:	pand   %xmm8,%xmm1
   7dd85:	pxor   %xmm2,%xmm2
   7dd89:	psubb  %xmm1,%xmm2
   7dd8d:	pand   %xmm9,%xmm0
   7dd92:	pxor   %xmm2,%xmm0
   7dd96:	movdqa %xmm0,%xmm1
   7dd9a:	pslldq $0x4,%xmm1
   7dd9f:	paddb  %xmm0,%xmm1
   7dda3:	movdqa %xmm1,%xmm0
   7dda7:	pslldq $0x8,%xmm0
   7ddac:	pshufd $0xff,%xmm3,%xmm2
   7ddb1:	paddb  %xmm1,%xmm0
   7ddb5:	paddb  %xmm2,%xmm0
   7ddb9:	movdqu %xmm0,0x38(%r12,%rdi,4)
   7ddc0:	add    $0x10,%rdi
   7ddc4:	pshufd $0xff,%xmm0,%xmm0
   7ddc9:	add    $0x40,%rax
   7ddcd:	mov    %r11,%r9
   7ddd0:	cmp    %rdi,%rcx
   7ddd3:	jne    7dbd0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   7ddd9:	mov    %r13d,%eax
   7dddc:	and    $0x1f0,%eax
   7dde1:	cmp    %r13,%rax
   7dde4:	mov    0x28(%rsp),%r14
   7dde9:	je     7dee8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7ddef:	movdqa %xmm0,0x30(%rsp)
   7ddf5:	lea    0x0(,%rax,4),%edx
   7ddfc:	mov    %rbp,%rcx
   7ddff:	sub    %rdx,%rcx
   7de02:	jb     7e1f3 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   7de08:	lea    0x8(%r12),%rsi
   7de0d:	add    %rsi,%rdx
   7de10:	mov    %rax,0x8(%rsp)
   7de15:	lea    0x30(%rsp),%rax
   7de1a:	mov    %rax,(%rsp)
   7de1e:	mov    $0x4,%r9d
   7de24:	lea    0x198(%rsp),%rdi
   7de2c:	mov    %r13,%r8
   7de2f:	mov    %rbp,%rsi
   7de32:	jmp    7de65 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   7de34:	lea    0x98(%rsp),%rax
   7de3c:	mov    %rax,(%rsp)
   7de40:	movq   $0x0,0x8(%rsp)
   7de49:	mov    $0x4,%r9d
   7de4f:	lea    0x198(%rsp),%rdi
   7de57:	mov    %rbp,%rsi
   7de5a:	lea    0x8(%r12),%rdx
   7de5f:	mov    %r13,%r8
   7de62:	mov    %rbp,%rcx
   7de65:	call   7d030 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7de6a:	jmp    7dee8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7de6c:	shr    $0x4,%al
   7de6f:	neg    %al
   7de71:	and    $0x1f,%al
   7de73:	movzbl %al,%eax
   7de76:	lea    0x98(%rsp),%rcx
   7de7e:	mov    %rcx,(%rsp)
   7de82:	mov    %eax,0x8(%rsp)
   7de86:	movq   $0x0,0x10(%rsp)
   7de8f:	mov    $0x4,%r9d
   7de95:	lea    0x198(%rsp),%rdi
   7de9d:	mov    %rbp,%rsi
   7dea0:	lea    0x8(%r12),%rdx
   7dea5:	mov    %r13,%r8
   7dea8:	mov    %rbp,%rcx
   7deab:	call   7d3e0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   7deb0:	jmp    7dee8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7deb2:	lea    0x98(%rsp),%rax
   7deba:	mov    %rax,(%rsp)
   7debe:	movq   $0x0,0x8(%rsp)
   7dec7:	mov    $0x4,%r9d
   7decd:	lea    0x198(%rsp),%rdi
   7ded5:	mov    %rbp,%rsi
   7ded8:	lea    0x8(%r12),%rdx
   7dedd:	mov    %r13,%r8
   7dee0:	mov    %rbp,%rcx
   7dee3:	call   7d220 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7dee8:	mov    0x90(%rsp),%rax
   7def0:	cmp    %rax,0x80(%rsp)
   7def8:	je     7e1c4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   7defe:	mov    0x4(%r12,%rbp,1),%eax
   7df03:	mov    %eax,0x98(%rsp)
   7df0a:	mov    0x408(%r12),%r15
   7df12:	cmp    $0x401,%r15
   7df19:	jae    7e1ad <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   7df1f:	mov    %eax,0x5c(%rsp)
   7df23:	mov    (%r12),%r13
   7df27:	mov    0x0(%r13),%rax
   7df2b:	mov    0x10(%r13),%rbp
   7df2f:	sub    %rbp,%rax
   7df32:	cmp    %rax,%r15
   7df35:	ja     7d71b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   7df3b:	test   %r15,%r15
   7df3e:	jne    7d738 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   7df44:	jmp    7d74d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   7df49:	cmp    $0x2,%ecx
   7df4c:	je     7e231 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7df52:	cmp    $0x3,%ecx
   7df55:	je     7e21a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7df5b:	test   %cl,%cl
   7df5d:	lea    -0x6f4a4(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7df64:	lea    -0x6f37b(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7df6b:	cmove  %rcx,%rax
   7df6f:	mov    0x48(%rsp),%r9
   7df74:	mov    %rax,(%rsp)
   7df78:	lea    0x30(%rsp),%rdi
   7df7d:	mov    0x28(%rsp),%rsi
   7df82:	mov    %r15,%rdx
   7df85:	mov    %r8,%rcx
   7df88:	lea    0x198(%rsp),%r8
   7df90:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7df95:	cmpb   $0x0,0x30(%rsp)
   7df9a:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7dfa0:	mov    %r14d,%eax
   7dfa3:	shr    $0x2,%al
   7dfa6:	and    $0x3,%al
   7dfa8:	cmp    $0x2,%al
   7dfaa:	je     7e231 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7dfb0:	movzbl %al,%ecx
   7dfb3:	cmp    $0x3,%ecx
   7dfb6:	je     7e21a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7dfbc:	mov    0x38(%rsp),%rcx
   7dfc1:	test   %al,%al
   7dfc3:	lea    -0x6f50a(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7dfca:	lea    -0x6f3e1(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dfd1:	cmove  %rdx,%rax
   7dfd5:	cmpb   $0x0,0x27(%rsp)
   7dfda:	lea    0x198(%rsp,%r13,1),%r8
   7dfe2:	lea    -0x6f849(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7dfe9:	cmove  %rdx,%rax
   7dfed:	mov    %rax,(%rsp)
   7dff1:	lea    0x30(%rsp),%rdi
   7dff6:	mov    0x28(%rsp),%rsi
   7dffb:	mov    %r15,%rdx
   7dffe:	mov    0x48(%rsp),%r9
   7e003:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7e008:	cmpb   $0x0,0x30(%rsp)
   7e00d:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e013:	mov    %r14d,%eax
   7e016:	shr    $0x4,%al
   7e019:	and    $0x3,%al
   7e01b:	cmp    $0x2,%al
   7e01d:	je     7e231 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e023:	movzbl %al,%ecx
   7e026:	cmp    $0x3,%ecx
   7e029:	je     7e21a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e02f:	mov    0x38(%rsp),%rcx
   7e034:	test   %al,%al
   7e036:	lea    -0x6f57d(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e03d:	lea    -0x6f454(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e044:	cmove  %rdx,%rax
   7e048:	cmpb   $0x0,0x27(%rsp)
   7e04d:	lea    0x198(%rsp,%r13,2),%r8
   7e055:	lea    -0x6f8bc(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e05c:	cmove  %rdx,%rax
   7e060:	mov    %rax,(%rsp)
   7e064:	lea    0x30(%rsp),%rdi
   7e069:	mov    0x28(%rsp),%rsi
   7e06e:	mov    %r15,%rdx
   7e071:	mov    0x48(%rsp),%r9
   7e076:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7e07b:	cmpb   $0x0,0x30(%rsp)
   7e080:	jne    7e0fa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e082:	shr    $0x6,%r14b
   7e086:	cmp    $0x2,%r14b
   7e08a:	je     7e231 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e090:	movzbl %r14b,%eax
   7e094:	cmp    $0x3,%eax
   7e097:	je     7e21a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e09d:	mov    0x38(%rsp),%rcx
   7e0a2:	test   %r14b,%r14b
   7e0a5:	lea    -0x6f5ec(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e0ac:	lea    -0x6f4c3(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e0b3:	cmove  %rdx,%rax
   7e0b7:	cmpb   $0x0,0x27(%rsp)
   7e0bc:	mov    0x68(%rsp),%rdx
   7e0c1:	lea    0x198(%rsp,%rdx,1),%r8
   7e0c9:	lea    -0x6f930(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e0d0:	cmove  %rdx,%rax
   7e0d4:	mov    %rax,(%rsp)
   7e0d8:	lea    0x30(%rsp),%rdi
   7e0dd:	mov    0x28(%rsp),%rsi
   7e0e2:	mov    %r15,%rdx
   7e0e5:	mov    0x48(%rsp),%r9
   7e0ea:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   7e0ef:	cmpb   $0x0,0x30(%rsp)
   7e0f4:	je     7db20 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   7e0fa:	movzbl 0x31(%rsp),%eax
   7e0ff:	add    $0x598,%rsp
   7e106:	pop    %rbx
   7e107:	pop    %r12
   7e109:	pop    %r13
   7e10b:	pop    %r14
   7e10d:	pop    %r15
   7e10f:	pop    %rbp
   7e110:	ret
   7e111:	lea    0xc7428(%rip),%rdi        # 145540 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x34b0>
   7e118:	call   *0xcc2ea(%rip)        # 14a408 <_DYNAMIC+0x3c8>
   7e11e:	mov    $0x7,%al
   7e120:	jmp    7e0ff <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e122:	lea    0x40(%rax),%rsi
   7e126:	lea    0xc6843(%rip),%rcx        # 144970 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28e0>
   7e12d:	mov    %rax,%rdi
   7e130:	mov    %rbp,%rdx
   7e133:	call   *0xcc277(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e139:	mov    0x60(%rsp),%rcx
   7e13e:	cmp    $0x100,%rcx
   7e145:	mov    $0x100,%eax
   7e14a:	cmovb  %rcx,%rax
   7e14e:	jmp    7e197 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   7e150:	mov    0x60(%rsp),%rcx
   7e155:	cmp    $0x100,%rcx
   7e15c:	mov    $0x100,%eax
   7e161:	cmovb  %rcx,%rax
   7e165:	mov    %eax,%eax
   7e167:	lea    (%rdi,%rax,2),%rdi
   7e16b:	lea    0xc73b6(%rip),%rcx        # 145528 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3498>
   7e172:	mov    %rbp,%rsi
   7e175:	mov    %rbp,%rdx
   7e178:	call   *0xcc232(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e17e:	mov    0x60(%rsp),%rcx
   7e183:	cmp    $0x100,%rcx
   7e18a:	mov    $0x100,%eax
   7e18f:	cmovb  %rcx,%rax
   7e193:	lea    (%rax,%rax,2),%rax
   7e197:	add    %rax,%rdi
   7e19a:	lea    0xc7387(%rip),%rcx        # 145528 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3498>
   7e1a1:	mov    %rbp,%rsi
   7e1a4:	mov    %rbp,%rdx
   7e1a7:	call   *0xcc203(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e1ad:	lea    0xca4e4(%rip),%rcx        # 148698 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6608>
   7e1b4:	mov    $0x400,%edx
   7e1b9:	xor    %edi,%edi
   7e1bb:	mov    %r15,%rsi
   7e1be:	call   *0xcc1ec(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e1c4:	lea    -0x4(%rbp),%rdi
   7e1c8:	lea    0xc6369(%rip),%rcx        # 144538 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   7e1cf:	mov    %rbp,%rsi
   7e1d2:	mov    %rbp,%rdx
   7e1d5:	call   *0xcc1d5(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e1db:	lea    0xc642e(%rip),%rcx        # 144610 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   7e1e2:	mov    0x70(%rsp),%rdi
   7e1e7:	mov    %r15,%rsi
   7e1ea:	mov    %r15,%rdx
   7e1ed:	call   *0xcc1bd(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e1f3:	lea    0xc675e(%rip),%rcx        # 144958 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28c8>
   7e1fa:	mov    %rdx,%rdi
   7e1fd:	mov    %rbp,%rsi
   7e200:	mov    %rbp,%rdx
   7e203:	call   *0xcc1a7(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e209:	lea    0xc6358(%rip),%rdx        # 144568 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   7e210:	xor    %edi,%edi
   7e212:	xor    %esi,%esi
   7e214:	call   *0xcc076(%rip)        # 14a290 <_DYNAMIC+0x250>
   7e21a:	lea    0xc638f(%rip),%rcx        # 1445b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   7e221:	xor    %edi,%edi
   7e223:	mov    %r13,%rsi
   7e226:	mov    0x48(%rsp),%rdx
   7e22b:	call   *0xcc17f(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   7e231:	lea    0xc6390(%rip),%rcx        # 1445c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   7e238:	xor    %edi,%edi
   7e23a:	mov    %r13,%rsi
   7e23d:	mov    0x48(%rsp),%rdx
   7e242:	call   *0xcc168(%rip)        # 14a3b0 <_DYNAMIC+0x370>
