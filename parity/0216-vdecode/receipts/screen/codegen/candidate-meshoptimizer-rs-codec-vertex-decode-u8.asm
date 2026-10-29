
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000074bb0 <meshoptimizer_rs::codec::vertex::decode::<[u8]>>:
   74bb0:	push   %rbp
   74bb1:	push   %r15
   74bb3:	push   %r14
   74bb5:	push   %r13
   74bb7:	push   %r12
   74bb9:	push   %rbx
   74bba:	sub    $0x5f8,%rsp
   74bc1:	mov    %r9,%rbx
   74bc4:	mov    %r8,%r15
   74bc7:	mov    %rcx,%r14
   74bca:	mov    %rdx,%rbp
   74bcd:	mov    %rsi,%r12
   74bd0:	mov    %rdi,%r13
   74bd3:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   74bd8:	cmp    $0x2,%al
   74bda:	jae    74c32 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x82>
   74bdc:	mov    $0x7,%al
   74bde:	test   %rbx,%rbx
   74be1:	je     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74be7:	movzbl (%r15),%edx
   74beb:	mov    %edx,%ecx
   74bed:	and    $0xf0,%cl
   74bf0:	cmp    $0xa0,%cl
   74bf3:	jne    756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74bf9:	and    $0xf,%dl
   74bfc:	mov    $0x5,%al
   74bfe:	mov    %dl,0x1f(%rsp)
   74c02:	cmp    $0x2,%dl
   74c05:	jae    756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74c0b:	cmpb   $0x0,0x1f(%rsp)
   74c10:	je     74cc7 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x117>
   74c16:	mov    %r14,%rcx
   74c19:	shr    $0x2,%rcx
   74c1d:	add    %r14,%rcx
   74c20:	cmp    $0x19,%rcx
   74c24:	mov    $0x18,%eax
   74c29:	cmovae %rcx,%rax
   74c2d:	jmp    74cd7 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x127>
   74c32:	cmp    $0x21,%rbp
   74c36:	jae    74c53 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xa3>
   74c38:	cmp    $0x4,%r14
   74c3c:	je     74c80 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xd0>
   74c3e:	cmp    $0xc,%r14
   74c42:	je     74c6e <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbe>
   74c44:	cmp    $0x20,%r14
   74c48:	jne    74c92 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xe2>
   74c4a:	lea    0xd11f(%rip),%rax        # 81d70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   74c51:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c53:	cmp    $0x4,%r14
   74c57:	je     74c89 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xd9>
   74c59:	cmp    $0xc,%r14
   74c5d:	je     74c77 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xc7>
   74c5f:	cmp    $0x20,%r14
   74c63:	jne    74c9b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xeb>
   74c65:	lea    0x6204(%rip),%rax        # 7ae70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   74c6c:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c6e:	lea    0x116db(%rip),%rax        # 86350 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, [u8]>>
   74c75:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c77:	lea    0xa132(%rip),%rax        # 7edb0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, [u8]>>
   74c7e:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c80:	lea    0xfb89(%rip),%rax        # 84810 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   74c87:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c89:	lea    0x8870(%rip),%rax        # 7d500 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   74c90:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c92:	lea    0xb837(%rip),%rax        # 804d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   74c99:	jmp    74ca2 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xf2>
   74c9b:	lea    0x4cee(%rip),%rax        # 79990 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   74ca2:	mov    %r13,%rdi
   74ca5:	mov    %r12,%rsi
   74ca8:	mov    %rbp,%rdx
   74cab:	mov    %r14,%rcx
   74cae:	mov    %r15,%r8
   74cb1:	mov    %rbx,%r9
   74cb4:	add    $0x5f8,%rsp
   74cbb:	pop    %rbx
   74cbc:	pop    %r12
   74cbe:	pop    %r13
   74cc0:	pop    %r14
   74cc2:	pop    %r15
   74cc4:	pop    %rbp
   74cc5:	jmp    *%rax
   74cc7:	cmp    $0x21,%r14
   74ccb:	mov    $0x20,%eax
   74cd0:	cmovae %r14,%rax
   74cd4:	mov    %r14,%rcx
   74cd7:	mov    %rax,0x90(%rsp)
   74cdf:	lea    0x1(%rax),%rdx
   74ce3:	mov    $0x7,%al
   74ce5:	cmp    %rdx,%rbx
   74ce8:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74cee:	mov    %rbx,%rsi
   74cf1:	sub    %rcx,%rsi
   74cf4:	lea    (%rsi,%r14,1),%rax
   74cf8:	mov    %rbx,%rcx
   74cfb:	mov    %rax,0x58(%rsp)
   74d00:	sub    %rax,%rcx
   74d03:	mov    %rcx,0x98(%rsp)
   74d0b:	jb     75726 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb76>
   74d11:	xorps  %xmm0,%xmm0
   74d14:	movaps %xmm0,0x1e0(%rsp)
   74d1c:	movaps %xmm0,0x1d0(%rsp)
   74d24:	movaps %xmm0,0x1c0(%rsp)
   74d2c:	movaps %xmm0,0x1b0(%rsp)
   74d34:	movaps %xmm0,0x1a0(%rsp)
   74d3c:	movaps %xmm0,0x190(%rsp)
   74d44:	movaps %xmm0,0x180(%rsp)
   74d4c:	movaps %xmm0,0x170(%rsp)
   74d54:	movaps %xmm0,0x160(%rsp)
   74d5c:	movaps %xmm0,0x150(%rsp)
   74d64:	movaps %xmm0,0x140(%rsp)
   74d6c:	movaps %xmm0,0x130(%rsp)
   74d74:	movaps %xmm0,0x120(%rsp)
   74d7c:	movaps %xmm0,0x110(%rsp)
   74d84:	movaps %xmm0,0x100(%rsp)
   74d8c:	movaps %xmm0,0xf0(%rsp)
   74d94:	cmp    $0x101,%r14
   74d9b:	jae    756e7 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb37>
   74da1:	lea    (%rsi,%r14,1),%rax
   74da5:	cmp    %rsi,%rax
   74da8:	jb     756fe <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb4e>
   74dae:	add    %r15,%rsi
   74db1:	lea    0xf0(%rsp),%rdi
   74db9:	mov    %r14,%rdx
   74dbc:	call   *0xb59c6(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   74dc2:	test   %r14,%r14
   74dc5:	je     7577e <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbce>
   74dcb:	mov    $0x2000,%ax
   74dcf:	xor    %edx,%edx
   74dd1:	div    %r14w
   74dd5:	and    $0x3ff0,%eax
   74dda:	cmp    $0x100,%eax
   74ddf:	mov    $0x100,%ecx
   74de4:	cmovb  %eax,%ecx
   74de7:	mov    %rcx,0xe0(%rsp)
   74def:	lea    0x1f8(%rsp),%rdi
   74df7:	mov    $0x400,%edx
   74dfc:	xor    %esi,%esi
   74dfe:	call   *0xb5994(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   74e04:	test   %rbp,%rbp
   74e07:	je     75681 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xad1>
   74e0d:	mov    %r13,0xd8(%rsp)
   74e15:	add    %r15,0x58(%rsp)
   74e1a:	mov    %r14,%rax
   74e1d:	shr    $0x2,%rax
   74e21:	mov    %rax,0x88(%rsp)
   74e29:	lea    0x3(%r14),%rax
   74e2d:	shr    $0x2,%rax
   74e31:	mov    %rax,0xe8(%rsp)
   74e39:	mov    $0x1,%r13d
   74e3f:	xor    %ecx,%ecx
   74e41:	mov    %rbx,0x30(%rsp)
   74e46:	mov    %r15,0x28(%rsp)
   74e4b:	mov    %r12,0xc0(%rsp)
   74e53:	mov    %rbp,0xb8(%rsp)
   74e5b:	mov    %rbp,%rdx
   74e5e:	mov    %rcx,%rdi
   74e61:	sub    %rcx,%rdx
   74e64:	mov    0xe0(%rsp),%rax
   74e6c:	cmp    %rax,%rdx
   74e6f:	cmovae %rax,%rdx
   74e73:	mov    %rdx,0x20(%rsp)
   74e78:	cmpb   $0x0,0x1f(%rsp)
   74e7d:	je     74ebb <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x30b>
   74e7f:	mov    %r13,%rcx
   74e82:	mov    $0x7,%al
   74e84:	add    0x88(%rsp),%rcx
   74e8c:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74e92:	cmp    %rbx,%rcx
   74e95:	ja     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   74e9b:	add    %r15,%r13
   74e9e:	mov    0x88(%rsp),%rax
   74ea6:	mov    %rax,0x80(%rsp)
   74eae:	mov    %r13,0xa0(%rsp)
   74eb6:	mov    %rcx,%r13
   74eb9:	jmp    74ed4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x324>
   74ebb:	movq   $0x0,0x80(%rsp)
   74ec7:	mov    $0x1,%eax
   74ecc:	mov    %rax,0xa0(%rsp)
   74ed4:	mov    0x20(%rsp),%rax
   74ed9:	lea    (%rax,%rdi,1),%rsi
   74edd:	imul   %r14,%rdi
   74ee1:	mov    %rsi,0xd0(%rsp)
   74ee9:	imul   %r14,%rsi
   74eed:	cmp    %rdi,%rsi
   74ef0:	jb     75716 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb66>
   74ef6:	cmp    %r12,%rsi
   74ef9:	ja     75716 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb66>
   74eff:	mov    0x20(%rsp),%rax
   74f04:	lea    0xf(%rax),%ecx
   74f07:	and    $0x3f0,%ecx
   74f0d:	mov    %rcx,0x48(%rsp)
   74f12:	mov    %rax,%rcx
   74f15:	imul   %r14,%rcx
   74f19:	mov    %rcx,0xb0(%rsp)
   74f21:	add    0xd8(%rsp),%rdi
   74f29:	mov    %rdi,0xa8(%rsp)
   74f31:	lea    -0x1(%rax),%rcx
   74f35:	imul   %r14,%rcx
   74f39:	mov    %rcx,0xc8(%rsp)
   74f41:	add    $0x4,%rcx
   74f45:	mov    %rcx,0x50(%rsp)
   74f4a:	lea    0x0(,%rax,4),%rcx
   74f52:	mov    %rcx,0x60(%rsp)
   74f57:	lea    0x1f8(%rsp,%rax,1),%rcx
   74f5f:	mov    %rcx,0x78(%rsp)
   74f64:	lea    0x1f8(%rsp,%rax,2),%rcx
   74f6c:	mov    %rcx,0x70(%rsp)
   74f71:	lea    (%rax,%rax,2),%rax
   74f75:	add    %rsp,%rax
   74f78:	add    $0x1f8,%rax
   74f7e:	mov    %rax,0x68(%rsp)
   74f83:	xor    %ebp,%ebp
   74f85:	xor    %r12d,%r12d
   74f88:	movzbl 0x1f(%rsp),%r15d
   74f8e:	jmp    74fea <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x43a>
   74f90:	lea    (%rsp,%r12,1),%rcx
   74f94:	add    $0xf0,%rcx
   74f9b:	shr    $0x4,%al
   74f9e:	neg    %al
   74fa0:	and    $0x1f,%al
   74fa2:	movzbl %al,%eax
   74fa5:	mov    %rcx,(%rsp)
   74fa9:	mov    %eax,0x8(%rsp)
   74fad:	movq   $0x0,0x10(%rsp)
   74fb6:	lea    0x1f8(%rsp),%rdi
   74fbe:	mov    0x60(%rsp),%rsi
   74fc3:	mov    0x50(%rsp),%rcx
   74fc8:	mov    0x20(%rsp),%r8
   74fcd:	mov    %r14,%r9
   74fd0:	call   73070 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   74fd5:	inc    %rbp
   74fd8:	add    $0x4,%r12
   74fdc:	cmp    %rbp,0xe8(%rsp)
   74fe4:	je     75614 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xa64>
   74fea:	test   %r15b,%r15b
   74fed:	je     7500b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x45b>
   74fef:	cmp    0x80(%rsp),%rbp
   74ff7:	jae    7574e <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb9e>
   74ffd:	mov    0xa0(%rsp),%rax
   75005:	movzbl (%rax,%rbp,1),%ebx
   75009:	jmp    7500d <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x45d>
   7500b:	xor    %ebx,%ebx
   7500d:	mov    %ebx,%eax
   7500f:	and    $0x3,%al
   75011:	movzbl %al,%ecx
   75014:	mov    0x48(%rsp),%rdx
   75019:	cmp    %rdx,0x20(%rsp)
   7501e:	ja     7546d <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x8bd>
   75024:	cmp    $0x2,%al
   75026:	je     75065 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x4b5>
   75028:	cmp    $0x3,%ecx
   7502b:	jne    7507f <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x4cf>
   7502d:	mov    %r13,%r15
   75030:	mov    $0x7,%al
   75032:	add    0x20(%rsp),%r15
   75037:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   7503d:	cmp    0x30(%rsp),%r15
   75042:	ja     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   75048:	add    0x28(%rsp),%r13
   7504d:	lea    0x1f8(%rsp),%rdi
   75055:	mov    %r13,%rsi
   75058:	mov    0x20(%rsp),%rdx
   7505d:	call   *0xb5725(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   75063:	jmp    750d9 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x529>
   75065:	lea    0x1f8(%rsp),%rdi
   7506d:	xor    %esi,%esi
   7506f:	mov    0x20(%rsp),%rdx
   75074:	call   *0xb571e(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   7507a:	mov    %r13,%r15
   7507d:	jmp    750d9 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x529>
   7507f:	test   %al,%al
   75081:	lea    -0x67458(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   75088:	lea    -0x6733f(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7508f:	cmove  %rcx,%rax
   75093:	test   %r15b,%r15b
   75096:	lea    -0x6776d(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7509d:	cmove  %rcx,%rax
   750a1:	mov    %rax,(%rsp)
   750a5:	lea    0x38(%rsp),%rdi
   750aa:	mov    0x28(%rsp),%rsi
   750af:	mov    0x30(%rsp),%rdx
   750b4:	mov    %r13,%rcx
   750b7:	lea    0x1f8(%rsp),%r8
   750bf:	mov    0x48(%rsp),%r9
   750c4:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   750c9:	cmpb   $0x0,0x38(%rsp)
   750ce:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   750d4:	mov    0x40(%rsp),%r15
   750d9:	mov    %ebx,%eax
   750db:	shr    $0x2,%al
   750de:	and    $0x3,%al
   750e0:	cmp    $0x3,%al
   750e2:	je     75103 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x553>
   750e4:	movzbl %al,%ecx
   750e7:	cmp    $0x2,%ecx
   750ea:	jne    7513b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x58b>
   750ec:	mov    0x78(%rsp),%rdi
   750f1:	xor    %esi,%esi
   750f3:	mov    0x20(%rsp),%rdx
   750f8:	call   *0xb569a(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   750fe:	jmp    75194 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x5e4>
   75103:	mov    %r15,%r13
   75106:	mov    $0x7,%al
   75108:	add    0x20(%rsp),%r13
   7510d:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   75113:	cmp    0x30(%rsp),%r13
   75118:	ja     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   7511e:	add    0x28(%rsp),%r15
   75123:	mov    0x78(%rsp),%rdi
   75128:	mov    %r15,%rsi
   7512b:	mov    0x20(%rsp),%rdx
   75130:	call   *0xb5652(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   75136:	mov    %r13,%r15
   75139:	jmp    75194 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x5e4>
   7513b:	test   %al,%al
   7513d:	lea    -0x67514(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   75144:	lea    -0x673fb(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7514b:	cmove  %rcx,%rax
   7514f:	cmpb   $0x0,0x1f(%rsp)
   75154:	lea    -0x6782b(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7515b:	cmove  %rcx,%rax
   7515f:	mov    %rax,(%rsp)
   75163:	lea    0x38(%rsp),%rdi
   75168:	mov    0x28(%rsp),%rsi
   7516d:	mov    0x30(%rsp),%rdx
   75172:	mov    %r15,%rcx
   75175:	mov    0x78(%rsp),%r8
   7517a:	mov    0x48(%rsp),%r9
   7517f:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   75184:	cmpb   $0x0,0x38(%rsp)
   75189:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   7518f:	mov    0x40(%rsp),%r15
   75194:	mov    %ebx,%eax
   75196:	shr    $0x4,%al
   75199:	and    $0x3,%al
   7519b:	cmp    $0x3,%al
   7519d:	je     751ca <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x61a>
   7519f:	movzbl %al,%ecx
   751a2:	cmp    $0x2,%ecx
   751a5:	jne    75249 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x699>
   751ab:	mov    0x70(%rsp),%rdi
   751b0:	xor    %esi,%esi
   751b2:	mov    0x20(%rsp),%rdx
   751b7:	call   *0xb55db(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   751bd:	shr    $0x6,%bl
   751c0:	cmp    $0x3,%bl
   751c3:	jne    7520c <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x65c>
   751c5:	jmp    752ae <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x6fe>
   751ca:	mov    %r15,%r13
   751cd:	mov    $0x7,%al
   751cf:	add    0x20(%rsp),%r13
   751d4:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   751da:	cmp    0x30(%rsp),%r13
   751df:	ja     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   751e5:	add    0x28(%rsp),%r15
   751ea:	mov    0x70(%rsp),%rdi
   751ef:	mov    %r15,%rsi
   751f2:	mov    0x20(%rsp),%rdx
   751f7:	call   *0xb558b(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   751fd:	mov    %r13,%r15
   75200:	shr    $0x6,%bl
   75203:	cmp    $0x3,%bl
   75206:	je     752ae <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x6fe>
   7520c:	movzbl %bl,%eax
   7520f:	cmp    $0x2,%eax
   75212:	jne    752fb <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x74b>
   75218:	mov    0x68(%rsp),%rdi
   7521d:	xor    %esi,%esi
   7521f:	mov    0x20(%rsp),%rdx
   75224:	call   *0xb556e(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   7522a:	mov    %r15,%r13
   7522d:	movzbl 0x1f(%rsp),%r15d
   75233:	test   %r15b,%r15b
   75236:	mov    0xb0(%rsp),%rsi
   7523e:	jne    75367 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x7b7>
   75244:	jmp    752f4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x744>
   75249:	test   %al,%al
   7524b:	lea    -0x67622(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   75252:	lea    -0x67509(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   75259:	cmove  %rcx,%rax
   7525d:	cmpb   $0x0,0x1f(%rsp)
   75262:	lea    -0x67939(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   75269:	cmove  %rcx,%rax
   7526d:	mov    %rax,(%rsp)
   75271:	lea    0x38(%rsp),%rdi
   75276:	mov    0x28(%rsp),%rsi
   7527b:	mov    0x30(%rsp),%rdx
   75280:	mov    %r15,%rcx
   75283:	mov    0x70(%rsp),%r8
   75288:	mov    0x48(%rsp),%r9
   7528d:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   75292:	cmpb   $0x0,0x38(%rsp)
   75297:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   7529d:	mov    0x40(%rsp),%r15
   752a2:	shr    $0x6,%bl
   752a5:	cmp    $0x3,%bl
   752a8:	jne    7520c <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x65c>
   752ae:	mov    %r15,%r13
   752b1:	mov    $0x7,%al
   752b3:	add    0x20(%rsp),%r13
   752b8:	jb     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   752be:	cmp    0x30(%rsp),%r13
   752c3:	ja     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   752c9:	add    0x28(%rsp),%r15
   752ce:	mov    0x68(%rsp),%rdi
   752d3:	mov    %r15,%rsi
   752d6:	mov    0x20(%rsp),%rdx
   752db:	call   *0xb54a7(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   752e1:	movzbl 0x1f(%rsp),%r15d
   752e7:	test   %r15b,%r15b
   752ea:	mov    0xb0(%rsp),%rsi
   752f2:	jne    75367 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x7b7>
   752f4:	xor    %eax,%eax
   752f6:	jmp    7537e <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x7ce>
   752fb:	test   %bl,%bl
   752fd:	lea    -0x676d4(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   75304:	lea    -0x675bb(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7530b:	cmove  %rcx,%rax
   7530f:	cmpb   $0x0,0x1f(%rsp)
   75314:	lea    -0x679eb(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7531b:	cmove  %rcx,%rax
   7531f:	mov    %rax,(%rsp)
   75323:	lea    0x38(%rsp),%rdi
   75328:	mov    0x28(%rsp),%rsi
   7532d:	mov    0x30(%rsp),%rdx
   75332:	mov    %r15,%rcx
   75335:	mov    0x68(%rsp),%r8
   7533a:	mov    0x48(%rsp),%r9
   7533f:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   75344:	cmpb   $0x0,0x38(%rsp)
   75349:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   7534f:	mov    0x40(%rsp),%r13
   75354:	movzbl 0x1f(%rsp),%r15d
   7535a:	test   %r15b,%r15b
   7535d:	mov    0xb0(%rsp),%rsi
   75365:	je     752f4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x744>
   75367:	cmp    0x98(%rsp),%rbp
   7536f:	jae    75766 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbb6>
   75375:	mov    0x58(%rsp),%rax
   7537a:	movzbl (%rax,%rbp,1),%eax
   7537e:	mov    %rsi,%rdx
   75381:	sub    %r12,%rdx
   75384:	jb     756d4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb24>
   7538a:	cmp    %rdx,0x50(%rsp)
   7538f:	ja     756c0 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb10>
   75395:	mov    0xa8(%rsp),%rcx
   7539d:	lea    (%rcx,%r12,1),%rdx
   753a1:	mov    %eax,%ecx
   753a3:	and    $0x3,%cl
   753a6:	movzbl %cl,%ecx
   753a9:	lea    -0x64be8(%rip),%rsi        # 107c8 <libm::math::generic::sqrt::RSQRT_TAB+0x3e2>
   753b0:	movslq (%rsi,%rcx,4),%rcx
   753b4:	add    %rsi,%rcx
   753b7:	jmp    *%rcx
   753b9:	mov    %rdx,%r15
   753bc:	lea    (%rsp,%r12,1),%rbx
   753c0:	add    $0xf0,%rbx
   753c7:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   753cc:	test   %al,%al
   753ce:	je     75433 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x883>
   753d0:	mov    %rbx,(%rsp)
   753d4:	lea    0x1f8(%rsp),%rdi
   753dc:	mov    0x60(%rsp),%rsi
   753e1:	mov    %r15,%rdx
   753e4:	mov    0x50(%rsp),%rcx
   753e9:	mov    0x20(%rsp),%r8
   753ee:	mov    %r14,%r9
   753f1:	call   76a10 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   753f6:	jmp    75462 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x8b2>
   753f8:	lea    (%rsp,%r12,1),%rax
   753fc:	add    $0xf0,%rax
   75402:	mov    %rax,(%rsp)
   75406:	movq   $0x0,0x8(%rsp)
   7540f:	lea    0x1f8(%rsp),%rdi
   75417:	mov    0x60(%rsp),%rsi
   7541c:	mov    0x50(%rsp),%rcx
   75421:	mov    0x20(%rsp),%r8
   75426:	mov    %r14,%r9
   75429:	call   72eb0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7542e:	jmp    74fd5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x425>
   75433:	mov    %rbx,(%rsp)
   75437:	movq   $0x0,0x8(%rsp)
   75440:	lea    0x1f8(%rsp),%rdi
   75448:	mov    0x60(%rsp),%rsi
   7544d:	mov    %r15,%rdx
   75450:	mov    0x50(%rsp),%rcx
   75455:	mov    0x20(%rsp),%r8
   7545a:	mov    %r14,%r9
   7545d:	call   72cc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   75462:	movzbl 0x1f(%rsp),%r15d
   75468:	jmp    74fd5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x425>
   7546d:	cmp    $0x2,%al
   7546f:	je     757a4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbf4>
   75475:	cmp    $0x3,%ecx
   75478:	je     7578b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbdb>
   7547e:	test   %al,%al
   75480:	lea    -0x67857(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   75487:	lea    -0x6773e(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7548e:	cmove  %rcx,%rax
   75492:	test   %r15b,%r15b
   75495:	lea    -0x67b6c(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7549c:	cmove  %rcx,%rax
   754a0:	mov    %rax,(%rsp)
   754a4:	lea    0x38(%rsp),%rdi
   754a9:	mov    0x28(%rsp),%rsi
   754ae:	mov    0x30(%rsp),%rdx
   754b3:	mov    %r13,%rcx
   754b6:	lea    0x1f8(%rsp),%r8
   754be:	mov    0x48(%rsp),%r9
   754c3:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   754c8:	cmpb   $0x0,0x38(%rsp)
   754cd:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   754d3:	mov    %ebx,%eax
   754d5:	shr    $0x2,%al
   754d8:	and    $0x3,%al
   754da:	cmp    $0x2,%al
   754dc:	movzbl 0x1f(%rsp),%edx
   754e1:	je     757a4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbf4>
   754e7:	movzbl %al,%ecx
   754ea:	cmp    $0x3,%ecx
   754ed:	je     7578b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbdb>
   754f3:	test   %al,%al
   754f5:	lea    -0x678cc(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   754fc:	lea    -0x677b3(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   75503:	cmove  %rcx,%rax
   75507:	mov    0x40(%rsp),%rcx
   7550c:	test   %dl,%dl
   7550e:	lea    -0x67be5(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   75515:	cmove  %rdx,%rax
   75519:	mov    %rax,(%rsp)
   7551d:	lea    0x38(%rsp),%rdi
   75522:	mov    0x28(%rsp),%rsi
   75527:	mov    0x30(%rsp),%rdx
   7552c:	mov    0x78(%rsp),%r8
   75531:	mov    0x48(%rsp),%r9
   75536:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   7553b:	cmpb   $0x0,0x38(%rsp)
   75540:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   75546:	mov    %ebx,%eax
   75548:	shr    $0x4,%al
   7554b:	and    $0x3,%al
   7554d:	cmp    $0x2,%al
   7554f:	movzbl 0x1f(%rsp),%edx
   75554:	je     757a4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbf4>
   7555a:	movzbl %al,%ecx
   7555d:	cmp    $0x3,%ecx
   75560:	je     7578b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbdb>
   75566:	test   %al,%al
   75568:	lea    -0x6793f(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7556f:	lea    -0x67826(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   75576:	cmove  %rcx,%rax
   7557a:	mov    0x40(%rsp),%rcx
   7557f:	test   %dl,%dl
   75581:	lea    -0x67c58(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   75588:	cmove  %rdx,%rax
   7558c:	mov    %rax,(%rsp)
   75590:	lea    0x38(%rsp),%rdi
   75595:	mov    0x28(%rsp),%rsi
   7559a:	mov    0x30(%rsp),%rdx
   7559f:	mov    0x70(%rsp),%r8
   755a4:	mov    0x48(%rsp),%r9
   755a9:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   755ae:	cmpb   $0x0,0x38(%rsp)
   755b3:	jne    756a5 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xaf5>
   755b9:	shr    $0x6,%bl
   755bc:	cmp    $0x2,%bl
   755bf:	movzbl 0x1f(%rsp),%edx
   755c4:	je     757a4 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbf4>
   755ca:	movzbl %bl,%eax
   755cd:	cmp    $0x3,%eax
   755d0:	je     7578b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xbdb>
   755d6:	test   %bl,%bl
   755d8:	lea    -0x679af(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   755df:	lea    -0x67896(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   755e6:	cmove  %rcx,%rax
   755ea:	mov    0x40(%rsp),%rcx
   755ef:	test   %dl,%dl
   755f1:	lea    -0x67cc8(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   755f8:	cmove  %rdx,%rax
   755fc:	mov    %rax,(%rsp)
   75600:	lea    0x38(%rsp),%rdi
   75605:	mov    0x28(%rsp),%rsi
   7560a:	mov    0x30(%rsp),%rdx
   7560f:	jmp    75335 <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x785>
   75614:	mov    0xb0(%rsp),%rdx
   7561c:	mov    0xc8(%rsp),%rdi
   75624:	cmp    %rdx,%rdi
   75627:	mov    0x28(%rsp),%r15
   7562c:	ja     7573e <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xb8e>
   75632:	mov    0xa8(%rsp),%rsi
   7563a:	add    %rdi,%rsi
   7563d:	lea    0xf0(%rsp),%rdi
   75645:	mov    %r14,%rdx
   75648:	call   *0xb513a(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   7564e:	mov    0xd0(%rsp),%rax
   75656:	mov    %rax,%rcx
   75659:	mov    0xb8(%rsp),%rbp
   75661:	cmp    %rbp,%rax
   75664:	mov    0x30(%rsp),%rbx
   75669:	mov    0xc0(%rsp),%r12
   75671:	mov    0x90(%rsp),%rax
   75679:	jb     74e5b <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0x2ab>
   7567f:	jmp    7568f <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xadf>
   75681:	mov    $0x1,%r13d
   75687:	mov    0x90(%rsp),%rax
   7568f:	sub    %r13,%rbx
   75692:	setae  %cl
   75695:	cmp    %rax,%rbx
   75698:	sete   %dl
   7569b:	mov    $0x7,%al
   7569d:	test   %dl,%cl
   7569f:	je     756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   756a1:	mov    $0xff,%al
   756a3:	jmp    756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   756a5:	movzbl 0x39(%rsp),%eax
   756aa:	add    $0x5f8,%rsp
   756b1:	pop    %rbx
   756b2:	pop    %r12
   756b4:	pop    %r13
   756b6:	pop    %r14
   756b8:	pop    %r15
   756ba:	pop    %rbp
   756bb:	ret
   756bc:	mov    $0x7,%al
   756be:	jmp    756aa <meshoptimizer_rs::codec::vertex::decode::<[u8]>+0xafa>
   756c0:	lea    0xafd21(%rip),%rcx        # 1253e8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   756c7:	xor    %edi,%edi
   756c9:	mov    0x50(%rsp),%rsi
   756ce:	call   *0xb520c(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   756d4:	lea    0xafd25(%rip),%rcx        # 125400 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   756db:	mov    %rsi,%rdx
   756de:	mov    %r12,%rdi
   756e1:	call   *0xb51f9(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   756e7:	lea    0xafd72(%rip),%rcx        # 125460 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   756ee:	mov    $0x100,%edx
   756f3:	xor    %edi,%edi
   756f5:	mov    %r14,%rsi
   756f8:	call   *0xb51e2(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   756fe:	lea    0xafd43(%rip),%rcx        # 125448 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   75705:	mov    %rsi,%rdi
   75708:	mov    0x58(%rsp),%rsi
   7570d:	mov    %rbx,%rdx
   75710:	call   *0xb51ca(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   75716:	lea    0xb3553(%rip),%rcx        # 128c70 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a58>
   7571d:	mov    %r12,%rdx
   75720:	call   *0xb51ba(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   75726:	lea    0xafd4b(%rip),%rcx        # 125478 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   7572d:	mov    0x58(%rsp),%rdi
   75732:	mov    %rbx,%rsi
   75735:	mov    %rbx,%rdx
   75738:	call   *0xb51a2(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   7573e:	lea    0xafc5b(%rip),%rcx        # 1253a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   75745:	mov    %rdx,%rsi
   75748:	call   *0xb5192(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   7574e:	lea    0xafc63(%rip),%rdx        # 1253b8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   75755:	mov    %rbp,%rdi
   75758:	mov    0x80(%rsp),%rsi
   75760:	call   *0xb505a(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   75766:	lea    0xafc63(%rip),%rdx        # 1253d0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   7576d:	mov    %rbp,%rdi
   75770:	mov    0x98(%rsp),%rsi
   75778:	call   *0xb5042(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   7577e:	lea    0xafc03(%rip),%rdi        # 125388 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   75785:	call   *0xb521d(%rip)        # 12a9a8 <_DYNAMIC+0x438>
   7578b:	lea    0xafc86(%rip),%rcx        # 125418 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   75792:	xor    %edi,%edi
   75794:	mov    0x20(%rsp),%rsi
   75799:	mov    0x48(%rsp),%rdx
   7579e:	call   *0xb513c(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   757a4:	lea    0xafc85(%rip),%rcx        # 125430 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   757ab:	xor    %edi,%edi
   757ad:	mov    0x20(%rsp),%rsi
   757b2:	mov    0x48(%rsp),%rdx
   757b7:	call   *0xb5123(%rip)        # 12a8e0 <_DYNAMIC+0x370>
