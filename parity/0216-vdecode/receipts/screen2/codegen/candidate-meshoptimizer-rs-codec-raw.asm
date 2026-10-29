
<artifacts>/screen2/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000ae0b0 <meshoptimizer_rs::codec::raw>:
   ae0b0:	push   %rbp
   ae0b1:	push   %r15
   ae0b3:	push   %r14
   ae0b5:	push   %r13
   ae0b7:	push   %r12
   ae0b9:	push   %rbx
   ae0ba:	sub    $0x608,%rsp
   ae0c1:	mov    %r8,%rbp
   ae0c4:	mov    %rcx,%r13
   ae0c7:	mov    %rdx,%r15
   ae0ca:	mov    %rsi,%r12
   ae0cd:	mov    0x640(%rsp),%rbx
   ae0d5:	test   %dil,%dil
   ae0d8:	je     ae249 <meshoptimizer_rs::codec::raw+0x199>
   ae0de:	movzbl %dil,%eax
   ae0e2:	cmp    $0x1,%eax
   ae0e5:	jne    ae2ae <meshoptimizer_rs::codec::raw+0x1fe>
   ae0eb:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   ae0f5:	mov    %r13,%rax
   ae0f8:	mul    %rdi
   ae0fb:	shr    $1,%rdx
   ae0fe:	lea    (%rdx,%rdx,2),%rcx
   ae102:	mov    %r13,%rax
   ae105:	sub    %rcx,%rax
   ae108:	cmp    $0x2,%rbp
   ae10c:	jne    ae31d <meshoptimizer_rs::codec::raw+0x26d>
   ae112:	test   %rax,%rax
   ae115:	jne    ae326 <meshoptimizer_rs::codec::raw+0x276>
   ae11b:	mov    %r13,%rax
   ae11e:	mul    %rdi
   ae121:	shr    $1,%rdx
   ae124:	lea    0x11(%rdx),%rcx
   ae128:	mov    $0x7,%al
   ae12a:	cmp    %rcx,%rbx
   ae12d:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae133:	movzbl (%r9),%ecx
   ae137:	mov    %ecx,%esi
   ae139:	and    $0xf0,%sil
   ae13d:	cmp    $0xe0,%sil
   ae141:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae147:	and    $0xf,%cl
   ae14a:	mov    $0x5,%al
   ae14c:	cmp    $0x1,%cl
   ae14f:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae155:	mov    %r9,%r10
   ae158:	lea    0x1(%rdx),%rsi
   ae15c:	pcmpeqd %xmm0,%xmm0
   ae160:	movdqa %xmm0,0x270(%rsp)
   ae169:	movdqa %xmm0,0x260(%rsp)
   ae172:	movdqa %xmm0,0x250(%rsp)
   ae17b:	movdqa %xmm0,0x240(%rsp)
   ae184:	movdqa %xmm0,0x230(%rsp)
   ae18d:	movdqa %xmm0,0x220(%rsp)
   ae196:	movdqa %xmm0,0x210(%rsp)
   ae19f:	movdqa %xmm0,0x200(%rsp)
   ae1a8:	movdqa %xmm0,0x130(%rsp)
   ae1b1:	movdqa %xmm0,0x120(%rsp)
   ae1ba:	movdqa %xmm0,0x110(%rsp)
   ae1c3:	movdqa %xmm0,0x100(%rsp)
   ae1cc:	xor    %r8d,%r8d
   ae1cf:	cmp    $0x1,%cl
   ae1d2:	setne  %r9b
   ae1d6:	lea    -0x10(%rbx),%rax
   ae1da:	mov    %rax,0x28(%rsp)
   ae1df:	mov    %r15,%rax
   ae1e2:	mov    %rdx,%rcx
   ae1e5:	mul    %rdi
   ae1e8:	shr    $0x2,%rdx
   ae1ec:	cmp    %rcx,%rdx
   ae1ef:	cmovae %rcx,%rdx
   ae1f3:	mov    %rdx,0x40(%rsp)
   ae1f8:	test   %rdx,%rdx
   ae1fb:	je     afbe8 <meshoptimizer_rs::codec::raw+0x1b38>
   ae201:	mov    %r9b,%r8b
   ae204:	lea    0xd(,%r8,2),%rax
   ae20c:	mov    %rax,0x58(%rsp)
   ae211:	mov    %rbx,0x38(%rsp)
   ae216:	mov    %r10,0x30(%rsp)
   ae21b:	lea    (%r10,%rbx,1),%rax
   ae21f:	add    $0xfffffffffffffff0,%rax
   ae223:	mov    %rax,0x60(%rsp)
   ae228:	add    $0x4,%r12
   ae22c:	xor    %r8d,%r8d
   ae22f:	xor    %r10d,%r10d
   ae232:	movl   $0x0,0x20(%rsp)
   ae23a:	xor    %r11d,%r11d
   ae23d:	xor    %edi,%edi
   ae23f:	mov    0x28(%rsp),%r9
   ae244:	jmp    ae72e <meshoptimizer_rs::codec::raw+0x67e>
   ae249:	mov    %r9,%r14
   ae24c:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   ae251:	cmp    $0x2,%al
   ae253:	jae    ae32d <meshoptimizer_rs::codec::raw+0x27d>
   ae259:	mov    $0x7,%al
   ae25b:	test   %rbx,%rbx
   ae25e:	je     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae264:	mov    %r14,%rdi
   ae267:	movzbl (%r14),%esi
   ae26b:	mov    %esi,%ecx
   ae26d:	and    $0xf0,%cl
   ae270:	cmp    $0xa0,%cl
   ae273:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae279:	and    $0xf,%sil
   ae27d:	mov    $0x5,%al
   ae27f:	cmp    $0x1,%sil
   ae283:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae289:	test   %sil,%sil
   ae28c:	je     af22f <meshoptimizer_rs::codec::raw+0x117f>
   ae292:	mov    %rbp,%rcx
   ae295:	shr    $0x2,%rcx
   ae299:	add    %rbp,%rcx
   ae29c:	cmp    $0x19,%rcx
   ae2a0:	mov    $0x18,%eax
   ae2a5:	cmovae %rcx,%rax
   ae2a9:	jmp    af23f <meshoptimizer_rs::codec::raw+0x118f>
   ae2ae:	add    $0x5,%r13
   ae2b2:	mov    $0x7,%al
   ae2b4:	cmp    $0x2,%rbp
   ae2b8:	jne    ae39d <meshoptimizer_rs::codec::raw+0x2ed>
   ae2be:	cmp    %r13,%rbx
   ae2c1:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae2c7:	test   %rbx,%rbx
   ae2ca:	je     afcc1 <meshoptimizer_rs::codec::raw+0x1c11>
   ae2d0:	movzbl (%r9),%ecx
   ae2d4:	mov    %ecx,%edx
   ae2d6:	and    $0xf0,%dl
   ae2d9:	cmp    $0xd0,%dl
   ae2dc:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae2e2:	mov    $0x5,%al
   ae2e4:	test   $0xe,%cl
   ae2e7:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae2ed:	dec    %rbx
   ae2f0:	movq   $0x0,0x200(%rsp)
   ae2fc:	movabs $0x7ffffffffffffffc,%rax
   ae306:	or     $0x2,%rax
   ae30a:	and    %rax,%r15
   ae30d:	je     ae688 <meshoptimizer_rs::codec::raw+0x5d8>
   ae313:	inc    %r9
   ae316:	xor    %eax,%eax
   ae318:	jmp    ae432 <meshoptimizer_rs::codec::raw+0x382>
   ae31d:	test   %rax,%rax
   ae320:	je     ae563 <meshoptimizer_rs::codec::raw+0x4b3>
   ae326:	xor    %eax,%eax
   ae328:	jmp    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae32d:	cmp    $0x20,%rbp
   ae331:	lea    -0x34218(%rip),%rax        # 7a120 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   ae338:	lea    -0x356ff(%rip),%rcx        # 78c40 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   ae33f:	cmove  %rax,%rcx
   ae343:	lea    -0x2dd3a(%rip),%rax        # 80610 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   ae34a:	lea    -0x2f5e1(%rip),%rdx        # 7ed70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   ae351:	cmove  %rax,%rdx
   ae355:	cmp    $0x4,%rbp
   ae359:	lea    -0x31bb0(%rip),%rax        # 7c7b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   ae360:	cmovne %rcx,%rax
   ae364:	lea    -0x2b2bb(%rip),%r11        # 830b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   ae36b:	cmovne %rdx,%r11
   ae36f:	cmp    $0x21,%r13
   ae373:	cmovae %rax,%r11
   ae377:	mov    %r12,%rdi
   ae37a:	mov    %r15,%rsi
   ae37d:	mov    %r13,%rdx
   ae380:	mov    %rbp,%rcx
   ae383:	mov    %r14,%r8
   ae386:	mov    %rbx,%r9
   ae389:	add    $0x608,%rsp
   ae390:	pop    %rbx
   ae391:	pop    %r12
   ae393:	pop    %r13
   ae395:	pop    %r14
   ae397:	pop    %r15
   ae399:	pop    %rbp
   ae39a:	jmp    *%r11
   ae39d:	cmp    %r13,%rbx
   ae3a0:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae3a6:	test   %rbx,%rbx
   ae3a9:	je     afcc1 <meshoptimizer_rs::codec::raw+0x1c11>
   ae3af:	movzbl (%r9),%ecx
   ae3b3:	mov    %ecx,%edx
   ae3b5:	and    $0xf0,%dl
   ae3b8:	cmp    $0xd0,%dl
   ae3bb:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae3c1:	mov    $0x5,%al
   ae3c3:	test   $0xe,%cl
   ae3c6:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae3cc:	dec    %rbx
   ae3cf:	movq   $0x0,0x200(%rsp)
   ae3db:	movabs $0x7ffffffffffffffc,%rax
   ae3e5:	and    %rax,%r15
   ae3e8:	je     ae688 <meshoptimizer_rs::codec::raw+0x5d8>
   ae3ee:	inc    %r9
   ae3f1:	xor    %eax,%eax
   ae3f3:	jmp    ae4e7 <meshoptimizer_rs::codec::raw+0x437>
   ae3f8:	dec    %rbx
   ae3fb:	inc    %r9
   ae3fe:	mov    %edx,%ecx
   ae400:	mov    %ecx,%edx
   ae402:	shr    $0x2,%edx
   ae405:	mov    %ecx,%esi
   ae407:	shl    $0x1e,%esi
   ae40a:	sar    $0x1f,%esi
   ae40d:	xor    %edx,%esi
   ae40f:	and    $0x1,%ecx
   ae412:	add    0x200(%rsp,%rcx,4),%esi
   ae419:	mov    %esi,0x200(%rsp,%rcx,4)
   ae420:	mov    %si,(%r12,%rax,1)
   ae425:	add    $0x2,%rax
   ae429:	cmp    %rax,%r15
   ae42c:	je     ae688 <meshoptimizer_rs::codec::raw+0x5d8>
   ae432:	cmp    $0x5,%rbx
   ae436:	jb     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   ae43c:	movzbl (%r9),%edx
   ae440:	test   %dl,%dl
   ae442:	jns    ae3f8 <meshoptimizer_rs::codec::raw+0x348>
   ae444:	and    $0x7f,%edx
   ae447:	movzbl 0x1(%r9),%ecx
   ae44c:	mov    %ecx,%esi
   ae44e:	and    $0x7f,%ecx
   ae451:	shl    $0x7,%ecx
   ae454:	or     %edx,%ecx
   ae456:	mov    $0x2,%edx
   ae45b:	test   %sil,%sil
   ae45e:	jns    ae4a3 <meshoptimizer_rs::codec::raw+0x3f3>
   ae460:	movzbl 0x2(%r9),%edx
   ae465:	mov    %edx,%esi
   ae467:	and    $0x7f,%edx
   ae46a:	shl    $0xe,%edx
   ae46d:	or     %edx,%ecx
   ae46f:	mov    $0x3,%edx
   ae474:	test   %sil,%sil
   ae477:	jns    ae4a3 <meshoptimizer_rs::codec::raw+0x3f3>
   ae479:	movzbl 0x3(%r9),%edx
   ae47e:	mov    %edx,%esi
   ae480:	and    $0x7f,%edx
   ae483:	shl    $0x15,%edx
   ae486:	or     %edx,%ecx
   ae488:	mov    $0x4,%edx
   ae48d:	test   %sil,%sil
   ae490:	jns    ae4a3 <meshoptimizer_rs::codec::raw+0x3f3>
   ae492:	movzbl 0x4(%r9),%esi
   ae497:	shl    $0x1c,%esi
   ae49a:	or     %ecx,%esi
   ae49c:	mov    $0x5,%edx
   ae4a1:	mov    %esi,%ecx
   ae4a3:	sub    %rdx,%rbx
   ae4a6:	add    %rdx,%r9
   ae4a9:	jmp    ae400 <meshoptimizer_rs::codec::raw+0x350>
   ae4ae:	dec    %rbx
   ae4b1:	inc    %r9
   ae4b4:	mov    %edx,%ecx
   ae4b6:	mov    %ecx,%edx
   ae4b8:	shr    $0x2,%edx
   ae4bb:	mov    %ecx,%esi
   ae4bd:	shl    $0x1e,%esi
   ae4c0:	sar    $0x1f,%esi
   ae4c3:	xor    %edx,%esi
   ae4c5:	and    $0x1,%ecx
   ae4c8:	add    0x200(%rsp,%rcx,4),%esi
   ae4cf:	mov    %esi,0x200(%rsp,%rcx,4)
   ae4d6:	mov    %esi,(%r12,%rax,1)
   ae4da:	add    $0x4,%rax
   ae4de:	cmp    %rax,%r15
   ae4e1:	je     ae688 <meshoptimizer_rs::codec::raw+0x5d8>
   ae4e7:	cmp    $0x5,%rbx
   ae4eb:	jb     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   ae4f1:	movzbl (%r9),%edx
   ae4f5:	test   %dl,%dl
   ae4f7:	jns    ae4ae <meshoptimizer_rs::codec::raw+0x3fe>
   ae4f9:	and    $0x7f,%edx
   ae4fc:	movzbl 0x1(%r9),%ecx
   ae501:	mov    %ecx,%esi
   ae503:	and    $0x7f,%ecx
   ae506:	shl    $0x7,%ecx
   ae509:	or     %edx,%ecx
   ae50b:	mov    $0x2,%edx
   ae510:	test   %sil,%sil
   ae513:	jns    ae558 <meshoptimizer_rs::codec::raw+0x4a8>
   ae515:	movzbl 0x2(%r9),%edx
   ae51a:	mov    %edx,%esi
   ae51c:	and    $0x7f,%edx
   ae51f:	shl    $0xe,%edx
   ae522:	or     %edx,%ecx
   ae524:	mov    $0x3,%edx
   ae529:	test   %sil,%sil
   ae52c:	jns    ae558 <meshoptimizer_rs::codec::raw+0x4a8>
   ae52e:	movzbl 0x3(%r9),%edx
   ae533:	mov    %edx,%esi
   ae535:	and    $0x7f,%edx
   ae538:	shl    $0x15,%edx
   ae53b:	or     %edx,%ecx
   ae53d:	mov    $0x4,%edx
   ae542:	test   %sil,%sil
   ae545:	jns    ae558 <meshoptimizer_rs::codec::raw+0x4a8>
   ae547:	movzbl 0x4(%r9),%esi
   ae54c:	shl    $0x1c,%esi
   ae54f:	or     %ecx,%esi
   ae551:	mov    $0x5,%edx
   ae556:	mov    %esi,%ecx
   ae558:	sub    %rdx,%rbx
   ae55b:	add    %rdx,%r9
   ae55e:	jmp    ae4b6 <meshoptimizer_rs::codec::raw+0x406>
   ae563:	lea    0x11(%rdx),%rcx
   ae567:	mov    $0x7,%al
   ae569:	cmp    %rcx,%rbx
   ae56c:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae572:	movzbl (%r9),%ecx
   ae576:	mov    %ecx,%esi
   ae578:	and    $0xf0,%sil
   ae57c:	cmp    $0xe0,%sil
   ae580:	jne    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae586:	and    $0xf,%cl
   ae589:	mov    $0x5,%al
   ae58b:	cmp    $0x1,%cl
   ae58e:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae594:	mov    %r9,%r10
   ae597:	lea    0x1(%rdx),%rsi
   ae59b:	pcmpeqd %xmm0,%xmm0
   ae59f:	movdqa %xmm0,0x270(%rsp)
   ae5a8:	movdqa %xmm0,0x260(%rsp)
   ae5b1:	movdqa %xmm0,0x250(%rsp)
   ae5ba:	movdqa %xmm0,0x240(%rsp)
   ae5c3:	movdqa %xmm0,0x230(%rsp)
   ae5cc:	movdqa %xmm0,0x220(%rsp)
   ae5d5:	movdqa %xmm0,0x210(%rsp)
   ae5de:	movdqa %xmm0,0x200(%rsp)
   ae5e7:	movdqa %xmm0,0x130(%rsp)
   ae5f0:	movdqa %xmm0,0x120(%rsp)
   ae5f9:	movdqa %xmm0,0x110(%rsp)
   ae602:	movdqa %xmm0,0x100(%rsp)
   ae60b:	xor    %r8d,%r8d
   ae60e:	cmp    $0x1,%cl
   ae611:	setne  %r9b
   ae615:	lea    -0x10(%rbx),%rax
   ae619:	mov    %rax,0x28(%rsp)
   ae61e:	mov    %r15,%rax
   ae621:	mov    %rdx,%rcx
   ae624:	mul    %rdi
   ae627:	shr    $0x3,%rdx
   ae62b:	cmp    %rcx,%rdx
   ae62e:	cmovae %rcx,%rdx
   ae632:	mov    %rdx,0x40(%rsp)
   ae637:	test   %rdx,%rdx
   ae63a:	je     afbe8 <meshoptimizer_rs::codec::raw+0x1b38>
   ae640:	mov    %r9b,%r8b
   ae643:	lea    0xd(,%r8,2),%rax
   ae64b:	mov    %rax,0x58(%rsp)
   ae650:	mov    %rbx,0x38(%rsp)
   ae655:	mov    %r10,0x30(%rsp)
   ae65a:	lea    (%r10,%rbx,1),%rax
   ae65e:	add    $0xfffffffffffffff0,%rax
   ae662:	mov    %rax,0x60(%rsp)
   ae667:	add    $0x8,%r12
   ae66b:	xor    %r8d,%r8d
   ae66e:	xor    %r10d,%r10d
   ae671:	movl   $0x0,0x20(%rsp)
   ae679:	xor    %r11d,%r11d
   ae67c:	xor    %edi,%edi
   ae67e:	mov    0x28(%rsp),%r9
   ae683:	jmp    aecf8 <meshoptimizer_rs::codec::raw+0xc48>
   ae688:	cmp    $0x4,%rbx
   ae68c:	sete   %al
   ae68f:	neg    %al
   ae691:	or     $0x7,%al
   ae693:	jmp    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   ae698:	mov    %bpl,%r14b
   ae69b:	add    %r14d,%r11d
   ae69e:	mov    %ebx,0x100(%rsp,%r10,4)
   ae6a6:	mov    %r11d,%ebp
   ae6a9:	not    %eax
   ae6ab:	shr    $0x4,%eax
   ae6ae:	add    %r8d,%eax
   ae6b1:	and    $0xf,%eax
   ae6b4:	mov    0x200(%rsp,%rax,8),%r11d
   ae6bc:	mov    0x204(%rsp,%rax,8),%r15d
   ae6c4:	add    %r14,%r10
   ae6c7:	mov    $0x2,%eax
   ae6cc:	mov    $0x1,%r12d
   ae6d2:	mov    %r8,%r13
   ae6d5:	inc    %rdi
   ae6d8:	mov    %ebx,0x200(%rsp,%r13,8)
   ae6e0:	mov    %r15d,0x204(%rsp,%r13,8)
   ae6e8:	add    %r8d,%r12d
   ae6eb:	and    $0xf,%r12d
   ae6ef:	mov    %r11d,0x200(%rsp,%r12,8)
   ae6f7:	mov    %ebx,0x204(%rsp,%r12,8)
   ae6ff:	add    %r8d,%eax
   ae702:	and    $0xf,%eax
   ae705:	and    $0xf,%r10d
   ae709:	mov    %rdx,%r12
   ae70c:	mov    %r11w,-0x4(%rdx)
   ae711:	mov    %r15w,-0x2(%rdx)
   ae716:	mov    %bx,(%rdx)
   ae719:	add    $0x6,%r12
   ae71d:	mov    %rax,%r8
   ae720:	mov    %ebp,%r11d
   ae723:	cmp    %rdi,0x40(%rsp)
   ae728:	je     afbed <meshoptimizer_rs::codec::raw+0x1b3d>
   ae72e:	cmp    %r9,%rsi
   ae731:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   ae737:	mov    %r12,%rdx
   ae73a:	mov    0x30(%rsp),%rcx
   ae73f:	movzbl 0x1(%rcx,%rdi,1),%eax
   ae744:	cmp    $0xf0,%al
   ae746:	jae    ae780 <meshoptimizer_rs::codec::raw+0x6d0>
   ae748:	mov    %eax,%r15d
   ae74b:	and    $0xf,%r15d
   ae74f:	cmp    %r15d,0x58(%rsp)
   ae754:	jbe    ae7be <meshoptimizer_rs::codec::raw+0x70e>
   ae756:	xor    %r14d,%r14d
   ae759:	mov    %r11d,%ebx
   ae75c:	test   %r15b,%r15b
   ae75f:	sete   %bpl
   ae763:	je     ae698 <meshoptimizer_rs::codec::raw+0x5e8>
   ae769:	not    %r15d
   ae76c:	add    %r10d,%r15d
   ae76f:	and    $0xf,%r15d
   ae773:	mov    0x100(%rsp,%r15,4),%ebx
   ae77b:	jmp    ae698 <meshoptimizer_rs::codec::raw+0x5e8>
   ae780:	cmp    $0xfe,%al
   ae782:	jae    ae7e5 <meshoptimizer_rs::codec::raw+0x735>
   ae784:	and    $0xf,%eax
   ae787:	mov    0x60(%rsp),%rcx
   ae78c:	movzbl (%rcx,%rax,1),%r13d
   ae791:	lea    0x1(%r11),%r15d
   ae795:	xor    %eax,%eax
   ae797:	mov    %r13d,%ecx
   ae79a:	shr    $0x4,%ecx
   ae79d:	sete   %r12b
   ae7a1:	je     ae828 <meshoptimizer_rs::codec::raw+0x778>
   ae7a7:	mov    %r10d,%ebx
   ae7aa:	sub    %ecx,%ebx
   ae7ac:	and    $0xf,%ebx
   ae7af:	mov    %r15d,%ebp
   ae7b2:	mov    0x100(%rsp,%rbx,4),%ecx
   ae7b9:	mov    %ecx,%r15d
   ae7bc:	jmp    ae82c <meshoptimizer_rs::codec::raw+0x77c>
   ae7be:	cmp    $0xf,%r15b
   ae7c2:	jne    ae8a8 <meshoptimizer_rs::codec::raw+0x7f8>
   ae7c8:	mov    0x30(%rsp),%r14
   ae7cd:	movzbl (%r14,%rsi,1),%ebp
   ae7d2:	test   %bpl,%bpl
   ae7d5:	js     ae916 <meshoptimizer_rs::codec::raw+0x866>
   ae7db:	inc    %rsi
   ae7de:	mov    %ebp,%ebx
   ae7e0:	jmp    aeaa9 <meshoptimizer_rs::codec::raw+0x9f9>
   ae7e5:	movzbl (%rcx,%rsi,1),%r14d
   ae7ea:	xor    %r15d,%r15d
   ae7ed:	cmp    $0xfe,%eax
   ae7f2:	sete   %r15b
   ae7f6:	test   %r14d,%r14d
   ae7f9:	cmove  %r14d,%r11d
   ae7fd:	add    %r11d,%r15d
   ae800:	mov    %r14d,%r12d
   ae803:	shr    $0x4,%r12d
   ae807:	je     ae8c3 <meshoptimizer_rs::codec::raw+0x813>
   ae80d:	mov    %r10d,%ecx
   ae810:	sub    %r12d,%ecx
   ae813:	and    $0xf,%ecx
   ae816:	mov    %r15d,%ebx
   ae819:	mov    0x100(%rsp,%rcx,4),%ecx
   ae820:	mov    %ecx,%r15d
   ae823:	jmp    ae8c7 <meshoptimizer_rs::codec::raw+0x817>
   ae828:	lea    0x2(%r11),%ebp
   ae82c:	xor    %r14d,%r14d
   ae82f:	mov    %ebp,%ebx
   ae831:	test   $0xf,%r13b
   ae835:	sete   %cl
   ae838:	je     ae84a <meshoptimizer_rs::codec::raw+0x79a>
   ae83a:	mov    %r10d,%ebx
   ae83d:	sub    %r13d,%ebx
   ae840:	and    $0xf,%ebx
   ae843:	mov    0x100(%rsp,%rbx,4),%ebx
   ae84a:	mov    %cl,%r14b
   ae84d:	add    %r14d,%ebp
   ae850:	mov    %r11d,0x100(%rsp,%r10,4)
   ae858:	lea    0x1(%r10),%rcx
   ae85c:	and    $0xf,%ecx
   ae85f:	mov    %r15d,0x100(%rsp,%rcx,4)
   ae867:	mov    %r12b,%al
   ae86a:	add    %r10,%rax
   ae86d:	inc    %rax
   ae870:	add    %rax,%r14
   ae873:	and    $0xf,%eax
   ae876:	mov    %ebx,0x100(%rsp,%rax,4)
   ae87d:	mov    %r15d,0x200(%rsp,%r8,8)
   ae885:	mov    %r11d,0x204(%rsp,%r8,8)
   ae88d:	lea    0x1(%r8),%r13d
   ae891:	and    $0xf,%r13d
   ae895:	mov    $0x3,%eax
   ae89a:	mov    $0x2,%r12d
   ae8a0:	mov    %r14,%r10
   ae8a3:	jmp    ae6d5 <meshoptimizer_rs::codec::raw+0x625>
   ae8a8:	movzbl %r15b,%ecx
   ae8ac:	xor    $0x3,%r15b
   ae8b0:	movzbl %r15b,%ebx
   ae8b4:	mov    0x20(%rsp),%ebp
   ae8b8:	add    %ecx,%ebp
   ae8ba:	sub    %ebx,%ebp
   ae8bc:	mov    %ebp,%ebx
   ae8be:	jmp    aeaba <meshoptimizer_rs::codec::raw+0xa0a>
   ae8c3:	lea    0x1(%r15),%ebx
   ae8c7:	mov    %r14d,%r13d
   ae8ca:	and    $0xf,%r13b
   ae8ce:	je     ae8e6 <meshoptimizer_rs::codec::raw+0x836>
   ae8d0:	mov    %r10d,%ecx
   ae8d3:	sub    %r14d,%ecx
   ae8d6:	and    $0xf,%ecx
   ae8d9:	mov    %ebx,%ebp
   ae8db:	mov    0x100(%rsp,%rcx,4),%ecx
   ae8e2:	mov    %ecx,%ebx
   ae8e4:	jmp    ae8e9 <meshoptimizer_rs::codec::raw+0x839>
   ae8e6:	lea    0x1(%rbx),%ebp
   ae8e9:	lea    0x1(%rsi),%r14
   ae8ed:	cmp    $0xfe,%eax
   ae8f2:	je     ae9f0 <meshoptimizer_rs::codec::raw+0x940>
   ae8f8:	mov    0x30(%rsp),%r9
   ae8fd:	movzbl 0x1(%r9,%rsi,1),%eax
   ae903:	test   %al,%al
   ae905:	js     ae936 <meshoptimizer_rs::codec::raw+0x886>
   ae907:	add    $0x2,%rsi
   ae90b:	mov    %rsi,%r14
   ae90e:	mov    %eax,%r11d
   ae911:	jmp    ae9d2 <meshoptimizer_rs::codec::raw+0x922>
   ae916:	and    $0x7f,%ebp
   ae919:	movzbl 0x1(%r14,%rsi,1),%ebx
   ae91f:	mov    %ebx,%ecx
   ae921:	and    $0x7f,%ebx
   ae924:	shl    $0x7,%ebx
   ae927:	or     %ebp,%ebx
   ae929:	test   %cl,%cl
   ae92b:	js     ae957 <meshoptimizer_rs::codec::raw+0x8a7>
   ae92d:	add    $0x2,%rsi
   ae931:	jmp    aeaa9 <meshoptimizer_rs::codec::raw+0x9f9>
   ae936:	and    $0x7f,%eax
   ae939:	movzbl 0x2(%r9,%rsi,1),%r11d
   ae93f:	mov    %r11d,%ecx
   ae942:	and    $0x7f,%r11d
   ae946:	shl    $0x7,%r11d
   ae94a:	or     %eax,%r11d
   ae94d:	test   %cl,%cl
   ae94f:	js     ae975 <meshoptimizer_rs::codec::raw+0x8c5>
   ae951:	add    $0x3,%rsi
   ae955:	jmp    ae9cf <meshoptimizer_rs::codec::raw+0x91f>
   ae957:	movzbl 0x2(%r14,%rsi,1),%ecx
   ae95d:	mov    %ecx,%ebp
   ae95f:	and    $0x7f,%ecx
   ae962:	shl    $0xe,%ecx
   ae965:	or     %ecx,%ebx
   ae967:	test   %bpl,%bpl
   ae96a:	js     ae990 <meshoptimizer_rs::codec::raw+0x8e0>
   ae96c:	add    $0x3,%rsi
   ae970:	jmp    aeaa9 <meshoptimizer_rs::codec::raw+0x9f9>
   ae975:	movzbl 0x3(%r9,%rsi,1),%eax
   ae97b:	mov    %eax,%ecx
   ae97d:	and    $0x7f,%eax
   ae980:	shl    $0xe,%eax
   ae983:	or     %eax,%r11d
   ae986:	test   %cl,%cl
   ae988:	js     ae9b2 <meshoptimizer_rs::codec::raw+0x902>
   ae98a:	add    $0x4,%rsi
   ae98e:	jmp    ae9cf <meshoptimizer_rs::codec::raw+0x91f>
   ae990:	movzbl 0x3(%r14,%rsi,1),%ecx
   ae996:	mov    %ecx,%ebp
   ae998:	and    $0x7f,%ecx
   ae99b:	shl    $0x15,%ecx
   ae99e:	or     %ecx,%ebx
   ae9a0:	test   %bpl,%bpl
   ae9a3:	js     aea98 <meshoptimizer_rs::codec::raw+0x9e8>
   ae9a9:	add    $0x4,%rsi
   ae9ad:	jmp    aeaa9 <meshoptimizer_rs::codec::raw+0x9f9>
   ae9b2:	movzbl 0x4(%r9,%rsi,1),%eax
   ae9b8:	mov    %eax,%ecx
   ae9ba:	and    $0x7f,%eax
   ae9bd:	shl    $0x15,%eax
   ae9c0:	or     %eax,%r11d
   ae9c3:	test   %cl,%cl
   ae9c5:	js     aead1 <meshoptimizer_rs::codec::raw+0xa21>
   ae9cb:	add    $0x5,%rsi
   ae9cf:	mov    %rsi,%r14
   ae9d2:	mov    0x28(%rsp),%r9
   ae9d7:	mov    %r11d,%eax
   ae9da:	shr    $1,%eax
   ae9dc:	and    $0x1,%r11d
   ae9e0:	neg    %r11d
   ae9e3:	xor    %eax,%r11d
   ae9e6:	add    0x20(%rsp),%r11d
   ae9eb:	mov    %r11d,0x20(%rsp)
   ae9f0:	cmp    $0xf,%r12b
   ae9f4:	jne    aeb20 <meshoptimizer_rs::codec::raw+0xa70>
   ae9fa:	cmp    $0xfffffffffffffffa,%r14
   ae9fe:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aea04:	lea    0x5(%r14),%rax
   aea08:	cmp    0x38(%rsp),%rax
   aea0d:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aea13:	mov    0x30(%rsp),%r9
   aea18:	movzbl (%r9,%r14,1),%esi
   aea1d:	test   %sil,%sil
   aea20:	js     aea30 <meshoptimizer_rs::codec::raw+0x980>
   aea22:	inc    %r14
   aea25:	mov    %r14,%rax
   aea28:	mov    %esi,%r15d
   aea2b:	jmp    aeaff <meshoptimizer_rs::codec::raw+0xa4f>
   aea30:	and    $0x7f,%esi
   aea33:	movzbl 0x1(%r9,%r14,1),%r15d
   aea39:	mov    %r15d,%ecx
   aea3c:	and    $0x7f,%r15d
   aea40:	shl    $0x7,%r15d
   aea44:	or     %esi,%r15d
   aea47:	test   %cl,%cl
   aea49:	js     aea57 <meshoptimizer_rs::codec::raw+0x9a7>
   aea4b:	add    $0x2,%r14
   aea4f:	mov    %r14,%rax
   aea52:	jmp    aeaff <meshoptimizer_rs::codec::raw+0xa4f>
   aea57:	movzbl 0x2(%r9,%r14,1),%ecx
   aea5d:	mov    %ecx,%esi
   aea5f:	and    $0x7f,%ecx
   aea62:	shl    $0xe,%ecx
   aea65:	or     %ecx,%r15d
   aea68:	test   %sil,%sil
   aea6b:	js     aea79 <meshoptimizer_rs::codec::raw+0x9c9>
   aea6d:	add    $0x3,%r14
   aea71:	mov    %r14,%rax
   aea74:	jmp    aeaff <meshoptimizer_rs::codec::raw+0xa4f>
   aea79:	movzbl 0x3(%r9,%r14,1),%ecx
   aea7f:	mov    %ecx,%esi
   aea81:	and    $0x7f,%ecx
   aea84:	shl    $0x15,%ecx
   aea87:	or     %ecx,%r15d
   aea8a:	test   %sil,%sil
   aea8d:	js     aeaeb <meshoptimizer_rs::codec::raw+0xa3b>
   aea8f:	add    $0x4,%r14
   aea93:	mov    %r14,%rax
   aea96:	jmp    aeaff <meshoptimizer_rs::codec::raw+0xa4f>
   aea98:	movzbl 0x4(%r14,%rsi,1),%ecx
   aea9e:	add    $0x5,%rsi
   aeaa2:	shl    $0x1c,%ecx
   aeaa5:	or     %ebx,%ecx
   aeaa7:	mov    %ecx,%ebx
   aeaa9:	mov    0x20(%rsp),%ebp
   aeaad:	mov    %ebx,%ecx
   aeaaf:	shr    $1,%ecx
   aeab1:	and    $0x1,%ebx
   aeab4:	neg    %ebx
   aeab6:	xor    %ecx,%ebx
   aeab8:	add    %ebp,%ebx
   aeaba:	mov    %ebx,0x100(%rsp,%r10,4)
   aeac2:	mov    $0x1,%r14d
   aeac8:	mov    %ebx,0x20(%rsp)
   aeacc:	jmp    ae6a6 <meshoptimizer_rs::codec::raw+0x5f6>
   aead1:	add    $0x6,%rsi
   aead5:	mov    0x30(%rsp),%rax
   aeada:	movzbl 0x4(%rax,%r14,1),%eax
   aeae0:	shl    $0x1c,%eax
   aeae3:	or     %r11d,%eax
   aeae6:	jmp    ae90b <meshoptimizer_rs::codec::raw+0x85b>
   aeaeb:	mov    0x30(%rsp),%rcx
   aeaf0:	movzbl 0x4(%rcx,%r14,1),%ecx
   aeaf6:	shl    $0x1c,%ecx
   aeaf9:	or     %r15d,%ecx
   aeafc:	mov    %ecx,%r15d
   aeaff:	mov    0x28(%rsp),%r9
   aeb04:	mov    %r15d,%ecx
   aeb07:	shr    $1,%ecx
   aeb09:	and    $0x1,%r15d
   aeb0d:	neg    %r15d
   aeb10:	xor    %ecx,%r15d
   aeb13:	add    0x20(%rsp),%r15d
   aeb18:	mov    %rax,%r14
   aeb1b:	mov    %r15d,0x20(%rsp)
   aeb20:	cmp    $0xf,%r13b
   aeb24:	jne    aebe5 <meshoptimizer_rs::codec::raw+0xb35>
   aeb2a:	cmp    $0xfffffffffffffffa,%r14
   aeb2e:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aeb34:	lea    0x5(%r14),%rax
   aeb38:	cmp    0x38(%rsp),%rax
   aeb3d:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aeb43:	mov    0x30(%rsp),%r9
   aeb48:	movzbl (%r9,%r14,1),%esi
   aeb4d:	test   %sil,%sil
   aeb50:	js     aeb5c <meshoptimizer_rs::codec::raw+0xaac>
   aeb52:	inc    %r14
   aeb55:	mov    %r14,%rax
   aeb58:	mov    %esi,%ebx
   aeb5a:	jmp    aebca <meshoptimizer_rs::codec::raw+0xb1a>
   aeb5c:	and    $0x7f,%esi
   aeb5f:	movzbl 0x1(%r9,%r14,1),%ebx
   aeb65:	mov    %ebx,%ecx
   aeb67:	and    $0x7f,%ebx
   aeb6a:	shl    $0x7,%ebx
   aeb6d:	or     %esi,%ebx
   aeb6f:	test   %cl,%cl
   aeb71:	js     aeb7c <meshoptimizer_rs::codec::raw+0xacc>
   aeb73:	add    $0x2,%r14
   aeb77:	mov    %r14,%rax
   aeb7a:	jmp    aebca <meshoptimizer_rs::codec::raw+0xb1a>
   aeb7c:	movzbl 0x2(%r9,%r14,1),%ecx
   aeb82:	mov    %ecx,%esi
   aeb84:	and    $0x7f,%ecx
   aeb87:	shl    $0xe,%ecx
   aeb8a:	or     %ecx,%ebx
   aeb8c:	test   %sil,%sil
   aeb8f:	js     aeb9a <meshoptimizer_rs::codec::raw+0xaea>
   aeb91:	add    $0x3,%r14
   aeb95:	mov    %r14,%rax
   aeb98:	jmp    aebca <meshoptimizer_rs::codec::raw+0xb1a>
   aeb9a:	movzbl 0x3(%r9,%r14,1),%ecx
   aeba0:	mov    %ecx,%esi
   aeba2:	and    $0x7f,%ecx
   aeba5:	shl    $0x15,%ecx
   aeba8:	or     %ecx,%ebx
   aebaa:	test   %sil,%sil
   aebad:	js     aebb8 <meshoptimizer_rs::codec::raw+0xb08>
   aebaf:	add    $0x4,%r14
   aebb3:	mov    %r14,%rax
   aebb6:	jmp    aebca <meshoptimizer_rs::codec::raw+0xb1a>
   aebb8:	mov    0x30(%rsp),%rcx
   aebbd:	movzbl 0x4(%rcx,%r14,1),%ecx
   aebc3:	shl    $0x1c,%ecx
   aebc6:	or     %ebx,%ecx
   aebc8:	mov    %ecx,%ebx
   aebca:	mov    0x28(%rsp),%r9
   aebcf:	mov    %ebx,%ecx
   aebd1:	shr    $1,%ecx
   aebd3:	and    $0x1,%ebx
   aebd6:	neg    %ebx
   aebd8:	xor    %ecx,%ebx
   aebda:	add    0x20(%rsp),%ebx
   aebde:	mov    %rax,%r14
   aebe1:	mov    %ebx,0x20(%rsp)
   aebe5:	cmp    $0xf,%r13b
   aebe9:	sete   %al
   aebec:	cmp    $0xf,%r12b
   aebf0:	sete   %cl
   aebf3:	test   %r13b,%r13b
   aebf6:	sete   %sil
   aebfa:	test   %r12b,%r12b
   aebfd:	sete   %r12b
   aec01:	mov    %r11d,0x100(%rsp,%r10,4)
   aec09:	lea    0x1(%r10),%r13
   aec0d:	and    $0xf,%r13d
   aec11:	or     %cl,%r12b
   aec14:	mov    %r15d,0x100(%rsp,%r13,4)
   aec1c:	movzbl %r12b,%ecx
   aec20:	add    %r10,%rcx
   aec23:	inc    %rcx
   aec26:	or     %al,%sil
   aec29:	movzbl %sil,%r10d
   aec2d:	add    %rcx,%r10
   aec30:	and    $0xf,%ecx
   aec33:	mov    %ebx,0x100(%rsp,%rcx,4)
   aec3a:	mov    %r15d,0x200(%rsp,%r8,8)
   aec42:	mov    %r11d,0x204(%rsp,%r8,8)
   aec4a:	lea    0x1(%r8),%r13d
   aec4e:	and    $0xf,%r13d
   aec52:	mov    $0x3,%eax
   aec57:	mov    $0x2,%r12d
   aec5d:	mov    %r14,%rsi
   aec60:	jmp    ae6d5 <meshoptimizer_rs::codec::raw+0x625>
   aec65:	mov    %bpl,%r14b
   aec68:	add    %r14d,%r11d
   aec6b:	mov    %ebx,0x100(%rsp,%r10,4)
   aec73:	mov    %r11d,%ebp
   aec76:	not    %eax
   aec78:	shr    $0x4,%eax
   aec7b:	add    %r8d,%eax
   aec7e:	and    $0xf,%eax
   aec81:	mov    0x200(%rsp,%rax,8),%r11d
   aec89:	mov    0x204(%rsp,%rax,8),%r15d
   aec91:	add    %r14,%r10
   aec94:	mov    $0x2,%eax
   aec99:	mov    $0x1,%r12d
   aec9f:	mov    %r8,%r13
   aeca2:	inc    %rdi
   aeca5:	mov    %ebx,0x200(%rsp,%r13,8)
   aecad:	mov    %r15d,0x204(%rsp,%r13,8)
   aecb5:	add    %r8d,%r12d
   aecb8:	and    $0xf,%r12d
   aecbc:	mov    %r11d,0x200(%rsp,%r12,8)
   aecc4:	mov    %ebx,0x204(%rsp,%r12,8)
   aeccc:	add    %r8d,%eax
   aeccf:	and    $0xf,%eax
   aecd2:	and    $0xf,%r10d
   aecd6:	mov    %rdx,%r12
   aecd9:	mov    %r11d,-0x8(%rdx)
   aecdd:	mov    %r15d,-0x4(%rdx)
   aece1:	mov    %ebx,(%rdx)
   aece3:	add    $0xc,%r12
   aece7:	mov    %rax,%r8
   aecea:	mov    %ebp,%r11d
   aeced:	cmp    %rdi,0x40(%rsp)
   aecf2:	je     afbed <meshoptimizer_rs::codec::raw+0x1b3d>
   aecf8:	cmp    %r9,%rsi
   aecfb:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aed01:	mov    %r12,%rdx
   aed04:	mov    0x30(%rsp),%rcx
   aed09:	movzbl 0x1(%rcx,%rdi,1),%eax
   aed0e:	cmp    $0xf0,%al
   aed10:	jae    aed4a <meshoptimizer_rs::codec::raw+0xc9a>
   aed12:	mov    %eax,%r15d
   aed15:	and    $0xf,%r15d
   aed19:	cmp    %r15d,0x58(%rsp)
   aed1e:	jbe    aed88 <meshoptimizer_rs::codec::raw+0xcd8>
   aed20:	xor    %r14d,%r14d
   aed23:	mov    %r11d,%ebx
   aed26:	test   %r15b,%r15b
   aed29:	sete   %bpl
   aed2d:	je     aec65 <meshoptimizer_rs::codec::raw+0xbb5>
   aed33:	not    %r15d
   aed36:	add    %r10d,%r15d
   aed39:	and    $0xf,%r15d
   aed3d:	mov    0x100(%rsp,%r15,4),%ebx
   aed45:	jmp    aec65 <meshoptimizer_rs::codec::raw+0xbb5>
   aed4a:	cmp    $0xfe,%al
   aed4c:	jae    aedaf <meshoptimizer_rs::codec::raw+0xcff>
   aed4e:	and    $0xf,%eax
   aed51:	mov    0x60(%rsp),%rcx
   aed56:	movzbl (%rcx,%rax,1),%r13d
   aed5b:	lea    0x1(%r11),%r15d
   aed5f:	xor    %eax,%eax
   aed61:	mov    %r13d,%ecx
   aed64:	shr    $0x4,%ecx
   aed67:	sete   %r12b
   aed6b:	je     aedf2 <meshoptimizer_rs::codec::raw+0xd42>
   aed71:	mov    %r10d,%ebx
   aed74:	sub    %ecx,%ebx
   aed76:	and    $0xf,%ebx
   aed79:	mov    %r15d,%ebp
   aed7c:	mov    0x100(%rsp,%rbx,4),%ecx
   aed83:	mov    %ecx,%r15d
   aed86:	jmp    aedf6 <meshoptimizer_rs::codec::raw+0xd46>
   aed88:	cmp    $0xf,%r15b
   aed8c:	jne    aee72 <meshoptimizer_rs::codec::raw+0xdc2>
   aed92:	mov    0x30(%rsp),%r14
   aed97:	movzbl (%r14,%rsi,1),%ebp
   aed9c:	test   %bpl,%bpl
   aed9f:	js     aeee0 <meshoptimizer_rs::codec::raw+0xe30>
   aeda5:	inc    %rsi
   aeda8:	mov    %ebp,%ebx
   aedaa:	jmp    af073 <meshoptimizer_rs::codec::raw+0xfc3>
   aedaf:	movzbl (%rcx,%rsi,1),%r14d
   aedb4:	xor    %r15d,%r15d
   aedb7:	cmp    $0xfe,%eax
   aedbc:	sete   %r15b
   aedc0:	test   %r14d,%r14d
   aedc3:	cmove  %r14d,%r11d
   aedc7:	add    %r11d,%r15d
   aedca:	mov    %r14d,%r12d
   aedcd:	shr    $0x4,%r12d
   aedd1:	je     aee8d <meshoptimizer_rs::codec::raw+0xddd>
   aedd7:	mov    %r10d,%ecx
   aedda:	sub    %r12d,%ecx
   aeddd:	and    $0xf,%ecx
   aede0:	mov    %r15d,%ebx
   aede3:	mov    0x100(%rsp,%rcx,4),%ecx
   aedea:	mov    %ecx,%r15d
   aeded:	jmp    aee91 <meshoptimizer_rs::codec::raw+0xde1>
   aedf2:	lea    0x2(%r11),%ebp
   aedf6:	xor    %r14d,%r14d
   aedf9:	mov    %ebp,%ebx
   aedfb:	test   $0xf,%r13b
   aedff:	sete   %cl
   aee02:	je     aee14 <meshoptimizer_rs::codec::raw+0xd64>
   aee04:	mov    %r10d,%ebx
   aee07:	sub    %r13d,%ebx
   aee0a:	and    $0xf,%ebx
   aee0d:	mov    0x100(%rsp,%rbx,4),%ebx
   aee14:	mov    %cl,%r14b
   aee17:	add    %r14d,%ebp
   aee1a:	mov    %r11d,0x100(%rsp,%r10,4)
   aee22:	lea    0x1(%r10),%rcx
   aee26:	and    $0xf,%ecx
   aee29:	mov    %r15d,0x100(%rsp,%rcx,4)
   aee31:	mov    %r12b,%al
   aee34:	add    %r10,%rax
   aee37:	inc    %rax
   aee3a:	add    %rax,%r14
   aee3d:	and    $0xf,%eax
   aee40:	mov    %ebx,0x100(%rsp,%rax,4)
   aee47:	mov    %r15d,0x200(%rsp,%r8,8)
   aee4f:	mov    %r11d,0x204(%rsp,%r8,8)
   aee57:	lea    0x1(%r8),%r13d
   aee5b:	and    $0xf,%r13d
   aee5f:	mov    $0x3,%eax
   aee64:	mov    $0x2,%r12d
   aee6a:	mov    %r14,%r10
   aee6d:	jmp    aeca2 <meshoptimizer_rs::codec::raw+0xbf2>
   aee72:	movzbl %r15b,%ecx
   aee76:	xor    $0x3,%r15b
   aee7a:	movzbl %r15b,%ebx
   aee7e:	mov    0x20(%rsp),%ebp
   aee82:	add    %ecx,%ebp
   aee84:	sub    %ebx,%ebp
   aee86:	mov    %ebp,%ebx
   aee88:	jmp    af084 <meshoptimizer_rs::codec::raw+0xfd4>
   aee8d:	lea    0x1(%r15),%ebx
   aee91:	mov    %r14d,%r13d
   aee94:	and    $0xf,%r13b
   aee98:	je     aeeb0 <meshoptimizer_rs::codec::raw+0xe00>
   aee9a:	mov    %r10d,%ecx
   aee9d:	sub    %r14d,%ecx
   aeea0:	and    $0xf,%ecx
   aeea3:	mov    %ebx,%ebp
   aeea5:	mov    0x100(%rsp,%rcx,4),%ecx
   aeeac:	mov    %ecx,%ebx
   aeeae:	jmp    aeeb3 <meshoptimizer_rs::codec::raw+0xe03>
   aeeb0:	lea    0x1(%rbx),%ebp
   aeeb3:	lea    0x1(%rsi),%r14
   aeeb7:	cmp    $0xfe,%eax
   aeebc:	je     aefba <meshoptimizer_rs::codec::raw+0xf0a>
   aeec2:	mov    0x30(%rsp),%r9
   aeec7:	movzbl 0x1(%r9,%rsi,1),%eax
   aeecd:	test   %al,%al
   aeecf:	js     aef00 <meshoptimizer_rs::codec::raw+0xe50>
   aeed1:	add    $0x2,%rsi
   aeed5:	mov    %rsi,%r14
   aeed8:	mov    %eax,%r11d
   aeedb:	jmp    aef9c <meshoptimizer_rs::codec::raw+0xeec>
   aeee0:	and    $0x7f,%ebp
   aeee3:	movzbl 0x1(%r14,%rsi,1),%ebx
   aeee9:	mov    %ebx,%ecx
   aeeeb:	and    $0x7f,%ebx
   aeeee:	shl    $0x7,%ebx
   aeef1:	or     %ebp,%ebx
   aeef3:	test   %cl,%cl
   aeef5:	js     aef21 <meshoptimizer_rs::codec::raw+0xe71>
   aeef7:	add    $0x2,%rsi
   aeefb:	jmp    af073 <meshoptimizer_rs::codec::raw+0xfc3>
   aef00:	and    $0x7f,%eax
   aef03:	movzbl 0x2(%r9,%rsi,1),%r11d
   aef09:	mov    %r11d,%ecx
   aef0c:	and    $0x7f,%r11d
   aef10:	shl    $0x7,%r11d
   aef14:	or     %eax,%r11d
   aef17:	test   %cl,%cl
   aef19:	js     aef3f <meshoptimizer_rs::codec::raw+0xe8f>
   aef1b:	add    $0x3,%rsi
   aef1f:	jmp    aef99 <meshoptimizer_rs::codec::raw+0xee9>
   aef21:	movzbl 0x2(%r14,%rsi,1),%ecx
   aef27:	mov    %ecx,%ebp
   aef29:	and    $0x7f,%ecx
   aef2c:	shl    $0xe,%ecx
   aef2f:	or     %ecx,%ebx
   aef31:	test   %bpl,%bpl
   aef34:	js     aef5a <meshoptimizer_rs::codec::raw+0xeaa>
   aef36:	add    $0x3,%rsi
   aef3a:	jmp    af073 <meshoptimizer_rs::codec::raw+0xfc3>
   aef3f:	movzbl 0x3(%r9,%rsi,1),%eax
   aef45:	mov    %eax,%ecx
   aef47:	and    $0x7f,%eax
   aef4a:	shl    $0xe,%eax
   aef4d:	or     %eax,%r11d
   aef50:	test   %cl,%cl
   aef52:	js     aef7c <meshoptimizer_rs::codec::raw+0xecc>
   aef54:	add    $0x4,%rsi
   aef58:	jmp    aef99 <meshoptimizer_rs::codec::raw+0xee9>
   aef5a:	movzbl 0x3(%r14,%rsi,1),%ecx
   aef60:	mov    %ecx,%ebp
   aef62:	and    $0x7f,%ecx
   aef65:	shl    $0x15,%ecx
   aef68:	or     %ecx,%ebx
   aef6a:	test   %bpl,%bpl
   aef6d:	js     af062 <meshoptimizer_rs::codec::raw+0xfb2>
   aef73:	add    $0x4,%rsi
   aef77:	jmp    af073 <meshoptimizer_rs::codec::raw+0xfc3>
   aef7c:	movzbl 0x4(%r9,%rsi,1),%eax
   aef82:	mov    %eax,%ecx
   aef84:	and    $0x7f,%eax
   aef87:	shl    $0x15,%eax
   aef8a:	or     %eax,%r11d
   aef8d:	test   %cl,%cl
   aef8f:	js     af09b <meshoptimizer_rs::codec::raw+0xfeb>
   aef95:	add    $0x5,%rsi
   aef99:	mov    %rsi,%r14
   aef9c:	mov    0x28(%rsp),%r9
   aefa1:	mov    %r11d,%eax
   aefa4:	shr    $1,%eax
   aefa6:	and    $0x1,%r11d
   aefaa:	neg    %r11d
   aefad:	xor    %eax,%r11d
   aefb0:	add    0x20(%rsp),%r11d
   aefb5:	mov    %r11d,0x20(%rsp)
   aefba:	cmp    $0xf,%r12b
   aefbe:	jne    af0ea <meshoptimizer_rs::codec::raw+0x103a>
   aefc4:	cmp    $0xfffffffffffffffa,%r14
   aefc8:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aefce:	lea    0x5(%r14),%rax
   aefd2:	cmp    0x38(%rsp),%rax
   aefd7:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   aefdd:	mov    0x30(%rsp),%r9
   aefe2:	movzbl (%r9,%r14,1),%esi
   aefe7:	test   %sil,%sil
   aefea:	js     aeffa <meshoptimizer_rs::codec::raw+0xf4a>
   aefec:	inc    %r14
   aefef:	mov    %r14,%rax
   aeff2:	mov    %esi,%r15d
   aeff5:	jmp    af0c9 <meshoptimizer_rs::codec::raw+0x1019>
   aeffa:	and    $0x7f,%esi
   aeffd:	movzbl 0x1(%r9,%r14,1),%r15d
   af003:	mov    %r15d,%ecx
   af006:	and    $0x7f,%r15d
   af00a:	shl    $0x7,%r15d
   af00e:	or     %esi,%r15d
   af011:	test   %cl,%cl
   af013:	js     af021 <meshoptimizer_rs::codec::raw+0xf71>
   af015:	add    $0x2,%r14
   af019:	mov    %r14,%rax
   af01c:	jmp    af0c9 <meshoptimizer_rs::codec::raw+0x1019>
   af021:	movzbl 0x2(%r9,%r14,1),%ecx
   af027:	mov    %ecx,%esi
   af029:	and    $0x7f,%ecx
   af02c:	shl    $0xe,%ecx
   af02f:	or     %ecx,%r15d
   af032:	test   %sil,%sil
   af035:	js     af043 <meshoptimizer_rs::codec::raw+0xf93>
   af037:	add    $0x3,%r14
   af03b:	mov    %r14,%rax
   af03e:	jmp    af0c9 <meshoptimizer_rs::codec::raw+0x1019>
   af043:	movzbl 0x3(%r9,%r14,1),%ecx
   af049:	mov    %ecx,%esi
   af04b:	and    $0x7f,%ecx
   af04e:	shl    $0x15,%ecx
   af051:	or     %ecx,%r15d
   af054:	test   %sil,%sil
   af057:	js     af0b5 <meshoptimizer_rs::codec::raw+0x1005>
   af059:	add    $0x4,%r14
   af05d:	mov    %r14,%rax
   af060:	jmp    af0c9 <meshoptimizer_rs::codec::raw+0x1019>
   af062:	movzbl 0x4(%r14,%rsi,1),%ecx
   af068:	add    $0x5,%rsi
   af06c:	shl    $0x1c,%ecx
   af06f:	or     %ebx,%ecx
   af071:	mov    %ecx,%ebx
   af073:	mov    0x20(%rsp),%ebp
   af077:	mov    %ebx,%ecx
   af079:	shr    $1,%ecx
   af07b:	and    $0x1,%ebx
   af07e:	neg    %ebx
   af080:	xor    %ecx,%ebx
   af082:	add    %ebp,%ebx
   af084:	mov    %ebx,0x100(%rsp,%r10,4)
   af08c:	mov    $0x1,%r14d
   af092:	mov    %ebx,0x20(%rsp)
   af096:	jmp    aec73 <meshoptimizer_rs::codec::raw+0xbc3>
   af09b:	add    $0x6,%rsi
   af09f:	mov    0x30(%rsp),%rax
   af0a4:	movzbl 0x4(%rax,%r14,1),%eax
   af0aa:	shl    $0x1c,%eax
   af0ad:	or     %r11d,%eax
   af0b0:	jmp    aeed5 <meshoptimizer_rs::codec::raw+0xe25>
   af0b5:	mov    0x30(%rsp),%rcx
   af0ba:	movzbl 0x4(%rcx,%r14,1),%ecx
   af0c0:	shl    $0x1c,%ecx
   af0c3:	or     %r15d,%ecx
   af0c6:	mov    %ecx,%r15d
   af0c9:	mov    0x28(%rsp),%r9
   af0ce:	mov    %r15d,%ecx
   af0d1:	shr    $1,%ecx
   af0d3:	and    $0x1,%r15d
   af0d7:	neg    %r15d
   af0da:	xor    %ecx,%r15d
   af0dd:	add    0x20(%rsp),%r15d
   af0e2:	mov    %rax,%r14
   af0e5:	mov    %r15d,0x20(%rsp)
   af0ea:	cmp    $0xf,%r13b
   af0ee:	jne    af1af <meshoptimizer_rs::codec::raw+0x10ff>
   af0f4:	cmp    $0xfffffffffffffffa,%r14
   af0f8:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   af0fe:	lea    0x5(%r14),%rax
   af102:	cmp    0x38(%rsp),%rax
   af107:	ja     afbf4 <meshoptimizer_rs::codec::raw+0x1b44>
   af10d:	mov    0x30(%rsp),%r9
   af112:	movzbl (%r9,%r14,1),%esi
   af117:	test   %sil,%sil
   af11a:	js     af126 <meshoptimizer_rs::codec::raw+0x1076>
   af11c:	inc    %r14
   af11f:	mov    %r14,%rax
   af122:	mov    %esi,%ebx
   af124:	jmp    af194 <meshoptimizer_rs::codec::raw+0x10e4>
   af126:	and    $0x7f,%esi
   af129:	movzbl 0x1(%r9,%r14,1),%ebx
   af12f:	mov    %ebx,%ecx
   af131:	and    $0x7f,%ebx
   af134:	shl    $0x7,%ebx
   af137:	or     %esi,%ebx
   af139:	test   %cl,%cl
   af13b:	js     af146 <meshoptimizer_rs::codec::raw+0x1096>
   af13d:	add    $0x2,%r14
   af141:	mov    %r14,%rax
   af144:	jmp    af194 <meshoptimizer_rs::codec::raw+0x10e4>
   af146:	movzbl 0x2(%r9,%r14,1),%ecx
   af14c:	mov    %ecx,%esi
   af14e:	and    $0x7f,%ecx
   af151:	shl    $0xe,%ecx
   af154:	or     %ecx,%ebx
   af156:	test   %sil,%sil
   af159:	js     af164 <meshoptimizer_rs::codec::raw+0x10b4>
   af15b:	add    $0x3,%r14
   af15f:	mov    %r14,%rax
   af162:	jmp    af194 <meshoptimizer_rs::codec::raw+0x10e4>
   af164:	movzbl 0x3(%r9,%r14,1),%ecx
   af16a:	mov    %ecx,%esi
   af16c:	and    $0x7f,%ecx
   af16f:	shl    $0x15,%ecx
   af172:	or     %ecx,%ebx
   af174:	test   %sil,%sil
   af177:	js     af182 <meshoptimizer_rs::codec::raw+0x10d2>
   af179:	add    $0x4,%r14
   af17d:	mov    %r14,%rax
   af180:	jmp    af194 <meshoptimizer_rs::codec::raw+0x10e4>
   af182:	mov    0x30(%rsp),%rcx
   af187:	movzbl 0x4(%rcx,%r14,1),%ecx
   af18d:	shl    $0x1c,%ecx
   af190:	or     %ebx,%ecx
   af192:	mov    %ecx,%ebx
   af194:	mov    0x28(%rsp),%r9
   af199:	mov    %ebx,%ecx
   af19b:	shr    $1,%ecx
   af19d:	and    $0x1,%ebx
   af1a0:	neg    %ebx
   af1a2:	xor    %ecx,%ebx
   af1a4:	add    0x20(%rsp),%ebx
   af1a8:	mov    %rax,%r14
   af1ab:	mov    %ebx,0x20(%rsp)
   af1af:	cmp    $0xf,%r13b
   af1b3:	sete   %al
   af1b6:	cmp    $0xf,%r12b
   af1ba:	sete   %cl
   af1bd:	test   %r13b,%r13b
   af1c0:	sete   %sil
   af1c4:	test   %r12b,%r12b
   af1c7:	sete   %r12b
   af1cb:	mov    %r11d,0x100(%rsp,%r10,4)
   af1d3:	lea    0x1(%r10),%r13
   af1d7:	and    $0xf,%r13d
   af1db:	or     %cl,%r12b
   af1de:	mov    %r15d,0x100(%rsp,%r13,4)
   af1e6:	movzbl %r12b,%ecx
   af1ea:	add    %r10,%rcx
   af1ed:	inc    %rcx
   af1f0:	or     %al,%sil
   af1f3:	movzbl %sil,%r10d
   af1f7:	add    %rcx,%r10
   af1fa:	and    $0xf,%ecx
   af1fd:	mov    %ebx,0x100(%rsp,%rcx,4)
   af204:	mov    %r15d,0x200(%rsp,%r8,8)
   af20c:	mov    %r11d,0x204(%rsp,%r8,8)
   af214:	lea    0x1(%r8),%r13d
   af218:	and    $0xf,%r13d
   af21c:	mov    $0x3,%eax
   af221:	mov    $0x2,%r12d
   af227:	mov    %r14,%rsi
   af22a:	jmp    aeca2 <meshoptimizer_rs::codec::raw+0xbf2>
   af22f:	cmp    $0x21,%rbp
   af233:	mov    $0x20,%eax
   af238:	cmovae %rbp,%rax
   af23c:	mov    %rbp,%rcx
   af23f:	mov    %rax,0xa0(%rsp)
   af247:	lea    0x1(%rax),%rdx
   af24b:	mov    $0x7,%al
   af24d:	cmp    %rdx,%rbx
   af250:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af256:	mov    %sil,0x40(%rsp)
   af25b:	mov    %rbx,%rsi
   af25e:	sub    %rcx,%rsi
   af261:	lea    (%rsi,%rbp,1),%rax
   af265:	mov    %rbx,%rcx
   af268:	mov    %rax,0x68(%rsp)
   af26d:	sub    %rax,%rcx
   af270:	mov    %rcx,0xb0(%rsp)
   af278:	jb     afc99 <meshoptimizer_rs::codec::raw+0x1be9>
   af27e:	pxor   %xmm0,%xmm0
   af282:	movdqa %xmm0,0x1f0(%rsp)
   af28b:	movdqa %xmm0,0x1e0(%rsp)
   af294:	movdqa %xmm0,0x1d0(%rsp)
   af29d:	movdqa %xmm0,0x1c0(%rsp)
   af2a6:	movdqa %xmm0,0x1b0(%rsp)
   af2af:	movdqa %xmm0,0x1a0(%rsp)
   af2b8:	movdqa %xmm0,0x190(%rsp)
   af2c1:	movdqa %xmm0,0x180(%rsp)
   af2ca:	movdqa %xmm0,0x170(%rsp)
   af2d3:	movdqa %xmm0,0x160(%rsp)
   af2dc:	movdqa %xmm0,0x150(%rsp)
   af2e5:	movdqa %xmm0,0x140(%rsp)
   af2ee:	movdqa %xmm0,0x130(%rsp)
   af2f7:	movdqa %xmm0,0x120(%rsp)
   af300:	movdqa %xmm0,0x110(%rsp)
   af309:	movdqa %xmm0,0x100(%rsp)
   af312:	cmp    $0x101,%rbp
   af319:	jae    afc5a <meshoptimizer_rs::codec::raw+0x1baa>
   af31f:	lea    (%rsi,%rbp,1),%rax
   af323:	cmp    %rsi,%rax
   af326:	jb     afc71 <meshoptimizer_rs::codec::raw+0x1bc1>
   af32c:	add    %rdi,%rsi
   af32f:	lea    0x100(%rsp),%rdi
   af337:	mov    %rbp,%rdx
   af33a:	call   *0x79e18(%rip)        # 129158 <memcpy@GLIBC_2.14>
   af340:	test   %rbp,%rbp
   af343:	je     afd02 <meshoptimizer_rs::codec::raw+0x1c52>
   af349:	mov    $0x2000,%ax
   af34d:	xor    %edx,%edx
   af34f:	div    %bp
   af352:	and    $0x3ff0,%eax
   af357:	cmp    $0x100,%eax
   af35c:	mov    $0x100,%ecx
   af361:	cmovb  %eax,%ecx
   af364:	mov    %rcx,0xf0(%rsp)
   af36c:	lea    0x200(%rsp),%rdi
   af374:	mov    $0x400,%edx
   af379:	xor    %esi,%esi
   af37b:	call   *0x79de7(%rip)        # 129168 <memset@GLIBC_2.2.5>
   af381:	test   %r13,%r13
   af384:	je     afc08 <meshoptimizer_rs::codec::raw+0x1b58>
   af38a:	mov    %r12,0xe8(%rsp)
   af392:	add    %r14,0x68(%rsp)
   af397:	mov    %rbp,%rax
   af39a:	shr    $0x2,%rax
   af39e:	mov    %rax,0x98(%rsp)
   af3a6:	lea    0x3(%rbp),%rax
   af3aa:	shr    $0x2,%rax
   af3ae:	mov    %rax,0xf8(%rsp)
   af3b6:	mov    $0x1,%r12d
   af3bc:	xor    %edi,%edi
   af3be:	mov    %rbx,0x38(%rsp)
   af3c3:	mov    %r14,0x30(%rsp)
   af3c8:	mov    %r15,0xd0(%rsp)
   af3d0:	mov    %r13,%rcx
   af3d3:	sub    %rdi,%rcx
   af3d6:	mov    0xf0(%rsp),%rax
   af3de:	cmp    %rax,%rcx
   af3e1:	cmovae %rax,%rcx
   af3e5:	mov    %rcx,0x20(%rsp)
   af3ea:	cmpb   $0x0,0x40(%rsp)
   af3ef:	je     af42d <meshoptimizer_rs::codec::raw+0x137d>
   af3f1:	mov    %r12,%rcx
   af3f4:	mov    $0x7,%al
   af3f6:	add    0x98(%rsp),%rcx
   af3fe:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af404:	cmp    %rbx,%rcx
   af407:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af40d:	add    %r14,%r12
   af410:	mov    0x98(%rsp),%rax
   af418:	mov    %rax,0x90(%rsp)
   af420:	mov    %r12,0xb8(%rsp)
   af428:	mov    %rcx,%r12
   af42b:	jmp    af446 <meshoptimizer_rs::codec::raw+0x1396>
   af42d:	movq   $0x0,0x90(%rsp)
   af439:	mov    $0x1,%eax
   af43e:	mov    %rax,0xb8(%rsp)
   af446:	mov    0x20(%rsp),%rax
   af44b:	lea    (%rax,%rdi,1),%rsi
   af44f:	imul   %rbp,%rdi
   af453:	mov    %rsi,0xe0(%rsp)
   af45b:	imul   %rbp,%rsi
   af45f:	cmp    %rdi,%rsi
   af462:	jb     afc89 <meshoptimizer_rs::codec::raw+0x1bd9>
   af468:	cmp    %r15,%rsi
   af46b:	ja     afc89 <meshoptimizer_rs::codec::raw+0x1bd9>
   af471:	mov    0x20(%rsp),%rax
   af476:	lea    0xf(%rax),%ecx
   af479:	and    $0x3f0,%ecx
   af47f:	mov    %rcx,0x28(%rsp)
   af484:	mov    %rax,%rcx
   af487:	imul   %rbp,%rcx
   af48b:	mov    %rcx,0xc8(%rsp)
   af493:	add    0xe8(%rsp),%rdi
   af49b:	mov    %rdi,0xc0(%rsp)
   af4a3:	lea    -0x1(%rax),%rcx
   af4a7:	imul   %rbp,%rcx
   af4ab:	mov    %rcx,0xd8(%rsp)
   af4b3:	add    $0x4,%rcx
   af4b7:	mov    %rcx,0x58(%rsp)
   af4bc:	lea    0x0(,%rax,4),%rcx
   af4c4:	mov    %rcx,0x70(%rsp)
   af4c9:	lea    0x200(%rsp,%rax,1),%rcx
   af4d1:	mov    %rcx,0x88(%rsp)
   af4d9:	lea    0x200(%rsp,%rax,2),%rcx
   af4e1:	mov    %rcx,0x80(%rsp)
   af4e9:	lea    (%rax,%rax,2),%rax
   af4ed:	add    %rsp,%rax
   af4f0:	add    $0x200,%rax
   af4f6:	mov    %rax,0x78(%rsp)
   af4fb:	xor    %r15d,%r15d
   af4fe:	xor    %r14d,%r14d
   af501:	movzbl 0x40(%rsp),%edx
   af506:	jmp    af567 <meshoptimizer_rs::codec::raw+0x14b7>
   af508:	lea    (%rsp,%r14,1),%rcx
   af50c:	add    $0x100,%rcx
   af513:	shr    $0x4,%al
   af516:	neg    %al
   af518:	and    $0x1f,%al
   af51a:	movzbl %al,%eax
   af51d:	mov    %rcx,(%rsp)
   af521:	mov    %eax,0x8(%rsp)
   af525:	movq   $0x0,0x10(%rsp)
   af52e:	lea    0x200(%rsp),%rdi
   af536:	mov    0x70(%rsp),%rsi
   af53b:	mov    0x58(%rsp),%rcx
   af540:	mov    0x20(%rsp),%r8
   af545:	mov    %rbp,%r9
   af548:	call   72f30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   af54d:	inc    %r15
   af550:	add    $0x4,%r14
   af554:	cmp    %r15,0xf8(%rsp)
   af55c:	movzbl 0x40(%rsp),%edx
   af561:	je     afb83 <meshoptimizer_rs::codec::raw+0x1ad3>
   af567:	test   %dl,%dl
   af569:	mov    %r14,0x60(%rsp)
   af56e:	je     af58d <meshoptimizer_rs::codec::raw+0x14dd>
   af570:	cmp    0x90(%rsp),%r15
   af578:	jae    afcd2 <meshoptimizer_rs::codec::raw+0x1c22>
   af57e:	mov    0xb8(%rsp),%rax
   af586:	movzbl (%rax,%r15,1),%r14d
   af58b:	jmp    af590 <meshoptimizer_rs::codec::raw+0x14e0>
   af58d:	xor    %r14d,%r14d
   af590:	mov    %r14d,%eax
   af593:	and    $0x3,%al
   af595:	movzbl %al,%ecx
   af598:	mov    0x28(%rsp),%rsi
   af59d:	cmp    %rsi,0x20(%rsp)
   af5a2:	ja     af9d1 <meshoptimizer_rs::codec::raw+0x1921>
   af5a8:	cmp    $0x2,%al
   af5aa:	je     af5e9 <meshoptimizer_rs::codec::raw+0x1539>
   af5ac:	cmp    $0x3,%ecx
   af5af:	jne    af603 <meshoptimizer_rs::codec::raw+0x1553>
   af5b1:	mov    %r12,%rbx
   af5b4:	mov    $0x7,%al
   af5b6:	add    0x20(%rsp),%rbx
   af5bb:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af5c1:	cmp    0x38(%rsp),%rbx
   af5c6:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af5cc:	add    0x30(%rsp),%r12
   af5d1:	lea    0x200(%rsp),%rdi
   af5d9:	mov    %r12,%rsi
   af5dc:	mov    0x20(%rsp),%rdx
   af5e1:	call   *0x79b71(%rip)        # 129158 <memcpy@GLIBC_2.14>
   af5e7:	jmp    af65c <meshoptimizer_rs::codec::raw+0x15ac>
   af5e9:	lea    0x200(%rsp),%rdi
   af5f1:	xor    %esi,%esi
   af5f3:	mov    0x20(%rsp),%rdx
   af5f8:	call   *0x79b6a(%rip)        # 129168 <memset@GLIBC_2.2.5>
   af5fe:	mov    %r12,%rbx
   af601:	jmp    af65c <meshoptimizer_rs::codec::raw+0x15ac>
   af603:	test   %al,%al
   af605:	lea    -0xa19dc(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af60c:	lea    -0xa18c3(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af613:	cmove  %rcx,%rax
   af617:	test   %dl,%dl
   af619:	lea    -0xa1cf0(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af620:	cmove  %rcx,%rax
   af624:	mov    %rax,(%rsp)
   af628:	lea    0x48(%rsp),%rdi
   af62d:	mov    0x30(%rsp),%rsi
   af632:	mov    0x38(%rsp),%rdx
   af637:	mov    %r12,%rcx
   af63a:	lea    0x200(%rsp),%r8
   af642:	mov    0x28(%rsp),%r9
   af647:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   af64c:	cmpb   $0x0,0x48(%rsp)
   af651:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   af657:	mov    0x50(%rsp),%rbx
   af65c:	mov    %r14d,%eax
   af65f:	shr    $0x2,%al
   af662:	and    $0x3,%al
   af664:	cmp    $0x3,%al
   af666:	movzbl 0x40(%rsp),%edx
   af66b:	je     af68f <meshoptimizer_rs::codec::raw+0x15df>
   af66d:	movzbl %al,%ecx
   af670:	cmp    $0x2,%ecx
   af673:	jne    af6ca <meshoptimizer_rs::codec::raw+0x161a>
   af675:	mov    0x88(%rsp),%rdi
   af67d:	xor    %esi,%esi
   af67f:	mov    0x20(%rsp),%rdx
   af684:	call   *0x79ade(%rip)        # 129168 <memset@GLIBC_2.2.5>
   af68a:	jmp    af723 <meshoptimizer_rs::codec::raw+0x1673>
   af68f:	mov    %rbx,%r12
   af692:	mov    $0x7,%al
   af694:	add    0x20(%rsp),%r12
   af699:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af69f:	cmp    0x38(%rsp),%r12
   af6a4:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af6aa:	add    0x30(%rsp),%rbx
   af6af:	mov    0x88(%rsp),%rdi
   af6b7:	mov    %rbx,%rsi
   af6ba:	mov    0x20(%rsp),%rdx
   af6bf:	call   *0x79a93(%rip)        # 129158 <memcpy@GLIBC_2.14>
   af6c5:	mov    %r12,%rbx
   af6c8:	jmp    af723 <meshoptimizer_rs::codec::raw+0x1673>
   af6ca:	test   %al,%al
   af6cc:	lea    -0xa1aa3(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af6d3:	lea    -0xa198a(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af6da:	cmove  %rcx,%rax
   af6de:	test   %dl,%dl
   af6e0:	lea    -0xa1db7(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af6e7:	cmove  %rcx,%rax
   af6eb:	mov    %rax,(%rsp)
   af6ef:	lea    0x48(%rsp),%rdi
   af6f4:	mov    0x30(%rsp),%rsi
   af6f9:	mov    0x38(%rsp),%rdx
   af6fe:	mov    %rbx,%rcx
   af701:	mov    0x88(%rsp),%r8
   af709:	mov    0x28(%rsp),%r9
   af70e:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   af713:	cmpb   $0x0,0x48(%rsp)
   af718:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   af71e:	mov    0x50(%rsp),%rbx
   af723:	mov    %r14d,%eax
   af726:	shr    $0x4,%al
   af729:	and    $0x3,%al
   af72b:	cmp    $0x3,%al
   af72d:	je     af751 <meshoptimizer_rs::codec::raw+0x16a1>
   af72f:	movzbl %al,%ecx
   af732:	cmp    $0x2,%ecx
   af735:	jne    af78c <meshoptimizer_rs::codec::raw+0x16dc>
   af737:	mov    0x80(%rsp),%rdi
   af73f:	xor    %esi,%esi
   af741:	mov    0x20(%rsp),%rdx
   af746:	call   *0x79a1c(%rip)        # 129168 <memset@GLIBC_2.2.5>
   af74c:	jmp    af7e8 <meshoptimizer_rs::codec::raw+0x1738>
   af751:	mov    %rbx,%r12
   af754:	mov    $0x7,%al
   af756:	add    0x20(%rsp),%r12
   af75b:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af761:	cmp    0x38(%rsp),%r12
   af766:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af76c:	add    0x30(%rsp),%rbx
   af771:	mov    0x80(%rsp),%rdi
   af779:	mov    %rbx,%rsi
   af77c:	mov    0x20(%rsp),%rdx
   af781:	call   *0x799d1(%rip)        # 129158 <memcpy@GLIBC_2.14>
   af787:	mov    %r12,%rbx
   af78a:	jmp    af7e8 <meshoptimizer_rs::codec::raw+0x1738>
   af78c:	test   %al,%al
   af78e:	lea    -0xa1b65(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af795:	lea    -0xa1a4c(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af79c:	cmove  %rcx,%rax
   af7a0:	cmpb   $0x0,0x40(%rsp)
   af7a5:	lea    -0xa1e7c(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af7ac:	cmove  %rcx,%rax
   af7b0:	mov    %rax,(%rsp)
   af7b4:	lea    0x48(%rsp),%rdi
   af7b9:	mov    0x30(%rsp),%rsi
   af7be:	mov    0x38(%rsp),%rdx
   af7c3:	mov    %rbx,%rcx
   af7c6:	mov    0x80(%rsp),%r8
   af7ce:	mov    0x28(%rsp),%r9
   af7d3:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   af7d8:	cmpb   $0x0,0x48(%rsp)
   af7dd:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   af7e3:	mov    0x50(%rsp),%rbx
   af7e8:	shr    $0x6,%r14b
   af7ec:	cmp    $0x3,%r14b
   af7f0:	je     af815 <meshoptimizer_rs::codec::raw+0x1765>
   af7f2:	movzbl %r14b,%eax
   af7f6:	cmp    $0x2,%eax
   af7f9:	jne    af84a <meshoptimizer_rs::codec::raw+0x179a>
   af7fb:	mov    0x78(%rsp),%rdi
   af800:	xor    %esi,%esi
   af802:	mov    0x20(%rsp),%rdx
   af807:	call   *0x7995b(%rip)        # 129168 <memset@GLIBC_2.2.5>
   af80d:	mov    %rbx,%r12
   af810:	jmp    af8a4 <meshoptimizer_rs::codec::raw+0x17f4>
   af815:	mov    %rbx,%r12
   af818:	mov    $0x7,%al
   af81a:	add    0x20(%rsp),%r12
   af81f:	jb     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af825:	cmp    0x38(%rsp),%r12
   af82a:	ja     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   af830:	add    0x30(%rsp),%rbx
   af835:	mov    0x78(%rsp),%rdi
   af83a:	mov    %rbx,%rsi
   af83d:	mov    0x20(%rsp),%rdx
   af842:	call   *0x79910(%rip)        # 129158 <memcpy@GLIBC_2.14>
   af848:	jmp    af8a4 <meshoptimizer_rs::codec::raw+0x17f4>
   af84a:	test   %r14b,%r14b
   af84d:	lea    -0xa1c24(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af854:	lea    -0xa1b0b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af85b:	cmove  %rcx,%rax
   af85f:	cmpb   $0x0,0x40(%rsp)
   af864:	lea    -0xa1f3b(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af86b:	cmove  %rcx,%rax
   af86f:	mov    %rax,(%rsp)
   af873:	lea    0x48(%rsp),%rdi
   af878:	mov    0x30(%rsp),%rsi
   af87d:	mov    0x38(%rsp),%rdx
   af882:	mov    %rbx,%rcx
   af885:	mov    0x78(%rsp),%r8
   af88a:	mov    0x28(%rsp),%r9
   af88f:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   af894:	cmpb   $0x0,0x48(%rsp)
   af899:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   af89f:	mov    0x50(%rsp),%r12
   af8a4:	cmpb   $0x0,0x40(%rsp)
   af8a9:	mov    0xc8(%rsp),%rsi
   af8b1:	mov    0x60(%rsp),%r14
   af8b6:	je     af8d2 <meshoptimizer_rs::codec::raw+0x1822>
   af8b8:	cmp    0xb0(%rsp),%r15
   af8c0:	jae    afcea <meshoptimizer_rs::codec::raw+0x1c3a>
   af8c6:	mov    0x68(%rsp),%rax
   af8cb:	movzbl (%rax,%r15,1),%eax
   af8d0:	jmp    af8d4 <meshoptimizer_rs::codec::raw+0x1824>
   af8d2:	xor    %eax,%eax
   af8d4:	mov    %rsi,%rdx
   af8d7:	sub    %r14,%rdx
   af8da:	jb     afc47 <meshoptimizer_rs::codec::raw+0x1b97>
   af8e0:	cmp    %rdx,0x58(%rsp)
   af8e5:	ja     afc33 <meshoptimizer_rs::codec::raw+0x1b83>
   af8eb:	mov    0xc0(%rsp),%rcx
   af8f3:	lea    (%rcx,%r14,1),%rdx
   af8f7:	mov    %eax,%ecx
   af8f9:	and    $0x3,%cl
   af8fc:	movzbl %cl,%ecx
   af8ff:	lea    -0x9f03e(%rip),%rsi        # 108c8 <libm::math::generic::sqrt::RSQRT_TAB+0x4e2>
   af906:	movslq (%rsi,%rcx,4),%rcx
   af90a:	add    %rsi,%rcx
   af90d:	jmp    *%rcx
   af90f:	mov    %rdx,0xa8(%rsp)
   af917:	mov    %r14,%rbx
   af91a:	add    %rsp,%r14
   af91d:	add    $0x100,%r14
   af924:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   af929:	test   %al,%al
   af92b:	je     af995 <meshoptimizer_rs::codec::raw+0x18e5>
   af92d:	mov    %r14,(%rsp)
   af931:	lea    0x200(%rsp),%rdi
   af939:	mov    0x70(%rsp),%rsi
   af93e:	mov    0xa8(%rsp),%rdx
   af946:	mov    0x58(%rsp),%rcx
   af94b:	mov    0x20(%rsp),%r8
   af950:	mov    %rbp,%r9
   af953:	call   75cc0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   af958:	jmp    af9c9 <meshoptimizer_rs::codec::raw+0x1919>
   af95a:	lea    (%rsp,%r14,1),%rax
   af95e:	add    $0x100,%rax
   af964:	mov    %rax,(%rsp)
   af968:	movq   $0x0,0x8(%rsp)
   af971:	lea    0x200(%rsp),%rdi
   af979:	mov    0x70(%rsp),%rsi
   af97e:	mov    0x58(%rsp),%rcx
   af983:	mov    0x20(%rsp),%r8
   af988:	mov    %rbp,%r9
   af98b:	call   72d70 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   af990:	jmp    af54d <meshoptimizer_rs::codec::raw+0x149d>
   af995:	mov    %r14,(%rsp)
   af999:	movq   $0x0,0x8(%rsp)
   af9a2:	lea    0x200(%rsp),%rdi
   af9aa:	mov    0x70(%rsp),%rsi
   af9af:	mov    0xa8(%rsp),%rdx
   af9b7:	mov    0x58(%rsp),%rcx
   af9bc:	mov    0x20(%rsp),%r8
   af9c1:	mov    %rbp,%r9
   af9c4:	call   72b80 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   af9c9:	mov    %rbx,%r14
   af9cc:	jmp    af54d <meshoptimizer_rs::codec::raw+0x149d>
   af9d1:	cmp    $0x2,%al
   af9d3:	je     afd0f <meshoptimizer_rs::codec::raw+0x1c5f>
   af9d9:	cmp    $0x3,%ecx
   af9dc:	je     afd28 <meshoptimizer_rs::codec::raw+0x1c78>
   af9e2:	test   %al,%al
   af9e4:	lea    -0xa1dbb(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af9eb:	lea    -0xa1ca2(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af9f2:	cmove  %rcx,%rax
   af9f6:	test   %dl,%dl
   af9f8:	lea    -0xa20cf(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af9ff:	cmove  %rcx,%rax
   afa03:	mov    %rax,(%rsp)
   afa07:	lea    0x48(%rsp),%rdi
   afa0c:	mov    0x30(%rsp),%rsi
   afa11:	mov    0x38(%rsp),%rdx
   afa16:	mov    %r12,%rcx
   afa19:	lea    0x200(%rsp),%r8
   afa21:	mov    0x28(%rsp),%r9
   afa26:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   afa2b:	cmpb   $0x0,0x48(%rsp)
   afa30:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   afa36:	mov    %r14d,%eax
   afa39:	shr    $0x2,%al
   afa3c:	and    $0x3,%al
   afa3e:	cmp    $0x2,%al
   afa40:	movzbl 0x40(%rsp),%edx
   afa45:	je     afd0f <meshoptimizer_rs::codec::raw+0x1c5f>
   afa4b:	movzbl %al,%ecx
   afa4e:	cmp    $0x3,%ecx
   afa51:	je     afd28 <meshoptimizer_rs::codec::raw+0x1c78>
   afa57:	test   %al,%al
   afa59:	lea    -0xa1e30(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   afa60:	lea    -0xa1d17(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   afa67:	cmove  %rcx,%rax
   afa6b:	mov    0x50(%rsp),%rcx
   afa70:	test   %dl,%dl
   afa72:	lea    -0xa2149(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   afa79:	cmove  %rdx,%rax
   afa7d:	mov    %rax,(%rsp)
   afa81:	lea    0x48(%rsp),%rdi
   afa86:	mov    0x30(%rsp),%rsi
   afa8b:	mov    0x38(%rsp),%rdx
   afa90:	mov    0x88(%rsp),%r8
   afa98:	mov    0x28(%rsp),%r9
   afa9d:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   afaa2:	cmpb   $0x0,0x48(%rsp)
   afaa7:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   afaad:	mov    %r14d,%eax
   afab0:	shr    $0x4,%al
   afab3:	and    $0x3,%al
   afab5:	cmp    $0x2,%al
   afab7:	movzbl 0x40(%rsp),%edx
   afabc:	je     afd0f <meshoptimizer_rs::codec::raw+0x1c5f>
   afac2:	movzbl %al,%ecx
   afac5:	cmp    $0x3,%ecx
   afac8:	je     afd28 <meshoptimizer_rs::codec::raw+0x1c78>
   aface:	test   %al,%al
   afad0:	lea    -0xa1ea7(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   afad7:	lea    -0xa1d8e(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   afade:	cmove  %rcx,%rax
   afae2:	mov    0x50(%rsp),%rcx
   afae7:	test   %dl,%dl
   afae9:	lea    -0xa21c0(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   afaf0:	cmove  %rdx,%rax
   afaf4:	mov    %rax,(%rsp)
   afaf8:	lea    0x48(%rsp),%rdi
   afafd:	mov    0x30(%rsp),%rsi
   afb02:	mov    0x38(%rsp),%rdx
   afb07:	mov    0x80(%rsp),%r8
   afb0f:	mov    0x28(%rsp),%r9
   afb14:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   afb19:	cmpb   $0x0,0x48(%rsp)
   afb1e:	jne    afc2c <meshoptimizer_rs::codec::raw+0x1b7c>
   afb24:	shr    $0x6,%r14b
   afb28:	cmp    $0x2,%r14b
   afb2c:	movzbl 0x40(%rsp),%edx
   afb31:	je     afd0f <meshoptimizer_rs::codec::raw+0x1c5f>
   afb37:	movzbl %r14b,%eax
   afb3b:	cmp    $0x3,%eax
   afb3e:	je     afd28 <meshoptimizer_rs::codec::raw+0x1c78>
   afb44:	test   %r14b,%r14b
   afb47:	lea    -0xa1f1e(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   afb4e:	lea    -0xa1e05(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   afb55:	cmove  %rcx,%rax
   afb59:	mov    0x50(%rsp),%rcx
   afb5e:	test   %dl,%dl
   afb60:	lea    -0xa2237(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   afb67:	cmove  %rdx,%rax
   afb6b:	mov    %rax,(%rsp)
   afb6f:	lea    0x48(%rsp),%rdi
   afb74:	mov    0x30(%rsp),%rsi
   afb79:	mov    0x38(%rsp),%rdx
   afb7e:	jmp    af885 <meshoptimizer_rs::codec::raw+0x17d5>
   afb83:	mov    0xc8(%rsp),%rdx
   afb8b:	mov    0xd8(%rsp),%rdi
   afb93:	cmp    %rdx,%rdi
   afb96:	mov    0x30(%rsp),%r14
   afb9b:	ja     afcb1 <meshoptimizer_rs::codec::raw+0x1c01>
   afba1:	mov    0xc0(%rsp),%rsi
   afba9:	add    %rdi,%rsi
   afbac:	lea    0x100(%rsp),%rdi
   afbb4:	mov    %rbp,%rdx
   afbb7:	call   *0x7959b(%rip)        # 129158 <memcpy@GLIBC_2.14>
   afbbd:	mov    0xe0(%rsp),%rax
   afbc5:	mov    %rax,%rdi
   afbc8:	cmp    %r13,%rax
   afbcb:	mov    0x38(%rsp),%rbx
   afbd0:	mov    0xd0(%rsp),%r15
   afbd8:	mov    0xa0(%rsp),%rax
   afbe0:	jb     af3d0 <meshoptimizer_rs::codec::raw+0x1320>
   afbe6:	jmp    afc16 <meshoptimizer_rs::codec::raw+0x1b66>
   afbe8:	mov    0x28(%rsp),%r9
   afbed:	mov    $0xff,%al
   afbef:	cmp    %r9,%rsi
   afbf2:	je     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   afbf4:	mov    $0x7,%al
   afbf6:	add    $0x608,%rsp
   afbfd:	pop    %rbx
   afbfe:	pop    %r12
   afc00:	pop    %r13
   afc02:	pop    %r14
   afc04:	pop    %r15
   afc06:	pop    %rbp
   afc07:	ret
   afc08:	mov    $0x1,%r12d
   afc0e:	mov    0xa0(%rsp),%rax
   afc16:	sub    %r12,%rbx
   afc19:	setae  %cl
   afc1c:	cmp    %rax,%rbx
   afc1f:	sete   %dl
   afc22:	mov    $0x7,%al
   afc24:	test   %dl,%cl
   afc26:	je     afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   afc28:	mov    $0xff,%al
   afc2a:	jmp    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   afc2c:	movzbl 0x49(%rsp),%eax
   afc31:	jmp    afbf6 <meshoptimizer_rs::codec::raw+0x1b46>
   afc33:	lea    0x7417e(%rip),%rcx        # 123db8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   afc3a:	xor    %edi,%edi
   afc3c:	mov    0x58(%rsp),%rsi
   afc41:	call   *0x79669(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afc47:	lea    0x74182(%rip),%rcx        # 123dd0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   afc4e:	mov    %rsi,%rdx
   afc51:	mov    %r14,%rdi
   afc54:	call   *0x79656(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afc5a:	lea    0x741cf(%rip),%rcx        # 123e30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   afc61:	mov    $0x100,%edx
   afc66:	xor    %edi,%edi
   afc68:	mov    %rbp,%rsi
   afc6b:	call   *0x7963f(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afc71:	lea    0x741a0(%rip),%rcx        # 123e18 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   afc78:	mov    %rsi,%rdi
   afc7b:	mov    0x68(%rsp),%rsi
   afc80:	mov    %rbx,%rdx
   afc83:	call   *0x79627(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afc89:	lea    0x779b0(%rip),%rcx        # 127640 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a58>
   afc90:	mov    %r15,%rdx
   afc93:	call   *0x79617(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afc99:	lea    0x741a8(%rip),%rcx        # 123e48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   afca0:	mov    0x68(%rsp),%rdi
   afca5:	mov    %rbx,%rsi
   afca8:	mov    %rbx,%rdx
   afcab:	call   *0x795ff(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afcb1:	lea    0x740b8(%rip),%rcx        # 123d70 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   afcb8:	mov    %rdx,%rsi
   afcbb:	call   *0x795ef(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afcc1:	lea    0x77168(%rip),%rdx        # 126e30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5248>
   afcc8:	xor    %edi,%edi
   afcca:	xor    %esi,%esi
   afccc:	call   *0x794be(%rip)        # 129190 <_DYNAMIC+0x250>
   afcd2:	lea    0x740af(%rip),%rdx        # 123d88 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   afcd9:	mov    %r15,%rdi
   afcdc:	mov    0x90(%rsp),%rsi
   afce4:	call   *0x794a6(%rip)        # 129190 <_DYNAMIC+0x250>
   afcea:	lea    0x740af(%rip),%rdx        # 123da0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   afcf1:	mov    %r15,%rdi
   afcf4:	mov    0xb0(%rsp),%rsi
   afcfc:	call   *0x7948e(%rip)        # 129190 <_DYNAMIC+0x250>
   afd02:	lea    0x7404f(%rip),%rdi        # 123d58 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   afd09:	call   *0x79669(%rip)        # 129378 <_DYNAMIC+0x438>
   afd0f:	lea    0x740ea(%rip),%rcx        # 123e00 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   afd16:	xor    %edi,%edi
   afd18:	mov    0x20(%rsp),%rsi
   afd1d:	mov    0x28(%rsp),%rdx
   afd22:	call   *0x79588(%rip)        # 1292b0 <_DYNAMIC+0x370>
   afd28:	lea    0x740b9(%rip),%rcx        # 123de8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   afd2f:	xor    %edi,%edi
   afd31:	mov    0x20(%rsp),%rsi
   afd36:	mov    0x28(%rsp),%rdx
   afd3b:	call   *0x7956f(%rip)        # 1292b0 <_DYNAMIC+0x370>
