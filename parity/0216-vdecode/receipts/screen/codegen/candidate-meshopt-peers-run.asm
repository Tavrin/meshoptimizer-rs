
<artifacts>/screen/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000034d30 <meshopt_peers::run>:
   34d30:	push   %rbp
   34d31:	push   %r15
   34d33:	push   %r14
   34d35:	push   %r13
   34d37:	push   %r12
   34d39:	push   %rbx
   34d3a:	sub    $0x1000,%rsp
   34d41:	movq   $0x0,(%rsp)
   34d49:	sub    $0x968,%rsp
   34d50:	mov    %rsi,0x290(%rsp)
   34d58:	mov    %rdx,0x298(%rsp)
   34d60:	mov    0x10(%rcx),%rbx
   34d64:	mov    0x8(%rcx),%r15
   34d68:	movb   $0x2,0x6e0(%rsp)
   34d70:	mov    %r15,0x6e8(%rsp)
   34d78:	mov    %rbx,0x6f0(%rsp)
   34d80:	movq   $0x0,0x718(%rsp)
   34d8c:	mov    0x78(%rcx),%rax
   34d90:	movd   0x80(%rcx),%xmm0
   34d98:	mov    %rax,0x220(%rsp)
   34da0:	movd   %xmm0,0x228(%rsp)
   34da9:	movl   $0x0,0x22c(%rsp)
   34db4:	add    $0xfffffffffffffffb,%rdx
   34db8:	cmp    $0xe,%rdx
   34dbc:	ja     36e8e <meshopt_peers::run+0x215e>
   34dc2:	mov    %r8,%r12
   34dc5:	mov    %rdi,%r14
   34dc8:	lea    -0x248d3(%rip),%rdi        # 104fc <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   34dcf:	movslq (%rdi,%rdx,4),%rdx
   34dd3:	add    %rdi,%rdx
   34dd6:	jmp    *%rdx
   34dd8:	mov    $0x68636163,%eax
   34ddd:	xor    (%rsi),%eax
   34ddf:	movzbl 0x4(%rsi),%edx
   34de3:	xor    $0x65,%edx
   34de6:	or     %eax,%edx
   34de8:	je     355d2 <meshopt_peers::run+0x8a2>
   34dee:	mov    $0x63746566,%eax
   34df3:	xor    (%rsi),%eax
   34df5:	movzbl 0x4(%rsi),%edx
   34df9:	xor    $0x68,%edx
   34dfc:	or     %eax,%edx
   34dfe:	jne    36e8e <meshopt_peers::run+0x215e>
   34e04:	mov    %rbx,%rax
   34e07:	shr    $0x20,%rax
   34e0b:	jne    372e1 <meshopt_peers::run+0x25b1>
   34e11:	mov    0x20(%rcx),%rsi
   34e15:	mov    0x28(%rcx),%rdx
   34e19:	lea    0x0(,%rbx,4),%rax
   34e21:	lea    (%rax,%rax,2),%rax
   34e25:	mov    %r15,0x350(%rsp)
   34e2d:	mov    %rax,0x358(%rsp)
   34e35:	mov    %rbx,0x360(%rsp)
   34e3d:	movq   $0xc,0x368(%rsp)
   34e49:	movq   $0xc,0x370(%rsp)
   34e55:	movq   $0x0,0x378(%rsp)
   34e61:	lea    0x738(%rsp),%rdi
   34e69:	lea    0x350(%rsp),%rcx
   34e71:	mov    %r12,%r8
   34e74:	call   *0xf5cc6(%rip)        # 12ab40 <_DYNAMIC+0x5d0>
   34e7a:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34e83:	je     36f15 <meshopt_peers::run+0x21e5>
   34e89:	mov    0x768(%rsp),%rax
   34e91:	mov    %rax,0x380(%rsp)
   34e99:	movups 0x738(%rsp),%xmm0
   34ea1:	movups 0x748(%rsp),%xmm1
   34ea9:	movups 0x758(%rsp),%xmm2
   34eb1:	movaps %xmm2,0x370(%rsp)
   34eb9:	movaps %xmm1,0x360(%rsp)
   34ec1:	movaps %xmm0,0x350(%rsp)
   34ec9:	mov    0x360(%rsp),%rax
   34ed1:	mov    %rax,0x18(%r14)
   34ed5:	movups %xmm0,0x8(%r14)
   34eda:	movups 0x368(%rsp),%xmm0
   34ee2:	movups %xmm0,0x20(%r14)
   34ee7:	mov    0x378(%rsp),%rax
   34eef:	mov    %rax,0x30(%r14)
   34ef3:	movl   $0x3,(%r14)
   34efa:	jmp    35aca <meshopt_peers::run+0xd9a>
   34eff:	movabs $0x7966696c706d6973,%rdx
   34f09:	xor    (%rsi),%rdx
   34f0c:	movabs $0x7970706f6c732d79,%rdi
   34f16:	xor    0x7(%rsi),%rdi
   34f1a:	or     %rdx,%rdi
   34f1d:	jne    36e8e <meshopt_peers::run+0x215e>
   34f23:	movdqu 0x20(%rcx),%xmm1
   34f28:	movq   $0x0,0x1d0(%rsp)
   34f34:	mov    %rax,0x170(%rsp)
   34f3c:	movd   %xmm0,0x230(%rsp)
   34f45:	movb   $0x0,0x365(%r12)
   34f4e:	movl   $0x0,0x361(%r12)
   34f5a:	pxor   %xmm0,%xmm0
   34f5e:	movdqu %xmm0,0x350(%r12)
   34f68:	mov    0x348(%r12),%rax
   34f70:	mov    %rax,0xe0(%rsp)
   34f78:	mov    %rax,0xe8(%rsp)
   34f80:	movdqu %xmm1,0x738(%rsp)
   34f89:	lea    0x6e0(%rsp),%rax
   34f91:	mov    %rax,0x748(%rsp)
   34f99:	lea    0xe0(%rsp),%rax
   34fa1:	mov    %rax,0x750(%rsp)
   34fa9:	lea    0x1d0(%rsp),%rax
   34fb1:	mov    %rax,0x758(%rsp)
   34fb9:	lea    0x170(%rsp),%rax
   34fc1:	mov    %rax,0x760(%rsp)
   34fc9:	lea    0x230(%rsp),%rax
   34fd1:	mov    %rax,0x768(%rsp)
   34fd9:	mov    %r12,0x770(%rsp)
   34fe1:	movq   $0x0,0x778(%rsp)
   34fed:	lea    0x350(%rsp),%rdi
   34ff5:	lea    0x738(%rsp),%rsi
   34ffd:	call   87ba0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   35002:	mov    0xe8(%rsp),%rax
   3500a:	sub    0xe0(%rsp),%rax
   35012:	mov    %rax,0x358(%r12)
   3501a:	mov    0x350(%rsp),%rax
   35022:	movzbl 0x358(%rsp),%ecx
   3502a:	cmp    $0xffffffffffffffff,%rax
   3502e:	je     35052 <meshopt_peers::run+0x322>
   35030:	mov    0x368(%rsp),%rdx
   35038:	mov    %rdx,0x2d8(%rsp)
   35040:	movdqu 0x359(%rsp),%xmm0
   35049:	movdqu %xmm0,0x2c9(%rsp)
   35052:	mov    %cl,0x2c8(%rsp)
   35059:	mov    %rax,0x2c0(%rsp)
   35061:	jmp    354cc <meshopt_peers::run+0x79c>
   35066:	movdqu (%rsi),%xmm0
   3506a:	movdqu 0x3(%rsi),%xmm1
   3506f:	pcmpeqb -0x27027(%rip),%xmm1        # e050 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   35077:	pcmpeqb -0x274af(%rip),%xmm0        # dbd0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   3507f:	pand   %xmm1,%xmm0
   35083:	pmovmskb %xmm0,%eax
   35087:	cmp    $0xffff,%eax
   3508c:	jne    36e8e <meshopt_peers::run+0x215e>
   35092:	mov    0x40(%rcx),%rax
   35096:	test   %rbx,%rbx
   35099:	setne  %dl
   3509c:	lea    0x0(,%rbx,4),%rsi
   350a4:	cmp    %rax,%rsi
   350a7:	seta   %sil
   350ab:	and    %dl,%sil
   350ae:	cmp    $0x1,%sil
   350b2:	je     37298 <meshopt_peers::run+0x2568>
   350b8:	mov    0x20(%rcx),%rsi
   350bc:	mov    0x28(%rcx),%rdx
   350c0:	mov    0x38(%rcx),%rcx
   350c4:	movb   $0x0,0x738(%rsp)
   350cc:	mov    %rcx,0x740(%rsp)
   350d4:	mov    %rax,0x748(%rsp)
   350dc:	mov    %rbx,0x750(%rsp)
   350e4:	movq   $0x4,0x758(%rsp)
   350f0:	movq   $0x4,0x760(%rsp)
   350fc:	pxor   %xmm0,%xmm0
   35100:	movdqu %xmm0,0x768(%rsp)
   35109:	sub    $0x30,%rsp
   3510d:	mov    %r12,0x20(%rsp)
   35112:	lea    0x250(%rsp),%rax
   3511a:	mov    %rax,0x18(%rsp)
   3511f:	movq   $0x0,0x8(%rsp)
   35128:	movq   $0x4,(%rsp)
   35130:	lea    -0x27097(%rip),%r9        # e0a0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   35137:	lea    0x2f0(%rsp),%rdi
   3513f:	lea    0x710(%rsp),%rcx
   35147:	lea    0x768(%rsp),%r8
   3514f:	jmp    354bb <meshopt_peers::run+0x78b>
   35154:	movabs $0x7966696c706d6973,%rax
   3515e:	cmp    %rax,(%rsi)
   35161:	je     3546e <meshopt_peers::run+0x73e>
   35167:	movabs $0x776172647265766f,%rax
   35171:	cmp    %rax,(%rsi)
   35174:	je     35990 <meshopt_peers::run+0xc60>
   3517a:	movabs $0x7374656c6873656d,%rax
   35184:	cmp    %rax,(%rsi)
   35187:	jne    36e8e <meshopt_peers::run+0x215e>
   3518d:	mov    0x20(%rcx),%rsi
   35191:	mov    0x28(%rcx),%rdx
   35195:	movq   $0x7c,0x360(%rsp)
   351a1:	movq   $0x0,0x354(%rsp)
   351ad:	movl   $0x1,0x350(%rsp)
   351b8:	lea    0x350(%rsp),%rax
   351c0:	lea    0x738(%rsp),%rdi
   351c8:	lea    0x6e0(%rsp),%r8
   351d0:	mov    $0x40,%r9d
   351d6:	mov    %rbx,%rcx
   351d9:	push   $0x2
   351db:	push   %r12
   351dd:	push   %rax
   351de:	push   $0x7c
   351e0:	call   b54b0 <meshoptimizer_rs::meshlet::build>
   351e5:	add    $0x20,%rsp
   351e9:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   351f2:	je     36f95 <meshopt_peers::run+0x2265>
   351f8:	mov    0x778(%rsp),%rax
   35200:	mov    %rax,0x390(%rsp)
   35208:	movups 0x738(%rsp),%xmm0
   35210:	movups 0x748(%rsp),%xmm1
   35218:	movups 0x758(%rsp),%xmm2
   35220:	movups 0x768(%rsp),%xmm3
   35228:	movaps %xmm3,0x380(%rsp)
   35230:	movaps %xmm2,0x370(%rsp)
   35238:	movaps %xmm1,0x360(%rsp)
   35240:	movaps %xmm0,0x350(%rsp)
   35248:	mov    0x358(%rsp),%rbx
   35250:	mov    0x360(%rsp),%r12
   35258:	test   %r12,%r12
   3525b:	je     359ed <meshopt_peers::run+0xcbd>
   35261:	mov    %r12,%r15
   35264:	shl    $0x4,%r15
   35268:	mov    %r15,%rdi
   3526b:	call   *0xf54f7(%rip)        # 12a768 <malloc@GLIBC_2.2.5>
   35271:	test   %rax,%rax
   35274:	je     37334 <meshopt_peers::run+0x2604>
   3527a:	mov    %r12d,%ecx
   3527d:	and    $0x3,%ecx
   35280:	cmp    $0x4,%r12
   35284:	jae    359f4 <meshopt_peers::run+0xcc4>
   3528a:	xor    %edx,%edx
   3528c:	jmp    35a48 <meshopt_peers::run+0xd18>
   35291:	movabs $0x65646f6365642d78,%rax
   3529b:	movabs $0x652d786574726576,%rdx
   352a5:	xor    (%rsi),%rdx
   352a8:	lea    0x9010000(%rax),%rdi
   352af:	xor    0x5(%rsi),%rdi
   352b3:	or     %rdx,%rdi
   352b6:	je     3550a <meshopt_peers::run+0x7da>
   352bc:	movabs $0x642d786574726576,%rdx
   352c6:	xor    (%rsi),%rdx
   352c9:	xor    0x5(%rsi),%rax
   352cd:	or     %rdx,%rax
   352d0:	jne    36e8e <meshopt_peers::run+0x215e>
   352d6:	mov    0x50(%rcx),%r9
   352da:	lea    0x738(%rsp),%rdi
   352e2:	mov    $0xc,%r8d
   352e8:	xor    %esi,%esi
   352ea:	xor    %edx,%edx
   352ec:	mov    %rcx,%rax
   352ef:	mov    %rbx,%rcx
   352f2:	push   %r12
   352f4:	push   0x58(%rax)
   352f7:	call   b49d0 <meshoptimizer_rs::codec::allocate>
   352fc:	add    $0x10,%rsp
   35300:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35309:	jne    355ad <meshopt_peers::run+0x87d>
   3530f:	movzbl 0x740(%rsp),%eax
   35317:	mov    %al,0x350(%rsp)
   3531e:	lea    -0x24d72(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   35325:	lea    0xee384(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3532c:	lea    0xee04d(%rip),%r8        # 123380 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   35333:	lea    0x350(%rsp),%rdx
   3533b:	mov    $0x2b,%esi
   35340:	call   *0xf54fa(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   35346:	movabs $0x6e652d7865646e69,%rax
   35350:	xor    (%rsi),%rax
   35353:	mov    0x8(%rsi),%edx
   35356:	xor    $0x65646f63,%rdx
   3535d:	or     %rax,%rdx
   35360:	je     35580 <meshopt_peers::run+0x850>
   35366:	movabs $0x65642d7865646e69,%rax
   35370:	xor    (%rsi),%rax
   35373:	mov    0x8(%rsi),%edx
   35376:	xor    $0x65646f63,%rdx
   3537d:	or     %rax,%rdx
   35380:	jne    36e8e <meshopt_peers::run+0x215e>
   35386:	mov    0x28(%rcx),%rax
   3538a:	mov    0x68(%rcx),%r9
   3538e:	lea    0x738(%rsp),%rdi
   35396:	mov    $0x4,%r8d
   3539c:	mov    $0x1,%esi
   353a1:	xor    %edx,%edx
   353a3:	mov    %rcx,%r10
   353a6:	mov    %rax,%rcx
   353a9:	push   %r12
   353ab:	push   0x70(%r10)
   353af:	call   b49d0 <meshoptimizer_rs::codec::allocate>
   353b4:	add    $0x10,%rsp
   353b8:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   353c1:	jne    355ad <meshopt_peers::run+0x87d>
   353c7:	movzbl 0x740(%rsp),%eax
   353cf:	mov    %al,0x350(%rsp)
   353d6:	lea    -0x24e2a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   353dd:	lea    0xee2cc(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   353e4:	lea    0xedf65(%rip),%r8        # 123350 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x138>
   353eb:	lea    0x350(%rsp),%rdx
   353f3:	mov    $0x2b,%esi
   353f8:	call   *0xf5442(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   353fe:	movabs $0x2d72657473756c63,%rax
   35408:	xor    (%rsi),%rax
   3540b:	movabs $0x646f6c2d72657473,%rdx
   35415:	xor    0x3(%rsi),%rdx
   35419:	or     %rax,%rdx
   3541c:	jne    36e8e <meshopt_peers::run+0x215e>
   35422:	mov    %r14,0x288(%rsp)
   3542a:	test   %rbx,%rbx
   3542d:	je     35634 <meshopt_peers::run+0x904>
   35433:	mov    %rcx,%rbp
   35436:	lea    0x0(,%rbx,4),%rax
   3543e:	lea    (%rax,%rax,2),%r13
   35442:	mov    %r13,%rdi
   35445:	call   *0xf531d(%rip)        # 12a768 <malloc@GLIBC_2.2.5>
   3544b:	test   %rax,%rax
   3544e:	je     37311 <meshopt_peers::run+0x25e1>
   35454:	mov    %rax,%r14
   35457:	mov    %rax,%rdi
   3545a:	mov    %r15,%rsi
   3545d:	mov    %r13,%rdx
   35460:	call   *0xf5322(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   35466:	mov    %rbp,%rcx
   35469:	jmp    3563a <meshopt_peers::run+0x90a>
   3546e:	mov    0x20(%rcx),%rsi
   35472:	mov    0x28(%rcx),%rdx
   35476:	movb   $0x2,0x738(%rsp)
   3547e:	sub    $0x30,%rsp
   35482:	mov    %r12,0x20(%rsp)
   35487:	lea    0x250(%rsp),%rax
   3548f:	mov    %rax,0x18(%rsp)
   35494:	pxor   %xmm0,%xmm0
   35498:	movdqu %xmm0,(%rsp)
   3549d:	lea    0x2f0(%rsp),%rdi
   354a5:	lea    0x710(%rsp),%rcx
   354ad:	lea    0x768(%rsp),%r8
   354b5:	mov    $0x4,%r9d
   354bb:	call   c0150 <meshoptimizer_rs::simplify::allocating>
   354c0:	add    $0x30,%rsp
   354c4:	mov    0x2c0(%rsp),%rax
   354cc:	cmp    $0xffffffffffffffff,%rax
   354d0:	je     36e57 <meshopt_peers::run+0x2127>
   354d6:	mov    0x2d0(%rsp),%rax
   354de:	mov    %rax,0x18(%r14)
   354e2:	movups 0x2c0(%rsp),%xmm0
   354ea:	movups %xmm0,0x8(%r14)
   354ef:	movss  0x2d8(%rsp),%xmm0
   354f8:	movss  %xmm0,0x4(%r14)
   354fe:	movl   $0x2,(%r14)
   35505:	jmp    35aca <meshopt_peers::run+0xd9a>
   3550a:	lea    0x0(,%rbx,4),%rax
   35512:	lea    (%rax,%rax,2),%rdx
   35516:	lea    0x738(%rsp),%rdi
   3551e:	mov    $0xc,%r8d
   35524:	mov    %r15,%rsi
   35527:	mov    %rbx,%rcx
   3552a:	mov    $0x1,%r9d
   35530:	push   %r12
   35532:	push   $0x2
   35534:	call   *0xf560e(%rip)        # 12ab48 <_DYNAMIC+0x5d8>
   3553a:	add    $0x10,%rsp
   3553e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35547:	jne    355ad <meshopt_peers::run+0x87d>
   35549:	movzbl 0x740(%rsp),%eax
   35551:	mov    %al,0x350(%rsp)
   35558:	lea    -0x24fac(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3555f:	lea    0xee14a(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   35566:	lea    0xede2b(%rip),%r8        # 123398 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   3556d:	lea    0x350(%rsp),%rdx
   35575:	mov    $0x2b,%esi
   3557a:	call   *0xf52c0(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   35580:	mov    0x20(%rcx),%rsi
   35584:	mov    0x28(%rcx),%rdx
   35588:	lea    0x738(%rsp),%rdi
   35590:	mov    $0x1,%ecx
   35595:	mov    %r12,%r8
   35598:	call   *0xf55b2(%rip)        # 12ab50 <_DYNAMIC+0x5e0>
   3559e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   355a7:	je     36fcc <meshopt_peers::run+0x229c>
   355ad:	mov    0x748(%rsp),%rax
   355b5:	mov    %rax,0x18(%r14)
   355b9:	movups 0x738(%rsp),%xmm0
   355c1:	movups %xmm0,0x8(%r14)
   355c6:	movl   $0x0,(%r14)
   355cd:	jmp    35aca <meshopt_peers::run+0xd9a>
   355d2:	mov    0x20(%rcx),%rsi
   355d6:	mov    0x28(%rcx),%rdx
   355da:	lea    0x738(%rsp),%rdi
   355e2:	mov    %rbx,%rcx
   355e5:	mov    %r12,%r8
   355e8:	call   *0xf556a(%rip)        # 12ab58 <_DYNAMIC+0x5e8>
   355ee:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   355f7:	jne    359c8 <meshopt_peers::run+0xc98>
   355fd:	movzbl 0x740(%rsp),%eax
   35605:	mov    %al,0x350(%rsp)
   3560c:	lea    -0x25060(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   35613:	lea    0xee096(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3561a:	lea    0xeddef(%rip),%r8        # 123410 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   35621:	lea    0x350(%rsp),%rdx
   35629:	mov    $0x2b,%esi
   3562e:	call   *0xf520c(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   35634:	mov    $0x4,%r14d
   3563a:	movq   $0x0,0x158(%rsp)
   35646:	movq   $0x4,0x160(%rsp)
   35652:	movq   $0x0,0x168(%rsp)
   3565e:	movl   $0x0,0x9c(%rsp)
   35669:	movq   $0x80,0x1d0(%rsp)
   35675:	movq   $0x2a,0x1d8(%rsp)
   35681:	movq   $0x80,0x1e0(%rsp)
   3568d:	movq   $0x10,0x1e8(%rsp)
   35699:	movaps -0x277b0(%rip),%xmm0        # def0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   356a0:	movups %xmm0,0x1f0(%rsp)
   356a8:	movaps -0x276ff(%rip),%xmm0        # dfb0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   356af:	movups %xmm0,0x200(%rsp)
   356b7:	movabs $0x100010100000101,%rax
   356c1:	mov    %rax,0x210(%rsp)
   356c9:	movl   $0x0,0x218(%rsp)
   356d4:	movb   $0x1,0x21c(%rsp)
   356dc:	mov    0x20(%rcx),%r13
   356e0:	mov    0x28(%rcx),%r15
   356e4:	mov    %r13,0x308(%rsp)
   356ec:	mov    %r15,0x310(%rsp)
   356f4:	mov    %r14,0x50(%rsp)
   356f9:	mov    %r14,0x318(%rsp)
   35701:	mov    %rbx,0x20(%rsp)
   35706:	mov    %rbx,0x320(%rsp)
   3570e:	movb   $0x2,0x2c0(%rsp)
   35716:	movq   $0x4,0x328(%rsp)
   35722:	xorps  %xmm0,%xmm0
   35725:	movups %xmm0,0x330(%rsp)
   3572d:	movl   $0x0,0x348(%rsp)
   35738:	movb   $0x0,0x365(%r12)
   35741:	movl   $0x0,0x361(%r12)
   3574d:	movups %xmm0,0x350(%r12)
   35756:	mov    0x348(%r12),%rbx
   3575e:	mov    %r12,%rdi
   35761:	xor    %esi,%esi
   35763:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35768:	mov    %eax,%ebp
   3576a:	cmp    $0xff,%al
   3576c:	jne    373e5 <meshopt_peers::run+0x26b5>
   35772:	mov    %r12,0x738(%rsp)
   3577a:	mov    %rbx,0x740(%rsp)
   35782:	mov    %rbx,0x748(%rsp)
   3578a:	movq   $0x0,0x750(%rsp)
   35796:	movl   $0x0,0x758(%rsp)
   357a1:	movl   $0x0,0x95c(%rsp)
   357ac:	movb   $0x1,0x1960(%rsp)
   357b4:	lea    0x2c0(%rsp),%rdi
   357bc:	lea    0x1d0(%rsp),%rsi
   357c4:	lea    0x738(%rsp),%rdx
   357cc:	mov    0x20(%rsp),%rbx
   357d1:	call   *0xf5389(%rip)        # 12ab60 <_DYNAMIC+0x5f0>
   357d7:	mov    %eax,%ebp
   357d9:	cmp    $0xff,%al
   357db:	jne    373c6 <meshopt_peers::run+0x2696>
   357e1:	movzbl 0x1960(%rsp),%r12d
   357ea:	movb   $0x2,0x350(%rsp)
   357f2:	mov    0x50(%rsp),%rax
   357f7:	mov    %rax,0x358(%rsp)
   357ff:	mov    %rbx,0x360(%rsp)
   35807:	movq   $0x0,0x388(%rsp)
   35813:	lea    0xe0(%rsp),%rdi
   3581b:	lea    0x350(%rsp),%rsi
   35823:	lea    0x738(%rsp),%rdx
   3582b:	call   *0xf5337(%rip)        # 12ab68 <_DYNAMIC+0x5f8>
   35831:	mov    0xe0(%rsp),%r14
   35839:	movzbl 0xe8(%rsp),%ebp
   35841:	cmp    $0xffffffffffffffff,%r14
   35845:	je     3592e <meshopt_peers::run+0xbfe>
   3584b:	mov    %r14,0x150(%rsp)
   35853:	movzbl %bpl,%eax
   35857:	movzbl 0xef(%rsp),%ecx
   3585f:	shl    $0x10,%ecx
   35862:	movzwl 0xed(%rsp),%edx
   3586a:	or     %ecx,%edx
   3586c:	mov    0xe9(%rsp),%ecx
   35873:	shl    $0x28,%rdx
   35877:	shl    $0x8,%rcx
   3587b:	or     %rdx,%rcx
   3587e:	or     %rax,%rcx
   35881:	mov    %rcx,0xc8(%rsp)
   35889:	mov    0x750(%rsp),%rsi
   35891:	add    %rbx,%rsi
   35894:	jb     36f4c <meshopt_peers::run+0x221c>
   3589a:	mov    0xf0(%rsp),%rax
   358a2:	mov    %rax,0x1c8(%rsp)
   358aa:	mov    %rsi,0x750(%rsp)
   358b2:	mov    0x738(%rsp),%rdi
   358ba:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   358bf:	mov    %eax,%ebp
   358c1:	cmp    $0xff,%al
   358c3:	jne    35915 <meshopt_peers::run+0xbe5>
   358c5:	mov    $0x1,%eax
   358ca:	mov    %rax,0x90(%rsp)
   358d2:	test   %rbx,%rbx
   358d5:	jne    35f33 <meshopt_peers::run+0x1203>
   358db:	mov    0x738(%rsp),%rdi
   358e3:	mov    0x750(%rsp),%rsi
   358eb:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   358f0:	mov    %eax,%ebp
   358f2:	mov    $0x1,%r14b
   358f5:	cmp    $0xff,%al
   358f7:	je     35adc <meshopt_peers::run+0xdac>
   358fd:	test   %rbx,%rbx
   35900:	je     35910 <meshopt_peers::run+0xbe0>
   35902:	mov    0x90(%rsp),%rdi
   3590a:	call   *0xf4e68(%rip)        # 12a778 <free@GLIBC_2.2.5>
   35910:	test   %r14b,%r14b
   35913:	je     3592e <meshopt_peers::run+0xbfe>
   35915:	cmpq   $0x0,0x150(%rsp)
   3591e:	je     3592e <meshopt_peers::run+0xbfe>
   35920:	mov    0xc8(%rsp),%rdi
   35928:	call   *0xf4e4a(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3592e:	mov    0x738(%rsp),%rax
   35936:	mov    0x748(%rsp),%rcx
   3593e:	sub    0x740(%rsp),%rcx
   35946:	mov    %rcx,0x358(%rax)
   3594d:	cmp    $0xff,%bpl
   35951:	jne    373e5 <meshopt_peers::run+0x26b5>
   35957:	mov    0x168(%rsp),%rax
   3595f:	mov    0x288(%rsp),%rcx
   35967:	mov    %rax,0x18(%rcx)
   3596b:	movups 0x158(%rsp),%xmm0
   35973:	movups %xmm0,0x8(%rcx)
   35977:	movl   $0x1,(%rcx)
   3597d:	test   %rbx,%rbx
   35980:	je     35aca <meshopt_peers::run+0xd9a>
   35986:	mov    0x50(%rsp),%rdi
   3598b:	jmp    35ac4 <meshopt_peers::run+0xd94>
   35990:	mov    0x20(%rcx),%rsi
   35994:	mov    0x28(%rcx),%rdx
   35998:	lea    0x738(%rsp),%rdi
   359a0:	lea    0x6e0(%rsp),%rcx
   359a8:	movd   -0x25d4c(%rip),%xmm0        # fc64 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x138>
   359b0:	mov    %r12,%r8
   359b3:	call   *0xf51b7(%rip)        # 12ab70 <_DYNAMIC+0x600>
   359b9:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   359c2:	je     3706c <meshopt_peers::run+0x233c>
   359c8:	mov    0x748(%rsp),%rax
   359d0:	mov    %rax,0x18(%r14)
   359d4:	movups 0x738(%rsp),%xmm0
   359dc:	movups %xmm0,0x8(%r14)
   359e1:	movl   $0x1,(%r14)
   359e8:	jmp    35aca <meshopt_peers::run+0xd9a>
   359ed:	mov    $0x4,%eax
   359f2:	jmp    35a71 <meshopt_peers::run+0xd41>
   359f4:	mov    %r12,%rsi
   359f7:	and    $0xfffffffffffffffc,%rsi
   359fb:	mov    $0x30,%edi
   35a00:	xor    %edx,%edx
   35a02:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   35a10:	movups -0x30(%rbx,%rdi,1),%xmm0
   35a15:	movups %xmm0,-0x30(%rax,%rdi,1)
   35a1a:	movups -0x20(%rbx,%rdi,1),%xmm0
   35a1f:	movups %xmm0,-0x20(%rax,%rdi,1)
   35a24:	movups -0x10(%rbx,%rdi,1),%xmm0
   35a29:	movups %xmm0,-0x10(%rax,%rdi,1)
   35a2e:	movups (%rbx,%rdi,1),%xmm0
   35a32:	movups %xmm0,(%rax,%rdi,1)
   35a36:	add    $0x4,%rdx
   35a3a:	add    $0x40,%rdi
   35a3e:	cmp    %rdx,%rsi
   35a41:	jne    35a10 <meshopt_peers::run+0xce0>
   35a43:	test   %rcx,%rcx
   35a46:	je     35a71 <meshopt_peers::run+0xd41>
   35a48:	shl    $0x4,%rdx
   35a4c:	mov    %rax,%rsi
   35a4f:	add    %rdx,%rsi
   35a52:	add    %rbx,%rdx
   35a55:	shl    $0x4,%ecx
   35a58:	xor    %edi,%edi
   35a5a:	nopw   0x0(%rax,%rax,1)
   35a60:	movups (%rdx,%rdi,1),%xmm0
   35a64:	movups %xmm0,(%rsi,%rdi,1)
   35a68:	add    $0x10,%rdi
   35a6c:	cmp    %rdi,%rcx
   35a6f:	jne    35a60 <meshopt_peers::run+0xd30>
   35a71:	mov    0x378(%rsp),%rcx
   35a79:	mov    %rcx,0x30(%r14)
   35a7d:	movups 0x368(%rsp),%xmm0
   35a85:	movups %xmm0,0x20(%r14)
   35a8a:	movaps 0x380(%rsp),%xmm0
   35a92:	movups %xmm0,0x38(%r14)
   35a97:	mov    0x390(%rsp),%rcx
   35a9f:	mov    %rcx,0x48(%r14)
   35aa3:	mov    %r12,0x8(%r14)
   35aa7:	mov    %rax,0x10(%r14)
   35aab:	mov    %r12,0x18(%r14)
   35aaf:	movl   $0x4,(%r14)
   35ab6:	cmpq   $0x0,0x350(%rsp)
   35abf:	je     35aca <meshopt_peers::run+0xd9a>
   35ac1:	mov    %rbx,%rdi
   35ac4:	call   *0xf4cae(%rip)        # 12a778 <free@GLIBC_2.2.5>
   35aca:	add    $0x1968,%rsp
   35ad1:	pop    %rbx
   35ad2:	pop    %r12
   35ad4:	pop    %r13
   35ad6:	pop    %r14
   35ad8:	pop    %r15
   35ada:	pop    %rbp
   35adb:	ret
   35adc:	movb   $0x2,0xe0(%rsp)
   35ae4:	mov    0x50(%rsp),%rax
   35ae9:	mov    %rax,0xe8(%rsp)
   35af1:	mov    %rbx,0xf0(%rsp)
   35af9:	movq   $0x0,0x118(%rsp)
   35b05:	movups 0x210(%rsp),%xmm0
   35b0d:	movaps %xmm0,0x390(%rsp)
   35b15:	movups 0x1d0(%rsp),%xmm0
   35b1d:	movups 0x1e0(%rsp),%xmm1
   35b25:	movups 0x1f0(%rsp),%xmm2
   35b2d:	movups 0x200(%rsp),%xmm3
   35b35:	movaps %xmm3,0x380(%rsp)
   35b3d:	movaps %xmm2,0x370(%rsp)
   35b45:	movaps %xmm1,0x360(%rsp)
   35b4d:	movaps %xmm0,0x350(%rsp)
   35b55:	movzbl %r12b,%r10d
   35b59:	lea    0x170(%rsp),%rdi
   35b61:	lea    0xe0(%rsp),%rcx
   35b69:	lea    0x350(%rsp),%r8
   35b71:	mov    %r13,%rsi
   35b74:	mov    %r15,%rdx
   35b77:	mov    $0xffffffff,%r9d
   35b7d:	lea    0x738(%rsp),%rax
   35b85:	push   %rax
   35b86:	mov    %r10,0x1c8(%rsp)
   35b8e:	push   %r10
   35b90:	call   *0xf4fe2(%rip)        # 12ab78 <_DYNAMIC+0x608>
   35b96:	add    $0x10,%rsp
   35b9a:	mov    0x170(%rsp),%rdi
   35ba2:	movzbl 0x178(%rsp),%ebp
   35baa:	cmp    $0xffffffffffffffff,%rdi
   35bae:	je     358fd <meshopt_peers::run+0xbcd>
   35bb4:	movzbl %bpl,%eax
   35bb8:	movzbl 0x17f(%rsp),%ecx
   35bc0:	shl    $0x10,%ecx
   35bc3:	movzwl 0x17d(%rsp),%edx
   35bcb:	or     %ecx,%edx
   35bcd:	mov    0x179(%rsp),%ecx
   35bd4:	mov    0x180(%rsp),%r9
   35bdc:	mov    0x188(%rsp),%r8
   35be4:	mov    0x190(%rsp),%rsi
   35bec:	mov    %rsi,0x28(%rsp)
   35bf1:	shl    $0x28,%rdx
   35bf5:	shl    $0x8,%rcx
   35bf9:	or     %rdx,%rcx
   35bfc:	or     %rax,%rcx
   35bff:	mov    %rcx,0x38(%rsp)
   35c04:	test   %r9,%r9
   35c07:	je     35ef7 <meshopt_peers::run+0x11c7>
   35c0d:	mov    %r8,0x70(%rsp)
   35c12:	mov    %rdi,0x10(%rsp)
   35c17:	mov    0x198(%rsp),%r14
   35c1f:	mov    %r9,0xd8(%rsp)
   35c27:	mov    %r9,%rax
   35c2a:	shl    $0x4,%rax
   35c2e:	lea    (%rax,%rax,2),%rax
   35c32:	mov    %rax,0x18(%rsp)
   35c37:	xor    %r12d,%r12d
   35c3a:	mov    0x738(%rsp),%r15
   35c42:	mov    0x740(%rsp),%rbp
   35c4a:	mov    0x748(%rsp),%rax
   35c52:	sub    %rbp,%rax
   35c55:	mov    %rax,0x358(%r15)
   35c5c:	mov    %r15,%rdi
   35c5f:	call   *0xf4ec3(%rip)        # 12ab28 <_DYNAMIC+0x5b8>
   35c65:	mov    0x340(%r15),%rax
   35c6c:	mov    0x750(%rsp),%r13
   35c74:	sub    %r13,%rax
   35c77:	jb     35f6c <meshopt_peers::run+0x123c>
   35c7d:	mov    0x348(%r15),%rcx
   35c84:	sub    0x358(%r15),%rcx
   35c8b:	mov    %rax,0x690(%rsp)
   35c93:	mov    %rcx,0x698(%rsp)
   35c9b:	movb   $0x0,0x6b0(%rsp)
   35ca3:	movb   $0x0,0x6b5(%rsp)
   35cab:	movl   $0x0,0x6b1(%rsp)
   35cb6:	movb   $0xff,0x6b8(%rsp)
   35cbe:	movw   $0x0,0x6b6(%rsp)
   35cc8:	lea    0x6a0(%rsp),%rax
   35cd0:	xorps  %xmm0,%xmm0
   35cd3:	movups %xmm0,(%rax)
   35cd6:	movq   $0xffffffffffffffff,0x3c8(%rsp)
   35ce2:	movq   $0xffffffffffffffff,0x5d0(%rsp)
   35cee:	movq   $0x0,0x350(%rsp)
   35cfa:	movq   $0x4,0x358(%rsp)
   35d06:	movups %xmm0,-0x340(%rax)
   35d0d:	movq   $0x4,0x370(%rsp)
   35d19:	movups %xmm0,-0x328(%rax)
   35d20:	movq   $0x4,0x388(%rsp)
   35d2c:	movups %xmm0,-0x310(%rax)
   35d33:	movq   $0x1,0x3a0(%rsp)
   35d3f:	movups %xmm0,-0x2f8(%rax)
   35d46:	movq   $0x2,0x3b8(%rsp)
   35d52:	movq   $0x0,0x3c0(%rsp)
   35d5e:	mov    0x38(%rsp),%rax
   35d63:	mov    0x8(%rax,%r12,1),%rdi
   35d68:	mov    0x10(%rax,%r12,1),%rdx
   35d6d:	mov    %rdx,%rsi
   35d70:	add    %rdi,%rsi
   35d73:	jb     36f80 <meshopt_peers::run+0x2250>
   35d79:	cmp    %r14,%rsi
   35d7c:	ja     36f80 <meshopt_peers::run+0x2250>
   35d82:	mov    0x28(%rsp),%rax
   35d87:	lea    (%rax,%rdi,4),%rsi
   35d8b:	movb   $0x2,0xe0(%rsp)
   35d93:	mov    0x50(%rsp),%rax
   35d98:	mov    %rax,0xe8(%rsp)
   35da0:	mov    %rbx,0xf0(%rsp)
   35da8:	movq   $0x0,0x118(%rsp)
   35db4:	lea    0x170(%rsp),%rdi
   35dbc:	lea    0xe0(%rsp),%rcx
   35dc4:	mov    0x1c0(%rsp),%r8
   35dcc:	lea    0x350(%rsp),%r9
   35dd4:	call   *0xf4da6(%rip)        # 12ab80 <_DYNAMIC+0x610>
   35dda:	mov    0x6a8(%rsp),%rsi
   35de2:	sub    %rsi,%rbp
   35de5:	jb     35eda <meshopt_peers::run+0x11aa>
   35deb:	mov    %rbp,0x740(%rsp)
   35df3:	add    0x6a0(%rsp),%r13
   35dfb:	jb     37003 <meshopt_peers::run+0x22d3>
   35e01:	mov    %r15,%rdi
   35e04:	mov    %r13,%rsi
   35e07:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35e0c:	mov    %eax,%ebp
   35e0e:	cmp    $0xff,%al
   35e10:	jne    37006 <meshopt_peers::run+0x22d6>
   35e16:	movzbl 0x170(%rsp),%ebx
   35e1e:	movzbl 0x171(%rsp),%ebp
   35e26:	lea    0x172(%rsp),%rcx
   35e2e:	mov    (%rcx),%rax
   35e31:	mov    0x6(%rcx),%rcx
   35e35:	mov    %rax,0x230(%rsp)
   35e3d:	mov    %rcx,0x236(%rsp)
   35e45:	movss  0x180(%rsp),%xmm0
   35e4e:	movaps %xmm0,0x80(%rsp)
   35e56:	lea    0x350(%rsp),%rdi
   35e5e:	call   33f10 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35e63:	cmp    $0x1,%bl
   35e66:	mov    0x20(%rsp),%rbx
   35e6b:	je     37013 <meshopt_peers::run+0x22e3>
   35e71:	lea    0x232(%rsp),%rcx
   35e79:	mov    0x8(%rcx),%eax
   35e7c:	lea    0x72c(%rsp),%rdx
   35e84:	mov    %eax,0x8(%rdx)
   35e87:	mov    (%rcx),%rax
   35e8a:	mov    %rax,(%rdx)
   35e8d:	mov    0x8(%rdx),%eax
   35e90:	mov    %eax,0x358(%rsp)
   35e97:	mov    (%rdx),%rcx
   35e9a:	mov    %rcx,0x350(%rsp)
   35ea2:	mov    0x38(%rsp),%rdx
   35ea7:	mov    %eax,0x24(%rdx,%r12,1)
   35eac:	mov    %rcx,0x1c(%rdx,%r12,1)
   35eb1:	xorps  %xmm0,%xmm0
   35eb4:	movaps 0x80(%rsp),%xmm1
   35ebc:	movss  %xmm1,%xmm0
   35ec0:	movlps %xmm0,0x28(%rdx,%r12,1)
   35ec6:	add    $0x30,%r12
   35eca:	cmp    %r12,0x18(%rsp)
   35ecf:	jne    35c3a <meshopt_peers::run+0xf0a>
   35ed5:	jmp    35f9e <meshopt_peers::run+0x126e>
   35eda:	lea    0x740(%rsp),%rdi
   35ee2:	call   *0xf4c50(%rip)        # 12ab38 <_DYNAMIC+0x5c8>
   35ee8:	mov    %eax,%ebp
   35eea:	cmp    $0xff,%al
   35eec:	je     35df3 <meshopt_peers::run+0x10c3>
   35ef2:	jmp    37006 <meshopt_peers::run+0x22d6>
   35ef7:	mov    %rdi,%r12
   35efa:	mov    %r8,0x80(%rsp)
   35f02:	lea    0x738(%rsp),%rdi
   35f0a:	mov    0x150(%rsp),%rsi
   35f12:	mov    0xc8(%rsp),%rdx
   35f1a:	call   33850 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35f1f:	mov    $0x1,%r13b
   35f22:	cmp    $0xff,%al
   35f24:	je     35f74 <meshopt_peers::run+0x1244>
   35f26:	mov    %eax,%ebp
   35f28:	xor    %r14d,%r14d
   35f2b:	mov    $0x1,%r15b
   35f2e:	jmp    360a4 <meshopt_peers::run+0x1374>
   35f33:	lea    0x350(%rsp),%rdi
   35f3b:	mov    $0x1,%edx
   35f40:	mov    $0x1,%r8d
   35f46:	mov    $0x1,%r9d
   35f4c:	xor    %esi,%esi
   35f4e:	mov    %rbx,%rcx
   35f51:	call   3b640 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35f56:	cmpl   $0x1,0x350(%rsp)
   35f5e:	jne    36045 <meshopt_peers::run+0x1315>
   35f64:	mov    $0xa,%bpl
   35f67:	jmp    35915 <meshopt_peers::run+0xbe5>
   35f6c:	mov    $0x9,%bpl
   35f6f:	jmp    37013 <meshopt_peers::run+0x22e3>
   35f74:	mov    0x750(%rsp),%rax
   35f7c:	sub    %rbx,%rax
   35f7f:	jae    36065 <meshopt_peers::run+0x1335>
   35f85:	mov    0x90(%rsp),%rdi
   35f8d:	call   *0xf47e5(%rip)        # 12a778 <free@GLIBC_2.2.5>
   35f93:	mov    $0x8,%bpl
   35f96:	xor    %r14d,%r14d
   35f99:	jmp    360a1 <meshopt_peers::run+0x1371>
   35f9e:	mov    0xd8(%rsp),%rdx
   35fa6:	xor    %r13d,%r13d
   35fa9:	cmp    $0x1,%rdx
   35fad:	jne    360b1 <meshopt_peers::run+0x1381>
   35fb3:	mov    0x10(%rsp),%r12
   35fb8:	mov    0x70(%rsp),%rax
   35fbd:	mov    %rax,0x80(%rsp)
   35fc5:	mov    0x38(%rsp),%rdi
   35fca:	movups 0x1c(%rdi),%xmm0
   35fce:	movaps %xmm0,0x2a0(%rsp)
   35fd6:	movl   $0x7f7fffff,0x2b0(%rsp)
   35fe1:	lea    0x9c(%rsp),%rax
   35fe9:	lea    0x158(%rsp),%r10
   35ff1:	lea    0x2a0(%rsp),%r11
   35ff9:	mov    $0x1,%esi
   35ffe:	mov    0x28(%rsp),%rdx
   36003:	mov    %r14,%rcx
   36006:	mov    0x50(%rsp),%r8
   3600b:	mov    %rbx,%r9
   3600e:	lea    0x738(%rsp),%r15
   36016:	push   %r15
   36018:	push   %rax
   36019:	push   %r10
   3601b:	push   %r13
   3601d:	push   %r11
   3601f:	push   $0x0
   36021:	call   33fa0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36026:	add    $0x30,%rsp
   3602a:	mov    %rax,%rbp
   3602d:	test   $0x1,%bpl
   36031:	je     35f02 <meshopt_peers::run+0x11d2>
   36037:	shr    $0x8,%ebp
   3603a:	mov    $0x1,%r14b
   3603d:	mov    $0x1,%r15b
   36040:	mov    $0x1,%r13b
   36043:	jmp    360a4 <meshopt_peers::run+0x1374>
   36045:	mov    0x358(%rsp),%rdi
   3604d:	mov    %rdi,0x90(%rsp)
   36055:	xor    %esi,%esi
   36057:	mov    %rbx,%rdx
   3605a:	call   *0xf4738(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   36060:	jmp    358db <meshopt_peers::run+0xbab>
   36065:	mov    %rax,0x750(%rsp)
   3606d:	test   %rbx,%rbx
   36070:	je     36080 <meshopt_peers::run+0x1350>
   36072:	mov    0x90(%rsp),%rdi
   3607a:	call   *0xf46f8(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36080:	xor    %r14d,%r14d
   36083:	lea    0x738(%rsp),%rdi
   3608b:	mov    $0x4,%edx
   36090:	xor    %esi,%esi
   36092:	call   337b0 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   36097:	cmp    $0xff,%al
   36099:	je     36e19 <meshopt_peers::run+0x20e9>
   3609f:	mov    %eax,%ebp
   360a1:	xor    %r15d,%r15d
   360a4:	mov    0x80(%rsp),%rax
   360ac:	jmp    37026 <meshopt_peers::run+0x22f6>
   360b1:	lea    0x170(%rsp),%rdi
   360b9:	mov    0xf4ac8(%rip),%r10        # 12ab88 <_DYNAMIC+0x618>
   360c0:	mov    %r14,%r8
   360c3:	movb   $0x2,0xe0(%rsp)
   360cb:	mov    0x50(%rsp),%rax
   360d0:	mov    %rax,0xe8(%rsp)
   360d8:	mov    %rbx,0xf0(%rsp)
   360e0:	movq   $0x0,0x118(%rsp)
   360ec:	movups 0x210(%rsp),%xmm0
   360f4:	movaps %xmm0,0x390(%rsp)
   360fc:	movups 0x1d0(%rsp),%xmm0
   36104:	movups 0x1e0(%rsp),%xmm1
   3610c:	movups 0x1f0(%rsp),%xmm2
   36114:	movups 0x200(%rsp),%xmm3
   3611c:	movaps %xmm3,0x380(%rsp)
   36124:	movaps %xmm2,0x370(%rsp)
   3612c:	movaps %xmm1,0x360(%rsp)
   36134:	movaps %xmm0,0x350(%rsp)
   3613c:	mov    0x38(%rsp),%rsi
   36141:	mov    %rdx,0xd8(%rsp)
   36149:	mov    0x28(%rsp),%rcx
   3614e:	mov    %r8,0x18(%rsp)
   36153:	mov    0xc8(%rsp),%r9
   3615b:	lea    0x738(%rsp),%r12
   36163:	push   %r12
   36165:	lea    0x358(%rsp),%rax
   3616d:	push   %rax
   3616e:	lea    0xf0(%rsp),%rax
   36176:	push   %rax
   36177:	push   0x1e0(%rsp)
   3617e:	call   *%r10
   36181:	add    $0x20,%rsp
   36185:	mov    %r13,0x148(%rsp)
   3618d:	mov    0x170(%rsp),%r15
   36195:	movzbl 0x178(%rsp),%ebp
   3619d:	cmp    $0xffffffffffffffff,%r15
   361a1:	je     37013 <meshopt_peers::run+0x22e3>
   361a7:	movzbl %bpl,%eax
   361ab:	movzbl 0x17f(%rsp),%ecx
   361b3:	shl    $0x10,%ecx
   361b6:	movzwl 0x17d(%rsp),%edx
   361be:	or     %ecx,%edx
   361c0:	mov    0x179(%rsp),%r8d
   361c8:	mov    0x180(%rsp),%r14
   361d0:	shl    $0x28,%rdx
   361d4:	shl    $0x8,%r8
   361d8:	or     %rdx,%r8
   361db:	or     %rax,%r8
   361de:	sub    $0x40,%rsp
   361e2:	mov    %r12,0x30(%rsp)
   361e7:	mov    0x208(%rsp),%rax
   361ef:	mov    %rax,0x18(%rsp)
   361f4:	mov    0x108(%rsp),%rax
   361fc:	mov    %rax,0x10(%rsp)
   36201:	mov    0x58(%rsp),%rax
   36206:	mov    %rax,0x8(%rsp)
   3620b:	mov    0x68(%rsp),%rax
   36210:	mov    %rax,(%rsp)
   36214:	movq   $0x0,0x20(%rsp)
   3621d:	mov    0xd0(%rsp),%rdi
   36225:	mov    %rbx,%rsi
   36228:	mov    0x78(%rsp),%rdx
   3622d:	mov    0x118(%rsp),%rcx
   36235:	mov    %r8,0x110(%rsp)
   3623d:	mov    %r14,%r9
   36240:	call   *0xf494a(%rip)        # 12ab90 <_DYNAMIC+0x620>
   36246:	add    $0x40,%rsp
   3624a:	mov    %eax,%ebp
   3624c:	cmp    $0xff,%al
   3624e:	jne    37059 <meshopt_peers::run+0x2329>
   36254:	mov    0x18(%rsp),%rax
   36259:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   36263:	mul    %rcx
   36266:	mov    %rdx,%rax
   36269:	shr    $1,%rax
   3626c:	mov    %rax,0x48(%rsp)
   36271:	mov    $0x30,%ecx
   36276:	mul    %rcx
   36279:	seto   %cl
   3627c:	test   %rax,%rax
   3627f:	sets   %dl
   36282:	or     %cl,%dl
   36284:	mov    $0x8,%bpl
   36287:	mov    0x10(%rsp),%r12
   3628c:	jne    36e0e <meshopt_peers::run+0x20de>
   36292:	mov    $0x1,%cl
   36294:	mov    %ecx,0x8(%rsp)
   36298:	add    0x750(%rsp),%rax
   362a0:	jb     36ee4 <meshopt_peers::run+0x21b4>
   362a6:	mov    %rax,0x750(%rsp)
   362ae:	mov    0x738(%rsp),%r13
   362b6:	mov    %r13,%rdi
   362b9:	mov    %rax,%r12
   362bc:	mov    %rax,%rsi
   362bf:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   362c4:	mov    %eax,%ebp
   362c6:	cmp    $0xff,%al
   362c8:	jne    37173 <meshopt_peers::run+0x2443>
   362ce:	mov    %r15,0x40(%rsp)
   362d3:	mov    0x18(%rsp),%r8
   362d8:	cmp    $0x3,%r8
   362dc:	jae    36d4e <meshopt_peers::run+0x201e>
   362e2:	mov    $0x8,%r15d
   362e8:	xor    %eax,%eax
   362ea:	mov    %rax,0x68(%rsp)
   362ef:	sub    0x48(%rsp),%rax
   362f4:	mov    $0x30,%ecx
   362f9:	mul    %rcx
   362fc:	seto   %cl
   362ff:	test   %rax,%rax
   36302:	sets   %dl
   36305:	or     %cl,%dl
   36307:	mov    $0x8,%bpl
   3630a:	jne    36ec9 <meshopt_peers::run+0x2199>
   36310:	test   %rax,%rax
   36313:	je     36340 <meshopt_peers::run+0x1610>
   36315:	add    %r12,%rax
   36318:	jb     36ec9 <meshopt_peers::run+0x2199>
   3631e:	mov    %rax,0x750(%rsp)
   36326:	mov    %r13,%rdi
   36329:	mov    %rax,%rsi
   3632c:	call   3c0b0 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   36331:	mov    %eax,%ebp
   36333:	cmp    $0xff,%al
   36335:	mov    0x18(%rsp),%r8
   3633a:	jne    36ec9 <meshopt_peers::run+0x2199>
   36340:	mov    %r15,%rdx
   36343:	shr    $0x8,%rdx
   36347:	mov    %r15,%rax
   3634a:	shr    $0x28,%rax
   3634e:	mov    %r15,%rcx
   36351:	shr    $0x38,%rcx
   36355:	mov    %cl,0xef(%rsp)
   3635c:	mov    %ax,0xed(%rsp)
   36364:	mov    %edx,0xe9(%rsp)
   3636b:	movq   $0x0,0xf0(%rsp)
   36377:	mov    0x68(%rsp),%rax
   3637c:	mov    %rax,0xe0(%rsp)
   36384:	mov    %r15b,0xe8(%rsp)
   3638c:	cmp    $0x2,%r8
   36390:	jbe    36419 <meshopt_peers::run+0x16e9>
   36396:	cmp    0x48(%rsp),%rax
   3639b:	jb     36d9d <meshopt_peers::run+0x206d>
   363a1:	xor    %r12d,%r12d
   363a4:	mov    0xe8(%rsp),%r15
   363ac:	lea    (%r12,%r12,2),%rax
   363b0:	shl    $0x4,%rax
   363b4:	xorps  %xmm0,%xmm0
   363b7:	movups %xmm0,0x20(%r15,%rax,1)
   363bd:	movups %xmm0,0x10(%r15,%rax,1)
   363c3:	movups %xmm0,(%r15,%rax,1)
   363c8:	cmpq   $0x6,0x18(%rsp)
   363ce:	jb     363f1 <meshopt_peers::run+0x16c1>
   363d0:	lea    (%r15,%rax,1),%rdi
   363d4:	add    $0x30,%rdi
   363d8:	mov    0x48(%rsp),%rax
   363dd:	lea    (%rax,%rax,2),%rdx
   363e1:	shl    $0x4,%rdx
   363e5:	add    $0xffffffffffffffd0,%rdx
   363e9:	xor    %esi,%esi
   363eb:	call   *0xf43a7(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   363f1:	mov    0xe0(%rsp),%rax
   363f9:	mov    %rax,0x68(%rsp)
   363fe:	cmp    $0xffffffffffffffff,%rax
   36402:	je     371b1 <meshopt_peers::run+0x2481>
   36408:	mov    %r15,%rdx
   3640b:	shr    $0x8,%rdx
   3640f:	add    %r12,0x48(%rsp)
   36414:	mov    0x18(%rsp),%r8
   36419:	movzbl %r15b,%eax
   3641d:	shl    $0x8,%rdx
   36421:	or     %rax,%rdx
   36424:	mov    %rdx,0xb0(%rsp)
   3642c:	lea    0x350(%rsp),%rdi
   36434:	lea    0x738(%rsp),%rsi
   3643c:	mov    %r8,%rdx
   3643f:	call   338a0 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   36444:	mov    0x350(%rsp),%rdx
   3644c:	movzbl 0x358(%rsp),%ebp
   36454:	cmp    $0xffffffffffffffff,%rdx
   36458:	je     3717d <meshopt_peers::run+0x244d>
   3645e:	lea    0x359(%rsp),%rcx
   36466:	mov    (%rcx),%rax
   36469:	mov    0x7(%rcx),%rcx
   3646d:	lea    0xe9(%rsp),%rsi
   36475:	mov    %rcx,0x7(%rsi)
   36479:	mov    %rax,(%rsi)
   3647c:	mov    %rdx,0xe0(%rsp)
   36484:	mov    %bpl,0xe8(%rsp)
   3648c:	mov    0xf0(%rsp),%rcx
   36494:	mov    0x18(%rsp),%rsi
   36499:	mov    %rsi,%r15
   3649c:	sub    %rcx,%r15
   3649f:	mov    %rdx,0x80(%rsp)
   364a7:	jbe    364fc <meshopt_peers::run+0x17cc>
   364a9:	mov    %rdx,%rax
   364ac:	mov    %rcx,%rsi
   364af:	sub    %rcx,%rax
   364b2:	cmp    %rax,%r15
   364b5:	ja     36dc9 <meshopt_peers::run+0x2099>
   364bb:	mov    0xe8(%rsp),%rbp
   364c3:	mov    %rsi,%rbx
   364c6:	lea    0x0(,%rsi,4),%rdi
   364ce:	add    %rbp,%rdi
   364d1:	lea    0x0(,%r15,4),%rdx
   364d9:	xor    %esi,%esi
   364db:	call   *0xf42b7(%rip)        # 12a798 <memset@GLIBC_2.2.5>
   364e1:	cmpq   $0xffffffffffffffff,0x80(%rsp)
   364ea:	je     3717d <meshopt_peers::run+0x244d>
   364f0:	add    %r15,%rbx
   364f3:	mov    %rbp,%rdx
   364f6:	shr    $0x8,%rdx
   364fa:	jmp    36522 <meshopt_peers::run+0x17f2>
   364fc:	movzbl 0xef(%rsp),%eax
   36504:	shl    $0x10,%eax
   36507:	movzwl 0xed(%rsp),%ecx
   3650f:	or     %eax,%ecx
   36511:	shl    $0x20,%rcx
   36515:	mov    0xe9(%rsp),%edx
   3651c:	or     %rcx,%rdx
   3651f:	mov    %rsi,%rbx
   36522:	movzbl %bpl,%eax
   36526:	shl    $0x8,%rdx
   3652a:	or     %rax,%rdx
   3652d:	cmp    $0x2,%r14
   36531:	mov    %rdx,0xc0(%rsp)
   36539:	mov    %rbx,0x140(%rsp)
   36541:	jb     36c58 <meshopt_peers::run+0x1f28>
   36547:	mov    0xb0(%rsp),%rax
   3654f:	add    $0x10,%rax
   36553:	mov    %rax,0x280(%rsp)
   3655b:	xor    %ebp,%ebp
   3655d:	xor    %r12d,%r12d
   36560:	mov    0xd0(%rsp),%rax
   36568:	mov    %rbp,0xb8(%rsp)
   36570:	add    $0x4,%rax
   36574:	mov    -0x4(%rax),%edi
   36577:	mov    %rax,0x1b8(%rsp)
   3657f:	mov    (%rax),%r15d
   36582:	cmp    %edi,%r15d
   36585:	mov    0xd8(%rsp),%rdx
   3658d:	jb     3731f <meshopt_peers::run+0x25ef>
   36593:	cmp    %r15,%rdx
   36596:	jb     3731f <meshopt_peers::run+0x25ef>
   3659c:	sub    %rdi,%r15
   3659f:	lea    (%rdi,%rdi,2),%r13
   365a3:	shl    $0x4,%r13
   365a7:	add    0x38(%rsp),%r13
   365ac:	movq   $0xffffffffffffffff,0xe0(%rsp)
   365b8:	lea    0x350(%rsp),%rdi
   365c0:	mov    %r13,%rsi
   365c3:	mov    %r15,%rdx
   365c6:	mov    0x28(%rsp),%rcx
   365cb:	mov    0x18(%rsp),%r8
   365d0:	lea    0x2c0(%rsp),%r9
   365d8:	lea    0x738(%rsp),%rax
   365e0:	push   %rax
   365e1:	lea    0x1d8(%rsp),%rax
   365e9:	push   %rax
   365ea:	push   0x30(%rsp)
   365ee:	push   0xa8(%rsp)
   365f5:	call   *0xf459d(%rip)        # 12ab98 <_DYNAMIC+0x628>
   365fb:	add    $0x20,%rsp
   365ff:	mov    0x350(%rsp),%rax
   36607:	movzbl 0x358(%rsp),%ebp
   3660f:	mov    %rax,0x8(%rsp)
   36614:	cmp    $0xffffffffffffffff,%rax
   36618:	je     3710e <meshopt_peers::run+0x23de>
   3661e:	movzbl %bpl,%eax
   36622:	movzbl 0x35f(%rsp),%ecx
   3662a:	shl    $0x10,%ecx
   3662d:	movzwl 0x35d(%rsp),%edx
   36635:	or     %ecx,%edx
   36637:	mov    0x359(%rsp),%ebx
   3663e:	mov    0x368(%rsp),%rcx
   36646:	mov    %rcx,0x60(%rsp)
   3664b:	mov    0x370(%rsp),%rcx
   36653:	mov    %rcx,0xa8(%rsp)
   3665b:	mov    0x380(%rsp),%rcx
   36663:	mov    %rcx,0x30(%rsp)
   36668:	mov    0x388(%rsp),%rcx
   36670:	mov    %rcx,0x58(%rsp)
   36675:	mov    0x390(%rsp),%rcx
   3667d:	mov    %rcx,0x78(%rsp)
   36682:	lea    0x359(%rsp),%rcx
   3668a:	movups 0x3f(%rcx),%xmm0
   3668e:	movaps %xmm0,0x260(%rsp)
   36696:	mov    0x4f(%rcx),%ecx
   36699:	mov    %ecx,0x270(%rsp)
   366a0:	shl    $0x28,%rdx
   366a4:	shl    $0x8,%rbx
   366a8:	or     %rdx,%rbx
   366ab:	or     %rax,%rbx
   366ae:	testb  $0x1,0x3ac(%rsp)
   366b6:	je     36827 <meshopt_peers::run+0x1af7>
   366bc:	mov    %r13,%rdi
   366bf:	mov    %r15,%rsi
   366c2:	mov    0x28(%rsp),%rdx
   366c7:	mov    0x18(%rsp),%rcx
   366cc:	mov    0x50(%rsp),%r8
   366d1:	mov    0x20(%rsp),%r9
   366d6:	lea    0x738(%rsp),%rax
   366de:	push   %rax
   366df:	lea    0xa4(%rsp),%rax
   366e7:	push   %rax
   366e8:	lea    0x168(%rsp),%rax
   366f0:	push   %rax
   366f1:	push   0x160(%rsp)
   366f8:	lea    0x280(%rsp),%rax
   36700:	push   %rax
   36701:	push   $0x0
   36703:	call   33fa0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36708:	add    $0x30,%rsp
   3670c:	mov    %rax,%rbp
   3670f:	test   $0x1,%bpl
   36713:	jne    371d3 <meshopt_peers::run+0x24a3>
   36719:	mov    0x750(%rsp),%rcx
   36721:	mov    0x8(%rsp),%rdx
   36726:	lea    0x0(,%rdx,4),%rax
   3672e:	shr    $0x3e,%rdx
   36732:	sete   %dl
   36735:	test   %rax,%rax
   36738:	setns  %sil
   3673c:	and    %dl,%sil
   3673f:	sub    %rax,%rcx
   36742:	setae  %al
   36745:	test   %al,%sil
   36748:	je     36f54 <meshopt_peers::run+0x2224>
   3674e:	mov    %rcx,0x750(%rsp)
   36756:	cmpq   $0x0,0x8(%rsp)
   3675c:	je     3676f <meshopt_peers::run+0x1a3f>
   3675e:	mov    %rbx,%rdi
   36761:	call   *0xf4011(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36767:	mov    0x750(%rsp),%rcx
   3676f:	mov    0x60(%rsp),%rax
   36774:	mov    $0x14,%edx
   36779:	mul    %rdx
   3677c:	jo     370e2 <meshopt_peers::run+0x23b2>
   36782:	test   %rax,%rax
   36785:	js     370e2 <meshopt_peers::run+0x23b2>
   3678b:	sub    %rax,%rcx
   3678e:	jb     370e2 <meshopt_peers::run+0x23b2>
   36794:	mov    %rcx,0x750(%rsp)
   3679c:	cmpq   $0x0,0x60(%rsp)
   367a2:	je     367ba <meshopt_peers::run+0x1a8a>
   367a4:	mov    0xa8(%rsp),%rdi
   367ac:	call   *0xf3fc6(%rip)        # 12a778 <free@GLIBC_2.2.5>
   367b2:	mov    0x750(%rsp),%rcx
   367ba:	movabs $0x3fffffffffffffff,%rax
   367c4:	cmp    %rax,0x30(%rsp)
   367c9:	ja     37198 <meshopt_peers::run+0x2468>
   367cf:	mov    0x30(%rsp),%rax
   367d4:	lea    0x0(,%rax,4),%rax
   367dc:	test   %rax,%rax
   367df:	js     37198 <meshopt_peers::run+0x2468>
   367e5:	sub    %rax,%rcx
   367e8:	jb     37198 <meshopt_peers::run+0x2468>
   367ee:	mov    %rcx,0x750(%rsp)
   367f6:	cmpq   $0x0,0x30(%rsp)
   367fc:	je     36809 <meshopt_peers::run+0x1ad9>
   367fe:	mov    0x58(%rsp),%rdi
   36803:	call   *0xf3f6f(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36809:	dec    %r14
   3680c:	mov    0x1b8(%rsp),%rax
   36814:	add    $0x4,%rax
   36818:	cmp    $0x1,%r14
   3681c:	ja     36574 <meshopt_peers::run+0x1844>
   36822:	jmp    36e01 <meshopt_peers::run+0x20d1>
   36827:	mov    %r13,%rdi
   3682a:	mov    %r15,%rsi
   3682d:	mov    0x28(%rsp),%rdx
   36832:	mov    0x18(%rsp),%rcx
   36837:	mov    0x50(%rsp),%r8
   3683c:	mov    0x20(%rsp),%r9
   36841:	lea    0x738(%rsp),%rax
   36849:	push   %rax
   3684a:	lea    0xa4(%rsp),%rax
   36852:	push   %rax
   36853:	lea    0x168(%rsp),%rax
   3685b:	push   %rax
   3685c:	push   0x160(%rsp)
   36863:	lea    0x280(%rsp),%rax
   3686b:	push   %rax
   3686c:	push   $0x0
   3686e:	call   33fa0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36873:	add    $0x30,%rsp
   36877:	mov    %rax,%rbp
   3687a:	test   $0x1,%bpl
   3687e:	jne    371d3 <meshopt_peers::run+0x24a3>
   36884:	shr    $0x20,%rbp
   36888:	movb   $0x2,0x170(%rsp)
   36890:	mov    0x50(%rsp),%rax
   36895:	mov    %rax,0x178(%rsp)
   3689d:	mov    0x20(%rsp),%rax
   368a2:	mov    %rax,0x180(%rsp)
   368aa:	movq   $0x0,0x1a8(%rsp)
   368b6:	movups 0x210(%rsp),%xmm0
   368be:	movaps %xmm0,0x390(%rsp)
   368c6:	movups 0x1d0(%rsp),%xmm0
   368ce:	movups 0x1e0(%rsp),%xmm1
   368d6:	movups 0x1f0(%rsp),%xmm2
   368de:	movups 0x200(%rsp),%xmm3
   368e6:	movaps %xmm3,0x380(%rsp)
   368ee:	movaps %xmm2,0x370(%rsp)
   368f6:	movaps %xmm1,0x360(%rsp)
   368fe:	movaps %xmm0,0x350(%rsp)
   36906:	lea    0x230(%rsp),%rdi
   3690e:	mov    0x58(%rsp),%rsi
   36913:	mov    0x78(%rsp),%rdx
   36918:	lea    0x170(%rsp),%rcx
   36920:	lea    0x350(%rsp),%r8
   36928:	mov    %ebp,%r9d
   3692b:	lea    0x738(%rsp),%rax
   36933:	push   %rax
   36934:	push   0x1c8(%rsp)
   3693b:	call   *0xf4237(%rip)        # 12ab78 <_DYNAMIC+0x608>
   36941:	add    $0x10,%rsp
   36945:	mov    0x230(%rsp),%r15
   3694d:	movzbl 0x238(%rsp),%ebp
   36955:	cmp    $0xffffffffffffffff,%r15
   36959:	je     371d6 <meshopt_peers::run+0x24a6>
   3695f:	movzbl %bpl,%ecx
   36963:	movzbl 0x23f(%rsp),%eax
   3696b:	shl    $0x10,%eax
   3696e:	movzwl 0x23d(%rsp),%esi
   36976:	or     %eax,%esi
   36978:	mov    0x239(%rsp),%r13d
   36980:	mov    0x240(%rsp),%rax
   36988:	mov    0x248(%rsp),%rdx
   36990:	mov    %rdx,0xa0(%rsp)
   36998:	mov    0x250(%rsp),%rdx
   369a0:	mov    %rdx,0x78(%rsp)
   369a5:	mov    0x258(%rsp),%rdx
   369ad:	shl    $0x28,%rsi
   369b1:	shl    $0x8,%r13
   369b5:	or     %rsi,%r13
   369b8:	or     %rcx,%r13
   369bb:	test   %rax,%rax
   369be:	je     36a67 <meshopt_peers::run+0x1d37>
   369c4:	lea    (%rax,%rax,2),%rax
   369c8:	shl    $0x4,%rax
   369cc:	add    %r13,%rax
   369cf:	lea    (%r12,%r12,2),%rcx
   369d3:	shl    $0x4,%rcx
   369d7:	add    0x280(%rsp),%rcx
   369df:	xor    %esi,%esi
   369e1:	mov    0x0(%r13,%rsi,1),%rdi
   369e6:	mov    0x8(%r13,%rsi,1),%r8
   369eb:	movups 0x10(%r13,%rsi,1),%xmm0
   369f1:	movaps %xmm0,0x6c0(%rsp)
   369f9:	mov    0x270(%rsp),%r9d
   36a01:	lea    0x6cc(%rsp),%r10
   36a09:	mov    %r9d,0x10(%r10)
   36a0d:	movaps 0x260(%rsp),%xmm0
   36a15:	movups %xmm0,(%r10)
   36a19:	cmp    0x48(%rsp),%r12
   36a1e:	jae    37417 <meshopt_peers::run+0x26e7>
   36a24:	lea    (%rsi,%r13,1),%r9
   36a28:	add    0xb8(%rsp),%r8
   36a30:	mov    %rdi,-0x10(%rcx,%rsi,1)
   36a35:	mov    %r8,-0x8(%rcx,%rsi,1)
   36a3a:	movaps 0x6c0(%rsp),%xmm0
   36a42:	movaps 0x6d0(%rsp),%xmm1
   36a4a:	movups %xmm0,(%rcx,%rsi,1)
   36a4e:	movups %xmm1,0x10(%rcx,%rsi,1)
   36a53:	inc    %r12
   36a56:	add    $0x30,%rsi
   36a5a:	add    $0x30,%r9
   36a5e:	cmp    %rax,%r9
   36a61:	jne    369e1 <meshopt_peers::run+0x1cb1>
   36a67:	mov    %rdx,%rbp
   36a6a:	add    0xb8(%rsp),%rbp
   36a72:	jb     3738c <meshopt_peers::run+0x265c>
   36a78:	cmp    0x140(%rsp),%rbp
   36a80:	ja     3738c <meshopt_peers::run+0x265c>
   36a86:	mov    0xc0(%rsp),%rax
   36a8e:	mov    0xb8(%rsp),%rcx
   36a96:	lea    (%rax,%rcx,4),%rdi
   36a9a:	shl    $0x2,%rdx
   36a9e:	mov    0x78(%rsp),%rsi
   36aa3:	call   *0xf3cdf(%rip)        # 12a788 <memcpy@GLIBC_2.14>
   36aa9:	mov    0x750(%rsp),%rcx
   36ab1:	mov    %r15,%rax
   36ab4:	mov    $0x30,%edx
   36ab9:	mul    %rdx
   36abc:	seto   %dl
   36abf:	not    %dl
   36ac1:	test   %rax,%rax
   36ac4:	setns  %sil
   36ac8:	and    %dl,%sil
   36acb:	sub    %rax,%rcx
   36ace:	setae  %al
   36ad1:	test   %al,%sil
   36ad4:	je     37233 <meshopt_peers::run+0x2503>
   36ada:	mov    %rcx,0x750(%rsp)
   36ae2:	test   %r15,%r15
   36ae5:	je     36af8 <meshopt_peers::run+0x1dc8>
   36ae7:	mov    %r13,%rdi
   36aea:	call   *0xf3c88(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36af0:	mov    0x750(%rsp),%rcx
   36af8:	mov    0xa0(%rsp),%rax
   36b00:	shr    $0x3e,%rax
   36b04:	jne    37241 <meshopt_peers::run+0x2511>
   36b0a:	mov    0xa0(%rsp),%rax
   36b12:	lea    0x0(,%rax,4),%rax
   36b1a:	test   %rax,%rax
   36b1d:	js     37241 <meshopt_peers::run+0x2511>
   36b23:	sub    %rax,%rcx
   36b26:	jb     37241 <meshopt_peers::run+0x2511>
   36b2c:	mov    %rcx,0x750(%rsp)
   36b34:	cmpq   $0x0,0xa0(%rsp)
   36b3d:	je     36b52 <meshopt_peers::run+0x1e22>
   36b3f:	mov    0x78(%rsp),%rdi
   36b44:	call   *0xf3c2e(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36b4a:	mov    0x750(%rsp),%rcx
   36b52:	movabs $0x3fffffffffffffff,%rax
   36b5c:	cmp    %rax,0x8(%rsp)
   36b61:	ja     372c8 <meshopt_peers::run+0x2598>
   36b67:	mov    0x8(%rsp),%rax
   36b6c:	lea    0x0(,%rax,4),%rax
   36b74:	test   %rax,%rax
   36b77:	js     372c8 <meshopt_peers::run+0x2598>
   36b7d:	sub    %rax,%rcx
   36b80:	jb     372c8 <meshopt_peers::run+0x2598>
   36b86:	mov    %rcx,0x750(%rsp)
   36b8e:	cmpq   $0x0,0x8(%rsp)
   36b94:	je     36ba7 <meshopt_peers::run+0x1e77>
   36b96:	mov    %rbx,%rdi
   36b99:	call   *0xf3bd9(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36b9f:	mov    0x750(%rsp),%rcx
   36ba7:	mov    0x60(%rsp),%rax
   36bac:	mov    $0x14,%edx
   36bb1:	mul    %rdx
   36bb4:	jo     37347 <meshopt_peers::run+0x2617>
   36bba:	test   %rax,%rax
   36bbd:	js     37347 <meshopt_peers::run+0x2617>
   36bc3:	sub    %rax,%rcx
   36bc6:	jb     37347 <meshopt_peers::run+0x2617>
   36bcc:	mov    %rcx,0x750(%rsp)
   36bd4:	cmpq   $0x0,0x60(%rsp)
   36bda:	je     36bf2 <meshopt_peers::run+0x1ec2>
   36bdc:	mov    0xa8(%rsp),%rdi
   36be4:	call   *0xf3b8e(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36bea:	mov    0x750(%rsp),%rcx
   36bf2:	movabs $0x3fffffffffffffff,%rax
   36bfc:	cmp    %rax,0x30(%rsp)
   36c01:	ja     37378 <meshopt_peers::run+0x2648>
   36c07:	mov    0x30(%rsp),%rax
   36c0c:	lea    0x0(,%rax,4),%rax
   36c14:	test   %rax,%rax
   36c17:	js     37378 <meshopt_peers::run+0x2648>
   36c1d:	sub    %rax,%rcx
   36c20:	jb     37378 <meshopt_peers::run+0x2648>
   36c26:	mov    %rcx,0x750(%rsp)
   36c2e:	cmpq   $0x0,0x30(%rsp)
   36c34:	je     36c41 <meshopt_peers::run+0x1f11>
   36c36:	mov    0x58(%rsp),%rdi
   36c3b:	call   *0xf3b37(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36c41:	dec    %r14
   36c44:	cmp    $0x1,%r14
   36c48:	mov    0x1b8(%rsp),%rax
   36c50:	ja     36568 <meshopt_peers::run+0x1838>
   36c56:	jmp    36c5d <meshopt_peers::run+0x1f2d>
   36c58:	xor    %r12d,%r12d
   36c5b:	xor    %ebp,%ebp
   36c5d:	mov    %rbp,0x18(%rsp)
   36c62:	lea    0x738(%rsp),%rbx
   36c6a:	mov    %rbx,%rdi
   36c6d:	mov    0x40(%rsp),%rsi
   36c72:	mov    0xd0(%rsp),%rdx
   36c7a:	call   33850 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36c7f:	mov    %eax,%ebp
   36c81:	mov    $0x1,%al
   36c83:	mov    %eax,0x8(%rsp)
   36c87:	cmp    $0xff,%bpl
   36c8b:	setne  %r13b
   36c8f:	jne    371cb <meshopt_peers::run+0x249b>
   36c95:	mov    0x48(%rsp),%r15
   36c9a:	cmp    %r15,%r12
   36c9d:	cmovb  %r12,%r15
   36ca1:	mov    0x140(%rsp),%r14
   36ca9:	mov    0x18(%rsp),%rax
   36cae:	cmp    %r14,%rax
   36cb1:	cmovb  %rax,%r14
   36cb5:	mov    %rbx,%rdi
   36cb8:	mov    0x10(%rsp),%rsi
   36cbd:	mov    0x38(%rsp),%rdx
   36cc2:	call   33800 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36cc7:	cmp    $0xff,%al
   36cc9:	jne    371ea <meshopt_peers::run+0x24ba>
   36ccf:	mov    %rbx,%rdi
   36cd2:	mov    0x70(%rsp),%rsi
   36cd7:	mov    0x28(%rsp),%rdx
   36cdc:	call   33850 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36ce1:	cmp    $0xff,%al
   36ce3:	jne    371f4 <meshopt_peers::run+0x24c4>
   36ce9:	mov    0x148(%rsp),%r13
   36cf1:	inc    %r13d
   36cf4:	mov    0x68(%rsp),%rax
   36cf9:	mov    %rax,0x10(%rsp)
   36cfe:	mov    %r15,%rdx
   36d01:	mov    0xb0(%rsp),%rax
   36d09:	mov    %rax,0x38(%rsp)
   36d0e:	mov    %r14,%r8
   36d11:	mov    0xc0(%rsp),%rax
   36d19:	mov    %rax,0x28(%rsp)
   36d1e:	mov    0x80(%rsp),%rax
   36d26:	mov    %rax,0x70(%rsp)
   36d2b:	cmp    $0x1,%r15
   36d2f:	mov    0x20(%rsp),%rbx
   36d34:	lea    0x170(%rsp),%rdi
   36d3c:	mov    0xf3e45(%rip),%r10        # 12ab88 <_DYNAMIC+0x618>
   36d43:	ja     360c3 <meshopt_peers::run+0x1393>
   36d49:	jmp    37206 <meshopt_peers::run+0x24d6>
   36d4e:	mov    $0x8,%edx
   36d53:	mov    $0x8,%r8d
   36d59:	mov    $0x30,%r9d
   36d5f:	lea    0x350(%rsp),%rdi
   36d67:	xor    %esi,%esi
   36d69:	mov    0x48(%rsp),%rcx
   36d6e:	call   3b640 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36d73:	cmpl   $0x1,0x350(%rsp)
   36d7b:	je     37286 <meshopt_peers::run+0x2556>
   36d81:	mov    0x358(%rsp),%r15
   36d89:	mov    0x48(%rsp),%rax
   36d8e:	mov    0x20(%rsp),%rbx
   36d93:	mov    0x18(%rsp),%r8
   36d98:	jmp    362ea <meshopt_peers::run+0x15ba>
   36d9d:	mov    $0x8,%ecx
   36da2:	mov    $0x30,%r8d
   36da8:	lea    0xe0(%rsp),%rdi
   36db0:	xor    %esi,%esi
   36db2:	mov    0x48(%rsp),%rdx
   36db7:	call   34c90 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36dbc:	mov    0xf0(%rsp),%r12
   36dc4:	jmp    363a4 <meshopt_peers::run+0x1674>
   36dc9:	mov    $0x4,%ecx
   36dce:	mov    $0x4,%r8d
   36dd4:	lea    0xe0(%rsp),%rdi
   36ddc:	mov    %r15,%rdx
   36ddf:	call   34c90 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36de4:	mov    0xe0(%rsp),%rax
   36dec:	mov    %rax,0x80(%rsp)
   36df4:	mov    0xf0(%rsp),%rsi
   36dfc:	jmp    364bb <meshopt_peers::run+0x178b>
   36e01:	mov    0xb8(%rsp),%rbp
   36e09:	jmp    36c5d <meshopt_peers::run+0x1f2d>
   36e0e:	mov    $0x1,%al
   36e10:	mov    %eax,0x8(%rsp)
   36e14:	jmp    36ee4 <meshopt_peers::run+0x21b4>
   36e19:	lea    0x738(%rsp),%rdi
   36e21:	mov    %r12,%rsi
   36e24:	mov    0x38(%rsp),%rdx
   36e29:	call   33800 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36e2e:	cmp    $0xff,%al
   36e30:	jne    373ae <meshopt_peers::run+0x267e>
   36e36:	lea    0x738(%rsp),%rdi
   36e3e:	mov    0x80(%rsp),%rsi
   36e46:	mov    0x28(%rsp),%rdx
   36e4b:	call   33850 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36e50:	mov    %eax,%ebp
   36e52:	jmp    3592e <meshopt_peers::run+0xbfe>
   36e57:	movzbl 0x2c8(%rsp),%eax
   36e5f:	mov    %al,0x738(%rsp)
   36e66:	lea    -0x268ba(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36e6d:	lea    0xec83c(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36e74:	lea    0xec5c5(%rip),%r8        # 123440 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36e7b:	lea    0x738(%rsp),%rdx
   36e83:	mov    $0x2b,%esi
   36e88:	call   *0xf39b2(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   36e8e:	lea    0x290(%rsp),%rax
   36e96:	mov    %rax,0x738(%rsp)
   36e9e:	lea    0x55fb(%rip),%rax        # 3c4a0 <<&str as core::fmt::Display>::fmt>
   36ea5:	mov    %rax,0x740(%rsp)
   36ead:	lea    -0x2a7b9(%rip),%rdi        # c6fb <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36eb4:	lea    0xec47d(%rip),%rdx        # 123338 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x120>
   36ebb:	lea    0x738(%rsp),%rsi
   36ec3:	call   *0xf3947(%rip)        # 12a810 <_DYNAMIC+0x2a0>
   36ec9:	cmpq   $0x0,0x68(%rsp)
   36ecf:	mov    0x10(%rsp),%r12
   36ed4:	je     36edf <meshopt_peers::run+0x21af>
   36ed6:	mov    %r15,%rdi
   36ed9:	call   *0xf3899(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36edf:	mov    0x40(%rsp),%r15
   36ee4:	mov    $0x1,%r13b
   36ee7:	test   %r15,%r15
   36eea:	je     36efa <meshopt_peers::run+0x21ca>
   36eec:	mov    0xd0(%rsp),%rdi
   36ef4:	call   *0xf387e(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36efa:	mov    $0x1,%r14b
   36efd:	mov    $0x1,%r15b
   36f00:	cmpb   $0x0,0x8(%rsp)
   36f05:	mov    0x70(%rsp),%rax
   36f0a:	jne    37026 <meshopt_peers::run+0x22f6>
   36f10:	jmp    37036 <meshopt_peers::run+0x2306>
   36f15:	movzbl 0x740(%rsp),%eax
   36f1d:	mov    %al,0x350(%rsp)
   36f24:	lea    -0x26978(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f2b:	lea    0xec77e(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f32:	lea    0xec4a7(%rip),%r8        # 1233e0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   36f39:	lea    0x350(%rsp),%rdx
   36f41:	mov    $0x2b,%esi
   36f46:	call   *0xf38f4(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   36f4c:	mov    $0x8,%bpl
   36f4f:	jmp    35915 <meshopt_peers::run+0xbe5>
   36f54:	mov    $0x8,%bpl
   36f57:	cmpq   $0x0,0x8(%rsp)
   36f5d:	je     370a3 <meshopt_peers::run+0x2373>
   36f63:	mov    %rbx,%rdi
   36f66:	call   *0xf380c(%rip)        # 12a778 <free@GLIBC_2.2.5>
   36f6c:	xor    %r15d,%r15d
   36f6f:	cmpq   $0x0,0x30(%rsp)
   36f75:	jne    370ae <meshopt_peers::run+0x237e>
   36f7b:	jmp    370b9 <meshopt_peers::run+0x2389>
   36f80:	lea    0xec399(%rip),%rcx        # 123320 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x108>
   36f87:	mov    %r14,%rdx
   36f8a:	call   *0xf3950(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   36f90:	jmp    3742c <meshopt_peers::run+0x26fc>
   36f95:	movzbl 0x740(%rsp),%eax
   36f9d:	mov    %al,0x350(%rsp)
   36fa4:	lea    -0x269f8(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36fab:	lea    0xec6fe(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36fb2:	lea    0xec3f7(%rip),%r8        # 1233b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36fb9:	lea    0x350(%rsp),%rdx
   36fc1:	mov    $0x2b,%esi
   36fc6:	call   *0xf3874(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   36fcc:	movzbl 0x740(%rsp),%eax
   36fd4:	mov    %al,0x350(%rsp)
   36fdb:	lea    -0x26a2f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36fe2:	lea    0xec6c7(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36fe9:	lea    0xec378(%rip),%r8        # 123368 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36ff0:	lea    0x350(%rsp),%rdx
   36ff8:	mov    $0x2b,%esi
   36ffd:	call   *0xf383d(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   37003:	mov    $0x8,%bpl
   37006:	lea    0x350(%rsp),%rdi
   3700e:	call   33f10 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   37013:	mov    $0x1,%r14b
   37016:	mov    $0x1,%r15b
   37019:	mov    $0x1,%r13b
   3701c:	mov    0x10(%rsp),%r12
   37021:	mov    0x70(%rsp),%rax
   37026:	test   %rax,%rax
   37029:	je     37036 <meshopt_peers::run+0x2306>
   3702b:	mov    0x28(%rsp),%rdi
   37030:	call   *0xf3742(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37036:	test   %r13b,%r13b
   37039:	je     3704b <meshopt_peers::run+0x231b>
   3703b:	test   %r12,%r12
   3703e:	je     3704b <meshopt_peers::run+0x231b>
   37040:	mov    0x38(%rsp),%rdi
   37045:	call   *0xf372d(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3704b:	test   %r15b,%r15b
   3704e:	jne    358fd <meshopt_peers::run+0xbcd>
   37054:	jmp    35910 <meshopt_peers::run+0xbe0>
   37059:	mov    $0x1,%al
   3705b:	mov    %eax,0x8(%rsp)
   3705f:	mov    $0x1,%r13b
   37062:	mov    0x10(%rsp),%r12
   37067:	jmp    36ee7 <meshopt_peers::run+0x21b7>
   3706c:	movzbl 0x740(%rsp),%eax
   37074:	mov    %al,0x350(%rsp)
   3707b:	lea    -0x26acf(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37082:	lea    0xec627(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   37089:	lea    0xec368(%rip),%r8        # 1233f8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   37090:	lea    0x350(%rsp),%rdx
   37098:	mov    $0x2b,%esi
   3709d:	call   *0xf379d(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   370a3:	xor    %r15d,%r15d
   370a6:	cmpq   $0x0,0x30(%rsp)
   370ac:	je     370b9 <meshopt_peers::run+0x2389>
   370ae:	mov    0x58(%rsp),%rdi
   370b3:	call   *0xf36bf(%rip)        # 12a778 <free@GLIBC_2.2.5>
   370b9:	cmpq   $0x0,0x60(%rsp)
   370bf:	je     370cf <meshopt_peers::run+0x239f>
   370c1:	mov    0xa8(%rsp),%rdi
   370c9:	call   *0xf36a9(%rip)        # 12a778 <free@GLIBC_2.2.5>
   370cf:	cmpq   $0x0,0x8(%rsp)
   370d5:	setne  %al
   370d8:	test   %r15b,%al
   370db:	je     3710e <meshopt_peers::run+0x23de>
   370dd:	mov    %rbx,%rdi
   370e0:	jmp    37108 <meshopt_peers::run+0x23d8>
   370e2:	cmpq   $0x0,0x60(%rsp)
   370e8:	je     370f8 <meshopt_peers::run+0x23c8>
   370ea:	mov    0xa8(%rsp),%rdi
   370f2:	call   *0xf3680(%rip)        # 12a778 <free@GLIBC_2.2.5>
   370f8:	mov    $0x8,%bpl
   370fb:	cmpq   $0x0,0x30(%rsp)
   37101:	je     3710e <meshopt_peers::run+0x23de>
   37103:	mov    0x58(%rsp),%rdi
   37108:	call   *0xf366a(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3710e:	lea    0xe0(%rsp),%rdi
   37116:	call   33940 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3711b:	mov    $0x1,%r15b
   3711e:	mov    $0x1,%al
   37120:	mov    %eax,0x8(%rsp)
   37124:	mov    $0x1,%r13b
   37127:	cmpq   $0x0,0x80(%rsp)
   37130:	je     37140 <meshopt_peers::run+0x2410>
   37132:	mov    0xc0(%rsp),%rdi
   3713a:	call   *0xf3638(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37140:	cmpq   $0x0,0x68(%rsp)
   37146:	je     37156 <meshopt_peers::run+0x2426>
   37148:	mov    0xb0(%rsp),%rdi
   37150:	call   *0xf3622(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37156:	test   %r15b,%r15b
   37159:	mov    0x20(%rsp),%rbx
   3715e:	mov    0x10(%rsp),%r12
   37163:	mov    0x40(%rsp),%r15
   37168:	jne    36ee7 <meshopt_peers::run+0x21b7>
   3716e:	jmp    36efa <meshopt_peers::run+0x21ca>
   37173:	mov    0x10(%rsp),%r12
   37178:	jmp    36ee4 <meshopt_peers::run+0x21b4>
   3717d:	cmpq   $0x0,0x68(%rsp)
   37183:	je     371ac <meshopt_peers::run+0x247c>
   37185:	mov    0xb0(%rsp),%rdi
   3718d:	call   *0xf35e5(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37193:	mov    $0x1,%r13b
   37196:	jmp    371b7 <meshopt_peers::run+0x2487>
   37198:	mov    $0x8,%bpl
   3719b:	cmpq   $0x0,0x30(%rsp)
   371a1:	je     3710e <meshopt_peers::run+0x23de>
   371a7:	jmp    37103 <meshopt_peers::run+0x23d3>
   371ac:	mov    $0x1,%r13b
   371af:	jmp    371b7 <meshopt_peers::run+0x2487>
   371b1:	mov    $0x1,%r13b
   371b4:	mov    %r15d,%ebp
   371b7:	mov    0x20(%rsp),%rbx
   371bc:	mov    0x10(%rsp),%r12
   371c1:	mov    0x40(%rsp),%r15
   371c6:	jmp    36ee7 <meshopt_peers::run+0x21b7>
   371cb:	xor    %r15d,%r15d
   371ce:	jmp    37127 <meshopt_peers::run+0x23f7>
   371d3:	shr    $0x8,%ebp
   371d6:	mov    $0x1,%r15b
   371d9:	cmpq   $0x0,0x30(%rsp)
   371df:	jne    370ae <meshopt_peers::run+0x237e>
   371e5:	jmp    370b9 <meshopt_peers::run+0x2389>
   371ea:	mov    %eax,%ebp
   371ec:	xor    %r15d,%r15d
   371ef:	jmp    37127 <meshopt_peers::run+0x23f7>
   371f4:	mov    %eax,%ebp
   371f6:	xor    %r15d,%r15d
   371f9:	movl   $0x0,0x8(%rsp)
   37201:	jmp    37127 <meshopt_peers::run+0x23f7>
   37206:	mov    0xb0(%rsp),%rax
   3720e:	mov    %rax,0x38(%rsp)
   37213:	mov    0xc0(%rsp),%rax
   3721b:	mov    %rax,0x28(%rsp)
   37220:	test   %r15,%r15
   37223:	mov    0x68(%rsp),%r12
   37228:	jne    35fc5 <meshopt_peers::run+0x1295>
   3722e:	jmp    35f02 <meshopt_peers::run+0x11d2>
   37233:	test   %r15,%r15
   37236:	je     37241 <meshopt_peers::run+0x2511>
   37238:	mov    %r13,%rdi
   3723b:	call   *0xf3537(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37241:	mov    $0x1,%r13b
   37244:	mov    $0x1,%r15b
   37247:	cmpq   $0x0,0xa0(%rsp)
   37250:	je     3725f <meshopt_peers::run+0x252f>
   37252:	mov    0x78(%rsp),%rdi
   37257:	call   *0xf351b(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3725d:	jmp    37262 <meshopt_peers::run+0x2532>
   3725f:	mov    $0x1,%r15b
   37262:	cmpq   $0x0,0x30(%rsp)
   37268:	je     37275 <meshopt_peers::run+0x2545>
   3726a:	mov    0x58(%rsp),%rdi
   3726f:	call   *0xf3503(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37275:	mov    $0x8,%bpl
   37278:	test   %r13b,%r13b
   3727b:	jne    370b9 <meshopt_peers::run+0x2389>
   37281:	jmp    370cf <meshopt_peers::run+0x239f>
   37286:	mov    $0xa,%bpl
   37289:	mov    0x20(%rsp),%rbx
   3728e:	mov    0x10(%rsp),%r12
   37293:	jmp    36edf <meshopt_peers::run+0x21af>
   37298:	movb   $0x2,0x738(%rsp)
   372a0:	lea    -0x26cf4(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   372a7:	lea    0xec402(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   372ae:	lea    0xec173(%rip),%r8        # 123428 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   372b5:	lea    0x738(%rsp),%rdx
   372bd:	mov    $0x2b,%esi
   372c2:	call   *0xf3578(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   372c8:	mov    $0x1,%r13b
   372cb:	xor    %r15d,%r15d
   372ce:	mov    %rbx,0x78(%rsp)
   372d3:	cmpq   $0x0,0x8(%rsp)
   372d9:	jne    37252 <meshopt_peers::run+0x2522>
   372df:	jmp    37262 <meshopt_peers::run+0x2532>
   372e1:	movb   $0x8,0x738(%rsp)
   372e9:	lea    -0x26d3d(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   372f0:	lea    0xec3b9(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   372f7:	lea    0xec0ca(%rip),%r8        # 1233c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   372fe:	lea    0x738(%rsp),%rdx
   37306:	mov    $0x2b,%esi
   3730b:	call   *0xf352f(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   37311:	mov    $0x4,%edi
   37316:	mov    %r13,%rsi
   37319:	call   *0xf3491(%rip)        # 12a7b0 <_DYNAMIC+0x240>
   3731f:	lea    0xebf9a(%rip),%rcx        # 1232c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   37326:	mov    %r15,%rsi
   37329:	call   *0xf35b1(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   3732f:	jmp    3742c <meshopt_peers::run+0x26fc>
   37334:	mov    $0x4,%edi
   37339:	mov    %r15,%rsi
   3733c:	call   *0xf346e(%rip)        # 12a7b0 <_DYNAMIC+0x240>
   37342:	jmp    3742c <meshopt_peers::run+0x26fc>
   37347:	xor    %r13d,%r13d
   3734a:	cmpq   $0x0,0x60(%rsp)
   37350:	je     37367 <meshopt_peers::run+0x2637>
   37352:	mov    0xa8(%rsp),%rax
   3735a:	mov    %rax,0x78(%rsp)
   3735f:	xor    %r15d,%r15d
   37362:	jmp    37252 <meshopt_peers::run+0x2522>
   37367:	movq   $0x0,0x60(%rsp)
   37370:	xor    %r15d,%r15d
   37373:	jmp    37262 <meshopt_peers::run+0x2532>
   37378:	mov    $0x8,%bpl
   3737b:	cmpq   $0x0,0x30(%rsp)
   37381:	je     3710e <meshopt_peers::run+0x23de>
   37387:	jmp    37103 <meshopt_peers::run+0x23d3>
   3738c:	lea    0xebf15(%rip),%rcx        # 1232a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   37393:	mov    0xb8(%rsp),%rdi
   3739b:	mov    %rbp,%rsi
   3739e:	mov    0x140(%rsp),%rdx
   373a6:	call   *0xf3534(%rip)        # 12a8e0 <_DYNAMIC+0x370>
   373ac:	jmp    3742c <meshopt_peers::run+0x26fc>
   373ae:	mov    %eax,%ebp
   373b0:	cmpq   $0x0,0x80(%rsp)
   373b9:	je     373c6 <meshopt_peers::run+0x2696>
   373bb:	mov    0x28(%rsp),%rdi
   373c0:	call   *0xf33b2(%rip)        # 12a778 <free@GLIBC_2.2.5>
   373c6:	mov    0x738(%rsp),%rax
   373ce:	mov    0x748(%rsp),%rcx
   373d6:	sub    0x740(%rsp),%rcx
   373de:	mov    %rcx,0x358(%rax)
   373e5:	mov    %bpl,0x738(%rsp)
   373ed:	lea    -0x26e41(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   373f4:	lea    0xec2b5(%rip),%rcx        # 1236b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   373fb:	lea    0xec056(%rip),%r8        # 123458 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   37402:	lea    0x738(%rsp),%rdx
   3740a:	mov    $0x2b,%esi
   3740f:	call   *0xf342b(%rip)        # 12a840 <_DYNAMIC+0x2d0>
   37415:	jmp    3742c <meshopt_peers::run+0x26fc>
   37417:	lea    0xebe72(%rip),%rdx        # 123290 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   3741e:	mov    %r12,%rdi
   37421:	mov    0x48(%rsp),%rsi
   37426:	call   *0xf3394(%rip)        # 12a7c0 <_DYNAMIC+0x250>
   3742c:	ud2
   3742e:	mov    %rax,%r14
   37431:	cmpq   $0x0,0xe0(%rsp)
   3743a:	je     375d3 <meshopt_peers::run+0x28a3>
   37440:	mov    0xe8(%rsp),%rdi
   37448:	jmp    375cd <meshopt_peers::run+0x289d>
   3744d:	mov    %rax,%r14
   37450:	cmpq   $0x0,0xe0(%rsp)
   37459:	je     375e9 <meshopt_peers::run+0x28b9>
   3745f:	mov    0xe8(%rsp),%r15
   37467:	jmp    37496 <meshopt_peers::run+0x2766>
   37469:	jmp    3746b <meshopt_peers::run+0x273b>
   3746b:	mov    %rax,%r14
   3746e:	cmpq   $0x0,0x30(%rsp)
   37474:	je     37508 <meshopt_peers::run+0x27d8>
   3747a:	jmp    37554 <meshopt_peers::run+0x2824>
   3747f:	mov    %rax,%r14
   37482:	jmp    37521 <meshopt_peers::run+0x27f1>
   37487:	mov    %rax,%r14
   3748a:	cmpq   $0x0,0x68(%rsp)
   37490:	je     375e9 <meshopt_peers::run+0x28b9>
   37496:	mov    %r15,%rdi
   37499:	jmp    375e3 <meshopt_peers::run+0x28b3>
   3749e:	mov    %rax,%r14
   374a1:	jmp    375d3 <meshopt_peers::run+0x28a3>
   374a6:	mov    %rax,%r14
   374a9:	mov    %r12,0x10(%rsp)
   374ae:	mov    0x80(%rsp),%rax
   374b6:	mov    %rax,0x70(%rsp)
   374bb:	cmpq   $0x0,0x70(%rsp)
   374c1:	je     375f9 <meshopt_peers::run+0x28c9>
   374c7:	jmp    37672 <meshopt_peers::run+0x2942>
   374cc:	mov    %rax,%r14
   374cf:	cmpq   $0x0,0x70(%rsp)
   374d5:	je     375f9 <meshopt_peers::run+0x28c9>
   374db:	jmp    37672 <meshopt_peers::run+0x2942>
   374e0:	mov    %r15,0x40(%rsp)
   374e5:	mov    %rax,%r14
   374e8:	jmp    375e9 <meshopt_peers::run+0x28b9>
   374ed:	mov    %rax,%r14
   374f0:	cmpq   $0x0,0xa0(%rsp)
   374f9:	jne    37533 <meshopt_peers::run+0x2803>
   374fb:	test   %r15,%r15
   374fe:	jne    37543 <meshopt_peers::run+0x2813>
   37500:	cmpq   $0x0,0x30(%rsp)
   37506:	jne    37554 <meshopt_peers::run+0x2824>
   37508:	cmpq   $0x0,0x60(%rsp)
   3750e:	jne    37567 <meshopt_peers::run+0x2837>
   37510:	cmpq   $0x0,0x8(%rsp)
   37516:	je     37521 <meshopt_peers::run+0x27f1>
   37518:	mov    %rbx,%rdi
   3751b:	call   *0xf3257(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37521:	lea    0xe0(%rsp),%rdi
   37529:	call   33940 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3752e:	jmp    375ba <meshopt_peers::run+0x288a>
   37533:	mov    0x78(%rsp),%rdi
   37538:	call   *0xf323a(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3753e:	test   %r15,%r15
   37541:	je     37500 <meshopt_peers::run+0x27d0>
   37543:	mov    %r13,%rdi
   37546:	call   *0xf322c(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3754c:	cmpq   $0x0,0x30(%rsp)
   37552:	je     37508 <meshopt_peers::run+0x27d8>
   37554:	mov    0x58(%rsp),%rdi
   37559:	call   *0xf3219(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3755f:	cmpq   $0x0,0x60(%rsp)
   37565:	je     37510 <meshopt_peers::run+0x27e0>
   37567:	mov    0xa8(%rsp),%rdi
   3756f:	call   *0xf3203(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37575:	cmpq   $0x0,0x8(%rsp)
   3757b:	jne    37518 <meshopt_peers::run+0x27e8>
   3757d:	jmp    37521 <meshopt_peers::run+0x27f1>
   3757f:	jmp    37581 <meshopt_peers::run+0x2851>
   37581:	mov    %rax,%r14
   37584:	cmpq   $0x0,0x20(%rsp)
   3758a:	je     37611 <meshopt_peers::run+0x28e1>
   37590:	jmp    3769c <meshopt_peers::run+0x296c>
   37595:	mov    %rax,%r14
   37598:	lea    0x350(%rsp),%rdi
   375a0:	call   33ed0 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   375a5:	mov    %r14,%rdi
   375a8:	call   1217c0 <_Unwind_Resume@plt>
   375ad:	jmp    3765a <meshopt_peers::run+0x292a>
   375b2:	jmp    3765a <meshopt_peers::run+0x292a>
   375b7:	mov    %rax,%r14
   375ba:	cmpq   $0x0,0x80(%rsp)
   375c3:	je     375d3 <meshopt_peers::run+0x28a3>
   375c5:	mov    0xc0(%rsp),%rdi
   375cd:	call   *0xf31a5(%rip)        # 12a778 <free@GLIBC_2.2.5>
   375d3:	cmpq   $0x0,0x68(%rsp)
   375d9:	je     375e9 <meshopt_peers::run+0x28b9>
   375db:	mov    0xb0(%rsp),%rdi
   375e3:	call   *0xf318f(%rip)        # 12a778 <free@GLIBC_2.2.5>
   375e9:	cmpq   $0x0,0x40(%rsp)
   375ef:	jne    37625 <meshopt_peers::run+0x28f5>
   375f1:	cmpq   $0x0,0x70(%rsp)
   375f7:	jne    37672 <meshopt_peers::run+0x2942>
   375f9:	cmpq   $0x0,0x10(%rsp)
   375ff:	jne    37685 <meshopt_peers::run+0x2955>
   37605:	cmpq   $0x0,0x20(%rsp)
   3760b:	jne    3769c <meshopt_peers::run+0x296c>
   37611:	cmpq   $0x0,0x150(%rsp)
   3761a:	jne    376b5 <meshopt_peers::run+0x2985>
   37620:	jmp    376c3 <meshopt_peers::run+0x2993>
   37625:	mov    0xd0(%rsp),%rdi
   3762d:	call   *0xf3145(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37633:	cmpq   $0x0,0x70(%rsp)
   37639:	je     375f9 <meshopt_peers::run+0x28c9>
   3763b:	jmp    37672 <meshopt_peers::run+0x2942>
   3763d:	mov    %rax,%r14
   37640:	cmpq   $0x0,0x150(%rsp)
   37649:	jne    376b5 <meshopt_peers::run+0x2985>
   3764b:	jmp    376c3 <meshopt_peers::run+0x2993>
   3764d:	mov    %rax,%r14
   37650:	jmp    376c3 <meshopt_peers::run+0x2993>
   37652:	mov    %rax,%r14
   37655:	jmp    376e2 <meshopt_peers::run+0x29b2>
   3765a:	mov    %rax,%r14
   3765d:	lea    0x350(%rsp),%rdi
   37665:	call   33f10 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3766a:	cmpq   $0x0,0x70(%rsp)
   37670:	je     375f9 <meshopt_peers::run+0x28c9>
   37672:	mov    0x28(%rsp),%rdi
   37677:	call   *0xf30fb(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3767d:	cmpq   $0x0,0x10(%rsp)
   37683:	je     37605 <meshopt_peers::run+0x28d5>
   37685:	mov    0x38(%rsp),%rdi
   3768a:	call   *0xf30e8(%rip)        # 12a778 <free@GLIBC_2.2.5>
   37690:	cmpq   $0x0,0x20(%rsp)
   37696:	je     37611 <meshopt_peers::run+0x28e1>
   3769c:	mov    0x90(%rsp),%rdi
   376a4:	call   *0xf30ce(%rip)        # 12a778 <free@GLIBC_2.2.5>
   376aa:	cmpq   $0x0,0x150(%rsp)
   376b3:	je     376c3 <meshopt_peers::run+0x2993>
   376b5:	mov    0xc8(%rsp),%rdi
   376bd:	call   *0xf30b5(%rip)        # 12a778 <free@GLIBC_2.2.5>
   376c3:	mov    0x738(%rsp),%rax
   376cb:	mov    0x748(%rsp),%rcx
   376d3:	sub    0x740(%rsp),%rcx
   376db:	mov    %rcx,0x358(%rax)
   376e2:	cmpq   $0x0,0x158(%rsp)
   376eb:	jne    376fd <meshopt_peers::run+0x29cd>
   376ed:	cmpq   $0x0,0x20(%rsp)
   376f3:	jne    37713 <meshopt_peers::run+0x29e3>
   376f5:	mov    %r14,%rdi
   376f8:	call   1217c0 <_Unwind_Resume@plt>
   376fd:	mov    0x160(%rsp),%rdi
   37705:	call   *0xf306d(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3770b:	cmpq   $0x0,0x20(%rsp)
   37711:	je     376f5 <meshopt_peers::run+0x29c5>
   37713:	mov    0x50(%rsp),%rdi
   37718:	call   *0xf305a(%rip)        # 12a778 <free@GLIBC_2.2.5>
   3771e:	mov    %r14,%rdi
   37721:	call   1217c0 <_Unwind_Resume@plt>
