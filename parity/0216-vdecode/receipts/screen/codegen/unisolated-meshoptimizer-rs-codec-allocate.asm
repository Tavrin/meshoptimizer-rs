
<artifacts>/screen/driver-unisolated:     file format elf64-x86-64


Disassembly of section .text:

00000000000b4a20 <meshoptimizer_rs::codec::allocate>:
   b4a20:	push   %rbp
   b4a21:	push   %r15
   b4a23:	push   %r14
   b4a25:	push   %r13
   b4a27:	push   %r12
   b4a29:	push   %rbx
   b4a2a:	sub    $0x468,%rsp
   b4a31:	mov    %r8,%r14
   b4a34:	mov    %edx,%r8d
   b4a37:	mov    0x4a8(%rsp),%r12
   b4a3f:	movb   $0x0,0x365(%r12)
   b4a48:	movl   $0x0,0x361(%r12)
   b4a54:	xorps  %xmm0,%xmm0
   b4a57:	movups %xmm0,0x350(%r12)
   b4a60:	mov    0x348(%r12),%r15
   b4a68:	mov    %r15,0x48(%rsp)
   b4a6d:	mov    %r15,0x50(%rsp)
   b4a72:	movzbl %sil,%r13d
   b4a76:	test   %r13b,%r13b
   b4a79:	je     b4ab0 <meshoptimizer_rs::codec::allocate+0x90>
   b4a7b:	cmp    $0x1,%r13d
   b4a7f:	jne    b4ace <meshoptimizer_rs::codec::allocate+0xae>
   b4a81:	cmp    $0x2,%r14
   b4a85:	je     b4a8f <meshoptimizer_rs::codec::allocate+0x6f>
   b4a87:	mov    $0x2,%al
   b4a89:	cmp    $0x4,%r14
   b4a8d:	jne    b4af4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4a8f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b4a99:	imul   %rcx,%rax
   b4a9d:	movabs $0x5555555555555556,%rdx
   b4aa7:	cmp    %rdx,%rax
   b4aaa:	jb     b4adc <meshoptimizer_rs::codec::allocate+0xbc>
   b4aac:	xor    %eax,%eax
   b4aae:	jmp    b4af4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4ab0:	lea    -0x1(%r14),%rax
   b4ab4:	cmp    $0x100,%rax
   b4aba:	setb   %r10b
   b4abe:	test   $0x3,%r14b
   b4ac2:	sete   %dl
   b4ac5:	mov    $0x2,%al
   b4ac7:	test   %dl,%r10b
   b4aca:	jne    b4adc <meshoptimizer_rs::codec::allocate+0xbc>
   b4acc:	jmp    b4af4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4ace:	cmp    $0x4,%r14
   b4ad2:	je     b4adc <meshoptimizer_rs::codec::allocate+0xbc>
   b4ad4:	mov    $0x2,%al
   b4ad6:	cmp    $0x2,%r14
   b4ada:	jne    b4af4 <meshoptimizer_rs::codec::allocate+0xd4>
   b4adc:	mov    %rcx,%rax
   b4adf:	mul    %r14
   b4ae2:	mov    %rax,%rbp
   b4ae5:	seto   %al
   b4ae8:	test   %rbp,%rbp
   b4aeb:	sets   %dl
   b4aee:	or     %al,%dl
   b4af0:	mov    $0x8,%al
   b4af2:	je     b4b10 <meshoptimizer_rs::codec::allocate+0xf0>
   b4af4:	mov    %al,0x8(%rdi)
   b4af7:	movq   $0xffffffffffffffff,(%rdi)
   b4afe:	add    $0x468,%rsp
   b4b05:	pop    %rbx
   b4b06:	pop    %r12
   b4b08:	pop    %r13
   b4b0a:	pop    %r14
   b4b0c:	pop    %r15
   b4b0e:	pop    %rbp
   b4b0f:	ret
   b4b10:	mov    %esi,0x3c(%rsp)
   b4b14:	mov    %rcx,0x10(%rsp)
   b4b19:	mov    %r9,0x30(%rsp)
   b4b1e:	mov    %r8d,0x38(%rsp)
   b4b23:	mov    %rdi,0x40(%rsp)
   b4b28:	mov    %r12,%rdi
   b4b2b:	mov    %rbp,%rsi
   b4b2e:	call   99550 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b4b33:	mov    %r15,%rbx
   b4b36:	cmp    $0xff,%al
   b4b38:	je     b4b56 <meshoptimizer_rs::codec::allocate+0x136>
   b4b3a:	sub    %rbx,%r15
   b4b3d:	mov    %r15,0x358(%r12)
   b4b45:	mov    0x40(%rsp),%rcx
   b4b4a:	mov    %al,0x8(%rcx)
   b4b4d:	movq   $0xffffffffffffffff,(%rcx)
   b4b54:	jmp    b4afe <meshoptimizer_rs::codec::allocate+0xde>
   b4b56:	mov    0x4a0(%rsp),%rdx
   b4b5e:	mov    %r15,%rax
   b4b61:	mov    %rdx,%rcx
   b4b64:	sub    %rdx,%rax
   b4b67:	jb     b4ed6 <meshoptimizer_rs::codec::allocate+0x4b6>
   b4b6d:	mov    %rax,0x48(%rsp)
   b4b72:	mov    %rax,%rbx
   b4b75:	mov    %rbp,%r10
   b4b78:	sub    %rbp,%rbx
   b4b7b:	jb     b4ede <meshoptimizer_rs::codec::allocate+0x4be>
   b4b81:	mov    %rbx,0x48(%rsp)
   b4b86:	cmpb   $0x0,0x38(%rsp)
   b4b8b:	mov    0x3c(%rsp),%ebp
   b4b8f:	je     b4ba5 <meshoptimizer_rs::codec::allocate+0x185>
   b4b91:	mov    %r10,%rsi
   b4b94:	shr    $0x2,%rsi
   b4b98:	mov    %rbx,%rax
   b4b9b:	sub    %rsi,%rax
   b4b9e:	jae    b4ba8 <meshoptimizer_rs::codec::allocate+0x188>
   b4ba0:	jmp    b4ee4 <meshoptimizer_rs::codec::allocate+0x4c4>
   b4ba5:	mov    %rbx,%rax
   b4ba8:	sub    %rax,%r15
   b4bab:	mov    %r15,0x358(%r12)
   b4bb3:	cmp    $0x2,%bpl
   b4bb7:	mov    0x40(%rsp),%rbx
   b4bbc:	je     b4bf1 <meshoptimizer_rs::codec::allocate+0x1d1>
   b4bbe:	cmp    $0x1,%r13d
   b4bc2:	mov    %rcx,%r15
   b4bc5:	jne    b4c9d <meshoptimizer_rs::codec::allocate+0x27d>
   b4bcb:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b4bd5:	mov    0x10(%rsp),%rax
   b4bda:	mul    %rcx
   b4bdd:	shr    $1,%rdx
   b4be0:	add    $0x11,%rdx
   b4be4:	mov    $0x7,%sil
   b4be7:	cmp    %rdx,%r15
   b4bea:	jae    b4c09 <meshoptimizer_rs::codec::allocate+0x1e9>
   b4bec:	jmp    b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4bf1:	mov    0x10(%rsp),%rax
   b4bf6:	lea    0x5(%rax),%rdx
   b4bfa:	mov    %rcx,%r15
   b4bfd:	mov    $0x7,%sil
   b4c00:	cmp    %rdx,%r15
   b4c03:	jb     b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4c09:	movq   $0x0,0x18(%rsp)
   b4c12:	movq   $0x1,0x20(%rsp)
   b4c1b:	movq   $0x0,0x28(%rsp)
   b4c24:	test   %r10,%r10
   b4c27:	jne    b4d6d <meshoptimizer_rs::codec::allocate+0x34d>
   b4c2d:	cmp    $0x2,%bpl
   b4c31:	je     b4db9 <meshoptimizer_rs::codec::allocate+0x399>
   b4c37:	test   %r13d,%r13d
   b4c3a:	jne    b4ce0 <meshoptimizer_rs::codec::allocate+0x2c0>
   b4c40:	cmp    $0x4,%r14
   b4c44:	sete   %al
   b4c47:	cmp    $0x100000,%r10
   b4c4e:	setae  %cl
   b4c51:	test   %cl,%al
   b4c53:	je     b4d15 <meshoptimizer_rs::codec::allocate+0x2f5>
   b4c59:	lea    0x60(%rsp),%rdi
   b4c5e:	mov    $0x400,%edx
   b4c63:	xor    %esi,%esi
   b4c65:	call   *0x75b7d(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   b4c6b:	lea    0x18(%rsp),%rax
   b4c70:	mov    %rax,0x58(%rsp)
   b4c75:	movq   $0x0,0x460(%rsp)
   b4c81:	lea    0x58(%rsp),%rdi
   b4c86:	mov    0x10(%rsp),%rsi
   b4c8b:	mov    0x30(%rsp),%rdx
   b4c90:	mov    %r15,%rcx
   b4c93:	call   73e40 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b4c98:	jmp    b4dd3 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4c9d:	mov    $0x7,%sil
   b4ca0:	test   %r15,%r15
   b4ca3:	je     b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4ca9:	mov    0x30(%rsp),%rax
   b4cae:	movzbl (%rax),%ecx
   b4cb1:	mov    %ecx,%eax
   b4cb3:	and    $0xf0,%al
   b4cb5:	cmp    $0xa0,%al
   b4cb7:	jne    b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4cbd:	and    $0xf,%cl
   b4cc0:	mov    $0x5,%sil
   b4cc3:	cmp    $0x1,%cl
   b4cc6:	ja     b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4ccc:	mov    0x10(%rsp),%rdi
   b4cd1:	cmp    $0x21,%rdi
   b4cd5:	jae    b4d34 <meshoptimizer_rs::codec::allocate+0x314>
   b4cd7:	xor    %eax,%eax
   b4cd9:	xor    %esi,%esi
   b4cdb:	jmp    b4e43 <meshoptimizer_rs::codec::allocate+0x423>
   b4ce0:	lea    0x18(%rsp),%rdi
   b4ce5:	mov    %r10,%rsi
   b4ce8:	call   92cd0 <<alloc::vec::Vec<u8>>::resize>
   b4ced:	mov    0x20(%rsp),%rsi
   b4cf2:	mov    0x28(%rsp),%rdx
   b4cf7:	mov    %r15,(%rsp)
   b4cfb:	mov    %r13d,%edi
   b4cfe:	mov    0x10(%rsp),%rcx
   b4d03:	mov    %r14,%r8
   b4d06:	mov    0x30(%rsp),%r9
   b4d0b:	call   af6f0 <meshoptimizer_rs::codec::raw>
   b4d10:	jmp    b4dd3 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4d15:	lea    0x18(%rsp),%rdi
   b4d1a:	mov    0x10(%rsp),%rsi
   b4d1f:	mov    %r14,%rdx
   b4d22:	mov    0x30(%rsp),%rcx
   b4d27:	mov    %r15,%r8
   b4d2a:	call   73170 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   b4d2f:	jmp    b4dd3 <meshoptimizer_rs::codec::allocate+0x3b3>
   b4d34:	mov    $0x2000,%ax
   b4d38:	xor    %edx,%edx
   b4d3a:	div    %r14w
   b4d3e:	and    $0x3ff0,%eax
   b4d43:	cmp    $0x100,%eax
   b4d48:	mov    $0x100,%esi
   b4d4d:	cmovb  %eax,%esi
   b4d50:	mov    %rdi,%rax
   b4d53:	shr    $0x20,%rax
   b4d57:	je     b4e2e <meshoptimizer_rs::codec::allocate+0x40e>
   b4d5d:	mov    %rdi,%rax
   b4d60:	xor    %edx,%edx
   b4d62:	div    %rsi
   b4d65:	mov    %rdx,%rdi
   b4d68:	jmp    b4e36 <meshoptimizer_rs::codec::allocate+0x416>
   b4d6d:	lea    0x58(%rsp),%rdi
   b4d72:	mov    $0x1,%edx
   b4d77:	mov    $0x1,%r8d
   b4d7d:	mov    $0x1,%r9d
   b4d83:	xor    %esi,%esi
   b4d85:	mov    %r10,%rcx
   b4d88:	mov    %r10,%r12
   b4d8b:	call   937c0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b4d90:	cmpb   $0x0,0x58(%rsp)
   b4d95:	je     b4d9d <meshoptimizer_rs::codec::allocate+0x37d>
   b4d97:	movb   $0xa,0x8(%rbx)
   b4d9b:	jmp    b4df5 <meshoptimizer_rs::codec::allocate+0x3d5>
   b4d9d:	mov    0x60(%rsp),%rax
   b4da2:	mov    %rax,0x20(%rsp)
   b4da7:	mov    %r12,0x18(%rsp)
   b4dac:	mov    %r12,%r10
   b4daf:	cmp    $0x2,%bpl
   b4db3:	jne    b4c37 <meshoptimizer_rs::codec::allocate+0x217>
   b4db9:	lea    0x18(%rsp),%rdi
   b4dbe:	mov    0x10(%rsp),%rsi
   b4dc3:	mov    %r14,%rdx
   b4dc6:	mov    0x30(%rsp),%rcx
   b4dcb:	mov    %r15,%r8
   b4dce:	call   d3b20 <meshoptimizer_rs::codec::index::sequence_append>
   b4dd3:	cmp    $0xff,%al
   b4dd5:	jne    b4df2 <meshoptimizer_rs::codec::allocate+0x3d2>
   b4dd7:	mov    0x20(%rsp),%rsi
   b4ddc:	mov    0x28(%rsp),%rdx
   b4de1:	movzbl 0x38(%rsp),%edi
   b4de6:	mov    %r14,%rcx
   b4de9:	call   b13d0 <meshoptimizer_rs::codec::apply>
   b4dee:	cmp    $0xff,%al
   b4df0:	je     b4e18 <meshoptimizer_rs::codec::allocate+0x3f8>
   b4df2:	mov    %al,0x8(%rbx)
   b4df5:	movq   $0xffffffffffffffff,(%rbx)
   b4dfc:	cmpq   $0x0,0x18(%rsp)
   b4e02:	je     b4afe <meshoptimizer_rs::codec::allocate+0xde>
   b4e08:	mov    0x20(%rsp),%rdi
   b4e0d:	call   *0x759b5(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   b4e13:	jmp    b4afe <meshoptimizer_rs::codec::allocate+0xde>
   b4e18:	mov    0x28(%rsp),%rax
   b4e1d:	mov    %rax,0x10(%rbx)
   b4e21:	movups 0x18(%rsp),%xmm0
   b4e26:	movups %xmm0,(%rbx)
   b4e29:	jmp    b4afe <meshoptimizer_rs::codec::allocate+0xde>
   b4e2e:	mov    %edi,%eax
   b4e30:	xor    %edx,%edx
   b4e32:	div    %esi
   b4e34:	mov    %edx,%edi
   b4e36:	shr    $0x4,%esi
   b4e39:	add    $0x3,%esi
   b4e3c:	shr    $0x2,%esi
   b4e3f:	imul   %rax,%rsi
   b4e43:	test   %cl,%cl
   b4e45:	je     b4e71 <meshoptimizer_rs::codec::allocate+0x451>
   b4e47:	cmp    $0x1,%rdi
   b4e4b:	sbb    $0xffffffffffffffff,%rax
   b4e4f:	mov    %r14,%rcx
   b4e52:	shr    $0x2,%rcx
   b4e56:	mul    %rcx
   b4e59:	jo     b4ef4 <meshoptimizer_rs::codec::allocate+0x4d4>
   b4e5f:	add    %r14,%rcx
   b4e62:	cmp    $0x19,%rcx
   b4e66:	mov    $0x18,%edx
   b4e6b:	cmovae %rcx,%rdx
   b4e6f:	jmp    b4ea9 <meshoptimizer_rs::codec::allocate+0x489>
   b4e71:	test   %rdi,%rdi
   b4e74:	je     b4e92 <meshoptimizer_rs::codec::allocate+0x472>
   b4e76:	mov    %rdi,%rax
   b4e79:	shr    $0x4,%rax
   b4e7d:	and    $0xf,%edi
   b4e80:	cmp    $0x1,%rdi
   b4e84:	sbb    $0xffffffffffffffff,%rax
   b4e88:	add    $0x3,%rax
   b4e8c:	shr    $0x2,%rax
   b4e90:	jmp    b4e94 <meshoptimizer_rs::codec::allocate+0x474>
   b4e92:	xor    %eax,%eax
   b4e94:	add    %rsi,%rax
   b4e97:	mul    %r14
   b4e9a:	jo     b4ef4 <meshoptimizer_rs::codec::allocate+0x4d4>
   b4e9c:	cmp    $0x21,%r14
   b4ea0:	mov    $0x20,%edx
   b4ea5:	cmovae %r14,%rdx
   b4ea9:	mov    $0x8,%sil
   b4eac:	add    %rax,%rdx
   b4eaf:	jb     b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4eb1:	cmp    $0xffffffffffffffff,%rdx
   b4eb5:	je     b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4eb7:	inc    %rdx
   b4eba:	mov    $0x7,%sil
   b4ebd:	cmp    %rdx,%r15
   b4ec0:	jae    b4c09 <meshoptimizer_rs::codec::allocate+0x1e9>
   b4ec6:	mov    %sil,0x8(%rbx)
   b4eca:	movq   $0xffffffffffffffff,(%rbx)
   b4ed1:	jmp    b4afe <meshoptimizer_rs::codec::allocate+0xde>
   b4ed6:	mov    %rcx,%rsi
   b4ed9:	mov    %r15,%rbx
   b4edc:	jmp    b4ee4 <meshoptimizer_rs::codec::allocate+0x4c4>
   b4ede:	mov    %r10,%rsi
   b4ee1:	mov    %rax,%rbx
   b4ee4:	lea    0x48(%rsp),%rdi
   b4ee9:	call   *0x75d99(%rip)        # 12ac88 <_DYNAMIC+0x6c8>
   b4eef:	jmp    b4b3a <meshoptimizer_rs::codec::allocate+0x11a>
   b4ef4:	mov    $0x8,%sil
   b4ef7:	jmp    b4ec6 <meshoptimizer_rs::codec::allocate+0x4a6>
   b4ef9:	mov    %rax,%rbx
   b4efc:	cmpq   $0x0,0x18(%rsp)
   b4f02:	je     b4f0f <meshoptimizer_rs::codec::allocate+0x4ef>
   b4f04:	mov    0x20(%rsp),%rdi
   b4f09:	call   *0x758b9(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   b4f0f:	mov    %rbx,%rdi
   b4f12:	call   121810 <_Unwind_Resume@plt>
