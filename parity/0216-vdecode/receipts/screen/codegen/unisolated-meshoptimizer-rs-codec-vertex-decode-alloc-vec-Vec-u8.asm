
<artifacts>/screen/driver-unisolated:     file format elf64-x86-64


Disassembly of section .text:

0000000000073170 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   73170:	push   %rbp
   73171:	push   %r15
   73173:	push   %r14
   73175:	push   %r13
   73177:	push   %r12
   73179:	push   %rbx
   7317a:	sub    $0x5f8,%rsp
   73181:	mov    %r8,%rbx
   73184:	mov    %rcx,%r12
   73187:	mov    %rdx,%r14
   7318a:	mov    %rsi,%r13
   7318d:	mov    %rdi,%rbp
   73190:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73195:	cmp    $0x2,%al
   73197:	jae    73200 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x90>
   73199:	mov    $0x7,%al
   7319b:	test   %rbx,%rbx
   7319e:	je     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   731a4:	movzbl (%r12),%edx
   731a9:	mov    %edx,%ecx
   731ab:	and    $0xf0,%cl
   731ae:	cmp    $0xa0,%cl
   731b1:	jne    73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   731b7:	and    $0xf,%dl
   731ba:	mov    $0x5,%al
   731bc:	mov    %dl,0x27(%rsp)
   731c0:	cmp    $0x2,%dl
   731c3:	jae    73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   731c9:	cmpb   $0x0,0x27(%rsp)
   731ce:	je     73292 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x122>
   731d4:	mov    %r14,%rcx
   731d7:	shr    $0x2,%rcx
   731db:	add    %r14,%rcx
   731de:	cmp    $0x19,%rcx
   731e2:	mov    $0x18,%r15d
   731e8:	cmovae %rcx,%r15
   731ec:	lea    0x1(%r15),%rdx
   731f0:	mov    $0x7,%al
   731f2:	cmp    %rdx,%rbx
   731f5:	jae    732b2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x142>
   731fb:	jmp    73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73200:	cmp    $0x21,%r13
   73204:	jae    73221 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb1>
   73206:	cmp    $0x4,%r14
   7320a:	je     7324e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xde>
   7320c:	cmp    $0xc,%r14
   73210:	je     7323c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcc>
   73212:	cmp    $0x20,%r14
   73216:	jne    73260 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf0>
   73218:	lea    0xd291(%rip),%rax        # 804b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   7321f:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73221:	cmp    $0x4,%r14
   73225:	je     73257 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xe7>
   73227:	cmp    $0xc,%r14
   7322b:	je     73245 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xd5>
   7322d:	cmp    $0x20,%r14
   73231:	jne    73269 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf9>
   73233:	lea    0x6546(%rip),%rax        # 79780 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   7323a:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   7323c:	lea    0x1184d(%rip),%rax        # 84a90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, alloc::vec::Vec<u8>>>
   73243:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73245:	lea    0xa444(%rip),%rax        # 7d690 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, alloc::vec::Vec<u8>>>
   7324c:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   7324e:	lea    0xea3b(%rip),%rax        # 81c90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   73255:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73257:	lea    0x7952(%rip),%rax        # 7abb0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   7325e:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73260:	lea    0xb8d9(%rip),%rax        # 7eb40 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   73267:	jmp    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73269:	lea    0x4fa0(%rip),%rax        # 78210 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   73270:	mov    %rbp,%rdi
   73273:	mov    %r13,%rsi
   73276:	mov    %r14,%rdx
   73279:	mov    %r12,%rcx
   7327c:	mov    %rbx,%r8
   7327f:	add    $0x5f8,%rsp
   73286:	pop    %rbx
   73287:	pop    %r12
   73289:	pop    %r13
   7328b:	pop    %r14
   7328d:	pop    %r15
   7328f:	pop    %rbp
   73290:	jmp    *%rax
   73292:	cmp    $0x21,%r14
   73296:	mov    $0x20,%r15d
   7329c:	cmovae %r14,%r15
   732a0:	mov    %r14,%rcx
   732a3:	lea    0x1(%r15),%rdx
   732a7:	mov    $0x7,%al
   732a9:	cmp    %rdx,%rbx
   732ac:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   732b2:	mov    %rbx,%rsi
   732b5:	sub    %rcx,%rsi
   732b8:	lea    (%rsi,%r14,1),%rax
   732bc:	mov    %rbx,%rcx
   732bf:	mov    %rax,0x70(%rsp)
   732c4:	sub    %rax,%rcx
   732c7:	mov    %rcx,0xa8(%rsp)
   732cf:	jb     73d9d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2d>
   732d5:	xorps  %xmm0,%xmm0
   732d8:	movaps %xmm0,0x1e0(%rsp)
   732e0:	movaps %xmm0,0x1d0(%rsp)
   732e8:	movaps %xmm0,0x1c0(%rsp)
   732f0:	movaps %xmm0,0x1b0(%rsp)
   732f8:	movaps %xmm0,0x1a0(%rsp)
   73300:	movaps %xmm0,0x190(%rsp)
   73308:	movaps %xmm0,0x180(%rsp)
   73310:	movaps %xmm0,0x170(%rsp)
   73318:	movaps %xmm0,0x160(%rsp)
   73320:	movaps %xmm0,0x150(%rsp)
   73328:	movaps %xmm0,0x140(%rsp)
   73330:	movaps %xmm0,0x130(%rsp)
   73338:	movaps %xmm0,0x120(%rsp)
   73340:	movaps %xmm0,0x110(%rsp)
   73348:	movaps %xmm0,0x100(%rsp)
   73350:	movaps %xmm0,0xf0(%rsp)
   73358:	cmp    $0x101,%r14
   7335f:	jae    73d59 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbe9>
   73365:	lea    (%rsi,%r14,1),%rax
   73369:	cmp    %rsi,%rax
   7336c:	jb     73d70 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc00>
   73372:	add    %r12,%rsi
   73375:	lea    0xf0(%rsp),%rdi
   7337d:	mov    %r14,%rdx
   73380:	call   *0xb7452(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   73386:	test   %r14,%r14
   73389:	je     73df5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc85>
   7338f:	mov    %r15,0xd8(%rsp)
   73397:	mov    $0x2000,%ax
   7339b:	xor    %edx,%edx
   7339d:	div    %r14w
   733a1:	and    $0x3ff0,%eax
   733a6:	cmp    $0x100,%eax
   733ab:	mov    $0x100,%ecx
   733b0:	cmovb  %eax,%ecx
   733b3:	mov    %rcx,0xe0(%rsp)
   733bb:	lea    0x1f8(%rsp),%rdi
   733c3:	mov    $0x400,%edx
   733c8:	xor    %esi,%esi
   733ca:	call   *0xb7418(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   733d0:	test   %r13,%r13
   733d3:	je     73cf3 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb83>
   733d9:	add    %r12,0x70(%rsp)
   733de:	mov    %r14,%rax
   733e1:	shr    $0x2,%rax
   733e5:	mov    %rax,0xa0(%rsp)
   733ed:	lea    0x3(%r14),%rax
   733f1:	shr    $0x2,%rax
   733f5:	mov    %rax,0xe8(%rsp)
   733fd:	mov    $0x1,%r15d
   73403:	xor    %eax,%eax
   73405:	mov    %rbp,0x68(%rsp)
   7340a:	mov    %rbx,0x30(%rsp)
   7340f:	mov    %r12,0x38(%rsp)
   73414:	mov    %r13,0xc0(%rsp)
   7341c:	mov    %r13,%rdx
   7341f:	mov    %rax,0x58(%rsp)
   73424:	sub    %rax,%rdx
   73427:	mov    0xe0(%rsp),%rax
   7342f:	cmp    %rax,%rdx
   73432:	cmovae %rax,%rdx
   73436:	mov    %rdx,0x28(%rsp)
   7343b:	cmpb   $0x0,0x27(%rsp)
   73440:	je     73485 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x315>
   73442:	mov    %r15,%rcx
   73445:	mov    $0x7,%al
   73447:	add    0xa0(%rsp),%rcx
   7344f:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73455:	cmp    %rbx,%rcx
   73458:	ja     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7345e:	add    0x38(%rsp),%r15
   73463:	mov    0xa0(%rsp),%rax
   7346b:	mov    %rax,0x98(%rsp)
   73473:	mov    %r15,0xb0(%rsp)
   7347b:	mov    %rcx,%r15
   7347e:	mov    0x68(%rsp),%rbp
   73483:	jmp    7349e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x32e>
   73485:	movq   $0x0,0x98(%rsp)
   73491:	mov    $0x1,%eax
   73496:	mov    %rax,0xb0(%rsp)
   7349e:	mov    0x58(%rsp),%rax
   734a3:	mov    0x28(%rsp),%rcx
   734a8:	lea    (%rcx,%rax,1),%r12
   734ac:	mov    %r12,0xd0(%rsp)
   734b4:	imul   %r14,%r12
   734b8:	mov    0x10(%rbp),%r13
   734bc:	mov    %rbp,%rax
   734bf:	mov    %r12,%rbp
   734c2:	mov    %r12,%rdx
   734c5:	sub    %r13,%rbp
   734c8:	jbe    73512 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x3a2>
   734ca:	mov    (%rax),%rax
   734cd:	sub    %r13,%rax
   734d0:	cmp    %rax,%rbp
   734d3:	ja     73ccc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5c>
   734d9:	mov    0x68(%rsp),%rax
   734de:	mov    0x8(%rax),%rbx
   734e2:	lea    (%rbx,%r13,1),%rdi
   734e6:	cmp    $0x2,%rbp
   734ea:	jb     73504 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x394>
   734ec:	lea    -0x1(%rbp),%rdx
   734f0:	xor    %esi,%esi
   734f2:	call   *0xb72f0(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   734f8:	add    %rbp,%r13
   734fb:	dec    %r13
   734fe:	add    %r13,%rbx
   73501:	mov    %rbx,%rdi
   73504:	movb   $0x0,(%rdi)
   73507:	inc    %r13
   7350a:	mov    %r13,%rdx
   7350d:	mov    0x68(%rsp),%rax
   73512:	mov    0x58(%rsp),%rcx
   73517:	imul   %r14,%rcx
   7351b:	mov    %rdx,0x10(%rax)
   7351f:	mov    %rcx,0x58(%rsp)
   73524:	cmp    %rcx,%r12
   73527:	jb     73d88 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   7352d:	cmp    %rdx,%r12
   73530:	ja     73d88 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   73536:	mov    0x28(%rsp),%rdx
   7353b:	lea    0xf(%rdx),%ecx
   7353e:	and    $0x3f0,%ecx
   73544:	mov    %rcx,0x50(%rsp)
   73549:	mov    %rdx,%rcx
   7354c:	imul   %r14,%rcx
   73550:	mov    %rcx,0xb8(%rsp)
   73558:	mov    0x58(%rsp),%rcx
   7355d:	add    0x8(%rax),%rcx
   73561:	mov    %rcx,0x58(%rsp)
   73566:	lea    -0x1(%rdx),%rax
   7356a:	imul   %r14,%rax
   7356e:	mov    %rax,0xc8(%rsp)
   73576:	add    $0x4,%rax
   7357a:	mov    %rax,0x60(%rsp)
   7357f:	lea    0x0(,%rdx,4),%rax
   73587:	mov    %rax,0x78(%rsp)
   7358c:	lea    0x1f8(%rsp,%rdx,1),%rax
   73594:	mov    %rax,0x90(%rsp)
   7359c:	lea    0x1f8(%rsp,%rdx,2),%rax
   735a4:	mov    %rax,0x88(%rsp)
   735ac:	lea    (%rdx,%rdx,2),%rax
   735b0:	add    %rsp,%rax
   735b3:	add    $0x1f8,%rax
   735b9:	mov    %rax,0x80(%rsp)
   735c1:	xor    %r13d,%r13d
   735c4:	xor    %ebp,%ebp
   735c6:	movzbl 0x27(%rsp),%r12d
   735cc:	jmp    73628 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4b8>
   735ce:	lea    (%rsp,%rbp,1),%rcx
   735d2:	add    $0xf0,%rcx
   735d9:	shr    $0x4,%al
   735dc:	neg    %al
   735de:	and    $0x1f,%al
   735e0:	movzbl %al,%eax
   735e3:	mov    %rcx,(%rsp)
   735e7:	mov    %eax,0x8(%rsp)
   735eb:	movq   $0x0,0x10(%rsp)
   735f4:	lea    0x1f8(%rsp),%rdi
   735fc:	mov    0x78(%rsp),%rsi
   73601:	mov    0x60(%rsp),%rcx
   73606:	mov    0x28(%rsp),%r8
   7360b:	mov    %r14,%r9
   7360e:	call   73000 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   73613:	inc    %r13
   73616:	add    $0x4,%rbp
   7361a:	cmp    %r13,0xe8(%rsp)
   73622:	je     73c72 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb02>
   73628:	test   %r12b,%r12b
   7362b:	je     7364a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4da>
   7362d:	cmp    0x98(%rsp),%r13
   73635:	jae    73dc5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc55>
   7363b:	mov    0xb0(%rsp),%rax
   73643:	movzbl (%rax,%r13,1),%ebx
   73648:	jmp    7364c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4dc>
   7364a:	xor    %ebx,%ebx
   7364c:	mov    %ebx,%eax
   7364e:	and    $0x3,%al
   73650:	movzbl %al,%ecx
   73653:	mov    0x50(%rsp),%rdx
   73658:	cmp    %rdx,0x28(%rsp)
   7365d:	ja     73ac5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x955>
   73663:	cmp    $0x2,%al
   73665:	je     736a4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x534>
   73667:	cmp    $0x3,%ecx
   7366a:	jne    736be <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x54e>
   7366c:	mov    %r15,%r12
   7366f:	mov    $0x7,%al
   73671:	add    0x28(%rsp),%r12
   73676:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7367c:	cmp    0x30(%rsp),%r12
   73681:	ja     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73687:	add    0x38(%rsp),%r15
   7368c:	lea    0x1f8(%rsp),%rdi
   73694:	mov    %r15,%rsi
   73697:	mov    0x28(%rsp),%rdx
   7369c:	call   *0xb7136(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   736a2:	jmp    73718 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   736a4:	lea    0x1f8(%rsp),%rdi
   736ac:	xor    %esi,%esi
   736ae:	mov    0x28(%rsp),%rdx
   736b3:	call   *0xb712f(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   736b9:	mov    %r15,%r12
   736bc:	jmp    73718 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   736be:	test   %al,%al
   736c0:	lea    -0x65a97(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   736c7:	lea    -0x6597e(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   736ce:	cmove  %rcx,%rax
   736d2:	test   %r12b,%r12b
   736d5:	lea    -0x65dac(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   736dc:	cmove  %rcx,%rax
   736e0:	mov    %rax,(%rsp)
   736e4:	lea    0x40(%rsp),%rdi
   736e9:	mov    0x38(%rsp),%rsi
   736ee:	mov    0x30(%rsp),%rdx
   736f3:	mov    %r15,%rcx
   736f6:	lea    0x1f8(%rsp),%r8
   736fe:	mov    0x50(%rsp),%r9
   73703:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   73708:	cmpb   $0x0,0x40(%rsp)
   7370d:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73713:	mov    0x48(%rsp),%r12
   73718:	mov    %ebx,%eax
   7371a:	shr    $0x2,%al
   7371d:	and    $0x3,%al
   7371f:	cmp    $0x3,%al
   73721:	je     73745 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5d5>
   73723:	movzbl %al,%ecx
   73726:	cmp    $0x2,%ecx
   73729:	jne    73780 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x610>
   7372b:	mov    0x90(%rsp),%rdi
   73733:	xor    %esi,%esi
   73735:	mov    0x28(%rsp),%rdx
   7373a:	call   *0xb70a8(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   73740:	jmp    737dc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   73745:	mov    %r12,%r15
   73748:	mov    $0x7,%al
   7374a:	add    0x28(%rsp),%r15
   7374f:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73755:	cmp    0x30(%rsp),%r15
   7375a:	ja     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73760:	add    0x38(%rsp),%r12
   73765:	mov    0x90(%rsp),%rdi
   7376d:	mov    %r12,%rsi
   73770:	mov    0x28(%rsp),%rdx
   73775:	call   *0xb705d(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   7377b:	mov    %r15,%r12
   7377e:	jmp    737dc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   73780:	test   %al,%al
   73782:	lea    -0x65b59(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73789:	lea    -0x65a40(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73790:	cmove  %rcx,%rax
   73794:	cmpb   $0x0,0x27(%rsp)
   73799:	lea    -0x65e70(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   737a0:	cmove  %rcx,%rax
   737a4:	mov    %rax,(%rsp)
   737a8:	lea    0x40(%rsp),%rdi
   737ad:	mov    0x38(%rsp),%rsi
   737b2:	mov    0x30(%rsp),%rdx
   737b7:	mov    %r12,%rcx
   737ba:	mov    0x90(%rsp),%r8
   737c2:	mov    0x50(%rsp),%r9
   737c7:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   737cc:	cmpb   $0x0,0x40(%rsp)
   737d1:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   737d7:	mov    0x48(%rsp),%r12
   737dc:	mov    %ebx,%eax
   737de:	shr    $0x4,%al
   737e1:	and    $0x3,%al
   737e3:	cmp    $0x3,%al
   737e5:	je     73815 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6a5>
   737e7:	movzbl %al,%ecx
   737ea:	cmp    $0x2,%ecx
   737ed:	jne    7389a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x72a>
   737f3:	mov    0x88(%rsp),%rdi
   737fb:	xor    %esi,%esi
   737fd:	mov    0x28(%rsp),%rdx
   73802:	call   *0xb6fe0(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   73808:	shr    $0x6,%bl
   7380b:	cmp    $0x3,%bl
   7380e:	jne    7385a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73810:	jmp    73902 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   73815:	mov    %r12,%r15
   73818:	mov    $0x7,%al
   7381a:	add    0x28(%rsp),%r15
   7381f:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73825:	cmp    0x30(%rsp),%r15
   7382a:	ja     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73830:	add    0x38(%rsp),%r12
   73835:	mov    0x88(%rsp),%rdi
   7383d:	mov    %r12,%rsi
   73840:	mov    0x28(%rsp),%rdx
   73845:	call   *0xb6f8d(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   7384b:	mov    %r15,%r12
   7384e:	shr    $0x6,%bl
   73851:	cmp    $0x3,%bl
   73854:	je     73902 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   7385a:	movzbl %bl,%eax
   7385d:	cmp    $0x2,%eax
   73860:	jne    73952 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7e2>
   73866:	mov    0x80(%rsp),%rdi
   7386e:	xor    %esi,%esi
   73870:	mov    0x28(%rsp),%rdx
   73875:	call   *0xb6f6d(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   7387b:	mov    %r12,%r15
   7387e:	movzbl 0x27(%rsp),%r12d
   73884:	test   %r12b,%r12b
   73887:	mov    0xb8(%rsp),%rcx
   7388f:	jne    739c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   73895:	jmp    7394b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   7389a:	test   %al,%al
   7389c:	lea    -0x65c73(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   738a3:	lea    -0x65b5a(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   738aa:	cmove  %rcx,%rax
   738ae:	cmpb   $0x0,0x27(%rsp)
   738b3:	lea    -0x65f8a(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   738ba:	cmove  %rcx,%rax
   738be:	mov    %rax,(%rsp)
   738c2:	lea    0x40(%rsp),%rdi
   738c7:	mov    0x38(%rsp),%rsi
   738cc:	mov    0x30(%rsp),%rdx
   738d1:	mov    %r12,%rcx
   738d4:	mov    0x88(%rsp),%r8
   738dc:	mov    0x50(%rsp),%r9
   738e1:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   738e6:	cmpb   $0x0,0x40(%rsp)
   738eb:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   738f1:	mov    0x48(%rsp),%r12
   738f6:	shr    $0x6,%bl
   738f9:	cmp    $0x3,%bl
   738fc:	jne    7385a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73902:	mov    %r12,%r15
   73905:	mov    $0x7,%al
   73907:	add    0x28(%rsp),%r15
   7390c:	jb     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73912:	cmp    0x30(%rsp),%r15
   73917:	ja     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7391d:	add    0x38(%rsp),%r12
   73922:	mov    0x80(%rsp),%rdi
   7392a:	mov    %r12,%rsi
   7392d:	mov    0x28(%rsp),%rdx
   73932:	call   *0xb6ea0(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   73938:	movzbl 0x27(%rsp),%r12d
   7393e:	test   %r12b,%r12b
   73941:	mov    0xb8(%rsp),%rcx
   73949:	jne    739c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   7394b:	xor    %eax,%eax
   7394d:	jmp    739d9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x869>
   73952:	test   %bl,%bl
   73954:	lea    -0x65d2b(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7395b:	lea    -0x65c12(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73962:	cmove  %rcx,%rax
   73966:	cmpb   $0x0,0x27(%rsp)
   7396b:	lea    -0x66042(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73972:	cmove  %rcx,%rax
   73976:	mov    %rax,(%rsp)
   7397a:	lea    0x40(%rsp),%rdi
   7397f:	mov    0x38(%rsp),%rsi
   73984:	mov    0x30(%rsp),%rdx
   73989:	mov    %r12,%rcx
   7398c:	mov    0x80(%rsp),%r8
   73994:	mov    0x50(%rsp),%r9
   73999:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   7399e:	cmpb   $0x0,0x40(%rsp)
   739a3:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   739a9:	mov    0x48(%rsp),%r15
   739ae:	movzbl 0x27(%rsp),%r12d
   739b4:	test   %r12b,%r12b
   739b7:	mov    0xb8(%rsp),%rcx
   739bf:	je     7394b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   739c1:	cmp    0xa8(%rsp),%r13
   739c9:	jae    73ddd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc6d>
   739cf:	mov    0x70(%rsp),%rax
   739d4:	movzbl (%rax,%r13,1),%eax
   739d9:	mov    %rcx,%rdx
   739dc:	sub    %rbp,%rdx
   739df:	jb     73d43 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd3>
   739e5:	cmp    %rdx,0x60(%rsp)
   739ea:	ja     73d2f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbbf>
   739f0:	mov    0x58(%rsp),%rcx
   739f5:	lea    (%rcx,%rbp,1),%rdx
   739f9:	mov    %eax,%ecx
   739fb:	and    $0x3,%cl
   739fe:	movzbl %cl,%ecx
   73a01:	lea    -0x63260(%rip),%rsi        # 107a8 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   73a08:	movslq (%rsi,%rcx,4),%rcx
   73a0c:	add    %rsi,%rcx
   73a0f:	jmp    *%rcx
   73a11:	mov    %rdx,%r12
   73a14:	lea    (%rsp,%rbp,1),%rbx
   73a18:	add    $0xf0,%rbx
   73a1f:	call   e5e80 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73a24:	test   %al,%al
   73a26:	je     73a8b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91b>
   73a28:	mov    %rbx,(%rsp)
   73a2c:	lea    0x1f8(%rsp),%rdi
   73a34:	mov    0x78(%rsp),%rsi
   73a39:	mov    %r12,%rdx
   73a3c:	mov    0x60(%rsp),%rcx
   73a41:	mov    0x28(%rsp),%r8
   73a46:	mov    %r14,%r9
   73a49:	call   75d90 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   73a4e:	jmp    73aba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x94a>
   73a50:	lea    (%rsp,%rbp,1),%rax
   73a54:	add    $0xf0,%rax
   73a5a:	mov    %rax,(%rsp)
   73a5e:	movq   $0x0,0x8(%rsp)
   73a67:	lea    0x1f8(%rsp),%rdi
   73a6f:	mov    0x78(%rsp),%rsi
   73a74:	mov    0x60(%rsp),%rcx
   73a79:	mov    0x28(%rsp),%r8
   73a7e:	mov    %r14,%r9
   73a81:	call   72e40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   73a86:	jmp    73613 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   73a8b:	mov    %rbx,(%rsp)
   73a8f:	movq   $0x0,0x8(%rsp)
   73a98:	lea    0x1f8(%rsp),%rdi
   73aa0:	mov    0x78(%rsp),%rsi
   73aa5:	mov    %r12,%rdx
   73aa8:	mov    0x60(%rsp),%rcx
   73aad:	mov    0x28(%rsp),%r8
   73ab2:	mov    %r14,%r9
   73ab5:	call   72c50 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   73aba:	movzbl 0x27(%rsp),%r12d
   73ac0:	jmp    73613 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   73ac5:	cmp    $0x2,%al
   73ac7:	je     73e1b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73acd:	cmp    $0x3,%ecx
   73ad0:	je     73e02 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73ad6:	test   %al,%al
   73ad8:	lea    -0x65eaf(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73adf:	lea    -0x65d96(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73ae6:	cmove  %rcx,%rax
   73aea:	test   %r12b,%r12b
   73aed:	lea    -0x661c4(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73af4:	cmove  %rcx,%rax
   73af8:	mov    %rax,(%rsp)
   73afc:	lea    0x40(%rsp),%rdi
   73b01:	mov    0x38(%rsp),%rsi
   73b06:	mov    0x30(%rsp),%rdx
   73b0b:	mov    %r15,%rcx
   73b0e:	lea    0x1f8(%rsp),%r8
   73b16:	mov    0x50(%rsp),%r9
   73b1b:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   73b20:	cmpb   $0x0,0x40(%rsp)
   73b25:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73b2b:	mov    %ebx,%eax
   73b2d:	shr    $0x2,%al
   73b30:	and    $0x3,%al
   73b32:	cmp    $0x2,%al
   73b34:	movzbl 0x27(%rsp),%edx
   73b39:	je     73e1b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73b3f:	movzbl %al,%ecx
   73b42:	cmp    $0x3,%ecx
   73b45:	je     73e02 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73b4b:	test   %al,%al
   73b4d:	lea    -0x65f24(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73b54:	lea    -0x65e0b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b5b:	cmove  %rcx,%rax
   73b5f:	mov    0x48(%rsp),%rcx
   73b64:	test   %dl,%dl
   73b66:	lea    -0x6623d(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b6d:	cmove  %rdx,%rax
   73b71:	mov    %rax,(%rsp)
   73b75:	lea    0x40(%rsp),%rdi
   73b7a:	mov    0x38(%rsp),%rsi
   73b7f:	mov    0x30(%rsp),%rdx
   73b84:	mov    0x90(%rsp),%r8
   73b8c:	mov    0x50(%rsp),%r9
   73b91:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   73b96:	cmpb   $0x0,0x40(%rsp)
   73b9b:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73ba1:	mov    %ebx,%eax
   73ba3:	shr    $0x4,%al
   73ba6:	and    $0x3,%al
   73ba8:	cmp    $0x2,%al
   73baa:	movzbl 0x27(%rsp),%edx
   73baf:	je     73e1b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73bb5:	movzbl %al,%ecx
   73bb8:	cmp    $0x3,%ecx
   73bbb:	je     73e02 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73bc1:	test   %al,%al
   73bc3:	lea    -0x65f9a(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73bca:	lea    -0x65e81(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73bd1:	cmove  %rcx,%rax
   73bd5:	mov    0x48(%rsp),%rcx
   73bda:	test   %dl,%dl
   73bdc:	lea    -0x662b3(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73be3:	cmove  %rdx,%rax
   73be7:	mov    %rax,(%rsp)
   73beb:	lea    0x40(%rsp),%rdi
   73bf0:	mov    0x38(%rsp),%rsi
   73bf5:	mov    0x30(%rsp),%rdx
   73bfa:	mov    0x88(%rsp),%r8
   73c02:	mov    0x50(%rsp),%r9
   73c07:	call   d7620 <meshoptimizer_rs::codec::vertex::bytes>
   73c0c:	cmpb   $0x0,0x40(%rsp)
   73c11:	jne    73d14 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73c17:	shr    $0x6,%bl
   73c1a:	cmp    $0x2,%bl
   73c1d:	movzbl 0x27(%rsp),%edx
   73c22:	je     73e1b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73c28:	movzbl %bl,%eax
   73c2b:	cmp    $0x3,%eax
   73c2e:	je     73e02 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73c34:	test   %bl,%bl
   73c36:	lea    -0x6600d(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73c3d:	lea    -0x65ef4(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73c44:	cmove  %rcx,%rax
   73c48:	mov    0x48(%rsp),%rcx
   73c4d:	test   %dl,%dl
   73c4f:	lea    -0x66326(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73c56:	cmove  %rdx,%rax
   73c5a:	mov    %rax,(%rsp)
   73c5e:	lea    0x40(%rsp),%rdi
   73c63:	mov    0x38(%rsp),%rsi
   73c68:	mov    0x30(%rsp),%rdx
   73c6d:	jmp    7398c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x81c>
   73c72:	mov    0xb8(%rsp),%rdx
   73c7a:	mov    0xc8(%rsp),%rdi
   73c82:	cmp    %rdx,%rdi
   73c85:	ja     73db5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc45>
   73c8b:	mov    0x58(%rsp),%rsi
   73c90:	add    %rdi,%rsi
   73c93:	lea    0xf0(%rsp),%rdi
   73c9b:	mov    %r14,%rdx
   73c9e:	call   *0xb6b34(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   73ca4:	mov    0xd0(%rsp),%rcx
   73cac:	mov    %rcx,%rax
   73caf:	mov    0xc0(%rsp),%r13
   73cb7:	cmp    %r13,%rcx
   73cba:	mov    0x30(%rsp),%rbx
   73cbf:	mov    0x68(%rsp),%rbp
   73cc4:	jb     7341c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2ac>
   73cca:	jmp    73cf9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb89>
   73ccc:	mov    $0x1,%ecx
   73cd1:	mov    $0x1,%r8d
   73cd7:	mov    0x68(%rsp),%rbx
   73cdc:	mov    %rbx,%rdi
   73cdf:	mov    %r13,%rsi
   73ce2:	mov    %rbp,%rdx
   73ce5:	call   86230 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73cea:	mov    0x10(%rbx),%r13
   73cee:	jmp    734d9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   73cf3:	mov    $0x1,%r15d
   73cf9:	sub    %r15,%rbx
   73cfc:	setae  %cl
   73cff:	cmp    0xd8(%rsp),%rbx
   73d07:	sete   %dl
   73d0a:	mov    $0x7,%al
   73d0c:	test   %dl,%cl
   73d0e:	je     73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d10:	mov    $0xff,%al
   73d12:	jmp    73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d14:	movzbl 0x41(%rsp),%eax
   73d19:	add    $0x5f8,%rsp
   73d20:	pop    %rbx
   73d21:	pop    %r12
   73d23:	pop    %r13
   73d25:	pop    %r14
   73d27:	pop    %r15
   73d29:	pop    %rbp
   73d2a:	ret
   73d2b:	mov    $0x7,%al
   73d2d:	jmp    73d19 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d2f:	lea    0xb1702(%rip),%rcx        # 125438 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   73d36:	xor    %edi,%edi
   73d38:	mov    0x60(%rsp),%rsi
   73d3d:	call   *0xb6bed(%rip)        # 12a930 <_DYNAMIC+0x370>
   73d43:	mov    %rcx,%rdx
   73d46:	lea    0xb1703(%rip),%rcx        # 125450 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   73d4d:	mov    %rbp,%rdi
   73d50:	mov    %rdx,%rsi
   73d53:	call   *0xb6bd7(%rip)        # 12a930 <_DYNAMIC+0x370>
   73d59:	lea    0xb1750(%rip),%rcx        # 1254b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   73d60:	mov    $0x100,%edx
   73d65:	xor    %edi,%edi
   73d67:	mov    %r14,%rsi
   73d6a:	call   *0xb6bc0(%rip)        # 12a930 <_DYNAMIC+0x370>
   73d70:	lea    0xb1721(%rip),%rcx        # 125498 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   73d77:	mov    %rsi,%rdi
   73d7a:	mov    0x70(%rsp),%rsi
   73d7f:	mov    %rbx,%rdx
   73d82:	call   *0xb6ba8(%rip)        # 12a930 <_DYNAMIC+0x370>
   73d88:	lea    0xb4ff1(%rip),%rcx        # 128d80 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5b18>
   73d8f:	mov    0x58(%rsp),%rdi
   73d94:	mov    %r12,%rsi
   73d97:	call   *0xb6b93(%rip)        # 12a930 <_DYNAMIC+0x370>
   73d9d:	lea    0xb1724(%rip),%rcx        # 1254c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73da4:	mov    0x70(%rsp),%rdi
   73da9:	mov    %rbx,%rsi
   73dac:	mov    %rbx,%rdx
   73daf:	call   *0xb6b7b(%rip)        # 12a930 <_DYNAMIC+0x370>
   73db5:	lea    0xb1634(%rip),%rcx        # 1253f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   73dbc:	mov    %rdx,%rsi
   73dbf:	call   *0xb6b6b(%rip)        # 12a930 <_DYNAMIC+0x370>
   73dc5:	lea    0xb163c(%rip),%rdx        # 125408 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   73dcc:	mov    %r13,%rdi
   73dcf:	mov    0x98(%rsp),%rsi
   73dd7:	call   *0xb6a33(%rip)        # 12a810 <_DYNAMIC+0x250>
   73ddd:	lea    0xb163c(%rip),%rdx        # 125420 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73de4:	mov    %r13,%rdi
   73de7:	mov    0xa8(%rsp),%rsi
   73def:	call   *0xb6a1b(%rip)        # 12a810 <_DYNAMIC+0x250>
   73df5:	lea    0xb15dc(%rip),%rdi        # 1253d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   73dfc:	call   *0xb6bf6(%rip)        # 12a9f8 <_DYNAMIC+0x438>
   73e02:	lea    0xb165f(%rip),%rcx        # 125468 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73e09:	xor    %edi,%edi
   73e0b:	mov    0x28(%rsp),%rsi
   73e10:	mov    0x50(%rsp),%rdx
   73e15:	call   *0xb6b15(%rip)        # 12a930 <_DYNAMIC+0x370>
   73e1b:	lea    0xb165e(%rip),%rcx        # 125480 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73e22:	xor    %edi,%edi
   73e24:	mov    0x28(%rsp),%rsi
   73e29:	mov    0x50(%rsp),%rdx
   73e2e:	call   *0xb6afc(%rip)        # 12a930 <_DYNAMIC+0x370>
