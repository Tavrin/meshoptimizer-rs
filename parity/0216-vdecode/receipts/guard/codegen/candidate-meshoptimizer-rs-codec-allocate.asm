
<artifacts>/guard/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000c0810 <meshoptimizer_rs::codec::allocate>:
   c0810:	push   %rbp
   c0811:	push   %r15
   c0813:	push   %r14
   c0815:	push   %r13
   c0817:	push   %r12
   c0819:	push   %rbx
   c081a:	sub    $0x468,%rsp
   c0821:	mov    %r8,%r14
   c0824:	mov    %edx,%r8d
   c0827:	mov    0x4a8(%rsp),%r12
   c082f:	movb   $0x0,0x365(%r12)
   c0838:	movl   $0x0,0x361(%r12)
   c0844:	xorps  %xmm0,%xmm0
   c0847:	movups %xmm0,0x350(%r12)
   c0850:	mov    0x348(%r12),%r15
   c0858:	mov    %r15,0x48(%rsp)
   c085d:	mov    %r15,0x50(%rsp)
   c0862:	movzbl %sil,%r13d
   c0866:	test   %r13b,%r13b
   c0869:	je     c08a0 <meshoptimizer_rs::codec::allocate+0x90>
   c086b:	cmp    $0x1,%r13d
   c086f:	jne    c08be <meshoptimizer_rs::codec::allocate+0xae>
   c0871:	cmp    $0x2,%r14
   c0875:	je     c087f <meshoptimizer_rs::codec::allocate+0x6f>
   c0877:	mov    $0x2,%al
   c0879:	cmp    $0x4,%r14
   c087d:	jne    c08e4 <meshoptimizer_rs::codec::allocate+0xd4>
   c087f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   c0889:	imul   %rcx,%rax
   c088d:	movabs $0x5555555555555556,%rdx
   c0897:	cmp    %rdx,%rax
   c089a:	jb     c08cc <meshoptimizer_rs::codec::allocate+0xbc>
   c089c:	xor    %eax,%eax
   c089e:	jmp    c08e4 <meshoptimizer_rs::codec::allocate+0xd4>
   c08a0:	lea    -0x1(%r14),%rax
   c08a4:	cmp    $0x100,%rax
   c08aa:	setb   %r10b
   c08ae:	test   $0x3,%r14b
   c08b2:	sete   %dl
   c08b5:	mov    $0x2,%al
   c08b7:	test   %dl,%r10b
   c08ba:	jne    c08cc <meshoptimizer_rs::codec::allocate+0xbc>
   c08bc:	jmp    c08e4 <meshoptimizer_rs::codec::allocate+0xd4>
   c08be:	cmp    $0x4,%r14
   c08c2:	je     c08cc <meshoptimizer_rs::codec::allocate+0xbc>
   c08c4:	mov    $0x2,%al
   c08c6:	cmp    $0x2,%r14
   c08ca:	jne    c08e4 <meshoptimizer_rs::codec::allocate+0xd4>
   c08cc:	mov    %rcx,%rax
   c08cf:	mul    %r14
   c08d2:	mov    %rax,%rbp
   c08d5:	seto   %al
   c08d8:	test   %rbp,%rbp
   c08db:	sets   %dl
   c08de:	or     %al,%dl
   c08e0:	mov    $0x8,%al
   c08e2:	je     c0900 <meshoptimizer_rs::codec::allocate+0xf0>
   c08e4:	mov    %al,0x8(%rdi)
   c08e7:	movq   $0xffffffffffffffff,(%rdi)
   c08ee:	add    $0x468,%rsp
   c08f5:	pop    %rbx
   c08f6:	pop    %r12
   c08f8:	pop    %r13
   c08fa:	pop    %r14
   c08fc:	pop    %r15
   c08fe:	pop    %rbp
   c08ff:	ret
   c0900:	mov    %esi,0x3c(%rsp)
   c0904:	mov    %rcx,0x10(%rsp)
   c0909:	mov    %r9,0x30(%rsp)
   c090e:	mov    %r8d,0x38(%rsp)
   c0913:	mov    %rdi,0x40(%rsp)
   c0918:	mov    %r12,%rdi
   c091b:	mov    %rbp,%rsi
   c091e:	call   a5360 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   c0923:	mov    %r15,%rbx
   c0926:	cmp    $0xff,%al
   c0928:	je     c0946 <meshoptimizer_rs::codec::allocate+0x136>
   c092a:	sub    %rbx,%r15
   c092d:	mov    %r15,0x358(%r12)
   c0935:	mov    0x40(%rsp),%rcx
   c093a:	mov    %al,0x8(%rcx)
   c093d:	movq   $0xffffffffffffffff,(%rcx)
   c0944:	jmp    c08ee <meshoptimizer_rs::codec::allocate+0xde>
   c0946:	mov    0x4a0(%rsp),%rdx
   c094e:	mov    %r15,%rax
   c0951:	mov    %rdx,%rcx
   c0954:	sub    %rdx,%rax
   c0957:	jb     c0cc6 <meshoptimizer_rs::codec::allocate+0x4b6>
   c095d:	mov    %rax,0x48(%rsp)
   c0962:	mov    %rax,%rbx
   c0965:	mov    %rbp,%r10
   c0968:	sub    %rbp,%rbx
   c096b:	jb     c0cce <meshoptimizer_rs::codec::allocate+0x4be>
   c0971:	mov    %rbx,0x48(%rsp)
   c0976:	cmpb   $0x0,0x38(%rsp)
   c097b:	mov    0x3c(%rsp),%ebp
   c097f:	je     c0995 <meshoptimizer_rs::codec::allocate+0x185>
   c0981:	mov    %r10,%rsi
   c0984:	shr    $0x2,%rsi
   c0988:	mov    %rbx,%rax
   c098b:	sub    %rsi,%rax
   c098e:	jae    c0998 <meshoptimizer_rs::codec::allocate+0x188>
   c0990:	jmp    c0cd4 <meshoptimizer_rs::codec::allocate+0x4c4>
   c0995:	mov    %rbx,%rax
   c0998:	sub    %rax,%r15
   c099b:	mov    %r15,0x358(%r12)
   c09a3:	cmp    $0x2,%bpl
   c09a7:	mov    0x40(%rsp),%rbx
   c09ac:	je     c09e1 <meshoptimizer_rs::codec::allocate+0x1d1>
   c09ae:	cmp    $0x1,%r13d
   c09b2:	mov    %rcx,%r15
   c09b5:	jne    c0a8d <meshoptimizer_rs::codec::allocate+0x27d>
   c09bb:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   c09c5:	mov    0x10(%rsp),%rax
   c09ca:	mul    %rcx
   c09cd:	shr    $1,%rdx
   c09d0:	add    $0x11,%rdx
   c09d4:	mov    $0x7,%sil
   c09d7:	cmp    %rdx,%r15
   c09da:	jae    c09f9 <meshoptimizer_rs::codec::allocate+0x1e9>
   c09dc:	jmp    c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c09e1:	mov    0x10(%rsp),%rax
   c09e6:	lea    0x5(%rax),%rdx
   c09ea:	mov    %rcx,%r15
   c09ed:	mov    $0x7,%sil
   c09f0:	cmp    %rdx,%r15
   c09f3:	jb     c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c09f9:	movq   $0x0,0x18(%rsp)
   c0a02:	movq   $0x1,0x20(%rsp)
   c0a0b:	movq   $0x0,0x28(%rsp)
   c0a14:	test   %r10,%r10
   c0a17:	jne    c0b5d <meshoptimizer_rs::codec::allocate+0x34d>
   c0a1d:	cmp    $0x2,%bpl
   c0a21:	je     c0ba9 <meshoptimizer_rs::codec::allocate+0x399>
   c0a27:	test   %r13d,%r13d
   c0a2a:	jne    c0ad0 <meshoptimizer_rs::codec::allocate+0x2c0>
   c0a30:	cmp    $0x4,%r14
   c0a34:	sete   %al
   c0a37:	cmp    $0x100000,%r10
   c0a3e:	setae  %cl
   c0a41:	test   %cl,%al
   c0a43:	je     c0b05 <meshoptimizer_rs::codec::allocate+0x2f5>
   c0a49:	lea    0x60(%rsp),%rdi
   c0a4e:	mov    $0x400,%edx
   c0a53:	xor    %esi,%esi
   c0a55:	call   *0x8bbad(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   c0a5b:	lea    0x18(%rsp),%rax
   c0a60:	mov    %rax,0x58(%rsp)
   c0a65:	movq   $0x0,0x460(%rsp)
   c0a71:	lea    0x58(%rsp),%rdi
   c0a76:	mov    0x10(%rsp),%rsi
   c0a7b:	mov    0x30(%rsp),%rdx
   c0a80:	mov    %r15,%rcx
   c0a83:	call   7d760 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   c0a88:	jmp    c0bc3 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0a8d:	mov    $0x7,%sil
   c0a90:	test   %r15,%r15
   c0a93:	je     c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0a99:	mov    0x30(%rsp),%rax
   c0a9e:	movzbl (%rax),%ecx
   c0aa1:	mov    %ecx,%eax
   c0aa3:	and    $0xf0,%al
   c0aa5:	cmp    $0xa0,%al
   c0aa7:	jne    c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0aad:	and    $0xf,%cl
   c0ab0:	mov    $0x5,%sil
   c0ab3:	cmp    $0x1,%cl
   c0ab6:	ja     c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0abc:	mov    0x10(%rsp),%rdi
   c0ac1:	cmp    $0x21,%rdi
   c0ac5:	jae    c0b24 <meshoptimizer_rs::codec::allocate+0x314>
   c0ac7:	xor    %eax,%eax
   c0ac9:	xor    %esi,%esi
   c0acb:	jmp    c0c33 <meshoptimizer_rs::codec::allocate+0x423>
   c0ad0:	lea    0x18(%rsp),%rdi
   c0ad5:	mov    %r10,%rsi
   c0ad8:	call   9e930 <<alloc::vec::Vec<u8>>::resize>
   c0add:	mov    0x20(%rsp),%rsi
   c0ae2:	mov    0x28(%rsp),%rdx
   c0ae7:	mov    %r15,(%rsp)
   c0aeb:	mov    %r13d,%edi
   c0aee:	mov    0x10(%rsp),%rcx
   c0af3:	mov    %r14,%r8
   c0af6:	mov    0x30(%rsp),%r9
   c0afb:	call   bb530 <meshoptimizer_rs::codec::raw>
   c0b00:	jmp    c0bc3 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0b05:	lea    0x18(%rsp),%rdi
   c0b0a:	mov    0x10(%rsp),%rsi
   c0b0f:	mov    %r14,%rdx
   c0b12:	mov    0x30(%rsp),%rcx
   c0b17:	mov    %r15,%r8
   c0b1a:	call   e9c40 <meshoptimizer_rs::codec::vertex::decode_owned>
   c0b1f:	jmp    c0bc3 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0b24:	mov    $0x2000,%ax
   c0b28:	xor    %edx,%edx
   c0b2a:	div    %r14w
   c0b2e:	and    $0x3ff0,%eax
   c0b33:	cmp    $0x100,%eax
   c0b38:	mov    $0x100,%esi
   c0b3d:	cmovb  %eax,%esi
   c0b40:	mov    %rdi,%rax
   c0b43:	shr    $0x20,%rax
   c0b47:	je     c0c1e <meshoptimizer_rs::codec::allocate+0x40e>
   c0b4d:	mov    %rdi,%rax
   c0b50:	xor    %edx,%edx
   c0b52:	div    %rsi
   c0b55:	mov    %rdx,%rdi
   c0b58:	jmp    c0c26 <meshoptimizer_rs::codec::allocate+0x416>
   c0b5d:	lea    0x58(%rsp),%rdi
   c0b62:	mov    $0x1,%edx
   c0b67:	mov    $0x1,%r8d
   c0b6d:	mov    $0x1,%r9d
   c0b73:	xor    %esi,%esi
   c0b75:	mov    %r10,%rcx
   c0b78:	mov    %r10,%r12
   c0b7b:	call   9f420 <<alloc::raw_vec::RawVecInner>::finish_grow>
   c0b80:	cmpb   $0x0,0x58(%rsp)
   c0b85:	je     c0b8d <meshoptimizer_rs::codec::allocate+0x37d>
   c0b87:	movb   $0xa,0x8(%rbx)
   c0b8b:	jmp    c0be5 <meshoptimizer_rs::codec::allocate+0x3d5>
   c0b8d:	mov    0x60(%rsp),%rax
   c0b92:	mov    %rax,0x20(%rsp)
   c0b97:	mov    %r12,0x18(%rsp)
   c0b9c:	mov    %r12,%r10
   c0b9f:	cmp    $0x2,%bpl
   c0ba3:	jne    c0a27 <meshoptimizer_rs::codec::allocate+0x217>
   c0ba9:	lea    0x18(%rsp),%rdi
   c0bae:	mov    0x10(%rsp),%rsi
   c0bb3:	mov    %r14,%rdx
   c0bb6:	mov    0x30(%rsp),%rcx
   c0bbb:	mov    %r15,%r8
   c0bbe:	call   e6140 <meshoptimizer_rs::codec::index::sequence_append>
   c0bc3:	cmp    $0xff,%al
   c0bc5:	jne    c0be2 <meshoptimizer_rs::codec::allocate+0x3d2>
   c0bc7:	mov    0x20(%rsp),%rsi
   c0bcc:	mov    0x28(%rsp),%rdx
   c0bd1:	movzbl 0x38(%rsp),%edi
   c0bd6:	mov    %r14,%rcx
   c0bd9:	call   bd1c0 <meshoptimizer_rs::codec::apply>
   c0bde:	cmp    $0xff,%al
   c0be0:	je     c0c08 <meshoptimizer_rs::codec::allocate+0x3f8>
   c0be2:	mov    %al,0x8(%rbx)
   c0be5:	movq   $0xffffffffffffffff,(%rbx)
   c0bec:	cmpq   $0x0,0x18(%rsp)
   c0bf2:	je     c08ee <meshoptimizer_rs::codec::allocate+0xde>
   c0bf8:	mov    0x20(%rsp),%rdi
   c0bfd:	call   *0x8b9e5(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   c0c03:	jmp    c08ee <meshoptimizer_rs::codec::allocate+0xde>
   c0c08:	mov    0x28(%rsp),%rax
   c0c0d:	mov    %rax,0x10(%rbx)
   c0c11:	movups 0x18(%rsp),%xmm0
   c0c16:	movups %xmm0,(%rbx)
   c0c19:	jmp    c08ee <meshoptimizer_rs::codec::allocate+0xde>
   c0c1e:	mov    %edi,%eax
   c0c20:	xor    %edx,%edx
   c0c22:	div    %esi
   c0c24:	mov    %edx,%edi
   c0c26:	shr    $0x4,%esi
   c0c29:	add    $0x3,%esi
   c0c2c:	shr    $0x2,%esi
   c0c2f:	imul   %rax,%rsi
   c0c33:	test   %cl,%cl
   c0c35:	je     c0c61 <meshoptimizer_rs::codec::allocate+0x451>
   c0c37:	cmp    $0x1,%rdi
   c0c3b:	sbb    $0xffffffffffffffff,%rax
   c0c3f:	mov    %r14,%rcx
   c0c42:	shr    $0x2,%rcx
   c0c46:	mul    %rcx
   c0c49:	jo     c0ce4 <meshoptimizer_rs::codec::allocate+0x4d4>
   c0c4f:	add    %r14,%rcx
   c0c52:	cmp    $0x19,%rcx
   c0c56:	mov    $0x18,%edx
   c0c5b:	cmovae %rcx,%rdx
   c0c5f:	jmp    c0c99 <meshoptimizer_rs::codec::allocate+0x489>
   c0c61:	test   %rdi,%rdi
   c0c64:	je     c0c82 <meshoptimizer_rs::codec::allocate+0x472>
   c0c66:	mov    %rdi,%rax
   c0c69:	shr    $0x4,%rax
   c0c6d:	and    $0xf,%edi
   c0c70:	cmp    $0x1,%rdi
   c0c74:	sbb    $0xffffffffffffffff,%rax
   c0c78:	add    $0x3,%rax
   c0c7c:	shr    $0x2,%rax
   c0c80:	jmp    c0c84 <meshoptimizer_rs::codec::allocate+0x474>
   c0c82:	xor    %eax,%eax
   c0c84:	add    %rsi,%rax
   c0c87:	mul    %r14
   c0c8a:	jo     c0ce4 <meshoptimizer_rs::codec::allocate+0x4d4>
   c0c8c:	cmp    $0x21,%r14
   c0c90:	mov    $0x20,%edx
   c0c95:	cmovae %r14,%rdx
   c0c99:	mov    $0x8,%sil
   c0c9c:	add    %rax,%rdx
   c0c9f:	jb     c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0ca1:	cmp    $0xffffffffffffffff,%rdx
   c0ca5:	je     c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0ca7:	inc    %rdx
   c0caa:	mov    $0x7,%sil
   c0cad:	cmp    %rdx,%r15
   c0cb0:	jae    c09f9 <meshoptimizer_rs::codec::allocate+0x1e9>
   c0cb6:	mov    %sil,0x8(%rbx)
   c0cba:	movq   $0xffffffffffffffff,(%rbx)
   c0cc1:	jmp    c08ee <meshoptimizer_rs::codec::allocate+0xde>
   c0cc6:	mov    %rcx,%rsi
   c0cc9:	mov    %r15,%rbx
   c0ccc:	jmp    c0cd4 <meshoptimizer_rs::codec::allocate+0x4c4>
   c0cce:	mov    %r10,%rsi
   c0cd1:	mov    %rax,%rbx
   c0cd4:	lea    0x48(%rsp),%rdi
   c0cd9:	call   *0x8be11(%rip)        # 14caf0 <_DYNAMIC+0x710>
   c0cdf:	jmp    c092a <meshoptimizer_rs::codec::allocate+0x11a>
   c0ce4:	mov    $0x8,%sil
   c0ce7:	jmp    c0cb6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0ce9:	mov    %rax,%rbx
   c0cec:	cmpq   $0x0,0x18(%rsp)
   c0cf2:	je     c0cff <meshoptimizer_rs::codec::allocate+0x4ef>
   c0cf4:	mov    0x20(%rsp),%rdi
   c0cf9:	call   *0x8b8e9(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   c0cff:	mov    %rbx,%rdi
   c0d02:	call   1429c0 <_Unwind_Resume@plt>
