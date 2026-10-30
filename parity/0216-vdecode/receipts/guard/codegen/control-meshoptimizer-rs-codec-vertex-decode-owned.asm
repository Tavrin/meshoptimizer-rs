
<artifacts>/guard/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000e78e0 <meshoptimizer_rs::codec::vertex::decode_owned>:
   e78e0:	push   %rbp
   e78e1:	push   %r15
   e78e3:	push   %r14
   e78e5:	push   %r13
   e78e7:	push   %r12
   e78e9:	push   %rbx
   e78ea:	sub    $0x5f8,%rsp
   e78f1:	mov    %r8,%r15
   e78f4:	mov    %rcx,%r12
   e78f7:	mov    %rdx,%r14
   e78fa:	mov    %rsi,%r13
   e78fd:	mov    %rdi,%rbp
   e7900:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   e7905:	cmp    $0x2,%al
   e7907:	jb     e796d <meshoptimizer_rs::codec::vertex::decode_owned+0x8d>
   e7909:	cmp    $0x20,%r14
   e790d:	lea    -0x65064(%rip),%rax        # 828b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   e7914:	lea    -0x665db(%rip),%rcx        # 81340 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   e791b:	cmove  %rax,%rcx
   e791f:	lea    -0x5f7f6(%rip),%rax        # 88130 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   e7926:	lea    -0x6116d(%rip),%rdx        # 867c0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   e792d:	cmove  %rax,%rdx
   e7931:	cmp    $0x4,%r14
   e7935:	lea    -0x63c5c(%rip),%rax        # 83ce0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   e793c:	cmovne %rcx,%rax
   e7940:	lea    -0x5e037(%rip),%r9        # 89910 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   e7947:	cmovne %rdx,%r9
   e794b:	cmp    $0x21,%r13
   e794f:	cmovae %rax,%r9
   e7953:	mov    %rbp,%rdi
   e7956:	mov    %r13,%rsi
   e7959:	mov    %r14,%rdx
   e795c:	mov    %r12,%rcx
   e795f:	mov    %r15,%r8
   e7962:	call   *%r9
   e7965:	cmp    $0xfe,%al
   e7967:	jne    e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e796d:	mov    $0x7,%al
   e796f:	test   %r15,%r15
   e7972:	je     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7978:	movzbl (%r12),%edx
   e797d:	mov    %edx,%ecx
   e797f:	and    $0xf0,%cl
   e7982:	cmp    $0xa0,%cl
   e7985:	jne    e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e798b:	and    $0xf,%dl
   e798e:	mov    $0x5,%al
   e7990:	mov    %dl,0x27(%rsp)
   e7994:	cmp    $0x1,%dl
   e7997:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e799d:	cmpb   $0x0,0x27(%rsp)
   e79a2:	je     e79cb <meshoptimizer_rs::codec::vertex::decode_owned+0xeb>
   e79a4:	mov    %r14,%rcx
   e79a7:	shr    $0x2,%rcx
   e79ab:	add    %r14,%rcx
   e79ae:	cmp    $0x19,%rcx
   e79b2:	mov    $0x18,%ebx
   e79b7:	cmovae %rcx,%rbx
   e79bb:	lea    0x1(%rbx),%rdx
   e79bf:	mov    $0x7,%al
   e79c1:	cmp    %rdx,%r15
   e79c4:	jae    e79ea <meshoptimizer_rs::codec::vertex::decode_owned+0x10a>
   e79c6:	jmp    e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e79cb:	cmp    $0x21,%r14
   e79cf:	mov    $0x20,%ebx
   e79d4:	cmovae %r14,%rbx
   e79d8:	mov    %r14,%rcx
   e79db:	lea    0x1(%rbx),%rdx
   e79df:	mov    $0x7,%al
   e79e1:	cmp    %rdx,%r15
   e79e4:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e79ea:	mov    %r15,%rsi
   e79ed:	sub    %rcx,%rsi
   e79f0:	lea    (%rsi,%r14,1),%rax
   e79f4:	mov    %r15,%rcx
   e79f7:	mov    %rax,0x70(%rsp)
   e79fc:	sub    %rax,%rcx
   e79ff:	mov    %rcx,0xa8(%rsp)
   e7a07:	jb     e84e4 <meshoptimizer_rs::codec::vertex::decode_owned+0xc04>
   e7a0d:	xorps  %xmm0,%xmm0
   e7a10:	movaps %xmm0,0x1e0(%rsp)
   e7a18:	movaps %xmm0,0x1d0(%rsp)
   e7a20:	movaps %xmm0,0x1c0(%rsp)
   e7a28:	movaps %xmm0,0x1b0(%rsp)
   e7a30:	movaps %xmm0,0x1a0(%rsp)
   e7a38:	movaps %xmm0,0x190(%rsp)
   e7a40:	movaps %xmm0,0x180(%rsp)
   e7a48:	movaps %xmm0,0x170(%rsp)
   e7a50:	movaps %xmm0,0x160(%rsp)
   e7a58:	movaps %xmm0,0x150(%rsp)
   e7a60:	movaps %xmm0,0x140(%rsp)
   e7a68:	movaps %xmm0,0x130(%rsp)
   e7a70:	movaps %xmm0,0x120(%rsp)
   e7a78:	movaps %xmm0,0x110(%rsp)
   e7a80:	movaps %xmm0,0x100(%rsp)
   e7a88:	movaps %xmm0,0xf0(%rsp)
   e7a90:	cmp    $0x101,%r14
   e7a97:	jae    e84a0 <meshoptimizer_rs::codec::vertex::decode_owned+0xbc0>
   e7a9d:	lea    (%rsi,%r14,1),%rax
   e7aa1:	cmp    %rsi,%rax
   e7aa4:	jb     e84b7 <meshoptimizer_rs::codec::vertex::decode_owned+0xbd7>
   e7aaa:	add    %r12,%rsi
   e7aad:	lea    0xf0(%rsp),%rdi
   e7ab5:	mov    %r14,%rdx
   e7ab8:	call   *0x6279a(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e7abe:	test   %r14,%r14
   e7ac1:	je     e853c <meshoptimizer_rs::codec::vertex::decode_owned+0xc5c>
   e7ac7:	mov    %rbx,0xd8(%rsp)
   e7acf:	mov    $0x2000,%ax
   e7ad3:	xor    %edx,%edx
   e7ad5:	div    %r14w
   e7ad9:	and    $0x3ff0,%eax
   e7ade:	cmp    $0x100,%eax
   e7ae3:	mov    $0x100,%ecx
   e7ae8:	cmovb  %eax,%ecx
   e7aeb:	mov    %rcx,0xe0(%rsp)
   e7af3:	lea    0x1f8(%rsp),%rdi
   e7afb:	mov    $0x400,%edx
   e7b00:	xor    %esi,%esi
   e7b02:	call   *0x62760(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7b08:	test   %r13,%r13
   e7b0b:	je     e843d <meshoptimizer_rs::codec::vertex::decode_owned+0xb5d>
   e7b11:	add    %r12,0x70(%rsp)
   e7b16:	mov    %r14,%rax
   e7b19:	shr    $0x2,%rax
   e7b1d:	mov    %rax,0xa0(%rsp)
   e7b25:	lea    0x3(%r14),%rax
   e7b29:	shr    $0x2,%rax
   e7b2d:	mov    %rax,0xe8(%rsp)
   e7b35:	mov    $0x1,%ebx
   e7b3a:	xor    %eax,%eax
   e7b3c:	mov    %rbp,0x68(%rsp)
   e7b41:	mov    %r15,0x30(%rsp)
   e7b46:	mov    %r12,0x38(%rsp)
   e7b4b:	mov    %r13,0xc0(%rsp)
   e7b53:	mov    %r13,%rdx
   e7b56:	mov    %rax,0x58(%rsp)
   e7b5b:	sub    %rax,%rdx
   e7b5e:	mov    0xe0(%rsp),%rax
   e7b66:	cmp    %rax,%rdx
   e7b69:	cmovae %rax,%rdx
   e7b6d:	mov    %rdx,0x28(%rsp)
   e7b72:	cmpb   $0x0,0x27(%rsp)
   e7b77:	je     e7bbc <meshoptimizer_rs::codec::vertex::decode_owned+0x2dc>
   e7b79:	mov    %rbx,%rcx
   e7b7c:	mov    $0x7,%al
   e7b7e:	add    0xa0(%rsp),%rcx
   e7b86:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7b8c:	cmp    %r15,%rcx
   e7b8f:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7b95:	add    0x38(%rsp),%rbx
   e7b9a:	mov    0xa0(%rsp),%rax
   e7ba2:	mov    %rax,0x98(%rsp)
   e7baa:	mov    %rbx,0xb0(%rsp)
   e7bb2:	mov    %rcx,%rbx
   e7bb5:	mov    0x68(%rsp),%rbp
   e7bba:	jmp    e7bd5 <meshoptimizer_rs::codec::vertex::decode_owned+0x2f5>
   e7bbc:	movq   $0x0,0x98(%rsp)
   e7bc8:	mov    $0x1,%eax
   e7bcd:	mov    %rax,0xb0(%rsp)
   e7bd5:	mov    0x58(%rsp),%rax
   e7bda:	mov    0x28(%rsp),%rcx
   e7bdf:	lea    (%rcx,%rax,1),%r15
   e7be3:	mov    %r15,0xd0(%rsp)
   e7beb:	imul   %r14,%r15
   e7bef:	mov    0x10(%rbp),%r13
   e7bf3:	mov    %rbp,%rax
   e7bf6:	mov    %r15,%rbp
   e7bf9:	mov    %r15,%rdx
   e7bfc:	sub    %r13,%rbp
   e7bff:	jbe    e7c49 <meshoptimizer_rs::codec::vertex::decode_owned+0x369>
   e7c01:	mov    (%rax),%rax
   e7c04:	sub    %r13,%rax
   e7c07:	cmp    %rax,%rbp
   e7c0a:	ja     e8415 <meshoptimizer_rs::codec::vertex::decode_owned+0xb35>
   e7c10:	mov    0x68(%rsp),%rax
   e7c15:	mov    0x8(%rax),%r12
   e7c19:	lea    (%r12,%r13,1),%rdi
   e7c1d:	cmp    $0x2,%rbp
   e7c21:	jb     e7c3b <meshoptimizer_rs::codec::vertex::decode_owned+0x35b>
   e7c23:	lea    -0x1(%rbp),%rdx
   e7c27:	xor    %esi,%esi
   e7c29:	call   *0x62639(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7c2f:	add    %rbp,%r13
   e7c32:	dec    %r13
   e7c35:	add    %r13,%r12
   e7c38:	mov    %r12,%rdi
   e7c3b:	movb   $0x0,(%rdi)
   e7c3e:	inc    %r13
   e7c41:	mov    %r13,%rdx
   e7c44:	mov    0x68(%rsp),%rax
   e7c49:	mov    0x58(%rsp),%rcx
   e7c4e:	imul   %r14,%rcx
   e7c52:	mov    %rdx,0x10(%rax)
   e7c56:	mov    %rcx,0x58(%rsp)
   e7c5b:	cmp    %rcx,%r15
   e7c5e:	jb     e84cf <meshoptimizer_rs::codec::vertex::decode_owned+0xbef>
   e7c64:	cmp    %rdx,%r15
   e7c67:	ja     e84cf <meshoptimizer_rs::codec::vertex::decode_owned+0xbef>
   e7c6d:	mov    0x28(%rsp),%rdx
   e7c72:	lea    0xf(%rdx),%ecx
   e7c75:	and    $0x3f0,%ecx
   e7c7b:	mov    %rcx,0x50(%rsp)
   e7c80:	mov    %rdx,%rcx
   e7c83:	imul   %r14,%rcx
   e7c87:	mov    %rcx,0xb8(%rsp)
   e7c8f:	mov    0x58(%rsp),%rcx
   e7c94:	add    0x8(%rax),%rcx
   e7c98:	mov    %rcx,0x58(%rsp)
   e7c9d:	lea    -0x1(%rdx),%rax
   e7ca1:	imul   %r14,%rax
   e7ca5:	mov    %rax,0xc8(%rsp)
   e7cad:	add    $0x4,%rax
   e7cb1:	mov    %rax,0x60(%rsp)
   e7cb6:	lea    0x0(,%rdx,4),%rax
   e7cbe:	mov    %rax,0x78(%rsp)
   e7cc3:	lea    0x1f8(%rsp,%rdx,1),%rax
   e7ccb:	mov    %rax,0x90(%rsp)
   e7cd3:	lea    0x1f8(%rsp,%rdx,2),%rax
   e7cdb:	mov    %rax,0x88(%rsp)
   e7ce3:	lea    (%rdx,%rdx,2),%rax
   e7ce7:	add    %rsp,%rax
   e7cea:	add    $0x1f8,%rax
   e7cf0:	mov    %rax,0x80(%rsp)
   e7cf8:	xor    %r13d,%r13d
   e7cfb:	xor    %ebp,%ebp
   e7cfd:	movzbl 0x27(%rsp),%r15d
   e7d03:	jmp    e7d5f <meshoptimizer_rs::codec::vertex::decode_owned+0x47f>
   e7d05:	lea    (%rsp,%rbp,1),%rcx
   e7d09:	add    $0xf0,%rcx
   e7d10:	shr    $0x4,%al
   e7d13:	neg    %al
   e7d15:	and    $0x1f,%al
   e7d17:	movzbl %al,%eax
   e7d1a:	mov    %rcx,(%rsp)
   e7d1e:	mov    %eax,0x8(%rsp)
   e7d22:	movq   $0x0,0x10(%rsp)
   e7d2b:	lea    0x1f8(%rsp),%rdi
   e7d33:	mov    0x78(%rsp),%rsi
   e7d38:	mov    0x60(%rsp),%rcx
   e7d3d:	mov    0x28(%rsp),%r8
   e7d42:	mov    %r14,%r9
   e7d45:	call   7d3e0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   e7d4a:	inc    %r13
   e7d4d:	add    $0x4,%rbp
   e7d51:	cmp    %r13,0xe8(%rsp)
   e7d59:	je     e83bb <meshoptimizer_rs::codec::vertex::decode_owned+0xadb>
   e7d5f:	test   %r15b,%r15b
   e7d62:	je     e7d81 <meshoptimizer_rs::codec::vertex::decode_owned+0x4a1>
   e7d64:	cmp    0x98(%rsp),%r13
   e7d6c:	jae    e850c <meshoptimizer_rs::codec::vertex::decode_owned+0xc2c>
   e7d72:	mov    0xb0(%rsp),%rax
   e7d7a:	movzbl (%rax,%r13,1),%r12d
   e7d7f:	jmp    e7d84 <meshoptimizer_rs::codec::vertex::decode_owned+0x4a4>
   e7d81:	xor    %r12d,%r12d
   e7d84:	mov    %r12d,%eax
   e7d87:	and    $0x3,%al
   e7d89:	movzbl %al,%ecx
   e7d8c:	mov    0x50(%rsp),%rdx
   e7d91:	cmp    %rdx,0x28(%rsp)
   e7d96:	ja     e8208 <meshoptimizer_rs::codec::vertex::decode_owned+0x928>
   e7d9c:	cmp    $0x2,%al
   e7d9e:	je     e7ddd <meshoptimizer_rs::codec::vertex::decode_owned+0x4fd>
   e7da0:	cmp    $0x3,%ecx
   e7da3:	jne    e7df7 <meshoptimizer_rs::codec::vertex::decode_owned+0x517>
   e7da5:	mov    %rbx,%r15
   e7da8:	mov    $0x7,%al
   e7daa:	add    0x28(%rsp),%r15
   e7daf:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7db5:	cmp    0x30(%rsp),%r15
   e7dba:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7dc0:	add    0x38(%rsp),%rbx
   e7dc5:	lea    0x1f8(%rsp),%rdi
   e7dcd:	mov    %rbx,%rsi
   e7dd0:	mov    0x28(%rsp),%rdx
   e7dd5:	call   *0x6247d(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e7ddb:	jmp    e7e51 <meshoptimizer_rs::codec::vertex::decode_owned+0x571>
   e7ddd:	lea    0x1f8(%rsp),%rdi
   e7de5:	xor    %esi,%esi
   e7de7:	mov    0x28(%rsp),%rdx
   e7dec:	call   *0x62476(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7df2:	mov    %rbx,%r15
   e7df5:	jmp    e7e51 <meshoptimizer_rs::codec::vertex::decode_owned+0x571>
   e7df7:	test   %al,%al
   e7df9:	lea    -0xd9340(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e7e00:	lea    -0xd9217(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e7e07:	cmove  %rcx,%rax
   e7e0b:	test   %r15b,%r15b
   e7e0e:	lea    -0xd9675(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e7e15:	cmove  %rcx,%rax
   e7e19:	mov    %rax,(%rsp)
   e7e1d:	lea    0x40(%rsp),%rdi
   e7e22:	mov    0x38(%rsp),%rsi
   e7e27:	mov    0x30(%rsp),%rdx
   e7e2c:	mov    %rbx,%rcx
   e7e2f:	lea    0x1f8(%rsp),%r8
   e7e37:	mov    0x50(%rsp),%r9
   e7e3c:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e7e41:	cmpb   $0x0,0x40(%rsp)
   e7e46:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e7e4c:	mov    0x48(%rsp),%r15
   e7e51:	mov    %r12d,%eax
   e7e54:	shr    $0x2,%al
   e7e57:	and    $0x3,%al
   e7e59:	cmp    $0x3,%al
   e7e5b:	je     e7e7f <meshoptimizer_rs::codec::vertex::decode_owned+0x59f>
   e7e5d:	movzbl %al,%ecx
   e7e60:	cmp    $0x2,%ecx
   e7e63:	jne    e7eba <meshoptimizer_rs::codec::vertex::decode_owned+0x5da>
   e7e65:	mov    0x90(%rsp),%rdi
   e7e6d:	xor    %esi,%esi
   e7e6f:	mov    0x28(%rsp),%rdx
   e7e74:	call   *0x623ee(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7e7a:	jmp    e7f16 <meshoptimizer_rs::codec::vertex::decode_owned+0x636>
   e7e7f:	mov    %r15,%rbx
   e7e82:	mov    $0x7,%al
   e7e84:	add    0x28(%rsp),%rbx
   e7e89:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7e8f:	cmp    0x30(%rsp),%rbx
   e7e94:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7e9a:	add    0x38(%rsp),%r15
   e7e9f:	mov    0x90(%rsp),%rdi
   e7ea7:	mov    %r15,%rsi
   e7eaa:	mov    0x28(%rsp),%rdx
   e7eaf:	call   *0x623a3(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e7eb5:	mov    %rbx,%r15
   e7eb8:	jmp    e7f16 <meshoptimizer_rs::codec::vertex::decode_owned+0x636>
   e7eba:	test   %al,%al
   e7ebc:	lea    -0xd9403(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e7ec3:	lea    -0xd92da(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e7eca:	cmove  %rcx,%rax
   e7ece:	cmpb   $0x0,0x27(%rsp)
   e7ed3:	lea    -0xd973a(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e7eda:	cmove  %rcx,%rax
   e7ede:	mov    %rax,(%rsp)
   e7ee2:	lea    0x40(%rsp),%rdi
   e7ee7:	mov    0x38(%rsp),%rsi
   e7eec:	mov    0x30(%rsp),%rdx
   e7ef1:	mov    %r15,%rcx
   e7ef4:	mov    0x90(%rsp),%r8
   e7efc:	mov    0x50(%rsp),%r9
   e7f01:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e7f06:	cmpb   $0x0,0x40(%rsp)
   e7f0b:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e7f11:	mov    0x48(%rsp),%r15
   e7f16:	mov    %r12d,%eax
   e7f19:	shr    $0x4,%al
   e7f1c:	and    $0x3,%al
   e7f1e:	cmp    $0x3,%al
   e7f20:	je     e7f52 <meshoptimizer_rs::codec::vertex::decode_owned+0x672>
   e7f22:	movzbl %al,%ecx
   e7f25:	cmp    $0x2,%ecx
   e7f28:	jne    e7fda <meshoptimizer_rs::codec::vertex::decode_owned+0x6fa>
   e7f2e:	mov    0x88(%rsp),%rdi
   e7f36:	xor    %esi,%esi
   e7f38:	mov    0x28(%rsp),%rdx
   e7f3d:	call   *0x62325(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7f43:	shr    $0x6,%r12b
   e7f47:	cmp    $0x3,%r12b
   e7f4b:	jne    e7f99 <meshoptimizer_rs::codec::vertex::decode_owned+0x6b9>
   e7f4d:	jmp    e8044 <meshoptimizer_rs::codec::vertex::decode_owned+0x764>
   e7f52:	mov    %r15,%rbx
   e7f55:	mov    $0x7,%al
   e7f57:	add    0x28(%rsp),%rbx
   e7f5c:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7f62:	cmp    0x30(%rsp),%rbx
   e7f67:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e7f6d:	add    0x38(%rsp),%r15
   e7f72:	mov    0x88(%rsp),%rdi
   e7f7a:	mov    %r15,%rsi
   e7f7d:	mov    0x28(%rsp),%rdx
   e7f82:	call   *0x622d0(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e7f88:	mov    %rbx,%r15
   e7f8b:	shr    $0x6,%r12b
   e7f8f:	cmp    $0x3,%r12b
   e7f93:	je     e8044 <meshoptimizer_rs::codec::vertex::decode_owned+0x764>
   e7f99:	movzbl %r12b,%eax
   e7f9d:	cmp    $0x2,%eax
   e7fa0:	jne    e8094 <meshoptimizer_rs::codec::vertex::decode_owned+0x7b4>
   e7fa6:	mov    0x80(%rsp),%rdi
   e7fae:	xor    %esi,%esi
   e7fb0:	mov    0x28(%rsp),%rdx
   e7fb5:	call   *0x622ad(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   e7fbb:	mov    %r15,%rbx
   e7fbe:	movzbl 0x27(%rsp),%r15d
   e7fc4:	test   %r15b,%r15b
   e7fc7:	mov    0xb8(%rsp),%rcx
   e7fcf:	jne    e8104 <meshoptimizer_rs::codec::vertex::decode_owned+0x824>
   e7fd5:	jmp    e808d <meshoptimizer_rs::codec::vertex::decode_owned+0x7ad>
   e7fda:	test   %al,%al
   e7fdc:	lea    -0xd9523(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e7fe3:	lea    -0xd93fa(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e7fea:	cmove  %rcx,%rax
   e7fee:	cmpb   $0x0,0x27(%rsp)
   e7ff3:	lea    -0xd985a(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e7ffa:	cmove  %rcx,%rax
   e7ffe:	mov    %rax,(%rsp)
   e8002:	lea    0x40(%rsp),%rdi
   e8007:	mov    0x38(%rsp),%rsi
   e800c:	mov    0x30(%rsp),%rdx
   e8011:	mov    %r15,%rcx
   e8014:	mov    0x88(%rsp),%r8
   e801c:	mov    0x50(%rsp),%r9
   e8021:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e8026:	cmpb   $0x0,0x40(%rsp)
   e802b:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e8031:	mov    0x48(%rsp),%r15
   e8036:	shr    $0x6,%r12b
   e803a:	cmp    $0x3,%r12b
   e803e:	jne    e7f99 <meshoptimizer_rs::codec::vertex::decode_owned+0x6b9>
   e8044:	mov    %r15,%rbx
   e8047:	mov    $0x7,%al
   e8049:	add    0x28(%rsp),%rbx
   e804e:	jb     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e8054:	cmp    0x30(%rsp),%rbx
   e8059:	ja     e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e805f:	add    0x38(%rsp),%r15
   e8064:	mov    0x80(%rsp),%rdi
   e806c:	mov    %r15,%rsi
   e806f:	mov    0x28(%rsp),%rdx
   e8074:	call   *0x621de(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e807a:	movzbl 0x27(%rsp),%r15d
   e8080:	test   %r15b,%r15b
   e8083:	mov    0xb8(%rsp),%rcx
   e808b:	jne    e8104 <meshoptimizer_rs::codec::vertex::decode_owned+0x824>
   e808d:	xor    %eax,%eax
   e808f:	jmp    e811c <meshoptimizer_rs::codec::vertex::decode_owned+0x83c>
   e8094:	test   %r12b,%r12b
   e8097:	lea    -0xd95de(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e809e:	lea    -0xd94b5(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e80a5:	cmove  %rcx,%rax
   e80a9:	cmpb   $0x0,0x27(%rsp)
   e80ae:	lea    -0xd9915(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e80b5:	cmove  %rcx,%rax
   e80b9:	mov    %rax,(%rsp)
   e80bd:	lea    0x40(%rsp),%rdi
   e80c2:	mov    0x38(%rsp),%rsi
   e80c7:	mov    0x30(%rsp),%rdx
   e80cc:	mov    %r15,%rcx
   e80cf:	mov    0x80(%rsp),%r8
   e80d7:	mov    0x50(%rsp),%r9
   e80dc:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e80e1:	cmpb   $0x0,0x40(%rsp)
   e80e6:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e80ec:	mov    0x48(%rsp),%rbx
   e80f1:	movzbl 0x27(%rsp),%r15d
   e80f7:	test   %r15b,%r15b
   e80fa:	mov    0xb8(%rsp),%rcx
   e8102:	je     e808d <meshoptimizer_rs::codec::vertex::decode_owned+0x7ad>
   e8104:	cmp    0xa8(%rsp),%r13
   e810c:	jae    e8524 <meshoptimizer_rs::codec::vertex::decode_owned+0xc44>
   e8112:	mov    0x70(%rsp),%rax
   e8117:	movzbl (%rax,%r13,1),%eax
   e811c:	mov    %rcx,%rdx
   e811f:	sub    %rbp,%rdx
   e8122:	jb     e848a <meshoptimizer_rs::codec::vertex::decode_owned+0xbaa>
   e8128:	cmp    %rdx,0x60(%rsp)
   e812d:	ja     e8476 <meshoptimizer_rs::codec::vertex::decode_owned+0xb96>
   e8133:	mov    0x58(%rsp),%rcx
   e8138:	lea    (%rcx,%rbp,1),%rdx
   e813c:	mov    %eax,%ecx
   e813e:	and    $0x3,%cl
   e8141:	movzbl %cl,%ecx
   e8144:	lea    -0xd6797(%rip),%rsi        # 119b4 <libm::math::generic::sqrt::RSQRT_TAB+0x70a>
   e814b:	movslq (%rsi,%rcx,4),%rcx
   e814f:	add    %rsi,%rcx
   e8152:	jmp    *%rcx
   e8154:	mov    %rdx,%r15
   e8157:	lea    (%rsp,%rbp,1),%r12
   e815b:	add    $0xf0,%r12
   e8162:	call   103ef0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   e8167:	test   %al,%al
   e8169:	je     e81ce <meshoptimizer_rs::codec::vertex::decode_owned+0x8ee>
   e816b:	mov    %r12,(%rsp)
   e816f:	lea    0x1f8(%rsp),%rdi
   e8177:	mov    0x78(%rsp),%rsi
   e817c:	mov    %r15,%rdx
   e817f:	mov    0x60(%rsp),%rcx
   e8184:	mov    0x28(%rsp),%r8
   e8189:	mov    %r14,%r9
   e818c:	call   7f930 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   e8191:	jmp    e81fd <meshoptimizer_rs::codec::vertex::decode_owned+0x91d>
   e8193:	lea    (%rsp,%rbp,1),%rax
   e8197:	add    $0xf0,%rax
   e819d:	mov    %rax,(%rsp)
   e81a1:	movq   $0x0,0x8(%rsp)
   e81aa:	lea    0x1f8(%rsp),%rdi
   e81b2:	mov    0x78(%rsp),%rsi
   e81b7:	mov    0x60(%rsp),%rcx
   e81bc:	mov    0x28(%rsp),%r8
   e81c1:	mov    %r14,%r9
   e81c4:	call   7d220 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   e81c9:	jmp    e7d4a <meshoptimizer_rs::codec::vertex::decode_owned+0x46a>
   e81ce:	mov    %r12,(%rsp)
   e81d2:	movq   $0x0,0x8(%rsp)
   e81db:	lea    0x1f8(%rsp),%rdi
   e81e3:	mov    0x78(%rsp),%rsi
   e81e8:	mov    %r15,%rdx
   e81eb:	mov    0x60(%rsp),%rcx
   e81f0:	mov    0x28(%rsp),%r8
   e81f5:	mov    %r14,%r9
   e81f8:	call   7d030 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   e81fd:	movzbl 0x27(%rsp),%r15d
   e8203:	jmp    e7d4a <meshoptimizer_rs::codec::vertex::decode_owned+0x46a>
   e8208:	cmp    $0x2,%al
   e820a:	je     e8562 <meshoptimizer_rs::codec::vertex::decode_owned+0xc82>
   e8210:	cmp    $0x3,%ecx
   e8213:	je     e8549 <meshoptimizer_rs::codec::vertex::decode_owned+0xc69>
   e8219:	test   %al,%al
   e821b:	lea    -0xd9762(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e8222:	lea    -0xd9639(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e8229:	cmove  %rcx,%rax
   e822d:	test   %r15b,%r15b
   e8230:	lea    -0xd9a97(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e8237:	cmove  %rcx,%rax
   e823b:	mov    %rax,(%rsp)
   e823f:	lea    0x40(%rsp),%rdi
   e8244:	mov    0x38(%rsp),%rsi
   e8249:	mov    0x30(%rsp),%rdx
   e824e:	mov    %rbx,%rcx
   e8251:	lea    0x1f8(%rsp),%r8
   e8259:	mov    0x50(%rsp),%r9
   e825e:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e8263:	cmpb   $0x0,0x40(%rsp)
   e8268:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e826e:	mov    %r12d,%eax
   e8271:	shr    $0x2,%al
   e8274:	and    $0x3,%al
   e8276:	cmp    $0x2,%al
   e8278:	movzbl 0x27(%rsp),%edx
   e827d:	je     e8562 <meshoptimizer_rs::codec::vertex::decode_owned+0xc82>
   e8283:	movzbl %al,%ecx
   e8286:	cmp    $0x3,%ecx
   e8289:	je     e8549 <meshoptimizer_rs::codec::vertex::decode_owned+0xc69>
   e828f:	test   %al,%al
   e8291:	lea    -0xd97d8(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e8298:	lea    -0xd96af(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e829f:	cmove  %rcx,%rax
   e82a3:	mov    0x48(%rsp),%rcx
   e82a8:	test   %dl,%dl
   e82aa:	lea    -0xd9b11(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e82b1:	cmove  %rdx,%rax
   e82b5:	mov    %rax,(%rsp)
   e82b9:	lea    0x40(%rsp),%rdi
   e82be:	mov    0x38(%rsp),%rsi
   e82c3:	mov    0x30(%rsp),%rdx
   e82c8:	mov    0x90(%rsp),%r8
   e82d0:	mov    0x50(%rsp),%r9
   e82d5:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e82da:	cmpb   $0x0,0x40(%rsp)
   e82df:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e82e5:	mov    %r12d,%eax
   e82e8:	shr    $0x4,%al
   e82eb:	and    $0x3,%al
   e82ed:	cmp    $0x2,%al
   e82ef:	movzbl 0x27(%rsp),%edx
   e82f4:	je     e8562 <meshoptimizer_rs::codec::vertex::decode_owned+0xc82>
   e82fa:	movzbl %al,%ecx
   e82fd:	cmp    $0x3,%ecx
   e8300:	je     e8549 <meshoptimizer_rs::codec::vertex::decode_owned+0xc69>
   e8306:	test   %al,%al
   e8308:	lea    -0xd984f(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e830f:	lea    -0xd9726(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e8316:	cmove  %rcx,%rax
   e831a:	mov    0x48(%rsp),%rcx
   e831f:	test   %dl,%dl
   e8321:	lea    -0xd9b88(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e8328:	cmove  %rdx,%rax
   e832c:	mov    %rax,(%rsp)
   e8330:	lea    0x40(%rsp),%rdi
   e8335:	mov    0x38(%rsp),%rsi
   e833a:	mov    0x30(%rsp),%rdx
   e833f:	mov    0x88(%rsp),%r8
   e8347:	mov    0x50(%rsp),%r9
   e834c:	call   e8580 <meshoptimizer_rs::codec::vertex::bytes>
   e8351:	cmpb   $0x0,0x40(%rsp)
   e8356:	jne    e845b <meshoptimizer_rs::codec::vertex::decode_owned+0xb7b>
   e835c:	shr    $0x6,%r12b
   e8360:	cmp    $0x2,%r12b
   e8364:	movzbl 0x27(%rsp),%edx
   e8369:	je     e8562 <meshoptimizer_rs::codec::vertex::decode_owned+0xc82>
   e836f:	movzbl %r12b,%eax
   e8373:	cmp    $0x3,%eax
   e8376:	je     e8549 <meshoptimizer_rs::codec::vertex::decode_owned+0xc69>
   e837c:	test   %r12b,%r12b
   e837f:	lea    -0xd98c6(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   e8386:	lea    -0xd979d(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   e838d:	cmove  %rcx,%rax
   e8391:	mov    0x48(%rsp),%rcx
   e8396:	test   %dl,%dl
   e8398:	lea    -0xd9bff(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   e839f:	cmove  %rdx,%rax
   e83a3:	mov    %rax,(%rsp)
   e83a7:	lea    0x40(%rsp),%rdi
   e83ac:	mov    0x38(%rsp),%rsi
   e83b1:	mov    0x30(%rsp),%rdx
   e83b6:	jmp    e80cf <meshoptimizer_rs::codec::vertex::decode_owned+0x7ef>
   e83bb:	mov    0xb8(%rsp),%rdx
   e83c3:	mov    0xc8(%rsp),%rdi
   e83cb:	cmp    %rdx,%rdi
   e83ce:	ja     e84fc <meshoptimizer_rs::codec::vertex::decode_owned+0xc1c>
   e83d4:	mov    0x58(%rsp),%rsi
   e83d9:	add    %rdi,%rsi
   e83dc:	lea    0xf0(%rsp),%rdi
   e83e4:	mov    %r14,%rdx
   e83e7:	call   *0x61e6b(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   e83ed:	mov    0xd0(%rsp),%rcx
   e83f5:	mov    %rcx,%rax
   e83f8:	mov    0xc0(%rsp),%r13
   e8400:	cmp    %r13,%rcx
   e8403:	mov    0x30(%rsp),%r15
   e8408:	mov    0x68(%rsp),%rbp
   e840d:	jb     e7b53 <meshoptimizer_rs::codec::vertex::decode_owned+0x273>
   e8413:	jmp    e8442 <meshoptimizer_rs::codec::vertex::decode_owned+0xb62>
   e8415:	mov    $0x1,%ecx
   e841a:	mov    $0x1,%r8d
   e8420:	mov    0x68(%rsp),%r12
   e8425:	mov    %r12,%rdi
   e8428:	mov    %r13,%rsi
   e842b:	mov    %rbp,%rdx
   e842e:	call   8c710 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   e8433:	mov    0x10(%r12),%r13
   e8438:	jmp    e7c10 <meshoptimizer_rs::codec::vertex::decode_owned+0x330>
   e843d:	mov    $0x1,%ebx
   e8442:	sub    %rbx,%r15
   e8445:	setae  %cl
   e8448:	cmp    0xd8(%rsp),%r15
   e8450:	sete   %al
   e8453:	and    %cl,%al
   e8455:	neg    %al
   e8457:	or     $0x7,%al
   e8459:	jmp    e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e845b:	movzbl 0x41(%rsp),%eax
   e8460:	add    $0x5f8,%rsp
   e8467:	pop    %rbx
   e8468:	pop    %r12
   e846a:	pop    %r13
   e846c:	pop    %r14
   e846e:	pop    %r15
   e8470:	pop    %rbp
   e8471:	ret
   e8472:	mov    $0x7,%al
   e8474:	jmp    e8460 <meshoptimizer_rs::codec::vertex::decode_owned+0xb80>
   e8476:	lea    0x5c103(%rip),%rcx        # 144580 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   e847d:	xor    %edi,%edi
   e847f:	mov    0x60(%rsp),%rsi
   e8484:	call   *0x61f26(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e848a:	mov    %rcx,%rdx
   e848d:	lea    0x5c104(%rip),%rcx        # 144598 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   e8494:	mov    %rbp,%rdi
   e8497:	mov    %rdx,%rsi
   e849a:	call   *0x61f10(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e84a0:	lea    0x5c151(%rip),%rcx        # 1445f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   e84a7:	mov    $0x100,%edx
   e84ac:	xor    %edi,%edi
   e84ae:	mov    %r14,%rsi
   e84b1:	call   *0x61ef9(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e84b7:	lea    0x5c122(%rip),%rcx        # 1445e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   e84be:	mov    %rsi,%rdi
   e84c1:	mov    0x70(%rsp),%rsi
   e84c6:	mov    %r15,%rdx
   e84c9:	call   *0x61ee1(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e84cf:	lea    0x60222(%rip),%rcx        # 1486f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6668>
   e84d6:	mov    0x58(%rsp),%rdi
   e84db:	mov    %r15,%rsi
   e84de:	call   *0x61ecc(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e84e4:	lea    0x5c125(%rip),%rcx        # 144610 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   e84eb:	mov    0x70(%rsp),%rdi
   e84f0:	mov    %r15,%rsi
   e84f3:	mov    %r15,%rdx
   e84f6:	call   *0x61eb4(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e84fc:	lea    0x5c035(%rip),%rcx        # 144538 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   e8503:	mov    %rdx,%rsi
   e8506:	call   *0x61ea4(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e850c:	lea    0x5c03d(%rip),%rdx        # 144550 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   e8513:	mov    %r13,%rdi
   e8516:	mov    0x98(%rsp),%rsi
   e851e:	call   *0x61d6c(%rip)        # 14a290 <_DYNAMIC+0x250>
   e8524:	lea    0x5c03d(%rip),%rdx        # 144568 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   e852b:	mov    %r13,%rdi
   e852e:	mov    0xa8(%rsp),%rsi
   e8536:	call   *0x61d54(%rip)        # 14a290 <_DYNAMIC+0x250>
   e853c:	lea    0x5bfdd(%rip),%rdi        # 144520 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2490>
   e8543:	call   *0x61f2f(%rip)        # 14a478 <_DYNAMIC+0x438>
   e8549:	lea    0x5c060(%rip),%rcx        # 1445b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   e8550:	xor    %edi,%edi
   e8552:	mov    0x28(%rsp),%rsi
   e8557:	mov    0x50(%rsp),%rdx
   e855c:	call   *0x61e4e(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   e8562:	lea    0x5c05f(%rip),%rcx        # 1445c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   e8569:	xor    %edi,%edi
   e856b:	mov    0x28(%rsp),%rsi
   e8570:	mov    0x50(%rsp),%rdx
   e8575:	call   *0x61e35(%rip)        # 14a3b0 <_DYNAMIC+0x370>
