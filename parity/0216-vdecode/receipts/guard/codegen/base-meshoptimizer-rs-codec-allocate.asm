
<artifacts>/guard/driver-base:     file format elf64-x86-64


Disassembly of section .text:

00000000000c0950 <meshoptimizer_rs::codec::allocate>:
   c0950:	push   %rbp
   c0951:	push   %r15
   c0953:	push   %r14
   c0955:	push   %r13
   c0957:	push   %r12
   c0959:	push   %rbx
   c095a:	sub    $0x468,%rsp
   c0961:	mov    %r8,%r14
   c0964:	mov    %edx,%r8d
   c0967:	mov    0x4a8(%rsp),%r12
   c096f:	movb   $0x0,0x365(%r12)
   c0978:	movl   $0x0,0x361(%r12)
   c0984:	xorps  %xmm0,%xmm0
   c0987:	movups %xmm0,0x350(%r12)
   c0990:	mov    0x348(%r12),%r15
   c0998:	mov    %r15,0x48(%rsp)
   c099d:	mov    %r15,0x50(%rsp)
   c09a2:	movzbl %sil,%r13d
   c09a6:	test   %r13b,%r13b
   c09a9:	je     c09e0 <meshoptimizer_rs::codec::allocate+0x90>
   c09ab:	cmp    $0x1,%r13d
   c09af:	jne    c09fe <meshoptimizer_rs::codec::allocate+0xae>
   c09b1:	cmp    $0x2,%r14
   c09b5:	je     c09bf <meshoptimizer_rs::codec::allocate+0x6f>
   c09b7:	mov    $0x2,%al
   c09b9:	cmp    $0x4,%r14
   c09bd:	jne    c0a24 <meshoptimizer_rs::codec::allocate+0xd4>
   c09bf:	movabs $0xaaaaaaaaaaaaaaab,%rax
   c09c9:	imul   %rcx,%rax
   c09cd:	movabs $0x5555555555555556,%rdx
   c09d7:	cmp    %rdx,%rax
   c09da:	jb     c0a0c <meshoptimizer_rs::codec::allocate+0xbc>
   c09dc:	xor    %eax,%eax
   c09de:	jmp    c0a24 <meshoptimizer_rs::codec::allocate+0xd4>
   c09e0:	lea    -0x1(%r14),%rax
   c09e4:	cmp    $0x100,%rax
   c09ea:	setb   %r10b
   c09ee:	test   $0x3,%r14b
   c09f2:	sete   %dl
   c09f5:	mov    $0x2,%al
   c09f7:	test   %dl,%r10b
   c09fa:	jne    c0a0c <meshoptimizer_rs::codec::allocate+0xbc>
   c09fc:	jmp    c0a24 <meshoptimizer_rs::codec::allocate+0xd4>
   c09fe:	cmp    $0x4,%r14
   c0a02:	je     c0a0c <meshoptimizer_rs::codec::allocate+0xbc>
   c0a04:	mov    $0x2,%al
   c0a06:	cmp    $0x2,%r14
   c0a0a:	jne    c0a24 <meshoptimizer_rs::codec::allocate+0xd4>
   c0a0c:	mov    %rcx,%rax
   c0a0f:	mul    %r14
   c0a12:	mov    %rax,%rbp
   c0a15:	seto   %al
   c0a18:	test   %rbp,%rbp
   c0a1b:	sets   %dl
   c0a1e:	or     %al,%dl
   c0a20:	mov    $0x8,%al
   c0a22:	je     c0a40 <meshoptimizer_rs::codec::allocate+0xf0>
   c0a24:	mov    %al,0x8(%rdi)
   c0a27:	movq   $0xffffffffffffffff,(%rdi)
   c0a2e:	add    $0x468,%rsp
   c0a35:	pop    %rbx
   c0a36:	pop    %r12
   c0a38:	pop    %r13
   c0a3a:	pop    %r14
   c0a3c:	pop    %r15
   c0a3e:	pop    %rbp
   c0a3f:	ret
   c0a40:	mov    %esi,0x3c(%rsp)
   c0a44:	mov    %rcx,0x10(%rsp)
   c0a49:	mov    %r9,0x30(%rsp)
   c0a4e:	mov    %r8d,0x38(%rsp)
   c0a53:	mov    %rdi,0x40(%rsp)
   c0a58:	mov    %r12,%rdi
   c0a5b:	mov    %rbp,%rsi
   c0a5e:	call   a3990 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   c0a63:	mov    %r15,%rbx
   c0a66:	cmp    $0xff,%al
   c0a68:	je     c0a86 <meshoptimizer_rs::codec::allocate+0x136>
   c0a6a:	sub    %rbx,%r15
   c0a6d:	mov    %r15,0x358(%r12)
   c0a75:	mov    0x40(%rsp),%rcx
   c0a7a:	mov    %al,0x8(%rcx)
   c0a7d:	movq   $0xffffffffffffffff,(%rcx)
   c0a84:	jmp    c0a2e <meshoptimizer_rs::codec::allocate+0xde>
   c0a86:	mov    0x4a0(%rsp),%rdx
   c0a8e:	mov    %r15,%rax
   c0a91:	mov    %rdx,%rcx
   c0a94:	sub    %rdx,%rax
   c0a97:	jb     c0e06 <meshoptimizer_rs::codec::allocate+0x4b6>
   c0a9d:	mov    %rax,0x48(%rsp)
   c0aa2:	mov    %rax,%rbx
   c0aa5:	mov    %rbp,%r10
   c0aa8:	sub    %rbp,%rbx
   c0aab:	jb     c0e0e <meshoptimizer_rs::codec::allocate+0x4be>
   c0ab1:	mov    %rbx,0x48(%rsp)
   c0ab6:	cmpb   $0x0,0x38(%rsp)
   c0abb:	mov    0x3c(%rsp),%ebp
   c0abf:	je     c0ad5 <meshoptimizer_rs::codec::allocate+0x185>
   c0ac1:	mov    %r10,%rsi
   c0ac4:	shr    $0x2,%rsi
   c0ac8:	mov    %rbx,%rax
   c0acb:	sub    %rsi,%rax
   c0ace:	jae    c0ad8 <meshoptimizer_rs::codec::allocate+0x188>
   c0ad0:	jmp    c0e14 <meshoptimizer_rs::codec::allocate+0x4c4>
   c0ad5:	mov    %rbx,%rax
   c0ad8:	sub    %rax,%r15
   c0adb:	mov    %r15,0x358(%r12)
   c0ae3:	cmp    $0x2,%bpl
   c0ae7:	mov    0x40(%rsp),%rbx
   c0aec:	je     c0b21 <meshoptimizer_rs::codec::allocate+0x1d1>
   c0aee:	cmp    $0x1,%r13d
   c0af2:	mov    %rcx,%r15
   c0af5:	jne    c0bcd <meshoptimizer_rs::codec::allocate+0x27d>
   c0afb:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   c0b05:	mov    0x10(%rsp),%rax
   c0b0a:	mul    %rcx
   c0b0d:	shr    $1,%rdx
   c0b10:	add    $0x11,%rdx
   c0b14:	mov    $0x7,%sil
   c0b17:	cmp    %rdx,%r15
   c0b1a:	jae    c0b39 <meshoptimizer_rs::codec::allocate+0x1e9>
   c0b1c:	jmp    c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0b21:	mov    0x10(%rsp),%rax
   c0b26:	lea    0x5(%rax),%rdx
   c0b2a:	mov    %rcx,%r15
   c0b2d:	mov    $0x7,%sil
   c0b30:	cmp    %rdx,%r15
   c0b33:	jb     c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0b39:	movq   $0x0,0x18(%rsp)
   c0b42:	movq   $0x1,0x20(%rsp)
   c0b4b:	movq   $0x0,0x28(%rsp)
   c0b54:	test   %r10,%r10
   c0b57:	jne    c0c9d <meshoptimizer_rs::codec::allocate+0x34d>
   c0b5d:	cmp    $0x2,%bpl
   c0b61:	je     c0ce9 <meshoptimizer_rs::codec::allocate+0x399>
   c0b67:	test   %r13d,%r13d
   c0b6a:	jne    c0c10 <meshoptimizer_rs::codec::allocate+0x2c0>
   c0b70:	cmp    $0x4,%r14
   c0b74:	sete   %al
   c0b77:	cmp    $0x100000,%r10
   c0b7e:	setae  %cl
   c0b81:	test   %cl,%al
   c0b83:	je     c0c45 <meshoptimizer_rs::codec::allocate+0x2f5>
   c0b89:	lea    0x60(%rsp),%rdi
   c0b8e:	mov    $0x400,%edx
   c0b93:	xor    %esi,%esi
   c0b95:	call   *0x883f5(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   c0b9b:	lea    0x18(%rsp),%rax
   c0ba0:	mov    %rax,0x58(%rsp)
   c0ba5:	movq   $0x0,0x460(%rsp)
   c0bb1:	lea    0x58(%rsp),%rdi
   c0bb6:	mov    0x10(%rsp),%rsi
   c0bbb:	mov    0x30(%rsp),%rdx
   c0bc0:	mov    %r15,%rcx
   c0bc3:	call   7ddf0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   c0bc8:	jmp    c0d03 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0bcd:	mov    $0x7,%sil
   c0bd0:	test   %r15,%r15
   c0bd3:	je     c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0bd9:	mov    0x30(%rsp),%rax
   c0bde:	movzbl (%rax),%ecx
   c0be1:	mov    %ecx,%eax
   c0be3:	and    $0xf0,%al
   c0be5:	cmp    $0xa0,%al
   c0be7:	jne    c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0bed:	and    $0xf,%cl
   c0bf0:	mov    $0x5,%sil
   c0bf3:	cmp    $0x1,%cl
   c0bf6:	ja     c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0bfc:	mov    0x10(%rsp),%rdi
   c0c01:	cmp    $0x21,%rdi
   c0c05:	jae    c0c64 <meshoptimizer_rs::codec::allocate+0x314>
   c0c07:	xor    %eax,%eax
   c0c09:	xor    %esi,%esi
   c0c0b:	jmp    c0d73 <meshoptimizer_rs::codec::allocate+0x423>
   c0c10:	lea    0x18(%rsp),%rdi
   c0c15:	mov    %r10,%rsi
   c0c18:	call   9cf60 <<alloc::vec::Vec<u8>>::resize>
   c0c1d:	mov    0x20(%rsp),%rsi
   c0c22:	mov    0x28(%rsp),%rdx
   c0c27:	mov    %r15,(%rsp)
   c0c2b:	mov    %r13d,%edi
   c0c2e:	mov    0x10(%rsp),%rcx
   c0c33:	mov    %r14,%r8
   c0c36:	mov    0x30(%rsp),%r9
   c0c3b:	call   bb670 <meshoptimizer_rs::codec::raw>
   c0c40:	jmp    c0d03 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0c45:	lea    0x18(%rsp),%rdi
   c0c4a:	mov    0x10(%rsp),%rsi
   c0c4f:	mov    %r14,%rdx
   c0c52:	mov    0x30(%rsp),%rcx
   c0c57:	mov    %r15,%r8
   c0c5a:	call   7d150 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   c0c5f:	jmp    c0d03 <meshoptimizer_rs::codec::allocate+0x3b3>
   c0c64:	mov    $0x2000,%ax
   c0c68:	xor    %edx,%edx
   c0c6a:	div    %r14w
   c0c6e:	and    $0x3ff0,%eax
   c0c73:	cmp    $0x100,%eax
   c0c78:	mov    $0x100,%esi
   c0c7d:	cmovb  %eax,%esi
   c0c80:	mov    %rdi,%rax
   c0c83:	shr    $0x20,%rax
   c0c87:	je     c0d5e <meshoptimizer_rs::codec::allocate+0x40e>
   c0c8d:	mov    %rdi,%rax
   c0c90:	xor    %edx,%edx
   c0c92:	div    %rsi
   c0c95:	mov    %rdx,%rdi
   c0c98:	jmp    c0d66 <meshoptimizer_rs::codec::allocate+0x416>
   c0c9d:	lea    0x58(%rsp),%rdi
   c0ca2:	mov    $0x1,%edx
   c0ca7:	mov    $0x1,%r8d
   c0cad:	mov    $0x1,%r9d
   c0cb3:	xor    %esi,%esi
   c0cb5:	mov    %r10,%rcx
   c0cb8:	mov    %r10,%r12
   c0cbb:	call   9da50 <<alloc::raw_vec::RawVecInner>::finish_grow>
   c0cc0:	cmpb   $0x0,0x58(%rsp)
   c0cc5:	je     c0ccd <meshoptimizer_rs::codec::allocate+0x37d>
   c0cc7:	movb   $0xa,0x8(%rbx)
   c0ccb:	jmp    c0d25 <meshoptimizer_rs::codec::allocate+0x3d5>
   c0ccd:	mov    0x60(%rsp),%rax
   c0cd2:	mov    %rax,0x20(%rsp)
   c0cd7:	mov    %r12,0x18(%rsp)
   c0cdc:	mov    %r12,%r10
   c0cdf:	cmp    $0x2,%bpl
   c0ce3:	jne    c0b67 <meshoptimizer_rs::codec::allocate+0x217>
   c0ce9:	lea    0x18(%rsp),%rdi
   c0cee:	mov    0x10(%rsp),%rsi
   c0cf3:	mov    %r14,%rdx
   c0cf6:	mov    0x30(%rsp),%rcx
   c0cfb:	mov    %r15,%r8
   c0cfe:	call   e5d70 <meshoptimizer_rs::codec::index::sequence_append>
   c0d03:	cmp    $0xff,%al
   c0d05:	jne    c0d22 <meshoptimizer_rs::codec::allocate+0x3d2>
   c0d07:	mov    0x20(%rsp),%rsi
   c0d0c:	mov    0x28(%rsp),%rdx
   c0d11:	movzbl 0x38(%rsp),%edi
   c0d16:	mov    %r14,%rcx
   c0d19:	call   bd300 <meshoptimizer_rs::codec::apply>
   c0d1e:	cmp    $0xff,%al
   c0d20:	je     c0d48 <meshoptimizer_rs::codec::allocate+0x3f8>
   c0d22:	mov    %al,0x8(%rbx)
   c0d25:	movq   $0xffffffffffffffff,(%rbx)
   c0d2c:	cmpq   $0x0,0x18(%rsp)
   c0d32:	je     c0a2e <meshoptimizer_rs::codec::allocate+0xde>
   c0d38:	mov    0x20(%rsp),%rdi
   c0d3d:	call   *0x8822d(%rip)        # 148f70 <free@GLIBC_2.2.5>
   c0d43:	jmp    c0a2e <meshoptimizer_rs::codec::allocate+0xde>
   c0d48:	mov    0x28(%rsp),%rax
   c0d4d:	mov    %rax,0x10(%rbx)
   c0d51:	movups 0x18(%rsp),%xmm0
   c0d56:	movups %xmm0,(%rbx)
   c0d59:	jmp    c0a2e <meshoptimizer_rs::codec::allocate+0xde>
   c0d5e:	mov    %edi,%eax
   c0d60:	xor    %edx,%edx
   c0d62:	div    %esi
   c0d64:	mov    %edx,%edi
   c0d66:	shr    $0x4,%esi
   c0d69:	add    $0x3,%esi
   c0d6c:	shr    $0x2,%esi
   c0d6f:	imul   %rax,%rsi
   c0d73:	test   %cl,%cl
   c0d75:	je     c0da1 <meshoptimizer_rs::codec::allocate+0x451>
   c0d77:	cmp    $0x1,%rdi
   c0d7b:	sbb    $0xffffffffffffffff,%rax
   c0d7f:	mov    %r14,%rcx
   c0d82:	shr    $0x2,%rcx
   c0d86:	mul    %rcx
   c0d89:	jo     c0e24 <meshoptimizer_rs::codec::allocate+0x4d4>
   c0d8f:	add    %r14,%rcx
   c0d92:	cmp    $0x19,%rcx
   c0d96:	mov    $0x18,%edx
   c0d9b:	cmovae %rcx,%rdx
   c0d9f:	jmp    c0dd9 <meshoptimizer_rs::codec::allocate+0x489>
   c0da1:	test   %rdi,%rdi
   c0da4:	je     c0dc2 <meshoptimizer_rs::codec::allocate+0x472>
   c0da6:	mov    %rdi,%rax
   c0da9:	shr    $0x4,%rax
   c0dad:	and    $0xf,%edi
   c0db0:	cmp    $0x1,%rdi
   c0db4:	sbb    $0xffffffffffffffff,%rax
   c0db8:	add    $0x3,%rax
   c0dbc:	shr    $0x2,%rax
   c0dc0:	jmp    c0dc4 <meshoptimizer_rs::codec::allocate+0x474>
   c0dc2:	xor    %eax,%eax
   c0dc4:	add    %rsi,%rax
   c0dc7:	mul    %r14
   c0dca:	jo     c0e24 <meshoptimizer_rs::codec::allocate+0x4d4>
   c0dcc:	cmp    $0x21,%r14
   c0dd0:	mov    $0x20,%edx
   c0dd5:	cmovae %r14,%rdx
   c0dd9:	mov    $0x8,%sil
   c0ddc:	add    %rax,%rdx
   c0ddf:	jb     c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0de1:	cmp    $0xffffffffffffffff,%rdx
   c0de5:	je     c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0de7:	inc    %rdx
   c0dea:	mov    $0x7,%sil
   c0ded:	cmp    %rdx,%r15
   c0df0:	jae    c0b39 <meshoptimizer_rs::codec::allocate+0x1e9>
   c0df6:	mov    %sil,0x8(%rbx)
   c0dfa:	movq   $0xffffffffffffffff,(%rbx)
   c0e01:	jmp    c0a2e <meshoptimizer_rs::codec::allocate+0xde>
   c0e06:	mov    %rcx,%rsi
   c0e09:	mov    %r15,%rbx
   c0e0c:	jmp    c0e14 <meshoptimizer_rs::codec::allocate+0x4c4>
   c0e0e:	mov    %r10,%rsi
   c0e11:	mov    %rax,%rbx
   c0e14:	lea    0x48(%rsp),%rdi
   c0e19:	call   *0x88661(%rip)        # 149480 <_DYNAMIC+0x718>
   c0e1f:	jmp    c0a6a <meshoptimizer_rs::codec::allocate+0x11a>
   c0e24:	mov    $0x8,%sil
   c0e27:	jmp    c0df6 <meshoptimizer_rs::codec::allocate+0x4a6>
   c0e29:	mov    %rax,%rbx
   c0e2c:	cmpq   $0x0,0x18(%rsp)
   c0e32:	je     c0e3f <meshoptimizer_rs::codec::allocate+0x4ef>
   c0e34:	mov    0x20(%rsp),%rdi
   c0e39:	call   *0x88131(%rip)        # 148f70 <free@GLIBC_2.2.5>
   c0e3f:	mov    %rbx,%rdi
   c0e42:	call   13f3c0 <_Unwind_Resume@plt>
