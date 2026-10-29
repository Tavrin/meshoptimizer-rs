
<artifacts>/screen3/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

00000000000b2680 <meshoptimizer_rs::codec::allocate>:
   b2680:	push   %rbp
   b2681:	push   %r15
   b2683:	push   %r14
   b2685:	push   %r13
   b2687:	push   %r12
   b2689:	push   %rbx
   b268a:	sub    $0x468,%rsp
   b2691:	mov    %r8,%r14
   b2694:	mov    %edx,%r8d
   b2697:	mov    0x4a8(%rsp),%r12
   b269f:	movb   $0x0,0x365(%r12)
   b26a8:	movl   $0x0,0x361(%r12)
   b26b4:	xorps  %xmm0,%xmm0
   b26b7:	movups %xmm0,0x350(%r12)
   b26c0:	mov    0x348(%r12),%r15
   b26c8:	mov    %r15,0x48(%rsp)
   b26cd:	mov    %r15,0x50(%rsp)
   b26d2:	movzbl %sil,%r13d
   b26d6:	test   %r13b,%r13b
   b26d9:	je     b2710 <meshoptimizer_rs::codec::allocate+0x90>
   b26db:	cmp    $0x1,%r13d
   b26df:	jne    b272e <meshoptimizer_rs::codec::allocate+0xae>
   b26e1:	cmp    $0x2,%r14
   b26e5:	je     b26ef <meshoptimizer_rs::codec::allocate+0x6f>
   b26e7:	mov    $0x2,%al
   b26e9:	cmp    $0x4,%r14
   b26ed:	jne    b2754 <meshoptimizer_rs::codec::allocate+0xd4>
   b26ef:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b26f9:	imul   %rcx,%rax
   b26fd:	movabs $0x5555555555555556,%rdx
   b2707:	cmp    %rdx,%rax
   b270a:	jb     b273c <meshoptimizer_rs::codec::allocate+0xbc>
   b270c:	xor    %eax,%eax
   b270e:	jmp    b2754 <meshoptimizer_rs::codec::allocate+0xd4>
   b2710:	lea    -0x1(%r14),%rax
   b2714:	cmp    $0x100,%rax
   b271a:	setb   %r10b
   b271e:	test   $0x3,%r14b
   b2722:	sete   %dl
   b2725:	mov    $0x2,%al
   b2727:	test   %dl,%r10b
   b272a:	jne    b273c <meshoptimizer_rs::codec::allocate+0xbc>
   b272c:	jmp    b2754 <meshoptimizer_rs::codec::allocate+0xd4>
   b272e:	cmp    $0x4,%r14
   b2732:	je     b273c <meshoptimizer_rs::codec::allocate+0xbc>
   b2734:	mov    $0x2,%al
   b2736:	cmp    $0x2,%r14
   b273a:	jne    b2754 <meshoptimizer_rs::codec::allocate+0xd4>
   b273c:	mov    %rcx,%rax
   b273f:	mul    %r14
   b2742:	mov    %rax,%rbp
   b2745:	seto   %al
   b2748:	test   %rbp,%rbp
   b274b:	sets   %dl
   b274e:	or     %al,%dl
   b2750:	mov    $0x8,%al
   b2752:	je     b2770 <meshoptimizer_rs::codec::allocate+0xf0>
   b2754:	mov    %al,0x8(%rdi)
   b2757:	movq   $0xffffffffffffffff,(%rdi)
   b275e:	add    $0x468,%rsp
   b2765:	pop    %rbx
   b2766:	pop    %r12
   b2768:	pop    %r13
   b276a:	pop    %r14
   b276c:	pop    %r15
   b276e:	pop    %rbp
   b276f:	ret
   b2770:	mov    %esi,0x3c(%rsp)
   b2774:	mov    %rcx,0x10(%rsp)
   b2779:	mov    %r9,0x30(%rsp)
   b277e:	mov    %r8d,0x38(%rsp)
   b2783:	mov    %rdi,0x40(%rsp)
   b2788:	mov    %r12,%rdi
   b278b:	mov    %rbp,%rsi
   b278e:	call   97200 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b2793:	mov    %r15,%rbx
   b2796:	cmp    $0xff,%al
   b2798:	je     b27b6 <meshoptimizer_rs::codec::allocate+0x136>
   b279a:	sub    %rbx,%r15
   b279d:	mov    %r15,0x358(%r12)
   b27a5:	mov    0x40(%rsp),%rcx
   b27aa:	mov    %al,0x8(%rcx)
   b27ad:	movq   $0xffffffffffffffff,(%rcx)
   b27b4:	jmp    b275e <meshoptimizer_rs::codec::allocate+0xde>
   b27b6:	mov    0x4a0(%rsp),%rdx
   b27be:	mov    %r15,%rax
   b27c1:	mov    %rdx,%rcx
   b27c4:	sub    %rdx,%rax
   b27c7:	jb     b2b36 <meshoptimizer_rs::codec::allocate+0x4b6>
   b27cd:	mov    %rax,0x48(%rsp)
   b27d2:	mov    %rax,%rbx
   b27d5:	mov    %rbp,%r10
   b27d8:	sub    %rbp,%rbx
   b27db:	jb     b2b3e <meshoptimizer_rs::codec::allocate+0x4be>
   b27e1:	mov    %rbx,0x48(%rsp)
   b27e6:	cmpb   $0x0,0x38(%rsp)
   b27eb:	mov    0x3c(%rsp),%ebp
   b27ef:	je     b2805 <meshoptimizer_rs::codec::allocate+0x185>
   b27f1:	mov    %r10,%rsi
   b27f4:	shr    $0x2,%rsi
   b27f8:	mov    %rbx,%rax
   b27fb:	sub    %rsi,%rax
   b27fe:	jae    b2808 <meshoptimizer_rs::codec::allocate+0x188>
   b2800:	jmp    b2b44 <meshoptimizer_rs::codec::allocate+0x4c4>
   b2805:	mov    %rbx,%rax
   b2808:	sub    %rax,%r15
   b280b:	mov    %r15,0x358(%r12)
   b2813:	cmp    $0x2,%bpl
   b2817:	mov    0x40(%rsp),%rbx
   b281c:	je     b2851 <meshoptimizer_rs::codec::allocate+0x1d1>
   b281e:	cmp    $0x1,%r13d
   b2822:	mov    %rcx,%r15
   b2825:	jne    b28fd <meshoptimizer_rs::codec::allocate+0x27d>
   b282b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b2835:	mov    0x10(%rsp),%rax
   b283a:	mul    %rcx
   b283d:	shr    $1,%rdx
   b2840:	add    $0x11,%rdx
   b2844:	mov    $0x7,%sil
   b2847:	cmp    %rdx,%r15
   b284a:	jae    b2869 <meshoptimizer_rs::codec::allocate+0x1e9>
   b284c:	jmp    b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2851:	mov    0x10(%rsp),%rax
   b2856:	lea    0x5(%rax),%rdx
   b285a:	mov    %rcx,%r15
   b285d:	mov    $0x7,%sil
   b2860:	cmp    %rdx,%r15
   b2863:	jb     b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2869:	movq   $0x0,0x18(%rsp)
   b2872:	movq   $0x1,0x20(%rsp)
   b287b:	movq   $0x0,0x28(%rsp)
   b2884:	test   %r10,%r10
   b2887:	jne    b29cd <meshoptimizer_rs::codec::allocate+0x34d>
   b288d:	cmp    $0x2,%bpl
   b2891:	je     b2a19 <meshoptimizer_rs::codec::allocate+0x399>
   b2897:	test   %r13d,%r13d
   b289a:	jne    b2940 <meshoptimizer_rs::codec::allocate+0x2c0>
   b28a0:	cmp    $0x4,%r14
   b28a4:	sete   %al
   b28a7:	cmp    $0x100000,%r10
   b28ae:	setae  %cl
   b28b1:	test   %cl,%al
   b28b3:	je     b2975 <meshoptimizer_rs::codec::allocate+0x2f5>
   b28b9:	lea    0x60(%rsp),%rdi
   b28be:	mov    $0x400,%edx
   b28c3:	xor    %esi,%esi
   b28c5:	call   *0x7685d(%rip)        # 129128 <memset@GLIBC_2.2.5>
   b28cb:	lea    0x18(%rsp),%rax
   b28d0:	mov    %rax,0x58(%rsp)
   b28d5:	movq   $0x0,0x460(%rsp)
   b28e1:	lea    0x58(%rsp),%rdi
   b28e6:	mov    0x10(%rsp),%rsi
   b28eb:	mov    0x30(%rsp),%rdx
   b28f0:	mov    %r15,%rcx
   b28f3:	call   73060 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b28f8:	jmp    b2a33 <meshoptimizer_rs::codec::allocate+0x3b3>
   b28fd:	mov    $0x7,%sil
   b2900:	test   %r15,%r15
   b2903:	je     b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2909:	mov    0x30(%rsp),%rax
   b290e:	movzbl (%rax),%ecx
   b2911:	mov    %ecx,%eax
   b2913:	and    $0xf0,%al
   b2915:	cmp    $0xa0,%al
   b2917:	jne    b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b291d:	and    $0xf,%cl
   b2920:	mov    $0x5,%sil
   b2923:	cmp    $0x1,%cl
   b2926:	ja     b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b292c:	mov    0x10(%rsp),%rdi
   b2931:	cmp    $0x21,%rdi
   b2935:	jae    b2994 <meshoptimizer_rs::codec::allocate+0x314>
   b2937:	xor    %eax,%eax
   b2939:	xor    %esi,%esi
   b293b:	jmp    b2aa3 <meshoptimizer_rs::codec::allocate+0x423>
   b2940:	lea    0x18(%rsp),%rdi
   b2945:	mov    %r10,%rsi
   b2948:	call   90980 <<alloc::vec::Vec<u8>>::resize>
   b294d:	mov    0x20(%rsp),%rsi
   b2952:	mov    0x28(%rsp),%rdx
   b2957:	mov    %r15,(%rsp)
   b295b:	mov    %r13d,%edi
   b295e:	mov    0x10(%rsp),%rcx
   b2963:	mov    %r14,%r8
   b2966:	mov    0x30(%rsp),%r9
   b296b:	call   ad3a0 <meshoptimizer_rs::codec::raw>
   b2970:	jmp    b2a33 <meshoptimizer_rs::codec::allocate+0x3b3>
   b2975:	lea    0x18(%rsp),%rdi
   b297a:	mov    0x10(%rsp),%rsi
   b297f:	mov    %r14,%rdx
   b2982:	mov    0x30(%rsp),%rcx
   b2987:	mov    %r15,%r8
   b298a:	call   d5280 <meshoptimizer_rs::codec::vertex::decode_owned>
   b298f:	jmp    b2a33 <meshoptimizer_rs::codec::allocate+0x3b3>
   b2994:	mov    $0x2000,%ax
   b2998:	xor    %edx,%edx
   b299a:	div    %r14w
   b299e:	and    $0x3ff0,%eax
   b29a3:	cmp    $0x100,%eax
   b29a8:	mov    $0x100,%esi
   b29ad:	cmovb  %eax,%esi
   b29b0:	mov    %rdi,%rax
   b29b3:	shr    $0x20,%rax
   b29b7:	je     b2a8e <meshoptimizer_rs::codec::allocate+0x40e>
   b29bd:	mov    %rdi,%rax
   b29c0:	xor    %edx,%edx
   b29c2:	div    %rsi
   b29c5:	mov    %rdx,%rdi
   b29c8:	jmp    b2a96 <meshoptimizer_rs::codec::allocate+0x416>
   b29cd:	lea    0x58(%rsp),%rdi
   b29d2:	mov    $0x1,%edx
   b29d7:	mov    $0x1,%r8d
   b29dd:	mov    $0x1,%r9d
   b29e3:	xor    %esi,%esi
   b29e5:	mov    %r10,%rcx
   b29e8:	mov    %r10,%r12
   b29eb:	call   91470 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b29f0:	cmpb   $0x0,0x58(%rsp)
   b29f5:	je     b29fd <meshoptimizer_rs::codec::allocate+0x37d>
   b29f7:	movb   $0xa,0x8(%rbx)
   b29fb:	jmp    b2a55 <meshoptimizer_rs::codec::allocate+0x3d5>
   b29fd:	mov    0x60(%rsp),%rax
   b2a02:	mov    %rax,0x20(%rsp)
   b2a07:	mov    %r12,0x18(%rsp)
   b2a0c:	mov    %r12,%r10
   b2a0f:	cmp    $0x2,%bpl
   b2a13:	jne    b2897 <meshoptimizer_rs::codec::allocate+0x217>
   b2a19:	lea    0x18(%rsp),%rdi
   b2a1e:	mov    0x10(%rsp),%rsi
   b2a23:	mov    %r14,%rdx
   b2a26:	mov    0x30(%rsp),%rcx
   b2a2b:	mov    %r15,%r8
   b2a2e:	call   d1780 <meshoptimizer_rs::codec::index::sequence_append>
   b2a33:	cmp    $0xff,%al
   b2a35:	jne    b2a52 <meshoptimizer_rs::codec::allocate+0x3d2>
   b2a37:	mov    0x20(%rsp),%rsi
   b2a3c:	mov    0x28(%rsp),%rdx
   b2a41:	movzbl 0x38(%rsp),%edi
   b2a46:	mov    %r14,%rcx
   b2a49:	call   af030 <meshoptimizer_rs::codec::apply>
   b2a4e:	cmp    $0xff,%al
   b2a50:	je     b2a78 <meshoptimizer_rs::codec::allocate+0x3f8>
   b2a52:	mov    %al,0x8(%rbx)
   b2a55:	movq   $0xffffffffffffffff,(%rbx)
   b2a5c:	cmpq   $0x0,0x18(%rsp)
   b2a62:	je     b275e <meshoptimizer_rs::codec::allocate+0xde>
   b2a68:	mov    0x20(%rsp),%rdi
   b2a6d:	call   *0x76695(%rip)        # 129108 <free@GLIBC_2.2.5>
   b2a73:	jmp    b275e <meshoptimizer_rs::codec::allocate+0xde>
   b2a78:	mov    0x28(%rsp),%rax
   b2a7d:	mov    %rax,0x10(%rbx)
   b2a81:	movups 0x18(%rsp),%xmm0
   b2a86:	movups %xmm0,(%rbx)
   b2a89:	jmp    b275e <meshoptimizer_rs::codec::allocate+0xde>
   b2a8e:	mov    %edi,%eax
   b2a90:	xor    %edx,%edx
   b2a92:	div    %esi
   b2a94:	mov    %edx,%edi
   b2a96:	shr    $0x4,%esi
   b2a99:	add    $0x3,%esi
   b2a9c:	shr    $0x2,%esi
   b2a9f:	imul   %rax,%rsi
   b2aa3:	test   %cl,%cl
   b2aa5:	je     b2ad1 <meshoptimizer_rs::codec::allocate+0x451>
   b2aa7:	cmp    $0x1,%rdi
   b2aab:	sbb    $0xffffffffffffffff,%rax
   b2aaf:	mov    %r14,%rcx
   b2ab2:	shr    $0x2,%rcx
   b2ab6:	mul    %rcx
   b2ab9:	jo     b2b54 <meshoptimizer_rs::codec::allocate+0x4d4>
   b2abf:	add    %r14,%rcx
   b2ac2:	cmp    $0x19,%rcx
   b2ac6:	mov    $0x18,%edx
   b2acb:	cmovae %rcx,%rdx
   b2acf:	jmp    b2b09 <meshoptimizer_rs::codec::allocate+0x489>
   b2ad1:	test   %rdi,%rdi
   b2ad4:	je     b2af2 <meshoptimizer_rs::codec::allocate+0x472>
   b2ad6:	mov    %rdi,%rax
   b2ad9:	shr    $0x4,%rax
   b2add:	and    $0xf,%edi
   b2ae0:	cmp    $0x1,%rdi
   b2ae4:	sbb    $0xffffffffffffffff,%rax
   b2ae8:	add    $0x3,%rax
   b2aec:	shr    $0x2,%rax
   b2af0:	jmp    b2af4 <meshoptimizer_rs::codec::allocate+0x474>
   b2af2:	xor    %eax,%eax
   b2af4:	add    %rsi,%rax
   b2af7:	mul    %r14
   b2afa:	jo     b2b54 <meshoptimizer_rs::codec::allocate+0x4d4>
   b2afc:	cmp    $0x21,%r14
   b2b00:	mov    $0x20,%edx
   b2b05:	cmovae %r14,%rdx
   b2b09:	mov    $0x8,%sil
   b2b0c:	add    %rax,%rdx
   b2b0f:	jb     b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2b11:	cmp    $0xffffffffffffffff,%rdx
   b2b15:	je     b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2b17:	inc    %rdx
   b2b1a:	mov    $0x7,%sil
   b2b1d:	cmp    %rdx,%r15
   b2b20:	jae    b2869 <meshoptimizer_rs::codec::allocate+0x1e9>
   b2b26:	mov    %sil,0x8(%rbx)
   b2b2a:	movq   $0xffffffffffffffff,(%rbx)
   b2b31:	jmp    b275e <meshoptimizer_rs::codec::allocate+0xde>
   b2b36:	mov    %rcx,%rsi
   b2b39:	mov    %r15,%rbx
   b2b3c:	jmp    b2b44 <meshoptimizer_rs::codec::allocate+0x4c4>
   b2b3e:	mov    %r10,%rsi
   b2b41:	mov    %rax,%rbx
   b2b44:	lea    0x48(%rsp),%rdi
   b2b49:	call   *0x76a79(%rip)        # 1295c8 <_DYNAMIC+0x6c8>
   b2b4f:	jmp    b279a <meshoptimizer_rs::codec::allocate+0x11a>
   b2b54:	mov    $0x8,%sil
   b2b57:	jmp    b2b26 <meshoptimizer_rs::codec::allocate+0x4a6>
   b2b59:	mov    %rax,%rbx
   b2b5c:	cmpq   $0x0,0x18(%rsp)
   b2b62:	je     b2b6f <meshoptimizer_rs::codec::allocate+0x4ef>
   b2b64:	mov    0x20(%rsp),%rdi
   b2b69:	call   *0x76599(%rip)        # 129108 <free@GLIBC_2.2.5>
   b2b6f:	mov    %rbx,%rdi
   b2b72:	call   120150 <_Unwind_Resume@plt>
