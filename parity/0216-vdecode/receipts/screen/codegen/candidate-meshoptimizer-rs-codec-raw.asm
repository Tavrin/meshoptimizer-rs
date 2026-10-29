
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000b0370 <meshoptimizer_rs::codec::raw>:
   b0370:	push   %rbp
   b0371:	push   %r15
   b0373:	push   %r14
   b0375:	push   %r13
   b0377:	push   %r12
   b0379:	push   %rbx
   b037a:	sub    $0xf8,%rsp
   b0381:	mov    %r9,%r10
   b0384:	mov    0x130(%rsp),%r9
   b038c:	test   %dil,%dil
   b038f:	je     b04da <meshoptimizer_rs::codec::raw+0x16a>
   b0395:	movzbl %dil,%eax
   b0399:	cmp    $0x1,%eax
   b039c:	jne    b04ff <meshoptimizer_rs::codec::raw+0x18f>
   b03a2:	mov    %rdx,%r11
   b03a5:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   b03af:	mov    %rcx,%rax
   b03b2:	mul    %rdi
   b03b5:	shr    $1,%rdx
   b03b8:	lea    (%rdx,%rdx,2),%rbx
   b03bc:	mov    %rcx,%rax
   b03bf:	sub    %rbx,%rax
   b03c2:	cmp    $0x2,%r8
   b03c6:	jne    b0569 <meshoptimizer_rs::codec::raw+0x1f9>
   b03cc:	test   %rax,%rax
   b03cf:	jne    b0572 <meshoptimizer_rs::codec::raw+0x202>
   b03d5:	mov    %rcx,%rax
   b03d8:	mul    %rdi
   b03db:	shr    $1,%rdx
   b03de:	lea    0x11(%rdx),%rcx
   b03e2:	mov    $0x7,%al
   b03e4:	cmp    %rcx,%r9
   b03e7:	jb     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b03ed:	movzbl (%r10),%r8d
   b03f1:	mov    %r8d,%ecx
   b03f4:	and    $0xf0,%cl
   b03f7:	cmp    $0xe0,%cl
   b03fa:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0400:	and    $0xf,%r8b
   b0404:	mov    $0x5,%al
   b0406:	cmp    $0x1,%r8b
   b040a:	ja     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0410:	mov    %r11,%rcx
   b0413:	lea    0x1(%rdx),%rbx
   b0417:	pcmpeqd %xmm0,%xmm0
   b041b:	movdqa %xmm0,0xe0(%rsp)
   b0424:	movdqa %xmm0,0xd0(%rsp)
   b042d:	movdqa %xmm0,0xc0(%rsp)
   b0436:	movdqa %xmm0,0xb0(%rsp)
   b043f:	movdqa %xmm0,0xa0(%rsp)
   b0448:	movdqa %xmm0,0x90(%rsp)
   b0451:	movdqa %xmm0,0x80(%rsp)
   b045a:	movdqa %xmm0,0x70(%rsp)
   b0460:	movdqa %xmm0,0x40(%rsp)
   b0466:	movdqa %xmm0,0x30(%rsp)
   b046c:	movdqa %xmm0,0x20(%rsp)
   b0472:	movdqa %xmm0,0x10(%rsp)
   b0478:	xor    %r11d,%r11d
   b047b:	cmp    $0x1,%r8b
   b047f:	setne  %r8b
   b0483:	lea    -0x10(%r9),%rax
   b0487:	mov    %rax,0x50(%rsp)
   b048c:	mov    %rcx,%rax
   b048f:	mov    %rdx,%rcx
   b0492:	mul    %rdi
   b0495:	shr    $0x2,%rdx
   b0499:	cmp    %rcx,%rdx
   b049c:	cmovae %rcx,%rdx
   b04a0:	mov    %rdx,0x68(%rsp)
   b04a5:	test   %rdx,%rdx
   b04a8:	je     b1343 <meshoptimizer_rs::codec::raw+0xfd3>
   b04ae:	mov    %r8b,%r11b
   b04b1:	lea    0xd(,%r11,2),%rax
   b04b9:	mov    %rax,0x60(%rsp)
   b04be:	add    $0x4,%rsi
   b04c2:	xor    %ecx,%ecx
   b04c4:	xor    %r14d,%r14d
   b04c7:	movl   $0x0,0x8(%rsp)
   b04cf:	xor    %r15d,%r15d
   b04d2:	xor    %r8d,%r8d
   b04d5:	jmp    b08d0 <meshoptimizer_rs::codec::raw+0x560>
   b04da:	mov    %rsi,%rdi
   b04dd:	mov    %rdx,%rsi
   b04e0:	mov    %rcx,%rdx
   b04e3:	mov    %r8,%rcx
   b04e6:	mov    %r10,%r8
   b04e9:	add    $0xf8,%rsp
   b04f0:	pop    %rbx
   b04f1:	pop    %r12
   b04f3:	pop    %r13
   b04f5:	pop    %r14
   b04f7:	pop    %r15
   b04f9:	pop    %rbp
   b04fa:	jmp    74bb0 <meshoptimizer_rs::codec::vertex::decode::<[u8]>>
   b04ff:	add    $0x5,%rcx
   b0503:	mov    $0x7,%al
   b0505:	cmp    $0x2,%r8
   b0509:	jne    b0579 <meshoptimizer_rs::codec::raw+0x209>
   b050b:	cmp    %rcx,%r9
   b050e:	jb     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0514:	test   %r9,%r9
   b0517:	je     b1360 <meshoptimizer_rs::codec::raw+0xff0>
   b051d:	movzbl (%r10),%ecx
   b0521:	mov    %ecx,%edi
   b0523:	and    $0xf0,%dil
   b0527:	cmp    $0xd0,%dil
   b052b:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0531:	mov    $0x5,%al
   b0533:	test   $0xe,%cl
   b0536:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b053c:	dec    %r9
   b053f:	movq   $0x0,0x70(%rsp)
   b0548:	movabs $0x7ffffffffffffffc,%rax
   b0552:	or     $0x2,%rax
   b0556:	and    %rax,%rdx
   b0559:	je     b0848 <meshoptimizer_rs::codec::raw+0x4d8>
   b055f:	inc    %r10
   b0562:	xor    %eax,%eax
   b0564:	jmp    b060d <meshoptimizer_rs::codec::raw+0x29d>
   b0569:	test   %rax,%rax
   b056c:	je     b074c <meshoptimizer_rs::codec::raw+0x3dc>
   b0572:	xor    %eax,%eax
   b0574:	jmp    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0579:	cmp    %rcx,%r9
   b057c:	jb     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0582:	test   %r9,%r9
   b0585:	je     b1360 <meshoptimizer_rs::codec::raw+0xff0>
   b058b:	movzbl (%r10),%ecx
   b058f:	mov    %ecx,%edi
   b0591:	and    $0xf0,%dil
   b0595:	cmp    $0xd0,%dil
   b0599:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b059f:	mov    $0x5,%al
   b05a1:	test   $0xe,%cl
   b05a4:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b05aa:	dec    %r9
   b05ad:	movq   $0x0,0x70(%rsp)
   b05b6:	movabs $0x7ffffffffffffffc,%rax
   b05c0:	and    %rax,%rdx
   b05c3:	je     b0848 <meshoptimizer_rs::codec::raw+0x4d8>
   b05c9:	inc    %r10
   b05cc:	xor    %eax,%eax
   b05ce:	jmp    b06c9 <meshoptimizer_rs::codec::raw+0x359>
   b05d3:	dec    %r9
   b05d6:	inc    %r10
   b05d9:	mov    %edi,%ecx
   b05db:	mov    %ecx,%edi
   b05dd:	shr    $0x2,%edi
   b05e0:	mov    %ecx,%r8d
   b05e3:	shl    $0x1e,%r8d
   b05e7:	sar    $0x1f,%r8d
   b05eb:	xor    %edi,%r8d
   b05ee:	and    $0x1,%ecx
   b05f1:	add    0x70(%rsp,%rcx,4),%r8d
   b05f6:	mov    %r8d,0x70(%rsp,%rcx,4)
   b05fb:	mov    %r8w,(%rsi,%rax,1)
   b0600:	add    $0x2,%rax
   b0604:	cmp    %rax,%rdx
   b0607:	je     b0848 <meshoptimizer_rs::codec::raw+0x4d8>
   b060d:	cmp    $0x5,%r9
   b0611:	jb     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0617:	movzbl (%r10),%edi
   b061b:	test   %dil,%dil
   b061e:	jns    b05d3 <meshoptimizer_rs::codec::raw+0x263>
   b0620:	and    $0x7f,%edi
   b0623:	movzbl 0x1(%r10),%ecx
   b0628:	mov    %ecx,%r8d
   b062b:	and    $0x7f,%ecx
   b062e:	shl    $0x7,%ecx
   b0631:	or     %edi,%ecx
   b0633:	mov    $0x2,%edi
   b0638:	test   %r8b,%r8b
   b063b:	jns    b0685 <meshoptimizer_rs::codec::raw+0x315>
   b063d:	movzbl 0x2(%r10),%edi
   b0642:	mov    %edi,%r8d
   b0645:	and    $0x7f,%edi
   b0648:	shl    $0xe,%edi
   b064b:	or     %edi,%ecx
   b064d:	mov    $0x3,%edi
   b0652:	test   %r8b,%r8b
   b0655:	jns    b0685 <meshoptimizer_rs::codec::raw+0x315>
   b0657:	movzbl 0x3(%r10),%edi
   b065c:	mov    %edi,%r8d
   b065f:	and    $0x7f,%edi
   b0662:	shl    $0x15,%edi
   b0665:	or     %edi,%ecx
   b0667:	mov    $0x4,%edi
   b066c:	test   %r8b,%r8b
   b066f:	jns    b0685 <meshoptimizer_rs::codec::raw+0x315>
   b0671:	movzbl 0x4(%r10),%r8d
   b0676:	shl    $0x1c,%r8d
   b067a:	or     %ecx,%r8d
   b067d:	mov    $0x5,%edi
   b0682:	mov    %r8d,%ecx
   b0685:	sub    %rdi,%r9
   b0688:	add    %rdi,%r10
   b068b:	jmp    b05db <meshoptimizer_rs::codec::raw+0x26b>
   b0690:	dec    %r9
   b0693:	inc    %r10
   b0696:	mov    %edi,%ecx
   b0698:	mov    %ecx,%edi
   b069a:	shr    $0x2,%edi
   b069d:	mov    %ecx,%r8d
   b06a0:	shl    $0x1e,%r8d
   b06a4:	sar    $0x1f,%r8d
   b06a8:	xor    %edi,%r8d
   b06ab:	and    $0x1,%ecx
   b06ae:	add    0x70(%rsp,%rcx,4),%r8d
   b06b3:	mov    %r8d,0x70(%rsp,%rcx,4)
   b06b8:	mov    %r8d,(%rsi,%rax,1)
   b06bc:	add    $0x4,%rax
   b06c0:	cmp    %rax,%rdx
   b06c3:	je     b0848 <meshoptimizer_rs::codec::raw+0x4d8>
   b06c9:	cmp    $0x5,%r9
   b06cd:	jb     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b06d3:	movzbl (%r10),%edi
   b06d7:	test   %dil,%dil
   b06da:	jns    b0690 <meshoptimizer_rs::codec::raw+0x320>
   b06dc:	and    $0x7f,%edi
   b06df:	movzbl 0x1(%r10),%ecx
   b06e4:	mov    %ecx,%r8d
   b06e7:	and    $0x7f,%ecx
   b06ea:	shl    $0x7,%ecx
   b06ed:	or     %edi,%ecx
   b06ef:	mov    $0x2,%edi
   b06f4:	test   %r8b,%r8b
   b06f7:	jns    b0741 <meshoptimizer_rs::codec::raw+0x3d1>
   b06f9:	movzbl 0x2(%r10),%edi
   b06fe:	mov    %edi,%r8d
   b0701:	and    $0x7f,%edi
   b0704:	shl    $0xe,%edi
   b0707:	or     %edi,%ecx
   b0709:	mov    $0x3,%edi
   b070e:	test   %r8b,%r8b
   b0711:	jns    b0741 <meshoptimizer_rs::codec::raw+0x3d1>
   b0713:	movzbl 0x3(%r10),%edi
   b0718:	mov    %edi,%r8d
   b071b:	and    $0x7f,%edi
   b071e:	shl    $0x15,%edi
   b0721:	or     %edi,%ecx
   b0723:	mov    $0x4,%edi
   b0728:	test   %r8b,%r8b
   b072b:	jns    b0741 <meshoptimizer_rs::codec::raw+0x3d1>
   b072d:	movzbl 0x4(%r10),%r8d
   b0732:	shl    $0x1c,%r8d
   b0736:	or     %ecx,%r8d
   b0739:	mov    $0x5,%edi
   b073e:	mov    %r8d,%ecx
   b0741:	sub    %rdi,%r9
   b0744:	add    %rdi,%r10
   b0747:	jmp    b0698 <meshoptimizer_rs::codec::raw+0x328>
   b074c:	lea    0x11(%rdx),%rcx
   b0750:	mov    $0x7,%al
   b0752:	cmp    %rcx,%r9
   b0755:	jb     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b075b:	movzbl (%r10),%r8d
   b075f:	mov    %r8d,%ecx
   b0762:	and    $0xf0,%cl
   b0765:	cmp    $0xe0,%cl
   b0768:	jne    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b076e:	and    $0xf,%r8b
   b0772:	mov    $0x5,%al
   b0774:	cmp    $0x1,%r8b
   b0778:	ja     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b077e:	mov    %r11,%rcx
   b0781:	lea    0x1(%rdx),%rbx
   b0785:	pcmpeqd %xmm0,%xmm0
   b0789:	movdqa %xmm0,0xe0(%rsp)
   b0792:	movdqa %xmm0,0xd0(%rsp)
   b079b:	movdqa %xmm0,0xc0(%rsp)
   b07a4:	movdqa %xmm0,0xb0(%rsp)
   b07ad:	movdqa %xmm0,0xa0(%rsp)
   b07b6:	movdqa %xmm0,0x90(%rsp)
   b07bf:	movdqa %xmm0,0x80(%rsp)
   b07c8:	movdqa %xmm0,0x70(%rsp)
   b07ce:	movdqa %xmm0,0x40(%rsp)
   b07d4:	movdqa %xmm0,0x30(%rsp)
   b07da:	movdqa %xmm0,0x20(%rsp)
   b07e0:	movdqa %xmm0,0x10(%rsp)
   b07e6:	xor    %r11d,%r11d
   b07e9:	cmp    $0x1,%r8b
   b07ed:	setne  %r8b
   b07f1:	lea    -0x10(%r9),%rax
   b07f5:	mov    %rax,0x50(%rsp)
   b07fa:	mov    %rcx,%rax
   b07fd:	mov    %rdx,%rcx
   b0800:	mul    %rdi
   b0803:	shr    $0x3,%rdx
   b0807:	cmp    %rcx,%rdx
   b080a:	cmovae %rcx,%rdx
   b080e:	mov    %rdx,0x68(%rsp)
   b0813:	test   %rdx,%rdx
   b0816:	je     b1343 <meshoptimizer_rs::codec::raw+0xfd3>
   b081c:	mov    %r8b,%r11b
   b081f:	lea    0xd(,%r11,2),%rax
   b0827:	mov    %rax,0x60(%rsp)
   b082c:	add    $0x8,%rsi
   b0830:	xor    %ecx,%ecx
   b0832:	xor    %r14d,%r14d
   b0835:	movl   $0x0,0x8(%rsp)
   b083d:	xor    %r15d,%r15d
   b0840:	xor    %r8d,%r8d
   b0843:	jmp    b0e44 <meshoptimizer_rs::codec::raw+0xad4>
   b0848:	cmp    $0x4,%r9
   b084c:	sete   %al
   b084f:	neg    %al
   b0851:	or     $0x7,%al
   b0853:	jmp    b134e <meshoptimizer_rs::codec::raw+0xfde>
   b0858:	mov    %r11b,%al
   b085b:	add    %eax,%r15d
   b085e:	mov    %r12d,0x10(%rsp,%r14,4)
   b0863:	mov    %r15d,%r13d
   b0866:	not    %edx
   b0868:	shr    $0x4,%edx
   b086b:	add    %ecx,%edx
   b086d:	and    $0xf,%edx
   b0870:	mov    0x70(%rsp,%rdx,8),%r15d
   b0875:	mov    0x74(%rsp,%rdx,8),%ebp
   b0879:	add    %rax,%r14
   b087c:	mov    $0x2,%eax
   b0881:	mov    $0x1,%edi
   b0886:	mov    %rcx,%r11
   b0889:	inc    %r8
   b088c:	mov    %r12d,0x70(%rsp,%r11,8)
   b0891:	mov    %ebp,0x74(%rsp,%r11,8)
   b0896:	add    %ecx,%edi
   b0898:	and    $0xf,%edi
   b089b:	mov    %r15d,0x70(%rsp,%rdi,8)
   b08a0:	mov    %r12d,0x74(%rsp,%rdi,8)
   b08a5:	add    %ecx,%eax
   b08a7:	and    $0xf,%eax
   b08aa:	and    $0xf,%r14d
   b08ae:	mov    %r15w,-0x4(%rsi)
   b08b3:	mov    %bp,-0x2(%rsi)
   b08b7:	mov    %r12w,(%rsi)
   b08bb:	add    $0x6,%rsi
   b08bf:	mov    %rax,%rcx
   b08c2:	mov    %r13d,%r15d
   b08c5:	cmp    %r8,0x68(%rsp)
   b08ca:	je     b1343 <meshoptimizer_rs::codec::raw+0xfd3>
   b08d0:	lea    -0x10(%r9),%rax
   b08d4:	cmp    %rax,%rbx
   b08d7:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b08dd:	movzbl 0x1(%r10,%r8,1),%edx
   b08e3:	cmp    $0xf0,%dl
   b08e6:	jae    b0917 <meshoptimizer_rs::codec::raw+0x5a7>
   b08e8:	mov    %edx,%edi
   b08ea:	and    $0xf,%edi
   b08ed:	cmp    %edi,0x60(%rsp)
   b08f1:	jbe    b0959 <meshoptimizer_rs::codec::raw+0x5e9>
   b08f3:	xor    %eax,%eax
   b08f5:	mov    %r15d,%r12d
   b08f8:	test   %dil,%dil
   b08fb:	sete   %r11b
   b08ff:	je     b0858 <meshoptimizer_rs::codec::raw+0x4e8>
   b0905:	not    %edi
   b0907:	add    %r14d,%edi
   b090a:	and    $0xf,%edi
   b090d:	mov    0x10(%rsp,%rdi,4),%r12d
   b0912:	jmp    b0858 <meshoptimizer_rs::codec::raw+0x4e8>
   b0917:	cmp    $0xfe,%dl
   b091a:	jae    b097b <meshoptimizer_rs::codec::raw+0x60b>
   b091c:	and    $0xf,%edx
   b091f:	lea    (%r10,%r9,1),%rax
   b0923:	add    $0xfffffffffffffff0,%rax
   b0927:	movzbl (%rax,%rdx,1),%r11d
   b092c:	lea    0x1(%r15),%ebp
   b0930:	xor    %eax,%eax
   b0932:	mov    %r11d,%edx
   b0935:	shr    $0x4,%edx
   b0938:	sete   %dil
   b093c:	mov    %rbx,0x58(%rsp)
   b0941:	je     b09b8 <meshoptimizer_rs::codec::raw+0x648>
   b0943:	mov    %r14d,%r12d
   b0946:	sub    %edx,%r12d
   b0949:	and    $0xf,%r12d
   b094d:	mov    %ebp,%r13d
   b0950:	mov    0x10(%rsp,%r12,4),%edx
   b0955:	mov    %edx,%ebp
   b0957:	jmp    b09bc <meshoptimizer_rs::codec::raw+0x64c>
   b0959:	cmp    $0xf,%dil
   b095d:	jne    b0a34 <meshoptimizer_rs::codec::raw+0x6c4>
   b0963:	movzbl (%r10,%rbx,1),%eax
   b0968:	test   %al,%al
   b096a:	js     b0aac <meshoptimizer_rs::codec::raw+0x73c>
   b0970:	inc    %rbx
   b0973:	mov    %eax,%r12d
   b0976:	jmp    b0b4a <meshoptimizer_rs::codec::raw+0x7da>
   b097b:	movzbl (%r10,%rbx,1),%r11d
   b0980:	xor    %ebp,%ebp
   b0982:	cmp    $0xfe,%edx
   b0988:	sete   %bpl
   b098c:	test   %r11d,%r11d
   b098f:	cmove  %r11d,%r15d
   b0993:	add    %r15d,%ebp
   b0996:	mov    %r11d,%eax
   b0999:	shr    $0x4,%eax
   b099c:	je     b0a53 <meshoptimizer_rs::codec::raw+0x6e3>
   b09a2:	mov    %r14d,%edi
   b09a5:	sub    %eax,%edi
   b09a7:	and    $0xf,%edi
   b09aa:	mov    %ebp,%r12d
   b09ad:	mov    0x10(%rsp,%rdi,4),%edi
   b09b1:	mov    %edi,%ebp
   b09b3:	jmp    b0a57 <meshoptimizer_rs::codec::raw+0x6e7>
   b09b8:	lea    0x2(%r15),%r13d
   b09bc:	xor    %edx,%edx
   b09be:	mov    %r13d,%ebx
   b09c1:	mov    %r13d,%r12d
   b09c4:	test   $0xf,%r11b
   b09c8:	sete   %r13b
   b09cc:	je     b09dd <meshoptimizer_rs::codec::raw+0x66d>
   b09ce:	mov    %r14d,%r12d
   b09d1:	sub    %r11d,%r12d
   b09d4:	and    $0xf,%r12d
   b09d8:	mov    0x10(%rsp,%r12,4),%r12d
   b09dd:	mov    %r13b,%dl
   b09e0:	mov    %ebx,%r13d
   b09e3:	add    %edx,%r13d
   b09e6:	mov    %r15d,0x10(%rsp,%r14,4)
   b09eb:	lea    0x1(%r14),%r11
   b09ef:	and    $0xf,%r11d
   b09f3:	mov    %ebp,0x10(%rsp,%r11,4)
   b09f8:	mov    %dil,%al
   b09fb:	add    %r14,%rax
   b09fe:	inc    %rax
   b0a01:	add    %rax,%rdx
   b0a04:	and    $0xf,%eax
   b0a07:	mov    %r12d,0x10(%rsp,%rax,4)
   b0a0c:	mov    %ebp,0x70(%rsp,%rcx,8)
   b0a10:	mov    %r15d,0x74(%rsp,%rcx,8)
   b0a15:	lea    0x1(%rcx),%r11d
   b0a19:	and    $0xf,%r11d
   b0a1d:	mov    $0x3,%eax
   b0a22:	mov    $0x2,%edi
   b0a27:	mov    %rdx,%r14
   b0a2a:	mov    0x58(%rsp),%rbx
   b0a2f:	jmp    b0889 <meshoptimizer_rs::codec::raw+0x519>
   b0a34:	movzbl %dil,%eax
   b0a38:	xor    $0x3,%dil
   b0a3c:	movzbl %dil,%edi
   b0a40:	mov    0x8(%rsp),%r11d
   b0a45:	add    %eax,%r11d
   b0a48:	sub    %edi,%r11d
   b0a4b:	mov    %r11d,%r12d
   b0a4e:	jmp    b0b60 <meshoptimizer_rs::codec::raw+0x7f0>
   b0a53:	lea    0x1(%rbp),%r12d
   b0a57:	mov    %r11d,%edi
   b0a5a:	and    $0xf,%dil
   b0a5e:	je     b0a79 <meshoptimizer_rs::codec::raw+0x709>
   b0a60:	mov    %r14d,%r13d
   b0a63:	sub    %r11d,%r13d
   b0a66:	and    $0xf,%r13d
   b0a6a:	mov    %r12d,0xc(%rsp)
   b0a6f:	mov    0x10(%rsp,%r13,4),%r11d
   b0a74:	mov    %r11d,%r12d
   b0a77:	jmp    b0a83 <meshoptimizer_rs::codec::raw+0x713>
   b0a79:	lea    0x1(%r12),%r11d
   b0a7e:	mov    %r11d,0xc(%rsp)
   b0a83:	lea    0x1(%rbx),%r13
   b0a87:	cmp    $0xfe,%edx
   b0a8d:	je     b0baf <meshoptimizer_rs::codec::raw+0x83f>
   b0a93:	movzbl 0x1(%r10,%rbx,1),%edx
   b0a99:	test   %dl,%dl
   b0a9b:	js     b0ace <meshoptimizer_rs::codec::raw+0x75e>
   b0a9d:	add    $0x2,%rbx
   b0aa1:	mov    %rbx,%r13
   b0aa4:	mov    %edx,%r15d
   b0aa7:	jmp    b0b96 <meshoptimizer_rs::codec::raw+0x826>
   b0aac:	and    $0x7f,%eax
   b0aaf:	movzbl 0x1(%r10,%rbx,1),%r12d
   b0ab5:	mov    %r12d,%edi
   b0ab8:	and    $0x7f,%r12d
   b0abc:	shl    $0x7,%r12d
   b0ac0:	or     %eax,%r12d
   b0ac3:	test   %dil,%dil
   b0ac6:	js     b0af3 <meshoptimizer_rs::codec::raw+0x783>
   b0ac8:	add    $0x2,%rbx
   b0acc:	jmp    b0b4a <meshoptimizer_rs::codec::raw+0x7da>
   b0ace:	and    $0x7f,%edx
   b0ad1:	movzbl 0x2(%r10,%rbx,1),%r15d
   b0ad7:	mov    %r15d,%r11d
   b0ada:	and    $0x7f,%r15d
   b0ade:	shl    $0x7,%r15d
   b0ae2:	or     %edx,%r15d
   b0ae5:	test   %r11b,%r11b
   b0ae8:	js     b0b0f <meshoptimizer_rs::codec::raw+0x79f>
   b0aea:	add    $0x3,%rbx
   b0aee:	jmp    b0b93 <meshoptimizer_rs::codec::raw+0x823>
   b0af3:	movzbl 0x2(%r10,%rbx,1),%eax
   b0af9:	mov    %eax,%edi
   b0afb:	and    $0x7f,%eax
   b0afe:	shl    $0xe,%eax
   b0b01:	or     %eax,%r12d
   b0b04:	test   %dil,%dil
   b0b07:	js     b0b2c <meshoptimizer_rs::codec::raw+0x7bc>
   b0b09:	add    $0x3,%rbx
   b0b0d:	jmp    b0b4a <meshoptimizer_rs::codec::raw+0x7da>
   b0b0f:	movzbl 0x3(%r10,%rbx,1),%edx
   b0b15:	mov    %edx,%r11d
   b0b18:	and    $0x7f,%edx
   b0b1b:	shl    $0xe,%edx
   b0b1e:	or     %edx,%r15d
   b0b21:	test   %r11b,%r11b
   b0b24:	js     b0b74 <meshoptimizer_rs::codec::raw+0x804>
   b0b26:	add    $0x4,%rbx
   b0b2a:	jmp    b0b93 <meshoptimizer_rs::codec::raw+0x823>
   b0b2c:	movzbl 0x3(%r10,%rbx,1),%eax
   b0b32:	mov    %eax,%edi
   b0b34:	and    $0x7f,%eax
   b0b37:	shl    $0x15,%eax
   b0b3a:	or     %eax,%r12d
   b0b3d:	test   %dil,%dil
   b0b40:	js     b0d81 <meshoptimizer_rs::codec::raw+0xa11>
   b0b46:	add    $0x4,%rbx
   b0b4a:	mov    0x8(%rsp),%edi
   b0b4e:	mov    %r12d,%eax
   b0b51:	shr    $1,%eax
   b0b53:	and    $0x1,%r12d
   b0b57:	neg    %r12d
   b0b5a:	xor    %eax,%r12d
   b0b5d:	add    %edi,%r12d
   b0b60:	mov    %r12d,0x10(%rsp,%r14,4)
   b0b65:	mov    $0x1,%eax
   b0b6a:	mov    %r12d,0x8(%rsp)
   b0b6f:	jmp    b0863 <meshoptimizer_rs::codec::raw+0x4f3>
   b0b74:	movzbl 0x4(%r10,%rbx,1),%edx
   b0b7a:	mov    %edx,%r11d
   b0b7d:	and    $0x7f,%edx
   b0b80:	shl    $0x15,%edx
   b0b83:	or     %edx,%r15d
   b0b86:	test   %r11b,%r11b
   b0b89:	js     b0d96 <meshoptimizer_rs::codec::raw+0xa26>
   b0b8f:	add    $0x5,%rbx
   b0b93:	mov    %rbx,%r13
   b0b96:	mov    %r15d,%edx
   b0b99:	shr    $1,%edx
   b0b9b:	and    $0x1,%r15d
   b0b9f:	neg    %r15d
   b0ba2:	xor    %edx,%r15d
   b0ba5:	add    0x8(%rsp),%r15d
   b0baa:	mov    %r15d,0x8(%rsp)
   b0baf:	cmp    $0xf,%al
   b0bb1:	jne    b0c5c <meshoptimizer_rs::codec::raw+0x8ec>
   b0bb7:	cmp    $0xfffffffffffffffa,%r13
   b0bbb:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0bc1:	lea    0x5(%r13),%rdx
   b0bc5:	cmp    %r9,%rdx
   b0bc8:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0bce:	movzbl (%r10,%r13,1),%r11d
   b0bd3:	test   %r11b,%r11b
   b0bd6:	js     b0be3 <meshoptimizer_rs::codec::raw+0x873>
   b0bd8:	inc    %r13
   b0bdb:	mov    %r13,%rdx
   b0bde:	mov    %r11d,%ebp
   b0be1:	jmp    b0c43 <meshoptimizer_rs::codec::raw+0x8d3>
   b0be3:	and    $0x7f,%r11d
   b0be7:	movzbl 0x1(%r10,%r13,1),%ebp
   b0bed:	mov    %ebp,%ebx
   b0bef:	and    $0x7f,%ebp
   b0bf2:	shl    $0x7,%ebp
   b0bf5:	or     %r11d,%ebp
   b0bf8:	test   %bl,%bl
   b0bfa:	js     b0c02 <meshoptimizer_rs::codec::raw+0x892>
   b0bfc:	add    $0x2,%r13
   b0c00:	jmp    b0c40 <meshoptimizer_rs::codec::raw+0x8d0>
   b0c02:	movzbl 0x2(%r10,%r13,1),%r11d
   b0c08:	mov    %r11d,%ebx
   b0c0b:	and    $0x7f,%r11d
   b0c0f:	shl    $0xe,%r11d
   b0c13:	or     %r11d,%ebp
   b0c16:	test   %bl,%bl
   b0c18:	js     b0c20 <meshoptimizer_rs::codec::raw+0x8b0>
   b0c1a:	add    $0x3,%r13
   b0c1e:	jmp    b0c40 <meshoptimizer_rs::codec::raw+0x8d0>
   b0c20:	movzbl 0x3(%r10,%r13,1),%r11d
   b0c26:	mov    %r11d,%ebx
   b0c29:	and    $0x7f,%r11d
   b0c2d:	shl    $0x15,%r11d
   b0c31:	or     %r11d,%ebp
   b0c34:	test   %bl,%bl
   b0c36:	js     b0dab <meshoptimizer_rs::codec::raw+0xa3b>
   b0c3c:	add    $0x4,%r13
   b0c40:	mov    %r13,%rdx
   b0c43:	mov    %ebp,%r11d
   b0c46:	shr    $1,%r11d
   b0c49:	and    $0x1,%ebp
   b0c4c:	neg    %ebp
   b0c4e:	xor    %r11d,%ebp
   b0c51:	add    0x8(%rsp),%ebp
   b0c55:	mov    %rdx,%r13
   b0c58:	mov    %ebp,0x8(%rsp)
   b0c5c:	cmp    $0xf,%dil
   b0c60:	jne    b0d12 <meshoptimizer_rs::codec::raw+0x9a2>
   b0c66:	cmp    $0xfffffffffffffffa,%r13
   b0c6a:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0c70:	lea    0x5(%r13),%rdx
   b0c74:	cmp    %r9,%rdx
   b0c77:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0c7d:	movzbl (%r10,%r13,1),%r11d
   b0c82:	test   %r11b,%r11b
   b0c85:	js     b0c92 <meshoptimizer_rs::codec::raw+0x922>
   b0c87:	inc    %r13
   b0c8a:	mov    %r13,%rdx
   b0c8d:	mov    %r11d,%r12d
   b0c90:	jmp    b0cf5 <meshoptimizer_rs::codec::raw+0x985>
   b0c92:	and    $0x7f,%r11d
   b0c96:	movzbl 0x1(%r10,%r13,1),%r12d
   b0c9c:	mov    %r12d,%ebx
   b0c9f:	and    $0x7f,%r12d
   b0ca3:	shl    $0x7,%r12d
   b0ca7:	or     %r11d,%r12d
   b0caa:	test   %bl,%bl
   b0cac:	js     b0cb4 <meshoptimizer_rs::codec::raw+0x944>
   b0cae:	add    $0x2,%r13
   b0cb2:	jmp    b0cf2 <meshoptimizer_rs::codec::raw+0x982>
   b0cb4:	movzbl 0x2(%r10,%r13,1),%r11d
   b0cba:	mov    %r11d,%ebx
   b0cbd:	and    $0x7f,%r11d
   b0cc1:	shl    $0xe,%r11d
   b0cc5:	or     %r11d,%r12d
   b0cc8:	test   %bl,%bl
   b0cca:	js     b0cd2 <meshoptimizer_rs::codec::raw+0x962>
   b0ccc:	add    $0x3,%r13
   b0cd0:	jmp    b0cf2 <meshoptimizer_rs::codec::raw+0x982>
   b0cd2:	movzbl 0x3(%r10,%r13,1),%r11d
   b0cd8:	mov    %r11d,%ebx
   b0cdb:	and    $0x7f,%r11d
   b0cdf:	shl    $0x15,%r11d
   b0ce3:	or     %r11d,%r12d
   b0ce6:	test   %bl,%bl
   b0ce8:	js     b0dbd <meshoptimizer_rs::codec::raw+0xa4d>
   b0cee:	add    $0x4,%r13
   b0cf2:	mov    %r13,%rdx
   b0cf5:	mov    %r12d,%r11d
   b0cf8:	shr    $1,%r11d
   b0cfb:	and    $0x1,%r12d
   b0cff:	neg    %r12d
   b0d02:	xor    %r11d,%r12d
   b0d05:	add    0x8(%rsp),%r12d
   b0d0a:	mov    %rdx,%r13
   b0d0d:	mov    %r12d,0x8(%rsp)
   b0d12:	cmp    $0xf,%dil
   b0d16:	sete   %dl
   b0d19:	cmp    $0xf,%al
   b0d1b:	sete   %r11b
   b0d1f:	test   %dil,%dil
   b0d22:	sete   %dil
   b0d26:	test   %al,%al
   b0d28:	sete   %al
   b0d2b:	mov    %r15d,0x10(%rsp,%r14,4)
   b0d30:	lea    0x1(%r14),%rbx
   b0d34:	and    $0xf,%ebx
   b0d37:	or     %r11b,%al
   b0d3a:	mov    %ebp,0x10(%rsp,%rbx,4)
   b0d3e:	movzbl %al,%eax
   b0d41:	add    %r14,%rax
   b0d44:	inc    %rax
   b0d47:	or     %dl,%dil
   b0d4a:	movzbl %dil,%r14d
   b0d4e:	add    %rax,%r14
   b0d51:	and    $0xf,%eax
   b0d54:	mov    %r12d,0x10(%rsp,%rax,4)
   b0d59:	mov    %ebp,0x70(%rsp,%rcx,8)
   b0d5d:	mov    %r15d,0x74(%rsp,%rcx,8)
   b0d62:	lea    0x1(%rcx),%r11d
   b0d66:	and    $0xf,%r11d
   b0d6a:	mov    $0x3,%eax
   b0d6f:	mov    $0x2,%edi
   b0d74:	mov    %r13,%rbx
   b0d77:	mov    0xc(%rsp),%r13d
   b0d7c:	jmp    b0889 <meshoptimizer_rs::codec::raw+0x519>
   b0d81:	movzbl 0x4(%r10,%rbx,1),%eax
   b0d87:	add    $0x5,%rbx
   b0d8b:	shl    $0x1c,%eax
   b0d8e:	or     %r12d,%eax
   b0d91:	jmp    b0973 <meshoptimizer_rs::codec::raw+0x603>
   b0d96:	add    $0x6,%rbx
   b0d9a:	movzbl 0x4(%r10,%r13,1),%edx
   b0da0:	shl    $0x1c,%edx
   b0da3:	or     %r15d,%edx
   b0da6:	jmp    b0aa1 <meshoptimizer_rs::codec::raw+0x731>
   b0dab:	movzbl 0x4(%r10,%r13,1),%r11d
   b0db1:	shl    $0x1c,%r11d
   b0db5:	or     %ebp,%r11d
   b0db8:	jmp    b0bde <meshoptimizer_rs::codec::raw+0x86e>
   b0dbd:	movzbl 0x4(%r10,%r13,1),%r11d
   b0dc3:	shl    $0x1c,%r11d
   b0dc7:	or     %r12d,%r11d
   b0dca:	jmp    b0c8d <meshoptimizer_rs::codec::raw+0x91d>
   b0dcf:	mov    %r11b,%al
   b0dd2:	add    %eax,%r15d
   b0dd5:	mov    %r12d,0x10(%rsp,%r14,4)
   b0dda:	mov    %r15d,%r13d
   b0ddd:	not    %edx
   b0ddf:	shr    $0x4,%edx
   b0de2:	add    %ecx,%edx
   b0de4:	and    $0xf,%edx
   b0de7:	mov    0x70(%rsp,%rdx,8),%r15d
   b0dec:	mov    0x74(%rsp,%rdx,8),%ebp
   b0df0:	add    %rax,%r14
   b0df3:	mov    $0x2,%eax
   b0df8:	mov    $0x1,%edi
   b0dfd:	mov    %rcx,%r11
   b0e00:	inc    %r8
   b0e03:	mov    %r12d,0x70(%rsp,%r11,8)
   b0e08:	mov    %ebp,0x74(%rsp,%r11,8)
   b0e0d:	add    %ecx,%edi
   b0e0f:	and    $0xf,%edi
   b0e12:	mov    %r15d,0x70(%rsp,%rdi,8)
   b0e17:	mov    %r12d,0x74(%rsp,%rdi,8)
   b0e1c:	add    %ecx,%eax
   b0e1e:	and    $0xf,%eax
   b0e21:	and    $0xf,%r14d
   b0e25:	mov    %r15d,-0x8(%rsi)
   b0e29:	mov    %ebp,-0x4(%rsi)
   b0e2c:	mov    %r12d,(%rsi)
   b0e2f:	add    $0xc,%rsi
   b0e33:	mov    %rax,%rcx
   b0e36:	mov    %r13d,%r15d
   b0e39:	cmp    %r8,0x68(%rsp)
   b0e3e:	je     b1343 <meshoptimizer_rs::codec::raw+0xfd3>
   b0e44:	lea    -0x10(%r9),%rax
   b0e48:	cmp    %rax,%rbx
   b0e4b:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b0e51:	movzbl 0x1(%r10,%r8,1),%edx
   b0e57:	cmp    $0xf0,%dl
   b0e5a:	jae    b0e8b <meshoptimizer_rs::codec::raw+0xb1b>
   b0e5c:	mov    %edx,%edi
   b0e5e:	and    $0xf,%edi
   b0e61:	cmp    %edi,0x60(%rsp)
   b0e65:	jbe    b0ecd <meshoptimizer_rs::codec::raw+0xb5d>
   b0e67:	xor    %eax,%eax
   b0e69:	mov    %r15d,%r12d
   b0e6c:	test   %dil,%dil
   b0e6f:	sete   %r11b
   b0e73:	je     b0dcf <meshoptimizer_rs::codec::raw+0xa5f>
   b0e79:	not    %edi
   b0e7b:	add    %r14d,%edi
   b0e7e:	and    $0xf,%edi
   b0e81:	mov    0x10(%rsp,%rdi,4),%r12d
   b0e86:	jmp    b0dcf <meshoptimizer_rs::codec::raw+0xa5f>
   b0e8b:	cmp    $0xfe,%dl
   b0e8e:	jae    b0eef <meshoptimizer_rs::codec::raw+0xb7f>
   b0e90:	and    $0xf,%edx
   b0e93:	lea    (%r10,%r9,1),%rax
   b0e97:	add    $0xfffffffffffffff0,%rax
   b0e9b:	movzbl (%rax,%rdx,1),%r11d
   b0ea0:	lea    0x1(%r15),%ebp
   b0ea4:	xor    %eax,%eax
   b0ea6:	mov    %r11d,%edx
   b0ea9:	shr    $0x4,%edx
   b0eac:	sete   %dil
   b0eb0:	mov    %rbx,0x58(%rsp)
   b0eb5:	je     b0f2c <meshoptimizer_rs::codec::raw+0xbbc>
   b0eb7:	mov    %r14d,%r12d
   b0eba:	sub    %edx,%r12d
   b0ebd:	and    $0xf,%r12d
   b0ec1:	mov    %ebp,%r13d
   b0ec4:	mov    0x10(%rsp,%r12,4),%edx
   b0ec9:	mov    %edx,%ebp
   b0ecb:	jmp    b0f30 <meshoptimizer_rs::codec::raw+0xbc0>
   b0ecd:	cmp    $0xf,%dil
   b0ed1:	jne    b0fa8 <meshoptimizer_rs::codec::raw+0xc38>
   b0ed7:	movzbl (%r10,%rbx,1),%eax
   b0edc:	test   %al,%al
   b0ede:	js     b1020 <meshoptimizer_rs::codec::raw+0xcb0>
   b0ee4:	inc    %rbx
   b0ee7:	mov    %eax,%r12d
   b0eea:	jmp    b10be <meshoptimizer_rs::codec::raw+0xd4e>
   b0eef:	movzbl (%r10,%rbx,1),%r11d
   b0ef4:	xor    %ebp,%ebp
   b0ef6:	cmp    $0xfe,%edx
   b0efc:	sete   %bpl
   b0f00:	test   %r11d,%r11d
   b0f03:	cmove  %r11d,%r15d
   b0f07:	add    %r15d,%ebp
   b0f0a:	mov    %r11d,%eax
   b0f0d:	shr    $0x4,%eax
   b0f10:	je     b0fc7 <meshoptimizer_rs::codec::raw+0xc57>
   b0f16:	mov    %r14d,%edi
   b0f19:	sub    %eax,%edi
   b0f1b:	and    $0xf,%edi
   b0f1e:	mov    %ebp,%r12d
   b0f21:	mov    0x10(%rsp,%rdi,4),%edi
   b0f25:	mov    %edi,%ebp
   b0f27:	jmp    b0fcb <meshoptimizer_rs::codec::raw+0xc5b>
   b0f2c:	lea    0x2(%r15),%r13d
   b0f30:	xor    %edx,%edx
   b0f32:	mov    %r13d,%ebx
   b0f35:	mov    %r13d,%r12d
   b0f38:	test   $0xf,%r11b
   b0f3c:	sete   %r13b
   b0f40:	je     b0f51 <meshoptimizer_rs::codec::raw+0xbe1>
   b0f42:	mov    %r14d,%r12d
   b0f45:	sub    %r11d,%r12d
   b0f48:	and    $0xf,%r12d
   b0f4c:	mov    0x10(%rsp,%r12,4),%r12d
   b0f51:	mov    %r13b,%dl
   b0f54:	mov    %ebx,%r13d
   b0f57:	add    %edx,%r13d
   b0f5a:	mov    %r15d,0x10(%rsp,%r14,4)
   b0f5f:	lea    0x1(%r14),%r11
   b0f63:	and    $0xf,%r11d
   b0f67:	mov    %ebp,0x10(%rsp,%r11,4)
   b0f6c:	mov    %dil,%al
   b0f6f:	add    %r14,%rax
   b0f72:	inc    %rax
   b0f75:	add    %rax,%rdx
   b0f78:	and    $0xf,%eax
   b0f7b:	mov    %r12d,0x10(%rsp,%rax,4)
   b0f80:	mov    %ebp,0x70(%rsp,%rcx,8)
   b0f84:	mov    %r15d,0x74(%rsp,%rcx,8)
   b0f89:	lea    0x1(%rcx),%r11d
   b0f8d:	and    $0xf,%r11d
   b0f91:	mov    $0x3,%eax
   b0f96:	mov    $0x2,%edi
   b0f9b:	mov    %rdx,%r14
   b0f9e:	mov    0x58(%rsp),%rbx
   b0fa3:	jmp    b0e00 <meshoptimizer_rs::codec::raw+0xa90>
   b0fa8:	movzbl %dil,%eax
   b0fac:	xor    $0x3,%dil
   b0fb0:	movzbl %dil,%edi
   b0fb4:	mov    0x8(%rsp),%r11d
   b0fb9:	add    %eax,%r11d
   b0fbc:	sub    %edi,%r11d
   b0fbf:	mov    %r11d,%r12d
   b0fc2:	jmp    b10d4 <meshoptimizer_rs::codec::raw+0xd64>
   b0fc7:	lea    0x1(%rbp),%r12d
   b0fcb:	mov    %r11d,%edi
   b0fce:	and    $0xf,%dil
   b0fd2:	je     b0fed <meshoptimizer_rs::codec::raw+0xc7d>
   b0fd4:	mov    %r14d,%r13d
   b0fd7:	sub    %r11d,%r13d
   b0fda:	and    $0xf,%r13d
   b0fde:	mov    %r12d,0xc(%rsp)
   b0fe3:	mov    0x10(%rsp,%r13,4),%r11d
   b0fe8:	mov    %r11d,%r12d
   b0feb:	jmp    b0ff7 <meshoptimizer_rs::codec::raw+0xc87>
   b0fed:	lea    0x1(%r12),%r11d
   b0ff2:	mov    %r11d,0xc(%rsp)
   b0ff7:	lea    0x1(%rbx),%r13
   b0ffb:	cmp    $0xfe,%edx
   b1001:	je     b1123 <meshoptimizer_rs::codec::raw+0xdb3>
   b1007:	movzbl 0x1(%r10,%rbx,1),%edx
   b100d:	test   %dl,%dl
   b100f:	js     b1042 <meshoptimizer_rs::codec::raw+0xcd2>
   b1011:	add    $0x2,%rbx
   b1015:	mov    %rbx,%r13
   b1018:	mov    %edx,%r15d
   b101b:	jmp    b110a <meshoptimizer_rs::codec::raw+0xd9a>
   b1020:	and    $0x7f,%eax
   b1023:	movzbl 0x1(%r10,%rbx,1),%r12d
   b1029:	mov    %r12d,%edi
   b102c:	and    $0x7f,%r12d
   b1030:	shl    $0x7,%r12d
   b1034:	or     %eax,%r12d
   b1037:	test   %dil,%dil
   b103a:	js     b1067 <meshoptimizer_rs::codec::raw+0xcf7>
   b103c:	add    $0x2,%rbx
   b1040:	jmp    b10be <meshoptimizer_rs::codec::raw+0xd4e>
   b1042:	and    $0x7f,%edx
   b1045:	movzbl 0x2(%r10,%rbx,1),%r15d
   b104b:	mov    %r15d,%r11d
   b104e:	and    $0x7f,%r15d
   b1052:	shl    $0x7,%r15d
   b1056:	or     %edx,%r15d
   b1059:	test   %r11b,%r11b
   b105c:	js     b1083 <meshoptimizer_rs::codec::raw+0xd13>
   b105e:	add    $0x3,%rbx
   b1062:	jmp    b1107 <meshoptimizer_rs::codec::raw+0xd97>
   b1067:	movzbl 0x2(%r10,%rbx,1),%eax
   b106d:	mov    %eax,%edi
   b106f:	and    $0x7f,%eax
   b1072:	shl    $0xe,%eax
   b1075:	or     %eax,%r12d
   b1078:	test   %dil,%dil
   b107b:	js     b10a0 <meshoptimizer_rs::codec::raw+0xd30>
   b107d:	add    $0x3,%rbx
   b1081:	jmp    b10be <meshoptimizer_rs::codec::raw+0xd4e>
   b1083:	movzbl 0x3(%r10,%rbx,1),%edx
   b1089:	mov    %edx,%r11d
   b108c:	and    $0x7f,%edx
   b108f:	shl    $0xe,%edx
   b1092:	or     %edx,%r15d
   b1095:	test   %r11b,%r11b
   b1098:	js     b10e8 <meshoptimizer_rs::codec::raw+0xd78>
   b109a:	add    $0x4,%rbx
   b109e:	jmp    b1107 <meshoptimizer_rs::codec::raw+0xd97>
   b10a0:	movzbl 0x3(%r10,%rbx,1),%eax
   b10a6:	mov    %eax,%edi
   b10a8:	and    $0x7f,%eax
   b10ab:	shl    $0x15,%eax
   b10ae:	or     %eax,%r12d
   b10b1:	test   %dil,%dil
   b10b4:	js     b12f5 <meshoptimizer_rs::codec::raw+0xf85>
   b10ba:	add    $0x4,%rbx
   b10be:	mov    0x8(%rsp),%edi
   b10c2:	mov    %r12d,%eax
   b10c5:	shr    $1,%eax
   b10c7:	and    $0x1,%r12d
   b10cb:	neg    %r12d
   b10ce:	xor    %eax,%r12d
   b10d1:	add    %edi,%r12d
   b10d4:	mov    %r12d,0x10(%rsp,%r14,4)
   b10d9:	mov    $0x1,%eax
   b10de:	mov    %r12d,0x8(%rsp)
   b10e3:	jmp    b0dda <meshoptimizer_rs::codec::raw+0xa6a>
   b10e8:	movzbl 0x4(%r10,%rbx,1),%edx
   b10ee:	mov    %edx,%r11d
   b10f1:	and    $0x7f,%edx
   b10f4:	shl    $0x15,%edx
   b10f7:	or     %edx,%r15d
   b10fa:	test   %r11b,%r11b
   b10fd:	js     b130a <meshoptimizer_rs::codec::raw+0xf9a>
   b1103:	add    $0x5,%rbx
   b1107:	mov    %rbx,%r13
   b110a:	mov    %r15d,%edx
   b110d:	shr    $1,%edx
   b110f:	and    $0x1,%r15d
   b1113:	neg    %r15d
   b1116:	xor    %edx,%r15d
   b1119:	add    0x8(%rsp),%r15d
   b111e:	mov    %r15d,0x8(%rsp)
   b1123:	cmp    $0xf,%al
   b1125:	jne    b11d0 <meshoptimizer_rs::codec::raw+0xe60>
   b112b:	cmp    $0xfffffffffffffffa,%r13
   b112f:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b1135:	lea    0x5(%r13),%rdx
   b1139:	cmp    %r9,%rdx
   b113c:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b1142:	movzbl (%r10,%r13,1),%r11d
   b1147:	test   %r11b,%r11b
   b114a:	js     b1157 <meshoptimizer_rs::codec::raw+0xde7>
   b114c:	inc    %r13
   b114f:	mov    %r13,%rdx
   b1152:	mov    %r11d,%ebp
   b1155:	jmp    b11b7 <meshoptimizer_rs::codec::raw+0xe47>
   b1157:	and    $0x7f,%r11d
   b115b:	movzbl 0x1(%r10,%r13,1),%ebp
   b1161:	mov    %ebp,%ebx
   b1163:	and    $0x7f,%ebp
   b1166:	shl    $0x7,%ebp
   b1169:	or     %r11d,%ebp
   b116c:	test   %bl,%bl
   b116e:	js     b1176 <meshoptimizer_rs::codec::raw+0xe06>
   b1170:	add    $0x2,%r13
   b1174:	jmp    b11b4 <meshoptimizer_rs::codec::raw+0xe44>
   b1176:	movzbl 0x2(%r10,%r13,1),%r11d
   b117c:	mov    %r11d,%ebx
   b117f:	and    $0x7f,%r11d
   b1183:	shl    $0xe,%r11d
   b1187:	or     %r11d,%ebp
   b118a:	test   %bl,%bl
   b118c:	js     b1194 <meshoptimizer_rs::codec::raw+0xe24>
   b118e:	add    $0x3,%r13
   b1192:	jmp    b11b4 <meshoptimizer_rs::codec::raw+0xe44>
   b1194:	movzbl 0x3(%r10,%r13,1),%r11d
   b119a:	mov    %r11d,%ebx
   b119d:	and    $0x7f,%r11d
   b11a1:	shl    $0x15,%r11d
   b11a5:	or     %r11d,%ebp
   b11a8:	test   %bl,%bl
   b11aa:	js     b131f <meshoptimizer_rs::codec::raw+0xfaf>
   b11b0:	add    $0x4,%r13
   b11b4:	mov    %r13,%rdx
   b11b7:	mov    %ebp,%r11d
   b11ba:	shr    $1,%r11d
   b11bd:	and    $0x1,%ebp
   b11c0:	neg    %ebp
   b11c2:	xor    %r11d,%ebp
   b11c5:	add    0x8(%rsp),%ebp
   b11c9:	mov    %rdx,%r13
   b11cc:	mov    %ebp,0x8(%rsp)
   b11d0:	cmp    $0xf,%dil
   b11d4:	jne    b1286 <meshoptimizer_rs::codec::raw+0xf16>
   b11da:	cmp    $0xfffffffffffffffa,%r13
   b11de:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b11e4:	lea    0x5(%r13),%rdx
   b11e8:	cmp    %r9,%rdx
   b11eb:	ja     b134c <meshoptimizer_rs::codec::raw+0xfdc>
   b11f1:	movzbl (%r10,%r13,1),%r11d
   b11f6:	test   %r11b,%r11b
   b11f9:	js     b1206 <meshoptimizer_rs::codec::raw+0xe96>
   b11fb:	inc    %r13
   b11fe:	mov    %r13,%rdx
   b1201:	mov    %r11d,%r12d
   b1204:	jmp    b1269 <meshoptimizer_rs::codec::raw+0xef9>
   b1206:	and    $0x7f,%r11d
   b120a:	movzbl 0x1(%r10,%r13,1),%r12d
   b1210:	mov    %r12d,%ebx
   b1213:	and    $0x7f,%r12d
   b1217:	shl    $0x7,%r12d
   b121b:	or     %r11d,%r12d
   b121e:	test   %bl,%bl
   b1220:	js     b1228 <meshoptimizer_rs::codec::raw+0xeb8>
   b1222:	add    $0x2,%r13
   b1226:	jmp    b1266 <meshoptimizer_rs::codec::raw+0xef6>
   b1228:	movzbl 0x2(%r10,%r13,1),%r11d
   b122e:	mov    %r11d,%ebx
   b1231:	and    $0x7f,%r11d
   b1235:	shl    $0xe,%r11d
   b1239:	or     %r11d,%r12d
   b123c:	test   %bl,%bl
   b123e:	js     b1246 <meshoptimizer_rs::codec::raw+0xed6>
   b1240:	add    $0x3,%r13
   b1244:	jmp    b1266 <meshoptimizer_rs::codec::raw+0xef6>
   b1246:	movzbl 0x3(%r10,%r13,1),%r11d
   b124c:	mov    %r11d,%ebx
   b124f:	and    $0x7f,%r11d
   b1253:	shl    $0x15,%r11d
   b1257:	or     %r11d,%r12d
   b125a:	test   %bl,%bl
   b125c:	js     b1331 <meshoptimizer_rs::codec::raw+0xfc1>
   b1262:	add    $0x4,%r13
   b1266:	mov    %r13,%rdx
   b1269:	mov    %r12d,%r11d
   b126c:	shr    $1,%r11d
   b126f:	and    $0x1,%r12d
   b1273:	neg    %r12d
   b1276:	xor    %r11d,%r12d
   b1279:	add    0x8(%rsp),%r12d
   b127e:	mov    %rdx,%r13
   b1281:	mov    %r12d,0x8(%rsp)
   b1286:	cmp    $0xf,%dil
   b128a:	sete   %dl
   b128d:	cmp    $0xf,%al
   b128f:	sete   %r11b
   b1293:	test   %dil,%dil
   b1296:	sete   %dil
   b129a:	test   %al,%al
   b129c:	sete   %al
   b129f:	mov    %r15d,0x10(%rsp,%r14,4)
   b12a4:	lea    0x1(%r14),%rbx
   b12a8:	and    $0xf,%ebx
   b12ab:	or     %r11b,%al
   b12ae:	mov    %ebp,0x10(%rsp,%rbx,4)
   b12b2:	movzbl %al,%eax
   b12b5:	add    %r14,%rax
   b12b8:	inc    %rax
   b12bb:	or     %dl,%dil
   b12be:	movzbl %dil,%r14d
   b12c2:	add    %rax,%r14
   b12c5:	and    $0xf,%eax
   b12c8:	mov    %r12d,0x10(%rsp,%rax,4)
   b12cd:	mov    %ebp,0x70(%rsp,%rcx,8)
   b12d1:	mov    %r15d,0x74(%rsp,%rcx,8)
   b12d6:	lea    0x1(%rcx),%r11d
   b12da:	and    $0xf,%r11d
   b12de:	mov    $0x3,%eax
   b12e3:	mov    $0x2,%edi
   b12e8:	mov    %r13,%rbx
   b12eb:	mov    0xc(%rsp),%r13d
   b12f0:	jmp    b0e00 <meshoptimizer_rs::codec::raw+0xa90>
   b12f5:	movzbl 0x4(%r10,%rbx,1),%eax
   b12fb:	add    $0x5,%rbx
   b12ff:	shl    $0x1c,%eax
   b1302:	or     %r12d,%eax
   b1305:	jmp    b0ee7 <meshoptimizer_rs::codec::raw+0xb77>
   b130a:	add    $0x6,%rbx
   b130e:	movzbl 0x4(%r10,%r13,1),%edx
   b1314:	shl    $0x1c,%edx
   b1317:	or     %r15d,%edx
   b131a:	jmp    b1015 <meshoptimizer_rs::codec::raw+0xca5>
   b131f:	movzbl 0x4(%r10,%r13,1),%r11d
   b1325:	shl    $0x1c,%r11d
   b1329:	or     %ebp,%r11d
   b132c:	jmp    b1152 <meshoptimizer_rs::codec::raw+0xde2>
   b1331:	movzbl 0x4(%r10,%r13,1),%r11d
   b1337:	shl    $0x1c,%r11d
   b133b:	or     %r12d,%r11d
   b133e:	jmp    b1201 <meshoptimizer_rs::codec::raw+0xe91>
   b1343:	mov    $0xff,%al
   b1345:	cmp    0x50(%rsp),%rbx
   b134a:	je     b134e <meshoptimizer_rs::codec::raw+0xfde>
   b134c:	mov    $0x7,%al
   b134e:	add    $0xf8,%rsp
   b1355:	pop    %rbx
   b1356:	pop    %r12
   b1358:	pop    %r13
   b135a:	pop    %r14
   b135c:	pop    %r15
   b135e:	pop    %rbp
   b135f:	ret
   b1360:	lea    0x770f9(%rip),%rdx        # 128460 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5248>
   b1367:	xor    %edi,%edi
   b1369:	xor    %esi,%esi
   b136b:	call   *0x7944f(%rip)        # 12a7c0 <_DYNAMIC+0x250>
