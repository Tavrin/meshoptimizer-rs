
<artifacts>/screen/driver-unisolated:     file format elf64-x86-64


Disassembly of section .text:

00000000000af6f0 <meshoptimizer_rs::codec::raw>:
   af6f0:	push   %rbp
   af6f1:	push   %r15
   af6f3:	push   %r14
   af6f5:	push   %r13
   af6f7:	push   %r12
   af6f9:	push   %rbx
   af6fa:	sub    $0x608,%rsp
   af701:	mov    %r8,%rbp
   af704:	mov    %rcx,%r13
   af707:	mov    %rdx,%r15
   af70a:	mov    %rsi,%r12
   af70d:	mov    0x640(%rsp),%rbx
   af715:	test   %dil,%dil
   af718:	je     af889 <meshoptimizer_rs::codec::raw+0x199>
   af71e:	movzbl %dil,%eax
   af722:	cmp    $0x1,%eax
   af725:	jne    af8ee <meshoptimizer_rs::codec::raw+0x1fe>
   af72b:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   af735:	mov    %r13,%rax
   af738:	mul    %rdi
   af73b:	shr    $1,%rdx
   af73e:	lea    (%rdx,%rdx,2),%rcx
   af742:	mov    %r13,%rax
   af745:	sub    %rcx,%rax
   af748:	cmp    $0x2,%rbp
   af74c:	jne    af95d <meshoptimizer_rs::codec::raw+0x26d>
   af752:	test   %rax,%rax
   af755:	jne    af966 <meshoptimizer_rs::codec::raw+0x276>
   af75b:	mov    %r13,%rax
   af75e:	mul    %rdi
   af761:	shr    $1,%rdx
   af764:	lea    0x11(%rdx),%rcx
   af768:	mov    $0x7,%al
   af76a:	cmp    %rcx,%rbx
   af76d:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af773:	movzbl (%r9),%ecx
   af777:	mov    %ecx,%esi
   af779:	and    $0xf0,%sil
   af77d:	cmp    $0xe0,%sil
   af781:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af787:	and    $0xf,%cl
   af78a:	mov    $0x5,%al
   af78c:	cmp    $0x1,%cl
   af78f:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af795:	mov    %r9,%r10
   af798:	lea    0x1(%rdx),%rsi
   af79c:	pcmpeqd %xmm0,%xmm0
   af7a0:	movdqa %xmm0,0x270(%rsp)
   af7a9:	movdqa %xmm0,0x260(%rsp)
   af7b2:	movdqa %xmm0,0x250(%rsp)
   af7bb:	movdqa %xmm0,0x240(%rsp)
   af7c4:	movdqa %xmm0,0x230(%rsp)
   af7cd:	movdqa %xmm0,0x220(%rsp)
   af7d6:	movdqa %xmm0,0x210(%rsp)
   af7df:	movdqa %xmm0,0x200(%rsp)
   af7e8:	movdqa %xmm0,0x130(%rsp)
   af7f1:	movdqa %xmm0,0x120(%rsp)
   af7fa:	movdqa %xmm0,0x110(%rsp)
   af803:	movdqa %xmm0,0x100(%rsp)
   af80c:	xor    %r8d,%r8d
   af80f:	cmp    $0x1,%cl
   af812:	setne  %r9b
   af816:	lea    -0x10(%rbx),%rax
   af81a:	mov    %rax,0x28(%rsp)
   af81f:	mov    %r15,%rax
   af822:	mov    %rdx,%rcx
   af825:	mul    %rdi
   af828:	shr    $0x2,%rdx
   af82c:	cmp    %rcx,%rdx
   af82f:	cmovae %rcx,%rdx
   af833:	mov    %rdx,0x40(%rsp)
   af838:	test   %rdx,%rdx
   af83b:	je     b1275 <meshoptimizer_rs::codec::raw+0x1b85>
   af841:	mov    %r9b,%r8b
   af844:	lea    0xd(,%r8,2),%rax
   af84c:	mov    %rax,0x58(%rsp)
   af851:	mov    %rbx,0x38(%rsp)
   af856:	mov    %r10,0x30(%rsp)
   af85b:	lea    (%r10,%rbx,1),%rax
   af85f:	add    $0xfffffffffffffff0,%rax
   af863:	mov    %rax,0x60(%rsp)
   af868:	add    $0x4,%r12
   af86c:	xor    %r8d,%r8d
   af86f:	xor    %r10d,%r10d
   af872:	movl   $0x0,0x20(%rsp)
   af87a:	xor    %r11d,%r11d
   af87d:	xor    %edi,%edi
   af87f:	mov    0x28(%rsp),%r9
   af884:	jmp    afdb8 <meshoptimizer_rs::codec::raw+0x6c8>
   af889:	mov    %r9,%r14
   af88c:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   af891:	cmp    $0x2,%al
   af893:	jae    af96d <meshoptimizer_rs::codec::raw+0x27d>
   af899:	mov    $0x7,%al
   af89b:	test   %rbx,%rbx
   af89e:	je     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af8a4:	movzbl (%r14),%esi
   af8a8:	mov    %esi,%ecx
   af8aa:	and    $0xf0,%cl
   af8ad:	cmp    $0xa0,%cl
   af8b0:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af8b6:	and    $0xf,%sil
   af8ba:	mov    $0x5,%al
   af8bc:	cmp    $0x1,%sil
   af8c0:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af8c6:	mov    %r14,%rdi
   af8c9:	test   %sil,%sil
   af8cc:	je     b08b9 <meshoptimizer_rs::codec::raw+0x11c9>
   af8d2:	mov    %rbp,%rcx
   af8d5:	shr    $0x2,%rcx
   af8d9:	add    %rbp,%rcx
   af8dc:	cmp    $0x19,%rcx
   af8e0:	mov    $0x18,%eax
   af8e5:	cmovae %rcx,%rax
   af8e9:	jmp    b08c9 <meshoptimizer_rs::codec::raw+0x11d9>
   af8ee:	add    $0x5,%r13
   af8f2:	mov    $0x7,%al
   af8f4:	cmp    $0x2,%rbp
   af8f8:	jne    af9a4 <meshoptimizer_rs::codec::raw+0x2b4>
   af8fe:	cmp    %r13,%rbx
   af901:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af907:	test   %rbx,%rbx
   af90a:	je     b134e <meshoptimizer_rs::codec::raw+0x1c5e>
   af910:	movzbl (%r9),%ecx
   af914:	mov    %ecx,%edx
   af916:	and    $0xf0,%dl
   af919:	cmp    $0xd0,%dl
   af91c:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af922:	mov    $0x5,%al
   af924:	test   $0xe,%cl
   af927:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af92d:	dec    %rbx
   af930:	movq   $0x0,0x200(%rsp)
   af93c:	movabs $0x7ffffffffffffffc,%rax
   af946:	or     $0x2,%rax
   af94a:	and    %rax,%r15
   af94d:	je     afcbc <meshoptimizer_rs::codec::raw+0x5cc>
   af953:	inc    %r9
   af956:	xor    %eax,%eax
   af958:	jmp    afa39 <meshoptimizer_rs::codec::raw+0x349>
   af95d:	test   %rax,%rax
   af960:	je     afb97 <meshoptimizer_rs::codec::raw+0x4a7>
   af966:	xor    %eax,%eax
   af968:	jmp    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af96d:	cmp    $0x21,%r13
   af971:	jae    afb6a <meshoptimizer_rs::codec::raw+0x47a>
   af977:	cmp    $0x4,%rbp
   af97b:	mov    %r14,%r8
   af97e:	je     afcde <meshoptimizer_rs::codec::raw+0x5ee>
   af984:	cmp    $0xc,%rbp
   af988:	je     afccc <meshoptimizer_rs::codec::raw+0x5dc>
   af98e:	cmp    $0x20,%rbp
   af992:	jne    afcf0 <meshoptimizer_rs::codec::raw+0x600>
   af998:	lea    -0x2e8af(%rip),%rax        # 810f0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   af99f:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   af9a4:	cmp    %r13,%rbx
   af9a7:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af9ad:	test   %rbx,%rbx
   af9b0:	je     b134e <meshoptimizer_rs::codec::raw+0x1c5e>
   af9b6:	movzbl (%r9),%ecx
   af9ba:	mov    %ecx,%edx
   af9bc:	and    $0xf0,%dl
   af9bf:	cmp    $0xd0,%dl
   af9c2:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af9c8:	mov    $0x5,%al
   af9ca:	test   $0xe,%cl
   af9cd:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   af9d3:	dec    %rbx
   af9d6:	movq   $0x0,0x200(%rsp)
   af9e2:	movabs $0x7ffffffffffffffc,%rax
   af9ec:	and    %rax,%r15
   af9ef:	je     afcbc <meshoptimizer_rs::codec::raw+0x5cc>
   af9f5:	inc    %r9
   af9f8:	xor    %eax,%eax
   af9fa:	jmp    afaee <meshoptimizer_rs::codec::raw+0x3fe>
   af9ff:	dec    %rbx
   afa02:	inc    %r9
   afa05:	mov    %edx,%ecx
   afa07:	mov    %ecx,%edx
   afa09:	shr    $0x2,%edx
   afa0c:	mov    %ecx,%esi
   afa0e:	shl    $0x1e,%esi
   afa11:	sar    $0x1f,%esi
   afa14:	xor    %edx,%esi
   afa16:	and    $0x1,%ecx
   afa19:	add    0x200(%rsp,%rcx,4),%esi
   afa20:	mov    %esi,0x200(%rsp,%rcx,4)
   afa27:	mov    %si,(%r12,%rax,1)
   afa2c:	add    $0x2,%rax
   afa30:	cmp    %rax,%r15
   afa33:	je     afcbc <meshoptimizer_rs::codec::raw+0x5cc>
   afa39:	cmp    $0x5,%rbx
   afa3d:	jb     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   afa43:	movzbl (%r9),%edx
   afa47:	test   %dl,%dl
   afa49:	jns    af9ff <meshoptimizer_rs::codec::raw+0x30f>
   afa4b:	and    $0x7f,%edx
   afa4e:	movzbl 0x1(%r9),%ecx
   afa53:	mov    %ecx,%esi
   afa55:	and    $0x7f,%ecx
   afa58:	shl    $0x7,%ecx
   afa5b:	or     %edx,%ecx
   afa5d:	mov    $0x2,%edx
   afa62:	test   %sil,%sil
   afa65:	jns    afaaa <meshoptimizer_rs::codec::raw+0x3ba>
   afa67:	movzbl 0x2(%r9),%edx
   afa6c:	mov    %edx,%esi
   afa6e:	and    $0x7f,%edx
   afa71:	shl    $0xe,%edx
   afa74:	or     %edx,%ecx
   afa76:	mov    $0x3,%edx
   afa7b:	test   %sil,%sil
   afa7e:	jns    afaaa <meshoptimizer_rs::codec::raw+0x3ba>
   afa80:	movzbl 0x3(%r9),%edx
   afa85:	mov    %edx,%esi
   afa87:	and    $0x7f,%edx
   afa8a:	shl    $0x15,%edx
   afa8d:	or     %edx,%ecx
   afa8f:	mov    $0x4,%edx
   afa94:	test   %sil,%sil
   afa97:	jns    afaaa <meshoptimizer_rs::codec::raw+0x3ba>
   afa99:	movzbl 0x4(%r9),%esi
   afa9e:	shl    $0x1c,%esi
   afaa1:	or     %ecx,%esi
   afaa3:	mov    $0x5,%edx
   afaa8:	mov    %esi,%ecx
   afaaa:	sub    %rdx,%rbx
   afaad:	add    %rdx,%r9
   afab0:	jmp    afa07 <meshoptimizer_rs::codec::raw+0x317>
   afab5:	dec    %rbx
   afab8:	inc    %r9
   afabb:	mov    %edx,%ecx
   afabd:	mov    %ecx,%edx
   afabf:	shr    $0x2,%edx
   afac2:	mov    %ecx,%esi
   afac4:	shl    $0x1e,%esi
   afac7:	sar    $0x1f,%esi
   afaca:	xor    %edx,%esi
   afacc:	and    $0x1,%ecx
   afacf:	add    0x200(%rsp,%rcx,4),%esi
   afad6:	mov    %esi,0x200(%rsp,%rcx,4)
   afadd:	mov    %esi,(%r12,%rax,1)
   afae1:	add    $0x4,%rax
   afae5:	cmp    %rax,%r15
   afae8:	je     afcbc <meshoptimizer_rs::codec::raw+0x5cc>
   afaee:	cmp    $0x5,%rbx
   afaf2:	jb     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   afaf8:	movzbl (%r9),%edx
   afafc:	test   %dl,%dl
   afafe:	jns    afab5 <meshoptimizer_rs::codec::raw+0x3c5>
   afb00:	and    $0x7f,%edx
   afb03:	movzbl 0x1(%r9),%ecx
   afb08:	mov    %ecx,%esi
   afb0a:	and    $0x7f,%ecx
   afb0d:	shl    $0x7,%ecx
   afb10:	or     %edx,%ecx
   afb12:	mov    $0x2,%edx
   afb17:	test   %sil,%sil
   afb1a:	jns    afb5f <meshoptimizer_rs::codec::raw+0x46f>
   afb1c:	movzbl 0x2(%r9),%edx
   afb21:	mov    %edx,%esi
   afb23:	and    $0x7f,%edx
   afb26:	shl    $0xe,%edx
   afb29:	or     %edx,%ecx
   afb2b:	mov    $0x3,%edx
   afb30:	test   %sil,%sil
   afb33:	jns    afb5f <meshoptimizer_rs::codec::raw+0x46f>
   afb35:	movzbl 0x3(%r9),%edx
   afb3a:	mov    %edx,%esi
   afb3c:	and    $0x7f,%edx
   afb3f:	shl    $0x15,%edx
   afb42:	or     %edx,%ecx
   afb44:	mov    $0x4,%edx
   afb49:	test   %sil,%sil
   afb4c:	jns    afb5f <meshoptimizer_rs::codec::raw+0x46f>
   afb4e:	movzbl 0x4(%r9),%esi
   afb53:	shl    $0x1c,%esi
   afb56:	or     %ecx,%esi
   afb58:	mov    $0x5,%edx
   afb5d:	mov    %esi,%ecx
   afb5f:	sub    %rdx,%rbx
   afb62:	add    %rdx,%r9
   afb65:	jmp    afabd <meshoptimizer_rs::codec::raw+0x3cd>
   afb6a:	cmp    $0x4,%rbp
   afb6e:	mov    %r14,%r8
   afb71:	je     afce7 <meshoptimizer_rs::codec::raw+0x5f7>
   afb77:	cmp    $0xc,%rbp
   afb7b:	je     afcd5 <meshoptimizer_rs::codec::raw+0x5e5>
   afb81:	cmp    $0x20,%rbp
   afb85:	jne    afcf9 <meshoptimizer_rs::codec::raw+0x609>
   afb8b:	lea    -0x359a2(%rip),%rax        # 7a1f0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   afb92:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afb97:	lea    0x11(%rdx),%rcx
   afb9b:	mov    $0x7,%al
   afb9d:	cmp    %rcx,%rbx
   afba0:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   afba6:	movzbl (%r9),%ecx
   afbaa:	mov    %ecx,%esi
   afbac:	and    $0xf0,%sil
   afbb0:	cmp    $0xe0,%sil
   afbb4:	jne    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   afbba:	and    $0xf,%cl
   afbbd:	mov    $0x5,%al
   afbbf:	cmp    $0x1,%cl
   afbc2:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   afbc8:	mov    %r9,%r10
   afbcb:	lea    0x1(%rdx),%rsi
   afbcf:	pcmpeqd %xmm0,%xmm0
   afbd3:	movdqa %xmm0,0x270(%rsp)
   afbdc:	movdqa %xmm0,0x260(%rsp)
   afbe5:	movdqa %xmm0,0x250(%rsp)
   afbee:	movdqa %xmm0,0x240(%rsp)
   afbf7:	movdqa %xmm0,0x230(%rsp)
   afc00:	movdqa %xmm0,0x220(%rsp)
   afc09:	movdqa %xmm0,0x210(%rsp)
   afc12:	movdqa %xmm0,0x200(%rsp)
   afc1b:	movdqa %xmm0,0x130(%rsp)
   afc24:	movdqa %xmm0,0x120(%rsp)
   afc2d:	movdqa %xmm0,0x110(%rsp)
   afc36:	movdqa %xmm0,0x100(%rsp)
   afc3f:	xor    %r8d,%r8d
   afc42:	cmp    $0x1,%cl
   afc45:	setne  %r9b
   afc49:	lea    -0x10(%rbx),%rax
   afc4d:	mov    %rax,0x28(%rsp)
   afc52:	mov    %r15,%rax
   afc55:	mov    %rdx,%rcx
   afc58:	mul    %rdi
   afc5b:	shr    $0x3,%rdx
   afc5f:	cmp    %rcx,%rdx
   afc62:	cmovae %rcx,%rdx
   afc66:	mov    %rdx,0x40(%rsp)
   afc6b:	test   %rdx,%rdx
   afc6e:	je     b1275 <meshoptimizer_rs::codec::raw+0x1b85>
   afc74:	mov    %r9b,%r8b
   afc77:	lea    0xd(,%r8,2),%rax
   afc7f:	mov    %rax,0x58(%rsp)
   afc84:	mov    %rbx,0x38(%rsp)
   afc89:	mov    %r10,0x30(%rsp)
   afc8e:	lea    (%r10,%rbx,1),%rax
   afc92:	add    $0xfffffffffffffff0,%rax
   afc96:	mov    %rax,0x60(%rsp)
   afc9b:	add    $0x8,%r12
   afc9f:	xor    %r8d,%r8d
   afca2:	xor    %r10d,%r10d
   afca5:	movl   $0x0,0x20(%rsp)
   afcad:	xor    %r11d,%r11d
   afcb0:	xor    %edi,%edi
   afcb2:	mov    0x28(%rsp),%r9
   afcb7:	jmp    b0382 <meshoptimizer_rs::codec::raw+0xc92>
   afcbc:	cmp    $0x4,%rbx
   afcc0:	sete   %al
   afcc3:	neg    %al
   afcc5:	or     $0x7,%al
   afcc7:	jmp    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   afccc:	lea    -0x2a603(%rip),%rax        # 856d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, [u8]>>
   afcd3:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afcd5:	lea    -0x31bac(%rip),%rax        # 7e130 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, [u8]>>
   afcdc:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afcde:	lea    -0x2c155(%rip),%rax        # 83b90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   afce5:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afce7:	lea    -0x3346e(%rip),%rax        # 7c880 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   afcee:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afcf0:	lea    -0x304a7(%rip),%rax        # 7f850 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   afcf7:	jmp    afd00 <meshoptimizer_rs::codec::raw+0x610>
   afcf9:	lea    -0x36ff0(%rip),%rax        # 78d10 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   afd00:	mov    %r12,%rdi
   afd03:	mov    %r15,%rsi
   afd06:	mov    %r13,%rdx
   afd09:	mov    %rbp,%rcx
   afd0c:	mov    %rbx,%r9
   afd0f:	add    $0x608,%rsp
   afd16:	pop    %rbx
   afd17:	pop    %r12
   afd19:	pop    %r13
   afd1b:	pop    %r14
   afd1d:	pop    %r15
   afd1f:	pop    %rbp
   afd20:	jmp    *%rax
   afd22:	mov    %bpl,%r14b
   afd25:	add    %r14d,%r11d
   afd28:	mov    %ebx,0x100(%rsp,%r10,4)
   afd30:	mov    %r11d,%ebp
   afd33:	not    %eax
   afd35:	shr    $0x4,%eax
   afd38:	add    %r8d,%eax
   afd3b:	and    $0xf,%eax
   afd3e:	mov    0x200(%rsp,%rax,8),%r11d
   afd46:	mov    0x204(%rsp,%rax,8),%r15d
   afd4e:	add    %r14,%r10
   afd51:	mov    $0x2,%eax
   afd56:	mov    $0x1,%r12d
   afd5c:	mov    %r8,%r13
   afd5f:	inc    %rdi
   afd62:	mov    %ebx,0x200(%rsp,%r13,8)
   afd6a:	mov    %r15d,0x204(%rsp,%r13,8)
   afd72:	add    %r8d,%r12d
   afd75:	and    $0xf,%r12d
   afd79:	mov    %r11d,0x200(%rsp,%r12,8)
   afd81:	mov    %ebx,0x204(%rsp,%r12,8)
   afd89:	add    %r8d,%eax
   afd8c:	and    $0xf,%eax
   afd8f:	and    $0xf,%r10d
   afd93:	mov    %rdx,%r12
   afd96:	mov    %r11w,-0x4(%rdx)
   afd9b:	mov    %r15w,-0x2(%rdx)
   afda0:	mov    %bx,(%rdx)
   afda3:	add    $0x6,%r12
   afda7:	mov    %rax,%r8
   afdaa:	mov    %ebp,%r11d
   afdad:	cmp    %rdi,0x40(%rsp)
   afdb2:	je     b127a <meshoptimizer_rs::codec::raw+0x1b8a>
   afdb8:	cmp    %r9,%rsi
   afdbb:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   afdc1:	mov    %r12,%rdx
   afdc4:	mov    0x30(%rsp),%rcx
   afdc9:	movzbl 0x1(%rcx,%rdi,1),%eax
   afdce:	cmp    $0xf0,%al
   afdd0:	jae    afe0a <meshoptimizer_rs::codec::raw+0x71a>
   afdd2:	mov    %eax,%r15d
   afdd5:	and    $0xf,%r15d
   afdd9:	cmp    %r15d,0x58(%rsp)
   afdde:	jbe    afe48 <meshoptimizer_rs::codec::raw+0x758>
   afde0:	xor    %r14d,%r14d
   afde3:	mov    %r11d,%ebx
   afde6:	test   %r15b,%r15b
   afde9:	sete   %bpl
   afded:	je     afd22 <meshoptimizer_rs::codec::raw+0x632>
   afdf3:	not    %r15d
   afdf6:	add    %r10d,%r15d
   afdf9:	and    $0xf,%r15d
   afdfd:	mov    0x100(%rsp,%r15,4),%ebx
   afe05:	jmp    afd22 <meshoptimizer_rs::codec::raw+0x632>
   afe0a:	cmp    $0xfe,%al
   afe0c:	jae    afe6f <meshoptimizer_rs::codec::raw+0x77f>
   afe0e:	and    $0xf,%eax
   afe11:	mov    0x60(%rsp),%rcx
   afe16:	movzbl (%rcx,%rax,1),%r13d
   afe1b:	lea    0x1(%r11),%r15d
   afe1f:	xor    %eax,%eax
   afe21:	mov    %r13d,%ecx
   afe24:	shr    $0x4,%ecx
   afe27:	sete   %r12b
   afe2b:	je     afeb2 <meshoptimizer_rs::codec::raw+0x7c2>
   afe31:	mov    %r10d,%ebx
   afe34:	sub    %ecx,%ebx
   afe36:	and    $0xf,%ebx
   afe39:	mov    %r15d,%ebp
   afe3c:	mov    0x100(%rsp,%rbx,4),%ecx
   afe43:	mov    %ecx,%r15d
   afe46:	jmp    afeb6 <meshoptimizer_rs::codec::raw+0x7c6>
   afe48:	cmp    $0xf,%r15b
   afe4c:	jne    aff32 <meshoptimizer_rs::codec::raw+0x842>
   afe52:	mov    0x30(%rsp),%r14
   afe57:	movzbl (%r14,%rsi,1),%ebp
   afe5c:	test   %bpl,%bpl
   afe5f:	js     affa0 <meshoptimizer_rs::codec::raw+0x8b0>
   afe65:	inc    %rsi
   afe68:	mov    %ebp,%ebx
   afe6a:	jmp    b0133 <meshoptimizer_rs::codec::raw+0xa43>
   afe6f:	movzbl (%rcx,%rsi,1),%r14d
   afe74:	xor    %r15d,%r15d
   afe77:	cmp    $0xfe,%eax
   afe7c:	sete   %r15b
   afe80:	test   %r14d,%r14d
   afe83:	cmove  %r14d,%r11d
   afe87:	add    %r11d,%r15d
   afe8a:	mov    %r14d,%r12d
   afe8d:	shr    $0x4,%r12d
   afe91:	je     aff4d <meshoptimizer_rs::codec::raw+0x85d>
   afe97:	mov    %r10d,%ecx
   afe9a:	sub    %r12d,%ecx
   afe9d:	and    $0xf,%ecx
   afea0:	mov    %r15d,%ebx
   afea3:	mov    0x100(%rsp,%rcx,4),%ecx
   afeaa:	mov    %ecx,%r15d
   afead:	jmp    aff51 <meshoptimizer_rs::codec::raw+0x861>
   afeb2:	lea    0x2(%r11),%ebp
   afeb6:	xor    %r14d,%r14d
   afeb9:	mov    %ebp,%ebx
   afebb:	test   $0xf,%r13b
   afebf:	sete   %cl
   afec2:	je     afed4 <meshoptimizer_rs::codec::raw+0x7e4>
   afec4:	mov    %r10d,%ebx
   afec7:	sub    %r13d,%ebx
   afeca:	and    $0xf,%ebx
   afecd:	mov    0x100(%rsp,%rbx,4),%ebx
   afed4:	mov    %cl,%r14b
   afed7:	add    %r14d,%ebp
   afeda:	mov    %r11d,0x100(%rsp,%r10,4)
   afee2:	lea    0x1(%r10),%rcx
   afee6:	and    $0xf,%ecx
   afee9:	mov    %r15d,0x100(%rsp,%rcx,4)
   afef1:	mov    %r12b,%al
   afef4:	add    %r10,%rax
   afef7:	inc    %rax
   afefa:	add    %rax,%r14
   afefd:	and    $0xf,%eax
   aff00:	mov    %ebx,0x100(%rsp,%rax,4)
   aff07:	mov    %r15d,0x200(%rsp,%r8,8)
   aff0f:	mov    %r11d,0x204(%rsp,%r8,8)
   aff17:	lea    0x1(%r8),%r13d
   aff1b:	and    $0xf,%r13d
   aff1f:	mov    $0x3,%eax
   aff24:	mov    $0x2,%r12d
   aff2a:	mov    %r14,%r10
   aff2d:	jmp    afd5f <meshoptimizer_rs::codec::raw+0x66f>
   aff32:	movzbl %r15b,%ecx
   aff36:	xor    $0x3,%r15b
   aff3a:	movzbl %r15b,%ebx
   aff3e:	mov    0x20(%rsp),%ebp
   aff42:	add    %ecx,%ebp
   aff44:	sub    %ebx,%ebp
   aff46:	mov    %ebp,%ebx
   aff48:	jmp    b0144 <meshoptimizer_rs::codec::raw+0xa54>
   aff4d:	lea    0x1(%r15),%ebx
   aff51:	mov    %r14d,%r13d
   aff54:	and    $0xf,%r13b
   aff58:	je     aff70 <meshoptimizer_rs::codec::raw+0x880>
   aff5a:	mov    %r10d,%ecx
   aff5d:	sub    %r14d,%ecx
   aff60:	and    $0xf,%ecx
   aff63:	mov    %ebx,%ebp
   aff65:	mov    0x100(%rsp,%rcx,4),%ecx
   aff6c:	mov    %ecx,%ebx
   aff6e:	jmp    aff73 <meshoptimizer_rs::codec::raw+0x883>
   aff70:	lea    0x1(%rbx),%ebp
   aff73:	lea    0x1(%rsi),%r14
   aff77:	cmp    $0xfe,%eax
   aff7c:	je     b007a <meshoptimizer_rs::codec::raw+0x98a>
   aff82:	mov    0x30(%rsp),%r9
   aff87:	movzbl 0x1(%r9,%rsi,1),%eax
   aff8d:	test   %al,%al
   aff8f:	js     affc0 <meshoptimizer_rs::codec::raw+0x8d0>
   aff91:	add    $0x2,%rsi
   aff95:	mov    %rsi,%r14
   aff98:	mov    %eax,%r11d
   aff9b:	jmp    b005c <meshoptimizer_rs::codec::raw+0x96c>
   affa0:	and    $0x7f,%ebp
   affa3:	movzbl 0x1(%r14,%rsi,1),%ebx
   affa9:	mov    %ebx,%ecx
   affab:	and    $0x7f,%ebx
   affae:	shl    $0x7,%ebx
   affb1:	or     %ebp,%ebx
   affb3:	test   %cl,%cl
   affb5:	js     affe1 <meshoptimizer_rs::codec::raw+0x8f1>
   affb7:	add    $0x2,%rsi
   affbb:	jmp    b0133 <meshoptimizer_rs::codec::raw+0xa43>
   affc0:	and    $0x7f,%eax
   affc3:	movzbl 0x2(%r9,%rsi,1),%r11d
   affc9:	mov    %r11d,%ecx
   affcc:	and    $0x7f,%r11d
   affd0:	shl    $0x7,%r11d
   affd4:	or     %eax,%r11d
   affd7:	test   %cl,%cl
   affd9:	js     affff <meshoptimizer_rs::codec::raw+0x90f>
   affdb:	add    $0x3,%rsi
   affdf:	jmp    b0059 <meshoptimizer_rs::codec::raw+0x969>
   affe1:	movzbl 0x2(%r14,%rsi,1),%ecx
   affe7:	mov    %ecx,%ebp
   affe9:	and    $0x7f,%ecx
   affec:	shl    $0xe,%ecx
   affef:	or     %ecx,%ebx
   afff1:	test   %bpl,%bpl
   afff4:	js     b001a <meshoptimizer_rs::codec::raw+0x92a>
   afff6:	add    $0x3,%rsi
   afffa:	jmp    b0133 <meshoptimizer_rs::codec::raw+0xa43>
   affff:	movzbl 0x3(%r9,%rsi,1),%eax
   b0005:	mov    %eax,%ecx
   b0007:	and    $0x7f,%eax
   b000a:	shl    $0xe,%eax
   b000d:	or     %eax,%r11d
   b0010:	test   %cl,%cl
   b0012:	js     b003c <meshoptimizer_rs::codec::raw+0x94c>
   b0014:	add    $0x4,%rsi
   b0018:	jmp    b0059 <meshoptimizer_rs::codec::raw+0x969>
   b001a:	movzbl 0x3(%r14,%rsi,1),%ecx
   b0020:	mov    %ecx,%ebp
   b0022:	and    $0x7f,%ecx
   b0025:	shl    $0x15,%ecx
   b0028:	or     %ecx,%ebx
   b002a:	test   %bpl,%bpl
   b002d:	js     b0122 <meshoptimizer_rs::codec::raw+0xa32>
   b0033:	add    $0x4,%rsi
   b0037:	jmp    b0133 <meshoptimizer_rs::codec::raw+0xa43>
   b003c:	movzbl 0x4(%r9,%rsi,1),%eax
   b0042:	mov    %eax,%ecx
   b0044:	and    $0x7f,%eax
   b0047:	shl    $0x15,%eax
   b004a:	or     %eax,%r11d
   b004d:	test   %cl,%cl
   b004f:	js     b015b <meshoptimizer_rs::codec::raw+0xa6b>
   b0055:	add    $0x5,%rsi
   b0059:	mov    %rsi,%r14
   b005c:	mov    0x28(%rsp),%r9
   b0061:	mov    %r11d,%eax
   b0064:	shr    $1,%eax
   b0066:	and    $0x1,%r11d
   b006a:	neg    %r11d
   b006d:	xor    %eax,%r11d
   b0070:	add    0x20(%rsp),%r11d
   b0075:	mov    %r11d,0x20(%rsp)
   b007a:	cmp    $0xf,%r12b
   b007e:	jne    b01aa <meshoptimizer_rs::codec::raw+0xaba>
   b0084:	cmp    $0xfffffffffffffffa,%r14
   b0088:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b008e:	lea    0x5(%r14),%rax
   b0092:	cmp    0x38(%rsp),%rax
   b0097:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b009d:	mov    0x30(%rsp),%r9
   b00a2:	movzbl (%r9,%r14,1),%esi
   b00a7:	test   %sil,%sil
   b00aa:	js     b00ba <meshoptimizer_rs::codec::raw+0x9ca>
   b00ac:	inc    %r14
   b00af:	mov    %r14,%rax
   b00b2:	mov    %esi,%r15d
   b00b5:	jmp    b0189 <meshoptimizer_rs::codec::raw+0xa99>
   b00ba:	and    $0x7f,%esi
   b00bd:	movzbl 0x1(%r9,%r14,1),%r15d
   b00c3:	mov    %r15d,%ecx
   b00c6:	and    $0x7f,%r15d
   b00ca:	shl    $0x7,%r15d
   b00ce:	or     %esi,%r15d
   b00d1:	test   %cl,%cl
   b00d3:	js     b00e1 <meshoptimizer_rs::codec::raw+0x9f1>
   b00d5:	add    $0x2,%r14
   b00d9:	mov    %r14,%rax
   b00dc:	jmp    b0189 <meshoptimizer_rs::codec::raw+0xa99>
   b00e1:	movzbl 0x2(%r9,%r14,1),%ecx
   b00e7:	mov    %ecx,%esi
   b00e9:	and    $0x7f,%ecx
   b00ec:	shl    $0xe,%ecx
   b00ef:	or     %ecx,%r15d
   b00f2:	test   %sil,%sil
   b00f5:	js     b0103 <meshoptimizer_rs::codec::raw+0xa13>
   b00f7:	add    $0x3,%r14
   b00fb:	mov    %r14,%rax
   b00fe:	jmp    b0189 <meshoptimizer_rs::codec::raw+0xa99>
   b0103:	movzbl 0x3(%r9,%r14,1),%ecx
   b0109:	mov    %ecx,%esi
   b010b:	and    $0x7f,%ecx
   b010e:	shl    $0x15,%ecx
   b0111:	or     %ecx,%r15d
   b0114:	test   %sil,%sil
   b0117:	js     b0175 <meshoptimizer_rs::codec::raw+0xa85>
   b0119:	add    $0x4,%r14
   b011d:	mov    %r14,%rax
   b0120:	jmp    b0189 <meshoptimizer_rs::codec::raw+0xa99>
   b0122:	movzbl 0x4(%r14,%rsi,1),%ecx
   b0128:	add    $0x5,%rsi
   b012c:	shl    $0x1c,%ecx
   b012f:	or     %ebx,%ecx
   b0131:	mov    %ecx,%ebx
   b0133:	mov    0x20(%rsp),%ebp
   b0137:	mov    %ebx,%ecx
   b0139:	shr    $1,%ecx
   b013b:	and    $0x1,%ebx
   b013e:	neg    %ebx
   b0140:	xor    %ecx,%ebx
   b0142:	add    %ebp,%ebx
   b0144:	mov    %ebx,0x100(%rsp,%r10,4)
   b014c:	mov    $0x1,%r14d
   b0152:	mov    %ebx,0x20(%rsp)
   b0156:	jmp    afd30 <meshoptimizer_rs::codec::raw+0x640>
   b015b:	add    $0x6,%rsi
   b015f:	mov    0x30(%rsp),%rax
   b0164:	movzbl 0x4(%rax,%r14,1),%eax
   b016a:	shl    $0x1c,%eax
   b016d:	or     %r11d,%eax
   b0170:	jmp    aff95 <meshoptimizer_rs::codec::raw+0x8a5>
   b0175:	mov    0x30(%rsp),%rcx
   b017a:	movzbl 0x4(%rcx,%r14,1),%ecx
   b0180:	shl    $0x1c,%ecx
   b0183:	or     %r15d,%ecx
   b0186:	mov    %ecx,%r15d
   b0189:	mov    0x28(%rsp),%r9
   b018e:	mov    %r15d,%ecx
   b0191:	shr    $1,%ecx
   b0193:	and    $0x1,%r15d
   b0197:	neg    %r15d
   b019a:	xor    %ecx,%r15d
   b019d:	add    0x20(%rsp),%r15d
   b01a2:	mov    %rax,%r14
   b01a5:	mov    %r15d,0x20(%rsp)
   b01aa:	cmp    $0xf,%r13b
   b01ae:	jne    b026f <meshoptimizer_rs::codec::raw+0xb7f>
   b01b4:	cmp    $0xfffffffffffffffa,%r14
   b01b8:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b01be:	lea    0x5(%r14),%rax
   b01c2:	cmp    0x38(%rsp),%rax
   b01c7:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b01cd:	mov    0x30(%rsp),%r9
   b01d2:	movzbl (%r9,%r14,1),%esi
   b01d7:	test   %sil,%sil
   b01da:	js     b01e6 <meshoptimizer_rs::codec::raw+0xaf6>
   b01dc:	inc    %r14
   b01df:	mov    %r14,%rax
   b01e2:	mov    %esi,%ebx
   b01e4:	jmp    b0254 <meshoptimizer_rs::codec::raw+0xb64>
   b01e6:	and    $0x7f,%esi
   b01e9:	movzbl 0x1(%r9,%r14,1),%ebx
   b01ef:	mov    %ebx,%ecx
   b01f1:	and    $0x7f,%ebx
   b01f4:	shl    $0x7,%ebx
   b01f7:	or     %esi,%ebx
   b01f9:	test   %cl,%cl
   b01fb:	js     b0206 <meshoptimizer_rs::codec::raw+0xb16>
   b01fd:	add    $0x2,%r14
   b0201:	mov    %r14,%rax
   b0204:	jmp    b0254 <meshoptimizer_rs::codec::raw+0xb64>
   b0206:	movzbl 0x2(%r9,%r14,1),%ecx
   b020c:	mov    %ecx,%esi
   b020e:	and    $0x7f,%ecx
   b0211:	shl    $0xe,%ecx
   b0214:	or     %ecx,%ebx
   b0216:	test   %sil,%sil
   b0219:	js     b0224 <meshoptimizer_rs::codec::raw+0xb34>
   b021b:	add    $0x3,%r14
   b021f:	mov    %r14,%rax
   b0222:	jmp    b0254 <meshoptimizer_rs::codec::raw+0xb64>
   b0224:	movzbl 0x3(%r9,%r14,1),%ecx
   b022a:	mov    %ecx,%esi
   b022c:	and    $0x7f,%ecx
   b022f:	shl    $0x15,%ecx
   b0232:	or     %ecx,%ebx
   b0234:	test   %sil,%sil
   b0237:	js     b0242 <meshoptimizer_rs::codec::raw+0xb52>
   b0239:	add    $0x4,%r14
   b023d:	mov    %r14,%rax
   b0240:	jmp    b0254 <meshoptimizer_rs::codec::raw+0xb64>
   b0242:	mov    0x30(%rsp),%rcx
   b0247:	movzbl 0x4(%rcx,%r14,1),%ecx
   b024d:	shl    $0x1c,%ecx
   b0250:	or     %ebx,%ecx
   b0252:	mov    %ecx,%ebx
   b0254:	mov    0x28(%rsp),%r9
   b0259:	mov    %ebx,%ecx
   b025b:	shr    $1,%ecx
   b025d:	and    $0x1,%ebx
   b0260:	neg    %ebx
   b0262:	xor    %ecx,%ebx
   b0264:	add    0x20(%rsp),%ebx
   b0268:	mov    %rax,%r14
   b026b:	mov    %ebx,0x20(%rsp)
   b026f:	cmp    $0xf,%r13b
   b0273:	sete   %al
   b0276:	cmp    $0xf,%r12b
   b027a:	sete   %cl
   b027d:	test   %r13b,%r13b
   b0280:	sete   %sil
   b0284:	test   %r12b,%r12b
   b0287:	sete   %r12b
   b028b:	mov    %r11d,0x100(%rsp,%r10,4)
   b0293:	lea    0x1(%r10),%r13
   b0297:	and    $0xf,%r13d
   b029b:	or     %cl,%r12b
   b029e:	mov    %r15d,0x100(%rsp,%r13,4)
   b02a6:	movzbl %r12b,%ecx
   b02aa:	add    %r10,%rcx
   b02ad:	inc    %rcx
   b02b0:	or     %al,%sil
   b02b3:	movzbl %sil,%r10d
   b02b7:	add    %rcx,%r10
   b02ba:	and    $0xf,%ecx
   b02bd:	mov    %ebx,0x100(%rsp,%rcx,4)
   b02c4:	mov    %r15d,0x200(%rsp,%r8,8)
   b02cc:	mov    %r11d,0x204(%rsp,%r8,8)
   b02d4:	lea    0x1(%r8),%r13d
   b02d8:	and    $0xf,%r13d
   b02dc:	mov    $0x3,%eax
   b02e1:	mov    $0x2,%r12d
   b02e7:	mov    %r14,%rsi
   b02ea:	jmp    afd5f <meshoptimizer_rs::codec::raw+0x66f>
   b02ef:	mov    %bpl,%r14b
   b02f2:	add    %r14d,%r11d
   b02f5:	mov    %ebx,0x100(%rsp,%r10,4)
   b02fd:	mov    %r11d,%ebp
   b0300:	not    %eax
   b0302:	shr    $0x4,%eax
   b0305:	add    %r8d,%eax
   b0308:	and    $0xf,%eax
   b030b:	mov    0x200(%rsp,%rax,8),%r11d
   b0313:	mov    0x204(%rsp,%rax,8),%r15d
   b031b:	add    %r14,%r10
   b031e:	mov    $0x2,%eax
   b0323:	mov    $0x1,%r12d
   b0329:	mov    %r8,%r13
   b032c:	inc    %rdi
   b032f:	mov    %ebx,0x200(%rsp,%r13,8)
   b0337:	mov    %r15d,0x204(%rsp,%r13,8)
   b033f:	add    %r8d,%r12d
   b0342:	and    $0xf,%r12d
   b0346:	mov    %r11d,0x200(%rsp,%r12,8)
   b034e:	mov    %ebx,0x204(%rsp,%r12,8)
   b0356:	add    %r8d,%eax
   b0359:	and    $0xf,%eax
   b035c:	and    $0xf,%r10d
   b0360:	mov    %rdx,%r12
   b0363:	mov    %r11d,-0x8(%rdx)
   b0367:	mov    %r15d,-0x4(%rdx)
   b036b:	mov    %ebx,(%rdx)
   b036d:	add    $0xc,%r12
   b0371:	mov    %rax,%r8
   b0374:	mov    %ebp,%r11d
   b0377:	cmp    %rdi,0x40(%rsp)
   b037c:	je     b127a <meshoptimizer_rs::codec::raw+0x1b8a>
   b0382:	cmp    %r9,%rsi
   b0385:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b038b:	mov    %r12,%rdx
   b038e:	mov    0x30(%rsp),%rcx
   b0393:	movzbl 0x1(%rcx,%rdi,1),%eax
   b0398:	cmp    $0xf0,%al
   b039a:	jae    b03d4 <meshoptimizer_rs::codec::raw+0xce4>
   b039c:	mov    %eax,%r15d
   b039f:	and    $0xf,%r15d
   b03a3:	cmp    %r15d,0x58(%rsp)
   b03a8:	jbe    b0412 <meshoptimizer_rs::codec::raw+0xd22>
   b03aa:	xor    %r14d,%r14d
   b03ad:	mov    %r11d,%ebx
   b03b0:	test   %r15b,%r15b
   b03b3:	sete   %bpl
   b03b7:	je     b02ef <meshoptimizer_rs::codec::raw+0xbff>
   b03bd:	not    %r15d
   b03c0:	add    %r10d,%r15d
   b03c3:	and    $0xf,%r15d
   b03c7:	mov    0x100(%rsp,%r15,4),%ebx
   b03cf:	jmp    b02ef <meshoptimizer_rs::codec::raw+0xbff>
   b03d4:	cmp    $0xfe,%al
   b03d6:	jae    b0439 <meshoptimizer_rs::codec::raw+0xd49>
   b03d8:	and    $0xf,%eax
   b03db:	mov    0x60(%rsp),%rcx
   b03e0:	movzbl (%rcx,%rax,1),%r13d
   b03e5:	lea    0x1(%r11),%r15d
   b03e9:	xor    %eax,%eax
   b03eb:	mov    %r13d,%ecx
   b03ee:	shr    $0x4,%ecx
   b03f1:	sete   %r12b
   b03f5:	je     b047c <meshoptimizer_rs::codec::raw+0xd8c>
   b03fb:	mov    %r10d,%ebx
   b03fe:	sub    %ecx,%ebx
   b0400:	and    $0xf,%ebx
   b0403:	mov    %r15d,%ebp
   b0406:	mov    0x100(%rsp,%rbx,4),%ecx
   b040d:	mov    %ecx,%r15d
   b0410:	jmp    b0480 <meshoptimizer_rs::codec::raw+0xd90>
   b0412:	cmp    $0xf,%r15b
   b0416:	jne    b04fc <meshoptimizer_rs::codec::raw+0xe0c>
   b041c:	mov    0x30(%rsp),%r14
   b0421:	movzbl (%r14,%rsi,1),%ebp
   b0426:	test   %bpl,%bpl
   b0429:	js     b056a <meshoptimizer_rs::codec::raw+0xe7a>
   b042f:	inc    %rsi
   b0432:	mov    %ebp,%ebx
   b0434:	jmp    b06fd <meshoptimizer_rs::codec::raw+0x100d>
   b0439:	movzbl (%rcx,%rsi,1),%r14d
   b043e:	xor    %r15d,%r15d
   b0441:	cmp    $0xfe,%eax
   b0446:	sete   %r15b
   b044a:	test   %r14d,%r14d
   b044d:	cmove  %r14d,%r11d
   b0451:	add    %r11d,%r15d
   b0454:	mov    %r14d,%r12d
   b0457:	shr    $0x4,%r12d
   b045b:	je     b0517 <meshoptimizer_rs::codec::raw+0xe27>
   b0461:	mov    %r10d,%ecx
   b0464:	sub    %r12d,%ecx
   b0467:	and    $0xf,%ecx
   b046a:	mov    %r15d,%ebx
   b046d:	mov    0x100(%rsp,%rcx,4),%ecx
   b0474:	mov    %ecx,%r15d
   b0477:	jmp    b051b <meshoptimizer_rs::codec::raw+0xe2b>
   b047c:	lea    0x2(%r11),%ebp
   b0480:	xor    %r14d,%r14d
   b0483:	mov    %ebp,%ebx
   b0485:	test   $0xf,%r13b
   b0489:	sete   %cl
   b048c:	je     b049e <meshoptimizer_rs::codec::raw+0xdae>
   b048e:	mov    %r10d,%ebx
   b0491:	sub    %r13d,%ebx
   b0494:	and    $0xf,%ebx
   b0497:	mov    0x100(%rsp,%rbx,4),%ebx
   b049e:	mov    %cl,%r14b
   b04a1:	add    %r14d,%ebp
   b04a4:	mov    %r11d,0x100(%rsp,%r10,4)
   b04ac:	lea    0x1(%r10),%rcx
   b04b0:	and    $0xf,%ecx
   b04b3:	mov    %r15d,0x100(%rsp,%rcx,4)
   b04bb:	mov    %r12b,%al
   b04be:	add    %r10,%rax
   b04c1:	inc    %rax
   b04c4:	add    %rax,%r14
   b04c7:	and    $0xf,%eax
   b04ca:	mov    %ebx,0x100(%rsp,%rax,4)
   b04d1:	mov    %r15d,0x200(%rsp,%r8,8)
   b04d9:	mov    %r11d,0x204(%rsp,%r8,8)
   b04e1:	lea    0x1(%r8),%r13d
   b04e5:	and    $0xf,%r13d
   b04e9:	mov    $0x3,%eax
   b04ee:	mov    $0x2,%r12d
   b04f4:	mov    %r14,%r10
   b04f7:	jmp    b032c <meshoptimizer_rs::codec::raw+0xc3c>
   b04fc:	movzbl %r15b,%ecx
   b0500:	xor    $0x3,%r15b
   b0504:	movzbl %r15b,%ebx
   b0508:	mov    0x20(%rsp),%ebp
   b050c:	add    %ecx,%ebp
   b050e:	sub    %ebx,%ebp
   b0510:	mov    %ebp,%ebx
   b0512:	jmp    b070e <meshoptimizer_rs::codec::raw+0x101e>
   b0517:	lea    0x1(%r15),%ebx
   b051b:	mov    %r14d,%r13d
   b051e:	and    $0xf,%r13b
   b0522:	je     b053a <meshoptimizer_rs::codec::raw+0xe4a>
   b0524:	mov    %r10d,%ecx
   b0527:	sub    %r14d,%ecx
   b052a:	and    $0xf,%ecx
   b052d:	mov    %ebx,%ebp
   b052f:	mov    0x100(%rsp,%rcx,4),%ecx
   b0536:	mov    %ecx,%ebx
   b0538:	jmp    b053d <meshoptimizer_rs::codec::raw+0xe4d>
   b053a:	lea    0x1(%rbx),%ebp
   b053d:	lea    0x1(%rsi),%r14
   b0541:	cmp    $0xfe,%eax
   b0546:	je     b0644 <meshoptimizer_rs::codec::raw+0xf54>
   b054c:	mov    0x30(%rsp),%r9
   b0551:	movzbl 0x1(%r9,%rsi,1),%eax
   b0557:	test   %al,%al
   b0559:	js     b058a <meshoptimizer_rs::codec::raw+0xe9a>
   b055b:	add    $0x2,%rsi
   b055f:	mov    %rsi,%r14
   b0562:	mov    %eax,%r11d
   b0565:	jmp    b0626 <meshoptimizer_rs::codec::raw+0xf36>
   b056a:	and    $0x7f,%ebp
   b056d:	movzbl 0x1(%r14,%rsi,1),%ebx
   b0573:	mov    %ebx,%ecx
   b0575:	and    $0x7f,%ebx
   b0578:	shl    $0x7,%ebx
   b057b:	or     %ebp,%ebx
   b057d:	test   %cl,%cl
   b057f:	js     b05ab <meshoptimizer_rs::codec::raw+0xebb>
   b0581:	add    $0x2,%rsi
   b0585:	jmp    b06fd <meshoptimizer_rs::codec::raw+0x100d>
   b058a:	and    $0x7f,%eax
   b058d:	movzbl 0x2(%r9,%rsi,1),%r11d
   b0593:	mov    %r11d,%ecx
   b0596:	and    $0x7f,%r11d
   b059a:	shl    $0x7,%r11d
   b059e:	or     %eax,%r11d
   b05a1:	test   %cl,%cl
   b05a3:	js     b05c9 <meshoptimizer_rs::codec::raw+0xed9>
   b05a5:	add    $0x3,%rsi
   b05a9:	jmp    b0623 <meshoptimizer_rs::codec::raw+0xf33>
   b05ab:	movzbl 0x2(%r14,%rsi,1),%ecx
   b05b1:	mov    %ecx,%ebp
   b05b3:	and    $0x7f,%ecx
   b05b6:	shl    $0xe,%ecx
   b05b9:	or     %ecx,%ebx
   b05bb:	test   %bpl,%bpl
   b05be:	js     b05e4 <meshoptimizer_rs::codec::raw+0xef4>
   b05c0:	add    $0x3,%rsi
   b05c4:	jmp    b06fd <meshoptimizer_rs::codec::raw+0x100d>
   b05c9:	movzbl 0x3(%r9,%rsi,1),%eax
   b05cf:	mov    %eax,%ecx
   b05d1:	and    $0x7f,%eax
   b05d4:	shl    $0xe,%eax
   b05d7:	or     %eax,%r11d
   b05da:	test   %cl,%cl
   b05dc:	js     b0606 <meshoptimizer_rs::codec::raw+0xf16>
   b05de:	add    $0x4,%rsi
   b05e2:	jmp    b0623 <meshoptimizer_rs::codec::raw+0xf33>
   b05e4:	movzbl 0x3(%r14,%rsi,1),%ecx
   b05ea:	mov    %ecx,%ebp
   b05ec:	and    $0x7f,%ecx
   b05ef:	shl    $0x15,%ecx
   b05f2:	or     %ecx,%ebx
   b05f4:	test   %bpl,%bpl
   b05f7:	js     b06ec <meshoptimizer_rs::codec::raw+0xffc>
   b05fd:	add    $0x4,%rsi
   b0601:	jmp    b06fd <meshoptimizer_rs::codec::raw+0x100d>
   b0606:	movzbl 0x4(%r9,%rsi,1),%eax
   b060c:	mov    %eax,%ecx
   b060e:	and    $0x7f,%eax
   b0611:	shl    $0x15,%eax
   b0614:	or     %eax,%r11d
   b0617:	test   %cl,%cl
   b0619:	js     b0725 <meshoptimizer_rs::codec::raw+0x1035>
   b061f:	add    $0x5,%rsi
   b0623:	mov    %rsi,%r14
   b0626:	mov    0x28(%rsp),%r9
   b062b:	mov    %r11d,%eax
   b062e:	shr    $1,%eax
   b0630:	and    $0x1,%r11d
   b0634:	neg    %r11d
   b0637:	xor    %eax,%r11d
   b063a:	add    0x20(%rsp),%r11d
   b063f:	mov    %r11d,0x20(%rsp)
   b0644:	cmp    $0xf,%r12b
   b0648:	jne    b0774 <meshoptimizer_rs::codec::raw+0x1084>
   b064e:	cmp    $0xfffffffffffffffa,%r14
   b0652:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b0658:	lea    0x5(%r14),%rax
   b065c:	cmp    0x38(%rsp),%rax
   b0661:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b0667:	mov    0x30(%rsp),%r9
   b066c:	movzbl (%r9,%r14,1),%esi
   b0671:	test   %sil,%sil
   b0674:	js     b0684 <meshoptimizer_rs::codec::raw+0xf94>
   b0676:	inc    %r14
   b0679:	mov    %r14,%rax
   b067c:	mov    %esi,%r15d
   b067f:	jmp    b0753 <meshoptimizer_rs::codec::raw+0x1063>
   b0684:	and    $0x7f,%esi
   b0687:	movzbl 0x1(%r9,%r14,1),%r15d
   b068d:	mov    %r15d,%ecx
   b0690:	and    $0x7f,%r15d
   b0694:	shl    $0x7,%r15d
   b0698:	or     %esi,%r15d
   b069b:	test   %cl,%cl
   b069d:	js     b06ab <meshoptimizer_rs::codec::raw+0xfbb>
   b069f:	add    $0x2,%r14
   b06a3:	mov    %r14,%rax
   b06a6:	jmp    b0753 <meshoptimizer_rs::codec::raw+0x1063>
   b06ab:	movzbl 0x2(%r9,%r14,1),%ecx
   b06b1:	mov    %ecx,%esi
   b06b3:	and    $0x7f,%ecx
   b06b6:	shl    $0xe,%ecx
   b06b9:	or     %ecx,%r15d
   b06bc:	test   %sil,%sil
   b06bf:	js     b06cd <meshoptimizer_rs::codec::raw+0xfdd>
   b06c1:	add    $0x3,%r14
   b06c5:	mov    %r14,%rax
   b06c8:	jmp    b0753 <meshoptimizer_rs::codec::raw+0x1063>
   b06cd:	movzbl 0x3(%r9,%r14,1),%ecx
   b06d3:	mov    %ecx,%esi
   b06d5:	and    $0x7f,%ecx
   b06d8:	shl    $0x15,%ecx
   b06db:	or     %ecx,%r15d
   b06de:	test   %sil,%sil
   b06e1:	js     b073f <meshoptimizer_rs::codec::raw+0x104f>
   b06e3:	add    $0x4,%r14
   b06e7:	mov    %r14,%rax
   b06ea:	jmp    b0753 <meshoptimizer_rs::codec::raw+0x1063>
   b06ec:	movzbl 0x4(%r14,%rsi,1),%ecx
   b06f2:	add    $0x5,%rsi
   b06f6:	shl    $0x1c,%ecx
   b06f9:	or     %ebx,%ecx
   b06fb:	mov    %ecx,%ebx
   b06fd:	mov    0x20(%rsp),%ebp
   b0701:	mov    %ebx,%ecx
   b0703:	shr    $1,%ecx
   b0705:	and    $0x1,%ebx
   b0708:	neg    %ebx
   b070a:	xor    %ecx,%ebx
   b070c:	add    %ebp,%ebx
   b070e:	mov    %ebx,0x100(%rsp,%r10,4)
   b0716:	mov    $0x1,%r14d
   b071c:	mov    %ebx,0x20(%rsp)
   b0720:	jmp    b02fd <meshoptimizer_rs::codec::raw+0xc0d>
   b0725:	add    $0x6,%rsi
   b0729:	mov    0x30(%rsp),%rax
   b072e:	movzbl 0x4(%rax,%r14,1),%eax
   b0734:	shl    $0x1c,%eax
   b0737:	or     %r11d,%eax
   b073a:	jmp    b055f <meshoptimizer_rs::codec::raw+0xe6f>
   b073f:	mov    0x30(%rsp),%rcx
   b0744:	movzbl 0x4(%rcx,%r14,1),%ecx
   b074a:	shl    $0x1c,%ecx
   b074d:	or     %r15d,%ecx
   b0750:	mov    %ecx,%r15d
   b0753:	mov    0x28(%rsp),%r9
   b0758:	mov    %r15d,%ecx
   b075b:	shr    $1,%ecx
   b075d:	and    $0x1,%r15d
   b0761:	neg    %r15d
   b0764:	xor    %ecx,%r15d
   b0767:	add    0x20(%rsp),%r15d
   b076c:	mov    %rax,%r14
   b076f:	mov    %r15d,0x20(%rsp)
   b0774:	cmp    $0xf,%r13b
   b0778:	jne    b0839 <meshoptimizer_rs::codec::raw+0x1149>
   b077e:	cmp    $0xfffffffffffffffa,%r14
   b0782:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b0788:	lea    0x5(%r14),%rax
   b078c:	cmp    0x38(%rsp),%rax
   b0791:	ja     b1281 <meshoptimizer_rs::codec::raw+0x1b91>
   b0797:	mov    0x30(%rsp),%r9
   b079c:	movzbl (%r9,%r14,1),%esi
   b07a1:	test   %sil,%sil
   b07a4:	js     b07b0 <meshoptimizer_rs::codec::raw+0x10c0>
   b07a6:	inc    %r14
   b07a9:	mov    %r14,%rax
   b07ac:	mov    %esi,%ebx
   b07ae:	jmp    b081e <meshoptimizer_rs::codec::raw+0x112e>
   b07b0:	and    $0x7f,%esi
   b07b3:	movzbl 0x1(%r9,%r14,1),%ebx
   b07b9:	mov    %ebx,%ecx
   b07bb:	and    $0x7f,%ebx
   b07be:	shl    $0x7,%ebx
   b07c1:	or     %esi,%ebx
   b07c3:	test   %cl,%cl
   b07c5:	js     b07d0 <meshoptimizer_rs::codec::raw+0x10e0>
   b07c7:	add    $0x2,%r14
   b07cb:	mov    %r14,%rax
   b07ce:	jmp    b081e <meshoptimizer_rs::codec::raw+0x112e>
   b07d0:	movzbl 0x2(%r9,%r14,1),%ecx
   b07d6:	mov    %ecx,%esi
   b07d8:	and    $0x7f,%ecx
   b07db:	shl    $0xe,%ecx
   b07de:	or     %ecx,%ebx
   b07e0:	test   %sil,%sil
   b07e3:	js     b07ee <meshoptimizer_rs::codec::raw+0x10fe>
   b07e5:	add    $0x3,%r14
   b07e9:	mov    %r14,%rax
   b07ec:	jmp    b081e <meshoptimizer_rs::codec::raw+0x112e>
   b07ee:	movzbl 0x3(%r9,%r14,1),%ecx
   b07f4:	mov    %ecx,%esi
   b07f6:	and    $0x7f,%ecx
   b07f9:	shl    $0x15,%ecx
   b07fc:	or     %ecx,%ebx
   b07fe:	test   %sil,%sil
   b0801:	js     b080c <meshoptimizer_rs::codec::raw+0x111c>
   b0803:	add    $0x4,%r14
   b0807:	mov    %r14,%rax
   b080a:	jmp    b081e <meshoptimizer_rs::codec::raw+0x112e>
   b080c:	mov    0x30(%rsp),%rcx
   b0811:	movzbl 0x4(%rcx,%r14,1),%ecx
   b0817:	shl    $0x1c,%ecx
   b081a:	or     %ebx,%ecx
   b081c:	mov    %ecx,%ebx
   b081e:	mov    0x28(%rsp),%r9
   b0823:	mov    %ebx,%ecx
   b0825:	shr    $1,%ecx
   b0827:	and    $0x1,%ebx
   b082a:	neg    %ebx
   b082c:	xor    %ecx,%ebx
   b082e:	add    0x20(%rsp),%ebx
   b0832:	mov    %rax,%r14
   b0835:	mov    %ebx,0x20(%rsp)
   b0839:	cmp    $0xf,%r13b
   b083d:	sete   %al
   b0840:	cmp    $0xf,%r12b
   b0844:	sete   %cl
   b0847:	test   %r13b,%r13b
   b084a:	sete   %sil
   b084e:	test   %r12b,%r12b
   b0851:	sete   %r12b
   b0855:	mov    %r11d,0x100(%rsp,%r10,4)
   b085d:	lea    0x1(%r10),%r13
   b0861:	and    $0xf,%r13d
   b0865:	or     %cl,%r12b
   b0868:	mov    %r15d,0x100(%rsp,%r13,4)
   b0870:	movzbl %r12b,%ecx
   b0874:	add    %r10,%rcx
   b0877:	inc    %rcx
   b087a:	or     %al,%sil
   b087d:	movzbl %sil,%r10d
   b0881:	add    %rcx,%r10
   b0884:	and    $0xf,%ecx
   b0887:	mov    %ebx,0x100(%rsp,%rcx,4)
   b088e:	mov    %r15d,0x200(%rsp,%r8,8)
   b0896:	mov    %r11d,0x204(%rsp,%r8,8)
   b089e:	lea    0x1(%r8),%r13d
   b08a2:	and    $0xf,%r13d
   b08a6:	mov    $0x3,%eax
   b08ab:	mov    $0x2,%r12d
   b08b1:	mov    %r14,%rsi
   b08b4:	jmp    b032c <meshoptimizer_rs::codec::raw+0xc3c>
   b08b9:	cmp    $0x21,%rbp
   b08bd:	mov    $0x20,%eax
   b08c2:	cmovae %rbp,%rax
   b08c6:	mov    %rbp,%rcx
   b08c9:	mov    %rax,0xa0(%rsp)
   b08d1:	lea    0x1(%rax),%rdx
   b08d5:	mov    $0x7,%al
   b08d7:	cmp    %rdx,%rbx
   b08da:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b08e0:	mov    %sil,0x40(%rsp)
   b08e5:	mov    %rbx,%rsi
   b08e8:	sub    %rcx,%rsi
   b08eb:	lea    (%rsi,%rbp,1),%rax
   b08ef:	mov    %rbx,%rcx
   b08f2:	mov    %rax,0x68(%rsp)
   b08f7:	sub    %rax,%rcx
   b08fa:	mov    %rcx,0xb0(%rsp)
   b0902:	jb     b1326 <meshoptimizer_rs::codec::raw+0x1c36>
   b0908:	pxor   %xmm0,%xmm0
   b090c:	movdqa %xmm0,0x1f0(%rsp)
   b0915:	movdqa %xmm0,0x1e0(%rsp)
   b091e:	movdqa %xmm0,0x1d0(%rsp)
   b0927:	movdqa %xmm0,0x1c0(%rsp)
   b0930:	movdqa %xmm0,0x1b0(%rsp)
   b0939:	movdqa %xmm0,0x1a0(%rsp)
   b0942:	movdqa %xmm0,0x190(%rsp)
   b094b:	movdqa %xmm0,0x180(%rsp)
   b0954:	movdqa %xmm0,0x170(%rsp)
   b095d:	movdqa %xmm0,0x160(%rsp)
   b0966:	movdqa %xmm0,0x150(%rsp)
   b096f:	movdqa %xmm0,0x140(%rsp)
   b0978:	movdqa %xmm0,0x130(%rsp)
   b0981:	movdqa %xmm0,0x120(%rsp)
   b098a:	movdqa %xmm0,0x110(%rsp)
   b0993:	movdqa %xmm0,0x100(%rsp)
   b099c:	cmp    $0x101,%rbp
   b09a3:	jae    b12e7 <meshoptimizer_rs::codec::raw+0x1bf7>
   b09a9:	lea    (%rsi,%rbp,1),%rax
   b09ad:	cmp    %rsi,%rax
   b09b0:	jb     b12fe <meshoptimizer_rs::codec::raw+0x1c0e>
   b09b6:	add    %rdi,%rsi
   b09b9:	lea    0x100(%rsp),%rdi
   b09c1:	mov    %rbp,%rdx
   b09c4:	call   *0x79e0e(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b09ca:	test   %rbp,%rbp
   b09cd:	je     b138f <meshoptimizer_rs::codec::raw+0x1c9f>
   b09d3:	mov    $0x2000,%ax
   b09d7:	xor    %edx,%edx
   b09d9:	div    %bp
   b09dc:	and    $0x3ff0,%eax
   b09e1:	cmp    $0x100,%eax
   b09e6:	mov    $0x100,%ecx
   b09eb:	cmovb  %eax,%ecx
   b09ee:	mov    %rcx,0xf0(%rsp)
   b09f6:	lea    0x200(%rsp),%rdi
   b09fe:	mov    $0x400,%edx
   b0a03:	xor    %esi,%esi
   b0a05:	call   *0x79ddd(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b0a0b:	test   %r13,%r13
   b0a0e:	je     b1295 <meshoptimizer_rs::codec::raw+0x1ba5>
   b0a14:	mov    %r12,0xe8(%rsp)
   b0a1c:	add    %r14,0x68(%rsp)
   b0a21:	mov    %rbp,%rax
   b0a24:	shr    $0x2,%rax
   b0a28:	mov    %rax,0x98(%rsp)
   b0a30:	lea    0x3(%rbp),%rax
   b0a34:	shr    $0x2,%rax
   b0a38:	mov    %rax,0xf8(%rsp)
   b0a40:	mov    $0x1,%r12d
   b0a46:	xor    %edi,%edi
   b0a48:	mov    %rbx,0x38(%rsp)
   b0a4d:	mov    %r14,0x30(%rsp)
   b0a52:	mov    %r15,0xd0(%rsp)
   b0a5a:	mov    %r13,%rcx
   b0a5d:	sub    %rdi,%rcx
   b0a60:	mov    0xf0(%rsp),%rax
   b0a68:	cmp    %rax,%rcx
   b0a6b:	cmovae %rax,%rcx
   b0a6f:	mov    %rcx,0x20(%rsp)
   b0a74:	cmpb   $0x0,0x40(%rsp)
   b0a79:	je     b0aba <meshoptimizer_rs::codec::raw+0x13ca>
   b0a7b:	mov    %r12,%rcx
   b0a7e:	mov    $0x7,%al
   b0a80:	add    0x98(%rsp),%rcx
   b0a88:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0a8e:	cmp    %rbx,%rcx
   b0a91:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0a97:	mov    %r14,%rdx
   b0a9a:	add    %rdx,%r12
   b0a9d:	mov    0x98(%rsp),%rax
   b0aa5:	mov    %rax,0x90(%rsp)
   b0aad:	mov    %r12,0xb8(%rsp)
   b0ab5:	mov    %rcx,%r12
   b0ab8:	jmp    b0ad3 <meshoptimizer_rs::codec::raw+0x13e3>
   b0aba:	movq   $0x0,0x90(%rsp)
   b0ac6:	mov    $0x1,%eax
   b0acb:	mov    %rax,0xb8(%rsp)
   b0ad3:	mov    0x20(%rsp),%rax
   b0ad8:	lea    (%rax,%rdi,1),%rsi
   b0adc:	imul   %rbp,%rdi
   b0ae0:	mov    %rsi,0xe0(%rsp)
   b0ae8:	imul   %rbp,%rsi
   b0aec:	cmp    %rdi,%rsi
   b0aef:	jb     b1316 <meshoptimizer_rs::codec::raw+0x1c26>
   b0af5:	cmp    %r15,%rsi
   b0af8:	ja     b1316 <meshoptimizer_rs::codec::raw+0x1c26>
   b0afe:	mov    0x20(%rsp),%rax
   b0b03:	lea    0xf(%rax),%ecx
   b0b06:	and    $0x3f0,%ecx
   b0b0c:	mov    %rcx,0x28(%rsp)
   b0b11:	mov    %rax,%rcx
   b0b14:	imul   %rbp,%rcx
   b0b18:	mov    %rcx,0xc8(%rsp)
   b0b20:	add    0xe8(%rsp),%rdi
   b0b28:	mov    %rdi,0xc0(%rsp)
   b0b30:	lea    -0x1(%rax),%rcx
   b0b34:	imul   %rbp,%rcx
   b0b38:	mov    %rcx,0xd8(%rsp)
   b0b40:	add    $0x4,%rcx
   b0b44:	mov    %rcx,0x58(%rsp)
   b0b49:	lea    0x0(,%rax,4),%rcx
   b0b51:	mov    %rcx,0x70(%rsp)
   b0b56:	lea    0x200(%rsp,%rax,1),%rcx
   b0b5e:	mov    %rcx,0x88(%rsp)
   b0b66:	lea    0x200(%rsp,%rax,2),%rcx
   b0b6e:	mov    %rcx,0x80(%rsp)
   b0b76:	lea    (%rax,%rax,2),%rax
   b0b7a:	add    %rsp,%rax
   b0b7d:	add    $0x200,%rax
   b0b83:	mov    %rax,0x78(%rsp)
   b0b88:	xor    %r15d,%r15d
   b0b8b:	xor    %r14d,%r14d
   b0b8e:	movzbl 0x40(%rsp),%edx
   b0b93:	jmp    b0bf4 <meshoptimizer_rs::codec::raw+0x1504>
   b0b95:	lea    (%rsp,%r14,1),%rcx
   b0b99:	add    $0x100,%rcx
   b0ba0:	shr    $0x4,%al
   b0ba3:	neg    %al
   b0ba5:	and    $0x1f,%al
   b0ba7:	movzbl %al,%eax
   b0baa:	mov    %rcx,(%rsp)
   b0bae:	mov    %eax,0x8(%rsp)
   b0bb2:	movq   $0x0,0x10(%rsp)
   b0bbb:	lea    0x200(%rsp),%rdi
   b0bc3:	mov    0x70(%rsp),%rsi
   b0bc8:	mov    0x58(%rsp),%rcx
   b0bcd:	mov    0x20(%rsp),%r8
   b0bd2:	mov    %rbp,%r9
   b0bd5:	call   73000 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   b0bda:	inc    %r15
   b0bdd:	add    $0x4,%r14
   b0be1:	cmp    %r15,0xf8(%rsp)
   b0be9:	movzbl 0x40(%rsp),%edx
   b0bee:	je     b1210 <meshoptimizer_rs::codec::raw+0x1b20>
   b0bf4:	test   %dl,%dl
   b0bf6:	mov    %r14,0x60(%rsp)
   b0bfb:	je     b0c1a <meshoptimizer_rs::codec::raw+0x152a>
   b0bfd:	cmp    0x90(%rsp),%r15
   b0c05:	jae    b135f <meshoptimizer_rs::codec::raw+0x1c6f>
   b0c0b:	mov    0xb8(%rsp),%rax
   b0c13:	movzbl (%rax,%r15,1),%r14d
   b0c18:	jmp    b0c1d <meshoptimizer_rs::codec::raw+0x152d>
   b0c1a:	xor    %r14d,%r14d
   b0c1d:	mov    %r14d,%eax
   b0c20:	and    $0x3,%al
   b0c22:	movzbl %al,%ecx
   b0c25:	mov    0x28(%rsp),%rsi
   b0c2a:	cmp    %rsi,0x20(%rsp)
   b0c2f:	ja     b105e <meshoptimizer_rs::codec::raw+0x196e>
   b0c35:	cmp    $0x2,%al
   b0c37:	je     b0c76 <meshoptimizer_rs::codec::raw+0x1586>
   b0c39:	cmp    $0x3,%ecx
   b0c3c:	jne    b0c90 <meshoptimizer_rs::codec::raw+0x15a0>
   b0c3e:	mov    %r12,%rbx
   b0c41:	mov    $0x7,%al
   b0c43:	add    0x20(%rsp),%rbx
   b0c48:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0c4e:	cmp    0x38(%rsp),%rbx
   b0c53:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0c59:	add    0x30(%rsp),%r12
   b0c5e:	lea    0x200(%rsp),%rdi
   b0c66:	mov    %r12,%rsi
   b0c69:	mov    0x20(%rsp),%rdx
   b0c6e:	call   *0x79b64(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b0c74:	jmp    b0ce9 <meshoptimizer_rs::codec::raw+0x15f9>
   b0c76:	lea    0x200(%rsp),%rdi
   b0c7e:	xor    %esi,%esi
   b0c80:	mov    0x20(%rsp),%rdx
   b0c85:	call   *0x79b5d(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b0c8b:	mov    %r12,%rbx
   b0c8e:	jmp    b0ce9 <meshoptimizer_rs::codec::raw+0x15f9>
   b0c90:	test   %al,%al
   b0c92:	lea    -0xa3069(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b0c99:	lea    -0xa2f50(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b0ca0:	cmove  %rcx,%rax
   b0ca4:	test   %dl,%dl
   b0ca6:	lea    -0xa337d(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b0cad:	cmove  %rcx,%rax
   b0cb1:	mov    %rax,(%rsp)
   b0cb5:	lea    0x48(%rsp),%rdi
   b0cba:	mov    0x30(%rsp),%rsi
   b0cbf:	mov    0x38(%rsp),%rdx
   b0cc4:	mov    %r12,%rcx
   b0cc7:	lea    0x200(%rsp),%r8
   b0ccf:	mov    0x28(%rsp),%r9
   b0cd4:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b0cd9:	cmpb   $0x0,0x48(%rsp)
   b0cde:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b0ce4:	mov    0x50(%rsp),%rbx
   b0ce9:	mov    %r14d,%eax
   b0cec:	shr    $0x2,%al
   b0cef:	and    $0x3,%al
   b0cf1:	cmp    $0x3,%al
   b0cf3:	movzbl 0x40(%rsp),%edx
   b0cf8:	je     b0d1c <meshoptimizer_rs::codec::raw+0x162c>
   b0cfa:	movzbl %al,%ecx
   b0cfd:	cmp    $0x2,%ecx
   b0d00:	jne    b0d57 <meshoptimizer_rs::codec::raw+0x1667>
   b0d02:	mov    0x88(%rsp),%rdi
   b0d0a:	xor    %esi,%esi
   b0d0c:	mov    0x20(%rsp),%rdx
   b0d11:	call   *0x79ad1(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b0d17:	jmp    b0db0 <meshoptimizer_rs::codec::raw+0x16c0>
   b0d1c:	mov    %rbx,%r12
   b0d1f:	mov    $0x7,%al
   b0d21:	add    0x20(%rsp),%r12
   b0d26:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0d2c:	cmp    0x38(%rsp),%r12
   b0d31:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0d37:	add    0x30(%rsp),%rbx
   b0d3c:	mov    0x88(%rsp),%rdi
   b0d44:	mov    %rbx,%rsi
   b0d47:	mov    0x20(%rsp),%rdx
   b0d4c:	call   *0x79a86(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b0d52:	mov    %r12,%rbx
   b0d55:	jmp    b0db0 <meshoptimizer_rs::codec::raw+0x16c0>
   b0d57:	test   %al,%al
   b0d59:	lea    -0xa3130(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b0d60:	lea    -0xa3017(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b0d67:	cmove  %rcx,%rax
   b0d6b:	test   %dl,%dl
   b0d6d:	lea    -0xa3444(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b0d74:	cmove  %rcx,%rax
   b0d78:	mov    %rax,(%rsp)
   b0d7c:	lea    0x48(%rsp),%rdi
   b0d81:	mov    0x30(%rsp),%rsi
   b0d86:	mov    0x38(%rsp),%rdx
   b0d8b:	mov    %rbx,%rcx
   b0d8e:	mov    0x88(%rsp),%r8
   b0d96:	mov    0x28(%rsp),%r9
   b0d9b:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b0da0:	cmpb   $0x0,0x48(%rsp)
   b0da5:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b0dab:	mov    0x50(%rsp),%rbx
   b0db0:	mov    %r14d,%eax
   b0db3:	shr    $0x4,%al
   b0db6:	and    $0x3,%al
   b0db8:	cmp    $0x3,%al
   b0dba:	je     b0dde <meshoptimizer_rs::codec::raw+0x16ee>
   b0dbc:	movzbl %al,%ecx
   b0dbf:	cmp    $0x2,%ecx
   b0dc2:	jne    b0e19 <meshoptimizer_rs::codec::raw+0x1729>
   b0dc4:	mov    0x80(%rsp),%rdi
   b0dcc:	xor    %esi,%esi
   b0dce:	mov    0x20(%rsp),%rdx
   b0dd3:	call   *0x79a0f(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b0dd9:	jmp    b0e75 <meshoptimizer_rs::codec::raw+0x1785>
   b0dde:	mov    %rbx,%r12
   b0de1:	mov    $0x7,%al
   b0de3:	add    0x20(%rsp),%r12
   b0de8:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0dee:	cmp    0x38(%rsp),%r12
   b0df3:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0df9:	add    0x30(%rsp),%rbx
   b0dfe:	mov    0x80(%rsp),%rdi
   b0e06:	mov    %rbx,%rsi
   b0e09:	mov    0x20(%rsp),%rdx
   b0e0e:	call   *0x799c4(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b0e14:	mov    %r12,%rbx
   b0e17:	jmp    b0e75 <meshoptimizer_rs::codec::raw+0x1785>
   b0e19:	test   %al,%al
   b0e1b:	lea    -0xa31f2(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b0e22:	lea    -0xa30d9(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b0e29:	cmove  %rcx,%rax
   b0e2d:	cmpb   $0x0,0x40(%rsp)
   b0e32:	lea    -0xa3509(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b0e39:	cmove  %rcx,%rax
   b0e3d:	mov    %rax,(%rsp)
   b0e41:	lea    0x48(%rsp),%rdi
   b0e46:	mov    0x30(%rsp),%rsi
   b0e4b:	mov    0x38(%rsp),%rdx
   b0e50:	mov    %rbx,%rcx
   b0e53:	mov    0x80(%rsp),%r8
   b0e5b:	mov    0x28(%rsp),%r9
   b0e60:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b0e65:	cmpb   $0x0,0x48(%rsp)
   b0e6a:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b0e70:	mov    0x50(%rsp),%rbx
   b0e75:	shr    $0x6,%r14b
   b0e79:	cmp    $0x3,%r14b
   b0e7d:	je     b0ea2 <meshoptimizer_rs::codec::raw+0x17b2>
   b0e7f:	movzbl %r14b,%eax
   b0e83:	cmp    $0x2,%eax
   b0e86:	jne    b0ed7 <meshoptimizer_rs::codec::raw+0x17e7>
   b0e88:	mov    0x78(%rsp),%rdi
   b0e8d:	xor    %esi,%esi
   b0e8f:	mov    0x20(%rsp),%rdx
   b0e94:	call   *0x7994e(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b0e9a:	mov    %rbx,%r12
   b0e9d:	jmp    b0f31 <meshoptimizer_rs::codec::raw+0x1841>
   b0ea2:	mov    %rbx,%r12
   b0ea5:	mov    $0x7,%al
   b0ea7:	add    0x20(%rsp),%r12
   b0eac:	jb     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0eb2:	cmp    0x38(%rsp),%r12
   b0eb7:	ja     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b0ebd:	add    0x30(%rsp),%rbx
   b0ec2:	mov    0x78(%rsp),%rdi
   b0ec7:	mov    %rbx,%rsi
   b0eca:	mov    0x20(%rsp),%rdx
   b0ecf:	call   *0x79903(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b0ed5:	jmp    b0f31 <meshoptimizer_rs::codec::raw+0x1841>
   b0ed7:	test   %r14b,%r14b
   b0eda:	lea    -0xa32b1(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b0ee1:	lea    -0xa3198(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b0ee8:	cmove  %rcx,%rax
   b0eec:	cmpb   $0x0,0x40(%rsp)
   b0ef1:	lea    -0xa35c8(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b0ef8:	cmove  %rcx,%rax
   b0efc:	mov    %rax,(%rsp)
   b0f00:	lea    0x48(%rsp),%rdi
   b0f05:	mov    0x30(%rsp),%rsi
   b0f0a:	mov    0x38(%rsp),%rdx
   b0f0f:	mov    %rbx,%rcx
   b0f12:	mov    0x78(%rsp),%r8
   b0f17:	mov    0x28(%rsp),%r9
   b0f1c:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b0f21:	cmpb   $0x0,0x48(%rsp)
   b0f26:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b0f2c:	mov    0x50(%rsp),%r12
   b0f31:	cmpb   $0x0,0x40(%rsp)
   b0f36:	mov    0xc8(%rsp),%rsi
   b0f3e:	mov    0x60(%rsp),%r14
   b0f43:	je     b0f5f <meshoptimizer_rs::codec::raw+0x186f>
   b0f45:	cmp    0xb0(%rsp),%r15
   b0f4d:	jae    b1377 <meshoptimizer_rs::codec::raw+0x1c87>
   b0f53:	mov    0x68(%rsp),%rax
   b0f58:	movzbl (%rax,%r15,1),%eax
   b0f5d:	jmp    b0f61 <meshoptimizer_rs::codec::raw+0x1871>
   b0f5f:	xor    %eax,%eax
   b0f61:	mov    %rsi,%rdx
   b0f64:	sub    %r14,%rdx
   b0f67:	jb     b12d4 <meshoptimizer_rs::codec::raw+0x1be4>
   b0f6d:	cmp    %rdx,0x58(%rsp)
   b0f72:	ja     b12c0 <meshoptimizer_rs::codec::raw+0x1bd0>
   b0f78:	mov    0xc0(%rsp),%rcx
   b0f80:	lea    (%rcx,%r14,1),%rdx
   b0f84:	mov    %eax,%ecx
   b0f86:	and    $0x3,%cl
   b0f89:	movzbl %cl,%ecx
   b0f8c:	lea    -0xa06ab(%rip),%rsi        # 108e8 <libm::math::generic::sqrt::RSQRT_TAB+0x502>
   b0f93:	movslq (%rsi,%rcx,4),%rcx
   b0f97:	add    %rsi,%rcx
   b0f9a:	jmp    *%rcx
   b0f9c:	mov    %rdx,0xa8(%rsp)
   b0fa4:	mov    %r14,%rbx
   b0fa7:	add    %rsp,%r14
   b0faa:	add    $0x100,%r14
   b0fb1:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   b0fb6:	test   %al,%al
   b0fb8:	je     b1022 <meshoptimizer_rs::codec::raw+0x1932>
   b0fba:	mov    %r14,(%rsp)
   b0fbe:	lea    0x200(%rsp),%rdi
   b0fc6:	mov    0x70(%rsp),%rsi
   b0fcb:	mov    0xa8(%rsp),%rdx
   b0fd3:	mov    0x58(%rsp),%rcx
   b0fd8:	mov    0x20(%rsp),%r8
   b0fdd:	mov    %rbp,%r9
   b0fe0:	call   75d90 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   b0fe5:	jmp    b1056 <meshoptimizer_rs::codec::raw+0x1966>
   b0fe7:	lea    (%rsp,%r14,1),%rax
   b0feb:	add    $0x100,%rax
   b0ff1:	mov    %rax,(%rsp)
   b0ff5:	movq   $0x0,0x8(%rsp)
   b0ffe:	lea    0x200(%rsp),%rdi
   b1006:	mov    0x70(%rsp),%rsi
   b100b:	mov    0x58(%rsp),%rcx
   b1010:	mov    0x20(%rsp),%r8
   b1015:	mov    %rbp,%r9
   b1018:	call   72e40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   b101d:	jmp    b0bda <meshoptimizer_rs::codec::raw+0x14ea>
   b1022:	mov    %r14,(%rsp)
   b1026:	movq   $0x0,0x8(%rsp)
   b102f:	lea    0x200(%rsp),%rdi
   b1037:	mov    0x70(%rsp),%rsi
   b103c:	mov    0xa8(%rsp),%rdx
   b1044:	mov    0x58(%rsp),%rcx
   b1049:	mov    0x20(%rsp),%r8
   b104e:	mov    %rbp,%r9
   b1051:	call   72c50 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   b1056:	mov    %rbx,%r14
   b1059:	jmp    b0bda <meshoptimizer_rs::codec::raw+0x14ea>
   b105e:	cmp    $0x2,%al
   b1060:	je     b139c <meshoptimizer_rs::codec::raw+0x1cac>
   b1066:	cmp    $0x3,%ecx
   b1069:	je     b13b5 <meshoptimizer_rs::codec::raw+0x1cc5>
   b106f:	test   %al,%al
   b1071:	lea    -0xa3448(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b1078:	lea    -0xa332f(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b107f:	cmove  %rcx,%rax
   b1083:	test   %dl,%dl
   b1085:	lea    -0xa375c(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b108c:	cmove  %rcx,%rax
   b1090:	mov    %rax,(%rsp)
   b1094:	lea    0x48(%rsp),%rdi
   b1099:	mov    0x30(%rsp),%rsi
   b109e:	mov    0x38(%rsp),%rdx
   b10a3:	mov    %r12,%rcx
   b10a6:	lea    0x200(%rsp),%r8
   b10ae:	mov    0x28(%rsp),%r9
   b10b3:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b10b8:	cmpb   $0x0,0x48(%rsp)
   b10bd:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b10c3:	mov    %r14d,%eax
   b10c6:	shr    $0x2,%al
   b10c9:	and    $0x3,%al
   b10cb:	cmp    $0x2,%al
   b10cd:	movzbl 0x40(%rsp),%edx
   b10d2:	je     b139c <meshoptimizer_rs::codec::raw+0x1cac>
   b10d8:	movzbl %al,%ecx
   b10db:	cmp    $0x3,%ecx
   b10de:	je     b13b5 <meshoptimizer_rs::codec::raw+0x1cc5>
   b10e4:	test   %al,%al
   b10e6:	lea    -0xa34bd(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b10ed:	lea    -0xa33a4(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b10f4:	cmove  %rcx,%rax
   b10f8:	mov    0x50(%rsp),%rcx
   b10fd:	test   %dl,%dl
   b10ff:	lea    -0xa37d6(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b1106:	cmove  %rdx,%rax
   b110a:	mov    %rax,(%rsp)
   b110e:	lea    0x48(%rsp),%rdi
   b1113:	mov    0x30(%rsp),%rsi
   b1118:	mov    0x38(%rsp),%rdx
   b111d:	mov    0x88(%rsp),%r8
   b1125:	mov    0x28(%rsp),%r9
   b112a:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b112f:	cmpb   $0x0,0x48(%rsp)
   b1134:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b113a:	mov    %r14d,%eax
   b113d:	shr    $0x4,%al
   b1140:	and    $0x3,%al
   b1142:	cmp    $0x2,%al
   b1144:	movzbl 0x40(%rsp),%edx
   b1149:	je     b139c <meshoptimizer_rs::codec::raw+0x1cac>
   b114f:	movzbl %al,%ecx
   b1152:	cmp    $0x3,%ecx
   b1155:	je     b13b5 <meshoptimizer_rs::codec::raw+0x1cc5>
   b115b:	test   %al,%al
   b115d:	lea    -0xa3534(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b1164:	lea    -0xa341b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b116b:	cmove  %rcx,%rax
   b116f:	mov    0x50(%rsp),%rcx
   b1174:	test   %dl,%dl
   b1176:	lea    -0xa384d(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b117d:	cmove  %rdx,%rax
   b1181:	mov    %rax,(%rsp)
   b1185:	lea    0x48(%rsp),%rdi
   b118a:	mov    0x30(%rsp),%rsi
   b118f:	mov    0x38(%rsp),%rdx
   b1194:	mov    0x80(%rsp),%r8
   b119c:	mov    0x28(%rsp),%r9
   b11a1:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   b11a6:	cmpb   $0x0,0x48(%rsp)
   b11ab:	jne    b12b9 <meshoptimizer_rs::codec::raw+0x1bc9>
   b11b1:	shr    $0x6,%r14b
   b11b5:	cmp    $0x2,%r14b
   b11b9:	movzbl 0x40(%rsp),%edx
   b11be:	je     b139c <meshoptimizer_rs::codec::raw+0x1cac>
   b11c4:	movzbl %r14b,%eax
   b11c8:	cmp    $0x3,%eax
   b11cb:	je     b13b5 <meshoptimizer_rs::codec::raw+0x1cc5>
   b11d1:	test   %r14b,%r14b
   b11d4:	lea    -0xa35ab(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   b11db:	lea    -0xa3492(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   b11e2:	cmove  %rcx,%rax
   b11e6:	mov    0x50(%rsp),%rcx
   b11eb:	test   %dl,%dl
   b11ed:	lea    -0xa38c4(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   b11f4:	cmove  %rdx,%rax
   b11f8:	mov    %rax,(%rsp)
   b11fc:	lea    0x48(%rsp),%rdi
   b1201:	mov    0x30(%rsp),%rsi
   b1206:	mov    0x38(%rsp),%rdx
   b120b:	jmp    b0f12 <meshoptimizer_rs::codec::raw+0x1822>
   b1210:	mov    0xc8(%rsp),%rdx
   b1218:	mov    0xd8(%rsp),%rdi
   b1220:	cmp    %rdx,%rdi
   b1223:	mov    0x30(%rsp),%r14
   b1228:	ja     b133e <meshoptimizer_rs::codec::raw+0x1c4e>
   b122e:	mov    0xc0(%rsp),%rsi
   b1236:	add    %rdi,%rsi
   b1239:	lea    0x100(%rsp),%rdi
   b1241:	mov    %rbp,%rdx
   b1244:	call   *0x7958e(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   b124a:	mov    0xe0(%rsp),%rax
   b1252:	mov    %rax,%rdi
   b1255:	cmp    %r13,%rax
   b1258:	mov    0x38(%rsp),%rbx
   b125d:	mov    0xd0(%rsp),%r15
   b1265:	mov    0xa0(%rsp),%rax
   b126d:	jb     b0a5a <meshoptimizer_rs::codec::raw+0x136a>
   b1273:	jmp    b12a3 <meshoptimizer_rs::codec::raw+0x1bb3>
   b1275:	mov    0x28(%rsp),%r9
   b127a:	mov    $0xff,%al
   b127c:	cmp    %r9,%rsi
   b127f:	je     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b1281:	mov    $0x7,%al
   b1283:	add    $0x608,%rsp
   b128a:	pop    %rbx
   b128b:	pop    %r12
   b128d:	pop    %r13
   b128f:	pop    %r14
   b1291:	pop    %r15
   b1293:	pop    %rbp
   b1294:	ret
   b1295:	mov    $0x1,%r12d
   b129b:	mov    0xa0(%rsp),%rax
   b12a3:	sub    %r12,%rbx
   b12a6:	setae  %cl
   b12a9:	cmp    %rax,%rbx
   b12ac:	sete   %dl
   b12af:	mov    $0x7,%al
   b12b1:	test   %dl,%cl
   b12b3:	je     b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b12b5:	mov    $0xff,%al
   b12b7:	jmp    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b12b9:	movzbl 0x49(%rsp),%eax
   b12be:	jmp    b1283 <meshoptimizer_rs::codec::raw+0x1b93>
   b12c0:	lea    0x74171(%rip),%rcx        # 125438 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   b12c7:	xor    %edi,%edi
   b12c9:	mov    0x58(%rsp),%rsi
   b12ce:	call   *0x7965c(%rip)        # 12a930 <_DYNAMIC+0x370>
   b12d4:	lea    0x74175(%rip),%rcx        # 125450 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   b12db:	mov    %rsi,%rdx
   b12de:	mov    %r14,%rdi
   b12e1:	call   *0x79649(%rip)        # 12a930 <_DYNAMIC+0x370>
   b12e7:	lea    0x741c2(%rip),%rcx        # 1254b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   b12ee:	mov    $0x100,%edx
   b12f3:	xor    %edi,%edi
   b12f5:	mov    %rbp,%rsi
   b12f8:	call   *0x79632(%rip)        # 12a930 <_DYNAMIC+0x370>
   b12fe:	lea    0x74193(%rip),%rcx        # 125498 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   b1305:	mov    %rsi,%rdi
   b1308:	mov    0x68(%rsp),%rsi
   b130d:	mov    %rbx,%rdx
   b1310:	call   *0x7961a(%rip)        # 12a930 <_DYNAMIC+0x370>
   b1316:	lea    0x779a3(%rip),%rcx        # 128cc0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a58>
   b131d:	mov    %r15,%rdx
   b1320:	call   *0x7960a(%rip)        # 12a930 <_DYNAMIC+0x370>
   b1326:	lea    0x7419b(%rip),%rcx        # 1254c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   b132d:	mov    0x68(%rsp),%rdi
   b1332:	mov    %rbx,%rsi
   b1335:	mov    %rbx,%rdx
   b1338:	call   *0x795f2(%rip)        # 12a930 <_DYNAMIC+0x370>
   b133e:	lea    0x740ab(%rip),%rcx        # 1253f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   b1345:	mov    %rdx,%rsi
   b1348:	call   *0x795e2(%rip)        # 12a930 <_DYNAMIC+0x370>
   b134e:	lea    0x7715b(%rip),%rdx        # 1284b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5248>
   b1355:	xor    %edi,%edi
   b1357:	xor    %esi,%esi
   b1359:	call   *0x794b1(%rip)        # 12a810 <_DYNAMIC+0x250>
   b135f:	lea    0x740a2(%rip),%rdx        # 125408 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   b1366:	mov    %r15,%rdi
   b1369:	mov    0x90(%rsp),%rsi
   b1371:	call   *0x79499(%rip)        # 12a810 <_DYNAMIC+0x250>
   b1377:	lea    0x740a2(%rip),%rdx        # 125420 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   b137e:	mov    %r15,%rdi
   b1381:	mov    0xb0(%rsp),%rsi
   b1389:	call   *0x79481(%rip)        # 12a810 <_DYNAMIC+0x250>
   b138f:	lea    0x74042(%rip),%rdi        # 1253d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   b1396:	call   *0x7965c(%rip)        # 12a9f8 <_DYNAMIC+0x438>
   b139c:	lea    0x740dd(%rip),%rcx        # 125480 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   b13a3:	xor    %edi,%edi
   b13a5:	mov    0x20(%rsp),%rsi
   b13aa:	mov    0x28(%rsp),%rdx
   b13af:	call   *0x7957b(%rip)        # 12a930 <_DYNAMIC+0x370>
   b13b5:	lea    0x740ac(%rip),%rcx        # 125468 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   b13bc:	xor    %edi,%edi
   b13be:	mov    0x20(%rsp),%rsi
   b13c3:	mov    0x28(%rsp),%rdx
   b13c8:	call   *0x79562(%rip)        # 12a930 <_DYNAMIC+0x370>
