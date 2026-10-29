
<artifacts>/screen2/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000730a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   730a0:	push   %rbp
   730a1:	push   %r15
   730a3:	push   %r14
   730a5:	push   %r13
   730a7:	push   %r12
   730a9:	push   %rbx
   730aa:	sub    $0x5f8,%rsp
   730b1:	mov    %r8,%rbx
   730b4:	mov    %rcx,%r12
   730b7:	mov    %rdx,%r14
   730ba:	mov    %rsi,%r13
   730bd:	mov    %rdi,%rbp
   730c0:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   730c5:	cmp    $0x2,%al
   730c7:	jae    73130 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x90>
   730c9:	mov    $0x7,%al
   730cb:	test   %rbx,%rbx
   730ce:	je     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   730d4:	movzbl (%r12),%edx
   730d9:	mov    %edx,%ecx
   730db:	and    $0xf0,%cl
   730de:	cmp    $0xa0,%cl
   730e1:	jne    73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   730e7:	and    $0xf,%dl
   730ea:	mov    $0x5,%al
   730ec:	mov    %dl,0x27(%rsp)
   730f0:	cmp    $0x2,%dl
   730f3:	jae    73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   730f9:	cmpb   $0x0,0x27(%rsp)
   730fe:	je     731c2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x122>
   73104:	mov    %r14,%rcx
   73107:	shr    $0x2,%rcx
   7310b:	add    %r14,%rcx
   7310e:	cmp    $0x19,%rcx
   73112:	mov    $0x18,%r15d
   73118:	cmovae %rcx,%r15
   7311c:	lea    0x1(%r15),%rdx
   73120:	mov    $0x7,%al
   73122:	cmp    %rdx,%rbx
   73125:	jae    731e2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x142>
   7312b:	jmp    73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73130:	cmp    $0x21,%r13
   73134:	jae    73151 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb1>
   73136:	cmp    $0x4,%r14
   7313a:	je     7317e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xde>
   7313c:	cmp    $0xc,%r14
   73140:	je     7316c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcc>
   73142:	cmp    $0x20,%r14
   73146:	jne    73190 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf0>
   73148:	lea    0xc881(%rip),%rax        # 7f9d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   7314f:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73151:	cmp    $0x4,%r14
   73155:	je     73187 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xe7>
   73157:	cmp    $0xc,%r14
   7315b:	je     73175 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xd5>
   7315d:	cmp    $0x20,%r14
   73161:	jne    73199 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xf9>
   73163:	lea    0x6546(%rip),%rax        # 796b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   7316a:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   7316c:	lea    0x10e3d(%rip),%rax        # 83fb0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 12, alloc::vec::Vec<u8>>>
   73173:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73175:	lea    0xa444(%rip),%rax        # 7d5c0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 12, alloc::vec::Vec<u8>>>
   7317c:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   7317e:	lea    0xe02b(%rip),%rax        # 811b0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   73185:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73187:	lea    0x7952(%rip),%rax        # 7aae0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   7318e:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73190:	lea    0xaec9(%rip),%rax        # 7e060 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   73197:	jmp    731a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x100>
   73199:	lea    0x4fa0(%rip),%rax        # 78140 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   731a0:	mov    %rbp,%rdi
   731a3:	mov    %r13,%rsi
   731a6:	mov    %r14,%rdx
   731a9:	mov    %r12,%rcx
   731ac:	mov    %rbx,%r8
   731af:	add    $0x5f8,%rsp
   731b6:	pop    %rbx
   731b7:	pop    %r12
   731b9:	pop    %r13
   731bb:	pop    %r14
   731bd:	pop    %r15
   731bf:	pop    %rbp
   731c0:	jmp    *%rax
   731c2:	cmp    $0x21,%r14
   731c6:	mov    $0x20,%r15d
   731cc:	cmovae %r14,%r15
   731d0:	mov    %r14,%rcx
   731d3:	lea    0x1(%r15),%rdx
   731d7:	mov    $0x7,%al
   731d9:	cmp    %rdx,%rbx
   731dc:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   731e2:	mov    %rbx,%rsi
   731e5:	sub    %rcx,%rsi
   731e8:	lea    (%rsi,%r14,1),%rax
   731ec:	mov    %rbx,%rcx
   731ef:	mov    %rax,0x70(%rsp)
   731f4:	sub    %rax,%rcx
   731f7:	mov    %rcx,0xa8(%rsp)
   731ff:	jb     73ccd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2d>
   73205:	xorps  %xmm0,%xmm0
   73208:	movaps %xmm0,0x1e0(%rsp)
   73210:	movaps %xmm0,0x1d0(%rsp)
   73218:	movaps %xmm0,0x1c0(%rsp)
   73220:	movaps %xmm0,0x1b0(%rsp)
   73228:	movaps %xmm0,0x1a0(%rsp)
   73230:	movaps %xmm0,0x190(%rsp)
   73238:	movaps %xmm0,0x180(%rsp)
   73240:	movaps %xmm0,0x170(%rsp)
   73248:	movaps %xmm0,0x160(%rsp)
   73250:	movaps %xmm0,0x150(%rsp)
   73258:	movaps %xmm0,0x140(%rsp)
   73260:	movaps %xmm0,0x130(%rsp)
   73268:	movaps %xmm0,0x120(%rsp)
   73270:	movaps %xmm0,0x110(%rsp)
   73278:	movaps %xmm0,0x100(%rsp)
   73280:	movaps %xmm0,0xf0(%rsp)
   73288:	cmp    $0x101,%r14
   7328f:	jae    73c89 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbe9>
   73295:	lea    (%rsi,%r14,1),%rax
   73299:	cmp    %rsi,%rax
   7329c:	jb     73ca0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc00>
   732a2:	add    %r12,%rsi
   732a5:	lea    0xf0(%rsp),%rdi
   732ad:	mov    %r14,%rdx
   732b0:	call   *0xb5ea2(%rip)        # 129158 <memcpy@GLIBC_2.14>
   732b6:	test   %r14,%r14
   732b9:	je     73d25 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc85>
   732bf:	mov    %r15,0xd8(%rsp)
   732c7:	mov    $0x2000,%ax
   732cb:	xor    %edx,%edx
   732cd:	div    %r14w
   732d1:	and    $0x3ff0,%eax
   732d6:	cmp    $0x100,%eax
   732db:	mov    $0x100,%ecx
   732e0:	cmovb  %eax,%ecx
   732e3:	mov    %rcx,0xe0(%rsp)
   732eb:	lea    0x1f8(%rsp),%rdi
   732f3:	mov    $0x400,%edx
   732f8:	xor    %esi,%esi
   732fa:	call   *0xb5e68(%rip)        # 129168 <memset@GLIBC_2.2.5>
   73300:	test   %r13,%r13
   73303:	je     73c23 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb83>
   73309:	add    %r12,0x70(%rsp)
   7330e:	mov    %r14,%rax
   73311:	shr    $0x2,%rax
   73315:	mov    %rax,0xa0(%rsp)
   7331d:	lea    0x3(%r14),%rax
   73321:	shr    $0x2,%rax
   73325:	mov    %rax,0xe8(%rsp)
   7332d:	mov    $0x1,%r15d
   73333:	xor    %eax,%eax
   73335:	mov    %rbp,0x68(%rsp)
   7333a:	mov    %rbx,0x30(%rsp)
   7333f:	mov    %r12,0x38(%rsp)
   73344:	mov    %r13,0xc0(%rsp)
   7334c:	mov    %r13,%rdx
   7334f:	mov    %rax,0x58(%rsp)
   73354:	sub    %rax,%rdx
   73357:	mov    0xe0(%rsp),%rax
   7335f:	cmp    %rax,%rdx
   73362:	cmovae %rax,%rdx
   73366:	mov    %rdx,0x28(%rsp)
   7336b:	cmpb   $0x0,0x27(%rsp)
   73370:	je     733b5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x315>
   73372:	mov    %r15,%rcx
   73375:	mov    $0x7,%al
   73377:	add    0xa0(%rsp),%rcx
   7337f:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73385:	cmp    %rbx,%rcx
   73388:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7338e:	add    0x38(%rsp),%r15
   73393:	mov    0xa0(%rsp),%rax
   7339b:	mov    %rax,0x98(%rsp)
   733a3:	mov    %r15,0xb0(%rsp)
   733ab:	mov    %rcx,%r15
   733ae:	mov    0x68(%rsp),%rbp
   733b3:	jmp    733ce <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x32e>
   733b5:	movq   $0x0,0x98(%rsp)
   733c1:	mov    $0x1,%eax
   733c6:	mov    %rax,0xb0(%rsp)
   733ce:	mov    0x58(%rsp),%rax
   733d3:	mov    0x28(%rsp),%rcx
   733d8:	lea    (%rcx,%rax,1),%r12
   733dc:	mov    %r12,0xd0(%rsp)
   733e4:	imul   %r14,%r12
   733e8:	mov    0x10(%rbp),%r13
   733ec:	mov    %rbp,%rax
   733ef:	mov    %r12,%rbp
   733f2:	mov    %r12,%rdx
   733f5:	sub    %r13,%rbp
   733f8:	jbe    73442 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x3a2>
   733fa:	mov    (%rax),%rax
   733fd:	sub    %r13,%rax
   73400:	cmp    %rax,%rbp
   73403:	ja     73bfc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5c>
   73409:	mov    0x68(%rsp),%rax
   7340e:	mov    0x8(%rax),%rbx
   73412:	lea    (%rbx,%r13,1),%rdi
   73416:	cmp    $0x2,%rbp
   7341a:	jb     73434 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x394>
   7341c:	lea    -0x1(%rbp),%rdx
   73420:	xor    %esi,%esi
   73422:	call   *0xb5d40(%rip)        # 129168 <memset@GLIBC_2.2.5>
   73428:	add    %rbp,%r13
   7342b:	dec    %r13
   7342e:	add    %r13,%rbx
   73431:	mov    %rbx,%rdi
   73434:	movb   $0x0,(%rdi)
   73437:	inc    %r13
   7343a:	mov    %r13,%rdx
   7343d:	mov    0x68(%rsp),%rax
   73442:	mov    0x58(%rsp),%rcx
   73447:	imul   %r14,%rcx
   7344b:	mov    %rdx,0x10(%rax)
   7344f:	mov    %rcx,0x58(%rsp)
   73454:	cmp    %rcx,%r12
   73457:	jb     73cb8 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   7345d:	cmp    %rdx,%r12
   73460:	ja     73cb8 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc18>
   73466:	mov    0x28(%rsp),%rdx
   7346b:	lea    0xf(%rdx),%ecx
   7346e:	and    $0x3f0,%ecx
   73474:	mov    %rcx,0x50(%rsp)
   73479:	mov    %rdx,%rcx
   7347c:	imul   %r14,%rcx
   73480:	mov    %rcx,0xb8(%rsp)
   73488:	mov    0x58(%rsp),%rcx
   7348d:	add    0x8(%rax),%rcx
   73491:	mov    %rcx,0x58(%rsp)
   73496:	lea    -0x1(%rdx),%rax
   7349a:	imul   %r14,%rax
   7349e:	mov    %rax,0xc8(%rsp)
   734a6:	add    $0x4,%rax
   734aa:	mov    %rax,0x60(%rsp)
   734af:	lea    0x0(,%rdx,4),%rax
   734b7:	mov    %rax,0x78(%rsp)
   734bc:	lea    0x1f8(%rsp,%rdx,1),%rax
   734c4:	mov    %rax,0x90(%rsp)
   734cc:	lea    0x1f8(%rsp,%rdx,2),%rax
   734d4:	mov    %rax,0x88(%rsp)
   734dc:	lea    (%rdx,%rdx,2),%rax
   734e0:	add    %rsp,%rax
   734e3:	add    $0x1f8,%rax
   734e9:	mov    %rax,0x80(%rsp)
   734f1:	xor    %r13d,%r13d
   734f4:	xor    %ebp,%ebp
   734f6:	movzbl 0x27(%rsp),%r12d
   734fc:	jmp    73558 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4b8>
   734fe:	lea    (%rsp,%rbp,1),%rcx
   73502:	add    $0xf0,%rcx
   73509:	shr    $0x4,%al
   7350c:	neg    %al
   7350e:	and    $0x1f,%al
   73510:	movzbl %al,%eax
   73513:	mov    %rcx,(%rsp)
   73517:	mov    %eax,0x8(%rsp)
   7351b:	movq   $0x0,0x10(%rsp)
   73524:	lea    0x1f8(%rsp),%rdi
   7352c:	mov    0x78(%rsp),%rsi
   73531:	mov    0x60(%rsp),%rcx
   73536:	mov    0x28(%rsp),%r8
   7353b:	mov    %r14,%r9
   7353e:	call   72f30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   73543:	inc    %r13
   73546:	add    $0x4,%rbp
   7354a:	cmp    %r13,0xe8(%rsp)
   73552:	je     73ba2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb02>
   73558:	test   %r12b,%r12b
   7355b:	je     7357a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4da>
   7355d:	cmp    0x98(%rsp),%r13
   73565:	jae    73cf5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc55>
   7356b:	mov    0xb0(%rsp),%rax
   73573:	movzbl (%rax,%r13,1),%ebx
   73578:	jmp    7357c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4dc>
   7357a:	xor    %ebx,%ebx
   7357c:	mov    %ebx,%eax
   7357e:	and    $0x3,%al
   73580:	movzbl %al,%ecx
   73583:	mov    0x50(%rsp),%rdx
   73588:	cmp    %rdx,0x28(%rsp)
   7358d:	ja     739f5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x955>
   73593:	cmp    $0x2,%al
   73595:	je     735d4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x534>
   73597:	cmp    $0x3,%ecx
   7359a:	jne    735ee <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x54e>
   7359c:	mov    %r15,%r12
   7359f:	mov    $0x7,%al
   735a1:	add    0x28(%rsp),%r12
   735a6:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   735ac:	cmp    0x30(%rsp),%r12
   735b1:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   735b7:	add    0x38(%rsp),%r15
   735bc:	lea    0x1f8(%rsp),%rdi
   735c4:	mov    %r15,%rsi
   735c7:	mov    0x28(%rsp),%rdx
   735cc:	call   *0xb5b86(%rip)        # 129158 <memcpy@GLIBC_2.14>
   735d2:	jmp    73648 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   735d4:	lea    0x1f8(%rsp),%rdi
   735dc:	xor    %esi,%esi
   735de:	mov    0x28(%rsp),%rdx
   735e3:	call   *0xb5b7f(%rip)        # 129168 <memset@GLIBC_2.2.5>
   735e9:	mov    %r15,%r12
   735ec:	jmp    73648 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5a8>
   735ee:	test   %al,%al
   735f0:	lea    -0x659c7(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   735f7:	lea    -0x658ae(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   735fe:	cmove  %rcx,%rax
   73602:	test   %r12b,%r12b
   73605:	lea    -0x65cdc(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7360c:	cmove  %rcx,%rax
   73610:	mov    %rax,(%rsp)
   73614:	lea    0x40(%rsp),%rdi
   73619:	mov    0x38(%rsp),%rsi
   7361e:	mov    0x30(%rsp),%rdx
   73623:	mov    %r15,%rcx
   73626:	lea    0x1f8(%rsp),%r8
   7362e:	mov    0x50(%rsp),%r9
   73633:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   73638:	cmpb   $0x0,0x40(%rsp)
   7363d:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73643:	mov    0x48(%rsp),%r12
   73648:	mov    %ebx,%eax
   7364a:	shr    $0x2,%al
   7364d:	and    $0x3,%al
   7364f:	cmp    $0x3,%al
   73651:	je     73675 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5d5>
   73653:	movzbl %al,%ecx
   73656:	cmp    $0x2,%ecx
   73659:	jne    736b0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x610>
   7365b:	mov    0x90(%rsp),%rdi
   73663:	xor    %esi,%esi
   73665:	mov    0x28(%rsp),%rdx
   7366a:	call   *0xb5af8(%rip)        # 129168 <memset@GLIBC_2.2.5>
   73670:	jmp    7370c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   73675:	mov    %r12,%r15
   73678:	mov    $0x7,%al
   7367a:	add    0x28(%rsp),%r15
   7367f:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73685:	cmp    0x30(%rsp),%r15
   7368a:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73690:	add    0x38(%rsp),%r12
   73695:	mov    0x90(%rsp),%rdi
   7369d:	mov    %r12,%rsi
   736a0:	mov    0x28(%rsp),%rdx
   736a5:	call   *0xb5aad(%rip)        # 129158 <memcpy@GLIBC_2.14>
   736ab:	mov    %r15,%r12
   736ae:	jmp    7370c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x66c>
   736b0:	test   %al,%al
   736b2:	lea    -0x65a89(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   736b9:	lea    -0x65970(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   736c0:	cmove  %rcx,%rax
   736c4:	cmpb   $0x0,0x27(%rsp)
   736c9:	lea    -0x65da0(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   736d0:	cmove  %rcx,%rax
   736d4:	mov    %rax,(%rsp)
   736d8:	lea    0x40(%rsp),%rdi
   736dd:	mov    0x38(%rsp),%rsi
   736e2:	mov    0x30(%rsp),%rdx
   736e7:	mov    %r12,%rcx
   736ea:	mov    0x90(%rsp),%r8
   736f2:	mov    0x50(%rsp),%r9
   736f7:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   736fc:	cmpb   $0x0,0x40(%rsp)
   73701:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73707:	mov    0x48(%rsp),%r12
   7370c:	mov    %ebx,%eax
   7370e:	shr    $0x4,%al
   73711:	and    $0x3,%al
   73713:	cmp    $0x3,%al
   73715:	je     73745 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6a5>
   73717:	movzbl %al,%ecx
   7371a:	cmp    $0x2,%ecx
   7371d:	jne    737ca <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x72a>
   73723:	mov    0x88(%rsp),%rdi
   7372b:	xor    %esi,%esi
   7372d:	mov    0x28(%rsp),%rdx
   73732:	call   *0xb5a30(%rip)        # 129168 <memset@GLIBC_2.2.5>
   73738:	shr    $0x6,%bl
   7373b:	cmp    $0x3,%bl
   7373e:	jne    7378a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73740:	jmp    73832 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   73745:	mov    %r12,%r15
   73748:	mov    $0x7,%al
   7374a:	add    0x28(%rsp),%r15
   7374f:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73755:	cmp    0x30(%rsp),%r15
   7375a:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73760:	add    0x38(%rsp),%r12
   73765:	mov    0x88(%rsp),%rdi
   7376d:	mov    %r12,%rsi
   73770:	mov    0x28(%rsp),%rdx
   73775:	call   *0xb59dd(%rip)        # 129158 <memcpy@GLIBC_2.14>
   7377b:	mov    %r15,%r12
   7377e:	shr    $0x6,%bl
   73781:	cmp    $0x3,%bl
   73784:	je     73832 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x792>
   7378a:	movzbl %bl,%eax
   7378d:	cmp    $0x2,%eax
   73790:	jne    73882 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7e2>
   73796:	mov    0x80(%rsp),%rdi
   7379e:	xor    %esi,%esi
   737a0:	mov    0x28(%rsp),%rdx
   737a5:	call   *0xb59bd(%rip)        # 129168 <memset@GLIBC_2.2.5>
   737ab:	mov    %r12,%r15
   737ae:	movzbl 0x27(%rsp),%r12d
   737b4:	test   %r12b,%r12b
   737b7:	mov    0xb8(%rsp),%rcx
   737bf:	jne    738f1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   737c5:	jmp    7387b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   737ca:	test   %al,%al
   737cc:	lea    -0x65ba3(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   737d3:	lea    -0x65a8a(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   737da:	cmove  %rcx,%rax
   737de:	cmpb   $0x0,0x27(%rsp)
   737e3:	lea    -0x65eba(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   737ea:	cmove  %rcx,%rax
   737ee:	mov    %rax,(%rsp)
   737f2:	lea    0x40(%rsp),%rdi
   737f7:	mov    0x38(%rsp),%rsi
   737fc:	mov    0x30(%rsp),%rdx
   73801:	mov    %r12,%rcx
   73804:	mov    0x88(%rsp),%r8
   7380c:	mov    0x50(%rsp),%r9
   73811:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   73816:	cmpb   $0x0,0x40(%rsp)
   7381b:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73821:	mov    0x48(%rsp),%r12
   73826:	shr    $0x6,%bl
   73829:	cmp    $0x3,%bl
   7382c:	jne    7378a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6ea>
   73832:	mov    %r12,%r15
   73835:	mov    $0x7,%al
   73837:	add    0x28(%rsp),%r15
   7383c:	jb     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73842:	cmp    0x30(%rsp),%r15
   73847:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   7384d:	add    0x38(%rsp),%r12
   73852:	mov    0x80(%rsp),%rdi
   7385a:	mov    %r12,%rsi
   7385d:	mov    0x28(%rsp),%rdx
   73862:	call   *0xb58f0(%rip)        # 129158 <memcpy@GLIBC_2.14>
   73868:	movzbl 0x27(%rsp),%r12d
   7386e:	test   %r12b,%r12b
   73871:	mov    0xb8(%rsp),%rcx
   73879:	jne    738f1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x851>
   7387b:	xor    %eax,%eax
   7387d:	jmp    73909 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x869>
   73882:	test   %bl,%bl
   73884:	lea    -0x65c5b(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7388b:	lea    -0x65b42(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73892:	cmove  %rcx,%rax
   73896:	cmpb   $0x0,0x27(%rsp)
   7389b:	lea    -0x65f72(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   738a2:	cmove  %rcx,%rax
   738a6:	mov    %rax,(%rsp)
   738aa:	lea    0x40(%rsp),%rdi
   738af:	mov    0x38(%rsp),%rsi
   738b4:	mov    0x30(%rsp),%rdx
   738b9:	mov    %r12,%rcx
   738bc:	mov    0x80(%rsp),%r8
   738c4:	mov    0x50(%rsp),%r9
   738c9:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   738ce:	cmpb   $0x0,0x40(%rsp)
   738d3:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   738d9:	mov    0x48(%rsp),%r15
   738de:	movzbl 0x27(%rsp),%r12d
   738e4:	test   %r12b,%r12b
   738e7:	mov    0xb8(%rsp),%rcx
   738ef:	je     7387b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7db>
   738f1:	cmp    0xa8(%rsp),%r13
   738f9:	jae    73d0d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc6d>
   738ff:	mov    0x70(%rsp),%rax
   73904:	movzbl (%rax,%r13,1),%eax
   73909:	mov    %rcx,%rdx
   7390c:	sub    %rbp,%rdx
   7390f:	jb     73c73 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd3>
   73915:	cmp    %rdx,0x60(%rsp)
   7391a:	ja     73c5f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbbf>
   73920:	mov    0x58(%rsp),%rcx
   73925:	lea    (%rcx,%rbp,1),%rdx
   73929:	mov    %eax,%ecx
   7392b:	and    $0x3,%cl
   7392e:	movzbl %cl,%ecx
   73931:	lea    -0x63190(%rip),%rsi        # 107a8 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   73938:	movslq (%rsi,%rcx,4),%rcx
   7393c:	add    %rsi,%rcx
   7393f:	jmp    *%rcx
   73941:	mov    %rdx,%r12
   73944:	lea    (%rsp,%rbp,1),%rbx
   73948:	add    $0xf0,%rbx
   7394f:	call   e4800 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73954:	test   %al,%al
   73956:	je     739bb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91b>
   73958:	mov    %rbx,(%rsp)
   7395c:	lea    0x1f8(%rsp),%rdi
   73964:	mov    0x78(%rsp),%rsi
   73969:	mov    %r12,%rdx
   7396c:	mov    0x60(%rsp),%rcx
   73971:	mov    0x28(%rsp),%r8
   73976:	mov    %r14,%r9
   73979:	call   75cc0 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   7397e:	jmp    739ea <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x94a>
   73980:	lea    (%rsp,%rbp,1),%rax
   73984:	add    $0xf0,%rax
   7398a:	mov    %rax,(%rsp)
   7398e:	movq   $0x0,0x8(%rsp)
   73997:	lea    0x1f8(%rsp),%rdi
   7399f:	mov    0x78(%rsp),%rsi
   739a4:	mov    0x60(%rsp),%rcx
   739a9:	mov    0x28(%rsp),%r8
   739ae:	mov    %r14,%r9
   739b1:	call   72d70 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   739b6:	jmp    73543 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   739bb:	mov    %rbx,(%rsp)
   739bf:	movq   $0x0,0x8(%rsp)
   739c8:	lea    0x1f8(%rsp),%rdi
   739d0:	mov    0x78(%rsp),%rsi
   739d5:	mov    %r12,%rdx
   739d8:	mov    0x60(%rsp),%rcx
   739dd:	mov    0x28(%rsp),%r8
   739e2:	mov    %r14,%r9
   739e5:	call   72b80 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   739ea:	movzbl 0x27(%rsp),%r12d
   739f0:	jmp    73543 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a3>
   739f5:	cmp    $0x2,%al
   739f7:	je     73d4b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   739fd:	cmp    $0x3,%ecx
   73a00:	je     73d32 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73a06:	test   %al,%al
   73a08:	lea    -0x65ddf(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73a0f:	lea    -0x65cc6(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73a16:	cmove  %rcx,%rax
   73a1a:	test   %r12b,%r12b
   73a1d:	lea    -0x660f4(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73a24:	cmove  %rcx,%rax
   73a28:	mov    %rax,(%rsp)
   73a2c:	lea    0x40(%rsp),%rdi
   73a31:	mov    0x38(%rsp),%rsi
   73a36:	mov    0x30(%rsp),%rdx
   73a3b:	mov    %r15,%rcx
   73a3e:	lea    0x1f8(%rsp),%r8
   73a46:	mov    0x50(%rsp),%r9
   73a4b:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   73a50:	cmpb   $0x0,0x40(%rsp)
   73a55:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73a5b:	mov    %ebx,%eax
   73a5d:	shr    $0x2,%al
   73a60:	and    $0x3,%al
   73a62:	cmp    $0x2,%al
   73a64:	movzbl 0x27(%rsp),%edx
   73a69:	je     73d4b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73a6f:	movzbl %al,%ecx
   73a72:	cmp    $0x3,%ecx
   73a75:	je     73d32 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73a7b:	test   %al,%al
   73a7d:	lea    -0x65e54(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73a84:	lea    -0x65d3b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73a8b:	cmove  %rcx,%rax
   73a8f:	mov    0x48(%rsp),%rcx
   73a94:	test   %dl,%dl
   73a96:	lea    -0x6616d(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73a9d:	cmove  %rdx,%rax
   73aa1:	mov    %rax,(%rsp)
   73aa5:	lea    0x40(%rsp),%rdi
   73aaa:	mov    0x38(%rsp),%rsi
   73aaf:	mov    0x30(%rsp),%rdx
   73ab4:	mov    0x90(%rsp),%r8
   73abc:	mov    0x50(%rsp),%r9
   73ac1:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   73ac6:	cmpb   $0x0,0x40(%rsp)
   73acb:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73ad1:	mov    %ebx,%eax
   73ad3:	shr    $0x4,%al
   73ad6:	and    $0x3,%al
   73ad8:	cmp    $0x2,%al
   73ada:	movzbl 0x27(%rsp),%edx
   73adf:	je     73d4b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73ae5:	movzbl %al,%ecx
   73ae8:	cmp    $0x3,%ecx
   73aeb:	je     73d32 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73af1:	test   %al,%al
   73af3:	lea    -0x65eca(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73afa:	lea    -0x65db1(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b01:	cmove  %rcx,%rax
   73b05:	mov    0x48(%rsp),%rcx
   73b0a:	test   %dl,%dl
   73b0c:	lea    -0x661e3(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b13:	cmove  %rdx,%rax
   73b17:	mov    %rax,(%rsp)
   73b1b:	lea    0x40(%rsp),%rdi
   73b20:	mov    0x38(%rsp),%rsi
   73b25:	mov    0x30(%rsp),%rdx
   73b2a:	mov    0x88(%rsp),%r8
   73b32:	mov    0x50(%rsp),%r9
   73b37:	call   d5fa0 <meshoptimizer_rs::codec::vertex::bytes>
   73b3c:	cmpb   $0x0,0x40(%rsp)
   73b41:	jne    73c44 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba4>
   73b47:	shr    $0x6,%bl
   73b4a:	cmp    $0x2,%bl
   73b4d:	movzbl 0x27(%rsp),%edx
   73b52:	je     73d4b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xcab>
   73b58:	movzbl %bl,%eax
   73b5b:	cmp    $0x3,%eax
   73b5e:	je     73d32 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc92>
   73b64:	test   %bl,%bl
   73b66:	lea    -0x65f3d(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73b6d:	lea    -0x65e24(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b74:	cmove  %rcx,%rax
   73b78:	mov    0x48(%rsp),%rcx
   73b7d:	test   %dl,%dl
   73b7f:	lea    -0x66256(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b86:	cmove  %rdx,%rax
   73b8a:	mov    %rax,(%rsp)
   73b8e:	lea    0x40(%rsp),%rdi
   73b93:	mov    0x38(%rsp),%rsi
   73b98:	mov    0x30(%rsp),%rdx
   73b9d:	jmp    738bc <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x81c>
   73ba2:	mov    0xb8(%rsp),%rdx
   73baa:	mov    0xc8(%rsp),%rdi
   73bb2:	cmp    %rdx,%rdi
   73bb5:	ja     73ce5 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc45>
   73bbb:	mov    0x58(%rsp),%rsi
   73bc0:	add    %rdi,%rsi
   73bc3:	lea    0xf0(%rsp),%rdi
   73bcb:	mov    %r14,%rdx
   73bce:	call   *0xb5584(%rip)        # 129158 <memcpy@GLIBC_2.14>
   73bd4:	mov    0xd0(%rsp),%rcx
   73bdc:	mov    %rcx,%rax
   73bdf:	mov    0xc0(%rsp),%r13
   73be7:	cmp    %r13,%rcx
   73bea:	mov    0x30(%rsp),%rbx
   73bef:	mov    0x68(%rsp),%rbp
   73bf4:	jb     7334c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2ac>
   73bfa:	jmp    73c29 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb89>
   73bfc:	mov    $0x1,%ecx
   73c01:	mov    $0x1,%r8d
   73c07:	mov    0x68(%rsp),%rbx
   73c0c:	mov    %rbx,%rdi
   73c0f:	mov    %r13,%rsi
   73c12:	mov    %rbp,%rdx
   73c15:	call   84bf0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73c1a:	mov    0x10(%rbx),%r13
   73c1e:	jmp    73409 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   73c23:	mov    $0x1,%r15d
   73c29:	sub    %r15,%rbx
   73c2c:	setae  %cl
   73c2f:	cmp    0xd8(%rsp),%rbx
   73c37:	sete   %dl
   73c3a:	mov    $0x7,%al
   73c3c:	test   %dl,%cl
   73c3e:	je     73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73c40:	mov    $0xff,%al
   73c42:	jmp    73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73c44:	movzbl 0x41(%rsp),%eax
   73c49:	add    $0x5f8,%rsp
   73c50:	pop    %rbx
   73c51:	pop    %r12
   73c53:	pop    %r13
   73c55:	pop    %r14
   73c57:	pop    %r15
   73c59:	pop    %rbp
   73c5a:	ret
   73c5b:	mov    $0x7,%al
   73c5d:	jmp    73c49 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xba9>
   73c5f:	lea    0xb0152(%rip),%rcx        # 123db8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   73c66:	xor    %edi,%edi
   73c68:	mov    0x60(%rsp),%rsi
   73c6d:	call   *0xb563d(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73c73:	mov    %rcx,%rdx
   73c76:	lea    0xb0153(%rip),%rcx        # 123dd0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   73c7d:	mov    %rbp,%rdi
   73c80:	mov    %rdx,%rsi
   73c83:	call   *0xb5627(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73c89:	lea    0xb01a0(%rip),%rcx        # 123e30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   73c90:	mov    $0x100,%edx
   73c95:	xor    %edi,%edi
   73c97:	mov    %r14,%rsi
   73c9a:	call   *0xb5610(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73ca0:	lea    0xb0171(%rip),%rcx        # 123e18 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   73ca7:	mov    %rsi,%rdi
   73caa:	mov    0x70(%rsp),%rsi
   73caf:	mov    %rbx,%rdx
   73cb2:	call   *0xb55f8(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73cb8:	lea    0xb3a41(%rip),%rcx        # 127700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5b18>
   73cbf:	mov    0x58(%rsp),%rdi
   73cc4:	mov    %r12,%rsi
   73cc7:	call   *0xb55e3(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73ccd:	lea    0xb0174(%rip),%rcx        # 123e48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73cd4:	mov    0x70(%rsp),%rdi
   73cd9:	mov    %rbx,%rsi
   73cdc:	mov    %rbx,%rdx
   73cdf:	call   *0xb55cb(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73ce5:	lea    0xb0084(%rip),%rcx        # 123d70 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   73cec:	mov    %rdx,%rsi
   73cef:	call   *0xb55bb(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73cf5:	lea    0xb008c(%rip),%rdx        # 123d88 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   73cfc:	mov    %r13,%rdi
   73cff:	mov    0x98(%rsp),%rsi
   73d07:	call   *0xb5483(%rip)        # 129190 <_DYNAMIC+0x250>
   73d0d:	lea    0xb008c(%rip),%rdx        # 123da0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73d14:	mov    %r13,%rdi
   73d17:	mov    0xa8(%rsp),%rsi
   73d1f:	call   *0xb546b(%rip)        # 129190 <_DYNAMIC+0x250>
   73d25:	lea    0xb002c(%rip),%rdi        # 123d58 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   73d2c:	call   *0xb5646(%rip)        # 129378 <_DYNAMIC+0x438>
   73d32:	lea    0xb00af(%rip),%rcx        # 123de8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73d39:	xor    %edi,%edi
   73d3b:	mov    0x28(%rsp),%rsi
   73d40:	mov    0x50(%rsp),%rdx
   73d45:	call   *0xb5565(%rip)        # 1292b0 <_DYNAMIC+0x370>
   73d4b:	lea    0xb00ae(%rip),%rcx        # 123e00 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73d52:	xor    %edi,%edi
   73d54:	mov    0x28(%rsp),%rsi
   73d59:	mov    0x50(%rsp),%rdx
   73d5e:	call   *0xb554c(%rip)        # 1292b0 <_DYNAMIC+0x370>
