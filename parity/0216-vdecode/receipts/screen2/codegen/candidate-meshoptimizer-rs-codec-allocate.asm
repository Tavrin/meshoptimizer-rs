
<artifacts>/screen2/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000b33a0 <meshoptimizer_rs::codec::allocate>:
   b33a0:	push   %rbp
   b33a1:	push   %r15
   b33a3:	push   %r14
   b33a5:	push   %r13
   b33a7:	push   %r12
   b33a9:	push   %rbx
   b33aa:	sub    $0x468,%rsp
   b33b1:	mov    %r8,%r14
   b33b4:	mov    %edx,%r8d
   b33b7:	mov    0x4a8(%rsp),%r12
   b33bf:	movb   $0x0,0x365(%r12)
   b33c8:	movl   $0x0,0x361(%r12)
   b33d4:	xorps  %xmm0,%xmm0
   b33d7:	movups %xmm0,0x350(%r12)
   b33e0:	mov    0x348(%r12),%r15
   b33e8:	mov    %r15,0x48(%rsp)
   b33ed:	mov    %r15,0x50(%rsp)
   b33f2:	movzbl %sil,%r13d
   b33f6:	test   %r13b,%r13b
   b33f9:	je     b3430 <meshoptimizer_rs::codec::allocate+0x90>
   b33fb:	cmp    $0x1,%r13d
   b33ff:	jne    b344e <meshoptimizer_rs::codec::allocate+0xae>
   b3401:	cmp    $0x2,%r14
   b3405:	je     b340f <meshoptimizer_rs::codec::allocate+0x6f>
   b3407:	mov    $0x2,%al
   b3409:	cmp    $0x4,%r14
   b340d:	jne    b3474 <meshoptimizer_rs::codec::allocate+0xd4>
   b340f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b3419:	imul   %rcx,%rax
   b341d:	movabs $0x5555555555555556,%rdx
   b3427:	cmp    %rdx,%rax
   b342a:	jb     b345c <meshoptimizer_rs::codec::allocate+0xbc>
   b342c:	xor    %eax,%eax
   b342e:	jmp    b3474 <meshoptimizer_rs::codec::allocate+0xd4>
   b3430:	lea    -0x1(%r14),%rax
   b3434:	cmp    $0x100,%rax
   b343a:	setb   %r10b
   b343e:	test   $0x3,%r14b
   b3442:	sete   %dl
   b3445:	mov    $0x2,%al
   b3447:	test   %dl,%r10b
   b344a:	jne    b345c <meshoptimizer_rs::codec::allocate+0xbc>
   b344c:	jmp    b3474 <meshoptimizer_rs::codec::allocate+0xd4>
   b344e:	cmp    $0x4,%r14
   b3452:	je     b345c <meshoptimizer_rs::codec::allocate+0xbc>
   b3454:	mov    $0x2,%al
   b3456:	cmp    $0x2,%r14
   b345a:	jne    b3474 <meshoptimizer_rs::codec::allocate+0xd4>
   b345c:	mov    %rcx,%rax
   b345f:	mul    %r14
   b3462:	mov    %rax,%rbp
   b3465:	seto   %al
   b3468:	test   %rbp,%rbp
   b346b:	sets   %dl
   b346e:	or     %al,%dl
   b3470:	mov    $0x8,%al
   b3472:	je     b3490 <meshoptimizer_rs::codec::allocate+0xf0>
   b3474:	mov    %al,0x8(%rdi)
   b3477:	movq   $0xffffffffffffffff,(%rdi)
   b347e:	add    $0x468,%rsp
   b3485:	pop    %rbx
   b3486:	pop    %r12
   b3488:	pop    %r13
   b348a:	pop    %r14
   b348c:	pop    %r15
   b348e:	pop    %rbp
   b348f:	ret
   b3490:	mov    %esi,0x3c(%rsp)
   b3494:	mov    %rcx,0x10(%rsp)
   b3499:	mov    %r9,0x30(%rsp)
   b349e:	mov    %r8d,0x38(%rsp)
   b34a3:	mov    %rdi,0x40(%rsp)
   b34a8:	mov    %r12,%rdi
   b34ab:	mov    %rbp,%rsi
   b34ae:	call   97f10 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b34b3:	mov    %r15,%rbx
   b34b6:	cmp    $0xff,%al
   b34b8:	je     b34d6 <meshoptimizer_rs::codec::allocate+0x136>
   b34ba:	sub    %rbx,%r15
   b34bd:	mov    %r15,0x358(%r12)
   b34c5:	mov    0x40(%rsp),%rcx
   b34ca:	mov    %al,0x8(%rcx)
   b34cd:	movq   $0xffffffffffffffff,(%rcx)
   b34d4:	jmp    b347e <meshoptimizer_rs::codec::allocate+0xde>
   b34d6:	mov    0x4a0(%rsp),%rdx
   b34de:	mov    %r15,%rax
   b34e1:	mov    %rdx,%rcx
   b34e4:	sub    %rdx,%rax
   b34e7:	jb     b3856 <meshoptimizer_rs::codec::allocate+0x4b6>
   b34ed:	mov    %rax,0x48(%rsp)
   b34f2:	mov    %rax,%rbx
   b34f5:	mov    %rbp,%r10
   b34f8:	sub    %rbp,%rbx
   b34fb:	jb     b385e <meshoptimizer_rs::codec::allocate+0x4be>
   b3501:	mov    %rbx,0x48(%rsp)
   b3506:	cmpb   $0x0,0x38(%rsp)
   b350b:	mov    0x3c(%rsp),%ebp
   b350f:	je     b3525 <meshoptimizer_rs::codec::allocate+0x185>
   b3511:	mov    %r10,%rsi
   b3514:	shr    $0x2,%rsi
   b3518:	mov    %rbx,%rax
   b351b:	sub    %rsi,%rax
   b351e:	jae    b3528 <meshoptimizer_rs::codec::allocate+0x188>
   b3520:	jmp    b3864 <meshoptimizer_rs::codec::allocate+0x4c4>
   b3525:	mov    %rbx,%rax
   b3528:	sub    %rax,%r15
   b352b:	mov    %r15,0x358(%r12)
   b3533:	cmp    $0x2,%bpl
   b3537:	mov    0x40(%rsp),%rbx
   b353c:	je     b3571 <meshoptimizer_rs::codec::allocate+0x1d1>
   b353e:	cmp    $0x1,%r13d
   b3542:	mov    %rcx,%r15
   b3545:	jne    b361d <meshoptimizer_rs::codec::allocate+0x27d>
   b354b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b3555:	mov    0x10(%rsp),%rax
   b355a:	mul    %rcx
   b355d:	shr    $1,%rdx
   b3560:	add    $0x11,%rdx
   b3564:	mov    $0x7,%sil
   b3567:	cmp    %rdx,%r15
   b356a:	jae    b3589 <meshoptimizer_rs::codec::allocate+0x1e9>
   b356c:	jmp    b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3571:	mov    0x10(%rsp),%rax
   b3576:	lea    0x5(%rax),%rdx
   b357a:	mov    %rcx,%r15
   b357d:	mov    $0x7,%sil
   b3580:	cmp    %rdx,%r15
   b3583:	jb     b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3589:	movq   $0x0,0x18(%rsp)
   b3592:	movq   $0x1,0x20(%rsp)
   b359b:	movq   $0x0,0x28(%rsp)
   b35a4:	test   %r10,%r10
   b35a7:	jne    b36ed <meshoptimizer_rs::codec::allocate+0x34d>
   b35ad:	cmp    $0x2,%bpl
   b35b1:	je     b3739 <meshoptimizer_rs::codec::allocate+0x399>
   b35b7:	test   %r13d,%r13d
   b35ba:	jne    b3660 <meshoptimizer_rs::codec::allocate+0x2c0>
   b35c0:	cmp    $0x4,%r14
   b35c4:	sete   %al
   b35c7:	cmp    $0x100000,%r10
   b35ce:	setae  %cl
   b35d1:	test   %cl,%al
   b35d3:	je     b3695 <meshoptimizer_rs::codec::allocate+0x2f5>
   b35d9:	lea    0x60(%rsp),%rdi
   b35de:	mov    $0x400,%edx
   b35e3:	xor    %esi,%esi
   b35e5:	call   *0x75b7d(%rip)        # 129168 <memset@GLIBC_2.2.5>
   b35eb:	lea    0x18(%rsp),%rax
   b35f0:	mov    %rax,0x58(%rsp)
   b35f5:	movq   $0x0,0x460(%rsp)
   b3601:	lea    0x58(%rsp),%rdi
   b3606:	mov    0x10(%rsp),%rsi
   b360b:	mov    0x30(%rsp),%rdx
   b3610:	mov    %r15,%rcx
   b3613:	call   73d70 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b3618:	jmp    b3753 <meshoptimizer_rs::codec::allocate+0x3b3>
   b361d:	mov    $0x7,%sil
   b3620:	test   %r15,%r15
   b3623:	je     b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3629:	mov    0x30(%rsp),%rax
   b362e:	movzbl (%rax),%ecx
   b3631:	mov    %ecx,%eax
   b3633:	and    $0xf0,%al
   b3635:	cmp    $0xa0,%al
   b3637:	jne    b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b363d:	and    $0xf,%cl
   b3640:	mov    $0x5,%sil
   b3643:	cmp    $0x1,%cl
   b3646:	ja     b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b364c:	mov    0x10(%rsp),%rdi
   b3651:	cmp    $0x21,%rdi
   b3655:	jae    b36b4 <meshoptimizer_rs::codec::allocate+0x314>
   b3657:	xor    %eax,%eax
   b3659:	xor    %esi,%esi
   b365b:	jmp    b37c3 <meshoptimizer_rs::codec::allocate+0x423>
   b3660:	lea    0x18(%rsp),%rdi
   b3665:	mov    %r10,%rsi
   b3668:	call   91690 <<alloc::vec::Vec<u8>>::resize>
   b366d:	mov    0x20(%rsp),%rsi
   b3672:	mov    0x28(%rsp),%rdx
   b3677:	mov    %r15,(%rsp)
   b367b:	mov    %r13d,%edi
   b367e:	mov    0x10(%rsp),%rcx
   b3683:	mov    %r14,%r8
   b3686:	mov    0x30(%rsp),%r9
   b368b:	call   ae0b0 <meshoptimizer_rs::codec::raw>
   b3690:	jmp    b3753 <meshoptimizer_rs::codec::allocate+0x3b3>
   b3695:	lea    0x18(%rsp),%rdi
   b369a:	mov    0x10(%rsp),%rsi
   b369f:	mov    %r14,%rdx
   b36a2:	mov    0x30(%rsp),%rcx
   b36a7:	mov    %r15,%r8
   b36aa:	call   730a0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   b36af:	jmp    b3753 <meshoptimizer_rs::codec::allocate+0x3b3>
   b36b4:	mov    $0x2000,%ax
   b36b8:	xor    %edx,%edx
   b36ba:	div    %r14w
   b36be:	and    $0x3ff0,%eax
   b36c3:	cmp    $0x100,%eax
   b36c8:	mov    $0x100,%esi
   b36cd:	cmovb  %eax,%esi
   b36d0:	mov    %rdi,%rax
   b36d3:	shr    $0x20,%rax
   b36d7:	je     b37ae <meshoptimizer_rs::codec::allocate+0x40e>
   b36dd:	mov    %rdi,%rax
   b36e0:	xor    %edx,%edx
   b36e2:	div    %rsi
   b36e5:	mov    %rdx,%rdi
   b36e8:	jmp    b37b6 <meshoptimizer_rs::codec::allocate+0x416>
   b36ed:	lea    0x58(%rsp),%rdi
   b36f2:	mov    $0x1,%edx
   b36f7:	mov    $0x1,%r8d
   b36fd:	mov    $0x1,%r9d
   b3703:	xor    %esi,%esi
   b3705:	mov    %r10,%rcx
   b3708:	mov    %r10,%r12
   b370b:	call   92180 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b3710:	cmpb   $0x0,0x58(%rsp)
   b3715:	je     b371d <meshoptimizer_rs::codec::allocate+0x37d>
   b3717:	movb   $0xa,0x8(%rbx)
   b371b:	jmp    b3775 <meshoptimizer_rs::codec::allocate+0x3d5>
   b371d:	mov    0x60(%rsp),%rax
   b3722:	mov    %rax,0x20(%rsp)
   b3727:	mov    %r12,0x18(%rsp)
   b372c:	mov    %r12,%r10
   b372f:	cmp    $0x2,%bpl
   b3733:	jne    b35b7 <meshoptimizer_rs::codec::allocate+0x217>
   b3739:	lea    0x18(%rsp),%rdi
   b373e:	mov    0x10(%rsp),%rsi
   b3743:	mov    %r14,%rdx
   b3746:	mov    0x30(%rsp),%rcx
   b374b:	mov    %r15,%r8
   b374e:	call   d24a0 <meshoptimizer_rs::codec::index::sequence_append>
   b3753:	cmp    $0xff,%al
   b3755:	jne    b3772 <meshoptimizer_rs::codec::allocate+0x3d2>
   b3757:	mov    0x20(%rsp),%rsi
   b375c:	mov    0x28(%rsp),%rdx
   b3761:	movzbl 0x38(%rsp),%edi
   b3766:	mov    %r14,%rcx
   b3769:	call   afd50 <meshoptimizer_rs::codec::apply>
   b376e:	cmp    $0xff,%al
   b3770:	je     b3798 <meshoptimizer_rs::codec::allocate+0x3f8>
   b3772:	mov    %al,0x8(%rbx)
   b3775:	movq   $0xffffffffffffffff,(%rbx)
   b377c:	cmpq   $0x0,0x18(%rsp)
   b3782:	je     b347e <meshoptimizer_rs::codec::allocate+0xde>
   b3788:	mov    0x20(%rsp),%rdi
   b378d:	call   *0x759b5(%rip)        # 129148 <free@GLIBC_2.2.5>
   b3793:	jmp    b347e <meshoptimizer_rs::codec::allocate+0xde>
   b3798:	mov    0x28(%rsp),%rax
   b379d:	mov    %rax,0x10(%rbx)
   b37a1:	movups 0x18(%rsp),%xmm0
   b37a6:	movups %xmm0,(%rbx)
   b37a9:	jmp    b347e <meshoptimizer_rs::codec::allocate+0xde>
   b37ae:	mov    %edi,%eax
   b37b0:	xor    %edx,%edx
   b37b2:	div    %esi
   b37b4:	mov    %edx,%edi
   b37b6:	shr    $0x4,%esi
   b37b9:	add    $0x3,%esi
   b37bc:	shr    $0x2,%esi
   b37bf:	imul   %rax,%rsi
   b37c3:	test   %cl,%cl
   b37c5:	je     b37f1 <meshoptimizer_rs::codec::allocate+0x451>
   b37c7:	cmp    $0x1,%rdi
   b37cb:	sbb    $0xffffffffffffffff,%rax
   b37cf:	mov    %r14,%rcx
   b37d2:	shr    $0x2,%rcx
   b37d6:	mul    %rcx
   b37d9:	jo     b3874 <meshoptimizer_rs::codec::allocate+0x4d4>
   b37df:	add    %r14,%rcx
   b37e2:	cmp    $0x19,%rcx
   b37e6:	mov    $0x18,%edx
   b37eb:	cmovae %rcx,%rdx
   b37ef:	jmp    b3829 <meshoptimizer_rs::codec::allocate+0x489>
   b37f1:	test   %rdi,%rdi
   b37f4:	je     b3812 <meshoptimizer_rs::codec::allocate+0x472>
   b37f6:	mov    %rdi,%rax
   b37f9:	shr    $0x4,%rax
   b37fd:	and    $0xf,%edi
   b3800:	cmp    $0x1,%rdi
   b3804:	sbb    $0xffffffffffffffff,%rax
   b3808:	add    $0x3,%rax
   b380c:	shr    $0x2,%rax
   b3810:	jmp    b3814 <meshoptimizer_rs::codec::allocate+0x474>
   b3812:	xor    %eax,%eax
   b3814:	add    %rsi,%rax
   b3817:	mul    %r14
   b381a:	jo     b3874 <meshoptimizer_rs::codec::allocate+0x4d4>
   b381c:	cmp    $0x21,%r14
   b3820:	mov    $0x20,%edx
   b3825:	cmovae %r14,%rdx
   b3829:	mov    $0x8,%sil
   b382c:	add    %rax,%rdx
   b382f:	jb     b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3831:	cmp    $0xffffffffffffffff,%rdx
   b3835:	je     b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3837:	inc    %rdx
   b383a:	mov    $0x7,%sil
   b383d:	cmp    %rdx,%r15
   b3840:	jae    b3589 <meshoptimizer_rs::codec::allocate+0x1e9>
   b3846:	mov    %sil,0x8(%rbx)
   b384a:	movq   $0xffffffffffffffff,(%rbx)
   b3851:	jmp    b347e <meshoptimizer_rs::codec::allocate+0xde>
   b3856:	mov    %rcx,%rsi
   b3859:	mov    %r15,%rbx
   b385c:	jmp    b3864 <meshoptimizer_rs::codec::allocate+0x4c4>
   b385e:	mov    %r10,%rsi
   b3861:	mov    %rax,%rbx
   b3864:	lea    0x48(%rsp),%rdi
   b3869:	call   *0x75d99(%rip)        # 129608 <_DYNAMIC+0x6c8>
   b386f:	jmp    b34ba <meshoptimizer_rs::codec::allocate+0x11a>
   b3874:	mov    $0x8,%sil
   b3877:	jmp    b3846 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3879:	mov    %rax,%rbx
   b387c:	cmpq   $0x0,0x18(%rsp)
   b3882:	je     b388f <meshoptimizer_rs::codec::allocate+0x4ef>
   b3884:	mov    0x20(%rsp),%rdi
   b3889:	call   *0x758b9(%rip)        # 129148 <free@GLIBC_2.2.5>
   b388f:	mov    %rbx,%rdi
   b3892:	call   120190 <_Unwind_Resume@plt>
