
<artifacts>/guard/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

000000000003a160 <meshopt_peers::run>:
   3a160:	push   %rbp
   3a161:	push   %r15
   3a163:	push   %r14
   3a165:	push   %r13
   3a167:	push   %r12
   3a169:	push   %rbx
   3a16a:	sub    $0x1000,%rsp
   3a171:	movq   $0x0,(%rsp)
   3a179:	sub    $0x988,%rsp
   3a180:	mov    %rdi,%rbx
   3a183:	mov    %rsi,0x6d0(%rsp)
   3a18b:	mov    %rdx,0x6d8(%rsp)
   3a193:	mov    0x28(%rcx),%r14
   3a197:	mov    0x20(%rcx),%r13
   3a19b:	movb   $0x2,0x688(%rsp)
   3a1a3:	mov    %r13,0x690(%rsp)
   3a1ab:	mov    %r14,0x698(%rsp)
   3a1b3:	movq   $0x0,0x6c0(%rsp)
   3a1bf:	mov    0x98(%rcx),%rax
   3a1c6:	movd   0xa0(%rcx),%xmm0
   3a1ce:	mov    %rax,0x220(%rsp)
   3a1d6:	movd   %xmm0,0x228(%rsp)
   3a1df:	movl   $0x0,0x22c(%rsp)
   3a1ea:	lea    -0x5(%rdx),%rdi
   3a1ee:	cmp    $0xe,%rdi
   3a1f2:	ja     3ab90 <meshopt_peers::run+0xa30>
   3a1f8:	mov    %r8,%r11
   3a1fb:	lea    -0x28e36(%rip),%r8        # 113cc <libm::math::generic::sqrt::RSQRT_TAB+0x122>
   3a202:	movslq (%r8,%rdi,4),%rdi
   3a206:	add    %r8,%rdi
   3a209:	jmp    *%rdi
   3a20b:	mov    $0x68636163,%eax
   3a210:	xor    (%rsi),%eax
   3a212:	movzbl 0x4(%rsi),%edi
   3a216:	xor    $0x65,%edi
   3a219:	or     %eax,%edi
   3a21b:	je     3aed8 <meshopt_peers::run+0xd78>
   3a221:	mov    $0x63746566,%eax
   3a226:	xor    (%rsi),%eax
   3a228:	movzbl 0x4(%rsi),%edi
   3a22c:	xor    $0x68,%edi
   3a22f:	or     %eax,%edi
   3a231:	je     3b429 <meshopt_peers::run+0x12c9>
   3a237:	mov    $0x616d6572,%eax
   3a23c:	xor    (%rsi),%eax
   3a23e:	movzbl 0x4(%rsi),%edi
   3a242:	xor    $0x70,%edi
   3a245:	or     %eax,%edi
   3a247:	je     3abcb <meshopt_peers::run+0xa6b>
   3a24d:	jmp    3ab90 <meshopt_peers::run+0xa30>
   3a252:	movabs $0x7966696c706d6973,%rax
   3a25c:	cmp    %rax,(%rsi)
   3a25f:	je     3adbb <meshopt_peers::run+0xc5b>
   3a265:	movabs $0x776172647265766f,%rax
   3a26f:	cmp    %rax,(%rsi)
   3a272:	je     3b352 <meshopt_peers::run+0x11f2>
   3a278:	movabs $0x7374656c6873656d,%rax
   3a282:	cmp    %rax,(%rsi)
   3a285:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a28b:	mov    0x38(%rcx),%rsi
   3a28f:	mov    0x40(%rcx),%rdx
   3a293:	movq   $0x7c,0x320(%rsp)
   3a29f:	movq   $0x0,0x314(%rsp)
   3a2ab:	movl   $0x1,0x310(%rsp)
   3a2b6:	lea    0x310(%rsp),%rax
   3a2be:	lea    0x750(%rsp),%rdi
   3a2c6:	lea    0x688(%rsp),%r8
   3a2ce:	mov    $0x40,%r9d
   3a2d4:	mov    %r14,%rcx
   3a2d7:	push   $0x2
   3a2d9:	push   %r11
   3a2db:	push   %rax
   3a2dc:	push   $0x7c
   3a2de:	call   c9df0 <meshoptimizer_rs::meshlet::build>
   3a2e3:	add    $0x20,%rsp
   3a2e7:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a2f0:	je     3d254 <meshopt_peers::run+0x30f4>
   3a2f6:	mov    0x790(%rsp),%rax
   3a2fe:	mov    %rax,0x350(%rsp)
   3a306:	movdqu 0x750(%rsp),%xmm0
   3a30f:	movups 0x760(%rsp),%xmm1
   3a317:	movdqu 0x770(%rsp),%xmm2
   3a320:	movdqu 0x780(%rsp),%xmm3
   3a329:	movdqa %xmm3,0x340(%rsp)
   3a332:	movdqa %xmm2,0x330(%rsp)
   3a33b:	movaps %xmm1,0x320(%rsp)
   3a343:	movdqa %xmm0,0x310(%rsp)
   3a34c:	mov    0x318(%rsp),%r14
   3a354:	mov    0x320(%rsp),%r12
   3a35c:	test   %r12,%r12
   3a35f:	je     3bb14 <meshopt_peers::run+0x19b4>
   3a365:	mov    %r12,%r15
   3a368:	shl    $0x4,%r15
   3a36c:	mov    %r15,%rdi
   3a36f:	call   *0x112263(%rip)        # 14c5d8 <malloc@GLIBC_2.2.5>
   3a375:	test   %rax,%rax
   3a378:	je     3d7b0 <meshopt_peers::run+0x3650>
   3a37e:	mov    %r12d,%ecx
   3a381:	and    $0x3,%ecx
   3a384:	cmp    $0x4,%r12
   3a388:	jae    3bb86 <meshopt_peers::run+0x1a26>
   3a38e:	xor    %edx,%edx
   3a390:	jmp    3bbde <meshopt_peers::run+0x1a7e>
   3a395:	movabs $0x7374656c6873656d,%rax
   3a39f:	xor    (%rsi),%rax
   3a3a2:	movabs $0x78656c662d737465,%rdx
   3a3ac:	xor    0x5(%rsi),%rdx
   3a3b0:	or     %rax,%rdx
   3a3b3:	je     3ae54 <meshopt_peers::run+0xcf4>
   3a3b9:	movabs $0x65646f6365642d78,%rax
   3a3c3:	movabs $0x652d786574726576,%rdx
   3a3cd:	xor    (%rsi),%rdx
   3a3d0:	lea    0x9010000(%rax),%rdi
   3a3d7:	xor    0x5(%rsi),%rdi
   3a3db:	or     %rdx,%rdi
   3a3de:	je     3b3c1 <meshopt_peers::run+0x1261>
   3a3e4:	movabs $0x642d786574726576,%rdx
   3a3ee:	xor    (%rsi),%rdx
   3a3f1:	xor    0x5(%rsi),%rax
   3a3f5:	or     %rdx,%rax
   3a3f8:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a3fe:	mov    0x68(%rcx),%r9
   3a402:	lea    0x750(%rsp),%rdi
   3a40a:	mov    $0xc,%r8d
   3a410:	xor    %esi,%esi
   3a412:	xor    %edx,%edx
   3a414:	mov    %rcx,%rax
   3a417:	mov    %r14,%rcx
   3a41a:	push   %r11
   3a41c:	push   0x70(%rax)
   3a41f:	call   c0810 <meshoptimizer_rs::codec::allocate>
   3a424:	add    $0x10,%rsp
   3a428:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a431:	jne    3b404 <meshopt_peers::run+0x12a4>
   3a437:	movzbl 0x758(%rsp),%eax
   3a43f:	mov    %al,0x310(%rsp)
   3a446:	lea    -0x28fc2(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a44d:	lea    0x10a6ec(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a454:	lea    0x10a325(%rip),%r8        # 144780 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x350>
   3a45b:	lea    0x310(%rsp),%rdx
   3a463:	mov    $0x2b,%esi
   3a468:	call   *0x112242(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3a46e:	mov    $0x64616873,%eax
   3a473:	xor    (%rsi),%eax
   3a475:	movzwl 0x4(%rsi),%edi
   3a479:	xor    $0x776f,%edi
   3a47f:	or     %eax,%edi
   3a481:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a487:	mov    %r14,%rax
   3a48a:	shr    $0x20,%rax
   3a48e:	jne    3d64a <meshopt_peers::run+0x34ea>
   3a494:	lea    0x0(,%r14,4),%rax
   3a49c:	lea    (%rax,%rax,2),%rax
   3a4a0:	mov    %r13,0x310(%rsp)
   3a4a8:	mov    %rax,0x318(%rsp)
   3a4b0:	mov    %r14,0x320(%rsp)
   3a4b8:	movq   $0xc,0x328(%rsp)
   3a4c4:	movq   $0xc,0x330(%rsp)
   3a4d0:	movq   $0x0,0x338(%rsp)
   3a4dc:	cmp    $0x6,%rdx
   3a4e0:	jne    3a4fb <meshopt_peers::run+0x39b>
   3a4e2:	mov    $0x64616873,%eax
   3a4e7:	xor    (%rsi),%eax
   3a4e9:	movzwl 0x4(%rsi),%edx
   3a4ed:	xor    $0x776f,%edx
   3a4f3:	or     %eax,%edx
   3a4f5:	je     3b6f1 <meshopt_peers::run+0x1591>
   3a4fb:	test   %r14,%r14
   3a4fe:	setne  %dl
   3a501:	mov    0x58(%rcx),%rax
   3a505:	shl    $0x2,%rax
   3a509:	mov    %r14,%rsi
   3a50c:	shl    $0x4,%rsi
   3a510:	cmp    %rax,%rsi
   3a513:	seta   %sil
   3a517:	and    %dl,%sil
   3a51a:	cmp    $0x1,%sil
   3a51e:	je     3d6da <meshopt_peers::run+0x357a>
   3a524:	mov    0x38(%rcx),%rsi
   3a528:	mov    0x40(%rcx),%rdx
   3a52c:	mov    0x50(%rcx),%rcx
   3a530:	movdqu 0x310(%rsp),%xmm0
   3a539:	movups 0x320(%rsp),%xmm1
   3a541:	movdqu 0x330(%rsp),%xmm2
   3a54a:	movdqa %xmm2,0x770(%rsp)
   3a553:	movaps %xmm1,0x760(%rsp)
   3a55b:	movdqa %xmm0,0x750(%rsp)
   3a564:	mov    %rcx,0x780(%rsp)
   3a56c:	mov    %rax,0x788(%rsp)
   3a574:	mov    %r14,0x790(%rsp)
   3a57c:	movq   $0x10,0x798(%rsp)
   3a588:	movq   $0x10,0x7a0(%rsp)
   3a594:	movq   $0x0,0x7a8(%rsp)
   3a5a0:	sub    $0x10,%rsp
   3a5a4:	mov    %r11,0x8(%rsp)
   3a5a9:	lea    0x270(%rsp),%rdi
   3a5b1:	lea    0x760(%rsp),%rcx
   3a5b9:	mov    $0x2,%r8d
   3a5bf:	xor    %r9d,%r9d
   3a5c2:	call   684e0 <meshoptimizer_rs::remap::shadow_filter::<false>>
   3a5c7:	add    $0x10,%rsp
   3a5cb:	mov    0x260(%rsp),%rax
   3a5d3:	movzbl 0x268(%rsp),%ecx
   3a5db:	cmp    $0xffffffffffffffff,%rax
   3a5df:	je     3a601 <meshopt_peers::run+0x4a1>
   3a5e1:	mov    0x269(%rsp),%rdx
   3a5e9:	mov    0x270(%rsp),%rsi
   3a5f1:	mov    %rsi,0x100(%rsp)
   3a5f9:	mov    %rdx,0xf9(%rsp)
   3a601:	mov    %cl,0xf8(%rsp)
   3a608:	mov    %rax,0xf0(%rsp)
   3a610:	cmp    $0xffffffffffffffff,%rax
   3a614:	je     3b724 <meshopt_peers::run+0x15c4>
   3a61a:	mov    0x100(%rsp),%rax
   3a622:	mov    %rax,0x18(%rbx)
   3a626:	movdqu 0xf0(%rsp),%xmm0
   3a62f:	jmp    3b6e1 <meshopt_peers::run+0x1581>
   3a634:	movabs $0x7966696c706d6973,%rax
   3a63e:	xor    (%rsi),%rax
   3a641:	movabs $0x656e7572702d7966,%rdx
   3a64b:	xor    0x6(%rsi),%rdx
   3a64f:	or     %rax,%rdx
   3a652:	je     3af3a <meshopt_peers::run+0xdda>
   3a658:	movabs $0x6e6f697469736f70,%rax
   3a662:	xor    (%rsi),%rax
   3a665:	movabs $0x70616d65722d6e6f,%rcx
   3a66f:	xor    0x6(%rsi),%rcx
   3a673:	or     %rax,%rcx
   3a676:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a67c:	lea    0x750(%rsp),%rdi
   3a684:	lea    0x688(%rsp),%rsi
   3a68c:	mov    %r11,%rdx
   3a68f:	call   *0x11231b(%rip)        # 14c9b0 <_DYNAMIC+0x5d0>
   3a695:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a69e:	jne    3b6cc <meshopt_peers::run+0x156c>
   3a6a4:	movzbl 0x758(%rsp),%eax
   3a6ac:	mov    %al,0x310(%rsp)
   3a6b3:	lea    -0x2922f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a6ba:	lea    0x10a47f(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a6c1:	lea    0x10a040(%rip),%r8        # 144708 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2d8>
   3a6c8:	lea    0x310(%rsp),%rdx
   3a6d0:	mov    $0x2b,%esi
   3a6d5:	call   *0x111fd5(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3a6db:	movabs $0x7966696c706d6973,%rdx
   3a6e5:	mov    (%rsi),%rdi
   3a6e8:	xor    %rdx,%rdi
   3a6eb:	movabs $0x7970706f6c732d79,%r8
   3a6f5:	xor    0x7(%rsi),%r8
   3a6f9:	or     %rdi,%r8
   3a6fc:	je     3afb0 <meshopt_peers::run+0xe50>
   3a702:	mov    (%rsi),%rax
   3a705:	xor    %rdx,%rax
   3a708:	movabs $0x6574616470752d79,%rdi
   3a712:	xor    0x7(%rsi),%rdi
   3a716:	or     %rax,%rdi
   3a719:	je     3b525 <meshopt_peers::run+0x13c5>
   3a71f:	xor    (%rsi),%rdx
   3a722:	movabs $0x73746e696f702d79,%rax
   3a72c:	xor    0x7(%rsi),%rax
   3a730:	or     %rdx,%rax
   3a733:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a739:	movb   $0xff,0x750(%rsp)
   3a741:	shr    $1,%r14
   3a744:	sub    $0x8,%rsp
   3a748:	lea    0x318(%rsp),%rdi
   3a750:	lea    0x690(%rsp),%rsi
   3a758:	lea    0x758(%rsp),%rdx
   3a760:	pxor   %xmm0,%xmm0
   3a764:	mov    %r14,%rcx
   3a767:	xor    %r8d,%r8d
   3a76a:	push   %r11
   3a76c:	call   f3fd0 <meshoptimizer_rs::simplify::extra::simplify_points_kernel>
   3a771:	add    $0x10,%rsp
   3a775:	mov    0x310(%rsp),%rcx
   3a77d:	movzbl 0x318(%rsp),%eax
   3a785:	cmp    $0xffffffffffffffff,%rcx
   3a789:	je     3d71d <meshopt_peers::run+0x35bd>
   3a78f:	mov    0x319(%rsp),%rdx
   3a797:	mov    0x320(%rsp),%rsi
   3a79f:	jmp    3af96 <meshopt_peers::run+0xe36>
   3a7a4:	movabs $0x756d2d70616d6572,%rax
   3a7ae:	xor    (%rsi),%rax
   3a7b1:	movabs $0x69746c756d2d7061,%rdi
   3a7bb:	xor    0x3(%rsi),%rdi
   3a7bf:	or     %rax,%rdi
   3a7c2:	je     3abcb <meshopt_peers::run+0xa6b>
   3a7c8:	movabs $0x6e692d70616d6572,%rax
   3a7d2:	xor    (%rsi),%rax
   3a7d5:	movabs $0x7865646e692d7061,%rdx
   3a7df:	xor    0x3(%rsi),%rdx
   3a7e3:	or     %rax,%rdx
   3a7e6:	je     3b699 <meshopt_peers::run+0x1539>
   3a7ec:	movabs $0x2d72657473756c63,%rax
   3a7f6:	xor    (%rsi),%rax
   3a7f9:	movabs $0x646f6c2d72657473,%rdx
   3a803:	xor    0x3(%rsi),%rdx
   3a807:	or     %rax,%rdx
   3a80a:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a810:	mov    %r14,%rax
   3a813:	mov    %rbx,(%rsp)
   3a817:	test   %r14,%r14
   3a81a:	je     3b80b <meshopt_peers::run+0x16ab>
   3a820:	mov    %r11,%rbp
   3a823:	mov    %rcx,%rbx
   3a826:	mov    %rax,%r14
   3a829:	lea    0x0(,%rax,4),%rax
   3a831:	lea    (%rax,%rax,2),%r15
   3a835:	mov    %r15,%rdi
   3a838:	call   *0x111d9a(%rip)        # 14c5d8 <malloc@GLIBC_2.2.5>
   3a83e:	test   %rax,%rax
   3a841:	je     3d7a2 <meshopt_peers::run+0x3642>
   3a847:	mov    %rax,%r12
   3a84a:	mov    %rax,%rdi
   3a84d:	mov    %r13,%rsi
   3a850:	mov    %r15,%rdx
   3a853:	call   *0x111d9f(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   3a859:	mov    %rbx,%rcx
   3a85c:	mov    %rbp,%r11
   3a85f:	jmp    3b814 <meshopt_peers::run+0x16b4>
   3a864:	movabs $0x6e652d7865646e69,%rax
   3a86e:	xor    (%rsi),%rax
   3a871:	mov    0x8(%rsi),%edi
   3a874:	xor    $0x65646f63,%rdi
   3a87b:	or     %rax,%rdi
   3a87e:	je     3b12d <meshopt_peers::run+0xfcd>
   3a884:	movabs $0x75632d70616d6572,%rax
   3a88e:	xor    (%rsi),%rax
   3a891:	mov    0x8(%rsi),%edi
   3a894:	xor    $0x6d6f7473,%rdi
   3a89b:	or     %rax,%rdi
   3a89e:	je     3abcb <meshopt_peers::run+0xa6b>
   3a8a4:	movabs $0x65762d70616d6572,%rax
   3a8ae:	xor    (%rsi),%rax
   3a8b1:	mov    0x8(%rsi),%edi
   3a8b4:	xor    $0x78657472,%rdi
   3a8bb:	or     %rax,%rdi
   3a8be:	je     3b76c <meshopt_peers::run+0x160c>
   3a8c4:	movabs $0x6d2d776f64616873,%rax
   3a8ce:	xor    (%rsi),%rax
   3a8d1:	mov    0x8(%rsi),%edi
   3a8d4:	xor    $0x69746c75,%rdi
   3a8db:	or     %rax,%rdi
   3a8de:	je     3a487 <meshopt_peers::run+0x327>
   3a8e4:	movabs $0x65642d7865646e69,%rax
   3a8ee:	xor    (%rsi),%rax
   3a8f1:	mov    0x8(%rsi),%edx
   3a8f4:	xor    $0x65646f63,%rdx
   3a8fb:	or     %rax,%rdx
   3a8fe:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a904:	mov    0x40(%rcx),%rax
   3a908:	mov    0x80(%rcx),%r9
   3a90f:	lea    0x750(%rsp),%rdi
   3a917:	mov    $0x4,%r8d
   3a91d:	mov    $0x1,%esi
   3a922:	xor    %edx,%edx
   3a924:	mov    %rcx,%r10
   3a927:	mov    %rax,%rcx
   3a92a:	push   %r11
   3a92c:	push   0x88(%r10)
   3a933:	call   c0810 <meshoptimizer_rs::codec::allocate>
   3a938:	add    $0x10,%rsp
   3a93c:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a945:	jne    3b404 <meshopt_peers::run+0x12a4>
   3a94b:	movzbl 0x758(%rsp),%eax
   3a953:	mov    %al,0x310(%rsp)
   3a95a:	lea    -0x294d6(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a961:	lea    0x10a1d8(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3a968:	lea    0x109cf1(%rip),%r8        # 144660 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x230>
   3a96f:	lea    0x310(%rsp),%rdx
   3a977:	mov    $0x2b,%esi
   3a97c:	call   *0x111d2e(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3a982:	movdqu (%rsi),%xmm0
   3a986:	movdqu 0x3(%rsi),%xmm1
   3a98b:	pcmpeqb -0x2baa3(%rip),%xmm1        # eef0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xa0>
   3a993:	pcmpeqb -0x2bf3b(%rip),%xmm0        # ea60 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x350>
   3a99b:	pand   %xmm1,%xmm0
   3a99f:	pmovmskb %xmm0,%eax
   3a9a3:	cmp    $0xffff,%eax
   3a9a8:	jne    3ab90 <meshopt_peers::run+0xa30>
   3a9ae:	mov    0x58(%rcx),%rax
   3a9b2:	test   %r14,%r14
   3a9b5:	setne  %dl
   3a9b8:	lea    0x0(,%r14,4),%rsi
   3a9c0:	cmp    %rax,%rsi
   3a9c3:	seta   %sil
   3a9c7:	and    %dl,%sil
   3a9ca:	cmp    $0x1,%sil
   3a9ce:	je     3d6aa <meshopt_peers::run+0x354a>
   3a9d4:	mov    0x38(%rcx),%rsi
   3a9d8:	mov    0x40(%rcx),%rdx
   3a9dc:	mov    0x50(%rcx),%rcx
   3a9e0:	movb   $0x0,0x750(%rsp)
   3a9e8:	mov    %rcx,0x758(%rsp)
   3a9f0:	mov    %rax,0x760(%rsp)
   3a9f8:	mov    %r14,0x768(%rsp)
   3aa00:	movq   $0x4,0x770(%rsp)
   3aa0c:	movq   $0x4,0x778(%rsp)
   3aa18:	pxor   %xmm0,%xmm0
   3aa1c:	movdqu %xmm0,0x780(%rsp)
   3aa25:	sub    $0x30,%rsp
   3aa29:	mov    %r11,0x20(%rsp)
   3aa2e:	lea    0x250(%rsp),%rax
   3aa36:	mov    %rax,0x18(%rsp)
   3aa3b:	movq   $0x0,0x8(%rsp)
   3aa44:	movq   $0x4,(%rsp)
   3aa4c:	lea    -0x2bb13(%rip),%r9        # ef40 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xf0>
   3aa53:	lea    0x290(%rsp),%rdi
   3aa5b:	lea    0x6b8(%rsp),%rcx
   3aa63:	lea    0x780(%rsp),%r8
   3aa6b:	jmp    3ae08 <meshopt_peers::run+0xca8>
   3aa70:	movdqu (%rsi),%xmm0
   3aa74:	pcmpeqb -0x2c1ac(%rip),%xmm0        # e8d0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x1c0>
   3aa7c:	pmovmskb %xmm0,%eax
   3aa80:	cmp    $0xffff,%eax
   3aa85:	jne    3ab90 <meshopt_peers::run+0xa30>
   3aa8b:	mov    0x38(%rcx),%rsi
   3aa8f:	mov    0x40(%rcx),%rdx
   3aa93:	movq   $0x29,0x268(%rsp)
   3aa9f:	movabs $0x3f00000000000002,%rax
   3aaa9:	mov    %rax,0x260(%rsp)
   3aab1:	lea    0x260(%rsp),%rax
   3aab9:	lea    0x750(%rsp),%rdi
   3aac1:	lea    0x688(%rsp),%r8
   3aac9:	mov    $0x40,%r9d
   3aacf:	mov    %r14,%rcx
   3aad2:	push   $0x2
   3aad4:	push   %r11
   3aad6:	push   %rax
   3aad7:	push   $0x7c
   3aad9:	call   c9df0 <meshoptimizer_rs::meshlet::build>
   3aade:	add    $0x20,%rsp
   3aae2:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3aaeb:	je     3aea1 <meshopt_peers::run+0xd41>
   3aaf1:	mov    0x790(%rsp),%rax
   3aaf9:	mov    %rax,0x350(%rsp)
   3ab01:	movdqu 0x750(%rsp),%xmm0
   3ab0a:	movups 0x760(%rsp),%xmm1
   3ab12:	movdqu 0x770(%rsp),%xmm2
   3ab1b:	movdqu 0x780(%rsp),%xmm3
   3ab24:	movdqa %xmm3,0x340(%rsp)
   3ab2d:	movdqa %xmm2,0x330(%rsp)
   3ab36:	movaps %xmm1,0x320(%rsp)
   3ab3e:	movdqa %xmm0,0x310(%rsp)
   3ab47:	mov    0x318(%rsp),%r14
   3ab4f:	mov    0x320(%rsp),%r12
   3ab57:	test   %r12,%r12
   3ab5a:	je     3ace4 <meshopt_peers::run+0xb84>
   3ab60:	mov    %r12,%r15
   3ab63:	shl    $0x4,%r15
   3ab67:	mov    %r15,%rdi
   3ab6a:	call   *0x111a68(%rip)        # 14c5d8 <malloc@GLIBC_2.2.5>
   3ab70:	test   %rax,%rax
   3ab73:	je     3d70a <meshopt_peers::run+0x35aa>
   3ab79:	mov    %r12d,%ecx
   3ab7c:	and    $0x3,%ecx
   3ab7f:	cmp    $0x4,%r12
   3ab83:	jae    3aceb <meshopt_peers::run+0xb8b>
   3ab89:	xor    %edx,%edx
   3ab8b:	jmp    3ad3e <meshopt_peers::run+0xbde>
   3ab90:	lea    0x6d0(%rsp),%rax
   3ab98:	mov    %rax,0x750(%rsp)
   3aba0:	lea    0x8959(%rip),%rax        # 43500 <<&str as core::fmt::Display>::fmt>
   3aba7:	mov    %rax,0x758(%rsp)
   3abaf:	lea    -0x2d65d(%rip),%rdi        # d559 <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x14a>
   3abb6:	lea    0x109a8b(%rip),%rdx        # 144648 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x218>
   3abbd:	lea    0x750(%rsp),%rsi
   3abc5:	call   *0x111ab5(%rip)        # 14c680 <_DYNAMIC+0x2a0>
   3abcb:	mov    %r14,%rax
   3abce:	shr    $0x20,%rax
   3abd2:	jne    3d67a <meshopt_peers::run+0x351a>
   3abd8:	lea    0x0(,%r14,4),%rax
   3abe0:	lea    (%rax,%rax,2),%rax
   3abe4:	cmp    $0x5,%rdx
   3abe8:	je     3b191 <meshopt_peers::run+0x1031>
   3abee:	cmp    $0xb,%rdx
   3abf2:	jne    3b1a7 <meshopt_peers::run+0x1047>
   3abf8:	movabs $0x756d2d70616d6572,%rdx
   3ac02:	xor    (%rsi),%rdx
   3ac05:	movabs $0x69746c756d2d7061,%rdi
   3ac0f:	xor    0x3(%rsi),%rdi
   3ac13:	or     %rdx,%rdi
   3ac16:	jne    3b1a7 <meshopt_peers::run+0x1047>
   3ac1c:	test   %r14,%r14
   3ac1f:	setne  %dl
   3ac22:	mov    0x58(%rcx),%rdi
   3ac26:	shl    $0x2,%rdi
   3ac2a:	mov    %r14,%rsi
   3ac2d:	shl    $0x4,%rsi
   3ac31:	cmp    %rdi,%rsi
   3ac34:	seta   %sil
   3ac38:	and    %dl,%sil
   3ac3b:	cmp    $0x1,%sil
   3ac3f:	je     3d85e <meshopt_peers::run+0x36fe>
   3ac45:	mov    0x38(%rcx),%rsi
   3ac49:	mov    0x40(%rcx),%rdx
   3ac4d:	mov    0x50(%rcx),%rcx
   3ac51:	mov    %r13,0x750(%rsp)
   3ac59:	mov    %rax,0x758(%rsp)
   3ac61:	mov    %r14,0x760(%rsp)
   3ac69:	movq   $0xc,0x768(%rsp)
   3ac75:	movq   $0xc,0x770(%rsp)
   3ac81:	movq   $0x0,0x778(%rsp)
   3ac8d:	mov    %rcx,0x780(%rsp)
   3ac95:	mov    %rdi,0x788(%rsp)
   3ac9d:	mov    %r14,0x790(%rsp)
   3aca5:	movq   $0x10,0x798(%rsp)
   3acb1:	movq   $0x10,0x7a0(%rsp)
   3acbd:	movq   $0x0,0x7a8(%rsp)
   3acc9:	lea    0x310(%rsp),%rdi
   3acd1:	lea    0x750(%rsp),%rcx
   3acd9:	mov    $0x2,%r8d
   3acdf:	jmp    3bb78 <meshopt_peers::run+0x1a18>
   3ace4:	mov    $0x4,%eax
   3ace9:	jmp    3ad63 <meshopt_peers::run+0xc03>
   3aceb:	mov    %r12,%rsi
   3acee:	and    $0xfffffffffffffffc,%rsi
   3acf2:	mov    $0x30,%edi
   3acf7:	xor    %edx,%edx
   3acf9:	nopl   0x0(%rax)
   3ad00:	movups -0x30(%r14,%rdi,1),%xmm0
   3ad06:	movups %xmm0,-0x30(%rax,%rdi,1)
   3ad0b:	movups -0x20(%r14,%rdi,1),%xmm0
   3ad11:	movups %xmm0,-0x20(%rax,%rdi,1)
   3ad16:	movups -0x10(%r14,%rdi,1),%xmm0
   3ad1c:	movups %xmm0,-0x10(%rax,%rdi,1)
   3ad21:	movdqu (%r14,%rdi,1),%xmm0
   3ad27:	movdqu %xmm0,(%rax,%rdi,1)
   3ad2c:	add    $0x4,%rdx
   3ad30:	add    $0x40,%rdi
   3ad34:	cmp    %rdx,%rsi
   3ad37:	jne    3ad00 <meshopt_peers::run+0xba0>
   3ad39:	test   %rcx,%rcx
   3ad3c:	je     3ad63 <meshopt_peers::run+0xc03>
   3ad3e:	shl    $0x4,%rdx
   3ad42:	mov    %rax,%rsi
   3ad45:	add    %rdx,%rsi
   3ad48:	add    %r14,%rdx
   3ad4b:	shl    $0x4,%ecx
   3ad4e:	xor    %edi,%edi
   3ad50:	movdqu (%rdx,%rdi,1),%xmm0
   3ad55:	movdqu %xmm0,(%rsi,%rdi,1)
   3ad5a:	add    $0x10,%rdi
   3ad5e:	cmp    %rdi,%rcx
   3ad61:	jne    3ad50 <meshopt_peers::run+0xbf0>
   3ad63:	mov    0x338(%rsp),%rcx
   3ad6b:	mov    %rcx,0x30(%rbx)
   3ad6f:	movups 0x328(%rsp),%xmm0
   3ad77:	movups %xmm0,0x20(%rbx)
   3ad7b:	movdqa 0x340(%rsp),%xmm0
   3ad84:	movdqu %xmm0,0x38(%rbx)
   3ad89:	mov    0x350(%rsp),%rcx
   3ad91:	mov    %rcx,0x48(%rbx)
   3ad95:	mov    %r12,0x8(%rbx)
   3ad99:	mov    %rax,0x10(%rbx)
   3ad9d:	mov    %r12,0x18(%rbx)
   3ada1:	movl   $0x6,(%rbx)
   3ada7:	cmpq   $0x0,0x310(%rsp)
   3adb0:	je     3de26 <meshopt_peers::run+0x3cc6>
   3adb6:	jmp    3bc56 <meshopt_peers::run+0x1af6>
   3adbb:	mov    0x38(%rcx),%rsi
   3adbf:	mov    0x40(%rcx),%rdx
   3adc3:	movb   $0x2,0x750(%rsp)
   3adcb:	sub    $0x30,%rsp
   3adcf:	mov    %r11,0x20(%rsp)
   3add4:	lea    0x250(%rsp),%rax
   3addc:	mov    %rax,0x18(%rsp)
   3ade1:	pxor   %xmm0,%xmm0
   3ade5:	movdqu %xmm0,(%rsp)
   3adea:	lea    0x290(%rsp),%rdi
   3adf2:	lea    0x6b8(%rsp),%rcx
   3adfa:	lea    0x780(%rsp),%r8
   3ae02:	mov    $0x4,%r9d
   3ae08:	call   d4a90 <meshoptimizer_rs::simplify::allocating>
   3ae0d:	add    $0x30,%rsp
   3ae11:	mov    0x260(%rsp),%rax
   3ae19:	cmp    $0xffffffffffffffff,%rax
   3ae1d:	je     3b0f6 <meshopt_peers::run+0xf96>
   3ae23:	mov    0x270(%rsp),%rax
   3ae2b:	mov    %rax,0x18(%rbx)
   3ae2f:	movups 0x260(%rsp),%xmm0
   3ae37:	movups %xmm0,0x8(%rbx)
   3ae3b:	movd   0x278(%rsp),%xmm0
   3ae44:	movd   %xmm0,0x4(%rbx)
   3ae49:	movl   $0x4,(%rbx)
   3ae4f:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3ae54:	mov    0x38(%rcx),%rsi
   3ae58:	mov    0x40(%rcx),%rdx
   3ae5c:	lea    0x750(%rsp),%rdi
   3ae64:	lea    0x688(%rsp),%rcx
   3ae6c:	movss  -0x2a408(%rip),%xmm1        # 10a6c <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa0>
   3ae74:	mov    $0x40,%r8d
   3ae7a:	mov    $0x7c,%r9d
   3ae80:	pxor   %xmm0,%xmm0
   3ae84:	push   %r11
   3ae86:	push   $0x29
   3ae88:	call   *0x111b2a(%rip)        # 14c9b8 <_DYNAMIC+0x5d8>
   3ae8e:	add    $0x10,%rsp
   3ae92:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3ae9b:	jne    3aaf1 <meshopt_peers::run+0x991>
   3aea1:	movzbl 0x758(%rsp),%eax
   3aea9:	mov    %al,0x260(%rsp)
   3aeb0:	lea    -0x29a2c(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3aeb7:	lea    0x109c82(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3aebe:	lea    0x1098eb(%rip),%r8        # 1447b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x380>
   3aec5:	lea    0x260(%rsp),%rdx
   3aecd:	mov    $0x2b,%esi
   3aed2:	call   *0x1117d8(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3aed8:	mov    0x38(%rcx),%rsi
   3aedc:	mov    0x40(%rcx),%rdx
   3aee0:	lea    0x750(%rsp),%rdi
   3aee8:	mov    %r14,%rcx
   3aeeb:	mov    %r11,%r8
   3aeee:	call   *0x111acc(%rip)        # 14c9c0 <_DYNAMIC+0x5e0>
   3aef4:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3aefd:	jne    3b6cc <meshopt_peers::run+0x156c>
   3af03:	movzbl 0x758(%rsp),%eax
   3af0b:	mov    %al,0x310(%rsp)
   3af12:	lea    -0x29a8e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3af19:	lea    0x109c20(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3af20:	lea    0x109901(%rip),%r8        # 144828 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3f8>
   3af27:	lea    0x310(%rsp),%rdx
   3af2f:	mov    $0x2b,%esi
   3af34:	call   *0x111776(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3af3a:	mov    0x38(%rcx),%rsi
   3af3e:	mov    0x40(%rcx),%rdx
   3af42:	sub    $0x8,%rsp
   3af46:	lea    0x758(%rsp),%rdi
   3af4e:	lea    0x690(%rsp),%rcx
   3af56:	movd   -0x2a4f6(%rip),%xmm0        # 10a68 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x9c>
   3af5e:	xor    %r8d,%r8d
   3af61:	push   %r11
   3af63:	call   f2830 <meshoptimizer_rs::simplify::extra::simplify_prune_kernel>
   3af68:	add    $0x10,%rsp
   3af6c:	mov    0x750(%rsp),%rcx
   3af74:	movzbl 0x758(%rsp),%eax
   3af7c:	cmp    $0xffffffffffffffff,%rcx
   3af80:	je     3d74c <meshopt_peers::run+0x35ec>
   3af86:	mov    0x759(%rsp),%rdx
   3af8e:	mov    0x760(%rsp),%rsi
   3af96:	mov    %rsi,0x18(%rbx)
   3af9a:	mov    %rdx,0x11(%rbx)
   3af9e:	mov    %rcx,0x8(%rbx)
   3afa2:	mov    %al,0x10(%rbx)
   3afa5:	movl   $0x3,(%rbx)
   3afab:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3afb0:	movdqu 0x38(%rcx),%xmm1
   3afb5:	movq   $0x0,0x1c8(%rsp)
   3afc1:	mov    %rax,0x168(%rsp)
   3afc9:	movd   %xmm0,0x230(%rsp)
   3afd2:	movb   $0x0,0x365(%r11)
   3afda:	movl   $0x0,0x361(%r11)
   3afe5:	pxor   %xmm0,%xmm0
   3afe9:	movdqu %xmm0,0x350(%r11)
   3aff2:	mov    0x348(%r11),%rax
   3aff9:	mov    %rax,0xf0(%rsp)
   3b001:	mov    %rax,0xf8(%rsp)
   3b009:	movdqu %xmm1,0x750(%rsp)
   3b012:	lea    0x688(%rsp),%rax
   3b01a:	mov    %rax,0x760(%rsp)
   3b022:	lea    0xf0(%rsp),%rax
   3b02a:	mov    %rax,0x768(%rsp)
   3b032:	lea    0x1c8(%rsp),%rax
   3b03a:	mov    %rax,0x770(%rsp)
   3b042:	lea    0x168(%rsp),%rax
   3b04a:	mov    %rax,0x778(%rsp)
   3b052:	lea    0x230(%rsp),%rax
   3b05a:	mov    %rax,0x780(%rsp)
   3b062:	mov    %r11,0x788(%rsp)
   3b06a:	movq   $0x0,0x790(%rsp)
   3b076:	lea    0x310(%rsp),%rdi
   3b07e:	lea    0x750(%rsp),%rsi
   3b086:	mov    %r11,%r14
   3b089:	call   8fba0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   3b08e:	mov    0xf8(%rsp),%rax
   3b096:	sub    0xf0(%rsp),%rax
   3b09e:	mov    %rax,0x358(%r14)
   3b0a5:	mov    0x310(%rsp),%rax
   3b0ad:	movzbl 0x318(%rsp),%ecx
   3b0b5:	cmp    $0xffffffffffffffff,%rax
   3b0b9:	je     3b0dd <meshopt_peers::run+0xf7d>
   3b0bb:	mov    0x328(%rsp),%rdx
   3b0c3:	mov    %rdx,0x278(%rsp)
   3b0cb:	movdqu 0x319(%rsp),%xmm0
   3b0d4:	movdqu %xmm0,0x269(%rsp)
   3b0dd:	mov    %cl,0x268(%rsp)
   3b0e4:	mov    %rax,0x260(%rsp)
   3b0ec:	cmp    $0xffffffffffffffff,%rax
   3b0f0:	jne    3ae23 <meshopt_peers::run+0xcc3>
   3b0f6:	movzbl 0x268(%rsp),%eax
   3b0fe:	mov    %al,0x750(%rsp)
   3b105:	lea    -0x29c81(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b10c:	lea    0x109a2d(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b113:	lea    0x109786(%rip),%r8        # 1448a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x470>
   3b11a:	lea    0x750(%rsp),%rdx
   3b122:	mov    $0x2b,%esi
   3b127:	call   *0x111583(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3b12d:	mov    0x38(%rcx),%rsi
   3b131:	mov    0x40(%rcx),%rdx
   3b135:	lea    0x750(%rsp),%rdi
   3b13d:	mov    $0x1,%ecx
   3b142:	mov    %r11,%r8
   3b145:	call   *0x11187d(%rip)        # 14c9c8 <_DYNAMIC+0x5e8>
   3b14b:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b154:	jne    3b404 <meshopt_peers::run+0x12a4>
   3b15a:	movzbl 0x758(%rsp),%eax
   3b162:	mov    %al,0x310(%rsp)
   3b169:	lea    -0x29ce5(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b170:	lea    0x1099c9(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b177:	lea    0x1095ea(%rip),%r8        # 144768 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x338>
   3b17e:	lea    0x310(%rsp),%rdx
   3b186:	mov    $0x2b,%esi
   3b18b:	call   *0x11151f(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3b191:	mov    $0x616d6572,%edx
   3b196:	xor    (%rsi),%edx
   3b198:	movzbl 0x4(%rsi),%esi
   3b19c:	xor    $0x70,%esi
   3b19f:	or     %edx,%esi
   3b1a1:	je     3bb1e <meshopt_peers::run+0x19be>
   3b1a7:	lea    0x48(%rcx),%rax
   3b1ab:	mov    0x38(%rcx),%rbp
   3b1af:	mov    0x40(%rcx),%r15
   3b1b3:	mov    %rax,0x168(%rsp)
   3b1bb:	mov    %rbp,0xf0(%rsp)
   3b1c3:	mov    %r15,0xf8(%rsp)
   3b1cb:	mov    %r14,0x1c8(%rsp)
   3b1d3:	movb   $0x0,0x230(%rsp)
   3b1db:	movb   $0x0,0x365(%r11)
   3b1e3:	movl   $0x0,0x361(%r11)
   3b1ee:	pxor   %xmm0,%xmm0
   3b1f2:	movdqu %xmm0,0x350(%r11)
   3b1fb:	mov    0x348(%r11),%rax
   3b202:	mov    %rax,0x260(%rsp)
   3b20a:	mov    %rax,0x268(%rsp)
   3b212:	lea    0xf0(%rsp),%rcx
   3b21a:	mov    %rcx,0x750(%rsp)
   3b222:	lea    0x1c8(%rsp),%rcx
   3b22a:	mov    %rcx,0x758(%rsp)
   3b232:	lea    0x260(%rsp),%rcx
   3b23a:	mov    %rcx,0x760(%rsp)
   3b242:	mov    %r11,0x768(%rsp)
   3b24a:	lea    0x230(%rsp),%rcx
   3b252:	mov    %rcx,0x770(%rsp)
   3b25a:	mov    %r13,0x778(%rsp)
   3b262:	mov    %r14,0x780(%rsp)
   3b26a:	mov    %r13,0x788(%rsp)
   3b272:	mov    %r14,0x790(%rsp)
   3b27a:	lea    0x168(%rsp),%rcx
   3b282:	mov    %rcx,0x798(%rsp)
   3b28a:	mov    %rax,%rcx
   3b28d:	sub    %r15,%rcx
   3b290:	jae    3b317 <meshopt_peers::run+0x11b7>
   3b296:	mov    %rbx,(%rsp)
   3b29a:	shl    $0x2,%r15
   3b29e:	xor    %ebx,%ebx
   3b2a0:	lea    0x260(%rsp),%rdi
   3b2a8:	mov    0x1116f9(%rip),%rcx        # 14c9a8 <_DYNAMIC+0x5c8>
   3b2af:	nop
   3b2b0:	cmp    %rbx,%r15
   3b2b3:	je     3b75b <meshopt_peers::run+0x15fb>
   3b2b9:	mov    0x0(%rbp,%rbx,1),%r12d
   3b2be:	test   %rax,%rax
   3b2c1:	je     3b2d9 <meshopt_peers::run+0x1179>
   3b2c3:	dec    %rax
   3b2c6:	mov    %rax,0x260(%rsp)
   3b2ce:	add    $0x4,%rbx
   3b2d2:	cmp    %r14d,%r12d
   3b2d5:	jb     3b2b0 <meshopt_peers::run+0x1150>
   3b2d7:	jmp    3b30c <meshopt_peers::run+0x11ac>
   3b2d9:	mov    %r11,0x8(%rsp)
   3b2de:	mov    $0x1,%esi
   3b2e3:	call   *%rcx
   3b2e5:	cmp    $0xff,%al
   3b2e7:	jne    3d3bc <meshopt_peers::run+0x325c>
   3b2ed:	xor    %eax,%eax
   3b2ef:	mov    0x8(%rsp),%r11
   3b2f4:	lea    0x260(%rsp),%rdi
   3b2fc:	mov    0x1116a5(%rip),%rcx        # 14c9a8 <_DYNAMIC+0x5c8>
   3b303:	add    $0x4,%rbx
   3b307:	cmp    %r14d,%r12d
   3b30a:	jb     3b2b0 <meshopt_peers::run+0x1150>
   3b30c:	mov    $0x1,%al
   3b30e:	mov    (%rsp),%rbx
   3b312:	jmp    3ddc1 <meshopt_peers::run+0x3c61>
   3b317:	test   %r15,%r15
   3b31a:	je     3dcd7 <meshopt_peers::run+0x3b77>
   3b320:	mov    0x0(%rbp),%edi
   3b323:	cmp    $0x1,%r15
   3b327:	je     3dccc <meshopt_peers::run+0x3b6c>
   3b32d:	movabs $0x3fffffffffffffff,%r8
   3b337:	lea    (%r15,%r8,1),%rsi
   3b33b:	mov    %rsi,%rdx
   3b33e:	and    %r8,%rdx
   3b341:	cmp    $0x8,%rdx
   3b345:	jae    3bcc5 <meshopt_peers::run+0x1b65>
   3b34b:	xor    %esi,%esi
   3b34d:	jmp    3dcb6 <meshopt_peers::run+0x3b56>
   3b352:	mov    0x38(%rcx),%rsi
   3b356:	mov    0x40(%rcx),%rdx
   3b35a:	lea    0x750(%rsp),%rdi
   3b362:	lea    0x688(%rsp),%rcx
   3b36a:	movd   -0x2a866(%rip),%xmm0        # 10b0c <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x140>
   3b372:	mov    %r11,%r8
   3b375:	call   *0x111655(%rip)        # 14c9d0 <_DYNAMIC+0x5f0>
   3b37b:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b384:	jne    3b6cc <meshopt_peers::run+0x156c>
   3b38a:	movzbl 0x758(%rsp),%eax
   3b392:	mov    %al,0x310(%rsp)
   3b399:	lea    -0x29f15(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b3a0:	lea    0x109799(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b3a7:	lea    0x109462(%rip),%r8        # 144810 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3e0>
   3b3ae:	lea    0x310(%rsp),%rdx
   3b3b6:	mov    $0x2b,%esi
   3b3bb:	call   *0x1112ef(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3b3c1:	lea    0x0(,%r14,4),%rax
   3b3c9:	lea    (%rax,%rax,2),%rdx
   3b3cd:	lea    0x750(%rsp),%rdi
   3b3d5:	mov    $0xc,%r8d
   3b3db:	mov    %r13,%rsi
   3b3de:	mov    %r14,%rcx
   3b3e1:	mov    $0x1,%r9d
   3b3e7:	push   %r11
   3b3e9:	push   $0x2
   3b3eb:	call   *0x1115e7(%rip)        # 14c9d8 <_DYNAMIC+0x5f8>
   3b3f1:	add    $0x10,%rsp
   3b3f5:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b3fe:	je     3d2c4 <meshopt_peers::run+0x3164>
   3b404:	mov    0x760(%rsp),%rax
   3b40c:	mov    %rax,0x18(%rbx)
   3b410:	movdqu 0x750(%rsp),%xmm0
   3b419:	movdqu %xmm0,0x8(%rbx)
   3b41e:	movl   $0x2,(%rbx)
   3b424:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3b429:	mov    %r14,%rax
   3b42c:	shr    $0x20,%rax
   3b430:	jne    3d7c3 <meshopt_peers::run+0x3663>
   3b436:	mov    0x38(%rcx),%rsi
   3b43a:	mov    0x40(%rcx),%rdx
   3b43e:	lea    0x0(,%r14,4),%rax
   3b446:	lea    (%rax,%rax,2),%rax
   3b44a:	mov    %r13,0x310(%rsp)
   3b452:	mov    %rax,0x318(%rsp)
   3b45a:	mov    %r14,0x320(%rsp)
   3b462:	movq   $0xc,0x328(%rsp)
   3b46e:	movq   $0xc,0x330(%rsp)
   3b47a:	movq   $0x0,0x338(%rsp)
   3b486:	lea    0x750(%rsp),%rdi
   3b48e:	lea    0x310(%rsp),%rcx
   3b496:	mov    %r11,%r8
   3b499:	call   *0x111541(%rip)        # 14c9e0 <_DYNAMIC+0x600>
   3b49f:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b4a8:	je     3d332 <meshopt_peers::run+0x31d2>
   3b4ae:	mov    0x780(%rsp),%rax
   3b4b6:	mov    %rax,0x340(%rsp)
   3b4be:	movups 0x750(%rsp),%xmm0
   3b4c6:	movups 0x760(%rsp),%xmm1
   3b4ce:	movdqu 0x770(%rsp),%xmm2
   3b4d7:	movdqa %xmm2,0x330(%rsp)
   3b4e0:	movaps %xmm1,0x320(%rsp)
   3b4e8:	movaps %xmm0,0x310(%rsp)
   3b4f0:	mov    0x320(%rsp),%rax
   3b4f8:	mov    %rax,0x18(%rbx)
   3b4fc:	movups %xmm0,0x8(%rbx)
   3b500:	movdqu 0x328(%rsp),%xmm0
   3b509:	movdqu %xmm0,0x20(%rbx)
   3b50e:	mov    0x338(%rsp),%rax
   3b516:	mov    %rax,0x30(%rbx)
   3b51a:	movl   $0x5,(%rbx)
   3b520:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3b525:	mov    0x40(%rcx),%rbp
   3b529:	mov    $0x4,%r15d
   3b52f:	mov    $0x4,%r12d
   3b535:	test   %rbp,%rbp
   3b538:	mov    %rbp,(%rsp)
   3b53c:	je     3b582 <meshopt_peers::run+0x1422>
   3b53e:	mov    %r11,0x8(%rsp)
   3b543:	mov    0x38(%rcx),%r15
   3b547:	lea    0x0(,%rbp,4),%rbp
   3b54f:	mov    %rbp,%rdi
   3b552:	call   *0x111080(%rip)        # 14c5d8 <malloc@GLIBC_2.2.5>
   3b558:	test   %rax,%rax
   3b55b:	je     3d80d <meshopt_peers::run+0x36ad>
   3b561:	mov    %rax,%r12
   3b564:	mov    %rax,%rdi
   3b567:	mov    %r15,%rsi
   3b56a:	mov    %rbp,%rdx
   3b56d:	call   *0x111085(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   3b573:	mov    0x8(%rsp),%r11
   3b578:	mov    $0x4,%r15d
   3b57e:	mov    (%rsp),%rbp
   3b582:	test   %r14,%r14
   3b585:	je     3b5cb <meshopt_peers::run+0x146b>
   3b587:	mov    %r12,0x8(%rsp)
   3b58c:	mov    %r11,%r12
   3b58f:	lea    0x0(,%r14,4),%rax
   3b597:	lea    (%rax,%rax,2),%rbp
   3b59b:	mov    %rbp,%rdi
   3b59e:	call   *0x111034(%rip)        # 14c5d8 <malloc@GLIBC_2.2.5>
   3b5a4:	test   %rax,%rax
   3b5a7:	je     3d84b <meshopt_peers::run+0x36eb>
   3b5ad:	mov    %rax,%r15
   3b5b0:	mov    %rax,%rdi
   3b5b3:	mov    %r13,%rsi
   3b5b6:	mov    %rbp,%rdx
   3b5b9:	call   *0x111039(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   3b5bf:	mov    %r12,%r11
   3b5c2:	mov    0x8(%rsp),%r12
   3b5c7:	mov    (%rsp),%rbp
   3b5cb:	movb   $0x0,0x750(%rsp)
   3b5d3:	mov    %r15,0x758(%rsp)
   3b5db:	mov    %r14,0x760(%rsp)
   3b5e3:	mov    %r14,0x768(%rsp)
   3b5eb:	movq   $0x3,0x770(%rsp)
   3b5f7:	movq   $0x0,0x778(%rsp)
   3b603:	sub    $0x30,%rsp
   3b607:	mov    %r11,0x20(%rsp)
   3b60c:	lea    0x250(%rsp),%rax
   3b614:	mov    %rax,0x18(%rsp)
   3b619:	pxor   %xmm0,%xmm0
   3b61d:	movdqu %xmm0,(%rsp)
   3b622:	lea    0x340(%rsp),%rdi
   3b62a:	lea    0x780(%rsp),%rcx
   3b632:	mov    $0x4,%r9d
   3b638:	mov    %r12,%rsi
   3b63b:	mov    %rbp,%rdx
   3b63e:	xor    %r8d,%r8d
   3b641:	call   *0x1113a1(%rip)        # 14c9e8 <_DYNAMIC+0x608>
   3b647:	add    $0x30,%rsp
   3b64b:	cmpb   $0x1,0x310(%rsp)
   3b653:	je     3d369 <meshopt_peers::run+0x3209>
   3b659:	mov    0x318(%rsp),%rax
   3b661:	movd   0x320(%rsp),%xmm0
   3b66a:	cmp    %rbp,%rax
   3b66d:	cmovae %rbp,%rax
   3b671:	mov    %rbp,0x8(%rbx)
   3b675:	mov    %r12,0x10(%rbx)
   3b679:	mov    %rax,0x18(%rbx)
   3b67d:	mov    %r14,0x20(%rbx)
   3b681:	mov    %r15,0x28(%rbx)
   3b685:	mov    %r14,0x30(%rbx)
   3b689:	movd   %xmm0,0x4(%rbx)
   3b68e:	movl   $0x1,(%rbx)
   3b694:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3b699:	mov    0x38(%rcx),%rsi
   3b69d:	mov    0x40(%rcx),%rdx
   3b6a1:	mov    0x8(%rcx),%rax
   3b6a5:	mov    0x10(%rcx),%r8
   3b6a9:	lea    0x750(%rsp),%rdi
   3b6b1:	mov    %rax,%rcx
   3b6b4:	mov    %r11,%r9
   3b6b7:	call   *0x111333(%rip)        # 14c9f0 <_DYNAMIC+0x610>
   3b6bd:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b6c6:	je     3d2fb <meshopt_peers::run+0x319b>
   3b6cc:	mov    0x760(%rsp),%rax
   3b6d4:	mov    %rax,0x18(%rbx)
   3b6d8:	movdqu 0x750(%rsp),%xmm0
   3b6e1:	movdqu %xmm0,0x8(%rbx)
   3b6e6:	movl   $0x3,(%rbx)
   3b6ec:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3b6f1:	mov    0x38(%rcx),%rsi
   3b6f5:	mov    0x40(%rcx),%rdx
   3b6f9:	lea    0xf0(%rsp),%rdi
   3b701:	lea    0x310(%rsp),%rcx
   3b709:	mov    %r11,%r8
   3b70c:	call   *0x1112e6(%rip)        # 14c9f8 <_DYNAMIC+0x618>
   3b712:	mov    0xf0(%rsp),%rax
   3b71a:	cmp    $0xffffffffffffffff,%rax
   3b71e:	jne    3a61a <meshopt_peers::run+0x4ba>
   3b724:	movzbl 0xf8(%rsp),%eax
   3b72c:	mov    %al,0x750(%rsp)
   3b733:	lea    -0x2a2af(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b73a:	lea    0x1093ff(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3b741:	lea    0x108f60(%rip),%r8        # 1446a8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x278>
   3b748:	lea    0x750(%rsp),%rdx
   3b750:	mov    $0x2b,%esi
   3b755:	call   *0x110f55(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3b75b:	mov    0x1c8(%rsp),%rax
   3b763:	mov    (%rsp),%rbx
   3b767:	jmp    3dce2 <meshopt_peers::run+0x3b82>
   3b76c:	mov    %r14,%rax
   3b76f:	shr    $0x20,%rax
   3b773:	jne    3d81b <meshopt_peers::run+0x36bb>
   3b779:	lea    0x0(,%r14,4),%rax
   3b781:	lea    (%rax,%rax,2),%rax
   3b785:	mov    %r13,0x750(%rsp)
   3b78d:	mov    %rax,0x758(%rsp)
   3b795:	mov    %r14,0x760(%rsp)
   3b79d:	movq   $0xc,0x768(%rsp)
   3b7a9:	movq   $0xc,0x770(%rsp)
   3b7b5:	movq   $0x0,0x778(%rsp)
   3b7c1:	mov    0x8(%rcx),%rdx
   3b7c5:	mov    0x10(%rcx),%rcx
   3b7c9:	lea    0x310(%rsp),%rdi
   3b7d1:	lea    0x750(%rsp),%rsi
   3b7d9:	mov    %r11,%r8
   3b7dc:	call   *0x11121e(%rip)        # 14ca00 <_DYNAMIC+0x620>
   3b7e2:	cmpq   $0xffffffffffffffff,0x310(%rsp)
   3b7eb:	je     3d3ca <meshopt_peers::run+0x326a>
   3b7f1:	mov    0x320(%rsp),%rax
   3b7f9:	mov    %rax,0x18(%rbx)
   3b7fd:	movdqu 0x310(%rsp),%xmm0
   3b806:	jmp    3b419 <meshopt_peers::run+0x12b9>
   3b80b:	mov    %rax,%r14
   3b80e:	mov    $0x4,%r12d
   3b814:	movq   $0x0,0x1b0(%rsp)
   3b820:	movq   $0x4,0x1b8(%rsp)
   3b82c:	movq   $0x0,0x1c0(%rsp)
   3b838:	movl   $0x0,0xac(%rsp)
   3b843:	movq   $0x80,0x1c8(%rsp)
   3b84f:	movq   $0x2a,0x1d0(%rsp)
   3b85b:	movq   $0x80,0x1d8(%rsp)
   3b867:	movq   $0x10,0x1e0(%rsp)
   3b873:	movaps -0x2cada(%rip),%xmm0        # eda0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x690>
   3b87a:	movups %xmm0,0x1e8(%rsp)
   3b882:	movaps -0x2ca29(%rip),%xmm0        # ee60 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3b889:	movups %xmm0,0x1f8(%rsp)
   3b891:	movabs $0x100010100000101,%rax
   3b89b:	mov    %rax,0x208(%rsp)
   3b8a3:	movl   $0x0,0x210(%rsp)
   3b8ae:	movb   $0x1,0x214(%rsp)
   3b8b6:	mov    0x38(%rcx),%r13
   3b8ba:	mov    0x40(%rcx),%r15
   3b8be:	mov    %r13,0x2a8(%rsp)
   3b8c6:	mov    %r15,0x2b0(%rsp)
   3b8ce:	mov    %r12,0x8(%rsp)
   3b8d3:	mov    %r12,0x2b8(%rsp)
   3b8db:	mov    %r14,0x2c0(%rsp)
   3b8e3:	movb   $0x2,0x260(%rsp)
   3b8eb:	movq   $0x4,0x2c8(%rsp)
   3b8f7:	xorps  %xmm0,%xmm0
   3b8fa:	movups %xmm0,0x2d0(%rsp)
   3b902:	movl   $0x0,0x2e8(%rsp)
   3b90d:	movb   $0x0,0x365(%r11)
   3b915:	movl   $0x0,0x361(%r11)
   3b920:	movups %xmm0,0x350(%r11)
   3b928:	mov    0x348(%r11),%rbx
   3b92f:	mov    %r11,%r12
   3b932:	mov    %r11,%rdi
   3b935:	xor    %esi,%esi
   3b937:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b93c:	mov    %eax,%ebp
   3b93e:	cmp    $0xff,%al
   3b940:	jne    3d92f <meshopt_peers::run+0x37cf>
   3b946:	mov    %r12,0x750(%rsp)
   3b94e:	mov    %rbx,0x758(%rsp)
   3b956:	mov    %rbx,0x760(%rsp)
   3b95e:	movq   $0x0,0x768(%rsp)
   3b96a:	movl   $0x0,0x770(%rsp)
   3b975:	movl   $0x0,0x974(%rsp)
   3b980:	movb   $0x1,0x1978(%rsp)
   3b988:	lea    0x260(%rsp),%rdi
   3b990:	lea    0x1c8(%rsp),%rsi
   3b998:	lea    0x750(%rsp),%rdx
   3b9a0:	call   *0x111062(%rip)        # 14ca08 <_DYNAMIC+0x628>
   3b9a6:	mov    (%rsp),%rbx
   3b9aa:	mov    %eax,%ebp
   3b9ac:	cmp    $0xff,%al
   3b9ae:	jne    3d910 <meshopt_peers::run+0x37b0>
   3b9b4:	movzbl 0x1978(%rsp),%r12d
   3b9bd:	movb   $0x2,0x310(%rsp)
   3b9c5:	mov    0x8(%rsp),%rax
   3b9ca:	mov    %rax,0x318(%rsp)
   3b9d2:	mov    %r14,0x320(%rsp)
   3b9da:	movq   $0x0,0x348(%rsp)
   3b9e6:	lea    0xf0(%rsp),%rdi
   3b9ee:	lea    0x310(%rsp),%rsi
   3b9f6:	lea    0x750(%rsp),%rdx
   3b9fe:	call   *0x11100c(%rip)        # 14ca10 <_DYNAMIC+0x630>
   3ba04:	mov    0xf0(%rsp),%rax
   3ba0c:	movzbl 0xf8(%rsp),%ebp
   3ba14:	mov    %rax,0x88(%rsp)
   3ba1c:	cmp    $0xffffffffffffffff,%rax
   3ba20:	je     3bab2 <meshopt_peers::run+0x1952>
   3ba26:	movzbl %bpl,%eax
   3ba2a:	movzbl 0xff(%rsp),%ecx
   3ba32:	shl    $0x10,%ecx
   3ba35:	movzwl 0xfd(%rsp),%edx
   3ba3d:	or     %ecx,%edx
   3ba3f:	mov    0xf9(%rsp),%ecx
   3ba46:	shl    $0x28,%rdx
   3ba4a:	shl    $0x8,%rcx
   3ba4e:	or     %rdx,%rcx
   3ba51:	or     %rax,%rcx
   3ba54:	mov    %rcx,0xe8(%rsp)
   3ba5c:	mov    0x768(%rsp),%rsi
   3ba64:	add    %r14,%rsi
   3ba67:	jb     3d28b <meshopt_peers::run+0x312b>
   3ba6d:	mov    0x100(%rsp),%rax
   3ba75:	mov    %rax,0x40(%rsp)
   3ba7a:	mov    %rsi,0x768(%rsp)
   3ba82:	mov    0x750(%rsp),%rdi
   3ba8a:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3ba8f:	mov    %eax,%ebp
   3ba91:	cmp    $0xff,%al
   3ba93:	je     3bc64 <meshopt_peers::run+0x1b04>
   3ba99:	cmpq   $0x0,0x88(%rsp)
   3baa2:	je     3bab2 <meshopt_peers::run+0x1952>
   3baa4:	mov    0xe8(%rsp),%rdi
   3baac:	call   *0x110b36(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3bab2:	mov    0x750(%rsp),%rax
   3baba:	mov    0x760(%rsp),%rcx
   3bac2:	sub    0x758(%rsp),%rcx
   3baca:	mov    %rcx,0x358(%rax)
   3bad1:	cmp    $0xff,%bpl
   3bad5:	jne    3d92f <meshopt_peers::run+0x37cf>
   3badb:	mov    0x1c0(%rsp),%rax
   3bae3:	mov    %rax,0x18(%rbx)
   3bae7:	movdqu 0x1b0(%rsp),%xmm0
   3baf0:	movdqu %xmm0,0x8(%rbx)
   3baf5:	movl   $0x3,(%rbx)
   3bafb:	test   %r14,%r14
   3bafe:	je     3de26 <meshopt_peers::run+0x3cc6>
   3bb04:	mov    0x8(%rsp),%rdi
   3bb09:	call   *0x110ad9(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3bb0f:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3bb14:	mov    $0x4,%eax
   3bb19:	jmp    3bc03 <meshopt_peers::run+0x1aa3>
   3bb1e:	mov    0x38(%rcx),%rsi
   3bb22:	mov    0x40(%rcx),%rdx
   3bb26:	mov    %r13,0x750(%rsp)
   3bb2e:	mov    %rax,0x758(%rsp)
   3bb36:	mov    %r14,0x760(%rsp)
   3bb3e:	movq   $0xc,0x768(%rsp)
   3bb4a:	movq   $0xc,0x770(%rsp)
   3bb56:	movq   $0x0,0x778(%rsp)
   3bb62:	lea    0x310(%rsp),%rdi
   3bb6a:	lea    0x750(%rsp),%rcx
   3bb72:	mov    $0x1,%r8d
   3bb78:	mov    %r11,%r9
   3bb7b:	call   *0x110e97(%rip)        # 14ca18 <_DYNAMIC+0x638>
   3bb81:	jmp    3ddeb <meshopt_peers::run+0x3c8b>
   3bb86:	mov    %r12,%rsi
   3bb89:	and    $0xfffffffffffffffc,%rsi
   3bb8d:	mov    $0x30,%edi
   3bb92:	xor    %edx,%edx
   3bb94:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   3bba0:	movups -0x30(%r14,%rdi,1),%xmm0
   3bba6:	movups %xmm0,-0x30(%rax,%rdi,1)
   3bbab:	movups -0x20(%r14,%rdi,1),%xmm0
   3bbb1:	movups %xmm0,-0x20(%rax,%rdi,1)
   3bbb6:	movups -0x10(%r14,%rdi,1),%xmm0
   3bbbc:	movups %xmm0,-0x10(%rax,%rdi,1)
   3bbc1:	movdqu (%r14,%rdi,1),%xmm0
   3bbc7:	movdqu %xmm0,(%rax,%rdi,1)
   3bbcc:	add    $0x4,%rdx
   3bbd0:	add    $0x40,%rdi
   3bbd4:	cmp    %rdx,%rsi
   3bbd7:	jne    3bba0 <meshopt_peers::run+0x1a40>
   3bbd9:	test   %rcx,%rcx
   3bbdc:	je     3bc03 <meshopt_peers::run+0x1aa3>
   3bbde:	shl    $0x4,%rdx
   3bbe2:	mov    %rax,%rsi
   3bbe5:	add    %rdx,%rsi
   3bbe8:	add    %r14,%rdx
   3bbeb:	shl    $0x4,%ecx
   3bbee:	xor    %edi,%edi
   3bbf0:	movdqu (%rdx,%rdi,1),%xmm0
   3bbf5:	movdqu %xmm0,(%rsi,%rdi,1)
   3bbfa:	add    $0x10,%rdi
   3bbfe:	cmp    %rdi,%rcx
   3bc01:	jne    3bbf0 <meshopt_peers::run+0x1a90>
   3bc03:	mov    0x338(%rsp),%rcx
   3bc0b:	mov    %rcx,0x30(%rbx)
   3bc0f:	movups 0x328(%rsp),%xmm0
   3bc17:	movups %xmm0,0x20(%rbx)
   3bc1b:	movdqa 0x340(%rsp),%xmm0
   3bc24:	movdqu %xmm0,0x38(%rbx)
   3bc29:	mov    0x350(%rsp),%rcx
   3bc31:	mov    %rcx,0x48(%rbx)
   3bc35:	mov    %r12,0x8(%rbx)
   3bc39:	mov    %rax,0x10(%rbx)
   3bc3d:	mov    %r12,0x18(%rbx)
   3bc41:	movl   $0x6,(%rbx)
   3bc47:	cmpq   $0x0,0x310(%rsp)
   3bc50:	je     3de26 <meshopt_peers::run+0x3cc6>
   3bc56:	mov    %r14,%rdi
   3bc59:	call   *0x110989(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3bc5f:	jmp    3de26 <meshopt_peers::run+0x3cc6>
   3bc64:	mov    $0x1,%eax
   3bc69:	mov    %rax,0xa0(%rsp)
   3bc71:	test   %r14,%r14
   3bc74:	jne    3c21d <meshopt_peers::run+0x20bd>
   3bc7a:	mov    0x750(%rsp),%rdi
   3bc82:	mov    0x768(%rsp),%rsi
   3bc8a:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3bc8f:	mov    %eax,%ebp
   3bc91:	mov    $0x1,%al
   3bc93:	cmp    $0xff,%bpl
   3bc97:	je     3bda7 <meshopt_peers::run+0x1c47>
   3bc9d:	test   %r14,%r14
   3bca0:	je     3d45e <meshopt_peers::run+0x32fe>
   3bca6:	mov    0xa0(%rsp),%rdi
   3bcae:	mov    %rax,%r15
   3bcb1:	call   *0x110931(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3bcb7:	test   %r15b,%r15b
   3bcba:	jne    3ba99 <meshopt_peers::run+0x1939>
   3bcc0:	jmp    3bab2 <meshopt_peers::run+0x1952>
   3bcc5:	add    $0xfffffffffffffff9,%r8
   3bcc9:	and    %r8,%rsi
   3bccc:	movd   %edi,%xmm0
   3bcd0:	pshufd $0x0,%xmm0,%xmm1
   3bcd5:	xor    %edi,%edi
   3bcd7:	movdqa -0x2ceaf(%rip),%xmm0        # ee30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x720>
   3bcdf:	movdqa %xmm1,%xmm2
   3bce3:	movdqu 0x4(%rbp,%rdi,4),%xmm3
   3bce9:	movdqu 0x14(%rbp,%rdi,4),%xmm4
   3bcef:	movdqa %xmm1,%xmm5
   3bcf3:	pxor   %xmm0,%xmm1
   3bcf7:	movdqa %xmm3,%xmm6
   3bcfb:	pxor   %xmm0,%xmm6
   3bcff:	pcmpgtd %xmm6,%xmm1
   3bd03:	pand   %xmm1,%xmm5
   3bd07:	pandn  %xmm3,%xmm1
   3bd0b:	por    %xmm5,%xmm1
   3bd0f:	movdqa %xmm2,%xmm3
   3bd13:	pxor   %xmm0,%xmm2
   3bd17:	movdqa %xmm4,%xmm5
   3bd1b:	pxor   %xmm0,%xmm5
   3bd1f:	pcmpgtd %xmm5,%xmm2
   3bd23:	pand   %xmm2,%xmm3
   3bd27:	pandn  %xmm4,%xmm2
   3bd2b:	por    %xmm3,%xmm2
   3bd2f:	add    $0x8,%rdi
   3bd33:	cmp    %rdi,%rsi
   3bd36:	jne    3bce3 <meshopt_peers::run+0x1b83>
   3bd38:	movdqa %xmm2,%xmm3
   3bd3c:	pxor   %xmm0,%xmm3
   3bd40:	movdqa %xmm1,%xmm4
   3bd44:	pxor   %xmm0,%xmm4
   3bd48:	pcmpgtd %xmm3,%xmm4
   3bd4c:	pand   %xmm4,%xmm1
   3bd50:	pandn  %xmm2,%xmm4
   3bd54:	por    %xmm1,%xmm4
   3bd58:	pshufd $0xee,%xmm4,%xmm1
   3bd5d:	movdqa %xmm4,%xmm2
   3bd61:	pxor   %xmm0,%xmm2
   3bd65:	movdqa %xmm1,%xmm3
   3bd69:	pxor   %xmm0,%xmm3
   3bd6d:	pcmpgtd %xmm3,%xmm2
   3bd71:	pand   %xmm2,%xmm4
   3bd75:	pandn  %xmm1,%xmm2
   3bd79:	por    %xmm4,%xmm2
   3bd7d:	pshufd $0x55,%xmm2,%xmm1
   3bd82:	movdqa %xmm2,%xmm3
   3bd86:	pxor   %xmm0,%xmm3
   3bd8a:	pxor   %xmm1,%xmm0
   3bd8e:	pcmpgtd %xmm0,%xmm3
   3bd92:	pand   %xmm3,%xmm2
   3bd96:	pandn  %xmm1,%xmm3
   3bd9a:	por    %xmm2,%xmm3
   3bd9e:	movd   %xmm3,%edi
   3bda2:	jmp    3dcc7 <meshopt_peers::run+0x3b67>
   3bda7:	movb   $0x2,0xf0(%rsp)
   3bdaf:	mov    0x8(%rsp),%rax
   3bdb4:	mov    %rax,0xf8(%rsp)
   3bdbc:	mov    %r14,0x100(%rsp)
   3bdc4:	movq   $0x0,0x128(%rsp)
   3bdd0:	movups 0x208(%rsp),%xmm0
   3bdd8:	movaps %xmm0,0x350(%rsp)
   3bde0:	movups 0x1c8(%rsp),%xmm0
   3bde8:	movups 0x1d8(%rsp),%xmm1
   3bdf0:	movdqu 0x1e8(%rsp),%xmm2
   3bdf9:	movdqu 0x1f8(%rsp),%xmm3
   3be02:	movdqa %xmm3,0x340(%rsp)
   3be0b:	movdqa %xmm2,0x330(%rsp)
   3be14:	movaps %xmm1,0x320(%rsp)
   3be1c:	movaps %xmm0,0x310(%rsp)
   3be24:	movzbl %r12b,%r10d
   3be28:	lea    0x168(%rsp),%rdi
   3be30:	lea    0xf0(%rsp),%rcx
   3be38:	lea    0x310(%rsp),%r8
   3be40:	mov    %r13,%rsi
   3be43:	mov    %r15,%rdx
   3be46:	mov    $0xffffffff,%r9d
   3be4c:	lea    0x750(%rsp),%rax
   3be54:	push   %rax
   3be55:	mov    %r10,0x220(%rsp)
   3be5d:	push   %r10
   3be5f:	call   *0x110bbb(%rip)        # 14ca20 <_DYNAMIC+0x640>
   3be65:	add    $0x10,%rsp
   3be69:	mov    0x168(%rsp),%rdi
   3be71:	movzbl 0x170(%rsp),%ebp
   3be79:	cmp    $0xffffffffffffffff,%rdi
   3be7d:	je     3c1cc <meshopt_peers::run+0x206c>
   3be83:	movzbl %bpl,%eax
   3be87:	movzbl 0x177(%rsp),%ecx
   3be8f:	shl    $0x10,%ecx
   3be92:	movzwl 0x175(%rsp),%edx
   3be9a:	or     %ecx,%edx
   3be9c:	mov    0x171(%rsp),%ecx
   3bea3:	mov    0x178(%rsp),%r9
   3beab:	mov    0x180(%rsp),%r8
   3beb3:	mov    0x188(%rsp),%rsi
   3bebb:	mov    %rsi,0x20(%rsp)
   3bec0:	shl    $0x28,%rdx
   3bec4:	shl    $0x8,%rcx
   3bec8:	or     %rdx,%rcx
   3becb:	or     %rax,%rcx
   3bece:	mov    %rcx,0x30(%rsp)
   3bed3:	test   %r9,%r9
   3bed6:	je     3c1d3 <meshopt_peers::run+0x2073>
   3bedc:	mov    %r8,0x78(%rsp)
   3bee1:	mov    %rdi,0x80(%rsp)
   3bee9:	mov    0x190(%rsp),%rax
   3bef1:	mov    %rax,0x10(%rsp)
   3bef6:	mov    %r9,0x70(%rsp)
   3befb:	mov    %r9,%rax
   3befe:	shl    $0x4,%rax
   3bf02:	lea    (%rax,%rax,2),%rax
   3bf06:	mov    %rax,0x18(%rsp)
   3bf0b:	xor    %r12d,%r12d
   3bf0e:	mov    0x750(%rsp),%r15
   3bf16:	mov    0x758(%rsp),%rbp
   3bf1e:	mov    0x760(%rsp),%rax
   3bf26:	sub    %rbp,%rax
   3bf29:	mov    %rax,0x358(%r15)
   3bf30:	mov    %r15,%rdi
   3bf33:	call   *0x110a5f(%rip)        # 14c998 <_DYNAMIC+0x5b8>
   3bf39:	mov    0x340(%r15),%rax
   3bf40:	mov    0x768(%rsp),%r13
   3bf48:	sub    %r13,%rax
   3bf4b:	jb     3c25c <meshopt_peers::run+0x20fc>
   3bf51:	mov    0x348(%r15),%rcx
   3bf58:	sub    0x358(%r15),%rcx
   3bf5f:	mov    %rax,0x650(%rsp)
   3bf67:	mov    %rcx,0x658(%rsp)
   3bf6f:	movb   $0x0,0x670(%rsp)
   3bf77:	movb   $0x0,0x675(%rsp)
   3bf7f:	movl   $0x0,0x671(%rsp)
   3bf8a:	movb   $0xff,0x678(%rsp)
   3bf92:	movw   $0x0,0x676(%rsp)
   3bf9c:	lea    0x660(%rsp),%rax
   3bfa4:	xorps  %xmm0,%xmm0
   3bfa7:	movups %xmm0,(%rax)
   3bfaa:	movq   $0xffffffffffffffff,0x388(%rsp)
   3bfb6:	movq   $0xffffffffffffffff,0x590(%rsp)
   3bfc2:	movq   $0x0,0x310(%rsp)
   3bfce:	movq   $0x4,0x318(%rsp)
   3bfda:	movups %xmm0,-0x340(%rax)
   3bfe1:	movq   $0x4,0x330(%rsp)
   3bfed:	movups %xmm0,-0x328(%rax)
   3bff4:	movq   $0x4,0x348(%rsp)
   3c000:	movups %xmm0,-0x310(%rax)
   3c007:	movq   $0x1,0x360(%rsp)
   3c013:	movups %xmm0,-0x2f8(%rax)
   3c01a:	movq   $0x2,0x378(%rsp)
   3c026:	movq   $0x0,0x380(%rsp)
   3c032:	mov    0x30(%rsp),%rax
   3c037:	mov    0x8(%rax,%r12,1),%rdi
   3c03c:	mov    0x10(%rax,%r12,1),%rdx
   3c041:	mov    %rdx,%rsi
   3c044:	add    %rdi,%rsi
   3c047:	jb     3d3a5 <meshopt_peers::run+0x3245>
   3c04d:	cmp    0x10(%rsp),%rsi
   3c052:	ja     3d3a5 <meshopt_peers::run+0x3245>
   3c058:	mov    0x20(%rsp),%rax
   3c05d:	lea    (%rax,%rdi,4),%rsi
   3c061:	movb   $0x2,0xf0(%rsp)
   3c069:	mov    0x8(%rsp),%rax
   3c06e:	mov    %rax,0xf8(%rsp)
   3c076:	mov    %r14,0x100(%rsp)
   3c07e:	movq   $0x0,0x128(%rsp)
   3c08a:	lea    0x168(%rsp),%rdi
   3c092:	lea    0xf0(%rsp),%rcx
   3c09a:	mov    0x218(%rsp),%r8
   3c0a2:	lea    0x310(%rsp),%r9
   3c0aa:	call   *0x110978(%rip)        # 14ca28 <_DYNAMIC+0x648>
   3c0b0:	mov    0x668(%rsp),%rsi
   3c0b8:	sub    %rsi,%rbp
   3c0bb:	jb     3c1af <meshopt_peers::run+0x204f>
   3c0c1:	mov    %rbp,0x758(%rsp)
   3c0c9:	add    0x660(%rsp),%r13
   3c0d1:	jb     3d401 <meshopt_peers::run+0x32a1>
   3c0d7:	mov    %r15,%rdi
   3c0da:	mov    %r13,%rsi
   3c0dd:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c0e2:	mov    %eax,%ebp
   3c0e4:	cmp    $0xff,%al
   3c0e6:	jne    3d404 <meshopt_peers::run+0x32a4>
   3c0ec:	movzbl 0x168(%rsp),%ebx
   3c0f4:	movzbl 0x169(%rsp),%ebp
   3c0fc:	lea    0x16a(%rsp),%rcx
   3c104:	mov    (%rcx),%rax
   3c107:	mov    0x6(%rcx),%rcx
   3c10b:	mov    %rax,0x230(%rsp)
   3c113:	mov    %rcx,0x236(%rsp)
   3c11b:	movss  0x178(%rsp),%xmm0
   3c124:	movaps %xmm0,0x90(%rsp)
   3c12c:	lea    0x310(%rsp),%rdi
   3c134:	call   36da0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3c139:	cmp    $0x1,%bl
   3c13c:	mov    (%rsp),%rbx
   3c140:	je     3d411 <meshopt_peers::run+0x32b1>
   3c146:	lea    0x232(%rsp),%rcx
   3c14e:	mov    0x8(%rcx),%eax
   3c151:	lea    0x744(%rsp),%rdx
   3c159:	mov    %eax,0x8(%rdx)
   3c15c:	mov    (%rcx),%rax
   3c15f:	mov    %rax,(%rdx)
   3c162:	mov    0x8(%rdx),%eax
   3c165:	mov    %eax,0x318(%rsp)
   3c16c:	mov    (%rdx),%rcx
   3c16f:	mov    %rcx,0x310(%rsp)
   3c177:	mov    0x30(%rsp),%rdx
   3c17c:	mov    %eax,0x24(%rdx,%r12,1)
   3c181:	mov    %rcx,0x1c(%rdx,%r12,1)
   3c186:	xorps  %xmm0,%xmm0
   3c189:	movaps 0x90(%rsp),%xmm1
   3c191:	movss  %xmm1,%xmm0
   3c195:	movlps %xmm0,0x28(%rdx,%r12,1)
   3c19b:	add    $0x30,%r12
   3c19f:	cmp    %r12,0x18(%rsp)
   3c1a4:	jne    3bf0e <meshopt_peers::run+0x1dae>
   3c1aa:	jmp    3c2b2 <meshopt_peers::run+0x2152>
   3c1af:	lea    0x758(%rsp),%rdi
   3c1b7:	call   *0x1107eb(%rip)        # 14c9a8 <_DYNAMIC+0x5c8>
   3c1bd:	mov    %eax,%ebp
   3c1bf:	cmp    $0xff,%al
   3c1c1:	je     3c0c9 <meshopt_peers::run+0x1f69>
   3c1c7:	jmp    3d404 <meshopt_peers::run+0x32a4>
   3c1cc:	mov    $0x1,%al
   3c1ce:	jmp    3bc9d <meshopt_peers::run+0x1b3d>
   3c1d3:	mov    %rdi,%r12
   3c1d6:	mov    %r8,0x90(%rsp)
   3c1de:	lea    0x750(%rsp),%rdi
   3c1e6:	mov    0x88(%rsp),%rsi
   3c1ee:	mov    0xe8(%rsp),%rdx
   3c1f6:	call   36140 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3c1fb:	mov    $0x1,%r13b
   3c1fe:	cmp    $0xff,%al
   3c200:	je     3c264 <meshopt_peers::run+0x2104>
   3c202:	mov    %eax,%ebp
   3c204:	movq   $0x0,0x40(%rsp)
   3c20d:	mov    $0x1,%r15b
   3c210:	mov    0x90(%rsp),%rcx
   3c218:	jmp    3d42b <meshopt_peers::run+0x32cb>
   3c21d:	lea    0x310(%rsp),%rdi
   3c225:	mov    $0x1,%edx
   3c22a:	mov    $0x1,%r8d
   3c230:	mov    $0x1,%r9d
   3c236:	xor    %esi,%esi
   3c238:	mov    %r14,%rbp
   3c23b:	mov    %r14,%rcx
   3c23e:	call   42400 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3c243:	cmpl   $0x1,0x310(%rsp)
   3c24b:	jne    3c36a <meshopt_peers::run+0x220a>
   3c251:	mov    %rbp,%r14
   3c254:	mov    $0xa,%bpl
   3c257:	jmp    3ba99 <meshopt_peers::run+0x1939>
   3c25c:	mov    $0x9,%bpl
   3c25f:	jmp    3d411 <meshopt_peers::run+0x32b1>
   3c264:	mov    %r12,0x18(%rsp)
   3c269:	mov    0x768(%rsp),%rax
   3c271:	mov    %r14,%r12
   3c274:	sub    %r14,%rax
   3c277:	jae    3c38d <meshopt_peers::run+0x222d>
   3c27d:	mov    0xa0(%rsp),%rdi
   3c285:	call   *0x11035d(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3c28b:	mov    $0x8,%bpl
   3c28e:	movq   $0x0,0x40(%rsp)
   3c297:	xor    %r15d,%r15d
   3c29a:	mov    0x18(%rsp),%rax
   3c29f:	mov    0x90(%rsp),%rcx
   3c2a7:	mov    %r12,%r14
   3c2aa:	mov    %rax,%r12
   3c2ad:	jmp    3c3e5 <meshopt_peers::run+0x2285>
   3c2b2:	mov    0x70(%rsp),%rdx
   3c2b7:	xor    %r13d,%r13d
   3c2ba:	cmp    $0x1,%rdx
   3c2be:	jne    3c3ed <meshopt_peers::run+0x228d>
   3c2c4:	mov    0x80(%rsp),%r12
   3c2cc:	mov    0x78(%rsp),%rax
   3c2d1:	mov    %rax,0x90(%rsp)
   3c2d9:	mov    0x30(%rsp),%rdi
   3c2de:	movups 0x1c(%rdi),%xmm0
   3c2e2:	movaps %xmm0,0x700(%rsp)
   3c2ea:	movl   $0x7f7fffff,0x710(%rsp)
   3c2f5:	lea    0xac(%rsp),%rax
   3c2fd:	lea    0x1b0(%rsp),%r10
   3c305:	lea    0x700(%rsp),%r11
   3c30d:	mov    $0x1,%esi
   3c312:	mov    0x20(%rsp),%rdx
   3c317:	mov    0x10(%rsp),%rcx
   3c31c:	mov    0x8(%rsp),%r8
   3c321:	mov    %r14,%r9
   3c324:	lea    0x750(%rsp),%r15
   3c32c:	push   %r15
   3c32e:	push   %rax
   3c32f:	push   %r10
   3c331:	push   %r13
   3c333:	push   %r11
   3c335:	push   $0x0
   3c337:	call   36e30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3c33c:	add    $0x30,%rsp
   3c340:	mov    %rax,%rbp
   3c343:	test   $0x1,%bpl
   3c347:	je     3c1de <meshopt_peers::run+0x207e>
   3c34d:	shr    $0x8,%ebp
   3c350:	mov    $0x1,%al
   3c352:	mov    %rax,0x40(%rsp)
   3c357:	mov    $0x1,%r15b
   3c35a:	mov    $0x1,%r13b
   3c35d:	mov    0x90(%rsp),%rcx
   3c365:	jmp    3d42b <meshopt_peers::run+0x32cb>
   3c36a:	mov    0x318(%rsp),%rdi
   3c372:	mov    %rdi,0xa0(%rsp)
   3c37a:	xor    %esi,%esi
   3c37c:	mov    %rbp,%r14
   3c37f:	mov    %rbp,%rdx
   3c382:	call   *0x110280(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   3c388:	jmp    3bc7a <meshopt_peers::run+0x1b1a>
   3c38d:	mov    %rax,0x768(%rsp)
   3c395:	mov    %r12,%r14
   3c398:	test   %r12,%r12
   3c39b:	je     3c3ab <meshopt_peers::run+0x224b>
   3c39d:	mov    0xa0(%rsp),%rdi
   3c3a5:	call   *0x11023d(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3c3ab:	movq   $0x0,0x40(%rsp)
   3c3b4:	lea    0x750(%rsp),%rdi
   3c3bc:	mov    $0x4,%edx
   3c3c1:	xor    %esi,%esi
   3c3c3:	call   360a0 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   3c3c8:	cmp    $0xff,%al
   3c3ca:	mov    0x18(%rsp),%rsi
   3c3cf:	je     3d18a <meshopt_peers::run+0x302a>
   3c3d5:	mov    %eax,%ebp
   3c3d7:	xor    %r15d,%r15d
   3c3da:	mov    %rsi,%r12
   3c3dd:	mov    0x90(%rsp),%rcx
   3c3e5:	mov    $0x1,%r13b
   3c3e8:	jmp    3d42b <meshopt_peers::run+0x32cb>
   3c3ed:	lea    0x168(%rsp),%rdi
   3c3f5:	mov    0x110634(%rip),%r10        # 14ca30 <_DYNAMIC+0x650>
   3c3fc:	mov    0x10(%rsp),%r8
   3c401:	movb   $0x2,0xf0(%rsp)
   3c409:	mov    0x8(%rsp),%rax
   3c40e:	mov    %rax,0xf8(%rsp)
   3c416:	mov    %r14,0x100(%rsp)
   3c41e:	movq   $0x0,0x128(%rsp)
   3c42a:	movups 0x208(%rsp),%xmm0
   3c432:	movaps %xmm0,0x350(%rsp)
   3c43a:	movups 0x1c8(%rsp),%xmm0
   3c442:	movups 0x1d8(%rsp),%xmm1
   3c44a:	movdqu 0x1e8(%rsp),%xmm2
   3c453:	movdqu 0x1f8(%rsp),%xmm3
   3c45c:	movdqa %xmm3,0x340(%rsp)
   3c465:	movdqa %xmm2,0x330(%rsp)
   3c46e:	movaps %xmm1,0x320(%rsp)
   3c476:	movaps %xmm0,0x310(%rsp)
   3c47e:	mov    0x30(%rsp),%rsi
   3c483:	mov    %rdx,0x70(%rsp)
   3c488:	mov    0x20(%rsp),%rcx
   3c48d:	mov    %r8,0x10(%rsp)
   3c492:	mov    0xe8(%rsp),%r9
   3c49a:	lea    0x750(%rsp),%rax
   3c4a2:	push   %rax
   3c4a3:	lea    0x318(%rsp),%rax
   3c4ab:	push   %rax
   3c4ac:	lea    0x100(%rsp),%rax
   3c4b4:	push   %rax
   3c4b5:	push   0x58(%rsp)
   3c4b9:	call   *%r10
   3c4bc:	add    $0x20,%rsp
   3c4c0:	mov    %r13,0x160(%rsp)
   3c4c8:	mov    0x168(%rsp),%r15
   3c4d0:	movzbl 0x170(%rsp),%ebp
   3c4d8:	cmp    $0xffffffffffffffff,%r15
   3c4dc:	je     3d411 <meshopt_peers::run+0x32b1>
   3c4e2:	movzbl %bpl,%eax
   3c4e6:	movzbl 0x177(%rsp),%ecx
   3c4ee:	shl    $0x10,%ecx
   3c4f1:	movzwl 0x175(%rsp),%edx
   3c4f9:	or     %ecx,%edx
   3c4fb:	mov    0x171(%rsp),%r8d
   3c503:	mov    0x178(%rsp),%r9
   3c50b:	shl    $0x28,%rdx
   3c50f:	shl    $0x8,%r8
   3c513:	or     %rdx,%r8
   3c516:	or     %rax,%r8
   3c519:	sub    $0x40,%rsp
   3c51d:	lea    0x790(%rsp),%rax
   3c525:	mov    %rax,0x30(%rsp)
   3c52a:	mov    0x80(%rsp),%rax
   3c532:	mov    %rax,0x18(%rsp)
   3c537:	mov    0x128(%rsp),%rax
   3c53f:	mov    %rax,0x10(%rsp)
   3c544:	mov    0x50(%rsp),%rax
   3c549:	mov    %rax,0x8(%rsp)
   3c54e:	mov    0x60(%rsp),%rax
   3c553:	mov    %rax,(%rsp)
   3c557:	movq   $0x0,0x20(%rsp)
   3c560:	mov    0xe0(%rsp),%rdi
   3c568:	mov    %r14,%rsi
   3c56b:	mov    0x70(%rsp),%rdx
   3c570:	mov    0xb0(%rsp),%rcx
   3c578:	mov    %r8,0x190(%rsp)
   3c580:	mov    %r9,0x120(%rsp)
   3c588:	call   *0x1104aa(%rip)        # 14ca38 <_DYNAMIC+0x658>
   3c58e:	add    $0x40,%rsp
   3c592:	mov    %eax,%ebp
   3c594:	mov    $0x1,%r12b
   3c597:	cmp    $0xff,%al
   3c599:	jne    3d1e6 <meshopt_peers::run+0x3086>
   3c59f:	mov    0x10(%rsp),%rax
   3c5a4:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   3c5ae:	mul    %rcx
   3c5b1:	mov    %rdx,%rax
   3c5b4:	shr    $1,%rax
   3c5b7:	mov    %rax,0x48(%rsp)
   3c5bc:	mov    $0x30,%ecx
   3c5c1:	mul    %rcx
   3c5c4:	seto   %cl
   3c5c7:	test   %rax,%rax
   3c5ca:	sets   %dl
   3c5cd:	or     %cl,%dl
   3c5cf:	mov    $0x8,%bpl
   3c5d2:	jne    3d1e6 <meshopt_peers::run+0x3086>
   3c5d8:	add    0x768(%rsp),%rax
   3c5e0:	jb     3d1e6 <meshopt_peers::run+0x3086>
   3c5e6:	mov    %rax,0x768(%rsp)
   3c5ee:	mov    0x750(%rsp),%r13
   3c5f6:	mov    %r13,%rdi
   3c5f9:	mov    %rax,%r12
   3c5fc:	mov    %rax,%rsi
   3c5ff:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c604:	mov    %eax,%ebp
   3c606:	mov    %r15,0x38(%rsp)
   3c60b:	cmp    $0xff,%al
   3c60d:	jne    3d1da <meshopt_peers::run+0x307a>
   3c613:	cmpq   $0x3,0x10(%rsp)
   3c619:	jae    3d0cc <meshopt_peers::run+0x2f6c>
   3c61f:	mov    $0x8,%r15d
   3c625:	xor    %eax,%eax
   3c627:	mov    %rax,0x18(%rsp)
   3c62c:	sub    0x48(%rsp),%rax
   3c631:	mov    $0x30,%ecx
   3c636:	mul    %rcx
   3c639:	seto   %cl
   3c63c:	test   %rax,%rax
   3c63f:	sets   %dl
   3c642:	or     %cl,%dl
   3c644:	mov    $0x8,%bpl
   3c647:	jne    3d1c9 <meshopt_peers::run+0x3069>
   3c64d:	test   %rax,%rax
   3c650:	je     3c678 <meshopt_peers::run+0x2518>
   3c652:	add    %r12,%rax
   3c655:	jb     3d1c9 <meshopt_peers::run+0x3069>
   3c65b:	mov    %rax,0x768(%rsp)
   3c663:	mov    %r13,%rdi
   3c666:	mov    %rax,%rsi
   3c669:	call   43110 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c66e:	mov    %eax,%ebp
   3c670:	cmp    $0xff,%al
   3c672:	jne    3d1c9 <meshopt_peers::run+0x3069>
   3c678:	mov    %r15,%rdx
   3c67b:	shr    $0x8,%rdx
   3c67f:	mov    %r15,%rax
   3c682:	shr    $0x28,%rax
   3c686:	mov    %r15,%rcx
   3c689:	shr    $0x38,%rcx
   3c68d:	mov    %cl,0xff(%rsp)
   3c694:	mov    %ax,0xfd(%rsp)
   3c69c:	mov    %edx,0xf9(%rsp)
   3c6a3:	movq   $0x0,0x100(%rsp)
   3c6af:	mov    0x18(%rsp),%rax
   3c6b4:	mov    %rax,0xf0(%rsp)
   3c6bc:	mov    %r15b,0xf8(%rsp)
   3c6c4:	cmpq   $0x2,0x10(%rsp)
   3c6ca:	ja     3c6d4 <meshopt_peers::run+0x2574>
   3c6cc:	mov    %rdx,%rcx
   3c6cf:	jmp    3c759 <meshopt_peers::run+0x25f9>
   3c6d4:	mov    0x48(%rsp),%rax
   3c6d9:	cmp    %rax,0x18(%rsp)
   3c6de:	jb     3d111 <meshopt_peers::run+0x2fb1>
   3c6e4:	xor    %ebx,%ebx
   3c6e6:	mov    0xf8(%rsp),%r15
   3c6ee:	lea    (%rbx,%rbx,2),%rax
   3c6f2:	shl    $0x4,%rax
   3c6f6:	xorps  %xmm0,%xmm0
   3c6f9:	movups %xmm0,0x20(%r15,%rax,1)
   3c6ff:	movups %xmm0,0x10(%r15,%rax,1)
   3c705:	movups %xmm0,(%r15,%rax,1)
   3c70a:	cmpq   $0x6,0x10(%rsp)
   3c710:	jb     3c733 <meshopt_peers::run+0x25d3>
   3c712:	lea    (%r15,%rax,1),%rdi
   3c716:	add    $0x30,%rdi
   3c71a:	mov    0x48(%rsp),%rax
   3c71f:	lea    (%rax,%rax,2),%rdx
   3c723:	shl    $0x4,%rdx
   3c727:	add    $0xffffffffffffffd0,%rdx
   3c72b:	xor    %esi,%esi
   3c72d:	call   *0x10fed5(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   3c733:	mov    0xf0(%rsp),%rax
   3c73b:	mov    %rax,0x18(%rsp)
   3c740:	cmp    $0xffffffffffffffff,%rax
   3c744:	je     3d573 <meshopt_peers::run+0x3413>
   3c74a:	mov    %r15,%rax
   3c74d:	shr    $0x8,%rax
   3c751:	mov    %rax,%rcx
   3c754:	add    %rbx,0x48(%rsp)
   3c759:	movzbl %r15b,%eax
   3c75d:	shl    $0x8,%rcx
   3c761:	or     %rax,%rcx
   3c764:	mov    %rcx,0xc8(%rsp)
   3c76c:	lea    0x310(%rsp),%rdi
   3c774:	lea    0x750(%rsp),%rsi
   3c77c:	mov    0x10(%rsp),%rdx
   3c781:	call   36190 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   3c786:	mov    0x310(%rsp),%rax
   3c78e:	movzbl 0x318(%rsp),%ebp
   3c796:	mov    %rax,0x90(%rsp)
   3c79e:	cmp    $0xffffffffffffffff,%rax
   3c7a2:	je     3d53f <meshopt_peers::run+0x33df>
   3c7a8:	lea    0x319(%rsp),%rcx
   3c7b0:	mov    (%rcx),%rax
   3c7b3:	mov    0x7(%rcx),%rcx
   3c7b7:	lea    0xf9(%rsp),%rdx
   3c7bf:	mov    %rcx,0x7(%rdx)
   3c7c3:	mov    %rax,(%rdx)
   3c7c6:	mov    0x90(%rsp),%rax
   3c7ce:	mov    %rax,0xf0(%rsp)
   3c7d6:	mov    %bpl,0xf8(%rsp)
   3c7de:	mov    0x100(%rsp),%rax
   3c7e6:	mov    0x10(%rsp),%r15
   3c7eb:	sub    %rax,%r15
   3c7ee:	jbe    3c848 <meshopt_peers::run+0x26e8>
   3c7f0:	mov    %rax,%rbx
   3c7f3:	mov    0x90(%rsp),%rax
   3c7fb:	sub    %rbx,%rax
   3c7fe:	cmp    %rax,%r15
   3c801:	ja     3d13d <meshopt_peers::run+0x2fdd>
   3c807:	mov    0xf8(%rsp),%rbp
   3c80f:	lea    0x0(,%rbx,4),%rdi
   3c817:	add    %rbp,%rdi
   3c81a:	lea    0x0(,%r15,4),%rdx
   3c822:	xor    %esi,%esi
   3c824:	call   *0x10fdde(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   3c82a:	cmpq   $0xffffffffffffffff,0x90(%rsp)
   3c833:	je     3d53f <meshopt_peers::run+0x33df>
   3c839:	add    %r15,%rbx
   3c83c:	mov    %rbp,%rax
   3c83f:	shr    $0x8,%rax
   3c843:	mov    %rax,%rcx
   3c846:	jmp    3c873 <meshopt_peers::run+0x2713>
   3c848:	movzbl 0xff(%rsp),%eax
   3c850:	shl    $0x10,%eax
   3c853:	movzwl 0xfd(%rsp),%ecx
   3c85b:	or     %eax,%ecx
   3c85d:	shl    $0x20,%rcx
   3c861:	mov    0xf9(%rsp),%eax
   3c868:	or     %rcx,%rax
   3c86b:	mov    %rax,%rcx
   3c86e:	mov    0x10(%rsp),%rbx
   3c873:	movzbl %bpl,%eax
   3c877:	shl    $0x8,%rcx
   3c87b:	or     %rax,%rcx
   3c87e:	mov    %rcx,0xd8(%rsp)
   3c886:	cmpq   $0x2,0xe0(%rsp)
   3c88f:	jb     3cfc9 <meshopt_peers::run+0x2e69>
   3c895:	mov    0xc8(%rsp),%rax
   3c89d:	add    $0x10,%rax
   3c8a1:	mov    %rax,0x308(%rsp)
   3c8a9:	movq   $0x0,0x68(%rsp)
   3c8b2:	xor    %r12d,%r12d
   3c8b5:	mov    0x150(%rsp),%rax
   3c8bd:	mov    %rax,0x158(%rsp)
   3c8c5:	mov    0x68(%rsp),%rax
   3c8ca:	mov    %rax,0xd0(%rsp)
   3c8d2:	addq   $0x4,0x158(%rsp)
   3c8db:	mov    0x158(%rsp),%rax
   3c8e3:	mov    -0x4(%rax),%edi
   3c8e6:	mov    (%rax),%r15d
   3c8e9:	cmp    %edi,%r15d
   3c8ec:	jb     3d7f3 <meshopt_peers::run+0x3693>
   3c8f2:	cmp    %r15,0x70(%rsp)
   3c8f7:	jb     3d7f3 <meshopt_peers::run+0x3693>
   3c8fd:	sub    %rdi,%r15
   3c900:	lea    (%rdi,%rdi,2),%r13
   3c904:	shl    $0x4,%r13
   3c908:	add    0x30(%rsp),%r13
   3c90d:	movq   $0xffffffffffffffff,0xf0(%rsp)
   3c919:	lea    0x310(%rsp),%rdi
   3c921:	mov    %r13,%rsi
   3c924:	mov    %r15,%rdx
   3c927:	mov    0x20(%rsp),%rcx
   3c92c:	mov    0x10(%rsp),%r8
   3c931:	lea    0x260(%rsp),%r9
   3c939:	lea    0x750(%rsp),%rax
   3c941:	push   %rax
   3c942:	lea    0x1d0(%rsp),%rax
   3c94a:	push   %rax
   3c94b:	push   %r14
   3c94d:	push   0xb8(%rsp)
   3c954:	call   *0x1100e6(%rip)        # 14ca40 <_DYNAMIC+0x660>
   3c95a:	add    $0x20,%rsp
   3c95e:	mov    0x310(%rsp),%rax
   3c966:	movzbl 0x318(%rsp),%ebp
   3c96e:	mov    %rax,0x60(%rsp)
   3c973:	cmp    $0xffffffffffffffff,%rax
   3c977:	je     3d4db <meshopt_peers::run+0x337b>
   3c97d:	movzbl %bpl,%eax
   3c981:	movzbl 0x31f(%rsp),%ecx
   3c989:	shl    $0x10,%ecx
   3c98c:	movzwl 0x31d(%rsp),%edx
   3c994:	or     %ecx,%edx
   3c996:	mov    0x319(%rsp),%esi
   3c99d:	mov    0x328(%rsp),%rcx
   3c9a5:	mov    %rcx,0x58(%rsp)
   3c9aa:	mov    0x330(%rsp),%rcx
   3c9b2:	mov    %rcx,0xb8(%rsp)
   3c9ba:	mov    0x340(%rsp),%rcx
   3c9c2:	mov    %rcx,0x28(%rsp)
   3c9c7:	mov    0x348(%rsp),%rcx
   3c9cf:	mov    %rcx,0x50(%rsp)
   3c9d4:	mov    0x350(%rsp),%rcx
   3c9dc:	mov    %rcx,0x68(%rsp)
   3c9e1:	lea    0x319(%rsp),%rcx
   3c9e9:	movups 0x3f(%rcx),%xmm0
   3c9ed:	movaps %xmm0,0x2f0(%rsp)
   3c9f5:	mov    0x4f(%rcx),%ecx
   3c9f8:	mov    %ecx,0x300(%rsp)
   3c9ff:	shl    $0x28,%rdx
   3ca03:	shl    $0x8,%rsi
   3ca07:	or     %rdx,%rsi
   3ca0a:	or     %rax,%rsi
   3ca0d:	mov    %rsi,0xc0(%rsp)
   3ca15:	testb  $0x1,0x36c(%rsp)
   3ca1d:	je     3cb9e <meshopt_peers::run+0x2a3e>
   3ca23:	mov    %r13,%rdi
   3ca26:	mov    %r15,%rsi
   3ca29:	mov    0x20(%rsp),%rdx
   3ca2e:	mov    0x10(%rsp),%rcx
   3ca33:	mov    0x8(%rsp),%r8
   3ca38:	mov    %r14,%r9
   3ca3b:	lea    0x750(%rsp),%rax
   3ca43:	push   %rax
   3ca44:	lea    0xb4(%rsp),%rax
   3ca4c:	push   %rax
   3ca4d:	lea    0x1c0(%rsp),%rax
   3ca55:	push   %rax
   3ca56:	push   0x178(%rsp)
   3ca5d:	lea    0x310(%rsp),%rax
   3ca65:	push   %rax
   3ca66:	push   $0x0
   3ca68:	call   36e30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3ca6d:	add    $0x30,%rsp
   3ca71:	mov    %rax,%rbp
   3ca74:	test   $0x1,%bpl
   3ca78:	jne    3d592 <meshopt_peers::run+0x3432>
   3ca7e:	mov    0x768(%rsp),%rcx
   3ca86:	mov    0x60(%rsp),%rdx
   3ca8b:	lea    0x0(,%rdx,4),%rax
   3ca93:	shr    $0x3e,%rdx
   3ca97:	sete   %dl
   3ca9a:	test   %rax,%rax
   3ca9d:	setns  %sil
   3caa1:	and    %dl,%sil
   3caa4:	sub    %rax,%rcx
   3caa7:	setae  %al
   3caaa:	test   %al,%sil
   3caad:	je     3d293 <meshopt_peers::run+0x3133>
   3cab3:	mov    %rcx,0x768(%rsp)
   3cabb:	cmpq   $0x0,0x60(%rsp)
   3cac1:	je     3cad9 <meshopt_peers::run+0x2979>
   3cac3:	mov    0xc0(%rsp),%rdi
   3cacb:	call   *0x10fb17(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cad1:	mov    0x768(%rsp),%rcx
   3cad9:	mov    0x58(%rsp),%rax
   3cade:	mov    $0x14,%edx
   3cae3:	mul    %rdx
   3cae6:	jo     3d4af <meshopt_peers::run+0x334f>
   3caec:	test   %rax,%rax
   3caef:	js     3d4af <meshopt_peers::run+0x334f>
   3caf5:	sub    %rax,%rcx
   3caf8:	jb     3d4af <meshopt_peers::run+0x334f>
   3cafe:	mov    %rcx,0x768(%rsp)
   3cb06:	cmpq   $0x0,0x58(%rsp)
   3cb0c:	je     3cb24 <meshopt_peers::run+0x29c4>
   3cb0e:	mov    0xb8(%rsp),%rdi
   3cb16:	call   *0x10facc(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cb1c:	mov    0x768(%rsp),%rcx
   3cb24:	movabs $0x3fffffffffffffff,%rax
   3cb2e:	cmp    %rax,0x28(%rsp)
   3cb33:	ja     3d55a <meshopt_peers::run+0x33fa>
   3cb39:	mov    0x28(%rsp),%rax
   3cb3e:	lea    0x0(,%rax,4),%rax
   3cb46:	test   %rax,%rax
   3cb49:	js     3d55a <meshopt_peers::run+0x33fa>
   3cb4f:	sub    %rax,%rcx
   3cb52:	jb     3d55a <meshopt_peers::run+0x33fa>
   3cb58:	mov    %rcx,0x768(%rsp)
   3cb60:	cmpq   $0x0,0x28(%rsp)
   3cb66:	je     3cb73 <meshopt_peers::run+0x2a13>
   3cb68:	mov    0x50(%rsp),%rdi
   3cb6d:	call   *0x10fa75(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cb73:	mov    0xe0(%rsp),%rax
   3cb7b:	dec    %rax
   3cb7e:	addq   $0x4,0x158(%rsp)
   3cb87:	mov    %rax,0xe0(%rsp)
   3cb8f:	cmp    $0x1,%rax
   3cb93:	ja     3c8db <meshopt_peers::run+0x277b>
   3cb99:	jmp    3d178 <meshopt_peers::run+0x3018>
   3cb9e:	mov    %r13,%rdi
   3cba1:	mov    %r15,%rsi
   3cba4:	mov    0x20(%rsp),%rdx
   3cba9:	mov    0x10(%rsp),%rcx
   3cbae:	mov    0x8(%rsp),%r8
   3cbb3:	mov    %r14,%r9
   3cbb6:	lea    0x750(%rsp),%rax
   3cbbe:	push   %rax
   3cbbf:	lea    0xb4(%rsp),%rax
   3cbc7:	push   %rax
   3cbc8:	lea    0x1c0(%rsp),%rax
   3cbd0:	push   %rax
   3cbd1:	push   0x178(%rsp)
   3cbd8:	lea    0x310(%rsp),%rax
   3cbe0:	push   %rax
   3cbe1:	push   $0x0
   3cbe3:	call   36e30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3cbe8:	add    $0x30,%rsp
   3cbec:	mov    %rax,%rbp
   3cbef:	test   $0x1,%bpl
   3cbf3:	jne    3d592 <meshopt_peers::run+0x3432>
   3cbf9:	shr    $0x20,%rbp
   3cbfd:	movb   $0x2,0x168(%rsp)
   3cc05:	mov    0x8(%rsp),%rax
   3cc0a:	mov    %rax,0x170(%rsp)
   3cc12:	mov    %r14,0x178(%rsp)
   3cc1a:	movq   $0x0,0x1a0(%rsp)
   3cc26:	movups 0x208(%rsp),%xmm0
   3cc2e:	movaps %xmm0,0x350(%rsp)
   3cc36:	movups 0x1c8(%rsp),%xmm0
   3cc3e:	movups 0x1d8(%rsp),%xmm1
   3cc46:	movdqu 0x1e8(%rsp),%xmm2
   3cc4f:	movdqu 0x1f8(%rsp),%xmm3
   3cc58:	movdqa %xmm3,0x340(%rsp)
   3cc61:	movdqa %xmm2,0x330(%rsp)
   3cc6a:	movaps %xmm1,0x320(%rsp)
   3cc72:	movaps %xmm0,0x310(%rsp)
   3cc7a:	lea    0x230(%rsp),%rdi
   3cc82:	mov    0x50(%rsp),%rsi
   3cc87:	mov    0x68(%rsp),%rdx
   3cc8c:	lea    0x168(%rsp),%rcx
   3cc94:	lea    0x310(%rsp),%r8
   3cc9c:	mov    %ebp,%r9d
   3cc9f:	lea    0x750(%rsp),%rax
   3cca7:	push   %rax
   3cca8:	push   0x220(%rsp)
   3ccaf:	call   *0x10fd6b(%rip)        # 14ca20 <_DYNAMIC+0x640>
   3ccb5:	add    $0x10,%rsp
   3ccb9:	mov    0x230(%rsp),%r15
   3ccc1:	movzbl 0x238(%rsp),%ebp
   3ccc9:	cmp    $0xffffffffffffffff,%r15
   3cccd:	je     3d595 <meshopt_peers::run+0x3435>
   3ccd3:	movzbl %bpl,%ecx
   3ccd7:	movzbl 0x23f(%rsp),%eax
   3ccdf:	shl    $0x10,%eax
   3cce2:	movzwl 0x23d(%rsp),%esi
   3ccea:	or     %eax,%esi
   3ccec:	mov    0x239(%rsp),%r13d
   3ccf4:	mov    0x240(%rsp),%rax
   3ccfc:	mov    0x248(%rsp),%rbp
   3cd04:	mov    0x250(%rsp),%rdx
   3cd0c:	mov    %rdx,0xb0(%rsp)
   3cd14:	mov    0x258(%rsp),%rdx
   3cd1c:	shl    $0x28,%rsi
   3cd20:	shl    $0x8,%r13
   3cd24:	or     %rsi,%r13
   3cd27:	or     %rcx,%r13
   3cd2a:	test   %rax,%rax
   3cd2d:	je     3cdd6 <meshopt_peers::run+0x2c76>
   3cd33:	lea    (%rax,%rax,2),%rax
   3cd37:	shl    $0x4,%rax
   3cd3b:	add    %r13,%rax
   3cd3e:	lea    (%r12,%r12,2),%rcx
   3cd42:	shl    $0x4,%rcx
   3cd46:	add    0x308(%rsp),%rcx
   3cd4e:	xor    %esi,%esi
   3cd50:	mov    0x0(%r13,%rsi,1),%rdi
   3cd55:	mov    0x8(%r13,%rsi,1),%r8
   3cd5a:	movups 0x10(%r13,%rsi,1),%xmm0
   3cd60:	movaps %xmm0,0x720(%rsp)
   3cd68:	mov    0x300(%rsp),%r9d
   3cd70:	lea    0x72c(%rsp),%r10
   3cd78:	mov    %r9d,0x10(%r10)
   3cd7c:	movaps 0x2f0(%rsp),%xmm0
   3cd84:	movups %xmm0,(%r10)
   3cd88:	cmp    0x48(%rsp),%r12
   3cd8d:	jae    3d964 <meshopt_peers::run+0x3804>
   3cd93:	lea    (%rsi,%r13,1),%r9
   3cd97:	add    0xd0(%rsp),%r8
   3cd9f:	mov    %rdi,-0x10(%rcx,%rsi,1)
   3cda4:	mov    %r8,-0x8(%rcx,%rsi,1)
   3cda9:	movaps 0x720(%rsp),%xmm0
   3cdb1:	movaps 0x730(%rsp),%xmm1
   3cdb9:	movups %xmm0,(%rcx,%rsi,1)
   3cdbd:	movups %xmm1,0x10(%rcx,%rsi,1)
   3cdc2:	inc    %r12
   3cdc5:	add    $0x30,%rsi
   3cdc9:	add    $0x30,%r9
   3cdcd:	cmp    %rax,%r9
   3cdd0:	jne    3cd50 <meshopt_peers::run+0x2bf0>
   3cdd6:	mov    %rdx,%rax
   3cdd9:	add    0xd0(%rsp),%rax
   3cde1:	mov    %rax,0x68(%rsp)
   3cde6:	jb     3d8d6 <meshopt_peers::run+0x3776>
   3cdec:	cmp    %rbx,0x68(%rsp)
   3cdf1:	ja     3d8d6 <meshopt_peers::run+0x3776>
   3cdf7:	mov    0xd8(%rsp),%rax
   3cdff:	mov    0xd0(%rsp),%rcx
   3ce07:	lea    (%rax,%rcx,4),%rdi
   3ce0b:	shl    $0x2,%rdx
   3ce0f:	mov    0xb0(%rsp),%rsi
   3ce17:	call   *0x10f7db(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   3ce1d:	mov    0x768(%rsp),%rcx
   3ce25:	mov    %r15,%rax
   3ce28:	mov    $0x30,%edx
   3ce2d:	mul    %rdx
   3ce30:	seto   %dl
   3ce33:	not    %dl
   3ce35:	test   %rax,%rax
   3ce38:	setns  %sil
   3ce3c:	and    %dl,%sil
   3ce3f:	sub    %rax,%rcx
   3ce42:	setae  %al
   3ce45:	test   %al,%sil
   3ce48:	je     3d5f2 <meshopt_peers::run+0x3492>
   3ce4e:	mov    %rcx,0x768(%rsp)
   3ce56:	test   %r15,%r15
   3ce59:	je     3ce6c <meshopt_peers::run+0x2d0c>
   3ce5b:	mov    %r13,%rdi
   3ce5e:	call   *0x10f784(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3ce64:	mov    0x768(%rsp),%rcx
   3ce6c:	mov    %rbp,%rax
   3ce6f:	shr    $0x3e,%rax
   3ce73:	jne    3d600 <meshopt_peers::run+0x34a0>
   3ce79:	lea    0x0(,%rbp,4),%rax
   3ce81:	test   %rax,%rax
   3ce84:	js     3d600 <meshopt_peers::run+0x34a0>
   3ce8a:	sub    %rax,%rcx
   3ce8d:	jb     3d600 <meshopt_peers::run+0x34a0>
   3ce93:	mov    %rcx,0x768(%rsp)
   3ce9b:	test   %rbp,%rbp
   3ce9e:	je     3ceb6 <meshopt_peers::run+0x2d56>
   3cea0:	mov    0xb0(%rsp),%rdi
   3cea8:	call   *0x10f73a(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3ceae:	mov    0x768(%rsp),%rcx
   3ceb6:	movabs $0x3fffffffffffffff,%rax
   3cec0:	cmp    %rax,0x60(%rsp)
   3cec5:	ja     3d77b <meshopt_peers::run+0x361b>
   3cecb:	mov    0x60(%rsp),%rax
   3ced0:	lea    0x0(,%rax,4),%rax
   3ced8:	test   %rax,%rax
   3cedb:	js     3d77b <meshopt_peers::run+0x361b>
   3cee1:	sub    %rax,%rcx
   3cee4:	jb     3d77b <meshopt_peers::run+0x361b>
   3ceea:	mov    %rcx,0x768(%rsp)
   3cef2:	cmpq   $0x0,0x60(%rsp)
   3cef8:	je     3cf10 <meshopt_peers::run+0x2db0>
   3cefa:	mov    0xc0(%rsp),%rdi
   3cf02:	call   *0x10f6e0(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cf08:	mov    0x768(%rsp),%rcx
   3cf10:	mov    0x58(%rsp),%rax
   3cf15:	mov    $0x14,%edx
   3cf1a:	mul    %rdx
   3cf1d:	jo     3d88e <meshopt_peers::run+0x372e>
   3cf23:	test   %rax,%rax
   3cf26:	js     3d88e <meshopt_peers::run+0x372e>
   3cf2c:	sub    %rax,%rcx
   3cf2f:	jb     3d88e <meshopt_peers::run+0x372e>
   3cf35:	mov    %rcx,0x768(%rsp)
   3cf3d:	cmpq   $0x0,0x58(%rsp)
   3cf43:	je     3cf5b <meshopt_peers::run+0x2dfb>
   3cf45:	mov    0xb8(%rsp),%rdi
   3cf4d:	call   *0x10f695(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cf53:	mov    0x768(%rsp),%rcx
   3cf5b:	movabs $0x3fffffffffffffff,%rax
   3cf65:	cmp    %rax,0x28(%rsp)
   3cf6a:	ja     3d8c2 <meshopt_peers::run+0x3762>
   3cf70:	mov    0x28(%rsp),%rax
   3cf75:	lea    0x0(,%rax,4),%rax
   3cf7d:	test   %rax,%rax
   3cf80:	js     3d8c2 <meshopt_peers::run+0x3762>
   3cf86:	sub    %rax,%rcx
   3cf89:	jb     3d8c2 <meshopt_peers::run+0x3762>
   3cf8f:	mov    %rcx,0x768(%rsp)
   3cf97:	cmpq   $0x0,0x28(%rsp)
   3cf9d:	je     3cfaa <meshopt_peers::run+0x2e4a>
   3cf9f:	mov    0x50(%rsp),%rdi
   3cfa4:	call   *0x10f63e(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3cfaa:	mov    0xe0(%rsp),%rax
   3cfb2:	dec    %rax
   3cfb5:	mov    %rax,0xe0(%rsp)
   3cfbd:	cmp    $0x1,%rax
   3cfc1:	ja     3c8c5 <meshopt_peers::run+0x2765>
   3cfc7:	jmp    3cfd5 <meshopt_peers::run+0x2e75>
   3cfc9:	xor    %r12d,%r12d
   3cfcc:	movq   $0x0,0x68(%rsp)
   3cfd5:	lea    0x750(%rsp),%rdi
   3cfdd:	mov    0x38(%rsp),%rsi
   3cfe2:	mov    0x150(%rsp),%rdx
   3cfea:	call   36140 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3cfef:	mov    %eax,%ebp
   3cff1:	mov    $0x1,%al
   3cff3:	mov    %eax,0x70(%rsp)
   3cff7:	cmp    $0xff,%bpl
   3cffb:	setne  %r13b
   3cfff:	jne    3d58a <meshopt_peers::run+0x342a>
   3d005:	mov    0x48(%rsp),%r15
   3d00a:	cmp    %r15,%r12
   3d00d:	cmovb  %r12,%r15
   3d011:	mov    0x68(%rsp),%rax
   3d016:	cmp    %rbx,%rax
   3d019:	cmovb  %rax,%rbx
   3d01d:	mov    %rbx,0x10(%rsp)
   3d022:	lea    0x750(%rsp),%rdi
   3d02a:	mov    0x80(%rsp),%rsi
   3d032:	mov    0x30(%rsp),%rdx
   3d037:	call   360f0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3d03c:	cmp    $0xff,%al
   3d03e:	jne    3d5a9 <meshopt_peers::run+0x3449>
   3d044:	lea    0x750(%rsp),%rdi
   3d04c:	mov    0x78(%rsp),%rsi
   3d051:	mov    0x20(%rsp),%rdx
   3d056:	call   36140 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3d05b:	cmp    $0xff,%al
   3d05d:	jne    3d5b3 <meshopt_peers::run+0x3453>
   3d063:	mov    0x160(%rsp),%r13
   3d06b:	inc    %r13d
   3d06e:	mov    0x18(%rsp),%rax
   3d073:	mov    %rax,0x80(%rsp)
   3d07b:	mov    %r15,%rdx
   3d07e:	mov    0xc8(%rsp),%rax
   3d086:	mov    %rax,0x30(%rsp)
   3d08b:	mov    0x10(%rsp),%r8
   3d090:	mov    0xd8(%rsp),%rax
   3d098:	mov    %rax,0x20(%rsp)
   3d09d:	mov    0x90(%rsp),%rax
   3d0a5:	mov    %rax,0x78(%rsp)
   3d0aa:	cmp    $0x1,%r15
   3d0ae:	mov    (%rsp),%rbx
   3d0b2:	lea    0x168(%rsp),%rdi
   3d0ba:	mov    0x10f96f(%rip),%r10        # 14ca30 <_DYNAMIC+0x650>
   3d0c1:	ja     3c401 <meshopt_peers::run+0x22a1>
   3d0c7:	jmp    3d5c5 <meshopt_peers::run+0x3465>
   3d0cc:	mov    $0x8,%edx
   3d0d1:	mov    $0x8,%r8d
   3d0d7:	mov    $0x30,%r9d
   3d0dd:	lea    0x310(%rsp),%rdi
   3d0e5:	xor    %esi,%esi
   3d0e7:	mov    0x48(%rsp),%rcx
   3d0ec:	call   42400 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3d0f1:	cmpl   $0x1,0x310(%rsp)
   3d0f9:	je     3d642 <meshopt_peers::run+0x34e2>
   3d0ff:	mov    0x318(%rsp),%r15
   3d107:	mov    0x48(%rsp),%rax
   3d10c:	jmp    3c627 <meshopt_peers::run+0x24c7>
   3d111:	mov    $0x8,%ecx
   3d116:	mov    $0x30,%r8d
   3d11c:	lea    0xf0(%rsp),%rdi
   3d124:	xor    %esi,%esi
   3d126:	mov    0x48(%rsp),%rdx
   3d12b:	call   39f20 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   3d130:	mov    0x100(%rsp),%rbx
   3d138:	jmp    3c6e6 <meshopt_peers::run+0x2586>
   3d13d:	mov    $0x4,%ecx
   3d142:	mov    $0x4,%r8d
   3d148:	lea    0xf0(%rsp),%rdi
   3d150:	mov    %rbx,%rsi
   3d153:	mov    %r15,%rdx
   3d156:	call   39f20 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   3d15b:	mov    0xf0(%rsp),%rax
   3d163:	mov    %rax,0x90(%rsp)
   3d16b:	mov    0x100(%rsp),%rbx
   3d173:	jmp    3c807 <meshopt_peers::run+0x26a7>
   3d178:	mov    0xd0(%rsp),%rax
   3d180:	mov    %rax,0x68(%rsp)
   3d185:	jmp    3cfd5 <meshopt_peers::run+0x2e75>
   3d18a:	lea    0x750(%rsp),%rdi
   3d192:	mov    0x30(%rsp),%rdx
   3d197:	call   360f0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3d19c:	cmp    $0xff,%al
   3d19e:	jne    3d8f8 <meshopt_peers::run+0x3798>
   3d1a4:	lea    0x750(%rsp),%rdi
   3d1ac:	mov    0x90(%rsp),%rsi
   3d1b4:	mov    0x20(%rsp),%rdx
   3d1b9:	call   36140 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3d1be:	mov    %eax,%ebp
   3d1c0:	mov    (%rsp),%rbx
   3d1c4:	jmp    3bab2 <meshopt_peers::run+0x1952>
   3d1c9:	cmpq   $0x0,0x18(%rsp)
   3d1cf:	je     3d1da <meshopt_peers::run+0x307a>
   3d1d1:	mov    %r15,%rdi
   3d1d4:	call   *0x10f40e(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d1da:	mov    (%rsp),%rbx
   3d1de:	mov    0x38(%rsp),%r15
   3d1e3:	mov    $0x1,%r12b
   3d1e6:	mov    $0x1,%r13b
   3d1e9:	mov    %r13b,(%rsp)
   3d1ed:	test   %r15,%r15
   3d1f0:	je     3d22a <meshopt_peers::run+0x30ca>
   3d1f2:	mov    0x150(%rsp),%rdi
   3d1fa:	call   *0x10f3e8(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d200:	mov    $0x1,%al
   3d202:	mov    %rax,0x40(%rsp)
   3d207:	mov    $0x1,%r15b
   3d20a:	test   %r12b,%r12b
   3d20d:	mov    0x80(%rsp),%r12
   3d215:	mov    0x78(%rsp),%rcx
   3d21a:	movzbl (%rsp),%r13d
   3d21f:	jne    3d42b <meshopt_peers::run+0x32cb>
   3d225:	jmp    3d43b <meshopt_peers::run+0x32db>
   3d22a:	movzbl (%rsp),%r13d
   3d22f:	mov    $0x1,%cl
   3d231:	mov    %rcx,0x40(%rsp)
   3d236:	mov    $0x1,%r15b
   3d239:	test   %r12b,%r12b
   3d23c:	mov    0x80(%rsp),%r12
   3d244:	mov    0x78(%rsp),%rcx
   3d249:	jne    3d42b <meshopt_peers::run+0x32cb>
   3d24f:	jmp    3d43b <meshopt_peers::run+0x32db>
   3d254:	movzbl 0x758(%rsp),%eax
   3d25c:	mov    %al,0x310(%rsp)
   3d263:	lea    -0x2bddf(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d26a:	lea    0x1078cf(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d271:	lea    0x107550(%rip),%r8        # 1447c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x398>
   3d278:	lea    0x310(%rsp),%rdx
   3d280:	mov    $0x2b,%esi
   3d285:	call   *0x10f425(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d28b:	mov    $0x8,%bpl
   3d28e:	jmp    3ba99 <meshopt_peers::run+0x1939>
   3d293:	mov    $0x8,%bpl
   3d296:	cmpq   $0x0,0x60(%rsp)
   3d29c:	je     3d46b <meshopt_peers::run+0x330b>
   3d2a2:	mov    0xc0(%rsp),%rdi
   3d2aa:	call   *0x10f338(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d2b0:	xor    %r15d,%r15d
   3d2b3:	cmpq   $0x0,0x28(%rsp)
   3d2b9:	jne    3d476 <meshopt_peers::run+0x3316>
   3d2bf:	jmp    3d481 <meshopt_peers::run+0x3321>
   3d2c4:	movzbl 0x758(%rsp),%eax
   3d2cc:	mov    %al,0x310(%rsp)
   3d2d3:	lea    -0x2be4f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d2da:	lea    0x10785f(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d2e1:	lea    0x1074b0(%rip),%r8        # 144798 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x368>
   3d2e8:	lea    0x310(%rsp),%rdx
   3d2f0:	mov    $0x2b,%esi
   3d2f5:	call   *0x10f3b5(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d2fb:	movzbl 0x758(%rsp),%eax
   3d303:	mov    %al,0x310(%rsp)
   3d30a:	lea    -0x2be86(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d311:	lea    0x107828(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d318:	lea    0x1073a1(%rip),%r8        # 1446c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x290>
   3d31f:	lea    0x310(%rsp),%rdx
   3d327:	mov    $0x2b,%esi
   3d32c:	call   *0x10f37e(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d332:	movzbl 0x758(%rsp),%eax
   3d33a:	mov    %al,0x310(%rsp)
   3d341:	lea    -0x2bebd(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d348:	lea    0x1077f1(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d34f:	lea    0x1074a2(%rip),%r8        # 1447f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3c8>
   3d356:	lea    0x310(%rsp),%rdx
   3d35e:	mov    $0x2b,%esi
   3d363:	call   *0x10f347(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d369:	movzbl 0x311(%rsp),%eax
   3d371:	mov    %al,0x260(%rsp)
   3d378:	lea    -0x2bef4(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d37f:	lea    0x1077ba(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d386:	lea    0x1074e3(%rip),%r8        # 144870 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x440>
   3d38d:	lea    0x260(%rsp),%rdx
   3d395:	mov    $0x2b,%esi
   3d39a:	call   *0x10f310(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d3a0:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d3a5:	lea    0x1071f4(%rip),%rcx        # 1445a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x170>
   3d3ac:	mov    0x10(%rsp),%rdx
   3d3b1:	call   *0x10f399(%rip)        # 14c750 <_DYNAMIC+0x370>
   3d3b7:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d3bc:	mov    (%rsp),%rbx
   3d3c0:	mov    0x8(%rsp),%r11
   3d3c5:	jmp    3ddc1 <meshopt_peers::run+0x3c61>
   3d3ca:	movzbl 0x318(%rsp),%eax
   3d3d2:	mov    %al,0x750(%rsp)
   3d3d9:	lea    -0x2bf55(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d3e0:	lea    0x107759(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d3e7:	lea    0x107302(%rip),%r8        # 1446f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2c0>
   3d3ee:	lea    0x750(%rsp),%rdx
   3d3f6:	mov    $0x2b,%esi
   3d3fb:	call   *0x10f2af(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d401:	mov    $0x8,%bpl
   3d404:	lea    0x310(%rsp),%rdi
   3d40c:	call   36da0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3d411:	mov    $0x1,%al
   3d413:	mov    %rax,0x40(%rsp)
   3d418:	mov    $0x1,%r15b
   3d41b:	mov    $0x1,%r13b
   3d41e:	mov    0x80(%rsp),%r12
   3d426:	mov    0x78(%rsp),%rcx
   3d42b:	test   %rcx,%rcx
   3d42e:	je     3d43b <meshopt_peers::run+0x32db>
   3d430:	mov    0x20(%rsp),%rdi
   3d435:	call   *0x10f1ad(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d43b:	test   %r13b,%r13b
   3d43e:	je     3d450 <meshopt_peers::run+0x32f0>
   3d440:	test   %r12,%r12
   3d443:	je     3d450 <meshopt_peers::run+0x32f0>
   3d445:	mov    0x30(%rsp),%rdi
   3d44a:	call   *0x10f198(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d450:	test   %r15b,%r15b
   3d453:	mov    0x40(%rsp),%rax
   3d458:	jne    3bc9d <meshopt_peers::run+0x1b3d>
   3d45e:	test   %al,%al
   3d460:	jne    3ba99 <meshopt_peers::run+0x1939>
   3d466:	jmp    3bab2 <meshopt_peers::run+0x1952>
   3d46b:	xor    %r15d,%r15d
   3d46e:	cmpq   $0x0,0x28(%rsp)
   3d474:	je     3d481 <meshopt_peers::run+0x3321>
   3d476:	mov    0x50(%rsp),%rdi
   3d47b:	call   *0x10f167(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d481:	cmpq   $0x0,0x58(%rsp)
   3d487:	je     3d497 <meshopt_peers::run+0x3337>
   3d489:	mov    0xb8(%rsp),%rdi
   3d491:	call   *0x10f151(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d497:	cmpq   $0x0,0x60(%rsp)
   3d49d:	setne  %al
   3d4a0:	test   %r15b,%al
   3d4a3:	je     3d4db <meshopt_peers::run+0x337b>
   3d4a5:	mov    0xc0(%rsp),%rdi
   3d4ad:	jmp    3d4d5 <meshopt_peers::run+0x3375>
   3d4af:	cmpq   $0x0,0x58(%rsp)
   3d4b5:	je     3d4c5 <meshopt_peers::run+0x3365>
   3d4b7:	mov    0xb8(%rsp),%rdi
   3d4bf:	call   *0x10f123(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d4c5:	mov    $0x8,%bpl
   3d4c8:	cmpq   $0x0,0x28(%rsp)
   3d4ce:	je     3d4db <meshopt_peers::run+0x337b>
   3d4d0:	mov    0x50(%rsp),%rdi
   3d4d5:	call   *0x10f10d(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d4db:	lea    0xf0(%rsp),%rdi
   3d4e3:	call   367b0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3d4e8:	mov    $0x1,%r15b
   3d4eb:	mov    $0x1,%al
   3d4ed:	mov    %eax,0x70(%rsp)
   3d4f1:	mov    $0x1,%r13b
   3d4f4:	cmpq   $0x0,0x90(%rsp)
   3d4fd:	je     3d50d <meshopt_peers::run+0x33ad>
   3d4ff:	mov    0xd8(%rsp),%rdi
   3d507:	call   *0x10f0db(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d50d:	cmpq   $0x0,0x18(%rsp)
   3d513:	je     3d523 <meshopt_peers::run+0x33c3>
   3d515:	mov    0xc8(%rsp),%rdi
   3d51d:	call   *0x10f0c5(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d523:	test   %r15b,%r15b
   3d526:	mov    (%rsp),%rbx
   3d52a:	mov    0x38(%rsp),%r15
   3d52f:	mov    0x70(%rsp),%r12d
   3d534:	jne    3d1e9 <meshopt_peers::run+0x3089>
   3d53a:	jmp    3d22f <meshopt_peers::run+0x30cf>
   3d53f:	cmpq   $0x0,0x18(%rsp)
   3d545:	je     3d56e <meshopt_peers::run+0x340e>
   3d547:	mov    0xc8(%rsp),%rdi
   3d54f:	call   *0x10f093(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d555:	mov    $0x1,%r13b
   3d558:	jmp    3d579 <meshopt_peers::run+0x3419>
   3d55a:	mov    $0x8,%bpl
   3d55d:	cmpq   $0x0,0x28(%rsp)
   3d563:	je     3d4db <meshopt_peers::run+0x337b>
   3d569:	jmp    3d4d0 <meshopt_peers::run+0x3370>
   3d56e:	mov    $0x1,%r13b
   3d571:	jmp    3d579 <meshopt_peers::run+0x3419>
   3d573:	mov    $0x1,%r13b
   3d576:	mov    %r15d,%ebp
   3d579:	mov    (%rsp),%rbx
   3d57d:	mov    0x38(%rsp),%r15
   3d582:	mov    $0x1,%r12b
   3d585:	jmp    3d1e9 <meshopt_peers::run+0x3089>
   3d58a:	xor    %r15d,%r15d
   3d58d:	jmp    3d4f4 <meshopt_peers::run+0x3394>
   3d592:	shr    $0x8,%ebp
   3d595:	mov    $0x1,%r15b
   3d598:	cmpq   $0x0,0x28(%rsp)
   3d59e:	jne    3d476 <meshopt_peers::run+0x3316>
   3d5a4:	jmp    3d481 <meshopt_peers::run+0x3321>
   3d5a9:	mov    %eax,%ebp
   3d5ab:	xor    %r15d,%r15d
   3d5ae:	jmp    3d4f4 <meshopt_peers::run+0x3394>
   3d5b3:	mov    %eax,%ebp
   3d5b5:	xor    %r15d,%r15d
   3d5b8:	movl   $0x0,0x70(%rsp)
   3d5c0:	jmp    3d4f4 <meshopt_peers::run+0x3394>
   3d5c5:	mov    0xc8(%rsp),%rax
   3d5cd:	mov    %rax,0x30(%rsp)
   3d5d2:	mov    0xd8(%rsp),%rax
   3d5da:	mov    %rax,0x20(%rsp)
   3d5df:	test   %r15,%r15
   3d5e2:	mov    0x18(%rsp),%r12
   3d5e7:	jne    3c2d9 <meshopt_peers::run+0x2179>
   3d5ed:	jmp    3c1de <meshopt_peers::run+0x207e>
   3d5f2:	test   %r15,%r15
   3d5f5:	je     3d600 <meshopt_peers::run+0x34a0>
   3d5f7:	mov    %r13,%rdi
   3d5fa:	call   *0x10efe8(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d600:	mov    $0x1,%r13b
   3d603:	mov    $0x1,%r15b
   3d606:	test   %rbp,%rbp
   3d609:	je     3d61b <meshopt_peers::run+0x34bb>
   3d60b:	mov    0xb0(%rsp),%rdi
   3d613:	call   *0x10efcf(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d619:	jmp    3d61e <meshopt_peers::run+0x34be>
   3d61b:	mov    $0x1,%r15b
   3d61e:	cmpq   $0x0,0x28(%rsp)
   3d624:	je     3d631 <meshopt_peers::run+0x34d1>
   3d626:	mov    0x50(%rsp),%rdi
   3d62b:	call   *0x10efb7(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d631:	mov    $0x8,%bpl
   3d634:	test   %r13b,%r13b
   3d637:	jne    3d481 <meshopt_peers::run+0x3321>
   3d63d:	jmp    3d497 <meshopt_peers::run+0x3337>
   3d642:	mov    $0xa,%bpl
   3d645:	jmp    3d1da <meshopt_peers::run+0x307a>
   3d64a:	movb   $0x8,0x750(%rsp)
   3d652:	lea    -0x2c1ce(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d659:	lea    0x1074e0(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d660:	lea    0x107011(%rip),%r8        # 144678 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x248>
   3d667:	lea    0x750(%rsp),%rdx
   3d66f:	mov    $0x2b,%esi
   3d674:	call   *0x10f036(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d67a:	movb   $0x8,0x750(%rsp)
   3d682:	lea    -0x2c1fe(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d689:	lea    0x1074b0(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d690:	lea    0x107089(%rip),%r8        # 144720 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2f0>
   3d697:	lea    0x750(%rsp),%rdx
   3d69f:	mov    $0x2b,%esi
   3d6a4:	call   *0x10f006(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d6aa:	movb   $0x2,0x750(%rsp)
   3d6b2:	lea    -0x2c22e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d6b9:	lea    0x107480(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d6c0:	lea    0x1071c1(%rip),%r8        # 144888 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x458>
   3d6c7:	lea    0x750(%rsp),%rdx
   3d6cf:	mov    $0x2b,%esi
   3d6d4:	call   *0x10efd6(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d6da:	movb   $0x2,0x750(%rsp)
   3d6e2:	lea    -0x2c25e(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d6e9:	lea    0x107450(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d6f0:	lea    0x106f99(%rip),%r8        # 144690 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x260>
   3d6f7:	lea    0x750(%rsp),%rdx
   3d6ff:	mov    $0x2b,%esi
   3d704:	call   *0x10efa6(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d70a:	mov    $0x4,%edi
   3d70f:	mov    %r15,%rsi
   3d712:	call   *0x10ef08(%rip)        # 14c620 <_DYNAMIC+0x240>
   3d718:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d71d:	mov    %al,0x750(%rsp)
   3d724:	lea    -0x2c2a0(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d72b:	lea    0x10740e(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d732:	lea    0x107107(%rip),%r8        # 144840 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x410>
   3d739:	lea    0x750(%rsp),%rdx
   3d741:	mov    $0x2b,%esi
   3d746:	call   *0x10ef64(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d74c:	mov    %al,0x750(%rsp)
   3d753:	lea    -0x2c2cf(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d75a:	lea    0x1073df(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d761:	lea    0x1070f0(%rip),%r8        # 144858 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x428>
   3d768:	lea    0x750(%rsp),%rdx
   3d770:	mov    $0x2b,%esi
   3d775:	call   *0x10ef35(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d77b:	mov    $0x1,%r13b
   3d77e:	xor    %r15d,%r15d
   3d781:	mov    0xc0(%rsp),%rax
   3d789:	mov    %rax,0xb0(%rsp)
   3d791:	cmpq   $0x0,0x60(%rsp)
   3d797:	jne    3d60b <meshopt_peers::run+0x34ab>
   3d79d:	jmp    3d61e <meshopt_peers::run+0x34be>
   3d7a2:	mov    $0x4,%edi
   3d7a7:	mov    %r15,%rsi
   3d7aa:	call   *0x10ee70(%rip)        # 14c620 <_DYNAMIC+0x240>
   3d7b0:	mov    $0x4,%edi
   3d7b5:	mov    %r15,%rsi
   3d7b8:	call   *0x10ee62(%rip)        # 14c620 <_DYNAMIC+0x240>
   3d7be:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d7c3:	movb   $0x8,0x750(%rsp)
   3d7cb:	lea    -0x2c347(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d7d2:	lea    0x107367(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d7d9:	lea    0x107000(%rip),%r8        # 1447e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3b0>
   3d7e0:	lea    0x750(%rsp),%rdx
   3d7e8:	mov    $0x2b,%esi
   3d7ed:	call   *0x10eebd(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d7f3:	lea    0x106cfe(%rip),%rcx        # 1444f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xc8>
   3d7fa:	mov    %r15,%rsi
   3d7fd:	mov    0x70(%rsp),%rdx
   3d802:	call   *0x10ef48(%rip)        # 14c750 <_DYNAMIC+0x370>
   3d808:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d80d:	mov    $0x4,%edi
   3d812:	mov    %rbp,%rsi
   3d815:	call   *0x10ee05(%rip)        # 14c620 <_DYNAMIC+0x240>
   3d81b:	movb   $0x8,0x750(%rsp)
   3d823:	lea    -0x2c39f(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d82a:	lea    0x10730f(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d831:	lea    0x106ea0(%rip),%r8        # 1446d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2a8>
   3d838:	lea    0x750(%rsp),%rdx
   3d840:	mov    $0x2b,%esi
   3d845:	call   *0x10ee65(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d84b:	mov    $0x4,%edi
   3d850:	mov    %rbp,%rsi
   3d853:	call   *0x10edc7(%rip)        # 14c620 <_DYNAMIC+0x240>
   3d859:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d85e:	movb   $0x2,0x750(%rsp)
   3d866:	lea    -0x2c3e2(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d86d:	lea    0x1072cc(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d874:	lea    0x106ebd(%rip),%r8        # 144738 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x308>
   3d87b:	lea    0x750(%rsp),%rdx
   3d883:	mov    $0x2b,%esi
   3d888:	call   *0x10ee22(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d88e:	xor    %r13d,%r13d
   3d891:	cmpq   $0x0,0x58(%rsp)
   3d897:	je     3d8b1 <meshopt_peers::run+0x3751>
   3d899:	mov    0xb8(%rsp),%rax
   3d8a1:	mov    %rax,0xb0(%rsp)
   3d8a9:	xor    %r15d,%r15d
   3d8ac:	jmp    3d60b <meshopt_peers::run+0x34ab>
   3d8b1:	movq   $0x0,0x58(%rsp)
   3d8ba:	xor    %r15d,%r15d
   3d8bd:	jmp    3d61e <meshopt_peers::run+0x34be>
   3d8c2:	mov    $0x8,%bpl
   3d8c5:	cmpq   $0x0,0x28(%rsp)
   3d8cb:	je     3d4db <meshopt_peers::run+0x337b>
   3d8d1:	jmp    3d4d0 <meshopt_peers::run+0x3370>
   3d8d6:	lea    0x106c03(%rip),%rcx        # 1444e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xb0>
   3d8dd:	mov    0xd0(%rsp),%rdi
   3d8e5:	mov    0x68(%rsp),%rsi
   3d8ea:	mov    %rbx,%rdx
   3d8ed:	call   *0x10ee5d(%rip)        # 14c750 <_DYNAMIC+0x370>
   3d8f3:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d8f8:	mov    %eax,%ebp
   3d8fa:	cmpq   $0x0,0x90(%rsp)
   3d903:	je     3d910 <meshopt_peers::run+0x37b0>
   3d905:	mov    0x20(%rsp),%rdi
   3d90a:	call   *0x10ecd8(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3d910:	mov    0x750(%rsp),%rax
   3d918:	mov    0x760(%rsp),%rcx
   3d920:	sub    0x758(%rsp),%rcx
   3d928:	mov    %rcx,0x358(%rax)
   3d92f:	mov    %bpl,0x750(%rsp)
   3d937:	lea    -0x2c4b3(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d93e:	lea    0x1071fb(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3d945:	lea    0x106f6c(%rip),%r8        # 1448b8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x488>
   3d94c:	lea    0x750(%rsp),%rdx
   3d954:	mov    $0x2b,%esi
   3d959:	call   *0x10ed51(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3d95f:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d964:	lea    0x106b5d(%rip),%rdx        # 1444c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x98>
   3d96b:	mov    %r12,%rdi
   3d96e:	mov    0x48(%rsp),%rsi
   3d973:	call   *0x10ecb7(%rip)        # 14c630 <_DYNAMIC+0x250>
   3d979:	jmp    3e0c4 <meshopt_peers::run+0x3f64>
   3d97e:	mov    %rax,%rbx
   3d981:	cmpq   $0x0,0xf0(%rsp)
   3d98a:	je     3db0e <meshopt_peers::run+0x39ae>
   3d990:	mov    0xf8(%rsp),%rdi
   3d998:	jmp    3db08 <meshopt_peers::run+0x39a8>
   3d99d:	mov    %rax,%rbx
   3d9a0:	cmpq   $0x0,0xf0(%rsp)
   3d9a9:	je     3db24 <meshopt_peers::run+0x39c4>
   3d9af:	mov    0xf8(%rsp),%r15
   3d9b7:	jmp    3d9e6 <meshopt_peers::run+0x3886>
   3d9b9:	jmp    3d9bb <meshopt_peers::run+0x385b>
   3d9bb:	mov    %rax,%rbx
   3d9be:	cmpq   $0x0,0x28(%rsp)
   3d9c4:	je     3da55 <meshopt_peers::run+0x38f5>
   3d9ca:	jmp    3daa6 <meshopt_peers::run+0x3946>
   3d9cf:	mov    %rax,%rbx
   3d9d2:	jmp    3da73 <meshopt_peers::run+0x3913>
   3d9d7:	mov    %rax,%rbx
   3d9da:	cmpq   $0x0,0x18(%rsp)
   3d9e0:	je     3db24 <meshopt_peers::run+0x39c4>
   3d9e6:	mov    %r15,%rdi
   3d9e9:	jmp    3db1e <meshopt_peers::run+0x39be>
   3d9ee:	mov    %rax,%rbx
   3d9f1:	jmp    3db0e <meshopt_peers::run+0x39ae>
   3d9f6:	mov    %rax,%rbx
   3d9f9:	mov    %r12,0x80(%rsp)
   3da01:	mov    0x90(%rsp),%rax
   3da09:	mov    %rax,0x78(%rsp)
   3da0e:	cmpq   $0x0,0x78(%rsp)
   3da14:	je     3db38 <meshopt_peers::run+0x39d8>
   3da1a:	jmp    3dbd9 <meshopt_peers::run+0x3a79>
   3da1f:	mov    %rax,%rbx
   3da22:	cmpq   $0x0,0x78(%rsp)
   3da28:	je     3db38 <meshopt_peers::run+0x39d8>
   3da2e:	jmp    3dbd9 <meshopt_peers::run+0x3a79>
   3da33:	mov    %r15,0x38(%rsp)
   3da38:	mov    %rax,%rbx
   3da3b:	jmp    3db24 <meshopt_peers::run+0x39c4>
   3da40:	mov    %rax,%rbx
   3da43:	test   %rbp,%rbp
   3da46:	jne    3da82 <meshopt_peers::run+0x3922>
   3da48:	test   %r15,%r15
   3da4b:	jne    3da95 <meshopt_peers::run+0x3935>
   3da4d:	cmpq   $0x0,0x28(%rsp)
   3da53:	jne    3daa6 <meshopt_peers::run+0x3946>
   3da55:	cmpq   $0x0,0x58(%rsp)
   3da5b:	jne    3dab9 <meshopt_peers::run+0x3959>
   3da5d:	cmpq   $0x0,0x60(%rsp)
   3da63:	je     3da73 <meshopt_peers::run+0x3913>
   3da65:	mov    0xc0(%rsp),%rdi
   3da6d:	call   *0x10eb75(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3da73:	lea    0xf0(%rsp),%rdi
   3da7b:	call   367b0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3da80:	jmp    3daf5 <meshopt_peers::run+0x3995>
   3da82:	mov    0xb0(%rsp),%rdi
   3da8a:	call   *0x10eb58(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3da90:	test   %r15,%r15
   3da93:	je     3da4d <meshopt_peers::run+0x38ed>
   3da95:	mov    %r13,%rdi
   3da98:	call   *0x10eb4a(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3da9e:	cmpq   $0x0,0x28(%rsp)
   3daa4:	je     3da55 <meshopt_peers::run+0x38f5>
   3daa6:	mov    0x50(%rsp),%rdi
   3daab:	call   *0x10eb37(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dab1:	cmpq   $0x0,0x58(%rsp)
   3dab7:	je     3da5d <meshopt_peers::run+0x38fd>
   3dab9:	mov    0xb8(%rsp),%rdi
   3dac1:	call   *0x10eb21(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dac7:	cmpq   $0x0,0x60(%rsp)
   3dacd:	jne    3da65 <meshopt_peers::run+0x3905>
   3dacf:	jmp    3da73 <meshopt_peers::run+0x3913>
   3dad1:	jmp    3dad3 <meshopt_peers::run+0x3973>
   3dad3:	mov    %rax,%rbx
   3dad6:	test   %r14,%r14
   3dad9:	je     3db50 <meshopt_peers::run+0x39f0>
   3dadb:	jmp    3dc07 <meshopt_peers::run+0x3aa7>
   3dae0:	jmp    3dbbd <meshopt_peers::run+0x3a5d>
   3dae5:	jmp    3dbbd <meshopt_peers::run+0x3a5d>
   3daea:	mov    %rax,%rbx
   3daed:	jmp    3dc98 <meshopt_peers::run+0x3b38>
   3daf2:	mov    %rax,%rbx
   3daf5:	cmpq   $0x0,0x90(%rsp)
   3dafe:	je     3db0e <meshopt_peers::run+0x39ae>
   3db00:	mov    0xd8(%rsp),%rdi
   3db08:	call   *0x10eada(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3db0e:	cmpq   $0x0,0x18(%rsp)
   3db14:	je     3db24 <meshopt_peers::run+0x39c4>
   3db16:	mov    0xc8(%rsp),%rdi
   3db1e:	call   *0x10eac4(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3db24:	cmpq   $0x0,0x38(%rsp)
   3db2a:	jne    3db64 <meshopt_peers::run+0x3a04>
   3db2c:	cmpq   $0x0,0x78(%rsp)
   3db32:	jne    3dbd9 <meshopt_peers::run+0x3a79>
   3db38:	cmpq   $0x0,0x80(%rsp)
   3db41:	jne    3dbf3 <meshopt_peers::run+0x3a93>
   3db47:	test   %r14,%r14
   3db4a:	jne    3dc07 <meshopt_peers::run+0x3aa7>
   3db50:	cmpq   $0x0,0x88(%rsp)
   3db59:	jne    3dc20 <meshopt_peers::run+0x3ac0>
   3db5f:	jmp    3dc2e <meshopt_peers::run+0x3ace>
   3db64:	mov    0x150(%rsp),%rdi
   3db6c:	call   *0x10ea76(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3db72:	cmpq   $0x0,0x78(%rsp)
   3db78:	je     3db38 <meshopt_peers::run+0x39d8>
   3db7a:	jmp    3dbd9 <meshopt_peers::run+0x3a79>
   3db7c:	mov    %rax,%rbx
   3db7f:	cmpq   $0x0,0x88(%rsp)
   3db88:	jne    3dc20 <meshopt_peers::run+0x3ac0>
   3db8e:	jmp    3dc2e <meshopt_peers::run+0x3ace>
   3db93:	jmp    3db9d <meshopt_peers::run+0x3a3d>
   3db95:	mov    %rax,%rbx
   3db98:	jmp    3dc2e <meshopt_peers::run+0x3ace>
   3db9d:	mov    %rax,%rbx
   3dba0:	lea    0x310(%rsp),%rdi
   3dba8:	call   36d60 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   3dbad:	mov    %rbx,%rdi
   3dbb0:	call   1429c0 <_Unwind_Resume@plt>
   3dbb5:	mov    %rax,%rbx
   3dbb8:	jmp    3dc4d <meshopt_peers::run+0x3aed>
   3dbbd:	mov    %rax,%rbx
   3dbc0:	lea    0x310(%rsp),%rdi
   3dbc8:	call   36da0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3dbcd:	cmpq   $0x0,0x78(%rsp)
   3dbd3:	je     3db38 <meshopt_peers::run+0x39d8>
   3dbd9:	mov    0x20(%rsp),%rdi
   3dbde:	call   *0x10ea04(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dbe4:	cmpq   $0x0,0x80(%rsp)
   3dbed:	je     3db47 <meshopt_peers::run+0x39e7>
   3dbf3:	mov    0x30(%rsp),%rdi
   3dbf8:	call   *0x10e9ea(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dbfe:	test   %r14,%r14
   3dc01:	je     3db50 <meshopt_peers::run+0x39f0>
   3dc07:	mov    0xa0(%rsp),%rdi
   3dc0f:	call   *0x10e9d3(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dc15:	cmpq   $0x0,0x88(%rsp)
   3dc1e:	je     3dc2e <meshopt_peers::run+0x3ace>
   3dc20:	mov    0xe8(%rsp),%rdi
   3dc28:	call   *0x10e9ba(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dc2e:	mov    0x750(%rsp),%rax
   3dc36:	mov    0x760(%rsp),%rcx
   3dc3e:	sub    0x758(%rsp),%rcx
   3dc46:	mov    %rcx,0x358(%rax)
   3dc4d:	cmpq   $0x0,0x1b0(%rsp)
   3dc56:	je     3dc66 <meshopt_peers::run+0x3b06>
   3dc58:	mov    0x1b8(%rsp),%rdi
   3dc60:	call   *0x10e982(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dc66:	test   %r14,%r14
   3dc69:	je     3e0d4 <meshopt_peers::run+0x3f74>
   3dc6f:	mov    0x8(%rsp),%rdi
   3dc74:	call   *0x10e96e(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dc7a:	mov    %rbx,%rdi
   3dc7d:	call   1429c0 <_Unwind_Resume@plt>
   3dc82:	mov    %r12,0x8(%rsp)
   3dc87:	mov    %rax,%rbx
   3dc8a:	test   %r14,%r14
   3dc8d:	je     3dc98 <meshopt_peers::run+0x3b38>
   3dc8f:	mov    %r15,%rdi
   3dc92:	call   *0x10e950(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dc98:	cmpq   $0x0,(%rsp)
   3dc9d:	je     3e0d4 <meshopt_peers::run+0x3f74>
   3dca3:	mov    0x8(%rsp),%rdi
   3dca8:	call   *0x10e93a(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3dcae:	mov    %rbx,%rdi
   3dcb1:	call   1429c0 <_Unwind_Resume@plt>
   3dcb6:	mov    %edi,%r8d
   3dcb9:	mov    0x4(%rbp,%rsi,4),%edi
   3dcbd:	cmp    %edi,%r8d
   3dcc0:	cmova  %r8d,%edi
   3dcc4:	inc    %rsi
   3dcc7:	cmp    %rsi,%rdx
   3dcca:	jne    3dcb6 <meshopt_peers::run+0x3b56>
   3dccc:	mov    %edi,%edx
   3dcce:	cmp    %rdx,%r14
   3dcd1:	jbe    3dd9a <meshopt_peers::run+0x3c3a>
   3dcd7:	mov    %rcx,0x260(%rsp)
   3dcdf:	mov    %r14,%rax
   3dce2:	mov    %rax,%rdx
   3dce5:	shr    $0x2,%rdx
   3dce9:	add    %rax,%rdx
   3dcec:	jb     3dd33 <meshopt_peers::run+0x3bd3>
   3dcee:	cmp    $0x1,%rdx
   3dcf2:	mov    %rdx,%rcx
   3dcf5:	adc    $0xffffffffffffffff,%rcx
   3dcf9:	bsr    %rcx,%rcx
   3dcfd:	not    %ecx
   3dcff:	mov    $0xffffffffffffffff,%rsi
   3dd06:	shr    %cl,%rsi
   3dd09:	xor    %r15d,%r15d
   3dd0c:	cmp    $0x2,%rdx
   3dd10:	cmovae %rsi,%r15
   3dd14:	cmp    $0xffffffffffffffff,%r15
   3dd18:	je     3dd33 <meshopt_peers::run+0x3bd3>
   3dd1a:	lea    0x0(,%rax,4),%rcx
   3dd22:	shr    $0x3e,%rax
   3dd26:	sete   %al
   3dd29:	test   %rcx,%rcx
   3dd2c:	setns  %dl
   3dd2f:	test   %dl,%al
   3dd31:	jne    3dd40 <meshopt_peers::run+0x3be0>
   3dd33:	movb   $0x8,0x318(%rsp)
   3dd3b:	jmp    3ddc8 <meshopt_peers::run+0x3c68>
   3dd40:	lea    0x1(%r15),%rbp
   3dd44:	mov    %rbp,0x6e0(%rsp)
   3dd4c:	pxor   %xmm0,%xmm0
   3dd50:	movdqu %xmm0,0x6e8(%rsp)
   3dd59:	movq   $0x0,0x6f8(%rsp)
   3dd65:	lea    0x6e0(%rsp),%rsi
   3dd6d:	mov    %r11,%r12
   3dd70:	mov    %r11,%rdi
   3dd73:	xor    %edx,%edx
   3dd75:	call   a55a0 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3dd7a:	cmp    $0xff,%al
   3dd7c:	je     3de38 <meshopt_peers::run+0x3cd8>
   3dd82:	mov    %al,0x318(%rsp)
   3dd89:	movq   $0xffffffffffffffff,0x310(%rsp)
   3dd95:	mov    %r12,%r11
   3dd98:	jmp    3ddd4 <meshopt_peers::run+0x3c74>
   3dd9a:	shl    $0x2,%r15
   3dd9e:	xor    %edx,%edx
   3dda0:	cmp    %rdx,%r15
   3dda3:	je     3dcd7 <meshopt_peers::run+0x3b77>
   3dda9:	dec    %rax
   3ddac:	cmp    %r14d,0x0(%rbp,%rdx,1)
   3ddb1:	lea    0x4(%rdx),%rdx
   3ddb5:	jb     3dda0 <meshopt_peers::run+0x3c40>
   3ddb7:	mov    %rax,0x260(%rsp)
   3ddbf:	mov    $0x1,%al
   3ddc1:	mov    %al,0x318(%rsp)
   3ddc8:	movq   $0xffffffffffffffff,0x310(%rsp)
   3ddd4:	mov    0x268(%rsp),%rax
   3dddc:	sub    0x260(%rsp),%rax
   3dde4:	mov    %rax,0x358(%r11)
   3ddeb:	cmpq   $0xffffffffffffffff,0x310(%rsp)
   3ddf4:	je     3e072 <meshopt_peers::run+0x3f12>
   3ddfa:	mov    0x320(%rsp),%rax
   3de02:	mov    %rax,0x20(%rbx)
   3de06:	movdqu 0x310(%rsp),%xmm0
   3de0f:	movdqu %xmm0,0x10(%rbx)
   3de14:	mov    0x328(%rsp),%rax
   3de1c:	mov    %rax,0x8(%rbx)
   3de20:	movl   $0x0,(%rbx)
   3de26:	add    $0x1988,%rsp
   3de2d:	pop    %rbx
   3de2e:	pop    %r12
   3de30:	pop    %r13
   3de32:	pop    %r14
   3de34:	pop    %r15
   3de36:	pop    %rbp
   3de37:	ret
   3de38:	mov    0x1c8(%rsp),%rsi
   3de40:	lea    0x310(%rsp),%rdi
   3de48:	call   *0x10ebfa(%rip)        # 14ca48 <_DYNAMIC+0x668>
   3de4e:	mov    0x310(%rsp),%rsi
   3de56:	movzbl 0x318(%rsp),%eax
   3de5e:	cmp    $0xffffffffffffffff,%rsi
   3de62:	je     3dd82 <meshopt_peers::run+0x3c22>
   3de68:	movzbl %al,%eax
   3de6b:	movzbl 0x31f(%rsp),%ecx
   3de73:	shl    $0x10,%ecx
   3de76:	movzwl 0x31d(%rsp),%edx
   3de7e:	or     %ecx,%edx
   3de80:	mov    0x319(%rsp),%ecx
   3de87:	mov    0x320(%rsp),%rdi
   3de8f:	mov    %rdi,0x8(%rsp)
   3de94:	shl    $0x28,%rdx
   3de98:	shl    $0x8,%rcx
   3de9c:	or     %rdx,%rcx
   3de9f:	or     %rax,%rcx
   3dea2:	mov    %rcx,(%rsp)
   3dea6:	cmp    0x1c8(%rsp),%rsi
   3deae:	mov    %r12,%r11
   3deb1:	jne    3df26 <meshopt_peers::run+0x3dc6>
   3deb3:	mov    %rsi,0x88(%rsp)
   3debb:	lea    0x788(%rsp),%rax
   3dec3:	cmpb   $0x0,0x230(%rsp)
   3decb:	je     3df98 <meshopt_peers::run+0x3e38>
   3ded1:	mov    0x28(%r11),%rdx
   3ded5:	cmp    %rdx,%r15
   3ded8:	jae    3e0a9 <meshopt_peers::run+0x3f49>
   3dede:	mov    0x20(%r11),%r9
   3dee2:	mov    0xf0(%rsp),%rcx
   3deea:	mov    0xf8(%rsp),%r8
   3def2:	sub    $0x8,%rsp
   3def6:	lea    0x318(%rsp),%rdi
   3defe:	mov    0x8(%rsp),%rsi
   3df03:	mov    0x10(%rsp),%rdx
   3df08:	lea    0x268(%rsp),%r10
   3df10:	push   %r10
   3df12:	push   %rax
   3df13:	push   %r14
   3df15:	push   %r13
   3df17:	push   %rbp
   3df18:	call   38de0 <meshoptimizer_rs::remap::generate_kernel::<true, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3df1d:	add    $0x30,%rsp
   3df21:	jmp    3dfe8 <meshopt_peers::run+0x3e88>
   3df26:	movabs $0x3fffffffffffffff,%rax
   3df30:	lea    0x0(,%rsi,4),%rcx
   3df38:	cmp    %rax,%rsi
   3df3b:	seta   %al
   3df3e:	test   %rcx,%rcx
   3df41:	sets   %dl
   3df44:	or     %al,%dl
   3df46:	mov    $0x8,%al
   3df48:	jne    3df77 <meshopt_peers::run+0x3e17>
   3df4a:	mov    %rsi,0x88(%rsp)
   3df52:	lea    0x6e0(%rsp),%rsi
   3df5a:	mov    %r11,%rdi
   3df5d:	xor    %edx,%edx
   3df5f:	call   a55a0 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3df64:	cmp    $0xff,%al
   3df66:	mov    %r12,%r11
   3df69:	mov    0x88(%rsp),%rsi
   3df71:	je     3deb3 <meshopt_peers::run+0x3d53>
   3df77:	mov    %al,0x318(%rsp)
   3df7e:	movq   $0xffffffffffffffff,0x310(%rsp)
   3df8a:	test   %rsi,%rsi
   3df8d:	je     3ddd4 <meshopt_peers::run+0x3c74>
   3df93:	jmp    3e01f <meshopt_peers::run+0x3ebf>
   3df98:	mov    0x28(%r11),%rdx
   3df9c:	cmp    %rdx,%r15
   3df9f:	jae    3e0b2 <meshopt_peers::run+0x3f52>
   3dfa5:	mov    0x20(%r11),%r9
   3dfa9:	mov    0xf0(%rsp),%rcx
   3dfb1:	mov    0xf8(%rsp),%r8
   3dfb9:	sub    $0x8,%rsp
   3dfbd:	lea    0x318(%rsp),%rdi
   3dfc5:	mov    0x8(%rsp),%rsi
   3dfca:	mov    0x10(%rsp),%rdx
   3dfcf:	lea    0x268(%rsp),%r10
   3dfd7:	push   %r10
   3dfd9:	push   %rax
   3dfda:	push   %r14
   3dfdc:	push   %r13
   3dfde:	push   %rbp
   3dfdf:	call   37b70 <meshoptimizer_rs::remap::generate_kernel::<false, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3dfe4:	add    $0x30,%rsp
   3dfe8:	cmpb   $0x1,0x310(%rsp)
   3dff0:	jne    3e031 <meshopt_peers::run+0x3ed1>
   3dff2:	movzbl 0x311(%rsp),%eax
   3dffa:	mov    %r12,%r11
   3dffd:	mov    %al,0x318(%rsp)
   3e004:	movq   $0xffffffffffffffff,0x310(%rsp)
   3e010:	cmpq   $0x0,0x88(%rsp)
   3e019:	je     3ddd4 <meshopt_peers::run+0x3c74>
   3e01f:	mov    (%rsp),%rdi
   3e023:	call   *0x10e5bf(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3e029:	mov    %r12,%r11
   3e02c:	jmp    3ddd4 <meshopt_peers::run+0x3c74>
   3e031:	mov    0x318(%rsp),%rax
   3e039:	mov    %r12,%r11
   3e03c:	mov    0x88(%rsp),%rcx
   3e044:	mov    %rcx,0x310(%rsp)
   3e04c:	mov    (%rsp),%rcx
   3e050:	mov    %rcx,0x318(%rsp)
   3e058:	mov    0x8(%rsp),%rcx
   3e05d:	mov    %rcx,0x320(%rsp)
   3e065:	mov    %rax,0x328(%rsp)
   3e06d:	jmp    3ddd4 <meshopt_peers::run+0x3c74>
   3e072:	movzbl 0x318(%rsp),%eax
   3e07a:	mov    %al,0x750(%rsp)
   3e081:	lea    -0x2cbfd(%rip),%rdi        # 1148b <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3e088:	lea    0x106ab1(%rip),%rcx        # 144b40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x710>
   3e08f:	lea    0x1066ba(%rip),%r8        # 144750 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x320>
   3e096:	lea    0x750(%rsp),%rdx
   3e09e:	mov    $0x2b,%esi
   3e0a3:	call   *0x10e607(%rip)        # 14c6b0 <_DYNAMIC+0x2d0>
   3e0a9:	lea    0x106520(%rip),%rcx        # 1445d0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x1a0>
   3e0b0:	jmp    3e0b9 <meshopt_peers::run+0x3f59>
   3e0b2:	lea    0x1064ff(%rip),%rcx        # 1445b8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x188>
   3e0b9:	xor    %edi,%edi
   3e0bb:	mov    %rbp,%rsi
   3e0be:	call   *0x10e68c(%rip)        # 14c750 <_DYNAMIC+0x370>
   3e0c4:	ud2
   3e0c6:	mov    %rax,%rbx
   3e0c9:	cmpq   $0x0,0x88(%rsp)
   3e0d2:	jne    3e0dc <meshopt_peers::run+0x3f7c>
   3e0d4:	mov    %rbx,%rdi
   3e0d7:	call   1429c0 <_Unwind_Resume@plt>
   3e0dc:	mov    (%rsp),%rdi
   3e0e0:	call   *0x10e502(%rip)        # 14c5e8 <free@GLIBC_2.2.5>
   3e0e6:	mov    %rbx,%rdi
   3e0e9:	call   1429c0 <_Unwind_Resume@plt>
