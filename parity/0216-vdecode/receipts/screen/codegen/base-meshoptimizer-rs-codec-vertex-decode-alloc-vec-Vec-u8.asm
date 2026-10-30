
<artifacts>/screen/driver-base:     file format elf64-x86-64


Disassembly of section .text:

0000000000073130 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   73130:	push   %rbp
   73131:	push   %r15
   73133:	push   %r14
   73135:	push   %r13
   73137:	push   %r12
   73139:	push   %rbx
   7313a:	sub    $0x5f8,%rsp
   73141:	mov    %r8,%r15
   73144:	mov    %rcx,%r12
   73147:	mov    %rdx,%r14
   7314a:	mov    %rsi,%r13
   7314d:	mov    %rdi,%rbp
   73150:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73155:	cmp    $0x2,%al
   73157:	jb     731bd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8d>
   73159:	cmp    $0x20,%r14
   7315d:	lea    0x5b3c(%rip),%rax        # 78ca0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   73164:	lea    0x45c5(%rip),%rcx        # 77730 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   7316b:	cmove  %rax,%rcx
   7316f:	lea    0xb3aa(%rip),%rax        # 7e520 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   73176:	lea    0x9a33(%rip),%rdx        # 7cbb0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   7317d:	cmove  %rax,%rdx
   73181:	cmp    $0x4,%r14
   73185:	lea    0x6f44(%rip),%rax        # 7a0d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   7318c:	cmovne %rcx,%rax
   73190:	lea    0xcb69(%rip),%r9        # 7fd00 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   73197:	cmovne %rdx,%r9
   7319b:	cmp    $0x21,%r13
   7319f:	cmovae %rax,%r9
   731a3:	mov    %rbp,%rdi
   731a6:	mov    %r13,%rsi
   731a9:	mov    %r14,%rdx
   731ac:	mov    %r12,%rcx
   731af:	mov    %r15,%r8
   731b2:	call   *%r9
   731b5:	cmp    $0xfe,%al
   731b7:	jne    73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   731bd:	mov    $0x7,%al
   731bf:	test   %r15,%r15
   731c2:	je     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   731c8:	movzbl (%r12),%edx
   731cd:	mov    %edx,%ecx
   731cf:	and    $0xf0,%cl
   731d2:	cmp    $0xa0,%cl
   731d5:	jne    73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   731db:	and    $0xf,%dl
   731de:	mov    $0x5,%al
   731e0:	mov    %dl,0x27(%rsp)
   731e4:	cmp    $0x1,%dl
   731e7:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   731ed:	cmpb   $0x0,0x27(%rsp)
   731f2:	je     7321b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xeb>
   731f4:	mov    %r14,%rcx
   731f7:	shr    $0x2,%rcx
   731fb:	add    %r14,%rcx
   731fe:	cmp    $0x19,%rcx
   73202:	mov    $0x18,%ebx
   73207:	cmovae %rcx,%rbx
   7320b:	lea    0x1(%rbx),%rdx
   7320f:	mov    $0x7,%al
   73211:	cmp    %rdx,%r15
   73214:	jae    7323a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x10a>
   73216:	jmp    73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7321b:	cmp    $0x21,%r14
   7321f:	mov    $0x20,%ebx
   73224:	cmovae %r14,%rbx
   73228:	mov    %r14,%rcx
   7322b:	lea    0x1(%rbx),%rdx
   7322f:	mov    $0x7,%al
   73231:	cmp    %rdx,%r15
   73234:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7323a:	mov    %r15,%rsi
   7323d:	sub    %rcx,%rsi
   73240:	lea    (%rsi,%r14,1),%rax
   73244:	mov    %r15,%rcx
   73247:	mov    %rax,0x70(%rsp)
   7324c:	sub    %rax,%rcx
   7324f:	mov    %rcx,0xa8(%rsp)
   73257:	jb     73d34 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc04>
   7325d:	xorps  %xmm0,%xmm0
   73260:	movaps %xmm0,0x1e0(%rsp)
   73268:	movaps %xmm0,0x1d0(%rsp)
   73270:	movaps %xmm0,0x1c0(%rsp)
   73278:	movaps %xmm0,0x1b0(%rsp)
   73280:	movaps %xmm0,0x1a0(%rsp)
   73288:	movaps %xmm0,0x190(%rsp)
   73290:	movaps %xmm0,0x180(%rsp)
   73298:	movaps %xmm0,0x170(%rsp)
   732a0:	movaps %xmm0,0x160(%rsp)
   732a8:	movaps %xmm0,0x150(%rsp)
   732b0:	movaps %xmm0,0x140(%rsp)
   732b8:	movaps %xmm0,0x130(%rsp)
   732c0:	movaps %xmm0,0x120(%rsp)
   732c8:	movaps %xmm0,0x110(%rsp)
   732d0:	movaps %xmm0,0x100(%rsp)
   732d8:	movaps %xmm0,0xf0(%rsp)
   732e0:	cmp    $0x101,%r14
   732e7:	jae    73cf0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbc0>
   732ed:	lea    (%rsi,%r14,1),%rax
   732f1:	cmp    %rsi,%rax
   732f4:	jb     73d07 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd7>
   732fa:	add    %r12,%rsi
   732fd:	lea    0xf0(%rsp),%rdi
   73305:	mov    %r14,%rdx
   73308:	call   *0xb33b2(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   7330e:	test   %r14,%r14
   73311:	je     73d8c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc5c>
   73317:	mov    %rbx,0xd8(%rsp)
   7331f:	mov    $0x2000,%ax
   73323:	xor    %edx,%edx
   73325:	div    %r14w
   73329:	and    $0x3ff0,%eax
   7332e:	cmp    $0x100,%eax
   73333:	mov    $0x100,%ecx
   73338:	cmovb  %eax,%ecx
   7333b:	mov    %rcx,0xe0(%rsp)
   73343:	lea    0x1f8(%rsp),%rdi
   7334b:	mov    $0x400,%edx
   73350:	xor    %esi,%esi
   73352:	call   *0xb3378(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   73358:	test   %r13,%r13
   7335b:	je     73c8d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5d>
   73361:	add    %r12,0x70(%rsp)
   73366:	mov    %r14,%rax
   73369:	shr    $0x2,%rax
   7336d:	mov    %rax,0xa0(%rsp)
   73375:	lea    0x3(%r14),%rax
   73379:	shr    $0x2,%rax
   7337d:	mov    %rax,0xe8(%rsp)
   73385:	mov    $0x1,%ebx
   7338a:	xor    %eax,%eax
   7338c:	mov    %rbp,0x68(%rsp)
   73391:	mov    %r15,0x30(%rsp)
   73396:	mov    %r12,0x38(%rsp)
   7339b:	mov    %r13,0xc0(%rsp)
   733a3:	mov    %r13,%rdx
   733a6:	mov    %rax,0x58(%rsp)
   733ab:	sub    %rax,%rdx
   733ae:	mov    0xe0(%rsp),%rax
   733b6:	cmp    %rax,%rdx
   733b9:	cmovae %rax,%rdx
   733bd:	mov    %rdx,0x28(%rsp)
   733c2:	cmpb   $0x0,0x27(%rsp)
   733c7:	je     7340c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2dc>
   733c9:	mov    %rbx,%rcx
   733cc:	mov    $0x7,%al
   733ce:	add    0xa0(%rsp),%rcx
   733d6:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   733dc:	cmp    %r15,%rcx
   733df:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   733e5:	add    0x38(%rsp),%rbx
   733ea:	mov    0xa0(%rsp),%rax
   733f2:	mov    %rax,0x98(%rsp)
   733fa:	mov    %rbx,0xb0(%rsp)
   73402:	mov    %rcx,%rbx
   73405:	mov    0x68(%rsp),%rbp
   7340a:	jmp    73425 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2f5>
   7340c:	movq   $0x0,0x98(%rsp)
   73418:	mov    $0x1,%eax
   7341d:	mov    %rax,0xb0(%rsp)
   73425:	mov    0x58(%rsp),%rax
   7342a:	mov    0x28(%rsp),%rcx
   7342f:	lea    (%rcx,%rax,1),%r15
   73433:	mov    %r15,0xd0(%rsp)
   7343b:	imul   %r14,%r15
   7343f:	mov    0x10(%rbp),%r13
   73443:	mov    %rbp,%rax
   73446:	mov    %r15,%rbp
   73449:	mov    %r15,%rdx
   7344c:	sub    %r13,%rbp
   7344f:	jbe    73499 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   73451:	mov    (%rax),%rax
   73454:	sub    %r13,%rax
   73457:	cmp    %rax,%rbp
   7345a:	ja     73c65 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb35>
   73460:	mov    0x68(%rsp),%rax
   73465:	mov    0x8(%rax),%r12
   73469:	lea    (%r12,%r13,1),%rdi
   7346d:	cmp    $0x2,%rbp
   73471:	jb     7348b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x35b>
   73473:	lea    -0x1(%rbp),%rdx
   73477:	xor    %esi,%esi
   73479:	call   *0xb3251(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   7347f:	add    %rbp,%r13
   73482:	dec    %r13
   73485:	add    %r13,%r12
   73488:	mov    %r12,%rdi
   7348b:	movb   $0x0,(%rdi)
   7348e:	inc    %r13
   73491:	mov    %r13,%rdx
   73494:	mov    0x68(%rsp),%rax
   73499:	mov    0x58(%rsp),%rcx
   7349e:	imul   %r14,%rcx
   734a2:	mov    %rdx,0x10(%rax)
   734a6:	mov    %rcx,0x58(%rsp)
   734ab:	cmp    %rcx,%r15
   734ae:	jb     73d1f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   734b4:	cmp    %rdx,%r15
   734b7:	ja     73d1f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   734bd:	mov    0x28(%rsp),%rdx
   734c2:	lea    0xf(%rdx),%ecx
   734c5:	and    $0x3f0,%ecx
   734cb:	mov    %rcx,0x50(%rsp)
   734d0:	mov    %rdx,%rcx
   734d3:	imul   %r14,%rcx
   734d7:	mov    %rcx,0xb8(%rsp)
   734df:	mov    0x58(%rsp),%rcx
   734e4:	add    0x8(%rax),%rcx
   734e8:	mov    %rcx,0x58(%rsp)
   734ed:	lea    -0x1(%rdx),%rax
   734f1:	imul   %r14,%rax
   734f5:	mov    %rax,0xc8(%rsp)
   734fd:	add    $0x4,%rax
   73501:	mov    %rax,0x60(%rsp)
   73506:	lea    0x0(,%rdx,4),%rax
   7350e:	mov    %rax,0x78(%rsp)
   73513:	lea    0x1f8(%rsp,%rdx,1),%rax
   7351b:	mov    %rax,0x90(%rsp)
   73523:	lea    0x1f8(%rsp,%rdx,2),%rax
   7352b:	mov    %rax,0x88(%rsp)
   73533:	lea    (%rdx,%rdx,2),%rax
   73537:	add    %rsp,%rax
   7353a:	add    $0x1f8,%rax
   73540:	mov    %rax,0x80(%rsp)
   73548:	xor    %r13d,%r13d
   7354b:	xor    %ebp,%ebp
   7354d:	movzbl 0x27(%rsp),%r15d
   73553:	jmp    735af <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x47f>
   73555:	lea    (%rsp,%rbp,1),%rcx
   73559:	add    $0xf0,%rcx
   73560:	shr    $0x4,%al
   73563:	neg    %al
   73565:	and    $0x1f,%al
   73567:	movzbl %al,%eax
   7356a:	mov    %rcx,(%rsp)
   7356e:	mov    %eax,0x8(%rsp)
   73572:	movq   $0x0,0x10(%rsp)
   7357b:	lea    0x1f8(%rsp),%rdi
   73583:	mov    0x78(%rsp),%rsi
   73588:	mov    0x60(%rsp),%rcx
   7358d:	mov    0x28(%rsp),%r8
   73592:	mov    %r14,%r9
   73595:	call   72fc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   7359a:	inc    %r13
   7359d:	add    $0x4,%rbp
   735a1:	cmp    %r13,0xe8(%rsp)
   735a9:	je     73c0b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xadb>
   735af:	test   %r15b,%r15b
   735b2:	je     735d1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a1>
   735b4:	cmp    0x98(%rsp),%r13
   735bc:	jae    73d5c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2c>
   735c2:	mov    0xb0(%rsp),%rax
   735ca:	movzbl (%rax,%r13,1),%r12d
   735cf:	jmp    735d4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a4>
   735d1:	xor    %r12d,%r12d
   735d4:	mov    %r12d,%eax
   735d7:	and    $0x3,%al
   735d9:	movzbl %al,%ecx
   735dc:	mov    0x50(%rsp),%rdx
   735e1:	cmp    %rdx,0x28(%rsp)
   735e6:	ja     73a58 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x928>
   735ec:	cmp    $0x2,%al
   735ee:	je     7362d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4fd>
   735f0:	cmp    $0x3,%ecx
   735f3:	jne    73647 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x517>
   735f5:	mov    %rbx,%r15
   735f8:	mov    $0x7,%al
   735fa:	add    0x28(%rsp),%r15
   735ff:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73605:	cmp    0x30(%rsp),%r15
   7360a:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73610:	add    0x38(%rsp),%rbx
   73615:	lea    0x1f8(%rsp),%rdi
   7361d:	mov    %rbx,%rsi
   73620:	mov    0x28(%rsp),%rdx
   73625:	call   *0xb3095(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   7362b:	jmp    736a1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   7362d:	lea    0x1f8(%rsp),%rdi
   73635:	xor    %esi,%esi
   73637:	mov    0x28(%rsp),%rdx
   7363c:	call   *0xb308e(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   73642:	mov    %rbx,%r15
   73645:	jmp    736a1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   73647:	test   %al,%al
   73649:	lea    -0x65ab0(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73650:	lea    -0x65997(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73657:	cmove  %rcx,%rax
   7365b:	test   %r15b,%r15b
   7365e:	lea    -0x65dc5(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73665:	cmove  %rcx,%rax
   73669:	mov    %rax,(%rsp)
   7366d:	lea    0x40(%rsp),%rdi
   73672:	mov    0x38(%rsp),%rsi
   73677:	mov    0x30(%rsp),%rdx
   7367c:	mov    %rbx,%rcx
   7367f:	lea    0x1f8(%rsp),%r8
   73687:	mov    0x50(%rsp),%r9
   7368c:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73691:	cmpb   $0x0,0x40(%rsp)
   73696:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7369c:	mov    0x48(%rsp),%r15
   736a1:	mov    %r12d,%eax
   736a4:	shr    $0x2,%al
   736a7:	and    $0x3,%al
   736a9:	cmp    $0x3,%al
   736ab:	je     736cf <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x59f>
   736ad:	movzbl %al,%ecx
   736b0:	cmp    $0x2,%ecx
   736b3:	jne    7370a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5da>
   736b5:	mov    0x90(%rsp),%rdi
   736bd:	xor    %esi,%esi
   736bf:	mov    0x28(%rsp),%rdx
   736c4:	call   *0xb3006(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   736ca:	jmp    73766 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   736cf:	mov    %r15,%rbx
   736d2:	mov    $0x7,%al
   736d4:	add    0x28(%rsp),%rbx
   736d9:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   736df:	cmp    0x30(%rsp),%rbx
   736e4:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   736ea:	add    0x38(%rsp),%r15
   736ef:	mov    0x90(%rsp),%rdi
   736f7:	mov    %r15,%rsi
   736fa:	mov    0x28(%rsp),%rdx
   736ff:	call   *0xb2fbb(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   73705:	mov    %rbx,%r15
   73708:	jmp    73766 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   7370a:	test   %al,%al
   7370c:	lea    -0x65b73(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73713:	lea    -0x65a5a(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7371a:	cmove  %rcx,%rax
   7371e:	cmpb   $0x0,0x27(%rsp)
   73723:	lea    -0x65e8a(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7372a:	cmove  %rcx,%rax
   7372e:	mov    %rax,(%rsp)
   73732:	lea    0x40(%rsp),%rdi
   73737:	mov    0x38(%rsp),%rsi
   7373c:	mov    0x30(%rsp),%rdx
   73741:	mov    %r15,%rcx
   73744:	mov    0x90(%rsp),%r8
   7374c:	mov    0x50(%rsp),%r9
   73751:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73756:	cmpb   $0x0,0x40(%rsp)
   7375b:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73761:	mov    0x48(%rsp),%r15
   73766:	mov    %r12d,%eax
   73769:	shr    $0x4,%al
   7376c:	and    $0x3,%al
   7376e:	cmp    $0x3,%al
   73770:	je     737a2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x672>
   73772:	movzbl %al,%ecx
   73775:	cmp    $0x2,%ecx
   73778:	jne    7382a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6fa>
   7377e:	mov    0x88(%rsp),%rdi
   73786:	xor    %esi,%esi
   73788:	mov    0x28(%rsp),%rdx
   7378d:	call   *0xb2f3d(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   73793:	shr    $0x6,%r12b
   73797:	cmp    $0x3,%r12b
   7379b:	jne    737e9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   7379d:	jmp    73894 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   737a2:	mov    %r15,%rbx
   737a5:	mov    $0x7,%al
   737a7:	add    0x28(%rsp),%rbx
   737ac:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   737b2:	cmp    0x30(%rsp),%rbx
   737b7:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   737bd:	add    0x38(%rsp),%r15
   737c2:	mov    0x88(%rsp),%rdi
   737ca:	mov    %r15,%rsi
   737cd:	mov    0x28(%rsp),%rdx
   737d2:	call   *0xb2ee8(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   737d8:	mov    %rbx,%r15
   737db:	shr    $0x6,%r12b
   737df:	cmp    $0x3,%r12b
   737e3:	je     73894 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   737e9:	movzbl %r12b,%eax
   737ed:	cmp    $0x2,%eax
   737f0:	jne    738e4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7b4>
   737f6:	mov    0x80(%rsp),%rdi
   737fe:	xor    %esi,%esi
   73800:	mov    0x28(%rsp),%rdx
   73805:	call   *0xb2ec5(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   7380b:	mov    %r15,%rbx
   7380e:	movzbl 0x27(%rsp),%r15d
   73814:	test   %r15b,%r15b
   73817:	mov    0xb8(%rsp),%rcx
   7381f:	jne    73954 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   73825:	jmp    738dd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   7382a:	test   %al,%al
   7382c:	lea    -0x65c93(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73833:	lea    -0x65b7a(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7383a:	cmove  %rcx,%rax
   7383e:	cmpb   $0x0,0x27(%rsp)
   73843:	lea    -0x65faa(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7384a:	cmove  %rcx,%rax
   7384e:	mov    %rax,(%rsp)
   73852:	lea    0x40(%rsp),%rdi
   73857:	mov    0x38(%rsp),%rsi
   7385c:	mov    0x30(%rsp),%rdx
   73861:	mov    %r15,%rcx
   73864:	mov    0x88(%rsp),%r8
   7386c:	mov    0x50(%rsp),%r9
   73871:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73876:	cmpb   $0x0,0x40(%rsp)
   7387b:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73881:	mov    0x48(%rsp),%r15
   73886:	shr    $0x6,%r12b
   7388a:	cmp    $0x3,%r12b
   7388e:	jne    737e9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   73894:	mov    %r15,%rbx
   73897:	mov    $0x7,%al
   73899:	add    0x28(%rsp),%rbx
   7389e:	jb     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   738a4:	cmp    0x30(%rsp),%rbx
   738a9:	ja     73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   738af:	add    0x38(%rsp),%r15
   738b4:	mov    0x80(%rsp),%rdi
   738bc:	mov    %r15,%rsi
   738bf:	mov    0x28(%rsp),%rdx
   738c4:	call   *0xb2df6(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   738ca:	movzbl 0x27(%rsp),%r15d
   738d0:	test   %r15b,%r15b
   738d3:	mov    0xb8(%rsp),%rcx
   738db:	jne    73954 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   738dd:	xor    %eax,%eax
   738df:	jmp    7396c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x83c>
   738e4:	test   %r12b,%r12b
   738e7:	lea    -0x65d4e(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   738ee:	lea    -0x65c35(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   738f5:	cmove  %rcx,%rax
   738f9:	cmpb   $0x0,0x27(%rsp)
   738fe:	lea    -0x66065(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73905:	cmove  %rcx,%rax
   73909:	mov    %rax,(%rsp)
   7390d:	lea    0x40(%rsp),%rdi
   73912:	mov    0x38(%rsp),%rsi
   73917:	mov    0x30(%rsp),%rdx
   7391c:	mov    %r15,%rcx
   7391f:	mov    0x80(%rsp),%r8
   73927:	mov    0x50(%rsp),%r9
   7392c:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73931:	cmpb   $0x0,0x40(%rsp)
   73936:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7393c:	mov    0x48(%rsp),%rbx
   73941:	movzbl 0x27(%rsp),%r15d
   73947:	test   %r15b,%r15b
   7394a:	mov    0xb8(%rsp),%rcx
   73952:	je     738dd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   73954:	cmp    0xa8(%rsp),%r13
   7395c:	jae    73d74 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc44>
   73962:	mov    0x70(%rsp),%rax
   73967:	movzbl (%rax,%r13,1),%eax
   7396c:	mov    %rcx,%rdx
   7396f:	sub    %rbp,%rdx
   73972:	jb     73cda <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbaa>
   73978:	cmp    %rdx,0x60(%rsp)
   7397d:	ja     73cc6 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb96>
   73983:	mov    0x58(%rsp),%rcx
   73988:	lea    (%rcx,%rbp,1),%rdx
   7398c:	mov    %eax,%ecx
   7398e:	and    $0x3,%cl
   73991:	movzbl %cl,%ecx
   73994:	lea    -0x6327b(%rip),%rsi        # 10720 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   7399b:	movslq (%rsi,%rcx,4),%rcx
   7399f:	add    %rsi,%rcx
   739a2:	jmp    *%rcx
   739a4:	mov    %rdx,%r15
   739a7:	lea    (%rsp,%rbp,1),%r12
   739ab:	add    $0xf0,%r12
   739b2:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   739b7:	test   %al,%al
   739b9:	je     73a1e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8ee>
   739bb:	mov    %r12,(%rsp)
   739bf:	lea    0x1f8(%rsp),%rdi
   739c7:	mov    0x78(%rsp),%rsi
   739cc:	mov    %r15,%rdx
   739cf:	mov    0x60(%rsp),%rcx
   739d4:	mov    0x28(%rsp),%r8
   739d9:	mov    %r14,%r9
   739dc:	call   75d20 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   739e1:	jmp    73a4d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91d>
   739e3:	lea    (%rsp,%rbp,1),%rax
   739e7:	add    $0xf0,%rax
   739ed:	mov    %rax,(%rsp)
   739f1:	movq   $0x0,0x8(%rsp)
   739fa:	lea    0x1f8(%rsp),%rdi
   73a02:	mov    0x78(%rsp),%rsi
   73a07:	mov    0x60(%rsp),%rcx
   73a0c:	mov    0x28(%rsp),%r8
   73a11:	mov    %r14,%r9
   73a14:	call   72e00 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   73a19:	jmp    7359a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   73a1e:	mov    %r12,(%rsp)
   73a22:	movq   $0x0,0x8(%rsp)
   73a2b:	lea    0x1f8(%rsp),%rdi
   73a33:	mov    0x78(%rsp),%rsi
   73a38:	mov    %r15,%rdx
   73a3b:	mov    0x60(%rsp),%rcx
   73a40:	mov    0x28(%rsp),%r8
   73a45:	mov    %r14,%r9
   73a48:	call   72c10 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   73a4d:	movzbl 0x27(%rsp),%r15d
   73a53:	jmp    7359a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   73a58:	cmp    $0x2,%al
   73a5a:	je     73db2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   73a60:	cmp    $0x3,%ecx
   73a63:	je     73d99 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73a69:	test   %al,%al
   73a6b:	lea    -0x65ed2(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73a72:	lea    -0x65db9(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73a79:	cmove  %rcx,%rax
   73a7d:	test   %r15b,%r15b
   73a80:	lea    -0x661e7(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73a87:	cmove  %rcx,%rax
   73a8b:	mov    %rax,(%rsp)
   73a8f:	lea    0x40(%rsp),%rdi
   73a94:	mov    0x38(%rsp),%rsi
   73a99:	mov    0x30(%rsp),%rdx
   73a9e:	mov    %rbx,%rcx
   73aa1:	lea    0x1f8(%rsp),%r8
   73aa9:	mov    0x50(%rsp),%r9
   73aae:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73ab3:	cmpb   $0x0,0x40(%rsp)
   73ab8:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73abe:	mov    %r12d,%eax
   73ac1:	shr    $0x2,%al
   73ac4:	and    $0x3,%al
   73ac6:	cmp    $0x2,%al
   73ac8:	movzbl 0x27(%rsp),%edx
   73acd:	je     73db2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   73ad3:	movzbl %al,%ecx
   73ad6:	cmp    $0x3,%ecx
   73ad9:	je     73d99 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73adf:	test   %al,%al
   73ae1:	lea    -0x65f48(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73ae8:	lea    -0x65e2f(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73aef:	cmove  %rcx,%rax
   73af3:	mov    0x48(%rsp),%rcx
   73af8:	test   %dl,%dl
   73afa:	lea    -0x66261(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b01:	cmove  %rdx,%rax
   73b05:	mov    %rax,(%rsp)
   73b09:	lea    0x40(%rsp),%rdi
   73b0e:	mov    0x38(%rsp),%rsi
   73b13:	mov    0x30(%rsp),%rdx
   73b18:	mov    0x90(%rsp),%r8
   73b20:	mov    0x50(%rsp),%r9
   73b25:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73b2a:	cmpb   $0x0,0x40(%rsp)
   73b2f:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73b35:	mov    %r12d,%eax
   73b38:	shr    $0x4,%al
   73b3b:	and    $0x3,%al
   73b3d:	cmp    $0x2,%al
   73b3f:	movzbl 0x27(%rsp),%edx
   73b44:	je     73db2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   73b4a:	movzbl %al,%ecx
   73b4d:	cmp    $0x3,%ecx
   73b50:	je     73d99 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73b56:	test   %al,%al
   73b58:	lea    -0x65fbf(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73b5f:	lea    -0x65ea6(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b66:	cmove  %rcx,%rax
   73b6a:	mov    0x48(%rsp),%rcx
   73b6f:	test   %dl,%dl
   73b71:	lea    -0x662d8(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b78:	cmove  %rdx,%rax
   73b7c:	mov    %rax,(%rsp)
   73b80:	lea    0x40(%rsp),%rdi
   73b85:	mov    0x38(%rsp),%rsi
   73b8a:	mov    0x30(%rsp),%rdx
   73b8f:	mov    0x88(%rsp),%r8
   73b97:	mov    0x50(%rsp),%r9
   73b9c:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   73ba1:	cmpb   $0x0,0x40(%rsp)
   73ba6:	jne    73cab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73bac:	shr    $0x6,%r12b
   73bb0:	cmp    $0x2,%r12b
   73bb4:	movzbl 0x27(%rsp),%edx
   73bb9:	je     73db2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   73bbf:	movzbl %r12b,%eax
   73bc3:	cmp    $0x3,%eax
   73bc6:	je     73d99 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73bcc:	test   %r12b,%r12b
   73bcf:	lea    -0x66036(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73bd6:	lea    -0x65f1d(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73bdd:	cmove  %rcx,%rax
   73be1:	mov    0x48(%rsp),%rcx
   73be6:	test   %dl,%dl
   73be8:	lea    -0x6634f(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73bef:	cmove  %rdx,%rax
   73bf3:	mov    %rax,(%rsp)
   73bf7:	lea    0x40(%rsp),%rdi
   73bfc:	mov    0x38(%rsp),%rsi
   73c01:	mov    0x30(%rsp),%rdx
   73c06:	jmp    7391f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ef>
   73c0b:	mov    0xb8(%rsp),%rdx
   73c13:	mov    0xc8(%rsp),%rdi
   73c1b:	cmp    %rdx,%rdi
   73c1e:	ja     73d4c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc1c>
   73c24:	mov    0x58(%rsp),%rsi
   73c29:	add    %rdi,%rsi
   73c2c:	lea    0xf0(%rsp),%rdi
   73c34:	mov    %r14,%rdx
   73c37:	call   *0xb2a83(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   73c3d:	mov    0xd0(%rsp),%rcx
   73c45:	mov    %rcx,%rax
   73c48:	mov    0xc0(%rsp),%r13
   73c50:	cmp    %r13,%rcx
   73c53:	mov    0x30(%rsp),%r15
   73c58:	mov    0x68(%rsp),%rbp
   73c5d:	jb     733a3 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x273>
   73c63:	jmp    73c92 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb62>
   73c65:	mov    $0x1,%ecx
   73c6a:	mov    $0x1,%r8d
   73c70:	mov    0x68(%rsp),%r12
   73c75:	mov    %r12,%rdi
   73c78:	mov    %r13,%rsi
   73c7b:	mov    %rbp,%rdx
   73c7e:	call   82b00 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73c83:	mov    0x10(%r12),%r13
   73c88:	jmp    73460 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x330>
   73c8d:	mov    $0x1,%ebx
   73c92:	sub    %rbx,%r15
   73c95:	setae  %cl
   73c98:	cmp    0xd8(%rsp),%r15
   73ca0:	sete   %al
   73ca3:	and    %cl,%al
   73ca5:	neg    %al
   73ca7:	or     $0x7,%al
   73ca9:	jmp    73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73cab:	movzbl 0x41(%rsp),%eax
   73cb0:	add    $0x5f8,%rsp
   73cb7:	pop    %rbx
   73cb8:	pop    %r12
   73cba:	pop    %r13
   73cbc:	pop    %r14
   73cbe:	pop    %r15
   73cc0:	pop    %rbp
   73cc1:	ret
   73cc2:	mov    $0x7,%al
   73cc4:	jmp    73cb0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73cc6:	lea    0xad6e3(%rip),%rcx        # 1213b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73ccd:	xor    %edi,%edi
   73ccf:	mov    0x60(%rsp),%rsi
   73cd4:	call   *0xb2b3e(%rip)        # 126818 <_DYNAMIC+0x370>
   73cda:	mov    %rcx,%rdx
   73cdd:	lea    0xad6e4(%rip),%rcx        # 1213c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   73ce4:	mov    %rbp,%rdi
   73ce7:	mov    %rdx,%rsi
   73cea:	call   *0xb2b28(%rip)        # 126818 <_DYNAMIC+0x370>
   73cf0:	lea    0xad731(%rip),%rcx        # 121428 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2290>
   73cf7:	mov    $0x100,%edx
   73cfc:	xor    %edi,%edi
   73cfe:	mov    %r14,%rsi
   73d01:	call   *0xb2b11(%rip)        # 126818 <_DYNAMIC+0x370>
   73d07:	lea    0xad702(%rip),%rcx        # 121410 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2278>
   73d0e:	mov    %rsi,%rdi
   73d11:	mov    0x70(%rsp),%rsi
   73d16:	mov    %r15,%rdx
   73d19:	call   *0xb2af9(%rip)        # 126818 <_DYNAMIC+0x370>
   73d1f:	lea    0xb0f42(%rip),%rcx        # 124c68 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ad0>
   73d26:	mov    0x58(%rsp),%rdi
   73d2b:	mov    %r15,%rsi
   73d2e:	call   *0xb2ae4(%rip)        # 126818 <_DYNAMIC+0x370>
   73d34:	lea    0xad705(%rip),%rcx        # 121440 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x22a8>
   73d3b:	mov    0x70(%rsp),%rdi
   73d40:	mov    %r15,%rsi
   73d43:	mov    %r15,%rdx
   73d46:	call   *0xb2acc(%rip)        # 126818 <_DYNAMIC+0x370>
   73d4c:	lea    0xad615(%rip),%rcx        # 121368 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   73d53:	mov    %rdx,%rsi
   73d56:	call   *0xb2abc(%rip)        # 126818 <_DYNAMIC+0x370>
   73d5c:	lea    0xad61d(%rip),%rdx        # 121380 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   73d63:	mov    %r13,%rdi
   73d66:	mov    0x98(%rsp),%rsi
   73d6e:	call   *0xb2984(%rip)        # 1266f8 <_DYNAMIC+0x250>
   73d74:	lea    0xad61d(%rip),%rdx        # 121398 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73d7b:	mov    %r13,%rdi
   73d7e:	mov    0xa8(%rsp),%rsi
   73d86:	call   *0xb296c(%rip)        # 1266f8 <_DYNAMIC+0x250>
   73d8c:	lea    0xad5bd(%rip),%rdi        # 121350 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73d93:	call   *0xb2b47(%rip)        # 1268e0 <_DYNAMIC+0x438>
   73d99:	lea    0xad640(%rip),%rcx        # 1213e0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   73da0:	xor    %edi,%edi
   73da2:	mov    0x28(%rsp),%rsi
   73da7:	mov    0x50(%rsp),%rdx
   73dac:	call   *0xb2a66(%rip)        # 126818 <_DYNAMIC+0x370>
   73db2:	lea    0xad63f(%rip),%rcx        # 1213f8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73db9:	xor    %edi,%edi
   73dbb:	mov    0x28(%rsp),%rsi
   73dc0:	mov    0x50(%rsp),%rdx
   73dc5:	call   *0xb2a4d(%rip)        # 126818 <_DYNAMIC+0x370>
