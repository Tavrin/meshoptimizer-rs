
<artifacts>/screen/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000abce0 <meshoptimizer_rs::codec::raw>:
   abce0:	push   %rbp
   abce1:	push   %r15
   abce3:	push   %r14
   abce5:	push   %r13
   abce7:	push   %r12
   abce9:	push   %rbx
   abcea:	sub    $0x5f8,%rsp
   abcf1:	mov    %r9,%r14
   abcf4:	mov    %r8,%rbp
   abcf7:	mov    %rcx,%r13
   abcfa:	mov    %rdx,%r12
   abcfd:	mov    %rsi,%r15
   abd00:	mov    0x630(%rsp),%rbx
   abd08:	test   %dil,%dil
   abd0b:	je     abe74 <meshoptimizer_rs::codec::raw+0x194>
   abd11:	movzbl %dil,%eax
   abd15:	cmp    $0x1,%eax
   abd18:	jne    abf36 <meshoptimizer_rs::codec::raw+0x256>
   abd1e:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   abd28:	mov    %r13,%rax
   abd2b:	mul    %rdi
   abd2e:	shr    $1,%rdx
   abd31:	lea    (%rdx,%rdx,2),%rcx
   abd35:	mov    %r13,%rax
   abd38:	sub    %rcx,%rax
   abd3b:	cmp    $0x2,%rbp
   abd3f:	jne    abfa1 <meshoptimizer_rs::codec::raw+0x2c1>
   abd45:	test   %rax,%rax
   abd48:	jne    abfaa <meshoptimizer_rs::codec::raw+0x2ca>
   abd4e:	mov    %r13,%rax
   abd51:	mul    %rdi
   abd54:	shr    $1,%rdx
   abd57:	lea    0x11(%rdx),%rcx
   abd5b:	mov    $0x7,%al
   abd5d:	cmp    %rcx,%rbx
   abd60:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abd66:	movzbl (%r14),%ecx
   abd6a:	mov    %ecx,%esi
   abd6c:	and    $0xf0,%sil
   abd70:	cmp    $0xe0,%sil
   abd74:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abd7a:	and    $0xf,%cl
   abd7d:	mov    $0x5,%al
   abd7f:	cmp    $0x1,%cl
   abd82:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abd88:	lea    0x1(%rdx),%rsi
   abd8c:	pcmpeqd %xmm0,%xmm0
   abd90:	movdqa %xmm0,0x260(%rsp)
   abd99:	movdqa %xmm0,0x250(%rsp)
   abda2:	movdqa %xmm0,0x240(%rsp)
   abdab:	movdqa %xmm0,0x230(%rsp)
   abdb4:	movdqa %xmm0,0x220(%rsp)
   abdbd:	movdqa %xmm0,0x210(%rsp)
   abdc6:	movdqa %xmm0,0x200(%rsp)
   abdcf:	movdqa %xmm0,0x1f0(%rsp)
   abdd8:	movdqa %xmm0,0x120(%rsp)
   abde1:	movdqa %xmm0,0x110(%rsp)
   abdea:	movdqa %xmm0,0x100(%rsp)
   abdf3:	movdqa %xmm0,0xf0(%rsp)
   abdfc:	xor    %r8d,%r8d
   abdff:	cmp    $0x1,%cl
   abe02:	setne  %r9b
   abe06:	lea    -0x10(%rbx),%rax
   abe0a:	mov    %rax,0x28(%rsp)
   abe0f:	mov    %r12,%rax
   abe12:	mov    %rdx,%rcx
   abe15:	mul    %rdi
   abe18:	shr    $0x2,%rdx
   abe1c:	cmp    %rcx,%rdx
   abe1f:	cmovae %rcx,%rdx
   abe23:	mov    %rdx,0x38(%rsp)
   abe28:	test   %rdx,%rdx
   abe2b:	je     ad82f <meshoptimizer_rs::codec::raw+0x1b4f>
   abe31:	mov    %r9b,%r8b
   abe34:	lea    0xd(,%r8,2),%rax
   abe3c:	mov    %rax,0x58(%rsp)
   abe41:	mov    %r14,0x30(%rsp)
   abe46:	lea    (%r14,%rbx,1),%rax
   abe4a:	add    $0xfffffffffffffff0,%rax
   abe4e:	mov    %rax,0x50(%rsp)
   abe53:	add    $0x4,%r15
   abe57:	xor    %r8d,%r8d
   abe5a:	xor    %r10d,%r10d
   abe5d:	movl   $0x0,0x20(%rsp)
   abe65:	xor    %r11d,%r11d
   abe68:	xor    %edi,%edi
   abe6a:	mov    0x28(%rsp),%r9
   abe6f:	jmp    ac336 <meshoptimizer_rs::codec::raw+0x656>
   abe74:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   abe79:	cmp    $0x2,%al
   abe7b:	jb     abee4 <meshoptimizer_rs::codec::raw+0x204>
   abe7d:	cmp    $0x20,%rbp
   abe81:	lea    -0x32a58(%rip),%rax        # 79430 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   abe88:	lea    -0x33f3f(%rip),%rcx        # 77f50 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   abe8f:	cmove  %rax,%rcx
   abe93:	lea    -0x2d01a(%rip),%rax        # 7ee80 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   abe9a:	lea    -0x2e8c1(%rip),%rdx        # 7d5e0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   abea1:	cmove  %rax,%rdx
   abea5:	cmp    $0x4,%rbp
   abea9:	lea    -0x303f0(%rip),%rax        # 7bac0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   abeb0:	cmovne %rcx,%rax
   abeb4:	lea    -0x2a59b(%rip),%r10        # 81920 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   abebb:	cmovne %rdx,%r10
   abebf:	cmp    $0x21,%r13
   abec3:	cmovae %rax,%r10
   abec7:	mov    %r15,%rdi
   abeca:	mov    %r12,%rsi
   abecd:	mov    %r13,%rdx
   abed0:	mov    %rbp,%rcx
   abed3:	mov    %r14,%r8
   abed6:	mov    %rbx,%r9
   abed9:	call   *%r10
   abedc:	cmp    $0xfe,%al
   abede:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abee4:	mov    $0x7,%al
   abee6:	test   %rbx,%rbx
   abee9:	je     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abeef:	movzbl (%r14),%edi
   abef3:	mov    %edi,%ecx
   abef5:	and    $0xf0,%cl
   abef8:	cmp    $0xa0,%cl
   abefb:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abf01:	and    $0xf,%dil
   abf05:	mov    $0x5,%al
   abf07:	cmp    $0x1,%dil
   abf0b:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abf11:	test   %dil,%dil
   abf14:	je     ace33 <meshoptimizer_rs::codec::raw+0x1153>
   abf1a:	mov    %rbp,%rcx
   abf1d:	shr    $0x2,%rcx
   abf21:	add    %rbp,%rcx
   abf24:	cmp    $0x19,%rcx
   abf28:	mov    $0x18,%eax
   abf2d:	cmovae %rcx,%rax
   abf31:	jmp    ace43 <meshoptimizer_rs::codec::raw+0x1163>
   abf36:	add    $0x5,%r13
   abf3a:	mov    $0x7,%al
   abf3c:	cmp    $0x2,%rbp
   abf40:	jne    abfb1 <meshoptimizer_rs::codec::raw+0x2d1>
   abf42:	cmp    %r13,%rbx
   abf45:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abf4b:	test   %rbx,%rbx
   abf4e:	je     ad8e4 <meshoptimizer_rs::codec::raw+0x1c04>
   abf54:	movzbl (%r14),%ecx
   abf58:	mov    %ecx,%edx
   abf5a:	and    $0xf0,%dl
   abf5d:	cmp    $0xd0,%dl
   abf60:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abf66:	mov    $0x5,%al
   abf68:	test   $0xe,%cl
   abf6b:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abf71:	dec    %rbx
   abf74:	movq   $0x0,0x1f0(%rsp)
   abf80:	movabs $0x7ffffffffffffffc,%rax
   abf8a:	or     $0x2,%rax
   abf8e:	and    %rax,%r12
   abf91:	je     ac294 <meshoptimizer_rs::codec::raw+0x5b4>
   abf97:	inc    %r14
   abf9a:	xor    %eax,%eax
   abf9c:	jmp    ac046 <meshoptimizer_rs::codec::raw+0x366>
   abfa1:	test   %rax,%rax
   abfa4:	je     ac177 <meshoptimizer_rs::codec::raw+0x497>
   abfaa:	xor    %eax,%eax
   abfac:	jmp    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abfb1:	cmp    %r13,%rbx
   abfb4:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abfba:	test   %rbx,%rbx
   abfbd:	je     ad8e4 <meshoptimizer_rs::codec::raw+0x1c04>
   abfc3:	movzbl (%r14),%ecx
   abfc7:	mov    %ecx,%edx
   abfc9:	and    $0xf0,%dl
   abfcc:	cmp    $0xd0,%dl
   abfcf:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abfd5:	mov    $0x5,%al
   abfd7:	test   $0xe,%cl
   abfda:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   abfe0:	dec    %rbx
   abfe3:	movq   $0x0,0x1f0(%rsp)
   abfef:	movabs $0x7ffffffffffffffc,%rax
   abff9:	and    %rax,%r12
   abffc:	je     ac294 <meshoptimizer_rs::codec::raw+0x5b4>
   ac002:	inc    %r14
   ac005:	xor    %eax,%eax
   ac007:	jmp    ac0fb <meshoptimizer_rs::codec::raw+0x41b>
   ac00c:	dec    %rbx
   ac00f:	inc    %r14
   ac012:	mov    %edx,%ecx
   ac014:	mov    %ecx,%edx
   ac016:	shr    $0x2,%edx
   ac019:	mov    %ecx,%esi
   ac01b:	shl    $0x1e,%esi
   ac01e:	sar    $0x1f,%esi
   ac021:	xor    %edx,%esi
   ac023:	and    $0x1,%ecx
   ac026:	add    0x1f0(%rsp,%rcx,4),%esi
   ac02d:	mov    %esi,0x1f0(%rsp,%rcx,4)
   ac034:	mov    %si,(%r15,%rax,1)
   ac039:	add    $0x2,%rax
   ac03d:	cmp    %rax,%r12
   ac040:	je     ac294 <meshoptimizer_rs::codec::raw+0x5b4>
   ac046:	cmp    $0x5,%rbx
   ac04a:	jb     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac050:	movzbl (%r14),%edx
   ac054:	test   %dl,%dl
   ac056:	jns    ac00c <meshoptimizer_rs::codec::raw+0x32c>
   ac058:	and    $0x7f,%edx
   ac05b:	movzbl 0x1(%r14),%ecx
   ac060:	mov    %ecx,%esi
   ac062:	and    $0x7f,%ecx
   ac065:	shl    $0x7,%ecx
   ac068:	or     %edx,%ecx
   ac06a:	mov    $0x2,%edx
   ac06f:	test   %sil,%sil
   ac072:	jns    ac0b7 <meshoptimizer_rs::codec::raw+0x3d7>
   ac074:	movzbl 0x2(%r14),%edx
   ac079:	mov    %edx,%esi
   ac07b:	and    $0x7f,%edx
   ac07e:	shl    $0xe,%edx
   ac081:	or     %edx,%ecx
   ac083:	mov    $0x3,%edx
   ac088:	test   %sil,%sil
   ac08b:	jns    ac0b7 <meshoptimizer_rs::codec::raw+0x3d7>
   ac08d:	movzbl 0x3(%r14),%edx
   ac092:	mov    %edx,%esi
   ac094:	and    $0x7f,%edx
   ac097:	shl    $0x15,%edx
   ac09a:	or     %edx,%ecx
   ac09c:	mov    $0x4,%edx
   ac0a1:	test   %sil,%sil
   ac0a4:	jns    ac0b7 <meshoptimizer_rs::codec::raw+0x3d7>
   ac0a6:	movzbl 0x4(%r14),%esi
   ac0ab:	shl    $0x1c,%esi
   ac0ae:	or     %ecx,%esi
   ac0b0:	mov    $0x5,%edx
   ac0b5:	mov    %esi,%ecx
   ac0b7:	sub    %rdx,%rbx
   ac0ba:	add    %rdx,%r14
   ac0bd:	jmp    ac014 <meshoptimizer_rs::codec::raw+0x334>
   ac0c2:	dec    %rbx
   ac0c5:	inc    %r14
   ac0c8:	mov    %edx,%ecx
   ac0ca:	mov    %ecx,%edx
   ac0cc:	shr    $0x2,%edx
   ac0cf:	mov    %ecx,%esi
   ac0d1:	shl    $0x1e,%esi
   ac0d4:	sar    $0x1f,%esi
   ac0d7:	xor    %edx,%esi
   ac0d9:	and    $0x1,%ecx
   ac0dc:	add    0x1f0(%rsp,%rcx,4),%esi
   ac0e3:	mov    %esi,0x1f0(%rsp,%rcx,4)
   ac0ea:	mov    %esi,(%r15,%rax,1)
   ac0ee:	add    $0x4,%rax
   ac0f2:	cmp    %rax,%r12
   ac0f5:	je     ac294 <meshoptimizer_rs::codec::raw+0x5b4>
   ac0fb:	cmp    $0x5,%rbx
   ac0ff:	jb     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac105:	movzbl (%r14),%edx
   ac109:	test   %dl,%dl
   ac10b:	jns    ac0c2 <meshoptimizer_rs::codec::raw+0x3e2>
   ac10d:	and    $0x7f,%edx
   ac110:	movzbl 0x1(%r14),%ecx
   ac115:	mov    %ecx,%esi
   ac117:	and    $0x7f,%ecx
   ac11a:	shl    $0x7,%ecx
   ac11d:	or     %edx,%ecx
   ac11f:	mov    $0x2,%edx
   ac124:	test   %sil,%sil
   ac127:	jns    ac16c <meshoptimizer_rs::codec::raw+0x48c>
   ac129:	movzbl 0x2(%r14),%edx
   ac12e:	mov    %edx,%esi
   ac130:	and    $0x7f,%edx
   ac133:	shl    $0xe,%edx
   ac136:	or     %edx,%ecx
   ac138:	mov    $0x3,%edx
   ac13d:	test   %sil,%sil
   ac140:	jns    ac16c <meshoptimizer_rs::codec::raw+0x48c>
   ac142:	movzbl 0x3(%r14),%edx
   ac147:	mov    %edx,%esi
   ac149:	and    $0x7f,%edx
   ac14c:	shl    $0x15,%edx
   ac14f:	or     %edx,%ecx
   ac151:	mov    $0x4,%edx
   ac156:	test   %sil,%sil
   ac159:	jns    ac16c <meshoptimizer_rs::codec::raw+0x48c>
   ac15b:	movzbl 0x4(%r14),%esi
   ac160:	shl    $0x1c,%esi
   ac163:	or     %ecx,%esi
   ac165:	mov    $0x5,%edx
   ac16a:	mov    %esi,%ecx
   ac16c:	sub    %rdx,%rbx
   ac16f:	add    %rdx,%r14
   ac172:	jmp    ac0ca <meshoptimizer_rs::codec::raw+0x3ea>
   ac177:	lea    0x11(%rdx),%rcx
   ac17b:	mov    $0x7,%al
   ac17d:	cmp    %rcx,%rbx
   ac180:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ac186:	movzbl (%r14),%ecx
   ac18a:	mov    %ecx,%esi
   ac18c:	and    $0xf0,%sil
   ac190:	cmp    $0xe0,%sil
   ac194:	jne    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ac19a:	and    $0xf,%cl
   ac19d:	mov    $0x5,%al
   ac19f:	cmp    $0x1,%cl
   ac1a2:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ac1a8:	lea    0x1(%rdx),%rsi
   ac1ac:	pcmpeqd %xmm0,%xmm0
   ac1b0:	movdqa %xmm0,0x260(%rsp)
   ac1b9:	movdqa %xmm0,0x250(%rsp)
   ac1c2:	movdqa %xmm0,0x240(%rsp)
   ac1cb:	movdqa %xmm0,0x230(%rsp)
   ac1d4:	movdqa %xmm0,0x220(%rsp)
   ac1dd:	movdqa %xmm0,0x210(%rsp)
   ac1e6:	movdqa %xmm0,0x200(%rsp)
   ac1ef:	movdqa %xmm0,0x1f0(%rsp)
   ac1f8:	movdqa %xmm0,0x120(%rsp)
   ac201:	movdqa %xmm0,0x110(%rsp)
   ac20a:	movdqa %xmm0,0x100(%rsp)
   ac213:	movdqa %xmm0,0xf0(%rsp)
   ac21c:	xor    %r8d,%r8d
   ac21f:	cmp    $0x1,%cl
   ac222:	setne  %r9b
   ac226:	lea    -0x10(%rbx),%rax
   ac22a:	mov    %rax,0x28(%rsp)
   ac22f:	mov    %r12,%rax
   ac232:	mov    %rdx,%rcx
   ac235:	mul    %rdi
   ac238:	shr    $0x3,%rdx
   ac23c:	cmp    %rcx,%rdx
   ac23f:	cmovae %rcx,%rdx
   ac243:	mov    %rdx,0x38(%rsp)
   ac248:	test   %rdx,%rdx
   ac24b:	je     ad82f <meshoptimizer_rs::codec::raw+0x1b4f>
   ac251:	mov    %r9b,%r8b
   ac254:	lea    0xd(,%r8,2),%rax
   ac25c:	mov    %rax,0x58(%rsp)
   ac261:	mov    %r14,0x30(%rsp)
   ac266:	lea    (%r14,%rbx,1),%rax
   ac26a:	add    $0xfffffffffffffff0,%rax
   ac26e:	mov    %rax,0x50(%rsp)
   ac273:	add    $0x8,%r15
   ac277:	xor    %r8d,%r8d
   ac27a:	xor    %r10d,%r10d
   ac27d:	movl   $0x0,0x20(%rsp)
   ac285:	xor    %r11d,%r11d
   ac288:	xor    %edi,%edi
   ac28a:	mov    0x28(%rsp),%r9
   ac28f:	jmp    ac8fe <meshoptimizer_rs::codec::raw+0xc1e>
   ac294:	cmp    $0x4,%rbx
   ac298:	sete   %al
   ac29b:	jmp    ad829 <meshoptimizer_rs::codec::raw+0x1b49>
   ac2a0:	mov    %bpl,%r14b
   ac2a3:	add    %r14d,%r11d
   ac2a6:	mov    %ebx,0xf0(%rsp,%r10,4)
   ac2ae:	mov    %r11d,%ebp
   ac2b1:	not    %eax
   ac2b3:	shr    $0x4,%eax
   ac2b6:	add    %r8d,%eax
   ac2b9:	and    $0xf,%eax
   ac2bc:	mov    0x1f0(%rsp,%rax,8),%r11d
   ac2c4:	mov    0x1f4(%rsp,%rax,8),%r15d
   ac2cc:	add    %r14,%r10
   ac2cf:	mov    $0x2,%eax
   ac2d4:	mov    $0x1,%r12d
   ac2da:	mov    %r8,%r13
   ac2dd:	inc    %rdi
   ac2e0:	mov    %ebx,0x1f0(%rsp,%r13,8)
   ac2e8:	mov    %r15d,0x1f4(%rsp,%r13,8)
   ac2f0:	add    %r8d,%r12d
   ac2f3:	and    $0xf,%r12d
   ac2f7:	mov    %r11d,0x1f0(%rsp,%r12,8)
   ac2ff:	mov    %ebx,0x1f4(%rsp,%r12,8)
   ac307:	add    %r8d,%eax
   ac30a:	and    $0xf,%eax
   ac30d:	and    $0xf,%r10d
   ac311:	mov    %r11w,-0x4(%rdx)
   ac316:	mov    %r15w,-0x2(%rdx)
   ac31b:	mov    %rdx,%r15
   ac31e:	mov    %bx,(%rdx)
   ac321:	add    $0x6,%r15
   ac325:	mov    %rax,%r8
   ac328:	mov    %ebp,%r11d
   ac32b:	cmp    %rdi,0x38(%rsp)
   ac330:	je     ad834 <meshoptimizer_rs::codec::raw+0x1b54>
   ac336:	cmp    %r9,%rsi
   ac339:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac33f:	mov    %r15,%rdx
   ac342:	mov    0x30(%rsp),%r14
   ac347:	movzbl 0x1(%r14,%rdi,1),%eax
   ac34d:	cmp    $0xf0,%al
   ac34f:	jae    ac389 <meshoptimizer_rs::codec::raw+0x6a9>
   ac351:	mov    %eax,%r15d
   ac354:	and    $0xf,%r15d
   ac358:	cmp    %r15d,0x58(%rsp)
   ac35d:	jbe    ac3c3 <meshoptimizer_rs::codec::raw+0x6e3>
   ac35f:	xor    %r14d,%r14d
   ac362:	mov    %r11d,%ebx
   ac365:	test   %r15b,%r15b
   ac368:	sete   %bpl
   ac36c:	je     ac2a0 <meshoptimizer_rs::codec::raw+0x5c0>
   ac372:	not    %r15d
   ac375:	add    %r10d,%r15d
   ac378:	and    $0xf,%r15d
   ac37c:	mov    0xf0(%rsp,%r15,4),%ebx
   ac384:	jmp    ac2a0 <meshoptimizer_rs::codec::raw+0x5c0>
   ac389:	cmp    $0xfe,%al
   ac38b:	jae    ac3e5 <meshoptimizer_rs::codec::raw+0x705>
   ac38d:	and    $0xf,%eax
   ac390:	mov    0x50(%rsp),%rcx
   ac395:	movzbl (%rcx,%rax,1),%r13d
   ac39a:	lea    0x1(%r11),%r15d
   ac39e:	xor    %eax,%eax
   ac3a0:	mov    %r13d,%ecx
   ac3a3:	shr    $0x4,%ecx
   ac3a6:	sete   %r12b
   ac3aa:	je     ac428 <meshoptimizer_rs::codec::raw+0x748>
   ac3ac:	mov    %r10d,%ebx
   ac3af:	sub    %ecx,%ebx
   ac3b1:	and    $0xf,%ebx
   ac3b4:	mov    %r15d,%ebp
   ac3b7:	mov    0xf0(%rsp,%rbx,4),%ecx
   ac3be:	mov    %ecx,%r15d
   ac3c1:	jmp    ac42c <meshoptimizer_rs::codec::raw+0x74c>
   ac3c3:	cmp    $0xf,%r15b
   ac3c7:	jne    ac4a8 <meshoptimizer_rs::codec::raw+0x7c8>
   ac3cd:	movzbl (%r14,%rsi,1),%ebp
   ac3d2:	test   %bpl,%bpl
   ac3d5:	js     ac516 <meshoptimizer_rs::codec::raw+0x836>
   ac3db:	inc    %rsi
   ac3de:	mov    %ebp,%ebx
   ac3e0:	jmp    ac6ac <meshoptimizer_rs::codec::raw+0x9cc>
   ac3e5:	movzbl (%r14,%rsi,1),%r14d
   ac3ea:	xor    %r15d,%r15d
   ac3ed:	cmp    $0xfe,%eax
   ac3f2:	sete   %r15b
   ac3f6:	test   %r14d,%r14d
   ac3f9:	cmove  %r14d,%r11d
   ac3fd:	add    %r11d,%r15d
   ac400:	mov    %r14d,%r12d
   ac403:	shr    $0x4,%r12d
   ac407:	je     ac4c3 <meshoptimizer_rs::codec::raw+0x7e3>
   ac40d:	mov    %r10d,%ecx
   ac410:	sub    %r12d,%ecx
   ac413:	and    $0xf,%ecx
   ac416:	mov    %r15d,%ebx
   ac419:	mov    0xf0(%rsp,%rcx,4),%ecx
   ac420:	mov    %ecx,%r15d
   ac423:	jmp    ac4c7 <meshoptimizer_rs::codec::raw+0x7e7>
   ac428:	lea    0x2(%r11),%ebp
   ac42c:	xor    %r14d,%r14d
   ac42f:	mov    %ebp,%ebx
   ac431:	test   $0xf,%r13b
   ac435:	sete   %cl
   ac438:	je     ac44a <meshoptimizer_rs::codec::raw+0x76a>
   ac43a:	mov    %r10d,%ebx
   ac43d:	sub    %r13d,%ebx
   ac440:	and    $0xf,%ebx
   ac443:	mov    0xf0(%rsp,%rbx,4),%ebx
   ac44a:	mov    %cl,%r14b
   ac44d:	add    %r14d,%ebp
   ac450:	mov    %r11d,0xf0(%rsp,%r10,4)
   ac458:	lea    0x1(%r10),%rcx
   ac45c:	and    $0xf,%ecx
   ac45f:	mov    %r15d,0xf0(%rsp,%rcx,4)
   ac467:	mov    %r12b,%al
   ac46a:	add    %r10,%rax
   ac46d:	inc    %rax
   ac470:	add    %rax,%r14
   ac473:	and    $0xf,%eax
   ac476:	mov    %ebx,0xf0(%rsp,%rax,4)
   ac47d:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ac485:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ac48d:	lea    0x1(%r8),%r13d
   ac491:	and    $0xf,%r13d
   ac495:	mov    $0x3,%eax
   ac49a:	mov    $0x2,%r12d
   ac4a0:	mov    %r14,%r10
   ac4a3:	jmp    ac2dd <meshoptimizer_rs::codec::raw+0x5fd>
   ac4a8:	movzbl %r15b,%ecx
   ac4ac:	xor    $0x3,%r15b
   ac4b0:	movzbl %r15b,%ebx
   ac4b4:	mov    0x20(%rsp),%ebp
   ac4b8:	add    %ecx,%ebp
   ac4ba:	sub    %ebx,%ebp
   ac4bc:	mov    %ebp,%ebx
   ac4be:	jmp    ac6bd <meshoptimizer_rs::codec::raw+0x9dd>
   ac4c3:	lea    0x1(%r15),%ebx
   ac4c7:	mov    %r14d,%r13d
   ac4ca:	and    $0xf,%r13b
   ac4ce:	je     ac4e6 <meshoptimizer_rs::codec::raw+0x806>
   ac4d0:	mov    %r10d,%ecx
   ac4d3:	sub    %r14d,%ecx
   ac4d6:	and    $0xf,%ecx
   ac4d9:	mov    %ebx,%ebp
   ac4db:	mov    0xf0(%rsp,%rcx,4),%ecx
   ac4e2:	mov    %ecx,%ebx
   ac4e4:	jmp    ac4e9 <meshoptimizer_rs::codec::raw+0x809>
   ac4e6:	lea    0x1(%rbx),%ebp
   ac4e9:	lea    0x1(%rsi),%r14
   ac4ed:	cmp    $0xfe,%eax
   ac4f2:	je     ac5f0 <meshoptimizer_rs::codec::raw+0x910>
   ac4f8:	mov    0x30(%rsp),%r9
   ac4fd:	movzbl 0x1(%r9,%rsi,1),%eax
   ac503:	test   %al,%al
   ac505:	js     ac536 <meshoptimizer_rs::codec::raw+0x856>
   ac507:	add    $0x2,%rsi
   ac50b:	mov    %rsi,%r14
   ac50e:	mov    %eax,%r11d
   ac511:	jmp    ac5d2 <meshoptimizer_rs::codec::raw+0x8f2>
   ac516:	and    $0x7f,%ebp
   ac519:	movzbl 0x1(%r14,%rsi,1),%ebx
   ac51f:	mov    %ebx,%ecx
   ac521:	and    $0x7f,%ebx
   ac524:	shl    $0x7,%ebx
   ac527:	or     %ebp,%ebx
   ac529:	test   %cl,%cl
   ac52b:	js     ac557 <meshoptimizer_rs::codec::raw+0x877>
   ac52d:	add    $0x2,%rsi
   ac531:	jmp    ac6ac <meshoptimizer_rs::codec::raw+0x9cc>
   ac536:	and    $0x7f,%eax
   ac539:	movzbl 0x2(%r9,%rsi,1),%r11d
   ac53f:	mov    %r11d,%ecx
   ac542:	and    $0x7f,%r11d
   ac546:	shl    $0x7,%r11d
   ac54a:	or     %eax,%r11d
   ac54d:	test   %cl,%cl
   ac54f:	js     ac575 <meshoptimizer_rs::codec::raw+0x895>
   ac551:	add    $0x3,%rsi
   ac555:	jmp    ac5cf <meshoptimizer_rs::codec::raw+0x8ef>
   ac557:	movzbl 0x2(%r14,%rsi,1),%ecx
   ac55d:	mov    %ecx,%ebp
   ac55f:	and    $0x7f,%ecx
   ac562:	shl    $0xe,%ecx
   ac565:	or     %ecx,%ebx
   ac567:	test   %bpl,%bpl
   ac56a:	js     ac590 <meshoptimizer_rs::codec::raw+0x8b0>
   ac56c:	add    $0x3,%rsi
   ac570:	jmp    ac6ac <meshoptimizer_rs::codec::raw+0x9cc>
   ac575:	movzbl 0x3(%r9,%rsi,1),%eax
   ac57b:	mov    %eax,%ecx
   ac57d:	and    $0x7f,%eax
   ac580:	shl    $0xe,%eax
   ac583:	or     %eax,%r11d
   ac586:	test   %cl,%cl
   ac588:	js     ac5b2 <meshoptimizer_rs::codec::raw+0x8d2>
   ac58a:	add    $0x4,%rsi
   ac58e:	jmp    ac5cf <meshoptimizer_rs::codec::raw+0x8ef>
   ac590:	movzbl 0x3(%r14,%rsi,1),%ecx
   ac596:	mov    %ecx,%ebp
   ac598:	and    $0x7f,%ecx
   ac59b:	shl    $0x15,%ecx
   ac59e:	or     %ecx,%ebx
   ac5a0:	test   %bpl,%bpl
   ac5a3:	js     ac69b <meshoptimizer_rs::codec::raw+0x9bb>
   ac5a9:	add    $0x4,%rsi
   ac5ad:	jmp    ac6ac <meshoptimizer_rs::codec::raw+0x9cc>
   ac5b2:	movzbl 0x4(%r9,%rsi,1),%eax
   ac5b8:	mov    %eax,%ecx
   ac5ba:	and    $0x7f,%eax
   ac5bd:	shl    $0x15,%eax
   ac5c0:	or     %eax,%r11d
   ac5c3:	test   %cl,%cl
   ac5c5:	js     ac6d4 <meshoptimizer_rs::codec::raw+0x9f4>
   ac5cb:	add    $0x5,%rsi
   ac5cf:	mov    %rsi,%r14
   ac5d2:	mov    0x28(%rsp),%r9
   ac5d7:	mov    %r11d,%eax
   ac5da:	shr    $1,%eax
   ac5dc:	and    $0x1,%r11d
   ac5e0:	neg    %r11d
   ac5e3:	xor    %eax,%r11d
   ac5e6:	add    0x20(%rsp),%r11d
   ac5eb:	mov    %r11d,0x20(%rsp)
   ac5f0:	cmp    $0xf,%r12b
   ac5f4:	jne    ac723 <meshoptimizer_rs::codec::raw+0xa43>
   ac5fa:	cmp    $0xfffffffffffffffa,%r14
   ac5fe:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac604:	lea    0x5(%r14),%rax
   ac608:	cmp    0x630(%rsp),%rax
   ac610:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac616:	mov    0x30(%rsp),%r9
   ac61b:	movzbl (%r9,%r14,1),%esi
   ac620:	test   %sil,%sil
   ac623:	js     ac633 <meshoptimizer_rs::codec::raw+0x953>
   ac625:	inc    %r14
   ac628:	mov    %r14,%rax
   ac62b:	mov    %esi,%r15d
   ac62e:	jmp    ac702 <meshoptimizer_rs::codec::raw+0xa22>
   ac633:	and    $0x7f,%esi
   ac636:	movzbl 0x1(%r9,%r14,1),%r15d
   ac63c:	mov    %r15d,%ecx
   ac63f:	and    $0x7f,%r15d
   ac643:	shl    $0x7,%r15d
   ac647:	or     %esi,%r15d
   ac64a:	test   %cl,%cl
   ac64c:	js     ac65a <meshoptimizer_rs::codec::raw+0x97a>
   ac64e:	add    $0x2,%r14
   ac652:	mov    %r14,%rax
   ac655:	jmp    ac702 <meshoptimizer_rs::codec::raw+0xa22>
   ac65a:	movzbl 0x2(%r9,%r14,1),%ecx
   ac660:	mov    %ecx,%esi
   ac662:	and    $0x7f,%ecx
   ac665:	shl    $0xe,%ecx
   ac668:	or     %ecx,%r15d
   ac66b:	test   %sil,%sil
   ac66e:	js     ac67c <meshoptimizer_rs::codec::raw+0x99c>
   ac670:	add    $0x3,%r14
   ac674:	mov    %r14,%rax
   ac677:	jmp    ac702 <meshoptimizer_rs::codec::raw+0xa22>
   ac67c:	movzbl 0x3(%r9,%r14,1),%ecx
   ac682:	mov    %ecx,%esi
   ac684:	and    $0x7f,%ecx
   ac687:	shl    $0x15,%ecx
   ac68a:	or     %ecx,%r15d
   ac68d:	test   %sil,%sil
   ac690:	js     ac6ee <meshoptimizer_rs::codec::raw+0xa0e>
   ac692:	add    $0x4,%r14
   ac696:	mov    %r14,%rax
   ac699:	jmp    ac702 <meshoptimizer_rs::codec::raw+0xa22>
   ac69b:	movzbl 0x4(%r14,%rsi,1),%ecx
   ac6a1:	add    $0x5,%rsi
   ac6a5:	shl    $0x1c,%ecx
   ac6a8:	or     %ebx,%ecx
   ac6aa:	mov    %ecx,%ebx
   ac6ac:	mov    0x20(%rsp),%ebp
   ac6b0:	mov    %ebx,%ecx
   ac6b2:	shr    $1,%ecx
   ac6b4:	and    $0x1,%ebx
   ac6b7:	neg    %ebx
   ac6b9:	xor    %ecx,%ebx
   ac6bb:	add    %ebp,%ebx
   ac6bd:	mov    %ebx,0xf0(%rsp,%r10,4)
   ac6c5:	mov    $0x1,%r14d
   ac6cb:	mov    %ebx,0x20(%rsp)
   ac6cf:	jmp    ac2ae <meshoptimizer_rs::codec::raw+0x5ce>
   ac6d4:	add    $0x6,%rsi
   ac6d8:	mov    0x30(%rsp),%rax
   ac6dd:	movzbl 0x4(%rax,%r14,1),%eax
   ac6e3:	shl    $0x1c,%eax
   ac6e6:	or     %r11d,%eax
   ac6e9:	jmp    ac50b <meshoptimizer_rs::codec::raw+0x82b>
   ac6ee:	mov    0x30(%rsp),%rcx
   ac6f3:	movzbl 0x4(%rcx,%r14,1),%ecx
   ac6f9:	shl    $0x1c,%ecx
   ac6fc:	or     %r15d,%ecx
   ac6ff:	mov    %ecx,%r15d
   ac702:	mov    0x28(%rsp),%r9
   ac707:	mov    %r15d,%ecx
   ac70a:	shr    $1,%ecx
   ac70c:	and    $0x1,%r15d
   ac710:	neg    %r15d
   ac713:	xor    %ecx,%r15d
   ac716:	add    0x20(%rsp),%r15d
   ac71b:	mov    %rax,%r14
   ac71e:	mov    %r15d,0x20(%rsp)
   ac723:	cmp    $0xf,%r13b
   ac727:	jne    ac7eb <meshoptimizer_rs::codec::raw+0xb0b>
   ac72d:	cmp    $0xfffffffffffffffa,%r14
   ac731:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac737:	lea    0x5(%r14),%rax
   ac73b:	cmp    0x630(%rsp),%rax
   ac743:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac749:	mov    0x30(%rsp),%r9
   ac74e:	movzbl (%r9,%r14,1),%esi
   ac753:	test   %sil,%sil
   ac756:	js     ac762 <meshoptimizer_rs::codec::raw+0xa82>
   ac758:	inc    %r14
   ac75b:	mov    %r14,%rax
   ac75e:	mov    %esi,%ebx
   ac760:	jmp    ac7d0 <meshoptimizer_rs::codec::raw+0xaf0>
   ac762:	and    $0x7f,%esi
   ac765:	movzbl 0x1(%r9,%r14,1),%ebx
   ac76b:	mov    %ebx,%ecx
   ac76d:	and    $0x7f,%ebx
   ac770:	shl    $0x7,%ebx
   ac773:	or     %esi,%ebx
   ac775:	test   %cl,%cl
   ac777:	js     ac782 <meshoptimizer_rs::codec::raw+0xaa2>
   ac779:	add    $0x2,%r14
   ac77d:	mov    %r14,%rax
   ac780:	jmp    ac7d0 <meshoptimizer_rs::codec::raw+0xaf0>
   ac782:	movzbl 0x2(%r9,%r14,1),%ecx
   ac788:	mov    %ecx,%esi
   ac78a:	and    $0x7f,%ecx
   ac78d:	shl    $0xe,%ecx
   ac790:	or     %ecx,%ebx
   ac792:	test   %sil,%sil
   ac795:	js     ac7a0 <meshoptimizer_rs::codec::raw+0xac0>
   ac797:	add    $0x3,%r14
   ac79b:	mov    %r14,%rax
   ac79e:	jmp    ac7d0 <meshoptimizer_rs::codec::raw+0xaf0>
   ac7a0:	movzbl 0x3(%r9,%r14,1),%ecx
   ac7a6:	mov    %ecx,%esi
   ac7a8:	and    $0x7f,%ecx
   ac7ab:	shl    $0x15,%ecx
   ac7ae:	or     %ecx,%ebx
   ac7b0:	test   %sil,%sil
   ac7b3:	js     ac7be <meshoptimizer_rs::codec::raw+0xade>
   ac7b5:	add    $0x4,%r14
   ac7b9:	mov    %r14,%rax
   ac7bc:	jmp    ac7d0 <meshoptimizer_rs::codec::raw+0xaf0>
   ac7be:	mov    0x30(%rsp),%rcx
   ac7c3:	movzbl 0x4(%rcx,%r14,1),%ecx
   ac7c9:	shl    $0x1c,%ecx
   ac7cc:	or     %ebx,%ecx
   ac7ce:	mov    %ecx,%ebx
   ac7d0:	mov    0x28(%rsp),%r9
   ac7d5:	mov    %ebx,%ecx
   ac7d7:	shr    $1,%ecx
   ac7d9:	and    $0x1,%ebx
   ac7dc:	neg    %ebx
   ac7de:	xor    %ecx,%ebx
   ac7e0:	add    0x20(%rsp),%ebx
   ac7e4:	mov    %rax,%r14
   ac7e7:	mov    %ebx,0x20(%rsp)
   ac7eb:	cmp    $0xf,%r13b
   ac7ef:	sete   %al
   ac7f2:	cmp    $0xf,%r12b
   ac7f6:	sete   %cl
   ac7f9:	test   %r13b,%r13b
   ac7fc:	sete   %sil
   ac800:	test   %r12b,%r12b
   ac803:	sete   %r12b
   ac807:	mov    %r11d,0xf0(%rsp,%r10,4)
   ac80f:	lea    0x1(%r10),%r13
   ac813:	and    $0xf,%r13d
   ac817:	or     %cl,%r12b
   ac81a:	mov    %r15d,0xf0(%rsp,%r13,4)
   ac822:	movzbl %r12b,%ecx
   ac826:	add    %r10,%rcx
   ac829:	inc    %rcx
   ac82c:	or     %al,%sil
   ac82f:	movzbl %sil,%r10d
   ac833:	add    %rcx,%r10
   ac836:	and    $0xf,%ecx
   ac839:	mov    %ebx,0xf0(%rsp,%rcx,4)
   ac840:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ac848:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ac850:	lea    0x1(%r8),%r13d
   ac854:	and    $0xf,%r13d
   ac858:	mov    $0x3,%eax
   ac85d:	mov    $0x2,%r12d
   ac863:	mov    %r14,%rsi
   ac866:	jmp    ac2dd <meshoptimizer_rs::codec::raw+0x5fd>
   ac86b:	mov    %bpl,%r14b
   ac86e:	add    %r14d,%r11d
   ac871:	mov    %ebx,0xf0(%rsp,%r10,4)
   ac879:	mov    %r11d,%ebp
   ac87c:	not    %eax
   ac87e:	shr    $0x4,%eax
   ac881:	add    %r8d,%eax
   ac884:	and    $0xf,%eax
   ac887:	mov    0x1f0(%rsp,%rax,8),%r11d
   ac88f:	mov    0x1f4(%rsp,%rax,8),%r15d
   ac897:	add    %r14,%r10
   ac89a:	mov    $0x2,%eax
   ac89f:	mov    $0x1,%r12d
   ac8a5:	mov    %r8,%r13
   ac8a8:	inc    %rdi
   ac8ab:	mov    %ebx,0x1f0(%rsp,%r13,8)
   ac8b3:	mov    %r15d,0x1f4(%rsp,%r13,8)
   ac8bb:	add    %r8d,%r12d
   ac8be:	and    $0xf,%r12d
   ac8c2:	mov    %r11d,0x1f0(%rsp,%r12,8)
   ac8ca:	mov    %ebx,0x1f4(%rsp,%r12,8)
   ac8d2:	add    %r8d,%eax
   ac8d5:	and    $0xf,%eax
   ac8d8:	and    $0xf,%r10d
   ac8dc:	mov    %r11d,-0x8(%rdx)
   ac8e0:	mov    %r15d,-0x4(%rdx)
   ac8e4:	mov    %rdx,%r15
   ac8e7:	mov    %ebx,(%rdx)
   ac8e9:	add    $0xc,%r15
   ac8ed:	mov    %rax,%r8
   ac8f0:	mov    %ebp,%r11d
   ac8f3:	cmp    %rdi,0x38(%rsp)
   ac8f8:	je     ad834 <meshoptimizer_rs::codec::raw+0x1b54>
   ac8fe:	cmp    %r9,%rsi
   ac901:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   ac907:	mov    %r15,%rdx
   ac90a:	mov    0x30(%rsp),%r14
   ac90f:	movzbl 0x1(%r14,%rdi,1),%eax
   ac915:	cmp    $0xf0,%al
   ac917:	jae    ac951 <meshoptimizer_rs::codec::raw+0xc71>
   ac919:	mov    %eax,%r15d
   ac91c:	and    $0xf,%r15d
   ac920:	cmp    %r15d,0x58(%rsp)
   ac925:	jbe    ac98b <meshoptimizer_rs::codec::raw+0xcab>
   ac927:	xor    %r14d,%r14d
   ac92a:	mov    %r11d,%ebx
   ac92d:	test   %r15b,%r15b
   ac930:	sete   %bpl
   ac934:	je     ac86b <meshoptimizer_rs::codec::raw+0xb8b>
   ac93a:	not    %r15d
   ac93d:	add    %r10d,%r15d
   ac940:	and    $0xf,%r15d
   ac944:	mov    0xf0(%rsp,%r15,4),%ebx
   ac94c:	jmp    ac86b <meshoptimizer_rs::codec::raw+0xb8b>
   ac951:	cmp    $0xfe,%al
   ac953:	jae    ac9ad <meshoptimizer_rs::codec::raw+0xccd>
   ac955:	and    $0xf,%eax
   ac958:	mov    0x50(%rsp),%rcx
   ac95d:	movzbl (%rcx,%rax,1),%r13d
   ac962:	lea    0x1(%r11),%r15d
   ac966:	xor    %eax,%eax
   ac968:	mov    %r13d,%ecx
   ac96b:	shr    $0x4,%ecx
   ac96e:	sete   %r12b
   ac972:	je     ac9f0 <meshoptimizer_rs::codec::raw+0xd10>
   ac974:	mov    %r10d,%ebx
   ac977:	sub    %ecx,%ebx
   ac979:	and    $0xf,%ebx
   ac97c:	mov    %r15d,%ebp
   ac97f:	mov    0xf0(%rsp,%rbx,4),%ecx
   ac986:	mov    %ecx,%r15d
   ac989:	jmp    ac9f4 <meshoptimizer_rs::codec::raw+0xd14>
   ac98b:	cmp    $0xf,%r15b
   ac98f:	jne    aca70 <meshoptimizer_rs::codec::raw+0xd90>
   ac995:	movzbl (%r14,%rsi,1),%ebp
   ac99a:	test   %bpl,%bpl
   ac99d:	js     acade <meshoptimizer_rs::codec::raw+0xdfe>
   ac9a3:	inc    %rsi
   ac9a6:	mov    %ebp,%ebx
   ac9a8:	jmp    acc74 <meshoptimizer_rs::codec::raw+0xf94>
   ac9ad:	movzbl (%r14,%rsi,1),%r14d
   ac9b2:	xor    %r15d,%r15d
   ac9b5:	cmp    $0xfe,%eax
   ac9ba:	sete   %r15b
   ac9be:	test   %r14d,%r14d
   ac9c1:	cmove  %r14d,%r11d
   ac9c5:	add    %r11d,%r15d
   ac9c8:	mov    %r14d,%r12d
   ac9cb:	shr    $0x4,%r12d
   ac9cf:	je     aca8b <meshoptimizer_rs::codec::raw+0xdab>
   ac9d5:	mov    %r10d,%ecx
   ac9d8:	sub    %r12d,%ecx
   ac9db:	and    $0xf,%ecx
   ac9de:	mov    %r15d,%ebx
   ac9e1:	mov    0xf0(%rsp,%rcx,4),%ecx
   ac9e8:	mov    %ecx,%r15d
   ac9eb:	jmp    aca8f <meshoptimizer_rs::codec::raw+0xdaf>
   ac9f0:	lea    0x2(%r11),%ebp
   ac9f4:	xor    %r14d,%r14d
   ac9f7:	mov    %ebp,%ebx
   ac9f9:	test   $0xf,%r13b
   ac9fd:	sete   %cl
   aca00:	je     aca12 <meshoptimizer_rs::codec::raw+0xd32>
   aca02:	mov    %r10d,%ebx
   aca05:	sub    %r13d,%ebx
   aca08:	and    $0xf,%ebx
   aca0b:	mov    0xf0(%rsp,%rbx,4),%ebx
   aca12:	mov    %cl,%r14b
   aca15:	add    %r14d,%ebp
   aca18:	mov    %r11d,0xf0(%rsp,%r10,4)
   aca20:	lea    0x1(%r10),%rcx
   aca24:	and    $0xf,%ecx
   aca27:	mov    %r15d,0xf0(%rsp,%rcx,4)
   aca2f:	mov    %r12b,%al
   aca32:	add    %r10,%rax
   aca35:	inc    %rax
   aca38:	add    %rax,%r14
   aca3b:	and    $0xf,%eax
   aca3e:	mov    %ebx,0xf0(%rsp,%rax,4)
   aca45:	mov    %r15d,0x1f0(%rsp,%r8,8)
   aca4d:	mov    %r11d,0x1f4(%rsp,%r8,8)
   aca55:	lea    0x1(%r8),%r13d
   aca59:	and    $0xf,%r13d
   aca5d:	mov    $0x3,%eax
   aca62:	mov    $0x2,%r12d
   aca68:	mov    %r14,%r10
   aca6b:	jmp    ac8a8 <meshoptimizer_rs::codec::raw+0xbc8>
   aca70:	movzbl %r15b,%ecx
   aca74:	xor    $0x3,%r15b
   aca78:	movzbl %r15b,%ebx
   aca7c:	mov    0x20(%rsp),%ebp
   aca80:	add    %ecx,%ebp
   aca82:	sub    %ebx,%ebp
   aca84:	mov    %ebp,%ebx
   aca86:	jmp    acc85 <meshoptimizer_rs::codec::raw+0xfa5>
   aca8b:	lea    0x1(%r15),%ebx
   aca8f:	mov    %r14d,%r13d
   aca92:	and    $0xf,%r13b
   aca96:	je     acaae <meshoptimizer_rs::codec::raw+0xdce>
   aca98:	mov    %r10d,%ecx
   aca9b:	sub    %r14d,%ecx
   aca9e:	and    $0xf,%ecx
   acaa1:	mov    %ebx,%ebp
   acaa3:	mov    0xf0(%rsp,%rcx,4),%ecx
   acaaa:	mov    %ecx,%ebx
   acaac:	jmp    acab1 <meshoptimizer_rs::codec::raw+0xdd1>
   acaae:	lea    0x1(%rbx),%ebp
   acab1:	lea    0x1(%rsi),%r14
   acab5:	cmp    $0xfe,%eax
   acaba:	je     acbb8 <meshoptimizer_rs::codec::raw+0xed8>
   acac0:	mov    0x30(%rsp),%r9
   acac5:	movzbl 0x1(%r9,%rsi,1),%eax
   acacb:	test   %al,%al
   acacd:	js     acafe <meshoptimizer_rs::codec::raw+0xe1e>
   acacf:	add    $0x2,%rsi
   acad3:	mov    %rsi,%r14
   acad6:	mov    %eax,%r11d
   acad9:	jmp    acb9a <meshoptimizer_rs::codec::raw+0xeba>
   acade:	and    $0x7f,%ebp
   acae1:	movzbl 0x1(%r14,%rsi,1),%ebx
   acae7:	mov    %ebx,%ecx
   acae9:	and    $0x7f,%ebx
   acaec:	shl    $0x7,%ebx
   acaef:	or     %ebp,%ebx
   acaf1:	test   %cl,%cl
   acaf3:	js     acb1f <meshoptimizer_rs::codec::raw+0xe3f>
   acaf5:	add    $0x2,%rsi
   acaf9:	jmp    acc74 <meshoptimizer_rs::codec::raw+0xf94>
   acafe:	and    $0x7f,%eax
   acb01:	movzbl 0x2(%r9,%rsi,1),%r11d
   acb07:	mov    %r11d,%ecx
   acb0a:	and    $0x7f,%r11d
   acb0e:	shl    $0x7,%r11d
   acb12:	or     %eax,%r11d
   acb15:	test   %cl,%cl
   acb17:	js     acb3d <meshoptimizer_rs::codec::raw+0xe5d>
   acb19:	add    $0x3,%rsi
   acb1d:	jmp    acb97 <meshoptimizer_rs::codec::raw+0xeb7>
   acb1f:	movzbl 0x2(%r14,%rsi,1),%ecx
   acb25:	mov    %ecx,%ebp
   acb27:	and    $0x7f,%ecx
   acb2a:	shl    $0xe,%ecx
   acb2d:	or     %ecx,%ebx
   acb2f:	test   %bpl,%bpl
   acb32:	js     acb58 <meshoptimizer_rs::codec::raw+0xe78>
   acb34:	add    $0x3,%rsi
   acb38:	jmp    acc74 <meshoptimizer_rs::codec::raw+0xf94>
   acb3d:	movzbl 0x3(%r9,%rsi,1),%eax
   acb43:	mov    %eax,%ecx
   acb45:	and    $0x7f,%eax
   acb48:	shl    $0xe,%eax
   acb4b:	or     %eax,%r11d
   acb4e:	test   %cl,%cl
   acb50:	js     acb7a <meshoptimizer_rs::codec::raw+0xe9a>
   acb52:	add    $0x4,%rsi
   acb56:	jmp    acb97 <meshoptimizer_rs::codec::raw+0xeb7>
   acb58:	movzbl 0x3(%r14,%rsi,1),%ecx
   acb5e:	mov    %ecx,%ebp
   acb60:	and    $0x7f,%ecx
   acb63:	shl    $0x15,%ecx
   acb66:	or     %ecx,%ebx
   acb68:	test   %bpl,%bpl
   acb6b:	js     acc63 <meshoptimizer_rs::codec::raw+0xf83>
   acb71:	add    $0x4,%rsi
   acb75:	jmp    acc74 <meshoptimizer_rs::codec::raw+0xf94>
   acb7a:	movzbl 0x4(%r9,%rsi,1),%eax
   acb80:	mov    %eax,%ecx
   acb82:	and    $0x7f,%eax
   acb85:	shl    $0x15,%eax
   acb88:	or     %eax,%r11d
   acb8b:	test   %cl,%cl
   acb8d:	js     acc9c <meshoptimizer_rs::codec::raw+0xfbc>
   acb93:	add    $0x5,%rsi
   acb97:	mov    %rsi,%r14
   acb9a:	mov    0x28(%rsp),%r9
   acb9f:	mov    %r11d,%eax
   acba2:	shr    $1,%eax
   acba4:	and    $0x1,%r11d
   acba8:	neg    %r11d
   acbab:	xor    %eax,%r11d
   acbae:	add    0x20(%rsp),%r11d
   acbb3:	mov    %r11d,0x20(%rsp)
   acbb8:	cmp    $0xf,%r12b
   acbbc:	jne    acceb <meshoptimizer_rs::codec::raw+0x100b>
   acbc2:	cmp    $0xfffffffffffffffa,%r14
   acbc6:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   acbcc:	lea    0x5(%r14),%rax
   acbd0:	cmp    0x630(%rsp),%rax
   acbd8:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   acbde:	mov    0x30(%rsp),%r9
   acbe3:	movzbl (%r9,%r14,1),%esi
   acbe8:	test   %sil,%sil
   acbeb:	js     acbfb <meshoptimizer_rs::codec::raw+0xf1b>
   acbed:	inc    %r14
   acbf0:	mov    %r14,%rax
   acbf3:	mov    %esi,%r15d
   acbf6:	jmp    accca <meshoptimizer_rs::codec::raw+0xfea>
   acbfb:	and    $0x7f,%esi
   acbfe:	movzbl 0x1(%r9,%r14,1),%r15d
   acc04:	mov    %r15d,%ecx
   acc07:	and    $0x7f,%r15d
   acc0b:	shl    $0x7,%r15d
   acc0f:	or     %esi,%r15d
   acc12:	test   %cl,%cl
   acc14:	js     acc22 <meshoptimizer_rs::codec::raw+0xf42>
   acc16:	add    $0x2,%r14
   acc1a:	mov    %r14,%rax
   acc1d:	jmp    accca <meshoptimizer_rs::codec::raw+0xfea>
   acc22:	movzbl 0x2(%r9,%r14,1),%ecx
   acc28:	mov    %ecx,%esi
   acc2a:	and    $0x7f,%ecx
   acc2d:	shl    $0xe,%ecx
   acc30:	or     %ecx,%r15d
   acc33:	test   %sil,%sil
   acc36:	js     acc44 <meshoptimizer_rs::codec::raw+0xf64>
   acc38:	add    $0x3,%r14
   acc3c:	mov    %r14,%rax
   acc3f:	jmp    accca <meshoptimizer_rs::codec::raw+0xfea>
   acc44:	movzbl 0x3(%r9,%r14,1),%ecx
   acc4a:	mov    %ecx,%esi
   acc4c:	and    $0x7f,%ecx
   acc4f:	shl    $0x15,%ecx
   acc52:	or     %ecx,%r15d
   acc55:	test   %sil,%sil
   acc58:	js     accb6 <meshoptimizer_rs::codec::raw+0xfd6>
   acc5a:	add    $0x4,%r14
   acc5e:	mov    %r14,%rax
   acc61:	jmp    accca <meshoptimizer_rs::codec::raw+0xfea>
   acc63:	movzbl 0x4(%r14,%rsi,1),%ecx
   acc69:	add    $0x5,%rsi
   acc6d:	shl    $0x1c,%ecx
   acc70:	or     %ebx,%ecx
   acc72:	mov    %ecx,%ebx
   acc74:	mov    0x20(%rsp),%ebp
   acc78:	mov    %ebx,%ecx
   acc7a:	shr    $1,%ecx
   acc7c:	and    $0x1,%ebx
   acc7f:	neg    %ebx
   acc81:	xor    %ecx,%ebx
   acc83:	add    %ebp,%ebx
   acc85:	mov    %ebx,0xf0(%rsp,%r10,4)
   acc8d:	mov    $0x1,%r14d
   acc93:	mov    %ebx,0x20(%rsp)
   acc97:	jmp    ac879 <meshoptimizer_rs::codec::raw+0xb99>
   acc9c:	add    $0x6,%rsi
   acca0:	mov    0x30(%rsp),%rax
   acca5:	movzbl 0x4(%rax,%r14,1),%eax
   accab:	shl    $0x1c,%eax
   accae:	or     %r11d,%eax
   accb1:	jmp    acad3 <meshoptimizer_rs::codec::raw+0xdf3>
   accb6:	mov    0x30(%rsp),%rcx
   accbb:	movzbl 0x4(%rcx,%r14,1),%ecx
   accc1:	shl    $0x1c,%ecx
   accc4:	or     %r15d,%ecx
   accc7:	mov    %ecx,%r15d
   accca:	mov    0x28(%rsp),%r9
   acccf:	mov    %r15d,%ecx
   accd2:	shr    $1,%ecx
   accd4:	and    $0x1,%r15d
   accd8:	neg    %r15d
   accdb:	xor    %ecx,%r15d
   accde:	add    0x20(%rsp),%r15d
   acce3:	mov    %rax,%r14
   acce6:	mov    %r15d,0x20(%rsp)
   acceb:	cmp    $0xf,%r13b
   accef:	jne    acdb3 <meshoptimizer_rs::codec::raw+0x10d3>
   accf5:	cmp    $0xfffffffffffffffa,%r14
   accf9:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   accff:	lea    0x5(%r14),%rax
   acd03:	cmp    0x630(%rsp),%rax
   acd0b:	ja     ad83b <meshoptimizer_rs::codec::raw+0x1b5b>
   acd11:	mov    0x30(%rsp),%r9
   acd16:	movzbl (%r9,%r14,1),%esi
   acd1b:	test   %sil,%sil
   acd1e:	js     acd2a <meshoptimizer_rs::codec::raw+0x104a>
   acd20:	inc    %r14
   acd23:	mov    %r14,%rax
   acd26:	mov    %esi,%ebx
   acd28:	jmp    acd98 <meshoptimizer_rs::codec::raw+0x10b8>
   acd2a:	and    $0x7f,%esi
   acd2d:	movzbl 0x1(%r9,%r14,1),%ebx
   acd33:	mov    %ebx,%ecx
   acd35:	and    $0x7f,%ebx
   acd38:	shl    $0x7,%ebx
   acd3b:	or     %esi,%ebx
   acd3d:	test   %cl,%cl
   acd3f:	js     acd4a <meshoptimizer_rs::codec::raw+0x106a>
   acd41:	add    $0x2,%r14
   acd45:	mov    %r14,%rax
   acd48:	jmp    acd98 <meshoptimizer_rs::codec::raw+0x10b8>
   acd4a:	movzbl 0x2(%r9,%r14,1),%ecx
   acd50:	mov    %ecx,%esi
   acd52:	and    $0x7f,%ecx
   acd55:	shl    $0xe,%ecx
   acd58:	or     %ecx,%ebx
   acd5a:	test   %sil,%sil
   acd5d:	js     acd68 <meshoptimizer_rs::codec::raw+0x1088>
   acd5f:	add    $0x3,%r14
   acd63:	mov    %r14,%rax
   acd66:	jmp    acd98 <meshoptimizer_rs::codec::raw+0x10b8>
   acd68:	movzbl 0x3(%r9,%r14,1),%ecx
   acd6e:	mov    %ecx,%esi
   acd70:	and    $0x7f,%ecx
   acd73:	shl    $0x15,%ecx
   acd76:	or     %ecx,%ebx
   acd78:	test   %sil,%sil
   acd7b:	js     acd86 <meshoptimizer_rs::codec::raw+0x10a6>
   acd7d:	add    $0x4,%r14
   acd81:	mov    %r14,%rax
   acd84:	jmp    acd98 <meshoptimizer_rs::codec::raw+0x10b8>
   acd86:	mov    0x30(%rsp),%rcx
   acd8b:	movzbl 0x4(%rcx,%r14,1),%ecx
   acd91:	shl    $0x1c,%ecx
   acd94:	or     %ebx,%ecx
   acd96:	mov    %ecx,%ebx
   acd98:	mov    0x28(%rsp),%r9
   acd9d:	mov    %ebx,%ecx
   acd9f:	shr    $1,%ecx
   acda1:	and    $0x1,%ebx
   acda4:	neg    %ebx
   acda6:	xor    %ecx,%ebx
   acda8:	add    0x20(%rsp),%ebx
   acdac:	mov    %rax,%r14
   acdaf:	mov    %ebx,0x20(%rsp)
   acdb3:	cmp    $0xf,%r13b
   acdb7:	sete   %al
   acdba:	cmp    $0xf,%r12b
   acdbe:	sete   %cl
   acdc1:	test   %r13b,%r13b
   acdc4:	sete   %sil
   acdc8:	test   %r12b,%r12b
   acdcb:	sete   %r12b
   acdcf:	mov    %r11d,0xf0(%rsp,%r10,4)
   acdd7:	lea    0x1(%r10),%r13
   acddb:	and    $0xf,%r13d
   acddf:	or     %cl,%r12b
   acde2:	mov    %r15d,0xf0(%rsp,%r13,4)
   acdea:	movzbl %r12b,%ecx
   acdee:	add    %r10,%rcx
   acdf1:	inc    %rcx
   acdf4:	or     %al,%sil
   acdf7:	movzbl %sil,%r10d
   acdfb:	add    %rcx,%r10
   acdfe:	and    $0xf,%ecx
   ace01:	mov    %ebx,0xf0(%rsp,%rcx,4)
   ace08:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ace10:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ace18:	lea    0x1(%r8),%r13d
   ace1c:	and    $0xf,%r13d
   ace20:	mov    $0x3,%eax
   ace25:	mov    $0x2,%r12d
   ace2b:	mov    %r14,%rsi
   ace2e:	jmp    ac8a8 <meshoptimizer_rs::codec::raw+0xbc8>
   ace33:	cmp    $0x21,%rbp
   ace37:	mov    $0x20,%eax
   ace3c:	cmovae %rbp,%rax
   ace40:	mov    %rbp,%rcx
   ace43:	mov    %rax,0xe0(%rsp)
   ace4b:	lea    0x1(%rax),%rdx
   ace4f:	mov    $0x7,%al
   ace51:	cmp    %rdx,%rbx
   ace54:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ace5a:	mov    %rbx,%rsi
   ace5d:	sub    %rcx,%rsi
   ace60:	lea    (%rsi,%rbp,1),%rax
   ace64:	mov    %rbx,%rcx
   ace67:	mov    %rax,0x60(%rsp)
   ace6c:	sub    %rax,%rcx
   ace6f:	mov    %rcx,0xa0(%rsp)
   ace77:	jb     ad8bc <meshoptimizer_rs::codec::raw+0x1bdc>
   ace7d:	pxor   %xmm0,%xmm0
   ace81:	movdqa %xmm0,0x1e0(%rsp)
   ace8a:	movdqa %xmm0,0x1d0(%rsp)
   ace93:	movdqa %xmm0,0x1c0(%rsp)
   ace9c:	movdqa %xmm0,0x1b0(%rsp)
   acea5:	movdqa %xmm0,0x1a0(%rsp)
   aceae:	movdqa %xmm0,0x190(%rsp)
   aceb7:	movdqa %xmm0,0x180(%rsp)
   acec0:	movdqa %xmm0,0x170(%rsp)
   acec9:	movdqa %xmm0,0x160(%rsp)
   aced2:	movdqa %xmm0,0x150(%rsp)
   acedb:	movdqa %xmm0,0x140(%rsp)
   acee4:	movdqa %xmm0,0x130(%rsp)
   aceed:	movdqa %xmm0,0x120(%rsp)
   acef6:	movdqa %xmm0,0x110(%rsp)
   aceff:	movdqa %xmm0,0x100(%rsp)
   acf08:	movdqa %xmm0,0xf0(%rsp)
   acf11:	cmp    $0x101,%rbp
   acf18:	jae    ad87d <meshoptimizer_rs::codec::raw+0x1b9d>
   acf1e:	mov    %dil,0x38(%rsp)
   acf23:	lea    (%rsi,%rbp,1),%rax
   acf27:	cmp    %rsi,%rax
   acf2a:	jb     ad894 <meshoptimizer_rs::codec::raw+0x1bb4>
   acf30:	add    %r14,%rsi
   acf33:	lea    0xf0(%rsp),%rdi
   acf3b:	mov    %rbp,%rdx
   acf3e:	call   *0x79e34(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   acf44:	test   %rbp,%rbp
   acf47:	je     ad925 <meshoptimizer_rs::codec::raw+0x1c45>
   acf4d:	mov    $0x2000,%ax
   acf51:	xor    %edx,%edx
   acf53:	div    %bp
   acf56:	and    $0x3ff0,%eax
   acf5b:	cmp    $0x100,%eax
   acf60:	mov    $0x100,%ecx
   acf65:	cmovb  %eax,%ecx
   acf68:	mov    %rcx,0xd8(%rsp)
   acf70:	lea    0x1f0(%rsp),%rdi
   acf78:	mov    $0x400,%edx
   acf7d:	xor    %esi,%esi
   acf7f:	call   *0x79e03(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   acf85:	test   %r13,%r13
   acf88:	je     ad810 <meshoptimizer_rs::codec::raw+0x1b30>
   acf8e:	mov    %r15,0xd0(%rsp)
   acf96:	add    %r14,0x60(%rsp)
   acf9b:	mov    %rbp,%rax
   acf9e:	shr    $0x2,%rax
   acfa2:	mov    %rax,0x98(%rsp)
   acfaa:	lea    0x3(%rbp),%rax
   acfae:	shr    $0x2,%rax
   acfb2:	mov    %rax,0xe8(%rsp)
   acfba:	mov    $0x1,%r15d
   acfc0:	xor    %eax,%eax
   acfc2:	mov    %r12,0x88(%rsp)
   acfca:	mov    %r14,0x30(%rsp)
   acfcf:	mov    %r13,%rcx
   acfd2:	mov    %rax,%rdi
   acfd5:	sub    %rax,%rcx
   acfd8:	mov    0xd8(%rsp),%rax
   acfe0:	cmp    %rax,%rcx
   acfe3:	cmovae %rax,%rcx
   acfe7:	mov    %rcx,0x20(%rsp)
   acfec:	movzbl 0x38(%rsp),%edx
   acff1:	test   %dl,%dl
   acff3:	je     ad039 <meshoptimizer_rs::codec::raw+0x1359>
   acff5:	mov    %r15,%rcx
   acff8:	mov    $0x7,%al
   acffa:	add    0x98(%rsp),%rcx
   ad002:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad008:	cmp    %rbx,%rcx
   ad00b:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad011:	add    %r14,%r15
   ad014:	mov    0x98(%rsp),%rax
   ad01c:	mov    %rax,0x90(%rsp)
   ad024:	mov    %r15,0xa8(%rsp)
   ad02c:	mov    %rcx,%r15
   ad02f:	mov    0x88(%rsp),%r12
   ad037:	jmp    ad052 <meshoptimizer_rs::codec::raw+0x1372>
   ad039:	movq   $0x0,0x90(%rsp)
   ad045:	mov    $0x1,%eax
   ad04a:	mov    %rax,0xa8(%rsp)
   ad052:	mov    0x20(%rsp),%rax
   ad057:	lea    (%rax,%rdi,1),%rsi
   ad05b:	imul   %rbp,%rdi
   ad05f:	mov    %rsi,0xc8(%rsp)
   ad067:	imul   %rbp,%rsi
   ad06b:	cmp    %rdi,%rsi
   ad06e:	jb     ad8ac <meshoptimizer_rs::codec::raw+0x1bcc>
   ad074:	cmp    %r12,%rsi
   ad077:	ja     ad8ac <meshoptimizer_rs::codec::raw+0x1bcc>
   ad07d:	mov    0x20(%rsp),%rax
   ad082:	lea    0xf(%rax),%ecx
   ad085:	and    $0x3f0,%ecx
   ad08b:	mov    %rcx,0x28(%rsp)
   ad090:	mov    %rax,%rcx
   ad093:	imul   %rbp,%rcx
   ad097:	mov    %rcx,0xb8(%rsp)
   ad09f:	add    0xd0(%rsp),%rdi
   ad0a7:	mov    %rdi,0xb0(%rsp)
   ad0af:	lea    -0x1(%rax),%rcx
   ad0b3:	imul   %rbp,%rcx
   ad0b7:	mov    %rcx,0xc0(%rsp)
   ad0bf:	add    $0x4,%rcx
   ad0c3:	mov    %rcx,0x50(%rsp)
   ad0c8:	lea    0x0(,%rax,4),%rcx
   ad0d0:	mov    %rcx,0x68(%rsp)
   ad0d5:	lea    0x1f0(%rsp,%rax,1),%rcx
   ad0dd:	mov    %rcx,0x80(%rsp)
   ad0e5:	lea    0x1f0(%rsp,%rax,2),%rcx
   ad0ed:	mov    %rcx,0x78(%rsp)
   ad0f2:	lea    (%rax,%rax,2),%rax
   ad0f6:	add    %rsp,%rax
   ad0f9:	add    $0x1f0,%rax
   ad0ff:	mov    %rax,0x70(%rsp)
   ad104:	xor    %r14d,%r14d
   ad107:	xor    %ebx,%ebx
   ad109:	jmp    ad16a <meshoptimizer_rs::codec::raw+0x148a>
   ad10b:	lea    (%rsp,%rbx,1),%rcx
   ad10f:	add    $0xf0,%rcx
   ad116:	shr    $0x4,%al
   ad119:	neg    %al
   ad11b:	and    $0x1f,%al
   ad11d:	movzbl %al,%eax
   ad120:	mov    %rcx,(%rsp)
   ad124:	mov    %eax,0x8(%rsp)
   ad128:	movq   $0x0,0x10(%rsp)
   ad131:	lea    0x1f0(%rsp),%rdi
   ad139:	mov    0x68(%rsp),%rsi
   ad13e:	mov    0x50(%rsp),%rcx
   ad143:	mov    0x20(%rsp),%r8
   ad148:	mov    %rbp,%r9
   ad14b:	call   72ce0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   ad150:	inc    %r14
   ad153:	add    $0x4,%rbx
   ad157:	cmp    %r14,0xe8(%rsp)
   ad15f:	movzbl 0x38(%rsp),%edx
   ad164:	je     ad7b8 <meshoptimizer_rs::codec::raw+0x1ad8>
   ad16a:	mov    %rbx,0x58(%rsp)
   ad16f:	test   %dl,%dl
   ad171:	je     ad190 <meshoptimizer_rs::codec::raw+0x14b0>
   ad173:	cmp    0x90(%rsp),%r14
   ad17b:	jae    ad8f5 <meshoptimizer_rs::codec::raw+0x1c15>
   ad181:	mov    0xa8(%rsp),%rax
   ad189:	movzbl (%rax,%r14,1),%ebx
   ad18e:	jmp    ad192 <meshoptimizer_rs::codec::raw+0x14b2>
   ad190:	xor    %ebx,%ebx
   ad192:	mov    %ebx,%eax
   ad194:	and    $0x3,%al
   ad196:	movzbl %al,%ecx
   ad199:	mov    0x28(%rsp),%rsi
   ad19e:	cmp    %rsi,0x20(%rsp)
   ad1a3:	ja     ad603 <meshoptimizer_rs::codec::raw+0x1923>
   ad1a9:	cmp    $0x2,%al
   ad1ab:	je     ad1ed <meshoptimizer_rs::codec::raw+0x150d>
   ad1ad:	cmp    $0x3,%ecx
   ad1b0:	jne    ad207 <meshoptimizer_rs::codec::raw+0x1527>
   ad1b2:	mov    %r15,%r12
   ad1b5:	mov    $0x7,%al
   ad1b7:	add    0x20(%rsp),%r12
   ad1bc:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad1c2:	cmp    0x630(%rsp),%r12
   ad1ca:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad1d0:	add    0x30(%rsp),%r15
   ad1d5:	lea    0x1f0(%rsp),%rdi
   ad1dd:	mov    %r15,%rsi
   ad1e0:	mov    0x20(%rsp),%rdx
   ad1e5:	call   *0x79b8d(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   ad1eb:	jmp    ad263 <meshoptimizer_rs::codec::raw+0x1583>
   ad1ed:	lea    0x1f0(%rsp),%rdi
   ad1f5:	xor    %esi,%esi
   ad1f7:	mov    0x20(%rsp),%rdx
   ad1fc:	call   *0x79b86(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   ad202:	mov    %r15,%r12
   ad205:	jmp    ad263 <meshoptimizer_rs::codec::raw+0x1583>
   ad207:	test   %al,%al
   ad209:	lea    -0x9f5e0(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad210:	lea    -0x9f4c7(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad217:	cmove  %rcx,%rax
   ad21b:	test   %dl,%dl
   ad21d:	lea    -0x9f8f4(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad224:	cmove  %rcx,%rax
   ad228:	mov    %rax,(%rsp)
   ad22c:	lea    0x40(%rsp),%rdi
   ad231:	mov    0x30(%rsp),%rsi
   ad236:	mov    0x630(%rsp),%rdx
   ad23e:	mov    %r15,%rcx
   ad241:	lea    0x1f0(%rsp),%r8
   ad249:	mov    0x28(%rsp),%r9
   ad24e:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad253:	cmpb   $0x0,0x40(%rsp)
   ad258:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad25e:	mov    0x48(%rsp),%r12
   ad263:	mov    %ebx,%eax
   ad265:	shr    $0x2,%al
   ad268:	and    $0x3,%al
   ad26a:	cmp    $0x3,%al
   ad26c:	movzbl 0x38(%rsp),%edx
   ad271:	je     ad295 <meshoptimizer_rs::codec::raw+0x15b5>
   ad273:	movzbl %al,%ecx
   ad276:	cmp    $0x2,%ecx
   ad279:	jne    ad2d3 <meshoptimizer_rs::codec::raw+0x15f3>
   ad27b:	mov    0x80(%rsp),%rdi
   ad283:	xor    %esi,%esi
   ad285:	mov    0x20(%rsp),%rdx
   ad28a:	call   *0x79af8(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   ad290:	jmp    ad32f <meshoptimizer_rs::codec::raw+0x164f>
   ad295:	mov    %r12,%r15
   ad298:	mov    $0x7,%al
   ad29a:	add    0x20(%rsp),%r15
   ad29f:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad2a5:	cmp    0x630(%rsp),%r15
   ad2ad:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad2b3:	add    0x30(%rsp),%r12
   ad2b8:	mov    0x80(%rsp),%rdi
   ad2c0:	mov    %r12,%rsi
   ad2c3:	mov    0x20(%rsp),%rdx
   ad2c8:	call   *0x79aaa(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   ad2ce:	mov    %r15,%r12
   ad2d1:	jmp    ad32f <meshoptimizer_rs::codec::raw+0x164f>
   ad2d3:	test   %al,%al
   ad2d5:	lea    -0x9f6ac(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad2dc:	lea    -0x9f593(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad2e3:	cmove  %rcx,%rax
   ad2e7:	test   %dl,%dl
   ad2e9:	lea    -0x9f9c0(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad2f0:	cmove  %rcx,%rax
   ad2f4:	mov    %rax,(%rsp)
   ad2f8:	lea    0x40(%rsp),%rdi
   ad2fd:	mov    0x30(%rsp),%rsi
   ad302:	mov    0x630(%rsp),%rdx
   ad30a:	mov    %r12,%rcx
   ad30d:	mov    0x80(%rsp),%r8
   ad315:	mov    0x28(%rsp),%r9
   ad31a:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad31f:	cmpb   $0x0,0x40(%rsp)
   ad324:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad32a:	mov    0x48(%rsp),%r12
   ad32f:	mov    %ebx,%eax
   ad331:	shr    $0x4,%al
   ad334:	and    $0x3,%al
   ad336:	cmp    $0x3,%al
   ad338:	je     ad386 <meshoptimizer_rs::codec::raw+0x16a6>
   ad33a:	movzbl %al,%ecx
   ad33d:	cmp    $0x2,%ecx
   ad340:	jne    ad402 <meshoptimizer_rs::codec::raw+0x1722>
   ad346:	mov    0x78(%rsp),%rdi
   ad34b:	xor    %esi,%esi
   ad34d:	mov    0x20(%rsp),%rdx
   ad352:	call   *0x79a30(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   ad358:	shr    $0x6,%bl
   ad35b:	cmp    $0x3,%bl
   ad35e:	je     ad3c7 <meshoptimizer_rs::codec::raw+0x16e7>
   ad360:	movzbl %bl,%eax
   ad363:	cmp    $0x2,%eax
   ad366:	jne    ad46f <meshoptimizer_rs::codec::raw+0x178f>
   ad36c:	mov    0x70(%rsp),%rdi
   ad371:	xor    %esi,%esi
   ad373:	mov    0x20(%rsp),%rdx
   ad378:	call   *0x79a0a(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   ad37e:	mov    %r12,%r15
   ad381:	jmp    ad4cb <meshoptimizer_rs::codec::raw+0x17eb>
   ad386:	mov    %r12,%r15
   ad389:	mov    $0x7,%al
   ad38b:	add    0x20(%rsp),%r15
   ad390:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad396:	cmp    0x630(%rsp),%r15
   ad39e:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad3a4:	add    0x30(%rsp),%r12
   ad3a9:	mov    0x78(%rsp),%rdi
   ad3ae:	mov    %r12,%rsi
   ad3b1:	mov    0x20(%rsp),%rdx
   ad3b6:	call   *0x799bc(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   ad3bc:	mov    %r15,%r12
   ad3bf:	shr    $0x6,%bl
   ad3c2:	cmp    $0x3,%bl
   ad3c5:	jne    ad360 <meshoptimizer_rs::codec::raw+0x1680>
   ad3c7:	mov    %r12,%r15
   ad3ca:	mov    $0x7,%al
   ad3cc:	add    0x20(%rsp),%r15
   ad3d1:	jb     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad3d7:	cmp    0x630(%rsp),%r15
   ad3df:	ja     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad3e5:	add    0x30(%rsp),%r12
   ad3ea:	mov    0x70(%rsp),%rdi
   ad3ef:	mov    %r12,%rsi
   ad3f2:	mov    0x20(%rsp),%rdx
   ad3f7:	call   *0x7997b(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   ad3fd:	jmp    ad4cb <meshoptimizer_rs::codec::raw+0x17eb>
   ad402:	test   %al,%al
   ad404:	lea    -0x9f7db(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad40b:	lea    -0x9f6c2(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad412:	cmove  %rcx,%rax
   ad416:	cmpb   $0x0,0x38(%rsp)
   ad41b:	lea    -0x9faf2(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad422:	cmove  %rcx,%rax
   ad426:	mov    %rax,(%rsp)
   ad42a:	lea    0x40(%rsp),%rdi
   ad42f:	mov    0x30(%rsp),%rsi
   ad434:	mov    0x630(%rsp),%rdx
   ad43c:	mov    %r12,%rcx
   ad43f:	mov    0x78(%rsp),%r8
   ad444:	mov    0x28(%rsp),%r9
   ad449:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad44e:	cmpb   $0x0,0x40(%rsp)
   ad453:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad459:	mov    0x48(%rsp),%r12
   ad45e:	shr    $0x6,%bl
   ad461:	cmp    $0x3,%bl
   ad464:	jne    ad360 <meshoptimizer_rs::codec::raw+0x1680>
   ad46a:	jmp    ad3c7 <meshoptimizer_rs::codec::raw+0x16e7>
   ad46f:	test   %bl,%bl
   ad471:	lea    -0x9f848(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad478:	lea    -0x9f72f(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad47f:	cmove  %rcx,%rax
   ad483:	cmpb   $0x0,0x38(%rsp)
   ad488:	lea    -0x9fb5f(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad48f:	cmove  %rcx,%rax
   ad493:	mov    %rax,(%rsp)
   ad497:	lea    0x40(%rsp),%rdi
   ad49c:	mov    0x30(%rsp),%rsi
   ad4a1:	mov    0x630(%rsp),%rdx
   ad4a9:	mov    %r12,%rcx
   ad4ac:	mov    0x70(%rsp),%r8
   ad4b1:	mov    0x28(%rsp),%r9
   ad4b6:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad4bb:	cmpb   $0x0,0x40(%rsp)
   ad4c0:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad4c6:	mov    0x48(%rsp),%r15
   ad4cb:	cmpb   $0x0,0x38(%rsp)
   ad4d0:	mov    0x88(%rsp),%r12
   ad4d8:	mov    0xb8(%rsp),%rsi
   ad4e0:	mov    0x58(%rsp),%rbx
   ad4e5:	je     ad50c <meshoptimizer_rs::codec::raw+0x182c>
   ad4e7:	cmp    0xa0(%rsp),%r14
   ad4ef:	jae    ad90d <meshoptimizer_rs::codec::raw+0x1c2d>
   ad4f5:	mov    0x60(%rsp),%rax
   ad4fa:	movzbl (%rax,%r14,1),%eax
   ad4ff:	mov    %rsi,%rdx
   ad502:	sub    %rbx,%rdx
   ad505:	jae    ad51a <meshoptimizer_rs::codec::raw+0x183a>
   ad507:	jmp    ad86a <meshoptimizer_rs::codec::raw+0x1b8a>
   ad50c:	xor    %eax,%eax
   ad50e:	mov    %rsi,%rdx
   ad511:	sub    %rbx,%rdx
   ad514:	jb     ad86a <meshoptimizer_rs::codec::raw+0x1b8a>
   ad51a:	cmp    %rdx,0x50(%rsp)
   ad51f:	ja     ad856 <meshoptimizer_rs::codec::raw+0x1b76>
   ad525:	mov    0xb0(%rsp),%rcx
   ad52d:	lea    (%rcx,%rbx,1),%rdx
   ad531:	mov    %eax,%ecx
   ad533:	and    $0x3,%cl
   ad536:	movzbl %cl,%ecx
   ad539:	lea    -0x9cc98(%rip),%rsi        # 108a8 <libm::math::generic::sqrt::RSQRT_TAB+0x4c2>
   ad540:	movslq (%rsi,%rcx,4),%rcx
   ad544:	add    %rsi,%rcx
   ad547:	jmp    *%rcx
   ad549:	mov    %rdx,%r12
   ad54c:	add    %rsp,%rbx
   ad54f:	add    $0xf0,%rbx
   ad556:	call   e2420 <meshoptimizer_rs::codec::simd::dispatch::selected>
   ad55b:	test   %al,%al
   ad55d:	je     ad5c2 <meshoptimizer_rs::codec::raw+0x18e2>
   ad55f:	mov    %rbx,(%rsp)
   ad563:	lea    0x1f0(%rsp),%rdi
   ad56b:	mov    0x68(%rsp),%rsi
   ad570:	mov    %r12,%rdx
   ad573:	mov    0x50(%rsp),%rcx
   ad578:	mov    0x20(%rsp),%r8
   ad57d:	mov    %rbp,%r9
   ad580:	call   75a40 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   ad585:	jmp    ad5f1 <meshoptimizer_rs::codec::raw+0x1911>
   ad587:	lea    (%rsp,%rbx,1),%rax
   ad58b:	add    $0xf0,%rax
   ad591:	mov    %rax,(%rsp)
   ad595:	movq   $0x0,0x8(%rsp)
   ad59e:	lea    0x1f0(%rsp),%rdi
   ad5a6:	mov    0x68(%rsp),%rsi
   ad5ab:	mov    0x50(%rsp),%rcx
   ad5b0:	mov    0x20(%rsp),%r8
   ad5b5:	mov    %rbp,%r9
   ad5b8:	call   72b20 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   ad5bd:	jmp    ad150 <meshoptimizer_rs::codec::raw+0x1470>
   ad5c2:	mov    %rbx,(%rsp)
   ad5c6:	movq   $0x0,0x8(%rsp)
   ad5cf:	lea    0x1f0(%rsp),%rdi
   ad5d7:	mov    0x68(%rsp),%rsi
   ad5dc:	mov    %r12,%rdx
   ad5df:	mov    0x50(%rsp),%rcx
   ad5e4:	mov    0x20(%rsp),%r8
   ad5e9:	mov    %rbp,%r9
   ad5ec:	call   72930 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   ad5f1:	mov    0x88(%rsp),%r12
   ad5f9:	mov    0x58(%rsp),%rbx
   ad5fe:	jmp    ad150 <meshoptimizer_rs::codec::raw+0x1470>
   ad603:	cmp    $0x2,%al
   ad605:	je     ad94b <meshoptimizer_rs::codec::raw+0x1c6b>
   ad60b:	cmp    $0x3,%ecx
   ad60e:	je     ad932 <meshoptimizer_rs::codec::raw+0x1c52>
   ad614:	test   %al,%al
   ad616:	lea    -0x9f9ed(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad61d:	lea    -0x9f8d4(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad624:	cmove  %rcx,%rax
   ad628:	test   %dl,%dl
   ad62a:	lea    -0x9fd01(%rip),%rcx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad631:	cmove  %rcx,%rax
   ad635:	mov    %rax,(%rsp)
   ad639:	lea    0x40(%rsp),%rdi
   ad63e:	mov    0x30(%rsp),%rsi
   ad643:	mov    0x630(%rsp),%rdx
   ad64b:	mov    %r15,%rcx
   ad64e:	lea    0x1f0(%rsp),%r8
   ad656:	mov    0x28(%rsp),%r9
   ad65b:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad660:	cmpb   $0x0,0x40(%rsp)
   ad665:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad66b:	mov    %ebx,%eax
   ad66d:	shr    $0x2,%al
   ad670:	and    $0x3,%al
   ad672:	cmp    $0x2,%al
   ad674:	movzbl 0x38(%rsp),%edx
   ad679:	je     ad94b <meshoptimizer_rs::codec::raw+0x1c6b>
   ad67f:	movzbl %al,%ecx
   ad682:	cmp    $0x3,%ecx
   ad685:	je     ad932 <meshoptimizer_rs::codec::raw+0x1c52>
   ad68b:	test   %al,%al
   ad68d:	lea    -0x9fa64(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad694:	lea    -0x9f94b(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad69b:	cmove  %rcx,%rax
   ad69f:	mov    0x48(%rsp),%rcx
   ad6a4:	test   %dl,%dl
   ad6a6:	lea    -0x9fd7d(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad6ad:	cmove  %rdx,%rax
   ad6b1:	mov    %rax,(%rsp)
   ad6b5:	lea    0x40(%rsp),%rdi
   ad6ba:	mov    0x30(%rsp),%rsi
   ad6bf:	mov    0x630(%rsp),%rdx
   ad6c7:	mov    0x80(%rsp),%r8
   ad6cf:	mov    0x28(%rsp),%r9
   ad6d4:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad6d9:	cmpb   $0x0,0x40(%rsp)
   ad6de:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad6e4:	mov    %ebx,%eax
   ad6e6:	shr    $0x4,%al
   ad6e9:	and    $0x3,%al
   ad6eb:	cmp    $0x2,%al
   ad6ed:	movzbl 0x38(%rsp),%edx
   ad6f2:	je     ad94b <meshoptimizer_rs::codec::raw+0x1c6b>
   ad6f8:	movzbl %al,%ecx
   ad6fb:	cmp    $0x3,%ecx
   ad6fe:	je     ad932 <meshoptimizer_rs::codec::raw+0x1c52>
   ad704:	test   %al,%al
   ad706:	lea    -0x9fadd(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad70d:	lea    -0x9f9c4(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad714:	cmove  %rcx,%rax
   ad718:	mov    0x48(%rsp),%rcx
   ad71d:	test   %dl,%dl
   ad71f:	lea    -0x9fdf6(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad726:	cmove  %rdx,%rax
   ad72a:	mov    %rax,(%rsp)
   ad72e:	lea    0x40(%rsp),%rdi
   ad733:	mov    0x30(%rsp),%rsi
   ad738:	mov    0x630(%rsp),%rdx
   ad740:	mov    0x78(%rsp),%r8
   ad745:	mov    0x28(%rsp),%r9
   ad74a:	call   d3bc0 <meshoptimizer_rs::codec::vertex::bytes>
   ad74f:	cmpb   $0x0,0x40(%rsp)
   ad754:	jne    ad84f <meshoptimizer_rs::codec::raw+0x1b6f>
   ad75a:	shr    $0x6,%bl
   ad75d:	cmp    $0x2,%bl
   ad760:	movzbl 0x38(%rsp),%edx
   ad765:	je     ad94b <meshoptimizer_rs::codec::raw+0x1c6b>
   ad76b:	movzbl %bl,%eax
   ad76e:	cmp    $0x3,%eax
   ad771:	je     ad932 <meshoptimizer_rs::codec::raw+0x1c52>
   ad777:	test   %bl,%bl
   ad779:	lea    -0x9fb50(%rip),%rax        # dc30 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   ad780:	lea    -0x9fa37(%rip),%rcx        # dd50 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   ad787:	cmove  %rcx,%rax
   ad78b:	mov    0x48(%rsp),%rcx
   ad790:	test   %dl,%dl
   ad792:	lea    -0x9fe69(%rip),%rdx        # d930 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   ad799:	cmove  %rdx,%rax
   ad79d:	mov    %rax,(%rsp)
   ad7a1:	lea    0x40(%rsp),%rdi
   ad7a6:	mov    0x30(%rsp),%rsi
   ad7ab:	mov    0x630(%rsp),%rdx
   ad7b3:	jmp    ad4ac <meshoptimizer_rs::codec::raw+0x17cc>
   ad7b8:	mov    0xb8(%rsp),%rdx
   ad7c0:	mov    0xc0(%rsp),%rdi
   ad7c8:	cmp    %rdx,%rdi
   ad7cb:	mov    0x30(%rsp),%r14
   ad7d0:	ja     ad8d4 <meshoptimizer_rs::codec::raw+0x1bf4>
   ad7d6:	mov    0xb0(%rsp),%rsi
   ad7de:	add    %rdi,%rsi
   ad7e1:	lea    0xf0(%rsp),%rdi
   ad7e9:	mov    %rbp,%rdx
   ad7ec:	call   *0x79586(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   ad7f2:	mov    0xc8(%rsp),%rcx
   ad7fa:	mov    %rcx,%rax
   ad7fd:	cmp    %r13,%rcx
   ad800:	mov    0x630(%rsp),%rbx
   ad808:	jb     acfcf <meshoptimizer_rs::codec::raw+0x12ef>
   ad80e:	jmp    ad816 <meshoptimizer_rs::codec::raw+0x1b36>
   ad810:	mov    $0x1,%r15d
   ad816:	sub    %r15,%rbx
   ad819:	setae  %cl
   ad81c:	cmp    0xe0(%rsp),%rbx
   ad824:	sete   %al
   ad827:	and    %cl,%al
   ad829:	neg    %al
   ad82b:	or     $0x7,%al
   ad82d:	jmp    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad82f:	mov    0x28(%rsp),%r9
   ad834:	mov    $0xff,%al
   ad836:	cmp    %r9,%rsi
   ad839:	je     ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad83b:	mov    $0x7,%al
   ad83d:	add    $0x5f8,%rsp
   ad844:	pop    %rbx
   ad845:	pop    %r12
   ad847:	pop    %r13
   ad849:	pop    %r14
   ad84b:	pop    %r15
   ad84d:	pop    %rbp
   ad84e:	ret
   ad84f:	movzbl 0x41(%rsp),%eax
   ad854:	jmp    ad83d <meshoptimizer_rs::codec::raw+0x1b5d>
   ad856:	lea    0x7417b(%rip),%rcx        # 1219d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   ad85d:	xor    %edi,%edi
   ad85f:	mov    0x50(%rsp),%rsi
   ad864:	call   *0x79666(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad86a:	lea    0x7417f(%rip),%rcx        # 1219f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   ad871:	mov    %rsi,%rdx
   ad874:	mov    %rbx,%rdi
   ad877:	call   *0x79653(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad87d:	lea    0x741cc(%rip),%rcx        # 121a50 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   ad884:	mov    $0x100,%edx
   ad889:	xor    %edi,%edi
   ad88b:	mov    %rbp,%rsi
   ad88e:	call   *0x7963c(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad894:	lea    0x7419d(%rip),%rcx        # 121a38 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   ad89b:	mov    %rsi,%rdi
   ad89e:	mov    0x60(%rsp),%rsi
   ad8a3:	mov    %rbx,%rdx
   ad8a6:	call   *0x79624(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad8ac:	lea    0x779ad(%rip),%rcx        # 125260 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a58>
   ad8b3:	mov    %r12,%rdx
   ad8b6:	call   *0x79614(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad8bc:	lea    0x741a5(%rip),%rcx        # 121a68 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   ad8c3:	mov    0x60(%rsp),%rdi
   ad8c8:	mov    %rbx,%rsi
   ad8cb:	mov    %rbx,%rdx
   ad8ce:	call   *0x795fc(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad8d4:	lea    0x740b5(%rip),%rcx        # 121990 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2188>
   ad8db:	mov    %rdx,%rsi
   ad8de:	call   *0x795ec(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad8e4:	lea    0x77165(%rip),%rdx        # 124a50 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5248>
   ad8eb:	xor    %edi,%edi
   ad8ed:	xor    %esi,%esi
   ad8ef:	call   *0x794bb(%rip)        # 126db0 <_DYNAMIC+0x250>
   ad8f5:	lea    0x740ac(%rip),%rdx        # 1219a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21a0>
   ad8fc:	mov    %r14,%rdi
   ad8ff:	mov    0x90(%rsp),%rsi
   ad907:	call   *0x794a3(%rip)        # 126db0 <_DYNAMIC+0x250>
   ad90d:	lea    0x740ac(%rip),%rdx        # 1219c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   ad914:	mov    %r14,%rdi
   ad917:	mov    0xa0(%rsp),%rsi
   ad91f:	call   *0x7948b(%rip)        # 126db0 <_DYNAMIC+0x250>
   ad925:	lea    0x7404c(%rip),%rdi        # 121978 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2170>
   ad92c:	call   *0x79666(%rip)        # 126f98 <_DYNAMIC+0x438>
   ad932:	lea    0x740cf(%rip),%rcx        # 121a08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   ad939:	xor    %edi,%edi
   ad93b:	mov    0x20(%rsp),%rsi
   ad940:	mov    0x28(%rsp),%rdx
   ad945:	call   *0x79585(%rip)        # 126ed0 <_DYNAMIC+0x370>
   ad94b:	lea    0x740ce(%rip),%rcx        # 121a20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   ad952:	xor    %edi,%edi
   ad954:	mov    0x20(%rsp),%rsi
   ad959:	mov    0x28(%rsp),%rdx
   ad95e:	call   *0x7956c(%rip)        # 126ed0 <_DYNAMIC+0x370>
