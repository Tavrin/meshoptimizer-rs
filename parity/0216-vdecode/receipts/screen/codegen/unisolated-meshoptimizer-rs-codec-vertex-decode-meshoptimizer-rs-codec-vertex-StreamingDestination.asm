
<artifacts>/screen/driver-unisolated:     file format elf64-x86-64


Disassembly of section .text:

0000000000073e40 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73e40:	push   %rbp
   73e41:	push   %r15
   73e43:	push   %r14
   73e45:	push   %r13
   73e47:	push   %r12
   73e49:	push   %rbx
   73e4a:	sub    $0x598,%rsp
   73e51:	mov    %rcx,%rbx
   73e54:	mov    %rdx,%r14
   73e57:	mov    %rsi,%r15
   73e5a:	mov    %rdi,%r12
   73e5d:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73e62:	cmp    $0x2,%al
   73e64:	jae    73fbb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x17b>
   73e6a:	mov    $0x7,%al
   73e6c:	test   %rbx,%rbx
   73e6f:	je     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73e75:	movzbl (%r14),%ebp
   73e79:	mov    %ebp,%ecx
   73e7b:	and    $0xf0,%cl
   73e7e:	cmp    $0xa0,%cl
   73e81:	jne    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73e87:	and    $0xf,%bpl
   73e8b:	mov    $0x5,%al
   73e8d:	cmp    $0x2,%bpl
   73e91:	jae    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73e97:	xor    %eax,%eax
   73e99:	test   %bpl,%bpl
   73e9c:	sete   %al
   73e9f:	lea    0x18(,%rax,8),%rcx
   73ea7:	mov    $0x7,%al
   73ea9:	cmp    %rcx,%rbx
   73eac:	jbe    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73eb2:	cmp    $0x1,%bpl
   73eb6:	mov    %rbx,%rax
   73eb9:	adc    $0x0,%rax
   73ebd:	lea    -0x1(%rax),%rdx
   73ec1:	mov    %rdx,0x70(%rsp)
   73ec6:	cmp    %rbx,%rdx
   73ec9:	ja     74ad2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc92>
   73ecf:	mov    %rcx,0x88(%rsp)
   73ed7:	add    $0xfffffffffffffffb,%rax
   73edb:	pxor   %xmm0,%xmm0
   73edf:	movdqu %xmm0,0x188(%rsp)
   73ee8:	movdqu %xmm0,0x17c(%rsp)
   73ef1:	movdqu %xmm0,0x16c(%rsp)
   73efa:	movdqu %xmm0,0x15c(%rsp)
   73f03:	movdqu %xmm0,0x14c(%rsp)
   73f0c:	movdqu %xmm0,0x13c(%rsp)
   73f15:	movdqu %xmm0,0x12c(%rsp)
   73f1e:	movdqu %xmm0,0x11c(%rsp)
   73f27:	movdqu %xmm0,0x10c(%rsp)
   73f30:	movdqu %xmm0,0xfc(%rsp)
   73f39:	movdqu %xmm0,0xec(%rsp)
   73f42:	movdqu %xmm0,0xdc(%rsp)
   73f4b:	movdqu %xmm0,0xcc(%rsp)
   73f54:	movdqu %xmm0,0xbc(%rsp)
   73f5d:	movdqu %xmm0,0xac(%rsp)
   73f66:	movdqu %xmm0,0x9c(%rsp)
   73f6f:	mov    (%r14,%rax,1),%r13d
   73f73:	mov    %r13d,0x98(%rsp)
   73f7b:	lea    0x198(%rsp),%rdi
   73f83:	mov    $0x400,%edx
   73f88:	xor    %esi,%esi
   73f8a:	call   *0xb6858(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   73f90:	test   %r15,%r15
   73f93:	je     73ff6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1b6>
   73f95:	mov    %r13d,0x54(%rsp)
   73f9a:	mov    $0x1,%r8d
   73fa0:	xor    %esi,%esi
   73fa2:	mov    %r14,0x48(%rsp)
   73fa7:	mov    %rbx,0x68(%rsp)
   73fac:	mov    %r15,0x78(%rsp)
   73fb1:	mov    %bpl,0x1f(%rsp)
   73fb6:	jmp    74076 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x236>
   73fbb:	cmp    $0x21,%r15
   73fbf:	lea    0xecaa(%rip),%rax        # 82c70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73fc6:	lea    0x7aa3(%rip),%r9        # 7ba70 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73fcd:	cmovb  %rax,%r9
   73fd1:	mov    $0x4,%edx
   73fd6:	mov    %r12,%rdi
   73fd9:	mov    %r15,%rsi
   73fdc:	mov    %r14,%rcx
   73fdf:	mov    %rbx,%r8
   73fe2:	add    $0x598,%rsp
   73fe9:	pop    %rbx
   73fea:	pop    %r12
   73fec:	pop    %r13
   73fee:	pop    %r14
   73ff0:	pop    %r15
   73ff2:	pop    %rbp
   73ff3:	jmp    *%r9
   73ff6:	mov    $0x1,%r8d
   73ffc:	sub    %r8,%rbx
   73fff:	setae  %cl
   74002:	cmp    0x88(%rsp),%rbx
   7400a:	sete   %dl
   7400d:	mov    $0x7,%al
   7400f:	test   %dl,%cl
   74011:	je     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74017:	mov    $0xff,%al
   74019:	jmp    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7401e:	mov    $0x1,%ecx
   74023:	mov    $0x1,%r8d
   74029:	mov    %r13,%rdi
   7402c:	mov    %rbp,%rsi
   7402f:	mov    %r14,%rdx
   74032:	call   86230 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   74037:	mov    0x10(%r13),%rbp
   7403b:	mov    0x8(%r13),%rdi
   7403f:	add    %rbp,%rdi
   74042:	lea    0x8(%r12),%rsi
   74047:	mov    %r14,%rdx
   7404a:	call   *0xb6788(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   74050:	add    %r14,%rbp
   74053:	mov    %rbp,0x10(%r13)
   74057:	mov    0x80(%rsp),%rax
   7405f:	mov    %rax,%rsi
   74062:	cmp    %r15,%rax
   74065:	mov    0x48(%rsp),%r14
   7406a:	movzbl 0x1f(%rsp),%ebp
   7406f:	mov    0x40(%rsp),%r8
   74074:	jae    73ffc <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1bc>
   74076:	sub    %rsi,%r15
   74079:	cmp    $0x100,%r15
   74080:	mov    $0x100,%r13d
   74086:	cmovb  %r15,%r13
   7408a:	lea    0xf(%r13),%r9d
   7408e:	and    $0x3f0,%r9d
   74095:	test   %bpl,%bpl
   74098:	mov    %rsi,0x90(%rsp)
   740a0:	mov    %r15,0x58(%rsp)
   740a5:	mov    %r9,0x38(%rsp)
   740aa:	je     74140 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x300>
   740b0:	mov    $0x7,%al
   740b2:	cmp    %rbx,%r8
   740b5:	jae    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   740bb:	lea    (%rsi,%r13,1),%rdx
   740bf:	lea    0x0(,%r13,4),%rbp
   740c7:	mov    %rbp,0x408(%r12)
   740cf:	movzbl (%r14,%r8,1),%edi
   740d4:	inc    %r8
   740d7:	mov    %edi,%ecx
   740d9:	and    $0x3,%ecx
   740dc:	lea    0x0(,%r13,2),%r10
   740e4:	add    %r13,%r10
   740e7:	mov    %r10,0x60(%rsp)
   740ec:	cmp    %r9,%r13
   740ef:	mov    %rdx,0x80(%rsp)
   740f7:	mov    %edi,0x40(%rsp)
   740fb:	ja     74846 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa06>
   74101:	cmp    $0x2,%cl
   74104:	je     74182 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x342>
   74106:	movzbl %cl,%edx
   74109:	cmp    $0x3,%edx
   7410c:	jne    7419a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35a>
   74112:	lea    (%r8,%r13,1),%r15
   74116:	cmp    %rbx,%r15
   74119:	ja     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7411f:	mov    0x48(%rsp),%r14
   74124:	add    %r14,%r8
   74127:	lea    0x198(%rsp),%rdi
   7412f:	mov    %r8,%rsi
   74132:	mov    %r13,%rdx
   74135:	call   *0xb669d(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   7413b:	jmp    741dd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   74140:	lea    (%rsi,%r13,1),%rax
   74144:	mov    %rax,0x80(%rsp)
   7414c:	lea    0x0(,%r13,4),%rbp
   74154:	mov    %rbp,0x408(%r12)
   7415c:	movl   $0x0,0x40(%rsp)
   74164:	lea    0x0(%r13,%r13,2),%rax
   74169:	mov    %rax,0x60(%rsp)
   7416e:	lea    -0x66845(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74175:	mov    %rcx,%rax
   74178:	cmp    %r9,%r13
   7417b:	jbe    741ae <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x36e>
   7417d:	jmp    74871 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa31>
   74182:	lea    0x198(%rsp),%rdi
   7418a:	xor    %esi,%esi
   7418c:	mov    %r13,%rdx
   7418f:	mov    %r8,%r15
   74192:	call   *0xb6650(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   74198:	jmp    741dd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   7419a:	test   %cl,%cl
   7419c:	lea    -0x66573(%rip),%rcx        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   741a3:	lea    -0x6645a(%rip),%rax        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   741aa:	cmove  %rax,%rcx
   741ae:	mov    %rcx,(%rsp)
   741b2:	lea    0x20(%rsp),%rdi
   741b7:	mov    %r14,%rsi
   741ba:	mov    %rbx,%rdx
   741bd:	mov    %r8,%rcx
   741c0:	lea    0x198(%rsp),%r8
   741c8:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   741cd:	cmpb   $0x0,0x20(%rsp)
   741d2:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   741d8:	mov    0x28(%rsp),%r15
   741dd:	mov    0x40(%rsp),%eax
   741e1:	shr    $0x2,%al
   741e4:	and    $0x3,%al
   741e6:	lea    (%rsp,%r13,1),%r8
   741ea:	add    $0x198,%r8
   741f1:	cmp    $0x3,%al
   741f3:	je     74210 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3d0>
   741f5:	movzbl %al,%ecx
   741f8:	cmp    $0x2,%ecx
   741fb:	jne    7424b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x40b>
   741fd:	mov    %r8,%rdi
   74200:	xor    %esi,%esi
   74202:	mov    %r13,%rdx
   74205:	call   *0xb65dd(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   7420b:	jmp    7429b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   74210:	mov    %r15,%r14
   74213:	mov    $0x7,%al
   74215:	add    %r13,%r14
   74218:	jb     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7421e:	cmp    %rbx,%r14
   74221:	ja     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74227:	mov    0x48(%rsp),%rbx
   7422c:	add    %rbx,%r15
   7422f:	mov    %r8,%rdi
   74232:	mov    %r15,%rsi
   74235:	mov    %r13,%rdx
   74238:	call   *0xb659a(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   7423e:	mov    %r14,%r15
   74241:	mov    %rbx,%r14
   74244:	mov    0x68(%rsp),%rbx
   74249:	jmp    7429b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   7424b:	test   %al,%al
   7424d:	lea    -0x66624(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74254:	lea    -0x6650b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7425b:	cmove  %rcx,%rax
   7425f:	cmpb   $0x0,0x1f(%rsp)
   74264:	lea    -0x6693b(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7426b:	cmove  %rcx,%rax
   7426f:	mov    %rax,(%rsp)
   74273:	lea    0x20(%rsp),%rdi
   74278:	mov    %r14,%rsi
   7427b:	mov    %rbx,%rdx
   7427e:	mov    %r15,%rcx
   74281:	mov    0x38(%rsp),%r9
   74286:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   7428b:	cmpb   $0x0,0x20(%rsp)
   74290:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74296:	mov    0x28(%rsp),%r15
   7429b:	mov    0x40(%rsp),%eax
   7429f:	shr    $0x4,%al
   742a2:	and    $0x3,%al
   742a4:	lea    (%rsp,%r13,2),%r8
   742a8:	add    $0x198,%r8
   742af:	cmp    $0x3,%al
   742b1:	je     742ce <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x48e>
   742b3:	movzbl %al,%ecx
   742b6:	cmp    $0x2,%ecx
   742b9:	jne    74309 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4c9>
   742bb:	mov    %r8,%rdi
   742be:	xor    %esi,%esi
   742c0:	mov    %r13,%rdx
   742c3:	call   *0xb651f(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   742c9:	jmp    74359 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   742ce:	mov    %r15,%r14
   742d1:	mov    $0x7,%al
   742d3:	add    %r13,%r14
   742d6:	jb     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   742dc:	cmp    %rbx,%r14
   742df:	ja     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   742e5:	mov    0x48(%rsp),%rbx
   742ea:	add    %rbx,%r15
   742ed:	mov    %r8,%rdi
   742f0:	mov    %r15,%rsi
   742f3:	mov    %r13,%rdx
   742f6:	call   *0xb64dc(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   742fc:	mov    %r14,%r15
   742ff:	mov    %rbx,%r14
   74302:	mov    0x68(%rsp),%rbx
   74307:	jmp    74359 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   74309:	test   %al,%al
   7430b:	lea    -0x666e2(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74312:	lea    -0x665c9(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74319:	cmove  %rcx,%rax
   7431d:	cmpb   $0x0,0x1f(%rsp)
   74322:	lea    -0x669f9(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74329:	cmove  %rcx,%rax
   7432d:	mov    %rax,(%rsp)
   74331:	lea    0x20(%rsp),%rdi
   74336:	mov    %r14,%rsi
   74339:	mov    %rbx,%rdx
   7433c:	mov    %r15,%rcx
   7433f:	mov    0x38(%rsp),%r9
   74344:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   74349:	cmpb   $0x0,0x20(%rsp)
   7434e:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74354:	mov    0x28(%rsp),%r15
   74359:	mov    0x40(%rsp),%ecx
   7435d:	shr    $0x6,%cl
   74360:	mov    0x60(%rsp),%rax
   74365:	lea    (%rsp,%rax,1),%r8
   74369:	add    $0x198,%r8
   74370:	cmp    $0x3,%cl
   74373:	je     7438d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x54d>
   74375:	movzbl %cl,%eax
   74378:	cmp    $0x2,%eax
   7437b:	jne    743d7 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x597>
   7437d:	mov    %r8,%rdi
   74380:	xor    %esi,%esi
   74382:	mov    %r13,%rdx
   74385:	call   *0xb645d(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   7438b:	jmp    743be <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x57e>
   7438d:	mov    %r15,%rcx
   74390:	mov    $0x7,%al
   74392:	add    %r13,%rcx
   74395:	jb     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7439b:	cmp    %rbx,%rcx
   7439e:	ja     749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   743a4:	mov    0x48(%rsp),%r14
   743a9:	add    %r14,%r15
   743ac:	mov    %r8,%rdi
   743af:	mov    %r15,%rsi
   743b2:	mov    %r13,%rdx
   743b5:	mov    %rcx,%r15
   743b8:	call   *0xb641a(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   743be:	mov    %r15,%rsi
   743c1:	cmpb   $0x0,0x1f(%rsp)
   743c6:	mov    0x78(%rsp),%r15
   743cb:	mov    %rsi,0x40(%rsp)
   743d0:	jne    74438 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5f8>
   743d2:	jmp    74460 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   743d7:	test   %cl,%cl
   743d9:	lea    -0x667b0(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   743e0:	lea    -0x66697(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   743e7:	cmove  %rcx,%rax
   743eb:	cmpb   $0x0,0x1f(%rsp)
   743f0:	lea    -0x66ac7(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   743f7:	cmove  %rcx,%rax
   743fb:	mov    %rax,(%rsp)
   743ff:	lea    0x20(%rsp),%rdi
   74404:	mov    %r14,%rsi
   74407:	mov    %rbx,%rdx
   7440a:	mov    %r15,%rcx
   7440d:	mov    0x38(%rsp),%r9
   74412:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   74417:	cmpb   $0x0,0x20(%rsp)
   7441c:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74422:	mov    0x28(%rsp),%rsi
   74427:	cmpb   $0x0,0x1f(%rsp)
   7442c:	mov    0x78(%rsp),%r15
   74431:	mov    %rsi,0x40(%rsp)
   74436:	je     74460 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   74438:	mov    0x70(%rsp),%rax
   7443d:	cmp    %rax,%rbx
   74440:	je     74b00 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcc0>
   74446:	movzbl (%r14,%rax,1),%eax
   7444b:	mov    %eax,%ecx
   7444d:	and    $0x3,%ecx
   74450:	lea    -0x63c9f(%rip),%rdx        # 107b8 <libm::math::generic::sqrt::RSQRT_TAB+0x3d2>
   74457:	movslq (%rdx,%rcx,4),%rcx
   7445b:	add    %rdx,%rcx
   7445e:	jmp    *%rcx
   74460:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   74465:	test   %al,%al
   74467:	je     74739 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8f9>
   7446d:	movd   0x54(%rsp),%xmm0
   74473:	pshufd $0x0,%xmm0,%xmm0
   74478:	mov    %r13,%rcx
   7447b:	shr    $0x4,%rcx
   7447f:	je     746de <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x89e>
   74485:	lea    0x0(,%r13,2),%r10
   7448d:	lea    0x0(,%r13,2),%rsi
   74495:	add    %r13,%rsi
   74498:	mov    %r13,%r8
   7449b:	neg    %r8
   7449e:	neg    %r10
   744a1:	mov    %rsi,%r14
   744a4:	neg    %r14
   744a7:	lea    (%rsp,%rsi,1),%r15
   744ab:	add    $0x198,%r15
   744b2:	shl    $0x4,%rcx
   744b6:	xor    %eax,%eax
   744b8:	mov    %rbp,%r9
   744bb:	xor    %edi,%edi
   744bd:	movdqa -0x66b66(%rip),%xmm8        # d960 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   744c6:	movdqa -0x66cff(%rip),%xmm9        # d7d0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x901>
   744cf:	nop
   744d0:	cmp    %rbp,%rdi
   744d3:	ja     74a91 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc51>
   744d9:	mov    %r9,%r11
   744dc:	sub    $0x10,%r11
   744e0:	jb     74a08 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   744e6:	lea    (%rdi,%r13,1),%rdx
   744ea:	cmp    %rbp,%rdx
   744ed:	ja     74a30 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbf0>
   744f3:	lea    (%r8,%r9,1),%rdx
   744f7:	cmp    $0x10,%rdx
   744fb:	jb     74a08 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74501:	lea    0x0(,%r13,2),%rdx
   74509:	add    %rdi,%rdx
   7450c:	cmp    %rbp,%rdx
   7450f:	ja     74a47 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc07>
   74515:	lea    (%r10,%r9,1),%rdx
   74519:	cmp    $0x10,%rdx
   7451d:	jb     74a08 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74523:	lea    (%rsi,%rdi,1),%rdx
   74527:	cmp    %rbp,%rdx
   7452a:	ja     74a75 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc35>
   74530:	add    %r14,%r9
   74533:	cmp    $0x10,%r9
   74537:	jb     74a08 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   7453d:	lea    0x3f(%rax),%rdx
   74541:	cmp    %rbp,%rdx
   74544:	jae    74a19 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd9>
   7454a:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   74553:	lea    (%rsp,%r13,1),%rdx
   74557:	add    $0x198,%rdx
   7455e:	movdqu (%rdx,%rdi,1),%xmm4
   74563:	lea    0x198(%rsp,%r13,2),%rdx
   7456b:	movdqu (%rdx,%rdi,1),%xmm5
   74570:	movdqa %xmm3,%xmm2
   74574:	punpckhbw %xmm4,%xmm2
   74578:	movdqu (%r15,%rdi,1),%xmm6
   7457e:	movdqa %xmm5,%xmm7
   74582:	punpckhbw %xmm6,%xmm7
   74586:	movdqa %xmm2,%xmm1
   7458a:	punpckhwd %xmm7,%xmm1
   7458e:	punpcklwd %xmm7,%xmm2
   74592:	punpcklbw %xmm4,%xmm3
   74596:	punpcklbw %xmm6,%xmm5
   7459a:	movdqa %xmm3,%xmm4
   7459e:	punpckhwd %xmm5,%xmm4
   745a2:	punpcklwd %xmm5,%xmm3
   745a6:	movdqa %xmm3,%xmm5
   745aa:	psrlw  $0x1,%xmm5
   745af:	pand   %xmm8,%xmm3
   745b4:	pxor   %xmm6,%xmm6
   745b8:	psubb  %xmm3,%xmm6
   745bc:	pand   %xmm9,%xmm5
   745c1:	pxor   %xmm6,%xmm5
   745c5:	movdqa %xmm5,%xmm3
   745c9:	pslldq $0x4,%xmm3
   745ce:	paddb  %xmm5,%xmm3
   745d2:	movdqa %xmm3,%xmm5
   745d6:	pslldq $0x8,%xmm5
   745db:	paddb  %xmm0,%xmm3
   745df:	paddb  %xmm5,%xmm3
   745e3:	movdqu %xmm3,0x8(%r12,%rdi,4)
   745ea:	movdqa %xmm4,%xmm0
   745ee:	psrlw  $0x1,%xmm0
   745f3:	pand   %xmm8,%xmm4
   745f8:	pxor   %xmm5,%xmm5
   745fc:	psubb  %xmm4,%xmm5
   74600:	pand   %xmm9,%xmm0
   74605:	pxor   %xmm5,%xmm0
   74609:	movdqa %xmm0,%xmm4
   7460d:	pslldq $0x4,%xmm4
   74612:	paddb  %xmm0,%xmm4
   74616:	movdqa %xmm4,%xmm0
   7461a:	pslldq $0x8,%xmm0
   7461f:	pshufd $0xff,%xmm3,%xmm3
   74624:	paddb  %xmm4,%xmm0
   74628:	paddb  %xmm3,%xmm0
   7462c:	movdqu %xmm0,0x18(%r12,%rdi,4)
   74633:	movdqa %xmm2,%xmm3
   74637:	psrlw  $0x1,%xmm3
   7463c:	pand   %xmm8,%xmm2
   74641:	pxor   %xmm4,%xmm4
   74645:	psubb  %xmm2,%xmm4
   74649:	pand   %xmm9,%xmm3
   7464e:	pxor   %xmm4,%xmm3
   74652:	movdqa %xmm3,%xmm2
   74656:	pslldq $0x4,%xmm2
   7465b:	paddb  %xmm3,%xmm2
   7465f:	movdqa %xmm2,%xmm3
   74663:	pslldq $0x8,%xmm3
   74668:	pshufd $0xff,%xmm0,%xmm0
   7466d:	paddb  %xmm2,%xmm3
   74671:	paddb  %xmm0,%xmm3
   74675:	movdqu %xmm3,0x28(%r12,%rdi,4)
   7467c:	movdqa %xmm1,%xmm0
   74680:	psrlw  $0x1,%xmm0
   74685:	pand   %xmm8,%xmm1
   7468a:	pxor   %xmm2,%xmm2
   7468e:	psubb  %xmm1,%xmm2
   74692:	pand   %xmm9,%xmm0
   74697:	pxor   %xmm2,%xmm0
   7469b:	movdqa %xmm0,%xmm1
   7469f:	pslldq $0x4,%xmm1
   746a4:	paddb  %xmm0,%xmm1
   746a8:	movdqa %xmm1,%xmm0
   746ac:	pslldq $0x8,%xmm0
   746b1:	pshufd $0xff,%xmm3,%xmm2
   746b6:	paddb  %xmm1,%xmm0
   746ba:	paddb  %xmm2,%xmm0
   746be:	movdqu %xmm0,0x38(%r12,%rdi,4)
   746c5:	add    $0x10,%rdi
   746c9:	pshufd $0xff,%xmm0,%xmm0
   746ce:	add    $0x40,%rax
   746d2:	mov    %r11,%r9
   746d5:	cmp    %rdi,%rcx
   746d8:	jne    744d0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x690>
   746de:	mov    %r13d,%eax
   746e1:	and    $0x1f0,%eax
   746e6:	cmp    %r13,%rax
   746e9:	mov    0x78(%rsp),%r15
   746ee:	je     747ed <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   746f4:	movdqa %xmm0,0x20(%rsp)
   746fa:	lea    0x0(,%rax,4),%edx
   74701:	mov    %rbp,%rcx
   74704:	sub    %rdx,%rcx
   74707:	jb     74aea <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcaa>
   7470d:	lea    0x8(%r12),%rsi
   74712:	add    %rsi,%rdx
   74715:	mov    %rax,0x8(%rsp)
   7471a:	lea    0x20(%rsp),%rax
   7471f:	mov    %rax,(%rsp)
   74723:	mov    $0x4,%r9d
   74729:	lea    0x198(%rsp),%rdi
   74731:	mov    %r13,%r8
   74734:	mov    %rbp,%rsi
   74737:	jmp    7476a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x92a>
   74739:	lea    0x98(%rsp),%rax
   74741:	mov    %rax,(%rsp)
   74745:	movq   $0x0,0x8(%rsp)
   7474e:	mov    $0x4,%r9d
   74754:	lea    0x198(%rsp),%rdi
   7475c:	mov    %rbp,%rsi
   7475f:	lea    0x8(%r12),%rdx
   74764:	mov    %r13,%r8
   74767:	mov    %rbp,%rcx
   7476a:	call   72c50 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7476f:	jmp    747ed <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   74771:	shr    $0x4,%al
   74774:	neg    %al
   74776:	and    $0x1f,%al
   74778:	movzbl %al,%eax
   7477b:	lea    0x98(%rsp),%rcx
   74783:	mov    %rcx,(%rsp)
   74787:	mov    %eax,0x8(%rsp)
   7478b:	movq   $0x0,0x10(%rsp)
   74794:	mov    $0x4,%r9d
   7479a:	lea    0x198(%rsp),%rdi
   747a2:	mov    %rbp,%rsi
   747a5:	lea    0x8(%r12),%rdx
   747aa:	mov    %r13,%r8
   747ad:	mov    %rbp,%rcx
   747b0:	call   73000 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   747b5:	jmp    747ed <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   747b7:	lea    0x98(%rsp),%rax
   747bf:	mov    %rax,(%rsp)
   747c3:	movq   $0x0,0x8(%rsp)
   747cc:	mov    $0x4,%r9d
   747d2:	lea    0x198(%rsp),%rdi
   747da:	mov    %rbp,%rsi
   747dd:	lea    0x8(%r12),%rdx
   747e2:	mov    %r13,%r8
   747e5:	mov    %rbp,%rcx
   747e8:	call   72e40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   747ed:	cmp    0x90(%rsp),%r15
   747f5:	je     74abb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc7b>
   747fb:	mov    0x4(%r12,%rbp,1),%eax
   74800:	mov    %eax,0x98(%rsp)
   74807:	mov    0x408(%r12),%r14
   7480f:	cmp    $0x401,%r14
   74816:	jae    74aa4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc64>
   7481c:	mov    %eax,0x54(%rsp)
   74820:	mov    (%r12),%r13
   74824:	mov    0x0(%r13),%rax
   74828:	mov    0x10(%r13),%rbp
   7482c:	sub    %rbp,%rax
   7482f:	cmp    %rax,%r14
   74832:	ja     7401e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1de>
   74838:	test   %r14,%r14
   7483b:	jne    7403b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fb>
   74841:	jmp    74050 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x210>
   74846:	cmp    $0x2,%ecx
   74849:	je     74b11 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7484f:	cmp    $0x3,%ecx
   74852:	je     74b28 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   74858:	test   %cl,%cl
   7485a:	lea    -0x66c31(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74861:	lea    -0x66b18(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74868:	cmove  %rcx,%rax
   7486c:	mov    0x38(%rsp),%r9
   74871:	mov    %rax,(%rsp)
   74875:	lea    0x20(%rsp),%rdi
   7487a:	mov    %r14,%rsi
   7487d:	mov    %rbx,%rdx
   74880:	mov    %r8,%rcx
   74883:	lea    0x198(%rsp),%r8
   7488b:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   74890:	cmpb   $0x0,0x20(%rsp)
   74895:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7489b:	mov    0x40(%rsp),%eax
   7489f:	shr    $0x2,%al
   748a2:	and    $0x3,%al
   748a4:	cmp    $0x2,%al
   748a6:	je     74b11 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   748ac:	movzbl %al,%ecx
   748af:	cmp    $0x3,%ecx
   748b2:	je     74b28 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   748b8:	mov    0x28(%rsp),%rcx
   748bd:	test   %al,%al
   748bf:	lea    -0x66c96(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   748c6:	lea    -0x66b7d(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   748cd:	cmove  %rdx,%rax
   748d1:	cmpb   $0x0,0x1f(%rsp)
   748d6:	lea    0x198(%rsp,%r13,1),%r8
   748de:	lea    -0x66fb5(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   748e5:	cmove  %rdx,%rax
   748e9:	mov    %rax,(%rsp)
   748ed:	lea    0x20(%rsp),%rdi
   748f2:	mov    %r14,%rsi
   748f5:	mov    %rbx,%rdx
   748f8:	mov    0x38(%rsp),%r9
   748fd:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   74902:	cmpb   $0x0,0x20(%rsp)
   74907:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7490d:	mov    0x40(%rsp),%eax
   74911:	shr    $0x4,%al
   74914:	and    $0x3,%al
   74916:	cmp    $0x2,%al
   74918:	je     74b11 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7491e:	movzbl %al,%ecx
   74921:	cmp    $0x3,%ecx
   74924:	je     74b28 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   7492a:	mov    0x28(%rsp),%rcx
   7492f:	test   %al,%al
   74931:	lea    -0x66d08(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74938:	lea    -0x66bef(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7493f:	cmove  %rdx,%rax
   74943:	cmpb   $0x0,0x1f(%rsp)
   74948:	lea    0x198(%rsp,%r13,2),%r8
   74950:	lea    -0x67027(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74957:	cmove  %rdx,%rax
   7495b:	mov    %rax,(%rsp)
   7495f:	lea    0x20(%rsp),%rdi
   74964:	mov    %r14,%rsi
   74967:	mov    %rbx,%rdx
   7496a:	mov    0x38(%rsp),%r9
   7496f:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   74974:	cmpb   $0x0,0x20(%rsp)
   74979:	jne    749f1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7497b:	mov    0x40(%rsp),%edx
   7497f:	shr    $0x6,%dl
   74982:	cmp    $0x2,%dl
   74985:	je     74b11 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7498b:	movzbl %dl,%eax
   7498e:	cmp    $0x3,%eax
   74991:	je     74b28 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   74997:	mov    0x28(%rsp),%rcx
   7499c:	test   %dl,%dl
   7499e:	lea    -0x66d75(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   749a5:	lea    -0x66c5c(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   749ac:	cmove  %rdx,%rax
   749b0:	cmpb   $0x0,0x1f(%rsp)
   749b5:	mov    0x60(%rsp),%rdx
   749ba:	lea    0x198(%rsp,%rdx,1),%r8
   749c2:	lea    -0x67099(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   749c9:	cmove  %rdx,%rax
   749cd:	mov    %rax,(%rsp)
   749d1:	lea    0x20(%rsp),%rdi
   749d6:	mov    %r14,%rsi
   749d9:	mov    %rbx,%rdx
   749dc:	mov    0x38(%rsp),%r9
   749e1:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   749e6:	cmpb   $0x0,0x20(%rsp)
   749eb:	je     74422 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e2>
   749f1:	movzbl 0x21(%rsp),%eax
   749f6:	add    $0x598,%rsp
   749fd:	pop    %rbx
   749fe:	pop    %r12
   74a00:	pop    %r13
   74a02:	pop    %r14
   74a04:	pop    %r15
   74a06:	pop    %rbp
   74a07:	ret
   74a08:	lea    0xb1611(%rip),%rdi        # 126020 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2db8>
   74a0f:	call   *0xb5f73(%rip)        # 12a988 <_DYNAMIC+0x3c8>
   74a15:	mov    $0x7,%al
   74a17:	jmp    749f6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74a19:	lea    0x40(%rax),%rsi
   74a1d:	lea    0xb0e04(%rip),%rcx        # 125828 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25c0>
   74a24:	mov    %rax,%rdi
   74a27:	mov    %rbp,%rdx
   74a2a:	call   *0xb5f00(%rip)        # 12a930 <_DYNAMIC+0x370>
   74a30:	mov    0x58(%rsp),%rcx
   74a35:	cmp    $0x100,%rcx
   74a3c:	mov    $0x100,%eax
   74a41:	cmovb  %rcx,%rax
   74a45:	jmp    74a8e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4e>
   74a47:	mov    0x58(%rsp),%rcx
   74a4c:	cmp    $0x100,%rcx
   74a53:	mov    $0x100,%eax
   74a58:	cmovb  %rcx,%rax
   74a5c:	mov    %eax,%eax
   74a5e:	lea    (%rdi,%rax,2),%rdi
   74a62:	lea    0xb159f(%rip),%rcx        # 126008 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74a69:	mov    %rbp,%rsi
   74a6c:	mov    %rbp,%rdx
   74a6f:	call   *0xb5ebb(%rip)        # 12a930 <_DYNAMIC+0x370>
   74a75:	mov    0x58(%rsp),%rcx
   74a7a:	cmp    $0x100,%rcx
   74a81:	mov    $0x100,%eax
   74a86:	cmovb  %rcx,%rax
   74a8a:	lea    (%rax,%rax,2),%rax
   74a8e:	add    %rax,%rdi
   74a91:	lea    0xb1570(%rip),%rcx        # 126008 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74a98:	mov    %rbp,%rsi
   74a9b:	mov    %rbp,%rdx
   74a9e:	call   *0xb5e8c(%rip)        # 12a930 <_DYNAMIC+0x370>
   74aa4:	lea    0xb4275(%rip),%rcx        # 128d20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ab8>
   74aab:	mov    $0x400,%edx
   74ab0:	xor    %edi,%edi
   74ab2:	mov    %r14,%rsi
   74ab5:	call   *0xb5e75(%rip)        # 12a930 <_DYNAMIC+0x370>
   74abb:	lea    -0x4(%rbp),%rdi
   74abf:	lea    0xb092a(%rip),%rcx        # 1253f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   74ac6:	mov    %rbp,%rsi
   74ac9:	mov    %rbp,%rdx
   74acc:	call   *0xb5e5e(%rip)        # 12a930 <_DYNAMIC+0x370>
   74ad2:	lea    0xb09ef(%rip),%rcx        # 1254c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   74ad9:	mov    0x70(%rsp),%rdi
   74ade:	mov    %rbx,%rsi
   74ae1:	mov    %rbx,%rdx
   74ae4:	call   *0xb5e46(%rip)        # 12a930 <_DYNAMIC+0x370>
   74aea:	lea    0xb0d1f(%rip),%rcx        # 125810 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25a8>
   74af1:	mov    %rdx,%rdi
   74af4:	mov    %rbp,%rsi
   74af7:	mov    %rbp,%rdx
   74afa:	call   *0xb5e30(%rip)        # 12a930 <_DYNAMIC+0x370>
   74b00:	lea    0xb0919(%rip),%rdx        # 125420 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   74b07:	xor    %edi,%edi
   74b09:	xor    %esi,%esi
   74b0b:	call   *0xb5cff(%rip)        # 12a810 <_DYNAMIC+0x250>
   74b11:	lea    0xb0968(%rip),%rcx        # 125480 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   74b18:	xor    %edi,%edi
   74b1a:	mov    %r13,%rsi
   74b1d:	mov    0x38(%rsp),%rdx
   74b22:	call   *0xb5e08(%rip)        # 12a930 <_DYNAMIC+0x370>
   74b28:	lea    0xb0939(%rip),%rcx        # 125468 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   74b2f:	xor    %edi,%edi
   74b31:	mov    %r13,%rsi
   74b34:	mov    0x38(%rsp),%rdx
   74b39:	call   *0xb5df1(%rip)        # 12a930 <_DYNAMIC+0x370>
