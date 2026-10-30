
<artifacts>/guard/driver-base:     file format elf64-x86-64


Disassembly of section .text:

00000000000bb670 <meshoptimizer_rs::codec::raw>:
   bb670:	push   %rbp
   bb671:	push   %r15
   bb673:	push   %r14
   bb675:	push   %r13
   bb677:	push   %r12
   bb679:	push   %rbx
   bb67a:	sub    $0x5f8,%rsp
   bb681:	mov    %r9,%r14
   bb684:	mov    %r8,%rbp
   bb687:	mov    %rcx,%r13
   bb68a:	mov    %rdx,%r12
   bb68d:	mov    %rsi,%r15
   bb690:	mov    0x630(%rsp),%rbx
   bb698:	test   %dil,%dil
   bb69b:	je     bb804 <meshoptimizer_rs::codec::raw+0x194>
   bb6a1:	movzbl %dil,%eax
   bb6a5:	cmp    $0x1,%eax
   bb6a8:	jne    bb8c6 <meshoptimizer_rs::codec::raw+0x256>
   bb6ae:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   bb6b8:	mov    %r13,%rax
   bb6bb:	mul    %rdi
   bb6be:	shr    $1,%rdx
   bb6c1:	lea    (%rdx,%rdx,2),%rcx
   bb6c5:	mov    %r13,%rax
   bb6c8:	sub    %rcx,%rax
   bb6cb:	cmp    $0x2,%rbp
   bb6cf:	jne    bb931 <meshoptimizer_rs::codec::raw+0x2c1>
   bb6d5:	test   %rax,%rax
   bb6d8:	jne    bb93a <meshoptimizer_rs::codec::raw+0x2ca>
   bb6de:	mov    %r13,%rax
   bb6e1:	mul    %rdi
   bb6e4:	shr    $1,%rdx
   bb6e7:	lea    0x11(%rdx),%rcx
   bb6eb:	mov    $0x7,%al
   bb6ed:	cmp    %rcx,%rbx
   bb6f0:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb6f6:	movzbl (%r14),%ecx
   bb6fa:	mov    %ecx,%esi
   bb6fc:	and    $0xf0,%sil
   bb700:	cmp    $0xe0,%sil
   bb704:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb70a:	and    $0xf,%cl
   bb70d:	mov    $0x5,%al
   bb70f:	cmp    $0x1,%cl
   bb712:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb718:	lea    0x1(%rdx),%rsi
   bb71c:	pcmpeqd %xmm0,%xmm0
   bb720:	movdqa %xmm0,0x260(%rsp)
   bb729:	movdqa %xmm0,0x250(%rsp)
   bb732:	movdqa %xmm0,0x240(%rsp)
   bb73b:	movdqa %xmm0,0x230(%rsp)
   bb744:	movdqa %xmm0,0x220(%rsp)
   bb74d:	movdqa %xmm0,0x210(%rsp)
   bb756:	movdqa %xmm0,0x200(%rsp)
   bb75f:	movdqa %xmm0,0x1f0(%rsp)
   bb768:	movdqa %xmm0,0x120(%rsp)
   bb771:	movdqa %xmm0,0x110(%rsp)
   bb77a:	movdqa %xmm0,0x100(%rsp)
   bb783:	movdqa %xmm0,0xf0(%rsp)
   bb78c:	xor    %r8d,%r8d
   bb78f:	cmp    $0x1,%cl
   bb792:	setne  %r9b
   bb796:	lea    -0x10(%rbx),%rax
   bb79a:	mov    %rax,0x28(%rsp)
   bb79f:	mov    %r12,%rax
   bb7a2:	mov    %rdx,%rcx
   bb7a5:	mul    %rdi
   bb7a8:	shr    $0x2,%rdx
   bb7ac:	cmp    %rcx,%rdx
   bb7af:	cmovae %rcx,%rdx
   bb7b3:	mov    %rdx,0x38(%rsp)
   bb7b8:	test   %rdx,%rdx
   bb7bb:	je     bd1bf <meshoptimizer_rs::codec::raw+0x1b4f>
   bb7c1:	mov    %r9b,%r8b
   bb7c4:	lea    0xd(,%r8,2),%rax
   bb7cc:	mov    %rax,0x58(%rsp)
   bb7d1:	mov    %r14,0x30(%rsp)
   bb7d6:	lea    (%r14,%rbx,1),%rax
   bb7da:	add    $0xfffffffffffffff0,%rax
   bb7de:	mov    %rax,0x50(%rsp)
   bb7e3:	add    $0x4,%r15
   bb7e7:	xor    %r8d,%r8d
   bb7ea:	xor    %r10d,%r10d
   bb7ed:	movl   $0x0,0x20(%rsp)
   bb7f5:	xor    %r11d,%r11d
   bb7f8:	xor    %edi,%edi
   bb7fa:	mov    0x28(%rsp),%r9
   bb7ff:	jmp    bbcc6 <meshoptimizer_rs::codec::raw+0x656>
   bb804:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   bb809:	cmp    $0x2,%al
   bb80b:	jb     bb874 <meshoptimizer_rs::codec::raw+0x204>
   bb80d:	cmp    $0x20,%rbp
   bb811:	lea    -0x37c58(%rip),%rax        # 83bc0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   bb818:	lea    -0x3913f(%rip),%rcx        # 826e0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   bb81f:	cmove  %rax,%rcx
   bb823:	lea    -0x3221a(%rip),%rax        # 89610 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   bb82a:	lea    -0x33ac1(%rip),%rdx        # 87d70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   bb831:	cmove  %rax,%rdx
   bb835:	cmp    $0x4,%rbp
   bb839:	lea    -0x355f0(%rip),%rax        # 86250 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   bb840:	cmovne %rcx,%rax
   bb844:	lea    -0x2f79b(%rip),%r10        # 8c0b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   bb84b:	cmovne %rdx,%r10
   bb84f:	cmp    $0x21,%r13
   bb853:	cmovae %rax,%r10
   bb857:	mov    %r15,%rdi
   bb85a:	mov    %r12,%rsi
   bb85d:	mov    %r13,%rdx
   bb860:	mov    %rbp,%rcx
   bb863:	mov    %r14,%r8
   bb866:	mov    %rbx,%r9
   bb869:	call   *%r10
   bb86c:	cmp    $0xfe,%al
   bb86e:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb874:	mov    $0x7,%al
   bb876:	test   %rbx,%rbx
   bb879:	je     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb87f:	movzbl (%r14),%edi
   bb883:	mov    %edi,%ecx
   bb885:	and    $0xf0,%cl
   bb888:	cmp    $0xa0,%cl
   bb88b:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb891:	and    $0xf,%dil
   bb895:	mov    $0x5,%al
   bb897:	cmp    $0x1,%dil
   bb89b:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb8a1:	test   %dil,%dil
   bb8a4:	je     bc7c3 <meshoptimizer_rs::codec::raw+0x1153>
   bb8aa:	mov    %rbp,%rcx
   bb8ad:	shr    $0x2,%rcx
   bb8b1:	add    %rbp,%rcx
   bb8b4:	cmp    $0x19,%rcx
   bb8b8:	mov    $0x18,%eax
   bb8bd:	cmovae %rcx,%rax
   bb8c1:	jmp    bc7d3 <meshoptimizer_rs::codec::raw+0x1163>
   bb8c6:	add    $0x5,%r13
   bb8ca:	mov    $0x7,%al
   bb8cc:	cmp    $0x2,%rbp
   bb8d0:	jne    bb941 <meshoptimizer_rs::codec::raw+0x2d1>
   bb8d2:	cmp    %r13,%rbx
   bb8d5:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb8db:	test   %rbx,%rbx
   bb8de:	je     bd274 <meshoptimizer_rs::codec::raw+0x1c04>
   bb8e4:	movzbl (%r14),%ecx
   bb8e8:	mov    %ecx,%edx
   bb8ea:	and    $0xf0,%dl
   bb8ed:	cmp    $0xd0,%dl
   bb8f0:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb8f6:	mov    $0x5,%al
   bb8f8:	test   $0xe,%cl
   bb8fb:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb901:	dec    %rbx
   bb904:	movq   $0x0,0x1f0(%rsp)
   bb910:	movabs $0x7ffffffffffffffc,%rax
   bb91a:	or     $0x2,%rax
   bb91e:	and    %rax,%r12
   bb921:	je     bbc24 <meshoptimizer_rs::codec::raw+0x5b4>
   bb927:	inc    %r14
   bb92a:	xor    %eax,%eax
   bb92c:	jmp    bb9d6 <meshoptimizer_rs::codec::raw+0x366>
   bb931:	test   %rax,%rax
   bb934:	je     bbb07 <meshoptimizer_rs::codec::raw+0x497>
   bb93a:	xor    %eax,%eax
   bb93c:	jmp    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb941:	cmp    %r13,%rbx
   bb944:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb94a:	test   %rbx,%rbx
   bb94d:	je     bd274 <meshoptimizer_rs::codec::raw+0x1c04>
   bb953:	movzbl (%r14),%ecx
   bb957:	mov    %ecx,%edx
   bb959:	and    $0xf0,%dl
   bb95c:	cmp    $0xd0,%dl
   bb95f:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb965:	mov    $0x5,%al
   bb967:	test   $0xe,%cl
   bb96a:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bb970:	dec    %rbx
   bb973:	movq   $0x0,0x1f0(%rsp)
   bb97f:	movabs $0x7ffffffffffffffc,%rax
   bb989:	and    %rax,%r12
   bb98c:	je     bbc24 <meshoptimizer_rs::codec::raw+0x5b4>
   bb992:	inc    %r14
   bb995:	xor    %eax,%eax
   bb997:	jmp    bba8b <meshoptimizer_rs::codec::raw+0x41b>
   bb99c:	dec    %rbx
   bb99f:	inc    %r14
   bb9a2:	mov    %edx,%ecx
   bb9a4:	mov    %ecx,%edx
   bb9a6:	shr    $0x2,%edx
   bb9a9:	mov    %ecx,%esi
   bb9ab:	shl    $0x1e,%esi
   bb9ae:	sar    $0x1f,%esi
   bb9b1:	xor    %edx,%esi
   bb9b3:	and    $0x1,%ecx
   bb9b6:	add    0x1f0(%rsp,%rcx,4),%esi
   bb9bd:	mov    %esi,0x1f0(%rsp,%rcx,4)
   bb9c4:	mov    %si,(%r15,%rax,1)
   bb9c9:	add    $0x2,%rax
   bb9cd:	cmp    %rax,%r12
   bb9d0:	je     bbc24 <meshoptimizer_rs::codec::raw+0x5b4>
   bb9d6:	cmp    $0x5,%rbx
   bb9da:	jb     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bb9e0:	movzbl (%r14),%edx
   bb9e4:	test   %dl,%dl
   bb9e6:	jns    bb99c <meshoptimizer_rs::codec::raw+0x32c>
   bb9e8:	and    $0x7f,%edx
   bb9eb:	movzbl 0x1(%r14),%ecx
   bb9f0:	mov    %ecx,%esi
   bb9f2:	and    $0x7f,%ecx
   bb9f5:	shl    $0x7,%ecx
   bb9f8:	or     %edx,%ecx
   bb9fa:	mov    $0x2,%edx
   bb9ff:	test   %sil,%sil
   bba02:	jns    bba47 <meshoptimizer_rs::codec::raw+0x3d7>
   bba04:	movzbl 0x2(%r14),%edx
   bba09:	mov    %edx,%esi
   bba0b:	and    $0x7f,%edx
   bba0e:	shl    $0xe,%edx
   bba11:	or     %edx,%ecx
   bba13:	mov    $0x3,%edx
   bba18:	test   %sil,%sil
   bba1b:	jns    bba47 <meshoptimizer_rs::codec::raw+0x3d7>
   bba1d:	movzbl 0x3(%r14),%edx
   bba22:	mov    %edx,%esi
   bba24:	and    $0x7f,%edx
   bba27:	shl    $0x15,%edx
   bba2a:	or     %edx,%ecx
   bba2c:	mov    $0x4,%edx
   bba31:	test   %sil,%sil
   bba34:	jns    bba47 <meshoptimizer_rs::codec::raw+0x3d7>
   bba36:	movzbl 0x4(%r14),%esi
   bba3b:	shl    $0x1c,%esi
   bba3e:	or     %ecx,%esi
   bba40:	mov    $0x5,%edx
   bba45:	mov    %esi,%ecx
   bba47:	sub    %rdx,%rbx
   bba4a:	add    %rdx,%r14
   bba4d:	jmp    bb9a4 <meshoptimizer_rs::codec::raw+0x334>
   bba52:	dec    %rbx
   bba55:	inc    %r14
   bba58:	mov    %edx,%ecx
   bba5a:	mov    %ecx,%edx
   bba5c:	shr    $0x2,%edx
   bba5f:	mov    %ecx,%esi
   bba61:	shl    $0x1e,%esi
   bba64:	sar    $0x1f,%esi
   bba67:	xor    %edx,%esi
   bba69:	and    $0x1,%ecx
   bba6c:	add    0x1f0(%rsp,%rcx,4),%esi
   bba73:	mov    %esi,0x1f0(%rsp,%rcx,4)
   bba7a:	mov    %esi,(%r15,%rax,1)
   bba7e:	add    $0x4,%rax
   bba82:	cmp    %rax,%r12
   bba85:	je     bbc24 <meshoptimizer_rs::codec::raw+0x5b4>
   bba8b:	cmp    $0x5,%rbx
   bba8f:	jb     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bba95:	movzbl (%r14),%edx
   bba99:	test   %dl,%dl
   bba9b:	jns    bba52 <meshoptimizer_rs::codec::raw+0x3e2>
   bba9d:	and    $0x7f,%edx
   bbaa0:	movzbl 0x1(%r14),%ecx
   bbaa5:	mov    %ecx,%esi
   bbaa7:	and    $0x7f,%ecx
   bbaaa:	shl    $0x7,%ecx
   bbaad:	or     %edx,%ecx
   bbaaf:	mov    $0x2,%edx
   bbab4:	test   %sil,%sil
   bbab7:	jns    bbafc <meshoptimizer_rs::codec::raw+0x48c>
   bbab9:	movzbl 0x2(%r14),%edx
   bbabe:	mov    %edx,%esi
   bbac0:	and    $0x7f,%edx
   bbac3:	shl    $0xe,%edx
   bbac6:	or     %edx,%ecx
   bbac8:	mov    $0x3,%edx
   bbacd:	test   %sil,%sil
   bbad0:	jns    bbafc <meshoptimizer_rs::codec::raw+0x48c>
   bbad2:	movzbl 0x3(%r14),%edx
   bbad7:	mov    %edx,%esi
   bbad9:	and    $0x7f,%edx
   bbadc:	shl    $0x15,%edx
   bbadf:	or     %edx,%ecx
   bbae1:	mov    $0x4,%edx
   bbae6:	test   %sil,%sil
   bbae9:	jns    bbafc <meshoptimizer_rs::codec::raw+0x48c>
   bbaeb:	movzbl 0x4(%r14),%esi
   bbaf0:	shl    $0x1c,%esi
   bbaf3:	or     %ecx,%esi
   bbaf5:	mov    $0x5,%edx
   bbafa:	mov    %esi,%ecx
   bbafc:	sub    %rdx,%rbx
   bbaff:	add    %rdx,%r14
   bbb02:	jmp    bba5a <meshoptimizer_rs::codec::raw+0x3ea>
   bbb07:	lea    0x11(%rdx),%rcx
   bbb0b:	mov    $0x7,%al
   bbb0d:	cmp    %rcx,%rbx
   bbb10:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bbb16:	movzbl (%r14),%ecx
   bbb1a:	mov    %ecx,%esi
   bbb1c:	and    $0xf0,%sil
   bbb20:	cmp    $0xe0,%sil
   bbb24:	jne    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bbb2a:	and    $0xf,%cl
   bbb2d:	mov    $0x5,%al
   bbb2f:	cmp    $0x1,%cl
   bbb32:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bbb38:	lea    0x1(%rdx),%rsi
   bbb3c:	pcmpeqd %xmm0,%xmm0
   bbb40:	movdqa %xmm0,0x260(%rsp)
   bbb49:	movdqa %xmm0,0x250(%rsp)
   bbb52:	movdqa %xmm0,0x240(%rsp)
   bbb5b:	movdqa %xmm0,0x230(%rsp)
   bbb64:	movdqa %xmm0,0x220(%rsp)
   bbb6d:	movdqa %xmm0,0x210(%rsp)
   bbb76:	movdqa %xmm0,0x200(%rsp)
   bbb7f:	movdqa %xmm0,0x1f0(%rsp)
   bbb88:	movdqa %xmm0,0x120(%rsp)
   bbb91:	movdqa %xmm0,0x110(%rsp)
   bbb9a:	movdqa %xmm0,0x100(%rsp)
   bbba3:	movdqa %xmm0,0xf0(%rsp)
   bbbac:	xor    %r8d,%r8d
   bbbaf:	cmp    $0x1,%cl
   bbbb2:	setne  %r9b
   bbbb6:	lea    -0x10(%rbx),%rax
   bbbba:	mov    %rax,0x28(%rsp)
   bbbbf:	mov    %r12,%rax
   bbbc2:	mov    %rdx,%rcx
   bbbc5:	mul    %rdi
   bbbc8:	shr    $0x3,%rdx
   bbbcc:	cmp    %rcx,%rdx
   bbbcf:	cmovae %rcx,%rdx
   bbbd3:	mov    %rdx,0x38(%rsp)
   bbbd8:	test   %rdx,%rdx
   bbbdb:	je     bd1bf <meshoptimizer_rs::codec::raw+0x1b4f>
   bbbe1:	mov    %r9b,%r8b
   bbbe4:	lea    0xd(,%r8,2),%rax
   bbbec:	mov    %rax,0x58(%rsp)
   bbbf1:	mov    %r14,0x30(%rsp)
   bbbf6:	lea    (%r14,%rbx,1),%rax
   bbbfa:	add    $0xfffffffffffffff0,%rax
   bbbfe:	mov    %rax,0x50(%rsp)
   bbc03:	add    $0x8,%r15
   bbc07:	xor    %r8d,%r8d
   bbc0a:	xor    %r10d,%r10d
   bbc0d:	movl   $0x0,0x20(%rsp)
   bbc15:	xor    %r11d,%r11d
   bbc18:	xor    %edi,%edi
   bbc1a:	mov    0x28(%rsp),%r9
   bbc1f:	jmp    bc28e <meshoptimizer_rs::codec::raw+0xc1e>
   bbc24:	cmp    $0x4,%rbx
   bbc28:	sete   %al
   bbc2b:	jmp    bd1b9 <meshoptimizer_rs::codec::raw+0x1b49>
   bbc30:	mov    %bpl,%r14b
   bbc33:	add    %r14d,%r11d
   bbc36:	mov    %ebx,0xf0(%rsp,%r10,4)
   bbc3e:	mov    %r11d,%ebp
   bbc41:	not    %eax
   bbc43:	shr    $0x4,%eax
   bbc46:	add    %r8d,%eax
   bbc49:	and    $0xf,%eax
   bbc4c:	mov    0x1f0(%rsp,%rax,8),%r11d
   bbc54:	mov    0x1f4(%rsp,%rax,8),%r15d
   bbc5c:	add    %r14,%r10
   bbc5f:	mov    $0x2,%eax
   bbc64:	mov    $0x1,%r12d
   bbc6a:	mov    %r8,%r13
   bbc6d:	inc    %rdi
   bbc70:	mov    %ebx,0x1f0(%rsp,%r13,8)
   bbc78:	mov    %r15d,0x1f4(%rsp,%r13,8)
   bbc80:	add    %r8d,%r12d
   bbc83:	and    $0xf,%r12d
   bbc87:	mov    %r11d,0x1f0(%rsp,%r12,8)
   bbc8f:	mov    %ebx,0x1f4(%rsp,%r12,8)
   bbc97:	add    %r8d,%eax
   bbc9a:	and    $0xf,%eax
   bbc9d:	and    $0xf,%r10d
   bbca1:	mov    %r11w,-0x4(%rdx)
   bbca6:	mov    %r15w,-0x2(%rdx)
   bbcab:	mov    %rdx,%r15
   bbcae:	mov    %bx,(%rdx)
   bbcb1:	add    $0x6,%r15
   bbcb5:	mov    %rax,%r8
   bbcb8:	mov    %ebp,%r11d
   bbcbb:	cmp    %rdi,0x38(%rsp)
   bbcc0:	je     bd1c4 <meshoptimizer_rs::codec::raw+0x1b54>
   bbcc6:	cmp    %r9,%rsi
   bbcc9:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bbccf:	mov    %r15,%rdx
   bbcd2:	mov    0x30(%rsp),%r14
   bbcd7:	movzbl 0x1(%r14,%rdi,1),%eax
   bbcdd:	cmp    $0xf0,%al
   bbcdf:	jae    bbd19 <meshoptimizer_rs::codec::raw+0x6a9>
   bbce1:	mov    %eax,%r15d
   bbce4:	and    $0xf,%r15d
   bbce8:	cmp    %r15d,0x58(%rsp)
   bbced:	jbe    bbd53 <meshoptimizer_rs::codec::raw+0x6e3>
   bbcef:	xor    %r14d,%r14d
   bbcf2:	mov    %r11d,%ebx
   bbcf5:	test   %r15b,%r15b
   bbcf8:	sete   %bpl
   bbcfc:	je     bbc30 <meshoptimizer_rs::codec::raw+0x5c0>
   bbd02:	not    %r15d
   bbd05:	add    %r10d,%r15d
   bbd08:	and    $0xf,%r15d
   bbd0c:	mov    0xf0(%rsp,%r15,4),%ebx
   bbd14:	jmp    bbc30 <meshoptimizer_rs::codec::raw+0x5c0>
   bbd19:	cmp    $0xfe,%al
   bbd1b:	jae    bbd75 <meshoptimizer_rs::codec::raw+0x705>
   bbd1d:	and    $0xf,%eax
   bbd20:	mov    0x50(%rsp),%rcx
   bbd25:	movzbl (%rcx,%rax,1),%r13d
   bbd2a:	lea    0x1(%r11),%r15d
   bbd2e:	xor    %eax,%eax
   bbd30:	mov    %r13d,%ecx
   bbd33:	shr    $0x4,%ecx
   bbd36:	sete   %r12b
   bbd3a:	je     bbdb8 <meshoptimizer_rs::codec::raw+0x748>
   bbd3c:	mov    %r10d,%ebx
   bbd3f:	sub    %ecx,%ebx
   bbd41:	and    $0xf,%ebx
   bbd44:	mov    %r15d,%ebp
   bbd47:	mov    0xf0(%rsp,%rbx,4),%ecx
   bbd4e:	mov    %ecx,%r15d
   bbd51:	jmp    bbdbc <meshoptimizer_rs::codec::raw+0x74c>
   bbd53:	cmp    $0xf,%r15b
   bbd57:	jne    bbe38 <meshoptimizer_rs::codec::raw+0x7c8>
   bbd5d:	movzbl (%r14,%rsi,1),%ebp
   bbd62:	test   %bpl,%bpl
   bbd65:	js     bbea6 <meshoptimizer_rs::codec::raw+0x836>
   bbd6b:	inc    %rsi
   bbd6e:	mov    %ebp,%ebx
   bbd70:	jmp    bc03c <meshoptimizer_rs::codec::raw+0x9cc>
   bbd75:	movzbl (%r14,%rsi,1),%r14d
   bbd7a:	xor    %r15d,%r15d
   bbd7d:	cmp    $0xfe,%eax
   bbd82:	sete   %r15b
   bbd86:	test   %r14d,%r14d
   bbd89:	cmove  %r14d,%r11d
   bbd8d:	add    %r11d,%r15d
   bbd90:	mov    %r14d,%r12d
   bbd93:	shr    $0x4,%r12d
   bbd97:	je     bbe53 <meshoptimizer_rs::codec::raw+0x7e3>
   bbd9d:	mov    %r10d,%ecx
   bbda0:	sub    %r12d,%ecx
   bbda3:	and    $0xf,%ecx
   bbda6:	mov    %r15d,%ebx
   bbda9:	mov    0xf0(%rsp,%rcx,4),%ecx
   bbdb0:	mov    %ecx,%r15d
   bbdb3:	jmp    bbe57 <meshoptimizer_rs::codec::raw+0x7e7>
   bbdb8:	lea    0x2(%r11),%ebp
   bbdbc:	xor    %r14d,%r14d
   bbdbf:	mov    %ebp,%ebx
   bbdc1:	test   $0xf,%r13b
   bbdc5:	sete   %cl
   bbdc8:	je     bbdda <meshoptimizer_rs::codec::raw+0x76a>
   bbdca:	mov    %r10d,%ebx
   bbdcd:	sub    %r13d,%ebx
   bbdd0:	and    $0xf,%ebx
   bbdd3:	mov    0xf0(%rsp,%rbx,4),%ebx
   bbdda:	mov    %cl,%r14b
   bbddd:	add    %r14d,%ebp
   bbde0:	mov    %r11d,0xf0(%rsp,%r10,4)
   bbde8:	lea    0x1(%r10),%rcx
   bbdec:	and    $0xf,%ecx
   bbdef:	mov    %r15d,0xf0(%rsp,%rcx,4)
   bbdf7:	mov    %r12b,%al
   bbdfa:	add    %r10,%rax
   bbdfd:	inc    %rax
   bbe00:	add    %rax,%r14
   bbe03:	and    $0xf,%eax
   bbe06:	mov    %ebx,0xf0(%rsp,%rax,4)
   bbe0d:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bbe15:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bbe1d:	lea    0x1(%r8),%r13d
   bbe21:	and    $0xf,%r13d
   bbe25:	mov    $0x3,%eax
   bbe2a:	mov    $0x2,%r12d
   bbe30:	mov    %r14,%r10
   bbe33:	jmp    bbc6d <meshoptimizer_rs::codec::raw+0x5fd>
   bbe38:	movzbl %r15b,%ecx
   bbe3c:	xor    $0x3,%r15b
   bbe40:	movzbl %r15b,%ebx
   bbe44:	mov    0x20(%rsp),%ebp
   bbe48:	add    %ecx,%ebp
   bbe4a:	sub    %ebx,%ebp
   bbe4c:	mov    %ebp,%ebx
   bbe4e:	jmp    bc04d <meshoptimizer_rs::codec::raw+0x9dd>
   bbe53:	lea    0x1(%r15),%ebx
   bbe57:	mov    %r14d,%r13d
   bbe5a:	and    $0xf,%r13b
   bbe5e:	je     bbe76 <meshoptimizer_rs::codec::raw+0x806>
   bbe60:	mov    %r10d,%ecx
   bbe63:	sub    %r14d,%ecx
   bbe66:	and    $0xf,%ecx
   bbe69:	mov    %ebx,%ebp
   bbe6b:	mov    0xf0(%rsp,%rcx,4),%ecx
   bbe72:	mov    %ecx,%ebx
   bbe74:	jmp    bbe79 <meshoptimizer_rs::codec::raw+0x809>
   bbe76:	lea    0x1(%rbx),%ebp
   bbe79:	lea    0x1(%rsi),%r14
   bbe7d:	cmp    $0xfe,%eax
   bbe82:	je     bbf80 <meshoptimizer_rs::codec::raw+0x910>
   bbe88:	mov    0x30(%rsp),%r9
   bbe8d:	movzbl 0x1(%r9,%rsi,1),%eax
   bbe93:	test   %al,%al
   bbe95:	js     bbec6 <meshoptimizer_rs::codec::raw+0x856>
   bbe97:	add    $0x2,%rsi
   bbe9b:	mov    %rsi,%r14
   bbe9e:	mov    %eax,%r11d
   bbea1:	jmp    bbf62 <meshoptimizer_rs::codec::raw+0x8f2>
   bbea6:	and    $0x7f,%ebp
   bbea9:	movzbl 0x1(%r14,%rsi,1),%ebx
   bbeaf:	mov    %ebx,%ecx
   bbeb1:	and    $0x7f,%ebx
   bbeb4:	shl    $0x7,%ebx
   bbeb7:	or     %ebp,%ebx
   bbeb9:	test   %cl,%cl
   bbebb:	js     bbee7 <meshoptimizer_rs::codec::raw+0x877>
   bbebd:	add    $0x2,%rsi
   bbec1:	jmp    bc03c <meshoptimizer_rs::codec::raw+0x9cc>
   bbec6:	and    $0x7f,%eax
   bbec9:	movzbl 0x2(%r9,%rsi,1),%r11d
   bbecf:	mov    %r11d,%ecx
   bbed2:	and    $0x7f,%r11d
   bbed6:	shl    $0x7,%r11d
   bbeda:	or     %eax,%r11d
   bbedd:	test   %cl,%cl
   bbedf:	js     bbf05 <meshoptimizer_rs::codec::raw+0x895>
   bbee1:	add    $0x3,%rsi
   bbee5:	jmp    bbf5f <meshoptimizer_rs::codec::raw+0x8ef>
   bbee7:	movzbl 0x2(%r14,%rsi,1),%ecx
   bbeed:	mov    %ecx,%ebp
   bbeef:	and    $0x7f,%ecx
   bbef2:	shl    $0xe,%ecx
   bbef5:	or     %ecx,%ebx
   bbef7:	test   %bpl,%bpl
   bbefa:	js     bbf20 <meshoptimizer_rs::codec::raw+0x8b0>
   bbefc:	add    $0x3,%rsi
   bbf00:	jmp    bc03c <meshoptimizer_rs::codec::raw+0x9cc>
   bbf05:	movzbl 0x3(%r9,%rsi,1),%eax
   bbf0b:	mov    %eax,%ecx
   bbf0d:	and    $0x7f,%eax
   bbf10:	shl    $0xe,%eax
   bbf13:	or     %eax,%r11d
   bbf16:	test   %cl,%cl
   bbf18:	js     bbf42 <meshoptimizer_rs::codec::raw+0x8d2>
   bbf1a:	add    $0x4,%rsi
   bbf1e:	jmp    bbf5f <meshoptimizer_rs::codec::raw+0x8ef>
   bbf20:	movzbl 0x3(%r14,%rsi,1),%ecx
   bbf26:	mov    %ecx,%ebp
   bbf28:	and    $0x7f,%ecx
   bbf2b:	shl    $0x15,%ecx
   bbf2e:	or     %ecx,%ebx
   bbf30:	test   %bpl,%bpl
   bbf33:	js     bc02b <meshoptimizer_rs::codec::raw+0x9bb>
   bbf39:	add    $0x4,%rsi
   bbf3d:	jmp    bc03c <meshoptimizer_rs::codec::raw+0x9cc>
   bbf42:	movzbl 0x4(%r9,%rsi,1),%eax
   bbf48:	mov    %eax,%ecx
   bbf4a:	and    $0x7f,%eax
   bbf4d:	shl    $0x15,%eax
   bbf50:	or     %eax,%r11d
   bbf53:	test   %cl,%cl
   bbf55:	js     bc064 <meshoptimizer_rs::codec::raw+0x9f4>
   bbf5b:	add    $0x5,%rsi
   bbf5f:	mov    %rsi,%r14
   bbf62:	mov    0x28(%rsp),%r9
   bbf67:	mov    %r11d,%eax
   bbf6a:	shr    $1,%eax
   bbf6c:	and    $0x1,%r11d
   bbf70:	neg    %r11d
   bbf73:	xor    %eax,%r11d
   bbf76:	add    0x20(%rsp),%r11d
   bbf7b:	mov    %r11d,0x20(%rsp)
   bbf80:	cmp    $0xf,%r12b
   bbf84:	jne    bc0b3 <meshoptimizer_rs::codec::raw+0xa43>
   bbf8a:	cmp    $0xfffffffffffffffa,%r14
   bbf8e:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bbf94:	lea    0x5(%r14),%rax
   bbf98:	cmp    0x630(%rsp),%rax
   bbfa0:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bbfa6:	mov    0x30(%rsp),%r9
   bbfab:	movzbl (%r9,%r14,1),%esi
   bbfb0:	test   %sil,%sil
   bbfb3:	js     bbfc3 <meshoptimizer_rs::codec::raw+0x953>
   bbfb5:	inc    %r14
   bbfb8:	mov    %r14,%rax
   bbfbb:	mov    %esi,%r15d
   bbfbe:	jmp    bc092 <meshoptimizer_rs::codec::raw+0xa22>
   bbfc3:	and    $0x7f,%esi
   bbfc6:	movzbl 0x1(%r9,%r14,1),%r15d
   bbfcc:	mov    %r15d,%ecx
   bbfcf:	and    $0x7f,%r15d
   bbfd3:	shl    $0x7,%r15d
   bbfd7:	or     %esi,%r15d
   bbfda:	test   %cl,%cl
   bbfdc:	js     bbfea <meshoptimizer_rs::codec::raw+0x97a>
   bbfde:	add    $0x2,%r14
   bbfe2:	mov    %r14,%rax
   bbfe5:	jmp    bc092 <meshoptimizer_rs::codec::raw+0xa22>
   bbfea:	movzbl 0x2(%r9,%r14,1),%ecx
   bbff0:	mov    %ecx,%esi
   bbff2:	and    $0x7f,%ecx
   bbff5:	shl    $0xe,%ecx
   bbff8:	or     %ecx,%r15d
   bbffb:	test   %sil,%sil
   bbffe:	js     bc00c <meshoptimizer_rs::codec::raw+0x99c>
   bc000:	add    $0x3,%r14
   bc004:	mov    %r14,%rax
   bc007:	jmp    bc092 <meshoptimizer_rs::codec::raw+0xa22>
   bc00c:	movzbl 0x3(%r9,%r14,1),%ecx
   bc012:	mov    %ecx,%esi
   bc014:	and    $0x7f,%ecx
   bc017:	shl    $0x15,%ecx
   bc01a:	or     %ecx,%r15d
   bc01d:	test   %sil,%sil
   bc020:	js     bc07e <meshoptimizer_rs::codec::raw+0xa0e>
   bc022:	add    $0x4,%r14
   bc026:	mov    %r14,%rax
   bc029:	jmp    bc092 <meshoptimizer_rs::codec::raw+0xa22>
   bc02b:	movzbl 0x4(%r14,%rsi,1),%ecx
   bc031:	add    $0x5,%rsi
   bc035:	shl    $0x1c,%ecx
   bc038:	or     %ebx,%ecx
   bc03a:	mov    %ecx,%ebx
   bc03c:	mov    0x20(%rsp),%ebp
   bc040:	mov    %ebx,%ecx
   bc042:	shr    $1,%ecx
   bc044:	and    $0x1,%ebx
   bc047:	neg    %ebx
   bc049:	xor    %ecx,%ebx
   bc04b:	add    %ebp,%ebx
   bc04d:	mov    %ebx,0xf0(%rsp,%r10,4)
   bc055:	mov    $0x1,%r14d
   bc05b:	mov    %ebx,0x20(%rsp)
   bc05f:	jmp    bbc3e <meshoptimizer_rs::codec::raw+0x5ce>
   bc064:	add    $0x6,%rsi
   bc068:	mov    0x30(%rsp),%rax
   bc06d:	movzbl 0x4(%rax,%r14,1),%eax
   bc073:	shl    $0x1c,%eax
   bc076:	or     %r11d,%eax
   bc079:	jmp    bbe9b <meshoptimizer_rs::codec::raw+0x82b>
   bc07e:	mov    0x30(%rsp),%rcx
   bc083:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc089:	shl    $0x1c,%ecx
   bc08c:	or     %r15d,%ecx
   bc08f:	mov    %ecx,%r15d
   bc092:	mov    0x28(%rsp),%r9
   bc097:	mov    %r15d,%ecx
   bc09a:	shr    $1,%ecx
   bc09c:	and    $0x1,%r15d
   bc0a0:	neg    %r15d
   bc0a3:	xor    %ecx,%r15d
   bc0a6:	add    0x20(%rsp),%r15d
   bc0ab:	mov    %rax,%r14
   bc0ae:	mov    %r15d,0x20(%rsp)
   bc0b3:	cmp    $0xf,%r13b
   bc0b7:	jne    bc17b <meshoptimizer_rs::codec::raw+0xb0b>
   bc0bd:	cmp    $0xfffffffffffffffa,%r14
   bc0c1:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc0c7:	lea    0x5(%r14),%rax
   bc0cb:	cmp    0x630(%rsp),%rax
   bc0d3:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc0d9:	mov    0x30(%rsp),%r9
   bc0de:	movzbl (%r9,%r14,1),%esi
   bc0e3:	test   %sil,%sil
   bc0e6:	js     bc0f2 <meshoptimizer_rs::codec::raw+0xa82>
   bc0e8:	inc    %r14
   bc0eb:	mov    %r14,%rax
   bc0ee:	mov    %esi,%ebx
   bc0f0:	jmp    bc160 <meshoptimizer_rs::codec::raw+0xaf0>
   bc0f2:	and    $0x7f,%esi
   bc0f5:	movzbl 0x1(%r9,%r14,1),%ebx
   bc0fb:	mov    %ebx,%ecx
   bc0fd:	and    $0x7f,%ebx
   bc100:	shl    $0x7,%ebx
   bc103:	or     %esi,%ebx
   bc105:	test   %cl,%cl
   bc107:	js     bc112 <meshoptimizer_rs::codec::raw+0xaa2>
   bc109:	add    $0x2,%r14
   bc10d:	mov    %r14,%rax
   bc110:	jmp    bc160 <meshoptimizer_rs::codec::raw+0xaf0>
   bc112:	movzbl 0x2(%r9,%r14,1),%ecx
   bc118:	mov    %ecx,%esi
   bc11a:	and    $0x7f,%ecx
   bc11d:	shl    $0xe,%ecx
   bc120:	or     %ecx,%ebx
   bc122:	test   %sil,%sil
   bc125:	js     bc130 <meshoptimizer_rs::codec::raw+0xac0>
   bc127:	add    $0x3,%r14
   bc12b:	mov    %r14,%rax
   bc12e:	jmp    bc160 <meshoptimizer_rs::codec::raw+0xaf0>
   bc130:	movzbl 0x3(%r9,%r14,1),%ecx
   bc136:	mov    %ecx,%esi
   bc138:	and    $0x7f,%ecx
   bc13b:	shl    $0x15,%ecx
   bc13e:	or     %ecx,%ebx
   bc140:	test   %sil,%sil
   bc143:	js     bc14e <meshoptimizer_rs::codec::raw+0xade>
   bc145:	add    $0x4,%r14
   bc149:	mov    %r14,%rax
   bc14c:	jmp    bc160 <meshoptimizer_rs::codec::raw+0xaf0>
   bc14e:	mov    0x30(%rsp),%rcx
   bc153:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc159:	shl    $0x1c,%ecx
   bc15c:	or     %ebx,%ecx
   bc15e:	mov    %ecx,%ebx
   bc160:	mov    0x28(%rsp),%r9
   bc165:	mov    %ebx,%ecx
   bc167:	shr    $1,%ecx
   bc169:	and    $0x1,%ebx
   bc16c:	neg    %ebx
   bc16e:	xor    %ecx,%ebx
   bc170:	add    0x20(%rsp),%ebx
   bc174:	mov    %rax,%r14
   bc177:	mov    %ebx,0x20(%rsp)
   bc17b:	cmp    $0xf,%r13b
   bc17f:	sete   %al
   bc182:	cmp    $0xf,%r12b
   bc186:	sete   %cl
   bc189:	test   %r13b,%r13b
   bc18c:	sete   %sil
   bc190:	test   %r12b,%r12b
   bc193:	sete   %r12b
   bc197:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc19f:	lea    0x1(%r10),%r13
   bc1a3:	and    $0xf,%r13d
   bc1a7:	or     %cl,%r12b
   bc1aa:	mov    %r15d,0xf0(%rsp,%r13,4)
   bc1b2:	movzbl %r12b,%ecx
   bc1b6:	add    %r10,%rcx
   bc1b9:	inc    %rcx
   bc1bc:	or     %al,%sil
   bc1bf:	movzbl %sil,%r10d
   bc1c3:	add    %rcx,%r10
   bc1c6:	and    $0xf,%ecx
   bc1c9:	mov    %ebx,0xf0(%rsp,%rcx,4)
   bc1d0:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc1d8:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc1e0:	lea    0x1(%r8),%r13d
   bc1e4:	and    $0xf,%r13d
   bc1e8:	mov    $0x3,%eax
   bc1ed:	mov    $0x2,%r12d
   bc1f3:	mov    %r14,%rsi
   bc1f6:	jmp    bbc6d <meshoptimizer_rs::codec::raw+0x5fd>
   bc1fb:	mov    %bpl,%r14b
   bc1fe:	add    %r14d,%r11d
   bc201:	mov    %ebx,0xf0(%rsp,%r10,4)
   bc209:	mov    %r11d,%ebp
   bc20c:	not    %eax
   bc20e:	shr    $0x4,%eax
   bc211:	add    %r8d,%eax
   bc214:	and    $0xf,%eax
   bc217:	mov    0x1f0(%rsp,%rax,8),%r11d
   bc21f:	mov    0x1f4(%rsp,%rax,8),%r15d
   bc227:	add    %r14,%r10
   bc22a:	mov    $0x2,%eax
   bc22f:	mov    $0x1,%r12d
   bc235:	mov    %r8,%r13
   bc238:	inc    %rdi
   bc23b:	mov    %ebx,0x1f0(%rsp,%r13,8)
   bc243:	mov    %r15d,0x1f4(%rsp,%r13,8)
   bc24b:	add    %r8d,%r12d
   bc24e:	and    $0xf,%r12d
   bc252:	mov    %r11d,0x1f0(%rsp,%r12,8)
   bc25a:	mov    %ebx,0x1f4(%rsp,%r12,8)
   bc262:	add    %r8d,%eax
   bc265:	and    $0xf,%eax
   bc268:	and    $0xf,%r10d
   bc26c:	mov    %r11d,-0x8(%rdx)
   bc270:	mov    %r15d,-0x4(%rdx)
   bc274:	mov    %rdx,%r15
   bc277:	mov    %ebx,(%rdx)
   bc279:	add    $0xc,%r15
   bc27d:	mov    %rax,%r8
   bc280:	mov    %ebp,%r11d
   bc283:	cmp    %rdi,0x38(%rsp)
   bc288:	je     bd1c4 <meshoptimizer_rs::codec::raw+0x1b54>
   bc28e:	cmp    %r9,%rsi
   bc291:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc297:	mov    %r15,%rdx
   bc29a:	mov    0x30(%rsp),%r14
   bc29f:	movzbl 0x1(%r14,%rdi,1),%eax
   bc2a5:	cmp    $0xf0,%al
   bc2a7:	jae    bc2e1 <meshoptimizer_rs::codec::raw+0xc71>
   bc2a9:	mov    %eax,%r15d
   bc2ac:	and    $0xf,%r15d
   bc2b0:	cmp    %r15d,0x58(%rsp)
   bc2b5:	jbe    bc31b <meshoptimizer_rs::codec::raw+0xcab>
   bc2b7:	xor    %r14d,%r14d
   bc2ba:	mov    %r11d,%ebx
   bc2bd:	test   %r15b,%r15b
   bc2c0:	sete   %bpl
   bc2c4:	je     bc1fb <meshoptimizer_rs::codec::raw+0xb8b>
   bc2ca:	not    %r15d
   bc2cd:	add    %r10d,%r15d
   bc2d0:	and    $0xf,%r15d
   bc2d4:	mov    0xf0(%rsp,%r15,4),%ebx
   bc2dc:	jmp    bc1fb <meshoptimizer_rs::codec::raw+0xb8b>
   bc2e1:	cmp    $0xfe,%al
   bc2e3:	jae    bc33d <meshoptimizer_rs::codec::raw+0xccd>
   bc2e5:	and    $0xf,%eax
   bc2e8:	mov    0x50(%rsp),%rcx
   bc2ed:	movzbl (%rcx,%rax,1),%r13d
   bc2f2:	lea    0x1(%r11),%r15d
   bc2f6:	xor    %eax,%eax
   bc2f8:	mov    %r13d,%ecx
   bc2fb:	shr    $0x4,%ecx
   bc2fe:	sete   %r12b
   bc302:	je     bc380 <meshoptimizer_rs::codec::raw+0xd10>
   bc304:	mov    %r10d,%ebx
   bc307:	sub    %ecx,%ebx
   bc309:	and    $0xf,%ebx
   bc30c:	mov    %r15d,%ebp
   bc30f:	mov    0xf0(%rsp,%rbx,4),%ecx
   bc316:	mov    %ecx,%r15d
   bc319:	jmp    bc384 <meshoptimizer_rs::codec::raw+0xd14>
   bc31b:	cmp    $0xf,%r15b
   bc31f:	jne    bc400 <meshoptimizer_rs::codec::raw+0xd90>
   bc325:	movzbl (%r14,%rsi,1),%ebp
   bc32a:	test   %bpl,%bpl
   bc32d:	js     bc46e <meshoptimizer_rs::codec::raw+0xdfe>
   bc333:	inc    %rsi
   bc336:	mov    %ebp,%ebx
   bc338:	jmp    bc604 <meshoptimizer_rs::codec::raw+0xf94>
   bc33d:	movzbl (%r14,%rsi,1),%r14d
   bc342:	xor    %r15d,%r15d
   bc345:	cmp    $0xfe,%eax
   bc34a:	sete   %r15b
   bc34e:	test   %r14d,%r14d
   bc351:	cmove  %r14d,%r11d
   bc355:	add    %r11d,%r15d
   bc358:	mov    %r14d,%r12d
   bc35b:	shr    $0x4,%r12d
   bc35f:	je     bc41b <meshoptimizer_rs::codec::raw+0xdab>
   bc365:	mov    %r10d,%ecx
   bc368:	sub    %r12d,%ecx
   bc36b:	and    $0xf,%ecx
   bc36e:	mov    %r15d,%ebx
   bc371:	mov    0xf0(%rsp,%rcx,4),%ecx
   bc378:	mov    %ecx,%r15d
   bc37b:	jmp    bc41f <meshoptimizer_rs::codec::raw+0xdaf>
   bc380:	lea    0x2(%r11),%ebp
   bc384:	xor    %r14d,%r14d
   bc387:	mov    %ebp,%ebx
   bc389:	test   $0xf,%r13b
   bc38d:	sete   %cl
   bc390:	je     bc3a2 <meshoptimizer_rs::codec::raw+0xd32>
   bc392:	mov    %r10d,%ebx
   bc395:	sub    %r13d,%ebx
   bc398:	and    $0xf,%ebx
   bc39b:	mov    0xf0(%rsp,%rbx,4),%ebx
   bc3a2:	mov    %cl,%r14b
   bc3a5:	add    %r14d,%ebp
   bc3a8:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc3b0:	lea    0x1(%r10),%rcx
   bc3b4:	and    $0xf,%ecx
   bc3b7:	mov    %r15d,0xf0(%rsp,%rcx,4)
   bc3bf:	mov    %r12b,%al
   bc3c2:	add    %r10,%rax
   bc3c5:	inc    %rax
   bc3c8:	add    %rax,%r14
   bc3cb:	and    $0xf,%eax
   bc3ce:	mov    %ebx,0xf0(%rsp,%rax,4)
   bc3d5:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc3dd:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc3e5:	lea    0x1(%r8),%r13d
   bc3e9:	and    $0xf,%r13d
   bc3ed:	mov    $0x3,%eax
   bc3f2:	mov    $0x2,%r12d
   bc3f8:	mov    %r14,%r10
   bc3fb:	jmp    bc238 <meshoptimizer_rs::codec::raw+0xbc8>
   bc400:	movzbl %r15b,%ecx
   bc404:	xor    $0x3,%r15b
   bc408:	movzbl %r15b,%ebx
   bc40c:	mov    0x20(%rsp),%ebp
   bc410:	add    %ecx,%ebp
   bc412:	sub    %ebx,%ebp
   bc414:	mov    %ebp,%ebx
   bc416:	jmp    bc615 <meshoptimizer_rs::codec::raw+0xfa5>
   bc41b:	lea    0x1(%r15),%ebx
   bc41f:	mov    %r14d,%r13d
   bc422:	and    $0xf,%r13b
   bc426:	je     bc43e <meshoptimizer_rs::codec::raw+0xdce>
   bc428:	mov    %r10d,%ecx
   bc42b:	sub    %r14d,%ecx
   bc42e:	and    $0xf,%ecx
   bc431:	mov    %ebx,%ebp
   bc433:	mov    0xf0(%rsp,%rcx,4),%ecx
   bc43a:	mov    %ecx,%ebx
   bc43c:	jmp    bc441 <meshoptimizer_rs::codec::raw+0xdd1>
   bc43e:	lea    0x1(%rbx),%ebp
   bc441:	lea    0x1(%rsi),%r14
   bc445:	cmp    $0xfe,%eax
   bc44a:	je     bc548 <meshoptimizer_rs::codec::raw+0xed8>
   bc450:	mov    0x30(%rsp),%r9
   bc455:	movzbl 0x1(%r9,%rsi,1),%eax
   bc45b:	test   %al,%al
   bc45d:	js     bc48e <meshoptimizer_rs::codec::raw+0xe1e>
   bc45f:	add    $0x2,%rsi
   bc463:	mov    %rsi,%r14
   bc466:	mov    %eax,%r11d
   bc469:	jmp    bc52a <meshoptimizer_rs::codec::raw+0xeba>
   bc46e:	and    $0x7f,%ebp
   bc471:	movzbl 0x1(%r14,%rsi,1),%ebx
   bc477:	mov    %ebx,%ecx
   bc479:	and    $0x7f,%ebx
   bc47c:	shl    $0x7,%ebx
   bc47f:	or     %ebp,%ebx
   bc481:	test   %cl,%cl
   bc483:	js     bc4af <meshoptimizer_rs::codec::raw+0xe3f>
   bc485:	add    $0x2,%rsi
   bc489:	jmp    bc604 <meshoptimizer_rs::codec::raw+0xf94>
   bc48e:	and    $0x7f,%eax
   bc491:	movzbl 0x2(%r9,%rsi,1),%r11d
   bc497:	mov    %r11d,%ecx
   bc49a:	and    $0x7f,%r11d
   bc49e:	shl    $0x7,%r11d
   bc4a2:	or     %eax,%r11d
   bc4a5:	test   %cl,%cl
   bc4a7:	js     bc4cd <meshoptimizer_rs::codec::raw+0xe5d>
   bc4a9:	add    $0x3,%rsi
   bc4ad:	jmp    bc527 <meshoptimizer_rs::codec::raw+0xeb7>
   bc4af:	movzbl 0x2(%r14,%rsi,1),%ecx
   bc4b5:	mov    %ecx,%ebp
   bc4b7:	and    $0x7f,%ecx
   bc4ba:	shl    $0xe,%ecx
   bc4bd:	or     %ecx,%ebx
   bc4bf:	test   %bpl,%bpl
   bc4c2:	js     bc4e8 <meshoptimizer_rs::codec::raw+0xe78>
   bc4c4:	add    $0x3,%rsi
   bc4c8:	jmp    bc604 <meshoptimizer_rs::codec::raw+0xf94>
   bc4cd:	movzbl 0x3(%r9,%rsi,1),%eax
   bc4d3:	mov    %eax,%ecx
   bc4d5:	and    $0x7f,%eax
   bc4d8:	shl    $0xe,%eax
   bc4db:	or     %eax,%r11d
   bc4de:	test   %cl,%cl
   bc4e0:	js     bc50a <meshoptimizer_rs::codec::raw+0xe9a>
   bc4e2:	add    $0x4,%rsi
   bc4e6:	jmp    bc527 <meshoptimizer_rs::codec::raw+0xeb7>
   bc4e8:	movzbl 0x3(%r14,%rsi,1),%ecx
   bc4ee:	mov    %ecx,%ebp
   bc4f0:	and    $0x7f,%ecx
   bc4f3:	shl    $0x15,%ecx
   bc4f6:	or     %ecx,%ebx
   bc4f8:	test   %bpl,%bpl
   bc4fb:	js     bc5f3 <meshoptimizer_rs::codec::raw+0xf83>
   bc501:	add    $0x4,%rsi
   bc505:	jmp    bc604 <meshoptimizer_rs::codec::raw+0xf94>
   bc50a:	movzbl 0x4(%r9,%rsi,1),%eax
   bc510:	mov    %eax,%ecx
   bc512:	and    $0x7f,%eax
   bc515:	shl    $0x15,%eax
   bc518:	or     %eax,%r11d
   bc51b:	test   %cl,%cl
   bc51d:	js     bc62c <meshoptimizer_rs::codec::raw+0xfbc>
   bc523:	add    $0x5,%rsi
   bc527:	mov    %rsi,%r14
   bc52a:	mov    0x28(%rsp),%r9
   bc52f:	mov    %r11d,%eax
   bc532:	shr    $1,%eax
   bc534:	and    $0x1,%r11d
   bc538:	neg    %r11d
   bc53b:	xor    %eax,%r11d
   bc53e:	add    0x20(%rsp),%r11d
   bc543:	mov    %r11d,0x20(%rsp)
   bc548:	cmp    $0xf,%r12b
   bc54c:	jne    bc67b <meshoptimizer_rs::codec::raw+0x100b>
   bc552:	cmp    $0xfffffffffffffffa,%r14
   bc556:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc55c:	lea    0x5(%r14),%rax
   bc560:	cmp    0x630(%rsp),%rax
   bc568:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc56e:	mov    0x30(%rsp),%r9
   bc573:	movzbl (%r9,%r14,1),%esi
   bc578:	test   %sil,%sil
   bc57b:	js     bc58b <meshoptimizer_rs::codec::raw+0xf1b>
   bc57d:	inc    %r14
   bc580:	mov    %r14,%rax
   bc583:	mov    %esi,%r15d
   bc586:	jmp    bc65a <meshoptimizer_rs::codec::raw+0xfea>
   bc58b:	and    $0x7f,%esi
   bc58e:	movzbl 0x1(%r9,%r14,1),%r15d
   bc594:	mov    %r15d,%ecx
   bc597:	and    $0x7f,%r15d
   bc59b:	shl    $0x7,%r15d
   bc59f:	or     %esi,%r15d
   bc5a2:	test   %cl,%cl
   bc5a4:	js     bc5b2 <meshoptimizer_rs::codec::raw+0xf42>
   bc5a6:	add    $0x2,%r14
   bc5aa:	mov    %r14,%rax
   bc5ad:	jmp    bc65a <meshoptimizer_rs::codec::raw+0xfea>
   bc5b2:	movzbl 0x2(%r9,%r14,1),%ecx
   bc5b8:	mov    %ecx,%esi
   bc5ba:	and    $0x7f,%ecx
   bc5bd:	shl    $0xe,%ecx
   bc5c0:	or     %ecx,%r15d
   bc5c3:	test   %sil,%sil
   bc5c6:	js     bc5d4 <meshoptimizer_rs::codec::raw+0xf64>
   bc5c8:	add    $0x3,%r14
   bc5cc:	mov    %r14,%rax
   bc5cf:	jmp    bc65a <meshoptimizer_rs::codec::raw+0xfea>
   bc5d4:	movzbl 0x3(%r9,%r14,1),%ecx
   bc5da:	mov    %ecx,%esi
   bc5dc:	and    $0x7f,%ecx
   bc5df:	shl    $0x15,%ecx
   bc5e2:	or     %ecx,%r15d
   bc5e5:	test   %sil,%sil
   bc5e8:	js     bc646 <meshoptimizer_rs::codec::raw+0xfd6>
   bc5ea:	add    $0x4,%r14
   bc5ee:	mov    %r14,%rax
   bc5f1:	jmp    bc65a <meshoptimizer_rs::codec::raw+0xfea>
   bc5f3:	movzbl 0x4(%r14,%rsi,1),%ecx
   bc5f9:	add    $0x5,%rsi
   bc5fd:	shl    $0x1c,%ecx
   bc600:	or     %ebx,%ecx
   bc602:	mov    %ecx,%ebx
   bc604:	mov    0x20(%rsp),%ebp
   bc608:	mov    %ebx,%ecx
   bc60a:	shr    $1,%ecx
   bc60c:	and    $0x1,%ebx
   bc60f:	neg    %ebx
   bc611:	xor    %ecx,%ebx
   bc613:	add    %ebp,%ebx
   bc615:	mov    %ebx,0xf0(%rsp,%r10,4)
   bc61d:	mov    $0x1,%r14d
   bc623:	mov    %ebx,0x20(%rsp)
   bc627:	jmp    bc209 <meshoptimizer_rs::codec::raw+0xb99>
   bc62c:	add    $0x6,%rsi
   bc630:	mov    0x30(%rsp),%rax
   bc635:	movzbl 0x4(%rax,%r14,1),%eax
   bc63b:	shl    $0x1c,%eax
   bc63e:	or     %r11d,%eax
   bc641:	jmp    bc463 <meshoptimizer_rs::codec::raw+0xdf3>
   bc646:	mov    0x30(%rsp),%rcx
   bc64b:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc651:	shl    $0x1c,%ecx
   bc654:	or     %r15d,%ecx
   bc657:	mov    %ecx,%r15d
   bc65a:	mov    0x28(%rsp),%r9
   bc65f:	mov    %r15d,%ecx
   bc662:	shr    $1,%ecx
   bc664:	and    $0x1,%r15d
   bc668:	neg    %r15d
   bc66b:	xor    %ecx,%r15d
   bc66e:	add    0x20(%rsp),%r15d
   bc673:	mov    %rax,%r14
   bc676:	mov    %r15d,0x20(%rsp)
   bc67b:	cmp    $0xf,%r13b
   bc67f:	jne    bc743 <meshoptimizer_rs::codec::raw+0x10d3>
   bc685:	cmp    $0xfffffffffffffffa,%r14
   bc689:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc68f:	lea    0x5(%r14),%rax
   bc693:	cmp    0x630(%rsp),%rax
   bc69b:	ja     bd1cb <meshoptimizer_rs::codec::raw+0x1b5b>
   bc6a1:	mov    0x30(%rsp),%r9
   bc6a6:	movzbl (%r9,%r14,1),%esi
   bc6ab:	test   %sil,%sil
   bc6ae:	js     bc6ba <meshoptimizer_rs::codec::raw+0x104a>
   bc6b0:	inc    %r14
   bc6b3:	mov    %r14,%rax
   bc6b6:	mov    %esi,%ebx
   bc6b8:	jmp    bc728 <meshoptimizer_rs::codec::raw+0x10b8>
   bc6ba:	and    $0x7f,%esi
   bc6bd:	movzbl 0x1(%r9,%r14,1),%ebx
   bc6c3:	mov    %ebx,%ecx
   bc6c5:	and    $0x7f,%ebx
   bc6c8:	shl    $0x7,%ebx
   bc6cb:	or     %esi,%ebx
   bc6cd:	test   %cl,%cl
   bc6cf:	js     bc6da <meshoptimizer_rs::codec::raw+0x106a>
   bc6d1:	add    $0x2,%r14
   bc6d5:	mov    %r14,%rax
   bc6d8:	jmp    bc728 <meshoptimizer_rs::codec::raw+0x10b8>
   bc6da:	movzbl 0x2(%r9,%r14,1),%ecx
   bc6e0:	mov    %ecx,%esi
   bc6e2:	and    $0x7f,%ecx
   bc6e5:	shl    $0xe,%ecx
   bc6e8:	or     %ecx,%ebx
   bc6ea:	test   %sil,%sil
   bc6ed:	js     bc6f8 <meshoptimizer_rs::codec::raw+0x1088>
   bc6ef:	add    $0x3,%r14
   bc6f3:	mov    %r14,%rax
   bc6f6:	jmp    bc728 <meshoptimizer_rs::codec::raw+0x10b8>
   bc6f8:	movzbl 0x3(%r9,%r14,1),%ecx
   bc6fe:	mov    %ecx,%esi
   bc700:	and    $0x7f,%ecx
   bc703:	shl    $0x15,%ecx
   bc706:	or     %ecx,%ebx
   bc708:	test   %sil,%sil
   bc70b:	js     bc716 <meshoptimizer_rs::codec::raw+0x10a6>
   bc70d:	add    $0x4,%r14
   bc711:	mov    %r14,%rax
   bc714:	jmp    bc728 <meshoptimizer_rs::codec::raw+0x10b8>
   bc716:	mov    0x30(%rsp),%rcx
   bc71b:	movzbl 0x4(%rcx,%r14,1),%ecx
   bc721:	shl    $0x1c,%ecx
   bc724:	or     %ebx,%ecx
   bc726:	mov    %ecx,%ebx
   bc728:	mov    0x28(%rsp),%r9
   bc72d:	mov    %ebx,%ecx
   bc72f:	shr    $1,%ecx
   bc731:	and    $0x1,%ebx
   bc734:	neg    %ebx
   bc736:	xor    %ecx,%ebx
   bc738:	add    0x20(%rsp),%ebx
   bc73c:	mov    %rax,%r14
   bc73f:	mov    %ebx,0x20(%rsp)
   bc743:	cmp    $0xf,%r13b
   bc747:	sete   %al
   bc74a:	cmp    $0xf,%r12b
   bc74e:	sete   %cl
   bc751:	test   %r13b,%r13b
   bc754:	sete   %sil
   bc758:	test   %r12b,%r12b
   bc75b:	sete   %r12b
   bc75f:	mov    %r11d,0xf0(%rsp,%r10,4)
   bc767:	lea    0x1(%r10),%r13
   bc76b:	and    $0xf,%r13d
   bc76f:	or     %cl,%r12b
   bc772:	mov    %r15d,0xf0(%rsp,%r13,4)
   bc77a:	movzbl %r12b,%ecx
   bc77e:	add    %r10,%rcx
   bc781:	inc    %rcx
   bc784:	or     %al,%sil
   bc787:	movzbl %sil,%r10d
   bc78b:	add    %rcx,%r10
   bc78e:	and    $0xf,%ecx
   bc791:	mov    %ebx,0xf0(%rsp,%rcx,4)
   bc798:	mov    %r15d,0x1f0(%rsp,%r8,8)
   bc7a0:	mov    %r11d,0x1f4(%rsp,%r8,8)
   bc7a8:	lea    0x1(%r8),%r13d
   bc7ac:	and    $0xf,%r13d
   bc7b0:	mov    $0x3,%eax
   bc7b5:	mov    $0x2,%r12d
   bc7bb:	mov    %r14,%rsi
   bc7be:	jmp    bc238 <meshoptimizer_rs::codec::raw+0xbc8>
   bc7c3:	cmp    $0x21,%rbp
   bc7c7:	mov    $0x20,%eax
   bc7cc:	cmovae %rbp,%rax
   bc7d0:	mov    %rbp,%rcx
   bc7d3:	mov    %rax,0xe0(%rsp)
   bc7db:	lea    0x1(%rax),%rdx
   bc7df:	mov    $0x7,%al
   bc7e1:	cmp    %rdx,%rbx
   bc7e4:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bc7ea:	mov    %rbx,%rsi
   bc7ed:	sub    %rcx,%rsi
   bc7f0:	lea    (%rsi,%rbp,1),%rax
   bc7f4:	mov    %rbx,%rcx
   bc7f7:	mov    %rax,0x60(%rsp)
   bc7fc:	sub    %rax,%rcx
   bc7ff:	mov    %rcx,0xa0(%rsp)
   bc807:	jb     bd24c <meshoptimizer_rs::codec::raw+0x1bdc>
   bc80d:	pxor   %xmm0,%xmm0
   bc811:	movdqa %xmm0,0x1e0(%rsp)
   bc81a:	movdqa %xmm0,0x1d0(%rsp)
   bc823:	movdqa %xmm0,0x1c0(%rsp)
   bc82c:	movdqa %xmm0,0x1b0(%rsp)
   bc835:	movdqa %xmm0,0x1a0(%rsp)
   bc83e:	movdqa %xmm0,0x190(%rsp)
   bc847:	movdqa %xmm0,0x180(%rsp)
   bc850:	movdqa %xmm0,0x170(%rsp)
   bc859:	movdqa %xmm0,0x160(%rsp)
   bc862:	movdqa %xmm0,0x150(%rsp)
   bc86b:	movdqa %xmm0,0x140(%rsp)
   bc874:	movdqa %xmm0,0x130(%rsp)
   bc87d:	movdqa %xmm0,0x120(%rsp)
   bc886:	movdqa %xmm0,0x110(%rsp)
   bc88f:	movdqa %xmm0,0x100(%rsp)
   bc898:	movdqa %xmm0,0xf0(%rsp)
   bc8a1:	cmp    $0x101,%rbp
   bc8a8:	jae    bd20d <meshoptimizer_rs::codec::raw+0x1b9d>
   bc8ae:	mov    %dil,0x38(%rsp)
   bc8b3:	lea    (%rsi,%rbp,1),%rax
   bc8b7:	cmp    %rsi,%rax
   bc8ba:	jb     bd224 <meshoptimizer_rs::codec::raw+0x1bb4>
   bc8c0:	add    %r14,%rsi
   bc8c3:	lea    0xf0(%rsp),%rdi
   bc8cb:	mov    %rbp,%rdx
   bc8ce:	call   *0x8c6ac(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bc8d4:	test   %rbp,%rbp
   bc8d7:	je     bd2b5 <meshoptimizer_rs::codec::raw+0x1c45>
   bc8dd:	mov    $0x2000,%ax
   bc8e1:	xor    %edx,%edx
   bc8e3:	div    %bp
   bc8e6:	and    $0x3ff0,%eax
   bc8eb:	cmp    $0x100,%eax
   bc8f0:	mov    $0x100,%ecx
   bc8f5:	cmovb  %eax,%ecx
   bc8f8:	mov    %rcx,0xd8(%rsp)
   bc900:	lea    0x1f0(%rsp),%rdi
   bc908:	mov    $0x400,%edx
   bc90d:	xor    %esi,%esi
   bc90f:	call   *0x8c67b(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   bc915:	test   %r13,%r13
   bc918:	je     bd1a0 <meshoptimizer_rs::codec::raw+0x1b30>
   bc91e:	mov    %r15,0xd0(%rsp)
   bc926:	add    %r14,0x60(%rsp)
   bc92b:	mov    %rbp,%rax
   bc92e:	shr    $0x2,%rax
   bc932:	mov    %rax,0x98(%rsp)
   bc93a:	lea    0x3(%rbp),%rax
   bc93e:	shr    $0x2,%rax
   bc942:	mov    %rax,0xe8(%rsp)
   bc94a:	mov    $0x1,%r15d
   bc950:	xor    %eax,%eax
   bc952:	mov    %r12,0x88(%rsp)
   bc95a:	mov    %r14,0x30(%rsp)
   bc95f:	mov    %r13,%rcx
   bc962:	mov    %rax,%rdi
   bc965:	sub    %rax,%rcx
   bc968:	mov    0xd8(%rsp),%rax
   bc970:	cmp    %rax,%rcx
   bc973:	cmovae %rax,%rcx
   bc977:	mov    %rcx,0x20(%rsp)
   bc97c:	movzbl 0x38(%rsp),%edx
   bc981:	test   %dl,%dl
   bc983:	je     bc9c9 <meshoptimizer_rs::codec::raw+0x1359>
   bc985:	mov    %r15,%rcx
   bc988:	mov    $0x7,%al
   bc98a:	add    0x98(%rsp),%rcx
   bc992:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bc998:	cmp    %rbx,%rcx
   bc99b:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bc9a1:	add    %r14,%r15
   bc9a4:	mov    0x98(%rsp),%rax
   bc9ac:	mov    %rax,0x90(%rsp)
   bc9b4:	mov    %r15,0xa8(%rsp)
   bc9bc:	mov    %rcx,%r15
   bc9bf:	mov    0x88(%rsp),%r12
   bc9c7:	jmp    bc9e2 <meshoptimizer_rs::codec::raw+0x1372>
   bc9c9:	movq   $0x0,0x90(%rsp)
   bc9d5:	mov    $0x1,%eax
   bc9da:	mov    %rax,0xa8(%rsp)
   bc9e2:	mov    0x20(%rsp),%rax
   bc9e7:	lea    (%rax,%rdi,1),%rsi
   bc9eb:	imul   %rbp,%rdi
   bc9ef:	mov    %rsi,0xc8(%rsp)
   bc9f7:	imul   %rbp,%rsi
   bc9fb:	cmp    %rdi,%rsi
   bc9fe:	jb     bd23c <meshoptimizer_rs::codec::raw+0x1bcc>
   bca04:	cmp    %r12,%rsi
   bca07:	ja     bd23c <meshoptimizer_rs::codec::raw+0x1bcc>
   bca0d:	mov    0x20(%rsp),%rax
   bca12:	lea    0xf(%rax),%ecx
   bca15:	and    $0x3f0,%ecx
   bca1b:	mov    %rcx,0x28(%rsp)
   bca20:	mov    %rax,%rcx
   bca23:	imul   %rbp,%rcx
   bca27:	mov    %rcx,0xb8(%rsp)
   bca2f:	add    0xd0(%rsp),%rdi
   bca37:	mov    %rdi,0xb0(%rsp)
   bca3f:	lea    -0x1(%rax),%rcx
   bca43:	imul   %rbp,%rcx
   bca47:	mov    %rcx,0xc0(%rsp)
   bca4f:	add    $0x4,%rcx
   bca53:	mov    %rcx,0x50(%rsp)
   bca58:	lea    0x0(,%rax,4),%rcx
   bca60:	mov    %rcx,0x68(%rsp)
   bca65:	lea    0x1f0(%rsp,%rax,1),%rcx
   bca6d:	mov    %rcx,0x80(%rsp)
   bca75:	lea    0x1f0(%rsp,%rax,2),%rcx
   bca7d:	mov    %rcx,0x78(%rsp)
   bca82:	lea    (%rax,%rax,2),%rax
   bca86:	add    %rsp,%rax
   bca89:	add    $0x1f0,%rax
   bca8f:	mov    %rax,0x70(%rsp)
   bca94:	xor    %r14d,%r14d
   bca97:	xor    %ebx,%ebx
   bca99:	jmp    bcafa <meshoptimizer_rs::codec::raw+0x148a>
   bca9b:	lea    (%rsp,%rbx,1),%rcx
   bca9f:	add    $0xf0,%rcx
   bcaa6:	shr    $0x4,%al
   bcaa9:	neg    %al
   bcaab:	and    $0x1f,%al
   bcaad:	movzbl %al,%eax
   bcab0:	mov    %rcx,(%rsp)
   bcab4:	mov    %eax,0x8(%rsp)
   bcab8:	movq   $0x0,0x10(%rsp)
   bcac1:	lea    0x1f0(%rsp),%rdi
   bcac9:	mov    0x68(%rsp),%rsi
   bcace:	mov    0x50(%rsp),%rcx
   bcad3:	mov    0x20(%rsp),%r8
   bcad8:	mov    %rbp,%r9
   bcadb:	call   7cfe0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   bcae0:	inc    %r14
   bcae3:	add    $0x4,%rbx
   bcae7:	cmp    %r14,0xe8(%rsp)
   bcaef:	movzbl 0x38(%rsp),%edx
   bcaf4:	je     bd148 <meshoptimizer_rs::codec::raw+0x1ad8>
   bcafa:	mov    %rbx,0x58(%rsp)
   bcaff:	test   %dl,%dl
   bcb01:	je     bcb20 <meshoptimizer_rs::codec::raw+0x14b0>
   bcb03:	cmp    0x90(%rsp),%r14
   bcb0b:	jae    bd285 <meshoptimizer_rs::codec::raw+0x1c15>
   bcb11:	mov    0xa8(%rsp),%rax
   bcb19:	movzbl (%rax,%r14,1),%ebx
   bcb1e:	jmp    bcb22 <meshoptimizer_rs::codec::raw+0x14b2>
   bcb20:	xor    %ebx,%ebx
   bcb22:	mov    %ebx,%eax
   bcb24:	and    $0x3,%al
   bcb26:	movzbl %al,%ecx
   bcb29:	mov    0x28(%rsp),%rsi
   bcb2e:	cmp    %rsi,0x20(%rsp)
   bcb33:	ja     bcf93 <meshoptimizer_rs::codec::raw+0x1923>
   bcb39:	cmp    $0x2,%al
   bcb3b:	je     bcb7d <meshoptimizer_rs::codec::raw+0x150d>
   bcb3d:	cmp    $0x3,%ecx
   bcb40:	jne    bcb97 <meshoptimizer_rs::codec::raw+0x1527>
   bcb42:	mov    %r15,%r12
   bcb45:	mov    $0x7,%al
   bcb47:	add    0x20(%rsp),%r12
   bcb4c:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcb52:	cmp    0x630(%rsp),%r12
   bcb5a:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcb60:	add    0x30(%rsp),%r15
   bcb65:	lea    0x1f0(%rsp),%rdi
   bcb6d:	mov    %r15,%rsi
   bcb70:	mov    0x20(%rsp),%rdx
   bcb75:	call   *0x8c405(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bcb7b:	jmp    bcbf3 <meshoptimizer_rs::codec::raw+0x1583>
   bcb7d:	lea    0x1f0(%rsp),%rdi
   bcb85:	xor    %esi,%esi
   bcb87:	mov    0x20(%rsp),%rdx
   bcb8c:	call   *0x8c3fe(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   bcb92:	mov    %r15,%r12
   bcb95:	jmp    bcbf3 <meshoptimizer_rs::codec::raw+0x1583>
   bcb97:	test   %al,%al
   bcb99:	lea    -0xae1a0(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcba0:	lea    -0xae077(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcba7:	cmove  %rcx,%rax
   bcbab:	test   %dl,%dl
   bcbad:	lea    -0xae4d4(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcbb4:	cmove  %rcx,%rax
   bcbb8:	mov    %rax,(%rsp)
   bcbbc:	lea    0x40(%rsp),%rdi
   bcbc1:	mov    0x30(%rsp),%rsi
   bcbc6:	mov    0x630(%rsp),%rdx
   bcbce:	mov    %r15,%rcx
   bcbd1:	lea    0x1f0(%rsp),%r8
   bcbd9:	mov    0x28(%rsp),%r9
   bcbde:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bcbe3:	cmpb   $0x0,0x40(%rsp)
   bcbe8:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bcbee:	mov    0x48(%rsp),%r12
   bcbf3:	mov    %ebx,%eax
   bcbf5:	shr    $0x2,%al
   bcbf8:	and    $0x3,%al
   bcbfa:	cmp    $0x3,%al
   bcbfc:	movzbl 0x38(%rsp),%edx
   bcc01:	je     bcc25 <meshoptimizer_rs::codec::raw+0x15b5>
   bcc03:	movzbl %al,%ecx
   bcc06:	cmp    $0x2,%ecx
   bcc09:	jne    bcc63 <meshoptimizer_rs::codec::raw+0x15f3>
   bcc0b:	mov    0x80(%rsp),%rdi
   bcc13:	xor    %esi,%esi
   bcc15:	mov    0x20(%rsp),%rdx
   bcc1a:	call   *0x8c370(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   bcc20:	jmp    bccbf <meshoptimizer_rs::codec::raw+0x164f>
   bcc25:	mov    %r12,%r15
   bcc28:	mov    $0x7,%al
   bcc2a:	add    0x20(%rsp),%r15
   bcc2f:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcc35:	cmp    0x630(%rsp),%r15
   bcc3d:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcc43:	add    0x30(%rsp),%r12
   bcc48:	mov    0x80(%rsp),%rdi
   bcc50:	mov    %r12,%rsi
   bcc53:	mov    0x20(%rsp),%rdx
   bcc58:	call   *0x8c322(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bcc5e:	mov    %r15,%r12
   bcc61:	jmp    bccbf <meshoptimizer_rs::codec::raw+0x164f>
   bcc63:	test   %al,%al
   bcc65:	lea    -0xae26c(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcc6c:	lea    -0xae143(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcc73:	cmove  %rcx,%rax
   bcc77:	test   %dl,%dl
   bcc79:	lea    -0xae5a0(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcc80:	cmove  %rcx,%rax
   bcc84:	mov    %rax,(%rsp)
   bcc88:	lea    0x40(%rsp),%rdi
   bcc8d:	mov    0x30(%rsp),%rsi
   bcc92:	mov    0x630(%rsp),%rdx
   bcc9a:	mov    %r12,%rcx
   bcc9d:	mov    0x80(%rsp),%r8
   bcca5:	mov    0x28(%rsp),%r9
   bccaa:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bccaf:	cmpb   $0x0,0x40(%rsp)
   bccb4:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bccba:	mov    0x48(%rsp),%r12
   bccbf:	mov    %ebx,%eax
   bccc1:	shr    $0x4,%al
   bccc4:	and    $0x3,%al
   bccc6:	cmp    $0x3,%al
   bccc8:	je     bcd16 <meshoptimizer_rs::codec::raw+0x16a6>
   bccca:	movzbl %al,%ecx
   bcccd:	cmp    $0x2,%ecx
   bccd0:	jne    bcd92 <meshoptimizer_rs::codec::raw+0x1722>
   bccd6:	mov    0x78(%rsp),%rdi
   bccdb:	xor    %esi,%esi
   bccdd:	mov    0x20(%rsp),%rdx
   bcce2:	call   *0x8c2a8(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   bcce8:	shr    $0x6,%bl
   bcceb:	cmp    $0x3,%bl
   bccee:	je     bcd57 <meshoptimizer_rs::codec::raw+0x16e7>
   bccf0:	movzbl %bl,%eax
   bccf3:	cmp    $0x2,%eax
   bccf6:	jne    bcdff <meshoptimizer_rs::codec::raw+0x178f>
   bccfc:	mov    0x70(%rsp),%rdi
   bcd01:	xor    %esi,%esi
   bcd03:	mov    0x20(%rsp),%rdx
   bcd08:	call   *0x8c282(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   bcd0e:	mov    %r12,%r15
   bcd11:	jmp    bce5b <meshoptimizer_rs::codec::raw+0x17eb>
   bcd16:	mov    %r12,%r15
   bcd19:	mov    $0x7,%al
   bcd1b:	add    0x20(%rsp),%r15
   bcd20:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcd26:	cmp    0x630(%rsp),%r15
   bcd2e:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcd34:	add    0x30(%rsp),%r12
   bcd39:	mov    0x78(%rsp),%rdi
   bcd3e:	mov    %r12,%rsi
   bcd41:	mov    0x20(%rsp),%rdx
   bcd46:	call   *0x8c234(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bcd4c:	mov    %r15,%r12
   bcd4f:	shr    $0x6,%bl
   bcd52:	cmp    $0x3,%bl
   bcd55:	jne    bccf0 <meshoptimizer_rs::codec::raw+0x1680>
   bcd57:	mov    %r12,%r15
   bcd5a:	mov    $0x7,%al
   bcd5c:	add    0x20(%rsp),%r15
   bcd61:	jb     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcd67:	cmp    0x630(%rsp),%r15
   bcd6f:	ja     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bcd75:	add    0x30(%rsp),%r12
   bcd7a:	mov    0x70(%rsp),%rdi
   bcd7f:	mov    %r12,%rsi
   bcd82:	mov    0x20(%rsp),%rdx
   bcd87:	call   *0x8c1f3(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bcd8d:	jmp    bce5b <meshoptimizer_rs::codec::raw+0x17eb>
   bcd92:	test   %al,%al
   bcd94:	lea    -0xae39b(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcd9b:	lea    -0xae272(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcda2:	cmove  %rcx,%rax
   bcda6:	cmpb   $0x0,0x38(%rsp)
   bcdab:	lea    -0xae6d2(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcdb2:	cmove  %rcx,%rax
   bcdb6:	mov    %rax,(%rsp)
   bcdba:	lea    0x40(%rsp),%rdi
   bcdbf:	mov    0x30(%rsp),%rsi
   bcdc4:	mov    0x630(%rsp),%rdx
   bcdcc:	mov    %r12,%rcx
   bcdcf:	mov    0x78(%rsp),%r8
   bcdd4:	mov    0x28(%rsp),%r9
   bcdd9:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bcdde:	cmpb   $0x0,0x40(%rsp)
   bcde3:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bcde9:	mov    0x48(%rsp),%r12
   bcdee:	shr    $0x6,%bl
   bcdf1:	cmp    $0x3,%bl
   bcdf4:	jne    bccf0 <meshoptimizer_rs::codec::raw+0x1680>
   bcdfa:	jmp    bcd57 <meshoptimizer_rs::codec::raw+0x16e7>
   bcdff:	test   %bl,%bl
   bce01:	lea    -0xae408(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bce08:	lea    -0xae2df(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bce0f:	cmove  %rcx,%rax
   bce13:	cmpb   $0x0,0x38(%rsp)
   bce18:	lea    -0xae73f(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bce1f:	cmove  %rcx,%rax
   bce23:	mov    %rax,(%rsp)
   bce27:	lea    0x40(%rsp),%rdi
   bce2c:	mov    0x30(%rsp),%rsi
   bce31:	mov    0x630(%rsp),%rdx
   bce39:	mov    %r12,%rcx
   bce3c:	mov    0x70(%rsp),%r8
   bce41:	mov    0x28(%rsp),%r9
   bce46:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bce4b:	cmpb   $0x0,0x40(%rsp)
   bce50:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bce56:	mov    0x48(%rsp),%r15
   bce5b:	cmpb   $0x0,0x38(%rsp)
   bce60:	mov    0x88(%rsp),%r12
   bce68:	mov    0xb8(%rsp),%rsi
   bce70:	mov    0x58(%rsp),%rbx
   bce75:	je     bce9c <meshoptimizer_rs::codec::raw+0x182c>
   bce77:	cmp    0xa0(%rsp),%r14
   bce7f:	jae    bd29d <meshoptimizer_rs::codec::raw+0x1c2d>
   bce85:	mov    0x60(%rsp),%rax
   bce8a:	movzbl (%rax,%r14,1),%eax
   bce8f:	mov    %rsi,%rdx
   bce92:	sub    %rbx,%rdx
   bce95:	jae    bceaa <meshoptimizer_rs::codec::raw+0x183a>
   bce97:	jmp    bd1fa <meshoptimizer_rs::codec::raw+0x1b8a>
   bce9c:	xor    %eax,%eax
   bce9e:	mov    %rsi,%rdx
   bcea1:	sub    %rbx,%rdx
   bcea4:	jb     bd1fa <meshoptimizer_rs::codec::raw+0x1b8a>
   bceaa:	cmp    %rdx,0x50(%rsp)
   bceaf:	ja     bd1e6 <meshoptimizer_rs::codec::raw+0x1b76>
   bceb5:	mov    0xb0(%rsp),%rcx
   bcebd:	lea    (%rcx,%rbx,1),%rdx
   bcec1:	mov    %eax,%ecx
   bcec3:	and    $0x3,%cl
   bcec6:	movzbl %cl,%ecx
   bcec9:	lea    -0xab7e8(%rip),%rsi        # 116e8 <libm::math::generic::sqrt::RSQRT_TAB+0x4f6>
   bced0:	movslq (%rsi,%rcx,4),%rcx
   bced4:	add    %rsi,%rcx
   bced7:	jmp    *%rcx
   bced9:	mov    %rdx,%r12
   bcedc:	add    %rsp,%rbx
   bcedf:	add    $0xf0,%rbx
   bcee6:	call   1051e0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   bceeb:	test   %al,%al
   bceed:	je     bcf52 <meshoptimizer_rs::codec::raw+0x18e2>
   bceef:	mov    %rbx,(%rsp)
   bcef3:	lea    0x1f0(%rsp),%rdi
   bcefb:	mov    0x68(%rsp),%rsi
   bcf00:	mov    %r12,%rdx
   bcf03:	mov    0x50(%rsp),%rcx
   bcf08:	mov    0x20(%rsp),%r8
   bcf0d:	mov    %rbp,%r9
   bcf10:	call   801d0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   bcf15:	jmp    bcf81 <meshoptimizer_rs::codec::raw+0x1911>
   bcf17:	lea    (%rsp,%rbx,1),%rax
   bcf1b:	add    $0xf0,%rax
   bcf21:	mov    %rax,(%rsp)
   bcf25:	movq   $0x0,0x8(%rsp)
   bcf2e:	lea    0x1f0(%rsp),%rdi
   bcf36:	mov    0x68(%rsp),%rsi
   bcf3b:	mov    0x50(%rsp),%rcx
   bcf40:	mov    0x20(%rsp),%r8
   bcf45:	mov    %rbp,%r9
   bcf48:	call   7ce20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   bcf4d:	jmp    bcae0 <meshoptimizer_rs::codec::raw+0x1470>
   bcf52:	mov    %rbx,(%rsp)
   bcf56:	movq   $0x0,0x8(%rsp)
   bcf5f:	lea    0x1f0(%rsp),%rdi
   bcf67:	mov    0x68(%rsp),%rsi
   bcf6c:	mov    %r12,%rdx
   bcf6f:	mov    0x50(%rsp),%rcx
   bcf74:	mov    0x20(%rsp),%r8
   bcf79:	mov    %rbp,%r9
   bcf7c:	call   7cc30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   bcf81:	mov    0x88(%rsp),%r12
   bcf89:	mov    0x58(%rsp),%rbx
   bcf8e:	jmp    bcae0 <meshoptimizer_rs::codec::raw+0x1470>
   bcf93:	cmp    $0x2,%al
   bcf95:	je     bd2db <meshoptimizer_rs::codec::raw+0x1c6b>
   bcf9b:	cmp    $0x3,%ecx
   bcf9e:	je     bd2c2 <meshoptimizer_rs::codec::raw+0x1c52>
   bcfa4:	test   %al,%al
   bcfa6:	lea    -0xae5ad(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bcfad:	lea    -0xae484(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bcfb4:	cmove  %rcx,%rax
   bcfb8:	test   %dl,%dl
   bcfba:	lea    -0xae8e1(%rip),%rcx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bcfc1:	cmove  %rcx,%rax
   bcfc5:	mov    %rax,(%rsp)
   bcfc9:	lea    0x40(%rsp),%rdi
   bcfce:	mov    0x30(%rsp),%rsi
   bcfd3:	mov    0x630(%rsp),%rdx
   bcfdb:	mov    %r15,%rcx
   bcfde:	lea    0x1f0(%rsp),%r8
   bcfe6:	mov    0x28(%rsp),%r9
   bcfeb:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bcff0:	cmpb   $0x0,0x40(%rsp)
   bcff5:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bcffb:	mov    %ebx,%eax
   bcffd:	shr    $0x2,%al
   bd000:	and    $0x3,%al
   bd002:	cmp    $0x2,%al
   bd004:	movzbl 0x38(%rsp),%edx
   bd009:	je     bd2db <meshoptimizer_rs::codec::raw+0x1c6b>
   bd00f:	movzbl %al,%ecx
   bd012:	cmp    $0x3,%ecx
   bd015:	je     bd2c2 <meshoptimizer_rs::codec::raw+0x1c52>
   bd01b:	test   %al,%al
   bd01d:	lea    -0xae624(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bd024:	lea    -0xae4fb(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bd02b:	cmove  %rcx,%rax
   bd02f:	mov    0x48(%rsp),%rcx
   bd034:	test   %dl,%dl
   bd036:	lea    -0xae95d(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bd03d:	cmove  %rdx,%rax
   bd041:	mov    %rax,(%rsp)
   bd045:	lea    0x40(%rsp),%rdi
   bd04a:	mov    0x30(%rsp),%rsi
   bd04f:	mov    0x630(%rsp),%rdx
   bd057:	mov    0x80(%rsp),%r8
   bd05f:	mov    0x28(%rsp),%r9
   bd064:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bd069:	cmpb   $0x0,0x40(%rsp)
   bd06e:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bd074:	mov    %ebx,%eax
   bd076:	shr    $0x4,%al
   bd079:	and    $0x3,%al
   bd07b:	cmp    $0x2,%al
   bd07d:	movzbl 0x38(%rsp),%edx
   bd082:	je     bd2db <meshoptimizer_rs::codec::raw+0x1c6b>
   bd088:	movzbl %al,%ecx
   bd08b:	cmp    $0x3,%ecx
   bd08e:	je     bd2c2 <meshoptimizer_rs::codec::raw+0x1c52>
   bd094:	test   %al,%al
   bd096:	lea    -0xae69d(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bd09d:	lea    -0xae574(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bd0a4:	cmove  %rcx,%rax
   bd0a8:	mov    0x48(%rsp),%rcx
   bd0ad:	test   %dl,%dl
   bd0af:	lea    -0xae9d6(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bd0b6:	cmove  %rdx,%rax
   bd0ba:	mov    %rax,(%rsp)
   bd0be:	lea    0x40(%rsp),%rdi
   bd0c3:	mov    0x30(%rsp),%rsi
   bd0c8:	mov    0x630(%rsp),%rdx
   bd0d0:	mov    0x78(%rsp),%r8
   bd0d5:	mov    0x28(%rsp),%r9
   bd0da:	call   e9870 <meshoptimizer_rs::codec::vertex::bytes>
   bd0df:	cmpb   $0x0,0x40(%rsp)
   bd0e4:	jne    bd1df <meshoptimizer_rs::codec::raw+0x1b6f>
   bd0ea:	shr    $0x6,%bl
   bd0ed:	cmp    $0x2,%bl
   bd0f0:	movzbl 0x38(%rsp),%edx
   bd0f5:	je     bd2db <meshoptimizer_rs::codec::raw+0x1c6b>
   bd0fb:	movzbl %bl,%eax
   bd0fe:	cmp    $0x3,%eax
   bd101:	je     bd2c2 <meshoptimizer_rs::codec::raw+0x1c52>
   bd107:	test   %bl,%bl
   bd109:	lea    -0xae710(%rip),%rax        # ea00 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   bd110:	lea    -0xae5e7(%rip),%rcx        # eb30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   bd117:	cmove  %rcx,%rax
   bd11b:	mov    0x48(%rsp),%rcx
   bd120:	test   %dl,%dl
   bd122:	lea    -0xaea49(%rip),%rdx        # e6e0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   bd129:	cmove  %rdx,%rax
   bd12d:	mov    %rax,(%rsp)
   bd131:	lea    0x40(%rsp),%rdi
   bd136:	mov    0x30(%rsp),%rsi
   bd13b:	mov    0x630(%rsp),%rdx
   bd143:	jmp    bce3c <meshoptimizer_rs::codec::raw+0x17cc>
   bd148:	mov    0xb8(%rsp),%rdx
   bd150:	mov    0xc0(%rsp),%rdi
   bd158:	cmp    %rdx,%rdi
   bd15b:	mov    0x30(%rsp),%r14
   bd160:	ja     bd264 <meshoptimizer_rs::codec::raw+0x1bf4>
   bd166:	mov    0xb0(%rsp),%rsi
   bd16e:	add    %rdi,%rsi
   bd171:	lea    0xf0(%rsp),%rdi
   bd179:	mov    %rbp,%rdx
   bd17c:	call   *0x8bdfe(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   bd182:	mov    0xc8(%rsp),%rcx
   bd18a:	mov    %rcx,%rax
   bd18d:	cmp    %r13,%rcx
   bd190:	mov    0x630(%rsp),%rbx
   bd198:	jb     bc95f <meshoptimizer_rs::codec::raw+0x12ef>
   bd19e:	jmp    bd1a6 <meshoptimizer_rs::codec::raw+0x1b36>
   bd1a0:	mov    $0x1,%r15d
   bd1a6:	sub    %r15,%rbx
   bd1a9:	setae  %cl
   bd1ac:	cmp    0xe0(%rsp),%rbx
   bd1b4:	sete   %al
   bd1b7:	and    %cl,%al
   bd1b9:	neg    %al
   bd1bb:	or     $0x7,%al
   bd1bd:	jmp    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bd1bf:	mov    0x28(%rsp),%r9
   bd1c4:	mov    $0xff,%al
   bd1c6:	cmp    %r9,%rsi
   bd1c9:	je     bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bd1cb:	mov    $0x7,%al
   bd1cd:	add    $0x5f8,%rsp
   bd1d4:	pop    %rbx
   bd1d5:	pop    %r12
   bd1d7:	pop    %r13
   bd1d9:	pop    %r14
   bd1db:	pop    %r15
   bd1dd:	pop    %rbp
   bd1de:	ret
   bd1df:	movzbl 0x41(%rsp),%eax
   bd1e4:	jmp    bd1cd <meshoptimizer_rs::codec::raw+0x1b5d>
   bd1e6:	lea    0x8614b(%rip),%rcx        # 143338 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   bd1ed:	xor    %edi,%edi
   bd1ef:	mov    0x50(%rsp),%rsi
   bd1f4:	call   *0x8bede(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd1fa:	lea    0x8614f(%rip),%rcx        # 143350 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   bd201:	mov    %rsi,%rdx
   bd204:	mov    %rbx,%rdi
   bd207:	call   *0x8becb(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd20d:	lea    0x8619c(%rip),%rcx        # 1433b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   bd214:	mov    $0x100,%edx
   bd219:	xor    %edi,%edi
   bd21b:	mov    %rbp,%rsi
   bd21e:	call   *0x8beb4(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd224:	lea    0x8616d(%rip),%rcx        # 143398 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   bd22b:	mov    %rsi,%rdi
   bd22e:	mov    0x60(%rsp),%rsi
   bd233:	mov    %rbx,%rdx
   bd236:	call   *0x8be9c(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd23c:	lea    0x8a11d(%rip),%rcx        # 147360 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6530>
   bd243:	mov    %r12,%rdx
   bd246:	call   *0x8be8c(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd24c:	lea    0x86175(%rip),%rcx        # 1433c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2598>
   bd253:	mov    0x60(%rsp),%rdi
   bd258:	mov    %rbx,%rsi
   bd25b:	mov    %rbx,%rdx
   bd25e:	call   *0x8be74(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd264:	lea    0x86085(%rip),%rcx        # 1432f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   bd26b:	mov    %rdx,%rsi
   bd26e:	call   *0x8be64(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd274:	lea    0x8988d(%rip),%rdx        # 146b08 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x5cd8>
   bd27b:	xor    %edi,%edi
   bd27d:	xor    %esi,%esi
   bd27f:	call   *0x8bd33(%rip)        # 148fb8 <_DYNAMIC+0x250>
   bd285:	lea    0x8607c(%rip),%rdx        # 143308 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   bd28c:	mov    %r14,%rdi
   bd28f:	mov    0x90(%rsp),%rsi
   bd297:	call   *0x8bd1b(%rip)        # 148fb8 <_DYNAMIC+0x250>
   bd29d:	lea    0x8607c(%rip),%rdx        # 143320 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   bd2a4:	mov    %r14,%rdi
   bd2a7:	mov    0xa0(%rsp),%rsi
   bd2af:	call   *0x8bd03(%rip)        # 148fb8 <_DYNAMIC+0x250>
   bd2b5:	lea    0x8601c(%rip),%rdi        # 1432d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   bd2bc:	call   *0x8bede(%rip)        # 1491a0 <_DYNAMIC+0x438>
   bd2c2:	lea    0x8609f(%rip),%rcx        # 143368 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   bd2c9:	xor    %edi,%edi
   bd2cb:	mov    0x20(%rsp),%rsi
   bd2d0:	mov    0x28(%rsp),%rdx
   bd2d5:	call   *0x8bdfd(%rip)        # 1490d8 <_DYNAMIC+0x370>
   bd2db:	lea    0x8609e(%rip),%rcx        # 143380 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   bd2e2:	xor    %edi,%edi
   bd2e4:	mov    0x20(%rsp),%rsi
   bd2e9:	mov    0x28(%rsp),%rdx
   bd2ee:	call   *0x8bde4(%rip)        # 1490d8 <_DYNAMIC+0x370>
