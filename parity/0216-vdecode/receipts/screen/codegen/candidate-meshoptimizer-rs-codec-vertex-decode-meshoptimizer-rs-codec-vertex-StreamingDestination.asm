
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000073eb0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73eb0:	push   %rbp
   73eb1:	push   %r15
   73eb3:	push   %r14
   73eb5:	push   %r13
   73eb7:	push   %r12
   73eb9:	push   %rbx
   73eba:	sub    $0x598,%rsp
   73ec1:	mov    %rcx,%rbx
   73ec4:	mov    %rdx,%r14
   73ec7:	mov    %rsi,%r15
   73eca:	mov    %rdi,%r12
   73ecd:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73ed2:	cmp    $0x2,%al
   73ed4:	jae    7402b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x17b>
   73eda:	mov    $0x7,%al
   73edc:	test   %rbx,%rbx
   73edf:	je     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73ee5:	movzbl (%r14),%ebp
   73ee9:	mov    %ebp,%ecx
   73eeb:	and    $0xf0,%cl
   73eee:	cmp    $0xa0,%cl
   73ef1:	jne    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73ef7:	and    $0xf,%bpl
   73efb:	mov    $0x5,%al
   73efd:	cmp    $0x2,%bpl
   73f01:	jae    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73f07:	xor    %eax,%eax
   73f09:	test   %bpl,%bpl
   73f0c:	sete   %al
   73f0f:	lea    0x18(,%rax,8),%rcx
   73f17:	mov    $0x7,%al
   73f19:	cmp    %rcx,%rbx
   73f1c:	jbe    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   73f22:	cmp    $0x1,%bpl
   73f26:	mov    %rbx,%rax
   73f29:	adc    $0x0,%rax
   73f2d:	lea    -0x1(%rax),%rdx
   73f31:	mov    %rdx,0x70(%rsp)
   73f36:	cmp    %rbx,%rdx
   73f39:	ja     74b42 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc92>
   73f3f:	mov    %rcx,0x88(%rsp)
   73f47:	add    $0xfffffffffffffffb,%rax
   73f4b:	pxor   %xmm0,%xmm0
   73f4f:	movdqu %xmm0,0x188(%rsp)
   73f58:	movdqu %xmm0,0x17c(%rsp)
   73f61:	movdqu %xmm0,0x16c(%rsp)
   73f6a:	movdqu %xmm0,0x15c(%rsp)
   73f73:	movdqu %xmm0,0x14c(%rsp)
   73f7c:	movdqu %xmm0,0x13c(%rsp)
   73f85:	movdqu %xmm0,0x12c(%rsp)
   73f8e:	movdqu %xmm0,0x11c(%rsp)
   73f97:	movdqu %xmm0,0x10c(%rsp)
   73fa0:	movdqu %xmm0,0xfc(%rsp)
   73fa9:	movdqu %xmm0,0xec(%rsp)
   73fb2:	movdqu %xmm0,0xdc(%rsp)
   73fbb:	movdqu %xmm0,0xcc(%rsp)
   73fc4:	movdqu %xmm0,0xbc(%rsp)
   73fcd:	movdqu %xmm0,0xac(%rsp)
   73fd6:	movdqu %xmm0,0x9c(%rsp)
   73fdf:	mov    (%r14,%rax,1),%r13d
   73fe3:	mov    %r13d,0x98(%rsp)
   73feb:	lea    0x198(%rsp),%rdi
   73ff3:	mov    $0x400,%edx
   73ff8:	xor    %esi,%esi
   73ffa:	call   *0xb6798(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   74000:	test   %r15,%r15
   74003:	je     74066 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1b6>
   74005:	mov    %r13d,0x54(%rsp)
   7400a:	mov    $0x1,%r8d
   74010:	xor    %esi,%esi
   74012:	mov    %r14,0x48(%rsp)
   74017:	mov    %rbx,0x68(%rsp)
   7401c:	mov    %r15,0x78(%rsp)
   74021:	mov    %bpl,0x1f(%rsp)
   74026:	jmp    740e6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x236>
   7402b:	cmp    $0x21,%r15
   7402f:	lea    0xf8ba(%rip),%rax        # 838f0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   74036:	lea    0x86b3(%rip),%r9        # 7c6f0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7403d:	cmovb  %rax,%r9
   74041:	mov    $0x4,%edx
   74046:	mov    %r12,%rdi
   74049:	mov    %r15,%rsi
   7404c:	mov    %r14,%rcx
   7404f:	mov    %rbx,%r8
   74052:	add    $0x598,%rsp
   74059:	pop    %rbx
   7405a:	pop    %r12
   7405c:	pop    %r13
   7405e:	pop    %r14
   74060:	pop    %r15
   74062:	pop    %rbp
   74063:	jmp    *%r9
   74066:	mov    $0x1,%r8d
   7406c:	sub    %r8,%rbx
   7406f:	setae  %cl
   74072:	cmp    0x88(%rsp),%rbx
   7407a:	sete   %dl
   7407d:	mov    $0x7,%al
   7407f:	test   %dl,%cl
   74081:	je     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74087:	mov    $0xff,%al
   74089:	jmp    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7408e:	mov    $0x1,%ecx
   74093:	mov    $0x1,%r8d
   74099:	mov    %r13,%rdi
   7409c:	mov    %rbp,%rsi
   7409f:	mov    %r14,%rdx
   740a2:	call   86eb0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   740a7:	mov    0x10(%r13),%rbp
   740ab:	mov    0x8(%r13),%rdi
   740af:	add    %rbp,%rdi
   740b2:	lea    0x8(%r12),%rsi
   740b7:	mov    %r14,%rdx
   740ba:	call   *0xb66c8(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   740c0:	add    %r14,%rbp
   740c3:	mov    %rbp,0x10(%r13)
   740c7:	mov    0x80(%rsp),%rax
   740cf:	mov    %rax,%rsi
   740d2:	cmp    %r15,%rax
   740d5:	mov    0x48(%rsp),%r14
   740da:	movzbl 0x1f(%rsp),%ebp
   740df:	mov    0x40(%rsp),%r8
   740e4:	jae    7406c <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1bc>
   740e6:	sub    %rsi,%r15
   740e9:	cmp    $0x100,%r15
   740f0:	mov    $0x100,%r13d
   740f6:	cmovb  %r15,%r13
   740fa:	lea    0xf(%r13),%r9d
   740fe:	and    $0x3f0,%r9d
   74105:	test   %bpl,%bpl
   74108:	mov    %rsi,0x90(%rsp)
   74110:	mov    %r15,0x58(%rsp)
   74115:	mov    %r9,0x38(%rsp)
   7411a:	je     741b0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x300>
   74120:	mov    $0x7,%al
   74122:	cmp    %rbx,%r8
   74125:	jae    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7412b:	lea    (%rsi,%r13,1),%rdx
   7412f:	lea    0x0(,%r13,4),%rbp
   74137:	mov    %rbp,0x408(%r12)
   7413f:	movzbl (%r14,%r8,1),%edi
   74144:	inc    %r8
   74147:	mov    %edi,%ecx
   74149:	and    $0x3,%ecx
   7414c:	lea    0x0(,%r13,2),%r10
   74154:	add    %r13,%r10
   74157:	mov    %r10,0x60(%rsp)
   7415c:	cmp    %r9,%r13
   7415f:	mov    %rdx,0x80(%rsp)
   74167:	mov    %edi,0x40(%rsp)
   7416b:	ja     748b6 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa06>
   74171:	cmp    $0x2,%cl
   74174:	je     741f2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x342>
   74176:	movzbl %cl,%edx
   74179:	cmp    $0x3,%edx
   7417c:	jne    7420a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35a>
   74182:	lea    (%r8,%r13,1),%r15
   74186:	cmp    %rbx,%r15
   74189:	ja     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7418f:	mov    0x48(%rsp),%r14
   74194:	add    %r14,%r8
   74197:	lea    0x198(%rsp),%rdi
   7419f:	mov    %r8,%rsi
   741a2:	mov    %r13,%rdx
   741a5:	call   *0xb65dd(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   741ab:	jmp    7424d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   741b0:	lea    (%rsi,%r13,1),%rax
   741b4:	mov    %rax,0x80(%rsp)
   741bc:	lea    0x0(,%r13,4),%rbp
   741c4:	mov    %rbp,0x408(%r12)
   741cc:	movl   $0x0,0x40(%rsp)
   741d4:	lea    0x0(%r13,%r13,2),%rax
   741d9:	mov    %rax,0x60(%rsp)
   741de:	lea    -0x668b5(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   741e5:	mov    %rcx,%rax
   741e8:	cmp    %r9,%r13
   741eb:	jbe    7421e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x36e>
   741ed:	jmp    748e1 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa31>
   741f2:	lea    0x198(%rsp),%rdi
   741fa:	xor    %esi,%esi
   741fc:	mov    %r13,%rdx
   741ff:	mov    %r8,%r15
   74202:	call   *0xb6590(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   74208:	jmp    7424d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x39d>
   7420a:	test   %cl,%cl
   7420c:	lea    -0x665e3(%rip),%rcx        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74213:	lea    -0x664ca(%rip),%rax        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7421a:	cmove  %rax,%rcx
   7421e:	mov    %rcx,(%rsp)
   74222:	lea    0x20(%rsp),%rdi
   74227:	mov    %r14,%rsi
   7422a:	mov    %rbx,%rdx
   7422d:	mov    %r8,%rcx
   74230:	lea    0x198(%rsp),%r8
   74238:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   7423d:	cmpb   $0x0,0x20(%rsp)
   74242:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74248:	mov    0x28(%rsp),%r15
   7424d:	mov    0x40(%rsp),%eax
   74251:	shr    $0x2,%al
   74254:	and    $0x3,%al
   74256:	lea    (%rsp,%r13,1),%r8
   7425a:	add    $0x198,%r8
   74261:	cmp    $0x3,%al
   74263:	je     74280 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3d0>
   74265:	movzbl %al,%ecx
   74268:	cmp    $0x2,%ecx
   7426b:	jne    742bb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x40b>
   7426d:	mov    %r8,%rdi
   74270:	xor    %esi,%esi
   74272:	mov    %r13,%rdx
   74275:	call   *0xb651d(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   7427b:	jmp    7430b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   74280:	mov    %r15,%r14
   74283:	mov    $0x7,%al
   74285:	add    %r13,%r14
   74288:	jb     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7428e:	cmp    %rbx,%r14
   74291:	ja     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74297:	mov    0x48(%rsp),%rbx
   7429c:	add    %rbx,%r15
   7429f:	mov    %r8,%rdi
   742a2:	mov    %r15,%rsi
   742a5:	mov    %r13,%rdx
   742a8:	call   *0xb64da(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   742ae:	mov    %r14,%r15
   742b1:	mov    %rbx,%r14
   742b4:	mov    0x68(%rsp),%rbx
   742b9:	jmp    7430b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x45b>
   742bb:	test   %al,%al
   742bd:	lea    -0x66694(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   742c4:	lea    -0x6657b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   742cb:	cmove  %rcx,%rax
   742cf:	cmpb   $0x0,0x1f(%rsp)
   742d4:	lea    -0x669ab(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   742db:	cmove  %rcx,%rax
   742df:	mov    %rax,(%rsp)
   742e3:	lea    0x20(%rsp),%rdi
   742e8:	mov    %r14,%rsi
   742eb:	mov    %rbx,%rdx
   742ee:	mov    %r15,%rcx
   742f1:	mov    0x38(%rsp),%r9
   742f6:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   742fb:	cmpb   $0x0,0x20(%rsp)
   74300:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74306:	mov    0x28(%rsp),%r15
   7430b:	mov    0x40(%rsp),%eax
   7430f:	shr    $0x4,%al
   74312:	and    $0x3,%al
   74314:	lea    (%rsp,%r13,2),%r8
   74318:	add    $0x198,%r8
   7431f:	cmp    $0x3,%al
   74321:	je     7433e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x48e>
   74323:	movzbl %al,%ecx
   74326:	cmp    $0x2,%ecx
   74329:	jne    74379 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4c9>
   7432b:	mov    %r8,%rdi
   7432e:	xor    %esi,%esi
   74330:	mov    %r13,%rdx
   74333:	call   *0xb645f(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   74339:	jmp    743c9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   7433e:	mov    %r15,%r14
   74341:	mov    $0x7,%al
   74343:	add    %r13,%r14
   74346:	jb     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7434c:	cmp    %rbx,%r14
   7434f:	ja     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74355:	mov    0x48(%rsp),%rbx
   7435a:	add    %rbx,%r15
   7435d:	mov    %r8,%rdi
   74360:	mov    %r15,%rsi
   74363:	mov    %r13,%rdx
   74366:	call   *0xb641c(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   7436c:	mov    %r14,%r15
   7436f:	mov    %rbx,%r14
   74372:	mov    0x68(%rsp),%rbx
   74377:	jmp    743c9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x519>
   74379:	test   %al,%al
   7437b:	lea    -0x66752(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74382:	lea    -0x66639(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74389:	cmove  %rcx,%rax
   7438d:	cmpb   $0x0,0x1f(%rsp)
   74392:	lea    -0x66a69(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74399:	cmove  %rcx,%rax
   7439d:	mov    %rax,(%rsp)
   743a1:	lea    0x20(%rsp),%rdi
   743a6:	mov    %r14,%rsi
   743a9:	mov    %rbx,%rdx
   743ac:	mov    %r15,%rcx
   743af:	mov    0x38(%rsp),%r9
   743b4:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   743b9:	cmpb   $0x0,0x20(%rsp)
   743be:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   743c4:	mov    0x28(%rsp),%r15
   743c9:	mov    0x40(%rsp),%ecx
   743cd:	shr    $0x6,%cl
   743d0:	mov    0x60(%rsp),%rax
   743d5:	lea    (%rsp,%rax,1),%r8
   743d9:	add    $0x198,%r8
   743e0:	cmp    $0x3,%cl
   743e3:	je     743fd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x54d>
   743e5:	movzbl %cl,%eax
   743e8:	cmp    $0x2,%eax
   743eb:	jne    74447 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x597>
   743ed:	mov    %r8,%rdi
   743f0:	xor    %esi,%esi
   743f2:	mov    %r13,%rdx
   743f5:	call   *0xb639d(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   743fb:	jmp    7442e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x57e>
   743fd:	mov    %r15,%rcx
   74400:	mov    $0x7,%al
   74402:	add    %r13,%rcx
   74405:	jb     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   7440b:	cmp    %rbx,%rcx
   7440e:	ja     74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74414:	mov    0x48(%rsp),%r14
   74419:	add    %r14,%r15
   7441c:	mov    %r8,%rdi
   7441f:	mov    %r15,%rsi
   74422:	mov    %r13,%rdx
   74425:	mov    %rcx,%r15
   74428:	call   *0xb635a(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   7442e:	mov    %r15,%rsi
   74431:	cmpb   $0x0,0x1f(%rsp)
   74436:	mov    0x78(%rsp),%r15
   7443b:	mov    %rsi,0x40(%rsp)
   74440:	jne    744a8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5f8>
   74442:	jmp    744d0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   74447:	test   %cl,%cl
   74449:	lea    -0x66820(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74450:	lea    -0x66707(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74457:	cmove  %rcx,%rax
   7445b:	cmpb   $0x0,0x1f(%rsp)
   74460:	lea    -0x66b37(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74467:	cmove  %rcx,%rax
   7446b:	mov    %rax,(%rsp)
   7446f:	lea    0x20(%rsp),%rdi
   74474:	mov    %r14,%rsi
   74477:	mov    %rbx,%rdx
   7447a:	mov    %r15,%rcx
   7447d:	mov    0x38(%rsp),%r9
   74482:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   74487:	cmpb   $0x0,0x20(%rsp)
   7448c:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   74492:	mov    0x28(%rsp),%rsi
   74497:	cmpb   $0x0,0x1f(%rsp)
   7449c:	mov    0x78(%rsp),%r15
   744a1:	mov    %rsi,0x40(%rsp)
   744a6:	je     744d0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x620>
   744a8:	mov    0x70(%rsp),%rax
   744ad:	cmp    %rax,%rbx
   744b0:	je     74b70 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcc0>
   744b6:	movzbl (%r14,%rax,1),%eax
   744bb:	mov    %eax,%ecx
   744bd:	and    $0x3,%ecx
   744c0:	lea    -0x63d0f(%rip),%rdx        # 107b8 <libm::math::generic::sqrt::RSQRT_TAB+0x3d2>
   744c7:	movslq (%rdx,%rcx,4),%rcx
   744cb:	add    %rdx,%rcx
   744ce:	jmp    *%rcx
   744d0:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   744d5:	test   %al,%al
   744d7:	je     747a9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8f9>
   744dd:	movd   0x54(%rsp),%xmm0
   744e3:	pshufd $0x0,%xmm0,%xmm0
   744e8:	mov    %r13,%rcx
   744eb:	shr    $0x4,%rcx
   744ef:	je     7474e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x89e>
   744f5:	lea    0x0(,%r13,2),%r10
   744fd:	lea    0x0(,%r13,2),%rsi
   74505:	add    %r13,%rsi
   74508:	mov    %r13,%r8
   7450b:	neg    %r8
   7450e:	neg    %r10
   74511:	mov    %rsi,%r14
   74514:	neg    %r14
   74517:	lea    (%rsp,%rsi,1),%r15
   7451b:	add    $0x198,%r15
   74522:	shl    $0x4,%rcx
   74526:	xor    %eax,%eax
   74528:	mov    %rbp,%r9
   7452b:	xor    %edi,%edi
   7452d:	movdqa -0x66bd6(%rip),%xmm8        # d960 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   74536:	movdqa -0x66d6f(%rip),%xmm9        # d7d0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x901>
   7453f:	nop
   74540:	cmp    %rbp,%rdi
   74543:	ja     74b01 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc51>
   74549:	mov    %r9,%r11
   7454c:	sub    $0x10,%r11
   74550:	jb     74a78 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74556:	lea    (%rdi,%r13,1),%rdx
   7455a:	cmp    %rbp,%rdx
   7455d:	ja     74aa0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbf0>
   74563:	lea    (%r8,%r9,1),%rdx
   74567:	cmp    $0x10,%rdx
   7456b:	jb     74a78 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74571:	lea    0x0(,%r13,2),%rdx
   74579:	add    %rdi,%rdx
   7457c:	cmp    %rbp,%rdx
   7457f:	ja     74ab7 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc07>
   74585:	lea    (%r10,%r9,1),%rdx
   74589:	cmp    $0x10,%rdx
   7458d:	jb     74a78 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   74593:	lea    (%rsi,%rdi,1),%rdx
   74597:	cmp    %rbp,%rdx
   7459a:	ja     74ae5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc35>
   745a0:	add    %r14,%r9
   745a3:	cmp    $0x10,%r9
   745a7:	jb     74a78 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc8>
   745ad:	lea    0x3f(%rax),%rdx
   745b1:	cmp    %rbp,%rdx
   745b4:	jae    74a89 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd9>
   745ba:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   745c3:	lea    (%rsp,%r13,1),%rdx
   745c7:	add    $0x198,%rdx
   745ce:	movdqu (%rdx,%rdi,1),%xmm4
   745d3:	lea    0x198(%rsp,%r13,2),%rdx
   745db:	movdqu (%rdx,%rdi,1),%xmm5
   745e0:	movdqa %xmm3,%xmm2
   745e4:	punpckhbw %xmm4,%xmm2
   745e8:	movdqu (%r15,%rdi,1),%xmm6
   745ee:	movdqa %xmm5,%xmm7
   745f2:	punpckhbw %xmm6,%xmm7
   745f6:	movdqa %xmm2,%xmm1
   745fa:	punpckhwd %xmm7,%xmm1
   745fe:	punpcklwd %xmm7,%xmm2
   74602:	punpcklbw %xmm4,%xmm3
   74606:	punpcklbw %xmm6,%xmm5
   7460a:	movdqa %xmm3,%xmm4
   7460e:	punpckhwd %xmm5,%xmm4
   74612:	punpcklwd %xmm5,%xmm3
   74616:	movdqa %xmm3,%xmm5
   7461a:	psrlw  $0x1,%xmm5
   7461f:	pand   %xmm8,%xmm3
   74624:	pxor   %xmm6,%xmm6
   74628:	psubb  %xmm3,%xmm6
   7462c:	pand   %xmm9,%xmm5
   74631:	pxor   %xmm6,%xmm5
   74635:	movdqa %xmm5,%xmm3
   74639:	pslldq $0x4,%xmm3
   7463e:	paddb  %xmm5,%xmm3
   74642:	movdqa %xmm3,%xmm5
   74646:	pslldq $0x8,%xmm5
   7464b:	paddb  %xmm0,%xmm3
   7464f:	paddb  %xmm5,%xmm3
   74653:	movdqu %xmm3,0x8(%r12,%rdi,4)
   7465a:	movdqa %xmm4,%xmm0
   7465e:	psrlw  $0x1,%xmm0
   74663:	pand   %xmm8,%xmm4
   74668:	pxor   %xmm5,%xmm5
   7466c:	psubb  %xmm4,%xmm5
   74670:	pand   %xmm9,%xmm0
   74675:	pxor   %xmm5,%xmm0
   74679:	movdqa %xmm0,%xmm4
   7467d:	pslldq $0x4,%xmm4
   74682:	paddb  %xmm0,%xmm4
   74686:	movdqa %xmm4,%xmm0
   7468a:	pslldq $0x8,%xmm0
   7468f:	pshufd $0xff,%xmm3,%xmm3
   74694:	paddb  %xmm4,%xmm0
   74698:	paddb  %xmm3,%xmm0
   7469c:	movdqu %xmm0,0x18(%r12,%rdi,4)
   746a3:	movdqa %xmm2,%xmm3
   746a7:	psrlw  $0x1,%xmm3
   746ac:	pand   %xmm8,%xmm2
   746b1:	pxor   %xmm4,%xmm4
   746b5:	psubb  %xmm2,%xmm4
   746b9:	pand   %xmm9,%xmm3
   746be:	pxor   %xmm4,%xmm3
   746c2:	movdqa %xmm3,%xmm2
   746c6:	pslldq $0x4,%xmm2
   746cb:	paddb  %xmm3,%xmm2
   746cf:	movdqa %xmm2,%xmm3
   746d3:	pslldq $0x8,%xmm3
   746d8:	pshufd $0xff,%xmm0,%xmm0
   746dd:	paddb  %xmm2,%xmm3
   746e1:	paddb  %xmm0,%xmm3
   746e5:	movdqu %xmm3,0x28(%r12,%rdi,4)
   746ec:	movdqa %xmm1,%xmm0
   746f0:	psrlw  $0x1,%xmm0
   746f5:	pand   %xmm8,%xmm1
   746fa:	pxor   %xmm2,%xmm2
   746fe:	psubb  %xmm1,%xmm2
   74702:	pand   %xmm9,%xmm0
   74707:	pxor   %xmm2,%xmm0
   7470b:	movdqa %xmm0,%xmm1
   7470f:	pslldq $0x4,%xmm1
   74714:	paddb  %xmm0,%xmm1
   74718:	movdqa %xmm1,%xmm0
   7471c:	pslldq $0x8,%xmm0
   74721:	pshufd $0xff,%xmm3,%xmm2
   74726:	paddb  %xmm1,%xmm0
   7472a:	paddb  %xmm2,%xmm0
   7472e:	movdqu %xmm0,0x38(%r12,%rdi,4)
   74735:	add    $0x10,%rdi
   74739:	pshufd $0xff,%xmm0,%xmm0
   7473e:	add    $0x40,%rax
   74742:	mov    %r11,%r9
   74745:	cmp    %rdi,%rcx
   74748:	jne    74540 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x690>
   7474e:	mov    %r13d,%eax
   74751:	and    $0x1f0,%eax
   74756:	cmp    %r13,%rax
   74759:	mov    0x78(%rsp),%r15
   7475e:	je     7485d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   74764:	movdqa %xmm0,0x20(%rsp)
   7476a:	lea    0x0(,%rax,4),%edx
   74771:	mov    %rbp,%rcx
   74774:	sub    %rdx,%rcx
   74777:	jb     74b5a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcaa>
   7477d:	lea    0x8(%r12),%rsi
   74782:	add    %rsi,%rdx
   74785:	mov    %rax,0x8(%rsp)
   7478a:	lea    0x20(%rsp),%rax
   7478f:	mov    %rax,(%rsp)
   74793:	mov    $0x4,%r9d
   74799:	lea    0x198(%rsp),%rdi
   747a1:	mov    %r13,%r8
   747a4:	mov    %rbp,%rsi
   747a7:	jmp    747da <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x92a>
   747a9:	lea    0x98(%rsp),%rax
   747b1:	mov    %rax,(%rsp)
   747b5:	movq   $0x0,0x8(%rsp)
   747be:	mov    $0x4,%r9d
   747c4:	lea    0x198(%rsp),%rdi
   747cc:	mov    %rbp,%rsi
   747cf:	lea    0x8(%r12),%rdx
   747d4:	mov    %r13,%r8
   747d7:	mov    %rbp,%rcx
   747da:	call   72cc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   747df:	jmp    7485d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   747e1:	shr    $0x4,%al
   747e4:	neg    %al
   747e6:	and    $0x1f,%al
   747e8:	movzbl %al,%eax
   747eb:	lea    0x98(%rsp),%rcx
   747f3:	mov    %rcx,(%rsp)
   747f7:	mov    %eax,0x8(%rsp)
   747fb:	movq   $0x0,0x10(%rsp)
   74804:	mov    $0x4,%r9d
   7480a:	lea    0x198(%rsp),%rdi
   74812:	mov    %rbp,%rsi
   74815:	lea    0x8(%r12),%rdx
   7481a:	mov    %r13,%r8
   7481d:	mov    %rbp,%rcx
   74820:	call   73070 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   74825:	jmp    7485d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9ad>
   74827:	lea    0x98(%rsp),%rax
   7482f:	mov    %rax,(%rsp)
   74833:	movq   $0x0,0x8(%rsp)
   7483c:	mov    $0x4,%r9d
   74842:	lea    0x198(%rsp),%rdi
   7484a:	mov    %rbp,%rsi
   7484d:	lea    0x8(%r12),%rdx
   74852:	mov    %r13,%r8
   74855:	mov    %rbp,%rcx
   74858:	call   72eb0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7485d:	cmp    0x90(%rsp),%r15
   74865:	je     74b2b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc7b>
   7486b:	mov    0x4(%r12,%rbp,1),%eax
   74870:	mov    %eax,0x98(%rsp)
   74877:	mov    0x408(%r12),%r14
   7487f:	cmp    $0x401,%r14
   74886:	jae    74b14 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc64>
   7488c:	mov    %eax,0x54(%rsp)
   74890:	mov    (%r12),%r13
   74894:	mov    0x0(%r13),%rax
   74898:	mov    0x10(%r13),%rbp
   7489c:	sub    %rbp,%rax
   7489f:	cmp    %rax,%r14
   748a2:	ja     7408e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1de>
   748a8:	test   %r14,%r14
   748ab:	jne    740ab <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fb>
   748b1:	jmp    740c0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x210>
   748b6:	cmp    $0x2,%ecx
   748b9:	je     74b81 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   748bf:	cmp    $0x3,%ecx
   748c2:	je     74b98 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   748c8:	test   %cl,%cl
   748ca:	lea    -0x66ca1(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   748d1:	lea    -0x66b88(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   748d8:	cmove  %rcx,%rax
   748dc:	mov    0x38(%rsp),%r9
   748e1:	mov    %rax,(%rsp)
   748e5:	lea    0x20(%rsp),%rdi
   748ea:	mov    %r14,%rsi
   748ed:	mov    %rbx,%rdx
   748f0:	mov    %r8,%rcx
   748f3:	lea    0x198(%rsp),%r8
   748fb:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   74900:	cmpb   $0x0,0x20(%rsp)
   74905:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7490b:	mov    0x40(%rsp),%eax
   7490f:	shr    $0x2,%al
   74912:	and    $0x3,%al
   74914:	cmp    $0x2,%al
   74916:	je     74b81 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7491c:	movzbl %al,%ecx
   7491f:	cmp    $0x3,%ecx
   74922:	je     74b98 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   74928:	mov    0x28(%rsp),%rcx
   7492d:	test   %al,%al
   7492f:	lea    -0x66d06(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74936:	lea    -0x66bed(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7493d:	cmove  %rdx,%rax
   74941:	cmpb   $0x0,0x1f(%rsp)
   74946:	lea    0x198(%rsp,%r13,1),%r8
   7494e:	lea    -0x67025(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74955:	cmove  %rdx,%rax
   74959:	mov    %rax,(%rsp)
   7495d:	lea    0x20(%rsp),%rdi
   74962:	mov    %r14,%rsi
   74965:	mov    %rbx,%rdx
   74968:	mov    0x38(%rsp),%r9
   7496d:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   74972:	cmpb   $0x0,0x20(%rsp)
   74977:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   7497d:	mov    0x40(%rsp),%eax
   74981:	shr    $0x4,%al
   74984:	and    $0x3,%al
   74986:	cmp    $0x2,%al
   74988:	je     74b81 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   7498e:	movzbl %al,%ecx
   74991:	cmp    $0x3,%ecx
   74994:	je     74b98 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   7499a:	mov    0x28(%rsp),%rcx
   7499f:	test   %al,%al
   749a1:	lea    -0x66d78(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   749a8:	lea    -0x66c5f(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   749af:	cmove  %rdx,%rax
   749b3:	cmpb   $0x0,0x1f(%rsp)
   749b8:	lea    0x198(%rsp,%r13,2),%r8
   749c0:	lea    -0x67097(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   749c7:	cmove  %rdx,%rax
   749cb:	mov    %rax,(%rsp)
   749cf:	lea    0x20(%rsp),%rdi
   749d4:	mov    %r14,%rsi
   749d7:	mov    %rbx,%rdx
   749da:	mov    0x38(%rsp),%r9
   749df:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   749e4:	cmpb   $0x0,0x20(%rsp)
   749e9:	jne    74a61 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb1>
   749eb:	mov    0x40(%rsp),%edx
   749ef:	shr    $0x6,%dl
   749f2:	cmp    $0x2,%dl
   749f5:	je     74b81 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcd1>
   749fb:	movzbl %dl,%eax
   749fe:	cmp    $0x3,%eax
   74a01:	je     74b98 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce8>
   74a07:	mov    0x28(%rsp),%rcx
   74a0c:	test   %dl,%dl
   74a0e:	lea    -0x66de5(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   74a15:	lea    -0x66ccc(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   74a1c:	cmove  %rdx,%rax
   74a20:	cmpb   $0x0,0x1f(%rsp)
   74a25:	mov    0x60(%rsp),%rdx
   74a2a:	lea    0x198(%rsp,%rdx,1),%r8
   74a32:	lea    -0x67109(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   74a39:	cmove  %rdx,%rax
   74a3d:	mov    %rax,(%rsp)
   74a41:	lea    0x20(%rsp),%rdi
   74a46:	mov    %r14,%rsi
   74a49:	mov    %rbx,%rdx
   74a4c:	mov    0x38(%rsp),%r9
   74a51:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   74a56:	cmpb   $0x0,0x20(%rsp)
   74a5b:	je     74492 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e2>
   74a61:	movzbl 0x21(%rsp),%eax
   74a66:	add    $0x598,%rsp
   74a6d:	pop    %rbx
   74a6e:	pop    %r12
   74a70:	pop    %r13
   74a72:	pop    %r14
   74a74:	pop    %r15
   74a76:	pop    %rbp
   74a77:	ret
   74a78:	lea    0xb1551(%rip),%rdi        # 125fd0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2db8>
   74a7f:	call   *0xb5eb3(%rip)        # 12a938 <_DYNAMIC+0x3c8>
   74a85:	mov    $0x7,%al
   74a87:	jmp    74a66 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbb6>
   74a89:	lea    0x40(%rax),%rsi
   74a8d:	lea    0xb0d44(%rip),%rcx        # 1257d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25c0>
   74a94:	mov    %rax,%rdi
   74a97:	mov    %rbp,%rdx
   74a9a:	call   *0xb5e40(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74aa0:	mov    0x58(%rsp),%rcx
   74aa5:	cmp    $0x100,%rcx
   74aac:	mov    $0x100,%eax
   74ab1:	cmovb  %rcx,%rax
   74ab5:	jmp    74afe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4e>
   74ab7:	mov    0x58(%rsp),%rcx
   74abc:	cmp    $0x100,%rcx
   74ac3:	mov    $0x100,%eax
   74ac8:	cmovb  %rcx,%rax
   74acc:	mov    %eax,%eax
   74ace:	lea    (%rdi,%rax,2),%rdi
   74ad2:	lea    0xb14df(%rip),%rcx        # 125fb8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74ad9:	mov    %rbp,%rsi
   74adc:	mov    %rbp,%rdx
   74adf:	call   *0xb5dfb(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74ae5:	mov    0x58(%rsp),%rcx
   74aea:	cmp    $0x100,%rcx
   74af1:	mov    $0x100,%eax
   74af6:	cmovb  %rcx,%rax
   74afa:	lea    (%rax,%rax,2),%rax
   74afe:	add    %rax,%rdi
   74b01:	lea    0xb14b0(%rip),%rcx        # 125fb8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   74b08:	mov    %rbp,%rsi
   74b0b:	mov    %rbp,%rdx
   74b0e:	call   *0xb5dcc(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b14:	lea    0xb41b5(%rip),%rcx        # 128cd0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ab8>
   74b1b:	mov    $0x400,%edx
   74b20:	xor    %edi,%edi
   74b22:	mov    %r14,%rsi
   74b25:	call   *0xb5db5(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b2b:	lea    -0x4(%rbp),%rdi
   74b2f:	lea    0xb086a(%rip),%rcx        # 1253a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   74b36:	mov    %rbp,%rsi
   74b39:	mov    %rbp,%rdx
   74b3c:	call   *0xb5d9e(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b42:	lea    0xb092f(%rip),%rcx        # 125478 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   74b49:	mov    0x70(%rsp),%rdi
   74b4e:	mov    %rbx,%rsi
   74b51:	mov    %rbx,%rdx
   74b54:	call   *0xb5d86(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b5a:	lea    0xb0c5f(%rip),%rcx        # 1257c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25a8>
   74b61:	mov    %rdx,%rdi
   74b64:	mov    %rbp,%rsi
   74b67:	mov    %rbp,%rdx
   74b6a:	call   *0xb5d70(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b70:	lea    0xb0859(%rip),%rdx        # 1253d0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   74b77:	xor    %edi,%edi
   74b79:	xor    %esi,%esi
   74b7b:	call   *0xb5c3f(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   74b81:	lea    0xb08a8(%rip),%rcx        # 125430 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   74b88:	xor    %edi,%edi
   74b8a:	mov    %r13,%rsi
   74b8d:	mov    0x38(%rsp),%rdx
   74b92:	call   *0xb5d48(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   74b98:	lea    0xb0879(%rip),%rcx        # 125418 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   74b9f:	xor    %edi,%edi
   74ba1:	mov    %r13,%rsi
   74ba4:	mov    0x38(%rsp),%rdx
   74ba9:	call   *0xb5d31(%rip)        # 12a8e0 <_DYNAMIC+0x370>
