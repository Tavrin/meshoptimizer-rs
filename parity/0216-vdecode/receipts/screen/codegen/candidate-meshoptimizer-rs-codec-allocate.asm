
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000b49d0 <meshoptimizer_rs::codec::allocate>:
   b49d0:	push   %rbp
   b49d1:	push   %r15
   b49d3:	push   %r14
   b49d5:	push   %r13
   b49d7:	push   %r12
   b49d9:	push   %rbx
   b49da:	sub    $0x468,%rsp
   b49e1:	mov    %r8,%r14
   b49e4:	mov    %edx,%r8d
   b49e7:	mov    0x4a8(%rsp),%r12
   b49ef:	movb   $0x0,0x365(%r12)
   b49f8:	movl   $0x0,0x361(%r12)
   b4a04:	xorps  %xmm0,%xmm0
   b4a07:	movups %xmm0,0x350(%r12)
   b4a10:	mov    0x348(%r12),%r15
   b4a18:	mov    %r15,0x48(%rsp)
   b4a1d:	mov    %r15,0x50(%rsp)
   b4a22:	movzbl %sil,%r13d
   b4a26:	test   %r13b,%r13b
   b4a29:	je     b4a60 <meshoptimizer_rs::codec::allocate+0x90>
   b4a2b:	cmp    $0x1,%r13d
   b4a2f:	jne    b4a7e <meshoptimizer_rs::codec::allocate+0xae>
   b4a31:	cmp    $0x2,%r14
   b4a35:	je     b4a3f <meshoptimizer_rs::codec::allocate+0x6f>
   b4a37:	mov    $0x2,%al
   b4a39:	cmp    $0x4,%r14
   b4a3d:	jne    b4aa4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4a3f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b4a49:	imul   %rcx,%rax
   b4a4d:	movabs $0x5555555555555556,%rdx
   b4a57:	cmp    %rdx,%rax
   b4a5a:	jb     b4a8c <meshoptimizer_rs::codec::allocate+0xbc>
   b4a5c:	xor    %eax,%eax
   b4a5e:	jmp    b4aa4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4a60:	lea    -0x1(%r14),%rax
   b4a64:	cmp    $0x100,%rax
   b4a6a:	setb   %r10b
   b4a6e:	test   $0x3,%r14b
   b4a72:	sete   %dl
   b4a75:	mov    $0x2,%al
   b4a77:	test   %dl,%r10b
   b4a7a:	jne    b4a8c <meshoptimizer_rs::codec::allocate+0xbc>
   b4a7c:	jmp    b4aa4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4a7e:	cmp    $0x4,%r14
   b4a82:	je     b4a8c <meshoptimizer_rs::codec::allocate+0xbc>
   b4a84:	mov    $0x2,%al
   b4a86:	cmp    $0x2,%r14
   b4a8a:	jne    b4aa4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4a8c:	mov    %rcx,%rax
   b4a8f:	mul    %r14
   b4a92:	mov    %rax,%rbp
   b4a95:	seto   %al
   b4a98:	test   %rbp,%rbp
   b4a9b:	sets   %dl
   b4a9e:	or     %al,%dl
   b4aa0:	mov    $0x8,%al
   b4aa2:	je     b4ac0 <meshoptimizer_rs::codec::allocate+0xf0>
   b4aa4:	mov    %al,0x8(%rdi)
   b4aa7:	movq   $0xffffffffffffffff,(%rdi)
   b4aae:	add    $0x468,%rsp
   b4ab5:	pop    %rbx
   b4ab6:	pop    %r12
   b4ab8:	pop    %r13
   b4aba:	pop    %r14
   b4abc:	pop    %r15
   b4abe:	pop    %rbp
   b4abf:	ret
   b4ac0:	mov    %esi,0x3c(%rsp)
   b4ac4:	mov    %rcx,0x10(%rsp)
   b4ac9:	mov    %r9,0x30(%rsp)
   b4ace:	mov    %r8d,0x38(%rsp)
   b4ad3:	mov    %rdi,0x40(%rsp)
   b4ad8:	mov    %r12,%rdi
   b4adb:	mov    %rbp,%rsi
   b4ade:	call   9a1d0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b4ae3:	mov    %r15,%rbx
   b4ae6:	cmp    $0xff,%al
   b4ae8:	je     b4b06 <meshoptimizer_rs::codec::allocate+0x136>
   b4aea:	sub    %rbx,%r15
   b4aed:	mov    %r15,0x358(%r12)
   b4af5:	mov    0x40(%rsp),%rcx
   b4afa:	mov    %al,0x8(%rcx)
   b4afd:	movq   $0xffffffffffffffff,(%rcx)
   b4b04:	jmp    b4aae <meshoptimizer_rs::codec::allocate+0xde>
   b4b06:	mov    0x4a0(%rsp),%rdx
   b4b0e:	mov    %r15,%rax
   b4b11:	mov    %rdx,%rcx
   b4b14:	sub    %rdx,%rax
   b4b17:	jb     b4e86 <meshoptimizer_rs::codec::allocate+0x4b6>
   b4b1d:	mov    %rax,0x48(%rsp)
   b4b22:	mov    %rax,%rbx
   b4b25:	mov    %rbp,%r10
   b4b28:	sub    %rbp,%rbx
   b4b2b:	jb     b4e8e <meshoptimizer_rs::codec::allocate+0x4be>
   b4b31:	mov    %rbx,0x48(%rsp)
   b4b36:	cmpb   $0x0,0x38(%rsp)
   b4b3b:	mov    0x3c(%rsp),%ebp
   b4b3f:	je     b4b55 <meshoptimizer_rs::codec::allocate+0x185>
   b4b41:	mov    %r10,%rsi
   b4b44:	shr    $0x2,%rsi
   b4b48:	mov    %rbx,%rax
   b4b4b:	sub    %rsi,%rax
   b4b4e:	jae    b4b58 <meshoptimizer_rs::codec::allocate+0x188>
   b4b50:	jmp    b4e94 <meshoptimizer_rs::codec::allocate+0x4c4>
   b4b55:	mov    %rbx,%rax
   b4b58:	sub    %rax,%r15
   b4b5b:	mov    %r15,0x358(%r12)
   b4b63:	cmp    $0x2,%bpl
   b4b67:	mov    0x40(%rsp),%rbx
   b4b6c:	je     b4ba1 <meshoptimizer_rs::codec::allocate+0x1d1>
   b4b6e:	cmp    $0x1,%r13d
   b4b72:	mov    %rcx,%r15
   b4b75:	jne    b4c4d <meshoptimizer_rs::codec::allocate+0x27d>
   b4b7b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b4b85:	mov    0x10(%rsp),%rax
   b4b8a:	mul    %rcx
   b4b8d:	shr    $1,%rdx
   b4b90:	add    $0x11,%rdx
   b4b94:	mov    $0x7,%sil
   b4b97:	cmp    %rdx,%r15
   b4b9a:	jae    b4bb9 <meshoptimizer_rs::codec::allocate+0x1e9>
   b4b9c:	jmp    b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4ba1:	mov    0x10(%rsp),%rax
   b4ba6:	lea    0x5(%rax),%rdx
   b4baa:	mov    %rcx,%r15
   b4bad:	mov    $0x7,%sil
   b4bb0:	cmp    %rdx,%r15
   b4bb3:	jb     b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4bb9:	movq   $0x0,0x18(%rsp)
   b4bc2:	movq   $0x1,0x20(%rsp)
   b4bcb:	movq   $0x0,0x28(%rsp)
   b4bd4:	test   %r10,%r10
   b4bd7:	jne    b4d1d <meshoptimizer_rs::codec::allocate+0x34d>
   b4bdd:	cmp    $0x2,%bpl
   b4be1:	je     b4d69 <meshoptimizer_rs::codec::allocate+0x399>
   b4be7:	test   %r13d,%r13d
   b4bea:	jne    b4c90 <meshoptimizer_rs::codec::allocate+0x2c0>
   b4bf0:	cmp    $0x4,%r14
   b4bf4:	sete   %al
   b4bf7:	cmp    $0x100000,%r10
   b4bfe:	setae  %cl
   b4c01:	test   %cl,%al
   b4c03:	je     b4cc5 <meshoptimizer_rs::codec::allocate+0x2f5>
   b4c09:	lea    0x60(%rsp),%rdi
   b4c0e:	mov    $0x400,%edx
   b4c13:	xor    %esi,%esi
   b4c15:	call   *0x75b7d(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   b4c1b:	lea    0x18(%rsp),%rax
   b4c20:	mov    %rax,0x58(%rsp)
   b4c25:	movq   $0x0,0x460(%rsp)
   b4c31:	lea    0x58(%rsp),%rdi
   b4c36:	mov    0x10(%rsp),%rsi
   b4c3b:	mov    0x30(%rsp),%rdx
   b4c40:	mov    %r15,%rcx
   b4c43:	call   73eb0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b4c48:	jmp    b4d83 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4c4d:	mov    $0x7,%sil
   b4c50:	test   %r15,%r15
   b4c53:	je     b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4c59:	mov    0x30(%rsp),%rax
   b4c5e:	movzbl (%rax),%ecx
   b4c61:	mov    %ecx,%eax
   b4c63:	and    $0xf0,%al
   b4c65:	cmp    $0xa0,%al
   b4c67:	jne    b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4c6d:	and    $0xf,%cl
   b4c70:	mov    $0x5,%sil
   b4c73:	cmp    $0x1,%cl
   b4c76:	ja     b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4c7c:	mov    0x10(%rsp),%rdi
   b4c81:	cmp    $0x21,%rdi
   b4c85:	jae    b4ce4 <meshoptimizer_rs::codec::allocate+0x314>
   b4c87:	xor    %eax,%eax
   b4c89:	xor    %esi,%esi
   b4c8b:	jmp    b4df3 <meshoptimizer_rs::codec::allocate+0x423>
   b4c90:	lea    0x18(%rsp),%rdi
   b4c95:	mov    %r10,%rsi
   b4c98:	call   93950 <<alloc::vec::Vec<u8>>::resize>
   b4c9d:	mov    0x20(%rsp),%rsi
   b4ca2:	mov    0x28(%rsp),%rdx
   b4ca7:	mov    %r15,(%rsp)
   b4cab:	mov    %r13d,%edi
   b4cae:	mov    0x10(%rsp),%rcx
   b4cb3:	mov    %r14,%r8
   b4cb6:	mov    0x30(%rsp),%r9
   b4cbb:	call   b0370 <meshoptimizer_rs::codec::raw>
   b4cc0:	jmp    b4d83 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4cc5:	lea    0x18(%rsp),%rdi
   b4cca:	mov    0x10(%rsp),%rsi
   b4ccf:	mov    %r14,%rdx
   b4cd2:	mov    0x30(%rsp),%rcx
   b4cd7:	mov    %r15,%r8
   b4cda:	call   731e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   b4cdf:	jmp    b4d83 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4ce4:	mov    $0x2000,%ax
   b4ce8:	xor    %edx,%edx
   b4cea:	div    %r14w
   b4cee:	and    $0x3ff0,%eax
   b4cf3:	cmp    $0x100,%eax
   b4cf8:	mov    $0x100,%esi
   b4cfd:	cmovb  %eax,%esi
   b4d00:	mov    %rdi,%rax
   b4d03:	shr    $0x20,%rax
   b4d07:	je     b4dde <meshoptimizer_rs::codec::allocate+0x40e>
   b4d0d:	mov    %rdi,%rax
   b4d10:	xor    %edx,%edx
   b4d12:	div    %rsi
   b4d15:	mov    %rdx,%rdi
   b4d18:	jmp    b4de6 <meshoptimizer_rs::codec::allocate+0x416>
   b4d1d:	lea    0x58(%rsp),%rdi
   b4d22:	mov    $0x1,%edx
   b4d27:	mov    $0x1,%r8d
   b4d2d:	mov    $0x1,%r9d
   b4d33:	xor    %esi,%esi
   b4d35:	mov    %r10,%rcx
   b4d38:	mov    %r10,%r12
   b4d3b:	call   94440 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b4d40:	cmpb   $0x0,0x58(%rsp)
   b4d45:	je     b4d4d <meshoptimizer_rs::codec::allocate+0x37d>
   b4d47:	movb   $0xa,0x8(%rbx)
   b4d4b:	jmp    b4da5 <meshoptimizer_rs::codec::allocate+0x3d5>
   b4d4d:	mov    0x60(%rsp),%rax
   b4d52:	mov    %rax,0x20(%rsp)
   b4d57:	mov    %r12,0x18(%rsp)
   b4d5c:	mov    %r12,%r10
   b4d5f:	cmp    $0x2,%bpl
   b4d63:	jne    b4be7 <meshoptimizer_rs::codec::allocate+0x217>
   b4d69:	lea    0x18(%rsp),%rdi
   b4d6e:	mov    0x10(%rsp),%rsi
   b4d73:	mov    %r14,%rdx
   b4d76:	mov    0x30(%rsp),%rcx
   b4d7b:	mov    %r15,%r8
   b4d7e:	call   d3ad0 <meshoptimizer_rs::codec::index::sequence_append>
   b4d83:	cmp    $0xff,%al
   b4d85:	jne    b4da2 <meshoptimizer_rs::codec::allocate+0x3d2>
   b4d87:	mov    0x20(%rsp),%rsi
   b4d8c:	mov    0x28(%rsp),%rdx
   b4d91:	movzbl 0x38(%rsp),%edi
   b4d96:	mov    %r14,%rcx
   b4d99:	call   b1380 <meshoptimizer_rs::codec::apply>
   b4d9e:	cmp    $0xff,%al
   b4da0:	je     b4dc8 <meshoptimizer_rs::codec::allocate+0x3f8>
   b4da2:	mov    %al,0x8(%rbx)
   b4da5:	movq   $0xffffffffffffffff,(%rbx)
   b4dac:	cmpq   $0x0,0x18(%rsp)
   b4db2:	je     b4aae <meshoptimizer_rs::codec::allocate+0xde>
   b4db8:	mov    0x20(%rsp),%rdi
   b4dbd:	call   *0x759b5(%rip)        # 12a778 <free@GLIBC_2.2.5>
   b4dc3:	jmp    b4aae <meshoptimizer_rs::codec::allocate+0xde>
   b4dc8:	mov    0x28(%rsp),%rax
   b4dcd:	mov    %rax,0x10(%rbx)
   b4dd1:	movups 0x18(%rsp),%xmm0
   b4dd6:	movups %xmm0,(%rbx)
   b4dd9:	jmp    b4aae <meshoptimizer_rs::codec::allocate+0xde>
   b4dde:	mov    %edi,%eax
   b4de0:	xor    %edx,%edx
   b4de2:	div    %esi
   b4de4:	mov    %edx,%edi
   b4de6:	shr    $0x4,%esi
   b4de9:	add    $0x3,%esi
   b4dec:	shr    $0x2,%esi
   b4def:	imul   %rax,%rsi
   b4df3:	test   %cl,%cl
   b4df5:	je     b4e21 <meshoptimizer_rs::codec::allocate+0x451>
   b4df7:	cmp    $0x1,%rdi
   b4dfb:	sbb    $0xffffffffffffffff,%rax
   b4dff:	mov    %r14,%rcx
   b4e02:	shr    $0x2,%rcx
   b4e06:	mul    %rcx
   b4e09:	jo     b4ea4 <meshoptimizer_rs::codec::allocate+0x4d4>
   b4e0f:	add    %r14,%rcx
   b4e12:	cmp    $0x19,%rcx
   b4e16:	mov    $0x18,%edx
   b4e1b:	cmovae %rcx,%rdx
   b4e1f:	jmp    b4e59 <meshoptimizer_rs::codec::allocate+0x489>
   b4e21:	test   %rdi,%rdi
   b4e24:	je     b4e42 <meshoptimizer_rs::codec::allocate+0x472>
   b4e26:	mov    %rdi,%rax
   b4e29:	shr    $0x4,%rax
   b4e2d:	and    $0xf,%edi
   b4e30:	cmp    $0x1,%rdi
   b4e34:	sbb    $0xffffffffffffffff,%rax
   b4e38:	add    $0x3,%rax
   b4e3c:	shr    $0x2,%rax
   b4e40:	jmp    b4e44 <meshoptimizer_rs::codec::allocate+0x474>
   b4e42:	xor    %eax,%eax
   b4e44:	add    %rsi,%rax
   b4e47:	mul    %r14
   b4e4a:	jo     b4ea4 <meshoptimizer_rs::codec::allocate+0x4d4>
   b4e4c:	cmp    $0x21,%r14
   b4e50:	mov    $0x20,%edx
   b4e55:	cmovae %r14,%rdx
   b4e59:	mov    $0x8,%sil
   b4e5c:	add    %rax,%rdx
   b4e5f:	jb     b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4e61:	cmp    $0xffffffffffffffff,%rdx
   b4e65:	je     b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4e67:	inc    %rdx
   b4e6a:	mov    $0x7,%sil
   b4e6d:	cmp    %rdx,%r15
   b4e70:	jae    b4bb9 <meshoptimizer_rs::codec::allocate+0x1e9>
   b4e76:	mov    %sil,0x8(%rbx)
   b4e7a:	movq   $0xffffffffffffffff,(%rbx)
   b4e81:	jmp    b4aae <meshoptimizer_rs::codec::allocate+0xde>
   b4e86:	mov    %rcx,%rsi
   b4e89:	mov    %r15,%rbx
   b4e8c:	jmp    b4e94 <meshoptimizer_rs::codec::allocate+0x4c4>
   b4e8e:	mov    %r10,%rsi
   b4e91:	mov    %rax,%rbx
   b4e94:	lea    0x48(%rsp),%rdi
   b4e99:	call   *0x75d99(%rip)        # 12ac38 <_DYNAMIC+0x6c8>
   b4e9f:	jmp    b4aea <meshoptimizer_rs::codec::allocate+0x11a>
   b4ea4:	mov    $0x8,%sil
   b4ea7:	jmp    b4e76 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4ea9:	mov    %rax,%rbx
   b4eac:	cmpq   $0x0,0x18(%rsp)
   b4eb2:	je     b4ebf <meshoptimizer_rs::codec::allocate+0x4ef>
   b4eb4:	mov    0x20(%rsp),%rdi
   b4eb9:	call   *0x758b9(%rip)        # 12a778 <free@GLIBC_2.2.5>
   b4ebf:	mov    %rbx,%rdi
   b4ec2:	call   1217c0 <_Unwind_Resume@plt>
