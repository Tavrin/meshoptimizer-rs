
<artifacts>/guard/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

000000000007d760 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>:
   7d760:	push   %rbp
   7d761:	push   %r15
   7d763:	push   %r14
   7d765:	push   %r13
   7d767:	push   %r12
   7d769:	push   %rbx
   7d76a:	sub    $0x598,%rsp
   7d771:	mov    %rcx,%r15
   7d774:	mov    %rdx,%r14
   7d777:	mov    %rsi,%rbx
   7d77a:	mov    %rdi,%r12
   7d77d:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7d782:	cmp    $0x2,%al
   7d784:	jb     7d7b8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x58>
   7d786:	cmp    $0x21,%rbx
   7d78a:	lea    0xe87f(%rip),%rax        # 8c010 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7d791:	lea    0x8088(%rip),%r9        # 85820 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, meshoptimizer_rs::codec::vertex::StreamingDestination>>
   7d798:	cmovb  %rax,%r9
   7d79c:	mov    $0x4,%edx
   7d7a1:	mov    %r12,%rdi
   7d7a4:	mov    %rbx,%rsi
   7d7a7:	mov    %r14,%rcx
   7d7aa:	mov    %r15,%r8
   7d7ad:	call   *%r9
   7d7b0:	cmp    $0xfe,%al
   7d7b2:	jne    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d7b8:	mov    $0x7,%al
   7d7ba:	test   %r15,%r15
   7d7bd:	je     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d7c3:	movzbl (%r14),%ebp
   7d7c7:	mov    %ebp,%ecx
   7d7c9:	and    $0xf0,%cl
   7d7cc:	cmp    $0xa0,%cl
   7d7cf:	jne    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d7d5:	and    $0xf,%bpl
   7d7d9:	mov    $0x5,%al
   7d7db:	cmp    $0x1,%bpl
   7d7df:	ja     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d7e5:	xor    %eax,%eax
   7d7e7:	test   %bpl,%bpl
   7d7ea:	sete   %al
   7d7ed:	lea    0x18(,%rax,8),%rcx
   7d7f5:	mov    $0x7,%al
   7d7f7:	cmp    %rcx,%r15
   7d7fa:	jbe    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d800:	cmp    $0x1,%bpl
   7d804:	mov    %r15,%rax
   7d807:	adc    $0x0,%rax
   7d80b:	lea    -0x1(%rax),%rdx
   7d80f:	mov    %rdx,0x70(%rsp)
   7d814:	cmp    %r15,%rdx
   7d817:	ja     7e3eb <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc8b>
   7d81d:	mov    %rcx,0x88(%rsp)
   7d825:	add    $0xfffffffffffffffb,%rax
   7d829:	pxor   %xmm0,%xmm0
   7d82d:	movdqu %xmm0,0x188(%rsp)
   7d836:	movdqu %xmm0,0x17c(%rsp)
   7d83f:	movdqu %xmm0,0x16c(%rsp)
   7d848:	movdqu %xmm0,0x15c(%rsp)
   7d851:	movdqu %xmm0,0x14c(%rsp)
   7d85a:	movdqu %xmm0,0x13c(%rsp)
   7d863:	movdqu %xmm0,0x12c(%rsp)
   7d86c:	movdqu %xmm0,0x11c(%rsp)
   7d875:	movdqu %xmm0,0x10c(%rsp)
   7d87e:	movdqu %xmm0,0xfc(%rsp)
   7d887:	movdqu %xmm0,0xec(%rsp)
   7d890:	movdqu %xmm0,0xdc(%rsp)
   7d899:	movdqu %xmm0,0xcc(%rsp)
   7d8a2:	movdqu %xmm0,0xbc(%rsp)
   7d8ab:	movdqu %xmm0,0xac(%rsp)
   7d8b4:	movdqu %xmm0,0x9c(%rsp)
   7d8bd:	mov    (%r14,%rax,1),%r13d
   7d8c1:	mov    %r13d,0x98(%rsp)
   7d8c9:	lea    0x198(%rsp),%rdi
   7d8d1:	mov    $0x400,%edx
   7d8d6:	xor    %esi,%esi
   7d8d8:	call   *0xced2a(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   7d8de:	test   %rbx,%rbx
   7d8e1:	je     7d909 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1a9>
   7d8e3:	mov    %r13d,0x5c(%rsp)
   7d8e8:	mov    $0x1,%r8d
   7d8ee:	xor    %ecx,%ecx
   7d8f0:	mov    %r15,0x50(%rsp)
   7d8f5:	mov    %r14,0x28(%rsp)
   7d8fa:	mov    %rbx,0x80(%rsp)
   7d902:	mov    %bpl,0x27(%rsp)
   7d907:	jmp    7d988 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x228>
   7d909:	mov    $0x1,%r8d
   7d90f:	sub    %r8,%r15
   7d912:	setae  %cl
   7d915:	cmp    0x88(%rsp),%r15
   7d91d:	sete   %al
   7d920:	and    %cl,%al
   7d922:	neg    %al
   7d924:	or     $0x7,%al
   7d926:	jmp    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d92b:	mov    $0x1,%ecx
   7d930:	mov    $0x1,%r8d
   7d936:	mov    %r13,%rdi
   7d939:	mov    %rbp,%rsi
   7d93c:	mov    %r15,%rdx
   7d93f:	call   8ea70 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   7d944:	mov    0x10(%r13),%rbp
   7d948:	mov    0x8(%r13),%rdi
   7d94c:	add    %rbp,%rdi
   7d94f:	lea    0x8(%r12),%rsi
   7d954:	mov    %r15,%rdx
   7d957:	call   *0xcec9b(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   7d95d:	add    %r15,%rbp
   7d960:	mov    %rbp,0x10(%r13)
   7d964:	mov    0x78(%rsp),%rax
   7d969:	mov    %rax,%rcx
   7d96c:	mov    0x80(%rsp),%rbx
   7d974:	cmp    %rbx,%rax
   7d977:	mov    0x50(%rsp),%r15
   7d97c:	movzbl 0x27(%rsp),%ebp
   7d981:	mov    0x48(%rsp),%r8
   7d986:	jae    7d90f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1af>
   7d988:	sub    %rcx,%rbx
   7d98b:	cmp    $0x100,%rbx
   7d992:	mov    $0x100,%r13d
   7d998:	cmovb  %rbx,%r13
   7d99c:	lea    0xf(%r13),%r9d
   7d9a0:	and    $0x3f0,%r9d
   7d9a7:	test   %bpl,%bpl
   7d9aa:	mov    %r9,0x48(%rsp)
   7d9af:	mov    %rcx,0x90(%rsp)
   7d9b7:	mov    %rbx,0x60(%rsp)
   7d9bc:	je     7da4d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x2ed>
   7d9c2:	mov    $0x7,%al
   7d9c4:	cmp    %r15,%r8
   7d9c7:	jae    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7d9cd:	add    %r13,%rcx
   7d9d0:	mov    %rcx,0x78(%rsp)
   7d9d5:	lea    0x0(,%r13,4),%rbp
   7d9dd:	mov    %rbp,0x408(%r12)
   7d9e5:	movzbl (%r14,%r8,1),%r14d
   7d9ea:	inc    %r8
   7d9ed:	mov    %r14d,%ecx
   7d9f0:	and    $0x3,%ecx
   7d9f3:	lea    0x0(,%r13,2),%rdx
   7d9fb:	add    %r13,%rdx
   7d9fe:	mov    %rdx,0x68(%rsp)
   7da03:	cmp    %r9,%r13
   7da06:	ja     7e159 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x9f9>
   7da0c:	cmp    $0x2,%cl
   7da0f:	je     7da8d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x32d>
   7da11:	movzbl %cl,%edx
   7da14:	cmp    $0x3,%edx
   7da17:	jne    7daa5 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x345>
   7da1d:	lea    (%r8,%r13,1),%rbx
   7da21:	cmp    %r15,%rbx
   7da24:	ja     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7da2a:	add    0x28(%rsp),%r8
   7da2f:	lea    0x198(%rsp),%rdi
   7da37:	mov    %r8,%rsi
   7da3a:	mov    %r13,%rdx
   7da3d:	call   *0xcebb5(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   7da43:	mov    0x50(%rsp),%r15
   7da48:	jmp    7daef <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7da4d:	lea    (%rcx,%r13,1),%rax
   7da51:	mov    %rax,0x78(%rsp)
   7da56:	lea    0x0(,%r13,4),%rbp
   7da5e:	mov    %rbp,0x408(%r12)
   7da66:	xor    %r14d,%r14d
   7da69:	lea    0x0(,%r13,2),%rax
   7da71:	add    %r13,%rax
   7da74:	mov    %rax,0x68(%rsp)
   7da79:	lea    -0x6f2e0(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7da80:	mov    %rcx,%rax
   7da83:	cmp    %r9,%r13
   7da86:	jbe    7dabe <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x35e>
   7da88:	jmp    7e184 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xa24>
   7da8d:	lea    0x198(%rsp),%rdi
   7da95:	xor    %esi,%esi
   7da97:	mov    %r13,%rdx
   7da9a:	mov    %r8,%rbx
   7da9d:	call   *0xceb65(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   7daa3:	jmp    7daef <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x38f>
   7daa5:	test   %cl,%cl
   7daa7:	lea    -0x6efee(%rip),%rcx        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7daae:	lea    -0x6eec5(%rip),%rax        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dab5:	cmove  %rax,%rcx
   7dab9:	mov    0x48(%rsp),%r9
   7dabe:	mov    %rcx,(%rsp)
   7dac2:	lea    0x30(%rsp),%rdi
   7dac7:	mov    0x28(%rsp),%rsi
   7dacc:	mov    %r15,%rdx
   7dacf:	mov    %r8,%rcx
   7dad2:	lea    0x198(%rsp),%r8
   7dada:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7dadf:	cmpb   $0x0,0x30(%rsp)
   7dae4:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7daea:	mov    0x38(%rsp),%rbx
   7daef:	mov    %r14d,%eax
   7daf2:	shr    $0x2,%al
   7daf5:	and    $0x3,%al
   7daf7:	lea    (%rsp,%r13,1),%r8
   7dafb:	add    $0x198,%r8
   7db02:	cmp    $0x3,%al
   7db04:	je     7db21 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3c1>
   7db06:	movzbl %al,%ecx
   7db09:	cmp    $0x2,%ecx
   7db0c:	jne    7db58 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x3f8>
   7db0e:	mov    %r8,%rdi
   7db11:	xor    %esi,%esi
   7db13:	mov    %r13,%rdx
   7db16:	call   *0xceaec(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   7db1c:	jmp    7dbaa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7db21:	mov    %rbx,%r15
   7db24:	mov    $0x7,%al
   7db26:	add    %r13,%r15
   7db29:	jb     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7db2f:	cmp    0x50(%rsp),%r15
   7db34:	ja     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7db3a:	add    0x28(%rsp),%rbx
   7db3f:	mov    %r8,%rdi
   7db42:	mov    %rbx,%rsi
   7db45:	mov    %r13,%rdx
   7db48:	call   *0xceaaa(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   7db4e:	mov    %r15,%rbx
   7db51:	mov    0x50(%rsp),%r15
   7db56:	jmp    7dbaa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x44a>
   7db58:	test   %al,%al
   7db5a:	lea    -0x6f0a1(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7db61:	lea    -0x6ef78(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7db68:	cmove  %rcx,%rax
   7db6c:	cmpb   $0x0,0x27(%rsp)
   7db71:	lea    -0x6f3d8(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7db78:	cmove  %rcx,%rax
   7db7c:	mov    %rax,(%rsp)
   7db80:	lea    0x30(%rsp),%rdi
   7db85:	mov    0x28(%rsp),%rsi
   7db8a:	mov    %r15,%rdx
   7db8d:	mov    %rbx,%rcx
   7db90:	mov    0x48(%rsp),%r9
   7db95:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7db9a:	cmpb   $0x0,0x30(%rsp)
   7db9f:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7dba5:	mov    0x38(%rsp),%rbx
   7dbaa:	mov    %r14d,%eax
   7dbad:	shr    $0x4,%al
   7dbb0:	and    $0x3,%al
   7dbb2:	lea    (%rsp,%r13,2),%r8
   7dbb6:	add    $0x198,%r8
   7dbbd:	cmp    $0x3,%al
   7dbbf:	je     7dbdc <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x47c>
   7dbc1:	movzbl %al,%ecx
   7dbc4:	cmp    $0x2,%ecx
   7dbc7:	jne    7dc13 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x4b3>
   7dbc9:	mov    %r8,%rdi
   7dbcc:	xor    %esi,%esi
   7dbce:	mov    %r13,%rdx
   7dbd1:	call   *0xcea31(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   7dbd7:	jmp    7dc65 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7dbdc:	mov    %rbx,%r15
   7dbdf:	mov    $0x7,%al
   7dbe1:	add    %r13,%r15
   7dbe4:	jb     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7dbea:	cmp    0x50(%rsp),%r15
   7dbef:	ja     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7dbf5:	add    0x28(%rsp),%rbx
   7dbfa:	mov    %r8,%rdi
   7dbfd:	mov    %rbx,%rsi
   7dc00:	mov    %r13,%rdx
   7dc03:	call   *0xce9ef(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   7dc09:	mov    %r15,%rbx
   7dc0c:	mov    0x50(%rsp),%r15
   7dc11:	jmp    7dc65 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x505>
   7dc13:	test   %al,%al
   7dc15:	lea    -0x6f15c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7dc1c:	lea    -0x6f033(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dc23:	cmove  %rcx,%rax
   7dc27:	cmpb   $0x0,0x27(%rsp)
   7dc2c:	lea    -0x6f493(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7dc33:	cmove  %rcx,%rax
   7dc37:	mov    %rax,(%rsp)
   7dc3b:	lea    0x30(%rsp),%rdi
   7dc40:	mov    0x28(%rsp),%rsi
   7dc45:	mov    %r15,%rdx
   7dc48:	mov    %rbx,%rcx
   7dc4b:	mov    0x48(%rsp),%r9
   7dc50:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7dc55:	cmpb   $0x0,0x30(%rsp)
   7dc5a:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7dc60:	mov    0x38(%rsp),%rbx
   7dc65:	shr    $0x6,%r14b
   7dc69:	mov    0x68(%rsp),%rax
   7dc6e:	lea    (%rsp,%rax,1),%r8
   7dc72:	add    $0x198,%r8
   7dc79:	cmp    $0x3,%r14b
   7dc7d:	je     7dc9b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x53b>
   7dc7f:	movzbl %r14b,%eax
   7dc83:	cmp    $0x2,%eax
   7dc86:	jne    7dce2 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x582>
   7dc88:	mov    %r8,%rdi
   7dc8b:	xor    %esi,%esi
   7dc8d:	mov    %r13,%rdx
   7dc90:	call   *0xce972(%rip)        # 14c608 <memset@GLIBC_2.2.5>
   7dc96:	jmp    7dd35 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d5>
   7dc9b:	mov    %rbx,%rcx
   7dc9e:	mov    $0x7,%al
   7dca0:	add    %r13,%rcx
   7dca3:	jb     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7dca9:	cmp    %r15,%rcx
   7dcac:	ja     7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7dcb2:	mov    0x28(%rsp),%r14
   7dcb7:	add    %r14,%rbx
   7dcba:	mov    %r8,%rdi
   7dcbd:	mov    %rbx,%rsi
   7dcc0:	mov    %r13,%rdx
   7dcc3:	mov    %rcx,%rbx
   7dcc6:	call   *0xce92c(%rip)        # 14c5f8 <memcpy@GLIBC_2.14>
   7dccc:	mov    0x50(%rsp),%r15
   7dcd1:	cmpb   $0x0,0x27(%rsp)
   7dcd6:	mov    %rbx,0x48(%rsp)
   7dcdb:	jne    7dd46 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5e6>
   7dcdd:	jmp    7dd6e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7dce2:	test   %r14b,%r14b
   7dce5:	lea    -0x6f22c(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7dcec:	lea    -0x6f103(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7dcf3:	cmove  %rcx,%rax
   7dcf7:	cmpb   $0x0,0x27(%rsp)
   7dcfc:	lea    -0x6f563(%rip),%rcx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7dd03:	cmove  %rcx,%rax
   7dd07:	mov    %rax,(%rsp)
   7dd0b:	lea    0x30(%rsp),%rdi
   7dd10:	mov    0x28(%rsp),%rsi
   7dd15:	mov    %r15,%rdx
   7dd18:	mov    %rbx,%rcx
   7dd1b:	mov    0x48(%rsp),%r9
   7dd20:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7dd25:	cmpb   $0x0,0x30(%rsp)
   7dd2a:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7dd30:	mov    0x38(%rsp),%rbx
   7dd35:	mov    0x28(%rsp),%r14
   7dd3a:	cmpb   $0x0,0x27(%rsp)
   7dd3f:	mov    %rbx,0x48(%rsp)
   7dd44:	je     7dd6e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x60e>
   7dd46:	mov    0x70(%rsp),%rax
   7dd4b:	cmp    %rax,%r15
   7dd4e:	je     7e419 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcb9>
   7dd54:	movzbl (%r14,%rax,1),%eax
   7dd59:	mov    %eax,%ecx
   7dd5b:	and    $0x3,%ecx
   7dd5e:	lea    -0x6c6e5(%rip),%rdx        # 11680 <libm::math::generic::sqrt::RSQRT_TAB+0x3d6>
   7dd65:	movslq (%rdx,%rcx,4),%rcx
   7dd69:	add    %rdx,%rcx
   7dd6c:	jmp    *%rcx
   7dd6e:	call   106290 <meshoptimizer_rs::codec::simd::dispatch::selected>
   7dd73:	test   %al,%al
   7dd75:	je     7e044 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x8e4>
   7dd7b:	movd   0x5c(%rsp),%xmm0
   7dd81:	pshufd $0x0,%xmm0,%xmm0
   7dd86:	mov    %r13,%rcx
   7dd89:	shr    $0x4,%rcx
   7dd8d:	je     7dfe9 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x889>
   7dd93:	lea    0x0(,%r13,2),%rdx
   7dd9b:	lea    0x0(,%r13,2),%rsi
   7dda3:	add    %r13,%rsi
   7dda6:	mov    %r13,%r8
   7dda9:	neg    %r8
   7ddac:	mov    %rdx,%r10
   7ddaf:	neg    %r10
   7ddb2:	mov    %rsi,%r14
   7ddb5:	neg    %r14
   7ddb8:	lea    (%rsp,%rsi,1),%r15
   7ddbc:	add    $0x198,%r15
   7ddc3:	shl    $0x4,%rcx
   7ddc7:	xor    %eax,%eax
   7ddc9:	mov    %rbp,%r9
   7ddcc:	xor    %edi,%edi
   7ddce:	movdqa -0x6f607(%rip),%xmm8        # e7d0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0xc0>
   7ddd7:	movdqa -0x6f7a0(%rip),%xmm9        # e640 <anon.9b1ebfc14efcaf6f77b453a6f385b165.152.llvm.16074470296022112615+0x913>
   7dde0:	cmp    %rbp,%rdi
   7dde3:	ja     7e3aa <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc4a>
   7dde9:	mov    %r9,%r11
   7ddec:	sub    $0x10,%r11
   7ddf0:	jb     7e321 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7ddf6:	lea    (%rdi,%r13,1),%rbx
   7ddfa:	cmp    %rbp,%rbx
   7ddfd:	ja     7e349 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbe9>
   7de03:	lea    (%r8,%r9,1),%rbx
   7de07:	cmp    $0x10,%rbx
   7de0b:	jb     7e321 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7de11:	lea    (%rdx,%rdi,1),%rbx
   7de15:	cmp    %rbp,%rbx
   7de18:	ja     7e360 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc00>
   7de1e:	lea    (%r10,%r9,1),%rbx
   7de22:	cmp    $0x10,%rbx
   7de26:	jb     7e321 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7de2c:	lea    (%rsi,%rdi,1),%rbx
   7de30:	cmp    %rbp,%rbx
   7de33:	ja     7e38e <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc2e>
   7de39:	add    %r14,%r9
   7de3c:	cmp    $0x10,%r9
   7de40:	jb     7e321 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbc1>
   7de46:	lea    0x3f(%rax),%r9
   7de4a:	cmp    %rbp,%r9
   7de4d:	jae    7e332 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbd2>
   7de53:	movdqu 0x198(%rsp,%rdi,1),%xmm3
   7de5c:	lea    (%rsp,%r13,1),%r9
   7de60:	add    $0x198,%r9
   7de67:	movdqu (%r9,%rdi,1),%xmm4
   7de6d:	lea    0x198(%rsp,%r13,2),%r9
   7de75:	movdqu (%r9,%rdi,1),%xmm5
   7de7b:	movdqa %xmm3,%xmm2
   7de7f:	punpckhbw %xmm4,%xmm2
   7de83:	movdqu (%r15,%rdi,1),%xmm6
   7de89:	movdqa %xmm5,%xmm7
   7de8d:	punpckhbw %xmm6,%xmm7
   7de91:	movdqa %xmm2,%xmm1
   7de95:	punpckhwd %xmm7,%xmm1
   7de99:	punpcklwd %xmm7,%xmm2
   7de9d:	punpcklbw %xmm4,%xmm3
   7dea1:	punpcklbw %xmm6,%xmm5
   7dea5:	movdqa %xmm3,%xmm4
   7dea9:	punpckhwd %xmm5,%xmm4
   7dead:	punpcklwd %xmm5,%xmm3
   7deb1:	movdqa %xmm3,%xmm5
   7deb5:	psrlw  $0x1,%xmm5
   7deba:	pand   %xmm8,%xmm3
   7debf:	pxor   %xmm6,%xmm6
   7dec3:	psubb  %xmm3,%xmm6
   7dec7:	pand   %xmm9,%xmm5
   7decc:	pxor   %xmm6,%xmm5
   7ded0:	movdqa %xmm5,%xmm3
   7ded4:	pslldq $0x4,%xmm3
   7ded9:	paddb  %xmm5,%xmm3
   7dedd:	movdqa %xmm3,%xmm5
   7dee1:	pslldq $0x8,%xmm5
   7dee6:	paddb  %xmm0,%xmm3
   7deea:	paddb  %xmm5,%xmm3
   7deee:	movdqu %xmm3,0x8(%r12,%rdi,4)
   7def5:	movdqa %xmm4,%xmm0
   7def9:	psrlw  $0x1,%xmm0
   7defe:	pand   %xmm8,%xmm4
   7df03:	pxor   %xmm5,%xmm5
   7df07:	psubb  %xmm4,%xmm5
   7df0b:	pand   %xmm9,%xmm0
   7df10:	pxor   %xmm5,%xmm0
   7df14:	movdqa %xmm0,%xmm4
   7df18:	pslldq $0x4,%xmm4
   7df1d:	paddb  %xmm0,%xmm4
   7df21:	movdqa %xmm4,%xmm0
   7df25:	pslldq $0x8,%xmm0
   7df2a:	pshufd $0xff,%xmm3,%xmm3
   7df2f:	paddb  %xmm4,%xmm0
   7df33:	paddb  %xmm3,%xmm0
   7df37:	movdqu %xmm0,0x18(%r12,%rdi,4)
   7df3e:	movdqa %xmm2,%xmm3
   7df42:	psrlw  $0x1,%xmm3
   7df47:	pand   %xmm8,%xmm2
   7df4c:	pxor   %xmm4,%xmm4
   7df50:	psubb  %xmm2,%xmm4
   7df54:	pand   %xmm9,%xmm3
   7df59:	pxor   %xmm4,%xmm3
   7df5d:	movdqa %xmm3,%xmm2
   7df61:	pslldq $0x4,%xmm2
   7df66:	paddb  %xmm3,%xmm2
   7df6a:	movdqa %xmm2,%xmm3
   7df6e:	pslldq $0x8,%xmm3
   7df73:	pshufd $0xff,%xmm0,%xmm0
   7df78:	paddb  %xmm2,%xmm3
   7df7c:	paddb  %xmm0,%xmm3
   7df80:	movdqu %xmm3,0x28(%r12,%rdi,4)
   7df87:	movdqa %xmm1,%xmm0
   7df8b:	psrlw  $0x1,%xmm0
   7df90:	pand   %xmm8,%xmm1
   7df95:	pxor   %xmm2,%xmm2
   7df99:	psubb  %xmm1,%xmm2
   7df9d:	pand   %xmm9,%xmm0
   7dfa2:	pxor   %xmm2,%xmm0
   7dfa6:	movdqa %xmm0,%xmm1
   7dfaa:	pslldq $0x4,%xmm1
   7dfaf:	paddb  %xmm0,%xmm1
   7dfb3:	movdqa %xmm1,%xmm0
   7dfb7:	pslldq $0x8,%xmm0
   7dfbc:	pshufd $0xff,%xmm3,%xmm2
   7dfc1:	paddb  %xmm1,%xmm0
   7dfc5:	paddb  %xmm2,%xmm0
   7dfc9:	movdqu %xmm0,0x38(%r12,%rdi,4)
   7dfd0:	add    $0x10,%rdi
   7dfd4:	pshufd $0xff,%xmm0,%xmm0
   7dfd9:	add    $0x40,%rax
   7dfdd:	mov    %r11,%r9
   7dfe0:	cmp    %rdi,%rcx
   7dfe3:	jne    7dde0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x680>
   7dfe9:	mov    %r13d,%eax
   7dfec:	and    $0x1f0,%eax
   7dff1:	cmp    %r13,%rax
   7dff4:	mov    0x28(%rsp),%r14
   7dff9:	je     7e0f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7dfff:	movdqa %xmm0,0x30(%rsp)
   7e005:	lea    0x0(,%rax,4),%edx
   7e00c:	mov    %rbp,%rcx
   7e00f:	sub    %rdx,%rcx
   7e012:	jb     7e403 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xca3>
   7e018:	lea    0x8(%r12),%rsi
   7e01d:	add    %rsi,%rdx
   7e020:	mov    %rax,0x8(%rsp)
   7e025:	lea    0x30(%rsp),%rax
   7e02a:	mov    %rax,(%rsp)
   7e02e:	mov    $0x4,%r9d
   7e034:	lea    0x198(%rsp),%rdi
   7e03c:	mov    %r13,%r8
   7e03f:	mov    %rbp,%rsi
   7e042:	jmp    7e075 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x915>
   7e044:	lea    0x98(%rsp),%rax
   7e04c:	mov    %rax,(%rsp)
   7e050:	movq   $0x0,0x8(%rsp)
   7e059:	mov    $0x4,%r9d
   7e05f:	lea    0x198(%rsp),%rdi
   7e067:	mov    %rbp,%rsi
   7e06a:	lea    0x8(%r12),%rdx
   7e06f:	mov    %r13,%r8
   7e072:	mov    %rbp,%rcx
   7e075:	call   7d240 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7e07a:	jmp    7e0f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7e07c:	shr    $0x4,%al
   7e07f:	neg    %al
   7e081:	and    $0x1f,%al
   7e083:	movzbl %al,%eax
   7e086:	lea    0x98(%rsp),%rcx
   7e08e:	mov    %rcx,(%rsp)
   7e092:	mov    %eax,0x8(%rsp)
   7e096:	movq   $0x0,0x10(%rsp)
   7e09f:	mov    $0x4,%r9d
   7e0a5:	lea    0x198(%rsp),%rdi
   7e0ad:	mov    %rbp,%rsi
   7e0b0:	lea    0x8(%r12),%rdx
   7e0b5:	mov    %r13,%r8
   7e0b8:	mov    %rbp,%rcx
   7e0bb:	call   7d5f0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   7e0c0:	jmp    7e0f8 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x998>
   7e0c2:	lea    0x98(%rsp),%rax
   7e0ca:	mov    %rax,(%rsp)
   7e0ce:	movq   $0x0,0x8(%rsp)
   7e0d7:	mov    $0x4,%r9d
   7e0dd:	lea    0x198(%rsp),%rdi
   7e0e5:	mov    %rbp,%rsi
   7e0e8:	lea    0x8(%r12),%rdx
   7e0ed:	mov    %r13,%r8
   7e0f0:	mov    %rbp,%rcx
   7e0f3:	call   7d430 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   7e0f8:	mov    0x90(%rsp),%rax
   7e100:	cmp    %rax,0x80(%rsp)
   7e108:	je     7e3d4 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc74>
   7e10e:	mov    0x4(%r12,%rbp,1),%eax
   7e113:	mov    %eax,0x98(%rsp)
   7e11a:	mov    0x408(%r12),%r15
   7e122:	cmp    $0x401,%r15
   7e129:	jae    7e3bd <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc5d>
   7e12f:	mov    %eax,0x5c(%rsp)
   7e133:	mov    (%r12),%r13
   7e137:	mov    0x0(%r13),%rax
   7e13b:	mov    0x10(%r13),%rbp
   7e13f:	sub    %rbp,%rax
   7e142:	cmp    %rax,%r15
   7e145:	ja     7d92b <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1cb>
   7e14b:	test   %r15,%r15
   7e14e:	jne    7d948 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1e8>
   7e154:	jmp    7d95d <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x1fd>
   7e159:	cmp    $0x2,%ecx
   7e15c:	je     7e441 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e162:	cmp    $0x3,%ecx
   7e165:	je     7e42a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e16b:	test   %cl,%cl
   7e16d:	lea    -0x6f6b4(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e174:	lea    -0x6f58b(%rip),%rcx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e17b:	cmove  %rcx,%rax
   7e17f:	mov    0x48(%rsp),%r9
   7e184:	mov    %rax,(%rsp)
   7e188:	lea    0x30(%rsp),%rdi
   7e18d:	mov    0x28(%rsp),%rsi
   7e192:	mov    %r15,%rdx
   7e195:	mov    %r8,%rcx
   7e198:	lea    0x198(%rsp),%r8
   7e1a0:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7e1a5:	cmpb   $0x0,0x30(%rsp)
   7e1aa:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e1b0:	mov    %r14d,%eax
   7e1b3:	shr    $0x2,%al
   7e1b6:	and    $0x3,%al
   7e1b8:	cmp    $0x2,%al
   7e1ba:	je     7e441 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e1c0:	movzbl %al,%ecx
   7e1c3:	cmp    $0x3,%ecx
   7e1c6:	je     7e42a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e1cc:	mov    0x38(%rsp),%rcx
   7e1d1:	test   %al,%al
   7e1d3:	lea    -0x6f71a(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e1da:	lea    -0x6f5f1(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e1e1:	cmove  %rdx,%rax
   7e1e5:	cmpb   $0x0,0x27(%rsp)
   7e1ea:	lea    0x198(%rsp,%r13,1),%r8
   7e1f2:	lea    -0x6fa59(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e1f9:	cmove  %rdx,%rax
   7e1fd:	mov    %rax,(%rsp)
   7e201:	lea    0x30(%rsp),%rdi
   7e206:	mov    0x28(%rsp),%rsi
   7e20b:	mov    %r15,%rdx
   7e20e:	mov    0x48(%rsp),%r9
   7e213:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7e218:	cmpb   $0x0,0x30(%rsp)
   7e21d:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e223:	mov    %r14d,%eax
   7e226:	shr    $0x4,%al
   7e229:	and    $0x3,%al
   7e22b:	cmp    $0x2,%al
   7e22d:	je     7e441 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e233:	movzbl %al,%ecx
   7e236:	cmp    $0x3,%ecx
   7e239:	je     7e42a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e23f:	mov    0x38(%rsp),%rcx
   7e244:	test   %al,%al
   7e246:	lea    -0x6f78d(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e24d:	lea    -0x6f664(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e254:	cmove  %rdx,%rax
   7e258:	cmpb   $0x0,0x27(%rsp)
   7e25d:	lea    0x198(%rsp,%r13,2),%r8
   7e265:	lea    -0x6facc(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e26c:	cmove  %rdx,%rax
   7e270:	mov    %rax,(%rsp)
   7e274:	lea    0x30(%rsp),%rdi
   7e279:	mov    0x28(%rsp),%rsi
   7e27e:	mov    %r15,%rdx
   7e281:	mov    0x48(%rsp),%r9
   7e286:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7e28b:	cmpb   $0x0,0x30(%rsp)
   7e290:	jne    7e30a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaa>
   7e292:	shr    $0x6,%r14b
   7e296:	cmp    $0x2,%r14b
   7e29a:	je     7e441 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xce1>
   7e2a0:	movzbl %r14b,%eax
   7e2a4:	cmp    $0x3,%eax
   7e2a7:	je     7e42a <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xcca>
   7e2ad:	mov    0x38(%rsp),%rcx
   7e2b2:	test   %r14b,%r14b
   7e2b5:	lea    -0x6f7fc(%rip),%rax        # eac0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x3b0>
   7e2bc:	lea    -0x6f6d3(%rip),%rdx        # ebf0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4e0>
   7e2c3:	cmove  %rdx,%rax
   7e2c7:	cmpb   $0x0,0x27(%rsp)
   7e2cc:	mov    0x68(%rsp),%rdx
   7e2d1:	lea    0x198(%rsp,%rdx,1),%r8
   7e2d9:	lea    -0x6fb40(%rip),%rdx        # e7a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7e2e0:	cmove  %rdx,%rax
   7e2e4:	mov    %rax,(%rsp)
   7e2e8:	lea    0x30(%rsp),%rdi
   7e2ed:	mov    0x28(%rsp),%rsi
   7e2f2:	mov    %r15,%rdx
   7e2f5:	mov    0x48(%rsp),%r9
   7e2fa:	call   ea920 <meshoptimizer_rs::codec::vertex::bytes>
   7e2ff:	cmpb   $0x0,0x30(%rsp)
   7e304:	je     7dd30 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0x5d0>
   7e30a:	movzbl 0x31(%rsp),%eax
   7e30f:	add    $0x598,%rsp
   7e316:	pop    %rbx
   7e317:	pop    %r12
   7e319:	pop    %r13
   7e31b:	pop    %r14
   7e31d:	pop    %r15
   7e31f:	pop    %rbp
   7e320:	ret
   7e321:	lea    0xc95b8(%rip),%rdi        # 1478e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x34b0>
   7e328:	call   *0xce47a(%rip)        # 14c7a8 <_DYNAMIC+0x3c8>
   7e32e:	mov    $0x7,%al
   7e330:	jmp    7e30f <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xbaf>
   7e332:	lea    0x40(%rax),%rsi
   7e336:	lea    0xc89d3(%rip),%rcx        # 146d10 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28e0>
   7e33d:	mov    %rax,%rdi
   7e340:	mov    %rbp,%rdx
   7e343:	call   *0xce407(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e349:	mov    0x60(%rsp),%rcx
   7e34e:	cmp    $0x100,%rcx
   7e355:	mov    $0x100,%eax
   7e35a:	cmovb  %rcx,%rax
   7e35e:	jmp    7e3a7 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>+0xc47>
   7e360:	mov    0x60(%rsp),%rcx
   7e365:	cmp    $0x100,%rcx
   7e36c:	mov    $0x100,%eax
   7e371:	cmovb  %rcx,%rax
   7e375:	mov    %eax,%eax
   7e377:	lea    (%rdi,%rax,2),%rdi
   7e37b:	lea    0xc9546(%rip),%rcx        # 1478c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3498>
   7e382:	mov    %rbp,%rsi
   7e385:	mov    %rbp,%rdx
   7e388:	call   *0xce3c2(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e38e:	mov    0x60(%rsp),%rcx
   7e393:	cmp    $0x100,%rcx
   7e39a:	mov    $0x100,%eax
   7e39f:	cmovb  %rcx,%rax
   7e3a3:	lea    (%rax,%rax,2),%rax
   7e3a7:	add    %rax,%rdi
   7e3aa:	lea    0xc9517(%rip),%rcx        # 1478c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3498>
   7e3b1:	mov    %rbp,%rsi
   7e3b4:	mov    %rbp,%rdx
   7e3b7:	call   *0xce393(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e3bd:	lea    0xcc674(%rip),%rcx        # 14aa38 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x6608>
   7e3c4:	mov    $0x400,%edx
   7e3c9:	xor    %edi,%edi
   7e3cb:	mov    %r15,%rsi
   7e3ce:	call   *0xce37c(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e3d4:	lea    -0x4(%rbp),%rdi
   7e3d8:	lea    0xc84f9(%rip),%rcx        # 1468d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24a8>
   7e3df:	mov    %rbp,%rsi
   7e3e2:	mov    %rbp,%rdx
   7e3e5:	call   *0xce365(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e3eb:	lea    0xc85be(%rip),%rcx        # 1469b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2580>
   7e3f2:	mov    0x70(%rsp),%rdi
   7e3f7:	mov    %r15,%rsi
   7e3fa:	mov    %r15,%rdx
   7e3fd:	call   *0xce34d(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e403:	lea    0xc88ee(%rip),%rcx        # 146cf8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x28c8>
   7e40a:	mov    %rdx,%rdi
   7e40d:	mov    %rbp,%rsi
   7e410:	mov    %rbp,%rdx
   7e413:	call   *0xce337(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e419:	lea    0xc84e8(%rip),%rdx        # 146908 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x24d8>
   7e420:	xor    %edi,%edi
   7e422:	xor    %esi,%esi
   7e424:	call   *0xce206(%rip)        # 14c630 <_DYNAMIC+0x250>
   7e42a:	lea    0xc851f(%rip),%rcx        # 146950 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2520>
   7e431:	xor    %edi,%edi
   7e433:	mov    %r13,%rsi
   7e436:	mov    0x48(%rsp),%rdx
   7e43b:	call   *0xce30f(%rip)        # 14c750 <_DYNAMIC+0x370>
   7e441:	lea    0xc8520(%rip),%rcx        # 146968 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2538>
   7e448:	xor    %edi,%edi
   7e44a:	mov    %r13,%rsi
   7e44d:	mov    0x48(%rsp),%rdx
   7e452:	call   *0xce2f8(%rip)        # 14c750 <_DYNAMIC+0x370>
