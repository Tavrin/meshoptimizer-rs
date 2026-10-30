
<artifacts>/screen/driver-control:     file format elf64-x86-64


Disassembly of section .text:

0000000000072e50 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>:
   72e50:	push   %rbp
   72e51:	push   %r15
   72e53:	push   %r14
   72e55:	push   %r13
   72e57:	push   %r12
   72e59:	push   %rbx
   72e5a:	sub    $0x5f8,%rsp
   72e61:	mov    %r8,%r15
   72e64:	mov    %rcx,%r12
   72e67:	mov    %rdx,%r14
   72e6a:	mov    %rsi,%r13
   72e6d:	mov    %rdi,%rbp
   72e70:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   72e75:	cmp    $0x2,%al
   72e77:	jb     72edd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8d>
   72e79:	cmp    $0x20,%r14
   72e7d:	lea    0x5b3c(%rip),%rax        # 789c0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, alloc::vec::Vec<u8>>>
   72e84:	lea    0x45c5(%rip),%rcx        # 77450 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, alloc::vec::Vec<u8>>>
   72e8b:	cmove  %rax,%rcx
   72e8f:	lea    0xb3aa(%rip),%rax        # 7e240 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, alloc::vec::Vec<u8>>>
   72e96:	lea    0x9a33(%rip),%rdx        # 7c8d0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, alloc::vec::Vec<u8>>>
   72e9d:	cmove  %rax,%rdx
   72ea1:	cmp    $0x4,%r14
   72ea5:	lea    0x6f44(%rip),%rax        # 79df0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, alloc::vec::Vec<u8>>>
   72eac:	cmovne %rcx,%rax
   72eb0:	lea    0xcb69(%rip),%r9        # 7fa20 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, alloc::vec::Vec<u8>>>
   72eb7:	cmovne %rdx,%r9
   72ebb:	cmp    $0x21,%r13
   72ebf:	cmovae %rax,%r9
   72ec3:	mov    %rbp,%rdi
   72ec6:	mov    %r13,%rsi
   72ec9:	mov    %r14,%rdx
   72ecc:	mov    %r12,%rcx
   72ecf:	mov    %r15,%r8
   72ed2:	call   *%r9
   72ed5:	cmp    $0xfe,%al
   72ed7:	jne    739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72edd:	mov    $0x7,%al
   72edf:	test   %r15,%r15
   72ee2:	je     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72ee8:	movzbl (%r12),%edx
   72eed:	mov    %edx,%ecx
   72eef:	and    $0xf0,%cl
   72ef2:	cmp    $0xa0,%cl
   72ef5:	jne    739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72efb:	and    $0xf,%dl
   72efe:	mov    $0x5,%al
   72f00:	mov    %dl,0x27(%rsp)
   72f04:	cmp    $0x1,%dl
   72f07:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72f0d:	cmpb   $0x0,0x27(%rsp)
   72f12:	je     72f3b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xeb>
   72f14:	mov    %r14,%rcx
   72f17:	shr    $0x2,%rcx
   72f1b:	add    %r14,%rcx
   72f1e:	cmp    $0x19,%rcx
   72f22:	mov    $0x18,%ebx
   72f27:	cmovae %rcx,%rbx
   72f2b:	lea    0x1(%rbx),%rdx
   72f2f:	mov    $0x7,%al
   72f31:	cmp    %rdx,%r15
   72f34:	jae    72f5a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x10a>
   72f36:	jmp    739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72f3b:	cmp    $0x21,%r14
   72f3f:	mov    $0x20,%ebx
   72f44:	cmovae %r14,%rbx
   72f48:	mov    %r14,%rcx
   72f4b:	lea    0x1(%rbx),%rdx
   72f4f:	mov    $0x7,%al
   72f51:	cmp    %rdx,%r15
   72f54:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   72f5a:	mov    %r15,%rsi
   72f5d:	sub    %rcx,%rsi
   72f60:	lea    (%rsi,%r14,1),%rax
   72f64:	mov    %r15,%rcx
   72f67:	mov    %rax,0x70(%rsp)
   72f6c:	sub    %rax,%rcx
   72f6f:	mov    %rcx,0xa8(%rsp)
   72f77:	jb     73a54 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc04>
   72f7d:	xorps  %xmm0,%xmm0
   72f80:	movaps %xmm0,0x1e0(%rsp)
   72f88:	movaps %xmm0,0x1d0(%rsp)
   72f90:	movaps %xmm0,0x1c0(%rsp)
   72f98:	movaps %xmm0,0x1b0(%rsp)
   72fa0:	movaps %xmm0,0x1a0(%rsp)
   72fa8:	movaps %xmm0,0x190(%rsp)
   72fb0:	movaps %xmm0,0x180(%rsp)
   72fb8:	movaps %xmm0,0x170(%rsp)
   72fc0:	movaps %xmm0,0x160(%rsp)
   72fc8:	movaps %xmm0,0x150(%rsp)
   72fd0:	movaps %xmm0,0x140(%rsp)
   72fd8:	movaps %xmm0,0x130(%rsp)
   72fe0:	movaps %xmm0,0x120(%rsp)
   72fe8:	movaps %xmm0,0x110(%rsp)
   72ff0:	movaps %xmm0,0x100(%rsp)
   72ff8:	movaps %xmm0,0xf0(%rsp)
   73000:	cmp    $0x101,%r14
   73007:	jae    73a10 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbc0>
   7300d:	lea    (%rsi,%r14,1),%rax
   73011:	cmp    %rsi,%rax
   73014:	jb     73a27 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbd7>
   7301a:	add    %r12,%rsi
   7301d:	lea    0xf0(%rsp),%rdi
   73025:	mov    %r14,%rdx
   73028:	call   *0xb3d4a(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   7302e:	test   %r14,%r14
   73031:	je     73aac <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc5c>
   73037:	mov    %rbx,0xd8(%rsp)
   7303f:	mov    $0x2000,%ax
   73043:	xor    %edx,%edx
   73045:	div    %r14w
   73049:	and    $0x3ff0,%eax
   7304e:	cmp    $0x100,%eax
   73053:	mov    $0x100,%ecx
   73058:	cmovb  %eax,%ecx
   7305b:	mov    %rcx,0xe0(%rsp)
   73063:	lea    0x1f8(%rsp),%rdi
   7306b:	mov    $0x400,%edx
   73070:	xor    %esi,%esi
   73072:	call   *0xb3d10(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73078:	test   %r13,%r13
   7307b:	je     739ad <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb5d>
   73081:	add    %r12,0x70(%rsp)
   73086:	mov    %r14,%rax
   73089:	shr    $0x2,%rax
   7308d:	mov    %rax,0xa0(%rsp)
   73095:	lea    0x3(%r14),%rax
   73099:	shr    $0x2,%rax
   7309d:	mov    %rax,0xe8(%rsp)
   730a5:	mov    $0x1,%ebx
   730aa:	xor    %eax,%eax
   730ac:	mov    %rbp,0x68(%rsp)
   730b1:	mov    %r15,0x30(%rsp)
   730b6:	mov    %r12,0x38(%rsp)
   730bb:	mov    %r13,0xc0(%rsp)
   730c3:	mov    %r13,%rdx
   730c6:	mov    %rax,0x58(%rsp)
   730cb:	sub    %rax,%rdx
   730ce:	mov    0xe0(%rsp),%rax
   730d6:	cmp    %rax,%rdx
   730d9:	cmovae %rax,%rdx
   730dd:	mov    %rdx,0x28(%rsp)
   730e2:	cmpb   $0x0,0x27(%rsp)
   730e7:	je     7312c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2dc>
   730e9:	mov    %rbx,%rcx
   730ec:	mov    $0x7,%al
   730ee:	add    0xa0(%rsp),%rcx
   730f6:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   730fc:	cmp    %r15,%rcx
   730ff:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73105:	add    0x38(%rsp),%rbx
   7310a:	mov    0xa0(%rsp),%rax
   73112:	mov    %rax,0x98(%rsp)
   7311a:	mov    %rbx,0xb0(%rsp)
   73122:	mov    %rcx,%rbx
   73125:	mov    0x68(%rsp),%rbp
   7312a:	jmp    73145 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x2f5>
   7312c:	movq   $0x0,0x98(%rsp)
   73138:	mov    $0x1,%eax
   7313d:	mov    %rax,0xb0(%rsp)
   73145:	mov    0x58(%rsp),%rax
   7314a:	mov    0x28(%rsp),%rcx
   7314f:	lea    (%rcx,%rax,1),%r15
   73153:	mov    %r15,0xd0(%rsp)
   7315b:	imul   %r14,%r15
   7315f:	mov    0x10(%rbp),%r13
   73163:	mov    %rbp,%rax
   73166:	mov    %r15,%rbp
   73169:	mov    %r15,%rdx
   7316c:	sub    %r13,%rbp
   7316f:	jbe    731b9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x369>
   73171:	mov    (%rax),%rax
   73174:	sub    %r13,%rax
   73177:	cmp    %rax,%rbp
   7317a:	ja     73985 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb35>
   73180:	mov    0x68(%rsp),%rax
   73185:	mov    0x8(%rax),%r12
   73189:	lea    (%r12,%r13,1),%rdi
   7318d:	cmp    $0x2,%rbp
   73191:	jb     731ab <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x35b>
   73193:	lea    -0x1(%rbp),%rdx
   73197:	xor    %esi,%esi
   73199:	call   *0xb3be9(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   7319f:	add    %rbp,%r13
   731a2:	dec    %r13
   731a5:	add    %r13,%r12
   731a8:	mov    %r12,%rdi
   731ab:	movb   $0x0,(%rdi)
   731ae:	inc    %r13
   731b1:	mov    %r13,%rdx
   731b4:	mov    0x68(%rsp),%rax
   731b9:	mov    0x58(%rsp),%rcx
   731be:	imul   %r14,%rcx
   731c2:	mov    %rdx,0x10(%rax)
   731c6:	mov    %rcx,0x58(%rsp)
   731cb:	cmp    %rcx,%r15
   731ce:	jb     73a3f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   731d4:	cmp    %rdx,%r15
   731d7:	ja     73a3f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbef>
   731dd:	mov    0x28(%rsp),%rdx
   731e2:	lea    0xf(%rdx),%ecx
   731e5:	and    $0x3f0,%ecx
   731eb:	mov    %rcx,0x50(%rsp)
   731f0:	mov    %rdx,%rcx
   731f3:	imul   %r14,%rcx
   731f7:	mov    %rcx,0xb8(%rsp)
   731ff:	mov    0x58(%rsp),%rcx
   73204:	add    0x8(%rax),%rcx
   73208:	mov    %rcx,0x58(%rsp)
   7320d:	lea    -0x1(%rdx),%rax
   73211:	imul   %r14,%rax
   73215:	mov    %rax,0xc8(%rsp)
   7321d:	add    $0x4,%rax
   73221:	mov    %rax,0x60(%rsp)
   73226:	lea    0x0(,%rdx,4),%rax
   7322e:	mov    %rax,0x78(%rsp)
   73233:	lea    0x1f8(%rsp,%rdx,1),%rax
   7323b:	mov    %rax,0x90(%rsp)
   73243:	lea    0x1f8(%rsp,%rdx,2),%rax
   7324b:	mov    %rax,0x88(%rsp)
   73253:	lea    (%rdx,%rdx,2),%rax
   73257:	add    %rsp,%rax
   7325a:	add    $0x1f8,%rax
   73260:	mov    %rax,0x80(%rsp)
   73268:	xor    %r13d,%r13d
   7326b:	xor    %ebp,%ebp
   7326d:	movzbl 0x27(%rsp),%r15d
   73273:	jmp    732cf <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x47f>
   73275:	lea    (%rsp,%rbp,1),%rcx
   73279:	add    $0xf0,%rcx
   73280:	shr    $0x4,%al
   73283:	neg    %al
   73285:	and    $0x1f,%al
   73287:	movzbl %al,%eax
   7328a:	mov    %rcx,(%rsp)
   7328e:	mov    %eax,0x8(%rsp)
   73292:	movq   $0x0,0x10(%rsp)
   7329b:	lea    0x1f8(%rsp),%rdi
   732a3:	mov    0x78(%rsp),%rsi
   732a8:	mov    0x60(%rsp),%rcx
   732ad:	mov    0x28(%rsp),%r8
   732b2:	mov    %r14,%r9
   732b5:	call   72ce0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   732ba:	inc    %r13
   732bd:	add    $0x4,%rbp
   732c1:	cmp    %r13,0xe8(%rsp)
   732c9:	je     7392b <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xadb>
   732cf:	test   %r15b,%r15b
   732d2:	je     732f1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a1>
   732d4:	cmp    0x98(%rsp),%r13
   732dc:	jae    73a7c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc2c>
   732e2:	mov    0xb0(%rsp),%rax
   732ea:	movzbl (%rax,%r13,1),%r12d
   732ef:	jmp    732f4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4a4>
   732f1:	xor    %r12d,%r12d
   732f4:	mov    %r12d,%eax
   732f7:	and    $0x3,%al
   732f9:	movzbl %al,%ecx
   732fc:	mov    0x50(%rsp),%rdx
   73301:	cmp    %rdx,0x28(%rsp)
   73306:	ja     73778 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x928>
   7330c:	cmp    $0x2,%al
   7330e:	je     7334d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x4fd>
   73310:	cmp    $0x3,%ecx
   73313:	jne    73367 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x517>
   73315:	mov    %rbx,%r15
   73318:	mov    $0x7,%al
   7331a:	add    0x28(%rsp),%r15
   7331f:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73325:	cmp    0x30(%rsp),%r15
   7332a:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   73330:	add    0x38(%rsp),%rbx
   73335:	lea    0x1f8(%rsp),%rdi
   7333d:	mov    %rbx,%rsi
   73340:	mov    0x28(%rsp),%rdx
   73345:	call   *0xb3a2d(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   7334b:	jmp    733c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   7334d:	lea    0x1f8(%rsp),%rdi
   73355:	xor    %esi,%esi
   73357:	mov    0x28(%rsp),%rdx
   7335c:	call   *0xb3a26(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   73362:	mov    %rbx,%r15
   73365:	jmp    733c1 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x571>
   73367:	test   %al,%al
   73369:	lea    -0x65740(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73370:	lea    -0x65627(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73377:	cmove  %rcx,%rax
   7337b:	test   %r15b,%r15b
   7337e:	lea    -0x65a55(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73385:	cmove  %rcx,%rax
   73389:	mov    %rax,(%rsp)
   7338d:	lea    0x40(%rsp),%rdi
   73392:	mov    0x38(%rsp),%rsi
   73397:	mov    0x30(%rsp),%rdx
   7339c:	mov    %rbx,%rcx
   7339f:	lea    0x1f8(%rsp),%r8
   733a7:	mov    0x50(%rsp),%r9
   733ac:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   733b1:	cmpb   $0x0,0x40(%rsp)
   733b6:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   733bc:	mov    0x48(%rsp),%r15
   733c1:	mov    %r12d,%eax
   733c4:	shr    $0x2,%al
   733c7:	and    $0x3,%al
   733c9:	cmp    $0x3,%al
   733cb:	je     733ef <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x59f>
   733cd:	movzbl %al,%ecx
   733d0:	cmp    $0x2,%ecx
   733d3:	jne    7342a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x5da>
   733d5:	mov    0x90(%rsp),%rdi
   733dd:	xor    %esi,%esi
   733df:	mov    0x28(%rsp),%rdx
   733e4:	call   *0xb399e(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   733ea:	jmp    73486 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   733ef:	mov    %r15,%rbx
   733f2:	mov    $0x7,%al
   733f4:	add    0x28(%rsp),%rbx
   733f9:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   733ff:	cmp    0x30(%rsp),%rbx
   73404:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   7340a:	add    0x38(%rsp),%r15
   7340f:	mov    0x90(%rsp),%rdi
   73417:	mov    %r15,%rsi
   7341a:	mov    0x28(%rsp),%rdx
   7341f:	call   *0xb3953(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   73425:	mov    %rbx,%r15
   73428:	jmp    73486 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x636>
   7342a:	test   %al,%al
   7342c:	lea    -0x65803(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73433:	lea    -0x656ea(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7343a:	cmove  %rcx,%rax
   7343e:	cmpb   $0x0,0x27(%rsp)
   73443:	lea    -0x65b1a(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7344a:	cmove  %rcx,%rax
   7344e:	mov    %rax,(%rsp)
   73452:	lea    0x40(%rsp),%rdi
   73457:	mov    0x38(%rsp),%rsi
   7345c:	mov    0x30(%rsp),%rdx
   73461:	mov    %r15,%rcx
   73464:	mov    0x90(%rsp),%r8
   7346c:	mov    0x50(%rsp),%r9
   73471:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73476:	cmpb   $0x0,0x40(%rsp)
   7347b:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73481:	mov    0x48(%rsp),%r15
   73486:	mov    %r12d,%eax
   73489:	shr    $0x4,%al
   7348c:	and    $0x3,%al
   7348e:	cmp    $0x3,%al
   73490:	je     734c2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x672>
   73492:	movzbl %al,%ecx
   73495:	cmp    $0x2,%ecx
   73498:	jne    7354a <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6fa>
   7349e:	mov    0x88(%rsp),%rdi
   734a6:	xor    %esi,%esi
   734a8:	mov    0x28(%rsp),%rdx
   734ad:	call   *0xb38d5(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   734b3:	shr    $0x6,%r12b
   734b7:	cmp    $0x3,%r12b
   734bb:	jne    73509 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   734bd:	jmp    735b4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   734c2:	mov    %r15,%rbx
   734c5:	mov    $0x7,%al
   734c7:	add    0x28(%rsp),%rbx
   734cc:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   734d2:	cmp    0x30(%rsp),%rbx
   734d7:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   734dd:	add    0x38(%rsp),%r15
   734e2:	mov    0x88(%rsp),%rdi
   734ea:	mov    %r15,%rsi
   734ed:	mov    0x28(%rsp),%rdx
   734f2:	call   *0xb3880(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   734f8:	mov    %rbx,%r15
   734fb:	shr    $0x6,%r12b
   734ff:	cmp    $0x3,%r12b
   73503:	je     735b4 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x764>
   73509:	movzbl %r12b,%eax
   7350d:	cmp    $0x2,%eax
   73510:	jne    73604 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7b4>
   73516:	mov    0x80(%rsp),%rdi
   7351e:	xor    %esi,%esi
   73520:	mov    0x28(%rsp),%rdx
   73525:	call   *0xb385d(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   7352b:	mov    %r15,%rbx
   7352e:	movzbl 0x27(%rsp),%r15d
   73534:	test   %r15b,%r15b
   73537:	mov    0xb8(%rsp),%rcx
   7353f:	jne    73674 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   73545:	jmp    735fd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   7354a:	test   %al,%al
   7354c:	lea    -0x65923(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73553:	lea    -0x6580a(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7355a:	cmove  %rcx,%rax
   7355e:	cmpb   $0x0,0x27(%rsp)
   73563:	lea    -0x65c3a(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7356a:	cmove  %rcx,%rax
   7356e:	mov    %rax,(%rsp)
   73572:	lea    0x40(%rsp),%rdi
   73577:	mov    0x38(%rsp),%rsi
   7357c:	mov    0x30(%rsp),%rdx
   73581:	mov    %r15,%rcx
   73584:	mov    0x88(%rsp),%r8
   7358c:	mov    0x50(%rsp),%r9
   73591:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73596:	cmpb   $0x0,0x40(%rsp)
   7359b:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   735a1:	mov    0x48(%rsp),%r15
   735a6:	shr    $0x6,%r12b
   735aa:	cmp    $0x3,%r12b
   735ae:	jne    73509 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x6b9>
   735b4:	mov    %r15,%rbx
   735b7:	mov    $0x7,%al
   735b9:	add    0x28(%rsp),%rbx
   735be:	jb     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   735c4:	cmp    0x30(%rsp),%rbx
   735c9:	ja     739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   735cf:	add    0x38(%rsp),%r15
   735d4:	mov    0x80(%rsp),%rdi
   735dc:	mov    %r15,%rsi
   735df:	mov    0x28(%rsp),%rdx
   735e4:	call   *0xb378e(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   735ea:	movzbl 0x27(%rsp),%r15d
   735f0:	test   %r15b,%r15b
   735f3:	mov    0xb8(%rsp),%rcx
   735fb:	jne    73674 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x824>
   735fd:	xor    %eax,%eax
   735ff:	jmp    7368c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x83c>
   73604:	test   %r12b,%r12b
   73607:	lea    -0x659de(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7360e:	lea    -0x658c5(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73615:	cmove  %rcx,%rax
   73619:	cmpb   $0x0,0x27(%rsp)
   7361e:	lea    -0x65cf5(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73625:	cmove  %rcx,%rax
   73629:	mov    %rax,(%rsp)
   7362d:	lea    0x40(%rsp),%rdi
   73632:	mov    0x38(%rsp),%rsi
   73637:	mov    0x30(%rsp),%rdx
   7363c:	mov    %r15,%rcx
   7363f:	mov    0x80(%rsp),%r8
   73647:	mov    0x50(%rsp),%r9
   7364c:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   73651:	cmpb   $0x0,0x40(%rsp)
   73656:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   7365c:	mov    0x48(%rsp),%rbx
   73661:	movzbl 0x27(%rsp),%r15d
   73667:	test   %r15b,%r15b
   7366a:	mov    0xb8(%rsp),%rcx
   73672:	je     735fd <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ad>
   73674:	cmp    0xa8(%rsp),%r13
   7367c:	jae    73a94 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc44>
   73682:	mov    0x70(%rsp),%rax
   73687:	movzbl (%rax,%r13,1),%eax
   7368c:	mov    %rcx,%rdx
   7368f:	sub    %rbp,%rdx
   73692:	jb     739fa <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xbaa>
   73698:	cmp    %rdx,0x60(%rsp)
   7369d:	ja     739e6 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb96>
   736a3:	mov    0x58(%rsp),%rcx
   736a8:	lea    (%rcx,%rbp,1),%rdx
   736ac:	mov    %eax,%ecx
   736ae:	and    $0x3,%cl
   736b1:	movzbl %cl,%ecx
   736b4:	lea    -0x62f13(%rip),%rsi        # 107a8 <libm::math::generic::sqrt::RSQRT_TAB+0x3c2>
   736bb:	movslq (%rsi,%rcx,4),%rcx
   736bf:	add    %rsi,%rcx
   736c2:	jmp    *%rcx
   736c4:	mov    %rdx,%r15
   736c7:	lea    (%rsp,%rbp,1),%r12
   736cb:	add    $0xf0,%r12
   736d2:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   736d7:	test   %al,%al
   736d9:	je     7373e <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x8ee>
   736db:	mov    %r12,(%rsp)
   736df:	lea    0x1f8(%rsp),%rdi
   736e7:	mov    0x78(%rsp),%rsi
   736ec:	mov    %r15,%rdx
   736ef:	mov    0x60(%rsp),%rcx
   736f4:	mov    0x28(%rsp),%r8
   736f9:	mov    %r14,%r9
   736fc:	call   75a40 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   73701:	jmp    7376d <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x91d>
   73703:	lea    (%rsp,%rbp,1),%rax
   73707:	add    $0xf0,%rax
   7370d:	mov    %rax,(%rsp)
   73711:	movq   $0x0,0x8(%rsp)
   7371a:	lea    0x1f8(%rsp),%rdi
   73722:	mov    0x78(%rsp),%rsi
   73727:	mov    0x60(%rsp),%rcx
   7372c:	mov    0x28(%rsp),%r8
   73731:	mov    %r14,%r9
   73734:	call   72b20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   73739:	jmp    732ba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   7373e:	mov    %r12,(%rsp)
   73742:	movq   $0x0,0x8(%rsp)
   7374b:	lea    0x1f8(%rsp),%rdi
   73753:	mov    0x78(%rsp),%rsi
   73758:	mov    %r15,%rdx
   7375b:	mov    0x60(%rsp),%rcx
   73760:	mov    0x28(%rsp),%r8
   73765:	mov    %r14,%r9
   73768:	call   72930 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   7376d:	movzbl 0x27(%rsp),%r15d
   73773:	jmp    732ba <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x46a>
   73778:	cmp    $0x2,%al
   7377a:	je     73ad2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   73780:	cmp    $0x3,%ecx
   73783:	je     73ab9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73789:	test   %al,%al
   7378b:	lea    -0x65b62(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73792:	lea    -0x65a49(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73799:	cmove  %rcx,%rax
   7379d:	test   %r15b,%r15b
   737a0:	lea    -0x65e77(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   737a7:	cmove  %rcx,%rax
   737ab:	mov    %rax,(%rsp)
   737af:	lea    0x40(%rsp),%rdi
   737b4:	mov    0x38(%rsp),%rsi
   737b9:	mov    0x30(%rsp),%rdx
   737be:	mov    %rbx,%rcx
   737c1:	lea    0x1f8(%rsp),%r8
   737c9:	mov    0x50(%rsp),%r9
   737ce:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   737d3:	cmpb   $0x0,0x40(%rsp)
   737d8:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   737de:	mov    %r12d,%eax
   737e1:	shr    $0x2,%al
   737e4:	and    $0x3,%al
   737e6:	cmp    $0x2,%al
   737e8:	movzbl 0x27(%rsp),%edx
   737ed:	je     73ad2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   737f3:	movzbl %al,%ecx
   737f6:	cmp    $0x3,%ecx
   737f9:	je     73ab9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   737ff:	test   %al,%al
   73801:	lea    -0x65bd8(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   73808:	lea    -0x65abf(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   7380f:	cmove  %rcx,%rax
   73813:	mov    0x48(%rsp),%rcx
   73818:	test   %dl,%dl
   7381a:	lea    -0x65ef1(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73821:	cmove  %rdx,%rax
   73825:	mov    %rax,(%rsp)
   73829:	lea    0x40(%rsp),%rdi
   7382e:	mov    0x38(%rsp),%rsi
   73833:	mov    0x30(%rsp),%rdx
   73838:	mov    0x90(%rsp),%r8
   73840:	mov    0x50(%rsp),%r9
   73845:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   7384a:	cmpb   $0x0,0x40(%rsp)
   7384f:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   73855:	mov    %r12d,%eax
   73858:	shr    $0x4,%al
   7385b:	and    $0x3,%al
   7385d:	cmp    $0x2,%al
   7385f:	movzbl 0x27(%rsp),%edx
   73864:	je     73ad2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   7386a:	movzbl %al,%ecx
   7386d:	cmp    $0x3,%ecx
   73870:	je     73ab9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   73876:	test   %al,%al
   73878:	lea    -0x65c4f(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   7387f:	lea    -0x65b36(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   73886:	cmove  %rcx,%rax
   7388a:	mov    0x48(%rsp),%rcx
   7388f:	test   %dl,%dl
   73891:	lea    -0x65f68(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   73898:	cmove  %rdx,%rax
   7389c:	mov    %rax,(%rsp)
   738a0:	lea    0x40(%rsp),%rdi
   738a5:	mov    0x38(%rsp),%rsi
   738aa:	mov    0x30(%rsp),%rdx
   738af:	mov    0x88(%rsp),%r8
   738b7:	mov    0x50(%rsp),%r9
   738bc:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   738c1:	cmpb   $0x0,0x40(%rsp)
   738c6:	jne    739cb <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb7b>
   738cc:	shr    $0x6,%r12b
   738d0:	cmp    $0x2,%r12b
   738d4:	movzbl 0x27(%rsp),%edx
   738d9:	je     73ad2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc82>
   738df:	movzbl %r12b,%eax
   738e3:	cmp    $0x3,%eax
   738e6:	je     73ab9 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc69>
   738ec:	test   %r12b,%r12b
   738ef:	lea    -0x65cc6(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   738f6:	lea    -0x65bad(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   738fd:	cmove  %rcx,%rax
   73901:	mov    0x48(%rsp),%rcx
   73906:	test   %dl,%dl
   73908:	lea    -0x65fdf(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   7390f:	cmove  %rdx,%rax
   73913:	mov    %rax,(%rsp)
   73917:	lea    0x40(%rsp),%rdi
   7391c:	mov    0x38(%rsp),%rsi
   73921:	mov    0x30(%rsp),%rdx
   73926:	jmp    7363f <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x7ef>
   7392b:	mov    0xb8(%rsp),%rdx
   73933:	mov    0xc8(%rsp),%rdi
   7393b:	cmp    %rdx,%rdi
   7393e:	ja     73a6c <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xc1c>
   73944:	mov    0x58(%rsp),%rsi
   73949:	add    %rdi,%rsi
   7394c:	lea    0xf0(%rsp),%rdi
   73954:	mov    %r14,%rdx
   73957:	call   *0xb341b(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   7395d:	mov    0xd0(%rsp),%rcx
   73965:	mov    %rcx,%rax
   73968:	mov    0xc0(%rsp),%r13
   73970:	cmp    %r13,%rcx
   73973:	mov    0x30(%rsp),%r15
   73978:	mov    0x68(%rsp),%rbp
   7397d:	jb     730c3 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x273>
   73983:	jmp    739b2 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb62>
   73985:	mov    $0x1,%ecx
   7398a:	mov    $0x1,%r8d
   73990:	mov    0x68(%rsp),%r12
   73995:	mov    %r12,%rdi
   73998:	mov    %r13,%rsi
   7399b:	mov    %rbp,%rdx
   7399e:	call   82820 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   739a3:	mov    0x10(%r12),%r13
   739a8:	jmp    73180 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0x330>
   739ad:	mov    $0x1,%ebx
   739b2:	sub    %rbx,%r15
   739b5:	setae  %cl
   739b8:	cmp    0xd8(%rsp),%r15
   739c0:	sete   %al
   739c3:	and    %cl,%al
   739c5:	neg    %al
   739c7:	or     $0x7,%al
   739c9:	jmp    739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   739cb:	movzbl 0x41(%rsp),%eax
   739d0:	add    $0x5f8,%rsp
   739d7:	pop    %rbx
   739d8:	pop    %r12
   739da:	pop    %r13
   739dc:	pop    %r14
   739de:	pop    %r15
   739e0:	pop    %rbp
   739e1:	ret
   739e2:	mov    $0x7,%al
   739e4:	jmp    739d0 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>+0xb80>
   739e6:	lea    0xadfeb(%rip),%rcx        # 1219d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   739ed:	xor    %edi,%edi
   739ef:	mov    0x60(%rsp),%rsi
   739f4:	call   *0xb34d6(%rip)        # 126ed0 <_DYNAMIC+0x370>
   739fa:	mov    %rcx,%rdx
   739fd:	lea    0xadfec(%rip),%rcx        # 1219f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   73a04:	mov    %rbp,%rdi
   73a07:	mov    %rdx,%rsi
   73a0a:	call   *0xb34c0(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a10:	lea    0xae039(%rip),%rcx        # 121a50 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   73a17:	mov    $0x100,%edx
   73a1c:	xor    %edi,%edi
   73a1e:	mov    %r14,%rsi
   73a21:	call   *0xb34a9(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a27:	lea    0xae00a(%rip),%rcx        # 121a38 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   73a2e:	mov    %rsi,%rdi
   73a31:	mov    0x70(%rsp),%rsi
   73a36:	mov    %r15,%rdx
   73a39:	call   *0xb3491(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a3f:	lea    0xb18da(%rip),%rcx        # 125320 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5b18>
   73a46:	mov    0x58(%rsp),%rdi
   73a4b:	mov    %r15,%rsi
   73a4e:	call   *0xb347c(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a54:	lea    0xae00d(%rip),%rcx        # 121a68 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   73a5b:	mov    0x70(%rsp),%rdi
   73a60:	mov    %r15,%rsi
   73a63:	mov    %r15,%rdx
   73a66:	call   *0xb3464(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a6c:	lea    0xadf1d(%rip),%rcx        # 121990 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   73a73:	mov    %rdx,%rsi
   73a76:	call   *0xb3454(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73a7c:	lea    0xadf25(%rip),%rdx        # 1219a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   73a83:	mov    %r13,%rdi
   73a86:	mov    0x98(%rsp),%rsi
   73a8e:	call   *0xb331c(%rip)        # 126db0 <_DYNAMIC+0x250>
   73a94:	lea    0xadf25(%rip),%rdx        # 1219c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   73a9b:	mov    %r13,%rdi
   73a9e:	mov    0xa8(%rsp),%rsi
   73aa6:	call   *0xb3304(%rip)        # 126db0 <_DYNAMIC+0x250>
   73aac:	lea    0xadec5(%rip),%rdi        # 121978 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   73ab3:	call   *0xb34df(%rip)        # 126f98 <_DYNAMIC+0x438>
   73ab9:	lea    0xadf48(%rip),%rcx        # 121a08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   73ac0:	xor    %edi,%edi
   73ac2:	mov    0x28(%rsp),%rsi
   73ac7:	mov    0x50(%rsp),%rdx
   73acc:	call   *0xb33fe(%rip)        # 126ed0 <_DYNAMIC+0x370>
   73ad2:	lea    0xadf47(%rip),%rcx        # 121a20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   73ad9:	xor    %edi,%edi
   73adb:	mov    0x28(%rsp),%rsi
   73ae0:	mov    0x50(%rsp),%rdx
   73ae5:	call   *0xb33e5(%rip)        # 126ed0 <_DYNAMIC+0x370>
