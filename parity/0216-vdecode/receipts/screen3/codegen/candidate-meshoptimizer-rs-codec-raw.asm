
<artifacts>/screen3/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000ad3a0 <meshoptimizer_rs::codec::raw>:
   ad3a0:	push   %rbp
   ad3a1:	push   %r15
   ad3a3:	push   %r14
   ad3a5:	push   %r13
   ad3a7:	push   %r12
   ad3a9:	push   %rbx
   ad3aa:	sub    $0x5f8,%rsp
   ad3b1:	mov    %r9,%r14
   ad3b4:	mov    %r8,%rbp
   ad3b7:	mov    %rcx,%r13
   ad3ba:	mov    %rdx,%r12
   ad3bd:	mov    %rsi,%r15
   ad3c0:	mov    0x630(%rsp),%rbx
   ad3c8:	test   %dil,%dil
   ad3cb:	je     ad534 <meshoptimizer_rs::codec::raw+0x194>
   ad3d1:	movzbl %dil,%eax
   ad3d5:	cmp    $0x1,%eax
   ad3d8:	jne    ad5f6 <meshoptimizer_rs::codec::raw+0x256>
   ad3de:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   ad3e8:	mov    %r13,%rax
   ad3eb:	mul    %rdi
   ad3ee:	shr    $1,%rdx
   ad3f1:	lea    (%rdx,%rdx,2),%rcx
   ad3f5:	mov    %r13,%rax
   ad3f8:	sub    %rcx,%rax
   ad3fb:	cmp    $0x2,%rbp
   ad3ff:	jne    ad661 <meshoptimizer_rs::codec::raw+0x2c1>
   ad405:	test   %rax,%rax
   ad408:	jne    ad66a <meshoptimizer_rs::codec::raw+0x2ca>
   ad40e:	mov    %r13,%rax
   ad411:	mul    %rdi
   ad414:	shr    $1,%rdx
   ad417:	lea    0x11(%rdx),%rcx
   ad41b:	mov    $0x7,%al
   ad41d:	cmp    %rcx,%rbx
   ad420:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad426:	movzbl (%r14),%ecx
   ad42a:	mov    %ecx,%esi
   ad42c:	and    $0xf0,%sil
   ad430:	cmp    $0xe0,%sil
   ad434:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad43a:	and    $0xf,%cl
   ad43d:	mov    $0x5,%al
   ad43f:	cmp    $0x1,%cl
   ad442:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad448:	lea    0x1(%rdx),%rsi
   ad44c:	pcmpeqd %xmm0,%xmm0
   ad450:	movdqa %xmm0,0x260(%rsp)
   ad459:	movdqa %xmm0,0x250(%rsp)
   ad462:	movdqa %xmm0,0x240(%rsp)
   ad46b:	movdqa %xmm0,0x230(%rsp)
   ad474:	movdqa %xmm0,0x220(%rsp)
   ad47d:	movdqa %xmm0,0x210(%rsp)
   ad486:	movdqa %xmm0,0x200(%rsp)
   ad48f:	movdqa %xmm0,0x1f0(%rsp)
   ad498:	movdqa %xmm0,0x120(%rsp)
   ad4a1:	movdqa %xmm0,0x110(%rsp)
   ad4aa:	movdqa %xmm0,0x100(%rsp)
   ad4b3:	movdqa %xmm0,0xf0(%rsp)
   ad4bc:	xor    %r8d,%r8d
   ad4bf:	cmp    $0x1,%cl
   ad4c2:	setne  %r9b
   ad4c6:	lea    -0x10(%rbx),%rax
   ad4ca:	mov    %rax,0x28(%rsp)
   ad4cf:	mov    %r12,%rax
   ad4d2:	mov    %rdx,%rcx
   ad4d5:	mul    %rdi
   ad4d8:	shr    $0x2,%rdx
   ad4dc:	cmp    %rcx,%rdx
   ad4df:	cmovae %rcx,%rdx
   ad4e3:	mov    %rdx,0x38(%rsp)
   ad4e8:	test   %rdx,%rdx
   ad4eb:	je     aeeef <meshoptimizer_rs::codec::raw+0x1b4f>
   ad4f1:	mov    %r9b,%r8b
   ad4f4:	lea    0xd(,%r8,2),%rax
   ad4fc:	mov    %rax,0x58(%rsp)
   ad501:	mov    %r14,0x30(%rsp)
   ad506:	lea    (%r14,%rbx,1),%rax
   ad50a:	add    $0xfffffffffffffff0,%rax
   ad50e:	mov    %rax,0x50(%rsp)
   ad513:	add    $0x4,%r15
   ad517:	xor    %r8d,%r8d
   ad51a:	xor    %r10d,%r10d
   ad51d:	movl   $0x0,0x20(%rsp)
   ad525:	xor    %r11d,%r11d
   ad528:	xor    %edi,%edi
   ad52a:	mov    0x28(%rsp),%r9
   ad52f:	jmp    ad9f6 <meshoptimizer_rs::codec::raw+0x656>
   ad534:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   ad539:	cmp    $0x2,%al
   ad53b:	jb     ad5a4 <meshoptimizer_rs::codec::raw+0x204>
   ad53d:	cmp    $0x20,%rbp
   ad541:	lea    -0x34138(%rip),%rax        # 79410 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   ad548:	lea    -0x3561f(%rip),%rcx        # 77f30 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   ad54f:	cmove  %rax,%rcx
   ad553:	lea    -0x2dc5a(%rip),%rax        # 7f900 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   ad55a:	lea    -0x2f501(%rip),%rdx        # 7e060 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   ad561:	cmove  %rax,%rdx
   ad565:	cmp    $0x4,%rbp
   ad569:	lea    -0x31ad0(%rip),%rax        # 7baa0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   ad570:	cmovne %rcx,%rax
   ad574:	lea    -0x2b1db(%rip),%r10        # 823a0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   ad57b:	cmovne %rdx,%r10
   ad57f:	cmp    $0x21,%r13
   ad583:	cmovae %rax,%r10
   ad587:	mov    %r15,%rdi
   ad58a:	mov    %r12,%rsi
   ad58d:	mov    %r13,%rdx
   ad590:	mov    %rbp,%rcx
   ad593:	mov    %r14,%r8
   ad596:	mov    %rbx,%r9
   ad599:	call   *%r10
   ad59c:	cmp    $0xfe,%al
   ad59e:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad5a4:	mov    $0x7,%al
   ad5a6:	test   %rbx,%rbx
   ad5a9:	je     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad5af:	movzbl (%r14),%edi
   ad5b3:	mov    %edi,%ecx
   ad5b5:	and    $0xf0,%cl
   ad5b8:	cmp    $0xa0,%cl
   ad5bb:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad5c1:	and    $0xf,%dil
   ad5c5:	mov    $0x5,%al
   ad5c7:	cmp    $0x1,%dil
   ad5cb:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad5d1:	test   %dil,%dil
   ad5d4:	je     ae4f3 <meshoptimizer_rs::codec::raw+0x1153>
   ad5da:	mov    %rbp,%rcx
   ad5dd:	shr    $0x2,%rcx
   ad5e1:	add    %rbp,%rcx
   ad5e4:	cmp    $0x19,%rcx
   ad5e8:	mov    $0x18,%eax
   ad5ed:	cmovae %rcx,%rax
   ad5f1:	jmp    ae503 <meshoptimizer_rs::codec::raw+0x1163>
   ad5f6:	add    $0x5,%r13
   ad5fa:	mov    $0x7,%al
   ad5fc:	cmp    $0x2,%rbp
   ad600:	jne    ad671 <meshoptimizer_rs::codec::raw+0x2d1>
   ad602:	cmp    %r13,%rbx
   ad605:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad60b:	test   %rbx,%rbx
   ad60e:	je     aefa4 <meshoptimizer_rs::codec::raw+0x1c04>
   ad614:	movzbl (%r14),%ecx
   ad618:	mov    %ecx,%edx
   ad61a:	and    $0xf0,%dl
   ad61d:	cmp    $0xd0,%dl
   ad620:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad626:	mov    $0x5,%al
   ad628:	test   $0xe,%cl
   ad62b:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad631:	dec    %rbx
   ad634:	movq   $0x0,0x1f0(%rsp)
   ad640:	movabs $0x7ffffffffffffffc,%rax
   ad64a:	or     $0x2,%rax
   ad64e:	and    %rax,%r12
   ad651:	je     ad954 <meshoptimizer_rs::codec::raw+0x5b4>
   ad657:	inc    %r14
   ad65a:	xor    %eax,%eax
   ad65c:	jmp    ad706 <meshoptimizer_rs::codec::raw+0x366>
   ad661:	test   %rax,%rax
   ad664:	je     ad837 <meshoptimizer_rs::codec::raw+0x497>
   ad66a:	xor    %eax,%eax
   ad66c:	jmp    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad671:	cmp    %r13,%rbx
   ad674:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad67a:	test   %rbx,%rbx
   ad67d:	je     aefa4 <meshoptimizer_rs::codec::raw+0x1c04>
   ad683:	movzbl (%r14),%ecx
   ad687:	mov    %ecx,%edx
   ad689:	and    $0xf0,%dl
   ad68c:	cmp    $0xd0,%dl
   ad68f:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad695:	mov    $0x5,%al
   ad697:	test   $0xe,%cl
   ad69a:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad6a0:	dec    %rbx
   ad6a3:	movq   $0x0,0x1f0(%rsp)
   ad6af:	movabs $0x7ffffffffffffffc,%rax
   ad6b9:	and    %rax,%r12
   ad6bc:	je     ad954 <meshoptimizer_rs::codec::raw+0x5b4>
   ad6c2:	inc    %r14
   ad6c5:	xor    %eax,%eax
   ad6c7:	jmp    ad7bb <meshoptimizer_rs::codec::raw+0x41b>
   ad6cc:	dec    %rbx
   ad6cf:	inc    %r14
   ad6d2:	mov    %edx,%ecx
   ad6d4:	mov    %ecx,%edx
   ad6d6:	shr    $0x2,%edx
   ad6d9:	mov    %ecx,%esi
   ad6db:	shl    $0x1e,%esi
   ad6de:	sar    $0x1f,%esi
   ad6e1:	xor    %edx,%esi
   ad6e3:	and    $0x1,%ecx
   ad6e6:	add    0x1f0(%rsp,%rcx,4),%esi
   ad6ed:	mov    %esi,0x1f0(%rsp,%rcx,4)
   ad6f4:	mov    %si,(%r15,%rax,1)
   ad6f9:	add    $0x2,%rax
   ad6fd:	cmp    %rax,%r12
   ad700:	je     ad954 <meshoptimizer_rs::codec::raw+0x5b4>
   ad706:	cmp    $0x5,%rbx
   ad70a:	jb     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ad710:	movzbl (%r14),%edx
   ad714:	test   %dl,%dl
   ad716:	jns    ad6cc <meshoptimizer_rs::codec::raw+0x32c>
   ad718:	and    $0x7f,%edx
   ad71b:	movzbl 0x1(%r14),%ecx
   ad720:	mov    %ecx,%esi
   ad722:	and    $0x7f,%ecx
   ad725:	shl    $0x7,%ecx
   ad728:	or     %edx,%ecx
   ad72a:	mov    $0x2,%edx
   ad72f:	test   %sil,%sil
   ad732:	jns    ad777 <meshoptimizer_rs::codec::raw+0x3d7>
   ad734:	movzbl 0x2(%r14),%edx
   ad739:	mov    %edx,%esi
   ad73b:	and    $0x7f,%edx
   ad73e:	shl    $0xe,%edx
   ad741:	or     %edx,%ecx
   ad743:	mov    $0x3,%edx
   ad748:	test   %sil,%sil
   ad74b:	jns    ad777 <meshoptimizer_rs::codec::raw+0x3d7>
   ad74d:	movzbl 0x3(%r14),%edx
   ad752:	mov    %edx,%esi
   ad754:	and    $0x7f,%edx
   ad757:	shl    $0x15,%edx
   ad75a:	or     %edx,%ecx
   ad75c:	mov    $0x4,%edx
   ad761:	test   %sil,%sil
   ad764:	jns    ad777 <meshoptimizer_rs::codec::raw+0x3d7>
   ad766:	movzbl 0x4(%r14),%esi
   ad76b:	shl    $0x1c,%esi
   ad76e:	or     %ecx,%esi
   ad770:	mov    $0x5,%edx
   ad775:	mov    %esi,%ecx
   ad777:	sub    %rdx,%rbx
   ad77a:	add    %rdx,%r14
   ad77d:	jmp    ad6d4 <meshoptimizer_rs::codec::raw+0x334>
   ad782:	dec    %rbx
   ad785:	inc    %r14
   ad788:	mov    %edx,%ecx
   ad78a:	mov    %ecx,%edx
   ad78c:	shr    $0x2,%edx
   ad78f:	mov    %ecx,%esi
   ad791:	shl    $0x1e,%esi
   ad794:	sar    $0x1f,%esi
   ad797:	xor    %edx,%esi
   ad799:	and    $0x1,%ecx
   ad79c:	add    0x1f0(%rsp,%rcx,4),%esi
   ad7a3:	mov    %esi,0x1f0(%rsp,%rcx,4)
   ad7aa:	mov    %esi,(%r15,%rax,1)
   ad7ae:	add    $0x4,%rax
   ad7b2:	cmp    %rax,%r12
   ad7b5:	je     ad954 <meshoptimizer_rs::codec::raw+0x5b4>
   ad7bb:	cmp    $0x5,%rbx
   ad7bf:	jb     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ad7c5:	movzbl (%r14),%edx
   ad7c9:	test   %dl,%dl
   ad7cb:	jns    ad782 <meshoptimizer_rs::codec::raw+0x3e2>
   ad7cd:	and    $0x7f,%edx
   ad7d0:	movzbl 0x1(%r14),%ecx
   ad7d5:	mov    %ecx,%esi
   ad7d7:	and    $0x7f,%ecx
   ad7da:	shl    $0x7,%ecx
   ad7dd:	or     %edx,%ecx
   ad7df:	mov    $0x2,%edx
   ad7e4:	test   %sil,%sil
   ad7e7:	jns    ad82c <meshoptimizer_rs::codec::raw+0x48c>
   ad7e9:	movzbl 0x2(%r14),%edx
   ad7ee:	mov    %edx,%esi
   ad7f0:	and    $0x7f,%edx
   ad7f3:	shl    $0xe,%edx
   ad7f6:	or     %edx,%ecx
   ad7f8:	mov    $0x3,%edx
   ad7fd:	test   %sil,%sil
   ad800:	jns    ad82c <meshoptimizer_rs::codec::raw+0x48c>
   ad802:	movzbl 0x3(%r14),%edx
   ad807:	mov    %edx,%esi
   ad809:	and    $0x7f,%edx
   ad80c:	shl    $0x15,%edx
   ad80f:	or     %edx,%ecx
   ad811:	mov    $0x4,%edx
   ad816:	test   %sil,%sil
   ad819:	jns    ad82c <meshoptimizer_rs::codec::raw+0x48c>
   ad81b:	movzbl 0x4(%r14),%esi
   ad820:	shl    $0x1c,%esi
   ad823:	or     %ecx,%esi
   ad825:	mov    $0x5,%edx
   ad82a:	mov    %esi,%ecx
   ad82c:	sub    %rdx,%rbx
   ad82f:	add    %rdx,%r14
   ad832:	jmp    ad78a <meshoptimizer_rs::codec::raw+0x3ea>
   ad837:	lea    0x11(%rdx),%rcx
   ad83b:	mov    $0x7,%al
   ad83d:	cmp    %rcx,%rbx
   ad840:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad846:	movzbl (%r14),%ecx
   ad84a:	mov    %ecx,%esi
   ad84c:	and    $0xf0,%sil
   ad850:	cmp    $0xe0,%sil
   ad854:	jne    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad85a:	and    $0xf,%cl
   ad85d:	mov    $0x5,%al
   ad85f:	cmp    $0x1,%cl
   ad862:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ad868:	lea    0x1(%rdx),%rsi
   ad86c:	pcmpeqd %xmm0,%xmm0
   ad870:	movdqa %xmm0,0x260(%rsp)
   ad879:	movdqa %xmm0,0x250(%rsp)
   ad882:	movdqa %xmm0,0x240(%rsp)
   ad88b:	movdqa %xmm0,0x230(%rsp)
   ad894:	movdqa %xmm0,0x220(%rsp)
   ad89d:	movdqa %xmm0,0x210(%rsp)
   ad8a6:	movdqa %xmm0,0x200(%rsp)
   ad8af:	movdqa %xmm0,0x1f0(%rsp)
   ad8b8:	movdqa %xmm0,0x120(%rsp)
   ad8c1:	movdqa %xmm0,0x110(%rsp)
   ad8ca:	movdqa %xmm0,0x100(%rsp)
   ad8d3:	movdqa %xmm0,0xf0(%rsp)
   ad8dc:	xor    %r8d,%r8d
   ad8df:	cmp    $0x1,%cl
   ad8e2:	setne  %r9b
   ad8e6:	lea    -0x10(%rbx),%rax
   ad8ea:	mov    %rax,0x28(%rsp)
   ad8ef:	mov    %r12,%rax
   ad8f2:	mov    %rdx,%rcx
   ad8f5:	mul    %rdi
   ad8f8:	shr    $0x3,%rdx
   ad8fc:	cmp    %rcx,%rdx
   ad8ff:	cmovae %rcx,%rdx
   ad903:	mov    %rdx,0x38(%rsp)
   ad908:	test   %rdx,%rdx
   ad90b:	je     aeeef <meshoptimizer_rs::codec::raw+0x1b4f>
   ad911:	mov    %r9b,%r8b
   ad914:	lea    0xd(,%r8,2),%rax
   ad91c:	mov    %rax,0x58(%rsp)
   ad921:	mov    %r14,0x30(%rsp)
   ad926:	lea    (%r14,%rbx,1),%rax
   ad92a:	add    $0xfffffffffffffff0,%rax
   ad92e:	mov    %rax,0x50(%rsp)
   ad933:	add    $0x8,%r15
   ad937:	xor    %r8d,%r8d
   ad93a:	xor    %r10d,%r10d
   ad93d:	movl   $0x0,0x20(%rsp)
   ad945:	xor    %r11d,%r11d
   ad948:	xor    %edi,%edi
   ad94a:	mov    0x28(%rsp),%r9
   ad94f:	jmp    adfbe <meshoptimizer_rs::codec::raw+0xc1e>
   ad954:	cmp    $0x4,%rbx
   ad958:	sete   %al
   ad95b:	jmp    aeee9 <meshoptimizer_rs::codec::raw+0x1b49>
   ad960:	mov    %bpl,%r14b
   ad963:	add    %r14d,%r11d
   ad966:	mov    %ebx,0xf0(%rsp,%r10,4)
   ad96e:	mov    %r11d,%ebp
   ad971:	not    %eax
   ad973:	shr    $0x4,%eax
   ad976:	add    %r8d,%eax
   ad979:	and    $0xf,%eax
   ad97c:	mov    0x1f0(%rsp,%rax,8),%r11d
   ad984:	mov    0x1f4(%rsp,%rax,8),%r15d
   ad98c:	add    %r14,%r10
   ad98f:	mov    $0x2,%eax
   ad994:	mov    $0x1,%r12d
   ad99a:	mov    %r8,%r13
   ad99d:	inc    %rdi
   ad9a0:	mov    %ebx,0x1f0(%rsp,%r13,8)
   ad9a8:	mov    %r15d,0x1f4(%rsp,%r13,8)
   ad9b0:	add    %r8d,%r12d
   ad9b3:	and    $0xf,%r12d
   ad9b7:	mov    %r11d,0x1f0(%rsp,%r12,8)
   ad9bf:	mov    %ebx,0x1f4(%rsp,%r12,8)
   ad9c7:	add    %r8d,%eax
   ad9ca:	and    $0xf,%eax
   ad9cd:	and    $0xf,%r10d
   ad9d1:	mov    %r11w,-0x4(%rdx)
   ad9d6:	mov    %r15w,-0x2(%rdx)
   ad9db:	mov    %rdx,%r15
   ad9de:	mov    %bx,(%rdx)
   ad9e1:	add    $0x6,%r15
   ad9e5:	mov    %rax,%r8
   ad9e8:	mov    %ebp,%r11d
   ad9eb:	cmp    %rdi,0x38(%rsp)
   ad9f0:	je     aeef4 <meshoptimizer_rs::codec::raw+0x1b54>
   ad9f6:	cmp    %r9,%rsi
   ad9f9:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ad9ff:	mov    %r15,%rdx
   ada02:	mov    0x30(%rsp),%r14
   ada07:	movzbl 0x1(%r14,%rdi,1),%eax
   ada0d:	cmp    $0xf0,%al
   ada0f:	jae    ada49 <meshoptimizer_rs::codec::raw+0x6a9>
   ada11:	mov    %eax,%r15d
   ada14:	and    $0xf,%r15d
   ada18:	cmp    %r15d,0x58(%rsp)
   ada1d:	jbe    ada83 <meshoptimizer_rs::codec::raw+0x6e3>
   ada1f:	xor    %r14d,%r14d
   ada22:	mov    %r11d,%ebx
   ada25:	test   %r15b,%r15b
   ada28:	sete   %bpl
   ada2c:	je     ad960 <meshoptimizer_rs::codec::raw+0x5c0>
   ada32:	not    %r15d
   ada35:	add    %r10d,%r15d
   ada38:	and    $0xf,%r15d
   ada3c:	mov    0xf0(%rsp,%r15,4),%ebx
   ada44:	jmp    ad960 <meshoptimizer_rs::codec::raw+0x5c0>
   ada49:	cmp    $0xfe,%al
   ada4b:	jae    adaa5 <meshoptimizer_rs::codec::raw+0x705>
   ada4d:	and    $0xf,%eax
   ada50:	mov    0x50(%rsp),%rcx
   ada55:	movzbl (%rcx,%rax,1),%r13d
   ada5a:	lea    0x1(%r11),%r15d
   ada5e:	xor    %eax,%eax
   ada60:	mov    %r13d,%ecx
   ada63:	shr    $0x4,%ecx
   ada66:	sete   %r12b
   ada6a:	je     adae8 <meshoptimizer_rs::codec::raw+0x748>
   ada6c:	mov    %r10d,%ebx
   ada6f:	sub    %ecx,%ebx
   ada71:	and    $0xf,%ebx
   ada74:	mov    %r15d,%ebp
   ada77:	mov    0xf0(%rsp,%rbx,4),%ecx
   ada7e:	mov    %ecx,%r15d
   ada81:	jmp    adaec <meshoptimizer_rs::codec::raw+0x74c>
   ada83:	cmp    $0xf,%r15b
   ada87:	jne    adb68 <meshoptimizer_rs::codec::raw+0x7c8>
   ada8d:	movzbl (%r14,%rsi,1),%ebp
   ada92:	test   %bpl,%bpl
   ada95:	js     adbd6 <meshoptimizer_rs::codec::raw+0x836>
   ada9b:	inc    %rsi
   ada9e:	mov    %ebp,%ebx
   adaa0:	jmp    add6c <meshoptimizer_rs::codec::raw+0x9cc>
   adaa5:	movzbl (%r14,%rsi,1),%r14d
   adaaa:	xor    %r15d,%r15d
   adaad:	cmp    $0xfe,%eax
   adab2:	sete   %r15b
   adab6:	test   %r14d,%r14d
   adab9:	cmove  %r14d,%r11d
   adabd:	add    %r11d,%r15d
   adac0:	mov    %r14d,%r12d
   adac3:	shr    $0x4,%r12d
   adac7:	je     adb83 <meshoptimizer_rs::codec::raw+0x7e3>
   adacd:	mov    %r10d,%ecx
   adad0:	sub    %r12d,%ecx
   adad3:	and    $0xf,%ecx
   adad6:	mov    %r15d,%ebx
   adad9:	mov    0xf0(%rsp,%rcx,4),%ecx
   adae0:	mov    %ecx,%r15d
   adae3:	jmp    adb87 <meshoptimizer_rs::codec::raw+0x7e7>
   adae8:	lea    0x2(%r11),%ebp
   adaec:	xor    %r14d,%r14d
   adaef:	mov    %ebp,%ebx
   adaf1:	test   $0xf,%r13b
   adaf5:	sete   %cl
   adaf8:	je     adb0a <meshoptimizer_rs::codec::raw+0x76a>
   adafa:	mov    %r10d,%ebx
   adafd:	sub    %r13d,%ebx
   adb00:	and    $0xf,%ebx
   adb03:	mov    0xf0(%rsp,%rbx,4),%ebx
   adb0a:	mov    %cl,%r14b
   adb0d:	add    %r14d,%ebp
   adb10:	mov    %r11d,0xf0(%rsp,%r10,4)
   adb18:	lea    0x1(%r10),%rcx
   adb1c:	and    $0xf,%ecx
   adb1f:	mov    %r15d,0xf0(%rsp,%rcx,4)
   adb27:	mov    %r12b,%al
   adb2a:	add    %r10,%rax
   adb2d:	inc    %rax
   adb30:	add    %rax,%r14
   adb33:	and    $0xf,%eax
   adb36:	mov    %ebx,0xf0(%rsp,%rax,4)
   adb3d:	mov    %r15d,0x1f0(%rsp,%r8,8)
   adb45:	mov    %r11d,0x1f4(%rsp,%r8,8)
   adb4d:	lea    0x1(%r8),%r13d
   adb51:	and    $0xf,%r13d
   adb55:	mov    $0x3,%eax
   adb5a:	mov    $0x2,%r12d
   adb60:	mov    %r14,%r10
   adb63:	jmp    ad99d <meshoptimizer_rs::codec::raw+0x5fd>
   adb68:	movzbl %r15b,%ecx
   adb6c:	xor    $0x3,%r15b
   adb70:	movzbl %r15b,%ebx
   adb74:	mov    0x20(%rsp),%ebp
   adb78:	add    %ecx,%ebp
   adb7a:	sub    %ebx,%ebp
   adb7c:	mov    %ebp,%ebx
   adb7e:	jmp    add7d <meshoptimizer_rs::codec::raw+0x9dd>
   adb83:	lea    0x1(%r15),%ebx
   adb87:	mov    %r14d,%r13d
   adb8a:	and    $0xf,%r13b
   adb8e:	je     adba6 <meshoptimizer_rs::codec::raw+0x806>
   adb90:	mov    %r10d,%ecx
   adb93:	sub    %r14d,%ecx
   adb96:	and    $0xf,%ecx
   adb99:	mov    %ebx,%ebp
   adb9b:	mov    0xf0(%rsp,%rcx,4),%ecx
   adba2:	mov    %ecx,%ebx
   adba4:	jmp    adba9 <meshoptimizer_rs::codec::raw+0x809>
   adba6:	lea    0x1(%rbx),%ebp
   adba9:	lea    0x1(%rsi),%r14
   adbad:	cmp    $0xfe,%eax
   adbb2:	je     adcb0 <meshoptimizer_rs::codec::raw+0x910>
   adbb8:	mov    0x30(%rsp),%r9
   adbbd:	movzbl 0x1(%r9,%rsi,1),%eax
   adbc3:	test   %al,%al
   adbc5:	js     adbf6 <meshoptimizer_rs::codec::raw+0x856>
   adbc7:	add    $0x2,%rsi
   adbcb:	mov    %rsi,%r14
   adbce:	mov    %eax,%r11d
   adbd1:	jmp    adc92 <meshoptimizer_rs::codec::raw+0x8f2>
   adbd6:	and    $0x7f,%ebp
   adbd9:	movzbl 0x1(%r14,%rsi,1),%ebx
   adbdf:	mov    %ebx,%ecx
   adbe1:	and    $0x7f,%ebx
   adbe4:	shl    $0x7,%ebx
   adbe7:	or     %ebp,%ebx
   adbe9:	test   %cl,%cl
   adbeb:	js     adc17 <meshoptimizer_rs::codec::raw+0x877>
   adbed:	add    $0x2,%rsi
   adbf1:	jmp    add6c <meshoptimizer_rs::codec::raw+0x9cc>
   adbf6:	and    $0x7f,%eax
   adbf9:	movzbl 0x2(%r9,%rsi,1),%r11d
   adbff:	mov    %r11d,%ecx
   adc02:	and    $0x7f,%r11d
   adc06:	shl    $0x7,%r11d
   adc0a:	or     %eax,%r11d
   adc0d:	test   %cl,%cl
   adc0f:	js     adc35 <meshoptimizer_rs::codec::raw+0x895>
   adc11:	add    $0x3,%rsi
   adc15:	jmp    adc8f <meshoptimizer_rs::codec::raw+0x8ef>
   adc17:	movzbl 0x2(%r14,%rsi,1),%ecx
   adc1d:	mov    %ecx,%ebp
   adc1f:	and    $0x7f,%ecx
   adc22:	shl    $0xe,%ecx
   adc25:	or     %ecx,%ebx
   adc27:	test   %bpl,%bpl
   adc2a:	js     adc50 <meshoptimizer_rs::codec::raw+0x8b0>
   adc2c:	add    $0x3,%rsi
   adc30:	jmp    add6c <meshoptimizer_rs::codec::raw+0x9cc>
   adc35:	movzbl 0x3(%r9,%rsi,1),%eax
   adc3b:	mov    %eax,%ecx
   adc3d:	and    $0x7f,%eax
   adc40:	shl    $0xe,%eax
   adc43:	or     %eax,%r11d
   adc46:	test   %cl,%cl
   adc48:	js     adc72 <meshoptimizer_rs::codec::raw+0x8d2>
   adc4a:	add    $0x4,%rsi
   adc4e:	jmp    adc8f <meshoptimizer_rs::codec::raw+0x8ef>
   adc50:	movzbl 0x3(%r14,%rsi,1),%ecx
   adc56:	mov    %ecx,%ebp
   adc58:	and    $0x7f,%ecx
   adc5b:	shl    $0x15,%ecx
   adc5e:	or     %ecx,%ebx
   adc60:	test   %bpl,%bpl
   adc63:	js     add5b <meshoptimizer_rs::codec::raw+0x9bb>
   adc69:	add    $0x4,%rsi
   adc6d:	jmp    add6c <meshoptimizer_rs::codec::raw+0x9cc>
   adc72:	movzbl 0x4(%r9,%rsi,1),%eax
   adc78:	mov    %eax,%ecx
   adc7a:	and    $0x7f,%eax
   adc7d:	shl    $0x15,%eax
   adc80:	or     %eax,%r11d
   adc83:	test   %cl,%cl
   adc85:	js     add94 <meshoptimizer_rs::codec::raw+0x9f4>
   adc8b:	add    $0x5,%rsi
   adc8f:	mov    %rsi,%r14
   adc92:	mov    0x28(%rsp),%r9
   adc97:	mov    %r11d,%eax
   adc9a:	shr    $1,%eax
   adc9c:	and    $0x1,%r11d
   adca0:	neg    %r11d
   adca3:	xor    %eax,%r11d
   adca6:	add    0x20(%rsp),%r11d
   adcab:	mov    %r11d,0x20(%rsp)
   adcb0:	cmp    $0xf,%r12b
   adcb4:	jne    adde3 <meshoptimizer_rs::codec::raw+0xa43>
   adcba:	cmp    $0xfffffffffffffffa,%r14
   adcbe:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   adcc4:	lea    0x5(%r14),%rax
   adcc8:	cmp    0x630(%rsp),%rax
   adcd0:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   adcd6:	mov    0x30(%rsp),%r9
   adcdb:	movzbl (%r9,%r14,1),%esi
   adce0:	test   %sil,%sil
   adce3:	js     adcf3 <meshoptimizer_rs::codec::raw+0x953>
   adce5:	inc    %r14
   adce8:	mov    %r14,%rax
   adceb:	mov    %esi,%r15d
   adcee:	jmp    addc2 <meshoptimizer_rs::codec::raw+0xa22>
   adcf3:	and    $0x7f,%esi
   adcf6:	movzbl 0x1(%r9,%r14,1),%r15d
   adcfc:	mov    %r15d,%ecx
   adcff:	and    $0x7f,%r15d
   add03:	shl    $0x7,%r15d
   add07:	or     %esi,%r15d
   add0a:	test   %cl,%cl
   add0c:	js     add1a <meshoptimizer_rs::codec::raw+0x97a>
   add0e:	add    $0x2,%r14
   add12:	mov    %r14,%rax
   add15:	jmp    addc2 <meshoptimizer_rs::codec::raw+0xa22>
   add1a:	movzbl 0x2(%r9,%r14,1),%ecx
   add20:	mov    %ecx,%esi
   add22:	and    $0x7f,%ecx
   add25:	shl    $0xe,%ecx
   add28:	or     %ecx,%r15d
   add2b:	test   %sil,%sil
   add2e:	js     add3c <meshoptimizer_rs::codec::raw+0x99c>
   add30:	add    $0x3,%r14
   add34:	mov    %r14,%rax
   add37:	jmp    addc2 <meshoptimizer_rs::codec::raw+0xa22>
   add3c:	movzbl 0x3(%r9,%r14,1),%ecx
   add42:	mov    %ecx,%esi
   add44:	and    $0x7f,%ecx
   add47:	shl    $0x15,%ecx
   add4a:	or     %ecx,%r15d
   add4d:	test   %sil,%sil
   add50:	js     addae <meshoptimizer_rs::codec::raw+0xa0e>
   add52:	add    $0x4,%r14
   add56:	mov    %r14,%rax
   add59:	jmp    addc2 <meshoptimizer_rs::codec::raw+0xa22>
   add5b:	movzbl 0x4(%r14,%rsi,1),%ecx
   add61:	add    $0x5,%rsi
   add65:	shl    $0x1c,%ecx
   add68:	or     %ebx,%ecx
   add6a:	mov    %ecx,%ebx
   add6c:	mov    0x20(%rsp),%ebp
   add70:	mov    %ebx,%ecx
   add72:	shr    $1,%ecx
   add74:	and    $0x1,%ebx
   add77:	neg    %ebx
   add79:	xor    %ecx,%ebx
   add7b:	add    %ebp,%ebx
   add7d:	mov    %ebx,0xf0(%rsp,%r10,4)
   add85:	mov    $0x1,%r14d
   add8b:	mov    %ebx,0x20(%rsp)
   add8f:	jmp    ad96e <meshoptimizer_rs::codec::raw+0x5ce>
   add94:	add    $0x6,%rsi
   add98:	mov    0x30(%rsp),%rax
   add9d:	movzbl 0x4(%rax,%r14,1),%eax
   adda3:	shl    $0x1c,%eax
   adda6:	or     %r11d,%eax
   adda9:	jmp    adbcb <meshoptimizer_rs::codec::raw+0x82b>
   addae:	mov    0x30(%rsp),%rcx
   addb3:	movzbl 0x4(%rcx,%r14,1),%ecx
   addb9:	shl    $0x1c,%ecx
   addbc:	or     %r15d,%ecx
   addbf:	mov    %ecx,%r15d
   addc2:	mov    0x28(%rsp),%r9
   addc7:	mov    %r15d,%ecx
   addca:	shr    $1,%ecx
   addcc:	and    $0x1,%r15d
   addd0:	neg    %r15d
   addd3:	xor    %ecx,%r15d
   addd6:	add    0x20(%rsp),%r15d
   adddb:	mov    %rax,%r14
   addde:	mov    %r15d,0x20(%rsp)
   adde3:	cmp    $0xf,%r13b
   adde7:	jne    adeab <meshoptimizer_rs::codec::raw+0xb0b>
   added:	cmp    $0xfffffffffffffffa,%r14
   addf1:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   addf7:	lea    0x5(%r14),%rax
   addfb:	cmp    0x630(%rsp),%rax
   ade03:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ade09:	mov    0x30(%rsp),%r9
   ade0e:	movzbl (%r9,%r14,1),%esi
   ade13:	test   %sil,%sil
   ade16:	js     ade22 <meshoptimizer_rs::codec::raw+0xa82>
   ade18:	inc    %r14
   ade1b:	mov    %r14,%rax
   ade1e:	mov    %esi,%ebx
   ade20:	jmp    ade90 <meshoptimizer_rs::codec::raw+0xaf0>
   ade22:	and    $0x7f,%esi
   ade25:	movzbl 0x1(%r9,%r14,1),%ebx
   ade2b:	mov    %ebx,%ecx
   ade2d:	and    $0x7f,%ebx
   ade30:	shl    $0x7,%ebx
   ade33:	or     %esi,%ebx
   ade35:	test   %cl,%cl
   ade37:	js     ade42 <meshoptimizer_rs::codec::raw+0xaa2>
   ade39:	add    $0x2,%r14
   ade3d:	mov    %r14,%rax
   ade40:	jmp    ade90 <meshoptimizer_rs::codec::raw+0xaf0>
   ade42:	movzbl 0x2(%r9,%r14,1),%ecx
   ade48:	mov    %ecx,%esi
   ade4a:	and    $0x7f,%ecx
   ade4d:	shl    $0xe,%ecx
   ade50:	or     %ecx,%ebx
   ade52:	test   %sil,%sil
   ade55:	js     ade60 <meshoptimizer_rs::codec::raw+0xac0>
   ade57:	add    $0x3,%r14
   ade5b:	mov    %r14,%rax
   ade5e:	jmp    ade90 <meshoptimizer_rs::codec::raw+0xaf0>
   ade60:	movzbl 0x3(%r9,%r14,1),%ecx
   ade66:	mov    %ecx,%esi
   ade68:	and    $0x7f,%ecx
   ade6b:	shl    $0x15,%ecx
   ade6e:	or     %ecx,%ebx
   ade70:	test   %sil,%sil
   ade73:	js     ade7e <meshoptimizer_rs::codec::raw+0xade>
   ade75:	add    $0x4,%r14
   ade79:	mov    %r14,%rax
   ade7c:	jmp    ade90 <meshoptimizer_rs::codec::raw+0xaf0>
   ade7e:	mov    0x30(%rsp),%rcx
   ade83:	movzbl 0x4(%rcx,%r14,1),%ecx
   ade89:	shl    $0x1c,%ecx
   ade8c:	or     %ebx,%ecx
   ade8e:	mov    %ecx,%ebx
   ade90:	mov    0x28(%rsp),%r9
   ade95:	mov    %ebx,%ecx
   ade97:	shr    $1,%ecx
   ade99:	and    $0x1,%ebx
   ade9c:	neg    %ebx
   ade9e:	xor    %ecx,%ebx
   adea0:	add    0x20(%rsp),%ebx
   adea4:	mov    %rax,%r14
   adea7:	mov    %ebx,0x20(%rsp)
   adeab:	cmp    $0xf,%r13b
   adeaf:	sete   %al
   adeb2:	cmp    $0xf,%r12b
   adeb6:	sete   %cl
   adeb9:	test   %r13b,%r13b
   adebc:	sete   %sil
   adec0:	test   %r12b,%r12b
   adec3:	sete   %r12b
   adec7:	mov    %r11d,0xf0(%rsp,%r10,4)
   adecf:	lea    0x1(%r10),%r13
   aded3:	and    $0xf,%r13d
   aded7:	or     %cl,%r12b
   adeda:	mov    %r15d,0xf0(%rsp,%r13,4)
   adee2:	movzbl %r12b,%ecx
   adee6:	add    %r10,%rcx
   adee9:	inc    %rcx
   adeec:	or     %al,%sil
   adeef:	movzbl %sil,%r10d
   adef3:	add    %rcx,%r10
   adef6:	and    $0xf,%ecx
   adef9:	mov    %ebx,0xf0(%rsp,%rcx,4)
   adf00:	mov    %r15d,0x1f0(%rsp,%r8,8)
   adf08:	mov    %r11d,0x1f4(%rsp,%r8,8)
   adf10:	lea    0x1(%r8),%r13d
   adf14:	and    $0xf,%r13d
   adf18:	mov    $0x3,%eax
   adf1d:	mov    $0x2,%r12d
   adf23:	mov    %r14,%rsi
   adf26:	jmp    ad99d <meshoptimizer_rs::codec::raw+0x5fd>
   adf2b:	mov    %bpl,%r14b
   adf2e:	add    %r14d,%r11d
   adf31:	mov    %ebx,0xf0(%rsp,%r10,4)
   adf39:	mov    %r11d,%ebp
   adf3c:	not    %eax
   adf3e:	shr    $0x4,%eax
   adf41:	add    %r8d,%eax
   adf44:	and    $0xf,%eax
   adf47:	mov    0x1f0(%rsp,%rax,8),%r11d
   adf4f:	mov    0x1f4(%rsp,%rax,8),%r15d
   adf57:	add    %r14,%r10
   adf5a:	mov    $0x2,%eax
   adf5f:	mov    $0x1,%r12d
   adf65:	mov    %r8,%r13
   adf68:	inc    %rdi
   adf6b:	mov    %ebx,0x1f0(%rsp,%r13,8)
   adf73:	mov    %r15d,0x1f4(%rsp,%r13,8)
   adf7b:	add    %r8d,%r12d
   adf7e:	and    $0xf,%r12d
   adf82:	mov    %r11d,0x1f0(%rsp,%r12,8)
   adf8a:	mov    %ebx,0x1f4(%rsp,%r12,8)
   adf92:	add    %r8d,%eax
   adf95:	and    $0xf,%eax
   adf98:	and    $0xf,%r10d
   adf9c:	mov    %r11d,-0x8(%rdx)
   adfa0:	mov    %r15d,-0x4(%rdx)
   adfa4:	mov    %rdx,%r15
   adfa7:	mov    %ebx,(%rdx)
   adfa9:	add    $0xc,%r15
   adfad:	mov    %rax,%r8
   adfb0:	mov    %ebp,%r11d
   adfb3:	cmp    %rdi,0x38(%rsp)
   adfb8:	je     aeef4 <meshoptimizer_rs::codec::raw+0x1b54>
   adfbe:	cmp    %r9,%rsi
   adfc1:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   adfc7:	mov    %r15,%rdx
   adfca:	mov    0x30(%rsp),%r14
   adfcf:	movzbl 0x1(%r14,%rdi,1),%eax
   adfd5:	cmp    $0xf0,%al
   adfd7:	jae    ae011 <meshoptimizer_rs::codec::raw+0xc71>
   adfd9:	mov    %eax,%r15d
   adfdc:	and    $0xf,%r15d
   adfe0:	cmp    %r15d,0x58(%rsp)
   adfe5:	jbe    ae04b <meshoptimizer_rs::codec::raw+0xcab>
   adfe7:	xor    %r14d,%r14d
   adfea:	mov    %r11d,%ebx
   adfed:	test   %r15b,%r15b
   adff0:	sete   %bpl
   adff4:	je     adf2b <meshoptimizer_rs::codec::raw+0xb8b>
   adffa:	not    %r15d
   adffd:	add    %r10d,%r15d
   ae000:	and    $0xf,%r15d
   ae004:	mov    0xf0(%rsp,%r15,4),%ebx
   ae00c:	jmp    adf2b <meshoptimizer_rs::codec::raw+0xb8b>
   ae011:	cmp    $0xfe,%al
   ae013:	jae    ae06d <meshoptimizer_rs::codec::raw+0xccd>
   ae015:	and    $0xf,%eax
   ae018:	mov    0x50(%rsp),%rcx
   ae01d:	movzbl (%rcx,%rax,1),%r13d
   ae022:	lea    0x1(%r11),%r15d
   ae026:	xor    %eax,%eax
   ae028:	mov    %r13d,%ecx
   ae02b:	shr    $0x4,%ecx
   ae02e:	sete   %r12b
   ae032:	je     ae0b0 <meshoptimizer_rs::codec::raw+0xd10>
   ae034:	mov    %r10d,%ebx
   ae037:	sub    %ecx,%ebx
   ae039:	and    $0xf,%ebx
   ae03c:	mov    %r15d,%ebp
   ae03f:	mov    0xf0(%rsp,%rbx,4),%ecx
   ae046:	mov    %ecx,%r15d
   ae049:	jmp    ae0b4 <meshoptimizer_rs::codec::raw+0xd14>
   ae04b:	cmp    $0xf,%r15b
   ae04f:	jne    ae130 <meshoptimizer_rs::codec::raw+0xd90>
   ae055:	movzbl (%r14,%rsi,1),%ebp
   ae05a:	test   %bpl,%bpl
   ae05d:	js     ae19e <meshoptimizer_rs::codec::raw+0xdfe>
   ae063:	inc    %rsi
   ae066:	mov    %ebp,%ebx
   ae068:	jmp    ae334 <meshoptimizer_rs::codec::raw+0xf94>
   ae06d:	movzbl (%r14,%rsi,1),%r14d
   ae072:	xor    %r15d,%r15d
   ae075:	cmp    $0xfe,%eax
   ae07a:	sete   %r15b
   ae07e:	test   %r14d,%r14d
   ae081:	cmove  %r14d,%r11d
   ae085:	add    %r11d,%r15d
   ae088:	mov    %r14d,%r12d
   ae08b:	shr    $0x4,%r12d
   ae08f:	je     ae14b <meshoptimizer_rs::codec::raw+0xdab>
   ae095:	mov    %r10d,%ecx
   ae098:	sub    %r12d,%ecx
   ae09b:	and    $0xf,%ecx
   ae09e:	mov    %r15d,%ebx
   ae0a1:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae0a8:	mov    %ecx,%r15d
   ae0ab:	jmp    ae14f <meshoptimizer_rs::codec::raw+0xdaf>
   ae0b0:	lea    0x2(%r11),%ebp
   ae0b4:	xor    %r14d,%r14d
   ae0b7:	mov    %ebp,%ebx
   ae0b9:	test   $0xf,%r13b
   ae0bd:	sete   %cl
   ae0c0:	je     ae0d2 <meshoptimizer_rs::codec::raw+0xd32>
   ae0c2:	mov    %r10d,%ebx
   ae0c5:	sub    %r13d,%ebx
   ae0c8:	and    $0xf,%ebx
   ae0cb:	mov    0xf0(%rsp,%rbx,4),%ebx
   ae0d2:	mov    %cl,%r14b
   ae0d5:	add    %r14d,%ebp
   ae0d8:	mov    %r11d,0xf0(%rsp,%r10,4)
   ae0e0:	lea    0x1(%r10),%rcx
   ae0e4:	and    $0xf,%ecx
   ae0e7:	mov    %r15d,0xf0(%rsp,%rcx,4)
   ae0ef:	mov    %r12b,%al
   ae0f2:	add    %r10,%rax
   ae0f5:	inc    %rax
   ae0f8:	add    %rax,%r14
   ae0fb:	and    $0xf,%eax
   ae0fe:	mov    %ebx,0xf0(%rsp,%rax,4)
   ae105:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ae10d:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ae115:	lea    0x1(%r8),%r13d
   ae119:	and    $0xf,%r13d
   ae11d:	mov    $0x3,%eax
   ae122:	mov    $0x2,%r12d
   ae128:	mov    %r14,%r10
   ae12b:	jmp    adf68 <meshoptimizer_rs::codec::raw+0xbc8>
   ae130:	movzbl %r15b,%ecx
   ae134:	xor    $0x3,%r15b
   ae138:	movzbl %r15b,%ebx
   ae13c:	mov    0x20(%rsp),%ebp
   ae140:	add    %ecx,%ebp
   ae142:	sub    %ebx,%ebp
   ae144:	mov    %ebp,%ebx
   ae146:	jmp    ae345 <meshoptimizer_rs::codec::raw+0xfa5>
   ae14b:	lea    0x1(%r15),%ebx
   ae14f:	mov    %r14d,%r13d
   ae152:	and    $0xf,%r13b
   ae156:	je     ae16e <meshoptimizer_rs::codec::raw+0xdce>
   ae158:	mov    %r10d,%ecx
   ae15b:	sub    %r14d,%ecx
   ae15e:	and    $0xf,%ecx
   ae161:	mov    %ebx,%ebp
   ae163:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae16a:	mov    %ecx,%ebx
   ae16c:	jmp    ae171 <meshoptimizer_rs::codec::raw+0xdd1>
   ae16e:	lea    0x1(%rbx),%ebp
   ae171:	lea    0x1(%rsi),%r14
   ae175:	cmp    $0xfe,%eax
   ae17a:	je     ae278 <meshoptimizer_rs::codec::raw+0xed8>
   ae180:	mov    0x30(%rsp),%r9
   ae185:	movzbl 0x1(%r9,%rsi,1),%eax
   ae18b:	test   %al,%al
   ae18d:	js     ae1be <meshoptimizer_rs::codec::raw+0xe1e>
   ae18f:	add    $0x2,%rsi
   ae193:	mov    %rsi,%r14
   ae196:	mov    %eax,%r11d
   ae199:	jmp    ae25a <meshoptimizer_rs::codec::raw+0xeba>
   ae19e:	and    $0x7f,%ebp
   ae1a1:	movzbl 0x1(%r14,%rsi,1),%ebx
   ae1a7:	mov    %ebx,%ecx
   ae1a9:	and    $0x7f,%ebx
   ae1ac:	shl    $0x7,%ebx
   ae1af:	or     %ebp,%ebx
   ae1b1:	test   %cl,%cl
   ae1b3:	js     ae1df <meshoptimizer_rs::codec::raw+0xe3f>
   ae1b5:	add    $0x2,%rsi
   ae1b9:	jmp    ae334 <meshoptimizer_rs::codec::raw+0xf94>
   ae1be:	and    $0x7f,%eax
   ae1c1:	movzbl 0x2(%r9,%rsi,1),%r11d
   ae1c7:	mov    %r11d,%ecx
   ae1ca:	and    $0x7f,%r11d
   ae1ce:	shl    $0x7,%r11d
   ae1d2:	or     %eax,%r11d
   ae1d5:	test   %cl,%cl
   ae1d7:	js     ae1fd <meshoptimizer_rs::codec::raw+0xe5d>
   ae1d9:	add    $0x3,%rsi
   ae1dd:	jmp    ae257 <meshoptimizer_rs::codec::raw+0xeb7>
   ae1df:	movzbl 0x2(%r14,%rsi,1),%ecx
   ae1e5:	mov    %ecx,%ebp
   ae1e7:	and    $0x7f,%ecx
   ae1ea:	shl    $0xe,%ecx
   ae1ed:	or     %ecx,%ebx
   ae1ef:	test   %bpl,%bpl
   ae1f2:	js     ae218 <meshoptimizer_rs::codec::raw+0xe78>
   ae1f4:	add    $0x3,%rsi
   ae1f8:	jmp    ae334 <meshoptimizer_rs::codec::raw+0xf94>
   ae1fd:	movzbl 0x3(%r9,%rsi,1),%eax
   ae203:	mov    %eax,%ecx
   ae205:	and    $0x7f,%eax
   ae208:	shl    $0xe,%eax
   ae20b:	or     %eax,%r11d
   ae20e:	test   %cl,%cl
   ae210:	js     ae23a <meshoptimizer_rs::codec::raw+0xe9a>
   ae212:	add    $0x4,%rsi
   ae216:	jmp    ae257 <meshoptimizer_rs::codec::raw+0xeb7>
   ae218:	movzbl 0x3(%r14,%rsi,1),%ecx
   ae21e:	mov    %ecx,%ebp
   ae220:	and    $0x7f,%ecx
   ae223:	shl    $0x15,%ecx
   ae226:	or     %ecx,%ebx
   ae228:	test   %bpl,%bpl
   ae22b:	js     ae323 <meshoptimizer_rs::codec::raw+0xf83>
   ae231:	add    $0x4,%rsi
   ae235:	jmp    ae334 <meshoptimizer_rs::codec::raw+0xf94>
   ae23a:	movzbl 0x4(%r9,%rsi,1),%eax
   ae240:	mov    %eax,%ecx
   ae242:	and    $0x7f,%eax
   ae245:	shl    $0x15,%eax
   ae248:	or     %eax,%r11d
   ae24b:	test   %cl,%cl
   ae24d:	js     ae35c <meshoptimizer_rs::codec::raw+0xfbc>
   ae253:	add    $0x5,%rsi
   ae257:	mov    %rsi,%r14
   ae25a:	mov    0x28(%rsp),%r9
   ae25f:	mov    %r11d,%eax
   ae262:	shr    $1,%eax
   ae264:	and    $0x1,%r11d
   ae268:	neg    %r11d
   ae26b:	xor    %eax,%r11d
   ae26e:	add    0x20(%rsp),%r11d
   ae273:	mov    %r11d,0x20(%rsp)
   ae278:	cmp    $0xf,%r12b
   ae27c:	jne    ae3ab <meshoptimizer_rs::codec::raw+0x100b>
   ae282:	cmp    $0xfffffffffffffffa,%r14
   ae286:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ae28c:	lea    0x5(%r14),%rax
   ae290:	cmp    0x630(%rsp),%rax
   ae298:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ae29e:	mov    0x30(%rsp),%r9
   ae2a3:	movzbl (%r9,%r14,1),%esi
   ae2a8:	test   %sil,%sil
   ae2ab:	js     ae2bb <meshoptimizer_rs::codec::raw+0xf1b>
   ae2ad:	inc    %r14
   ae2b0:	mov    %r14,%rax
   ae2b3:	mov    %esi,%r15d
   ae2b6:	jmp    ae38a <meshoptimizer_rs::codec::raw+0xfea>
   ae2bb:	and    $0x7f,%esi
   ae2be:	movzbl 0x1(%r9,%r14,1),%r15d
   ae2c4:	mov    %r15d,%ecx
   ae2c7:	and    $0x7f,%r15d
   ae2cb:	shl    $0x7,%r15d
   ae2cf:	or     %esi,%r15d
   ae2d2:	test   %cl,%cl
   ae2d4:	js     ae2e2 <meshoptimizer_rs::codec::raw+0xf42>
   ae2d6:	add    $0x2,%r14
   ae2da:	mov    %r14,%rax
   ae2dd:	jmp    ae38a <meshoptimizer_rs::codec::raw+0xfea>
   ae2e2:	movzbl 0x2(%r9,%r14,1),%ecx
   ae2e8:	mov    %ecx,%esi
   ae2ea:	and    $0x7f,%ecx
   ae2ed:	shl    $0xe,%ecx
   ae2f0:	or     %ecx,%r15d
   ae2f3:	test   %sil,%sil
   ae2f6:	js     ae304 <meshoptimizer_rs::codec::raw+0xf64>
   ae2f8:	add    $0x3,%r14
   ae2fc:	mov    %r14,%rax
   ae2ff:	jmp    ae38a <meshoptimizer_rs::codec::raw+0xfea>
   ae304:	movzbl 0x3(%r9,%r14,1),%ecx
   ae30a:	mov    %ecx,%esi
   ae30c:	and    $0x7f,%ecx
   ae30f:	shl    $0x15,%ecx
   ae312:	or     %ecx,%r15d
   ae315:	test   %sil,%sil
   ae318:	js     ae376 <meshoptimizer_rs::codec::raw+0xfd6>
   ae31a:	add    $0x4,%r14
   ae31e:	mov    %r14,%rax
   ae321:	jmp    ae38a <meshoptimizer_rs::codec::raw+0xfea>
   ae323:	movzbl 0x4(%r14,%rsi,1),%ecx
   ae329:	add    $0x5,%rsi
   ae32d:	shl    $0x1c,%ecx
   ae330:	or     %ebx,%ecx
   ae332:	mov    %ecx,%ebx
   ae334:	mov    0x20(%rsp),%ebp
   ae338:	mov    %ebx,%ecx
   ae33a:	shr    $1,%ecx
   ae33c:	and    $0x1,%ebx
   ae33f:	neg    %ebx
   ae341:	xor    %ecx,%ebx
   ae343:	add    %ebp,%ebx
   ae345:	mov    %ebx,0xf0(%rsp,%r10,4)
   ae34d:	mov    $0x1,%r14d
   ae353:	mov    %ebx,0x20(%rsp)
   ae357:	jmp    adf39 <meshoptimizer_rs::codec::raw+0xb99>
   ae35c:	add    $0x6,%rsi
   ae360:	mov    0x30(%rsp),%rax
   ae365:	movzbl 0x4(%rax,%r14,1),%eax
   ae36b:	shl    $0x1c,%eax
   ae36e:	or     %r11d,%eax
   ae371:	jmp    ae193 <meshoptimizer_rs::codec::raw+0xdf3>
   ae376:	mov    0x30(%rsp),%rcx
   ae37b:	movzbl 0x4(%rcx,%r14,1),%ecx
   ae381:	shl    $0x1c,%ecx
   ae384:	or     %r15d,%ecx
   ae387:	mov    %ecx,%r15d
   ae38a:	mov    0x28(%rsp),%r9
   ae38f:	mov    %r15d,%ecx
   ae392:	shr    $1,%ecx
   ae394:	and    $0x1,%r15d
   ae398:	neg    %r15d
   ae39b:	xor    %ecx,%r15d
   ae39e:	add    0x20(%rsp),%r15d
   ae3a3:	mov    %rax,%r14
   ae3a6:	mov    %r15d,0x20(%rsp)
   ae3ab:	cmp    $0xf,%r13b
   ae3af:	jne    ae473 <meshoptimizer_rs::codec::raw+0x10d3>
   ae3b5:	cmp    $0xfffffffffffffffa,%r14
   ae3b9:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ae3bf:	lea    0x5(%r14),%rax
   ae3c3:	cmp    0x630(%rsp),%rax
   ae3cb:	ja     aeefb <meshoptimizer_rs::codec::raw+0x1b5b>
   ae3d1:	mov    0x30(%rsp),%r9
   ae3d6:	movzbl (%r9,%r14,1),%esi
   ae3db:	test   %sil,%sil
   ae3de:	js     ae3ea <meshoptimizer_rs::codec::raw+0x104a>
   ae3e0:	inc    %r14
   ae3e3:	mov    %r14,%rax
   ae3e6:	mov    %esi,%ebx
   ae3e8:	jmp    ae458 <meshoptimizer_rs::codec::raw+0x10b8>
   ae3ea:	and    $0x7f,%esi
   ae3ed:	movzbl 0x1(%r9,%r14,1),%ebx
   ae3f3:	mov    %ebx,%ecx
   ae3f5:	and    $0x7f,%ebx
   ae3f8:	shl    $0x7,%ebx
   ae3fb:	or     %esi,%ebx
   ae3fd:	test   %cl,%cl
   ae3ff:	js     ae40a <meshoptimizer_rs::codec::raw+0x106a>
   ae401:	add    $0x2,%r14
   ae405:	mov    %r14,%rax
   ae408:	jmp    ae458 <meshoptimizer_rs::codec::raw+0x10b8>
   ae40a:	movzbl 0x2(%r9,%r14,1),%ecx
   ae410:	mov    %ecx,%esi
   ae412:	and    $0x7f,%ecx
   ae415:	shl    $0xe,%ecx
   ae418:	or     %ecx,%ebx
   ae41a:	test   %sil,%sil
   ae41d:	js     ae428 <meshoptimizer_rs::codec::raw+0x1088>
   ae41f:	add    $0x3,%r14
   ae423:	mov    %r14,%rax
   ae426:	jmp    ae458 <meshoptimizer_rs::codec::raw+0x10b8>
   ae428:	movzbl 0x3(%r9,%r14,1),%ecx
   ae42e:	mov    %ecx,%esi
   ae430:	and    $0x7f,%ecx
   ae433:	shl    $0x15,%ecx
   ae436:	or     %ecx,%ebx
   ae438:	test   %sil,%sil
   ae43b:	js     ae446 <meshoptimizer_rs::codec::raw+0x10a6>
   ae43d:	add    $0x4,%r14
   ae441:	mov    %r14,%rax
   ae444:	jmp    ae458 <meshoptimizer_rs::codec::raw+0x10b8>
   ae446:	mov    0x30(%rsp),%rcx
   ae44b:	movzbl 0x4(%rcx,%r14,1),%ecx
   ae451:	shl    $0x1c,%ecx
   ae454:	or     %ebx,%ecx
   ae456:	mov    %ecx,%ebx
   ae458:	mov    0x28(%rsp),%r9
   ae45d:	mov    %ebx,%ecx
   ae45f:	shr    $1,%ecx
   ae461:	and    $0x1,%ebx
   ae464:	neg    %ebx
   ae466:	xor    %ecx,%ebx
   ae468:	add    0x20(%rsp),%ebx
   ae46c:	mov    %rax,%r14
   ae46f:	mov    %ebx,0x20(%rsp)
   ae473:	cmp    $0xf,%r13b
   ae477:	sete   %al
   ae47a:	cmp    $0xf,%r12b
   ae47e:	sete   %cl
   ae481:	test   %r13b,%r13b
   ae484:	sete   %sil
   ae488:	test   %r12b,%r12b
   ae48b:	sete   %r12b
   ae48f:	mov    %r11d,0xf0(%rsp,%r10,4)
   ae497:	lea    0x1(%r10),%r13
   ae49b:	and    $0xf,%r13d
   ae49f:	or     %cl,%r12b
   ae4a2:	mov    %r15d,0xf0(%rsp,%r13,4)
   ae4aa:	movzbl %r12b,%ecx
   ae4ae:	add    %r10,%rcx
   ae4b1:	inc    %rcx
   ae4b4:	or     %al,%sil
   ae4b7:	movzbl %sil,%r10d
   ae4bb:	add    %rcx,%r10
   ae4be:	and    $0xf,%ecx
   ae4c1:	mov    %ebx,0xf0(%rsp,%rcx,4)
   ae4c8:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ae4d0:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ae4d8:	lea    0x1(%r8),%r13d
   ae4dc:	and    $0xf,%r13d
   ae4e0:	mov    $0x3,%eax
   ae4e5:	mov    $0x2,%r12d
   ae4eb:	mov    %r14,%rsi
   ae4ee:	jmp    adf68 <meshoptimizer_rs::codec::raw+0xbc8>
   ae4f3:	cmp    $0x21,%rbp
   ae4f7:	mov    $0x20,%eax
   ae4fc:	cmovae %rbp,%rax
   ae500:	mov    %rbp,%rcx
   ae503:	mov    %rax,0xe0(%rsp)
   ae50b:	lea    0x1(%rax),%rdx
   ae50f:	mov    $0x7,%al
   ae511:	cmp    %rdx,%rbx
   ae514:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae51a:	mov    %rbx,%rsi
   ae51d:	sub    %rcx,%rsi
   ae520:	lea    (%rsi,%rbp,1),%rax
   ae524:	mov    %rbx,%rcx
   ae527:	mov    %rax,0x60(%rsp)
   ae52c:	sub    %rax,%rcx
   ae52f:	mov    %rcx,0xa0(%rsp)
   ae537:	jb     aef7c <meshoptimizer_rs::codec::raw+0x1bdc>
   ae53d:	pxor   %xmm0,%xmm0
   ae541:	movdqa %xmm0,0x1e0(%rsp)
   ae54a:	movdqa %xmm0,0x1d0(%rsp)
   ae553:	movdqa %xmm0,0x1c0(%rsp)
   ae55c:	movdqa %xmm0,0x1b0(%rsp)
   ae565:	movdqa %xmm0,0x1a0(%rsp)
   ae56e:	movdqa %xmm0,0x190(%rsp)
   ae577:	movdqa %xmm0,0x180(%rsp)
   ae580:	movdqa %xmm0,0x170(%rsp)
   ae589:	movdqa %xmm0,0x160(%rsp)
   ae592:	movdqa %xmm0,0x150(%rsp)
   ae59b:	movdqa %xmm0,0x140(%rsp)
   ae5a4:	movdqa %xmm0,0x130(%rsp)
   ae5ad:	movdqa %xmm0,0x120(%rsp)
   ae5b6:	movdqa %xmm0,0x110(%rsp)
   ae5bf:	movdqa %xmm0,0x100(%rsp)
   ae5c8:	movdqa %xmm0,0xf0(%rsp)
   ae5d1:	cmp    $0x101,%rbp
   ae5d8:	jae    aef3d <meshoptimizer_rs::codec::raw+0x1b9d>
   ae5de:	mov    %dil,0x38(%rsp)
   ae5e3:	lea    (%rsi,%rbp,1),%rax
   ae5e7:	cmp    %rsi,%rax
   ae5ea:	jb     aef54 <meshoptimizer_rs::codec::raw+0x1bb4>
   ae5f0:	add    %r14,%rsi
   ae5f3:	lea    0xf0(%rsp),%rdi
   ae5fb:	mov    %rbp,%rdx
   ae5fe:	call   *0x7ab14(%rip)        # 129118 <memcpy@GLIBC_2.14>
   ae604:	test   %rbp,%rbp
   ae607:	je     aefe5 <meshoptimizer_rs::codec::raw+0x1c45>
   ae60d:	mov    $0x2000,%ax
   ae611:	xor    %edx,%edx
   ae613:	div    %bp
   ae616:	and    $0x3ff0,%eax
   ae61b:	cmp    $0x100,%eax
   ae620:	mov    $0x100,%ecx
   ae625:	cmovb  %eax,%ecx
   ae628:	mov    %rcx,0xd8(%rsp)
   ae630:	lea    0x1f0(%rsp),%rdi
   ae638:	mov    $0x400,%edx
   ae63d:	xor    %esi,%esi
   ae63f:	call   *0x7aae3(%rip)        # 129128 <memset@GLIBC_2.2.5>
   ae645:	test   %r13,%r13
   ae648:	je     aeed0 <meshoptimizer_rs::codec::raw+0x1b30>
   ae64e:	mov    %r15,0xd0(%rsp)
   ae656:	add    %r14,0x60(%rsp)
   ae65b:	mov    %rbp,%rax
   ae65e:	shr    $0x2,%rax
   ae662:	mov    %rax,0x98(%rsp)
   ae66a:	lea    0x3(%rbp),%rax
   ae66e:	shr    $0x2,%rax
   ae672:	mov    %rax,0xe8(%rsp)
   ae67a:	mov    $0x1,%r15d
   ae680:	xor    %eax,%eax
   ae682:	mov    %r12,0x88(%rsp)
   ae68a:	mov    %r14,0x30(%rsp)
   ae68f:	mov    %r13,%rcx
   ae692:	mov    %rax,%rdi
   ae695:	sub    %rax,%rcx
   ae698:	mov    0xd8(%rsp),%rax
   ae6a0:	cmp    %rax,%rcx
   ae6a3:	cmovae %rax,%rcx
   ae6a7:	mov    %rcx,0x20(%rsp)
   ae6ac:	movzbl 0x38(%rsp),%edx
   ae6b1:	test   %dl,%dl
   ae6b3:	je     ae6f9 <meshoptimizer_rs::codec::raw+0x1359>
   ae6b5:	mov    %r15,%rcx
   ae6b8:	mov    $0x7,%al
   ae6ba:	add    0x98(%rsp),%rcx
   ae6c2:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae6c8:	cmp    %rbx,%rcx
   ae6cb:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae6d1:	add    %r14,%r15
   ae6d4:	mov    0x98(%rsp),%rax
   ae6dc:	mov    %rax,0x90(%rsp)
   ae6e4:	mov    %r15,0xa8(%rsp)
   ae6ec:	mov    %rcx,%r15
   ae6ef:	mov    0x88(%rsp),%r12
   ae6f7:	jmp    ae712 <meshoptimizer_rs::codec::raw+0x1372>
   ae6f9:	movq   $0x0,0x90(%rsp)
   ae705:	mov    $0x1,%eax
   ae70a:	mov    %rax,0xa8(%rsp)
   ae712:	mov    0x20(%rsp),%rax
   ae717:	lea    (%rax,%rdi,1),%rsi
   ae71b:	imul   %rbp,%rdi
   ae71f:	mov    %rsi,0xc8(%rsp)
   ae727:	imul   %rbp,%rsi
   ae72b:	cmp    %rdi,%rsi
   ae72e:	jb     aef6c <meshoptimizer_rs::codec::raw+0x1bcc>
   ae734:	cmp    %r12,%rsi
   ae737:	ja     aef6c <meshoptimizer_rs::codec::raw+0x1bcc>
   ae73d:	mov    0x20(%rsp),%rax
   ae742:	lea    0xf(%rax),%ecx
   ae745:	and    $0x3f0,%ecx
   ae74b:	mov    %rcx,0x28(%rsp)
   ae750:	mov    %rax,%rcx
   ae753:	imul   %rbp,%rcx
   ae757:	mov    %rcx,0xb8(%rsp)
   ae75f:	add    0xd0(%rsp),%rdi
   ae767:	mov    %rdi,0xb0(%rsp)
   ae76f:	lea    -0x1(%rax),%rcx
   ae773:	imul   %rbp,%rcx
   ae777:	mov    %rcx,0xc0(%rsp)
   ae77f:	add    $0x4,%rcx
   ae783:	mov    %rcx,0x50(%rsp)
   ae788:	lea    0x0(,%rax,4),%rcx
   ae790:	mov    %rcx,0x68(%rsp)
   ae795:	lea    0x1f0(%rsp,%rax,1),%rcx
   ae79d:	mov    %rcx,0x80(%rsp)
   ae7a5:	lea    0x1f0(%rsp,%rax,2),%rcx
   ae7ad:	mov    %rcx,0x78(%rsp)
   ae7b2:	lea    (%rax,%rax,2),%rax
   ae7b6:	add    %rsp,%rax
   ae7b9:	add    $0x1f0,%rax
   ae7bf:	mov    %rax,0x70(%rsp)
   ae7c4:	xor    %r14d,%r14d
   ae7c7:	xor    %ebx,%ebx
   ae7c9:	jmp    ae82a <meshoptimizer_rs::codec::raw+0x148a>
   ae7cb:	lea    (%rsp,%rbx,1),%rcx
   ae7cf:	add    $0xf0,%rcx
   ae7d6:	shr    $0x4,%al
   ae7d9:	neg    %al
   ae7db:	and    $0x1f,%al
   ae7dd:	movzbl %al,%eax
   ae7e0:	mov    %rcx,(%rsp)
   ae7e4:	mov    %eax,0x8(%rsp)
   ae7e8:	movq   $0x0,0x10(%rsp)
   ae7f1:	lea    0x1f0(%rsp),%rdi
   ae7f9:	mov    0x68(%rsp),%rsi
   ae7fe:	mov    0x50(%rsp),%rcx
   ae803:	mov    0x20(%rsp),%r8
   ae808:	mov    %rbp,%r9
   ae80b:	call   72ef0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   ae810:	inc    %r14
   ae813:	add    $0x4,%rbx
   ae817:	cmp    %r14,0xe8(%rsp)
   ae81f:	movzbl 0x38(%rsp),%edx
   ae824:	je     aee78 <meshoptimizer_rs::codec::raw+0x1ad8>
   ae82a:	mov    %rbx,0x58(%rsp)
   ae82f:	test   %dl,%dl
   ae831:	je     ae850 <meshoptimizer_rs::codec::raw+0x14b0>
   ae833:	cmp    0x90(%rsp),%r14
   ae83b:	jae    aefb5 <meshoptimizer_rs::codec::raw+0x1c15>
   ae841:	mov    0xa8(%rsp),%rax
   ae849:	movzbl (%rax,%r14,1),%ebx
   ae84e:	jmp    ae852 <meshoptimizer_rs::codec::raw+0x14b2>
   ae850:	xor    %ebx,%ebx
   ae852:	mov    %ebx,%eax
   ae854:	and    $0x3,%al
   ae856:	movzbl %al,%ecx
   ae859:	mov    0x28(%rsp),%rsi
   ae85e:	cmp    %rsi,0x20(%rsp)
   ae863:	ja     aecc3 <meshoptimizer_rs::codec::raw+0x1923>
   ae869:	cmp    $0x2,%al
   ae86b:	je     ae8ad <meshoptimizer_rs::codec::raw+0x150d>
   ae86d:	cmp    $0x3,%ecx
   ae870:	jne    ae8c7 <meshoptimizer_rs::codec::raw+0x1527>
   ae872:	mov    %r15,%r12
   ae875:	mov    $0x7,%al
   ae877:	add    0x20(%rsp),%r12
   ae87c:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae882:	cmp    0x630(%rsp),%r12
   ae88a:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae890:	add    0x30(%rsp),%r15
   ae895:	lea    0x1f0(%rsp),%rdi
   ae89d:	mov    %r15,%rsi
   ae8a0:	mov    0x20(%rsp),%rdx
   ae8a5:	call   *0x7a86d(%rip)        # 129118 <memcpy@GLIBC_2.14>
   ae8ab:	jmp    ae923 <meshoptimizer_rs::codec::raw+0x1583>
   ae8ad:	lea    0x1f0(%rsp),%rdi
   ae8b5:	xor    %esi,%esi
   ae8b7:	mov    0x20(%rsp),%rdx
   ae8bc:	call   *0x7a866(%rip)        # 129128 <memset@GLIBC_2.2.5>
   ae8c2:	mov    %r15,%r12
   ae8c5:	jmp    ae923 <meshoptimizer_rs::codec::raw+0x1583>
   ae8c7:	test   %al,%al
   ae8c9:	lea    -0xa0ca0(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ae8d0:	lea    -0xa0b87(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ae8d7:	cmove  %rcx,%rax
   ae8db:	test   %dl,%dl
   ae8dd:	lea    -0xa0fb4(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ae8e4:	cmove  %rcx,%rax
   ae8e8:	mov    %rax,(%rsp)
   ae8ec:	lea    0x40(%rsp),%rdi
   ae8f1:	mov    0x30(%rsp),%rsi
   ae8f6:	mov    0x630(%rsp),%rdx
   ae8fe:	mov    %r15,%rcx
   ae901:	lea    0x1f0(%rsp),%r8
   ae909:	mov    0x28(%rsp),%r9
   ae90e:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   ae913:	cmpb   $0x0,0x40(%rsp)
   ae918:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   ae91e:	mov    0x48(%rsp),%r12
   ae923:	mov    %ebx,%eax
   ae925:	shr    $0x2,%al
   ae928:	and    $0x3,%al
   ae92a:	cmp    $0x3,%al
   ae92c:	movzbl 0x38(%rsp),%edx
   ae931:	je     ae955 <meshoptimizer_rs::codec::raw+0x15b5>
   ae933:	movzbl %al,%ecx
   ae936:	cmp    $0x2,%ecx
   ae939:	jne    ae993 <meshoptimizer_rs::codec::raw+0x15f3>
   ae93b:	mov    0x80(%rsp),%rdi
   ae943:	xor    %esi,%esi
   ae945:	mov    0x20(%rsp),%rdx
   ae94a:	call   *0x7a7d8(%rip)        # 129128 <memset@GLIBC_2.2.5>
   ae950:	jmp    ae9ef <meshoptimizer_rs::codec::raw+0x164f>
   ae955:	mov    %r12,%r15
   ae958:	mov    $0x7,%al
   ae95a:	add    0x20(%rsp),%r15
   ae95f:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae965:	cmp    0x630(%rsp),%r15
   ae96d:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   ae973:	add    0x30(%rsp),%r12
   ae978:	mov    0x80(%rsp),%rdi
   ae980:	mov    %r12,%rsi
   ae983:	mov    0x20(%rsp),%rdx
   ae988:	call   *0x7a78a(%rip)        # 129118 <memcpy@GLIBC_2.14>
   ae98e:	mov    %r15,%r12
   ae991:	jmp    ae9ef <meshoptimizer_rs::codec::raw+0x164f>
   ae993:	test   %al,%al
   ae995:	lea    -0xa0d6c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ae99c:	lea    -0xa0c53(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ae9a3:	cmove  %rcx,%rax
   ae9a7:	test   %dl,%dl
   ae9a9:	lea    -0xa1080(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ae9b0:	cmove  %rcx,%rax
   ae9b4:	mov    %rax,(%rsp)
   ae9b8:	lea    0x40(%rsp),%rdi
   ae9bd:	mov    0x30(%rsp),%rsi
   ae9c2:	mov    0x630(%rsp),%rdx
   ae9ca:	mov    %r12,%rcx
   ae9cd:	mov    0x80(%rsp),%r8
   ae9d5:	mov    0x28(%rsp),%r9
   ae9da:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   ae9df:	cmpb   $0x0,0x40(%rsp)
   ae9e4:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   ae9ea:	mov    0x48(%rsp),%r12
   ae9ef:	mov    %ebx,%eax
   ae9f1:	shr    $0x4,%al
   ae9f4:	and    $0x3,%al
   ae9f6:	cmp    $0x3,%al
   ae9f8:	je     aea46 <meshoptimizer_rs::codec::raw+0x16a6>
   ae9fa:	movzbl %al,%ecx
   ae9fd:	cmp    $0x2,%ecx
   aea00:	jne    aeac2 <meshoptimizer_rs::codec::raw+0x1722>
   aea06:	mov    0x78(%rsp),%rdi
   aea0b:	xor    %esi,%esi
   aea0d:	mov    0x20(%rsp),%rdx
   aea12:	call   *0x7a710(%rip)        # 129128 <memset@GLIBC_2.2.5>
   aea18:	shr    $0x6,%bl
   aea1b:	cmp    $0x3,%bl
   aea1e:	je     aea87 <meshoptimizer_rs::codec::raw+0x16e7>
   aea20:	movzbl %bl,%eax
   aea23:	cmp    $0x2,%eax
   aea26:	jne    aeb2f <meshoptimizer_rs::codec::raw+0x178f>
   aea2c:	mov    0x70(%rsp),%rdi
   aea31:	xor    %esi,%esi
   aea33:	mov    0x20(%rsp),%rdx
   aea38:	call   *0x7a6ea(%rip)        # 129128 <memset@GLIBC_2.2.5>
   aea3e:	mov    %r12,%r15
   aea41:	jmp    aeb8b <meshoptimizer_rs::codec::raw+0x17eb>
   aea46:	mov    %r12,%r15
   aea49:	mov    $0x7,%al
   aea4b:	add    0x20(%rsp),%r15
   aea50:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aea56:	cmp    0x630(%rsp),%r15
   aea5e:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aea64:	add    0x30(%rsp),%r12
   aea69:	mov    0x78(%rsp),%rdi
   aea6e:	mov    %r12,%rsi
   aea71:	mov    0x20(%rsp),%rdx
   aea76:	call   *0x7a69c(%rip)        # 129118 <memcpy@GLIBC_2.14>
   aea7c:	mov    %r15,%r12
   aea7f:	shr    $0x6,%bl
   aea82:	cmp    $0x3,%bl
   aea85:	jne    aea20 <meshoptimizer_rs::codec::raw+0x1680>
   aea87:	mov    %r12,%r15
   aea8a:	mov    $0x7,%al
   aea8c:	add    0x20(%rsp),%r15
   aea91:	jb     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aea97:	cmp    0x630(%rsp),%r15
   aea9f:	ja     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aeaa5:	add    0x30(%rsp),%r12
   aeaaa:	mov    0x70(%rsp),%rdi
   aeaaf:	mov    %r12,%rsi
   aeab2:	mov    0x20(%rsp),%rdx
   aeab7:	call   *0x7a65b(%rip)        # 129118 <memcpy@GLIBC_2.14>
   aeabd:	jmp    aeb8b <meshoptimizer_rs::codec::raw+0x17eb>
   aeac2:	test   %al,%al
   aeac4:	lea    -0xa0e9b(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aeacb:	lea    -0xa0d82(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aead2:	cmove  %rcx,%rax
   aead6:	cmpb   $0x0,0x38(%rsp)
   aeadb:	lea    -0xa11b2(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aeae2:	cmove  %rcx,%rax
   aeae6:	mov    %rax,(%rsp)
   aeaea:	lea    0x40(%rsp),%rdi
   aeaef:	mov    0x30(%rsp),%rsi
   aeaf4:	mov    0x630(%rsp),%rdx
   aeafc:	mov    %r12,%rcx
   aeaff:	mov    0x78(%rsp),%r8
   aeb04:	mov    0x28(%rsp),%r9
   aeb09:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   aeb0e:	cmpb   $0x0,0x40(%rsp)
   aeb13:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   aeb19:	mov    0x48(%rsp),%r12
   aeb1e:	shr    $0x6,%bl
   aeb21:	cmp    $0x3,%bl
   aeb24:	jne    aea20 <meshoptimizer_rs::codec::raw+0x1680>
   aeb2a:	jmp    aea87 <meshoptimizer_rs::codec::raw+0x16e7>
   aeb2f:	test   %bl,%bl
   aeb31:	lea    -0xa0f08(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aeb38:	lea    -0xa0def(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aeb3f:	cmove  %rcx,%rax
   aeb43:	cmpb   $0x0,0x38(%rsp)
   aeb48:	lea    -0xa121f(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aeb4f:	cmove  %rcx,%rax
   aeb53:	mov    %rax,(%rsp)
   aeb57:	lea    0x40(%rsp),%rdi
   aeb5c:	mov    0x30(%rsp),%rsi
   aeb61:	mov    0x630(%rsp),%rdx
   aeb69:	mov    %r12,%rcx
   aeb6c:	mov    0x70(%rsp),%r8
   aeb71:	mov    0x28(%rsp),%r9
   aeb76:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   aeb7b:	cmpb   $0x0,0x40(%rsp)
   aeb80:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   aeb86:	mov    0x48(%rsp),%r15
   aeb8b:	cmpb   $0x0,0x38(%rsp)
   aeb90:	mov    0x88(%rsp),%r12
   aeb98:	mov    0xb8(%rsp),%rsi
   aeba0:	mov    0x58(%rsp),%rbx
   aeba5:	je     aebcc <meshoptimizer_rs::codec::raw+0x182c>
   aeba7:	cmp    0xa0(%rsp),%r14
   aebaf:	jae    aefcd <meshoptimizer_rs::codec::raw+0x1c2d>
   aebb5:	mov    0x60(%rsp),%rax
   aebba:	movzbl (%rax,%r14,1),%eax
   aebbf:	mov    %rsi,%rdx
   aebc2:	sub    %rbx,%rdx
   aebc5:	jae    aebda <meshoptimizer_rs::codec::raw+0x183a>
   aebc7:	jmp    aef2a <meshoptimizer_rs::codec::raw+0x1b8a>
   aebcc:	xor    %eax,%eax
   aebce:	mov    %rsi,%rdx
   aebd1:	sub    %rbx,%rdx
   aebd4:	jb     aef2a <meshoptimizer_rs::codec::raw+0x1b8a>
   aebda:	cmp    %rdx,0x50(%rsp)
   aebdf:	ja     aef16 <meshoptimizer_rs::codec::raw+0x1b76>
   aebe5:	mov    0xb0(%rsp),%rcx
   aebed:	lea    (%rcx,%rbx,1),%rdx
   aebf1:	mov    %eax,%ecx
   aebf3:	and    $0x3,%cl
   aebf6:	movzbl %cl,%ecx
   aebf9:	lea    -0x9e348(%rip),%rsi        # 108b8 <libm::math::generic::sqrt::RSQRT_TAB+0x4d2>
   aec00:	movslq (%rsi,%rcx,4),%rcx
   aec04:	add    %rsi,%rcx
   aec07:	jmp    *%rcx
   aec09:	mov    %rdx,%r12
   aec0c:	add    %rsp,%rbx
   aec0f:	add    $0xf0,%rbx
   aec16:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   aec1b:	test   %al,%al
   aec1d:	je     aec82 <meshoptimizer_rs::codec::raw+0x18e2>
   aec1f:	mov    %rbx,(%rsp)
   aec23:	lea    0x1f0(%rsp),%rdi
   aec2b:	mov    0x68(%rsp),%rsi
   aec30:	mov    %r12,%rdx
   aec33:	mov    0x50(%rsp),%rcx
   aec38:	mov    0x20(%rsp),%r8
   aec3d:	mov    %rbp,%r9
   aec40:	call   74fb0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   aec45:	jmp    aecb1 <meshoptimizer_rs::codec::raw+0x1911>
   aec47:	lea    (%rsp,%rbx,1),%rax
   aec4b:	add    $0xf0,%rax
   aec51:	mov    %rax,(%rsp)
   aec55:	movq   $0x0,0x8(%rsp)
   aec5e:	lea    0x1f0(%rsp),%rdi
   aec66:	mov    0x68(%rsp),%rsi
   aec6b:	mov    0x50(%rsp),%rcx
   aec70:	mov    0x20(%rsp),%r8
   aec75:	mov    %rbp,%r9
   aec78:	call   72d30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   aec7d:	jmp    ae810 <meshoptimizer_rs::codec::raw+0x1470>
   aec82:	mov    %rbx,(%rsp)
   aec86:	movq   $0x0,0x8(%rsp)
   aec8f:	lea    0x1f0(%rsp),%rdi
   aec97:	mov    0x68(%rsp),%rsi
   aec9c:	mov    %r12,%rdx
   aec9f:	mov    0x50(%rsp),%rcx
   aeca4:	mov    0x20(%rsp),%r8
   aeca9:	mov    %rbp,%r9
   aecac:	call   72b40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   aecb1:	mov    0x88(%rsp),%r12
   aecb9:	mov    0x58(%rsp),%rbx
   aecbe:	jmp    ae810 <meshoptimizer_rs::codec::raw+0x1470>
   aecc3:	cmp    $0x2,%al
   aecc5:	je     af00b <meshoptimizer_rs::codec::raw+0x1c6b>
   aeccb:	cmp    $0x3,%ecx
   aecce:	je     aeff2 <meshoptimizer_rs::codec::raw+0x1c52>
   aecd4:	test   %al,%al
   aecd6:	lea    -0xa10ad(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aecdd:	lea    -0xa0f94(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aece4:	cmove  %rcx,%rax
   aece8:	test   %dl,%dl
   aecea:	lea    -0xa13c1(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aecf1:	cmove  %rcx,%rax
   aecf5:	mov    %rax,(%rsp)
   aecf9:	lea    0x40(%rsp),%rdi
   aecfe:	mov    0x30(%rsp),%rsi
   aed03:	mov    0x630(%rsp),%rdx
   aed0b:	mov    %r15,%rcx
   aed0e:	lea    0x1f0(%rsp),%r8
   aed16:	mov    0x28(%rsp),%r9
   aed1b:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   aed20:	cmpb   $0x0,0x40(%rsp)
   aed25:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   aed2b:	mov    %ebx,%eax
   aed2d:	shr    $0x2,%al
   aed30:	and    $0x3,%al
   aed32:	cmp    $0x2,%al
   aed34:	movzbl 0x38(%rsp),%edx
   aed39:	je     af00b <meshoptimizer_rs::codec::raw+0x1c6b>
   aed3f:	movzbl %al,%ecx
   aed42:	cmp    $0x3,%ecx
   aed45:	je     aeff2 <meshoptimizer_rs::codec::raw+0x1c52>
   aed4b:	test   %al,%al
   aed4d:	lea    -0xa1124(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aed54:	lea    -0xa100b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aed5b:	cmove  %rcx,%rax
   aed5f:	mov    0x48(%rsp),%rcx
   aed64:	test   %dl,%dl
   aed66:	lea    -0xa143d(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aed6d:	cmove  %rdx,%rax
   aed71:	mov    %rax,(%rsp)
   aed75:	lea    0x40(%rsp),%rdi
   aed7a:	mov    0x30(%rsp),%rsi
   aed7f:	mov    0x630(%rsp),%rdx
   aed87:	mov    0x80(%rsp),%r8
   aed8f:	mov    0x28(%rsp),%r9
   aed94:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   aed99:	cmpb   $0x0,0x40(%rsp)
   aed9e:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   aeda4:	mov    %ebx,%eax
   aeda6:	shr    $0x4,%al
   aeda9:	and    $0x3,%al
   aedab:	cmp    $0x2,%al
   aedad:	movzbl 0x38(%rsp),%edx
   aedb2:	je     af00b <meshoptimizer_rs::codec::raw+0x1c6b>
   aedb8:	movzbl %al,%ecx
   aedbb:	cmp    $0x3,%ecx
   aedbe:	je     aeff2 <meshoptimizer_rs::codec::raw+0x1c52>
   aedc4:	test   %al,%al
   aedc6:	lea    -0xa119d(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aedcd:	lea    -0xa1084(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aedd4:	cmove  %rcx,%rax
   aedd8:	mov    0x48(%rsp),%rcx
   aeddd:	test   %dl,%dl
   aeddf:	lea    -0xa14b6(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aede6:	cmove  %rdx,%rax
   aedea:	mov    %rax,(%rsp)
   aedee:	lea    0x40(%rsp),%rdi
   aedf3:	mov    0x30(%rsp),%rsi
   aedf8:	mov    0x630(%rsp),%rdx
   aee00:	mov    0x78(%rsp),%r8
   aee05:	mov    0x28(%rsp),%r9
   aee0a:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   aee0f:	cmpb   $0x0,0x40(%rsp)
   aee14:	jne    aef0f <meshoptimizer_rs::codec::raw+0x1b6f>
   aee1a:	shr    $0x6,%bl
   aee1d:	cmp    $0x2,%bl
   aee20:	movzbl 0x38(%rsp),%edx
   aee25:	je     af00b <meshoptimizer_rs::codec::raw+0x1c6b>
   aee2b:	movzbl %bl,%eax
   aee2e:	cmp    $0x3,%eax
   aee31:	je     aeff2 <meshoptimizer_rs::codec::raw+0x1c52>
   aee37:	test   %bl,%bl
   aee39:	lea    -0xa1210(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   aee40:	lea    -0xa10f7(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   aee47:	cmove  %rcx,%rax
   aee4b:	mov    0x48(%rsp),%rcx
   aee50:	test   %dl,%dl
   aee52:	lea    -0xa1529(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   aee59:	cmove  %rdx,%rax
   aee5d:	mov    %rax,(%rsp)
   aee61:	lea    0x40(%rsp),%rdi
   aee66:	mov    0x30(%rsp),%rsi
   aee6b:	mov    0x630(%rsp),%rdx
   aee73:	jmp    aeb6c <meshoptimizer_rs::codec::raw+0x17cc>
   aee78:	mov    0xb8(%rsp),%rdx
   aee80:	mov    0xc0(%rsp),%rdi
   aee88:	cmp    %rdx,%rdi
   aee8b:	mov    0x30(%rsp),%r14
   aee90:	ja     aef94 <meshoptimizer_rs::codec::raw+0x1bf4>
   aee96:	mov    0xb0(%rsp),%rsi
   aee9e:	add    %rdi,%rsi
   aeea1:	lea    0xf0(%rsp),%rdi
   aeea9:	mov    %rbp,%rdx
   aeeac:	call   *0x7a266(%rip)        # 129118 <memcpy@GLIBC_2.14>
   aeeb2:	mov    0xc8(%rsp),%rcx
   aeeba:	mov    %rcx,%rax
   aeebd:	cmp    %r13,%rcx
   aeec0:	mov    0x630(%rsp),%rbx
   aeec8:	jb     ae68f <meshoptimizer_rs::codec::raw+0x12ef>
   aeece:	jmp    aeed6 <meshoptimizer_rs::codec::raw+0x1b36>
   aeed0:	mov    $0x1,%r15d
   aeed6:	sub    %r15,%rbx
   aeed9:	setae  %cl
   aeedc:	cmp    0xe0(%rsp),%rbx
   aeee4:	sete   %al
   aeee7:	and    %cl,%al
   aeee9:	neg    %al
   aeeeb:	or     $0x7,%al
   aeeed:	jmp    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aeeef:	mov    0x28(%rsp),%r9
   aeef4:	mov    $0xff,%al
   aeef6:	cmp    %r9,%rsi
   aeef9:	je     aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aeefb:	mov    $0x7,%al
   aeefd:	add    $0x5f8,%rsp
   aef04:	pop    %rbx
   aef05:	pop    %r12
   aef07:	pop    %r13
   aef09:	pop    %r14
   aef0b:	pop    %r15
   aef0d:	pop    %rbp
   aef0e:	ret
   aef0f:	movzbl 0x41(%rsp),%eax
   aef14:	jmp    aeefd <meshoptimizer_rs::codec::raw+0x1b5d>
   aef16:	lea    0x74e5b(%rip),%rcx        # 123d78 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   aef1d:	xor    %edi,%edi
   aef1f:	mov    0x50(%rsp),%rsi
   aef24:	call   *0x7a346(%rip)        # 129270 <_DYNAMIC+0x370>
   aef2a:	lea    0x74e5f(%rip),%rcx        # 123d90 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   aef31:	mov    %rsi,%rdx
   aef34:	mov    %rbx,%rdi
   aef37:	call   *0x7a333(%rip)        # 129270 <_DYNAMIC+0x370>
   aef3d:	lea    0x74eac(%rip),%rcx        # 123df0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   aef44:	mov    $0x100,%edx
   aef49:	xor    %edi,%edi
   aef4b:	mov    %rbp,%rsi
   aef4e:	call   *0x7a31c(%rip)        # 129270 <_DYNAMIC+0x370>
   aef54:	lea    0x74e7d(%rip),%rcx        # 123dd8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   aef5b:	mov    %rsi,%rdi
   aef5e:	mov    0x60(%rsp),%rsi
   aef63:	mov    %rbx,%rdx
   aef66:	call   *0x7a304(%rip)        # 129270 <_DYNAMIC+0x370>
   aef6c:	lea    0x7868d(%rip),%rcx        # 127600 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a58>
   aef73:	mov    %r12,%rdx
   aef76:	call   *0x7a2f4(%rip)        # 129270 <_DYNAMIC+0x370>
   aef7c:	lea    0x74e85(%rip),%rcx        # 123e08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   aef83:	mov    0x60(%rsp),%rdi
   aef88:	mov    %rbx,%rsi
   aef8b:	mov    %rbx,%rdx
   aef8e:	call   *0x7a2dc(%rip)        # 129270 <_DYNAMIC+0x370>
   aef94:	lea    0x74d95(%rip),%rcx        # 123d30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   aef9b:	mov    %rdx,%rsi
   aef9e:	call   *0x7a2cc(%rip)        # 129270 <_DYNAMIC+0x370>
   aefa4:	lea    0x77e45(%rip),%rdx        # 126df0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5248>
   aefab:	xor    %edi,%edi
   aefad:	xor    %esi,%esi
   aefaf:	call   *0x7a19b(%rip)        # 129150 <_DYNAMIC+0x250>
   aefb5:	lea    0x74d8c(%rip),%rdx        # 123d48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   aefbc:	mov    %r14,%rdi
   aefbf:	mov    0x90(%rsp),%rsi
   aefc7:	call   *0x7a183(%rip)        # 129150 <_DYNAMIC+0x250>
   aefcd:	lea    0x74d8c(%rip),%rdx        # 123d60 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   aefd4:	mov    %r14,%rdi
   aefd7:	mov    0xa0(%rsp),%rsi
   aefdf:	call   *0x7a16b(%rip)        # 129150 <_DYNAMIC+0x250>
   aefe5:	lea    0x74d2c(%rip),%rdi        # 123d18 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   aefec:	call   *0x7a346(%rip)        # 129338 <_DYNAMIC+0x438>
   aeff2:	lea    0x74daf(%rip),%rcx        # 123da8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   aeff9:	xor    %edi,%edi
   aeffb:	mov    0x20(%rsp),%rsi
   af000:	mov    0x28(%rsp),%rdx
   af005:	call   *0x7a265(%rip)        # 129270 <_DYNAMIC+0x370>
   af00b:	lea    0x74dae(%rip),%rcx        # 123dc0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   af012:	xor    %edi,%edi
   af014:	mov    0x20(%rsp),%rsi
   af019:	mov    0x28(%rsp),%rdx
   af01e:	call   *0x7a24c(%rip)        # 129270 <_DYNAMIC+0x370>
