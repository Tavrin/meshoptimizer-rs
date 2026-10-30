
<artifacts>/guard/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000e9c40 <meshoptimizer_rs::codec::vertex::decode_owned>:
   e9c40:	push   %rbp
   e9c41:	push   %r15
   e9c43:	push   %r14
   e9c45:	push   %r13
   e9c47:	push   %r12
   e9c49:	push   %rbx
   e9c4a:	sub    $0x5f8,%rsp
   e9c51:	mov    %r8,%rbx
   e9c54:	mov    %rcx,%r14
   e9c57:	mov    %rdx,%r12
   e9c5a:	mov    %rsi,%r13
   e9c5d:	mov    %rdi,%rbp
   e9c60:	cmp    $0xc,%rdx
   e9c64:	jne    e9c73 <meshoptimizer_rs::codec::vertex::decode_owned+0x33>
   e9c66:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   e9c6b:	cmp    $0x2,%al
   e9c6d:	jae    e9d3e <meshoptimizer_rs::codec::vertex::decode_owned+0xfe>
   e9c73:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   e9c78:	cmp    $0x2,%al
   e9c7a:	jb     e9ce0 <meshoptimizer_rs::codec::vertex::decode_owned+0xa0>
   e9c7c:	cmp    $0x20,%r12
   e9c80:	lea    -0x66757(%rip),%rax        # 83530 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   e9c87:	lea    -0x67cce(%rip),%rcx        # 81fc0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   e9c8e:	cmove  %rax,%rcx
   e9c92:	lea    -0x60449(%rip),%rax        # 89850 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   e9c99:	lea    -0x61dc0(%rip),%rdx        # 87ee0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   e9ca0:	cmove  %rax,%rdx
   e9ca4:	cmp    $0x4,%r12
   e9ca8:	lea    -0x6534f(%rip),%rax        # 84960 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   e9caf:	cmovne %rcx,%rax
   e9cb3:	lea    -0x5ec8a(%rip),%r9        # 8b030 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   e9cba:	cmovne %rdx,%r9
   e9cbe:	cmp    $0x21,%r13
   e9cc2:	cmovae %rax,%r9
   e9cc6:	mov    %rbp,%rdi
   e9cc9:	mov    %r13,%rsi
   e9ccc:	mov    %r12,%rdx
   e9ccf:	mov    %r14,%rcx
   e9cd2:	mov    %rbx,%r8
   e9cd5:	call   *%r9
   e9cd8:	cmp    $0xfe,%al
   e9cda:	jne    ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9ce0:	mov    $0x7,%al
   e9ce2:	test   %rbx,%rbx
   e9ce5:	je     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9ceb:	movzbl (%r14),%edx
   e9cef:	mov    %edx,%ecx
   e9cf1:	and    $0xf0,%cl
   e9cf4:	cmp    $0xa0,%cl
   e9cf7:	jne    ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9cfd:	and    $0xf,%dl
   e9d00:	mov    $0x5,%al
   e9d02:	mov    %dl,0x27(%rsp)
   e9d06:	cmp    $0x1,%dl
   e9d09:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9d0f:	cmpb   $0x0,0x27(%rsp)
   e9d14:	je     e9d79 <meshoptimizer_rs::codec::vertex::decode_owned+0x139>
   e9d16:	mov    %r12,%rcx
   e9d19:	shr    $0x2,%rcx
   e9d1d:	add    %r12,%rcx
   e9d20:	cmp    $0x19,%rcx
   e9d24:	mov    $0x18,%r15d
   e9d2a:	cmovae %rcx,%r15
   e9d2e:	lea    0x1(%r15),%rdx
   e9d32:	mov    $0x7,%al
   e9d34:	cmp    %rdx,%rbx
   e9d37:	jae    e9d99 <meshoptimizer_rs::codec::vertex::decode_owned+0x159>
   e9d39:	jmp    ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9d3e:	cmp    $0x21,%r13
   e9d42:	lea    -0x5bf19(%rip),%rax        # 8de30 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, alloc::vec::Vec<u8>>>
   e9d49:	lea    -0x62910(%rip),%r9        # 87440 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, alloc::vec::Vec<u8>>>
   e9d50:	cmovb  %rax,%r9
   e9d54:	mov    $0xc,%edx
   e9d59:	mov    %rbp,%rdi
   e9d5c:	mov    %r13,%rsi
   e9d5f:	mov    %r14,%rcx
   e9d62:	mov    %rbx,%r8
   e9d65:	add    $0x5f8,%rsp
   e9d6c:	pop    %rbx
   e9d6d:	pop    %r12
   e9d6f:	pop    %r13
   e9d71:	pop    %r14
   e9d73:	pop    %r15
   e9d75:	pop    %rbp
   e9d76:	jmp    *%r9
   e9d79:	cmp    $0x21,%r12
   e9d7d:	mov    $0x20,%r15d
   e9d83:	cmovae %r12,%r15
   e9d87:	mov    %r12,%rcx
   e9d8a:	lea    0x1(%r15),%rdx
   e9d8e:	mov    $0x7,%al
   e9d90:	cmp    %rdx,%rbx
   e9d93:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9d99:	mov    %rbx,%rsi
   e9d9c:	sub    %rcx,%rsi
   e9d9f:	lea    (%rsi,%r12,1),%rax
   e9da3:	mov    %rbx,%rcx
   e9da6:	mov    %rax,0x70(%rsp)
   e9dab:	sub    %rax,%rcx
   e9dae:	mov    %rcx,0xa8(%rsp)
   e9db6:	jb     ea883 <meshoptimizer_rs::codec::vertex::decode_owned+0xc43>
   e9dbc:	xorps  %xmm0,%xmm0
   e9dbf:	movaps %xmm0,0x1e0(%rsp)
   e9dc7:	movaps %xmm0,0x1d0(%rsp)
   e9dcf:	movaps %xmm0,0x1c0(%rsp)
   e9dd7:	movaps %xmm0,0x1b0(%rsp)
   e9ddf:	movaps %xmm0,0x1a0(%rsp)
   e9de7:	movaps %xmm0,0x190(%rsp)
   e9def:	movaps %xmm0,0x180(%rsp)
   e9df7:	movaps %xmm0,0x170(%rsp)
   e9dff:	movaps %xmm0,0x160(%rsp)
   e9e07:	movaps %xmm0,0x150(%rsp)
   e9e0f:	movaps %xmm0,0x140(%rsp)
   e9e17:	movaps %xmm0,0x130(%rsp)
   e9e1f:	movaps %xmm0,0x120(%rsp)
   e9e27:	movaps %xmm0,0x110(%rsp)
   e9e2f:	movaps %xmm0,0x100(%rsp)
   e9e37:	movaps %xmm0,0xf0(%rsp)
   e9e3f:	cmp    $0x101,%r12
   e9e46:	jae    ea83f <meshoptimizer_rs::codec::vertex::decode_owned+0xbff>
   e9e4c:	lea    (%rsi,%r12,1),%rax
   e9e50:	cmp    %rsi,%rax
   e9e53:	jb     ea856 <meshoptimizer_rs::codec::vertex::decode_owned+0xc16>
   e9e59:	add    %r14,%rsi
   e9e5c:	lea    0xf0(%rsp),%rdi
   e9e64:	mov    %r12,%rdx
   e9e67:	call   *0x6278b(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   e9e6d:	test   %r12,%r12
   e9e70:	je     ea8db <meshoptimizer_rs::codec::vertex::decode_owned+0xc9b>
   e9e76:	mov    %r15,0xd8(%rsp)
   e9e7e:	mov    $0x2000,%ax
   e9e82:	xor    %edx,%edx
   e9e84:	div    %r12w
   e9e88:	and    $0x3ff0,%eax
   e9e8d:	cmp    $0x100,%eax
   e9e92:	mov    $0x100,%ecx
   e9e97:	cmovb  %eax,%ecx
   e9e9a:	mov    %rcx,0xe0(%rsp)
   e9ea2:	lea    0x1f8(%rsp),%rdi
   e9eaa:	mov    $0x400,%edx
   e9eaf:	xor    %esi,%esi
   e9eb1:	call   *0x62751(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   e9eb7:	test   %r13,%r13
   e9eba:	je     ea7db <meshoptimizer_rs::codec::vertex::decode_owned+0xb9b>
   e9ec0:	add    %r14,0x70(%rsp)
   e9ec5:	mov    %r12,%rax
   e9ec8:	shr    $0x2,%rax
   e9ecc:	mov    %rax,0xa0(%rsp)
   e9ed4:	lea    0x3(%r12),%rax
   e9ed9:	shr    $0x2,%rax
   e9edd:	mov    %rax,0xe8(%rsp)
   e9ee5:	mov    $0x1,%r15d
   e9eeb:	xor    %eax,%eax
   e9eed:	mov    %rbp,0x68(%rsp)
   e9ef2:	mov    %rbx,0x30(%rsp)
   e9ef7:	mov    %r14,0x38(%rsp)
   e9efc:	mov    %r13,0xc0(%rsp)
   e9f04:	mov    %r13,%rdx
   e9f07:	mov    %rax,0x58(%rsp)
   e9f0c:	sub    %rax,%rdx
   e9f0f:	mov    0xe0(%rsp),%rax
   e9f17:	cmp    %rax,%rdx
   e9f1a:	cmovae %rax,%rdx
   e9f1e:	mov    %rdx,0x28(%rsp)
   e9f23:	cmpb   $0x0,0x27(%rsp)
   e9f28:	je     e9f6d <meshoptimizer_rs::codec::vertex::decode_owned+0x32d>
   e9f2a:	mov    %r15,%rcx
   e9f2d:	mov    $0x7,%al
   e9f2f:	add    0xa0(%rsp),%rcx
   e9f37:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9f3d:	cmp    %rbx,%rcx
   e9f40:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   e9f46:	add    0x38(%rsp),%r15
   e9f4b:	mov    0xa0(%rsp),%rax
   e9f53:	mov    %rax,0x98(%rsp)
   e9f5b:	mov    %r15,0xb0(%rsp)
   e9f63:	mov    %rcx,%r15
   e9f66:	mov    0x68(%rsp),%rbp
   e9f6b:	jmp    e9f86 <meshoptimizer_rs::codec::vertex::decode_owned+0x346>
   e9f6d:	movq   $0x0,0x98(%rsp)
   e9f79:	mov    $0x1,%eax
   e9f7e:	mov    %rax,0xb0(%rsp)
   e9f86:	mov    0x58(%rsp),%rax
   e9f8b:	mov    0x28(%rsp),%rcx
   e9f90:	lea    (%rcx,%rax,1),%r14
   e9f94:	mov    %r14,0xd0(%rsp)
   e9f9c:	imul   %r12,%r14
   e9fa0:	mov    0x10(%rbp),%r13
   e9fa4:	mov    %rbp,%rax
   e9fa7:	mov    %r14,%rbp
   e9faa:	mov    %r14,%rdx
   e9fad:	sub    %r13,%rbp
   e9fb0:	jbe    e9ffa <meshoptimizer_rs::codec::vertex::decode_owned+0x3ba>
   e9fb2:	mov    (%rax),%rax
   e9fb5:	sub    %r13,%rax
   e9fb8:	cmp    %rax,%rbp
   e9fbb:	ja     ea7b4 <meshoptimizer_rs::codec::vertex::decode_owned+0xb74>
   e9fc1:	mov    0x68(%rsp),%rax
   e9fc6:	mov    0x8(%rax),%rbx
   e9fca:	lea    (%rbx,%r13,1),%rdi
   e9fce:	cmp    $0x2,%rbp
   e9fd2:	jb     e9fec <meshoptimizer_rs::codec::vertex::decode_owned+0x3ac>
   e9fd4:	lea    -0x1(%rbp),%rdx
   e9fd8:	xor    %esi,%esi
   e9fda:	call   *0x62628(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   e9fe0:	add    %rbp,%r13
   e9fe3:	dec    %r13
   e9fe6:	add    %r13,%rbx
   e9fe9:	mov    %rbx,%rdi
   e9fec:	movb   $0x0,(%rdi)
   e9fef:	inc    %r13
   e9ff2:	mov    %r13,%rdx
   e9ff5:	mov    0x68(%rsp),%rax
   e9ffa:	mov    0x58(%rsp),%rcx
   e9fff:	imul   %r12,%rcx
   ea003:	mov    %rdx,0x10(%rax)
   ea007:	mov    %rcx,0x58(%rsp)
   ea00c:	cmp    %rcx,%r14
   ea00f:	jb     ea86e <meshoptimizer_rs::codec::vertex::decode_owned+0xc2e>
   ea015:	cmp    %rdx,%r14
   ea018:	ja     ea86e <meshoptimizer_rs::codec::vertex::decode_owned+0xc2e>
   ea01e:	mov    0x28(%rsp),%rdx
   ea023:	lea    0xf(%rdx),%ecx
   ea026:	and    $0x3f0,%ecx
   ea02c:	mov    %rcx,0x50(%rsp)
   ea031:	mov    %rdx,%rcx
   ea034:	imul   %r12,%rcx
   ea038:	mov    %rcx,0xb8(%rsp)
   ea040:	mov    0x58(%rsp),%rcx
   ea045:	add    0x8(%rax),%rcx
   ea049:	mov    %rcx,0x58(%rsp)
   ea04e:	lea    -0x1(%rdx),%rax
   ea052:	imul   %r12,%rax
   ea056:	mov    %rax,0xc8(%rsp)
   ea05e:	add    $0x4,%rax
   ea062:	mov    %rax,0x60(%rsp)
   ea067:	lea    0x0(,%rdx,4),%rax
   ea06f:	mov    %rax,0x78(%rsp)
   ea074:	lea    0x1f8(%rsp,%rdx,1),%rax
   ea07c:	mov    %rax,0x90(%rsp)
   ea084:	lea    0x1f8(%rsp,%rdx,2),%rax
   ea08c:	mov    %rax,0x88(%rsp)
   ea094:	lea    (%rdx,%rdx,2),%rax
   ea098:	add    %rsp,%rax
   ea09b:	add    $0x1f8,%rax
   ea0a1:	mov    %rax,0x80(%rsp)
   ea0a9:	xor    %r13d,%r13d
   ea0ac:	xor    %ebp,%ebp
   ea0ae:	movzbl 0x27(%rsp),%r14d
   ea0b4:	jmp    ea110 <meshoptimizer_rs::codec::vertex::decode_owned+0x4d0>
   ea0b6:	lea    (%rsp,%rbp,1),%rcx
   ea0ba:	add    $0xf0,%rcx
   ea0c1:	shr    $0x4,%al
   ea0c4:	neg    %al
   ea0c6:	and    $0x1f,%al
   ea0c8:	movzbl %al,%eax
   ea0cb:	mov    %rcx,(%rsp)
   ea0cf:	mov    %eax,0x8(%rsp)
   ea0d3:	movq   $0x0,0x10(%rsp)
   ea0dc:	lea    0x1f8(%rsp),%rdi
   ea0e4:	mov    0x78(%rsp),%rsi
   ea0e9:	mov    0x60(%rsp),%rcx
   ea0ee:	mov    0x28(%rsp),%r8
   ea0f3:	mov    %r12,%r9
   ea0f6:	call   7d5f0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   ea0fb:	inc    %r13
   ea0fe:	add    $0x4,%rbp
   ea102:	cmp    %r13,0xe8(%rsp)
   ea10a:	je     ea75a <meshoptimizer_rs::codec::vertex::decode_owned+0xb1a>
   ea110:	test   %r14b,%r14b
   ea113:	je     ea132 <meshoptimizer_rs::codec::vertex::decode_owned+0x4f2>
   ea115:	cmp    0x98(%rsp),%r13
   ea11d:	jae    ea8ab <meshoptimizer_rs::codec::vertex::decode_owned+0xc6b>
   ea123:	mov    0xb0(%rsp),%rax
   ea12b:	movzbl (%rax,%r13,1),%ebx
   ea130:	jmp    ea134 <meshoptimizer_rs::codec::vertex::decode_owned+0x4f4>
   ea132:	xor    %ebx,%ebx
   ea134:	mov    %ebx,%eax
   ea136:	and    $0x3,%al
   ea138:	movzbl %al,%ecx
   ea13b:	mov    0x50(%rsp),%rdx
   ea140:	cmp    %rdx,0x28(%rsp)
   ea145:	ja     ea5ad <meshoptimizer_rs::codec::vertex::decode_owned+0x96d>
   ea14b:	cmp    $0x2,%al
   ea14d:	je     ea18c <meshoptimizer_rs::codec::vertex::decode_owned+0x54c>
   ea14f:	cmp    $0x3,%ecx
   ea152:	jne    ea1a6 <meshoptimizer_rs::codec::vertex::decode_owned+0x566>
   ea154:	mov    %r15,%r14
   ea157:	mov    $0x7,%al
   ea159:	add    0x28(%rsp),%r14
   ea15e:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea164:	cmp    0x30(%rsp),%r14
   ea169:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea16f:	add    0x38(%rsp),%r15
   ea174:	lea    0x1f8(%rsp),%rdi
   ea17c:	mov    %r15,%rsi
   ea17f:	mov    0x28(%rsp),%rdx
   ea184:	call   *0x6246e(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   ea18a:	jmp    ea200 <meshoptimizer_rs::codec::vertex::decode_owned+0x5c0>
   ea18c:	lea    0x1f8(%rsp),%rdi
   ea194:	xor    %esi,%esi
   ea196:	mov    0x28(%rsp),%rdx
   ea19b:	call   *0x62467(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   ea1a1:	mov    %r15,%r14
   ea1a4:	jmp    ea200 <meshoptimizer_rs::codec::vertex::decode_owned+0x5c0>
   ea1a6:	test   %al,%al
   ea1a8:	lea    -0xdb6ef(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea1af:	lea    -0xdb5c6(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea1b6:	cmove  %rcx,%rax
   ea1ba:	test   %r14b,%r14b
   ea1bd:	lea    -0xdba24(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea1c4:	cmove  %rcx,%rax
   ea1c8:	mov    %rax,(%rsp)
   ea1cc:	lea    0x40(%rsp),%rdi
   ea1d1:	mov    0x38(%rsp),%rsi
   ea1d6:	mov    0x30(%rsp),%rdx
   ea1db:	mov    %r15,%rcx
   ea1de:	lea    0x1f8(%rsp),%r8
   ea1e6:	mov    0x50(%rsp),%r9
   ea1eb:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea1f0:	cmpb   $0x0,0x40(%rsp)
   ea1f5:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea1fb:	mov    0x48(%rsp),%r14
   ea200:	mov    %ebx,%eax
   ea202:	shr    $0x2,%al
   ea205:	and    $0x3,%al
   ea207:	cmp    $0x3,%al
   ea209:	je     ea22d <meshoptimizer_rs::codec::vertex::decode_owned+0x5ed>
   ea20b:	movzbl %al,%ecx
   ea20e:	cmp    $0x2,%ecx
   ea211:	jne    ea268 <meshoptimizer_rs::codec::vertex::decode_owned+0x628>
   ea213:	mov    0x90(%rsp),%rdi
   ea21b:	xor    %esi,%esi
   ea21d:	mov    0x28(%rsp),%rdx
   ea222:	call   *0x623e0(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   ea228:	jmp    ea2c4 <meshoptimizer_rs::codec::vertex::decode_owned+0x684>
   ea22d:	mov    %r14,%r15
   ea230:	mov    $0x7,%al
   ea232:	add    0x28(%rsp),%r15
   ea237:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea23d:	cmp    0x30(%rsp),%r15
   ea242:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea248:	add    0x38(%rsp),%r14
   ea24d:	mov    0x90(%rsp),%rdi
   ea255:	mov    %r14,%rsi
   ea258:	mov    0x28(%rsp),%rdx
   ea25d:	call   *0x62395(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   ea263:	mov    %r15,%r14
   ea266:	jmp    ea2c4 <meshoptimizer_rs::codec::vertex::decode_owned+0x684>
   ea268:	test   %al,%al
   ea26a:	lea    -0xdb7b1(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea271:	lea    -0xdb688(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea278:	cmove  %rcx,%rax
   ea27c:	cmpb   $0x0,0x27(%rsp)
   ea281:	lea    -0xdbae8(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea288:	cmove  %rcx,%rax
   ea28c:	mov    %rax,(%rsp)
   ea290:	lea    0x40(%rsp),%rdi
   ea295:	mov    0x38(%rsp),%rsi
   ea29a:	mov    0x30(%rsp),%rdx
   ea29f:	mov    %r14,%rcx
   ea2a2:	mov    0x90(%rsp),%r8
   ea2aa:	mov    0x50(%rsp),%r9
   ea2af:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea2b4:	cmpb   $0x0,0x40(%rsp)
   ea2b9:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea2bf:	mov    0x48(%rsp),%r14
   ea2c4:	mov    %ebx,%eax
   ea2c6:	shr    $0x4,%al
   ea2c9:	and    $0x3,%al
   ea2cb:	cmp    $0x3,%al
   ea2cd:	je     ea2fd <meshoptimizer_rs::codec::vertex::decode_owned+0x6bd>
   ea2cf:	movzbl %al,%ecx
   ea2d2:	cmp    $0x2,%ecx
   ea2d5:	jne    ea382 <meshoptimizer_rs::codec::vertex::decode_owned+0x742>
   ea2db:	mov    0x88(%rsp),%rdi
   ea2e3:	xor    %esi,%esi
   ea2e5:	mov    0x28(%rsp),%rdx
   ea2ea:	call   *0x62318(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   ea2f0:	shr    $0x6,%bl
   ea2f3:	cmp    $0x3,%bl
   ea2f6:	jne    ea342 <meshoptimizer_rs::codec::vertex::decode_owned+0x702>
   ea2f8:	jmp    ea3ea <meshoptimizer_rs::codec::vertex::decode_owned+0x7aa>
   ea2fd:	mov    %r14,%r15
   ea300:	mov    $0x7,%al
   ea302:	add    0x28(%rsp),%r15
   ea307:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea30d:	cmp    0x30(%rsp),%r15
   ea312:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea318:	add    0x38(%rsp),%r14
   ea31d:	mov    0x88(%rsp),%rdi
   ea325:	mov    %r14,%rsi
   ea328:	mov    0x28(%rsp),%rdx
   ea32d:	call   *0x622c5(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   ea333:	mov    %r15,%r14
   ea336:	shr    $0x6,%bl
   ea339:	cmp    $0x3,%bl
   ea33c:	je     ea3ea <meshoptimizer_rs::codec::vertex::decode_owned+0x7aa>
   ea342:	movzbl %bl,%eax
   ea345:	cmp    $0x2,%eax
   ea348:	jne    ea43a <meshoptimizer_rs::codec::vertex::decode_owned+0x7fa>
   ea34e:	mov    0x80(%rsp),%rdi
   ea356:	xor    %esi,%esi
   ea358:	mov    0x28(%rsp),%rdx
   ea35d:	call   *0x622a5(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   ea363:	mov    %r14,%r15
   ea366:	movzbl 0x27(%rsp),%r14d
   ea36c:	test   %r14b,%r14b
   ea36f:	mov    0xb8(%rsp),%rcx
   ea377:	jne    ea4a9 <meshoptimizer_rs::codec::vertex::decode_owned+0x869>
   ea37d:	jmp    ea433 <meshoptimizer_rs::codec::vertex::decode_owned+0x7f3>
   ea382:	test   %al,%al
   ea384:	lea    -0xdb8cb(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea38b:	lea    -0xdb7a2(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea392:	cmove  %rcx,%rax
   ea396:	cmpb   $0x0,0x27(%rsp)
   ea39b:	lea    -0xdbc02(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea3a2:	cmove  %rcx,%rax
   ea3a6:	mov    %rax,(%rsp)
   ea3aa:	lea    0x40(%rsp),%rdi
   ea3af:	mov    0x38(%rsp),%rsi
   ea3b4:	mov    0x30(%rsp),%rdx
   ea3b9:	mov    %r14,%rcx
   ea3bc:	mov    0x88(%rsp),%r8
   ea3c4:	mov    0x50(%rsp),%r9
   ea3c9:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea3ce:	cmpb   $0x0,0x40(%rsp)
   ea3d3:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea3d9:	mov    0x48(%rsp),%r14
   ea3de:	shr    $0x6,%bl
   ea3e1:	cmp    $0x3,%bl
   ea3e4:	jne    ea342 <meshoptimizer_rs::codec::vertex::decode_owned+0x702>
   ea3ea:	mov    %r14,%r15
   ea3ed:	mov    $0x7,%al
   ea3ef:	add    0x28(%rsp),%r15
   ea3f4:	jb     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea3fa:	cmp    0x30(%rsp),%r15
   ea3ff:	ja     ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea405:	add    0x38(%rsp),%r14
   ea40a:	mov    0x80(%rsp),%rdi
   ea412:	mov    %r14,%rsi
   ea415:	mov    0x28(%rsp),%rdx
   ea41a:	call   *0x621d8(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   ea420:	movzbl 0x27(%rsp),%r14d
   ea426:	test   %r14b,%r14b
   ea429:	mov    0xb8(%rsp),%rcx
   ea431:	jne    ea4a9 <meshoptimizer_rs::codec::vertex::decode_owned+0x869>
   ea433:	xor    %eax,%eax
   ea435:	jmp    ea4c1 <meshoptimizer_rs::codec::vertex::decode_owned+0x881>
   ea43a:	test   %bl,%bl
   ea43c:	lea    -0xdb983(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea443:	lea    -0xdb85a(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea44a:	cmove  %rcx,%rax
   ea44e:	cmpb   $0x0,0x27(%rsp)
   ea453:	lea    -0xdbcba(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea45a:	cmove  %rcx,%rax
   ea45e:	mov    %rax,(%rsp)
   ea462:	lea    0x40(%rsp),%rdi
   ea467:	mov    0x38(%rsp),%rsi
   ea46c:	mov    0x30(%rsp),%rdx
   ea471:	mov    %r14,%rcx
   ea474:	mov    0x80(%rsp),%r8
   ea47c:	mov    0x50(%rsp),%r9
   ea481:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea486:	cmpb   $0x0,0x40(%rsp)
   ea48b:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea491:	mov    0x48(%rsp),%r15
   ea496:	movzbl 0x27(%rsp),%r14d
   ea49c:	test   %r14b,%r14b
   ea49f:	mov    0xb8(%rsp),%rcx
   ea4a7:	je     ea433 <meshoptimizer_rs::codec::vertex::decode_owned+0x7f3>
   ea4a9:	cmp    0xa8(%rsp),%r13
   ea4b1:	jae    ea8c3 <meshoptimizer_rs::codec::vertex::decode_owned+0xc83>
   ea4b7:	mov    0x70(%rsp),%rax
   ea4bc:	movzbl (%rax,%r13,1),%eax
   ea4c1:	mov    %rcx,%rdx
   ea4c4:	sub    %rbp,%rdx
   ea4c7:	jb     ea829 <meshoptimizer_rs::codec::vertex::decode_owned+0xbe9>
   ea4cd:	cmp    %rdx,0x60(%rsp)
   ea4d2:	ja     ea815 <meshoptimizer_rs::codec::vertex::decode_owned+0xbd5>
   ea4d8:	mov    0x58(%rsp),%rcx
   ea4dd:	lea    (%rcx,%rbp,1),%rdx
   ea4e1:	mov    %eax,%ecx
   ea4e3:	and    $0x3,%cl
   ea4e6:	movzbl %cl,%ecx
   ea4e9:	lea    -0xd8b1c(%rip),%rsi        # 119d4 <libm::math::generic::sqrt::RSQRT_TAB+0x72a>
   ea4f0:	movslq (%rsi,%rcx,4),%rcx
   ea4f4:	add    %rsi,%rcx
   ea4f7:	jmp    *%rcx
   ea4f9:	mov    %rdx,%r14
   ea4fc:	lea    (%rsp,%rbp,1),%rbx
   ea500:	add    $0xf0,%rbx
   ea507:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   ea50c:	test   %al,%al
   ea50e:	je     ea573 <meshoptimizer_rs::codec::vertex::decode_owned+0x933>
   ea510:	mov    %rbx,(%rsp)
   ea514:	lea    0x1f8(%rsp),%rdi
   ea51c:	mov    0x78(%rsp),%rsi
   ea521:	mov    %r14,%rdx
   ea524:	mov    0x60(%rsp),%rcx
   ea529:	mov    0x28(%rsp),%r8
   ea52e:	mov    %r12,%r9
   ea531:	call   7fb40 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   ea536:	jmp    ea5a2 <meshoptimizer_rs::codec::vertex::decode_owned+0x962>
   ea538:	lea    (%rsp,%rbp,1),%rax
   ea53c:	add    $0xf0,%rax
   ea542:	mov    %rax,(%rsp)
   ea546:	movq   $0x0,0x8(%rsp)
   ea54f:	lea    0x1f8(%rsp),%rdi
   ea557:	mov    0x78(%rsp),%rsi
   ea55c:	mov    0x60(%rsp),%rcx
   ea561:	mov    0x28(%rsp),%r8
   ea566:	mov    %r12,%r9
   ea569:	call   7d430 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   ea56e:	jmp    ea0fb <meshoptimizer_rs::codec::vertex::decode_owned+0x4bb>
   ea573:	mov    %rbx,(%rsp)
   ea577:	movq   $0x0,0x8(%rsp)
   ea580:	lea    0x1f8(%rsp),%rdi
   ea588:	mov    0x78(%rsp),%rsi
   ea58d:	mov    %r14,%rdx
   ea590:	mov    0x60(%rsp),%rcx
   ea595:	mov    0x28(%rsp),%r8
   ea59a:	mov    %r12,%r9
   ea59d:	call   7d240 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   ea5a2:	movzbl 0x27(%rsp),%r14d
   ea5a8:	jmp    ea0fb <meshoptimizer_rs::codec::vertex::decode_owned+0x4bb>
   ea5ad:	cmp    $0x2,%al
   ea5af:	je     ea901 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   ea5b5:	cmp    $0x3,%ecx
   ea5b8:	je     ea8e8 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   ea5be:	test   %al,%al
   ea5c0:	lea    -0xdbb07(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea5c7:	lea    -0xdb9de(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea5ce:	cmove  %rcx,%rax
   ea5d2:	test   %r14b,%r14b
   ea5d5:	lea    -0xdbe3c(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea5dc:	cmove  %rcx,%rax
   ea5e0:	mov    %rax,(%rsp)
   ea5e4:	lea    0x40(%rsp),%rdi
   ea5e9:	mov    0x38(%rsp),%rsi
   ea5ee:	mov    0x30(%rsp),%rdx
   ea5f3:	mov    %r15,%rcx
   ea5f6:	lea    0x1f8(%rsp),%r8
   ea5fe:	mov    0x50(%rsp),%r9
   ea603:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea608:	cmpb   $0x0,0x40(%rsp)
   ea60d:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea613:	mov    %ebx,%eax
   ea615:	shr    $0x2,%al
   ea618:	and    $0x3,%al
   ea61a:	cmp    $0x2,%al
   ea61c:	movzbl 0x27(%rsp),%edx
   ea621:	je     ea901 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   ea627:	movzbl %al,%ecx
   ea62a:	cmp    $0x3,%ecx
   ea62d:	je     ea8e8 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   ea633:	test   %al,%al
   ea635:	lea    -0xdbb7c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea63c:	lea    -0xdba53(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea643:	cmove  %rcx,%rax
   ea647:	mov    0x48(%rsp),%rcx
   ea64c:	test   %dl,%dl
   ea64e:	lea    -0xdbeb5(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea655:	cmove  %rdx,%rax
   ea659:	mov    %rax,(%rsp)
   ea65d:	lea    0x40(%rsp),%rdi
   ea662:	mov    0x38(%rsp),%rsi
   ea667:	mov    0x30(%rsp),%rdx
   ea66c:	mov    0x90(%rsp),%r8
   ea674:	mov    0x50(%rsp),%r9
   ea679:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea67e:	cmpb   $0x0,0x40(%rsp)
   ea683:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea689:	mov    %ebx,%eax
   ea68b:	shr    $0x4,%al
   ea68e:	and    $0x3,%al
   ea690:	cmp    $0x2,%al
   ea692:	movzbl 0x27(%rsp),%edx
   ea697:	je     ea901 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   ea69d:	movzbl %al,%ecx
   ea6a0:	cmp    $0x3,%ecx
   ea6a3:	je     ea8e8 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   ea6a9:	test   %al,%al
   ea6ab:	lea    -0xdbbf2(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea6b2:	lea    -0xdbac9(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea6b9:	cmove  %rcx,%rax
   ea6bd:	mov    0x48(%rsp),%rcx
   ea6c2:	test   %dl,%dl
   ea6c4:	lea    -0xdbf2b(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea6cb:	cmove  %rdx,%rax
   ea6cf:	mov    %rax,(%rsp)
   ea6d3:	lea    0x40(%rsp),%rdi
   ea6d8:	mov    0x38(%rsp),%rsi
   ea6dd:	mov    0x30(%rsp),%rdx
   ea6e2:	mov    0x88(%rsp),%r8
   ea6ea:	mov    0x50(%rsp),%r9
   ea6ef:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   ea6f4:	cmpb   $0x0,0x40(%rsp)
   ea6f9:	jne    ea7fa <meshoptimizer_rs::codec::vertex::decode_owned+0xbba>
   ea6ff:	shr    $0x6,%bl
   ea702:	cmp    $0x2,%bl
   ea705:	movzbl 0x27(%rsp),%edx
   ea70a:	je     ea901 <meshoptimizer_rs::codec::vertex::decode_owned+0xcc1>
   ea710:	movzbl %bl,%eax
   ea713:	cmp    $0x3,%eax
   ea716:	je     ea8e8 <meshoptimizer_rs::codec::vertex::decode_owned+0xca8>
   ea71c:	test   %bl,%bl
   ea71e:	lea    -0xdbc65(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   ea725:	lea    -0xdbb3c(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   ea72c:	cmove  %rcx,%rax
   ea730:	mov    0x48(%rsp),%rcx
   ea735:	test   %dl,%dl
   ea737:	lea    -0xdbf9e(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ea73e:	cmove  %rdx,%rax
   ea742:	mov    %rax,(%rsp)
   ea746:	lea    0x40(%rsp),%rdi
   ea74b:	mov    0x38(%rsp),%rsi
   ea750:	mov    0x30(%rsp),%rdx
   ea755:	jmp    ea474 <meshoptimizer_rs::codec::vertex::decode_owned+0x834>
   ea75a:	mov    0xb8(%rsp),%rdx
   ea762:	mov    0xc8(%rsp),%rdi
   ea76a:	cmp    %rdx,%rdi
   ea76d:	ja     ea89b <meshoptimizer_rs::codec::vertex::decode_owned+0xc5b>
   ea773:	mov    0x58(%rsp),%rsi
   ea778:	add    %rdi,%rsi
   ea77b:	lea    0xf0(%rsp),%rdi
   ea783:	mov    %r12,%rdx
   ea786:	call   *0x61e6c(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   ea78c:	mov    0xd0(%rsp),%rcx
   ea794:	mov    %rcx,%rax
   ea797:	mov    0xc0(%rsp),%r13
   ea79f:	cmp    %r13,%rcx
   ea7a2:	mov    0x30(%rsp),%rbx
   ea7a7:	mov    0x68(%rsp),%rbp
   ea7ac:	jb     e9f04 <meshoptimizer_rs::codec::vertex::decode_owned+0x2c4>
   ea7b2:	jmp    ea7e1 <meshoptimizer_rs::codec::vertex::decode_owned+0xba1>
   ea7b4:	mov    $0x1,%ecx
   ea7b9:	mov    $0x1,%r8d
   ea7bf:	mov    0x68(%rsp),%rbx
   ea7c4:	mov    %rbx,%rdi
   ea7c7:	mov    %r13,%rsi
   ea7ca:	mov    %rbp,%rdx
   ea7cd:	call   8ea70 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   ea7d2:	mov    0x10(%rbx),%r13
   ea7d6:	jmp    e9fc1 <meshoptimizer_rs::codec::vertex::decode_owned+0x381>
   ea7db:	mov    $0x1,%r15d
   ea7e1:	sub    %r15,%rbx
   ea7e4:	setae  %cl
   ea7e7:	cmp    0xd8(%rsp),%rbx
   ea7ef:	sete   %al
   ea7f2:	and    %cl,%al
   ea7f4:	neg    %al
   ea7f6:	or     $0x7,%al
   ea7f8:	jmp    ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea7fa:	movzbl 0x41(%rsp),%eax
   ea7ff:	add    $0x5f8,%rsp
   ea806:	pop    %rbx
   ea807:	pop    %r12
   ea809:	pop    %r13
   ea80b:	pop    %r14
   ea80d:	pop    %r15
   ea80f:	pop    %rbp
   ea810:	ret
   ea811:	mov    $0x7,%al
   ea813:	jmp    ea7ff <meshoptimizer_rs::codec::vertex::decode_owned+0xbbf>
   ea815:	lea    0x5c104(%rip),%rcx        # 146920 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24f0>
   ea81c:	xor    %edi,%edi
   ea81e:	mov    0x60(%rsp),%rsi
   ea823:	call   *0x61f27(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea829:	mov    %rcx,%rdx
   ea82c:	lea    0x5c105(%rip),%rcx        # 146938 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2508>
   ea833:	mov    %rbp,%rdi
   ea836:	mov    %rdx,%rsi
   ea839:	call   *0x61f11(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea83f:	lea    0x5c152(%rip),%rcx        # 146998 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2568>
   ea846:	mov    $0x100,%edx
   ea84b:	xor    %edi,%edi
   ea84d:	mov    %r12,%rsi
   ea850:	call   *0x61efa(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea856:	lea    0x5c123(%rip),%rcx        # 146980 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2550>
   ea85d:	mov    %rsi,%rdi
   ea860:	mov    0x70(%rsp),%rsi
   ea865:	mov    %rbx,%rdx
   ea868:	call   *0x61ee2(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea86e:	lea    0x60223(%rip),%rcx        # 14aa98 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6668>
   ea875:	mov    0x58(%rsp),%rdi
   ea87a:	mov    %r14,%rsi
   ea87d:	call   *0x61ecd(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea883:	lea    0x5c126(%rip),%rcx        # 1469b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   ea88a:	mov    0x70(%rsp),%rdi
   ea88f:	mov    %rbx,%rsi
   ea892:	mov    %rbx,%rdx
   ea895:	call   *0x61eb5(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea89b:	lea    0x5c036(%rip),%rcx        # 1468d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   ea8a2:	mov    %rdx,%rsi
   ea8a5:	call   *0x61ea5(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea8ab:	lea    0x5c03e(%rip),%rdx        # 1468f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24c0>
   ea8b2:	mov    %r13,%rdi
   ea8b5:	mov    0x98(%rsp),%rsi
   ea8bd:	call   *0x61d6d(%rip)        # 14c630 <_DYNAMIC+0x250>
   ea8c3:	lea    0x5c03e(%rip),%rdx        # 146908 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   ea8ca:	mov    %r13,%rdi
   ea8cd:	mov    0xa8(%rsp),%rsi
   ea8d5:	call   *0x61d55(%rip)        # 14c630 <_DYNAMIC+0x250>
   ea8db:	lea    0x5bfde(%rip),%rdi        # 1468c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2490>
   ea8e2:	call   *0x61f30(%rip)        # 14c818 <_DYNAMIC+0x438>
   ea8e8:	lea    0x5c061(%rip),%rcx        # 146950 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   ea8ef:	xor    %edi,%edi
   ea8f1:	mov    0x28(%rsp),%rsi
   ea8f6:	mov    0x50(%rsp),%rdx
   ea8fb:	call   *0x61e4f(%rip)        # 14c750 <_DYNAMIC+0x370>
   ea901:	lea    0x5c060(%rip),%rcx        # 146968 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   ea908:	xor    %edi,%edi
   ea90a:	mov    0x28(%rsp),%rsi
   ea90f:	mov    0x50(%rsp),%rdx
   ea914:	call   *0x61e36(%rip)        # 14c750 <_DYNAMIC+0x370>
