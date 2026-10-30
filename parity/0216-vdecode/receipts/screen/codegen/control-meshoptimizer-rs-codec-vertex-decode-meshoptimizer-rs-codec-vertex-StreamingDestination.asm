
<artifacts>/screen/driver-control:     file format elf64-x86-64


Disassembly of section .text:

0000000000073af0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73af0:	push   %rbp
   73af1:	push   %r15
   73af3:	push   %r14
   73af5:	push   %r13
   73af7:	push   %r12
   73af9:	push   %rbx
   73afa:	sub    $0x598,%rsp
   73b01:	mov    %rcx,%r15
   73b04:	mov    %rdx,%r14
   73b07:	mov    %rsi,%rbx
   73b0a:	mov    %rdi,%r12
   73b0d:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73b12:	cmp    $0x2,%al
   73b14:	jb     73b48 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   73b16:	cmp    $0x21,%rbx
   73b1a:	lea    0xcedf(%rip),%rax        # 80a00 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73b21:	lea    0x7188(%rip),%r9        # 7acb0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73b28:	cmovb  %rax,%r9
   73b2c:	mov    $0x4,%edx
   73b31:	mov    %r12,%rdi
   73b34:	mov    %rbx,%rsi
   73b37:	mov    %r14,%rcx
   73b3a:	mov    %r15,%r8
   73b3d:	call   *%r9
   73b40:	cmp    $0xfe,%al
   73b42:	jne    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73b48:	mov    $0x7,%al
   73b4a:	test   %r15,%r15
   73b4d:	je     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73b53:	movzbl (%r14),%ebp
   73b57:	mov    %ebp,%ecx
   73b59:	and    $0xf0,%cl
   73b5c:	cmp    $0xa0,%cl
   73b5f:	jne    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73b65:	and    $0xf,%bpl
   73b69:	mov    $0x5,%al
   73b6b:	cmp    $0x1,%bpl
   73b6f:	ja     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73b75:	xor    %eax,%eax
   73b77:	test   %bpl,%bpl
   73b7a:	sete   %al
   73b7d:	lea    0x18(,%rax,8),%rcx
   73b85:	mov    $0x7,%al
   73b87:	cmp    %rcx,%r15
   73b8a:	jbe    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73b90:	cmp    $0x1,%bpl
   73b94:	mov    %r15,%rax
   73b97:	adc    $0x0,%rax
   73b9b:	lea    -0x1(%rax),%rdx
   73b9f:	mov    %rdx,0x70(%rsp)
   73ba4:	cmp    %r15,%rdx
   73ba7:	ja     7477b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   73bad:	mov    %rcx,0x88(%rsp)
   73bb5:	add    $0xfffffffffffffffb,%rax
   73bb9:	pxor   %xmm0,%xmm0
   73bbd:	movdqu %xmm0,0x188(%rsp)
   73bc6:	movdqu %xmm0,0x17c(%rsp)
   73bcf:	movdqu %xmm0,0x16c(%rsp)
   73bd8:	movdqu %xmm0,0x15c(%rsp)
   73be1:	movdqu %xmm0,0x14c(%rsp)
   73bea:	movdqu %xmm0,0x13c(%rsp)
   73bf3:	movdqu %xmm0,0x12c(%rsp)
   73bfc:	movdqu %xmm0,0x11c(%rsp)
   73c05:	movdqu %xmm0,0x10c(%rsp)
   73c0e:	movdqu %xmm0,0xfc(%rsp)
   73c17:	movdqu %xmm0,0xec(%rsp)
   73c20:	movdqu %xmm0,0xdc(%rsp)
   73c29:	movdqu %xmm0,0xcc(%rsp)
   73c32:	movdqu %xmm0,0xbc(%rsp)
   73c3b:	movdqu %xmm0,0xac(%rsp)
   73c44:	movdqu %xmm0,0x9c(%rsp)
   73c4d:	mov    (%r14,%rax,1),%r13d
   73c51:	mov    %r13d,0x98(%rsp)
   73c59:	lea    0x198(%rsp),%rdi
   73c61:	mov    $0x400,%edx
   73c66:	xor    %esi,%esi
   73c68:	call   *0xb311a(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73c6e:	test   %rbx,%rbx
   73c71:	je     73c99 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   73c73:	mov    %r13d,0x5c(%rsp)
   73c78:	mov    $0x1,%r8d
   73c7e:	xor    %ecx,%ecx
   73c80:	mov    %r15,0x50(%rsp)
   73c85:	mov    %r14,0x28(%rsp)
   73c8a:	mov    %rbx,0x80(%rsp)
   73c92:	mov    %bpl,0x27(%rsp)
   73c97:	jmp    73d18 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   73c99:	mov    $0x1,%r8d
   73c9f:	sub    %r8,%r15
   73ca2:	setae  %cl
   73ca5:	cmp    0x88(%rsp),%r15
   73cad:	sete   %al
   73cb0:	and    %cl,%al
   73cb2:	neg    %al
   73cb4:	or     $0x7,%al
   73cb6:	jmp    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73cbb:	mov    $0x1,%ecx
   73cc0:	mov    $0x1,%r8d
   73cc6:	mov    %r13,%rdi
   73cc9:	mov    %rbp,%rsi
   73ccc:	mov    %r15,%rdx
   73ccf:	call   82820 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73cd4:	mov    0x10(%r13),%rbp
   73cd8:	mov    0x8(%r13),%rdi
   73cdc:	add    %rbp,%rdi
   73cdf:	lea    0x8(%r12),%rsi
   73ce4:	mov    %r15,%rdx
   73ce7:	call   *0xb308b(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   73ced:	add    %r15,%rbp
   73cf0:	mov    %rbp,0x10(%r13)
   73cf4:	mov    0x78(%rsp),%rax
   73cf9:	mov    %rax,%rcx
   73cfc:	mov    0x80(%rsp),%rbx
   73d04:	cmp    %rbx,%rax
   73d07:	mov    0x50(%rsp),%r15
   73d0c:	movzbl 0x27(%rsp),%ebp
   73d11:	mov    0x48(%rsp),%r8
   73d16:	jae    73c9f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   73d18:	sub    %rcx,%rbx
   73d1b:	cmp    $0x100,%rbx
   73d22:	mov    $0x100,%r13d
   73d28:	cmovb  %rbx,%r13
   73d2c:	lea    0xf(%r13),%r9d
   73d30:	and    $0x3f0,%r9d
   73d37:	test   %bpl,%bpl
   73d3a:	mov    %r9,0x48(%rsp)
   73d3f:	mov    %rcx,0x90(%rsp)
   73d47:	mov    %rbx,0x60(%rsp)
   73d4c:	je     73ddd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   73d52:	mov    $0x7,%al
   73d54:	cmp    %r15,%r8
   73d57:	jae    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73d5d:	add    %r13,%rcx
   73d60:	mov    %rcx,0x78(%rsp)
   73d65:	lea    0x0(,%r13,4),%rbp
   73d6d:	mov    %rbp,0x408(%r12)
   73d75:	movzbl (%r14,%r8,1),%r14d
   73d7a:	inc    %r8
   73d7d:	mov    %r14d,%ecx
   73d80:	and    $0x3,%ecx
   73d83:	lea    0x0(,%r13,2),%rdx
   73d8b:	add    %r13,%rdx
   73d8e:	mov    %rdx,0x68(%rsp)
   73d93:	cmp    %r9,%r13
   73d96:	ja     744e9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   73d9c:	cmp    $0x2,%cl
   73d9f:	je     73e1d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   73da1:	movzbl %cl,%edx
   73da4:	cmp    $0x3,%edx
   73da7:	jne    73e35 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   73dad:	lea    (%r8,%r13,1),%rbx
   73db1:	cmp    %r15,%rbx
   73db4:	ja     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73dba:	add    0x28(%rsp),%r8
   73dbf:	lea    0x198(%rsp),%rdi
   73dc7:	mov    %r8,%rsi
   73dca:	mov    %r13,%rdx
   73dcd:	call   *0xb2fa5(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   73dd3:	mov    0x50(%rsp),%r15
   73dd8:	jmp    73e7f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   73ddd:	lea    (%rcx,%r13,1),%rax
   73de1:	mov    %rax,0x78(%rsp)
   73de6:	lea    0x0(,%r13,4),%rbp
   73dee:	mov    %rbp,0x408(%r12)
   73df6:	xor    %r14d,%r14d
   73df9:	lea    0x0(,%r13,2),%rax
   73e01:	add    %r13,%rax
   73e04:	mov    %rax,0x68(%rsp)
   73e09:	lea    -0x664e0(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73e10:	mov    %rcx,%rax
   73e13:	cmp    %r9,%r13
   73e16:	jbe    73e4e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   73e18:	jmp    74514 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   73e1d:	lea    0x198(%rsp),%rdi
   73e25:	xor    %esi,%esi
   73e27:	mov    %r13,%rdx
   73e2a:	mov    %r8,%rbx
   73e2d:	call   *0xb2f55(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73e33:	jmp    73e7f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   73e35:	test   %cl,%cl
   73e37:	lea    -0x6620e(%rip),%rcx        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73e3e:	lea    -0x660f5(%rip),%rax        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73e45:	cmove  %rax,%rcx
   73e49:	mov    0x48(%rsp),%r9
   73e4e:	mov    %rcx,(%rsp)
   73e52:	lea    0x30(%rsp),%rdi
   73e57:	mov    0x28(%rsp),%rsi
   73e5c:	mov    %r15,%rdx
   73e5f:	mov    %r8,%rcx
   73e62:	lea    0x198(%rsp),%r8
   73e6a:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73e6f:	cmpb   $0x0,0x30(%rsp)
   73e74:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73e7a:	mov    0x38(%rsp),%rbx
   73e7f:	mov    %r14d,%eax
   73e82:	shr    $0x2,%al
   73e85:	and    $0x3,%al
   73e87:	lea    (%rsp,%r13,1),%r8
   73e8b:	add    $0x198,%r8
   73e92:	cmp    $0x3,%al
   73e94:	je     73eb1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   73e96:	movzbl %al,%ecx
   73e99:	cmp    $0x2,%ecx
   73e9c:	jne    73ee8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   73e9e:	mov    %r8,%rdi
   73ea1:	xor    %esi,%esi
   73ea3:	mov    %r13,%rdx
   73ea6:	call   *0xb2edc(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73eac:	jmp    73f3a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   73eb1:	mov    %rbx,%r15
   73eb4:	mov    $0x7,%al
   73eb6:	add    %r13,%r15
   73eb9:	jb     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73ebf:	cmp    0x50(%rsp),%r15
   73ec4:	ja     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73eca:	add    0x28(%rsp),%rbx
   73ecf:	mov    %r8,%rdi
   73ed2:	mov    %rbx,%rsi
   73ed5:	mov    %r13,%rdx
   73ed8:	call   *0xb2e9a(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   73ede:	mov    %r15,%rbx
   73ee1:	mov    0x50(%rsp),%r15
   73ee6:	jmp    73f3a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   73ee8:	test   %al,%al
   73eea:	lea    -0x662c1(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73ef1:	lea    -0x661a8(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73ef8:	cmove  %rcx,%rax
   73efc:	cmpb   $0x0,0x27(%rsp)
   73f01:	lea    -0x665d8(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73f08:	cmove  %rcx,%rax
   73f0c:	mov    %rax,(%rsp)
   73f10:	lea    0x30(%rsp),%rdi
   73f15:	mov    0x28(%rsp),%rsi
   73f1a:	mov    %r15,%rdx
   73f1d:	mov    %rbx,%rcx
   73f20:	mov    0x48(%rsp),%r9
   73f25:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73f2a:	cmpb   $0x0,0x30(%rsp)
   73f2f:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73f35:	mov    0x38(%rsp),%rbx
   73f3a:	mov    %r14d,%eax
   73f3d:	shr    $0x4,%al
   73f40:	and    $0x3,%al
   73f42:	lea    (%rsp,%r13,2),%r8
   73f46:	add    $0x198,%r8
   73f4d:	cmp    $0x3,%al
   73f4f:	je     73f6c <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   73f51:	movzbl %al,%ecx
   73f54:	cmp    $0x2,%ecx
   73f57:	jne    73fa3 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   73f59:	mov    %r8,%rdi
   73f5c:	xor    %esi,%esi
   73f5e:	mov    %r13,%rdx
   73f61:	call   *0xb2e21(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73f67:	jmp    73ff5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   73f6c:	mov    %rbx,%r15
   73f6f:	mov    $0x7,%al
   73f71:	add    %r13,%r15
   73f74:	jb     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73f7a:	cmp    0x50(%rsp),%r15
   73f7f:	ja     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73f85:	add    0x28(%rsp),%rbx
   73f8a:	mov    %r8,%rdi
   73f8d:	mov    %rbx,%rsi
   73f90:	mov    %r13,%rdx
   73f93:	call   *0xb2ddf(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   73f99:	mov    %r15,%rbx
   73f9c:	mov    0x50(%rsp),%r15
   73fa1:	jmp    73ff5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   73fa3:	test   %al,%al
   73fa5:	lea    -0x6637c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73fac:	lea    -0x66263(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73fb3:	cmove  %rcx,%rax
   73fb7:	cmpb   $0x0,0x27(%rsp)
   73fbc:	lea    -0x66693(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73fc3:	cmove  %rcx,%rax
   73fc7:	mov    %rax,(%rsp)
   73fcb:	lea    0x30(%rsp),%rdi
   73fd0:	mov    0x28(%rsp),%rsi
   73fd5:	mov    %r15,%rdx
   73fd8:	mov    %rbx,%rcx
   73fdb:	mov    0x48(%rsp),%r9
   73fe0:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73fe5:	cmpb   $0x0,0x30(%rsp)
   73fea:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73ff0:	mov    0x38(%rsp),%rbx
   73ff5:	shr    $0x6,%r14b
   73ff9:	mov    0x68(%rsp),%rax
   73ffe:	lea    (%rsp,%rax,1),%r8
   74002:	add    $0x198,%r8
   74009:	cmp    $0x3,%r14b
   7400d:	je     7402b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   7400f:	movzbl %r14b,%eax
   74013:	cmp    $0x2,%eax
   74016:	jne    74072 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   74018:	mov    %r8,%rdi
   7401b:	xor    %esi,%esi
   7401d:	mov    %r13,%rdx
   74020:	call   *0xb2d62(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   74026:	jmp    740c5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7402b:	mov    %rbx,%rcx
   7402e:	mov    $0x7,%al
   74030:	add    %r13,%rcx
   74033:	jb     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   74039:	cmp    %r15,%rcx
   7403c:	ja     7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   74042:	mov    0x28(%rsp),%r14
   74047:	add    %r14,%rbx
   7404a:	mov    %r8,%rdi
   7404d:	mov    %rbx,%rsi
   74050:	mov    %r13,%rdx
   74053:	mov    %rcx,%rbx
   74056:	call   *0xb2d1c(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   7405c:	mov    0x50(%rsp),%r15
   74061:	cmpb   $0x0,0x27(%rsp)
   74066:	mov    %rbx,0x48(%rsp)
   7406b:	jne    740d6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   7406d:	jmp    740fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   74072:	test   %r14b,%r14b
   74075:	lea    -0x6644c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7407c:	lea    -0x66333(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74083:	cmove  %rcx,%rax
   74087:	cmpb   $0x0,0x27(%rsp)
   7408c:	lea    -0x66763(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74093:	cmove  %rcx,%rax
   74097:	mov    %rax,(%rsp)
   7409b:	lea    0x30(%rsp),%rdi
   740a0:	mov    0x28(%rsp),%rsi
   740a5:	mov    %r15,%rdx
   740a8:	mov    %rbx,%rcx
   740ab:	mov    0x48(%rsp),%r9
   740b0:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   740b5:	cmpb   $0x0,0x30(%rsp)
   740ba:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   740c0:	mov    0x38(%rsp),%rbx
   740c5:	mov    0x28(%rsp),%r14
   740ca:	cmpb   $0x0,0x27(%rsp)
   740cf:	mov    %rbx,0x48(%rsp)
   740d4:	je     740fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   740d6:	mov    0x70(%rsp),%rax
   740db:	cmp    %rax,%r15
   740de:	je     747a9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   740e4:	movzbl (%r14,%rax,1),%eax
   740e9:	mov    %eax,%ecx
   740eb:	and    $0x3,%ecx
   740ee:	lea    -0x6393d(%rip),%rdx        # 107b8 <libm::math::generic::sqrt::RSQRT_TAB+0x3d2>
   740f5:	movslq (%rdx,%rcx,4),%rcx
   740f9:	add    %rdx,%rcx
   740fc:	jmp    *%rcx
   740fe:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   74103:	test   %al,%al
   74105:	je     743d4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   7410b:	movd   0x5c(%rsp),%xmm0
   74111:	pshufd $0x0,%xmm0,%xmm0
   74116:	mov    %r13,%rcx
   74119:	shr    $0x4,%rcx
   7411d:	je     74379 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   74123:	lea    0x0(,%r13,2),%rdx
   7412b:	lea    0x0(,%r13,2),%rsi
   74133:	add    %r13,%rsi
   74136:	mov    %r13,%r8
   74139:	neg    %r8
   7413c:	mov    %rdx,%r10
   7413f:	neg    %r10
   74142:	mov    %rsi,%r14
   74145:	neg    %r14
   74148:	lea    (%rsp,%rsi,1),%r15
   7414c:	add    $0x198,%r15
   74153:	shl    $0x4,%rcx
   74157:	xor    %eax,%eax
   74159:	mov    %rbp,%r9
   7415c:	xor    %edi,%edi
   7415e:	movdqa -0x66807(%rip),%xmm8        # d960 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   74167:	movdqa -0x669a0(%rip),%xmm9        # d7d0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x901>
   74170:	cmp    %rbp,%rdi
   74173:	ja     7473a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   74179:	mov    %r9,%r11
   7417c:	sub    $0x10,%r11
   74180:	jb     746b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   74186:	lea    (%rdi,%r13,1),%rbx
   7418a:	cmp    %rbp,%rbx
   7418d:	ja     746d9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   74193:	lea    (%r8,%r9,1),%rbx
   74197:	cmp    $0x10,%rbx
   7419b:	jb     746b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   741a1:	lea    (%rdx,%rdi,1),%rbx
   741a5:	cmp    %rbp,%rbx
   741a8:	ja     746f0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   741ae:	lea    (%r10,%r9,1),%rbx
   741b2:	cmp    $0x10,%rbx
   741b6:	jb     746b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   741bc:	lea    (%rsi,%rdi,1),%rbx
   741c0:	cmp    %rbp,%rbx
   741c3:	ja     7471e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   741c9:	add    %r14,%r9
   741cc:	cmp    $0x10,%r9
   741d0:	jb     746b1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   741d6:	lea    0x3f(%rax),%r9
   741da:	cmp    %rbp,%r9
   741dd:	jae    746c2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   741e3:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   741ec:	lea    (%rsp,%r13,1),%r9
   741f0:	add    $0x198,%r9
   741f7:	movdqu (%r9,%rdi,1),%xmm4
   741fd:	lea    0x198(%rsp,%r13,2),%r9
   74205:	movdqu (%r9,%rdi,1),%xmm5
   7420b:	movdqa %xmm3,%xmm2
   7420f:	punpckhbw %xmm4,%xmm2
   74213:	movdqu (%r15,%rdi,1),%xmm6
   74219:	movdqa %xmm5,%xmm7
   7421d:	punpckhbw %xmm6,%xmm7
   74221:	movdqa %xmm2,%xmm1
   74225:	punpckhwd %xmm7,%xmm1
   74229:	punpcklwd %xmm7,%xmm2
   7422d:	punpcklbw %xmm4,%xmm3
   74231:	punpcklbw %xmm6,%xmm5
   74235:	movdqa %xmm3,%xmm4
   74239:	punpckhwd %xmm5,%xmm4
   7423d:	punpcklwd %xmm5,%xmm3
   74241:	movdqa %xmm3,%xmm5
   74245:	psrlw  $0x1,%xmm5
   7424a:	pand   %xmm8,%xmm3
   7424f:	pxor   %xmm6,%xmm6
   74253:	psubb  %xmm3,%xmm6
   74257:	pand   %xmm9,%xmm5
   7425c:	pxor   %xmm6,%xmm5
   74260:	movdqa %xmm5,%xmm3
   74264:	pslldq $0x4,%xmm3
   74269:	paddb  %xmm5,%xmm3
   7426d:	movdqa %xmm3,%xmm5
   74271:	pslldq $0x8,%xmm5
   74276:	paddb  %xmm0,%xmm3
   7427a:	paddb  %xmm5,%xmm3
   7427e:	movdqu %xmm3,0x8(%r12,%rdi,4)
   74285:	movdqa %xmm4,%xmm0
   74289:	psrlw  $0x1,%xmm0
   7428e:	pand   %xmm8,%xmm4
   74293:	pxor   %xmm5,%xmm5
   74297:	psubb  %xmm4,%xmm5
   7429b:	pand   %xmm9,%xmm0
   742a0:	pxor   %xmm5,%xmm0
   742a4:	movdqa %xmm0,%xmm4
   742a8:	pslldq $0x4,%xmm4
   742ad:	paddb  %xmm0,%xmm4
   742b1:	movdqa %xmm4,%xmm0
   742b5:	pslldq $0x8,%xmm0
   742ba:	pshufd $0xff,%xmm3,%xmm3
   742bf:	paddb  %xmm4,%xmm0
   742c3:	paddb  %xmm3,%xmm0
   742c7:	movdqu %xmm0,0x18(%r12,%rdi,4)
   742ce:	movdqa %xmm2,%xmm3
   742d2:	psrlw  $0x1,%xmm3
   742d7:	pand   %xmm8,%xmm2
   742dc:	pxor   %xmm4,%xmm4
   742e0:	psubb  %xmm2,%xmm4
   742e4:	pand   %xmm9,%xmm3
   742e9:	pxor   %xmm4,%xmm3
   742ed:	movdqa %xmm3,%xmm2
   742f1:	pslldq $0x4,%xmm2
   742f6:	paddb  %xmm3,%xmm2
   742fa:	movdqa %xmm2,%xmm3
   742fe:	pslldq $0x8,%xmm3
   74303:	pshufd $0xff,%xmm0,%xmm0
   74308:	paddb  %xmm2,%xmm3
   7430c:	paddb  %xmm0,%xmm3
   74310:	movdqu %xmm3,0x28(%r12,%rdi,4)
   74317:	movdqa %xmm1,%xmm0
   7431b:	psrlw  $0x1,%xmm0
   74320:	pand   %xmm8,%xmm1
   74325:	pxor   %xmm2,%xmm2
   74329:	psubb  %xmm1,%xmm2
   7432d:	pand   %xmm9,%xmm0
   74332:	pxor   %xmm2,%xmm0
   74336:	movdqa %xmm0,%xmm1
   7433a:	pslldq $0x4,%xmm1
   7433f:	paddb  %xmm0,%xmm1
   74343:	movdqa %xmm1,%xmm0
   74347:	pslldq $0x8,%xmm0
   7434c:	pshufd $0xff,%xmm3,%xmm2
   74351:	paddb  %xmm1,%xmm0
   74355:	paddb  %xmm2,%xmm0
   74359:	movdqu %xmm0,0x38(%r12,%rdi,4)
   74360:	add    $0x10,%rdi
   74364:	pshufd $0xff,%xmm0,%xmm0
   74369:	add    $0x40,%rax
   7436d:	mov    %r11,%r9
   74370:	cmp    %rdi,%rcx
   74373:	jne    74170 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   74379:	mov    %r13d,%eax
   7437c:	and    $0x1f0,%eax
   74381:	cmp    %r13,%rax
   74384:	mov    0x28(%rsp),%r14
   74389:	je     74488 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7438f:	movdqa %xmm0,0x30(%rsp)
   74395:	lea    0x0(,%rax,4),%edx
   7439c:	mov    %rbp,%rcx
   7439f:	sub    %rdx,%rcx
   743a2:	jb     74793 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   743a8:	lea    0x8(%r12),%rsi
   743ad:	add    %rsi,%rdx
   743b0:	mov    %rax,0x8(%rsp)
   743b5:	lea    0x30(%rsp),%rax
   743ba:	mov    %rax,(%rsp)
   743be:	mov    $0x4,%r9d
   743c4:	lea    0x198(%rsp),%rdi
   743cc:	mov    %r13,%r8
   743cf:	mov    %rbp,%rsi
   743d2:	jmp    74405 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   743d4:	lea    0x98(%rsp),%rax
   743dc:	mov    %rax,(%rsp)
   743e0:	movq   $0x0,0x8(%rsp)
   743e9:	mov    $0x4,%r9d
   743ef:	lea    0x198(%rsp),%rdi
   743f7:	mov    %rbp,%rsi
   743fa:	lea    0x8(%r12),%rdx
   743ff:	mov    %r13,%r8
   74402:	mov    %rbp,%rcx
   74405:	call   72930 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7440a:	jmp    74488 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7440c:	shr    $0x4,%al
   7440f:	neg    %al
   74411:	and    $0x1f,%al
   74413:	movzbl %al,%eax
   74416:	lea    0x98(%rsp),%rcx
   7441e:	mov    %rcx,(%rsp)
   74422:	mov    %eax,0x8(%rsp)
   74426:	movq   $0x0,0x10(%rsp)
   7442f:	mov    $0x4,%r9d
   74435:	lea    0x198(%rsp),%rdi
   7443d:	mov    %rbp,%rsi
   74440:	lea    0x8(%r12),%rdx
   74445:	mov    %r13,%r8
   74448:	mov    %rbp,%rcx
   7444b:	call   72ce0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   74450:	jmp    74488 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   74452:	lea    0x98(%rsp),%rax
   7445a:	mov    %rax,(%rsp)
   7445e:	movq   $0x0,0x8(%rsp)
   74467:	mov    $0x4,%r9d
   7446d:	lea    0x198(%rsp),%rdi
   74475:	mov    %rbp,%rsi
   74478:	lea    0x8(%r12),%rdx
   7447d:	mov    %r13,%r8
   74480:	mov    %rbp,%rcx
   74483:	call   72b20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   74488:	mov    0x90(%rsp),%rax
   74490:	cmp    %rax,0x80(%rsp)
   74498:	je     74764 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   7449e:	mov    0x4(%r12,%rbp,1),%eax
   744a3:	mov    %eax,0x98(%rsp)
   744aa:	mov    0x408(%r12),%r15
   744b2:	cmp    $0x401,%r15
   744b9:	jae    7474d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   744bf:	mov    %eax,0x5c(%rsp)
   744c3:	mov    (%r12),%r13
   744c7:	mov    0x0(%r13),%rax
   744cb:	mov    0x10(%r13),%rbp
   744cf:	sub    %rbp,%rax
   744d2:	cmp    %rax,%r15
   744d5:	ja     73cbb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   744db:	test   %r15,%r15
   744de:	jne    73cd8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   744e4:	jmp    73ced <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   744e9:	cmp    $0x2,%ecx
   744ec:	je     747d1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   744f2:	cmp    $0x3,%ecx
   744f5:	je     747ba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   744fb:	test   %cl,%cl
   744fd:	lea    -0x668d4(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74504:	lea    -0x667bb(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7450b:	cmove  %rcx,%rax
   7450f:	mov    0x48(%rsp),%r9
   74514:	mov    %rax,(%rsp)
   74518:	lea    0x30(%rsp),%rdi
   7451d:	mov    0x28(%rsp),%rsi
   74522:	mov    %r15,%rdx
   74525:	mov    %r8,%rcx
   74528:	lea    0x198(%rsp),%r8
   74530:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   74535:	cmpb   $0x0,0x30(%rsp)
   7453a:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74540:	mov    %r14d,%eax
   74543:	shr    $0x2,%al
   74546:	and    $0x3,%al
   74548:	cmp    $0x2,%al
   7454a:	je     747d1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   74550:	movzbl %al,%ecx
   74553:	cmp    $0x3,%ecx
   74556:	je     747ba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7455c:	mov    0x38(%rsp),%rcx
   74561:	test   %al,%al
   74563:	lea    -0x6693a(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7456a:	lea    -0x66821(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74571:	cmove  %rdx,%rax
   74575:	cmpb   $0x0,0x27(%rsp)
   7457a:	lea    0x198(%rsp,%r13,1),%r8
   74582:	lea    -0x66c59(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74589:	cmove  %rdx,%rax
   7458d:	mov    %rax,(%rsp)
   74591:	lea    0x30(%rsp),%rdi
   74596:	mov    0x28(%rsp),%rsi
   7459b:	mov    %r15,%rdx
   7459e:	mov    0x48(%rsp),%r9
   745a3:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   745a8:	cmpb   $0x0,0x30(%rsp)
   745ad:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   745b3:	mov    %r14d,%eax
   745b6:	shr    $0x4,%al
   745b9:	and    $0x3,%al
   745bb:	cmp    $0x2,%al
   745bd:	je     747d1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   745c3:	movzbl %al,%ecx
   745c6:	cmp    $0x3,%ecx
   745c9:	je     747ba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   745cf:	mov    0x38(%rsp),%rcx
   745d4:	test   %al,%al
   745d6:	lea    -0x669ad(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   745dd:	lea    -0x66894(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   745e4:	cmove  %rdx,%rax
   745e8:	cmpb   $0x0,0x27(%rsp)
   745ed:	lea    0x198(%rsp,%r13,2),%r8
   745f5:	lea    -0x66ccc(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   745fc:	cmove  %rdx,%rax
   74600:	mov    %rax,(%rsp)
   74604:	lea    0x30(%rsp),%rdi
   74609:	mov    0x28(%rsp),%rsi
   7460e:	mov    %r15,%rdx
   74611:	mov    0x48(%rsp),%r9
   74616:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   7461b:	cmpb   $0x0,0x30(%rsp)
   74620:	jne    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   74622:	shr    $0x6,%r14b
   74626:	cmp    $0x2,%r14b
   7462a:	je     747d1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   74630:	movzbl %r14b,%eax
   74634:	cmp    $0x3,%eax
   74637:	je     747ba <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7463d:	mov    0x38(%rsp),%rcx
   74642:	test   %r14b,%r14b
   74645:	lea    -0x66a1c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7464c:	lea    -0x66903(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74653:	cmove  %rdx,%rax
   74657:	cmpb   $0x0,0x27(%rsp)
   7465c:	mov    0x68(%rsp),%rdx
   74661:	lea    0x198(%rsp,%rdx,1),%r8
   74669:	lea    -0x66d40(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74670:	cmove  %rdx,%rax
   74674:	mov    %rax,(%rsp)
   74678:	lea    0x30(%rsp),%rdi
   7467d:	mov    0x28(%rsp),%rsi
   74682:	mov    %r15,%rdx
   74685:	mov    0x48(%rsp),%r9
   7468a:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   7468f:	cmpb   $0x0,0x30(%rsp)
   74694:	je     740c0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   7469a:	movzbl 0x31(%rsp),%eax
   7469f:	add    $0x598,%rsp
   746a6:	pop    %rbx
   746a7:	pop    %r12
   746a9:	pop    %r13
   746ab:	pop    %r14
   746ad:	pop    %r15
   746af:	pop    %rbp
   746b0:	ret
   746b1:	lea    0xadf08(%rip),%rdi        # 1225c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2db8>
   746b8:	call   *0xb286a(%rip)        # 126f28 <_DYNAMIC+0x3c8>
   746be:	mov    $0x7,%al
   746c0:	jmp    7469f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   746c2:	lea    0x40(%rax),%rsi
   746c6:	lea    0xad6fb(%rip),%rcx        # 121dc8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25c0>
   746cd:	mov    %rax,%rdi
   746d0:	mov    %rbp,%rdx
   746d3:	call   *0xb27f7(%rip)        # 126ed0 <_DYNAMIC+0x370>
   746d9:	mov    0x60(%rsp),%rcx
   746de:	cmp    $0x100,%rcx
   746e5:	mov    $0x100,%eax
   746ea:	cmovb  %rcx,%rax
   746ee:	jmp    74737 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   746f0:	mov    0x60(%rsp),%rcx
   746f5:	cmp    $0x100,%rcx
   746fc:	mov    $0x100,%eax
   74701:	cmovb  %rcx,%rax
   74705:	mov    %eax,%eax
   74707:	lea    (%rdi,%rax,2),%rdi
   7470b:	lea    0xade96(%rip),%rcx        # 1225a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74712:	mov    %rbp,%rsi
   74715:	mov    %rbp,%rdx
   74718:	call   *0xb27b2(%rip)        # 126ed0 <_DYNAMIC+0x370>
   7471e:	mov    0x60(%rsp),%rcx
   74723:	cmp    $0x100,%rcx
   7472a:	mov    $0x100,%eax
   7472f:	cmovb  %rcx,%rax
   74733:	lea    (%rax,%rax,2),%rax
   74737:	add    %rax,%rdi
   7473a:	lea    0xade67(%rip),%rcx        # 1225a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74741:	mov    %rbp,%rsi
   74744:	mov    %rbp,%rdx
   74747:	call   *0xb2783(%rip)        # 126ed0 <_DYNAMIC+0x370>
   7474d:	lea    0xb0b6c(%rip),%rcx        # 1252c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ab8>
   74754:	mov    $0x400,%edx
   74759:	xor    %edi,%edi
   7475b:	mov    %r15,%rsi
   7475e:	call   *0xb276c(%rip)        # 126ed0 <_DYNAMIC+0x370>
   74764:	lea    -0x4(%rbp),%rdi
   74768:	lea    0xad221(%rip),%rcx        # 121990 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   7476f:	mov    %rbp,%rsi
   74772:	mov    %rbp,%rdx
   74775:	call   *0xb2755(%rip)        # 126ed0 <_DYNAMIC+0x370>
   7477b:	lea    0xad2e6(%rip),%rcx        # 121a68 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   74782:	mov    0x70(%rsp),%rdi
   74787:	mov    %r15,%rsi
   7478a:	mov    %r15,%rdx
   7478d:	call   *0xb273d(%rip)        # 126ed0 <_DYNAMIC+0x370>
   74793:	lea    0xad616(%rip),%rcx        # 121db0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25a8>
   7479a:	mov    %rdx,%rdi
   7479d:	mov    %rbp,%rsi
   747a0:	mov    %rbp,%rdx
   747a3:	call   *0xb2727(%rip)        # 126ed0 <_DYNAMIC+0x370>
   747a9:	lea    0xad210(%rip),%rdx        # 1219c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   747b0:	xor    %edi,%edi
   747b2:	xor    %esi,%esi
   747b4:	call   *0xb25f6(%rip)        # 126db0 <_DYNAMIC+0x250>
   747ba:	lea    0xad247(%rip),%rcx        # 121a08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   747c1:	xor    %edi,%edi
   747c3:	mov    %r13,%rsi
   747c6:	mov    0x48(%rsp),%rdx
   747cb:	call   *0xb26ff(%rip)        # 126ed0 <_DYNAMIC+0x370>
   747d1:	lea    0xad248(%rip),%rcx        # 121a20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   747d8:	xor    %edi,%edi
   747da:	mov    %r13,%rsi
   747dd:	mov    0x48(%rsp),%rdx
   747e2:	call   *0xb26e8(%rip)        # 126ed0 <_DYNAMIC+0x370>
