
<artifacts>/guard/driver-control:     file format elf64-x86-64


Disassembly of section .text:

0000000000039f50 <meshopt_peers::run>:
   39f50:	push   %rbp
   39f51:	push   %r15
   39f53:	push   %r14
   39f55:	push   %r13
   39f57:	push   %r12
   39f59:	push   %rbx
   39f5a:	sub    $0x1000,%rsp
   39f61:	movq   $0x0,(%rsp)
   39f69:	sub    $0x988,%rsp
   39f70:	mov    %rdi,%rbx
   39f73:	mov    %rsi,0x6d0(%rsp)
   39f7b:	mov    %rdx,0x6d8(%rsp)
   39f83:	mov    0x28(%rcx),%r14
   39f87:	mov    0x20(%rcx),%r13
   39f8b:	movb   $0x2,0x688(%rsp)
   39f93:	mov    %r13,0x690(%rsp)
   39f9b:	mov    %r14,0x698(%rsp)
   39fa3:	movq   $0x0,0x6c0(%rsp)
   39faf:	mov    0x98(%rcx),%rax
   39fb6:	movd   0xa0(%rcx),%xmm0
   39fbe:	mov    %rax,0x220(%rsp)
   39fc6:	movd   %xmm0,0x228(%rsp)
   39fcf:	movl   $0x0,0x22c(%rsp)
   39fda:	lea    -0x5(%rdx),%rdi
   39fde:	cmp    $0xe,%rdi
   39fe2:	ja     3a980 <meshopt_peers::run+0xa30>
   39fe8:	mov    %r8,%r11
   39feb:	lea    -0x28c26(%rip),%r8        # 113cc <libm::math::generic::sqrt::RSQRT_TAB+0x122>
   39ff2:	movslq (%r8,%rdi,4),%rdi
   39ff6:	add    %r8,%rdi
   39ff9:	jmp    *%rdi
   39ffb:	mov    $0x68636163,%eax
   3a000:	xor    (%rsi),%eax
   3a002:	movzbl 0x4(%rsi),%edi
   3a006:	xor    $0x65,%edi
   3a009:	or     %eax,%edi
   3a00b:	je     3acc8 <meshopt_peers::run+0xd78>
   3a011:	mov    $0x63746566,%eax
   3a016:	xor    (%rsi),%eax
   3a018:	movzbl 0x4(%rsi),%edi
   3a01c:	xor    $0x68,%edi
   3a01f:	or     %eax,%edi
   3a021:	je     3b219 <meshopt_peers::run+0x12c9>
   3a027:	mov    $0x616d6572,%eax
   3a02c:	xor    (%rsi),%eax
   3a02e:	movzbl 0x4(%rsi),%edi
   3a032:	xor    $0x70,%edi
   3a035:	or     %eax,%edi
   3a037:	je     3a9bb <meshopt_peers::run+0xa6b>
   3a03d:	jmp    3a980 <meshopt_peers::run+0xa30>
   3a042:	movabs $0x7966696c706d6973,%rax
   3a04c:	cmp    %rax,(%rsi)
   3a04f:	je     3abab <meshopt_peers::run+0xc5b>
   3a055:	movabs $0x776172647265766f,%rax
   3a05f:	cmp    %rax,(%rsi)
   3a062:	je     3b142 <meshopt_peers::run+0x11f2>
   3a068:	movabs $0x7374656c6873656d,%rax
   3a072:	cmp    %rax,(%rsi)
   3a075:	jne    3a980 <meshopt_peers::run+0xa30>
   3a07b:	mov    0x38(%rcx),%rsi
   3a07f:	mov    0x40(%rcx),%rdx
   3a083:	movq   $0x7c,0x320(%rsp)
   3a08f:	movq   $0x0,0x314(%rsp)
   3a09b:	movl   $0x1,0x310(%rsp)
   3a0a6:	lea    0x310(%rsp),%rax
   3a0ae:	lea    0x750(%rsp),%rdi
   3a0b6:	lea    0x688(%rsp),%r8
   3a0be:	mov    $0x40,%r9d
   3a0c4:	mov    %r14,%rcx
   3a0c7:	push   $0x2
   3a0c9:	push   %r11
   3a0cb:	push   %rax
   3a0cc:	push   $0x7c
   3a0ce:	call   c7a90 <meshoptimizer_rs::meshlet::build>
   3a0d3:	add    $0x20,%rsp
   3a0d7:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a0e0:	je     3d044 <meshopt_peers::run+0x30f4>
   3a0e6:	mov    0x790(%rsp),%rax
   3a0ee:	mov    %rax,0x350(%rsp)
   3a0f6:	movdqu 0x750(%rsp),%xmm0
   3a0ff:	movups 0x760(%rsp),%xmm1
   3a107:	movdqu 0x770(%rsp),%xmm2
   3a110:	movdqu 0x780(%rsp),%xmm3
   3a119:	movdqa %xmm3,0x340(%rsp)
   3a122:	movdqa %xmm2,0x330(%rsp)
   3a12b:	movaps %xmm1,0x320(%rsp)
   3a133:	movdqa %xmm0,0x310(%rsp)
   3a13c:	mov    0x318(%rsp),%r14
   3a144:	mov    0x320(%rsp),%r12
   3a14c:	test   %r12,%r12
   3a14f:	je     3b904 <meshopt_peers::run+0x19b4>
   3a155:	mov    %r12,%r15
   3a158:	shl    $0x4,%r15
   3a15c:	mov    %r15,%rdi
   3a15f:	call   *0x1100d3(%rip)        # 14a238 <malloc@GLIBC_2.2.5>
   3a165:	test   %rax,%rax
   3a168:	je     3d5a0 <meshopt_peers::run+0x3650>
   3a16e:	mov    %r12d,%ecx
   3a171:	and    $0x3,%ecx
   3a174:	cmp    $0x4,%r12
   3a178:	jae    3b976 <meshopt_peers::run+0x1a26>
   3a17e:	xor    %edx,%edx
   3a180:	jmp    3b9ce <meshopt_peers::run+0x1a7e>
   3a185:	movabs $0x7374656c6873656d,%rax
   3a18f:	xor    (%rsi),%rax
   3a192:	movabs $0x78656c662d737465,%rdx
   3a19c:	xor    0x5(%rsi),%rdx
   3a1a0:	or     %rax,%rdx
   3a1a3:	je     3ac44 <meshopt_peers::run+0xcf4>
   3a1a9:	movabs $0x65646f6365642d78,%rax
   3a1b3:	movabs $0x652d786574726576,%rdx
   3a1bd:	xor    (%rsi),%rdx
   3a1c0:	lea    0x9010000(%rax),%rdi
   3a1c7:	xor    0x5(%rsi),%rdi
   3a1cb:	or     %rdx,%rdi
   3a1ce:	je     3b1b1 <meshopt_peers::run+0x1261>
   3a1d4:	movabs $0x642d786574726576,%rdx
   3a1de:	xor    (%rsi),%rdx
   3a1e1:	xor    0x5(%rsi),%rax
   3a1e5:	or     %rdx,%rax
   3a1e8:	jne    3a980 <meshopt_peers::run+0xa30>
   3a1ee:	mov    0x68(%rcx),%r9
   3a1f2:	lea    0x750(%rsp),%rdi
   3a1fa:	mov    $0xc,%r8d
   3a200:	xor    %esi,%esi
   3a202:	xor    %edx,%edx
   3a204:	mov    %rcx,%rax
   3a207:	mov    %r14,%rcx
   3a20a:	push   %r11
   3a20c:	push   0x70(%rax)
   3a20f:	call   be4b0 <meshoptimizer_rs::codec::allocate>
   3a214:	add    $0x10,%rsp
   3a218:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a221:	jne    3b1f4 <meshopt_peers::run+0x12a4>
   3a227:	movzbl 0x758(%rsp),%eax
   3a22f:	mov    %al,0x310(%rsp)
   3a236:	lea    -0x28db2(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a23d:	lea    0x10855c(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a244:	lea    0x108195(%rip),%r8        # 1423e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x350>
   3a24b:	lea    0x310(%rsp),%rdx
   3a253:	mov    $0x2b,%esi
   3a258:	call   *0x1100b2(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3a25e:	mov    $0x64616873,%eax
   3a263:	xor    (%rsi),%eax
   3a265:	movzwl 0x4(%rsi),%edi
   3a269:	xor    $0x776f,%edi
   3a26f:	or     %eax,%edi
   3a271:	jne    3a980 <meshopt_peers::run+0xa30>
   3a277:	mov    %r14,%rax
   3a27a:	shr    $0x20,%rax
   3a27e:	jne    3d43a <meshopt_peers::run+0x34ea>
   3a284:	lea    0x0(,%r14,4),%rax
   3a28c:	lea    (%rax,%rax,2),%rax
   3a290:	mov    %r13,0x310(%rsp)
   3a298:	mov    %rax,0x318(%rsp)
   3a2a0:	mov    %r14,0x320(%rsp)
   3a2a8:	movq   $0xc,0x328(%rsp)
   3a2b4:	movq   $0xc,0x330(%rsp)
   3a2c0:	movq   $0x0,0x338(%rsp)
   3a2cc:	cmp    $0x6,%rdx
   3a2d0:	jne    3a2eb <meshopt_peers::run+0x39b>
   3a2d2:	mov    $0x64616873,%eax
   3a2d7:	xor    (%rsi),%eax
   3a2d9:	movzwl 0x4(%rsi),%edx
   3a2dd:	xor    $0x776f,%edx
   3a2e3:	or     %eax,%edx
   3a2e5:	je     3b4e1 <meshopt_peers::run+0x1591>
   3a2eb:	test   %r14,%r14
   3a2ee:	setne  %dl
   3a2f1:	mov    0x58(%rcx),%rax
   3a2f5:	shl    $0x2,%rax
   3a2f9:	mov    %r14,%rsi
   3a2fc:	shl    $0x4,%rsi
   3a300:	cmp    %rax,%rsi
   3a303:	seta   %sil
   3a307:	and    %dl,%sil
   3a30a:	cmp    $0x1,%sil
   3a30e:	je     3d4ca <meshopt_peers::run+0x357a>
   3a314:	mov    0x38(%rcx),%rsi
   3a318:	mov    0x40(%rcx),%rdx
   3a31c:	mov    0x50(%rcx),%rcx
   3a320:	movdqu 0x310(%rsp),%xmm0
   3a329:	movups 0x320(%rsp),%xmm1
   3a331:	movdqu 0x330(%rsp),%xmm2
   3a33a:	movdqa %xmm2,0x770(%rsp)
   3a343:	movaps %xmm1,0x760(%rsp)
   3a34b:	movdqa %xmm0,0x750(%rsp)
   3a354:	mov    %rcx,0x780(%rsp)
   3a35c:	mov    %rax,0x788(%rsp)
   3a364:	mov    %r14,0x790(%rsp)
   3a36c:	movq   $0x10,0x798(%rsp)
   3a378:	movq   $0x10,0x7a0(%rsp)
   3a384:	movq   $0x0,0x7a8(%rsp)
   3a390:	sub    $0x10,%rsp
   3a394:	mov    %r11,0x8(%rsp)
   3a399:	lea    0x270(%rsp),%rdi
   3a3a1:	lea    0x760(%rsp),%rcx
   3a3a9:	mov    $0x2,%r8d
   3a3af:	xor    %r9d,%r9d
   3a3b2:	call   682d0 <meshoptimizer_rs::remap::shadow_filter::<false>>
   3a3b7:	add    $0x10,%rsp
   3a3bb:	mov    0x260(%rsp),%rax
   3a3c3:	movzbl 0x268(%rsp),%ecx
   3a3cb:	cmp    $0xffffffffffffffff,%rax
   3a3cf:	je     3a3f1 <meshopt_peers::run+0x4a1>
   3a3d1:	mov    0x269(%rsp),%rdx
   3a3d9:	mov    0x270(%rsp),%rsi
   3a3e1:	mov    %rsi,0x100(%rsp)
   3a3e9:	mov    %rdx,0xf9(%rsp)
   3a3f1:	mov    %cl,0xf8(%rsp)
   3a3f8:	mov    %rax,0xf0(%rsp)
   3a400:	cmp    $0xffffffffffffffff,%rax
   3a404:	je     3b514 <meshopt_peers::run+0x15c4>
   3a40a:	mov    0x100(%rsp),%rax
   3a412:	mov    %rax,0x18(%rbx)
   3a416:	movdqu 0xf0(%rsp),%xmm0
   3a41f:	jmp    3b4d1 <meshopt_peers::run+0x1581>
   3a424:	movabs $0x7966696c706d6973,%rax
   3a42e:	xor    (%rsi),%rax
   3a431:	movabs $0x656e7572702d7966,%rdx
   3a43b:	xor    0x6(%rsi),%rdx
   3a43f:	or     %rax,%rdx
   3a442:	je     3ad2a <meshopt_peers::run+0xdda>
   3a448:	movabs $0x6e6f697469736f70,%rax
   3a452:	xor    (%rsi),%rax
   3a455:	movabs $0x70616d65722d6e6f,%rcx
   3a45f:	xor    0x6(%rsi),%rcx
   3a463:	or     %rax,%rcx
   3a466:	jne    3a980 <meshopt_peers::run+0xa30>
   3a46c:	lea    0x750(%rsp),%rdi
   3a474:	lea    0x688(%rsp),%rsi
   3a47c:	mov    %r11,%rdx
   3a47f:	call   *0x11018b(%rip)        # 14a610 <_DYNAMIC+0x5d0>
   3a485:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a48e:	jne    3b4bc <meshopt_peers::run+0x156c>
   3a494:	movzbl 0x758(%rsp),%eax
   3a49c:	mov    %al,0x310(%rsp)
   3a4a3:	lea    -0x2901f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a4aa:	lea    0x1082ef(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a4b1:	lea    0x107eb0(%rip),%r8        # 142368 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2d8>
   3a4b8:	lea    0x310(%rsp),%rdx
   3a4c0:	mov    $0x2b,%esi
   3a4c5:	call   *0x10fe45(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3a4cb:	movabs $0x7966696c706d6973,%rdx
   3a4d5:	mov    (%rsi),%rdi
   3a4d8:	xor    %rdx,%rdi
   3a4db:	movabs $0x7970706f6c732d79,%r8
   3a4e5:	xor    0x7(%rsi),%r8
   3a4e9:	or     %rdi,%r8
   3a4ec:	je     3ada0 <meshopt_peers::run+0xe50>
   3a4f2:	mov    (%rsi),%rax
   3a4f5:	xor    %rdx,%rax
   3a4f8:	movabs $0x6574616470752d79,%rdi
   3a502:	xor    0x7(%rsi),%rdi
   3a506:	or     %rax,%rdi
   3a509:	je     3b315 <meshopt_peers::run+0x13c5>
   3a50f:	xor    (%rsi),%rdx
   3a512:	movabs $0x73746e696f702d79,%rax
   3a51c:	xor    0x7(%rsi),%rax
   3a520:	or     %rdx,%rax
   3a523:	jne    3a980 <meshopt_peers::run+0xa30>
   3a529:	movb   $0xff,0x750(%rsp)
   3a531:	shr    $1,%r14
   3a534:	sub    $0x8,%rsp
   3a538:	lea    0x318(%rsp),%rdi
   3a540:	lea    0x690(%rsp),%rsi
   3a548:	lea    0x758(%rsp),%rdx
   3a550:	pxor   %xmm0,%xmm0
   3a554:	mov    %r14,%rcx
   3a557:	xor    %r8d,%r8d
   3a55a:	push   %r11
   3a55c:	call   f1c30 <meshoptimizer_rs::simplify::extra::simplify_points_kernel>
   3a561:	add    $0x10,%rsp
   3a565:	mov    0x310(%rsp),%rcx
   3a56d:	movzbl 0x318(%rsp),%eax
   3a575:	cmp    $0xffffffffffffffff,%rcx
   3a579:	je     3d50d <meshopt_peers::run+0x35bd>
   3a57f:	mov    0x319(%rsp),%rdx
   3a587:	mov    0x320(%rsp),%rsi
   3a58f:	jmp    3ad86 <meshopt_peers::run+0xe36>
   3a594:	movabs $0x756d2d70616d6572,%rax
   3a59e:	xor    (%rsi),%rax
   3a5a1:	movabs $0x69746c756d2d7061,%rdi
   3a5ab:	xor    0x3(%rsi),%rdi
   3a5af:	or     %rax,%rdi
   3a5b2:	je     3a9bb <meshopt_peers::run+0xa6b>
   3a5b8:	movabs $0x6e692d70616d6572,%rax
   3a5c2:	xor    (%rsi),%rax
   3a5c5:	movabs $0x7865646e692d7061,%rdx
   3a5cf:	xor    0x3(%rsi),%rdx
   3a5d3:	or     %rax,%rdx
   3a5d6:	je     3b489 <meshopt_peers::run+0x1539>
   3a5dc:	movabs $0x2d72657473756c63,%rax
   3a5e6:	xor    (%rsi),%rax
   3a5e9:	movabs $0x646f6c2d72657473,%rdx
   3a5f3:	xor    0x3(%rsi),%rdx
   3a5f7:	or     %rax,%rdx
   3a5fa:	jne    3a980 <meshopt_peers::run+0xa30>
   3a600:	mov    %r14,%rax
   3a603:	mov    %rbx,(%rsp)
   3a607:	test   %r14,%r14
   3a60a:	je     3b5fb <meshopt_peers::run+0x16ab>
   3a610:	mov    %r11,%rbp
   3a613:	mov    %rcx,%rbx
   3a616:	mov    %rax,%r14
   3a619:	lea    0x0(,%rax,4),%rax
   3a621:	lea    (%rax,%rax,2),%r15
   3a625:	mov    %r15,%rdi
   3a628:	call   *0x10fc0a(%rip)        # 14a238 <malloc@GLIBC_2.2.5>
   3a62e:	test   %rax,%rax
   3a631:	je     3d592 <meshopt_peers::run+0x3642>
   3a637:	mov    %rax,%r12
   3a63a:	mov    %rax,%rdi
   3a63d:	mov    %r13,%rsi
   3a640:	mov    %r15,%rdx
   3a643:	call   *0x10fc0f(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   3a649:	mov    %rbx,%rcx
   3a64c:	mov    %rbp,%r11
   3a64f:	jmp    3b604 <meshopt_peers::run+0x16b4>
   3a654:	movabs $0x6e652d7865646e69,%rax
   3a65e:	xor    (%rsi),%rax
   3a661:	mov    0x8(%rsi),%edi
   3a664:	xor    $0x65646f63,%rdi
   3a66b:	or     %rax,%rdi
   3a66e:	je     3af1d <meshopt_peers::run+0xfcd>
   3a674:	movabs $0x75632d70616d6572,%rax
   3a67e:	xor    (%rsi),%rax
   3a681:	mov    0x8(%rsi),%edi
   3a684:	xor    $0x6d6f7473,%rdi
   3a68b:	or     %rax,%rdi
   3a68e:	je     3a9bb <meshopt_peers::run+0xa6b>
   3a694:	movabs $0x65762d70616d6572,%rax
   3a69e:	xor    (%rsi),%rax
   3a6a1:	mov    0x8(%rsi),%edi
   3a6a4:	xor    $0x78657472,%rdi
   3a6ab:	or     %rax,%rdi
   3a6ae:	je     3b55c <meshopt_peers::run+0x160c>
   3a6b4:	movabs $0x6d2d776f64616873,%rax
   3a6be:	xor    (%rsi),%rax
   3a6c1:	mov    0x8(%rsi),%edi
   3a6c4:	xor    $0x69746c75,%rdi
   3a6cb:	or     %rax,%rdi
   3a6ce:	je     3a277 <meshopt_peers::run+0x327>
   3a6d4:	movabs $0x65642d7865646e69,%rax
   3a6de:	xor    (%rsi),%rax
   3a6e1:	mov    0x8(%rsi),%edx
   3a6e4:	xor    $0x65646f63,%rdx
   3a6eb:	or     %rax,%rdx
   3a6ee:	jne    3a980 <meshopt_peers::run+0xa30>
   3a6f4:	mov    0x40(%rcx),%rax
   3a6f8:	mov    0x80(%rcx),%r9
   3a6ff:	lea    0x750(%rsp),%rdi
   3a707:	mov    $0x4,%r8d
   3a70d:	mov    $0x1,%esi
   3a712:	xor    %edx,%edx
   3a714:	mov    %rcx,%r10
   3a717:	mov    %rax,%rcx
   3a71a:	push   %r11
   3a71c:	push   0x88(%r10)
   3a723:	call   be4b0 <meshoptimizer_rs::codec::allocate>
   3a728:	add    $0x10,%rsp
   3a72c:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a735:	jne    3b1f4 <meshopt_peers::run+0x12a4>
   3a73b:	movzbl 0x758(%rsp),%eax
   3a743:	mov    %al,0x310(%rsp)
   3a74a:	lea    -0x292c6(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a751:	lea    0x108048(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a758:	lea    0x107b61(%rip),%r8        # 1422c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x230>
   3a75f:	lea    0x310(%rsp),%rdx
   3a767:	mov    $0x2b,%esi
   3a76c:	call   *0x10fb9e(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3a772:	movdqu (%rsi),%xmm0
   3a776:	movdqu 0x3(%rsi),%xmm1
   3a77b:	pcmpeqb -0x2b893(%rip),%xmm1        # eef0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xa0>
   3a783:	pcmpeqb -0x2bd2b(%rip),%xmm0        # ea60 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x350>
   3a78b:	pand   %xmm1,%xmm0
   3a78f:	pmovmskb %xmm0,%eax
   3a793:	cmp    $0xffff,%eax
   3a798:	jne    3a980 <meshopt_peers::run+0xa30>
   3a79e:	mov    0x58(%rcx),%rax
   3a7a2:	test   %r14,%r14
   3a7a5:	setne  %dl
   3a7a8:	lea    0x0(,%r14,4),%rsi
   3a7b0:	cmp    %rax,%rsi
   3a7b3:	seta   %sil
   3a7b7:	and    %dl,%sil
   3a7ba:	cmp    $0x1,%sil
   3a7be:	je     3d49a <meshopt_peers::run+0x354a>
   3a7c4:	mov    0x38(%rcx),%rsi
   3a7c8:	mov    0x40(%rcx),%rdx
   3a7cc:	mov    0x50(%rcx),%rcx
   3a7d0:	movb   $0x0,0x750(%rsp)
   3a7d8:	mov    %rcx,0x758(%rsp)
   3a7e0:	mov    %rax,0x760(%rsp)
   3a7e8:	mov    %r14,0x768(%rsp)
   3a7f0:	movq   $0x4,0x770(%rsp)
   3a7fc:	movq   $0x4,0x778(%rsp)
   3a808:	pxor   %xmm0,%xmm0
   3a80c:	movdqu %xmm0,0x780(%rsp)
   3a815:	sub    $0x30,%rsp
   3a819:	mov    %r11,0x20(%rsp)
   3a81e:	lea    0x250(%rsp),%rax
   3a826:	mov    %rax,0x18(%rsp)
   3a82b:	movq   $0x0,0x8(%rsp)
   3a834:	movq   $0x4,(%rsp)
   3a83c:	lea    -0x2b903(%rip),%r9        # ef40 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xf0>
   3a843:	lea    0x290(%rsp),%rdi
   3a84b:	lea    0x6b8(%rsp),%rcx
   3a853:	lea    0x780(%rsp),%r8
   3a85b:	jmp    3abf8 <meshopt_peers::run+0xca8>
   3a860:	movdqu (%rsi),%xmm0
   3a864:	pcmpeqb -0x2bf9c(%rip),%xmm0        # e8d0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x1c0>
   3a86c:	pmovmskb %xmm0,%eax
   3a870:	cmp    $0xffff,%eax
   3a875:	jne    3a980 <meshopt_peers::run+0xa30>
   3a87b:	mov    0x38(%rcx),%rsi
   3a87f:	mov    0x40(%rcx),%rdx
   3a883:	movq   $0x29,0x268(%rsp)
   3a88f:	movabs $0x3f00000000000002,%rax
   3a899:	mov    %rax,0x260(%rsp)
   3a8a1:	lea    0x260(%rsp),%rax
   3a8a9:	lea    0x750(%rsp),%rdi
   3a8b1:	lea    0x688(%rsp),%r8
   3a8b9:	mov    $0x40,%r9d
   3a8bf:	mov    %r14,%rcx
   3a8c2:	push   $0x2
   3a8c4:	push   %r11
   3a8c6:	push   %rax
   3a8c7:	push   $0x7c
   3a8c9:	call   c7a90 <meshoptimizer_rs::meshlet::build>
   3a8ce:	add    $0x20,%rsp
   3a8d2:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a8db:	je     3ac91 <meshopt_peers::run+0xd41>
   3a8e1:	mov    0x790(%rsp),%rax
   3a8e9:	mov    %rax,0x350(%rsp)
   3a8f1:	movdqu 0x750(%rsp),%xmm0
   3a8fa:	movups 0x760(%rsp),%xmm1
   3a902:	movdqu 0x770(%rsp),%xmm2
   3a90b:	movdqu 0x780(%rsp),%xmm3
   3a914:	movdqa %xmm3,0x340(%rsp)
   3a91d:	movdqa %xmm2,0x330(%rsp)
   3a926:	movaps %xmm1,0x320(%rsp)
   3a92e:	movdqa %xmm0,0x310(%rsp)
   3a937:	mov    0x318(%rsp),%r14
   3a93f:	mov    0x320(%rsp),%r12
   3a947:	test   %r12,%r12
   3a94a:	je     3aad4 <meshopt_peers::run+0xb84>
   3a950:	mov    %r12,%r15
   3a953:	shl    $0x4,%r15
   3a957:	mov    %r15,%rdi
   3a95a:	call   *0x10f8d8(%rip)        # 14a238 <malloc@GLIBC_2.2.5>
   3a960:	test   %rax,%rax
   3a963:	je     3d4fa <meshopt_peers::run+0x35aa>
   3a969:	mov    %r12d,%ecx
   3a96c:	and    $0x3,%ecx
   3a96f:	cmp    $0x4,%r12
   3a973:	jae    3aadb <meshopt_peers::run+0xb8b>
   3a979:	xor    %edx,%edx
   3a97b:	jmp    3ab2e <meshopt_peers::run+0xbde>
   3a980:	lea    0x6d0(%rsp),%rax
   3a988:	mov    %rax,0x750(%rsp)
   3a990:	lea    0x8959(%rip),%rax        # 432f0 <<&str as core::fmt::Display>::fmt>
   3a997:	mov    %rax,0x758(%rsp)
   3a99f:	lea    -0x2d44d(%rip),%rdi        # d559 <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x14a>
   3a9a6:	lea    0x1078fb(%rip),%rdx        # 1422a8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x218>
   3a9ad:	lea    0x750(%rsp),%rsi
   3a9b5:	call   *0x10f925(%rip)        # 14a2e0 <_DYNAMIC+0x2a0>
   3a9bb:	mov    %r14,%rax
   3a9be:	shr    $0x20,%rax
   3a9c2:	jne    3d46a <meshopt_peers::run+0x351a>
   3a9c8:	lea    0x0(,%r14,4),%rax
   3a9d0:	lea    (%rax,%rax,2),%rax
   3a9d4:	cmp    $0x5,%rdx
   3a9d8:	je     3af81 <meshopt_peers::run+0x1031>
   3a9de:	cmp    $0xb,%rdx
   3a9e2:	jne    3af97 <meshopt_peers::run+0x1047>
   3a9e8:	movabs $0x756d2d70616d6572,%rdx
   3a9f2:	xor    (%rsi),%rdx
   3a9f5:	movabs $0x69746c756d2d7061,%rdi
   3a9ff:	xor    0x3(%rsi),%rdi
   3aa03:	or     %rdx,%rdi
   3aa06:	jne    3af97 <meshopt_peers::run+0x1047>
   3aa0c:	test   %r14,%r14
   3aa0f:	setne  %dl
   3aa12:	mov    0x58(%rcx),%rdi
   3aa16:	shl    $0x2,%rdi
   3aa1a:	mov    %r14,%rsi
   3aa1d:	shl    $0x4,%rsi
   3aa21:	cmp    %rdi,%rsi
   3aa24:	seta   %sil
   3aa28:	and    %dl,%sil
   3aa2b:	cmp    $0x1,%sil
   3aa2f:	je     3d64e <meshopt_peers::run+0x36fe>
   3aa35:	mov    0x38(%rcx),%rsi
   3aa39:	mov    0x40(%rcx),%rdx
   3aa3d:	mov    0x50(%rcx),%rcx
   3aa41:	mov    %r13,0x750(%rsp)
   3aa49:	mov    %rax,0x758(%rsp)
   3aa51:	mov    %r14,0x760(%rsp)
   3aa59:	movq   $0xc,0x768(%rsp)
   3aa65:	movq   $0xc,0x770(%rsp)
   3aa71:	movq   $0x0,0x778(%rsp)
   3aa7d:	mov    %rcx,0x780(%rsp)
   3aa85:	mov    %rdi,0x788(%rsp)
   3aa8d:	mov    %r14,0x790(%rsp)
   3aa95:	movq   $0x10,0x798(%rsp)
   3aaa1:	movq   $0x10,0x7a0(%rsp)
   3aaad:	movq   $0x0,0x7a8(%rsp)
   3aab9:	lea    0x310(%rsp),%rdi
   3aac1:	lea    0x750(%rsp),%rcx
   3aac9:	mov    $0x2,%r8d
   3aacf:	jmp    3b968 <meshopt_peers::run+0x1a18>
   3aad4:	mov    $0x4,%eax
   3aad9:	jmp    3ab53 <meshopt_peers::run+0xc03>
   3aadb:	mov    %r12,%rsi
   3aade:	and    $0xfffffffffffffffc,%rsi
   3aae2:	mov    $0x30,%edi
   3aae7:	xor    %edx,%edx
   3aae9:	nopl   0x0(%rax)
   3aaf0:	movups -0x30(%r14,%rdi,1),%xmm0
   3aaf6:	movups %xmm0,-0x30(%rax,%rdi,1)
   3aafb:	movups -0x20(%r14,%rdi,1),%xmm0
   3ab01:	movups %xmm0,-0x20(%rax,%rdi,1)
   3ab06:	movups -0x10(%r14,%rdi,1),%xmm0
   3ab0c:	movups %xmm0,-0x10(%rax,%rdi,1)
   3ab11:	movdqu (%r14,%rdi,1),%xmm0
   3ab17:	movdqu %xmm0,(%rax,%rdi,1)
   3ab1c:	add    $0x4,%rdx
   3ab20:	add    $0x40,%rdi
   3ab24:	cmp    %rdx,%rsi
   3ab27:	jne    3aaf0 <meshopt_peers::run+0xba0>
   3ab29:	test   %rcx,%rcx
   3ab2c:	je     3ab53 <meshopt_peers::run+0xc03>
   3ab2e:	shl    $0x4,%rdx
   3ab32:	mov    %rax,%rsi
   3ab35:	add    %rdx,%rsi
   3ab38:	add    %r14,%rdx
   3ab3b:	shl    $0x4,%ecx
   3ab3e:	xor    %edi,%edi
   3ab40:	movdqu (%rdx,%rdi,1),%xmm0
   3ab45:	movdqu %xmm0,(%rsi,%rdi,1)
   3ab4a:	add    $0x10,%rdi
   3ab4e:	cmp    %rdi,%rcx
   3ab51:	jne    3ab40 <meshopt_peers::run+0xbf0>
   3ab53:	mov    0x338(%rsp),%rcx
   3ab5b:	mov    %rcx,0x30(%rbx)
   3ab5f:	movups 0x328(%rsp),%xmm0
   3ab67:	movups %xmm0,0x20(%rbx)
   3ab6b:	movdqa 0x340(%rsp),%xmm0
   3ab74:	movdqu %xmm0,0x38(%rbx)
   3ab79:	mov    0x350(%rsp),%rcx
   3ab81:	mov    %rcx,0x48(%rbx)
   3ab85:	mov    %r12,0x8(%rbx)
   3ab89:	mov    %rax,0x10(%rbx)
   3ab8d:	mov    %r12,0x18(%rbx)
   3ab91:	movl   $0x6,(%rbx)
   3ab97:	cmpq   $0x0,0x310(%rsp)
   3aba0:	je     3dc16 <meshopt_peers::run+0x3cc6>
   3aba6:	jmp    3ba46 <meshopt_peers::run+0x1af6>
   3abab:	mov    0x38(%rcx),%rsi
   3abaf:	mov    0x40(%rcx),%rdx
   3abb3:	movb   $0x2,0x750(%rsp)
   3abbb:	sub    $0x30,%rsp
   3abbf:	mov    %r11,0x20(%rsp)
   3abc4:	lea    0x250(%rsp),%rax
   3abcc:	mov    %rax,0x18(%rsp)
   3abd1:	pxor   %xmm0,%xmm0
   3abd5:	movdqu %xmm0,(%rsp)
   3abda:	lea    0x290(%rsp),%rdi
   3abe2:	lea    0x6b8(%rsp),%rcx
   3abea:	lea    0x780(%rsp),%r8
   3abf2:	mov    $0x4,%r9d
   3abf8:	call   d2730 <meshoptimizer_rs::simplify::allocating>
   3abfd:	add    $0x30,%rsp
   3ac01:	mov    0x260(%rsp),%rax
   3ac09:	cmp    $0xffffffffffffffff,%rax
   3ac0d:	je     3aee6 <meshopt_peers::run+0xf96>
   3ac13:	mov    0x270(%rsp),%rax
   3ac1b:	mov    %rax,0x18(%rbx)
   3ac1f:	movups 0x260(%rsp),%xmm0
   3ac27:	movups %xmm0,0x8(%rbx)
   3ac2b:	movd   0x278(%rsp),%xmm0
   3ac34:	movd   %xmm0,0x4(%rbx)
   3ac39:	movl   $0x4,(%rbx)
   3ac3f:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3ac44:	mov    0x38(%rcx),%rsi
   3ac48:	mov    0x40(%rcx),%rdx
   3ac4c:	lea    0x750(%rsp),%rdi
   3ac54:	lea    0x688(%rsp),%rcx
   3ac5c:	movss  -0x2a1f8(%rip),%xmm1        # 10a6c <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa0>
   3ac64:	mov    $0x40,%r8d
   3ac6a:	mov    $0x7c,%r9d
   3ac70:	pxor   %xmm0,%xmm0
   3ac74:	push   %r11
   3ac76:	push   $0x29
   3ac78:	call   *0x10f99a(%rip)        # 14a618 <_DYNAMIC+0x5d8>
   3ac7e:	add    $0x10,%rsp
   3ac82:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3ac8b:	jne    3a8e1 <meshopt_peers::run+0x991>
   3ac91:	movzbl 0x758(%rsp),%eax
   3ac99:	mov    %al,0x260(%rsp)
   3aca0:	lea    -0x2981c(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3aca7:	lea    0x107af2(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3acae:	lea    0x10775b(%rip),%r8        # 142410 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x380>
   3acb5:	lea    0x260(%rsp),%rdx
   3acbd:	mov    $0x2b,%esi
   3acc2:	call   *0x10f648(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3acc8:	mov    0x38(%rcx),%rsi
   3accc:	mov    0x40(%rcx),%rdx
   3acd0:	lea    0x750(%rsp),%rdi
   3acd8:	mov    %r14,%rcx
   3acdb:	mov    %r11,%r8
   3acde:	call   *0x10f93c(%rip)        # 14a620 <_DYNAMIC+0x5e0>
   3ace4:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3aced:	jne    3b4bc <meshopt_peers::run+0x156c>
   3acf3:	movzbl 0x758(%rsp),%eax
   3acfb:	mov    %al,0x310(%rsp)
   3ad02:	lea    -0x2987e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3ad09:	lea    0x107a90(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3ad10:	lea    0x107771(%rip),%r8        # 142488 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3f8>
   3ad17:	lea    0x310(%rsp),%rdx
   3ad1f:	mov    $0x2b,%esi
   3ad24:	call   *0x10f5e6(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3ad2a:	mov    0x38(%rcx),%rsi
   3ad2e:	mov    0x40(%rcx),%rdx
   3ad32:	sub    $0x8,%rsp
   3ad36:	lea    0x758(%rsp),%rdi
   3ad3e:	lea    0x690(%rsp),%rcx
   3ad46:	movd   -0x2a2e6(%rip),%xmm0        # 10a68 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x9c>
   3ad4e:	xor    %r8d,%r8d
   3ad51:	push   %r11
   3ad53:	call   f0490 <meshoptimizer_rs::simplify::extra::simplify_prune_kernel>
   3ad58:	add    $0x10,%rsp
   3ad5c:	mov    0x750(%rsp),%rcx
   3ad64:	movzbl 0x758(%rsp),%eax
   3ad6c:	cmp    $0xffffffffffffffff,%rcx
   3ad70:	je     3d53c <meshopt_peers::run+0x35ec>
   3ad76:	mov    0x759(%rsp),%rdx
   3ad7e:	mov    0x760(%rsp),%rsi
   3ad86:	mov    %rsi,0x18(%rbx)
   3ad8a:	mov    %rdx,0x11(%rbx)
   3ad8e:	mov    %rcx,0x8(%rbx)
   3ad92:	mov    %al,0x10(%rbx)
   3ad95:	movl   $0x3,(%rbx)
   3ad9b:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3ada0:	movdqu 0x38(%rcx),%xmm1
   3ada5:	movq   $0x0,0x1c8(%rsp)
   3adb1:	mov    %rax,0x168(%rsp)
   3adb9:	movd   %xmm0,0x230(%rsp)
   3adc2:	movb   $0x0,0x365(%r11)
   3adca:	movl   $0x0,0x361(%r11)
   3add5:	pxor   %xmm0,%xmm0
   3add9:	movdqu %xmm0,0x350(%r11)
   3ade2:	mov    0x348(%r11),%rax
   3ade9:	mov    %rax,0xf0(%rsp)
   3adf1:	mov    %rax,0xf8(%rsp)
   3adf9:	movdqu %xmm1,0x750(%rsp)
   3ae02:	lea    0x688(%rsp),%rax
   3ae0a:	mov    %rax,0x760(%rsp)
   3ae12:	lea    0xf0(%rsp),%rax
   3ae1a:	mov    %rax,0x768(%rsp)
   3ae22:	lea    0x1c8(%rsp),%rax
   3ae2a:	mov    %rax,0x770(%rsp)
   3ae32:	lea    0x168(%rsp),%rax
   3ae3a:	mov    %rax,0x778(%rsp)
   3ae42:	lea    0x230(%rsp),%rax
   3ae4a:	mov    %rax,0x780(%rsp)
   3ae52:	mov    %r11,0x788(%rsp)
   3ae5a:	movq   $0x0,0x790(%rsp)
   3ae66:	lea    0x310(%rsp),%rdi
   3ae6e:	lea    0x750(%rsp),%rsi
   3ae76:	mov    %r11,%r14
   3ae79:	call   8d840 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   3ae7e:	mov    0xf8(%rsp),%rax
   3ae86:	sub    0xf0(%rsp),%rax
   3ae8e:	mov    %rax,0x358(%r14)
   3ae95:	mov    0x310(%rsp),%rax
   3ae9d:	movzbl 0x318(%rsp),%ecx
   3aea5:	cmp    $0xffffffffffffffff,%rax
   3aea9:	je     3aecd <meshopt_peers::run+0xf7d>
   3aeab:	mov    0x328(%rsp),%rdx
   3aeb3:	mov    %rdx,0x278(%rsp)
   3aebb:	movdqu 0x319(%rsp),%xmm0
   3aec4:	movdqu %xmm0,0x269(%rsp)
   3aecd:	mov    %cl,0x268(%rsp)
   3aed4:	mov    %rax,0x260(%rsp)
   3aedc:	cmp    $0xffffffffffffffff,%rax
   3aee0:	jne    3ac13 <meshopt_peers::run+0xcc3>
   3aee6:	movzbl 0x268(%rsp),%eax
   3aeee:	mov    %al,0x750(%rsp)
   3aef5:	lea    -0x29a71(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3aefc:	lea    0x10789d(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3af03:	lea    0x1075f6(%rip),%r8        # 142500 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x470>
   3af0a:	lea    0x750(%rsp),%rdx
   3af12:	mov    $0x2b,%esi
   3af17:	call   *0x10f3f3(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3af1d:	mov    0x38(%rcx),%rsi
   3af21:	mov    0x40(%rcx),%rdx
   3af25:	lea    0x750(%rsp),%rdi
   3af2d:	mov    $0x1,%ecx
   3af32:	mov    %r11,%r8
   3af35:	call   *0x10f6ed(%rip)        # 14a628 <_DYNAMIC+0x5e8>
   3af3b:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3af44:	jne    3b1f4 <meshopt_peers::run+0x12a4>
   3af4a:	movzbl 0x758(%rsp),%eax
   3af52:	mov    %al,0x310(%rsp)
   3af59:	lea    -0x29ad5(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3af60:	lea    0x107839(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3af67:	lea    0x10745a(%rip),%r8        # 1423c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x338>
   3af6e:	lea    0x310(%rsp),%rdx
   3af76:	mov    $0x2b,%esi
   3af7b:	call   *0x10f38f(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3af81:	mov    $0x616d6572,%edx
   3af86:	xor    (%rsi),%edx
   3af88:	movzbl 0x4(%rsi),%esi
   3af8c:	xor    $0x70,%esi
   3af8f:	or     %edx,%esi
   3af91:	je     3b90e <meshopt_peers::run+0x19be>
   3af97:	lea    0x48(%rcx),%rax
   3af9b:	mov    0x38(%rcx),%rbp
   3af9f:	mov    0x40(%rcx),%r15
   3afa3:	mov    %rax,0x168(%rsp)
   3afab:	mov    %rbp,0xf0(%rsp)
   3afb3:	mov    %r15,0xf8(%rsp)
   3afbb:	mov    %r14,0x1c8(%rsp)
   3afc3:	movb   $0x0,0x230(%rsp)
   3afcb:	movb   $0x0,0x365(%r11)
   3afd3:	movl   $0x0,0x361(%r11)
   3afde:	pxor   %xmm0,%xmm0
   3afe2:	movdqu %xmm0,0x350(%r11)
   3afeb:	mov    0x348(%r11),%rax
   3aff2:	mov    %rax,0x260(%rsp)
   3affa:	mov    %rax,0x268(%rsp)
   3b002:	lea    0xf0(%rsp),%rcx
   3b00a:	mov    %rcx,0x750(%rsp)
   3b012:	lea    0x1c8(%rsp),%rcx
   3b01a:	mov    %rcx,0x758(%rsp)
   3b022:	lea    0x260(%rsp),%rcx
   3b02a:	mov    %rcx,0x760(%rsp)
   3b032:	mov    %r11,0x768(%rsp)
   3b03a:	lea    0x230(%rsp),%rcx
   3b042:	mov    %rcx,0x770(%rsp)
   3b04a:	mov    %r13,0x778(%rsp)
   3b052:	mov    %r14,0x780(%rsp)
   3b05a:	mov    %r13,0x788(%rsp)
   3b062:	mov    %r14,0x790(%rsp)
   3b06a:	lea    0x168(%rsp),%rcx
   3b072:	mov    %rcx,0x798(%rsp)
   3b07a:	mov    %rax,%rcx
   3b07d:	sub    %r15,%rcx
   3b080:	jae    3b107 <meshopt_peers::run+0x11b7>
   3b086:	mov    %rbx,(%rsp)
   3b08a:	shl    $0x2,%r15
   3b08e:	xor    %ebx,%ebx
   3b090:	lea    0x260(%rsp),%rdi
   3b098:	mov    0x10f569(%rip),%rcx        # 14a608 <_DYNAMIC+0x5c8>
   3b09f:	nop
   3b0a0:	cmp    %rbx,%r15
   3b0a3:	je     3b54b <meshopt_peers::run+0x15fb>
   3b0a9:	mov    0x0(%rbp,%rbx,1),%r12d
   3b0ae:	test   %rax,%rax
   3b0b1:	je     3b0c9 <meshopt_peers::run+0x1179>
   3b0b3:	dec    %rax
   3b0b6:	mov    %rax,0x260(%rsp)
   3b0be:	add    $0x4,%rbx
   3b0c2:	cmp    %r14d,%r12d
   3b0c5:	jb     3b0a0 <meshopt_peers::run+0x1150>
   3b0c7:	jmp    3b0fc <meshopt_peers::run+0x11ac>
   3b0c9:	mov    %r11,0x8(%rsp)
   3b0ce:	mov    $0x1,%esi
   3b0d3:	call   *%rcx
   3b0d5:	cmp    $0xff,%al
   3b0d7:	jne    3d1ac <meshopt_peers::run+0x325c>
   3b0dd:	xor    %eax,%eax
   3b0df:	mov    0x8(%rsp),%r11
   3b0e4:	lea    0x260(%rsp),%rdi
   3b0ec:	mov    0x10f515(%rip),%rcx        # 14a608 <_DYNAMIC+0x5c8>
   3b0f3:	add    $0x4,%rbx
   3b0f7:	cmp    %r14d,%r12d
   3b0fa:	jb     3b0a0 <meshopt_peers::run+0x1150>
   3b0fc:	mov    $0x1,%al
   3b0fe:	mov    (%rsp),%rbx
   3b102:	jmp    3dbb1 <meshopt_peers::run+0x3c61>
   3b107:	test   %r15,%r15
   3b10a:	je     3dac7 <meshopt_peers::run+0x3b77>
   3b110:	mov    0x0(%rbp),%edi
   3b113:	cmp    $0x1,%r15
   3b117:	je     3dabc <meshopt_peers::run+0x3b6c>
   3b11d:	movabs $0x3fffffffffffffff,%r8
   3b127:	lea    (%r15,%r8,1),%rsi
   3b12b:	mov    %rsi,%rdx
   3b12e:	and    %r8,%rdx
   3b131:	cmp    $0x8,%rdx
   3b135:	jae    3bab5 <meshopt_peers::run+0x1b65>
   3b13b:	xor    %esi,%esi
   3b13d:	jmp    3daa6 <meshopt_peers::run+0x3b56>
   3b142:	mov    0x38(%rcx),%rsi
   3b146:	mov    0x40(%rcx),%rdx
   3b14a:	lea    0x750(%rsp),%rdi
   3b152:	lea    0x688(%rsp),%rcx
   3b15a:	movd   -0x2a656(%rip),%xmm0        # 10b0c <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x140>
   3b162:	mov    %r11,%r8
   3b165:	call   *0x10f4c5(%rip)        # 14a630 <_DYNAMIC+0x5f0>
   3b16b:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b174:	jne    3b4bc <meshopt_peers::run+0x156c>
   3b17a:	movzbl 0x758(%rsp),%eax
   3b182:	mov    %al,0x310(%rsp)
   3b189:	lea    -0x29d05(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b190:	lea    0x107609(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b197:	lea    0x1072d2(%rip),%r8        # 142470 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3e0>
   3b19e:	lea    0x310(%rsp),%rdx
   3b1a6:	mov    $0x2b,%esi
   3b1ab:	call   *0x10f15f(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3b1b1:	lea    0x0(,%r14,4),%rax
   3b1b9:	lea    (%rax,%rax,2),%rdx
   3b1bd:	lea    0x750(%rsp),%rdi
   3b1c5:	mov    $0xc,%r8d
   3b1cb:	mov    %r13,%rsi
   3b1ce:	mov    %r14,%rcx
   3b1d1:	mov    $0x1,%r9d
   3b1d7:	push   %r11
   3b1d9:	push   $0x2
   3b1db:	call   *0x10f457(%rip)        # 14a638 <_DYNAMIC+0x5f8>
   3b1e1:	add    $0x10,%rsp
   3b1e5:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b1ee:	je     3d0b4 <meshopt_peers::run+0x3164>
   3b1f4:	mov    0x760(%rsp),%rax
   3b1fc:	mov    %rax,0x18(%rbx)
   3b200:	movdqu 0x750(%rsp),%xmm0
   3b209:	movdqu %xmm0,0x8(%rbx)
   3b20e:	movl   $0x2,(%rbx)
   3b214:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3b219:	mov    %r14,%rax
   3b21c:	shr    $0x20,%rax
   3b220:	jne    3d5b3 <meshopt_peers::run+0x3663>
   3b226:	mov    0x38(%rcx),%rsi
   3b22a:	mov    0x40(%rcx),%rdx
   3b22e:	lea    0x0(,%r14,4),%rax
   3b236:	lea    (%rax,%rax,2),%rax
   3b23a:	mov    %r13,0x310(%rsp)
   3b242:	mov    %rax,0x318(%rsp)
   3b24a:	mov    %r14,0x320(%rsp)
   3b252:	movq   $0xc,0x328(%rsp)
   3b25e:	movq   $0xc,0x330(%rsp)
   3b26a:	movq   $0x0,0x338(%rsp)
   3b276:	lea    0x750(%rsp),%rdi
   3b27e:	lea    0x310(%rsp),%rcx
   3b286:	mov    %r11,%r8
   3b289:	call   *0x10f3b1(%rip)        # 14a640 <_DYNAMIC+0x600>
   3b28f:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b298:	je     3d122 <meshopt_peers::run+0x31d2>
   3b29e:	mov    0x780(%rsp),%rax
   3b2a6:	mov    %rax,0x340(%rsp)
   3b2ae:	movups 0x750(%rsp),%xmm0
   3b2b6:	movups 0x760(%rsp),%xmm1
   3b2be:	movdqu 0x770(%rsp),%xmm2
   3b2c7:	movdqa %xmm2,0x330(%rsp)
   3b2d0:	movaps %xmm1,0x320(%rsp)
   3b2d8:	movaps %xmm0,0x310(%rsp)
   3b2e0:	mov    0x320(%rsp),%rax
   3b2e8:	mov    %rax,0x18(%rbx)
   3b2ec:	movups %xmm0,0x8(%rbx)
   3b2f0:	movdqu 0x328(%rsp),%xmm0
   3b2f9:	movdqu %xmm0,0x20(%rbx)
   3b2fe:	mov    0x338(%rsp),%rax
   3b306:	mov    %rax,0x30(%rbx)
   3b30a:	movl   $0x5,(%rbx)
   3b310:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3b315:	mov    0x40(%rcx),%rbp
   3b319:	mov    $0x4,%r15d
   3b31f:	mov    $0x4,%r12d
   3b325:	test   %rbp,%rbp
   3b328:	mov    %rbp,(%rsp)
   3b32c:	je     3b372 <meshopt_peers::run+0x1422>
   3b32e:	mov    %r11,0x8(%rsp)
   3b333:	mov    0x38(%rcx),%r15
   3b337:	lea    0x0(,%rbp,4),%rbp
   3b33f:	mov    %rbp,%rdi
   3b342:	call   *0x10eef0(%rip)        # 14a238 <malloc@GLIBC_2.2.5>
   3b348:	test   %rax,%rax
   3b34b:	je     3d5fd <meshopt_peers::run+0x36ad>
   3b351:	mov    %rax,%r12
   3b354:	mov    %rax,%rdi
   3b357:	mov    %r15,%rsi
   3b35a:	mov    %rbp,%rdx
   3b35d:	call   *0x10eef5(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   3b363:	mov    0x8(%rsp),%r11
   3b368:	mov    $0x4,%r15d
   3b36e:	mov    (%rsp),%rbp
   3b372:	test   %r14,%r14
   3b375:	je     3b3bb <meshopt_peers::run+0x146b>
   3b377:	mov    %r12,0x8(%rsp)
   3b37c:	mov    %r11,%r12
   3b37f:	lea    0x0(,%r14,4),%rax
   3b387:	lea    (%rax,%rax,2),%rbp
   3b38b:	mov    %rbp,%rdi
   3b38e:	call   *0x10eea4(%rip)        # 14a238 <malloc@GLIBC_2.2.5>
   3b394:	test   %rax,%rax
   3b397:	je     3d63b <meshopt_peers::run+0x36eb>
   3b39d:	mov    %rax,%r15
   3b3a0:	mov    %rax,%rdi
   3b3a3:	mov    %r13,%rsi
   3b3a6:	mov    %rbp,%rdx
   3b3a9:	call   *0x10eea9(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   3b3af:	mov    %r12,%r11
   3b3b2:	mov    0x8(%rsp),%r12
   3b3b7:	mov    (%rsp),%rbp
   3b3bb:	movb   $0x0,0x750(%rsp)
   3b3c3:	mov    %r15,0x758(%rsp)
   3b3cb:	mov    %r14,0x760(%rsp)
   3b3d3:	mov    %r14,0x768(%rsp)
   3b3db:	movq   $0x3,0x770(%rsp)
   3b3e7:	movq   $0x0,0x778(%rsp)
   3b3f3:	sub    $0x30,%rsp
   3b3f7:	mov    %r11,0x20(%rsp)
   3b3fc:	lea    0x250(%rsp),%rax
   3b404:	mov    %rax,0x18(%rsp)
   3b409:	pxor   %xmm0,%xmm0
   3b40d:	movdqu %xmm0,(%rsp)
   3b412:	lea    0x340(%rsp),%rdi
   3b41a:	lea    0x780(%rsp),%rcx
   3b422:	mov    $0x4,%r9d
   3b428:	mov    %r12,%rsi
   3b42b:	mov    %rbp,%rdx
   3b42e:	xor    %r8d,%r8d
   3b431:	call   *0x10f211(%rip)        # 14a648 <_DYNAMIC+0x608>
   3b437:	add    $0x30,%rsp
   3b43b:	cmpb   $0x1,0x310(%rsp)
   3b443:	je     3d159 <meshopt_peers::run+0x3209>
   3b449:	mov    0x318(%rsp),%rax
   3b451:	movd   0x320(%rsp),%xmm0
   3b45a:	cmp    %rbp,%rax
   3b45d:	cmovae %rbp,%rax
   3b461:	mov    %rbp,0x8(%rbx)
   3b465:	mov    %r12,0x10(%rbx)
   3b469:	mov    %rax,0x18(%rbx)
   3b46d:	mov    %r14,0x20(%rbx)
   3b471:	mov    %r15,0x28(%rbx)
   3b475:	mov    %r14,0x30(%rbx)
   3b479:	movd   %xmm0,0x4(%rbx)
   3b47e:	movl   $0x1,(%rbx)
   3b484:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3b489:	mov    0x38(%rcx),%rsi
   3b48d:	mov    0x40(%rcx),%rdx
   3b491:	mov    0x8(%rcx),%rax
   3b495:	mov    0x10(%rcx),%r8
   3b499:	lea    0x750(%rsp),%rdi
   3b4a1:	mov    %rax,%rcx
   3b4a4:	mov    %r11,%r9
   3b4a7:	call   *0x10f1a3(%rip)        # 14a650 <_DYNAMIC+0x610>
   3b4ad:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b4b6:	je     3d0eb <meshopt_peers::run+0x319b>
   3b4bc:	mov    0x760(%rsp),%rax
   3b4c4:	mov    %rax,0x18(%rbx)
   3b4c8:	movdqu 0x750(%rsp),%xmm0
   3b4d1:	movdqu %xmm0,0x8(%rbx)
   3b4d6:	movl   $0x3,(%rbx)
   3b4dc:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3b4e1:	mov    0x38(%rcx),%rsi
   3b4e5:	mov    0x40(%rcx),%rdx
   3b4e9:	lea    0xf0(%rsp),%rdi
   3b4f1:	lea    0x310(%rsp),%rcx
   3b4f9:	mov    %r11,%r8
   3b4fc:	call   *0x10f156(%rip)        # 14a658 <_DYNAMIC+0x618>
   3b502:	mov    0xf0(%rsp),%rax
   3b50a:	cmp    $0xffffffffffffffff,%rax
   3b50e:	jne    3a40a <meshopt_peers::run+0x4ba>
   3b514:	movzbl 0xf8(%rsp),%eax
   3b51c:	mov    %al,0x750(%rsp)
   3b523:	lea    -0x2a09f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b52a:	lea    0x10726f(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b531:	lea    0x106dd0(%rip),%r8        # 142308 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x278>
   3b538:	lea    0x750(%rsp),%rdx
   3b540:	mov    $0x2b,%esi
   3b545:	call   *0x10edc5(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3b54b:	mov    0x1c8(%rsp),%rax
   3b553:	mov    (%rsp),%rbx
   3b557:	jmp    3dad2 <meshopt_peers::run+0x3b82>
   3b55c:	mov    %r14,%rax
   3b55f:	shr    $0x20,%rax
   3b563:	jne    3d60b <meshopt_peers::run+0x36bb>
   3b569:	lea    0x0(,%r14,4),%rax
   3b571:	lea    (%rax,%rax,2),%rax
   3b575:	mov    %r13,0x750(%rsp)
   3b57d:	mov    %rax,0x758(%rsp)
   3b585:	mov    %r14,0x760(%rsp)
   3b58d:	movq   $0xc,0x768(%rsp)
   3b599:	movq   $0xc,0x770(%rsp)
   3b5a5:	movq   $0x0,0x778(%rsp)
   3b5b1:	mov    0x8(%rcx),%rdx
   3b5b5:	mov    0x10(%rcx),%rcx
   3b5b9:	lea    0x310(%rsp),%rdi
   3b5c1:	lea    0x750(%rsp),%rsi
   3b5c9:	mov    %r11,%r8
   3b5cc:	call   *0x10f08e(%rip)        # 14a660 <_DYNAMIC+0x620>
   3b5d2:	cmpq   $0xffffffffffffffff,0x310(%rsp)
   3b5db:	je     3d1ba <meshopt_peers::run+0x326a>
   3b5e1:	mov    0x320(%rsp),%rax
   3b5e9:	mov    %rax,0x18(%rbx)
   3b5ed:	movdqu 0x310(%rsp),%xmm0
   3b5f6:	jmp    3b209 <meshopt_peers::run+0x12b9>
   3b5fb:	mov    %rax,%r14
   3b5fe:	mov    $0x4,%r12d
   3b604:	movq   $0x0,0x1b0(%rsp)
   3b610:	movq   $0x4,0x1b8(%rsp)
   3b61c:	movq   $0x0,0x1c0(%rsp)
   3b628:	movl   $0x0,0xac(%rsp)
   3b633:	movq   $0x80,0x1c8(%rsp)
   3b63f:	movq   $0x2a,0x1d0(%rsp)
   3b64b:	movq   $0x80,0x1d8(%rsp)
   3b657:	movq   $0x10,0x1e0(%rsp)
   3b663:	movaps -0x2c8ca(%rip),%xmm0        # eda0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x690>
   3b66a:	movups %xmm0,0x1e8(%rsp)
   3b672:	movaps -0x2c819(%rip),%xmm0        # ee60 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3b679:	movups %xmm0,0x1f8(%rsp)
   3b681:	movabs $0x100010100000101,%rax
   3b68b:	mov    %rax,0x208(%rsp)
   3b693:	movl   $0x0,0x210(%rsp)
   3b69e:	movb   $0x1,0x214(%rsp)
   3b6a6:	mov    0x38(%rcx),%r13
   3b6aa:	mov    0x40(%rcx),%r15
   3b6ae:	mov    %r13,0x2a8(%rsp)
   3b6b6:	mov    %r15,0x2b0(%rsp)
   3b6be:	mov    %r12,0x8(%rsp)
   3b6c3:	mov    %r12,0x2b8(%rsp)
   3b6cb:	mov    %r14,0x2c0(%rsp)
   3b6d3:	movb   $0x2,0x260(%rsp)
   3b6db:	movq   $0x4,0x2c8(%rsp)
   3b6e7:	xorps  %xmm0,%xmm0
   3b6ea:	movups %xmm0,0x2d0(%rsp)
   3b6f2:	movl   $0x0,0x2e8(%rsp)
   3b6fd:	movb   $0x0,0x365(%r11)
   3b705:	movl   $0x0,0x361(%r11)
   3b710:	movups %xmm0,0x350(%r11)
   3b718:	mov    0x348(%r11),%rbx
   3b71f:	mov    %r11,%r12
   3b722:	mov    %r11,%rdi
   3b725:	xor    %esi,%esi
   3b727:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b72c:	mov    %eax,%ebp
   3b72e:	cmp    $0xff,%al
   3b730:	jne    3d71f <meshopt_peers::run+0x37cf>
   3b736:	mov    %r12,0x750(%rsp)
   3b73e:	mov    %rbx,0x758(%rsp)
   3b746:	mov    %rbx,0x760(%rsp)
   3b74e:	movq   $0x0,0x768(%rsp)
   3b75a:	movl   $0x0,0x770(%rsp)
   3b765:	movl   $0x0,0x974(%rsp)
   3b770:	movb   $0x1,0x1978(%rsp)
   3b778:	lea    0x260(%rsp),%rdi
   3b780:	lea    0x1c8(%rsp),%rsi
   3b788:	lea    0x750(%rsp),%rdx
   3b790:	call   *0x10eed2(%rip)        # 14a668 <_DYNAMIC+0x628>
   3b796:	mov    (%rsp),%rbx
   3b79a:	mov    %eax,%ebp
   3b79c:	cmp    $0xff,%al
   3b79e:	jne    3d700 <meshopt_peers::run+0x37b0>
   3b7a4:	movzbl 0x1978(%rsp),%r12d
   3b7ad:	movb   $0x2,0x310(%rsp)
   3b7b5:	mov    0x8(%rsp),%rax
   3b7ba:	mov    %rax,0x318(%rsp)
   3b7c2:	mov    %r14,0x320(%rsp)
   3b7ca:	movq   $0x0,0x348(%rsp)
   3b7d6:	lea    0xf0(%rsp),%rdi
   3b7de:	lea    0x310(%rsp),%rsi
   3b7e6:	lea    0x750(%rsp),%rdx
   3b7ee:	call   *0x10ee7c(%rip)        # 14a670 <_DYNAMIC+0x630>
   3b7f4:	mov    0xf0(%rsp),%rax
   3b7fc:	movzbl 0xf8(%rsp),%ebp
   3b804:	mov    %rax,0x88(%rsp)
   3b80c:	cmp    $0xffffffffffffffff,%rax
   3b810:	je     3b8a2 <meshopt_peers::run+0x1952>
   3b816:	movzbl %bpl,%eax
   3b81a:	movzbl 0xff(%rsp),%ecx
   3b822:	shl    $0x10,%ecx
   3b825:	movzwl 0xfd(%rsp),%edx
   3b82d:	or     %ecx,%edx
   3b82f:	mov    0xf9(%rsp),%ecx
   3b836:	shl    $0x28,%rdx
   3b83a:	shl    $0x8,%rcx
   3b83e:	or     %rdx,%rcx
   3b841:	or     %rax,%rcx
   3b844:	mov    %rcx,0xe8(%rsp)
   3b84c:	mov    0x768(%rsp),%rsi
   3b854:	add    %r14,%rsi
   3b857:	jb     3d07b <meshopt_peers::run+0x312b>
   3b85d:	mov    0x100(%rsp),%rax
   3b865:	mov    %rax,0x40(%rsp)
   3b86a:	mov    %rsi,0x768(%rsp)
   3b872:	mov    0x750(%rsp),%rdi
   3b87a:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b87f:	mov    %eax,%ebp
   3b881:	cmp    $0xff,%al
   3b883:	je     3ba54 <meshopt_peers::run+0x1b04>
   3b889:	cmpq   $0x0,0x88(%rsp)
   3b892:	je     3b8a2 <meshopt_peers::run+0x1952>
   3b894:	mov    0xe8(%rsp),%rdi
   3b89c:	call   *0x10e9a6(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3b8a2:	mov    0x750(%rsp),%rax
   3b8aa:	mov    0x760(%rsp),%rcx
   3b8b2:	sub    0x758(%rsp),%rcx
   3b8ba:	mov    %rcx,0x358(%rax)
   3b8c1:	cmp    $0xff,%bpl
   3b8c5:	jne    3d71f <meshopt_peers::run+0x37cf>
   3b8cb:	mov    0x1c0(%rsp),%rax
   3b8d3:	mov    %rax,0x18(%rbx)
   3b8d7:	movdqu 0x1b0(%rsp),%xmm0
   3b8e0:	movdqu %xmm0,0x8(%rbx)
   3b8e5:	movl   $0x3,(%rbx)
   3b8eb:	test   %r14,%r14
   3b8ee:	je     3dc16 <meshopt_peers::run+0x3cc6>
   3b8f4:	mov    0x8(%rsp),%rdi
   3b8f9:	call   *0x10e949(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3b8ff:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3b904:	mov    $0x4,%eax
   3b909:	jmp    3b9f3 <meshopt_peers::run+0x1aa3>
   3b90e:	mov    0x38(%rcx),%rsi
   3b912:	mov    0x40(%rcx),%rdx
   3b916:	mov    %r13,0x750(%rsp)
   3b91e:	mov    %rax,0x758(%rsp)
   3b926:	mov    %r14,0x760(%rsp)
   3b92e:	movq   $0xc,0x768(%rsp)
   3b93a:	movq   $0xc,0x770(%rsp)
   3b946:	movq   $0x0,0x778(%rsp)
   3b952:	lea    0x310(%rsp),%rdi
   3b95a:	lea    0x750(%rsp),%rcx
   3b962:	mov    $0x1,%r8d
   3b968:	mov    %r11,%r9
   3b96b:	call   *0x10ed07(%rip)        # 14a678 <_DYNAMIC+0x638>
   3b971:	jmp    3dbdb <meshopt_peers::run+0x3c8b>
   3b976:	mov    %r12,%rsi
   3b979:	and    $0xfffffffffffffffc,%rsi
   3b97d:	mov    $0x30,%edi
   3b982:	xor    %edx,%edx
   3b984:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   3b990:	movups -0x30(%r14,%rdi,1),%xmm0
   3b996:	movups %xmm0,-0x30(%rax,%rdi,1)
   3b99b:	movups -0x20(%r14,%rdi,1),%xmm0
   3b9a1:	movups %xmm0,-0x20(%rax,%rdi,1)
   3b9a6:	movups -0x10(%r14,%rdi,1),%xmm0
   3b9ac:	movups %xmm0,-0x10(%rax,%rdi,1)
   3b9b1:	movdqu (%r14,%rdi,1),%xmm0
   3b9b7:	movdqu %xmm0,(%rax,%rdi,1)
   3b9bc:	add    $0x4,%rdx
   3b9c0:	add    $0x40,%rdi
   3b9c4:	cmp    %rdx,%rsi
   3b9c7:	jne    3b990 <meshopt_peers::run+0x1a40>
   3b9c9:	test   %rcx,%rcx
   3b9cc:	je     3b9f3 <meshopt_peers::run+0x1aa3>
   3b9ce:	shl    $0x4,%rdx
   3b9d2:	mov    %rax,%rsi
   3b9d5:	add    %rdx,%rsi
   3b9d8:	add    %r14,%rdx
   3b9db:	shl    $0x4,%ecx
   3b9de:	xor    %edi,%edi
   3b9e0:	movdqu (%rdx,%rdi,1),%xmm0
   3b9e5:	movdqu %xmm0,(%rsi,%rdi,1)
   3b9ea:	add    $0x10,%rdi
   3b9ee:	cmp    %rdi,%rcx
   3b9f1:	jne    3b9e0 <meshopt_peers::run+0x1a90>
   3b9f3:	mov    0x338(%rsp),%rcx
   3b9fb:	mov    %rcx,0x30(%rbx)
   3b9ff:	movups 0x328(%rsp),%xmm0
   3ba07:	movups %xmm0,0x20(%rbx)
   3ba0b:	movdqa 0x340(%rsp),%xmm0
   3ba14:	movdqu %xmm0,0x38(%rbx)
   3ba19:	mov    0x350(%rsp),%rcx
   3ba21:	mov    %rcx,0x48(%rbx)
   3ba25:	mov    %r12,0x8(%rbx)
   3ba29:	mov    %rax,0x10(%rbx)
   3ba2d:	mov    %r12,0x18(%rbx)
   3ba31:	movl   $0x6,(%rbx)
   3ba37:	cmpq   $0x0,0x310(%rsp)
   3ba40:	je     3dc16 <meshopt_peers::run+0x3cc6>
   3ba46:	mov    %r14,%rdi
   3ba49:	call   *0x10e7f9(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3ba4f:	jmp    3dc16 <meshopt_peers::run+0x3cc6>
   3ba54:	mov    $0x1,%eax
   3ba59:	mov    %rax,0xa0(%rsp)
   3ba61:	test   %r14,%r14
   3ba64:	jne    3c00d <meshopt_peers::run+0x20bd>
   3ba6a:	mov    0x750(%rsp),%rdi
   3ba72:	mov    0x768(%rsp),%rsi
   3ba7a:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3ba7f:	mov    %eax,%ebp
   3ba81:	mov    $0x1,%al
   3ba83:	cmp    $0xff,%bpl
   3ba87:	je     3bb97 <meshopt_peers::run+0x1c47>
   3ba8d:	test   %r14,%r14
   3ba90:	je     3d24e <meshopt_peers::run+0x32fe>
   3ba96:	mov    0xa0(%rsp),%rdi
   3ba9e:	mov    %rax,%r15
   3baa1:	call   *0x10e7a1(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3baa7:	test   %r15b,%r15b
   3baaa:	jne    3b889 <meshopt_peers::run+0x1939>
   3bab0:	jmp    3b8a2 <meshopt_peers::run+0x1952>
   3bab5:	add    $0xfffffffffffffff9,%r8
   3bab9:	and    %r8,%rsi
   3babc:	movd   %edi,%xmm0
   3bac0:	pshufd $0x0,%xmm0,%xmm1
   3bac5:	xor    %edi,%edi
   3bac7:	movdqa -0x2cc9f(%rip),%xmm0        # ee30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x720>
   3bacf:	movdqa %xmm1,%xmm2
   3bad3:	movdqu 0x4(%rbp,%rdi,4),%xmm3
   3bad9:	movdqu 0x14(%rbp,%rdi,4),%xmm4
   3badf:	movdqa %xmm1,%xmm5
   3bae3:	pxor   %xmm0,%xmm1
   3bae7:	movdqa %xmm3,%xmm6
   3baeb:	pxor   %xmm0,%xmm6
   3baef:	pcmpgtd %xmm6,%xmm1
   3baf3:	pand   %xmm1,%xmm5
   3baf7:	pandn  %xmm3,%xmm1
   3bafb:	por    %xmm5,%xmm1
   3baff:	movdqa %xmm2,%xmm3
   3bb03:	pxor   %xmm0,%xmm2
   3bb07:	movdqa %xmm4,%xmm5
   3bb0b:	pxor   %xmm0,%xmm5
   3bb0f:	pcmpgtd %xmm5,%xmm2
   3bb13:	pand   %xmm2,%xmm3
   3bb17:	pandn  %xmm4,%xmm2
   3bb1b:	por    %xmm3,%xmm2
   3bb1f:	add    $0x8,%rdi
   3bb23:	cmp    %rdi,%rsi
   3bb26:	jne    3bad3 <meshopt_peers::run+0x1b83>
   3bb28:	movdqa %xmm2,%xmm3
   3bb2c:	pxor   %xmm0,%xmm3
   3bb30:	movdqa %xmm1,%xmm4
   3bb34:	pxor   %xmm0,%xmm4
   3bb38:	pcmpgtd %xmm3,%xmm4
   3bb3c:	pand   %xmm4,%xmm1
   3bb40:	pandn  %xmm2,%xmm4
   3bb44:	por    %xmm1,%xmm4
   3bb48:	pshufd $0xee,%xmm4,%xmm1
   3bb4d:	movdqa %xmm4,%xmm2
   3bb51:	pxor   %xmm0,%xmm2
   3bb55:	movdqa %xmm1,%xmm3
   3bb59:	pxor   %xmm0,%xmm3
   3bb5d:	pcmpgtd %xmm3,%xmm2
   3bb61:	pand   %xmm2,%xmm4
   3bb65:	pandn  %xmm1,%xmm2
   3bb69:	por    %xmm4,%xmm2
   3bb6d:	pshufd $0x55,%xmm2,%xmm1
   3bb72:	movdqa %xmm2,%xmm3
   3bb76:	pxor   %xmm0,%xmm3
   3bb7a:	pxor   %xmm1,%xmm0
   3bb7e:	pcmpgtd %xmm0,%xmm3
   3bb82:	pand   %xmm3,%xmm2
   3bb86:	pandn  %xmm1,%xmm3
   3bb8a:	por    %xmm2,%xmm3
   3bb8e:	movd   %xmm3,%edi
   3bb92:	jmp    3dab7 <meshopt_peers::run+0x3b67>
   3bb97:	movb   $0x2,0xf0(%rsp)
   3bb9f:	mov    0x8(%rsp),%rax
   3bba4:	mov    %rax,0xf8(%rsp)
   3bbac:	mov    %r14,0x100(%rsp)
   3bbb4:	movq   $0x0,0x128(%rsp)
   3bbc0:	movups 0x208(%rsp),%xmm0
   3bbc8:	movaps %xmm0,0x350(%rsp)
   3bbd0:	movups 0x1c8(%rsp),%xmm0
   3bbd8:	movups 0x1d8(%rsp),%xmm1
   3bbe0:	movdqu 0x1e8(%rsp),%xmm2
   3bbe9:	movdqu 0x1f8(%rsp),%xmm3
   3bbf2:	movdqa %xmm3,0x340(%rsp)
   3bbfb:	movdqa %xmm2,0x330(%rsp)
   3bc04:	movaps %xmm1,0x320(%rsp)
   3bc0c:	movaps %xmm0,0x310(%rsp)
   3bc14:	movzbl %r12b,%r10d
   3bc18:	lea    0x168(%rsp),%rdi
   3bc20:	lea    0xf0(%rsp),%rcx
   3bc28:	lea    0x310(%rsp),%r8
   3bc30:	mov    %r13,%rsi
   3bc33:	mov    %r15,%rdx
   3bc36:	mov    $0xffffffff,%r9d
   3bc3c:	lea    0x750(%rsp),%rax
   3bc44:	push   %rax
   3bc45:	mov    %r10,0x220(%rsp)
   3bc4d:	push   %r10
   3bc4f:	call   *0x10ea2b(%rip)        # 14a680 <_DYNAMIC+0x640>
   3bc55:	add    $0x10,%rsp
   3bc59:	mov    0x168(%rsp),%rdi
   3bc61:	movzbl 0x170(%rsp),%ebp
   3bc69:	cmp    $0xffffffffffffffff,%rdi
   3bc6d:	je     3bfbc <meshopt_peers::run+0x206c>
   3bc73:	movzbl %bpl,%eax
   3bc77:	movzbl 0x177(%rsp),%ecx
   3bc7f:	shl    $0x10,%ecx
   3bc82:	movzwl 0x175(%rsp),%edx
   3bc8a:	or     %ecx,%edx
   3bc8c:	mov    0x171(%rsp),%ecx
   3bc93:	mov    0x178(%rsp),%r9
   3bc9b:	mov    0x180(%rsp),%r8
   3bca3:	mov    0x188(%rsp),%rsi
   3bcab:	mov    %rsi,0x20(%rsp)
   3bcb0:	shl    $0x28,%rdx
   3bcb4:	shl    $0x8,%rcx
   3bcb8:	or     %rdx,%rcx
   3bcbb:	or     %rax,%rcx
   3bcbe:	mov    %rcx,0x30(%rsp)
   3bcc3:	test   %r9,%r9
   3bcc6:	je     3bfc3 <meshopt_peers::run+0x2073>
   3bccc:	mov    %r8,0x78(%rsp)
   3bcd1:	mov    %rdi,0x80(%rsp)
   3bcd9:	mov    0x190(%rsp),%rax
   3bce1:	mov    %rax,0x10(%rsp)
   3bce6:	mov    %r9,0x70(%rsp)
   3bceb:	mov    %r9,%rax
   3bcee:	shl    $0x4,%rax
   3bcf2:	lea    (%rax,%rax,2),%rax
   3bcf6:	mov    %rax,0x18(%rsp)
   3bcfb:	xor    %r12d,%r12d
   3bcfe:	mov    0x750(%rsp),%r15
   3bd06:	mov    0x758(%rsp),%rbp
   3bd0e:	mov    0x760(%rsp),%rax
   3bd16:	sub    %rbp,%rax
   3bd19:	mov    %rax,0x358(%r15)
   3bd20:	mov    %r15,%rdi
   3bd23:	call   *0x10e8cf(%rip)        # 14a5f8 <_DYNAMIC+0x5b8>
   3bd29:	mov    0x340(%r15),%rax
   3bd30:	mov    0x768(%rsp),%r13
   3bd38:	sub    %r13,%rax
   3bd3b:	jb     3c04c <meshopt_peers::run+0x20fc>
   3bd41:	mov    0x348(%r15),%rcx
   3bd48:	sub    0x358(%r15),%rcx
   3bd4f:	mov    %rax,0x650(%rsp)
   3bd57:	mov    %rcx,0x658(%rsp)
   3bd5f:	movb   $0x0,0x670(%rsp)
   3bd67:	movb   $0x0,0x675(%rsp)
   3bd6f:	movl   $0x0,0x671(%rsp)
   3bd7a:	movb   $0xff,0x678(%rsp)
   3bd82:	movw   $0x0,0x676(%rsp)
   3bd8c:	lea    0x660(%rsp),%rax
   3bd94:	xorps  %xmm0,%xmm0
   3bd97:	movups %xmm0,(%rax)
   3bd9a:	movq   $0xffffffffffffffff,0x388(%rsp)
   3bda6:	movq   $0xffffffffffffffff,0x590(%rsp)
   3bdb2:	movq   $0x0,0x310(%rsp)
   3bdbe:	movq   $0x4,0x318(%rsp)
   3bdca:	movups %xmm0,-0x340(%rax)
   3bdd1:	movq   $0x4,0x330(%rsp)
   3bddd:	movups %xmm0,-0x328(%rax)
   3bde4:	movq   $0x4,0x348(%rsp)
   3bdf0:	movups %xmm0,-0x310(%rax)
   3bdf7:	movq   $0x1,0x360(%rsp)
   3be03:	movups %xmm0,-0x2f8(%rax)
   3be0a:	movq   $0x2,0x378(%rsp)
   3be16:	movq   $0x0,0x380(%rsp)
   3be22:	mov    0x30(%rsp),%rax
   3be27:	mov    0x8(%rax,%r12,1),%rdi
   3be2c:	mov    0x10(%rax,%r12,1),%rdx
   3be31:	mov    %rdx,%rsi
   3be34:	add    %rdi,%rsi
   3be37:	jb     3d195 <meshopt_peers::run+0x3245>
   3be3d:	cmp    0x10(%rsp),%rsi
   3be42:	ja     3d195 <meshopt_peers::run+0x3245>
   3be48:	mov    0x20(%rsp),%rax
   3be4d:	lea    (%rax,%rdi,4),%rsi
   3be51:	movb   $0x2,0xf0(%rsp)
   3be59:	mov    0x8(%rsp),%rax
   3be5e:	mov    %rax,0xf8(%rsp)
   3be66:	mov    %r14,0x100(%rsp)
   3be6e:	movq   $0x0,0x128(%rsp)
   3be7a:	lea    0x168(%rsp),%rdi
   3be82:	lea    0xf0(%rsp),%rcx
   3be8a:	mov    0x218(%rsp),%r8
   3be92:	lea    0x310(%rsp),%r9
   3be9a:	call   *0x10e7e8(%rip)        # 14a688 <_DYNAMIC+0x648>
   3bea0:	mov    0x668(%rsp),%rsi
   3bea8:	sub    %rsi,%rbp
   3beab:	jb     3bf9f <meshopt_peers::run+0x204f>
   3beb1:	mov    %rbp,0x758(%rsp)
   3beb9:	add    0x660(%rsp),%r13
   3bec1:	jb     3d1f1 <meshopt_peers::run+0x32a1>
   3bec7:	mov    %r15,%rdi
   3beca:	mov    %r13,%rsi
   3becd:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3bed2:	mov    %eax,%ebp
   3bed4:	cmp    $0xff,%al
   3bed6:	jne    3d1f4 <meshopt_peers::run+0x32a4>
   3bedc:	movzbl 0x168(%rsp),%ebx
   3bee4:	movzbl 0x169(%rsp),%ebp
   3beec:	lea    0x16a(%rsp),%rcx
   3bef4:	mov    (%rcx),%rax
   3bef7:	mov    0x6(%rcx),%rcx
   3befb:	mov    %rax,0x230(%rsp)
   3bf03:	mov    %rcx,0x236(%rsp)
   3bf0b:	movss  0x178(%rsp),%xmm0
   3bf14:	movaps %xmm0,0x90(%rsp)
   3bf1c:	lea    0x310(%rsp),%rdi
   3bf24:	call   36b90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3bf29:	cmp    $0x1,%bl
   3bf2c:	mov    (%rsp),%rbx
   3bf30:	je     3d201 <meshopt_peers::run+0x32b1>
   3bf36:	lea    0x232(%rsp),%rcx
   3bf3e:	mov    0x8(%rcx),%eax
   3bf41:	lea    0x744(%rsp),%rdx
   3bf49:	mov    %eax,0x8(%rdx)
   3bf4c:	mov    (%rcx),%rax
   3bf4f:	mov    %rax,(%rdx)
   3bf52:	mov    0x8(%rdx),%eax
   3bf55:	mov    %eax,0x318(%rsp)
   3bf5c:	mov    (%rdx),%rcx
   3bf5f:	mov    %rcx,0x310(%rsp)
   3bf67:	mov    0x30(%rsp),%rdx
   3bf6c:	mov    %eax,0x24(%rdx,%r12,1)
   3bf71:	mov    %rcx,0x1c(%rdx,%r12,1)
   3bf76:	xorps  %xmm0,%xmm0
   3bf79:	movaps 0x90(%rsp),%xmm1
   3bf81:	movss  %xmm1,%xmm0
   3bf85:	movlps %xmm0,0x28(%rdx,%r12,1)
   3bf8b:	add    $0x30,%r12
   3bf8f:	cmp    %r12,0x18(%rsp)
   3bf94:	jne    3bcfe <meshopt_peers::run+0x1dae>
   3bf9a:	jmp    3c0a2 <meshopt_peers::run+0x2152>
   3bf9f:	lea    0x758(%rsp),%rdi
   3bfa7:	call   *0x10e65b(%rip)        # 14a608 <_DYNAMIC+0x5c8>
   3bfad:	mov    %eax,%ebp
   3bfaf:	cmp    $0xff,%al
   3bfb1:	je     3beb9 <meshopt_peers::run+0x1f69>
   3bfb7:	jmp    3d1f4 <meshopt_peers::run+0x32a4>
   3bfbc:	mov    $0x1,%al
   3bfbe:	jmp    3ba8d <meshopt_peers::run+0x1b3d>
   3bfc3:	mov    %rdi,%r12
   3bfc6:	mov    %r8,0x90(%rsp)
   3bfce:	lea    0x750(%rsp),%rdi
   3bfd6:	mov    0x88(%rsp),%rsi
   3bfde:	mov    0xe8(%rsp),%rdx
   3bfe6:	call   35f30 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3bfeb:	mov    $0x1,%r13b
   3bfee:	cmp    $0xff,%al
   3bff0:	je     3c054 <meshopt_peers::run+0x2104>
   3bff2:	mov    %eax,%ebp
   3bff4:	movq   $0x0,0x40(%rsp)
   3bffd:	mov    $0x1,%r15b
   3c000:	mov    0x90(%rsp),%rcx
   3c008:	jmp    3d21b <meshopt_peers::run+0x32cb>
   3c00d:	lea    0x310(%rsp),%rdi
   3c015:	mov    $0x1,%edx
   3c01a:	mov    $0x1,%r8d
   3c020:	mov    $0x1,%r9d
   3c026:	xor    %esi,%esi
   3c028:	mov    %r14,%rbp
   3c02b:	mov    %r14,%rcx
   3c02e:	call   421f0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3c033:	cmpl   $0x1,0x310(%rsp)
   3c03b:	jne    3c15a <meshopt_peers::run+0x220a>
   3c041:	mov    %rbp,%r14
   3c044:	mov    $0xa,%bpl
   3c047:	jmp    3b889 <meshopt_peers::run+0x1939>
   3c04c:	mov    $0x9,%bpl
   3c04f:	jmp    3d201 <meshopt_peers::run+0x32b1>
   3c054:	mov    %r12,0x18(%rsp)
   3c059:	mov    0x768(%rsp),%rax
   3c061:	mov    %r14,%r12
   3c064:	sub    %r14,%rax
   3c067:	jae    3c17d <meshopt_peers::run+0x222d>
   3c06d:	mov    0xa0(%rsp),%rdi
   3c075:	call   *0x10e1cd(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3c07b:	mov    $0x8,%bpl
   3c07e:	movq   $0x0,0x40(%rsp)
   3c087:	xor    %r15d,%r15d
   3c08a:	mov    0x18(%rsp),%rax
   3c08f:	mov    0x90(%rsp),%rcx
   3c097:	mov    %r12,%r14
   3c09a:	mov    %rax,%r12
   3c09d:	jmp    3c1d5 <meshopt_peers::run+0x2285>
   3c0a2:	mov    0x70(%rsp),%rdx
   3c0a7:	xor    %r13d,%r13d
   3c0aa:	cmp    $0x1,%rdx
   3c0ae:	jne    3c1dd <meshopt_peers::run+0x228d>
   3c0b4:	mov    0x80(%rsp),%r12
   3c0bc:	mov    0x78(%rsp),%rax
   3c0c1:	mov    %rax,0x90(%rsp)
   3c0c9:	mov    0x30(%rsp),%rdi
   3c0ce:	movups 0x1c(%rdi),%xmm0
   3c0d2:	movaps %xmm0,0x700(%rsp)
   3c0da:	movl   $0x7f7fffff,0x710(%rsp)
   3c0e5:	lea    0xac(%rsp),%rax
   3c0ed:	lea    0x1b0(%rsp),%r10
   3c0f5:	lea    0x700(%rsp),%r11
   3c0fd:	mov    $0x1,%esi
   3c102:	mov    0x20(%rsp),%rdx
   3c107:	mov    0x10(%rsp),%rcx
   3c10c:	mov    0x8(%rsp),%r8
   3c111:	mov    %r14,%r9
   3c114:	lea    0x750(%rsp),%r15
   3c11c:	push   %r15
   3c11e:	push   %rax
   3c11f:	push   %r10
   3c121:	push   %r13
   3c123:	push   %r11
   3c125:	push   $0x0
   3c127:	call   36c20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3c12c:	add    $0x30,%rsp
   3c130:	mov    %rax,%rbp
   3c133:	test   $0x1,%bpl
   3c137:	je     3bfce <meshopt_peers::run+0x207e>
   3c13d:	shr    $0x8,%ebp
   3c140:	mov    $0x1,%al
   3c142:	mov    %rax,0x40(%rsp)
   3c147:	mov    $0x1,%r15b
   3c14a:	mov    $0x1,%r13b
   3c14d:	mov    0x90(%rsp),%rcx
   3c155:	jmp    3d21b <meshopt_peers::run+0x32cb>
   3c15a:	mov    0x318(%rsp),%rdi
   3c162:	mov    %rdi,0xa0(%rsp)
   3c16a:	xor    %esi,%esi
   3c16c:	mov    %rbp,%r14
   3c16f:	mov    %rbp,%rdx
   3c172:	call   *0x10e0f0(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   3c178:	jmp    3ba6a <meshopt_peers::run+0x1b1a>
   3c17d:	mov    %rax,0x768(%rsp)
   3c185:	mov    %r12,%r14
   3c188:	test   %r12,%r12
   3c18b:	je     3c19b <meshopt_peers::run+0x224b>
   3c18d:	mov    0xa0(%rsp),%rdi
   3c195:	call   *0x10e0ad(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3c19b:	movq   $0x0,0x40(%rsp)
   3c1a4:	lea    0x750(%rsp),%rdi
   3c1ac:	mov    $0x4,%edx
   3c1b1:	xor    %esi,%esi
   3c1b3:	call   35e90 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   3c1b8:	cmp    $0xff,%al
   3c1ba:	mov    0x18(%rsp),%rsi
   3c1bf:	je     3cf7a <meshopt_peers::run+0x302a>
   3c1c5:	mov    %eax,%ebp
   3c1c7:	xor    %r15d,%r15d
   3c1ca:	mov    %rsi,%r12
   3c1cd:	mov    0x90(%rsp),%rcx
   3c1d5:	mov    $0x1,%r13b
   3c1d8:	jmp    3d21b <meshopt_peers::run+0x32cb>
   3c1dd:	lea    0x168(%rsp),%rdi
   3c1e5:	mov    0x10e4a4(%rip),%r10        # 14a690 <_DYNAMIC+0x650>
   3c1ec:	mov    0x10(%rsp),%r8
   3c1f1:	movb   $0x2,0xf0(%rsp)
   3c1f9:	mov    0x8(%rsp),%rax
   3c1fe:	mov    %rax,0xf8(%rsp)
   3c206:	mov    %r14,0x100(%rsp)
   3c20e:	movq   $0x0,0x128(%rsp)
   3c21a:	movups 0x208(%rsp),%xmm0
   3c222:	movaps %xmm0,0x350(%rsp)
   3c22a:	movups 0x1c8(%rsp),%xmm0
   3c232:	movups 0x1d8(%rsp),%xmm1
   3c23a:	movdqu 0x1e8(%rsp),%xmm2
   3c243:	movdqu 0x1f8(%rsp),%xmm3
   3c24c:	movdqa %xmm3,0x340(%rsp)
   3c255:	movdqa %xmm2,0x330(%rsp)
   3c25e:	movaps %xmm1,0x320(%rsp)
   3c266:	movaps %xmm0,0x310(%rsp)
   3c26e:	mov    0x30(%rsp),%rsi
   3c273:	mov    %rdx,0x70(%rsp)
   3c278:	mov    0x20(%rsp),%rcx
   3c27d:	mov    %r8,0x10(%rsp)
   3c282:	mov    0xe8(%rsp),%r9
   3c28a:	lea    0x750(%rsp),%rax
   3c292:	push   %rax
   3c293:	lea    0x318(%rsp),%rax
   3c29b:	push   %rax
   3c29c:	lea    0x100(%rsp),%rax
   3c2a4:	push   %rax
   3c2a5:	push   0x58(%rsp)
   3c2a9:	call   *%r10
   3c2ac:	add    $0x20,%rsp
   3c2b0:	mov    %r13,0x160(%rsp)
   3c2b8:	mov    0x168(%rsp),%r15
   3c2c0:	movzbl 0x170(%rsp),%ebp
   3c2c8:	cmp    $0xffffffffffffffff,%r15
   3c2cc:	je     3d201 <meshopt_peers::run+0x32b1>
   3c2d2:	movzbl %bpl,%eax
   3c2d6:	movzbl 0x177(%rsp),%ecx
   3c2de:	shl    $0x10,%ecx
   3c2e1:	movzwl 0x175(%rsp),%edx
   3c2e9:	or     %ecx,%edx
   3c2eb:	mov    0x171(%rsp),%r8d
   3c2f3:	mov    0x178(%rsp),%r9
   3c2fb:	shl    $0x28,%rdx
   3c2ff:	shl    $0x8,%r8
   3c303:	or     %rdx,%r8
   3c306:	or     %rax,%r8
   3c309:	sub    $0x40,%rsp
   3c30d:	lea    0x790(%rsp),%rax
   3c315:	mov    %rax,0x30(%rsp)
   3c31a:	mov    0x80(%rsp),%rax
   3c322:	mov    %rax,0x18(%rsp)
   3c327:	mov    0x128(%rsp),%rax
   3c32f:	mov    %rax,0x10(%rsp)
   3c334:	mov    0x50(%rsp),%rax
   3c339:	mov    %rax,0x8(%rsp)
   3c33e:	mov    0x60(%rsp),%rax
   3c343:	mov    %rax,(%rsp)
   3c347:	movq   $0x0,0x20(%rsp)
   3c350:	mov    0xe0(%rsp),%rdi
   3c358:	mov    %r14,%rsi
   3c35b:	mov    0x70(%rsp),%rdx
   3c360:	mov    0xb0(%rsp),%rcx
   3c368:	mov    %r8,0x190(%rsp)
   3c370:	mov    %r9,0x120(%rsp)
   3c378:	call   *0x10e31a(%rip)        # 14a698 <_DYNAMIC+0x658>
   3c37e:	add    $0x40,%rsp
   3c382:	mov    %eax,%ebp
   3c384:	mov    $0x1,%r12b
   3c387:	cmp    $0xff,%al
   3c389:	jne    3cfd6 <meshopt_peers::run+0x3086>
   3c38f:	mov    0x10(%rsp),%rax
   3c394:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   3c39e:	mul    %rcx
   3c3a1:	mov    %rdx,%rax
   3c3a4:	shr    $1,%rax
   3c3a7:	mov    %rax,0x48(%rsp)
   3c3ac:	mov    $0x30,%ecx
   3c3b1:	mul    %rcx
   3c3b4:	seto   %cl
   3c3b7:	test   %rax,%rax
   3c3ba:	sets   %dl
   3c3bd:	or     %cl,%dl
   3c3bf:	mov    $0x8,%bpl
   3c3c2:	jne    3cfd6 <meshopt_peers::run+0x3086>
   3c3c8:	add    0x768(%rsp),%rax
   3c3d0:	jb     3cfd6 <meshopt_peers::run+0x3086>
   3c3d6:	mov    %rax,0x768(%rsp)
   3c3de:	mov    0x750(%rsp),%r13
   3c3e6:	mov    %r13,%rdi
   3c3e9:	mov    %rax,%r12
   3c3ec:	mov    %rax,%rsi
   3c3ef:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c3f4:	mov    %eax,%ebp
   3c3f6:	mov    %r15,0x38(%rsp)
   3c3fb:	cmp    $0xff,%al
   3c3fd:	jne    3cfca <meshopt_peers::run+0x307a>
   3c403:	cmpq   $0x3,0x10(%rsp)
   3c409:	jae    3cebc <meshopt_peers::run+0x2f6c>
   3c40f:	mov    $0x8,%r15d
   3c415:	xor    %eax,%eax
   3c417:	mov    %rax,0x18(%rsp)
   3c41c:	sub    0x48(%rsp),%rax
   3c421:	mov    $0x30,%ecx
   3c426:	mul    %rcx
   3c429:	seto   %cl
   3c42c:	test   %rax,%rax
   3c42f:	sets   %dl
   3c432:	or     %cl,%dl
   3c434:	mov    $0x8,%bpl
   3c437:	jne    3cfb9 <meshopt_peers::run+0x3069>
   3c43d:	test   %rax,%rax
   3c440:	je     3c468 <meshopt_peers::run+0x2518>
   3c442:	add    %r12,%rax
   3c445:	jb     3cfb9 <meshopt_peers::run+0x3069>
   3c44b:	mov    %rax,0x768(%rsp)
   3c453:	mov    %r13,%rdi
   3c456:	mov    %rax,%rsi
   3c459:	call   42f00 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c45e:	mov    %eax,%ebp
   3c460:	cmp    $0xff,%al
   3c462:	jne    3cfb9 <meshopt_peers::run+0x3069>
   3c468:	mov    %r15,%rdx
   3c46b:	shr    $0x8,%rdx
   3c46f:	mov    %r15,%rax
   3c472:	shr    $0x28,%rax
   3c476:	mov    %r15,%rcx
   3c479:	shr    $0x38,%rcx
   3c47d:	mov    %cl,0xff(%rsp)
   3c484:	mov    %ax,0xfd(%rsp)
   3c48c:	mov    %edx,0xf9(%rsp)
   3c493:	movq   $0x0,0x100(%rsp)
   3c49f:	mov    0x18(%rsp),%rax
   3c4a4:	mov    %rax,0xf0(%rsp)
   3c4ac:	mov    %r15b,0xf8(%rsp)
   3c4b4:	cmpq   $0x2,0x10(%rsp)
   3c4ba:	ja     3c4c4 <meshopt_peers::run+0x2574>
   3c4bc:	mov    %rdx,%rcx
   3c4bf:	jmp    3c549 <meshopt_peers::run+0x25f9>
   3c4c4:	mov    0x48(%rsp),%rax
   3c4c9:	cmp    %rax,0x18(%rsp)
   3c4ce:	jb     3cf01 <meshopt_peers::run+0x2fb1>
   3c4d4:	xor    %ebx,%ebx
   3c4d6:	mov    0xf8(%rsp),%r15
   3c4de:	lea    (%rbx,%rbx,2),%rax
   3c4e2:	shl    $0x4,%rax
   3c4e6:	xorps  %xmm0,%xmm0
   3c4e9:	movups %xmm0,0x20(%r15,%rax,1)
   3c4ef:	movups %xmm0,0x10(%r15,%rax,1)
   3c4f5:	movups %xmm0,(%r15,%rax,1)
   3c4fa:	cmpq   $0x6,0x10(%rsp)
   3c500:	jb     3c523 <meshopt_peers::run+0x25d3>
   3c502:	lea    (%r15,%rax,1),%rdi
   3c506:	add    $0x30,%rdi
   3c50a:	mov    0x48(%rsp),%rax
   3c50f:	lea    (%rax,%rax,2),%rdx
   3c513:	shl    $0x4,%rdx
   3c517:	add    $0xffffffffffffffd0,%rdx
   3c51b:	xor    %esi,%esi
   3c51d:	call   *0x10dd45(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   3c523:	mov    0xf0(%rsp),%rax
   3c52b:	mov    %rax,0x18(%rsp)
   3c530:	cmp    $0xffffffffffffffff,%rax
   3c534:	je     3d363 <meshopt_peers::run+0x3413>
   3c53a:	mov    %r15,%rax
   3c53d:	shr    $0x8,%rax
   3c541:	mov    %rax,%rcx
   3c544:	add    %rbx,0x48(%rsp)
   3c549:	movzbl %r15b,%eax
   3c54d:	shl    $0x8,%rcx
   3c551:	or     %rax,%rcx
   3c554:	mov    %rcx,0xc8(%rsp)
   3c55c:	lea    0x310(%rsp),%rdi
   3c564:	lea    0x750(%rsp),%rsi
   3c56c:	mov    0x10(%rsp),%rdx
   3c571:	call   35f80 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   3c576:	mov    0x310(%rsp),%rax
   3c57e:	movzbl 0x318(%rsp),%ebp
   3c586:	mov    %rax,0x90(%rsp)
   3c58e:	cmp    $0xffffffffffffffff,%rax
   3c592:	je     3d32f <meshopt_peers::run+0x33df>
   3c598:	lea    0x319(%rsp),%rcx
   3c5a0:	mov    (%rcx),%rax
   3c5a3:	mov    0x7(%rcx),%rcx
   3c5a7:	lea    0xf9(%rsp),%rdx
   3c5af:	mov    %rcx,0x7(%rdx)
   3c5b3:	mov    %rax,(%rdx)
   3c5b6:	mov    0x90(%rsp),%rax
   3c5be:	mov    %rax,0xf0(%rsp)
   3c5c6:	mov    %bpl,0xf8(%rsp)
   3c5ce:	mov    0x100(%rsp),%rax
   3c5d6:	mov    0x10(%rsp),%r15
   3c5db:	sub    %rax,%r15
   3c5de:	jbe    3c638 <meshopt_peers::run+0x26e8>
   3c5e0:	mov    %rax,%rbx
   3c5e3:	mov    0x90(%rsp),%rax
   3c5eb:	sub    %rbx,%rax
   3c5ee:	cmp    %rax,%r15
   3c5f1:	ja     3cf2d <meshopt_peers::run+0x2fdd>
   3c5f7:	mov    0xf8(%rsp),%rbp
   3c5ff:	lea    0x0(,%rbx,4),%rdi
   3c607:	add    %rbp,%rdi
   3c60a:	lea    0x0(,%r15,4),%rdx
   3c612:	xor    %esi,%esi
   3c614:	call   *0x10dc4e(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   3c61a:	cmpq   $0xffffffffffffffff,0x90(%rsp)
   3c623:	je     3d32f <meshopt_peers::run+0x33df>
   3c629:	add    %r15,%rbx
   3c62c:	mov    %rbp,%rax
   3c62f:	shr    $0x8,%rax
   3c633:	mov    %rax,%rcx
   3c636:	jmp    3c663 <meshopt_peers::run+0x2713>
   3c638:	movzbl 0xff(%rsp),%eax
   3c640:	shl    $0x10,%eax
   3c643:	movzwl 0xfd(%rsp),%ecx
   3c64b:	or     %eax,%ecx
   3c64d:	shl    $0x20,%rcx
   3c651:	mov    0xf9(%rsp),%eax
   3c658:	or     %rcx,%rax
   3c65b:	mov    %rax,%rcx
   3c65e:	mov    0x10(%rsp),%rbx
   3c663:	movzbl %bpl,%eax
   3c667:	shl    $0x8,%rcx
   3c66b:	or     %rax,%rcx
   3c66e:	mov    %rcx,0xd8(%rsp)
   3c676:	cmpq   $0x2,0xe0(%rsp)
   3c67f:	jb     3cdb9 <meshopt_peers::run+0x2e69>
   3c685:	mov    0xc8(%rsp),%rax
   3c68d:	add    $0x10,%rax
   3c691:	mov    %rax,0x308(%rsp)
   3c699:	movq   $0x0,0x68(%rsp)
   3c6a2:	xor    %r12d,%r12d
   3c6a5:	mov    0x150(%rsp),%rax
   3c6ad:	mov    %rax,0x158(%rsp)
   3c6b5:	mov    0x68(%rsp),%rax
   3c6ba:	mov    %rax,0xd0(%rsp)
   3c6c2:	addq   $0x4,0x158(%rsp)
   3c6cb:	mov    0x158(%rsp),%rax
   3c6d3:	mov    -0x4(%rax),%edi
   3c6d6:	mov    (%rax),%r15d
   3c6d9:	cmp    %edi,%r15d
   3c6dc:	jb     3d5e3 <meshopt_peers::run+0x3693>
   3c6e2:	cmp    %r15,0x70(%rsp)
   3c6e7:	jb     3d5e3 <meshopt_peers::run+0x3693>
   3c6ed:	sub    %rdi,%r15
   3c6f0:	lea    (%rdi,%rdi,2),%r13
   3c6f4:	shl    $0x4,%r13
   3c6f8:	add    0x30(%rsp),%r13
   3c6fd:	movq   $0xffffffffffffffff,0xf0(%rsp)
   3c709:	lea    0x310(%rsp),%rdi
   3c711:	mov    %r13,%rsi
   3c714:	mov    %r15,%rdx
   3c717:	mov    0x20(%rsp),%rcx
   3c71c:	mov    0x10(%rsp),%r8
   3c721:	lea    0x260(%rsp),%r9
   3c729:	lea    0x750(%rsp),%rax
   3c731:	push   %rax
   3c732:	lea    0x1d0(%rsp),%rax
   3c73a:	push   %rax
   3c73b:	push   %r14
   3c73d:	push   0xb8(%rsp)
   3c744:	call   *0x10df56(%rip)        # 14a6a0 <_DYNAMIC+0x660>
   3c74a:	add    $0x20,%rsp
   3c74e:	mov    0x310(%rsp),%rax
   3c756:	movzbl 0x318(%rsp),%ebp
   3c75e:	mov    %rax,0x60(%rsp)
   3c763:	cmp    $0xffffffffffffffff,%rax
   3c767:	je     3d2cb <meshopt_peers::run+0x337b>
   3c76d:	movzbl %bpl,%eax
   3c771:	movzbl 0x31f(%rsp),%ecx
   3c779:	shl    $0x10,%ecx
   3c77c:	movzwl 0x31d(%rsp),%edx
   3c784:	or     %ecx,%edx
   3c786:	mov    0x319(%rsp),%esi
   3c78d:	mov    0x328(%rsp),%rcx
   3c795:	mov    %rcx,0x58(%rsp)
   3c79a:	mov    0x330(%rsp),%rcx
   3c7a2:	mov    %rcx,0xb8(%rsp)
   3c7aa:	mov    0x340(%rsp),%rcx
   3c7b2:	mov    %rcx,0x28(%rsp)
   3c7b7:	mov    0x348(%rsp),%rcx
   3c7bf:	mov    %rcx,0x50(%rsp)
   3c7c4:	mov    0x350(%rsp),%rcx
   3c7cc:	mov    %rcx,0x68(%rsp)
   3c7d1:	lea    0x319(%rsp),%rcx
   3c7d9:	movups 0x3f(%rcx),%xmm0
   3c7dd:	movaps %xmm0,0x2f0(%rsp)
   3c7e5:	mov    0x4f(%rcx),%ecx
   3c7e8:	mov    %ecx,0x300(%rsp)
   3c7ef:	shl    $0x28,%rdx
   3c7f3:	shl    $0x8,%rsi
   3c7f7:	or     %rdx,%rsi
   3c7fa:	or     %rax,%rsi
   3c7fd:	mov    %rsi,0xc0(%rsp)
   3c805:	testb  $0x1,0x36c(%rsp)
   3c80d:	je     3c98e <meshopt_peers::run+0x2a3e>
   3c813:	mov    %r13,%rdi
   3c816:	mov    %r15,%rsi
   3c819:	mov    0x20(%rsp),%rdx
   3c81e:	mov    0x10(%rsp),%rcx
   3c823:	mov    0x8(%rsp),%r8
   3c828:	mov    %r14,%r9
   3c82b:	lea    0x750(%rsp),%rax
   3c833:	push   %rax
   3c834:	lea    0xb4(%rsp),%rax
   3c83c:	push   %rax
   3c83d:	lea    0x1c0(%rsp),%rax
   3c845:	push   %rax
   3c846:	push   0x178(%rsp)
   3c84d:	lea    0x310(%rsp),%rax
   3c855:	push   %rax
   3c856:	push   $0x0
   3c858:	call   36c20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3c85d:	add    $0x30,%rsp
   3c861:	mov    %rax,%rbp
   3c864:	test   $0x1,%bpl
   3c868:	jne    3d382 <meshopt_peers::run+0x3432>
   3c86e:	mov    0x768(%rsp),%rcx
   3c876:	mov    0x60(%rsp),%rdx
   3c87b:	lea    0x0(,%rdx,4),%rax
   3c883:	shr    $0x3e,%rdx
   3c887:	sete   %dl
   3c88a:	test   %rax,%rax
   3c88d:	setns  %sil
   3c891:	and    %dl,%sil
   3c894:	sub    %rax,%rcx
   3c897:	setae  %al
   3c89a:	test   %al,%sil
   3c89d:	je     3d083 <meshopt_peers::run+0x3133>
   3c8a3:	mov    %rcx,0x768(%rsp)
   3c8ab:	cmpq   $0x0,0x60(%rsp)
   3c8b1:	je     3c8c9 <meshopt_peers::run+0x2979>
   3c8b3:	mov    0xc0(%rsp),%rdi
   3c8bb:	call   *0x10d987(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3c8c1:	mov    0x768(%rsp),%rcx
   3c8c9:	mov    0x58(%rsp),%rax
   3c8ce:	mov    $0x14,%edx
   3c8d3:	mul    %rdx
   3c8d6:	jo     3d29f <meshopt_peers::run+0x334f>
   3c8dc:	test   %rax,%rax
   3c8df:	js     3d29f <meshopt_peers::run+0x334f>
   3c8e5:	sub    %rax,%rcx
   3c8e8:	jb     3d29f <meshopt_peers::run+0x334f>
   3c8ee:	mov    %rcx,0x768(%rsp)
   3c8f6:	cmpq   $0x0,0x58(%rsp)
   3c8fc:	je     3c914 <meshopt_peers::run+0x29c4>
   3c8fe:	mov    0xb8(%rsp),%rdi
   3c906:	call   *0x10d93c(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3c90c:	mov    0x768(%rsp),%rcx
   3c914:	movabs $0x3fffffffffffffff,%rax
   3c91e:	cmp    %rax,0x28(%rsp)
   3c923:	ja     3d34a <meshopt_peers::run+0x33fa>
   3c929:	mov    0x28(%rsp),%rax
   3c92e:	lea    0x0(,%rax,4),%rax
   3c936:	test   %rax,%rax
   3c939:	js     3d34a <meshopt_peers::run+0x33fa>
   3c93f:	sub    %rax,%rcx
   3c942:	jb     3d34a <meshopt_peers::run+0x33fa>
   3c948:	mov    %rcx,0x768(%rsp)
   3c950:	cmpq   $0x0,0x28(%rsp)
   3c956:	je     3c963 <meshopt_peers::run+0x2a13>
   3c958:	mov    0x50(%rsp),%rdi
   3c95d:	call   *0x10d8e5(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3c963:	mov    0xe0(%rsp),%rax
   3c96b:	dec    %rax
   3c96e:	addq   $0x4,0x158(%rsp)
   3c977:	mov    %rax,0xe0(%rsp)
   3c97f:	cmp    $0x1,%rax
   3c983:	ja     3c6cb <meshopt_peers::run+0x277b>
   3c989:	jmp    3cf68 <meshopt_peers::run+0x3018>
   3c98e:	mov    %r13,%rdi
   3c991:	mov    %r15,%rsi
   3c994:	mov    0x20(%rsp),%rdx
   3c999:	mov    0x10(%rsp),%rcx
   3c99e:	mov    0x8(%rsp),%r8
   3c9a3:	mov    %r14,%r9
   3c9a6:	lea    0x750(%rsp),%rax
   3c9ae:	push   %rax
   3c9af:	lea    0xb4(%rsp),%rax
   3c9b7:	push   %rax
   3c9b8:	lea    0x1c0(%rsp),%rax
   3c9c0:	push   %rax
   3c9c1:	push   0x178(%rsp)
   3c9c8:	lea    0x310(%rsp),%rax
   3c9d0:	push   %rax
   3c9d1:	push   $0x0
   3c9d3:	call   36c20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3c9d8:	add    $0x30,%rsp
   3c9dc:	mov    %rax,%rbp
   3c9df:	test   $0x1,%bpl
   3c9e3:	jne    3d382 <meshopt_peers::run+0x3432>
   3c9e9:	shr    $0x20,%rbp
   3c9ed:	movb   $0x2,0x168(%rsp)
   3c9f5:	mov    0x8(%rsp),%rax
   3c9fa:	mov    %rax,0x170(%rsp)
   3ca02:	mov    %r14,0x178(%rsp)
   3ca0a:	movq   $0x0,0x1a0(%rsp)
   3ca16:	movups 0x208(%rsp),%xmm0
   3ca1e:	movaps %xmm0,0x350(%rsp)
   3ca26:	movups 0x1c8(%rsp),%xmm0
   3ca2e:	movups 0x1d8(%rsp),%xmm1
   3ca36:	movdqu 0x1e8(%rsp),%xmm2
   3ca3f:	movdqu 0x1f8(%rsp),%xmm3
   3ca48:	movdqa %xmm3,0x340(%rsp)
   3ca51:	movdqa %xmm2,0x330(%rsp)
   3ca5a:	movaps %xmm1,0x320(%rsp)
   3ca62:	movaps %xmm0,0x310(%rsp)
   3ca6a:	lea    0x230(%rsp),%rdi
   3ca72:	mov    0x50(%rsp),%rsi
   3ca77:	mov    0x68(%rsp),%rdx
   3ca7c:	lea    0x168(%rsp),%rcx
   3ca84:	lea    0x310(%rsp),%r8
   3ca8c:	mov    %ebp,%r9d
   3ca8f:	lea    0x750(%rsp),%rax
   3ca97:	push   %rax
   3ca98:	push   0x220(%rsp)
   3ca9f:	call   *0x10dbdb(%rip)        # 14a680 <_DYNAMIC+0x640>
   3caa5:	add    $0x10,%rsp
   3caa9:	mov    0x230(%rsp),%r15
   3cab1:	movzbl 0x238(%rsp),%ebp
   3cab9:	cmp    $0xffffffffffffffff,%r15
   3cabd:	je     3d385 <meshopt_peers::run+0x3435>
   3cac3:	movzbl %bpl,%ecx
   3cac7:	movzbl 0x23f(%rsp),%eax
   3cacf:	shl    $0x10,%eax
   3cad2:	movzwl 0x23d(%rsp),%esi
   3cada:	or     %eax,%esi
   3cadc:	mov    0x239(%rsp),%r13d
   3cae4:	mov    0x240(%rsp),%rax
   3caec:	mov    0x248(%rsp),%rbp
   3caf4:	mov    0x250(%rsp),%rdx
   3cafc:	mov    %rdx,0xb0(%rsp)
   3cb04:	mov    0x258(%rsp),%rdx
   3cb0c:	shl    $0x28,%rsi
   3cb10:	shl    $0x8,%r13
   3cb14:	or     %rsi,%r13
   3cb17:	or     %rcx,%r13
   3cb1a:	test   %rax,%rax
   3cb1d:	je     3cbc6 <meshopt_peers::run+0x2c76>
   3cb23:	lea    (%rax,%rax,2),%rax
   3cb27:	shl    $0x4,%rax
   3cb2b:	add    %r13,%rax
   3cb2e:	lea    (%r12,%r12,2),%rcx
   3cb32:	shl    $0x4,%rcx
   3cb36:	add    0x308(%rsp),%rcx
   3cb3e:	xor    %esi,%esi
   3cb40:	mov    0x0(%r13,%rsi,1),%rdi
   3cb45:	mov    0x8(%r13,%rsi,1),%r8
   3cb4a:	movups 0x10(%r13,%rsi,1),%xmm0
   3cb50:	movaps %xmm0,0x720(%rsp)
   3cb58:	mov    0x300(%rsp),%r9d
   3cb60:	lea    0x72c(%rsp),%r10
   3cb68:	mov    %r9d,0x10(%r10)
   3cb6c:	movaps 0x2f0(%rsp),%xmm0
   3cb74:	movups %xmm0,(%r10)
   3cb78:	cmp    0x48(%rsp),%r12
   3cb7d:	jae    3d754 <meshopt_peers::run+0x3804>
   3cb83:	lea    (%rsi,%r13,1),%r9
   3cb87:	add    0xd0(%rsp),%r8
   3cb8f:	mov    %rdi,-0x10(%rcx,%rsi,1)
   3cb94:	mov    %r8,-0x8(%rcx,%rsi,1)
   3cb99:	movaps 0x720(%rsp),%xmm0
   3cba1:	movaps 0x730(%rsp),%xmm1
   3cba9:	movups %xmm0,(%rcx,%rsi,1)
   3cbad:	movups %xmm1,0x10(%rcx,%rsi,1)
   3cbb2:	inc    %r12
   3cbb5:	add    $0x30,%rsi
   3cbb9:	add    $0x30,%r9
   3cbbd:	cmp    %rax,%r9
   3cbc0:	jne    3cb40 <meshopt_peers::run+0x2bf0>
   3cbc6:	mov    %rdx,%rax
   3cbc9:	add    0xd0(%rsp),%rax
   3cbd1:	mov    %rax,0x68(%rsp)
   3cbd6:	jb     3d6c6 <meshopt_peers::run+0x3776>
   3cbdc:	cmp    %rbx,0x68(%rsp)
   3cbe1:	ja     3d6c6 <meshopt_peers::run+0x3776>
   3cbe7:	mov    0xd8(%rsp),%rax
   3cbef:	mov    0xd0(%rsp),%rcx
   3cbf7:	lea    (%rax,%rcx,4),%rdi
   3cbfb:	shl    $0x2,%rdx
   3cbff:	mov    0xb0(%rsp),%rsi
   3cc07:	call   *0x10d64b(%rip)        # 14a258 <memcpy@GLIBC_2.14>
   3cc0d:	mov    0x768(%rsp),%rcx
   3cc15:	mov    %r15,%rax
   3cc18:	mov    $0x30,%edx
   3cc1d:	mul    %rdx
   3cc20:	seto   %dl
   3cc23:	not    %dl
   3cc25:	test   %rax,%rax
   3cc28:	setns  %sil
   3cc2c:	and    %dl,%sil
   3cc2f:	sub    %rax,%rcx
   3cc32:	setae  %al
   3cc35:	test   %al,%sil
   3cc38:	je     3d3e2 <meshopt_peers::run+0x3492>
   3cc3e:	mov    %rcx,0x768(%rsp)
   3cc46:	test   %r15,%r15
   3cc49:	je     3cc5c <meshopt_peers::run+0x2d0c>
   3cc4b:	mov    %r13,%rdi
   3cc4e:	call   *0x10d5f4(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cc54:	mov    0x768(%rsp),%rcx
   3cc5c:	mov    %rbp,%rax
   3cc5f:	shr    $0x3e,%rax
   3cc63:	jne    3d3f0 <meshopt_peers::run+0x34a0>
   3cc69:	lea    0x0(,%rbp,4),%rax
   3cc71:	test   %rax,%rax
   3cc74:	js     3d3f0 <meshopt_peers::run+0x34a0>
   3cc7a:	sub    %rax,%rcx
   3cc7d:	jb     3d3f0 <meshopt_peers::run+0x34a0>
   3cc83:	mov    %rcx,0x768(%rsp)
   3cc8b:	test   %rbp,%rbp
   3cc8e:	je     3cca6 <meshopt_peers::run+0x2d56>
   3cc90:	mov    0xb0(%rsp),%rdi
   3cc98:	call   *0x10d5aa(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cc9e:	mov    0x768(%rsp),%rcx
   3cca6:	movabs $0x3fffffffffffffff,%rax
   3ccb0:	cmp    %rax,0x60(%rsp)
   3ccb5:	ja     3d56b <meshopt_peers::run+0x361b>
   3ccbb:	mov    0x60(%rsp),%rax
   3ccc0:	lea    0x0(,%rax,4),%rax
   3ccc8:	test   %rax,%rax
   3cccb:	js     3d56b <meshopt_peers::run+0x361b>
   3ccd1:	sub    %rax,%rcx
   3ccd4:	jb     3d56b <meshopt_peers::run+0x361b>
   3ccda:	mov    %rcx,0x768(%rsp)
   3cce2:	cmpq   $0x0,0x60(%rsp)
   3cce8:	je     3cd00 <meshopt_peers::run+0x2db0>
   3ccea:	mov    0xc0(%rsp),%rdi
   3ccf2:	call   *0x10d550(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3ccf8:	mov    0x768(%rsp),%rcx
   3cd00:	mov    0x58(%rsp),%rax
   3cd05:	mov    $0x14,%edx
   3cd0a:	mul    %rdx
   3cd0d:	jo     3d67e <meshopt_peers::run+0x372e>
   3cd13:	test   %rax,%rax
   3cd16:	js     3d67e <meshopt_peers::run+0x372e>
   3cd1c:	sub    %rax,%rcx
   3cd1f:	jb     3d67e <meshopt_peers::run+0x372e>
   3cd25:	mov    %rcx,0x768(%rsp)
   3cd2d:	cmpq   $0x0,0x58(%rsp)
   3cd33:	je     3cd4b <meshopt_peers::run+0x2dfb>
   3cd35:	mov    0xb8(%rsp),%rdi
   3cd3d:	call   *0x10d505(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cd43:	mov    0x768(%rsp),%rcx
   3cd4b:	movabs $0x3fffffffffffffff,%rax
   3cd55:	cmp    %rax,0x28(%rsp)
   3cd5a:	ja     3d6b2 <meshopt_peers::run+0x3762>
   3cd60:	mov    0x28(%rsp),%rax
   3cd65:	lea    0x0(,%rax,4),%rax
   3cd6d:	test   %rax,%rax
   3cd70:	js     3d6b2 <meshopt_peers::run+0x3762>
   3cd76:	sub    %rax,%rcx
   3cd79:	jb     3d6b2 <meshopt_peers::run+0x3762>
   3cd7f:	mov    %rcx,0x768(%rsp)
   3cd87:	cmpq   $0x0,0x28(%rsp)
   3cd8d:	je     3cd9a <meshopt_peers::run+0x2e4a>
   3cd8f:	mov    0x50(%rsp),%rdi
   3cd94:	call   *0x10d4ae(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cd9a:	mov    0xe0(%rsp),%rax
   3cda2:	dec    %rax
   3cda5:	mov    %rax,0xe0(%rsp)
   3cdad:	cmp    $0x1,%rax
   3cdb1:	ja     3c6b5 <meshopt_peers::run+0x2765>
   3cdb7:	jmp    3cdc5 <meshopt_peers::run+0x2e75>
   3cdb9:	xor    %r12d,%r12d
   3cdbc:	movq   $0x0,0x68(%rsp)
   3cdc5:	lea    0x750(%rsp),%rdi
   3cdcd:	mov    0x38(%rsp),%rsi
   3cdd2:	mov    0x150(%rsp),%rdx
   3cdda:	call   35f30 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3cddf:	mov    %eax,%ebp
   3cde1:	mov    $0x1,%al
   3cde3:	mov    %eax,0x70(%rsp)
   3cde7:	cmp    $0xff,%bpl
   3cdeb:	setne  %r13b
   3cdef:	jne    3d37a <meshopt_peers::run+0x342a>
   3cdf5:	mov    0x48(%rsp),%r15
   3cdfa:	cmp    %r15,%r12
   3cdfd:	cmovb  %r12,%r15
   3ce01:	mov    0x68(%rsp),%rax
   3ce06:	cmp    %rbx,%rax
   3ce09:	cmovb  %rax,%rbx
   3ce0d:	mov    %rbx,0x10(%rsp)
   3ce12:	lea    0x750(%rsp),%rdi
   3ce1a:	mov    0x80(%rsp),%rsi
   3ce22:	mov    0x30(%rsp),%rdx
   3ce27:	call   35ee0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3ce2c:	cmp    $0xff,%al
   3ce2e:	jne    3d399 <meshopt_peers::run+0x3449>
   3ce34:	lea    0x750(%rsp),%rdi
   3ce3c:	mov    0x78(%rsp),%rsi
   3ce41:	mov    0x20(%rsp),%rdx
   3ce46:	call   35f30 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3ce4b:	cmp    $0xff,%al
   3ce4d:	jne    3d3a3 <meshopt_peers::run+0x3453>
   3ce53:	mov    0x160(%rsp),%r13
   3ce5b:	inc    %r13d
   3ce5e:	mov    0x18(%rsp),%rax
   3ce63:	mov    %rax,0x80(%rsp)
   3ce6b:	mov    %r15,%rdx
   3ce6e:	mov    0xc8(%rsp),%rax
   3ce76:	mov    %rax,0x30(%rsp)
   3ce7b:	mov    0x10(%rsp),%r8
   3ce80:	mov    0xd8(%rsp),%rax
   3ce88:	mov    %rax,0x20(%rsp)
   3ce8d:	mov    0x90(%rsp),%rax
   3ce95:	mov    %rax,0x78(%rsp)
   3ce9a:	cmp    $0x1,%r15
   3ce9e:	mov    (%rsp),%rbx
   3cea2:	lea    0x168(%rsp),%rdi
   3ceaa:	mov    0x10d7df(%rip),%r10        # 14a690 <_DYNAMIC+0x650>
   3ceb1:	ja     3c1f1 <meshopt_peers::run+0x22a1>
   3ceb7:	jmp    3d3b5 <meshopt_peers::run+0x3465>
   3cebc:	mov    $0x8,%edx
   3cec1:	mov    $0x8,%r8d
   3cec7:	mov    $0x30,%r9d
   3cecd:	lea    0x310(%rsp),%rdi
   3ced5:	xor    %esi,%esi
   3ced7:	mov    0x48(%rsp),%rcx
   3cedc:	call   421f0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3cee1:	cmpl   $0x1,0x310(%rsp)
   3cee9:	je     3d432 <meshopt_peers::run+0x34e2>
   3ceef:	mov    0x318(%rsp),%r15
   3cef7:	mov    0x48(%rsp),%rax
   3cefc:	jmp    3c417 <meshopt_peers::run+0x24c7>
   3cf01:	mov    $0x8,%ecx
   3cf06:	mov    $0x30,%r8d
   3cf0c:	lea    0xf0(%rsp),%rdi
   3cf14:	xor    %esi,%esi
   3cf16:	mov    0x48(%rsp),%rdx
   3cf1b:	call   39d10 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   3cf20:	mov    0x100(%rsp),%rbx
   3cf28:	jmp    3c4d6 <meshopt_peers::run+0x2586>
   3cf2d:	mov    $0x4,%ecx
   3cf32:	mov    $0x4,%r8d
   3cf38:	lea    0xf0(%rsp),%rdi
   3cf40:	mov    %rbx,%rsi
   3cf43:	mov    %r15,%rdx
   3cf46:	call   39d10 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   3cf4b:	mov    0xf0(%rsp),%rax
   3cf53:	mov    %rax,0x90(%rsp)
   3cf5b:	mov    0x100(%rsp),%rbx
   3cf63:	jmp    3c5f7 <meshopt_peers::run+0x26a7>
   3cf68:	mov    0xd0(%rsp),%rax
   3cf70:	mov    %rax,0x68(%rsp)
   3cf75:	jmp    3cdc5 <meshopt_peers::run+0x2e75>
   3cf7a:	lea    0x750(%rsp),%rdi
   3cf82:	mov    0x30(%rsp),%rdx
   3cf87:	call   35ee0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3cf8c:	cmp    $0xff,%al
   3cf8e:	jne    3d6e8 <meshopt_peers::run+0x3798>
   3cf94:	lea    0x750(%rsp),%rdi
   3cf9c:	mov    0x90(%rsp),%rsi
   3cfa4:	mov    0x20(%rsp),%rdx
   3cfa9:	call   35f30 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3cfae:	mov    %eax,%ebp
   3cfb0:	mov    (%rsp),%rbx
   3cfb4:	jmp    3b8a2 <meshopt_peers::run+0x1952>
   3cfb9:	cmpq   $0x0,0x18(%rsp)
   3cfbf:	je     3cfca <meshopt_peers::run+0x307a>
   3cfc1:	mov    %r15,%rdi
   3cfc4:	call   *0x10d27e(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cfca:	mov    (%rsp),%rbx
   3cfce:	mov    0x38(%rsp),%r15
   3cfd3:	mov    $0x1,%r12b
   3cfd6:	mov    $0x1,%r13b
   3cfd9:	mov    %r13b,(%rsp)
   3cfdd:	test   %r15,%r15
   3cfe0:	je     3d01a <meshopt_peers::run+0x30ca>
   3cfe2:	mov    0x150(%rsp),%rdi
   3cfea:	call   *0x10d258(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3cff0:	mov    $0x1,%al
   3cff2:	mov    %rax,0x40(%rsp)
   3cff7:	mov    $0x1,%r15b
   3cffa:	test   %r12b,%r12b
   3cffd:	mov    0x80(%rsp),%r12
   3d005:	mov    0x78(%rsp),%rcx
   3d00a:	movzbl (%rsp),%r13d
   3d00f:	jne    3d21b <meshopt_peers::run+0x32cb>
   3d015:	jmp    3d22b <meshopt_peers::run+0x32db>
   3d01a:	movzbl (%rsp),%r13d
   3d01f:	mov    $0x1,%cl
   3d021:	mov    %rcx,0x40(%rsp)
   3d026:	mov    $0x1,%r15b
   3d029:	test   %r12b,%r12b
   3d02c:	mov    0x80(%rsp),%r12
   3d034:	mov    0x78(%rsp),%rcx
   3d039:	jne    3d21b <meshopt_peers::run+0x32cb>
   3d03f:	jmp    3d22b <meshopt_peers::run+0x32db>
   3d044:	movzbl 0x758(%rsp),%eax
   3d04c:	mov    %al,0x310(%rsp)
   3d053:	lea    -0x2bbcf(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d05a:	lea    0x10573f(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d061:	lea    0x1053c0(%rip),%r8        # 142428 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x398>
   3d068:	lea    0x310(%rsp),%rdx
   3d070:	mov    $0x2b,%esi
   3d075:	call   *0x10d295(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d07b:	mov    $0x8,%bpl
   3d07e:	jmp    3b889 <meshopt_peers::run+0x1939>
   3d083:	mov    $0x8,%bpl
   3d086:	cmpq   $0x0,0x60(%rsp)
   3d08c:	je     3d25b <meshopt_peers::run+0x330b>
   3d092:	mov    0xc0(%rsp),%rdi
   3d09a:	call   *0x10d1a8(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d0a0:	xor    %r15d,%r15d
   3d0a3:	cmpq   $0x0,0x28(%rsp)
   3d0a9:	jne    3d266 <meshopt_peers::run+0x3316>
   3d0af:	jmp    3d271 <meshopt_peers::run+0x3321>
   3d0b4:	movzbl 0x758(%rsp),%eax
   3d0bc:	mov    %al,0x310(%rsp)
   3d0c3:	lea    -0x2bc3f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d0ca:	lea    0x1056cf(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d0d1:	lea    0x105320(%rip),%r8        # 1423f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x368>
   3d0d8:	lea    0x310(%rsp),%rdx
   3d0e0:	mov    $0x2b,%esi
   3d0e5:	call   *0x10d225(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d0eb:	movzbl 0x758(%rsp),%eax
   3d0f3:	mov    %al,0x310(%rsp)
   3d0fa:	lea    -0x2bc76(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d101:	lea    0x105698(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d108:	lea    0x105211(%rip),%r8        # 142320 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x290>
   3d10f:	lea    0x310(%rsp),%rdx
   3d117:	mov    $0x2b,%esi
   3d11c:	call   *0x10d1ee(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d122:	movzbl 0x758(%rsp),%eax
   3d12a:	mov    %al,0x310(%rsp)
   3d131:	lea    -0x2bcad(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d138:	lea    0x105661(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d13f:	lea    0x105312(%rip),%r8        # 142458 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3c8>
   3d146:	lea    0x310(%rsp),%rdx
   3d14e:	mov    $0x2b,%esi
   3d153:	call   *0x10d1b7(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d159:	movzbl 0x311(%rsp),%eax
   3d161:	mov    %al,0x260(%rsp)
   3d168:	lea    -0x2bce4(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d16f:	lea    0x10562a(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d176:	lea    0x105353(%rip),%r8        # 1424d0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x440>
   3d17d:	lea    0x260(%rsp),%rdx
   3d185:	mov    $0x2b,%esi
   3d18a:	call   *0x10d180(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d190:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d195:	lea    0x105064(%rip),%rcx        # 142200 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x170>
   3d19c:	mov    0x10(%rsp),%rdx
   3d1a1:	call   *0x10d209(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   3d1a7:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d1ac:	mov    (%rsp),%rbx
   3d1b0:	mov    0x8(%rsp),%r11
   3d1b5:	jmp    3dbb1 <meshopt_peers::run+0x3c61>
   3d1ba:	movzbl 0x318(%rsp),%eax
   3d1c2:	mov    %al,0x750(%rsp)
   3d1c9:	lea    -0x2bd45(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d1d0:	lea    0x1055c9(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d1d7:	lea    0x105172(%rip),%r8        # 142350 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2c0>
   3d1de:	lea    0x750(%rsp),%rdx
   3d1e6:	mov    $0x2b,%esi
   3d1eb:	call   *0x10d11f(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d1f1:	mov    $0x8,%bpl
   3d1f4:	lea    0x310(%rsp),%rdi
   3d1fc:	call   36b90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3d201:	mov    $0x1,%al
   3d203:	mov    %rax,0x40(%rsp)
   3d208:	mov    $0x1,%r15b
   3d20b:	mov    $0x1,%r13b
   3d20e:	mov    0x80(%rsp),%r12
   3d216:	mov    0x78(%rsp),%rcx
   3d21b:	test   %rcx,%rcx
   3d21e:	je     3d22b <meshopt_peers::run+0x32db>
   3d220:	mov    0x20(%rsp),%rdi
   3d225:	call   *0x10d01d(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d22b:	test   %r13b,%r13b
   3d22e:	je     3d240 <meshopt_peers::run+0x32f0>
   3d230:	test   %r12,%r12
   3d233:	je     3d240 <meshopt_peers::run+0x32f0>
   3d235:	mov    0x30(%rsp),%rdi
   3d23a:	call   *0x10d008(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d240:	test   %r15b,%r15b
   3d243:	mov    0x40(%rsp),%rax
   3d248:	jne    3ba8d <meshopt_peers::run+0x1b3d>
   3d24e:	test   %al,%al
   3d250:	jne    3b889 <meshopt_peers::run+0x1939>
   3d256:	jmp    3b8a2 <meshopt_peers::run+0x1952>
   3d25b:	xor    %r15d,%r15d
   3d25e:	cmpq   $0x0,0x28(%rsp)
   3d264:	je     3d271 <meshopt_peers::run+0x3321>
   3d266:	mov    0x50(%rsp),%rdi
   3d26b:	call   *0x10cfd7(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d271:	cmpq   $0x0,0x58(%rsp)
   3d277:	je     3d287 <meshopt_peers::run+0x3337>
   3d279:	mov    0xb8(%rsp),%rdi
   3d281:	call   *0x10cfc1(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d287:	cmpq   $0x0,0x60(%rsp)
   3d28d:	setne  %al
   3d290:	test   %r15b,%al
   3d293:	je     3d2cb <meshopt_peers::run+0x337b>
   3d295:	mov    0xc0(%rsp),%rdi
   3d29d:	jmp    3d2c5 <meshopt_peers::run+0x3375>
   3d29f:	cmpq   $0x0,0x58(%rsp)
   3d2a5:	je     3d2b5 <meshopt_peers::run+0x3365>
   3d2a7:	mov    0xb8(%rsp),%rdi
   3d2af:	call   *0x10cf93(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d2b5:	mov    $0x8,%bpl
   3d2b8:	cmpq   $0x0,0x28(%rsp)
   3d2be:	je     3d2cb <meshopt_peers::run+0x337b>
   3d2c0:	mov    0x50(%rsp),%rdi
   3d2c5:	call   *0x10cf7d(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d2cb:	lea    0xf0(%rsp),%rdi
   3d2d3:	call   365a0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3d2d8:	mov    $0x1,%r15b
   3d2db:	mov    $0x1,%al
   3d2dd:	mov    %eax,0x70(%rsp)
   3d2e1:	mov    $0x1,%r13b
   3d2e4:	cmpq   $0x0,0x90(%rsp)
   3d2ed:	je     3d2fd <meshopt_peers::run+0x33ad>
   3d2ef:	mov    0xd8(%rsp),%rdi
   3d2f7:	call   *0x10cf4b(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d2fd:	cmpq   $0x0,0x18(%rsp)
   3d303:	je     3d313 <meshopt_peers::run+0x33c3>
   3d305:	mov    0xc8(%rsp),%rdi
   3d30d:	call   *0x10cf35(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d313:	test   %r15b,%r15b
   3d316:	mov    (%rsp),%rbx
   3d31a:	mov    0x38(%rsp),%r15
   3d31f:	mov    0x70(%rsp),%r12d
   3d324:	jne    3cfd9 <meshopt_peers::run+0x3089>
   3d32a:	jmp    3d01f <meshopt_peers::run+0x30cf>
   3d32f:	cmpq   $0x0,0x18(%rsp)
   3d335:	je     3d35e <meshopt_peers::run+0x340e>
   3d337:	mov    0xc8(%rsp),%rdi
   3d33f:	call   *0x10cf03(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d345:	mov    $0x1,%r13b
   3d348:	jmp    3d369 <meshopt_peers::run+0x3419>
   3d34a:	mov    $0x8,%bpl
   3d34d:	cmpq   $0x0,0x28(%rsp)
   3d353:	je     3d2cb <meshopt_peers::run+0x337b>
   3d359:	jmp    3d2c0 <meshopt_peers::run+0x3370>
   3d35e:	mov    $0x1,%r13b
   3d361:	jmp    3d369 <meshopt_peers::run+0x3419>
   3d363:	mov    $0x1,%r13b
   3d366:	mov    %r15d,%ebp
   3d369:	mov    (%rsp),%rbx
   3d36d:	mov    0x38(%rsp),%r15
   3d372:	mov    $0x1,%r12b
   3d375:	jmp    3cfd9 <meshopt_peers::run+0x3089>
   3d37a:	xor    %r15d,%r15d
   3d37d:	jmp    3d2e4 <meshopt_peers::run+0x3394>
   3d382:	shr    $0x8,%ebp
   3d385:	mov    $0x1,%r15b
   3d388:	cmpq   $0x0,0x28(%rsp)
   3d38e:	jne    3d266 <meshopt_peers::run+0x3316>
   3d394:	jmp    3d271 <meshopt_peers::run+0x3321>
   3d399:	mov    %eax,%ebp
   3d39b:	xor    %r15d,%r15d
   3d39e:	jmp    3d2e4 <meshopt_peers::run+0x3394>
   3d3a3:	mov    %eax,%ebp
   3d3a5:	xor    %r15d,%r15d
   3d3a8:	movl   $0x0,0x70(%rsp)
   3d3b0:	jmp    3d2e4 <meshopt_peers::run+0x3394>
   3d3b5:	mov    0xc8(%rsp),%rax
   3d3bd:	mov    %rax,0x30(%rsp)
   3d3c2:	mov    0xd8(%rsp),%rax
   3d3ca:	mov    %rax,0x20(%rsp)
   3d3cf:	test   %r15,%r15
   3d3d2:	mov    0x18(%rsp),%r12
   3d3d7:	jne    3c0c9 <meshopt_peers::run+0x2179>
   3d3dd:	jmp    3bfce <meshopt_peers::run+0x207e>
   3d3e2:	test   %r15,%r15
   3d3e5:	je     3d3f0 <meshopt_peers::run+0x34a0>
   3d3e7:	mov    %r13,%rdi
   3d3ea:	call   *0x10ce58(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d3f0:	mov    $0x1,%r13b
   3d3f3:	mov    $0x1,%r15b
   3d3f6:	test   %rbp,%rbp
   3d3f9:	je     3d40b <meshopt_peers::run+0x34bb>
   3d3fb:	mov    0xb0(%rsp),%rdi
   3d403:	call   *0x10ce3f(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d409:	jmp    3d40e <meshopt_peers::run+0x34be>
   3d40b:	mov    $0x1,%r15b
   3d40e:	cmpq   $0x0,0x28(%rsp)
   3d414:	je     3d421 <meshopt_peers::run+0x34d1>
   3d416:	mov    0x50(%rsp),%rdi
   3d41b:	call   *0x10ce27(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d421:	mov    $0x8,%bpl
   3d424:	test   %r13b,%r13b
   3d427:	jne    3d271 <meshopt_peers::run+0x3321>
   3d42d:	jmp    3d287 <meshopt_peers::run+0x3337>
   3d432:	mov    $0xa,%bpl
   3d435:	jmp    3cfca <meshopt_peers::run+0x307a>
   3d43a:	movb   $0x8,0x750(%rsp)
   3d442:	lea    -0x2bfbe(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d449:	lea    0x105350(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d450:	lea    0x104e81(%rip),%r8        # 1422d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x248>
   3d457:	lea    0x750(%rsp),%rdx
   3d45f:	mov    $0x2b,%esi
   3d464:	call   *0x10cea6(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d46a:	movb   $0x8,0x750(%rsp)
   3d472:	lea    -0x2bfee(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d479:	lea    0x105320(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d480:	lea    0x104ef9(%rip),%r8        # 142380 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2f0>
   3d487:	lea    0x750(%rsp),%rdx
   3d48f:	mov    $0x2b,%esi
   3d494:	call   *0x10ce76(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d49a:	movb   $0x2,0x750(%rsp)
   3d4a2:	lea    -0x2c01e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d4a9:	lea    0x1052f0(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d4b0:	lea    0x105031(%rip),%r8        # 1424e8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x458>
   3d4b7:	lea    0x750(%rsp),%rdx
   3d4bf:	mov    $0x2b,%esi
   3d4c4:	call   *0x10ce46(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d4ca:	movb   $0x2,0x750(%rsp)
   3d4d2:	lea    -0x2c04e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d4d9:	lea    0x1052c0(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d4e0:	lea    0x104e09(%rip),%r8        # 1422f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x260>
   3d4e7:	lea    0x750(%rsp),%rdx
   3d4ef:	mov    $0x2b,%esi
   3d4f4:	call   *0x10ce16(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d4fa:	mov    $0x4,%edi
   3d4ff:	mov    %r15,%rsi
   3d502:	call   *0x10cd78(%rip)        # 14a280 <_DYNAMIC+0x240>
   3d508:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d50d:	mov    %al,0x750(%rsp)
   3d514:	lea    -0x2c090(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d51b:	lea    0x10527e(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d522:	lea    0x104f77(%rip),%r8        # 1424a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x410>
   3d529:	lea    0x750(%rsp),%rdx
   3d531:	mov    $0x2b,%esi
   3d536:	call   *0x10cdd4(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d53c:	mov    %al,0x750(%rsp)
   3d543:	lea    -0x2c0bf(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d54a:	lea    0x10524f(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d551:	lea    0x104f60(%rip),%r8        # 1424b8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x428>
   3d558:	lea    0x750(%rsp),%rdx
   3d560:	mov    $0x2b,%esi
   3d565:	call   *0x10cda5(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d56b:	mov    $0x1,%r13b
   3d56e:	xor    %r15d,%r15d
   3d571:	mov    0xc0(%rsp),%rax
   3d579:	mov    %rax,0xb0(%rsp)
   3d581:	cmpq   $0x0,0x60(%rsp)
   3d587:	jne    3d3fb <meshopt_peers::run+0x34ab>
   3d58d:	jmp    3d40e <meshopt_peers::run+0x34be>
   3d592:	mov    $0x4,%edi
   3d597:	mov    %r15,%rsi
   3d59a:	call   *0x10cce0(%rip)        # 14a280 <_DYNAMIC+0x240>
   3d5a0:	mov    $0x4,%edi
   3d5a5:	mov    %r15,%rsi
   3d5a8:	call   *0x10ccd2(%rip)        # 14a280 <_DYNAMIC+0x240>
   3d5ae:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d5b3:	movb   $0x8,0x750(%rsp)
   3d5bb:	lea    -0x2c137(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d5c2:	lea    0x1051d7(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d5c9:	lea    0x104e70(%rip),%r8        # 142440 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3b0>
   3d5d0:	lea    0x750(%rsp),%rdx
   3d5d8:	mov    $0x2b,%esi
   3d5dd:	call   *0x10cd2d(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d5e3:	lea    0x104b6e(%rip),%rcx        # 142158 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xc8>
   3d5ea:	mov    %r15,%rsi
   3d5ed:	mov    0x70(%rsp),%rdx
   3d5f2:	call   *0x10cdb8(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   3d5f8:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d5fd:	mov    $0x4,%edi
   3d602:	mov    %rbp,%rsi
   3d605:	call   *0x10cc75(%rip)        # 14a280 <_DYNAMIC+0x240>
   3d60b:	movb   $0x8,0x750(%rsp)
   3d613:	lea    -0x2c18f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d61a:	lea    0x10517f(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d621:	lea    0x104d10(%rip),%r8        # 142338 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2a8>
   3d628:	lea    0x750(%rsp),%rdx
   3d630:	mov    $0x2b,%esi
   3d635:	call   *0x10ccd5(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d63b:	mov    $0x4,%edi
   3d640:	mov    %rbp,%rsi
   3d643:	call   *0x10cc37(%rip)        # 14a280 <_DYNAMIC+0x240>
   3d649:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d64e:	movb   $0x2,0x750(%rsp)
   3d656:	lea    -0x2c1d2(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d65d:	lea    0x10513c(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d664:	lea    0x104d2d(%rip),%r8        # 142398 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x308>
   3d66b:	lea    0x750(%rsp),%rdx
   3d673:	mov    $0x2b,%esi
   3d678:	call   *0x10cc92(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d67e:	xor    %r13d,%r13d
   3d681:	cmpq   $0x0,0x58(%rsp)
   3d687:	je     3d6a1 <meshopt_peers::run+0x3751>
   3d689:	mov    0xb8(%rsp),%rax
   3d691:	mov    %rax,0xb0(%rsp)
   3d699:	xor    %r15d,%r15d
   3d69c:	jmp    3d3fb <meshopt_peers::run+0x34ab>
   3d6a1:	movq   $0x0,0x58(%rsp)
   3d6aa:	xor    %r15d,%r15d
   3d6ad:	jmp    3d40e <meshopt_peers::run+0x34be>
   3d6b2:	mov    $0x8,%bpl
   3d6b5:	cmpq   $0x0,0x28(%rsp)
   3d6bb:	je     3d2cb <meshopt_peers::run+0x337b>
   3d6c1:	jmp    3d2c0 <meshopt_peers::run+0x3370>
   3d6c6:	lea    0x104a73(%rip),%rcx        # 142140 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xb0>
   3d6cd:	mov    0xd0(%rsp),%rdi
   3d6d5:	mov    0x68(%rsp),%rsi
   3d6da:	mov    %rbx,%rdx
   3d6dd:	call   *0x10cccd(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   3d6e3:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d6e8:	mov    %eax,%ebp
   3d6ea:	cmpq   $0x0,0x90(%rsp)
   3d6f3:	je     3d700 <meshopt_peers::run+0x37b0>
   3d6f5:	mov    0x20(%rsp),%rdi
   3d6fa:	call   *0x10cb48(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d700:	mov    0x750(%rsp),%rax
   3d708:	mov    0x760(%rsp),%rcx
   3d710:	sub    0x758(%rsp),%rcx
   3d718:	mov    %rcx,0x358(%rax)
   3d71f:	mov    %bpl,0x750(%rsp)
   3d727:	lea    -0x2c2a3(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d72e:	lea    0x10506b(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d735:	lea    0x104ddc(%rip),%r8        # 142518 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x488>
   3d73c:	lea    0x750(%rsp),%rdx
   3d744:	mov    $0x2b,%esi
   3d749:	call   *0x10cbc1(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3d74f:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d754:	lea    0x1049cd(%rip),%rdx        # 142128 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x98>
   3d75b:	mov    %r12,%rdi
   3d75e:	mov    0x48(%rsp),%rsi
   3d763:	call   *0x10cb27(%rip)        # 14a290 <_DYNAMIC+0x250>
   3d769:	jmp    3deb4 <meshopt_peers::run+0x3f64>
   3d76e:	mov    %rax,%rbx
   3d771:	cmpq   $0x0,0xf0(%rsp)
   3d77a:	je     3d8fe <meshopt_peers::run+0x39ae>
   3d780:	mov    0xf8(%rsp),%rdi
   3d788:	jmp    3d8f8 <meshopt_peers::run+0x39a8>
   3d78d:	mov    %rax,%rbx
   3d790:	cmpq   $0x0,0xf0(%rsp)
   3d799:	je     3d914 <meshopt_peers::run+0x39c4>
   3d79f:	mov    0xf8(%rsp),%r15
   3d7a7:	jmp    3d7d6 <meshopt_peers::run+0x3886>
   3d7a9:	jmp    3d7ab <meshopt_peers::run+0x385b>
   3d7ab:	mov    %rax,%rbx
   3d7ae:	cmpq   $0x0,0x28(%rsp)
   3d7b4:	je     3d845 <meshopt_peers::run+0x38f5>
   3d7ba:	jmp    3d896 <meshopt_peers::run+0x3946>
   3d7bf:	mov    %rax,%rbx
   3d7c2:	jmp    3d863 <meshopt_peers::run+0x3913>
   3d7c7:	mov    %rax,%rbx
   3d7ca:	cmpq   $0x0,0x18(%rsp)
   3d7d0:	je     3d914 <meshopt_peers::run+0x39c4>
   3d7d6:	mov    %r15,%rdi
   3d7d9:	jmp    3d90e <meshopt_peers::run+0x39be>
   3d7de:	mov    %rax,%rbx
   3d7e1:	jmp    3d8fe <meshopt_peers::run+0x39ae>
   3d7e6:	mov    %rax,%rbx
   3d7e9:	mov    %r12,0x80(%rsp)
   3d7f1:	mov    0x90(%rsp),%rax
   3d7f9:	mov    %rax,0x78(%rsp)
   3d7fe:	cmpq   $0x0,0x78(%rsp)
   3d804:	je     3d928 <meshopt_peers::run+0x39d8>
   3d80a:	jmp    3d9c9 <meshopt_peers::run+0x3a79>
   3d80f:	mov    %rax,%rbx
   3d812:	cmpq   $0x0,0x78(%rsp)
   3d818:	je     3d928 <meshopt_peers::run+0x39d8>
   3d81e:	jmp    3d9c9 <meshopt_peers::run+0x3a79>
   3d823:	mov    %r15,0x38(%rsp)
   3d828:	mov    %rax,%rbx
   3d82b:	jmp    3d914 <meshopt_peers::run+0x39c4>
   3d830:	mov    %rax,%rbx
   3d833:	test   %rbp,%rbp
   3d836:	jne    3d872 <meshopt_peers::run+0x3922>
   3d838:	test   %r15,%r15
   3d83b:	jne    3d885 <meshopt_peers::run+0x3935>
   3d83d:	cmpq   $0x0,0x28(%rsp)
   3d843:	jne    3d896 <meshopt_peers::run+0x3946>
   3d845:	cmpq   $0x0,0x58(%rsp)
   3d84b:	jne    3d8a9 <meshopt_peers::run+0x3959>
   3d84d:	cmpq   $0x0,0x60(%rsp)
   3d853:	je     3d863 <meshopt_peers::run+0x3913>
   3d855:	mov    0xc0(%rsp),%rdi
   3d85d:	call   *0x10c9e5(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d863:	lea    0xf0(%rsp),%rdi
   3d86b:	call   365a0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3d870:	jmp    3d8e5 <meshopt_peers::run+0x3995>
   3d872:	mov    0xb0(%rsp),%rdi
   3d87a:	call   *0x10c9c8(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d880:	test   %r15,%r15
   3d883:	je     3d83d <meshopt_peers::run+0x38ed>
   3d885:	mov    %r13,%rdi
   3d888:	call   *0x10c9ba(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d88e:	cmpq   $0x0,0x28(%rsp)
   3d894:	je     3d845 <meshopt_peers::run+0x38f5>
   3d896:	mov    0x50(%rsp),%rdi
   3d89b:	call   *0x10c9a7(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d8a1:	cmpq   $0x0,0x58(%rsp)
   3d8a7:	je     3d84d <meshopt_peers::run+0x38fd>
   3d8a9:	mov    0xb8(%rsp),%rdi
   3d8b1:	call   *0x10c991(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d8b7:	cmpq   $0x0,0x60(%rsp)
   3d8bd:	jne    3d855 <meshopt_peers::run+0x3905>
   3d8bf:	jmp    3d863 <meshopt_peers::run+0x3913>
   3d8c1:	jmp    3d8c3 <meshopt_peers::run+0x3973>
   3d8c3:	mov    %rax,%rbx
   3d8c6:	test   %r14,%r14
   3d8c9:	je     3d940 <meshopt_peers::run+0x39f0>
   3d8cb:	jmp    3d9f7 <meshopt_peers::run+0x3aa7>
   3d8d0:	jmp    3d9ad <meshopt_peers::run+0x3a5d>
   3d8d5:	jmp    3d9ad <meshopt_peers::run+0x3a5d>
   3d8da:	mov    %rax,%rbx
   3d8dd:	jmp    3da88 <meshopt_peers::run+0x3b38>
   3d8e2:	mov    %rax,%rbx
   3d8e5:	cmpq   $0x0,0x90(%rsp)
   3d8ee:	je     3d8fe <meshopt_peers::run+0x39ae>
   3d8f0:	mov    0xd8(%rsp),%rdi
   3d8f8:	call   *0x10c94a(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d8fe:	cmpq   $0x0,0x18(%rsp)
   3d904:	je     3d914 <meshopt_peers::run+0x39c4>
   3d906:	mov    0xc8(%rsp),%rdi
   3d90e:	call   *0x10c934(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d914:	cmpq   $0x0,0x38(%rsp)
   3d91a:	jne    3d954 <meshopt_peers::run+0x3a04>
   3d91c:	cmpq   $0x0,0x78(%rsp)
   3d922:	jne    3d9c9 <meshopt_peers::run+0x3a79>
   3d928:	cmpq   $0x0,0x80(%rsp)
   3d931:	jne    3d9e3 <meshopt_peers::run+0x3a93>
   3d937:	test   %r14,%r14
   3d93a:	jne    3d9f7 <meshopt_peers::run+0x3aa7>
   3d940:	cmpq   $0x0,0x88(%rsp)
   3d949:	jne    3da10 <meshopt_peers::run+0x3ac0>
   3d94f:	jmp    3da1e <meshopt_peers::run+0x3ace>
   3d954:	mov    0x150(%rsp),%rdi
   3d95c:	call   *0x10c8e6(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d962:	cmpq   $0x0,0x78(%rsp)
   3d968:	je     3d928 <meshopt_peers::run+0x39d8>
   3d96a:	jmp    3d9c9 <meshopt_peers::run+0x3a79>
   3d96c:	mov    %rax,%rbx
   3d96f:	cmpq   $0x0,0x88(%rsp)
   3d978:	jne    3da10 <meshopt_peers::run+0x3ac0>
   3d97e:	jmp    3da1e <meshopt_peers::run+0x3ace>
   3d983:	jmp    3d98d <meshopt_peers::run+0x3a3d>
   3d985:	mov    %rax,%rbx
   3d988:	jmp    3da1e <meshopt_peers::run+0x3ace>
   3d98d:	mov    %rax,%rbx
   3d990:	lea    0x310(%rsp),%rdi
   3d998:	call   36b50 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   3d99d:	mov    %rbx,%rdi
   3d9a0:	call   140620 <_Unwind_Resume@plt>
   3d9a5:	mov    %rax,%rbx
   3d9a8:	jmp    3da3d <meshopt_peers::run+0x3aed>
   3d9ad:	mov    %rax,%rbx
   3d9b0:	lea    0x310(%rsp),%rdi
   3d9b8:	call   36b90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3d9bd:	cmpq   $0x0,0x78(%rsp)
   3d9c3:	je     3d928 <meshopt_peers::run+0x39d8>
   3d9c9:	mov    0x20(%rsp),%rdi
   3d9ce:	call   *0x10c874(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d9d4:	cmpq   $0x0,0x80(%rsp)
   3d9dd:	je     3d937 <meshopt_peers::run+0x39e7>
   3d9e3:	mov    0x30(%rsp),%rdi
   3d9e8:	call   *0x10c85a(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3d9ee:	test   %r14,%r14
   3d9f1:	je     3d940 <meshopt_peers::run+0x39f0>
   3d9f7:	mov    0xa0(%rsp),%rdi
   3d9ff:	call   *0x10c843(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da05:	cmpq   $0x0,0x88(%rsp)
   3da0e:	je     3da1e <meshopt_peers::run+0x3ace>
   3da10:	mov    0xe8(%rsp),%rdi
   3da18:	call   *0x10c82a(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da1e:	mov    0x750(%rsp),%rax
   3da26:	mov    0x760(%rsp),%rcx
   3da2e:	sub    0x758(%rsp),%rcx
   3da36:	mov    %rcx,0x358(%rax)
   3da3d:	cmpq   $0x0,0x1b0(%rsp)
   3da46:	je     3da56 <meshopt_peers::run+0x3b06>
   3da48:	mov    0x1b8(%rsp),%rdi
   3da50:	call   *0x10c7f2(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da56:	test   %r14,%r14
   3da59:	je     3dec4 <meshopt_peers::run+0x3f74>
   3da5f:	mov    0x8(%rsp),%rdi
   3da64:	call   *0x10c7de(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da6a:	mov    %rbx,%rdi
   3da6d:	call   140620 <_Unwind_Resume@plt>
   3da72:	mov    %r12,0x8(%rsp)
   3da77:	mov    %rax,%rbx
   3da7a:	test   %r14,%r14
   3da7d:	je     3da88 <meshopt_peers::run+0x3b38>
   3da7f:	mov    %r15,%rdi
   3da82:	call   *0x10c7c0(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da88:	cmpq   $0x0,(%rsp)
   3da8d:	je     3dec4 <meshopt_peers::run+0x3f74>
   3da93:	mov    0x8(%rsp),%rdi
   3da98:	call   *0x10c7aa(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3da9e:	mov    %rbx,%rdi
   3daa1:	call   140620 <_Unwind_Resume@plt>
   3daa6:	mov    %edi,%r8d
   3daa9:	mov    0x4(%rbp,%rsi,4),%edi
   3daad:	cmp    %edi,%r8d
   3dab0:	cmova  %r8d,%edi
   3dab4:	inc    %rsi
   3dab7:	cmp    %rsi,%rdx
   3daba:	jne    3daa6 <meshopt_peers::run+0x3b56>
   3dabc:	mov    %edi,%edx
   3dabe:	cmp    %rdx,%r14
   3dac1:	jbe    3db8a <meshopt_peers::run+0x3c3a>
   3dac7:	mov    %rcx,0x260(%rsp)
   3dacf:	mov    %r14,%rax
   3dad2:	mov    %rax,%rdx
   3dad5:	shr    $0x2,%rdx
   3dad9:	add    %rax,%rdx
   3dadc:	jb     3db23 <meshopt_peers::run+0x3bd3>
   3dade:	cmp    $0x1,%rdx
   3dae2:	mov    %rdx,%rcx
   3dae5:	adc    $0xffffffffffffffff,%rcx
   3dae9:	bsr    %rcx,%rcx
   3daed:	not    %ecx
   3daef:	mov    $0xffffffffffffffff,%rsi
   3daf6:	shr    %cl,%rsi
   3daf9:	xor    %r15d,%r15d
   3dafc:	cmp    $0x2,%rdx
   3db00:	cmovae %rsi,%r15
   3db04:	cmp    $0xffffffffffffffff,%r15
   3db08:	je     3db23 <meshopt_peers::run+0x3bd3>
   3db0a:	lea    0x0(,%rax,4),%rcx
   3db12:	shr    $0x3e,%rax
   3db16:	sete   %al
   3db19:	test   %rcx,%rcx
   3db1c:	setns  %dl
   3db1f:	test   %dl,%al
   3db21:	jne    3db30 <meshopt_peers::run+0x3be0>
   3db23:	movb   $0x8,0x318(%rsp)
   3db2b:	jmp    3dbb8 <meshopt_peers::run+0x3c68>
   3db30:	lea    0x1(%r15),%rbp
   3db34:	mov    %rbp,0x6e0(%rsp)
   3db3c:	pxor   %xmm0,%xmm0
   3db40:	movdqu %xmm0,0x6e8(%rsp)
   3db49:	movq   $0x0,0x6f8(%rsp)
   3db55:	lea    0x6e0(%rsp),%rsi
   3db5d:	mov    %r11,%r12
   3db60:	mov    %r11,%rdi
   3db63:	xor    %edx,%edx
   3db65:	call   a3240 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3db6a:	cmp    $0xff,%al
   3db6c:	je     3dc28 <meshopt_peers::run+0x3cd8>
   3db72:	mov    %al,0x318(%rsp)
   3db79:	movq   $0xffffffffffffffff,0x310(%rsp)
   3db85:	mov    %r12,%r11
   3db88:	jmp    3dbc4 <meshopt_peers::run+0x3c74>
   3db8a:	shl    $0x2,%r15
   3db8e:	xor    %edx,%edx
   3db90:	cmp    %rdx,%r15
   3db93:	je     3dac7 <meshopt_peers::run+0x3b77>
   3db99:	dec    %rax
   3db9c:	cmp    %r14d,0x0(%rbp,%rdx,1)
   3dba1:	lea    0x4(%rdx),%rdx
   3dba5:	jb     3db90 <meshopt_peers::run+0x3c40>
   3dba7:	mov    %rax,0x260(%rsp)
   3dbaf:	mov    $0x1,%al
   3dbb1:	mov    %al,0x318(%rsp)
   3dbb8:	movq   $0xffffffffffffffff,0x310(%rsp)
   3dbc4:	mov    0x268(%rsp),%rax
   3dbcc:	sub    0x260(%rsp),%rax
   3dbd4:	mov    %rax,0x358(%r11)
   3dbdb:	cmpq   $0xffffffffffffffff,0x310(%rsp)
   3dbe4:	je     3de62 <meshopt_peers::run+0x3f12>
   3dbea:	mov    0x320(%rsp),%rax
   3dbf2:	mov    %rax,0x20(%rbx)
   3dbf6:	movdqu 0x310(%rsp),%xmm0
   3dbff:	movdqu %xmm0,0x10(%rbx)
   3dc04:	mov    0x328(%rsp),%rax
   3dc0c:	mov    %rax,0x8(%rbx)
   3dc10:	movl   $0x0,(%rbx)
   3dc16:	add    $0x1988,%rsp
   3dc1d:	pop    %rbx
   3dc1e:	pop    %r12
   3dc20:	pop    %r13
   3dc22:	pop    %r14
   3dc24:	pop    %r15
   3dc26:	pop    %rbp
   3dc27:	ret
   3dc28:	mov    0x1c8(%rsp),%rsi
   3dc30:	lea    0x310(%rsp),%rdi
   3dc38:	call   *0x10ca6a(%rip)        # 14a6a8 <_DYNAMIC+0x668>
   3dc3e:	mov    0x310(%rsp),%rsi
   3dc46:	movzbl 0x318(%rsp),%eax
   3dc4e:	cmp    $0xffffffffffffffff,%rsi
   3dc52:	je     3db72 <meshopt_peers::run+0x3c22>
   3dc58:	movzbl %al,%eax
   3dc5b:	movzbl 0x31f(%rsp),%ecx
   3dc63:	shl    $0x10,%ecx
   3dc66:	movzwl 0x31d(%rsp),%edx
   3dc6e:	or     %ecx,%edx
   3dc70:	mov    0x319(%rsp),%ecx
   3dc77:	mov    0x320(%rsp),%rdi
   3dc7f:	mov    %rdi,0x8(%rsp)
   3dc84:	shl    $0x28,%rdx
   3dc88:	shl    $0x8,%rcx
   3dc8c:	or     %rdx,%rcx
   3dc8f:	or     %rax,%rcx
   3dc92:	mov    %rcx,(%rsp)
   3dc96:	cmp    0x1c8(%rsp),%rsi
   3dc9e:	mov    %r12,%r11
   3dca1:	jne    3dd16 <meshopt_peers::run+0x3dc6>
   3dca3:	mov    %rsi,0x88(%rsp)
   3dcab:	lea    0x788(%rsp),%rax
   3dcb3:	cmpb   $0x0,0x230(%rsp)
   3dcbb:	je     3dd88 <meshopt_peers::run+0x3e38>
   3dcc1:	mov    0x28(%r11),%rdx
   3dcc5:	cmp    %rdx,%r15
   3dcc8:	jae    3de99 <meshopt_peers::run+0x3f49>
   3dcce:	mov    0x20(%r11),%r9
   3dcd2:	mov    0xf0(%rsp),%rcx
   3dcda:	mov    0xf8(%rsp),%r8
   3dce2:	sub    $0x8,%rsp
   3dce6:	lea    0x318(%rsp),%rdi
   3dcee:	mov    0x8(%rsp),%rsi
   3dcf3:	mov    0x10(%rsp),%rdx
   3dcf8:	lea    0x268(%rsp),%r10
   3dd00:	push   %r10
   3dd02:	push   %rax
   3dd03:	push   %r14
   3dd05:	push   %r13
   3dd07:	push   %rbp
   3dd08:	call   38bd0 <meshoptimizer_rs::remap::generate_kernel::<true, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3dd0d:	add    $0x30,%rsp
   3dd11:	jmp    3ddd8 <meshopt_peers::run+0x3e88>
   3dd16:	movabs $0x3fffffffffffffff,%rax
   3dd20:	lea    0x0(,%rsi,4),%rcx
   3dd28:	cmp    %rax,%rsi
   3dd2b:	seta   %al
   3dd2e:	test   %rcx,%rcx
   3dd31:	sets   %dl
   3dd34:	or     %al,%dl
   3dd36:	mov    $0x8,%al
   3dd38:	jne    3dd67 <meshopt_peers::run+0x3e17>
   3dd3a:	mov    %rsi,0x88(%rsp)
   3dd42:	lea    0x6e0(%rsp),%rsi
   3dd4a:	mov    %r11,%rdi
   3dd4d:	xor    %edx,%edx
   3dd4f:	call   a3240 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3dd54:	cmp    $0xff,%al
   3dd56:	mov    %r12,%r11
   3dd59:	mov    0x88(%rsp),%rsi
   3dd61:	je     3dca3 <meshopt_peers::run+0x3d53>
   3dd67:	mov    %al,0x318(%rsp)
   3dd6e:	movq   $0xffffffffffffffff,0x310(%rsp)
   3dd7a:	test   %rsi,%rsi
   3dd7d:	je     3dbc4 <meshopt_peers::run+0x3c74>
   3dd83:	jmp    3de0f <meshopt_peers::run+0x3ebf>
   3dd88:	mov    0x28(%r11),%rdx
   3dd8c:	cmp    %rdx,%r15
   3dd8f:	jae    3dea2 <meshopt_peers::run+0x3f52>
   3dd95:	mov    0x20(%r11),%r9
   3dd99:	mov    0xf0(%rsp),%rcx
   3dda1:	mov    0xf8(%rsp),%r8
   3dda9:	sub    $0x8,%rsp
   3ddad:	lea    0x318(%rsp),%rdi
   3ddb5:	mov    0x8(%rsp),%rsi
   3ddba:	mov    0x10(%rsp),%rdx
   3ddbf:	lea    0x268(%rsp),%r10
   3ddc7:	push   %r10
   3ddc9:	push   %rax
   3ddca:	push   %r14
   3ddcc:	push   %r13
   3ddce:	push   %rbp
   3ddcf:	call   37960 <meshoptimizer_rs::remap::generate_kernel::<false, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3ddd4:	add    $0x30,%rsp
   3ddd8:	cmpb   $0x1,0x310(%rsp)
   3dde0:	jne    3de21 <meshopt_peers::run+0x3ed1>
   3dde2:	movzbl 0x311(%rsp),%eax
   3ddea:	mov    %r12,%r11
   3dded:	mov    %al,0x318(%rsp)
   3ddf4:	movq   $0xffffffffffffffff,0x310(%rsp)
   3de00:	cmpq   $0x0,0x88(%rsp)
   3de09:	je     3dbc4 <meshopt_peers::run+0x3c74>
   3de0f:	mov    (%rsp),%rdi
   3de13:	call   *0x10c42f(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3de19:	mov    %r12,%r11
   3de1c:	jmp    3dbc4 <meshopt_peers::run+0x3c74>
   3de21:	mov    0x318(%rsp),%rax
   3de29:	mov    %r12,%r11
   3de2c:	mov    0x88(%rsp),%rcx
   3de34:	mov    %rcx,0x310(%rsp)
   3de3c:	mov    (%rsp),%rcx
   3de40:	mov    %rcx,0x318(%rsp)
   3de48:	mov    0x8(%rsp),%rcx
   3de4d:	mov    %rcx,0x320(%rsp)
   3de55:	mov    %rax,0x328(%rsp)
   3de5d:	jmp    3dbc4 <meshopt_peers::run+0x3c74>
   3de62:	movzbl 0x318(%rsp),%eax
   3de6a:	mov    %al,0x750(%rsp)
   3de71:	lea    -0x2c9ed(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3de78:	lea    0x104921(%rip),%rcx        # 1427a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3de7f:	lea    0x10452a(%rip),%r8        # 1423b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x320>
   3de86:	lea    0x750(%rsp),%rdx
   3de8e:	mov    $0x2b,%esi
   3de93:	call   *0x10c477(%rip)        # 14a310 <_DYNAMIC+0x2d0>
   3de99:	lea    0x104390(%rip),%rcx        # 142230 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x1a0>
   3dea0:	jmp    3dea9 <meshopt_peers::run+0x3f59>
   3dea2:	lea    0x10436f(%rip),%rcx        # 142218 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x188>
   3dea9:	xor    %edi,%edi
   3deab:	mov    %rbp,%rsi
   3deae:	call   *0x10c4fc(%rip)        # 14a3b0 <_DYNAMIC+0x370>
   3deb4:	ud2
   3deb6:	mov    %rax,%rbx
   3deb9:	cmpq   $0x0,0x88(%rsp)
   3dec2:	jne    3decc <meshopt_peers::run+0x3f7c>
   3dec4:	mov    %rbx,%rdi
   3dec7:	call   140620 <_Unwind_Resume@plt>
   3decc:	mov    (%rsp),%rdi
   3ded0:	call   *0x10c372(%rip)        # 14a248 <free@GLIBC_2.2.5>
   3ded6:	mov    %rbx,%rdi
   3ded9:	call   140620 <_Unwind_Resume@plt>
