
<artifacts>/screen2/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000073d70 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73d70:	push   %rbp
   73d71:	push   %r15
   73d73:	push   %r14
   73d75:	push   %r13
   73d77:	push   %r12
   73d79:	push   %rbx
   73d7a:	sub    $0x598,%rsp
   73d81:	mov    %rcx,%rbx
   73d84:	mov    %rdx,%r14
   73d87:	mov    %rsi,%r15
   73d8a:	mov    %rdi,%r12
   73d8d:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73d92:	cmp    $0x2,%al
   73d94:	jae    73eeb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x17b>
   73d9a:	mov    $0x7,%al
   73d9c:	test   %rbx,%rbx
   73d9f:	je     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73da5:	movzbl (%r14),%ebp
   73da9:	mov    %ebp,%ecx
   73dab:	and    $0xf0,%cl
   73dae:	cmp    $0xa0,%cl
   73db1:	jne    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73db7:	and    $0xf,%bpl
   73dbb:	mov    $0x5,%al
   73dbd:	cmp    $0x2,%bpl
   73dc1:	jae    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73dc7:	xor    %eax,%eax
   73dc9:	test   %bpl,%bpl
   73dcc:	sete   %al
   73dcf:	lea    0x18(,%rax,8),%rcx
   73dd7:	mov    $0x7,%al
   73dd9:	cmp    %rcx,%rbx
   73ddc:	jbe    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73de2:	cmp    $0x1,%bpl
   73de6:	mov    %rbx,%rax
   73de9:	adc    $0x0,%rax
   73ded:	lea    -0x1(%rax),%rdx
   73df1:	mov    %rdx,0x70(%rsp)
   73df6:	cmp    %rbx,%rdx
   73df9:	ja     74a02 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc92>
   73dff:	mov    %rcx,0x88(%rsp)
   73e07:	add    $0xfffffffffffffffb,%rax
   73e0b:	pxor   %xmm0,%xmm0
   73e0f:	movdqu %xmm0,0x188(%rsp)
   73e18:	movdqu %xmm0,0x17c(%rsp)
   73e21:	movdqu %xmm0,0x16c(%rsp)
   73e2a:	movdqu %xmm0,0x15c(%rsp)
   73e33:	movdqu %xmm0,0x14c(%rsp)
   73e3c:	movdqu %xmm0,0x13c(%rsp)
   73e45:	movdqu %xmm0,0x12c(%rsp)
   73e4e:	movdqu %xmm0,0x11c(%rsp)
   73e57:	movdqu %xmm0,0x10c(%rsp)
   73e60:	movdqu %xmm0,0xfc(%rsp)
   73e69:	movdqu %xmm0,0xec(%rsp)
   73e72:	movdqu %xmm0,0xdc(%rsp)
   73e7b:	movdqu %xmm0,0xcc(%rsp)
   73e84:	movdqu %xmm0,0xbc(%rsp)
   73e8d:	movdqu %xmm0,0xac(%rsp)
   73e96:	movdqu %xmm0,0x9c(%rsp)
   73e9f:	mov    (%r14,%rax,1),%r13d
   73ea3:	mov    %r13d,0x98(%rsp)
   73eab:	lea    0x198(%rsp),%rdi
   73eb3:	mov    $0x400,%edx
   73eb8:	xor    %esi,%esi
   73eba:	call   *0xb52a8(%rip)        # 129168 <memset@GLIBC_2.2.5>
   73ec0:	test   %r15,%r15
   73ec3:	je     73f26 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1b6>
   73ec5:	mov    %r13d,0x54(%rsp)
   73eca:	mov    $0x1,%r8d
   73ed0:	xor    %esi,%esi
   73ed2:	mov    %r14,0x48(%rsp)
   73ed7:	mov    %rbx,0x68(%rsp)
   73edc:	mov    %r15,0x78(%rsp)
   73ee1:	mov    %bpl,0x1f(%rsp)
   73ee6:	jmp    73fa6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x236>
   73eeb:	cmp    $0x21,%r15
   73eef:	lea    0xe29a(%rip),%rax        # 82190 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73ef6:	lea    0x7aa3(%rip),%r9        # 7b9a0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73efd:	cmovb  %rax,%r9
   73f01:	mov    $0x4,%edx
   73f06:	mov    %r12,%rdi
   73f09:	mov    %r15,%rsi
   73f0c:	mov    %r14,%rcx
   73f0f:	mov    %rbx,%r8
   73f12:	add    $0x598,%rsp
   73f19:	pop    %rbx
   73f1a:	pop    %r12
   73f1c:	pop    %r13
   73f1e:	pop    %r14
   73f20:	pop    %r15
   73f22:	pop    %rbp
   73f23:	jmp    *%r9
   73f26:	mov    $0x1,%r8d
   73f2c:	sub    %r8,%rbx
   73f2f:	setae  %cl
   73f32:	cmp    0x88(%rsp),%rbx
   73f3a:	sete   %dl
   73f3d:	mov    $0x7,%al
   73f3f:	test   %dl,%cl
   73f41:	je     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73f47:	mov    $0xff,%al
   73f49:	jmp    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73f4e:	mov    $0x1,%ecx
   73f53:	mov    $0x1,%r8d
   73f59:	mov    %r13,%rdi
   73f5c:	mov    %rbp,%rsi
   73f5f:	mov    %r14,%rdx
   73f62:	call   84bf0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73f67:	mov    0x10(%r13),%rbp
   73f6b:	mov    0x8(%r13),%rdi
   73f6f:	add    %rbp,%rdi
   73f72:	lea    0x8(%r12),%rsi
   73f77:	mov    %r14,%rdx
   73f7a:	call   *0xb51d8(%rip)        # 129158 <memcpy@GLIBC_2.14>
   73f80:	add    %r14,%rbp
   73f83:	mov    %rbp,0x10(%r13)
   73f87:	mov    0x80(%rsp),%rax
   73f8f:	mov    %rax,%rsi
   73f92:	cmp    %r15,%rax
   73f95:	mov    0x48(%rsp),%r14
   73f9a:	movzbl 0x1f(%rsp),%ebp
   73f9f:	mov    0x40(%rsp),%r8
   73fa4:	jae    73f2c <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1bc>
   73fa6:	sub    %rsi,%r15
   73fa9:	cmp    $0x100,%r15
   73fb0:	mov    $0x100,%r13d
   73fb6:	cmovb  %r15,%r13
   73fba:	lea    0xf(%r13),%r9d
   73fbe:	and    $0x3f0,%r9d
   73fc5:	test   %bpl,%bpl
   73fc8:	mov    %rsi,0x90(%rsp)
   73fd0:	mov    %r15,0x58(%rsp)
   73fd5:	mov    %r9,0x38(%rsp)
   73fda:	je     74070 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x300>
   73fe0:	mov    $0x7,%al
   73fe2:	cmp    %rbx,%r8
   73fe5:	jae    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73feb:	lea    (%rsi,%r13,1),%rdx
   73fef:	lea    0x0(,%r13,4),%rbp
   73ff7:	mov    %rbp,0x408(%r12)
   73fff:	movzbl (%r14,%r8,1),%edi
   74004:	inc    %r8
   74007:	mov    %edi,%ecx
   74009:	and    $0x3,%ecx
   7400c:	lea    0x0(,%r13,2),%r10
   74014:	add    %r13,%r10
   74017:	mov    %r10,0x60(%rsp)
   7401c:	cmp    %r9,%r13
   7401f:	mov    %rdx,0x80(%rsp)
   74027:	mov    %edi,0x40(%rsp)
   7402b:	ja     74776 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa06>
   74031:	cmp    $0x2,%cl
   74034:	je     740b2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x342>
   74036:	movzbl %cl,%edx
   74039:	cmp    $0x3,%edx
   7403c:	jne    740ca <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35a>
   74042:	lea    (%r8,%r13,1),%r15
   74046:	cmp    %rbx,%r15
   74049:	ja     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7404f:	mov    0x48(%rsp),%r14
   74054:	add    %r14,%r8
   74057:	lea    0x198(%rsp),%rdi
   7405f:	mov    %r8,%rsi
   74062:	mov    %r13,%rdx
   74065:	call   *0xb50ed(%rip)        # 129158 <memcpy@GLIBC_2.14>
   7406b:	jmp    7410d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   74070:	lea    (%rsi,%r13,1),%rax
   74074:	mov    %rax,0x80(%rsp)
   7407c:	lea    0x0(,%r13,4),%rbp
   74084:	mov    %rbp,0x408(%r12)
   7408c:	movl   $0x0,0x40(%rsp)
   74094:	lea    0x0(%r13,%r13,2),%rax
   74099:	mov    %rax,0x60(%rsp)
   7409e:	lea    -0x66775(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   740a5:	mov    %rcx,%rax
   740a8:	cmp    %r9,%r13
   740ab:	jbe    740de <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x36e>
   740ad:	jmp    747a1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa31>
   740b2:	lea    0x198(%rsp),%rdi
   740ba:	xor    %esi,%esi
   740bc:	mov    %r13,%rdx
   740bf:	mov    %r8,%r15
   740c2:	call   *0xb50a0(%rip)        # 129168 <memset@GLIBC_2.2.5>
   740c8:	jmp    7410d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   740ca:	test   %cl,%cl
   740cc:	lea    -0x664a3(%rip),%rcx        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   740d3:	lea    -0x6638a(%rip),%rax        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   740da:	cmove  %rax,%rcx
   740de:	mov    %rcx,(%rsp)
   740e2:	lea    0x20(%rsp),%rdi
   740e7:	mov    %r14,%rsi
   740ea:	mov    %rbx,%rdx
   740ed:	mov    %r8,%rcx
   740f0:	lea    0x198(%rsp),%r8
   740f8:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   740fd:	cmpb   $0x0,0x20(%rsp)
   74102:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74108:	mov    0x28(%rsp),%r15
   7410d:	mov    0x40(%rsp),%eax
   74111:	shr    $0x2,%al
   74114:	and    $0x3,%al
   74116:	lea    (%rsp,%r13,1),%r8
   7411a:	add    $0x198,%r8
   74121:	cmp    $0x3,%al
   74123:	je     74140 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3d0>
   74125:	movzbl %al,%ecx
   74128:	cmp    $0x2,%ecx
   7412b:	jne    7417b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x40b>
   7412d:	mov    %r8,%rdi
   74130:	xor    %esi,%esi
   74132:	mov    %r13,%rdx
   74135:	call   *0xb502d(%rip)        # 129168 <memset@GLIBC_2.2.5>
   7413b:	jmp    741cb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   74140:	mov    %r15,%r14
   74143:	mov    $0x7,%al
   74145:	add    %r13,%r14
   74148:	jb     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7414e:	cmp    %rbx,%r14
   74151:	ja     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74157:	mov    0x48(%rsp),%rbx
   7415c:	add    %rbx,%r15
   7415f:	mov    %r8,%rdi
   74162:	mov    %r15,%rsi
   74165:	mov    %r13,%rdx
   74168:	call   *0xb4fea(%rip)        # 129158 <memcpy@GLIBC_2.14>
   7416e:	mov    %r14,%r15
   74171:	mov    %rbx,%r14
   74174:	mov    0x68(%rsp),%rbx
   74179:	jmp    741cb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   7417b:	test   %al,%al
   7417d:	lea    -0x66554(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74184:	lea    -0x6643b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7418b:	cmove  %rcx,%rax
   7418f:	cmpb   $0x0,0x1f(%rsp)
   74194:	lea    -0x6686b(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7419b:	cmove  %rcx,%rax
   7419f:	mov    %rax,(%rsp)
   741a3:	lea    0x20(%rsp),%rdi
   741a8:	mov    %r14,%rsi
   741ab:	mov    %rbx,%rdx
   741ae:	mov    %r15,%rcx
   741b1:	mov    0x38(%rsp),%r9
   741b6:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   741bb:	cmpb   $0x0,0x20(%rsp)
   741c0:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   741c6:	mov    0x28(%rsp),%r15
   741cb:	mov    0x40(%rsp),%eax
   741cf:	shr    $0x4,%al
   741d2:	and    $0x3,%al
   741d4:	lea    (%rsp,%r13,2),%r8
   741d8:	add    $0x198,%r8
   741df:	cmp    $0x3,%al
   741e1:	je     741fe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x48e>
   741e3:	movzbl %al,%ecx
   741e6:	cmp    $0x2,%ecx
   741e9:	jne    74239 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4c9>
   741eb:	mov    %r8,%rdi
   741ee:	xor    %esi,%esi
   741f0:	mov    %r13,%rdx
   741f3:	call   *0xb4f6f(%rip)        # 129168 <memset@GLIBC_2.2.5>
   741f9:	jmp    74289 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   741fe:	mov    %r15,%r14
   74201:	mov    $0x7,%al
   74203:	add    %r13,%r14
   74206:	jb     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7420c:	cmp    %rbx,%r14
   7420f:	ja     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74215:	mov    0x48(%rsp),%rbx
   7421a:	add    %rbx,%r15
   7421d:	mov    %r8,%rdi
   74220:	mov    %r15,%rsi
   74223:	mov    %r13,%rdx
   74226:	call   *0xb4f2c(%rip)        # 129158 <memcpy@GLIBC_2.14>
   7422c:	mov    %r14,%r15
   7422f:	mov    %rbx,%r14
   74232:	mov    0x68(%rsp),%rbx
   74237:	jmp    74289 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   74239:	test   %al,%al
   7423b:	lea    -0x66612(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74242:	lea    -0x664f9(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74249:	cmove  %rcx,%rax
   7424d:	cmpb   $0x0,0x1f(%rsp)
   74252:	lea    -0x66929(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74259:	cmove  %rcx,%rax
   7425d:	mov    %rax,(%rsp)
   74261:	lea    0x20(%rsp),%rdi
   74266:	mov    %r14,%rsi
   74269:	mov    %rbx,%rdx
   7426c:	mov    %r15,%rcx
   7426f:	mov    0x38(%rsp),%r9
   74274:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   74279:	cmpb   $0x0,0x20(%rsp)
   7427e:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74284:	mov    0x28(%rsp),%r15
   74289:	mov    0x40(%rsp),%ecx
   7428d:	shr    $0x6,%cl
   74290:	mov    0x60(%rsp),%rax
   74295:	lea    (%rsp,%rax,1),%r8
   74299:	add    $0x198,%r8
   742a0:	cmp    $0x3,%cl
   742a3:	je     742bd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x54d>
   742a5:	movzbl %cl,%eax
   742a8:	cmp    $0x2,%eax
   742ab:	jne    74307 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x597>
   742ad:	mov    %r8,%rdi
   742b0:	xor    %esi,%esi
   742b2:	mov    %r13,%rdx
   742b5:	call   *0xb4ead(%rip)        # 129168 <memset@GLIBC_2.2.5>
   742bb:	jmp    742ee <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x57e>
   742bd:	mov    %r15,%rcx
   742c0:	mov    $0x7,%al
   742c2:	add    %r13,%rcx
   742c5:	jb     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   742cb:	cmp    %rbx,%rcx
   742ce:	ja     74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   742d4:	mov    0x48(%rsp),%r14
   742d9:	add    %r14,%r15
   742dc:	mov    %r8,%rdi
   742df:	mov    %r15,%rsi
   742e2:	mov    %r13,%rdx
   742e5:	mov    %rcx,%r15
   742e8:	call   *0xb4e6a(%rip)        # 129158 <memcpy@GLIBC_2.14>
   742ee:	mov    %r15,%rsi
   742f1:	cmpb   $0x0,0x1f(%rsp)
   742f6:	mov    0x78(%rsp),%r15
   742fb:	mov    %rsi,0x40(%rsp)
   74300:	jne    74368 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5f8>
   74302:	jmp    74390 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   74307:	test   %cl,%cl
   74309:	lea    -0x666e0(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74310:	lea    -0x665c7(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74317:	cmove  %rcx,%rax
   7431b:	cmpb   $0x0,0x1f(%rsp)
   74320:	lea    -0x669f7(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74327:	cmove  %rcx,%rax
   7432b:	mov    %rax,(%rsp)
   7432f:	lea    0x20(%rsp),%rdi
   74334:	mov    %r14,%rsi
   74337:	mov    %rbx,%rdx
   7433a:	mov    %r15,%rcx
   7433d:	mov    0x38(%rsp),%r9
   74342:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   74347:	cmpb   $0x0,0x20(%rsp)
   7434c:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74352:	mov    0x28(%rsp),%rsi
   74357:	cmpb   $0x0,0x1f(%rsp)
   7435c:	mov    0x78(%rsp),%r15
   74361:	mov    %rsi,0x40(%rsp)
   74366:	je     74390 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   74368:	mov    0x70(%rsp),%rax
   7436d:	cmp    %rax,%rbx
   74370:	je     74a30 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcc0>
   74376:	movzbl (%r14,%rax,1),%eax
   7437b:	mov    %eax,%ecx
   7437d:	and    $0x3,%ecx
   74380:	lea    -0x63bcf(%rip),%rdx        # 107b8 <libm::math::generic::sqrt::RSQRT_TAB+0x3d2>
   74387:	movslq (%rdx,%rcx,4),%rcx
   7438b:	add    %rdx,%rcx
   7438e:	jmp    *%rcx
   74390:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   74395:	test   %al,%al
   74397:	je     74669 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8f9>
   7439d:	movd   0x54(%rsp),%xmm0
   743a3:	pshufd $0x0,%xmm0,%xmm0
   743a8:	mov    %r13,%rcx
   743ab:	shr    $0x4,%rcx
   743af:	je     7460e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x89e>
   743b5:	lea    0x0(,%r13,2),%r10
   743bd:	lea    0x0(,%r13,2),%rsi
   743c5:	add    %r13,%rsi
   743c8:	mov    %r13,%r8
   743cb:	neg    %r8
   743ce:	neg    %r10
   743d1:	mov    %rsi,%r14
   743d4:	neg    %r14
   743d7:	lea    (%rsp,%rsi,1),%r15
   743db:	add    $0x198,%r15
   743e2:	shl    $0x4,%rcx
   743e6:	xor    %eax,%eax
   743e8:	mov    %rbp,%r9
   743eb:	xor    %edi,%edi
   743ed:	movdqa -0x66a96(%rip),%xmm8        # d960 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   743f6:	movdqa -0x66c2f(%rip),%xmm9        # d7d0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x901>
   743ff:	nop
   74400:	cmp    %rbp,%rdi
   74403:	ja     749c1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc51>
   74409:	mov    %r9,%r11
   7440c:	sub    $0x10,%r11
   74410:	jb     74938 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74416:	lea    (%rdi,%r13,1),%rdx
   7441a:	cmp    %rbp,%rdx
   7441d:	ja     74960 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbf0>
   74423:	lea    (%r8,%r9,1),%rdx
   74427:	cmp    $0x10,%rdx
   7442b:	jb     74938 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74431:	lea    0x0(,%r13,2),%rdx
   74439:	add    %rdi,%rdx
   7443c:	cmp    %rbp,%rdx
   7443f:	ja     74977 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc07>
   74445:	lea    (%r10,%r9,1),%rdx
   74449:	cmp    $0x10,%rdx
   7444d:	jb     74938 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74453:	lea    (%rsi,%rdi,1),%rdx
   74457:	cmp    %rbp,%rdx
   7445a:	ja     749a5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc35>
   74460:	add    %r14,%r9
   74463:	cmp    $0x10,%r9
   74467:	jb     74938 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   7446d:	lea    0x3f(%rax),%rdx
   74471:	cmp    %rbp,%rdx
   74474:	jae    74949 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd9>
   7447a:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   74483:	lea    (%rsp,%r13,1),%rdx
   74487:	add    $0x198,%rdx
   7448e:	movdqu (%rdx,%rdi,1),%xmm4
   74493:	lea    0x198(%rsp,%r13,2),%rdx
   7449b:	movdqu (%rdx,%rdi,1),%xmm5
   744a0:	movdqa %xmm3,%xmm2
   744a4:	punpckhbw %xmm4,%xmm2
   744a8:	movdqu (%r15,%rdi,1),%xmm6
   744ae:	movdqa %xmm5,%xmm7
   744b2:	punpckhbw %xmm6,%xmm7
   744b6:	movdqa %xmm2,%xmm1
   744ba:	punpckhwd %xmm7,%xmm1
   744be:	punpcklwd %xmm7,%xmm2
   744c2:	punpcklbw %xmm4,%xmm3
   744c6:	punpcklbw %xmm6,%xmm5
   744ca:	movdqa %xmm3,%xmm4
   744ce:	punpckhwd %xmm5,%xmm4
   744d2:	punpcklwd %xmm5,%xmm3
   744d6:	movdqa %xmm3,%xmm5
   744da:	psrlw  $0x1,%xmm5
   744df:	pand   %xmm8,%xmm3
   744e4:	pxor   %xmm6,%xmm6
   744e8:	psubb  %xmm3,%xmm6
   744ec:	pand   %xmm9,%xmm5
   744f1:	pxor   %xmm6,%xmm5
   744f5:	movdqa %xmm5,%xmm3
   744f9:	pslldq $0x4,%xmm3
   744fe:	paddb  %xmm5,%xmm3
   74502:	movdqa %xmm3,%xmm5
   74506:	pslldq $0x8,%xmm5
   7450b:	paddb  %xmm0,%xmm3
   7450f:	paddb  %xmm5,%xmm3
   74513:	movdqu %xmm3,0x8(%r12,%rdi,4)
   7451a:	movdqa %xmm4,%xmm0
   7451e:	psrlw  $0x1,%xmm0
   74523:	pand   %xmm8,%xmm4
   74528:	pxor   %xmm5,%xmm5
   7452c:	psubb  %xmm4,%xmm5
   74530:	pand   %xmm9,%xmm0
   74535:	pxor   %xmm5,%xmm0
   74539:	movdqa %xmm0,%xmm4
   7453d:	pslldq $0x4,%xmm4
   74542:	paddb  %xmm0,%xmm4
   74546:	movdqa %xmm4,%xmm0
   7454a:	pslldq $0x8,%xmm0
   7454f:	pshufd $0xff,%xmm3,%xmm3
   74554:	paddb  %xmm4,%xmm0
   74558:	paddb  %xmm3,%xmm0
   7455c:	movdqu %xmm0,0x18(%r12,%rdi,4)
   74563:	movdqa %xmm2,%xmm3
   74567:	psrlw  $0x1,%xmm3
   7456c:	pand   %xmm8,%xmm2
   74571:	pxor   %xmm4,%xmm4
   74575:	psubb  %xmm2,%xmm4
   74579:	pand   %xmm9,%xmm3
   7457e:	pxor   %xmm4,%xmm3
   74582:	movdqa %xmm3,%xmm2
   74586:	pslldq $0x4,%xmm2
   7458b:	paddb  %xmm3,%xmm2
   7458f:	movdqa %xmm2,%xmm3
   74593:	pslldq $0x8,%xmm3
   74598:	pshufd $0xff,%xmm0,%xmm0
   7459d:	paddb  %xmm2,%xmm3
   745a1:	paddb  %xmm0,%xmm3
   745a5:	movdqu %xmm3,0x28(%r12,%rdi,4)
   745ac:	movdqa %xmm1,%xmm0
   745b0:	psrlw  $0x1,%xmm0
   745b5:	pand   %xmm8,%xmm1
   745ba:	pxor   %xmm2,%xmm2
   745be:	psubb  %xmm1,%xmm2
   745c2:	pand   %xmm9,%xmm0
   745c7:	pxor   %xmm2,%xmm0
   745cb:	movdqa %xmm0,%xmm1
   745cf:	pslldq $0x4,%xmm1
   745d4:	paddb  %xmm0,%xmm1
   745d8:	movdqa %xmm1,%xmm0
   745dc:	pslldq $0x8,%xmm0
   745e1:	pshufd $0xff,%xmm3,%xmm2
   745e6:	paddb  %xmm1,%xmm0
   745ea:	paddb  %xmm2,%xmm0
   745ee:	movdqu %xmm0,0x38(%r12,%rdi,4)
   745f5:	add    $0x10,%rdi
   745f9:	pshufd $0xff,%xmm0,%xmm0
   745fe:	add    $0x40,%rax
   74602:	mov    %r11,%r9
   74605:	cmp    %rdi,%rcx
   74608:	jne    74400 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x690>
   7460e:	mov    %r13d,%eax
   74611:	and    $0x1f0,%eax
   74616:	cmp    %r13,%rax
   74619:	mov    0x78(%rsp),%r15
   7461e:	je     7471d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   74624:	movdqa %xmm0,0x20(%rsp)
   7462a:	lea    0x0(,%rax,4),%edx
   74631:	mov    %rbp,%rcx
   74634:	sub    %rdx,%rcx
   74637:	jb     74a1a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcaa>
   7463d:	lea    0x8(%r12),%rsi
   74642:	add    %rsi,%rdx
   74645:	mov    %rax,0x8(%rsp)
   7464a:	lea    0x20(%rsp),%rax
   7464f:	mov    %rax,(%rsp)
   74653:	mov    $0x4,%r9d
   74659:	lea    0x198(%rsp),%rdi
   74661:	mov    %r13,%r8
   74664:	mov    %rbp,%rsi
   74667:	jmp    7469a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x92a>
   74669:	lea    0x98(%rsp),%rax
   74671:	mov    %rax,(%rsp)
   74675:	movq   $0x0,0x8(%rsp)
   7467e:	mov    $0x4,%r9d
   74684:	lea    0x198(%rsp),%rdi
   7468c:	mov    %rbp,%rsi
   7468f:	lea    0x8(%r12),%rdx
   74694:	mov    %r13,%r8
   74697:	mov    %rbp,%rcx
   7469a:	call   72b80 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7469f:	jmp    7471d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   746a1:	shr    $0x4,%al
   746a4:	neg    %al
   746a6:	and    $0x1f,%al
   746a8:	movzbl %al,%eax
   746ab:	lea    0x98(%rsp),%rcx
   746b3:	mov    %rcx,(%rsp)
   746b7:	mov    %eax,0x8(%rsp)
   746bb:	movq   $0x0,0x10(%rsp)
   746c4:	mov    $0x4,%r9d
   746ca:	lea    0x198(%rsp),%rdi
   746d2:	mov    %rbp,%rsi
   746d5:	lea    0x8(%r12),%rdx
   746da:	mov    %r13,%r8
   746dd:	mov    %rbp,%rcx
   746e0:	call   72f30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   746e5:	jmp    7471d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   746e7:	lea    0x98(%rsp),%rax
   746ef:	mov    %rax,(%rsp)
   746f3:	movq   $0x0,0x8(%rsp)
   746fc:	mov    $0x4,%r9d
   74702:	lea    0x198(%rsp),%rdi
   7470a:	mov    %rbp,%rsi
   7470d:	lea    0x8(%r12),%rdx
   74712:	mov    %r13,%r8
   74715:	mov    %rbp,%rcx
   74718:	call   72d70 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7471d:	cmp    0x90(%rsp),%r15
   74725:	je     749eb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc7b>
   7472b:	mov    0x4(%r12,%rbp,1),%eax
   74730:	mov    %eax,0x98(%rsp)
   74737:	mov    0x408(%r12),%r14
   7473f:	cmp    $0x401,%r14
   74746:	jae    749d4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc64>
   7474c:	mov    %eax,0x54(%rsp)
   74750:	mov    (%r12),%r13
   74754:	mov    0x0(%r13),%rax
   74758:	mov    0x10(%r13),%rbp
   7475c:	sub    %rbp,%rax
   7475f:	cmp    %rax,%r14
   74762:	ja     73f4e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1de>
   74768:	test   %r14,%r14
   7476b:	jne    73f6b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fb>
   74771:	jmp    73f80 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x210>
   74776:	cmp    $0x2,%ecx
   74779:	je     74a41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7477f:	cmp    $0x3,%ecx
   74782:	je     74a58 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   74788:	test   %cl,%cl
   7478a:	lea    -0x66b61(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74791:	lea    -0x66a48(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74798:	cmove  %rcx,%rax
   7479c:	mov    0x38(%rsp),%r9
   747a1:	mov    %rax,(%rsp)
   747a5:	lea    0x20(%rsp),%rdi
   747aa:	mov    %r14,%rsi
   747ad:	mov    %rbx,%rdx
   747b0:	mov    %r8,%rcx
   747b3:	lea    0x198(%rsp),%r8
   747bb:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   747c0:	cmpb   $0x0,0x20(%rsp)
   747c5:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   747cb:	mov    0x40(%rsp),%eax
   747cf:	shr    $0x2,%al
   747d2:	and    $0x3,%al
   747d4:	cmp    $0x2,%al
   747d6:	je     74a41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   747dc:	movzbl %al,%ecx
   747df:	cmp    $0x3,%ecx
   747e2:	je     74a58 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   747e8:	mov    0x28(%rsp),%rcx
   747ed:	test   %al,%al
   747ef:	lea    -0x66bc6(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   747f6:	lea    -0x66aad(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   747fd:	cmove  %rdx,%rax
   74801:	cmpb   $0x0,0x1f(%rsp)
   74806:	lea    0x198(%rsp,%r13,1),%r8
   7480e:	lea    -0x66ee5(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74815:	cmove  %rdx,%rax
   74819:	mov    %rax,(%rsp)
   7481d:	lea    0x20(%rsp),%rdi
   74822:	mov    %r14,%rsi
   74825:	mov    %rbx,%rdx
   74828:	mov    0x38(%rsp),%r9
   7482d:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   74832:	cmpb   $0x0,0x20(%rsp)
   74837:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7483d:	mov    0x40(%rsp),%eax
   74841:	shr    $0x4,%al
   74844:	and    $0x3,%al
   74846:	cmp    $0x2,%al
   74848:	je     74a41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7484e:	movzbl %al,%ecx
   74851:	cmp    $0x3,%ecx
   74854:	je     74a58 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   7485a:	mov    0x28(%rsp),%rcx
   7485f:	test   %al,%al
   74861:	lea    -0x66c38(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74868:	lea    -0x66b1f(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7486f:	cmove  %rdx,%rax
   74873:	cmpb   $0x0,0x1f(%rsp)
   74878:	lea    0x198(%rsp,%r13,2),%r8
   74880:	lea    -0x66f57(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74887:	cmove  %rdx,%rax
   7488b:	mov    %rax,(%rsp)
   7488f:	lea    0x20(%rsp),%rdi
   74894:	mov    %r14,%rsi
   74897:	mov    %rbx,%rdx
   7489a:	mov    0x38(%rsp),%r9
   7489f:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   748a4:	cmpb   $0x0,0x20(%rsp)
   748a9:	jne    74921 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   748ab:	mov    0x40(%rsp),%edx
   748af:	shr    $0x6,%dl
   748b2:	cmp    $0x2,%dl
   748b5:	je     74a41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   748bb:	movzbl %dl,%eax
   748be:	cmp    $0x3,%eax
   748c1:	je     74a58 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   748c7:	mov    0x28(%rsp),%rcx
   748cc:	test   %dl,%dl
   748ce:	lea    -0x66ca5(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   748d5:	lea    -0x66b8c(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   748dc:	cmove  %rdx,%rax
   748e0:	cmpb   $0x0,0x1f(%rsp)
   748e5:	mov    0x60(%rsp),%rdx
   748ea:	lea    0x198(%rsp,%rdx,1),%r8
   748f2:	lea    -0x66fc9(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   748f9:	cmove  %rdx,%rax
   748fd:	mov    %rax,(%rsp)
   74901:	lea    0x20(%rsp),%rdi
   74906:	mov    %r14,%rsi
   74909:	mov    %rbx,%rdx
   7490c:	mov    0x38(%rsp),%r9
   74911:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   74916:	cmpb   $0x0,0x20(%rsp)
   7491b:	je     74352 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e2>
   74921:	movzbl 0x21(%rsp),%eax
   74926:	add    $0x598,%rsp
   7492d:	pop    %rbx
   7492e:	pop    %r12
   74930:	pop    %r13
   74932:	pop    %r14
   74934:	pop    %r15
   74936:	pop    %rbp
   74937:	ret
   74938:	lea    0xb0061(%rip),%rdi        # 1249a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2db8>
   7493f:	call   *0xb49c3(%rip)        # 129308 <_DYNAMIC+0x3c8>
   74945:	mov    $0x7,%al
   74947:	jmp    74926 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74949:	lea    0x40(%rax),%rsi
   7494d:	lea    0xaf854(%rip),%rcx        # 1241a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25c0>
   74954:	mov    %rax,%rdi
   74957:	mov    %rbp,%rdx
   7495a:	call   *0xb4950(%rip)        # 1292b0 <_DYNAMIC+0x370>
   74960:	mov    0x58(%rsp),%rcx
   74965:	cmp    $0x100,%rcx
   7496c:	mov    $0x100,%eax
   74971:	cmovb  %rcx,%rax
   74975:	jmp    749be <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4e>
   74977:	mov    0x58(%rsp),%rcx
   7497c:	cmp    $0x100,%rcx
   74983:	mov    $0x100,%eax
   74988:	cmovb  %rcx,%rax
   7498c:	mov    %eax,%eax
   7498e:	lea    (%rdi,%rax,2),%rdi
   74992:	lea    0xaffef(%rip),%rcx        # 124988 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74999:	mov    %rbp,%rsi
   7499c:	mov    %rbp,%rdx
   7499f:	call   *0xb490b(%rip)        # 1292b0 <_DYNAMIC+0x370>
   749a5:	mov    0x58(%rsp),%rcx
   749aa:	cmp    $0x100,%rcx
   749b1:	mov    $0x100,%eax
   749b6:	cmovb  %rcx,%rax
   749ba:	lea    (%rax,%rax,2),%rax
   749be:	add    %rax,%rdi
   749c1:	lea    0xaffc0(%rip),%rcx        # 124988 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   749c8:	mov    %rbp,%rsi
   749cb:	mov    %rbp,%rdx
   749ce:	call   *0xb48dc(%rip)        # 1292b0 <_DYNAMIC+0x370>
   749d4:	lea    0xb2cc5(%rip),%rcx        # 1276a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ab8>
   749db:	mov    $0x400,%edx
   749e0:	xor    %edi,%edi
   749e2:	mov    %r14,%rsi
   749e5:	call   *0xb48c5(%rip)        # 1292b0 <_DYNAMIC+0x370>
   749eb:	lea    -0x4(%rbp),%rdi
   749ef:	lea    0xaf37a(%rip),%rcx        # 123d70 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   749f6:	mov    %rbp,%rsi
   749f9:	mov    %rbp,%rdx
   749fc:	call   *0xb48ae(%rip)        # 1292b0 <_DYNAMIC+0x370>
   74a02:	lea    0xaf43f(%rip),%rcx        # 123e48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   74a09:	mov    0x70(%rsp),%rdi
   74a0e:	mov    %rbx,%rsi
   74a11:	mov    %rbx,%rdx
   74a14:	call   *0xb4896(%rip)        # 1292b0 <_DYNAMIC+0x370>
   74a1a:	lea    0xaf76f(%rip),%rcx        # 124190 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25a8>
   74a21:	mov    %rdx,%rdi
   74a24:	mov    %rbp,%rsi
   74a27:	mov    %rbp,%rdx
   74a2a:	call   *0xb4880(%rip)        # 1292b0 <_DYNAMIC+0x370>
   74a30:	lea    0xaf369(%rip),%rdx        # 123da0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   74a37:	xor    %edi,%edi
   74a39:	xor    %esi,%esi
   74a3b:	call   *0xb474f(%rip)        # 129190 <_DYNAMIC+0x250>
   74a41:	lea    0xaf3b8(%rip),%rcx        # 123e00 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   74a48:	xor    %edi,%edi
   74a4a:	mov    %r13,%rsi
   74a4d:	mov    0x38(%rsp),%rdx
   74a52:	call   *0xb4858(%rip)        # 1292b0 <_DYNAMIC+0x370>
   74a58:	lea    0xaf389(%rip),%rcx        # 123de8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   74a5f:	xor    %edi,%edi
   74a61:	mov    %r13,%rsi
   74a64:	mov    0x38(%rsp),%rdx
   74a69:	call   *0xb4841(%rip)        # 1292b0 <_DYNAMIC+0x370>
