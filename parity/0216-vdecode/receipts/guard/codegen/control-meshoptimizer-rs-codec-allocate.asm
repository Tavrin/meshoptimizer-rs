
<artifacts>/guard/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000be4b0 <meshoptimizer_rs::codec::allocate>:
   be4b0:	push   %rbp
   be4b1:	push   %r15
   be4b3:	push   %r14
   be4b5:	push   %r13
   be4b7:	push   %r12
   be4b9:	push   %rbx
   be4ba:	sub    $0x468,%rsp
   be4c1:	mov    %r8,%r14
   be4c4:	mov    %edx,%r8d
   be4c7:	mov    0x4a8(%rsp),%r12
   be4cf:	movb   $0x0,0x365(%r12)
   be4d8:	movl   $0x0,0x361(%r12)
   be4e4:	xorps  %xmm0,%xmm0
   be4e7:	movups %xmm0,0x350(%r12)
   be4f0:	mov    0x348(%r12),%r15
   be4f8:	mov    %r15,0x48(%rsp)
   be4fd:	mov    %r15,0x50(%rsp)
   be502:	movzbl %sil,%r13d
   be506:	test   %r13b,%r13b
   be509:	je     be540 <meshoptimizer_rs::codec::allocate+0x90>
   be50b:	cmp    $0x1,%r13d
   be50f:	jne    be55e <meshoptimizer_rs::codec::allocate+0xae>
   be511:	cmp    $0x2,%r14
   be515:	je     be51f <meshoptimizer_rs::codec::allocate+0x6f>
   be517:	mov    $0x2,%al
   be519:	cmp    $0x4,%r14
   be51d:	jne    be584 <meshoptimizer_rs::codec::allocate+0xd4>
   be51f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   be529:	imul   %rcx,%rax
   be52d:	movabs $0x5555555555555556,%rdx
   be537:	cmp    %rdx,%rax
   be53a:	jb     be56c <meshoptimizer_rs::codec::allocate+0xbc>
   be53c:	xor    %eax,%eax
   be53e:	jmp    be584 <meshoptimizer_rs::codec::allocate+0xd4>
   be540:	lea    -0x1(%r14),%rax
   be544:	cmp    $0x100,%rax
   be54a:	setb   %r10b
   be54e:	test   $0x3,%r14b
   be552:	sete   %dl
   be555:	mov    $0x2,%al
   be557:	test   %dl,%r10b
   be55a:	jne    be56c <meshoptimizer_rs::codec::allocate+0xbc>
   be55c:	jmp    be584 <meshoptimizer_rs::codec::allocate+0xd4>
   be55e:	cmp    $0x4,%r14
   be562:	je     be56c <meshoptimizer_rs::codec::allocate+0xbc>
   be564:	mov    $0x2,%al
   be566:	cmp    $0x2,%r14
   be56a:	jne    be584 <meshoptimizer_rs::codec::allocate+0xd4>
   be56c:	mov    %rcx,%rax
   be56f:	mul    %r14
   be572:	mov    %rax,%rbp
   be575:	seto   %al
   be578:	test   %rbp,%rbp
   be57b:	sets   %dl
   be57e:	or     %al,%dl
   be580:	mov    $0x8,%al
   be582:	je     be5a0 <meshoptimizer_rs::codec::allocate+0xf0>
   be584:	mov    %al,0x8(%rdi)
   be587:	movq   $0xffffffffffffffff,(%rdi)
   be58e:	add    $0x468,%rsp
   be595:	pop    %rbx
   be596:	pop    %r12
   be598:	pop    %r13
   be59a:	pop    %r14
   be59c:	pop    %r15
   be59e:	pop    %rbp
   be59f:	ret
   be5a0:	mov    %esi,0x3c(%rsp)
   be5a4:	mov    %rcx,0x10(%rsp)
   be5a9:	mov    %r9,0x30(%rsp)
   be5ae:	mov    %r8d,0x38(%rsp)
   be5b3:	mov    %rdi,0x40(%rsp)
   be5b8:	mov    %r12,%rdi
   be5bb:	mov    %rbp,%rsi
   be5be:	call   a3000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   be5c3:	mov    %r15,%rbx
   be5c6:	cmp    $0xff,%al
   be5c8:	je     be5e6 <meshoptimizer_rs::codec::allocate+0x136>
   be5ca:	sub    %rbx,%r15
   be5cd:	mov    %r15,0x358(%r12)
   be5d5:	mov    0x40(%rsp),%rcx
   be5da:	mov    %al,0x8(%rcx)
   be5dd:	movq   $0xffffffffffffffff,(%rcx)
   be5e4:	jmp    be58e <meshoptimizer_rs::codec::allocate+0xde>
   be5e6:	mov    0x4a0(%rsp),%rdx
   be5ee:	mov    %r15,%rax
   be5f1:	mov    %rdx,%rcx
   be5f4:	sub    %rdx,%rax
   be5f7:	jb     be966 <meshoptimizer_rs::codec::allocate+0x4b6>
   be5fd:	mov    %rax,0x48(%rsp)
   be602:	mov    %rax,%rbx
   be605:	mov    %rbp,%r10
   be608:	sub    %rbp,%rbx
   be60b:	jb     be96e <meshoptimizer_rs::codec::allocate+0x4be>
   be611:	mov    %rbx,0x48(%rsp)
   be616:	cmpb   $0x0,0x38(%rsp)
   be61b:	mov    0x3c(%rsp),%ebp
   be61f:	je     be635 <meshoptimizer_rs::codec::allocate+0x185>
   be621:	mov    %r10,%rsi
   be624:	shr    $0x2,%rsi
   be628:	mov    %rbx,%rax
   be62b:	sub    %rsi,%rax
   be62e:	jae    be638 <meshoptimizer_rs::codec::allocate+0x188>
   be630:	jmp    be974 <meshoptimizer_rs::codec::allocate+0x4c4>
   be635:	mov    %rbx,%rax
   be638:	sub    %rax,%r15
   be63b:	mov    %r15,0x358(%r12)
   be643:	cmp    $0x2,%bpl
   be647:	mov    0x40(%rsp),%rbx
   be64c:	je     be681 <meshoptimizer_rs::codec::allocate+0x1d1>
   be64e:	cmp    $0x1,%r13d
   be652:	mov    %rcx,%r15
   be655:	jne    be72d <meshoptimizer_rs::codec::allocate+0x27d>
   be65b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   be665:	mov    0x10(%rsp),%rax
   be66a:	mul    %rcx
   be66d:	shr    $1,%rdx
   be670:	add    $0x11,%rdx
   be674:	mov    $0x7,%sil
   be677:	cmp    %rdx,%r15
   be67a:	jae    be699 <meshoptimizer_rs::codec::allocate+0x1e9>
   be67c:	jmp    be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be681:	mov    0x10(%rsp),%rax
   be686:	lea    0x5(%rax),%rdx
   be68a:	mov    %rcx,%r15
   be68d:	mov    $0x7,%sil
   be690:	cmp    %rdx,%r15
   be693:	jb     be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be699:	movq   $0x0,0x18(%rsp)
   be6a2:	movq   $0x1,0x20(%rsp)
   be6ab:	movq   $0x0,0x28(%rsp)
   be6b4:	test   %r10,%r10
   be6b7:	jne    be7fd <meshoptimizer_rs::codec::allocate+0x34d>
   be6bd:	cmp    $0x2,%bpl
   be6c1:	je     be849 <meshoptimizer_rs::codec::allocate+0x399>
   be6c7:	test   %r13d,%r13d
   be6ca:	jne    be770 <meshoptimizer_rs::codec::allocate+0x2c0>
   be6d0:	cmp    $0x4,%r14
   be6d4:	sete   %al
   be6d7:	cmp    $0x100000,%r10
   be6de:	setae  %cl
   be6e1:	test   %cl,%al
   be6e3:	je     be7a5 <meshoptimizer_rs::codec::allocate+0x2f5>
   be6e9:	lea    0x60(%rsp),%rdi
   be6ee:	mov    $0x400,%edx
   be6f3:	xor    %esi,%esi
   be6f5:	call   *0x8bb6d(%rip)        # 14a268 <memset@GLIBC_2.2.5>
   be6fb:	lea    0x18(%rsp),%rax
   be700:	mov    %rax,0x58(%rsp)
   be705:	movq   $0x0,0x460(%rsp)
   be711:	lea    0x58(%rsp),%rdi
   be716:	mov    0x10(%rsp),%rsi
   be71b:	mov    0x30(%rsp),%rdx
   be720:	mov    %r15,%rcx
   be723:	call   7d550 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   be728:	jmp    be863 <meshoptimizer_rs::codec::allocate+0x3b3>
   be72d:	mov    $0x7,%sil
   be730:	test   %r15,%r15
   be733:	je     be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be739:	mov    0x30(%rsp),%rax
   be73e:	movzbl (%rax),%ecx
   be741:	mov    %ecx,%eax
   be743:	and    $0xf0,%al
   be745:	cmp    $0xa0,%al
   be747:	jne    be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be74d:	and    $0xf,%cl
   be750:	mov    $0x5,%sil
   be753:	cmp    $0x1,%cl
   be756:	ja     be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be75c:	mov    0x10(%rsp),%rdi
   be761:	cmp    $0x21,%rdi
   be765:	jae    be7c4 <meshoptimizer_rs::codec::allocate+0x314>
   be767:	xor    %eax,%eax
   be769:	xor    %esi,%esi
   be76b:	jmp    be8d3 <meshoptimizer_rs::codec::allocate+0x423>
   be770:	lea    0x18(%rsp),%rdi
   be775:	mov    %r10,%rsi
   be778:	call   9c5d0 <<alloc::vec::Vec<u8>>::resize>
   be77d:	mov    0x20(%rsp),%rsi
   be782:	mov    0x28(%rsp),%rdx
   be787:	mov    %r15,(%rsp)
   be78b:	mov    %r13d,%edi
   be78e:	mov    0x10(%rsp),%rcx
   be793:	mov    %r14,%r8
   be796:	mov    0x30(%rsp),%r9
   be79b:	call   b91d0 <meshoptimizer_rs::codec::raw>
   be7a0:	jmp    be863 <meshoptimizer_rs::codec::allocate+0x3b3>
   be7a5:	lea    0x18(%rsp),%rdi
   be7aa:	mov    0x10(%rsp),%rsi
   be7af:	mov    %r14,%rdx
   be7b2:	mov    0x30(%rsp),%rcx
   be7b7:	mov    %r15,%r8
   be7ba:	call   e78e0 <meshoptimizer_rs::codec::vertex::decode_owned>
   be7bf:	jmp    be863 <meshoptimizer_rs::codec::allocate+0x3b3>
   be7c4:	mov    $0x2000,%ax
   be7c8:	xor    %edx,%edx
   be7ca:	div    %r14w
   be7ce:	and    $0x3ff0,%eax
   be7d3:	cmp    $0x100,%eax
   be7d8:	mov    $0x100,%esi
   be7dd:	cmovb  %eax,%esi
   be7e0:	mov    %rdi,%rax
   be7e3:	shr    $0x20,%rax
   be7e7:	je     be8be <meshoptimizer_rs::codec::allocate+0x40e>
   be7ed:	mov    %rdi,%rax
   be7f0:	xor    %edx,%edx
   be7f2:	div    %rsi
   be7f5:	mov    %rdx,%rdi
   be7f8:	jmp    be8c6 <meshoptimizer_rs::codec::allocate+0x416>
   be7fd:	lea    0x58(%rsp),%rdi
   be802:	mov    $0x1,%edx
   be807:	mov    $0x1,%r8d
   be80d:	mov    $0x1,%r9d
   be813:	xor    %esi,%esi
   be815:	mov    %r10,%rcx
   be818:	mov    %r10,%r12
   be81b:	call   9d0c0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   be820:	cmpb   $0x0,0x58(%rsp)
   be825:	je     be82d <meshoptimizer_rs::codec::allocate+0x37d>
   be827:	movb   $0xa,0x8(%rbx)
   be82b:	jmp    be885 <meshoptimizer_rs::codec::allocate+0x3d5>
   be82d:	mov    0x60(%rsp),%rax
   be832:	mov    %rax,0x20(%rsp)
   be837:	mov    %r12,0x18(%rsp)
   be83c:	mov    %r12,%r10
   be83f:	cmp    $0x2,%bpl
   be843:	jne    be6c7 <meshoptimizer_rs::codec::allocate+0x217>
   be849:	lea    0x18(%rsp),%rdi
   be84e:	mov    0x10(%rsp),%rsi
   be853:	mov    %r14,%rdx
   be856:	mov    0x30(%rsp),%rcx
   be85b:	mov    %r15,%r8
   be85e:	call   e3de0 <meshoptimizer_rs::codec::index::sequence_append>
   be863:	cmp    $0xff,%al
   be865:	jne    be882 <meshoptimizer_rs::codec::allocate+0x3d2>
   be867:	mov    0x20(%rsp),%rsi
   be86c:	mov    0x28(%rsp),%rdx
   be871:	movzbl 0x38(%rsp),%edi
   be876:	mov    %r14,%rcx
   be879:	call   bae60 <meshoptimizer_rs::codec::apply>
   be87e:	cmp    $0xff,%al
   be880:	je     be8a8 <meshoptimizer_rs::codec::allocate+0x3f8>
   be882:	mov    %al,0x8(%rbx)
   be885:	movq   $0xffffffffffffffff,(%rbx)
   be88c:	cmpq   $0x0,0x18(%rsp)
   be892:	je     be58e <meshoptimizer_rs::codec::allocate+0xde>
   be898:	mov    0x20(%rsp),%rdi
   be89d:	call   *0x8b9a5(%rip)        # 14a248 <free@GLIBC_2.2.5>
   be8a3:	jmp    be58e <meshoptimizer_rs::codec::allocate+0xde>
   be8a8:	mov    0x28(%rsp),%rax
   be8ad:	mov    %rax,0x10(%rbx)
   be8b1:	movups 0x18(%rsp),%xmm0
   be8b6:	movups %xmm0,(%rbx)
   be8b9:	jmp    be58e <meshoptimizer_rs::codec::allocate+0xde>
   be8be:	mov    %edi,%eax
   be8c0:	xor    %edx,%edx
   be8c2:	div    %esi
   be8c4:	mov    %edx,%edi
   be8c6:	shr    $0x4,%esi
   be8c9:	add    $0x3,%esi
   be8cc:	shr    $0x2,%esi
   be8cf:	imul   %rax,%rsi
   be8d3:	test   %cl,%cl
   be8d5:	je     be901 <meshoptimizer_rs::codec::allocate+0x451>
   be8d7:	cmp    $0x1,%rdi
   be8db:	sbb    $0xffffffffffffffff,%rax
   be8df:	mov    %r14,%rcx
   be8e2:	shr    $0x2,%rcx
   be8e6:	mul    %rcx
   be8e9:	jo     be984 <meshoptimizer_rs::codec::allocate+0x4d4>
   be8ef:	add    %r14,%rcx
   be8f2:	cmp    $0x19,%rcx
   be8f6:	mov    $0x18,%edx
   be8fb:	cmovae %rcx,%rdx
   be8ff:	jmp    be939 <meshoptimizer_rs::codec::allocate+0x489>
   be901:	test   %rdi,%rdi
   be904:	je     be922 <meshoptimizer_rs::codec::allocate+0x472>
   be906:	mov    %rdi,%rax
   be909:	shr    $0x4,%rax
   be90d:	and    $0xf,%edi
   be910:	cmp    $0x1,%rdi
   be914:	sbb    $0xffffffffffffffff,%rax
   be918:	add    $0x3,%rax
   be91c:	shr    $0x2,%rax
   be920:	jmp    be924 <meshoptimizer_rs::codec::allocate+0x474>
   be922:	xor    %eax,%eax
   be924:	add    %rsi,%rax
   be927:	mul    %r14
   be92a:	jo     be984 <meshoptimizer_rs::codec::allocate+0x4d4>
   be92c:	cmp    $0x21,%r14
   be930:	mov    $0x20,%edx
   be935:	cmovae %r14,%rdx
   be939:	mov    $0x8,%sil
   be93c:	add    %rax,%rdx
   be93f:	jb     be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be941:	cmp    $0xffffffffffffffff,%rdx
   be945:	je     be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be947:	inc    %rdx
   be94a:	mov    $0x7,%sil
   be94d:	cmp    %rdx,%r15
   be950:	jae    be699 <meshoptimizer_rs::codec::allocate+0x1e9>
   be956:	mov    %sil,0x8(%rbx)
   be95a:	movq   $0xffffffffffffffff,(%rbx)
   be961:	jmp    be58e <meshoptimizer_rs::codec::allocate+0xde>
   be966:	mov    %rcx,%rsi
   be969:	mov    %r15,%rbx
   be96c:	jmp    be974 <meshoptimizer_rs::codec::allocate+0x4c4>
   be96e:	mov    %r10,%rsi
   be971:	mov    %rax,%rbx
   be974:	lea    0x48(%rsp),%rdi
   be979:	call   *0x8bdd1(%rip)        # 14a750 <_DYNAMIC+0x710>
   be97f:	jmp    be5ca <meshoptimizer_rs::codec::allocate+0x11a>
   be984:	mov    $0x8,%sil
   be987:	jmp    be956 <meshoptimizer_rs::codec::allocate+0x4a6>
   be989:	mov    %rax,%rbx
   be98c:	cmpq   $0x0,0x18(%rsp)
   be992:	je     be99f <meshoptimizer_rs::codec::allocate+0x4ef>
   be994:	mov    0x20(%rsp),%rdi
   be999:	call   *0x8b8a9(%rip)        # 14a248 <free@GLIBC_2.2.5>
   be99f:	mov    %rbx,%rdi
   be9a2:	call   140620 <_Unwind_Resume@plt>
