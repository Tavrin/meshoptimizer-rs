
<artifacts>/screen3/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000d5280 <meshoptimizer_rs::codec::vertex::decode_owned>:
   d5280:	push   %rbp
   d5281:	push   %r15
   d5283:	push   %r14
   d5285:	push   %r13
   d5287:	push   %r12
   d5289:	push   %rbx
   d528a:	sub    $0x5f8,%rsp
   d5291:	mov    %r8,%rbx
   d5294:	mov    %rcx,%r14
   d5297:	mov    %rdx,%r12
   d529a:	mov    %rsi,%r13
   d529d:	mov    %rdi,%rbp
   d52a0:	cmp    $0xc,%rdx
   d52a4:	jne    d52b3 <meshoptimizer_rs::codec::vertex::decode_owned+0x33>
   d52a6:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   d52ab:	cmp    $0x2,%al
   d52ad:	jae    d537e <meshoptimizer_rs::codec::vertex::decode_owned+0xfe>
   d52b3:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   d52b8:	cmp    $0x2,%al
   d52ba:	jb     d5320 <meshoptimizer_rs::codec::vertex::decode_owned+0xa0>
   d52bc:	cmp    $0x20,%r12
   d52c0:	lea    -0x5c927(%rip),%rax        # 789a0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   d52c7:	lea    -0x5de9e(%rip),%rcx        # 77430 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   d52ce:	cmove  %rax,%rcx
   d52d2:	lea    -0x56619(%rip),%rax        # 7ecc0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   d52d9:	lea    -0x57f90(%rip),%rdx        # 7d350 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   d52e0:	cmove  %rax,%rdx
   d52e4:	cmp    $0x4,%r12
   d52e8:	lea    -0x5b51f(%rip),%rax        # 79dd0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   d52ef:	cmovne %rcx,%rax
   d52f3:	lea    -0x54e5a(%rip),%r9        # 804a0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   d52fa:	cmovne %rdx,%r9
   d52fe:	cmp    $0x21,%r13
   d5302:	cmovae %rax,%r9
   d5306:	mov    %rbp,%rdi
   d5309:	mov    %r13,%rsi
   d530c:	mov    %r12,%rdx
   d530f:	mov    %r14,%rcx
   d5312:	mov    %rbx,%r8
   d5315:	call   *%r9
   d5318:	cmp    $0xfe,%al
   d531a:	jne    d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5320:	mov    $0x7,%al
   d5322:	test   %rbx,%rbx
   d5325:	je     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d532b:	movzbl (%r14),%edx
   d532f:	mov    %edx,%ecx
   d5331:	and    $0xf0,%cl
   d5334:	cmp    $0xa0,%cl
   d5337:	jne    d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d533d:	and    $0xf,%dl
   d5340:	mov    $0x5,%al
   d5342:	mov    %dl,0x27(%rsp)
   d5346:	cmp    $0x1,%dl
   d5349:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d534f:	cmpb   $0x0,0x27(%rsp)
   d5354:	je     d53b9 <meshoptimizer_rs::codec::vertex::decode_owned+0x139>
   d5356:	mov    %r12,%rcx
   d5359:	shr    $0x2,%rcx
   d535d:	add    %r12,%rcx
   d5360:	cmp    $0x19,%rcx
   d5364:	mov    $0x18,%r15d
   d536a:	cmovae %rcx,%r15
   d536e:	lea    0x1(%r15),%rdx
   d5372:	mov    $0x7,%al
   d5374:	cmp    %rdx,%rbx
   d5377:	jae    d53d9 <meshoptimizer_rs::codec::vertex::decode_owned+0x159>
   d5379:	jmp    d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d537e:	cmp    $0x21,%r13
   d5382:	lea    -0x520e9(%rip),%rax        # 832a0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, alloc::vec::Vec<u8>>>
   d5389:	lea    -0x58ae0(%rip),%r9        # 7c8b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, alloc::vec::Vec<u8>>>
   d5390:	cmovb  %rax,%r9
   d5394:	mov    $0xc,%edx
   d5399:	mov    %rbp,%rdi
   d539c:	mov    %r13,%rsi
   d539f:	mov    %r14,%rcx
   d53a2:	mov    %rbx,%r8
   d53a5:	add    $0x5f8,%rsp
   d53ac:	pop    %rbx
   d53ad:	pop    %r12
   d53af:	pop    %r13
   d53b1:	pop    %r14
   d53b3:	pop    %r15
   d53b5:	pop    %rbp
   d53b6:	jmp    *%r9
   d53b9:	cmp    $0x21,%r12
   d53bd:	mov    $0x20,%r15d
   d53c3:	cmovae %r12,%r15
   d53c7:	mov    %r12,%rcx
   d53ca:	lea    0x1(%r15),%rdx
   d53ce:	mov    $0x7,%al
   d53d0:	cmp    %rdx,%rbx
   d53d3:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d53d9:	mov    %rbx,%rsi
   d53dc:	sub    %rcx,%rsi
   d53df:	lea    (%rsi,%r12,1),%rax
   d53e3:	mov    %rbx,%rcx
   d53e6:	mov    %rax,0x70(%rsp)
   d53eb:	sub    %rax,%rcx
   d53ee:	mov    %rcx,0xa8(%rsp)
   d53f6:	jb     d5ec3 <meshoptimizer_rs::codec::vertex::decode_owned+0xc43>
   d53fc:	xorps  %xmm0,%xmm0
   d53ff:	movaps %xmm0,0x1e0(%rsp)
   d5407:	movaps %xmm0,0x1d0(%rsp)
   d540f:	movaps %xmm0,0x1c0(%rsp)
   d5417:	movaps %xmm0,0x1b0(%rsp)
   d541f:	movaps %xmm0,0x1a0(%rsp)
   d5427:	movaps %xmm0,0x190(%rsp)
   d542f:	movaps %xmm0,0x180(%rsp)
   d5437:	movaps %xmm0,0x170(%rsp)
   d543f:	movaps %xmm0,0x160(%rsp)
   d5447:	movaps %xmm0,0x150(%rsp)
   d544f:	movaps %xmm0,0x140(%rsp)
   d5457:	movaps %xmm0,0x130(%rsp)
   d545f:	movaps %xmm0,0x120(%rsp)
   d5467:	movaps %xmm0,0x110(%rsp)
   d546f:	movaps %xmm0,0x100(%rsp)
   d5477:	movaps %xmm0,0xf0(%rsp)
   d547f:	cmp    $0x101,%r12
   d5486:	jae    d5e7f <meshoptimizer_rs::codec::vertex::decode_owned+0xbff>
   d548c:	lea    (%rsi,%r12,1),%rax
   d5490:	cmp    %rsi,%rax
   d5493:	jb     d5e96 <meshoptimizer_rs::codec::vertex::decode_owned+0xc16>
   d5499:	add    %r14,%rsi
   d549c:	lea    0xf0(%rsp),%rdi
   d54a4:	mov    %r12,%rdx
   d54a7:	call   *0x53c6b(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d54ad:	test   %r12,%r12
   d54b0:	je     d5f1b <meshoptimizer_rs::codec::vertex::decode_owned+0xc9b>
   d54b6:	mov    %r15,0xd8(%rsp)
   d54be:	mov    $0x2000,%ax
   d54c2:	xor    %edx,%edx
   d54c4:	div    %r12w
   d54c8:	and    $0x3ff0,%eax
   d54cd:	cmp    $0x100,%eax
   d54d2:	mov    $0x100,%ecx
   d54d7:	cmovb  %eax,%ecx
   d54da:	mov    %rcx,0xe0(%rsp)
   d54e2:	lea    0x1f8(%rsp),%rdi
   d54ea:	mov    $0x400,%edx
   d54ef:	xor    %esi,%esi
   d54f1:	call   *0x53c31(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d54f7:	test   %r13,%r13
   d54fa:	je     d5e1b <meshoptimizer_rs::codec::vertex::decode_owned+0xb9b>
   d5500:	add    %r14,0x70(%rsp)
   d5505:	mov    %r12,%rax
   d5508:	shr    $0x2,%rax
   d550c:	mov    %rax,0xa0(%rsp)
   d5514:	lea    0x3(%r12),%rax
   d5519:	shr    $0x2,%rax
   d551d:	mov    %rax,0xe8(%rsp)
   d5525:	mov    $0x1,%r15d
   d552b:	xor    %eax,%eax
   d552d:	mov    %rbp,0x68(%rsp)
   d5532:	mov    %rbx,0x30(%rsp)
   d5537:	mov    %r14,0x38(%rsp)
   d553c:	mov    %r13,0xc0(%rsp)
   d5544:	mov    %r13,%rdx
   d5547:	mov    %rax,0x58(%rsp)
   d554c:	sub    %rax,%rdx
   d554f:	mov    0xe0(%rsp),%rax
   d5557:	cmp    %rax,%rdx
   d555a:	cmovae %rax,%rdx
   d555e:	mov    %rdx,0x28(%rsp)
   d5563:	cmpb   $0x0,0x27(%rsp)
   d5568:	je     d55ad <meshoptimizer_rs::codec::vertex::decode_owned+0x32d>
   d556a:	mov    %r15,%rcx
   d556d:	mov    $0x7,%al
   d556f:	add    0xa0(%rsp),%rcx
   d5577:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d557d:	cmp    %rbx,%rcx
   d5580:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5586:	add    0x38(%rsp),%r15
   d558b:	mov    0xa0(%rsp),%rax
   d5593:	mov    %rax,0x98(%rsp)
   d559b:	mov    %r15,0xb0(%rsp)
   d55a3:	mov    %rcx,%r15
   d55a6:	mov    0x68(%rsp),%rbp
   d55ab:	jmp    d55c6 <meshoptimizer_rs::codec::vertex::decode_owned+0x346>
   d55ad:	movq   $0x0,0x98(%rsp)
   d55b9:	mov    $0x1,%eax
   d55be:	mov    %rax,0xb0(%rsp)
   d55c6:	mov    0x58(%rsp),%rax
   d55cb:	mov    0x28(%rsp),%rcx
   d55d0:	lea    (%rcx,%rax,1),%r14
   d55d4:	mov    %r14,0xd0(%rsp)
   d55dc:	imul   %r12,%r14
   d55e0:	mov    0x10(%rbp),%r13
   d55e4:	mov    %rbp,%rax
   d55e7:	mov    %r14,%rbp
   d55ea:	mov    %r14,%rdx
   d55ed:	sub    %r13,%rbp
   d55f0:	jbe    d563a <meshoptimizer_rs::codec::vertex::decode_owned+0x3ba>
   d55f2:	mov    (%rax),%rax
   d55f5:	sub    %r13,%rax
   d55f8:	cmp    %rax,%rbp
   d55fb:	ja     d5df4 <meshoptimizer_rs::codec::vertex::decode_owned+0xb74>
   d5601:	mov    0x68(%rsp),%rax
   d5606:	mov    0x8(%rax),%rbx
   d560a:	lea    (%rbx,%r13,1),%rdi
   d560e:	cmp    $0x2,%rbp
   d5612:	jb     d562c <meshoptimizer_rs::codec::vertex::decode_owned+0x3ac>
   d5614:	lea    -0x1(%rbp),%rdx
   d5618:	xor    %esi,%esi
   d561a:	call   *0x53b08(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d5620:	add    %rbp,%r13
   d5623:	dec    %r13
   d5626:	add    %r13,%rbx
   d5629:	mov    %rbx,%rdi
   d562c:	movb   $0x0,(%rdi)
   d562f:	inc    %r13
   d5632:	mov    %r13,%rdx
   d5635:	mov    0x68(%rsp),%rax
   d563a:	mov    0x58(%rsp),%rcx
   d563f:	imul   %r12,%rcx
   d5643:	mov    %rdx,0x10(%rax)
   d5647:	mov    %rcx,0x58(%rsp)
   d564c:	cmp    %rcx,%r14
   d564f:	jb     d5eae <meshoptimizer_rs::codec::vertex::decode_owned+0xc2e>
   d5655:	cmp    %rdx,%r14
   d5658:	ja     d5eae <meshoptimizer_rs::codec::vertex::decode_owned+0xc2e>
   d565e:	mov    0x28(%rsp),%rdx
   d5663:	lea    0xf(%rdx),%ecx
   d5666:	and    $0x3f0,%ecx
   d566c:	mov    %rcx,0x50(%rsp)
   d5671:	mov    %rdx,%rcx
   d5674:	imul   %r12,%rcx
   d5678:	mov    %rcx,0xb8(%rsp)
   d5680:	mov    0x58(%rsp),%rcx
   d5685:	add    0x8(%rax),%rcx
   d5689:	mov    %rcx,0x58(%rsp)
   d568e:	lea    -0x1(%rdx),%rax
   d5692:	imul   %r12,%rax
   d5696:	mov    %rax,0xc8(%rsp)
   d569e:	add    $0x4,%rax
   d56a2:	mov    %rax,0x60(%rsp)
   d56a7:	lea    0x0(,%rdx,4),%rax
   d56af:	mov    %rax,0x78(%rsp)
   d56b4:	lea    0x1f8(%rsp,%rdx,1),%rax
   d56bc:	mov    %rax,0x90(%rsp)
   d56c4:	lea    0x1f8(%rsp,%rdx,2),%rax
   d56cc:	mov    %rax,0x88(%rsp)
   d56d4:	lea    (%rdx,%rdx,2),%rax
   d56d8:	add    %rsp,%rax
   d56db:	add    $0x1f8,%rax
   d56e1:	mov    %rax,0x80(%rsp)
   d56e9:	xor    %r13d,%r13d
   d56ec:	xor    %ebp,%ebp
   d56ee:	movzbl 0x27(%rsp),%r14d
   d56f4:	jmp    d5750 <meshoptimizer_rs::codec::vertex::decode_owned+0x4d0>
   d56f6:	lea    (%rsp,%rbp,1),%rcx
   d56fa:	add    $0xf0,%rcx
   d5701:	shr    $0x4,%al
   d5704:	neg    %al
   d5706:	and    $0x1f,%al
   d5708:	movzbl %al,%eax
   d570b:	mov    %rcx,(%rsp)
   d570f:	mov    %eax,0x8(%rsp)
   d5713:	movq   $0x0,0x10(%rsp)
   d571c:	lea    0x1f8(%rsp),%rdi
   d5724:	mov    0x78(%rsp),%rsi
   d5729:	mov    0x60(%rsp),%rcx
   d572e:	mov    0x28(%rsp),%r8
   d5733:	mov    %r12,%r9
   d5736:	call   72ef0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   d573b:	inc    %r13
   d573e:	add    $0x4,%rbp
   d5742:	cmp    %r13,0xe8(%rsp)
   d574a:	je     d5d9a <meshoptimizer_rs::codec::vertex::decode_owned+0xb1a>
   d5750:	test   %r14b,%r14b
   d5753:	je     d5772 <meshoptimizer_rs::codec::vertex::decode_owned+0x4f2>
   d5755:	cmp    0x98(%rsp),%r13
   d575d:	jae    d5eeb <meshoptimizer_rs::codec::vertex::decode_owned+0xc6b>
   d5763:	mov    0xb0(%rsp),%rax
   d576b:	movzbl (%rax,%r13,1),%ebx
   d5770:	jmp    d5774 <meshoptimizer_rs::codec::vertex::decode_owned+0x4f4>
   d5772:	xor    %ebx,%ebx
   d5774:	mov    %ebx,%eax
   d5776:	and    $0x3,%al
   d5778:	movzbl %al,%ecx
   d577b:	mov    0x50(%rsp),%rdx
   d5780:	cmp    %rdx,0x28(%rsp)
   d5785:	ja     d5bed <meshoptimizer_rs::codec::vertex::decode_owned+0x96d>
   d578b:	cmp    $0x2,%al
   d578d:	je     d57cc <meshoptimizer_rs::codec::vertex::decode_owned+0x54c>
   d578f:	cmp    $0x3,%ecx
   d5792:	jne    d57e6 <meshoptimizer_rs::codec::vertex::decode_owned+0x566>
   d5794:	mov    %r15,%r14
   d5797:	mov    $0x7,%al
   d5799:	add    0x28(%rsp),%r14
   d579e:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d57a4:	cmp    0x30(%rsp),%r14
   d57a9:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d57af:	add    0x38(%rsp),%r15
   d57b4:	lea    0x1f8(%rsp),%rdi
   d57bc:	mov    %r15,%rsi
   d57bf:	mov    0x28(%rsp),%rdx
   d57c4:	call   *0x5394e(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d57ca:	jmp    d5840 <meshoptimizer_rs::codec::vertex::decode_owned+0x5c0>
   d57cc:	lea    0x1f8(%rsp),%rdi
   d57d4:	xor    %esi,%esi
   d57d6:	mov    0x28(%rsp),%rdx
   d57db:	call   *0x53947(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d57e1:	mov    %r15,%r14
   d57e4:	jmp    d5840 <meshoptimizer_rs::codec::vertex::decode_owned+0x5c0>
   d57e6:	test   %al,%al
   d57e8:	lea    -0xc7bbf(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d57ef:	lea    -0xc7aa6(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d57f6:	cmove  %rcx,%rax
   d57fa:	test   %r14b,%r14b
   d57fd:	lea    -0xc7ed4(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5804:	cmove  %rcx,%rax
   d5808:	mov    %rax,(%rsp)
   d580c:	lea    0x40(%rsp),%rdi
   d5811:	mov    0x38(%rsp),%rsi
   d5816:	mov    0x30(%rsp),%rdx
   d581b:	mov    %r15,%rcx
   d581e:	lea    0x1f8(%rsp),%r8
   d5826:	mov    0x50(%rsp),%r9
   d582b:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5830:	cmpb   $0x0,0x40(%rsp)
   d5835:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d583b:	mov    0x48(%rsp),%r14
   d5840:	mov    %ebx,%eax
   d5842:	shr    $0x2,%al
   d5845:	and    $0x3,%al
   d5847:	cmp    $0x3,%al
   d5849:	je     d586d <meshoptimizer_rs::codec::vertex::decode_owned+0x5ed>
   d584b:	movzbl %al,%ecx
   d584e:	cmp    $0x2,%ecx
   d5851:	jne    d58a8 <meshoptimizer_rs::codec::vertex::decode_owned+0x628>
   d5853:	mov    0x90(%rsp),%rdi
   d585b:	xor    %esi,%esi
   d585d:	mov    0x28(%rsp),%rdx
   d5862:	call   *0x538c0(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d5868:	jmp    d5904 <meshoptimizer_rs::codec::vertex::decode_owned+0x684>
   d586d:	mov    %r14,%r15
   d5870:	mov    $0x7,%al
   d5872:	add    0x28(%rsp),%r15
   d5877:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d587d:	cmp    0x30(%rsp),%r15
   d5882:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5888:	add    0x38(%rsp),%r14
   d588d:	mov    0x90(%rsp),%rdi
   d5895:	mov    %r14,%rsi
   d5898:	mov    0x28(%rsp),%rdx
   d589d:	call   *0x53875(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d58a3:	mov    %r15,%r14
   d58a6:	jmp    d5904 <meshoptimizer_rs::codec::vertex::decode_owned+0x684>
   d58a8:	test   %al,%al
   d58aa:	lea    -0xc7c81(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d58b1:	lea    -0xc7b68(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d58b8:	cmove  %rcx,%rax
   d58bc:	cmpb   $0x0,0x27(%rsp)
   d58c1:	lea    -0xc7f98(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d58c8:	cmove  %rcx,%rax
   d58cc:	mov    %rax,(%rsp)
   d58d0:	lea    0x40(%rsp),%rdi
   d58d5:	mov    0x38(%rsp),%rsi
   d58da:	mov    0x30(%rsp),%rdx
   d58df:	mov    %r14,%rcx
   d58e2:	mov    0x90(%rsp),%r8
   d58ea:	mov    0x50(%rsp),%r9
   d58ef:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d58f4:	cmpb   $0x0,0x40(%rsp)
   d58f9:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d58ff:	mov    0x48(%rsp),%r14
   d5904:	mov    %ebx,%eax
   d5906:	shr    $0x4,%al
   d5909:	and    $0x3,%al
   d590b:	cmp    $0x3,%al
   d590d:	je     d593d <meshoptimizer_rs::codec::vertex::decode_owned+0x6bd>
   d590f:	movzbl %al,%ecx
   d5912:	cmp    $0x2,%ecx
   d5915:	jne    d59c2 <meshoptimizer_rs::codec::vertex::decode_owned+0x742>
   d591b:	mov    0x88(%rsp),%rdi
   d5923:	xor    %esi,%esi
   d5925:	mov    0x28(%rsp),%rdx
   d592a:	call   *0x537f8(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d5930:	shr    $0x6,%bl
   d5933:	cmp    $0x3,%bl
   d5936:	jne    d5982 <meshoptimizer_rs::codec::vertex::decode_owned+0x702>
   d5938:	jmp    d5a2a <meshoptimizer_rs::codec::vertex::decode_owned+0x7aa>
   d593d:	mov    %r14,%r15
   d5940:	mov    $0x7,%al
   d5942:	add    0x28(%rsp),%r15
   d5947:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d594d:	cmp    0x30(%rsp),%r15
   d5952:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5958:	add    0x38(%rsp),%r14
   d595d:	mov    0x88(%rsp),%rdi
   d5965:	mov    %r14,%rsi
   d5968:	mov    0x28(%rsp),%rdx
   d596d:	call   *0x537a5(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d5973:	mov    %r15,%r14
   d5976:	shr    $0x6,%bl
   d5979:	cmp    $0x3,%bl
   d597c:	je     d5a2a <meshoptimizer_rs::codec::vertex::decode_owned+0x7aa>
   d5982:	movzbl %bl,%eax
   d5985:	cmp    $0x2,%eax
   d5988:	jne    d5a7a <meshoptimizer_rs::codec::vertex::decode_owned+0x7fa>
   d598e:	mov    0x80(%rsp),%rdi
   d5996:	xor    %esi,%esi
   d5998:	mov    0x28(%rsp),%rdx
   d599d:	call   *0x53785(%rip)        # 129128 <memset@GLIBC_2.2.5>
   d59a3:	mov    %r14,%r15
   d59a6:	movzbl 0x27(%rsp),%r14d
   d59ac:	test   %r14b,%r14b
   d59af:	mov    0xb8(%rsp),%rcx
   d59b7:	jne    d5ae9 <meshoptimizer_rs::codec::vertex::decode_owned+0x869>
   d59bd:	jmp    d5a73 <meshoptimizer_rs::codec::vertex::decode_owned+0x7f3>
   d59c2:	test   %al,%al
   d59c4:	lea    -0xc7d9b(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d59cb:	lea    -0xc7c82(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d59d2:	cmove  %rcx,%rax
   d59d6:	cmpb   $0x0,0x27(%rsp)
   d59db:	lea    -0xc80b2(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d59e2:	cmove  %rcx,%rax
   d59e6:	mov    %rax,(%rsp)
   d59ea:	lea    0x40(%rsp),%rdi
   d59ef:	mov    0x38(%rsp),%rsi
   d59f4:	mov    0x30(%rsp),%rdx
   d59f9:	mov    %r14,%rcx
   d59fc:	mov    0x88(%rsp),%r8
   d5a04:	mov    0x50(%rsp),%r9
   d5a09:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5a0e:	cmpb   $0x0,0x40(%rsp)
   d5a13:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d5a19:	mov    0x48(%rsp),%r14
   d5a1e:	shr    $0x6,%bl
   d5a21:	cmp    $0x3,%bl
   d5a24:	jne    d5982 <meshoptimizer_rs::codec::vertex::decode_owned+0x702>
   d5a2a:	mov    %r14,%r15
   d5a2d:	mov    $0x7,%al
   d5a2f:	add    0x28(%rsp),%r15
   d5a34:	jb     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5a3a:	cmp    0x30(%rsp),%r15
   d5a3f:	ja     d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5a45:	add    0x38(%rsp),%r14
   d5a4a:	mov    0x80(%rsp),%rdi
   d5a52:	mov    %r14,%rsi
   d5a55:	mov    0x28(%rsp),%rdx
   d5a5a:	call   *0x536b8(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d5a60:	movzbl 0x27(%rsp),%r14d
   d5a66:	test   %r14b,%r14b
   d5a69:	mov    0xb8(%rsp),%rcx
   d5a71:	jne    d5ae9 <meshoptimizer_rs::codec::vertex::decode_owned+0x869>
   d5a73:	xor    %eax,%eax
   d5a75:	jmp    d5b01 <meshoptimizer_rs::codec::vertex::decode_owned+0x881>
   d5a7a:	test   %bl,%bl
   d5a7c:	lea    -0xc7e53(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d5a83:	lea    -0xc7d3a(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d5a8a:	cmove  %rcx,%rax
   d5a8e:	cmpb   $0x0,0x27(%rsp)
   d5a93:	lea    -0xc816a(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5a9a:	cmove  %rcx,%rax
   d5a9e:	mov    %rax,(%rsp)
   d5aa2:	lea    0x40(%rsp),%rdi
   d5aa7:	mov    0x38(%rsp),%rsi
   d5aac:	mov    0x30(%rsp),%rdx
   d5ab1:	mov    %r14,%rcx
   d5ab4:	mov    0x80(%rsp),%r8
   d5abc:	mov    0x50(%rsp),%r9
   d5ac1:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5ac6:	cmpb   $0x0,0x40(%rsp)
   d5acb:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d5ad1:	mov    0x48(%rsp),%r15
   d5ad6:	movzbl 0x27(%rsp),%r14d
   d5adc:	test   %r14b,%r14b
   d5adf:	mov    0xb8(%rsp),%rcx
   d5ae7:	je     d5a73 <meshoptimizer_rs::codec::vertex::decode_owned+0x7f3>
   d5ae9:	cmp    0xa8(%rsp),%r13
   d5af1:	jae    d5f03 <meshoptimizer_rs::codec::vertex::decode_owned+0xc83>
   d5af7:	mov    0x70(%rsp),%rax
   d5afc:	movzbl (%rax,%r13,1),%eax
   d5b01:	mov    %rcx,%rdx
   d5b04:	sub    %rbp,%rdx
   d5b07:	jb     d5e69 <meshoptimizer_rs::codec::vertex::decode_owned+0xbe9>
   d5b0d:	cmp    %rdx,0x60(%rsp)
   d5b12:	ja     d5e55 <meshoptimizer_rs::codec::vertex::decode_owned+0xbd5>
   d5b18:	mov    0x58(%rsp),%rcx
   d5b1d:	lea    (%rcx,%rbp,1),%rdx
   d5b21:	mov    %eax,%ecx
   d5b23:	and    $0x3,%cl
   d5b26:	movzbl %cl,%ecx
   d5b29:	lea    -0xc5064(%rip),%rsi        # 10acc <libm::math::generic::sqrt::RSQRT_TAB+0x6e6>
   d5b30:	movslq (%rsi,%rcx,4),%rcx
   d5b34:	add    %rsi,%rcx
   d5b37:	jmp    *%rcx
   d5b39:	mov    %rdx,%r14
   d5b3c:	lea    (%rsp,%rbp,1),%rbx
   d5b40:	add    $0xf0,%rbx
   d5b47:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   d5b4c:	test   %al,%al
   d5b4e:	je     d5bb3 <meshoptimizer_rs::codec::vertex::decode_owned+0x933>
   d5b50:	mov    %rbx,(%rsp)
   d5b54:	lea    0x1f8(%rsp),%rdi
   d5b5c:	mov    0x78(%rsp),%rsi
   d5b61:	mov    %r14,%rdx
   d5b64:	mov    0x60(%rsp),%rcx
   d5b69:	mov    0x28(%rsp),%r8
   d5b6e:	mov    %r12,%r9
   d5b71:	call   74fb0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   d5b76:	jmp    d5be2 <meshoptimizer_rs::codec::vertex::decode_owned+0x962>
   d5b78:	lea    (%rsp,%rbp,1),%rax
   d5b7c:	add    $0xf0,%rax
   d5b82:	mov    %rax,(%rsp)
   d5b86:	movq   $0x0,0x8(%rsp)
   d5b8f:	lea    0x1f8(%rsp),%rdi
   d5b97:	mov    0x78(%rsp),%rsi
   d5b9c:	mov    0x60(%rsp),%rcx
   d5ba1:	mov    0x28(%rsp),%r8
   d5ba6:	mov    %r12,%r9
   d5ba9:	call   72d30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   d5bae:	jmp    d573b <meshoptimizer_rs::codec::vertex::decode_owned+0x4bb>
   d5bb3:	mov    %rbx,(%rsp)
   d5bb7:	movq   $0x0,0x8(%rsp)
   d5bc0:	lea    0x1f8(%rsp),%rdi
   d5bc8:	mov    0x78(%rsp),%rsi
   d5bcd:	mov    %r14,%rdx
   d5bd0:	mov    0x60(%rsp),%rcx
   d5bd5:	mov    0x28(%rsp),%r8
   d5bda:	mov    %r12,%r9
   d5bdd:	call   72b40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   d5be2:	movzbl 0x27(%rsp),%r14d
   d5be8:	jmp    d573b <meshoptimizer_rs::codec::vertex::decode_owned+0x4bb>
   d5bed:	cmp    $0x2,%al
   d5bef:	je     d5f41 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   d5bf5:	cmp    $0x3,%ecx
   d5bf8:	je     d5f28 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   d5bfe:	test   %al,%al
   d5c00:	lea    -0xc7fd7(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d5c07:	lea    -0xc7ebe(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d5c0e:	cmove  %rcx,%rax
   d5c12:	test   %r14b,%r14b
   d5c15:	lea    -0xc82ec(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5c1c:	cmove  %rcx,%rax
   d5c20:	mov    %rax,(%rsp)
   d5c24:	lea    0x40(%rsp),%rdi
   d5c29:	mov    0x38(%rsp),%rsi
   d5c2e:	mov    0x30(%rsp),%rdx
   d5c33:	mov    %r15,%rcx
   d5c36:	lea    0x1f8(%rsp),%r8
   d5c3e:	mov    0x50(%rsp),%r9
   d5c43:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5c48:	cmpb   $0x0,0x40(%rsp)
   d5c4d:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d5c53:	mov    %ebx,%eax
   d5c55:	shr    $0x2,%al
   d5c58:	and    $0x3,%al
   d5c5a:	cmp    $0x2,%al
   d5c5c:	movzbl 0x27(%rsp),%edx
   d5c61:	je     d5f41 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   d5c67:	movzbl %al,%ecx
   d5c6a:	cmp    $0x3,%ecx
   d5c6d:	je     d5f28 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   d5c73:	test   %al,%al
   d5c75:	lea    -0xc804c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d5c7c:	lea    -0xc7f33(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d5c83:	cmove  %rcx,%rax
   d5c87:	mov    0x48(%rsp),%rcx
   d5c8c:	test   %dl,%dl
   d5c8e:	lea    -0xc8365(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5c95:	cmove  %rdx,%rax
   d5c99:	mov    %rax,(%rsp)
   d5c9d:	lea    0x40(%rsp),%rdi
   d5ca2:	mov    0x38(%rsp),%rsi
   d5ca7:	mov    0x30(%rsp),%rdx
   d5cac:	mov    0x90(%rsp),%r8
   d5cb4:	mov    0x50(%rsp),%r9
   d5cb9:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5cbe:	cmpb   $0x0,0x40(%rsp)
   d5cc3:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d5cc9:	mov    %ebx,%eax
   d5ccb:	shr    $0x4,%al
   d5cce:	and    $0x3,%al
   d5cd0:	cmp    $0x2,%al
   d5cd2:	movzbl 0x27(%rsp),%edx
   d5cd7:	je     d5f41 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   d5cdd:	movzbl %al,%ecx
   d5ce0:	cmp    $0x3,%ecx
   d5ce3:	je     d5f28 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   d5ce9:	test   %al,%al
   d5ceb:	lea    -0xc80c2(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d5cf2:	lea    -0xc7fa9(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d5cf9:	cmove  %rcx,%rax
   d5cfd:	mov    0x48(%rsp),%rcx
   d5d02:	test   %dl,%dl
   d5d04:	lea    -0xc83db(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5d0b:	cmove  %rdx,%rax
   d5d0f:	mov    %rax,(%rsp)
   d5d13:	lea    0x40(%rsp),%rdi
   d5d18:	mov    0x38(%rsp),%rsi
   d5d1d:	mov    0x30(%rsp),%rdx
   d5d22:	mov    0x88(%rsp),%r8
   d5d2a:	mov    0x50(%rsp),%r9
   d5d2f:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   d5d34:	cmpb   $0x0,0x40(%rsp)
   d5d39:	jne    d5e3a <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   d5d3f:	shr    $0x6,%bl
   d5d42:	cmp    $0x2,%bl
   d5d45:	movzbl 0x27(%rsp),%edx
   d5d4a:	je     d5f41 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   d5d50:	movzbl %bl,%eax
   d5d53:	cmp    $0x3,%eax
   d5d56:	je     d5f28 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   d5d5c:	test   %bl,%bl
   d5d5e:	lea    -0xc8135(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   d5d65:	lea    -0xc801c(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   d5d6c:	cmove  %rcx,%rax
   d5d70:	mov    0x48(%rsp),%rcx
   d5d75:	test   %dl,%dl
   d5d77:	lea    -0xc844e(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   d5d7e:	cmove  %rdx,%rax
   d5d82:	mov    %rax,(%rsp)
   d5d86:	lea    0x40(%rsp),%rdi
   d5d8b:	mov    0x38(%rsp),%rsi
   d5d90:	mov    0x30(%rsp),%rdx
   d5d95:	jmp    d5ab4 <meshoptimizer_rs::codec::vertex::decode_owned+0x834>
   d5d9a:	mov    0xb8(%rsp),%rdx
   d5da2:	mov    0xc8(%rsp),%rdi
   d5daa:	cmp    %rdx,%rdi
   d5dad:	ja     d5edb <meshoptimizer_rs::codec::vertex::decode_owned+0xc5b>
   d5db3:	mov    0x58(%rsp),%rsi
   d5db8:	add    %rdi,%rsi
   d5dbb:	lea    0xf0(%rsp),%rdi
   d5dc3:	mov    %r12,%rdx
   d5dc6:	call   *0x5334c(%rip)        # 129118 <memcpy@GLIBC_2.14>
   d5dcc:	mov    0xd0(%rsp),%rcx
   d5dd4:	mov    %rcx,%rax
   d5dd7:	mov    0xc0(%rsp),%r13
   d5ddf:	cmp    %r13,%rcx
   d5de2:	mov    0x30(%rsp),%rbx
   d5de7:	mov    0x68(%rsp),%rbp
   d5dec:	jb     d5544 <meshoptimizer_rs::codec::vertex::decode_owned+0x2c4>
   d5df2:	jmp    d5e21 <meshoptimizer_rs::codec::vertex::decode_owned+0xba1>
   d5df4:	mov    $0x1,%ecx
   d5df9:	mov    $0x1,%r8d
   d5dff:	mov    0x68(%rsp),%rbx
   d5e04:	mov    %rbx,%rdi
   d5e07:	mov    %r13,%rsi
   d5e0a:	mov    %rbp,%rdx
   d5e0d:	call   83ee0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   d5e12:	mov    0x10(%rbx),%r13
   d5e16:	jmp    d5601 <meshoptimizer_rs::codec::vertex::decode_owned+0x381>
   d5e1b:	mov    $0x1,%r15d
   d5e21:	sub    %r15,%rbx
   d5e24:	setae  %cl
   d5e27:	cmp    0xd8(%rsp),%rbx
   d5e2f:	sete   %al
   d5e32:	and    %cl,%al
   d5e34:	neg    %al
   d5e36:	or     $0x7,%al
   d5e38:	jmp    d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5e3a:	movzbl 0x41(%rsp),%eax
   d5e3f:	add    $0x5f8,%rsp
   d5e46:	pop    %rbx
   d5e47:	pop    %r12
   d5e49:	pop    %r13
   d5e4b:	pop    %r14
   d5e4d:	pop    %r15
   d5e4f:	pop    %rbp
   d5e50:	ret
   d5e51:	mov    $0x7,%al
   d5e53:	jmp    d5e3f <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   d5e55:	lea    0x4df1c(%rip),%rcx        # 123d78 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   d5e5c:	xor    %edi,%edi
   d5e5e:	mov    0x60(%rsp),%rsi
   d5e63:	call   *0x53407(%rip)        # 129270 <_DYNAMIC+0x370>
   d5e69:	mov    %rcx,%rdx
   d5e6c:	lea    0x4df1d(%rip),%rcx        # 123d90 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   d5e73:	mov    %rbp,%rdi
   d5e76:	mov    %rdx,%rsi
   d5e79:	call   *0x533f1(%rip)        # 129270 <_DYNAMIC+0x370>
   d5e7f:	lea    0x4df6a(%rip),%rcx        # 123df0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   d5e86:	mov    $0x100,%edx
   d5e8b:	xor    %edi,%edi
   d5e8d:	mov    %r12,%rsi
   d5e90:	call   *0x533da(%rip)        # 129270 <_DYNAMIC+0x370>
   d5e96:	lea    0x4df3b(%rip),%rcx        # 123dd8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   d5e9d:	mov    %rsi,%rdi
   d5ea0:	mov    0x70(%rsp),%rsi
   d5ea5:	mov    %rbx,%rdx
   d5ea8:	call   *0x533c2(%rip)        # 129270 <_DYNAMIC+0x370>
   d5eae:	lea    0x5180b(%rip),%rcx        # 1276c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5b18>
   d5eb5:	mov    0x58(%rsp),%rdi
   d5eba:	mov    %r14,%rsi
   d5ebd:	call   *0x533ad(%rip)        # 129270 <_DYNAMIC+0x370>
   d5ec3:	lea    0x4df3e(%rip),%rcx        # 123e08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   d5eca:	mov    0x70(%rsp),%rdi
   d5ecf:	mov    %rbx,%rsi
   d5ed2:	mov    %rbx,%rdx
   d5ed5:	call   *0x53395(%rip)        # 129270 <_DYNAMIC+0x370>
   d5edb:	lea    0x4de4e(%rip),%rcx        # 123d30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   d5ee2:	mov    %rdx,%rsi
   d5ee5:	call   *0x53385(%rip)        # 129270 <_DYNAMIC+0x370>
   d5eeb:	lea    0x4de56(%rip),%rdx        # 123d48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   d5ef2:	mov    %r13,%rdi
   d5ef5:	mov    0x98(%rsp),%rsi
   d5efd:	call   *0x5324d(%rip)        # 129150 <_DYNAMIC+0x250>
   d5f03:	lea    0x4de56(%rip),%rdx        # 123d60 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   d5f0a:	mov    %r13,%rdi
   d5f0d:	mov    0xa8(%rsp),%rsi
   d5f15:	call   *0x53235(%rip)        # 129150 <_DYNAMIC+0x250>
   d5f1b:	lea    0x4ddf6(%rip),%rdi        # 123d18 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   d5f22:	call   *0x53410(%rip)        # 129338 <_DYNAMIC+0x438>
   d5f28:	lea    0x4de79(%rip),%rcx        # 123da8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   d5f2f:	xor    %edi,%edi
   d5f31:	mov    0x28(%rsp),%rsi
   d5f36:	mov    0x50(%rsp),%rdx
   d5f3b:	call   *0x5332f(%rip)        # 129270 <_DYNAMIC+0x370>
   d5f41:	lea    0x4de78(%rip),%rcx        # 123dc0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   d5f48:	xor    %edi,%edi
   d5f4a:	mov    0x28(%rsp),%rsi
   d5f4f:	mov    0x50(%rsp),%rdx
   d5f54:	call   *0x53316(%rip)        # 129270 <_DYNAMIC+0x370>
