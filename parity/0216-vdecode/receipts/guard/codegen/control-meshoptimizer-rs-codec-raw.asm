
<artifacts>/guard/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000b91d0 <meshoptimizer_rs::codec::raw>:
   b91d0:	push   %rbp
   b91d1:	push   %r15
   b91d3:	push   %r14
   b91d5:	push   %r13
   b91d7:	push   %r12
   b91d9:	push   %rbx
   b91da:	sub    $0x5f8,%rsp
   b91e1:	mov    %r9,%r14
   b91e4:	mov    %r8,%rbp
   b91e7:	mov    %rcx,%r13
   b91ea:	mov    %rdx,%r12
   b91ed:	mov    %rsi,%r15
   b91f0:	mov    0x630(%rsp),%rbx
   b91f8:	test   %dil,%dil
   b91fb:	je     b9364 <meshoptimizer_rs::codec::raw+0x194>
   b9201:	movzbl %dil,%eax
   b9205:	cmp    $0x1,%eax
   b9208:	jne    b9426 <meshoptimizer_rs::codec::raw+0x256>
   b920e:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   b9218:	mov    %r13,%rax
   b921b:	mul    %rdi
   b921e:	shr    $1,%rdx
   b9221:	lea    (%rdx,%rdx,2),%rcx
   b9225:	mov    %r13,%rax
   b9228:	sub    %rcx,%rax
   b922b:	cmp    $0x2,%rbp
   b922f:	jne    b9491 <meshoptimizer_rs::codec::raw+0x2c1>
   b9235:	test   %rax,%rax
   b9238:	jne    b949a <meshoptimizer_rs::codec::raw+0x2ca>
   b923e:	mov    %r13,%rax
   b9241:	mul    %rdi
   b9244:	shr    $1,%rdx
   b9247:	lea    0x11(%rdx),%rcx
   b924b:	mov    $0x7,%al
   b924d:	cmp    %rcx,%rbx
   b9250:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9256:	movzbl (%r14),%ecx
   b925a:	mov    %ecx,%esi
   b925c:	and    $0xf0,%sil
   b9260:	cmp    $0xe0,%sil
   b9264:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b926a:	and    $0xf,%cl
   b926d:	mov    $0x5,%al
   b926f:	cmp    $0x1,%cl
   b9272:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9278:	lea    0x1(%rdx),%rsi
   b927c:	pcmpeqd %xmm0,%xmm0
   b9280:	movdqa %xmm0,0x260(%rsp)
   b9289:	movdqa %xmm0,0x250(%rsp)
   b9292:	movdqa %xmm0,0x240(%rsp)
   b929b:	movdqa %xmm0,0x230(%rsp)
   b92a4:	movdqa %xmm0,0x220(%rsp)
   b92ad:	movdqa %xmm0,0x210(%rsp)
   b92b6:	movdqa %xmm0,0x200(%rsp)
   b92bf:	movdqa %xmm0,0x1f0(%rsp)
   b92c8:	movdqa %xmm0,0x120(%rsp)
   b92d1:	movdqa %xmm0,0x110(%rsp)
   b92da:	movdqa %xmm0,0x100(%rsp)
   b92e3:	movdqa %xmm0,0xf0(%rsp)
   b92ec:	xor    %r8d,%r8d
   b92ef:	cmp    $0x1,%cl
   b92f2:	setne  %r9b
   b92f6:	lea    -0x10(%rbx),%rax
   b92fa:	mov    %rax,0x28(%rsp)
   b92ff:	mov    %r12,%rax
   b9302:	mov    %rdx,%rcx
   b9305:	mul    %rdi
   b9308:	shr    $0x2,%rdx
   b930c:	cmp    %rcx,%rdx
   b930f:	cmovae %rcx,%rdx
   b9313:	mov    %rdx,0x38(%rsp)
   b9318:	test   %rdx,%rdx
   b931b:	je     bad1f <meshoptimizer_rs::codec::raw+0x1b4f>
   b9321:	mov    %r9b,%r8b
   b9324:	lea    0xd(,%r8,2),%rax
   b932c:	mov    %rax,0x58(%rsp)
   b9331:	mov    %r14,0x30(%rsp)
   b9336:	lea    (%r14,%rbx,1),%rax
   b933a:	add    $0xfffffffffffffff0,%rax
   b933e:	mov    %rax,0x50(%rsp)
   b9343:	add    $0x4,%r15
   b9347:	xor    %r8d,%r8d
   b934a:	xor    %r10d,%r10d
   b934d:	movl   $0x0,0x20(%rsp)
   b9355:	xor    %r11d,%r11d
   b9358:	xor    %edi,%edi
   b935a:	mov    0x28(%rsp),%r9
   b935f:	jmp    b9826 <meshoptimizer_rs::codec::raw+0x656>
   b9364:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   b9369:	cmp    $0x2,%al
   b936b:	jb     b93d4 <meshoptimizer_rs::codec::raw+0x204>
   b936d:	cmp    $0x20,%rbp
   b9371:	lea    -0x36058(%rip),%rax        # 83320 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   b9378:	lea    -0x3753f(%rip),%rcx        # 81e40 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   b937f:	cmove  %rax,%rcx
   b9383:	lea    -0x3061a(%rip),%rax        # 88d70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   b938a:	lea    -0x31ec1(%rip),%rdx        # 874d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   b9391:	cmove  %rax,%rdx
   b9395:	cmp    $0x4,%rbp
   b9399:	lea    -0x339f0(%rip),%rax        # 859b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   b93a0:	cmovne %rcx,%rax
   b93a4:	lea    -0x2db9b(%rip),%r10        # 8b810 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   b93ab:	cmovne %rdx,%r10
   b93af:	cmp    $0x21,%r13
   b93b3:	cmovae %rax,%r10
   b93b7:	mov    %r15,%rdi
   b93ba:	mov    %r12,%rsi
   b93bd:	mov    %r13,%rdx
   b93c0:	mov    %rbp,%rcx
   b93c3:	mov    %r14,%r8
   b93c6:	mov    %rbx,%r9
   b93c9:	call   *%r10
   b93cc:	cmp    $0xfe,%al
   b93ce:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b93d4:	mov    $0x7,%al
   b93d6:	test   %rbx,%rbx
   b93d9:	je     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b93df:	movzbl (%r14),%edi
   b93e3:	mov    %edi,%ecx
   b93e5:	and    $0xf0,%cl
   b93e8:	cmp    $0xa0,%cl
   b93eb:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b93f1:	and    $0xf,%dil
   b93f5:	mov    $0x5,%al
   b93f7:	cmp    $0x1,%dil
   b93fb:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9401:	test   %dil,%dil
   b9404:	je     ba323 <meshoptimizer_rs::codec::raw+0x1153>
   b940a:	mov    %rbp,%rcx
   b940d:	shr    $0x2,%rcx
   b9411:	add    %rbp,%rcx
   b9414:	cmp    $0x19,%rcx
   b9418:	mov    $0x18,%eax
   b941d:	cmovae %rcx,%rax
   b9421:	jmp    ba333 <meshoptimizer_rs::codec::raw+0x1163>
   b9426:	add    $0x5,%r13
   b942a:	mov    $0x7,%al
   b942c:	cmp    $0x2,%rbp
   b9430:	jne    b94a1 <meshoptimizer_rs::codec::raw+0x2d1>
   b9432:	cmp    %r13,%rbx
   b9435:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b943b:	test   %rbx,%rbx
   b943e:	je     badd4 <meshoptimizer_rs::codec::raw+0x1c04>
   b9444:	movzbl (%r14),%ecx
   b9448:	mov    %ecx,%edx
   b944a:	and    $0xf0,%dl
   b944d:	cmp    $0xd0,%dl
   b9450:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9456:	mov    $0x5,%al
   b9458:	test   $0xe,%cl
   b945b:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9461:	dec    %rbx
   b9464:	movq   $0x0,0x1f0(%rsp)
   b9470:	movabs $0x7ffffffffffffffc,%rax
   b947a:	or     $0x2,%rax
   b947e:	and    %rax,%r12
   b9481:	je     b9784 <meshoptimizer_rs::codec::raw+0x5b4>
   b9487:	inc    %r14
   b948a:	xor    %eax,%eax
   b948c:	jmp    b9536 <meshoptimizer_rs::codec::raw+0x366>
   b9491:	test   %rax,%rax
   b9494:	je     b9667 <meshoptimizer_rs::codec::raw+0x497>
   b949a:	xor    %eax,%eax
   b949c:	jmp    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b94a1:	cmp    %r13,%rbx
   b94a4:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b94aa:	test   %rbx,%rbx
   b94ad:	je     badd4 <meshoptimizer_rs::codec::raw+0x1c04>
   b94b3:	movzbl (%r14),%ecx
   b94b7:	mov    %ecx,%edx
   b94b9:	and    $0xf0,%dl
   b94bc:	cmp    $0xd0,%dl
   b94bf:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b94c5:	mov    $0x5,%al
   b94c7:	test   $0xe,%cl
   b94ca:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b94d0:	dec    %rbx
   b94d3:	movq   $0x0,0x1f0(%rsp)
   b94df:	movabs $0x7ffffffffffffffc,%rax
   b94e9:	and    %rax,%r12
   b94ec:	je     b9784 <meshoptimizer_rs::codec::raw+0x5b4>
   b94f2:	inc    %r14
   b94f5:	xor    %eax,%eax
   b94f7:	jmp    b95eb <meshoptimizer_rs::codec::raw+0x41b>
   b94fc:	dec    %rbx
   b94ff:	inc    %r14
   b9502:	mov    %edx,%ecx
   b9504:	mov    %ecx,%edx
   b9506:	shr    $0x2,%edx
   b9509:	mov    %ecx,%esi
   b950b:	shl    $0x1e,%esi
   b950e:	sar    $0x1f,%esi
   b9511:	xor    %edx,%esi
   b9513:	and    $0x1,%ecx
   b9516:	add    0x1f0(%rsp,%rcx,4),%esi
   b951d:	mov    %esi,0x1f0(%rsp,%rcx,4)
   b9524:	mov    %si,(%r15,%rax,1)
   b9529:	add    $0x2,%rax
   b952d:	cmp    %rax,%r12
   b9530:	je     b9784 <meshoptimizer_rs::codec::raw+0x5b4>
   b9536:	cmp    $0x5,%rbx
   b953a:	jb     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9540:	movzbl (%r14),%edx
   b9544:	test   %dl,%dl
   b9546:	jns    b94fc <meshoptimizer_rs::codec::raw+0x32c>
   b9548:	and    $0x7f,%edx
   b954b:	movzbl 0x1(%r14),%ecx
   b9550:	mov    %ecx,%esi
   b9552:	and    $0x7f,%ecx
   b9555:	shl    $0x7,%ecx
   b9558:	or     %edx,%ecx
   b955a:	mov    $0x2,%edx
   b955f:	test   %sil,%sil
   b9562:	jns    b95a7 <meshoptimizer_rs::codec::raw+0x3d7>
   b9564:	movzbl 0x2(%r14),%edx
   b9569:	mov    %edx,%esi
   b956b:	and    $0x7f,%edx
   b956e:	shl    $0xe,%edx
   b9571:	or     %edx,%ecx
   b9573:	mov    $0x3,%edx
   b9578:	test   %sil,%sil
   b957b:	jns    b95a7 <meshoptimizer_rs::codec::raw+0x3d7>
   b957d:	movzbl 0x3(%r14),%edx
   b9582:	mov    %edx,%esi
   b9584:	and    $0x7f,%edx
   b9587:	shl    $0x15,%edx
   b958a:	or     %edx,%ecx
   b958c:	mov    $0x4,%edx
   b9591:	test   %sil,%sil
   b9594:	jns    b95a7 <meshoptimizer_rs::codec::raw+0x3d7>
   b9596:	movzbl 0x4(%r14),%esi
   b959b:	shl    $0x1c,%esi
   b959e:	or     %ecx,%esi
   b95a0:	mov    $0x5,%edx
   b95a5:	mov    %esi,%ecx
   b95a7:	sub    %rdx,%rbx
   b95aa:	add    %rdx,%r14
   b95ad:	jmp    b9504 <meshoptimizer_rs::codec::raw+0x334>
   b95b2:	dec    %rbx
   b95b5:	inc    %r14
   b95b8:	mov    %edx,%ecx
   b95ba:	mov    %ecx,%edx
   b95bc:	shr    $0x2,%edx
   b95bf:	mov    %ecx,%esi
   b95c1:	shl    $0x1e,%esi
   b95c4:	sar    $0x1f,%esi
   b95c7:	xor    %edx,%esi
   b95c9:	and    $0x1,%ecx
   b95cc:	add    0x1f0(%rsp,%rcx,4),%esi
   b95d3:	mov    %esi,0x1f0(%rsp,%rcx,4)
   b95da:	mov    %esi,(%r15,%rax,1)
   b95de:	add    $0x4,%rax
   b95e2:	cmp    %rax,%r12
   b95e5:	je     b9784 <meshoptimizer_rs::codec::raw+0x5b4>
   b95eb:	cmp    $0x5,%rbx
   b95ef:	jb     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b95f5:	movzbl (%r14),%edx
   b95f9:	test   %dl,%dl
   b95fb:	jns    b95b2 <meshoptimizer_rs::codec::raw+0x3e2>
   b95fd:	and    $0x7f,%edx
   b9600:	movzbl 0x1(%r14),%ecx
   b9605:	mov    %ecx,%esi
   b9607:	and    $0x7f,%ecx
   b960a:	shl    $0x7,%ecx
   b960d:	or     %edx,%ecx
   b960f:	mov    $0x2,%edx
   b9614:	test   %sil,%sil
   b9617:	jns    b965c <meshoptimizer_rs::codec::raw+0x48c>
   b9619:	movzbl 0x2(%r14),%edx
   b961e:	mov    %edx,%esi
   b9620:	and    $0x7f,%edx
   b9623:	shl    $0xe,%edx
   b9626:	or     %edx,%ecx
   b9628:	mov    $0x3,%edx
   b962d:	test   %sil,%sil
   b9630:	jns    b965c <meshoptimizer_rs::codec::raw+0x48c>
   b9632:	movzbl 0x3(%r14),%edx
   b9637:	mov    %edx,%esi
   b9639:	and    $0x7f,%edx
   b963c:	shl    $0x15,%edx
   b963f:	or     %edx,%ecx
   b9641:	mov    $0x4,%edx
   b9646:	test   %sil,%sil
   b9649:	jns    b965c <meshoptimizer_rs::codec::raw+0x48c>
   b964b:	movzbl 0x4(%r14),%esi
   b9650:	shl    $0x1c,%esi
   b9653:	or     %ecx,%esi
   b9655:	mov    $0x5,%edx
   b965a:	mov    %esi,%ecx
   b965c:	sub    %rdx,%rbx
   b965f:	add    %rdx,%r14
   b9662:	jmp    b95ba <meshoptimizer_rs::codec::raw+0x3ea>
   b9667:	lea    0x11(%rdx),%rcx
   b966b:	mov    $0x7,%al
   b966d:	cmp    %rcx,%rbx
   b9670:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9676:	movzbl (%r14),%ecx
   b967a:	mov    %ecx,%esi
   b967c:	and    $0xf0,%sil
   b9680:	cmp    $0xe0,%sil
   b9684:	jne    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b968a:	and    $0xf,%cl
   b968d:	mov    $0x5,%al
   b968f:	cmp    $0x1,%cl
   b9692:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   b9698:	lea    0x1(%rdx),%rsi
   b969c:	pcmpeqd %xmm0,%xmm0
   b96a0:	movdqa %xmm0,0x260(%rsp)
   b96a9:	movdqa %xmm0,0x250(%rsp)
   b96b2:	movdqa %xmm0,0x240(%rsp)
   b96bb:	movdqa %xmm0,0x230(%rsp)
   b96c4:	movdqa %xmm0,0x220(%rsp)
   b96cd:	movdqa %xmm0,0x210(%rsp)
   b96d6:	movdqa %xmm0,0x200(%rsp)
   b96df:	movdqa %xmm0,0x1f0(%rsp)
   b96e8:	movdqa %xmm0,0x120(%rsp)
   b96f1:	movdqa %xmm0,0x110(%rsp)
   b96fa:	movdqa %xmm0,0x100(%rsp)
   b9703:	movdqa %xmm0,0xf0(%rsp)
   b970c:	xor    %r8d,%r8d
   b970f:	cmp    $0x1,%cl
   b9712:	setne  %r9b
   b9716:	lea    -0x10(%rbx),%rax
   b971a:	mov    %rax,0x28(%rsp)
   b971f:	mov    %r12,%rax
   b9722:	mov    %rdx,%rcx
   b9725:	mul    %rdi
   b9728:	shr    $0x3,%rdx
   b972c:	cmp    %rcx,%rdx
   b972f:	cmovae %rcx,%rdx
   b9733:	mov    %rdx,0x38(%rsp)
   b9738:	test   %rdx,%rdx
   b973b:	je     bad1f <meshoptimizer_rs::codec::raw+0x1b4f>
   b9741:	mov    %r9b,%r8b
   b9744:	lea    0xd(,%r8,2),%rax
   b974c:	mov    %rax,0x58(%rsp)
   b9751:	mov    %r14,0x30(%rsp)
   b9756:	lea    (%r14,%rbx,1),%rax
   b975a:	add    $0xfffffffffffffff0,%rax
   b975e:	mov    %rax,0x50(%rsp)
   b9763:	add    $0x8,%r15
   b9767:	xor    %r8d,%r8d
   b976a:	xor    %r10d,%r10d
   b976d:	movl   $0x0,0x20(%rsp)
   b9775:	xor    %r11d,%r11d
   b9778:	xor    %edi,%edi
   b977a:	mov    0x28(%rsp),%r9
   b977f:	jmp    b9dee <meshoptimizer_rs::codec::raw+0xc1e>
   b9784:	cmp    $0x4,%rbx
   b9788:	sete   %al
   b978b:	jmp    bad19 <meshoptimizer_rs::codec::raw+0x1b49>
   b9790:	mov    %bpl,%r14b
   b9793:	add    %r14d,%r11d
   b9796:	mov    %ebx,0xf0(%rsp,%r10,4)
   b979e:	mov    %r11d,%ebp
   b97a1:	not    %eax
   b97a3:	shr    $0x4,%eax
   b97a6:	add    %r8d,%eax
   b97a9:	and    $0xf,%eax
   b97ac:	mov    0x1f0(%rsp,%rax,8),%r11d
   b97b4:	mov    0x1f4(%rsp,%rax,8),%r15d
   b97bc:	add    %r14,%r10
   b97bf:	mov    $0x2,%eax
   b97c4:	mov    $0x1,%r12d
   b97ca:	mov    %r8,%r13
   b97cd:	inc    %rdi
   b97d0:	mov    %ebx,0x1f0(%rsp,%r13,8)
   b97d8:	mov    %r15d,0x1f4(%rsp,%r13,8)
   b97e0:	add    %r8d,%r12d
   b97e3:	and    $0xf,%r12d
   b97e7:	mov    %r11d,0x1f0(%rsp,%r12,8)
   b97ef:	mov    %ebx,0x1f4(%rsp,%r12,8)
   b97f7:	add    %r8d,%eax
   b97fa:	and    $0xf,%eax
   b97fd:	and    $0xf,%r10d
   b9801:	mov    %r11w,-0x4(%rdx)
   b9806:	mov    %r15w,-0x2(%rdx)
   b980b:	mov    %rdx,%r15
   b980e:	mov    %bx,(%rdx)
   b9811:	add    $0x6,%r15
   b9815:	mov    %rax,%r8
   b9818:	mov    %ebp,%r11d
   b981b:	cmp    %rdi,0x38(%rsp)
   b9820:	je     bad24 <meshoptimizer_rs::codec::raw+0x1b54>
   b9826:	cmp    %r9,%rsi
   b9829:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b982f:	mov    %r15,%rdx
   b9832:	mov    0x30(%rsp),%r14
   b9837:	movzbl 0x1(%r14,%rdi,1),%eax
   b983d:	cmp    $0xf0,%al
   b983f:	jae    b9879 <meshoptimizer_rs::codec::raw+0x6a9>
   b9841:	mov    %eax,%r15d
   b9844:	and    $0xf,%r15d
   b9848:	cmp    %r15d,0x58(%rsp)
   b984d:	jbe    b98b3 <meshoptimizer_rs::codec::raw+0x6e3>
   b984f:	xor    %r14d,%r14d
   b9852:	mov    %r11d,%ebx
   b9855:	test   %r15b,%r15b
   b9858:	sete   %bpl
   b985c:	je     b9790 <meshoptimizer_rs::codec::raw+0x5c0>
   b9862:	not    %r15d
   b9865:	add    %r10d,%r15d
   b9868:	and    $0xf,%r15d
   b986c:	mov    0xf0(%rsp,%r15,4),%ebx
   b9874:	jmp    b9790 <meshoptimizer_rs::codec::raw+0x5c0>
   b9879:	cmp    $0xfe,%al
   b987b:	jae    b98d5 <meshoptimizer_rs::codec::raw+0x705>
   b987d:	and    $0xf,%eax
   b9880:	mov    0x50(%rsp),%rcx
   b9885:	movzbl (%rcx,%rax,1),%r13d
   b988a:	lea    0x1(%r11),%r15d
   b988e:	xor    %eax,%eax
   b9890:	mov    %r13d,%ecx
   b9893:	shr    $0x4,%ecx
   b9896:	sete   %r12b
   b989a:	je     b9918 <meshoptimizer_rs::codec::raw+0x748>
   b989c:	mov    %r10d,%ebx
   b989f:	sub    %ecx,%ebx
   b98a1:	and    $0xf,%ebx
   b98a4:	mov    %r15d,%ebp
   b98a7:	mov    0xf0(%rsp,%rbx,4),%ecx
   b98ae:	mov    %ecx,%r15d
   b98b1:	jmp    b991c <meshoptimizer_rs::codec::raw+0x74c>
   b98b3:	cmp    $0xf,%r15b
   b98b7:	jne    b9998 <meshoptimizer_rs::codec::raw+0x7c8>
   b98bd:	movzbl (%r14,%rsi,1),%ebp
   b98c2:	test   %bpl,%bpl
   b98c5:	js     b9a06 <meshoptimizer_rs::codec::raw+0x836>
   b98cb:	inc    %rsi
   b98ce:	mov    %ebp,%ebx
   b98d0:	jmp    b9b9c <meshoptimizer_rs::codec::raw+0x9cc>
   b98d5:	movzbl (%r14,%rsi,1),%r14d
   b98da:	xor    %r15d,%r15d
   b98dd:	cmp    $0xfe,%eax
   b98e2:	sete   %r15b
   b98e6:	test   %r14d,%r14d
   b98e9:	cmove  %r14d,%r11d
   b98ed:	add    %r11d,%r15d
   b98f0:	mov    %r14d,%r12d
   b98f3:	shr    $0x4,%r12d
   b98f7:	je     b99b3 <meshoptimizer_rs::codec::raw+0x7e3>
   b98fd:	mov    %r10d,%ecx
   b9900:	sub    %r12d,%ecx
   b9903:	and    $0xf,%ecx
   b9906:	mov    %r15d,%ebx
   b9909:	mov    0xf0(%rsp,%rcx,4),%ecx
   b9910:	mov    %ecx,%r15d
   b9913:	jmp    b99b7 <meshoptimizer_rs::codec::raw+0x7e7>
   b9918:	lea    0x2(%r11),%ebp
   b991c:	xor    %r14d,%r14d
   b991f:	mov    %ebp,%ebx
   b9921:	test   $0xf,%r13b
   b9925:	sete   %cl
   b9928:	je     b993a <meshoptimizer_rs::codec::raw+0x76a>
   b992a:	mov    %r10d,%ebx
   b992d:	sub    %r13d,%ebx
   b9930:	and    $0xf,%ebx
   b9933:	mov    0xf0(%rsp,%rbx,4),%ebx
   b993a:	mov    %cl,%r14b
   b993d:	add    %r14d,%ebp
   b9940:	mov    %r11d,0xf0(%rsp,%r10,4)
   b9948:	lea    0x1(%r10),%rcx
   b994c:	and    $0xf,%ecx
   b994f:	mov    %r15d,0xf0(%rsp,%rcx,4)
   b9957:	mov    %r12b,%al
   b995a:	add    %r10,%rax
   b995d:	inc    %rax
   b9960:	add    %rax,%r14
   b9963:	and    $0xf,%eax
   b9966:	mov    %ebx,0xf0(%rsp,%rax,4)
   b996d:	mov    %r15d,0x1f0(%rsp,%r8,8)
   b9975:	mov    %r11d,0x1f4(%rsp,%r8,8)
   b997d:	lea    0x1(%r8),%r13d
   b9981:	and    $0xf,%r13d
   b9985:	mov    $0x3,%eax
   b998a:	mov    $0x2,%r12d
   b9990:	mov    %r14,%r10
   b9993:	jmp    b97cd <meshoptimizer_rs::codec::raw+0x5fd>
   b9998:	movzbl %r15b,%ecx
   b999c:	xor    $0x3,%r15b
   b99a0:	movzbl %r15b,%ebx
   b99a4:	mov    0x20(%rsp),%ebp
   b99a8:	add    %ecx,%ebp
   b99aa:	sub    %ebx,%ebp
   b99ac:	mov    %ebp,%ebx
   b99ae:	jmp    b9bad <meshoptimizer_rs::codec::raw+0x9dd>
   b99b3:	lea    0x1(%r15),%ebx
   b99b7:	mov    %r14d,%r13d
   b99ba:	and    $0xf,%r13b
   b99be:	je     b99d6 <meshoptimizer_rs::codec::raw+0x806>
   b99c0:	mov    %r10d,%ecx
   b99c3:	sub    %r14d,%ecx
   b99c6:	and    $0xf,%ecx
   b99c9:	mov    %ebx,%ebp
   b99cb:	mov    0xf0(%rsp,%rcx,4),%ecx
   b99d2:	mov    %ecx,%ebx
   b99d4:	jmp    b99d9 <meshoptimizer_rs::codec::raw+0x809>
   b99d6:	lea    0x1(%rbx),%ebp
   b99d9:	lea    0x1(%rsi),%r14
   b99dd:	cmp    $0xfe,%eax
   b99e2:	je     b9ae0 <meshoptimizer_rs::codec::raw+0x910>
   b99e8:	mov    0x30(%rsp),%r9
   b99ed:	movzbl 0x1(%r9,%rsi,1),%eax
   b99f3:	test   %al,%al
   b99f5:	js     b9a26 <meshoptimizer_rs::codec::raw+0x856>
   b99f7:	add    $0x2,%rsi
   b99fb:	mov    %rsi,%r14
   b99fe:	mov    %eax,%r11d
   b9a01:	jmp    b9ac2 <meshoptimizer_rs::codec::raw+0x8f2>
   b9a06:	and    $0x7f,%ebp
   b9a09:	movzbl 0x1(%r14,%rsi,1),%ebx
   b9a0f:	mov    %ebx,%ecx
   b9a11:	and    $0x7f,%ebx
   b9a14:	shl    $0x7,%ebx
   b9a17:	or     %ebp,%ebx
   b9a19:	test   %cl,%cl
   b9a1b:	js     b9a47 <meshoptimizer_rs::codec::raw+0x877>
   b9a1d:	add    $0x2,%rsi
   b9a21:	jmp    b9b9c <meshoptimizer_rs::codec::raw+0x9cc>
   b9a26:	and    $0x7f,%eax
   b9a29:	movzbl 0x2(%r9,%rsi,1),%r11d
   b9a2f:	mov    %r11d,%ecx
   b9a32:	and    $0x7f,%r11d
   b9a36:	shl    $0x7,%r11d
   b9a3a:	or     %eax,%r11d
   b9a3d:	test   %cl,%cl
   b9a3f:	js     b9a65 <meshoptimizer_rs::codec::raw+0x895>
   b9a41:	add    $0x3,%rsi
   b9a45:	jmp    b9abf <meshoptimizer_rs::codec::raw+0x8ef>
   b9a47:	movzbl 0x2(%r14,%rsi,1),%ecx
   b9a4d:	mov    %ecx,%ebp
   b9a4f:	and    $0x7f,%ecx
   b9a52:	shl    $0xe,%ecx
   b9a55:	or     %ecx,%ebx
   b9a57:	test   %bpl,%bpl
   b9a5a:	js     b9a80 <meshoptimizer_rs::codec::raw+0x8b0>
   b9a5c:	add    $0x3,%rsi
   b9a60:	jmp    b9b9c <meshoptimizer_rs::codec::raw+0x9cc>
   b9a65:	movzbl 0x3(%r9,%rsi,1),%eax
   b9a6b:	mov    %eax,%ecx
   b9a6d:	and    $0x7f,%eax
   b9a70:	shl    $0xe,%eax
   b9a73:	or     %eax,%r11d
   b9a76:	test   %cl,%cl
   b9a78:	js     b9aa2 <meshoptimizer_rs::codec::raw+0x8d2>
   b9a7a:	add    $0x4,%rsi
   b9a7e:	jmp    b9abf <meshoptimizer_rs::codec::raw+0x8ef>
   b9a80:	movzbl 0x3(%r14,%rsi,1),%ecx
   b9a86:	mov    %ecx,%ebp
   b9a88:	and    $0x7f,%ecx
   b9a8b:	shl    $0x15,%ecx
   b9a8e:	or     %ecx,%ebx
   b9a90:	test   %bpl,%bpl
   b9a93:	js     b9b8b <meshoptimizer_rs::codec::raw+0x9bb>
   b9a99:	add    $0x4,%rsi
   b9a9d:	jmp    b9b9c <meshoptimizer_rs::codec::raw+0x9cc>
   b9aa2:	movzbl 0x4(%r9,%rsi,1),%eax
   b9aa8:	mov    %eax,%ecx
   b9aaa:	and    $0x7f,%eax
   b9aad:	shl    $0x15,%eax
   b9ab0:	or     %eax,%r11d
   b9ab3:	test   %cl,%cl
   b9ab5:	js     b9bc4 <meshoptimizer_rs::codec::raw+0x9f4>
   b9abb:	add    $0x5,%rsi
   b9abf:	mov    %rsi,%r14
   b9ac2:	mov    0x28(%rsp),%r9
   b9ac7:	mov    %r11d,%eax
   b9aca:	shr    $1,%eax
   b9acc:	and    $0x1,%r11d
   b9ad0:	neg    %r11d
   b9ad3:	xor    %eax,%r11d
   b9ad6:	add    0x20(%rsp),%r11d
   b9adb:	mov    %r11d,0x20(%rsp)
   b9ae0:	cmp    $0xf,%r12b
   b9ae4:	jne    b9c13 <meshoptimizer_rs::codec::raw+0xa43>
   b9aea:	cmp    $0xfffffffffffffffa,%r14
   b9aee:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9af4:	lea    0x5(%r14),%rax
   b9af8:	cmp    0x630(%rsp),%rax
   b9b00:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9b06:	mov    0x30(%rsp),%r9
   b9b0b:	movzbl (%r9,%r14,1),%esi
   b9b10:	test   %sil,%sil
   b9b13:	js     b9b23 <meshoptimizer_rs::codec::raw+0x953>
   b9b15:	inc    %r14
   b9b18:	mov    %r14,%rax
   b9b1b:	mov    %esi,%r15d
   b9b1e:	jmp    b9bf2 <meshoptimizer_rs::codec::raw+0xa22>
   b9b23:	and    $0x7f,%esi
   b9b26:	movzbl 0x1(%r9,%r14,1),%r15d
   b9b2c:	mov    %r15d,%ecx
   b9b2f:	and    $0x7f,%r15d
   b9b33:	shl    $0x7,%r15d
   b9b37:	or     %esi,%r15d
   b9b3a:	test   %cl,%cl
   b9b3c:	js     b9b4a <meshoptimizer_rs::codec::raw+0x97a>
   b9b3e:	add    $0x2,%r14
   b9b42:	mov    %r14,%rax
   b9b45:	jmp    b9bf2 <meshoptimizer_rs::codec::raw+0xa22>
   b9b4a:	movzbl 0x2(%r9,%r14,1),%ecx
   b9b50:	mov    %ecx,%esi
   b9b52:	and    $0x7f,%ecx
   b9b55:	shl    $0xe,%ecx
   b9b58:	or     %ecx,%r15d
   b9b5b:	test   %sil,%sil
   b9b5e:	js     b9b6c <meshoptimizer_rs::codec::raw+0x99c>
   b9b60:	add    $0x3,%r14
   b9b64:	mov    %r14,%rax
   b9b67:	jmp    b9bf2 <meshoptimizer_rs::codec::raw+0xa22>
   b9b6c:	movzbl 0x3(%r9,%r14,1),%ecx
   b9b72:	mov    %ecx,%esi
   b9b74:	and    $0x7f,%ecx
   b9b77:	shl    $0x15,%ecx
   b9b7a:	or     %ecx,%r15d
   b9b7d:	test   %sil,%sil
   b9b80:	js     b9bde <meshoptimizer_rs::codec::raw+0xa0e>
   b9b82:	add    $0x4,%r14
   b9b86:	mov    %r14,%rax
   b9b89:	jmp    b9bf2 <meshoptimizer_rs::codec::raw+0xa22>
   b9b8b:	movzbl 0x4(%r14,%rsi,1),%ecx
   b9b91:	add    $0x5,%rsi
   b9b95:	shl    $0x1c,%ecx
   b9b98:	or     %ebx,%ecx
   b9b9a:	mov    %ecx,%ebx
   b9b9c:	mov    0x20(%rsp),%ebp
   b9ba0:	mov    %ebx,%ecx
   b9ba2:	shr    $1,%ecx
   b9ba4:	and    $0x1,%ebx
   b9ba7:	neg    %ebx
   b9ba9:	xor    %ecx,%ebx
   b9bab:	add    %ebp,%ebx
   b9bad:	mov    %ebx,0xf0(%rsp,%r10,4)
   b9bb5:	mov    $0x1,%r14d
   b9bbb:	mov    %ebx,0x20(%rsp)
   b9bbf:	jmp    b979e <meshoptimizer_rs::codec::raw+0x5ce>
   b9bc4:	add    $0x6,%rsi
   b9bc8:	mov    0x30(%rsp),%rax
   b9bcd:	movzbl 0x4(%rax,%r14,1),%eax
   b9bd3:	shl    $0x1c,%eax
   b9bd6:	or     %r11d,%eax
   b9bd9:	jmp    b99fb <meshoptimizer_rs::codec::raw+0x82b>
   b9bde:	mov    0x30(%rsp),%rcx
   b9be3:	movzbl 0x4(%rcx,%r14,1),%ecx
   b9be9:	shl    $0x1c,%ecx
   b9bec:	or     %r15d,%ecx
   b9bef:	mov    %ecx,%r15d
   b9bf2:	mov    0x28(%rsp),%r9
   b9bf7:	mov    %r15d,%ecx
   b9bfa:	shr    $1,%ecx
   b9bfc:	and    $0x1,%r15d
   b9c00:	neg    %r15d
   b9c03:	xor    %ecx,%r15d
   b9c06:	add    0x20(%rsp),%r15d
   b9c0b:	mov    %rax,%r14
   b9c0e:	mov    %r15d,0x20(%rsp)
   b9c13:	cmp    $0xf,%r13b
   b9c17:	jne    b9cdb <meshoptimizer_rs::codec::raw+0xb0b>
   b9c1d:	cmp    $0xfffffffffffffffa,%r14
   b9c21:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9c27:	lea    0x5(%r14),%rax
   b9c2b:	cmp    0x630(%rsp),%rax
   b9c33:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9c39:	mov    0x30(%rsp),%r9
   b9c3e:	movzbl (%r9,%r14,1),%esi
   b9c43:	test   %sil,%sil
   b9c46:	js     b9c52 <meshoptimizer_rs::codec::raw+0xa82>
   b9c48:	inc    %r14
   b9c4b:	mov    %r14,%rax
   b9c4e:	mov    %esi,%ebx
   b9c50:	jmp    b9cc0 <meshoptimizer_rs::codec::raw+0xaf0>
   b9c52:	and    $0x7f,%esi
   b9c55:	movzbl 0x1(%r9,%r14,1),%ebx
   b9c5b:	mov    %ebx,%ecx
   b9c5d:	and    $0x7f,%ebx
   b9c60:	shl    $0x7,%ebx
   b9c63:	or     %esi,%ebx
   b9c65:	test   %cl,%cl
   b9c67:	js     b9c72 <meshoptimizer_rs::codec::raw+0xaa2>
   b9c69:	add    $0x2,%r14
   b9c6d:	mov    %r14,%rax
   b9c70:	jmp    b9cc0 <meshoptimizer_rs::codec::raw+0xaf0>
   b9c72:	movzbl 0x2(%r9,%r14,1),%ecx
   b9c78:	mov    %ecx,%esi
   b9c7a:	and    $0x7f,%ecx
   b9c7d:	shl    $0xe,%ecx
   b9c80:	or     %ecx,%ebx
   b9c82:	test   %sil,%sil
   b9c85:	js     b9c90 <meshoptimizer_rs::codec::raw+0xac0>
   b9c87:	add    $0x3,%r14
   b9c8b:	mov    %r14,%rax
   b9c8e:	jmp    b9cc0 <meshoptimizer_rs::codec::raw+0xaf0>
   b9c90:	movzbl 0x3(%r9,%r14,1),%ecx
   b9c96:	mov    %ecx,%esi
   b9c98:	and    $0x7f,%ecx
   b9c9b:	shl    $0x15,%ecx
   b9c9e:	or     %ecx,%ebx
   b9ca0:	test   %sil,%sil
   b9ca3:	js     b9cae <meshoptimizer_rs::codec::raw+0xade>
   b9ca5:	add    $0x4,%r14
   b9ca9:	mov    %r14,%rax
   b9cac:	jmp    b9cc0 <meshoptimizer_rs::codec::raw+0xaf0>
   b9cae:	mov    0x30(%rsp),%rcx
   b9cb3:	movzbl 0x4(%rcx,%r14,1),%ecx
   b9cb9:	shl    $0x1c,%ecx
   b9cbc:	or     %ebx,%ecx
   b9cbe:	mov    %ecx,%ebx
   b9cc0:	mov    0x28(%rsp),%r9
   b9cc5:	mov    %ebx,%ecx
   b9cc7:	shr    $1,%ecx
   b9cc9:	and    $0x1,%ebx
   b9ccc:	neg    %ebx
   b9cce:	xor    %ecx,%ebx
   b9cd0:	add    0x20(%rsp),%ebx
   b9cd4:	mov    %rax,%r14
   b9cd7:	mov    %ebx,0x20(%rsp)
   b9cdb:	cmp    $0xf,%r13b
   b9cdf:	sete   %al
   b9ce2:	cmp    $0xf,%r12b
   b9ce6:	sete   %cl
   b9ce9:	test   %r13b,%r13b
   b9cec:	sete   %sil
   b9cf0:	test   %r12b,%r12b
   b9cf3:	sete   %r12b
   b9cf7:	mov    %r11d,0xf0(%rsp,%r10,4)
   b9cff:	lea    0x1(%r10),%r13
   b9d03:	and    $0xf,%r13d
   b9d07:	or     %cl,%r12b
   b9d0a:	mov    %r15d,0xf0(%rsp,%r13,4)
   b9d12:	movzbl %r12b,%ecx
   b9d16:	add    %r10,%rcx
   b9d19:	inc    %rcx
   b9d1c:	or     %al,%sil
   b9d1f:	movzbl %sil,%r10d
   b9d23:	add    %rcx,%r10
   b9d26:	and    $0xf,%ecx
   b9d29:	mov    %ebx,0xf0(%rsp,%rcx,4)
   b9d30:	mov    %r15d,0x1f0(%rsp,%r8,8)
   b9d38:	mov    %r11d,0x1f4(%rsp,%r8,8)
   b9d40:	lea    0x1(%r8),%r13d
   b9d44:	and    $0xf,%r13d
   b9d48:	mov    $0x3,%eax
   b9d4d:	mov    $0x2,%r12d
   b9d53:	mov    %r14,%rsi
   b9d56:	jmp    b97cd <meshoptimizer_rs::codec::raw+0x5fd>
   b9d5b:	mov    %bpl,%r14b
   b9d5e:	add    %r14d,%r11d
   b9d61:	mov    %ebx,0xf0(%rsp,%r10,4)
   b9d69:	mov    %r11d,%ebp
   b9d6c:	not    %eax
   b9d6e:	shr    $0x4,%eax
   b9d71:	add    %r8d,%eax
   b9d74:	and    $0xf,%eax
   b9d77:	mov    0x1f0(%rsp,%rax,8),%r11d
   b9d7f:	mov    0x1f4(%rsp,%rax,8),%r15d
   b9d87:	add    %r14,%r10
   b9d8a:	mov    $0x2,%eax
   b9d8f:	mov    $0x1,%r12d
   b9d95:	mov    %r8,%r13
   b9d98:	inc    %rdi
   b9d9b:	mov    %ebx,0x1f0(%rsp,%r13,8)
   b9da3:	mov    %r15d,0x1f4(%rsp,%r13,8)
   b9dab:	add    %r8d,%r12d
   b9dae:	and    $0xf,%r12d
   b9db2:	mov    %r11d,0x1f0(%rsp,%r12,8)
   b9dba:	mov    %ebx,0x1f4(%rsp,%r12,8)
   b9dc2:	add    %r8d,%eax
   b9dc5:	and    $0xf,%eax
   b9dc8:	and    $0xf,%r10d
   b9dcc:	mov    %r11d,-0x8(%rdx)
   b9dd0:	mov    %r15d,-0x4(%rdx)
   b9dd4:	mov    %rdx,%r15
   b9dd7:	mov    %ebx,(%rdx)
   b9dd9:	add    $0xc,%r15
   b9ddd:	mov    %rax,%r8
   b9de0:	mov    %ebp,%r11d
   b9de3:	cmp    %rdi,0x38(%rsp)
   b9de8:	je     bad24 <meshoptimizer_rs::codec::raw+0x1b54>
   b9dee:	cmp    %r9,%rsi
   b9df1:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   b9df7:	mov    %r15,%rdx
   b9dfa:	mov    0x30(%rsp),%r14
   b9dff:	movzbl 0x1(%r14,%rdi,1),%eax
   b9e05:	cmp    $0xf0,%al
   b9e07:	jae    b9e41 <meshoptimizer_rs::codec::raw+0xc71>
   b9e09:	mov    %eax,%r15d
   b9e0c:	and    $0xf,%r15d
   b9e10:	cmp    %r15d,0x58(%rsp)
   b9e15:	jbe    b9e7b <meshoptimizer_rs::codec::raw+0xcab>
   b9e17:	xor    %r14d,%r14d
   b9e1a:	mov    %r11d,%ebx
   b9e1d:	test   %r15b,%r15b
   b9e20:	sete   %bpl
   b9e24:	je     b9d5b <meshoptimizer_rs::codec::raw+0xb8b>
   b9e2a:	not    %r15d
   b9e2d:	add    %r10d,%r15d
   b9e30:	and    $0xf,%r15d
   b9e34:	mov    0xf0(%rsp,%r15,4),%ebx
   b9e3c:	jmp    b9d5b <meshoptimizer_rs::codec::raw+0xb8b>
   b9e41:	cmp    $0xfe,%al
   b9e43:	jae    b9e9d <meshoptimizer_rs::codec::raw+0xccd>
   b9e45:	and    $0xf,%eax
   b9e48:	mov    0x50(%rsp),%rcx
   b9e4d:	movzbl (%rcx,%rax,1),%r13d
   b9e52:	lea    0x1(%r11),%r15d
   b9e56:	xor    %eax,%eax
   b9e58:	mov    %r13d,%ecx
   b9e5b:	shr    $0x4,%ecx
   b9e5e:	sete   %r12b
   b9e62:	je     b9ee0 <meshoptimizer_rs::codec::raw+0xd10>
   b9e64:	mov    %r10d,%ebx
   b9e67:	sub    %ecx,%ebx
   b9e69:	and    $0xf,%ebx
   b9e6c:	mov    %r15d,%ebp
   b9e6f:	mov    0xf0(%rsp,%rbx,4),%ecx
   b9e76:	mov    %ecx,%r15d
   b9e79:	jmp    b9ee4 <meshoptimizer_rs::codec::raw+0xd14>
   b9e7b:	cmp    $0xf,%r15b
   b9e7f:	jne    b9f60 <meshoptimizer_rs::codec::raw+0xd90>
   b9e85:	movzbl (%r14,%rsi,1),%ebp
   b9e8a:	test   %bpl,%bpl
   b9e8d:	js     b9fce <meshoptimizer_rs::codec::raw+0xdfe>
   b9e93:	inc    %rsi
   b9e96:	mov    %ebp,%ebx
   b9e98:	jmp    ba164 <meshoptimizer_rs::codec::raw+0xf94>
   b9e9d:	movzbl (%r14,%rsi,1),%r14d
   b9ea2:	xor    %r15d,%r15d
   b9ea5:	cmp    $0xfe,%eax
   b9eaa:	sete   %r15b
   b9eae:	test   %r14d,%r14d
   b9eb1:	cmove  %r14d,%r11d
   b9eb5:	add    %r11d,%r15d
   b9eb8:	mov    %r14d,%r12d
   b9ebb:	shr    $0x4,%r12d
   b9ebf:	je     b9f7b <meshoptimizer_rs::codec::raw+0xdab>
   b9ec5:	mov    %r10d,%ecx
   b9ec8:	sub    %r12d,%ecx
   b9ecb:	and    $0xf,%ecx
   b9ece:	mov    %r15d,%ebx
   b9ed1:	mov    0xf0(%rsp,%rcx,4),%ecx
   b9ed8:	mov    %ecx,%r15d
   b9edb:	jmp    b9f7f <meshoptimizer_rs::codec::raw+0xdaf>
   b9ee0:	lea    0x2(%r11),%ebp
   b9ee4:	xor    %r14d,%r14d
   b9ee7:	mov    %ebp,%ebx
   b9ee9:	test   $0xf,%r13b
   b9eed:	sete   %cl
   b9ef0:	je     b9f02 <meshoptimizer_rs::codec::raw+0xd32>
   b9ef2:	mov    %r10d,%ebx
   b9ef5:	sub    %r13d,%ebx
   b9ef8:	and    $0xf,%ebx
   b9efb:	mov    0xf0(%rsp,%rbx,4),%ebx
   b9f02:	mov    %cl,%r14b
   b9f05:	add    %r14d,%ebp
   b9f08:	mov    %r11d,0xf0(%rsp,%r10,4)
   b9f10:	lea    0x1(%r10),%rcx
   b9f14:	and    $0xf,%ecx
   b9f17:	mov    %r15d,0xf0(%rsp,%rcx,4)
   b9f1f:	mov    %r12b,%al
   b9f22:	add    %r10,%rax
   b9f25:	inc    %rax
   b9f28:	add    %rax,%r14
   b9f2b:	and    $0xf,%eax
   b9f2e:	mov    %ebx,0xf0(%rsp,%rax,4)
   b9f35:	mov    %r15d,0x1f0(%rsp,%r8,8)
   b9f3d:	mov    %r11d,0x1f4(%rsp,%r8,8)
   b9f45:	lea    0x1(%r8),%r13d
   b9f49:	and    $0xf,%r13d
   b9f4d:	mov    $0x3,%eax
   b9f52:	mov    $0x2,%r12d
   b9f58:	mov    %r14,%r10
   b9f5b:	jmp    b9d98 <meshoptimizer_rs::codec::raw+0xbc8>
   b9f60:	movzbl %r15b,%ecx
   b9f64:	xor    $0x3,%r15b
   b9f68:	movzbl %r15b,%ebx
   b9f6c:	mov    0x20(%rsp),%ebp
   b9f70:	add    %ecx,%ebp
   b9f72:	sub    %ebx,%ebp
   b9f74:	mov    %ebp,%ebx
   b9f76:	jmp    ba175 <meshoptimizer_rs::codec::raw+0xfa5>
   b9f7b:	lea    0x1(%r15),%ebx
   b9f7f:	mov    %r14d,%r13d
   b9f82:	and    $0xf,%r13b
   b9f86:	je     b9f9e <meshoptimizer_rs::codec::raw+0xdce>
   b9f88:	mov    %r10d,%ecx
   b9f8b:	sub    %r14d,%ecx
   b9f8e:	and    $0xf,%ecx
   b9f91:	mov    %ebx,%ebp
   b9f93:	mov    0xf0(%rsp,%rcx,4),%ecx
   b9f9a:	mov    %ecx,%ebx
   b9f9c:	jmp    b9fa1 <meshoptimizer_rs::codec::raw+0xdd1>
   b9f9e:	lea    0x1(%rbx),%ebp
   b9fa1:	lea    0x1(%rsi),%r14
   b9fa5:	cmp    $0xfe,%eax
   b9faa:	je     ba0a8 <meshoptimizer_rs::codec::raw+0xed8>
   b9fb0:	mov    0x30(%rsp),%r9
   b9fb5:	movzbl 0x1(%r9,%rsi,1),%eax
   b9fbb:	test   %al,%al
   b9fbd:	js     b9fee <meshoptimizer_rs::codec::raw+0xe1e>
   b9fbf:	add    $0x2,%rsi
   b9fc3:	mov    %rsi,%r14
   b9fc6:	mov    %eax,%r11d
   b9fc9:	jmp    ba08a <meshoptimizer_rs::codec::raw+0xeba>
   b9fce:	and    $0x7f,%ebp
   b9fd1:	movzbl 0x1(%r14,%rsi,1),%ebx
   b9fd7:	mov    %ebx,%ecx
   b9fd9:	and    $0x7f,%ebx
   b9fdc:	shl    $0x7,%ebx
   b9fdf:	or     %ebp,%ebx
   b9fe1:	test   %cl,%cl
   b9fe3:	js     ba00f <meshoptimizer_rs::codec::raw+0xe3f>
   b9fe5:	add    $0x2,%rsi
   b9fe9:	jmp    ba164 <meshoptimizer_rs::codec::raw+0xf94>
   b9fee:	and    $0x7f,%eax
   b9ff1:	movzbl 0x2(%r9,%rsi,1),%r11d
   b9ff7:	mov    %r11d,%ecx
   b9ffa:	and    $0x7f,%r11d
   b9ffe:	shl    $0x7,%r11d
   ba002:	or     %eax,%r11d
   ba005:	test   %cl,%cl
   ba007:	js     ba02d <meshoptimizer_rs::codec::raw+0xe5d>
   ba009:	add    $0x3,%rsi
   ba00d:	jmp    ba087 <meshoptimizer_rs::codec::raw+0xeb7>
   ba00f:	movzbl 0x2(%r14,%rsi,1),%ecx
   ba015:	mov    %ecx,%ebp
   ba017:	and    $0x7f,%ecx
   ba01a:	shl    $0xe,%ecx
   ba01d:	or     %ecx,%ebx
   ba01f:	test   %bpl,%bpl
   ba022:	js     ba048 <meshoptimizer_rs::codec::raw+0xe78>
   ba024:	add    $0x3,%rsi
   ba028:	jmp    ba164 <meshoptimizer_rs::codec::raw+0xf94>
   ba02d:	movzbl 0x3(%r9,%rsi,1),%eax
   ba033:	mov    %eax,%ecx
   ba035:	and    $0x7f,%eax
   ba038:	shl    $0xe,%eax
   ba03b:	or     %eax,%r11d
   ba03e:	test   %cl,%cl
   ba040:	js     ba06a <meshoptimizer_rs::codec::raw+0xe9a>
   ba042:	add    $0x4,%rsi
   ba046:	jmp    ba087 <meshoptimizer_rs::codec::raw+0xeb7>
   ba048:	movzbl 0x3(%r14,%rsi,1),%ecx
   ba04e:	mov    %ecx,%ebp
   ba050:	and    $0x7f,%ecx
   ba053:	shl    $0x15,%ecx
   ba056:	or     %ecx,%ebx
   ba058:	test   %bpl,%bpl
   ba05b:	js     ba153 <meshoptimizer_rs::codec::raw+0xf83>
   ba061:	add    $0x4,%rsi
   ba065:	jmp    ba164 <meshoptimizer_rs::codec::raw+0xf94>
   ba06a:	movzbl 0x4(%r9,%rsi,1),%eax
   ba070:	mov    %eax,%ecx
   ba072:	and    $0x7f,%eax
   ba075:	shl    $0x15,%eax
   ba078:	or     %eax,%r11d
   ba07b:	test   %cl,%cl
   ba07d:	js     ba18c <meshoptimizer_rs::codec::raw+0xfbc>
   ba083:	add    $0x5,%rsi
   ba087:	mov    %rsi,%r14
   ba08a:	mov    0x28(%rsp),%r9
   ba08f:	mov    %r11d,%eax
   ba092:	shr    $1,%eax
   ba094:	and    $0x1,%r11d
   ba098:	neg    %r11d
   ba09b:	xor    %eax,%r11d
   ba09e:	add    0x20(%rsp),%r11d
   ba0a3:	mov    %r11d,0x20(%rsp)
   ba0a8:	cmp    $0xf,%r12b
   ba0ac:	jne    ba1db <meshoptimizer_rs::codec::raw+0x100b>
   ba0b2:	cmp    $0xfffffffffffffffa,%r14
   ba0b6:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   ba0bc:	lea    0x5(%r14),%rax
   ba0c0:	cmp    0x630(%rsp),%rax
   ba0c8:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   ba0ce:	mov    0x30(%rsp),%r9
   ba0d3:	movzbl (%r9,%r14,1),%esi
   ba0d8:	test   %sil,%sil
   ba0db:	js     ba0eb <meshoptimizer_rs::codec::raw+0xf1b>
   ba0dd:	inc    %r14
   ba0e0:	mov    %r14,%rax
   ba0e3:	mov    %esi,%r15d
   ba0e6:	jmp    ba1ba <meshoptimizer_rs::codec::raw+0xfea>
   ba0eb:	and    $0x7f,%esi
   ba0ee:	movzbl 0x1(%r9,%r14,1),%r15d
   ba0f4:	mov    %r15d,%ecx
   ba0f7:	and    $0x7f,%r15d
   ba0fb:	shl    $0x7,%r15d
   ba0ff:	or     %esi,%r15d
   ba102:	test   %cl,%cl
   ba104:	js     ba112 <meshoptimizer_rs::codec::raw+0xf42>
   ba106:	add    $0x2,%r14
   ba10a:	mov    %r14,%rax
   ba10d:	jmp    ba1ba <meshoptimizer_rs::codec::raw+0xfea>
   ba112:	movzbl 0x2(%r9,%r14,1),%ecx
   ba118:	mov    %ecx,%esi
   ba11a:	and    $0x7f,%ecx
   ba11d:	shl    $0xe,%ecx
   ba120:	or     %ecx,%r15d
   ba123:	test   %sil,%sil
   ba126:	js     ba134 <meshoptimizer_rs::codec::raw+0xf64>
   ba128:	add    $0x3,%r14
   ba12c:	mov    %r14,%rax
   ba12f:	jmp    ba1ba <meshoptimizer_rs::codec::raw+0xfea>
   ba134:	movzbl 0x3(%r9,%r14,1),%ecx
   ba13a:	mov    %ecx,%esi
   ba13c:	and    $0x7f,%ecx
   ba13f:	shl    $0x15,%ecx
   ba142:	or     %ecx,%r15d
   ba145:	test   %sil,%sil
   ba148:	js     ba1a6 <meshoptimizer_rs::codec::raw+0xfd6>
   ba14a:	add    $0x4,%r14
   ba14e:	mov    %r14,%rax
   ba151:	jmp    ba1ba <meshoptimizer_rs::codec::raw+0xfea>
   ba153:	movzbl 0x4(%r14,%rsi,1),%ecx
   ba159:	add    $0x5,%rsi
   ba15d:	shl    $0x1c,%ecx
   ba160:	or     %ebx,%ecx
   ba162:	mov    %ecx,%ebx
   ba164:	mov    0x20(%rsp),%ebp
   ba168:	mov    %ebx,%ecx
   ba16a:	shr    $1,%ecx
   ba16c:	and    $0x1,%ebx
   ba16f:	neg    %ebx
   ba171:	xor    %ecx,%ebx
   ba173:	add    %ebp,%ebx
   ba175:	mov    %ebx,0xf0(%rsp,%r10,4)
   ba17d:	mov    $0x1,%r14d
   ba183:	mov    %ebx,0x20(%rsp)
   ba187:	jmp    b9d69 <meshoptimizer_rs::codec::raw+0xb99>
   ba18c:	add    $0x6,%rsi
   ba190:	mov    0x30(%rsp),%rax
   ba195:	movzbl 0x4(%rax,%r14,1),%eax
   ba19b:	shl    $0x1c,%eax
   ba19e:	or     %r11d,%eax
   ba1a1:	jmp    b9fc3 <meshoptimizer_rs::codec::raw+0xdf3>
   ba1a6:	mov    0x30(%rsp),%rcx
   ba1ab:	movzbl 0x4(%rcx,%r14,1),%ecx
   ba1b1:	shl    $0x1c,%ecx
   ba1b4:	or     %r15d,%ecx
   ba1b7:	mov    %ecx,%r15d
   ba1ba:	mov    0x28(%rsp),%r9
   ba1bf:	mov    %r15d,%ecx
   ba1c2:	shr    $1,%ecx
   ba1c4:	and    $0x1,%r15d
   ba1c8:	neg    %r15d
   ba1cb:	xor    %ecx,%r15d
   ba1ce:	add    0x20(%rsp),%r15d
   ba1d3:	mov    %rax,%r14
   ba1d6:	mov    %r15d,0x20(%rsp)
   ba1db:	cmp    $0xf,%r13b
   ba1df:	jne    ba2a3 <meshoptimizer_rs::codec::raw+0x10d3>
   ba1e5:	cmp    $0xfffffffffffffffa,%r14
   ba1e9:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   ba1ef:	lea    0x5(%r14),%rax
   ba1f3:	cmp    0x630(%rsp),%rax
   ba1fb:	ja     bad2b <meshoptimizer_rs::codec::raw+0x1b5b>
   ba201:	mov    0x30(%rsp),%r9
   ba206:	movzbl (%r9,%r14,1),%esi
   ba20b:	test   %sil,%sil
   ba20e:	js     ba21a <meshoptimizer_rs::codec::raw+0x104a>
   ba210:	inc    %r14
   ba213:	mov    %r14,%rax
   ba216:	mov    %esi,%ebx
   ba218:	jmp    ba288 <meshoptimizer_rs::codec::raw+0x10b8>
   ba21a:	and    $0x7f,%esi
   ba21d:	movzbl 0x1(%r9,%r14,1),%ebx
   ba223:	mov    %ebx,%ecx
   ba225:	and    $0x7f,%ebx
   ba228:	shl    $0x7,%ebx
   ba22b:	or     %esi,%ebx
   ba22d:	test   %cl,%cl
   ba22f:	js     ba23a <meshoptimizer_rs::codec::raw+0x106a>
   ba231:	add    $0x2,%r14
   ba235:	mov    %r14,%rax
   ba238:	jmp    ba288 <meshoptimizer_rs::codec::raw+0x10b8>
   ba23a:	movzbl 0x2(%r9,%r14,1),%ecx
   ba240:	mov    %ecx,%esi
   ba242:	and    $0x7f,%ecx
   ba245:	shl    $0xe,%ecx
   ba248:	or     %ecx,%ebx
   ba24a:	test   %sil,%sil
   ba24d:	js     ba258 <meshoptimizer_rs::codec::raw+0x1088>
   ba24f:	add    $0x3,%r14
   ba253:	mov    %r14,%rax
   ba256:	jmp    ba288 <meshoptimizer_rs::codec::raw+0x10b8>
   ba258:	movzbl 0x3(%r9,%r14,1),%ecx
   ba25e:	mov    %ecx,%esi
   ba260:	and    $0x7f,%ecx
   ba263:	shl    $0x15,%ecx
   ba266:	or     %ecx,%ebx
   ba268:	test   %sil,%sil
   ba26b:	js     ba276 <meshoptimizer_rs::codec::raw+0x10a6>
   ba26d:	add    $0x4,%r14
   ba271:	mov    %r14,%rax
   ba274:	jmp    ba288 <meshoptimizer_rs::codec::raw+0x10b8>
   ba276:	mov    0x30(%rsp),%rcx
   ba27b:	movzbl 0x4(%rcx,%r14,1),%ecx
   ba281:	shl    $0x1c,%ecx
   ba284:	or     %ebx,%ecx
   ba286:	mov    %ecx,%ebx
   ba288:	mov    0x28(%rsp),%r9
   ba28d:	mov    %ebx,%ecx
   ba28f:	shr    $1,%ecx
   ba291:	and    $0x1,%ebx
   ba294:	neg    %ebx
   ba296:	xor    %ecx,%ebx
   ba298:	add    0x20(%rsp),%ebx
   ba29c:	mov    %rax,%r14
   ba29f:	mov    %ebx,0x20(%rsp)
   ba2a3:	cmp    $0xf,%r13b
   ba2a7:	sete   %al
   ba2aa:	cmp    $0xf,%r12b
   ba2ae:	sete   %cl
   ba2b1:	test   %r13b,%r13b
   ba2b4:	sete   %sil
   ba2b8:	test   %r12b,%r12b
   ba2bb:	sete   %r12b
   ba2bf:	mov    %r11d,0xf0(%rsp,%r10,4)
   ba2c7:	lea    0x1(%r10),%r13
   ba2cb:	and    $0xf,%r13d
   ba2cf:	or     %cl,%r12b
   ba2d2:	mov    %r15d,0xf0(%rsp,%r13,4)
   ba2da:	movzbl %r12b,%ecx
   ba2de:	add    %r10,%rcx
   ba2e1:	inc    %rcx
   ba2e4:	or     %al,%sil
   ba2e7:	movzbl %sil,%r10d
   ba2eb:	add    %rcx,%r10
   ba2ee:	and    $0xf,%ecx
   ba2f1:	mov    %ebx,0xf0(%rsp,%rcx,4)
   ba2f8:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ba300:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ba308:	lea    0x1(%r8),%r13d
   ba30c:	and    $0xf,%r13d
   ba310:	mov    $0x3,%eax
   ba315:	mov    $0x2,%r12d
   ba31b:	mov    %r14,%rsi
   ba31e:	jmp    b9d98 <meshoptimizer_rs::codec::raw+0xbc8>
   ba323:	cmp    $0x21,%rbp
   ba327:	mov    $0x20,%eax
   ba32c:	cmovae %rbp,%rax
   ba330:	mov    %rbp,%rcx
   ba333:	mov    %rax,0xe0(%rsp)
   ba33b:	lea    0x1(%rax),%rdx
   ba33f:	mov    $0x7,%al
   ba341:	cmp    %rdx,%rbx
   ba344:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba34a:	mov    %rbx,%rsi
   ba34d:	sub    %rcx,%rsi
   ba350:	lea    (%rsi,%rbp,1),%rax
   ba354:	mov    %rbx,%rcx
   ba357:	mov    %rax,0x60(%rsp)
   ba35c:	sub    %rax,%rcx
   ba35f:	mov    %rcx,0xa0(%rsp)
   ba367:	jb     badac <meshoptimizer_rs::codec::raw+0x1bdc>
   ba36d:	pxor   %xmm0,%xmm0
   ba371:	movdqa %xmm0,0x1e0(%rsp)
   ba37a:	movdqa %xmm0,0x1d0(%rsp)
   ba383:	movdqa %xmm0,0x1c0(%rsp)
   ba38c:	movdqa %xmm0,0x1b0(%rsp)
   ba395:	movdqa %xmm0,0x1a0(%rsp)
   ba39e:	movdqa %xmm0,0x190(%rsp)
   ba3a7:	movdqa %xmm0,0x180(%rsp)
   ba3b0:	movdqa %xmm0,0x170(%rsp)
   ba3b9:	movdqa %xmm0,0x160(%rsp)
   ba3c2:	movdqa %xmm0,0x150(%rsp)
   ba3cb:	movdqa %xmm0,0x140(%rsp)
   ba3d4:	movdqa %xmm0,0x130(%rsp)
   ba3dd:	movdqa %xmm0,0x120(%rsp)
   ba3e6:	movdqa %xmm0,0x110(%rsp)
   ba3ef:	movdqa %xmm0,0x100(%rsp)
   ba3f8:	movdqa %xmm0,0xf0(%rsp)
   ba401:	cmp    $0x101,%rbp
   ba408:	jae    bad6d <meshoptimizer_rs::codec::raw+0x1b9d>
   ba40e:	mov    %dil,0x38(%rsp)
   ba413:	lea    (%rsi,%rbp,1),%rax
   ba417:	cmp    %rsi,%rax
   ba41a:	jb     bad84 <meshoptimizer_rs::codec::raw+0x1bb4>
   ba420:	add    %r14,%rsi
   ba423:	lea    0xf0(%rsp),%rdi
   ba42b:	mov    %rbp,%rdx
   ba42e:	call   *0x8fe24(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   ba434:	test   %rbp,%rbp
   ba437:	je     bae15 <meshoptimizer_rs::codec::raw+0x1c45>
   ba43d:	mov    $0x2000,%ax
   ba441:	xor    %edx,%edx
   ba443:	div    %bp
   ba446:	and    $0x3ff0,%eax
   ba44b:	cmp    $0x100,%eax
   ba450:	mov    $0x100,%ecx
   ba455:	cmovb  %eax,%ecx
   ba458:	mov    %rcx,0xd8(%rsp)
   ba460:	lea    0x1f0(%rsp),%rdi
   ba468:	mov    $0x400,%edx
   ba46d:	xor    %esi,%esi
   ba46f:	call   *0x8fdf3(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   ba475:	test   %r13,%r13
   ba478:	je     bad00 <meshoptimizer_rs::codec::raw+0x1b30>
   ba47e:	mov    %r15,0xd0(%rsp)
   ba486:	add    %r14,0x60(%rsp)
   ba48b:	mov    %rbp,%rax
   ba48e:	shr    $0x2,%rax
   ba492:	mov    %rax,0x98(%rsp)
   ba49a:	lea    0x3(%rbp),%rax
   ba49e:	shr    $0x2,%rax
   ba4a2:	mov    %rax,0xe8(%rsp)
   ba4aa:	mov    $0x1,%r15d
   ba4b0:	xor    %eax,%eax
   ba4b2:	mov    %r12,0x88(%rsp)
   ba4ba:	mov    %r14,0x30(%rsp)
   ba4bf:	mov    %r13,%rcx
   ba4c2:	mov    %rax,%rdi
   ba4c5:	sub    %rax,%rcx
   ba4c8:	mov    0xd8(%rsp),%rax
   ba4d0:	cmp    %rax,%rcx
   ba4d3:	cmovae %rax,%rcx
   ba4d7:	mov    %rcx,0x20(%rsp)
   ba4dc:	movzbl 0x38(%rsp),%edx
   ba4e1:	test   %dl,%dl
   ba4e3:	je     ba529 <meshoptimizer_rs::codec::raw+0x1359>
   ba4e5:	mov    %r15,%rcx
   ba4e8:	mov    $0x7,%al
   ba4ea:	add    0x98(%rsp),%rcx
   ba4f2:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba4f8:	cmp    %rbx,%rcx
   ba4fb:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba501:	add    %r14,%r15
   ba504:	mov    0x98(%rsp),%rax
   ba50c:	mov    %rax,0x90(%rsp)
   ba514:	mov    %r15,0xa8(%rsp)
   ba51c:	mov    %rcx,%r15
   ba51f:	mov    0x88(%rsp),%r12
   ba527:	jmp    ba542 <meshoptimizer_rs::codec::raw+0x1372>
   ba529:	movq   $0x0,0x90(%rsp)
   ba535:	mov    $0x1,%eax
   ba53a:	mov    %rax,0xa8(%rsp)
   ba542:	mov    0x20(%rsp),%rax
   ba547:	lea    (%rax,%rdi,1),%rsi
   ba54b:	imul   %rbp,%rdi
   ba54f:	mov    %rsi,0xc8(%rsp)
   ba557:	imul   %rbp,%rsi
   ba55b:	cmp    %rdi,%rsi
   ba55e:	jb     bad9c <meshoptimizer_rs::codec::raw+0x1bcc>
   ba564:	cmp    %r12,%rsi
   ba567:	ja     bad9c <meshoptimizer_rs::codec::raw+0x1bcc>
   ba56d:	mov    0x20(%rsp),%rax
   ba572:	lea    0xf(%rax),%ecx
   ba575:	and    $0x3f0,%ecx
   ba57b:	mov    %rcx,0x28(%rsp)
   ba580:	mov    %rax,%rcx
   ba583:	imul   %rbp,%rcx
   ba587:	mov    %rcx,0xb8(%rsp)
   ba58f:	add    0xd0(%rsp),%rdi
   ba597:	mov    %rdi,0xb0(%rsp)
   ba59f:	lea    -0x1(%rax),%rcx
   ba5a3:	imul   %rbp,%rcx
   ba5a7:	mov    %rcx,0xc0(%rsp)
   ba5af:	add    $0x4,%rcx
   ba5b3:	mov    %rcx,0x50(%rsp)
   ba5b8:	lea    0x0(,%rax,4),%rcx
   ba5c0:	mov    %rcx,0x68(%rsp)
   ba5c5:	lea    0x1f0(%rsp,%rax,1),%rcx
   ba5cd:	mov    %rcx,0x80(%rsp)
   ba5d5:	lea    0x1f0(%rsp,%rax,2),%rcx
   ba5dd:	mov    %rcx,0x78(%rsp)
   ba5e2:	lea    (%rax,%rax,2),%rax
   ba5e6:	add    %rsp,%rax
   ba5e9:	add    $0x1f0,%rax
   ba5ef:	mov    %rax,0x70(%rsp)
   ba5f4:	xor    %r14d,%r14d
   ba5f7:	xor    %ebx,%ebx
   ba5f9:	jmp    ba65a <meshoptimizer_rs::codec::raw+0x148a>
   ba5fb:	lea    (%rsp,%rbx,1),%rcx
   ba5ff:	add    $0xf0,%rcx
   ba606:	shr    $0x4,%al
   ba609:	neg    %al
   ba60b:	and    $0x1f,%al
   ba60d:	movzbl %al,%eax
   ba610:	mov    %rcx,(%rsp)
   ba614:	mov    %eax,0x8(%rsp)
   ba618:	movq   $0x0,0x10(%rsp)
   ba621:	lea    0x1f0(%rsp),%rdi
   ba629:	mov    0x68(%rsp),%rsi
   ba62e:	mov    0x50(%rsp),%rcx
   ba633:	mov    0x20(%rsp),%r8
   ba638:	mov    %rbp,%r9
   ba63b:	call   7d3e0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   ba640:	inc    %r14
   ba643:	add    $0x4,%rbx
   ba647:	cmp    %r14,0xe8(%rsp)
   ba64f:	movzbl 0x38(%rsp),%edx
   ba654:	je     baca8 <meshoptimizer_rs::codec::raw+0x1ad8>
   ba65a:	mov    %rbx,0x58(%rsp)
   ba65f:	test   %dl,%dl
   ba661:	je     ba680 <meshoptimizer_rs::codec::raw+0x14b0>
   ba663:	cmp    0x90(%rsp),%r14
   ba66b:	jae    bade5 <meshoptimizer_rs::codec::raw+0x1c15>
   ba671:	mov    0xa8(%rsp),%rax
   ba679:	movzbl (%rax,%r14,1),%ebx
   ba67e:	jmp    ba682 <meshoptimizer_rs::codec::raw+0x14b2>
   ba680:	xor    %ebx,%ebx
   ba682:	mov    %ebx,%eax
   ba684:	and    $0x3,%al
   ba686:	movzbl %al,%ecx
   ba689:	mov    0x28(%rsp),%rsi
   ba68e:	cmp    %rsi,0x20(%rsp)
   ba693:	ja     baaf3 <meshoptimizer_rs::codec::raw+0x1923>
   ba699:	cmp    $0x2,%al
   ba69b:	je     ba6dd <meshoptimizer_rs::codec::raw+0x150d>
   ba69d:	cmp    $0x3,%ecx
   ba6a0:	jne    ba6f7 <meshoptimizer_rs::codec::raw+0x1527>
   ba6a2:	mov    %r15,%r12
   ba6a5:	mov    $0x7,%al
   ba6a7:	add    0x20(%rsp),%r12
   ba6ac:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba6b2:	cmp    0x630(%rsp),%r12
   ba6ba:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba6c0:	add    0x30(%rsp),%r15
   ba6c5:	lea    0x1f0(%rsp),%rdi
   ba6cd:	mov    %r15,%rsi
   ba6d0:	mov    0x20(%rsp),%rdx
   ba6d5:	call   *0x8fb7d(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   ba6db:	jmp    ba753 <meshoptimizer_rs::codec::raw+0x1583>
   ba6dd:	lea    0x1f0(%rsp),%rdi
   ba6e5:	xor    %esi,%esi
   ba6e7:	mov    0x20(%rsp),%rdx
   ba6ec:	call   *0x8fb76(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   ba6f2:	mov    %r15,%r12
   ba6f5:	jmp    ba753 <meshoptimizer_rs::codec::raw+0x1583>
   ba6f7:	test   %al,%al
   ba6f9:	lea    -0xabc40(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ba700:	lea    -0xabb17(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ba707:	cmove  %rcx,%rax
   ba70b:	test   %dl,%dl
   ba70d:	lea    -0xabf74(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ba714:	cmove  %rcx,%rax
   ba718:	mov    %rax,(%rsp)
   ba71c:	lea    0x40(%rsp),%rdi
   ba721:	mov    0x30(%rsp),%rsi
   ba726:	mov    0x630(%rsp),%rdx
   ba72e:	mov    %r15,%rcx
   ba731:	lea    0x1f0(%rsp),%r8
   ba739:	mov    0x28(%rsp),%r9
   ba73e:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   ba743:	cmpb   $0x0,0x40(%rsp)
   ba748:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   ba74e:	mov    0x48(%rsp),%r12
   ba753:	mov    %ebx,%eax
   ba755:	shr    $0x2,%al
   ba758:	and    $0x3,%al
   ba75a:	cmp    $0x3,%al
   ba75c:	movzbl 0x38(%rsp),%edx
   ba761:	je     ba785 <meshoptimizer_rs::codec::raw+0x15b5>
   ba763:	movzbl %al,%ecx
   ba766:	cmp    $0x2,%ecx
   ba769:	jne    ba7c3 <meshoptimizer_rs::codec::raw+0x15f3>
   ba76b:	mov    0x80(%rsp),%rdi
   ba773:	xor    %esi,%esi
   ba775:	mov    0x20(%rsp),%rdx
   ba77a:	call   *0x8fae8(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   ba780:	jmp    ba81f <meshoptimizer_rs::codec::raw+0x164f>
   ba785:	mov    %r12,%r15
   ba788:	mov    $0x7,%al
   ba78a:	add    0x20(%rsp),%r15
   ba78f:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba795:	cmp    0x630(%rsp),%r15
   ba79d:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba7a3:	add    0x30(%rsp),%r12
   ba7a8:	mov    0x80(%rsp),%rdi
   ba7b0:	mov    %r12,%rsi
   ba7b3:	mov    0x20(%rsp),%rdx
   ba7b8:	call   *0x8fa9a(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   ba7be:	mov    %r15,%r12
   ba7c1:	jmp    ba81f <meshoptimizer_rs::codec::raw+0x164f>
   ba7c3:	test   %al,%al
   ba7c5:	lea    -0xabd0c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ba7cc:	lea    -0xabbe3(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ba7d3:	cmove  %rcx,%rax
   ba7d7:	test   %dl,%dl
   ba7d9:	lea    -0xac040(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ba7e0:	cmove  %rcx,%rax
   ba7e4:	mov    %rax,(%rsp)
   ba7e8:	lea    0x40(%rsp),%rdi
   ba7ed:	mov    0x30(%rsp),%rsi
   ba7f2:	mov    0x630(%rsp),%rdx
   ba7fa:	mov    %r12,%rcx
   ba7fd:	mov    0x80(%rsp),%r8
   ba805:	mov    0x28(%rsp),%r9
   ba80a:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   ba80f:	cmpb   $0x0,0x40(%rsp)
   ba814:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   ba81a:	mov    0x48(%rsp),%r12
   ba81f:	mov    %ebx,%eax
   ba821:	shr    $0x4,%al
   ba824:	and    $0x3,%al
   ba826:	cmp    $0x3,%al
   ba828:	je     ba876 <meshoptimizer_rs::codec::raw+0x16a6>
   ba82a:	movzbl %al,%ecx
   ba82d:	cmp    $0x2,%ecx
   ba830:	jne    ba8f2 <meshoptimizer_rs::codec::raw+0x1722>
   ba836:	mov    0x78(%rsp),%rdi
   ba83b:	xor    %esi,%esi
   ba83d:	mov    0x20(%rsp),%rdx
   ba842:	call   *0x8fa20(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   ba848:	shr    $0x6,%bl
   ba84b:	cmp    $0x3,%bl
   ba84e:	je     ba8b7 <meshoptimizer_rs::codec::raw+0x16e7>
   ba850:	movzbl %bl,%eax
   ba853:	cmp    $0x2,%eax
   ba856:	jne    ba95f <meshoptimizer_rs::codec::raw+0x178f>
   ba85c:	mov    0x70(%rsp),%rdi
   ba861:	xor    %esi,%esi
   ba863:	mov    0x20(%rsp),%rdx
   ba868:	call   *0x8f9fa(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   ba86e:	mov    %r12,%r15
   ba871:	jmp    ba9bb <meshoptimizer_rs::codec::raw+0x17eb>
   ba876:	mov    %r12,%r15
   ba879:	mov    $0x7,%al
   ba87b:	add    0x20(%rsp),%r15
   ba880:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba886:	cmp    0x630(%rsp),%r15
   ba88e:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba894:	add    0x30(%rsp),%r12
   ba899:	mov    0x78(%rsp),%rdi
   ba89e:	mov    %r12,%rsi
   ba8a1:	mov    0x20(%rsp),%rdx
   ba8a6:	call   *0x8f9ac(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   ba8ac:	mov    %r15,%r12
   ba8af:	shr    $0x6,%bl
   ba8b2:	cmp    $0x3,%bl
   ba8b5:	jne    ba850 <meshoptimizer_rs::codec::raw+0x1680>
   ba8b7:	mov    %r12,%r15
   ba8ba:	mov    $0x7,%al
   ba8bc:	add    0x20(%rsp),%r15
   ba8c1:	jb     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba8c7:	cmp    0x630(%rsp),%r15
   ba8cf:	ja     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   ba8d5:	add    0x30(%rsp),%r12
   ba8da:	mov    0x70(%rsp),%rdi
   ba8df:	mov    %r12,%rsi
   ba8e2:	mov    0x20(%rsp),%rdx
   ba8e7:	call   *0x8f96b(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   ba8ed:	jmp    ba9bb <meshoptimizer_rs::codec::raw+0x17eb>
   ba8f2:	test   %al,%al
   ba8f4:	lea    -0xabe3b(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ba8fb:	lea    -0xabd12(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ba902:	cmove  %rcx,%rax
   ba906:	cmpb   $0x0,0x38(%rsp)
   ba90b:	lea    -0xac172(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ba912:	cmove  %rcx,%rax
   ba916:	mov    %rax,(%rsp)
   ba91a:	lea    0x40(%rsp),%rdi
   ba91f:	mov    0x30(%rsp),%rsi
   ba924:	mov    0x630(%rsp),%rdx
   ba92c:	mov    %r12,%rcx
   ba92f:	mov    0x78(%rsp),%r8
   ba934:	mov    0x28(%rsp),%r9
   ba939:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   ba93e:	cmpb   $0x0,0x40(%rsp)
   ba943:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   ba949:	mov    0x48(%rsp),%r12
   ba94e:	shr    $0x6,%bl
   ba951:	cmp    $0x3,%bl
   ba954:	jne    ba850 <meshoptimizer_rs::codec::raw+0x1680>
   ba95a:	jmp    ba8b7 <meshoptimizer_rs::codec::raw+0x16e7>
   ba95f:	test   %bl,%bl
   ba961:	lea    -0xabea8(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ba968:	lea    -0xabd7f(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ba96f:	cmove  %rcx,%rax
   ba973:	cmpb   $0x0,0x38(%rsp)
   ba978:	lea    -0xac1df(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ba97f:	cmove  %rcx,%rax
   ba983:	mov    %rax,(%rsp)
   ba987:	lea    0x40(%rsp),%rdi
   ba98c:	mov    0x30(%rsp),%rsi
   ba991:	mov    0x630(%rsp),%rdx
   ba999:	mov    %r12,%rcx
   ba99c:	mov    0x70(%rsp),%r8
   ba9a1:	mov    0x28(%rsp),%r9
   ba9a6:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   ba9ab:	cmpb   $0x0,0x40(%rsp)
   ba9b0:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   ba9b6:	mov    0x48(%rsp),%r15
   ba9bb:	cmpb   $0x0,0x38(%rsp)
   ba9c0:	mov    0x88(%rsp),%r12
   ba9c8:	mov    0xb8(%rsp),%rsi
   ba9d0:	mov    0x58(%rsp),%rbx
   ba9d5:	je     ba9fc <meshoptimizer_rs::codec::raw+0x182c>
   ba9d7:	cmp    0xa0(%rsp),%r14
   ba9df:	jae    badfd <meshoptimizer_rs::codec::raw+0x1c2d>
   ba9e5:	mov    0x60(%rsp),%rax
   ba9ea:	movzbl (%rax,%r14,1),%eax
   ba9ef:	mov    %rsi,%rdx
   ba9f2:	sub    %rbx,%rdx
   ba9f5:	jae    baa0a <meshoptimizer_rs::codec::raw+0x183a>
   ba9f7:	jmp    bad5a <meshoptimizer_rs::codec::raw+0x1b8a>
   ba9fc:	xor    %eax,%eax
   ba9fe:	mov    %rsi,%rdx
   baa01:	sub    %rbx,%rdx
   baa04:	jb     bad5a <meshoptimizer_rs::codec::raw+0x1b8a>
   baa0a:	cmp    %rdx,0x50(%rsp)
   baa0f:	ja     bad46 <meshoptimizer_rs::codec::raw+0x1b76>
   baa15:	mov    0xb0(%rsp),%rcx
   baa1d:	lea    (%rcx,%rbx,1),%rdx
   baa21:	mov    %eax,%ecx
   baa23:	and    $0x3,%cl
   baa26:	movzbl %cl,%ecx
   baa29:	lea    -0xa92a0(%rip),%rsi        # 11790 <libm::math::generic::sqrt::RSQRT_TAB+0x4e6>
   baa30:	movslq (%rsi,%rcx,4),%rcx
   baa34:	add    %rsi,%rcx
   baa37:	jmp    *%rcx
   baa39:	mov    %rdx,%r12
   baa3c:	add    %rsp,%rbx
   baa3f:	add    $0xf0,%rbx
   baa46:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   baa4b:	test   %al,%al
   baa4d:	je     baab2 <meshoptimizer_rs::codec::raw+0x18e2>
   baa4f:	mov    %rbx,(%rsp)
   baa53:	lea    0x1f0(%rsp),%rdi
   baa5b:	mov    0x68(%rsp),%rsi
   baa60:	mov    %r12,%rdx
   baa63:	mov    0x50(%rsp),%rcx
   baa68:	mov    0x20(%rsp),%r8
   baa6d:	mov    %rbp,%r9
   baa70:	call   7f930 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   baa75:	jmp    baae1 <meshoptimizer_rs::codec::raw+0x1911>
   baa77:	lea    (%rsp,%rbx,1),%rax
   baa7b:	add    $0xf0,%rax
   baa81:	mov    %rax,(%rsp)
   baa85:	movq   $0x0,0x8(%rsp)
   baa8e:	lea    0x1f0(%rsp),%rdi
   baa96:	mov    0x68(%rsp),%rsi
   baa9b:	mov    0x50(%rsp),%rcx
   baaa0:	mov    0x20(%rsp),%r8
   baaa5:	mov    %rbp,%r9
   baaa8:	call   7d220 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   baaad:	jmp    ba640 <meshoptimizer_rs::codec::raw+0x1470>
   baab2:	mov    %rbx,(%rsp)
   baab6:	movq   $0x0,0x8(%rsp)
   baabf:	lea    0x1f0(%rsp),%rdi
   baac7:	mov    0x68(%rsp),%rsi
   baacc:	mov    %r12,%rdx
   baacf:	mov    0x50(%rsp),%rcx
   baad4:	mov    0x20(%rsp),%r8
   baad9:	mov    %rbp,%r9
   baadc:	call   7d030 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   baae1:	mov    0x88(%rsp),%r12
   baae9:	mov    0x58(%rsp),%rbx
   baaee:	jmp    ba640 <meshoptimizer_rs::codec::raw+0x1470>
   baaf3:	cmp    $0x2,%al
   baaf5:	je     bae3b <meshoptimizer_rs::codec::raw+0x1c6b>
   baafb:	cmp    $0x3,%ecx
   baafe:	je     bae22 <meshoptimizer_rs::codec::raw+0x1c52>
   bab04:	test   %al,%al
   bab06:	lea    -0xac04d(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bab0d:	lea    -0xabf24(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bab14:	cmove  %rcx,%rax
   bab18:	test   %dl,%dl
   bab1a:	lea    -0xac381(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bab21:	cmove  %rcx,%rax
   bab25:	mov    %rax,(%rsp)
   bab29:	lea    0x40(%rsp),%rdi
   bab2e:	mov    0x30(%rsp),%rsi
   bab33:	mov    0x630(%rsp),%rdx
   bab3b:	mov    %r15,%rcx
   bab3e:	lea    0x1f0(%rsp),%r8
   bab46:	mov    0x28(%rsp),%r9
   bab4b:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   bab50:	cmpb   $0x0,0x40(%rsp)
   bab55:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   bab5b:	mov    %ebx,%eax
   bab5d:	shr    $0x2,%al
   bab60:	and    $0x3,%al
   bab62:	cmp    $0x2,%al
   bab64:	movzbl 0x38(%rsp),%edx
   bab69:	je     bae3b <meshoptimizer_rs::codec::raw+0x1c6b>
   bab6f:	movzbl %al,%ecx
   bab72:	cmp    $0x3,%ecx
   bab75:	je     bae22 <meshoptimizer_rs::codec::raw+0x1c52>
   bab7b:	test   %al,%al
   bab7d:	lea    -0xac0c4(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bab84:	lea    -0xabf9b(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bab8b:	cmove  %rcx,%rax
   bab8f:	mov    0x48(%rsp),%rcx
   bab94:	test   %dl,%dl
   bab96:	lea    -0xac3fd(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bab9d:	cmove  %rdx,%rax
   baba1:	mov    %rax,(%rsp)
   baba5:	lea    0x40(%rsp),%rdi
   babaa:	mov    0x30(%rsp),%rsi
   babaf:	mov    0x630(%rsp),%rdx
   babb7:	mov    0x80(%rsp),%r8
   babbf:	mov    0x28(%rsp),%r9
   babc4:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   babc9:	cmpb   $0x0,0x40(%rsp)
   babce:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   babd4:	mov    %ebx,%eax
   babd6:	shr    $0x4,%al
   babd9:	and    $0x3,%al
   babdb:	cmp    $0x2,%al
   babdd:	movzbl 0x38(%rsp),%edx
   babe2:	je     bae3b <meshoptimizer_rs::codec::raw+0x1c6b>
   babe8:	movzbl %al,%ecx
   babeb:	cmp    $0x3,%ecx
   babee:	je     bae22 <meshoptimizer_rs::codec::raw+0x1c52>
   babf4:	test   %al,%al
   babf6:	lea    -0xac13d(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   babfd:	lea    -0xac014(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bac04:	cmove  %rcx,%rax
   bac08:	mov    0x48(%rsp),%rcx
   bac0d:	test   %dl,%dl
   bac0f:	lea    -0xac476(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bac16:	cmove  %rdx,%rax
   bac1a:	mov    %rax,(%rsp)
   bac1e:	lea    0x40(%rsp),%rdi
   bac23:	mov    0x30(%rsp),%rsi
   bac28:	mov    0x630(%rsp),%rdx
   bac30:	mov    0x78(%rsp),%r8
   bac35:	mov    0x28(%rsp),%r9
   bac3a:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   bac3f:	cmpb   $0x0,0x40(%rsp)
   bac44:	jne    bad3f <meshoptimizer_rs::codec::raw+0x1b6f>
   bac4a:	shr    $0x6,%bl
   bac4d:	cmp    $0x2,%bl
   bac50:	movzbl 0x38(%rsp),%edx
   bac55:	je     bae3b <meshoptimizer_rs::codec::raw+0x1c6b>
   bac5b:	movzbl %bl,%eax
   bac5e:	cmp    $0x3,%eax
   bac61:	je     bae22 <meshoptimizer_rs::codec::raw+0x1c52>
   bac67:	test   %bl,%bl
   bac69:	lea    -0xac1b0(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bac70:	lea    -0xac087(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bac77:	cmove  %rcx,%rax
   bac7b:	mov    0x48(%rsp),%rcx
   bac80:	test   %dl,%dl
   bac82:	lea    -0xac4e9(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bac89:	cmove  %rdx,%rax
   bac8d:	mov    %rax,(%rsp)
   bac91:	lea    0x40(%rsp),%rdi
   bac96:	mov    0x30(%rsp),%rsi
   bac9b:	mov    0x630(%rsp),%rdx
   baca3:	jmp    ba99c <meshoptimizer_rs::codec::raw+0x17cc>
   baca8:	mov    0xb8(%rsp),%rdx
   bacb0:	mov    0xc0(%rsp),%rdi
   bacb8:	cmp    %rdx,%rdi
   bacbb:	mov    0x30(%rsp),%r14
   bacc0:	ja     badc4 <meshoptimizer_rs::codec::raw+0x1bf4>
   bacc6:	mov    0xb0(%rsp),%rsi
   bacce:	add    %rdi,%rsi
   bacd1:	lea    0xf0(%rsp),%rdi
   bacd9:	mov    %rbp,%rdx
   bacdc:	call   *0x8f576(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   bace2:	mov    0xc8(%rsp),%rcx
   bacea:	mov    %rcx,%rax
   baced:	cmp    %r13,%rcx
   bacf0:	mov    0x630(%rsp),%rbx
   bacf8:	jb     ba4bf <meshoptimizer_rs::codec::raw+0x12ef>
   bacfe:	jmp    bad06 <meshoptimizer_rs::codec::raw+0x1b36>
   bad00:	mov    $0x1,%r15d
   bad06:	sub    %r15,%rbx
   bad09:	setae  %cl
   bad0c:	cmp    0xe0(%rsp),%rbx
   bad14:	sete   %al
   bad17:	and    %cl,%al
   bad19:	neg    %al
   bad1b:	or     $0x7,%al
   bad1d:	jmp    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   bad1f:	mov    0x28(%rsp),%r9
   bad24:	mov    $0xff,%al
   bad26:	cmp    %r9,%rsi
   bad29:	je     bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   bad2b:	mov    $0x7,%al
   bad2d:	add    $0x5f8,%rsp
   bad34:	pop    %rbx
   bad35:	pop    %r12
   bad37:	pop    %r13
   bad39:	pop    %r14
   bad3b:	pop    %r15
   bad3d:	pop    %rbp
   bad3e:	ret
   bad3f:	movzbl 0x41(%rsp),%eax
   bad44:	jmp    bad2d <meshoptimizer_rs::codec::raw+0x1b5d>
   bad46:	lea    0x89833(%rip),%rcx        # 144580 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   bad4d:	xor    %edi,%edi
   bad4f:	mov    0x50(%rsp),%rsi
   bad54:	call   *0x8f656(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   bad5a:	lea    0x89837(%rip),%rcx        # 144598 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   bad61:	mov    %rsi,%rdx
   bad64:	mov    %rbx,%rdi
   bad67:	call   *0x8f643(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   bad6d:	lea    0x89884(%rip),%rcx        # 1445f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   bad74:	mov    $0x100,%edx
   bad79:	xor    %edi,%edi
   bad7b:	mov    %rbp,%rsi
   bad7e:	call   *0x8f62c(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   bad84:	lea    0x89855(%rip),%rcx        # 1445e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   bad8b:	mov    %rsi,%rdi
   bad8e:	mov    0x60(%rsp),%rsi
   bad93:	mov    %rbx,%rdx
   bad96:	call   *0x8f614(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   bad9c:	lea    0x8d895(%rip),%rcx        # 148638 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x65a8>
   bada3:	mov    %r12,%rdx
   bada6:	call   *0x8f604(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   badac:	lea    0x8985d(%rip),%rcx        # 144610 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   badb3:	mov    0x60(%rsp),%rdi
   badb8:	mov    %rbx,%rsi
   badbb:	mov    %rbx,%rdx
   badbe:	call   *0x8f5ec(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   badc4:	lea    0x8976d(%rip),%rcx        # 144538 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   badcb:	mov    %rdx,%rsi
   badce:	call   *0x8f5dc(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   badd4:	lea    0x8cfd5(%rip),%rdx        # 147db0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x5d20>
   baddb:	xor    %edi,%edi
   baddd:	xor    %esi,%esi
   baddf:	call   *0x8f4ab(%rip)        # 14a290 <_DYNAMIC+0x250>
   bade5:	lea    0x89764(%rip),%rdx        # 144550 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   badec:	mov    %r14,%rdi
   badef:	mov    0x90(%rsp),%rsi
   badf7:	call   *0x8f493(%rip)        # 14a290 <_DYNAMIC+0x250>
   badfd:	lea    0x89764(%rip),%rdx        # 144568 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   bae04:	mov    %r14,%rdi
   bae07:	mov    0xa0(%rsp),%rsi
   bae0f:	call   *0x8f47b(%rip)        # 14a290 <_DYNAMIC+0x250>
   bae15:	lea    0x89704(%rip),%rdi        # 144520 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2490>
   bae1c:	call   *0x8f656(%rip)        # 14a478 <_DYNAMIC+0x438>
   bae22:	lea    0x89787(%rip),%rcx        # 1445b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   bae29:	xor    %edi,%edi
   bae2b:	mov    0x20(%rsp),%rsi
   bae30:	mov    0x28(%rsp),%rdx
   bae35:	call   *0x8f575(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   bae3b:	lea    0x89786(%rip),%rcx        # 1445c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   bae42:	xor    %edi,%edi
   bae44:	mov    0x20(%rsp),%rsi
   bae49:	mov    0x28(%rsp),%rdx
   bae4e:	call   *0x8f55c(%rip)        # 14a3b0 <_DYNAMIC+0x370>
