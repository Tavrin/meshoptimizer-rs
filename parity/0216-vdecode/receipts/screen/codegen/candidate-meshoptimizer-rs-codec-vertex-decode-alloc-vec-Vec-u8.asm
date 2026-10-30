
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000731e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   731e0:	push   %rbp
   731e1:	push   %r15
   731e3:	push   %r14
   731e5:	push   %r13
   731e7:	push   %r12
   731e9:	push   %rbx
   731ea:	sub    $0x5f8,%rsp
   731f1:	mov    %r8,%rbx
   731f4:	mov    %rcx,%r12
   731f7:	mov    %rdx,%r14
   731fa:	mov    %rsi,%r13
   731fd:	mov    %rdi,%rbp
   73200:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73205:	cmp    $0x2,%al
   73207:	jae    73270 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x90>
   73209:	mov    $0x7,%al
   7320b:	test   %rbx,%rbx
   7320e:	je     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73214:	movzbl (%r12),%edx
   73219:	mov    %edx,%ecx
   7321b:	and    $0xf0,%cl
   7321e:	cmp    $0xa0,%cl
   73221:	jne    73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73227:	and    $0xf,%dl
   7322a:	mov    $0x5,%al
   7322c:	mov    %dl,0x27(%rsp)
   73230:	cmp    $0x2,%dl
   73233:	jae    73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73239:	cmpb   $0x0,0x27(%rsp)
   7323e:	je     73302 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x122>
   73244:	mov    %r14,%rcx
   73247:	shr    $0x2,%rcx
   7324b:	add    %r14,%rcx
   7324e:	cmp    $0x19,%rcx
   73252:	mov    $0x18,%r15d
   73258:	cmovae %rcx,%r15
   7325c:	lea    0x1(%r15),%rdx
   73260:	mov    $0x7,%al
   73262:	cmp    %rdx,%rbx
   73265:	jae    73322 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x142>
   7326b:	jmp    73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73270:	cmp    $0x21,%r13
   73274:	jae    73291 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb1>
   73276:	cmp    $0x4,%r14
   7327a:	je     732be <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xde>
   7327c:	cmp    $0xc,%r14
   73280:	je     732ac <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcc>
   73282:	cmp    $0x20,%r14
   73286:	jne    732d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf0>
   73288:	lea    0xdea1(%rip),%rax        # 81130 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   7328f:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73291:	cmp    $0x4,%r14
   73295:	je     732c7 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xe7>
   73297:	cmp    $0xc,%r14
   7329b:	je     732b5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xd5>
   7329d:	cmp    $0x20,%r14
   732a1:	jne    732d9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf9>
   732a3:	lea    0x7156(%rip),%rax        # 7a400 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   732aa:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732ac:	lea    0x1245d(%rip),%rax        # 85710 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, alloc::vec::Vec<u8>>>
   732b3:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732b5:	lea    0xb054(%rip),%rax        # 7e310 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, alloc::vec::Vec<u8>>>
   732bc:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732be:	lea    0xf64b(%rip),%rax        # 82910 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   732c5:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732c7:	lea    0x8562(%rip),%rax        # 7b830 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   732ce:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732d0:	lea    0xc4e9(%rip),%rax        # 7f7c0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   732d7:	jmp    732e0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   732d9:	lea    0x5bb0(%rip),%rax        # 78e90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   732e0:	mov    %rbp,%rdi
   732e3:	mov    %r13,%rsi
   732e6:	mov    %r14,%rdx
   732e9:	mov    %r12,%rcx
   732ec:	mov    %rbx,%r8
   732ef:	add    $0x5f8,%rsp
   732f6:	pop    %rbx
   732f7:	pop    %r12
   732f9:	pop    %r13
   732fb:	pop    %r14
   732fd:	pop    %r15
   732ff:	pop    %rbp
   73300:	jmp    *%rax
   73302:	cmp    $0x21,%r14
   73306:	mov    $0x20,%r15d
   7330c:	cmovae %r14,%r15
   73310:	mov    %r14,%rcx
   73313:	lea    0x1(%r15),%rdx
   73317:	mov    $0x7,%al
   73319:	cmp    %rdx,%rbx
   7331c:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73322:	mov    %rbx,%rsi
   73325:	sub    %rcx,%rsi
   73328:	lea    (%rsi,%r14,1),%rax
   7332c:	mov    %rbx,%rcx
   7332f:	mov    %rax,0x70(%rsp)
   73334:	sub    %rax,%rcx
   73337:	mov    %rcx,0xa8(%rsp)
   7333f:	jb     73e0d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2d>
   73345:	xorps  %xmm0,%xmm0
   73348:	movaps %xmm0,0x1e0(%rsp)
   73350:	movaps %xmm0,0x1d0(%rsp)
   73358:	movaps %xmm0,0x1c0(%rsp)
   73360:	movaps %xmm0,0x1b0(%rsp)
   73368:	movaps %xmm0,0x1a0(%rsp)
   73370:	movaps %xmm0,0x190(%rsp)
   73378:	movaps %xmm0,0x180(%rsp)
   73380:	movaps %xmm0,0x170(%rsp)
   73388:	movaps %xmm0,0x160(%rsp)
   73390:	movaps %xmm0,0x150(%rsp)
   73398:	movaps %xmm0,0x140(%rsp)
   733a0:	movaps %xmm0,0x130(%rsp)
   733a8:	movaps %xmm0,0x120(%rsp)
   733b0:	movaps %xmm0,0x110(%rsp)
   733b8:	movaps %xmm0,0x100(%rsp)
   733c0:	movaps %xmm0,0xf0(%rsp)
   733c8:	cmp    $0x101,%r14
   733cf:	jae    73dc9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbe9>
   733d5:	lea    (%rsi,%r14,1),%rax
   733d9:	cmp    %rsi,%rax
   733dc:	jb     73de0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc00>
   733e2:	add    %r12,%rsi
   733e5:	lea    0xf0(%rsp),%rdi
   733ed:	mov    %r14,%rdx
   733f0:	call   *0xb7392(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   733f6:	test   %r14,%r14
   733f9:	je     73e65 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc85>
   733ff:	mov    %r15,0xd8(%rsp)
   73407:	mov    $0x2000,%ax
   7340b:	xor    %edx,%edx
   7340d:	div    %r14w
   73411:	and    $0x3ff0,%eax
   73416:	cmp    $0x100,%eax
   7341b:	mov    $0x100,%ecx
   73420:	cmovb  %eax,%ecx
   73423:	mov    %rcx,0xe0(%rsp)
   7342b:	lea    0x1f8(%rsp),%rdi
   73433:	mov    $0x400,%edx
   73438:	xor    %esi,%esi
   7343a:	call   *0xb7358(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   73440:	test   %r13,%r13
   73443:	je     73d63 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb83>
   73449:	add    %r12,0x70(%rsp)
   7344e:	mov    %r14,%rax
   73451:	shr    $0x2,%rax
   73455:	mov    %rax,0xa0(%rsp)
   7345d:	lea    0x3(%r14),%rax
   73461:	shr    $0x2,%rax
   73465:	mov    %rax,0xe8(%rsp)
   7346d:	mov    $0x1,%r15d
   73473:	xor    %eax,%eax
   73475:	mov    %rbp,0x68(%rsp)
   7347a:	mov    %rbx,0x30(%rsp)
   7347f:	mov    %r12,0x38(%rsp)
   73484:	mov    %r13,0xc0(%rsp)
   7348c:	mov    %r13,%rdx
   7348f:	mov    %rax,0x58(%rsp)
   73494:	sub    %rax,%rdx
   73497:	mov    0xe0(%rsp),%rax
   7349f:	cmp    %rax,%rdx
   734a2:	cmovae %rax,%rdx
   734a6:	mov    %rdx,0x28(%rsp)
   734ab:	cmpb   $0x0,0x27(%rsp)
   734b0:	je     734f5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x315>
   734b2:	mov    %r15,%rcx
   734b5:	mov    $0x7,%al
   734b7:	add    0xa0(%rsp),%rcx
   734bf:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   734c5:	cmp    %rbx,%rcx
   734c8:	ja     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   734ce:	add    0x38(%rsp),%r15
   734d3:	mov    0xa0(%rsp),%rax
   734db:	mov    %rax,0x98(%rsp)
   734e3:	mov    %r15,0xb0(%rsp)
   734eb:	mov    %rcx,%r15
   734ee:	mov    0x68(%rsp),%rbp
   734f3:	jmp    7350e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x32e>
   734f5:	movq   $0x0,0x98(%rsp)
   73501:	mov    $0x1,%eax
   73506:	mov    %rax,0xb0(%rsp)
   7350e:	mov    0x58(%rsp),%rax
   73513:	mov    0x28(%rsp),%rcx
   73518:	lea    (%rcx,%rax,1),%r12
   7351c:	mov    %r12,0xd0(%rsp)
   73524:	imul   %r14,%r12
   73528:	mov    0x10(%rbp),%r13
   7352c:	mov    %rbp,%rax
   7352f:	mov    %r12,%rbp
   73532:	mov    %r12,%rdx
   73535:	sub    %r13,%rbp
   73538:	jbe    73582 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x3a2>
   7353a:	mov    (%rax),%rax
   7353d:	sub    %r13,%rax
   73540:	cmp    %rax,%rbp
   73543:	ja     73d3c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5c>
   73549:	mov    0x68(%rsp),%rax
   7354e:	mov    0x8(%rax),%rbx
   73552:	lea    (%rbx,%r13,1),%rdi
   73556:	cmp    $0x2,%rbp
   7355a:	jb     73574 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x394>
   7355c:	lea    -0x1(%rbp),%rdx
   73560:	xor    %esi,%esi
   73562:	call   *0xb7230(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   73568:	add    %rbp,%r13
   7356b:	dec    %r13
   7356e:	add    %r13,%rbx
   73571:	mov    %rbx,%rdi
   73574:	movb   $0x0,(%rdi)
   73577:	inc    %r13
   7357a:	mov    %r13,%rdx
   7357d:	mov    0x68(%rsp),%rax
   73582:	mov    0x58(%rsp),%rcx
   73587:	imul   %r14,%rcx
   7358b:	mov    %rdx,0x10(%rax)
   7358f:	mov    %rcx,0x58(%rsp)
   73594:	cmp    %rcx,%r12
   73597:	jb     73df8 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   7359d:	cmp    %rdx,%r12
   735a0:	ja     73df8 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   735a6:	mov    0x28(%rsp),%rdx
   735ab:	lea    0xf(%rdx),%ecx
   735ae:	and    $0x3f0,%ecx
   735b4:	mov    %rcx,0x50(%rsp)
   735b9:	mov    %rdx,%rcx
   735bc:	imul   %r14,%rcx
   735c0:	mov    %rcx,0xb8(%rsp)
   735c8:	mov    0x58(%rsp),%rcx
   735cd:	add    0x8(%rax),%rcx
   735d1:	mov    %rcx,0x58(%rsp)
   735d6:	lea    -0x1(%rdx),%rax
   735da:	imul   %r14,%rax
   735de:	mov    %rax,0xc8(%rsp)
   735e6:	add    $0x4,%rax
   735ea:	mov    %rax,0x60(%rsp)
   735ef:	lea    0x0(,%rdx,4),%rax
   735f7:	mov    %rax,0x78(%rsp)
   735fc:	lea    0x1f8(%rsp,%rdx,1),%rax
   73604:	mov    %rax,0x90(%rsp)
   7360c:	lea    0x1f8(%rsp,%rdx,2),%rax
   73614:	mov    %rax,0x88(%rsp)
   7361c:	lea    (%rdx,%rdx,2),%rax
   73620:	add    %rsp,%rax
   73623:	add    $0x1f8,%rax
   73629:	mov    %rax,0x80(%rsp)
   73631:	xor    %r13d,%r13d
   73634:	xor    %ebp,%ebp
   73636:	movzbl 0x27(%rsp),%r12d
   7363c:	jmp    73698 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4b8>
   7363e:	lea    (%rsp,%rbp,1),%rcx
   73642:	add    $0xf0,%rcx
   73649:	shr    $0x4,%al
   7364c:	neg    %al
   7364e:	and    $0x1f,%al
   73650:	movzbl %al,%eax
   73653:	mov    %rcx,(%rsp)
   73657:	mov    %eax,0x8(%rsp)
   7365b:	movq   $0x0,0x10(%rsp)
   73664:	lea    0x1f8(%rsp),%rdi
   7366c:	mov    0x78(%rsp),%rsi
   73671:	mov    0x60(%rsp),%rcx
   73676:	mov    0x28(%rsp),%r8
   7367b:	mov    %r14,%r9
   7367e:	call   73070 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   73683:	inc    %r13
   73686:	add    $0x4,%rbp
   7368a:	cmp    %r13,0xe8(%rsp)
   73692:	je     73ce2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb02>
   73698:	test   %r12b,%r12b
   7369b:	je     736ba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4da>
   7369d:	cmp    0x98(%rsp),%r13
   736a5:	jae    73e35 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc55>
   736ab:	mov    0xb0(%rsp),%rax
   736b3:	movzbl (%rax,%r13,1),%ebx
   736b8:	jmp    736bc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4dc>
   736ba:	xor    %ebx,%ebx
   736bc:	mov    %ebx,%eax
   736be:	and    $0x3,%al
   736c0:	movzbl %al,%ecx
   736c3:	mov    0x50(%rsp),%rdx
   736c8:	cmp    %rdx,0x28(%rsp)
   736cd:	ja     73b35 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x955>
   736d3:	cmp    $0x2,%al
   736d5:	je     73714 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x534>
   736d7:	cmp    $0x3,%ecx
   736da:	jne    7372e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x54e>
   736dc:	mov    %r15,%r12
   736df:	mov    $0x7,%al
   736e1:	add    0x28(%rsp),%r12
   736e6:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   736ec:	cmp    0x30(%rsp),%r12
   736f1:	ja     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   736f7:	add    0x38(%rsp),%r15
   736fc:	lea    0x1f8(%rsp),%rdi
   73704:	mov    %r15,%rsi
   73707:	mov    0x28(%rsp),%rdx
   7370c:	call   *0xb7076(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   73712:	jmp    73788 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   73714:	lea    0x1f8(%rsp),%rdi
   7371c:	xor    %esi,%esi
   7371e:	mov    0x28(%rsp),%rdx
   73723:	call   *0xb706f(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   73729:	mov    %r15,%r12
   7372c:	jmp    73788 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   7372e:	test   %al,%al
   73730:	lea    -0x65b07(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73737:	lea    -0x659ee(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7373e:	cmove  %rcx,%rax
   73742:	test   %r12b,%r12b
   73745:	lea    -0x65e1c(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7374c:	cmove  %rcx,%rax
   73750:	mov    %rax,(%rsp)
   73754:	lea    0x40(%rsp),%rdi
   73759:	mov    0x38(%rsp),%rsi
   7375e:	mov    0x30(%rsp),%rdx
   73763:	mov    %r15,%rcx
   73766:	lea    0x1f8(%rsp),%r8
   7376e:	mov    0x50(%rsp),%r9
   73773:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73778:	cmpb   $0x0,0x40(%rsp)
   7377d:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73783:	mov    0x48(%rsp),%r12
   73788:	mov    %ebx,%eax
   7378a:	shr    $0x2,%al
   7378d:	and    $0x3,%al
   7378f:	cmp    $0x3,%al
   73791:	je     737b5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5d5>
   73793:	movzbl %al,%ecx
   73796:	cmp    $0x2,%ecx
   73799:	jne    737f0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x610>
   7379b:	mov    0x90(%rsp),%rdi
   737a3:	xor    %esi,%esi
   737a5:	mov    0x28(%rsp),%rdx
   737aa:	call   *0xb6fe8(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   737b0:	jmp    7384c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   737b5:	mov    %r12,%r15
   737b8:	mov    $0x7,%al
   737ba:	add    0x28(%rsp),%r15
   737bf:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   737c5:	cmp    0x30(%rsp),%r15
   737ca:	ja     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   737d0:	add    0x38(%rsp),%r12
   737d5:	mov    0x90(%rsp),%rdi
   737dd:	mov    %r12,%rsi
   737e0:	mov    0x28(%rsp),%rdx
   737e5:	call   *0xb6f9d(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   737eb:	mov    %r15,%r12
   737ee:	jmp    7384c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   737f0:	test   %al,%al
   737f2:	lea    -0x65bc9(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   737f9:	lea    -0x65ab0(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73800:	cmove  %rcx,%rax
   73804:	cmpb   $0x0,0x27(%rsp)
   73809:	lea    -0x65ee0(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73810:	cmove  %rcx,%rax
   73814:	mov    %rax,(%rsp)
   73818:	lea    0x40(%rsp),%rdi
   7381d:	mov    0x38(%rsp),%rsi
   73822:	mov    0x30(%rsp),%rdx
   73827:	mov    %r12,%rcx
   7382a:	mov    0x90(%rsp),%r8
   73832:	mov    0x50(%rsp),%r9
   73837:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   7383c:	cmpb   $0x0,0x40(%rsp)
   73841:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73847:	mov    0x48(%rsp),%r12
   7384c:	mov    %ebx,%eax
   7384e:	shr    $0x4,%al
   73851:	and    $0x3,%al
   73853:	cmp    $0x3,%al
   73855:	je     73885 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6a5>
   73857:	movzbl %al,%ecx
   7385a:	cmp    $0x2,%ecx
   7385d:	jne    7390a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x72a>
   73863:	mov    0x88(%rsp),%rdi
   7386b:	xor    %esi,%esi
   7386d:	mov    0x28(%rsp),%rdx
   73872:	call   *0xb6f20(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   73878:	shr    $0x6,%bl
   7387b:	cmp    $0x3,%bl
   7387e:	jne    738ca <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73880:	jmp    73972 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   73885:	mov    %r12,%r15
   73888:	mov    $0x7,%al
   7388a:	add    0x28(%rsp),%r15
   7388f:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73895:	cmp    0x30(%rsp),%r15
   7389a:	ja     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   738a0:	add    0x38(%rsp),%r12
   738a5:	mov    0x88(%rsp),%rdi
   738ad:	mov    %r12,%rsi
   738b0:	mov    0x28(%rsp),%rdx
   738b5:	call   *0xb6ecd(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   738bb:	mov    %r15,%r12
   738be:	shr    $0x6,%bl
   738c1:	cmp    $0x3,%bl
   738c4:	je     73972 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   738ca:	movzbl %bl,%eax
   738cd:	cmp    $0x2,%eax
   738d0:	jne    739c2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7e2>
   738d6:	mov    0x80(%rsp),%rdi
   738de:	xor    %esi,%esi
   738e0:	mov    0x28(%rsp),%rdx
   738e5:	call   *0xb6ead(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   738eb:	mov    %r12,%r15
   738ee:	movzbl 0x27(%rsp),%r12d
   738f4:	test   %r12b,%r12b
   738f7:	mov    0xb8(%rsp),%rcx
   738ff:	jne    73a31 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   73905:	jmp    739bb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   7390a:	test   %al,%al
   7390c:	lea    -0x65ce3(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73913:	lea    -0x65bca(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7391a:	cmove  %rcx,%rax
   7391e:	cmpb   $0x0,0x27(%rsp)
   73923:	lea    -0x65ffa(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7392a:	cmove  %rcx,%rax
   7392e:	mov    %rax,(%rsp)
   73932:	lea    0x40(%rsp),%rdi
   73937:	mov    0x38(%rsp),%rsi
   7393c:	mov    0x30(%rsp),%rdx
   73941:	mov    %r12,%rcx
   73944:	mov    0x88(%rsp),%r8
   7394c:	mov    0x50(%rsp),%r9
   73951:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73956:	cmpb   $0x0,0x40(%rsp)
   7395b:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73961:	mov    0x48(%rsp),%r12
   73966:	shr    $0x6,%bl
   73969:	cmp    $0x3,%bl
   7396c:	jne    738ca <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73972:	mov    %r12,%r15
   73975:	mov    $0x7,%al
   73977:	add    0x28(%rsp),%r15
   7397c:	jb     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73982:	cmp    0x30(%rsp),%r15
   73987:	ja     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7398d:	add    0x38(%rsp),%r12
   73992:	mov    0x80(%rsp),%rdi
   7399a:	mov    %r12,%rsi
   7399d:	mov    0x28(%rsp),%rdx
   739a2:	call   *0xb6de0(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   739a8:	movzbl 0x27(%rsp),%r12d
   739ae:	test   %r12b,%r12b
   739b1:	mov    0xb8(%rsp),%rcx
   739b9:	jne    73a31 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   739bb:	xor    %eax,%eax
   739bd:	jmp    73a49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x869>
   739c2:	test   %bl,%bl
   739c4:	lea    -0x65d9b(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   739cb:	lea    -0x65c82(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   739d2:	cmove  %rcx,%rax
   739d6:	cmpb   $0x0,0x27(%rsp)
   739db:	lea    -0x660b2(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   739e2:	cmove  %rcx,%rax
   739e6:	mov    %rax,(%rsp)
   739ea:	lea    0x40(%rsp),%rdi
   739ef:	mov    0x38(%rsp),%rsi
   739f4:	mov    0x30(%rsp),%rdx
   739f9:	mov    %r12,%rcx
   739fc:	mov    0x80(%rsp),%r8
   73a04:	mov    0x50(%rsp),%r9
   73a09:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73a0e:	cmpb   $0x0,0x40(%rsp)
   73a13:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73a19:	mov    0x48(%rsp),%r15
   73a1e:	movzbl 0x27(%rsp),%r12d
   73a24:	test   %r12b,%r12b
   73a27:	mov    0xb8(%rsp),%rcx
   73a2f:	je     739bb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   73a31:	cmp    0xa8(%rsp),%r13
   73a39:	jae    73e4d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc6d>
   73a3f:	mov    0x70(%rsp),%rax
   73a44:	movzbl (%rax,%r13,1),%eax
   73a49:	mov    %rcx,%rdx
   73a4c:	sub    %rbp,%rdx
   73a4f:	jb     73db3 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd3>
   73a55:	cmp    %rdx,0x60(%rsp)
   73a5a:	ja     73d9f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbbf>
   73a60:	mov    0x58(%rsp),%rcx
   73a65:	lea    (%rcx,%rbp,1),%rdx
   73a69:	mov    %eax,%ecx
   73a6b:	and    $0x3,%cl
   73a6e:	movzbl %cl,%ecx
   73a71:	lea    -0x632d0(%rip),%rsi        # 107a8 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   73a78:	movslq (%rsi,%rcx,4),%rcx
   73a7c:	add    %rsi,%rcx
   73a7f:	jmp    *%rcx
   73a81:	mov    %rdx,%r12
   73a84:	lea    (%rsp,%rbp,1),%rbx
   73a88:	add    $0xf0,%rbx
   73a8f:	call   e5e30 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73a94:	test   %al,%al
   73a96:	je     73afb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91b>
   73a98:	mov    %rbx,(%rsp)
   73a9c:	lea    0x1f8(%rsp),%rdi
   73aa4:	mov    0x78(%rsp),%rsi
   73aa9:	mov    %r12,%rdx
   73aac:	mov    0x60(%rsp),%rcx
   73ab1:	mov    0x28(%rsp),%r8
   73ab6:	mov    %r14,%r9
   73ab9:	call   76a10 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   73abe:	jmp    73b2a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x94a>
   73ac0:	lea    (%rsp,%rbp,1),%rax
   73ac4:	add    $0xf0,%rax
   73aca:	mov    %rax,(%rsp)
   73ace:	movq   $0x0,0x8(%rsp)
   73ad7:	lea    0x1f8(%rsp),%rdi
   73adf:	mov    0x78(%rsp),%rsi
   73ae4:	mov    0x60(%rsp),%rcx
   73ae9:	mov    0x28(%rsp),%r8
   73aee:	mov    %r14,%r9
   73af1:	call   72eb0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   73af6:	jmp    73683 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   73afb:	mov    %rbx,(%rsp)
   73aff:	movq   $0x0,0x8(%rsp)
   73b08:	lea    0x1f8(%rsp),%rdi
   73b10:	mov    0x78(%rsp),%rsi
   73b15:	mov    %r12,%rdx
   73b18:	mov    0x60(%rsp),%rcx
   73b1d:	mov    0x28(%rsp),%r8
   73b22:	mov    %r14,%r9
   73b25:	call   72cc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   73b2a:	movzbl 0x27(%rsp),%r12d
   73b30:	jmp    73683 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   73b35:	cmp    $0x2,%al
   73b37:	je     73e8b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73b3d:	cmp    $0x3,%ecx
   73b40:	je     73e72 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73b46:	test   %al,%al
   73b48:	lea    -0x65f1f(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73b4f:	lea    -0x65e06(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b56:	cmove  %rcx,%rax
   73b5a:	test   %r12b,%r12b
   73b5d:	lea    -0x66234(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b64:	cmove  %rcx,%rax
   73b68:	mov    %rax,(%rsp)
   73b6c:	lea    0x40(%rsp),%rdi
   73b71:	mov    0x38(%rsp),%rsi
   73b76:	mov    0x30(%rsp),%rdx
   73b7b:	mov    %r15,%rcx
   73b7e:	lea    0x1f8(%rsp),%r8
   73b86:	mov    0x50(%rsp),%r9
   73b8b:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73b90:	cmpb   $0x0,0x40(%rsp)
   73b95:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73b9b:	mov    %ebx,%eax
   73b9d:	shr    $0x2,%al
   73ba0:	and    $0x3,%al
   73ba2:	cmp    $0x2,%al
   73ba4:	movzbl 0x27(%rsp),%edx
   73ba9:	je     73e8b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73baf:	movzbl %al,%ecx
   73bb2:	cmp    $0x3,%ecx
   73bb5:	je     73e72 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73bbb:	test   %al,%al
   73bbd:	lea    -0x65f94(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73bc4:	lea    -0x65e7b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73bcb:	cmove  %rcx,%rax
   73bcf:	mov    0x48(%rsp),%rcx
   73bd4:	test   %dl,%dl
   73bd6:	lea    -0x662ad(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73bdd:	cmove  %rdx,%rax
   73be1:	mov    %rax,(%rsp)
   73be5:	lea    0x40(%rsp),%rdi
   73bea:	mov    0x38(%rsp),%rsi
   73bef:	mov    0x30(%rsp),%rdx
   73bf4:	mov    0x90(%rsp),%r8
   73bfc:	mov    0x50(%rsp),%r9
   73c01:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73c06:	cmpb   $0x0,0x40(%rsp)
   73c0b:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73c11:	mov    %ebx,%eax
   73c13:	shr    $0x4,%al
   73c16:	and    $0x3,%al
   73c18:	cmp    $0x2,%al
   73c1a:	movzbl 0x27(%rsp),%edx
   73c1f:	je     73e8b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73c25:	movzbl %al,%ecx
   73c28:	cmp    $0x3,%ecx
   73c2b:	je     73e72 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73c31:	test   %al,%al
   73c33:	lea    -0x6600a(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73c3a:	lea    -0x65ef1(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73c41:	cmove  %rcx,%rax
   73c45:	mov    0x48(%rsp),%rcx
   73c4a:	test   %dl,%dl
   73c4c:	lea    -0x66323(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73c53:	cmove  %rdx,%rax
   73c57:	mov    %rax,(%rsp)
   73c5b:	lea    0x40(%rsp),%rdi
   73c60:	mov    0x38(%rsp),%rsi
   73c65:	mov    0x30(%rsp),%rdx
   73c6a:	mov    0x88(%rsp),%r8
   73c72:	mov    0x50(%rsp),%r9
   73c77:	call   d75d0 <meshoptimizer_rs::codec::vertex::bytes>
   73c7c:	cmpb   $0x0,0x40(%rsp)
   73c81:	jne    73d84 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73c87:	shr    $0x6,%bl
   73c8a:	cmp    $0x2,%bl
   73c8d:	movzbl 0x27(%rsp),%edx
   73c92:	je     73e8b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73c98:	movzbl %bl,%eax
   73c9b:	cmp    $0x3,%eax
   73c9e:	je     73e72 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73ca4:	test   %bl,%bl
   73ca6:	lea    -0x6607d(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73cad:	lea    -0x65f64(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73cb4:	cmove  %rcx,%rax
   73cb8:	mov    0x48(%rsp),%rcx
   73cbd:	test   %dl,%dl
   73cbf:	lea    -0x66396(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73cc6:	cmove  %rdx,%rax
   73cca:	mov    %rax,(%rsp)
   73cce:	lea    0x40(%rsp),%rdi
   73cd3:	mov    0x38(%rsp),%rsi
   73cd8:	mov    0x30(%rsp),%rdx
   73cdd:	jmp    739fc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x81c>
   73ce2:	mov    0xb8(%rsp),%rdx
   73cea:	mov    0xc8(%rsp),%rdi
   73cf2:	cmp    %rdx,%rdi
   73cf5:	ja     73e25 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc45>
   73cfb:	mov    0x58(%rsp),%rsi
   73d00:	add    %rdi,%rsi
   73d03:	lea    0xf0(%rsp),%rdi
   73d0b:	mov    %r14,%rdx
   73d0e:	call   *0xb6a74(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   73d14:	mov    0xd0(%rsp),%rcx
   73d1c:	mov    %rcx,%rax
   73d1f:	mov    0xc0(%rsp),%r13
   73d27:	cmp    %r13,%rcx
   73d2a:	mov    0x30(%rsp),%rbx
   73d2f:	mov    0x68(%rsp),%rbp
   73d34:	jb     7348c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2ac>
   73d3a:	jmp    73d69 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb89>
   73d3c:	mov    $0x1,%ecx
   73d41:	mov    $0x1,%r8d
   73d47:	mov    0x68(%rsp),%rbx
   73d4c:	mov    %rbx,%rdi
   73d4f:	mov    %r13,%rsi
   73d52:	mov    %rbp,%rdx
   73d55:	call   86eb0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73d5a:	mov    0x10(%rbx),%r13
   73d5e:	jmp    73549 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   73d63:	mov    $0x1,%r15d
   73d69:	sub    %r15,%rbx
   73d6c:	setae  %cl
   73d6f:	cmp    0xd8(%rsp),%rbx
   73d77:	sete   %dl
   73d7a:	mov    $0x7,%al
   73d7c:	test   %dl,%cl
   73d7e:	je     73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d80:	mov    $0xff,%al
   73d82:	jmp    73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d84:	movzbl 0x41(%rsp),%eax
   73d89:	add    $0x5f8,%rsp
   73d90:	pop    %rbx
   73d91:	pop    %r12
   73d93:	pop    %r13
   73d95:	pop    %r14
   73d97:	pop    %r15
   73d99:	pop    %rbp
   73d9a:	ret
   73d9b:	mov    $0x7,%al
   73d9d:	jmp    73d89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73d9f:	lea    0xb1642(%rip),%rcx        # 1253e8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   73da6:	xor    %edi,%edi
   73da8:	mov    0x60(%rsp),%rsi
   73dad:	call   *0xb6b2d(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73db3:	mov    %rcx,%rdx
   73db6:	lea    0xb1643(%rip),%rcx        # 125400 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   73dbd:	mov    %rbp,%rdi
   73dc0:	mov    %rdx,%rsi
   73dc3:	call   *0xb6b17(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73dc9:	lea    0xb1690(%rip),%rcx        # 125460 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   73dd0:	mov    $0x100,%edx
   73dd5:	xor    %edi,%edi
   73dd7:	mov    %r14,%rsi
   73dda:	call   *0xb6b00(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73de0:	lea    0xb1661(%rip),%rcx        # 125448 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   73de7:	mov    %rsi,%rdi
   73dea:	mov    0x70(%rsp),%rsi
   73def:	mov    %rbx,%rdx
   73df2:	call   *0xb6ae8(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73df8:	lea    0xb4f31(%rip),%rcx        # 128d30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5b18>
   73dff:	mov    0x58(%rsp),%rdi
   73e04:	mov    %r12,%rsi
   73e07:	call   *0xb6ad3(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73e0d:	lea    0xb1664(%rip),%rcx        # 125478 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73e14:	mov    0x70(%rsp),%rdi
   73e19:	mov    %rbx,%rsi
   73e1c:	mov    %rbx,%rdx
   73e1f:	call   *0xb6abb(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73e25:	lea    0xb1574(%rip),%rcx        # 1253a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   73e2c:	mov    %rdx,%rsi
   73e2f:	call   *0xb6aab(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73e35:	lea    0xb157c(%rip),%rdx        # 1253b8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   73e3c:	mov    %r13,%rdi
   73e3f:	mov    0x98(%rsp),%rsi
   73e47:	call   *0xb6973(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   73e4d:	lea    0xb157c(%rip),%rdx        # 1253d0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73e54:	mov    %r13,%rdi
   73e57:	mov    0xa8(%rsp),%rsi
   73e5f:	call   *0xb695b(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   73e65:	lea    0xb151c(%rip),%rdi        # 125388 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   73e6c:	call   *0xb6b36(%rip)        # 12a9a8 <_DYNAMIC+0x438>
   73e72:	lea    0xb159f(%rip),%rcx        # 125418 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73e79:	xor    %edi,%edi
   73e7b:	mov    0x28(%rsp),%rsi
   73e80:	mov    0x50(%rsp),%rdx
   73e85:	call   *0xb6a55(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   73e8b:	lea    0xb159e(%rip),%rcx        # 125430 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73e92:	xor    %edi,%edi
   73e94:	mov    0x28(%rsp),%rsi
   73e99:	mov    0x50(%rsp),%rdx
   73e9e:	call   *0xb6a3c(%rip)        # 12a8e0 <_DYNAMIC+0x370>
