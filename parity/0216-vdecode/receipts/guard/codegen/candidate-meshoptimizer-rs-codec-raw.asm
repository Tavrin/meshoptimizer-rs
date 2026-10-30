
<artifacts>/guard/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000bb530 <meshoptimizer_rs::codec::raw>:
   bb530:	push   %rbp
   bb531:	push   %r15
   bb533:	push   %r14
   bb535:	push   %r13
   bb537:	push   %r12
   bb539:	push   %rbx
   bb53a:	sub    $0x5f8,%rsp
   bb541:	mov    %r9,%r14
   bb544:	mov    %r8,%rbp
   bb547:	mov    %rcx,%r13
   bb54a:	mov    %rdx,%r12
   bb54d:	mov    %rsi,%r15
   bb550:	mov    0x630(%rsp),%rbx
   bb558:	test   %dil,%dil
   bb55b:	je     bb6c4 <meshoptimizer_rs::codec::raw+0x194>
   bb561:	movzbl %dil,%eax
   bb565:	cmp    $0x1,%eax
   bb568:	jne    bb786 <meshoptimizer_rs::codec::raw+0x256>
   bb56e:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   bb578:	mov    %r13,%rax
   bb57b:	mul    %rdi
   bb57e:	shr    $1,%rdx
   bb581:	lea    (%rdx,%rdx,2),%rcx
   bb585:	mov    %r13,%rax
   bb588:	sub    %rcx,%rax
   bb58b:	cmp    $0x2,%rbp
   bb58f:	jne    bb7f1 <meshoptimizer_rs::codec::raw+0x2c1>
   bb595:	test   %rax,%rax
   bb598:	jne    bb7fa <meshoptimizer_rs::codec::raw+0x2ca>
   bb59e:	mov    %r13,%rax
   bb5a1:	mul    %rdi
   bb5a4:	shr    $1,%rdx
   bb5a7:	lea    0x11(%rdx),%rcx
   bb5ab:	mov    $0x7,%al
   bb5ad:	cmp    %rcx,%rbx
   bb5b0:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb5b6:	movzbl (%r14),%ecx
   bb5ba:	mov    %ecx,%esi
   bb5bc:	and    $0xf0,%sil
   bb5c0:	cmp    $0xe0,%sil
   bb5c4:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb5ca:	and    $0xf,%cl
   bb5cd:	mov    $0x5,%al
   bb5cf:	cmp    $0x1,%cl
   bb5d2:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb5d8:	lea    0x1(%rdx),%rsi
   bb5dc:	pcmpeqd %xmm0,%xmm0
   bb5e0:	movdqa %xmm0,0x260(%rsp)
   bb5e9:	movdqa %xmm0,0x250(%rsp)
   bb5f2:	movdqa %xmm0,0x240(%rsp)
   bb5fb:	movdqa %xmm0,0x230(%rsp)
   bb604:	movdqa %xmm0,0x220(%rsp)
   bb60d:	movdqa %xmm0,0x210(%rsp)
   bb616:	movdqa %xmm0,0x200(%rsp)
   bb61f:	movdqa %xmm0,0x1f0(%rsp)
   bb628:	movdqa %xmm0,0x120(%rsp)
   bb631:	movdqa %xmm0,0x110(%rsp)
   bb63a:	movdqa %xmm0,0x100(%rsp)
   bb643:	movdqa %xmm0,0xf0(%rsp)
   bb64c:	xor    %r8d,%r8d
   bb64f:	cmp    $0x1,%cl
   bb652:	setne  %r9b
   bb656:	lea    -0x10(%rbx),%rax
   bb65a:	mov    %rax,0x28(%rsp)
   bb65f:	mov    %r12,%rax
   bb662:	mov    %rdx,%rcx
   bb665:	mul    %rdi
   bb668:	shr    $0x2,%rdx
   bb66c:	cmp    %rcx,%rdx
   bb66f:	cmovae %rcx,%rdx
   bb673:	mov    %rdx,0x38(%rsp)
   bb678:	test   %rdx,%rdx
   bb67b:	je     bd07f <meshoptimizer_rs::codec::raw+0x1b4f>
   bb681:	mov    %r9b,%r8b
   bb684:	lea    0xd(,%r8,2),%rax
   bb68c:	mov    %rax,0x58(%rsp)
   bb691:	mov    %r14,0x30(%rsp)
   bb696:	lea    (%r14,%rbx,1),%rax
   bb69a:	add    $0xfffffffffffffff0,%rax
   bb69e:	mov    %rax,0x50(%rsp)
   bb6a3:	add    $0x4,%r15
   bb6a7:	xor    %r8d,%r8d
   bb6aa:	xor    %r10d,%r10d
   bb6ad:	movl   $0x0,0x20(%rsp)
   bb6b5:	xor    %r11d,%r11d
   bb6b8:	xor    %edi,%edi
   bb6ba:	mov    0x28(%rsp),%r9
   bb6bf:	jmp    bbb86 <meshoptimizer_rs::codec::raw+0x656>
   bb6c4:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   bb6c9:	cmp    $0x2,%al
   bb6cb:	jb     bb734 <meshoptimizer_rs::codec::raw+0x204>
   bb6cd:	cmp    $0x20,%rbp
   bb6d1:	lea    -0x37738(%rip),%rax        # 83fa0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   bb6d8:	lea    -0x38c1f(%rip),%rcx        # 82ac0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   bb6df:	cmove  %rax,%rcx
   bb6e3:	lea    -0x3125a(%rip),%rax        # 8a490 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   bb6ea:	lea    -0x32b01(%rip),%rdx        # 88bf0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   bb6f1:	cmove  %rax,%rdx
   bb6f5:	cmp    $0x4,%rbp
   bb6f9:	lea    -0x350d0(%rip),%rax        # 86630 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   bb700:	cmovne %rcx,%rax
   bb704:	lea    -0x2e7db(%rip),%r10        # 8cf30 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   bb70b:	cmovne %rdx,%r10
   bb70f:	cmp    $0x21,%r13
   bb713:	cmovae %rax,%r10
   bb717:	mov    %r15,%rdi
   bb71a:	mov    %r12,%rsi
   bb71d:	mov    %r13,%rdx
   bb720:	mov    %rbp,%rcx
   bb723:	mov    %r14,%r8
   bb726:	mov    %rbx,%r9
   bb729:	call   *%r10
   bb72c:	cmp    $0xfe,%al
   bb72e:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb734:	mov    $0x7,%al
   bb736:	test   %rbx,%rbx
   bb739:	je     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb73f:	movzbl (%r14),%edi
   bb743:	mov    %edi,%ecx
   bb745:	and    $0xf0,%cl
   bb748:	cmp    $0xa0,%cl
   bb74b:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb751:	and    $0xf,%dil
   bb755:	mov    $0x5,%al
   bb757:	cmp    $0x1,%dil
   bb75b:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb761:	test   %dil,%dil
   bb764:	je     bc683 <meshoptimizer_rs::codec::raw+0x1153>
   bb76a:	mov    %rbp,%rcx
   bb76d:	shr    $0x2,%rcx
   bb771:	add    %rbp,%rcx
   bb774:	cmp    $0x19,%rcx
   bb778:	mov    $0x18,%eax
   bb77d:	cmovae %rcx,%rax
   bb781:	jmp    bc693 <meshoptimizer_rs::codec::raw+0x1163>
   bb786:	add    $0x5,%r13
   bb78a:	mov    $0x7,%al
   bb78c:	cmp    $0x2,%rbp
   bb790:	jne    bb801 <meshoptimizer_rs::codec::raw+0x2d1>
   bb792:	cmp    %r13,%rbx
   bb795:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb79b:	test   %rbx,%rbx
   bb79e:	je     bd134 <meshoptimizer_rs::codec::raw+0x1c04>
   bb7a4:	movzbl (%r14),%ecx
   bb7a8:	mov    %ecx,%edx
   bb7aa:	and    $0xf0,%dl
   bb7ad:	cmp    $0xd0,%dl
   bb7b0:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb7b6:	mov    $0x5,%al
   bb7b8:	test   $0xe,%cl
   bb7bb:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb7c1:	dec    %rbx
   bb7c4:	movq   $0x0,0x1f0(%rsp)
   bb7d0:	movabs $0x7ffffffffffffffc,%rax
   bb7da:	or     $0x2,%rax
   bb7de:	and    %rax,%r12
   bb7e1:	je     bbae4 <meshoptimizer_rs::codec::raw+0x5b4>
   bb7e7:	inc    %r14
   bb7ea:	xor    %eax,%eax
   bb7ec:	jmp    bb896 <meshoptimizer_rs::codec::raw+0x366>
   bb7f1:	test   %rax,%rax
   bb7f4:	je     bb9c7 <meshoptimizer_rs::codec::raw+0x497>
   bb7fa:	xor    %eax,%eax
   bb7fc:	jmp    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb801:	cmp    %r13,%rbx
   bb804:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb80a:	test   %rbx,%rbx
   bb80d:	je     bd134 <meshoptimizer_rs::codec::raw+0x1c04>
   bb813:	movzbl (%r14),%ecx
   bb817:	mov    %ecx,%edx
   bb819:	and    $0xf0,%dl
   bb81c:	cmp    $0xd0,%dl
   bb81f:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb825:	mov    $0x5,%al
   bb827:	test   $0xe,%cl
   bb82a:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb830:	dec    %rbx
   bb833:	movq   $0x0,0x1f0(%rsp)
   bb83f:	movabs $0x7ffffffffffffffc,%rax
   bb849:	and    %rax,%r12
   bb84c:	je     bbae4 <meshoptimizer_rs::codec::raw+0x5b4>
   bb852:	inc    %r14
   bb855:	xor    %eax,%eax
   bb857:	jmp    bb94b <meshoptimizer_rs::codec::raw+0x41b>
   bb85c:	dec    %rbx
   bb85f:	inc    %r14
   bb862:	mov    %edx,%ecx
   bb864:	mov    %ecx,%edx
   bb866:	shr    $0x2,%edx
   bb869:	mov    %ecx,%esi
   bb86b:	shl    $0x1e,%esi
   bb86e:	sar    $0x1f,%esi
   bb871:	xor    %edx,%esi
   bb873:	and    $0x1,%ecx
   bb876:	add    0x1f0(%rsp,%rcx,4),%esi
   bb87d:	mov    %esi,0x1f0(%rsp,%rcx,4)
   bb884:	mov    %si,(%r15,%rax,1)
   bb889:	add    $0x2,%rax
   bb88d:	cmp    %rax,%r12
   bb890:	je     bbae4 <meshoptimizer_rs::codec::raw+0x5b4>
   bb896:	cmp    $0x5,%rbx
   bb89a:	jb     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bb8a0:	movzbl (%r14),%edx
   bb8a4:	test   %dl,%dl
   bb8a6:	jns    bb85c <meshoptimizer_rs::codec::raw+0x32c>
   bb8a8:	and    $0x7f,%edx
   bb8ab:	movzbl 0x1(%r14),%ecx
   bb8b0:	mov    %ecx,%esi
   bb8b2:	and    $0x7f,%ecx
   bb8b5:	shl    $0x7,%ecx
   bb8b8:	or     %edx,%ecx
   bb8ba:	mov    $0x2,%edx
   bb8bf:	test   %sil,%sil
   bb8c2:	jns    bb907 <meshoptimizer_rs::codec::raw+0x3d7>
   bb8c4:	movzbl 0x2(%r14),%edx
   bb8c9:	mov    %edx,%esi
   bb8cb:	and    $0x7f,%edx
   bb8ce:	shl    $0xe,%edx
   bb8d1:	or     %edx,%ecx
   bb8d3:	mov    $0x3,%edx
   bb8d8:	test   %sil,%sil
   bb8db:	jns    bb907 <meshoptimizer_rs::codec::raw+0x3d7>
   bb8dd:	movzbl 0x3(%r14),%edx
   bb8e2:	mov    %edx,%esi
   bb8e4:	and    $0x7f,%edx
   bb8e7:	shl    $0x15,%edx
   bb8ea:	or     %edx,%ecx
   bb8ec:	mov    $0x4,%edx
   bb8f1:	test   %sil,%sil
   bb8f4:	jns    bb907 <meshoptimizer_rs::codec::raw+0x3d7>
   bb8f6:	movzbl 0x4(%r14),%esi
   bb8fb:	shl    $0x1c,%esi
   bb8fe:	or     %ecx,%esi
   bb900:	mov    $0x5,%edx
   bb905:	mov    %esi,%ecx
   bb907:	sub    %rdx,%rbx
   bb90a:	add    %rdx,%r14
   bb90d:	jmp    bb864 <meshoptimizer_rs::codec::raw+0x334>
   bb912:	dec    %rbx
   bb915:	inc    %r14
   bb918:	mov    %edx,%ecx
   bb91a:	mov    %ecx,%edx
   bb91c:	shr    $0x2,%edx
   bb91f:	mov    %ecx,%esi
   bb921:	shl    $0x1e,%esi
   bb924:	sar    $0x1f,%esi
   bb927:	xor    %edx,%esi
   bb929:	and    $0x1,%ecx
   bb92c:	add    0x1f0(%rsp,%rcx,4),%esi
   bb933:	mov    %esi,0x1f0(%rsp,%rcx,4)
   bb93a:	mov    %esi,(%r15,%rax,1)
   bb93e:	add    $0x4,%rax
   bb942:	cmp    %rax,%r12
   bb945:	je     bbae4 <meshoptimizer_rs::codec::raw+0x5b4>
   bb94b:	cmp    $0x5,%rbx
   bb94f:	jb     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bb955:	movzbl (%r14),%edx
   bb959:	test   %dl,%dl
   bb95b:	jns    bb912 <meshoptimizer_rs::codec::raw+0x3e2>
   bb95d:	and    $0x7f,%edx
   bb960:	movzbl 0x1(%r14),%ecx
   bb965:	mov    %ecx,%esi
   bb967:	and    $0x7f,%ecx
   bb96a:	shl    $0x7,%ecx
   bb96d:	or     %edx,%ecx
   bb96f:	mov    $0x2,%edx
   bb974:	test   %sil,%sil
   bb977:	jns    bb9bc <meshoptimizer_rs::codec::raw+0x48c>
   bb979:	movzbl 0x2(%r14),%edx
   bb97e:	mov    %edx,%esi
   bb980:	and    $0x7f,%edx
   bb983:	shl    $0xe,%edx
   bb986:	or     %edx,%ecx
   bb988:	mov    $0x3,%edx
   bb98d:	test   %sil,%sil
   bb990:	jns    bb9bc <meshoptimizer_rs::codec::raw+0x48c>
   bb992:	movzbl 0x3(%r14),%edx
   bb997:	mov    %edx,%esi
   bb999:	and    $0x7f,%edx
   bb99c:	shl    $0x15,%edx
   bb99f:	or     %edx,%ecx
   bb9a1:	mov    $0x4,%edx
   bb9a6:	test   %sil,%sil
   bb9a9:	jns    bb9bc <meshoptimizer_rs::codec::raw+0x48c>
   bb9ab:	movzbl 0x4(%r14),%esi
   bb9b0:	shl    $0x1c,%esi
   bb9b3:	or     %ecx,%esi
   bb9b5:	mov    $0x5,%edx
   bb9ba:	mov    %esi,%ecx
   bb9bc:	sub    %rdx,%rbx
   bb9bf:	add    %rdx,%r14
   bb9c2:	jmp    bb91a <meshoptimizer_rs::codec::raw+0x3ea>
   bb9c7:	lea    0x11(%rdx),%rcx
   bb9cb:	mov    $0x7,%al
   bb9cd:	cmp    %rcx,%rbx
   bb9d0:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb9d6:	movzbl (%r14),%ecx
   bb9da:	mov    %ecx,%esi
   bb9dc:	and    $0xf0,%sil
   bb9e0:	cmp    $0xe0,%sil
   bb9e4:	jne    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb9ea:	and    $0xf,%cl
   bb9ed:	mov    $0x5,%al
   bb9ef:	cmp    $0x1,%cl
   bb9f2:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bb9f8:	lea    0x1(%rdx),%rsi
   bb9fc:	pcmpeqd %xmm0,%xmm0
   bba00:	movdqa %xmm0,0x260(%rsp)
   bba09:	movdqa %xmm0,0x250(%rsp)
   bba12:	movdqa %xmm0,0x240(%rsp)
   bba1b:	movdqa %xmm0,0x230(%rsp)
   bba24:	movdqa %xmm0,0x220(%rsp)
   bba2d:	movdqa %xmm0,0x210(%rsp)
   bba36:	movdqa %xmm0,0x200(%rsp)
   bba3f:	movdqa %xmm0,0x1f0(%rsp)
   bba48:	movdqa %xmm0,0x120(%rsp)
   bba51:	movdqa %xmm0,0x110(%rsp)
   bba5a:	movdqa %xmm0,0x100(%rsp)
   bba63:	movdqa %xmm0,0xf0(%rsp)
   bba6c:	xor    %r8d,%r8d
   bba6f:	cmp    $0x1,%cl
   bba72:	setne  %r9b
   bba76:	lea    -0x10(%rbx),%rax
   bba7a:	mov    %rax,0x28(%rsp)
   bba7f:	mov    %r12,%rax
   bba82:	mov    %rdx,%rcx
   bba85:	mul    %rdi
   bba88:	shr    $0x3,%rdx
   bba8c:	cmp    %rcx,%rdx
   bba8f:	cmovae %rcx,%rdx
   bba93:	mov    %rdx,0x38(%rsp)
   bba98:	test   %rdx,%rdx
   bba9b:	je     bd07f <meshoptimizer_rs::codec::raw+0x1b4f>
   bbaa1:	mov    %r9b,%r8b
   bbaa4:	lea    0xd(,%r8,2),%rax
   bbaac:	mov    %rax,0x58(%rsp)
   bbab1:	mov    %r14,0x30(%rsp)
   bbab6:	lea    (%r14,%rbx,1),%rax
   bbaba:	add    $0xfffffffffffffff0,%rax
   bbabe:	mov    %rax,0x50(%rsp)
   bbac3:	add    $0x8,%r15
   bbac7:	xor    %r8d,%r8d
   bbaca:	xor    %r10d,%r10d
   bbacd:	movl   $0x0,0x20(%rsp)
   bbad5:	xor    %r11d,%r11d
   bbad8:	xor    %edi,%edi
   bbada:	mov    0x28(%rsp),%r9
   bbadf:	jmp    bc14e <meshoptimizer_rs::codec::raw+0xc1e>
   bbae4:	cmp    $0x4,%rbx
   bbae8:	sete   %al
   bbaeb:	jmp    bd079 <meshoptimizer_rs::codec::raw+0x1b49>
   bbaf0:	mov    %bpl,%r14b
   bbaf3:	add    %r14d,%r11d
   bbaf6:	mov    %ebx,0xf0(%rsp,%r10,4)
   bbafe:	mov    %r11d,%ebp
   bbb01:	not    %eax
   bbb03:	shr    $0x4,%eax
   bbb06:	add    %r8d,%eax
   bbb09:	and    $0xf,%eax
   bbb0c:	mov    0x1f0(%rsp,%rax,8),%r11d
   bbb14:	mov    0x1f4(%rsp,%rax,8),%r15d
   bbb1c:	add    %r14,%r10
   bbb1f:	mov    $0x2,%eax
   bbb24:	mov    $0x1,%r12d
   bbb2a:	mov    %r8,%r13
   bbb2d:	inc    %rdi
   bbb30:	mov    %ebx,0x1f0(%rsp,%r13,8)
   bbb38:	mov    %r15d,0x1f4(%rsp,%r13,8)
   bbb40:	add    %r8d,%r12d
   bbb43:	and    $0xf,%r12d
   bbb47:	mov    %r11d,0x1f0(%rsp,%r12,8)
   bbb4f:	mov    %ebx,0x1f4(%rsp,%r12,8)
   bbb57:	add    %r8d,%eax
   bbb5a:	and    $0xf,%eax
   bbb5d:	and    $0xf,%r10d
   bbb61:	mov    %r11w,-0x4(%rdx)
   bbb66:	mov    %r15w,-0x2(%rdx)
   bbb6b:	mov    %rdx,%r15
   bbb6e:	mov    %bx,(%rdx)
   bbb71:	add    $0x6,%r15
   bbb75:	mov    %rax,%r8
   bbb78:	mov    %ebp,%r11d
   bbb7b:	cmp    %rdi,0x38(%rsp)
   bbb80:	je     bd084 <meshoptimizer_rs::codec::raw+0x1b54>
   bbb86:	cmp    %r9,%rsi
   bbb89:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bbb8f:	mov    %r15,%rdx
   bbb92:	mov    0x30(%rsp),%r14
   bbb97:	movzbl 0x1(%r14,%rdi,1),%eax
   bbb9d:	cmp    $0xf0,%al
   bbb9f:	jae    bbbd9 <meshoptimizer_rs::codec::raw+0x6a9>
   bbba1:	mov    %eax,%r15d
   bbba4:	and    $0xf,%r15d
   bbba8:	cmp    %r15d,0x58(%rsp)
   bbbad:	jbe    bbc13 <meshoptimizer_rs::codec::raw+0x6e3>
   bbbaf:	xor    %r14d,%r14d
   bbbb2:	mov    %r11d,%ebx
   bbbb5:	test   %r15b,%r15b
   bbbb8:	sete   %bpl
   bbbbc:	je     bbaf0 <meshoptimizer_rs::codec::raw+0x5c0>
   bbbc2:	not    %r15d
   bbbc5:	add    %r10d,%r15d
   bbbc8:	and    $0xf,%r15d
   bbbcc:	mov    0xf0(%rsp,%r15,4),%ebx
   bbbd4:	jmp    bbaf0 <meshoptimizer_rs::codec::raw+0x5c0>
   bbbd9:	cmp    $0xfe,%al
   bbbdb:	jae    bbc35 <meshoptimizer_rs::codec::raw+0x705>
   bbbdd:	and    $0xf,%eax
   bbbe0:	mov    0x50(%rsp),%rcx
   bbbe5:	movzbl (%rcx,%rax,1),%r13d
   bbbea:	lea    0x1(%r11),%r15d
   bbbee:	xor    %eax,%eax
   bbbf0:	mov    %r13d,%ecx
   bbbf3:	shr    $0x4,%ecx
   bbbf6:	sete   %r12b
   bbbfa:	je     bbc78 <meshoptimizer_rs::codec::raw+0x748>
   bbbfc:	mov    %r10d,%ebx
   bbbff:	sub    %ecx,%ebx
   bbc01:	and    $0xf,%ebx
   bbc04:	mov    %r15d,%ebp
   bbc07:	mov    0xf0(%rsp,%rbx,4),%ecx
   bbc0e:	mov    %ecx,%r15d
   bbc11:	jmp    bbc7c <meshoptimizer_rs::codec::raw+0x74c>
   bbc13:	cmp    $0xf,%r15b
   bbc17:	jne    bbcf8 <meshoptimizer_rs::codec::raw+0x7c8>
   bbc1d:	movzbl (%r14,%rsi,1),%ebp
   bbc22:	test   %bpl,%bpl
   bbc25:	js     bbd66 <meshoptimizer_rs::codec::raw+0x836>
   bbc2b:	inc    %rsi
   bbc2e:	mov    %ebp,%ebx
   bbc30:	jmp    bbefc <meshoptimizer_rs::codec::raw+0x9cc>
   bbc35:	movzbl (%r14,%rsi,1),%r14d
   bbc3a:	xor    %r15d,%r15d
   bbc3d:	cmp    $0xfe,%eax
   bbc42:	sete   %r15b
   bbc46:	test   %r14d,%r14d
   bbc49:	cmove  %r14d,%r11d
   bbc4d:	add    %r11d,%r15d
   bbc50:	mov    %r14d,%r12d
   bbc53:	shr    $0x4,%r12d
   bbc57:	je     bbd13 <meshoptimizer_rs::codec::raw+0x7e3>
   bbc5d:	mov    %r10d,%ecx
   bbc60:	sub    %r12d,%ecx
   bbc63:	and    $0xf,%ecx
   bbc66:	mov    %r15d,%ebx
   bbc69:	mov    0xf0(%rsp,%rcx,4),%ecx
   bbc70:	mov    %ecx,%r15d
   bbc73:	jmp    bbd17 <meshoptimizer_rs::codec::raw+0x7e7>
   bbc78:	lea    0x2(%r11),%ebp
   bbc7c:	xor    %r14d,%r14d
   bbc7f:	mov    %ebp,%ebx
   bbc81:	test   $0xf,%r13b
   bbc85:	sete   %cl
   bbc88:	je     bbc9a <meshoptimizer_rs::codec::raw+0x76a>
   bbc8a:	mov    %r10d,%ebx
   bbc8d:	sub    %r13d,%ebx
   bbc90:	and    $0xf,%ebx
   bbc93:	mov    0xf0(%rsp,%rbx,4),%ebx
   bbc9a:	mov    %cl,%r14b
   bbc9d:	add    %r14d,%ebp
   bbca0:	mov    %r11d,0xf0(%rsp,%r10,4)
   bbca8:	lea    0x1(%r10),%rcx
   bbcac:	and    $0xf,%ecx
   bbcaf:	mov    %r15d,0xf0(%rsp,%rcx,4)
   bbcb7:	mov    %r12b,%al
   bbcba:	add    %r10,%rax
   bbcbd:	inc    %rax
   bbcc0:	add    %rax,%r14
   bbcc3:	and    $0xf,%eax
   bbcc6:	mov    %ebx,0xf0(%rsp,%rax,4)
   bbccd:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bbcd5:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bbcdd:	lea    0x1(%r8),%r13d
   bbce1:	and    $0xf,%r13d
   bbce5:	mov    $0x3,%eax
   bbcea:	mov    $0x2,%r12d
   bbcf0:	mov    %r14,%r10
   bbcf3:	jmp    bbb2d <meshoptimizer_rs::codec::raw+0x5fd>
   bbcf8:	movzbl %r15b,%ecx
   bbcfc:	xor    $0x3,%r15b
   bbd00:	movzbl %r15b,%ebx
   bbd04:	mov    0x20(%rsp),%ebp
   bbd08:	add    %ecx,%ebp
   bbd0a:	sub    %ebx,%ebp
   bbd0c:	mov    %ebp,%ebx
   bbd0e:	jmp    bbf0d <meshoptimizer_rs::codec::raw+0x9dd>
   bbd13:	lea    0x1(%r15),%ebx
   bbd17:	mov    %r14d,%r13d
   bbd1a:	and    $0xf,%r13b
   bbd1e:	je     bbd36 <meshoptimizer_rs::codec::raw+0x806>
   bbd20:	mov    %r10d,%ecx
   bbd23:	sub    %r14d,%ecx
   bbd26:	and    $0xf,%ecx
   bbd29:	mov    %ebx,%ebp
   bbd2b:	mov    0xf0(%rsp,%rcx,4),%ecx
   bbd32:	mov    %ecx,%ebx
   bbd34:	jmp    bbd39 <meshoptimizer_rs::codec::raw+0x809>
   bbd36:	lea    0x1(%rbx),%ebp
   bbd39:	lea    0x1(%rsi),%r14
   bbd3d:	cmp    $0xfe,%eax
   bbd42:	je     bbe40 <meshoptimizer_rs::codec::raw+0x910>
   bbd48:	mov    0x30(%rsp),%r9
   bbd4d:	movzbl 0x1(%r9,%rsi,1),%eax
   bbd53:	test   %al,%al
   bbd55:	js     bbd86 <meshoptimizer_rs::codec::raw+0x856>
   bbd57:	add    $0x2,%rsi
   bbd5b:	mov    %rsi,%r14
   bbd5e:	mov    %eax,%r11d
   bbd61:	jmp    bbe22 <meshoptimizer_rs::codec::raw+0x8f2>
   bbd66:	and    $0x7f,%ebp
   bbd69:	movzbl 0x1(%r14,%rsi,1),%ebx
   bbd6f:	mov    %ebx,%ecx
   bbd71:	and    $0x7f,%ebx
   bbd74:	shl    $0x7,%ebx
   bbd77:	or     %ebp,%ebx
   bbd79:	test   %cl,%cl
   bbd7b:	js     bbda7 <meshoptimizer_rs::codec::raw+0x877>
   bbd7d:	add    $0x2,%rsi
   bbd81:	jmp    bbefc <meshoptimizer_rs::codec::raw+0x9cc>
   bbd86:	and    $0x7f,%eax
   bbd89:	movzbl 0x2(%r9,%rsi,1),%r11d
   bbd8f:	mov    %r11d,%ecx
   bbd92:	and    $0x7f,%r11d
   bbd96:	shl    $0x7,%r11d
   bbd9a:	or     %eax,%r11d
   bbd9d:	test   %cl,%cl
   bbd9f:	js     bbdc5 <meshoptimizer_rs::codec::raw+0x895>
   bbda1:	add    $0x3,%rsi
   bbda5:	jmp    bbe1f <meshoptimizer_rs::codec::raw+0x8ef>
   bbda7:	movzbl 0x2(%r14,%rsi,1),%ecx
   bbdad:	mov    %ecx,%ebp
   bbdaf:	and    $0x7f,%ecx
   bbdb2:	shl    $0xe,%ecx
   bbdb5:	or     %ecx,%ebx
   bbdb7:	test   %bpl,%bpl
   bbdba:	js     bbde0 <meshoptimizer_rs::codec::raw+0x8b0>
   bbdbc:	add    $0x3,%rsi
   bbdc0:	jmp    bbefc <meshoptimizer_rs::codec::raw+0x9cc>
   bbdc5:	movzbl 0x3(%r9,%rsi,1),%eax
   bbdcb:	mov    %eax,%ecx
   bbdcd:	and    $0x7f,%eax
   bbdd0:	shl    $0xe,%eax
   bbdd3:	or     %eax,%r11d
   bbdd6:	test   %cl,%cl
   bbdd8:	js     bbe02 <meshoptimizer_rs::codec::raw+0x8d2>
   bbdda:	add    $0x4,%rsi
   bbdde:	jmp    bbe1f <meshoptimizer_rs::codec::raw+0x8ef>
   bbde0:	movzbl 0x3(%r14,%rsi,1),%ecx
   bbde6:	mov    %ecx,%ebp
   bbde8:	and    $0x7f,%ecx
   bbdeb:	shl    $0x15,%ecx
   bbdee:	or     %ecx,%ebx
   bbdf0:	test   %bpl,%bpl
   bbdf3:	js     bbeeb <meshoptimizer_rs::codec::raw+0x9bb>
   bbdf9:	add    $0x4,%rsi
   bbdfd:	jmp    bbefc <meshoptimizer_rs::codec::raw+0x9cc>
   bbe02:	movzbl 0x4(%r9,%rsi,1),%eax
   bbe08:	mov    %eax,%ecx
   bbe0a:	and    $0x7f,%eax
   bbe0d:	shl    $0x15,%eax
   bbe10:	or     %eax,%r11d
   bbe13:	test   %cl,%cl
   bbe15:	js     bbf24 <meshoptimizer_rs::codec::raw+0x9f4>
   bbe1b:	add    $0x5,%rsi
   bbe1f:	mov    %rsi,%r14
   bbe22:	mov    0x28(%rsp),%r9
   bbe27:	mov    %r11d,%eax
   bbe2a:	shr    $1,%eax
   bbe2c:	and    $0x1,%r11d
   bbe30:	neg    %r11d
   bbe33:	xor    %eax,%r11d
   bbe36:	add    0x20(%rsp),%r11d
   bbe3b:	mov    %r11d,0x20(%rsp)
   bbe40:	cmp    $0xf,%r12b
   bbe44:	jne    bbf73 <meshoptimizer_rs::codec::raw+0xa43>
   bbe4a:	cmp    $0xfffffffffffffffa,%r14
   bbe4e:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bbe54:	lea    0x5(%r14),%rax
   bbe58:	cmp    0x630(%rsp),%rax
   bbe60:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bbe66:	mov    0x30(%rsp),%r9
   bbe6b:	movzbl (%r9,%r14,1),%esi
   bbe70:	test   %sil,%sil
   bbe73:	js     bbe83 <meshoptimizer_rs::codec::raw+0x953>
   bbe75:	inc    %r14
   bbe78:	mov    %r14,%rax
   bbe7b:	mov    %esi,%r15d
   bbe7e:	jmp    bbf52 <meshoptimizer_rs::codec::raw+0xa22>
   bbe83:	and    $0x7f,%esi
   bbe86:	movzbl 0x1(%r9,%r14,1),%r15d
   bbe8c:	mov    %r15d,%ecx
   bbe8f:	and    $0x7f,%r15d
   bbe93:	shl    $0x7,%r15d
   bbe97:	or     %esi,%r15d
   bbe9a:	test   %cl,%cl
   bbe9c:	js     bbeaa <meshoptimizer_rs::codec::raw+0x97a>
   bbe9e:	add    $0x2,%r14
   bbea2:	mov    %r14,%rax
   bbea5:	jmp    bbf52 <meshoptimizer_rs::codec::raw+0xa22>
   bbeaa:	movzbl 0x2(%r9,%r14,1),%ecx
   bbeb0:	mov    %ecx,%esi
   bbeb2:	and    $0x7f,%ecx
   bbeb5:	shl    $0xe,%ecx
   bbeb8:	or     %ecx,%r15d
   bbebb:	test   %sil,%sil
   bbebe:	js     bbecc <meshoptimizer_rs::codec::raw+0x99c>
   bbec0:	add    $0x3,%r14
   bbec4:	mov    %r14,%rax
   bbec7:	jmp    bbf52 <meshoptimizer_rs::codec::raw+0xa22>
   bbecc:	movzbl 0x3(%r9,%r14,1),%ecx
   bbed2:	mov    %ecx,%esi
   bbed4:	and    $0x7f,%ecx
   bbed7:	shl    $0x15,%ecx
   bbeda:	or     %ecx,%r15d
   bbedd:	test   %sil,%sil
   bbee0:	js     bbf3e <meshoptimizer_rs::codec::raw+0xa0e>
   bbee2:	add    $0x4,%r14
   bbee6:	mov    %r14,%rax
   bbee9:	jmp    bbf52 <meshoptimizer_rs::codec::raw+0xa22>
   bbeeb:	movzbl 0x4(%r14,%rsi,1),%ecx
   bbef1:	add    $0x5,%rsi
   bbef5:	shl    $0x1c,%ecx
   bbef8:	or     %ebx,%ecx
   bbefa:	mov    %ecx,%ebx
   bbefc:	mov    0x20(%rsp),%ebp
   bbf00:	mov    %ebx,%ecx
   bbf02:	shr    $1,%ecx
   bbf04:	and    $0x1,%ebx
   bbf07:	neg    %ebx
   bbf09:	xor    %ecx,%ebx
   bbf0b:	add    %ebp,%ebx
   bbf0d:	mov    %ebx,0xf0(%rsp,%r10,4)
   bbf15:	mov    $0x1,%r14d
   bbf1b:	mov    %ebx,0x20(%rsp)
   bbf1f:	jmp    bbafe <meshoptimizer_rs::codec::raw+0x5ce>
   bbf24:	add    $0x6,%rsi
   bbf28:	mov    0x30(%rsp),%rax
   bbf2d:	movzbl 0x4(%rax,%r14,1),%eax
   bbf33:	shl    $0x1c,%eax
   bbf36:	or     %r11d,%eax
   bbf39:	jmp    bbd5b <meshoptimizer_rs::codec::raw+0x82b>
   bbf3e:	mov    0x30(%rsp),%rcx
   bbf43:	movzbl 0x4(%rcx,%r14,1),%ecx
   bbf49:	shl    $0x1c,%ecx
   bbf4c:	or     %r15d,%ecx
   bbf4f:	mov    %ecx,%r15d
   bbf52:	mov    0x28(%rsp),%r9
   bbf57:	mov    %r15d,%ecx
   bbf5a:	shr    $1,%ecx
   bbf5c:	and    $0x1,%r15d
   bbf60:	neg    %r15d
   bbf63:	xor    %ecx,%r15d
   bbf66:	add    0x20(%rsp),%r15d
   bbf6b:	mov    %rax,%r14
   bbf6e:	mov    %r15d,0x20(%rsp)
   bbf73:	cmp    $0xf,%r13b
   bbf77:	jne    bc03b <meshoptimizer_rs::codec::raw+0xb0b>
   bbf7d:	cmp    $0xfffffffffffffffa,%r14
   bbf81:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bbf87:	lea    0x5(%r14),%rax
   bbf8b:	cmp    0x630(%rsp),%rax
   bbf93:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bbf99:	mov    0x30(%rsp),%r9
   bbf9e:	movzbl (%r9,%r14,1),%esi
   bbfa3:	test   %sil,%sil
   bbfa6:	js     bbfb2 <meshoptimizer_rs::codec::raw+0xa82>
   bbfa8:	inc    %r14
   bbfab:	mov    %r14,%rax
   bbfae:	mov    %esi,%ebx
   bbfb0:	jmp    bc020 <meshoptimizer_rs::codec::raw+0xaf0>
   bbfb2:	and    $0x7f,%esi
   bbfb5:	movzbl 0x1(%r9,%r14,1),%ebx
   bbfbb:	mov    %ebx,%ecx
   bbfbd:	and    $0x7f,%ebx
   bbfc0:	shl    $0x7,%ebx
   bbfc3:	or     %esi,%ebx
   bbfc5:	test   %cl,%cl
   bbfc7:	js     bbfd2 <meshoptimizer_rs::codec::raw+0xaa2>
   bbfc9:	add    $0x2,%r14
   bbfcd:	mov    %r14,%rax
   bbfd0:	jmp    bc020 <meshoptimizer_rs::codec::raw+0xaf0>
   bbfd2:	movzbl 0x2(%r9,%r14,1),%ecx
   bbfd8:	mov    %ecx,%esi
   bbfda:	and    $0x7f,%ecx
   bbfdd:	shl    $0xe,%ecx
   bbfe0:	or     %ecx,%ebx
   bbfe2:	test   %sil,%sil
   bbfe5:	js     bbff0 <meshoptimizer_rs::codec::raw+0xac0>
   bbfe7:	add    $0x3,%r14
   bbfeb:	mov    %r14,%rax
   bbfee:	jmp    bc020 <meshoptimizer_rs::codec::raw+0xaf0>
   bbff0:	movzbl 0x3(%r9,%r14,1),%ecx
   bbff6:	mov    %ecx,%esi
   bbff8:	and    $0x7f,%ecx
   bbffb:	shl    $0x15,%ecx
   bbffe:	or     %ecx,%ebx
   bc000:	test   %sil,%sil
   bc003:	js     bc00e <meshoptimizer_rs::codec::raw+0xade>
   bc005:	add    $0x4,%r14
   bc009:	mov    %r14,%rax
   bc00c:	jmp    bc020 <meshoptimizer_rs::codec::raw+0xaf0>
   bc00e:	mov    0x30(%rsp),%rcx
   bc013:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc019:	shl    $0x1c,%ecx
   bc01c:	or     %ebx,%ecx
   bc01e:	mov    %ecx,%ebx
   bc020:	mov    0x28(%rsp),%r9
   bc025:	mov    %ebx,%ecx
   bc027:	shr    $1,%ecx
   bc029:	and    $0x1,%ebx
   bc02c:	neg    %ebx
   bc02e:	xor    %ecx,%ebx
   bc030:	add    0x20(%rsp),%ebx
   bc034:	mov    %rax,%r14
   bc037:	mov    %ebx,0x20(%rsp)
   bc03b:	cmp    $0xf,%r13b
   bc03f:	sete   %al
   bc042:	cmp    $0xf,%r12b
   bc046:	sete   %cl
   bc049:	test   %r13b,%r13b
   bc04c:	sete   %sil
   bc050:	test   %r12b,%r12b
   bc053:	sete   %r12b
   bc057:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc05f:	lea    0x1(%r10),%r13
   bc063:	and    $0xf,%r13d
   bc067:	or     %cl,%r12b
   bc06a:	mov    %r15d,0xf0(%rsp,%r13,4)
   bc072:	movzbl %r12b,%ecx
   bc076:	add    %r10,%rcx
   bc079:	inc    %rcx
   bc07c:	or     %al,%sil
   bc07f:	movzbl %sil,%r10d
   bc083:	add    %rcx,%r10
   bc086:	and    $0xf,%ecx
   bc089:	mov    %ebx,0xf0(%rsp,%rcx,4)
   bc090:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc098:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc0a0:	lea    0x1(%r8),%r13d
   bc0a4:	and    $0xf,%r13d
   bc0a8:	mov    $0x3,%eax
   bc0ad:	mov    $0x2,%r12d
   bc0b3:	mov    %r14,%rsi
   bc0b6:	jmp    bbb2d <meshoptimizer_rs::codec::raw+0x5fd>
   bc0bb:	mov    %bpl,%r14b
   bc0be:	add    %r14d,%r11d
   bc0c1:	mov    %ebx,0xf0(%rsp,%r10,4)
   bc0c9:	mov    %r11d,%ebp
   bc0cc:	not    %eax
   bc0ce:	shr    $0x4,%eax
   bc0d1:	add    %r8d,%eax
   bc0d4:	and    $0xf,%eax
   bc0d7:	mov    0x1f0(%rsp,%rax,8),%r11d
   bc0df:	mov    0x1f4(%rsp,%rax,8),%r15d
   bc0e7:	add    %r14,%r10
   bc0ea:	mov    $0x2,%eax
   bc0ef:	mov    $0x1,%r12d
   bc0f5:	mov    %r8,%r13
   bc0f8:	inc    %rdi
   bc0fb:	mov    %ebx,0x1f0(%rsp,%r13,8)
   bc103:	mov    %r15d,0x1f4(%rsp,%r13,8)
   bc10b:	add    %r8d,%r12d
   bc10e:	and    $0xf,%r12d
   bc112:	mov    %r11d,0x1f0(%rsp,%r12,8)
   bc11a:	mov    %ebx,0x1f4(%rsp,%r12,8)
   bc122:	add    %r8d,%eax
   bc125:	and    $0xf,%eax
   bc128:	and    $0xf,%r10d
   bc12c:	mov    %r11d,-0x8(%rdx)
   bc130:	mov    %r15d,-0x4(%rdx)
   bc134:	mov    %rdx,%r15
   bc137:	mov    %ebx,(%rdx)
   bc139:	add    $0xc,%r15
   bc13d:	mov    %rax,%r8
   bc140:	mov    %ebp,%r11d
   bc143:	cmp    %rdi,0x38(%rsp)
   bc148:	je     bd084 <meshoptimizer_rs::codec::raw+0x1b54>
   bc14e:	cmp    %r9,%rsi
   bc151:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bc157:	mov    %r15,%rdx
   bc15a:	mov    0x30(%rsp),%r14
   bc15f:	movzbl 0x1(%r14,%rdi,1),%eax
   bc165:	cmp    $0xf0,%al
   bc167:	jae    bc1a1 <meshoptimizer_rs::codec::raw+0xc71>
   bc169:	mov    %eax,%r15d
   bc16c:	and    $0xf,%r15d
   bc170:	cmp    %r15d,0x58(%rsp)
   bc175:	jbe    bc1db <meshoptimizer_rs::codec::raw+0xcab>
   bc177:	xor    %r14d,%r14d
   bc17a:	mov    %r11d,%ebx
   bc17d:	test   %r15b,%r15b
   bc180:	sete   %bpl
   bc184:	je     bc0bb <meshoptimizer_rs::codec::raw+0xb8b>
   bc18a:	not    %r15d
   bc18d:	add    %r10d,%r15d
   bc190:	and    $0xf,%r15d
   bc194:	mov    0xf0(%rsp,%r15,4),%ebx
   bc19c:	jmp    bc0bb <meshoptimizer_rs::codec::raw+0xb8b>
   bc1a1:	cmp    $0xfe,%al
   bc1a3:	jae    bc1fd <meshoptimizer_rs::codec::raw+0xccd>
   bc1a5:	and    $0xf,%eax
   bc1a8:	mov    0x50(%rsp),%rcx
   bc1ad:	movzbl (%rcx,%rax,1),%r13d
   bc1b2:	lea    0x1(%r11),%r15d
   bc1b6:	xor    %eax,%eax
   bc1b8:	mov    %r13d,%ecx
   bc1bb:	shr    $0x4,%ecx
   bc1be:	sete   %r12b
   bc1c2:	je     bc240 <meshoptimizer_rs::codec::raw+0xd10>
   bc1c4:	mov    %r10d,%ebx
   bc1c7:	sub    %ecx,%ebx
   bc1c9:	and    $0xf,%ebx
   bc1cc:	mov    %r15d,%ebp
   bc1cf:	mov    0xf0(%rsp,%rbx,4),%ecx
   bc1d6:	mov    %ecx,%r15d
   bc1d9:	jmp    bc244 <meshoptimizer_rs::codec::raw+0xd14>
   bc1db:	cmp    $0xf,%r15b
   bc1df:	jne    bc2c0 <meshoptimizer_rs::codec::raw+0xd90>
   bc1e5:	movzbl (%r14,%rsi,1),%ebp
   bc1ea:	test   %bpl,%bpl
   bc1ed:	js     bc32e <meshoptimizer_rs::codec::raw+0xdfe>
   bc1f3:	inc    %rsi
   bc1f6:	mov    %ebp,%ebx
   bc1f8:	jmp    bc4c4 <meshoptimizer_rs::codec::raw+0xf94>
   bc1fd:	movzbl (%r14,%rsi,1),%r14d
   bc202:	xor    %r15d,%r15d
   bc205:	cmp    $0xfe,%eax
   bc20a:	sete   %r15b
   bc20e:	test   %r14d,%r14d
   bc211:	cmove  %r14d,%r11d
   bc215:	add    %r11d,%r15d
   bc218:	mov    %r14d,%r12d
   bc21b:	shr    $0x4,%r12d
   bc21f:	je     bc2db <meshoptimizer_rs::codec::raw+0xdab>
   bc225:	mov    %r10d,%ecx
   bc228:	sub    %r12d,%ecx
   bc22b:	and    $0xf,%ecx
   bc22e:	mov    %r15d,%ebx
   bc231:	mov    0xf0(%rsp,%rcx,4),%ecx
   bc238:	mov    %ecx,%r15d
   bc23b:	jmp    bc2df <meshoptimizer_rs::codec::raw+0xdaf>
   bc240:	lea    0x2(%r11),%ebp
   bc244:	xor    %r14d,%r14d
   bc247:	mov    %ebp,%ebx
   bc249:	test   $0xf,%r13b
   bc24d:	sete   %cl
   bc250:	je     bc262 <meshoptimizer_rs::codec::raw+0xd32>
   bc252:	mov    %r10d,%ebx
   bc255:	sub    %r13d,%ebx
   bc258:	and    $0xf,%ebx
   bc25b:	mov    0xf0(%rsp,%rbx,4),%ebx
   bc262:	mov    %cl,%r14b
   bc265:	add    %r14d,%ebp
   bc268:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc270:	lea    0x1(%r10),%rcx
   bc274:	and    $0xf,%ecx
   bc277:	mov    %r15d,0xf0(%rsp,%rcx,4)
   bc27f:	mov    %r12b,%al
   bc282:	add    %r10,%rax
   bc285:	inc    %rax
   bc288:	add    %rax,%r14
   bc28b:	and    $0xf,%eax
   bc28e:	mov    %ebx,0xf0(%rsp,%rax,4)
   bc295:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc29d:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc2a5:	lea    0x1(%r8),%r13d
   bc2a9:	and    $0xf,%r13d
   bc2ad:	mov    $0x3,%eax
   bc2b2:	mov    $0x2,%r12d
   bc2b8:	mov    %r14,%r10
   bc2bb:	jmp    bc0f8 <meshoptimizer_rs::codec::raw+0xbc8>
   bc2c0:	movzbl %r15b,%ecx
   bc2c4:	xor    $0x3,%r15b
   bc2c8:	movzbl %r15b,%ebx
   bc2cc:	mov    0x20(%rsp),%ebp
   bc2d0:	add    %ecx,%ebp
   bc2d2:	sub    %ebx,%ebp
   bc2d4:	mov    %ebp,%ebx
   bc2d6:	jmp    bc4d5 <meshoptimizer_rs::codec::raw+0xfa5>
   bc2db:	lea    0x1(%r15),%ebx
   bc2df:	mov    %r14d,%r13d
   bc2e2:	and    $0xf,%r13b
   bc2e6:	je     bc2fe <meshoptimizer_rs::codec::raw+0xdce>
   bc2e8:	mov    %r10d,%ecx
   bc2eb:	sub    %r14d,%ecx
   bc2ee:	and    $0xf,%ecx
   bc2f1:	mov    %ebx,%ebp
   bc2f3:	mov    0xf0(%rsp,%rcx,4),%ecx
   bc2fa:	mov    %ecx,%ebx
   bc2fc:	jmp    bc301 <meshoptimizer_rs::codec::raw+0xdd1>
   bc2fe:	lea    0x1(%rbx),%ebp
   bc301:	lea    0x1(%rsi),%r14
   bc305:	cmp    $0xfe,%eax
   bc30a:	je     bc408 <meshoptimizer_rs::codec::raw+0xed8>
   bc310:	mov    0x30(%rsp),%r9
   bc315:	movzbl 0x1(%r9,%rsi,1),%eax
   bc31b:	test   %al,%al
   bc31d:	js     bc34e <meshoptimizer_rs::codec::raw+0xe1e>
   bc31f:	add    $0x2,%rsi
   bc323:	mov    %rsi,%r14
   bc326:	mov    %eax,%r11d
   bc329:	jmp    bc3ea <meshoptimizer_rs::codec::raw+0xeba>
   bc32e:	and    $0x7f,%ebp
   bc331:	movzbl 0x1(%r14,%rsi,1),%ebx
   bc337:	mov    %ebx,%ecx
   bc339:	and    $0x7f,%ebx
   bc33c:	shl    $0x7,%ebx
   bc33f:	or     %ebp,%ebx
   bc341:	test   %cl,%cl
   bc343:	js     bc36f <meshoptimizer_rs::codec::raw+0xe3f>
   bc345:	add    $0x2,%rsi
   bc349:	jmp    bc4c4 <meshoptimizer_rs::codec::raw+0xf94>
   bc34e:	and    $0x7f,%eax
   bc351:	movzbl 0x2(%r9,%rsi,1),%r11d
   bc357:	mov    %r11d,%ecx
   bc35a:	and    $0x7f,%r11d
   bc35e:	shl    $0x7,%r11d
   bc362:	or     %eax,%r11d
   bc365:	test   %cl,%cl
   bc367:	js     bc38d <meshoptimizer_rs::codec::raw+0xe5d>
   bc369:	add    $0x3,%rsi
   bc36d:	jmp    bc3e7 <meshoptimizer_rs::codec::raw+0xeb7>
   bc36f:	movzbl 0x2(%r14,%rsi,1),%ecx
   bc375:	mov    %ecx,%ebp
   bc377:	and    $0x7f,%ecx
   bc37a:	shl    $0xe,%ecx
   bc37d:	or     %ecx,%ebx
   bc37f:	test   %bpl,%bpl
   bc382:	js     bc3a8 <meshoptimizer_rs::codec::raw+0xe78>
   bc384:	add    $0x3,%rsi
   bc388:	jmp    bc4c4 <meshoptimizer_rs::codec::raw+0xf94>
   bc38d:	movzbl 0x3(%r9,%rsi,1),%eax
   bc393:	mov    %eax,%ecx
   bc395:	and    $0x7f,%eax
   bc398:	shl    $0xe,%eax
   bc39b:	or     %eax,%r11d
   bc39e:	test   %cl,%cl
   bc3a0:	js     bc3ca <meshoptimizer_rs::codec::raw+0xe9a>
   bc3a2:	add    $0x4,%rsi
   bc3a6:	jmp    bc3e7 <meshoptimizer_rs::codec::raw+0xeb7>
   bc3a8:	movzbl 0x3(%r14,%rsi,1),%ecx
   bc3ae:	mov    %ecx,%ebp
   bc3b0:	and    $0x7f,%ecx
   bc3b3:	shl    $0x15,%ecx
   bc3b6:	or     %ecx,%ebx
   bc3b8:	test   %bpl,%bpl
   bc3bb:	js     bc4b3 <meshoptimizer_rs::codec::raw+0xf83>
   bc3c1:	add    $0x4,%rsi
   bc3c5:	jmp    bc4c4 <meshoptimizer_rs::codec::raw+0xf94>
   bc3ca:	movzbl 0x4(%r9,%rsi,1),%eax
   bc3d0:	mov    %eax,%ecx
   bc3d2:	and    $0x7f,%eax
   bc3d5:	shl    $0x15,%eax
   bc3d8:	or     %eax,%r11d
   bc3db:	test   %cl,%cl
   bc3dd:	js     bc4ec <meshoptimizer_rs::codec::raw+0xfbc>
   bc3e3:	add    $0x5,%rsi
   bc3e7:	mov    %rsi,%r14
   bc3ea:	mov    0x28(%rsp),%r9
   bc3ef:	mov    %r11d,%eax
   bc3f2:	shr    $1,%eax
   bc3f4:	and    $0x1,%r11d
   bc3f8:	neg    %r11d
   bc3fb:	xor    %eax,%r11d
   bc3fe:	add    0x20(%rsp),%r11d
   bc403:	mov    %r11d,0x20(%rsp)
   bc408:	cmp    $0xf,%r12b
   bc40c:	jne    bc53b <meshoptimizer_rs::codec::raw+0x100b>
   bc412:	cmp    $0xfffffffffffffffa,%r14
   bc416:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bc41c:	lea    0x5(%r14),%rax
   bc420:	cmp    0x630(%rsp),%rax
   bc428:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bc42e:	mov    0x30(%rsp),%r9
   bc433:	movzbl (%r9,%r14,1),%esi
   bc438:	test   %sil,%sil
   bc43b:	js     bc44b <meshoptimizer_rs::codec::raw+0xf1b>
   bc43d:	inc    %r14
   bc440:	mov    %r14,%rax
   bc443:	mov    %esi,%r15d
   bc446:	jmp    bc51a <meshoptimizer_rs::codec::raw+0xfea>
   bc44b:	and    $0x7f,%esi
   bc44e:	movzbl 0x1(%r9,%r14,1),%r15d
   bc454:	mov    %r15d,%ecx
   bc457:	and    $0x7f,%r15d
   bc45b:	shl    $0x7,%r15d
   bc45f:	or     %esi,%r15d
   bc462:	test   %cl,%cl
   bc464:	js     bc472 <meshoptimizer_rs::codec::raw+0xf42>
   bc466:	add    $0x2,%r14
   bc46a:	mov    %r14,%rax
   bc46d:	jmp    bc51a <meshoptimizer_rs::codec::raw+0xfea>
   bc472:	movzbl 0x2(%r9,%r14,1),%ecx
   bc478:	mov    %ecx,%esi
   bc47a:	and    $0x7f,%ecx
   bc47d:	shl    $0xe,%ecx
   bc480:	or     %ecx,%r15d
   bc483:	test   %sil,%sil
   bc486:	js     bc494 <meshoptimizer_rs::codec::raw+0xf64>
   bc488:	add    $0x3,%r14
   bc48c:	mov    %r14,%rax
   bc48f:	jmp    bc51a <meshoptimizer_rs::codec::raw+0xfea>
   bc494:	movzbl 0x3(%r9,%r14,1),%ecx
   bc49a:	mov    %ecx,%esi
   bc49c:	and    $0x7f,%ecx
   bc49f:	shl    $0x15,%ecx
   bc4a2:	or     %ecx,%r15d
   bc4a5:	test   %sil,%sil
   bc4a8:	js     bc506 <meshoptimizer_rs::codec::raw+0xfd6>
   bc4aa:	add    $0x4,%r14
   bc4ae:	mov    %r14,%rax
   bc4b1:	jmp    bc51a <meshoptimizer_rs::codec::raw+0xfea>
   bc4b3:	movzbl 0x4(%r14,%rsi,1),%ecx
   bc4b9:	add    $0x5,%rsi
   bc4bd:	shl    $0x1c,%ecx
   bc4c0:	or     %ebx,%ecx
   bc4c2:	mov    %ecx,%ebx
   bc4c4:	mov    0x20(%rsp),%ebp
   bc4c8:	mov    %ebx,%ecx
   bc4ca:	shr    $1,%ecx
   bc4cc:	and    $0x1,%ebx
   bc4cf:	neg    %ebx
   bc4d1:	xor    %ecx,%ebx
   bc4d3:	add    %ebp,%ebx
   bc4d5:	mov    %ebx,0xf0(%rsp,%r10,4)
   bc4dd:	mov    $0x1,%r14d
   bc4e3:	mov    %ebx,0x20(%rsp)
   bc4e7:	jmp    bc0c9 <meshoptimizer_rs::codec::raw+0xb99>
   bc4ec:	add    $0x6,%rsi
   bc4f0:	mov    0x30(%rsp),%rax
   bc4f5:	movzbl 0x4(%rax,%r14,1),%eax
   bc4fb:	shl    $0x1c,%eax
   bc4fe:	or     %r11d,%eax
   bc501:	jmp    bc323 <meshoptimizer_rs::codec::raw+0xdf3>
   bc506:	mov    0x30(%rsp),%rcx
   bc50b:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc511:	shl    $0x1c,%ecx
   bc514:	or     %r15d,%ecx
   bc517:	mov    %ecx,%r15d
   bc51a:	mov    0x28(%rsp),%r9
   bc51f:	mov    %r15d,%ecx
   bc522:	shr    $1,%ecx
   bc524:	and    $0x1,%r15d
   bc528:	neg    %r15d
   bc52b:	xor    %ecx,%r15d
   bc52e:	add    0x20(%rsp),%r15d
   bc533:	mov    %rax,%r14
   bc536:	mov    %r15d,0x20(%rsp)
   bc53b:	cmp    $0xf,%r13b
   bc53f:	jne    bc603 <meshoptimizer_rs::codec::raw+0x10d3>
   bc545:	cmp    $0xfffffffffffffffa,%r14
   bc549:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bc54f:	lea    0x5(%r14),%rax
   bc553:	cmp    0x630(%rsp),%rax
   bc55b:	ja     bd08b <meshoptimizer_rs::codec::raw+0x1b5b>
   bc561:	mov    0x30(%rsp),%r9
   bc566:	movzbl (%r9,%r14,1),%esi
   bc56b:	test   %sil,%sil
   bc56e:	js     bc57a <meshoptimizer_rs::codec::raw+0x104a>
   bc570:	inc    %r14
   bc573:	mov    %r14,%rax
   bc576:	mov    %esi,%ebx
   bc578:	jmp    bc5e8 <meshoptimizer_rs::codec::raw+0x10b8>
   bc57a:	and    $0x7f,%esi
   bc57d:	movzbl 0x1(%r9,%r14,1),%ebx
   bc583:	mov    %ebx,%ecx
   bc585:	and    $0x7f,%ebx
   bc588:	shl    $0x7,%ebx
   bc58b:	or     %esi,%ebx
   bc58d:	test   %cl,%cl
   bc58f:	js     bc59a <meshoptimizer_rs::codec::raw+0x106a>
   bc591:	add    $0x2,%r14
   bc595:	mov    %r14,%rax
   bc598:	jmp    bc5e8 <meshoptimizer_rs::codec::raw+0x10b8>
   bc59a:	movzbl 0x2(%r9,%r14,1),%ecx
   bc5a0:	mov    %ecx,%esi
   bc5a2:	and    $0x7f,%ecx
   bc5a5:	shl    $0xe,%ecx
   bc5a8:	or     %ecx,%ebx
   bc5aa:	test   %sil,%sil
   bc5ad:	js     bc5b8 <meshoptimizer_rs::codec::raw+0x1088>
   bc5af:	add    $0x3,%r14
   bc5b3:	mov    %r14,%rax
   bc5b6:	jmp    bc5e8 <meshoptimizer_rs::codec::raw+0x10b8>
   bc5b8:	movzbl 0x3(%r9,%r14,1),%ecx
   bc5be:	mov    %ecx,%esi
   bc5c0:	and    $0x7f,%ecx
   bc5c3:	shl    $0x15,%ecx
   bc5c6:	or     %ecx,%ebx
   bc5c8:	test   %sil,%sil
   bc5cb:	js     bc5d6 <meshoptimizer_rs::codec::raw+0x10a6>
   bc5cd:	add    $0x4,%r14
   bc5d1:	mov    %r14,%rax
   bc5d4:	jmp    bc5e8 <meshoptimizer_rs::codec::raw+0x10b8>
   bc5d6:	mov    0x30(%rsp),%rcx
   bc5db:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc5e1:	shl    $0x1c,%ecx
   bc5e4:	or     %ebx,%ecx
   bc5e6:	mov    %ecx,%ebx
   bc5e8:	mov    0x28(%rsp),%r9
   bc5ed:	mov    %ebx,%ecx
   bc5ef:	shr    $1,%ecx
   bc5f1:	and    $0x1,%ebx
   bc5f4:	neg    %ebx
   bc5f6:	xor    %ecx,%ebx
   bc5f8:	add    0x20(%rsp),%ebx
   bc5fc:	mov    %rax,%r14
   bc5ff:	mov    %ebx,0x20(%rsp)
   bc603:	cmp    $0xf,%r13b
   bc607:	sete   %al
   bc60a:	cmp    $0xf,%r12b
   bc60e:	sete   %cl
   bc611:	test   %r13b,%r13b
   bc614:	sete   %sil
   bc618:	test   %r12b,%r12b
   bc61b:	sete   %r12b
   bc61f:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc627:	lea    0x1(%r10),%r13
   bc62b:	and    $0xf,%r13d
   bc62f:	or     %cl,%r12b
   bc632:	mov    %r15d,0xf0(%rsp,%r13,4)
   bc63a:	movzbl %r12b,%ecx
   bc63e:	add    %r10,%rcx
   bc641:	inc    %rcx
   bc644:	or     %al,%sil
   bc647:	movzbl %sil,%r10d
   bc64b:	add    %rcx,%r10
   bc64e:	and    $0xf,%ecx
   bc651:	mov    %ebx,0xf0(%rsp,%rcx,4)
   bc658:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc660:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc668:	lea    0x1(%r8),%r13d
   bc66c:	and    $0xf,%r13d
   bc670:	mov    $0x3,%eax
   bc675:	mov    $0x2,%r12d
   bc67b:	mov    %r14,%rsi
   bc67e:	jmp    bc0f8 <meshoptimizer_rs::codec::raw+0xbc8>
   bc683:	cmp    $0x21,%rbp
   bc687:	mov    $0x20,%eax
   bc68c:	cmovae %rbp,%rax
   bc690:	mov    %rbp,%rcx
   bc693:	mov    %rax,0xe0(%rsp)
   bc69b:	lea    0x1(%rax),%rdx
   bc69f:	mov    $0x7,%al
   bc6a1:	cmp    %rdx,%rbx
   bc6a4:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bc6aa:	mov    %rbx,%rsi
   bc6ad:	sub    %rcx,%rsi
   bc6b0:	lea    (%rsi,%rbp,1),%rax
   bc6b4:	mov    %rbx,%rcx
   bc6b7:	mov    %rax,0x60(%rsp)
   bc6bc:	sub    %rax,%rcx
   bc6bf:	mov    %rcx,0xa0(%rsp)
   bc6c7:	jb     bd10c <meshoptimizer_rs::codec::raw+0x1bdc>
   bc6cd:	pxor   %xmm0,%xmm0
   bc6d1:	movdqa %xmm0,0x1e0(%rsp)
   bc6da:	movdqa %xmm0,0x1d0(%rsp)
   bc6e3:	movdqa %xmm0,0x1c0(%rsp)
   bc6ec:	movdqa %xmm0,0x1b0(%rsp)
   bc6f5:	movdqa %xmm0,0x1a0(%rsp)
   bc6fe:	movdqa %xmm0,0x190(%rsp)
   bc707:	movdqa %xmm0,0x180(%rsp)
   bc710:	movdqa %xmm0,0x170(%rsp)
   bc719:	movdqa %xmm0,0x160(%rsp)
   bc722:	movdqa %xmm0,0x150(%rsp)
   bc72b:	movdqa %xmm0,0x140(%rsp)
   bc734:	movdqa %xmm0,0x130(%rsp)
   bc73d:	movdqa %xmm0,0x120(%rsp)
   bc746:	movdqa %xmm0,0x110(%rsp)
   bc74f:	movdqa %xmm0,0x100(%rsp)
   bc758:	movdqa %xmm0,0xf0(%rsp)
   bc761:	cmp    $0x101,%rbp
   bc768:	jae    bd0cd <meshoptimizer_rs::codec::raw+0x1b9d>
   bc76e:	mov    %dil,0x38(%rsp)
   bc773:	lea    (%rsi,%rbp,1),%rax
   bc777:	cmp    %rsi,%rax
   bc77a:	jb     bd0e4 <meshoptimizer_rs::codec::raw+0x1bb4>
   bc780:	add    %r14,%rsi
   bc783:	lea    0xf0(%rsp),%rdi
   bc78b:	mov    %rbp,%rdx
   bc78e:	call   *0x8fe64(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bc794:	test   %rbp,%rbp
   bc797:	je     bd175 <meshoptimizer_rs::codec::raw+0x1c45>
   bc79d:	mov    $0x2000,%ax
   bc7a1:	xor    %edx,%edx
   bc7a3:	div    %bp
   bc7a6:	and    $0x3ff0,%eax
   bc7ab:	cmp    $0x100,%eax
   bc7b0:	mov    $0x100,%ecx
   bc7b5:	cmovb  %eax,%ecx
   bc7b8:	mov    %rcx,0xd8(%rsp)
   bc7c0:	lea    0x1f0(%rsp),%rdi
   bc7c8:	mov    $0x400,%edx
   bc7cd:	xor    %esi,%esi
   bc7cf:	call   *0x8fe33(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   bc7d5:	test   %r13,%r13
   bc7d8:	je     bd060 <meshoptimizer_rs::codec::raw+0x1b30>
   bc7de:	mov    %r15,0xd0(%rsp)
   bc7e6:	add    %r14,0x60(%rsp)
   bc7eb:	mov    %rbp,%rax
   bc7ee:	shr    $0x2,%rax
   bc7f2:	mov    %rax,0x98(%rsp)
   bc7fa:	lea    0x3(%rbp),%rax
   bc7fe:	shr    $0x2,%rax
   bc802:	mov    %rax,0xe8(%rsp)
   bc80a:	mov    $0x1,%r15d
   bc810:	xor    %eax,%eax
   bc812:	mov    %r12,0x88(%rsp)
   bc81a:	mov    %r14,0x30(%rsp)
   bc81f:	mov    %r13,%rcx
   bc822:	mov    %rax,%rdi
   bc825:	sub    %rax,%rcx
   bc828:	mov    0xd8(%rsp),%rax
   bc830:	cmp    %rax,%rcx
   bc833:	cmovae %rax,%rcx
   bc837:	mov    %rcx,0x20(%rsp)
   bc83c:	movzbl 0x38(%rsp),%edx
   bc841:	test   %dl,%dl
   bc843:	je     bc889 <meshoptimizer_rs::codec::raw+0x1359>
   bc845:	mov    %r15,%rcx
   bc848:	mov    $0x7,%al
   bc84a:	add    0x98(%rsp),%rcx
   bc852:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bc858:	cmp    %rbx,%rcx
   bc85b:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bc861:	add    %r14,%r15
   bc864:	mov    0x98(%rsp),%rax
   bc86c:	mov    %rax,0x90(%rsp)
   bc874:	mov    %r15,0xa8(%rsp)
   bc87c:	mov    %rcx,%r15
   bc87f:	mov    0x88(%rsp),%r12
   bc887:	jmp    bc8a2 <meshoptimizer_rs::codec::raw+0x1372>
   bc889:	movq   $0x0,0x90(%rsp)
   bc895:	mov    $0x1,%eax
   bc89a:	mov    %rax,0xa8(%rsp)
   bc8a2:	mov    0x20(%rsp),%rax
   bc8a7:	lea    (%rax,%rdi,1),%rsi
   bc8ab:	imul   %rbp,%rdi
   bc8af:	mov    %rsi,0xc8(%rsp)
   bc8b7:	imul   %rbp,%rsi
   bc8bb:	cmp    %rdi,%rsi
   bc8be:	jb     bd0fc <meshoptimizer_rs::codec::raw+0x1bcc>
   bc8c4:	cmp    %r12,%rsi
   bc8c7:	ja     bd0fc <meshoptimizer_rs::codec::raw+0x1bcc>
   bc8cd:	mov    0x20(%rsp),%rax
   bc8d2:	lea    0xf(%rax),%ecx
   bc8d5:	and    $0x3f0,%ecx
   bc8db:	mov    %rcx,0x28(%rsp)
   bc8e0:	mov    %rax,%rcx
   bc8e3:	imul   %rbp,%rcx
   bc8e7:	mov    %rcx,0xb8(%rsp)
   bc8ef:	add    0xd0(%rsp),%rdi
   bc8f7:	mov    %rdi,0xb0(%rsp)
   bc8ff:	lea    -0x1(%rax),%rcx
   bc903:	imul   %rbp,%rcx
   bc907:	mov    %rcx,0xc0(%rsp)
   bc90f:	add    $0x4,%rcx
   bc913:	mov    %rcx,0x50(%rsp)
   bc918:	lea    0x0(,%rax,4),%rcx
   bc920:	mov    %rcx,0x68(%rsp)
   bc925:	lea    0x1f0(%rsp,%rax,1),%rcx
   bc92d:	mov    %rcx,0x80(%rsp)
   bc935:	lea    0x1f0(%rsp,%rax,2),%rcx
   bc93d:	mov    %rcx,0x78(%rsp)
   bc942:	lea    (%rax,%rax,2),%rax
   bc946:	add    %rsp,%rax
   bc949:	add    $0x1f0,%rax
   bc94f:	mov    %rax,0x70(%rsp)
   bc954:	xor    %r14d,%r14d
   bc957:	xor    %ebx,%ebx
   bc959:	jmp    bc9ba <meshoptimizer_rs::codec::raw+0x148a>
   bc95b:	lea    (%rsp,%rbx,1),%rcx
   bc95f:	add    $0xf0,%rcx
   bc966:	shr    $0x4,%al
   bc969:	neg    %al
   bc96b:	and    $0x1f,%al
   bc96d:	movzbl %al,%eax
   bc970:	mov    %rcx,(%rsp)
   bc974:	mov    %eax,0x8(%rsp)
   bc978:	movq   $0x0,0x10(%rsp)
   bc981:	lea    0x1f0(%rsp),%rdi
   bc989:	mov    0x68(%rsp),%rsi
   bc98e:	mov    0x50(%rsp),%rcx
   bc993:	mov    0x20(%rsp),%r8
   bc998:	mov    %rbp,%r9
   bc99b:	call   7d5f0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   bc9a0:	inc    %r14
   bc9a3:	add    $0x4,%rbx
   bc9a7:	cmp    %r14,0xe8(%rsp)
   bc9af:	movzbl 0x38(%rsp),%edx
   bc9b4:	je     bd008 <meshoptimizer_rs::codec::raw+0x1ad8>
   bc9ba:	mov    %rbx,0x58(%rsp)
   bc9bf:	test   %dl,%dl
   bc9c1:	je     bc9e0 <meshoptimizer_rs::codec::raw+0x14b0>
   bc9c3:	cmp    0x90(%rsp),%r14
   bc9cb:	jae    bd145 <meshoptimizer_rs::codec::raw+0x1c15>
   bc9d1:	mov    0xa8(%rsp),%rax
   bc9d9:	movzbl (%rax,%r14,1),%ebx
   bc9de:	jmp    bc9e2 <meshoptimizer_rs::codec::raw+0x14b2>
   bc9e0:	xor    %ebx,%ebx
   bc9e2:	mov    %ebx,%eax
   bc9e4:	and    $0x3,%al
   bc9e6:	movzbl %al,%ecx
   bc9e9:	mov    0x28(%rsp),%rsi
   bc9ee:	cmp    %rsi,0x20(%rsp)
   bc9f3:	ja     bce53 <meshoptimizer_rs::codec::raw+0x1923>
   bc9f9:	cmp    $0x2,%al
   bc9fb:	je     bca3d <meshoptimizer_rs::codec::raw+0x150d>
   bc9fd:	cmp    $0x3,%ecx
   bca00:	jne    bca57 <meshoptimizer_rs::codec::raw+0x1527>
   bca02:	mov    %r15,%r12
   bca05:	mov    $0x7,%al
   bca07:	add    0x20(%rsp),%r12
   bca0c:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bca12:	cmp    0x630(%rsp),%r12
   bca1a:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bca20:	add    0x30(%rsp),%r15
   bca25:	lea    0x1f0(%rsp),%rdi
   bca2d:	mov    %r15,%rsi
   bca30:	mov    0x20(%rsp),%rdx
   bca35:	call   *0x8fbbd(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bca3b:	jmp    bcab3 <meshoptimizer_rs::codec::raw+0x1583>
   bca3d:	lea    0x1f0(%rsp),%rdi
   bca45:	xor    %esi,%esi
   bca47:	mov    0x20(%rsp),%rdx
   bca4c:	call   *0x8fbb6(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   bca52:	mov    %r15,%r12
   bca55:	jmp    bcab3 <meshoptimizer_rs::codec::raw+0x1583>
   bca57:	test   %al,%al
   bca59:	lea    -0xadfa0(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bca60:	lea    -0xade77(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bca67:	cmove  %rcx,%rax
   bca6b:	test   %dl,%dl
   bca6d:	lea    -0xae2d4(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bca74:	cmove  %rcx,%rax
   bca78:	mov    %rax,(%rsp)
   bca7c:	lea    0x40(%rsp),%rdi
   bca81:	mov    0x30(%rsp),%rsi
   bca86:	mov    0x630(%rsp),%rdx
   bca8e:	mov    %r15,%rcx
   bca91:	lea    0x1f0(%rsp),%r8
   bca99:	mov    0x28(%rsp),%r9
   bca9e:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcaa3:	cmpb   $0x0,0x40(%rsp)
   bcaa8:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcaae:	mov    0x48(%rsp),%r12
   bcab3:	mov    %ebx,%eax
   bcab5:	shr    $0x2,%al
   bcab8:	and    $0x3,%al
   bcaba:	cmp    $0x3,%al
   bcabc:	movzbl 0x38(%rsp),%edx
   bcac1:	je     bcae5 <meshoptimizer_rs::codec::raw+0x15b5>
   bcac3:	movzbl %al,%ecx
   bcac6:	cmp    $0x2,%ecx
   bcac9:	jne    bcb23 <meshoptimizer_rs::codec::raw+0x15f3>
   bcacb:	mov    0x80(%rsp),%rdi
   bcad3:	xor    %esi,%esi
   bcad5:	mov    0x20(%rsp),%rdx
   bcada:	call   *0x8fb28(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   bcae0:	jmp    bcb7f <meshoptimizer_rs::codec::raw+0x164f>
   bcae5:	mov    %r12,%r15
   bcae8:	mov    $0x7,%al
   bcaea:	add    0x20(%rsp),%r15
   bcaef:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcaf5:	cmp    0x630(%rsp),%r15
   bcafd:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcb03:	add    0x30(%rsp),%r12
   bcb08:	mov    0x80(%rsp),%rdi
   bcb10:	mov    %r12,%rsi
   bcb13:	mov    0x20(%rsp),%rdx
   bcb18:	call   *0x8fada(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bcb1e:	mov    %r15,%r12
   bcb21:	jmp    bcb7f <meshoptimizer_rs::codec::raw+0x164f>
   bcb23:	test   %al,%al
   bcb25:	lea    -0xae06c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcb2c:	lea    -0xadf43(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcb33:	cmove  %rcx,%rax
   bcb37:	test   %dl,%dl
   bcb39:	lea    -0xae3a0(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcb40:	cmove  %rcx,%rax
   bcb44:	mov    %rax,(%rsp)
   bcb48:	lea    0x40(%rsp),%rdi
   bcb4d:	mov    0x30(%rsp),%rsi
   bcb52:	mov    0x630(%rsp),%rdx
   bcb5a:	mov    %r12,%rcx
   bcb5d:	mov    0x80(%rsp),%r8
   bcb65:	mov    0x28(%rsp),%r9
   bcb6a:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcb6f:	cmpb   $0x0,0x40(%rsp)
   bcb74:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcb7a:	mov    0x48(%rsp),%r12
   bcb7f:	mov    %ebx,%eax
   bcb81:	shr    $0x4,%al
   bcb84:	and    $0x3,%al
   bcb86:	cmp    $0x3,%al
   bcb88:	je     bcbd6 <meshoptimizer_rs::codec::raw+0x16a6>
   bcb8a:	movzbl %al,%ecx
   bcb8d:	cmp    $0x2,%ecx
   bcb90:	jne    bcc52 <meshoptimizer_rs::codec::raw+0x1722>
   bcb96:	mov    0x78(%rsp),%rdi
   bcb9b:	xor    %esi,%esi
   bcb9d:	mov    0x20(%rsp),%rdx
   bcba2:	call   *0x8fa60(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   bcba8:	shr    $0x6,%bl
   bcbab:	cmp    $0x3,%bl
   bcbae:	je     bcc17 <meshoptimizer_rs::codec::raw+0x16e7>
   bcbb0:	movzbl %bl,%eax
   bcbb3:	cmp    $0x2,%eax
   bcbb6:	jne    bccbf <meshoptimizer_rs::codec::raw+0x178f>
   bcbbc:	mov    0x70(%rsp),%rdi
   bcbc1:	xor    %esi,%esi
   bcbc3:	mov    0x20(%rsp),%rdx
   bcbc8:	call   *0x8fa3a(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   bcbce:	mov    %r12,%r15
   bcbd1:	jmp    bcd1b <meshoptimizer_rs::codec::raw+0x17eb>
   bcbd6:	mov    %r12,%r15
   bcbd9:	mov    $0x7,%al
   bcbdb:	add    0x20(%rsp),%r15
   bcbe0:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcbe6:	cmp    0x630(%rsp),%r15
   bcbee:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcbf4:	add    0x30(%rsp),%r12
   bcbf9:	mov    0x78(%rsp),%rdi
   bcbfe:	mov    %r12,%rsi
   bcc01:	mov    0x20(%rsp),%rdx
   bcc06:	call   *0x8f9ec(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bcc0c:	mov    %r15,%r12
   bcc0f:	shr    $0x6,%bl
   bcc12:	cmp    $0x3,%bl
   bcc15:	jne    bcbb0 <meshoptimizer_rs::codec::raw+0x1680>
   bcc17:	mov    %r12,%r15
   bcc1a:	mov    $0x7,%al
   bcc1c:	add    0x20(%rsp),%r15
   bcc21:	jb     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcc27:	cmp    0x630(%rsp),%r15
   bcc2f:	ja     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bcc35:	add    0x30(%rsp),%r12
   bcc3a:	mov    0x70(%rsp),%rdi
   bcc3f:	mov    %r12,%rsi
   bcc42:	mov    0x20(%rsp),%rdx
   bcc47:	call   *0x8f9ab(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bcc4d:	jmp    bcd1b <meshoptimizer_rs::codec::raw+0x17eb>
   bcc52:	test   %al,%al
   bcc54:	lea    -0xae19b(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcc5b:	lea    -0xae072(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcc62:	cmove  %rcx,%rax
   bcc66:	cmpb   $0x0,0x38(%rsp)
   bcc6b:	lea    -0xae4d2(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcc72:	cmove  %rcx,%rax
   bcc76:	mov    %rax,(%rsp)
   bcc7a:	lea    0x40(%rsp),%rdi
   bcc7f:	mov    0x30(%rsp),%rsi
   bcc84:	mov    0x630(%rsp),%rdx
   bcc8c:	mov    %r12,%rcx
   bcc8f:	mov    0x78(%rsp),%r8
   bcc94:	mov    0x28(%rsp),%r9
   bcc99:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcc9e:	cmpb   $0x0,0x40(%rsp)
   bcca3:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcca9:	mov    0x48(%rsp),%r12
   bccae:	shr    $0x6,%bl
   bccb1:	cmp    $0x3,%bl
   bccb4:	jne    bcbb0 <meshoptimizer_rs::codec::raw+0x1680>
   bccba:	jmp    bcc17 <meshoptimizer_rs::codec::raw+0x16e7>
   bccbf:	test   %bl,%bl
   bccc1:	lea    -0xae208(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bccc8:	lea    -0xae0df(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcccf:	cmove  %rcx,%rax
   bccd3:	cmpb   $0x0,0x38(%rsp)
   bccd8:	lea    -0xae53f(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bccdf:	cmove  %rcx,%rax
   bcce3:	mov    %rax,(%rsp)
   bcce7:	lea    0x40(%rsp),%rdi
   bccec:	mov    0x30(%rsp),%rsi
   bccf1:	mov    0x630(%rsp),%rdx
   bccf9:	mov    %r12,%rcx
   bccfc:	mov    0x70(%rsp),%r8
   bcd01:	mov    0x28(%rsp),%r9
   bcd06:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcd0b:	cmpb   $0x0,0x40(%rsp)
   bcd10:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcd16:	mov    0x48(%rsp),%r15
   bcd1b:	cmpb   $0x0,0x38(%rsp)
   bcd20:	mov    0x88(%rsp),%r12
   bcd28:	mov    0xb8(%rsp),%rsi
   bcd30:	mov    0x58(%rsp),%rbx
   bcd35:	je     bcd5c <meshoptimizer_rs::codec::raw+0x182c>
   bcd37:	cmp    0xa0(%rsp),%r14
   bcd3f:	jae    bd15d <meshoptimizer_rs::codec::raw+0x1c2d>
   bcd45:	mov    0x60(%rsp),%rax
   bcd4a:	movzbl (%rax,%r14,1),%eax
   bcd4f:	mov    %rsi,%rdx
   bcd52:	sub    %rbx,%rdx
   bcd55:	jae    bcd6a <meshoptimizer_rs::codec::raw+0x183a>
   bcd57:	jmp    bd0ba <meshoptimizer_rs::codec::raw+0x1b8a>
   bcd5c:	xor    %eax,%eax
   bcd5e:	mov    %rsi,%rdx
   bcd61:	sub    %rbx,%rdx
   bcd64:	jb     bd0ba <meshoptimizer_rs::codec::raw+0x1b8a>
   bcd6a:	cmp    %rdx,0x50(%rsp)
   bcd6f:	ja     bd0a6 <meshoptimizer_rs::codec::raw+0x1b76>
   bcd75:	mov    0xb0(%rsp),%rcx
   bcd7d:	lea    (%rcx,%rbx,1),%rdx
   bcd81:	mov    %eax,%ecx
   bcd83:	and    $0x3,%cl
   bcd86:	movzbl %cl,%ecx
   bcd89:	lea    -0xab5e0(%rip),%rsi        # 117b0 <libm::math::generic::sqrt::RSQRT_TAB+0x506>
   bcd90:	movslq (%rsi,%rcx,4),%rcx
   bcd94:	add    %rsi,%rcx
   bcd97:	jmp    *%rcx
   bcd99:	mov    %rdx,%r12
   bcd9c:	add    %rsp,%rbx
   bcd9f:	add    $0xf0,%rbx
   bcda6:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   bcdab:	test   %al,%al
   bcdad:	je     bce12 <meshoptimizer_rs::codec::raw+0x18e2>
   bcdaf:	mov    %rbx,(%rsp)
   bcdb3:	lea    0x1f0(%rsp),%rdi
   bcdbb:	mov    0x68(%rsp),%rsi
   bcdc0:	mov    %r12,%rdx
   bcdc3:	mov    0x50(%rsp),%rcx
   bcdc8:	mov    0x20(%rsp),%r8
   bcdcd:	mov    %rbp,%r9
   bcdd0:	call   7fb40 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   bcdd5:	jmp    bce41 <meshoptimizer_rs::codec::raw+0x1911>
   bcdd7:	lea    (%rsp,%rbx,1),%rax
   bcddb:	add    $0xf0,%rax
   bcde1:	mov    %rax,(%rsp)
   bcde5:	movq   $0x0,0x8(%rsp)
   bcdee:	lea    0x1f0(%rsp),%rdi
   bcdf6:	mov    0x68(%rsp),%rsi
   bcdfb:	mov    0x50(%rsp),%rcx
   bce00:	mov    0x20(%rsp),%r8
   bce05:	mov    %rbp,%r9
   bce08:	call   7d430 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   bce0d:	jmp    bc9a0 <meshoptimizer_rs::codec::raw+0x1470>
   bce12:	mov    %rbx,(%rsp)
   bce16:	movq   $0x0,0x8(%rsp)
   bce1f:	lea    0x1f0(%rsp),%rdi
   bce27:	mov    0x68(%rsp),%rsi
   bce2c:	mov    %r12,%rdx
   bce2f:	mov    0x50(%rsp),%rcx
   bce34:	mov    0x20(%rsp),%r8
   bce39:	mov    %rbp,%r9
   bce3c:	call   7d240 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   bce41:	mov    0x88(%rsp),%r12
   bce49:	mov    0x58(%rsp),%rbx
   bce4e:	jmp    bc9a0 <meshoptimizer_rs::codec::raw+0x1470>
   bce53:	cmp    $0x2,%al
   bce55:	je     bd19b <meshoptimizer_rs::codec::raw+0x1c6b>
   bce5b:	cmp    $0x3,%ecx
   bce5e:	je     bd182 <meshoptimizer_rs::codec::raw+0x1c52>
   bce64:	test   %al,%al
   bce66:	lea    -0xae3ad(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bce6d:	lea    -0xae284(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bce74:	cmove  %rcx,%rax
   bce78:	test   %dl,%dl
   bce7a:	lea    -0xae6e1(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bce81:	cmove  %rcx,%rax
   bce85:	mov    %rax,(%rsp)
   bce89:	lea    0x40(%rsp),%rdi
   bce8e:	mov    0x30(%rsp),%rsi
   bce93:	mov    0x630(%rsp),%rdx
   bce9b:	mov    %r15,%rcx
   bce9e:	lea    0x1f0(%rsp),%r8
   bcea6:	mov    0x28(%rsp),%r9
   bceab:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bceb0:	cmpb   $0x0,0x40(%rsp)
   bceb5:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcebb:	mov    %ebx,%eax
   bcebd:	shr    $0x2,%al
   bcec0:	and    $0x3,%al
   bcec2:	cmp    $0x2,%al
   bcec4:	movzbl 0x38(%rsp),%edx
   bcec9:	je     bd19b <meshoptimizer_rs::codec::raw+0x1c6b>
   bcecf:	movzbl %al,%ecx
   bced2:	cmp    $0x3,%ecx
   bced5:	je     bd182 <meshoptimizer_rs::codec::raw+0x1c52>
   bcedb:	test   %al,%al
   bcedd:	lea    -0xae424(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcee4:	lea    -0xae2fb(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bceeb:	cmove  %rcx,%rax
   bceef:	mov    0x48(%rsp),%rcx
   bcef4:	test   %dl,%dl
   bcef6:	lea    -0xae75d(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcefd:	cmove  %rdx,%rax
   bcf01:	mov    %rax,(%rsp)
   bcf05:	lea    0x40(%rsp),%rdi
   bcf0a:	mov    0x30(%rsp),%rsi
   bcf0f:	mov    0x630(%rsp),%rdx
   bcf17:	mov    0x80(%rsp),%r8
   bcf1f:	mov    0x28(%rsp),%r9
   bcf24:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcf29:	cmpb   $0x0,0x40(%rsp)
   bcf2e:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcf34:	mov    %ebx,%eax
   bcf36:	shr    $0x4,%al
   bcf39:	and    $0x3,%al
   bcf3b:	cmp    $0x2,%al
   bcf3d:	movzbl 0x38(%rsp),%edx
   bcf42:	je     bd19b <meshoptimizer_rs::codec::raw+0x1c6b>
   bcf48:	movzbl %al,%ecx
   bcf4b:	cmp    $0x3,%ecx
   bcf4e:	je     bd182 <meshoptimizer_rs::codec::raw+0x1c52>
   bcf54:	test   %al,%al
   bcf56:	lea    -0xae49d(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcf5d:	lea    -0xae374(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcf64:	cmove  %rcx,%rax
   bcf68:	mov    0x48(%rsp),%rcx
   bcf6d:	test   %dl,%dl
   bcf6f:	lea    -0xae7d6(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcf76:	cmove  %rdx,%rax
   bcf7a:	mov    %rax,(%rsp)
   bcf7e:	lea    0x40(%rsp),%rdi
   bcf83:	mov    0x30(%rsp),%rsi
   bcf88:	mov    0x630(%rsp),%rdx
   bcf90:	mov    0x78(%rsp),%r8
   bcf95:	mov    0x28(%rsp),%r9
   bcf9a:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   bcf9f:	cmpb   $0x0,0x40(%rsp)
   bcfa4:	jne    bd09f <meshoptimizer_rs::codec::raw+0x1b6f>
   bcfaa:	shr    $0x6,%bl
   bcfad:	cmp    $0x2,%bl
   bcfb0:	movzbl 0x38(%rsp),%edx
   bcfb5:	je     bd19b <meshoptimizer_rs::codec::raw+0x1c6b>
   bcfbb:	movzbl %bl,%eax
   bcfbe:	cmp    $0x3,%eax
   bcfc1:	je     bd182 <meshoptimizer_rs::codec::raw+0x1c52>
   bcfc7:	test   %bl,%bl
   bcfc9:	lea    -0xae510(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcfd0:	lea    -0xae3e7(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcfd7:	cmove  %rcx,%rax
   bcfdb:	mov    0x48(%rsp),%rcx
   bcfe0:	test   %dl,%dl
   bcfe2:	lea    -0xae849(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcfe9:	cmove  %rdx,%rax
   bcfed:	mov    %rax,(%rsp)
   bcff1:	lea    0x40(%rsp),%rdi
   bcff6:	mov    0x30(%rsp),%rsi
   bcffb:	mov    0x630(%rsp),%rdx
   bd003:	jmp    bccfc <meshoptimizer_rs::codec::raw+0x17cc>
   bd008:	mov    0xb8(%rsp),%rdx
   bd010:	mov    0xc0(%rsp),%rdi
   bd018:	cmp    %rdx,%rdi
   bd01b:	mov    0x30(%rsp),%r14
   bd020:	ja     bd124 <meshoptimizer_rs::codec::raw+0x1bf4>
   bd026:	mov    0xb0(%rsp),%rsi
   bd02e:	add    %rdi,%rsi
   bd031:	lea    0xf0(%rsp),%rdi
   bd039:	mov    %rbp,%rdx
   bd03c:	call   *0x8f5b6(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   bd042:	mov    0xc8(%rsp),%rcx
   bd04a:	mov    %rcx,%rax
   bd04d:	cmp    %r13,%rcx
   bd050:	mov    0x630(%rsp),%rbx
   bd058:	jb     bc81f <meshoptimizer_rs::codec::raw+0x12ef>
   bd05e:	jmp    bd066 <meshoptimizer_rs::codec::raw+0x1b36>
   bd060:	mov    $0x1,%r15d
   bd066:	sub    %r15,%rbx
   bd069:	setae  %cl
   bd06c:	cmp    0xe0(%rsp),%rbx
   bd074:	sete   %al
   bd077:	and    %cl,%al
   bd079:	neg    %al
   bd07b:	or     $0x7,%al
   bd07d:	jmp    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bd07f:	mov    0x28(%rsp),%r9
   bd084:	mov    $0xff,%al
   bd086:	cmp    %r9,%rsi
   bd089:	je     bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bd08b:	mov    $0x7,%al
   bd08d:	add    $0x5f8,%rsp
   bd094:	pop    %rbx
   bd095:	pop    %r12
   bd097:	pop    %r13
   bd099:	pop    %r14
   bd09b:	pop    %r15
   bd09d:	pop    %rbp
   bd09e:	ret
   bd09f:	movzbl 0x41(%rsp),%eax
   bd0a4:	jmp    bd08d <meshoptimizer_rs::codec::raw+0x1b5d>
   bd0a6:	lea    0x89873(%rip),%rcx        # 146920 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   bd0ad:	xor    %edi,%edi
   bd0af:	mov    0x50(%rsp),%rsi
   bd0b4:	call   *0x8f696(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd0ba:	lea    0x89877(%rip),%rcx        # 146938 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   bd0c1:	mov    %rsi,%rdx
   bd0c4:	mov    %rbx,%rdi
   bd0c7:	call   *0x8f683(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd0cd:	lea    0x898c4(%rip),%rcx        # 146998 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   bd0d4:	mov    $0x100,%edx
   bd0d9:	xor    %edi,%edi
   bd0db:	mov    %rbp,%rsi
   bd0de:	call   *0x8f66c(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd0e4:	lea    0x89895(%rip),%rcx        # 146980 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   bd0eb:	mov    %rsi,%rdi
   bd0ee:	mov    0x60(%rsp),%rsi
   bd0f3:	mov    %rbx,%rdx
   bd0f6:	call   *0x8f654(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd0fc:	lea    0x8d8d5(%rip),%rcx        # 14a9d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x65a8>
   bd103:	mov    %r12,%rdx
   bd106:	call   *0x8f644(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd10c:	lea    0x8989d(%rip),%rcx        # 1469b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   bd113:	mov    0x60(%rsp),%rdi
   bd118:	mov    %rbx,%rsi
   bd11b:	mov    %rbx,%rdx
   bd11e:	call   *0x8f62c(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd124:	lea    0x897ad(%rip),%rcx        # 1468d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   bd12b:	mov    %rdx,%rsi
   bd12e:	call   *0x8f61c(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd134:	lea    0x8d015(%rip),%rdx        # 14a150 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x5d20>
   bd13b:	xor    %edi,%edi
   bd13d:	xor    %esi,%esi
   bd13f:	call   *0x8f4eb(%rip)        # 14c630 <_DYNAMIC+0x250>
   bd145:	lea    0x897a4(%rip),%rdx        # 1468f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   bd14c:	mov    %r14,%rdi
   bd14f:	mov    0x90(%rsp),%rsi
   bd157:	call   *0x8f4d3(%rip)        # 14c630 <_DYNAMIC+0x250>
   bd15d:	lea    0x897a4(%rip),%rdx        # 146908 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   bd164:	mov    %r14,%rdi
   bd167:	mov    0xa0(%rsp),%rsi
   bd16f:	call   *0x8f4bb(%rip)        # 14c630 <_DYNAMIC+0x250>
   bd175:	lea    0x89744(%rip),%rdi        # 1468c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2490>
   bd17c:	call   *0x8f696(%rip)        # 14c818 <_DYNAMIC+0x438>
   bd182:	lea    0x897c7(%rip),%rcx        # 146950 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   bd189:	xor    %edi,%edi
   bd18b:	mov    0x20(%rsp),%rsi
   bd190:	mov    0x28(%rsp),%rdx
   bd195:	call   *0x8f5b5(%rip)        # 14c750 <_DYNAMIC+0x370>
   bd19b:	lea    0x897c6(%rip),%rcx        # 146968 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   bd1a2:	xor    %edi,%edi
   bd1a4:	mov    0x20(%rsp),%rsi
   bd1a9:	mov    0x28(%rsp),%rdx
   bd1ae:	call   *0x8f59c(%rip)        # 14c750 <_DYNAMIC+0x370>
