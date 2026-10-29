
<artifacts>/screen3/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000073060 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   73060:	push   %rbp
   73061:	push   %r15
   73063:	push   %r14
   73065:	push   %r13
   73067:	push   %r12
   73069:	push   %rbx
   7306a:	sub    $0x598,%rsp
   73071:	mov    %rcx,%r15
   73074:	mov    %rdx,%r14
   73077:	mov    %rsi,%rbx
   7307a:	mov    %rdi,%r12
   7307d:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73082:	cmp    $0x2,%al
   73084:	jb     730b8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   73086:	cmp    $0x21,%rbx
   7308a:	lea    0xe3ef(%rip),%rax        # 81480 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73091:	lea    0x7bf8(%rip),%r9        # 7ac90 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   73098:	cmovb  %rax,%r9
   7309c:	mov    $0x4,%edx
   730a1:	mov    %r12,%rdi
   730a4:	mov    %rbx,%rsi
   730a7:	mov    %r14,%rcx
   730aa:	mov    %r15,%r8
   730ad:	call   *%r9
   730b0:	cmp    $0xfe,%al
   730b2:	jne    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   730b8:	mov    $0x7,%al
   730ba:	test   %r15,%r15
   730bd:	je     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   730c3:	movzbl (%r14),%ebp
   730c7:	mov    %ebp,%ecx
   730c9:	and    $0xf0,%cl
   730cc:	cmp    $0xa0,%cl
   730cf:	jne    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   730d5:	and    $0xf,%bpl
   730d9:	mov    $0x5,%al
   730db:	cmp    $0x1,%bpl
   730df:	ja     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   730e5:	xor    %eax,%eax
   730e7:	test   %bpl,%bpl
   730ea:	sete   %al
   730ed:	lea    0x18(,%rax,8),%rcx
   730f5:	mov    $0x7,%al
   730f7:	cmp    %rcx,%r15
   730fa:	jbe    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73100:	cmp    $0x1,%bpl
   73104:	mov    %r15,%rax
   73107:	adc    $0x0,%rax
   7310b:	lea    -0x1(%rax),%rdx
   7310f:	mov    %rdx,0x70(%rsp)
   73114:	cmp    %r15,%rdx
   73117:	ja     73ceb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   7311d:	mov    %rcx,0x88(%rsp)
   73125:	add    $0xfffffffffffffffb,%rax
   73129:	pxor   %xmm0,%xmm0
   7312d:	movdqu %xmm0,0x188(%rsp)
   73136:	movdqu %xmm0,0x17c(%rsp)
   7313f:	movdqu %xmm0,0x16c(%rsp)
   73148:	movdqu %xmm0,0x15c(%rsp)
   73151:	movdqu %xmm0,0x14c(%rsp)
   7315a:	movdqu %xmm0,0x13c(%rsp)
   73163:	movdqu %xmm0,0x12c(%rsp)
   7316c:	movdqu %xmm0,0x11c(%rsp)
   73175:	movdqu %xmm0,0x10c(%rsp)
   7317e:	movdqu %xmm0,0xfc(%rsp)
   73187:	movdqu %xmm0,0xec(%rsp)
   73190:	movdqu %xmm0,0xdc(%rsp)
   73199:	movdqu %xmm0,0xcc(%rsp)
   731a2:	movdqu %xmm0,0xbc(%rsp)
   731ab:	movdqu %xmm0,0xac(%rsp)
   731b4:	movdqu %xmm0,0x9c(%rsp)
   731bd:	mov    (%r14,%rax,1),%r13d
   731c1:	mov    %r13d,0x98(%rsp)
   731c9:	lea    0x198(%rsp),%rdi
   731d1:	mov    $0x400,%edx
   731d6:	xor    %esi,%esi
   731d8:	call   *0xb5f4a(%rip)        # 129128 <memset@GLIBC_2.2.5>
   731de:	test   %rbx,%rbx
   731e1:	je     73209 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   731e3:	mov    %r13d,0x5c(%rsp)
   731e8:	mov    $0x1,%r8d
   731ee:	xor    %ecx,%ecx
   731f0:	mov    %r15,0x50(%rsp)
   731f5:	mov    %r14,0x28(%rsp)
   731fa:	mov    %rbx,0x80(%rsp)
   73202:	mov    %bpl,0x27(%rsp)
   73207:	jmp    73288 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   73209:	mov    $0x1,%r8d
   7320f:	sub    %r8,%r15
   73212:	setae  %cl
   73215:	cmp    0x88(%rsp),%r15
   7321d:	sete   %al
   73220:	and    %cl,%al
   73222:	neg    %al
   73224:	or     $0x7,%al
   73226:	jmp    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7322b:	mov    $0x1,%ecx
   73230:	mov    $0x1,%r8d
   73236:	mov    %r13,%rdi
   73239:	mov    %rbp,%rsi
   7323c:	mov    %r15,%rdx
   7323f:	call   83ee0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   73244:	mov    0x10(%r13),%rbp
   73248:	mov    0x8(%r13),%rdi
   7324c:	add    %rbp,%rdi
   7324f:	lea    0x8(%r12),%rsi
   73254:	mov    %r15,%rdx
   73257:	call   *0xb5ebb(%rip)        # 129118 <memcpy@GLIBC_2.14>
   7325d:	add    %r15,%rbp
   73260:	mov    %rbp,0x10(%r13)
   73264:	mov    0x78(%rsp),%rax
   73269:	mov    %rax,%rcx
   7326c:	mov    0x80(%rsp),%rbx
   73274:	cmp    %rbx,%rax
   73277:	mov    0x50(%rsp),%r15
   7327c:	movzbl 0x27(%rsp),%ebp
   73281:	mov    0x48(%rsp),%r8
   73286:	jae    7320f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   73288:	sub    %rcx,%rbx
   7328b:	cmp    $0x100,%rbx
   73292:	mov    $0x100,%r13d
   73298:	cmovb  %rbx,%r13
   7329c:	lea    0xf(%r13),%r9d
   732a0:	and    $0x3f0,%r9d
   732a7:	test   %bpl,%bpl
   732aa:	mov    %r9,0x48(%rsp)
   732af:	mov    %rcx,0x90(%rsp)
   732b7:	mov    %rbx,0x60(%rsp)
   732bc:	je     7334d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   732c2:	mov    $0x7,%al
   732c4:	cmp    %r15,%r8
   732c7:	jae    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   732cd:	add    %r13,%rcx
   732d0:	mov    %rcx,0x78(%rsp)
   732d5:	lea    0x0(,%r13,4),%rbp
   732dd:	mov    %rbp,0x408(%r12)
   732e5:	movzbl (%r14,%r8,1),%r14d
   732ea:	inc    %r8
   732ed:	mov    %r14d,%ecx
   732f0:	and    $0x3,%ecx
   732f3:	lea    0x0(,%r13,2),%rdx
   732fb:	add    %r13,%rdx
   732fe:	mov    %rdx,0x68(%rsp)
   73303:	cmp    %r9,%r13
   73306:	ja     73a59 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   7330c:	cmp    $0x2,%cl
   7330f:	je     7338d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   73311:	movzbl %cl,%edx
   73314:	cmp    $0x3,%edx
   73317:	jne    733a5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   7331d:	lea    (%r8,%r13,1),%rbx
   73321:	cmp    %r15,%rbx
   73324:	ja     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7332a:	add    0x28(%rsp),%r8
   7332f:	lea    0x198(%rsp),%rdi
   73337:	mov    %r8,%rsi
   7333a:	mov    %r13,%rdx
   7333d:	call   *0xb5dd5(%rip)        # 129118 <memcpy@GLIBC_2.14>
   73343:	mov    0x50(%rsp),%r15
   73348:	jmp    733ef <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7334d:	lea    (%rcx,%r13,1),%rax
   73351:	mov    %rax,0x78(%rsp)
   73356:	lea    0x0(,%r13,4),%rbp
   7335e:	mov    %rbp,0x408(%r12)
   73366:	xor    %r14d,%r14d
   73369:	lea    0x0(,%r13,2),%rax
   73371:	add    %r13,%rax
   73374:	mov    %rax,0x68(%rsp)
   73379:	lea    -0x65a50(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73380:	mov    %rcx,%rax
   73383:	cmp    %r9,%r13
   73386:	jbe    733be <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   73388:	jmp    73a84 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   7338d:	lea    0x198(%rsp),%rdi
   73395:	xor    %esi,%esi
   73397:	mov    %r13,%rdx
   7339a:	mov    %r8,%rbx
   7339d:	call   *0xb5d85(%rip)        # 129128 <memset@GLIBC_2.2.5>
   733a3:	jmp    733ef <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   733a5:	test   %cl,%cl
   733a7:	lea    -0x6577e(%rip),%rcx        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   733ae:	lea    -0x65665(%rip),%rax        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   733b5:	cmove  %rax,%rcx
   733b9:	mov    0x48(%rsp),%r9
   733be:	mov    %rcx,(%rsp)
   733c2:	lea    0x30(%rsp),%rdi
   733c7:	mov    0x28(%rsp),%rsi
   733cc:	mov    %r15,%rdx
   733cf:	mov    %r8,%rcx
   733d2:	lea    0x198(%rsp),%r8
   733da:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   733df:	cmpb   $0x0,0x30(%rsp)
   733e4:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   733ea:	mov    0x38(%rsp),%rbx
   733ef:	mov    %r14d,%eax
   733f2:	shr    $0x2,%al
   733f5:	and    $0x3,%al
   733f7:	lea    (%rsp,%r13,1),%r8
   733fb:	add    $0x198,%r8
   73402:	cmp    $0x3,%al
   73404:	je     73421 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   73406:	movzbl %al,%ecx
   73409:	cmp    $0x2,%ecx
   7340c:	jne    73458 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   7340e:	mov    %r8,%rdi
   73411:	xor    %esi,%esi
   73413:	mov    %r13,%rdx
   73416:	call   *0xb5d0c(%rip)        # 129128 <memset@GLIBC_2.2.5>
   7341c:	jmp    734aa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   73421:	mov    %rbx,%r15
   73424:	mov    $0x7,%al
   73426:	add    %r13,%r15
   73429:	jb     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7342f:	cmp    0x50(%rsp),%r15
   73434:	ja     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7343a:	add    0x28(%rsp),%rbx
   7343f:	mov    %r8,%rdi
   73442:	mov    %rbx,%rsi
   73445:	mov    %r13,%rdx
   73448:	call   *0xb5cca(%rip)        # 129118 <memcpy@GLIBC_2.14>
   7344e:	mov    %r15,%rbx
   73451:	mov    0x50(%rsp),%r15
   73456:	jmp    734aa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   73458:	test   %al,%al
   7345a:	lea    -0x65831(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73461:	lea    -0x65718(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73468:	cmove  %rcx,%rax
   7346c:	cmpb   $0x0,0x27(%rsp)
   73471:	lea    -0x65b48(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73478:	cmove  %rcx,%rax
   7347c:	mov    %rax,(%rsp)
   73480:	lea    0x30(%rsp),%rdi
   73485:	mov    0x28(%rsp),%rsi
   7348a:	mov    %r15,%rdx
   7348d:	mov    %rbx,%rcx
   73490:	mov    0x48(%rsp),%r9
   73495:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   7349a:	cmpb   $0x0,0x30(%rsp)
   7349f:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   734a5:	mov    0x38(%rsp),%rbx
   734aa:	mov    %r14d,%eax
   734ad:	shr    $0x4,%al
   734b0:	and    $0x3,%al
   734b2:	lea    (%rsp,%r13,2),%r8
   734b6:	add    $0x198,%r8
   734bd:	cmp    $0x3,%al
   734bf:	je     734dc <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   734c1:	movzbl %al,%ecx
   734c4:	cmp    $0x2,%ecx
   734c7:	jne    73513 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   734c9:	mov    %r8,%rdi
   734cc:	xor    %esi,%esi
   734ce:	mov    %r13,%rdx
   734d1:	call   *0xb5c51(%rip)        # 129128 <memset@GLIBC_2.2.5>
   734d7:	jmp    73565 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   734dc:	mov    %rbx,%r15
   734df:	mov    $0x7,%al
   734e1:	add    %r13,%r15
   734e4:	jb     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   734ea:	cmp    0x50(%rsp),%r15
   734ef:	ja     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   734f5:	add    0x28(%rsp),%rbx
   734fa:	mov    %r8,%rdi
   734fd:	mov    %rbx,%rsi
   73500:	mov    %r13,%rdx
   73503:	call   *0xb5c0f(%rip)        # 129118 <memcpy@GLIBC_2.14>
   73509:	mov    %r15,%rbx
   7350c:	mov    0x50(%rsp),%r15
   73511:	jmp    73565 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   73513:	test   %al,%al
   73515:	lea    -0x658ec(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7351c:	lea    -0x657d3(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73523:	cmove  %rcx,%rax
   73527:	cmpb   $0x0,0x27(%rsp)
   7352c:	lea    -0x65c03(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73533:	cmove  %rcx,%rax
   73537:	mov    %rax,(%rsp)
   7353b:	lea    0x30(%rsp),%rdi
   73540:	mov    0x28(%rsp),%rsi
   73545:	mov    %r15,%rdx
   73548:	mov    %rbx,%rcx
   7354b:	mov    0x48(%rsp),%r9
   73550:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73555:	cmpb   $0x0,0x30(%rsp)
   7355a:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73560:	mov    0x38(%rsp),%rbx
   73565:	shr    $0x6,%r14b
   73569:	mov    0x68(%rsp),%rax
   7356e:	lea    (%rsp,%rax,1),%r8
   73572:	add    $0x198,%r8
   73579:	cmp    $0x3,%r14b
   7357d:	je     7359b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   7357f:	movzbl %r14b,%eax
   73583:	cmp    $0x2,%eax
   73586:	jne    735e2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   73588:	mov    %r8,%rdi
   7358b:	xor    %esi,%esi
   7358d:	mov    %r13,%rdx
   73590:	call   *0xb5b92(%rip)        # 129128 <memset@GLIBC_2.2.5>
   73596:	jmp    73635 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7359b:	mov    %rbx,%rcx
   7359e:	mov    $0x7,%al
   735a0:	add    %r13,%rcx
   735a3:	jb     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   735a9:	cmp    %r15,%rcx
   735ac:	ja     73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   735b2:	mov    0x28(%rsp),%r14
   735b7:	add    %r14,%rbx
   735ba:	mov    %r8,%rdi
   735bd:	mov    %rbx,%rsi
   735c0:	mov    %r13,%rdx
   735c3:	mov    %rcx,%rbx
   735c6:	call   *0xb5b4c(%rip)        # 129118 <memcpy@GLIBC_2.14>
   735cc:	mov    0x50(%rsp),%r15
   735d1:	cmpb   $0x0,0x27(%rsp)
   735d6:	mov    %rbx,0x48(%rsp)
   735db:	jne    73646 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   735dd:	jmp    7366e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   735e2:	test   %r14b,%r14b
   735e5:	lea    -0x659bc(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   735ec:	lea    -0x658a3(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   735f3:	cmove  %rcx,%rax
   735f7:	cmpb   $0x0,0x27(%rsp)
   735fc:	lea    -0x65cd3(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73603:	cmove  %rcx,%rax
   73607:	mov    %rax,(%rsp)
   7360b:	lea    0x30(%rsp),%rdi
   73610:	mov    0x28(%rsp),%rsi
   73615:	mov    %r15,%rdx
   73618:	mov    %rbx,%rcx
   7361b:	mov    0x48(%rsp),%r9
   73620:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73625:	cmpb   $0x0,0x30(%rsp)
   7362a:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73630:	mov    0x38(%rsp),%rbx
   73635:	mov    0x28(%rsp),%r14
   7363a:	cmpb   $0x0,0x27(%rsp)
   7363f:	mov    %rbx,0x48(%rsp)
   73644:	je     7366e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   73646:	mov    0x70(%rsp),%rax
   7364b:	cmp    %rax,%r15
   7364e:	je     73d19 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   73654:	movzbl (%r14,%rax,1),%eax
   73659:	mov    %eax,%ecx
   7365b:	and    $0x3,%ecx
   7365e:	lea    -0x62ebd(%rip),%rdx        # 107a8 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   73665:	movslq (%rdx,%rcx,4),%rcx
   73669:	add    %rdx,%rcx
   7366c:	jmp    *%rcx
   7366e:	call   e47c0 <meshoptimizer_rs::codec::simd::dispatch::selected>
   73673:	test   %al,%al
   73675:	je     73944 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   7367b:	movd   0x5c(%rsp),%xmm0
   73681:	pshufd $0x0,%xmm0,%xmm0
   73686:	mov    %r13,%rcx
   73689:	shr    $0x4,%rcx
   7368d:	je     738e9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   73693:	lea    0x0(,%r13,2),%rdx
   7369b:	lea    0x0(,%r13,2),%rsi
   736a3:	add    %r13,%rsi
   736a6:	mov    %r13,%r8
   736a9:	neg    %r8
   736ac:	mov    %rdx,%r10
   736af:	neg    %r10
   736b2:	mov    %rsi,%r14
   736b5:	neg    %r14
   736b8:	lea    (%rsp,%rsi,1),%r15
   736bc:	add    $0x198,%r15
   736c3:	shl    $0x4,%rcx
   736c7:	xor    %eax,%eax
   736c9:	mov    %rbp,%r9
   736cc:	xor    %edi,%edi
   736ce:	movdqa -0x65d77(%rip),%xmm8        # d960 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   736d7:	movdqa -0x65f10(%rip),%xmm9        # d7d0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x901>
   736e0:	cmp    %rbp,%rdi
   736e3:	ja     73caa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   736e9:	mov    %r9,%r11
   736ec:	sub    $0x10,%r11
   736f0:	jb     73c21 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   736f6:	lea    (%rdi,%r13,1),%rbx
   736fa:	cmp    %rbp,%rbx
   736fd:	ja     73c49 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   73703:	lea    (%r8,%r9,1),%rbx
   73707:	cmp    $0x10,%rbx
   7370b:	jb     73c21 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   73711:	lea    (%rdx,%rdi,1),%rbx
   73715:	cmp    %rbp,%rbx
   73718:	ja     73c60 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   7371e:	lea    (%r10,%r9,1),%rbx
   73722:	cmp    $0x10,%rbx
   73726:	jb     73c21 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7372c:	lea    (%rsi,%rdi,1),%rbx
   73730:	cmp    %rbp,%rbx
   73733:	ja     73c8e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   73739:	add    %r14,%r9
   7373c:	cmp    $0x10,%r9
   73740:	jb     73c21 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   73746:	lea    0x3f(%rax),%r9
   7374a:	cmp    %rbp,%r9
   7374d:	jae    73c32 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   73753:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   7375c:	lea    (%rsp,%r13,1),%r9
   73760:	add    $0x198,%r9
   73767:	movdqu (%r9,%rdi,1),%xmm4
   7376d:	lea    0x198(%rsp,%r13,2),%r9
   73775:	movdqu (%r9,%rdi,1),%xmm5
   7377b:	movdqa %xmm3,%xmm2
   7377f:	punpckhbw %xmm4,%xmm2
   73783:	movdqu (%r15,%rdi,1),%xmm6
   73789:	movdqa %xmm5,%xmm7
   7378d:	punpckhbw %xmm6,%xmm7
   73791:	movdqa %xmm2,%xmm1
   73795:	punpckhwd %xmm7,%xmm1
   73799:	punpcklwd %xmm7,%xmm2
   7379d:	punpcklbw %xmm4,%xmm3
   737a1:	punpcklbw %xmm6,%xmm5
   737a5:	movdqa %xmm3,%xmm4
   737a9:	punpckhwd %xmm5,%xmm4
   737ad:	punpcklwd %xmm5,%xmm3
   737b1:	movdqa %xmm3,%xmm5
   737b5:	psrlw  $0x1,%xmm5
   737ba:	pand   %xmm8,%xmm3
   737bf:	pxor   %xmm6,%xmm6
   737c3:	psubb  %xmm3,%xmm6
   737c7:	pand   %xmm9,%xmm5
   737cc:	pxor   %xmm6,%xmm5
   737d0:	movdqa %xmm5,%xmm3
   737d4:	pslldq $0x4,%xmm3
   737d9:	paddb  %xmm5,%xmm3
   737dd:	movdqa %xmm3,%xmm5
   737e1:	pslldq $0x8,%xmm5
   737e6:	paddb  %xmm0,%xmm3
   737ea:	paddb  %xmm5,%xmm3
   737ee:	movdqu %xmm3,0x8(%r12,%rdi,4)
   737f5:	movdqa %xmm4,%xmm0
   737f9:	psrlw  $0x1,%xmm0
   737fe:	pand   %xmm8,%xmm4
   73803:	pxor   %xmm5,%xmm5
   73807:	psubb  %xmm4,%xmm5
   7380b:	pand   %xmm9,%xmm0
   73810:	pxor   %xmm5,%xmm0
   73814:	movdqa %xmm0,%xmm4
   73818:	pslldq $0x4,%xmm4
   7381d:	paddb  %xmm0,%xmm4
   73821:	movdqa %xmm4,%xmm0
   73825:	pslldq $0x8,%xmm0
   7382a:	pshufd $0xff,%xmm3,%xmm3
   7382f:	paddb  %xmm4,%xmm0
   73833:	paddb  %xmm3,%xmm0
   73837:	movdqu %xmm0,0x18(%r12,%rdi,4)
   7383e:	movdqa %xmm2,%xmm3
   73842:	psrlw  $0x1,%xmm3
   73847:	pand   %xmm8,%xmm2
   7384c:	pxor   %xmm4,%xmm4
   73850:	psubb  %xmm2,%xmm4
   73854:	pand   %xmm9,%xmm3
   73859:	pxor   %xmm4,%xmm3
   7385d:	movdqa %xmm3,%xmm2
   73861:	pslldq $0x4,%xmm2
   73866:	paddb  %xmm3,%xmm2
   7386a:	movdqa %xmm2,%xmm3
   7386e:	pslldq $0x8,%xmm3
   73873:	pshufd $0xff,%xmm0,%xmm0
   73878:	paddb  %xmm2,%xmm3
   7387c:	paddb  %xmm0,%xmm3
   73880:	movdqu %xmm3,0x28(%r12,%rdi,4)
   73887:	movdqa %xmm1,%xmm0
   7388b:	psrlw  $0x1,%xmm0
   73890:	pand   %xmm8,%xmm1
   73895:	pxor   %xmm2,%xmm2
   73899:	psubb  %xmm1,%xmm2
   7389d:	pand   %xmm9,%xmm0
   738a2:	pxor   %xmm2,%xmm0
   738a6:	movdqa %xmm0,%xmm1
   738aa:	pslldq $0x4,%xmm1
   738af:	paddb  %xmm0,%xmm1
   738b3:	movdqa %xmm1,%xmm0
   738b7:	pslldq $0x8,%xmm0
   738bc:	pshufd $0xff,%xmm3,%xmm2
   738c1:	paddb  %xmm1,%xmm0
   738c5:	paddb  %xmm2,%xmm0
   738c9:	movdqu %xmm0,0x38(%r12,%rdi,4)
   738d0:	add    $0x10,%rdi
   738d4:	pshufd $0xff,%xmm0,%xmm0
   738d9:	add    $0x40,%rax
   738dd:	mov    %r11,%r9
   738e0:	cmp    %rdi,%rcx
   738e3:	jne    736e0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   738e9:	mov    %r13d,%eax
   738ec:	and    $0x1f0,%eax
   738f1:	cmp    %r13,%rax
   738f4:	mov    0x28(%rsp),%r14
   738f9:	je     739f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   738ff:	movdqa %xmm0,0x30(%rsp)
   73905:	lea    0x0(,%rax,4),%edx
   7390c:	mov    %rbp,%rcx
   7390f:	sub    %rdx,%rcx
   73912:	jb     73d03 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   73918:	lea    0x8(%r12),%rsi
   7391d:	add    %rsi,%rdx
   73920:	mov    %rax,0x8(%rsp)
   73925:	lea    0x30(%rsp),%rax
   7392a:	mov    %rax,(%rsp)
   7392e:	mov    $0x4,%r9d
   73934:	lea    0x198(%rsp),%rdi
   7393c:	mov    %r13,%r8
   7393f:	mov    %rbp,%rsi
   73942:	jmp    73975 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   73944:	lea    0x98(%rsp),%rax
   7394c:	mov    %rax,(%rsp)
   73950:	movq   $0x0,0x8(%rsp)
   73959:	mov    $0x4,%r9d
   7395f:	lea    0x198(%rsp),%rdi
   73967:	mov    %rbp,%rsi
   7396a:	lea    0x8(%r12),%rdx
   7396f:	mov    %r13,%r8
   73972:	mov    %rbp,%rcx
   73975:	call   72b40 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7397a:	jmp    739f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7397c:	shr    $0x4,%al
   7397f:	neg    %al
   73981:	and    $0x1f,%al
   73983:	movzbl %al,%eax
   73986:	lea    0x98(%rsp),%rcx
   7398e:	mov    %rcx,(%rsp)
   73992:	mov    %eax,0x8(%rsp)
   73996:	movq   $0x0,0x10(%rsp)
   7399f:	mov    $0x4,%r9d
   739a5:	lea    0x198(%rsp),%rdi
   739ad:	mov    %rbp,%rsi
   739b0:	lea    0x8(%r12),%rdx
   739b5:	mov    %r13,%r8
   739b8:	mov    %rbp,%rcx
   739bb:	call   72ef0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   739c0:	jmp    739f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   739c2:	lea    0x98(%rsp),%rax
   739ca:	mov    %rax,(%rsp)
   739ce:	movq   $0x0,0x8(%rsp)
   739d7:	mov    $0x4,%r9d
   739dd:	lea    0x198(%rsp),%rdi
   739e5:	mov    %rbp,%rsi
   739e8:	lea    0x8(%r12),%rdx
   739ed:	mov    %r13,%r8
   739f0:	mov    %rbp,%rcx
   739f3:	call   72d30 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   739f8:	mov    0x90(%rsp),%rax
   73a00:	cmp    %rax,0x80(%rsp)
   73a08:	je     73cd4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   73a0e:	mov    0x4(%r12,%rbp,1),%eax
   73a13:	mov    %eax,0x98(%rsp)
   73a1a:	mov    0x408(%r12),%r15
   73a22:	cmp    $0x401,%r15
   73a29:	jae    73cbd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   73a2f:	mov    %eax,0x5c(%rsp)
   73a33:	mov    (%r12),%r13
   73a37:	mov    0x0(%r13),%rax
   73a3b:	mov    0x10(%r13),%rbp
   73a3f:	sub    %rbp,%rax
   73a42:	cmp    %rax,%r15
   73a45:	ja     7322b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   73a4b:	test   %r15,%r15
   73a4e:	jne    73248 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   73a54:	jmp    7325d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   73a59:	cmp    $0x2,%ecx
   73a5c:	je     73d41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   73a62:	cmp    $0x3,%ecx
   73a65:	je     73d2a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   73a6b:	test   %cl,%cl
   73a6d:	lea    -0x65e44(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73a74:	lea    -0x65d2b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73a7b:	cmove  %rcx,%rax
   73a7f:	mov    0x48(%rsp),%r9
   73a84:	mov    %rax,(%rsp)
   73a88:	lea    0x30(%rsp),%rdi
   73a8d:	mov    0x28(%rsp),%rsi
   73a92:	mov    %r15,%rdx
   73a95:	mov    %r8,%rcx
   73a98:	lea    0x198(%rsp),%r8
   73aa0:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73aa5:	cmpb   $0x0,0x30(%rsp)
   73aaa:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73ab0:	mov    %r14d,%eax
   73ab3:	shr    $0x2,%al
   73ab6:	and    $0x3,%al
   73ab8:	cmp    $0x2,%al
   73aba:	je     73d41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   73ac0:	movzbl %al,%ecx
   73ac3:	cmp    $0x3,%ecx
   73ac6:	je     73d2a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   73acc:	mov    0x38(%rsp),%rcx
   73ad1:	test   %al,%al
   73ad3:	lea    -0x65eaa(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73ada:	lea    -0x65d91(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73ae1:	cmove  %rdx,%rax
   73ae5:	cmpb   $0x0,0x27(%rsp)
   73aea:	lea    0x198(%rsp,%r13,1),%r8
   73af2:	lea    -0x661c9(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73af9:	cmove  %rdx,%rax
   73afd:	mov    %rax,(%rsp)
   73b01:	lea    0x30(%rsp),%rdi
   73b06:	mov    0x28(%rsp),%rsi
   73b0b:	mov    %r15,%rdx
   73b0e:	mov    0x48(%rsp),%r9
   73b13:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73b18:	cmpb   $0x0,0x30(%rsp)
   73b1d:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73b23:	mov    %r14d,%eax
   73b26:	shr    $0x4,%al
   73b29:	and    $0x3,%al
   73b2b:	cmp    $0x2,%al
   73b2d:	je     73d41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   73b33:	movzbl %al,%ecx
   73b36:	cmp    $0x3,%ecx
   73b39:	je     73d2a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   73b3f:	mov    0x38(%rsp),%rcx
   73b44:	test   %al,%al
   73b46:	lea    -0x65f1d(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73b4d:	lea    -0x65e04(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73b54:	cmove  %rdx,%rax
   73b58:	cmpb   $0x0,0x27(%rsp)
   73b5d:	lea    0x198(%rsp,%r13,2),%r8
   73b65:	lea    -0x6623c(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73b6c:	cmove  %rdx,%rax
   73b70:	mov    %rax,(%rsp)
   73b74:	lea    0x30(%rsp),%rdi
   73b79:	mov    0x28(%rsp),%rsi
   73b7e:	mov    %r15,%rdx
   73b81:	mov    0x48(%rsp),%r9
   73b86:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73b8b:	cmpb   $0x0,0x30(%rsp)
   73b90:	jne    73c0a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   73b92:	shr    $0x6,%r14b
   73b96:	cmp    $0x2,%r14b
   73b9a:	je     73d41 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   73ba0:	movzbl %r14b,%eax
   73ba4:	cmp    $0x3,%eax
   73ba7:	je     73d2a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   73bad:	mov    0x38(%rsp),%rcx
   73bb2:	test   %r14b,%r14b
   73bb5:	lea    -0x65f8c(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73bbc:	lea    -0x65e73(%rip),%rdx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73bc3:	cmove  %rdx,%rax
   73bc7:	cmpb   $0x0,0x27(%rsp)
   73bcc:	mov    0x68(%rsp),%rdx
   73bd1:	lea    0x198(%rsp,%rdx,1),%r8
   73bd9:	lea    -0x662b0(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73be0:	cmove  %rdx,%rax
   73be4:	mov    %rax,(%rsp)
   73be8:	lea    0x30(%rsp),%rdi
   73bed:	mov    0x28(%rsp),%rsi
   73bf2:	mov    %r15,%rdx
   73bf5:	mov    0x48(%rsp),%r9
   73bfa:	call   d5f60 <meshoptimizer_rs::codec::vertex::bytes>
   73bff:	cmpb   $0x0,0x30(%rsp)
   73c04:	je     73630 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   73c0a:	movzbl 0x31(%rsp),%eax
   73c0f:	add    $0x598,%rsp
   73c16:	pop    %rbx
   73c17:	pop    %r12
   73c19:	pop    %r13
   73c1b:	pop    %r14
   73c1d:	pop    %r15
   73c1f:	pop    %rbp
   73c20:	ret
   73c21:	lea    0xb0d38(%rip),%rdi        # 124960 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2db8>
   73c28:	call   *0xb569a(%rip)        # 1292c8 <_DYNAMIC+0x3c8>
   73c2e:	mov    $0x7,%al
   73c30:	jmp    73c0f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   73c32:	lea    0x40(%rax),%rsi
   73c36:	lea    0xb052b(%rip),%rcx        # 124168 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25c0>
   73c3d:	mov    %rax,%rdi
   73c40:	mov    %rbp,%rdx
   73c43:	call   *0xb5627(%rip)        # 129270 <_DYNAMIC+0x370>
   73c49:	mov    0x60(%rsp),%rcx
   73c4e:	cmp    $0x100,%rcx
   73c55:	mov    $0x100,%eax
   73c5a:	cmovb  %rcx,%rax
   73c5e:	jmp    73ca7 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   73c60:	mov    0x60(%rsp),%rcx
   73c65:	cmp    $0x100,%rcx
   73c6c:	mov    $0x100,%eax
   73c71:	cmovb  %rcx,%rax
   73c75:	mov    %eax,%eax
   73c77:	lea    (%rdi,%rax,2),%rdi
   73c7b:	lea    0xb0cc6(%rip),%rcx        # 124948 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   73c82:	mov    %rbp,%rsi
   73c85:	mov    %rbp,%rdx
   73c88:	call   *0xb55e2(%rip)        # 129270 <_DYNAMIC+0x370>
   73c8e:	mov    0x60(%rsp),%rcx
   73c93:	cmp    $0x100,%rcx
   73c9a:	mov    $0x100,%eax
   73c9f:	cmovb  %rcx,%rax
   73ca3:	lea    (%rax,%rax,2),%rax
   73ca7:	add    %rax,%rdi
   73caa:	lea    0xb0c97(%rip),%rcx        # 124948 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2da0>
   73cb1:	mov    %rbp,%rsi
   73cb4:	mov    %rbp,%rdx
   73cb7:	call   *0xb55b3(%rip)        # 129270 <_DYNAMIC+0x370>
   73cbd:	lea    0xb399c(%rip),%rcx        # 127660 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5ab8>
   73cc4:	mov    $0x400,%edx
   73cc9:	xor    %edi,%edi
   73ccb:	mov    %r15,%rsi
   73cce:	call   *0xb559c(%rip)        # 129270 <_DYNAMIC+0x370>
   73cd4:	lea    -0x4(%rbp),%rdi
   73cd8:	lea    0xb0051(%rip),%rcx        # 123d30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   73cdf:	mov    %rbp,%rsi
   73ce2:	mov    %rbp,%rdx
   73ce5:	call   *0xb5585(%rip)        # 129270 <_DYNAMIC+0x370>
   73ceb:	lea    0xb0116(%rip),%rcx        # 123e08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73cf2:	mov    0x70(%rsp),%rdi
   73cf7:	mov    %r15,%rsi
   73cfa:	mov    %r15,%rdx
   73cfd:	call   *0xb556d(%rip)        # 129270 <_DYNAMIC+0x370>
   73d03:	lea    0xb0446(%rip),%rcx        # 124150 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x25a8>
   73d0a:	mov    %rdx,%rdi
   73d0d:	mov    %rbp,%rsi
   73d10:	mov    %rbp,%rdx
   73d13:	call   *0xb5557(%rip)        # 129270 <_DYNAMIC+0x370>
   73d19:	lea    0xb0040(%rip),%rdx        # 123d60 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73d20:	xor    %edi,%edi
   73d22:	xor    %esi,%esi
   73d24:	call   *0xb5426(%rip)        # 129150 <_DYNAMIC+0x250>
   73d2a:	lea    0xb0077(%rip),%rcx        # 123da8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73d31:	xor    %edi,%edi
   73d33:	mov    %r13,%rsi
   73d36:	mov    0x48(%rsp),%rdx
   73d3b:	call   *0xb552f(%rip)        # 129270 <_DYNAMIC+0x370>
   73d41:	lea    0xb0078(%rip),%rcx        # 123dc0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73d48:	xor    %edi,%edi
   73d4a:	mov    %r13,%rsi
   73d4d:	mov    0x48(%rsp),%rdx
   73d52:	call   *0xb5518(%rip)        # 129270 <_DYNAMIC+0x370>
