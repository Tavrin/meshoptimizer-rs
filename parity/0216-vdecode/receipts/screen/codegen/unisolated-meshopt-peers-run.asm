
<artifacts>/screen/driver-unisolated:     file format elf64-x86-64


Disassembly of section .text:

0000000000034cc0 <meshopt_peers::run>:
   34cc0:	push   %rbp
   34cc1:	push   %r15
   34cc3:	push   %r14
   34cc5:	push   %r13
   34cc7:	push   %r12
   34cc9:	push   %rbx
   34cca:	sub    $0x1000,%rsp
   34cd1:	movq   $0x0,(%rsp)
   34cd9:	sub    $0x968,%rsp
   34ce0:	mov    %rsi,0x290(%rsp)
   34ce8:	mov    %rdx,0x298(%rsp)
   34cf0:	mov    0x10(%rcx),%rbx
   34cf4:	mov    0x8(%rcx),%r15
   34cf8:	movb   $0x2,0x6e0(%rsp)
   34d00:	mov    %r15,0x6e8(%rsp)
   34d08:	mov    %rbx,0x6f0(%rsp)
   34d10:	movq   $0x0,0x718(%rsp)
   34d1c:	mov    0x78(%rcx),%rax
   34d20:	movd   0x80(%rcx),%xmm0
   34d28:	mov    %rax,0x220(%rsp)
   34d30:	movd   %xmm0,0x228(%rsp)
   34d39:	movl   $0x0,0x22c(%rsp)
   34d44:	add    $0xfffffffffffffffb,%rdx
   34d48:	cmp    $0xe,%rdx
   34d4c:	ja     36e1e <meshopt_peers::run+0x215e>
   34d52:	mov    %r8,%r12
   34d55:	mov    %rdi,%r14
   34d58:	lea    -0x24863(%rip),%rdi        # 104fc <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   34d5f:	movslq (%rdi,%rdx,4),%rdx
   34d63:	add    %rdi,%rdx
   34d66:	jmp    *%rdx
   34d68:	mov    $0x68636163,%eax
   34d6d:	xor    (%rsi),%eax
   34d6f:	movzbl 0x4(%rsi),%edx
   34d73:	xor    $0x65,%edx
   34d76:	or     %eax,%edx
   34d78:	je     35562 <meshopt_peers::run+0x8a2>
   34d7e:	mov    $0x63746566,%eax
   34d83:	xor    (%rsi),%eax
   34d85:	movzbl 0x4(%rsi),%edx
   34d89:	xor    $0x68,%edx
   34d8c:	or     %eax,%edx
   34d8e:	jne    36e1e <meshopt_peers::run+0x215e>
   34d94:	mov    %rbx,%rax
   34d97:	shr    $0x20,%rax
   34d9b:	jne    37271 <meshopt_peers::run+0x25b1>
   34da1:	mov    0x20(%rcx),%rsi
   34da5:	mov    0x28(%rcx),%rdx
   34da9:	lea    0x0(,%rbx,4),%rax
   34db1:	lea    (%rax,%rax,2),%rax
   34db5:	mov    %r15,0x350(%rsp)
   34dbd:	mov    %rax,0x358(%rsp)
   34dc5:	mov    %rbx,0x360(%rsp)
   34dcd:	movq   $0xc,0x368(%rsp)
   34dd9:	movq   $0xc,0x370(%rsp)
   34de5:	movq   $0x0,0x378(%rsp)
   34df1:	lea    0x738(%rsp),%rdi
   34df9:	lea    0x350(%rsp),%rcx
   34e01:	mov    %r12,%r8
   34e04:	call   *0xf5d86(%rip)        # 12ab90 <_DYNAMIC+0x5d0>
   34e0a:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34e13:	je     36ea5 <meshopt_peers::run+0x21e5>
   34e19:	mov    0x768(%rsp),%rax
   34e21:	mov    %rax,0x380(%rsp)
   34e29:	movups 0x738(%rsp),%xmm0
   34e31:	movups 0x748(%rsp),%xmm1
   34e39:	movups 0x758(%rsp),%xmm2
   34e41:	movaps %xmm2,0x370(%rsp)
   34e49:	movaps %xmm1,0x360(%rsp)
   34e51:	movaps %xmm0,0x350(%rsp)
   34e59:	mov    0x360(%rsp),%rax
   34e61:	mov    %rax,0x18(%r14)
   34e65:	movups %xmm0,0x8(%r14)
   34e6a:	movups 0x368(%rsp),%xmm0
   34e72:	movups %xmm0,0x20(%r14)
   34e77:	mov    0x378(%rsp),%rax
   34e7f:	mov    %rax,0x30(%r14)
   34e83:	movl   $0x3,(%r14)
   34e8a:	jmp    35a5a <meshopt_peers::run+0xd9a>
   34e8f:	movabs $0x7966696c706d6973,%rdx
   34e99:	xor    (%rsi),%rdx
   34e9c:	movabs $0x7970706f6c732d79,%rdi
   34ea6:	xor    0x7(%rsi),%rdi
   34eaa:	or     %rdx,%rdi
   34ead:	jne    36e1e <meshopt_peers::run+0x215e>
   34eb3:	movdqu 0x20(%rcx),%xmm1
   34eb8:	movq   $0x0,0x1d0(%rsp)
   34ec4:	mov    %rax,0x170(%rsp)
   34ecc:	movd   %xmm0,0x230(%rsp)
   34ed5:	movb   $0x0,0x365(%r12)
   34ede:	movl   $0x0,0x361(%r12)
   34eea:	pxor   %xmm0,%xmm0
   34eee:	movdqu %xmm0,0x350(%r12)
   34ef8:	mov    0x348(%r12),%rax
   34f00:	mov    %rax,0xe0(%rsp)
   34f08:	mov    %rax,0xe8(%rsp)
   34f10:	movdqu %xmm1,0x738(%rsp)
   34f19:	lea    0x6e0(%rsp),%rax
   34f21:	mov    %rax,0x748(%rsp)
   34f29:	lea    0xe0(%rsp),%rax
   34f31:	mov    %rax,0x750(%rsp)
   34f39:	lea    0x1d0(%rsp),%rax
   34f41:	mov    %rax,0x758(%rsp)
   34f49:	lea    0x170(%rsp),%rax
   34f51:	mov    %rax,0x760(%rsp)
   34f59:	lea    0x230(%rsp),%rax
   34f61:	mov    %rax,0x768(%rsp)
   34f69:	mov    %r12,0x770(%rsp)
   34f71:	movq   $0x0,0x778(%rsp)
   34f7d:	lea    0x350(%rsp),%rdi
   34f85:	lea    0x738(%rsp),%rsi
   34f8d:	call   86f20 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   34f92:	mov    0xe8(%rsp),%rax
   34f9a:	sub    0xe0(%rsp),%rax
   34fa2:	mov    %rax,0x358(%r12)
   34faa:	mov    0x350(%rsp),%rax
   34fb2:	movzbl 0x358(%rsp),%ecx
   34fba:	cmp    $0xffffffffffffffff,%rax
   34fbe:	je     34fe2 <meshopt_peers::run+0x322>
   34fc0:	mov    0x368(%rsp),%rdx
   34fc8:	mov    %rdx,0x2d8(%rsp)
   34fd0:	movdqu 0x359(%rsp),%xmm0
   34fd9:	movdqu %xmm0,0x2c9(%rsp)
   34fe2:	mov    %cl,0x2c8(%rsp)
   34fe9:	mov    %rax,0x2c0(%rsp)
   34ff1:	jmp    3545c <meshopt_peers::run+0x79c>
   34ff6:	movdqu (%rsi),%xmm0
   34ffa:	movdqu 0x3(%rsi),%xmm1
   34fff:	pcmpeqb -0x26fb7(%rip),%xmm1        # e050 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   35007:	pcmpeqb -0x2743f(%rip),%xmm0        # dbd0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   3500f:	pand   %xmm1,%xmm0
   35013:	pmovmskb %xmm0,%eax
   35017:	cmp    $0xffff,%eax
   3501c:	jne    36e1e <meshopt_peers::run+0x215e>
   35022:	mov    0x40(%rcx),%rax
   35026:	test   %rbx,%rbx
   35029:	setne  %dl
   3502c:	lea    0x0(,%rbx,4),%rsi
   35034:	cmp    %rax,%rsi
   35037:	seta   %sil
   3503b:	and    %dl,%sil
   3503e:	cmp    $0x1,%sil
   35042:	je     37228 <meshopt_peers::run+0x2568>
   35048:	mov    0x20(%rcx),%rsi
   3504c:	mov    0x28(%rcx),%rdx
   35050:	mov    0x38(%rcx),%rcx
   35054:	movb   $0x0,0x738(%rsp)
   3505c:	mov    %rcx,0x740(%rsp)
   35064:	mov    %rax,0x748(%rsp)
   3506c:	mov    %rbx,0x750(%rsp)
   35074:	movq   $0x4,0x758(%rsp)
   35080:	movq   $0x4,0x760(%rsp)
   3508c:	pxor   %xmm0,%xmm0
   35090:	movdqu %xmm0,0x768(%rsp)
   35099:	sub    $0x30,%rsp
   3509d:	mov    %r12,0x20(%rsp)
   350a2:	lea    0x250(%rsp),%rax
   350aa:	mov    %rax,0x18(%rsp)
   350af:	movq   $0x0,0x8(%rsp)
   350b8:	movq   $0x4,(%rsp)
   350c0:	lea    -0x27027(%rip),%r9        # e0a0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   350c7:	lea    0x2f0(%rsp),%rdi
   350cf:	lea    0x710(%rsp),%rcx
   350d7:	lea    0x768(%rsp),%r8
   350df:	jmp    3544b <meshopt_peers::run+0x78b>
   350e4:	movabs $0x7966696c706d6973,%rax
   350ee:	cmp    %rax,(%rsi)
   350f1:	je     353fe <meshopt_peers::run+0x73e>
   350f7:	movabs $0x776172647265766f,%rax
   35101:	cmp    %rax,(%rsi)
   35104:	je     35920 <meshopt_peers::run+0xc60>
   3510a:	movabs $0x7374656c6873656d,%rax
   35114:	cmp    %rax,(%rsi)
   35117:	jne    36e1e <meshopt_peers::run+0x215e>
   3511d:	mov    0x20(%rcx),%rsi
   35121:	mov    0x28(%rcx),%rdx
   35125:	movq   $0x7c,0x360(%rsp)
   35131:	movq   $0x0,0x354(%rsp)
   3513d:	movl   $0x1,0x350(%rsp)
   35148:	lea    0x350(%rsp),%rax
   35150:	lea    0x738(%rsp),%rdi
   35158:	lea    0x6e0(%rsp),%r8
   35160:	mov    $0x40,%r9d
   35166:	mov    %rbx,%rcx
   35169:	push   $0x2
   3516b:	push   %r12
   3516d:	push   %rax
   3516e:	push   $0x7c
   35170:	call   b5500 <meshoptimizer_rs::meshlet::build>
   35175:	add    $0x20,%rsp
   35179:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35182:	je     36f25 <meshopt_peers::run+0x2265>
   35188:	mov    0x778(%rsp),%rax
   35190:	mov    %rax,0x390(%rsp)
   35198:	movups 0x738(%rsp),%xmm0
   351a0:	movups 0x748(%rsp),%xmm1
   351a8:	movups 0x758(%rsp),%xmm2
   351b0:	movups 0x768(%rsp),%xmm3
   351b8:	movaps %xmm3,0x380(%rsp)
   351c0:	movaps %xmm2,0x370(%rsp)
   351c8:	movaps %xmm1,0x360(%rsp)
   351d0:	movaps %xmm0,0x350(%rsp)
   351d8:	mov    0x358(%rsp),%rbx
   351e0:	mov    0x360(%rsp),%r12
   351e8:	test   %r12,%r12
   351eb:	je     3597d <meshopt_peers::run+0xcbd>
   351f1:	mov    %r12,%r15
   351f4:	shl    $0x4,%r15
   351f8:	mov    %r15,%rdi
   351fb:	call   *0xf55b7(%rip)        # 12a7b8 <malloc@GLIBC_2.2.5>
   35201:	test   %rax,%rax
   35204:	je     372c4 <meshopt_peers::run+0x2604>
   3520a:	mov    %r12d,%ecx
   3520d:	and    $0x3,%ecx
   35210:	cmp    $0x4,%r12
   35214:	jae    35984 <meshopt_peers::run+0xcc4>
   3521a:	xor    %edx,%edx
   3521c:	jmp    359d8 <meshopt_peers::run+0xd18>
   35221:	movabs $0x65646f6365642d78,%rax
   3522b:	movabs $0x652d786574726576,%rdx
   35235:	xor    (%rsi),%rdx
   35238:	lea    0x9010000(%rax),%rdi
   3523f:	xor    0x5(%rsi),%rdi
   35243:	or     %rdx,%rdi
   35246:	je     3549a <meshopt_peers::run+0x7da>
   3524c:	movabs $0x642d786574726576,%rdx
   35256:	xor    (%rsi),%rdx
   35259:	xor    0x5(%rsi),%rax
   3525d:	or     %rdx,%rax
   35260:	jne    36e1e <meshopt_peers::run+0x215e>
   35266:	mov    0x50(%rcx),%r9
   3526a:	lea    0x738(%rsp),%rdi
   35272:	mov    $0xc,%r8d
   35278:	xor    %esi,%esi
   3527a:	xor    %edx,%edx
   3527c:	mov    %rcx,%rax
   3527f:	mov    %rbx,%rcx
   35282:	push   %r12
   35284:	push   0x58(%rax)
   35287:	call   b4a20 <meshoptimizer_rs::codec::allocate>
   3528c:	add    $0x10,%rsp
   35290:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35299:	jne    3553d <meshopt_peers::run+0x87d>
   3529f:	movzbl 0x740(%rsp),%eax
   352a7:	mov    %al,0x350(%rsp)
   352ae:	lea    -0x24d02(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   352b5:	lea    0xee444(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   352bc:	lea    0xee10d(%rip),%r8        # 1233d0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   352c3:	lea    0x350(%rsp),%rdx
   352cb:	mov    $0x2b,%esi
   352d0:	call   *0xf55ba(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   352d6:	movabs $0x6e652d7865646e69,%rax
   352e0:	xor    (%rsi),%rax
   352e3:	mov    0x8(%rsi),%edx
   352e6:	xor    $0x65646f63,%rdx
   352ed:	or     %rax,%rdx
   352f0:	je     35510 <meshopt_peers::run+0x850>
   352f6:	movabs $0x65642d7865646e69,%rax
   35300:	xor    (%rsi),%rax
   35303:	mov    0x8(%rsi),%edx
   35306:	xor    $0x65646f63,%rdx
   3530d:	or     %rax,%rdx
   35310:	jne    36e1e <meshopt_peers::run+0x215e>
   35316:	mov    0x28(%rcx),%rax
   3531a:	mov    0x68(%rcx),%r9
   3531e:	lea    0x738(%rsp),%rdi
   35326:	mov    $0x4,%r8d
   3532c:	mov    $0x1,%esi
   35331:	xor    %edx,%edx
   35333:	mov    %rcx,%r10
   35336:	mov    %rax,%rcx
   35339:	push   %r12
   3533b:	push   0x70(%r10)
   3533f:	call   b4a20 <meshoptimizer_rs::codec::allocate>
   35344:	add    $0x10,%rsp
   35348:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35351:	jne    3553d <meshopt_peers::run+0x87d>
   35357:	movzbl 0x740(%rsp),%eax
   3535f:	mov    %al,0x350(%rsp)
   35366:	lea    -0x24dba(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3536d:	lea    0xee38c(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   35374:	lea    0xee025(%rip),%r8        # 1233a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x138>
   3537b:	lea    0x350(%rsp),%rdx
   35383:	mov    $0x2b,%esi
   35388:	call   *0xf5502(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   3538e:	movabs $0x2d72657473756c63,%rax
   35398:	xor    (%rsi),%rax
   3539b:	movabs $0x646f6c2d72657473,%rdx
   353a5:	xor    0x3(%rsi),%rdx
   353a9:	or     %rax,%rdx
   353ac:	jne    36e1e <meshopt_peers::run+0x215e>
   353b2:	mov    %r14,0x288(%rsp)
   353ba:	test   %rbx,%rbx
   353bd:	je     355c4 <meshopt_peers::run+0x904>
   353c3:	mov    %rcx,%rbp
   353c6:	lea    0x0(,%rbx,4),%rax
   353ce:	lea    (%rax,%rax,2),%r13
   353d2:	mov    %r13,%rdi
   353d5:	call   *0xf53dd(%rip)        # 12a7b8 <malloc@GLIBC_2.2.5>
   353db:	test   %rax,%rax
   353de:	je     372a1 <meshopt_peers::run+0x25e1>
   353e4:	mov    %rax,%r14
   353e7:	mov    %rax,%rdi
   353ea:	mov    %r15,%rsi
   353ed:	mov    %r13,%rdx
   353f0:	call   *0xf53e2(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   353f6:	mov    %rbp,%rcx
   353f9:	jmp    355ca <meshopt_peers::run+0x90a>
   353fe:	mov    0x20(%rcx),%rsi
   35402:	mov    0x28(%rcx),%rdx
   35406:	movb   $0x2,0x738(%rsp)
   3540e:	sub    $0x30,%rsp
   35412:	mov    %r12,0x20(%rsp)
   35417:	lea    0x250(%rsp),%rax
   3541f:	mov    %rax,0x18(%rsp)
   35424:	pxor   %xmm0,%xmm0
   35428:	movdqu %xmm0,(%rsp)
   3542d:	lea    0x2f0(%rsp),%rdi
   35435:	lea    0x710(%rsp),%rcx
   3543d:	lea    0x768(%rsp),%r8
   35445:	mov    $0x4,%r9d
   3544b:	call   c01a0 <meshoptimizer_rs::simplify::allocating>
   35450:	add    $0x30,%rsp
   35454:	mov    0x2c0(%rsp),%rax
   3545c:	cmp    $0xffffffffffffffff,%rax
   35460:	je     36de7 <meshopt_peers::run+0x2127>
   35466:	mov    0x2d0(%rsp),%rax
   3546e:	mov    %rax,0x18(%r14)
   35472:	movups 0x2c0(%rsp),%xmm0
   3547a:	movups %xmm0,0x8(%r14)
   3547f:	movss  0x2d8(%rsp),%xmm0
   35488:	movss  %xmm0,0x4(%r14)
   3548e:	movl   $0x2,(%r14)
   35495:	jmp    35a5a <meshopt_peers::run+0xd9a>
   3549a:	lea    0x0(,%rbx,4),%rax
   354a2:	lea    (%rax,%rax,2),%rdx
   354a6:	lea    0x738(%rsp),%rdi
   354ae:	mov    $0xc,%r8d
   354b4:	mov    %r15,%rsi
   354b7:	mov    %rbx,%rcx
   354ba:	mov    $0x1,%r9d
   354c0:	push   %r12
   354c2:	push   $0x2
   354c4:	call   *0xf56ce(%rip)        # 12ab98 <_DYNAMIC+0x5d8>
   354ca:	add    $0x10,%rsp
   354ce:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   354d7:	jne    3553d <meshopt_peers::run+0x87d>
   354d9:	movzbl 0x740(%rsp),%eax
   354e1:	mov    %al,0x350(%rsp)
   354e8:	lea    -0x24f3c(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   354ef:	lea    0xee20a(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   354f6:	lea    0xedeeb(%rip),%r8        # 1233e8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   354fd:	lea    0x350(%rsp),%rdx
   35505:	mov    $0x2b,%esi
   3550a:	call   *0xf5380(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   35510:	mov    0x20(%rcx),%rsi
   35514:	mov    0x28(%rcx),%rdx
   35518:	lea    0x738(%rsp),%rdi
   35520:	mov    $0x1,%ecx
   35525:	mov    %r12,%r8
   35528:	call   *0xf5672(%rip)        # 12aba0 <_DYNAMIC+0x5e0>
   3552e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35537:	je     36f5c <meshopt_peers::run+0x229c>
   3553d:	mov    0x748(%rsp),%rax
   35545:	mov    %rax,0x18(%r14)
   35549:	movups 0x738(%rsp),%xmm0
   35551:	movups %xmm0,0x8(%r14)
   35556:	movl   $0x0,(%r14)
   3555d:	jmp    35a5a <meshopt_peers::run+0xd9a>
   35562:	mov    0x20(%rcx),%rsi
   35566:	mov    0x28(%rcx),%rdx
   3556a:	lea    0x738(%rsp),%rdi
   35572:	mov    %rbx,%rcx
   35575:	mov    %r12,%r8
   35578:	call   *0xf562a(%rip)        # 12aba8 <_DYNAMIC+0x5e8>
   3557e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35587:	jne    35958 <meshopt_peers::run+0xc98>
   3558d:	movzbl 0x740(%rsp),%eax
   35595:	mov    %al,0x350(%rsp)
   3559c:	lea    -0x24ff0(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   355a3:	lea    0xee156(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   355aa:	lea    0xedeaf(%rip),%r8        # 123460 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   355b1:	lea    0x350(%rsp),%rdx
   355b9:	mov    $0x2b,%esi
   355be:	call   *0xf52cc(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   355c4:	mov    $0x4,%r14d
   355ca:	movq   $0x0,0x158(%rsp)
   355d6:	movq   $0x4,0x160(%rsp)
   355e2:	movq   $0x0,0x168(%rsp)
   355ee:	movl   $0x0,0x9c(%rsp)
   355f9:	movq   $0x80,0x1d0(%rsp)
   35605:	movq   $0x2a,0x1d8(%rsp)
   35611:	movq   $0x80,0x1e0(%rsp)
   3561d:	movq   $0x10,0x1e8(%rsp)
   35629:	movaps -0x27740(%rip),%xmm0        # def0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   35630:	movups %xmm0,0x1f0(%rsp)
   35638:	movaps -0x2768f(%rip),%xmm0        # dfb0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3563f:	movups %xmm0,0x200(%rsp)
   35647:	movabs $0x100010100000101,%rax
   35651:	mov    %rax,0x210(%rsp)
   35659:	movl   $0x0,0x218(%rsp)
   35664:	movb   $0x1,0x21c(%rsp)
   3566c:	mov    0x20(%rcx),%r13
   35670:	mov    0x28(%rcx),%r15
   35674:	mov    %r13,0x308(%rsp)
   3567c:	mov    %r15,0x310(%rsp)
   35684:	mov    %r14,0x50(%rsp)
   35689:	mov    %r14,0x318(%rsp)
   35691:	mov    %rbx,0x20(%rsp)
   35696:	mov    %rbx,0x320(%rsp)
   3569e:	movb   $0x2,0x2c0(%rsp)
   356a6:	movq   $0x4,0x328(%rsp)
   356b2:	xorps  %xmm0,%xmm0
   356b5:	movups %xmm0,0x330(%rsp)
   356bd:	movl   $0x0,0x348(%rsp)
   356c8:	movb   $0x0,0x365(%r12)
   356d1:	movl   $0x0,0x361(%r12)
   356dd:	movups %xmm0,0x350(%r12)
   356e6:	mov    0x348(%r12),%rbx
   356ee:	mov    %r12,%rdi
   356f1:	xor    %esi,%esi
   356f3:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   356f8:	mov    %eax,%ebp
   356fa:	cmp    $0xff,%al
   356fc:	jne    37375 <meshopt_peers::run+0x26b5>
   35702:	mov    %r12,0x738(%rsp)
   3570a:	mov    %rbx,0x740(%rsp)
   35712:	mov    %rbx,0x748(%rsp)
   3571a:	movq   $0x0,0x750(%rsp)
   35726:	movl   $0x0,0x758(%rsp)
   35731:	movl   $0x0,0x95c(%rsp)
   3573c:	movb   $0x1,0x1960(%rsp)
   35744:	lea    0x2c0(%rsp),%rdi
   3574c:	lea    0x1d0(%rsp),%rsi
   35754:	lea    0x738(%rsp),%rdx
   3575c:	mov    0x20(%rsp),%rbx
   35761:	call   *0xf5449(%rip)        # 12abb0 <_DYNAMIC+0x5f0>
   35767:	mov    %eax,%ebp
   35769:	cmp    $0xff,%al
   3576b:	jne    37356 <meshopt_peers::run+0x2696>
   35771:	movzbl 0x1960(%rsp),%r12d
   3577a:	movb   $0x2,0x350(%rsp)
   35782:	mov    0x50(%rsp),%rax
   35787:	mov    %rax,0x358(%rsp)
   3578f:	mov    %rbx,0x360(%rsp)
   35797:	movq   $0x0,0x388(%rsp)
   357a3:	lea    0xe0(%rsp),%rdi
   357ab:	lea    0x350(%rsp),%rsi
   357b3:	lea    0x738(%rsp),%rdx
   357bb:	call   *0xf53f7(%rip)        # 12abb8 <_DYNAMIC+0x5f8>
   357c1:	mov    0xe0(%rsp),%r14
   357c9:	movzbl 0xe8(%rsp),%ebp
   357d1:	cmp    $0xffffffffffffffff,%r14
   357d5:	je     358be <meshopt_peers::run+0xbfe>
   357db:	mov    %r14,0x150(%rsp)
   357e3:	movzbl %bpl,%eax
   357e7:	movzbl 0xef(%rsp),%ecx
   357ef:	shl    $0x10,%ecx
   357f2:	movzwl 0xed(%rsp),%edx
   357fa:	or     %ecx,%edx
   357fc:	mov    0xe9(%rsp),%ecx
   35803:	shl    $0x28,%rdx
   35807:	shl    $0x8,%rcx
   3580b:	or     %rdx,%rcx
   3580e:	or     %rax,%rcx
   35811:	mov    %rcx,0xc8(%rsp)
   35819:	mov    0x750(%rsp),%rsi
   35821:	add    %rbx,%rsi
   35824:	jb     36edc <meshopt_peers::run+0x221c>
   3582a:	mov    0xf0(%rsp),%rax
   35832:	mov    %rax,0x1c8(%rsp)
   3583a:	mov    %rsi,0x750(%rsp)
   35842:	mov    0x738(%rsp),%rdi
   3584a:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3584f:	mov    %eax,%ebp
   35851:	cmp    $0xff,%al
   35853:	jne    358a5 <meshopt_peers::run+0xbe5>
   35855:	mov    $0x1,%eax
   3585a:	mov    %rax,0x90(%rsp)
   35862:	test   %rbx,%rbx
   35865:	jne    35ec3 <meshopt_peers::run+0x1203>
   3586b:	mov    0x738(%rsp),%rdi
   35873:	mov    0x750(%rsp),%rsi
   3587b:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35880:	mov    %eax,%ebp
   35882:	mov    $0x1,%r14b
   35885:	cmp    $0xff,%al
   35887:	je     35a6c <meshopt_peers::run+0xdac>
   3588d:	test   %rbx,%rbx
   35890:	je     358a0 <meshopt_peers::run+0xbe0>
   35892:	mov    0x90(%rsp),%rdi
   3589a:	call   *0xf4f28(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   358a0:	test   %r14b,%r14b
   358a3:	je     358be <meshopt_peers::run+0xbfe>
   358a5:	cmpq   $0x0,0x150(%rsp)
   358ae:	je     358be <meshopt_peers::run+0xbfe>
   358b0:	mov    0xc8(%rsp),%rdi
   358b8:	call   *0xf4f0a(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   358be:	mov    0x738(%rsp),%rax
   358c6:	mov    0x748(%rsp),%rcx
   358ce:	sub    0x740(%rsp),%rcx
   358d6:	mov    %rcx,0x358(%rax)
   358dd:	cmp    $0xff,%bpl
   358e1:	jne    37375 <meshopt_peers::run+0x26b5>
   358e7:	mov    0x168(%rsp),%rax
   358ef:	mov    0x288(%rsp),%rcx
   358f7:	mov    %rax,0x18(%rcx)
   358fb:	movups 0x158(%rsp),%xmm0
   35903:	movups %xmm0,0x8(%rcx)
   35907:	movl   $0x1,(%rcx)
   3590d:	test   %rbx,%rbx
   35910:	je     35a5a <meshopt_peers::run+0xd9a>
   35916:	mov    0x50(%rsp),%rdi
   3591b:	jmp    35a54 <meshopt_peers::run+0xd94>
   35920:	mov    0x20(%rcx),%rsi
   35924:	mov    0x28(%rcx),%rdx
   35928:	lea    0x738(%rsp),%rdi
   35930:	lea    0x6e0(%rsp),%rcx
   35938:	movd   -0x25cdc(%rip),%xmm0        # fc64 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x138>
   35940:	mov    %r12,%r8
   35943:	call   *0xf5277(%rip)        # 12abc0 <_DYNAMIC+0x600>
   35949:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35952:	je     36ffc <meshopt_peers::run+0x233c>
   35958:	mov    0x748(%rsp),%rax
   35960:	mov    %rax,0x18(%r14)
   35964:	movups 0x738(%rsp),%xmm0
   3596c:	movups %xmm0,0x8(%r14)
   35971:	movl   $0x1,(%r14)
   35978:	jmp    35a5a <meshopt_peers::run+0xd9a>
   3597d:	mov    $0x4,%eax
   35982:	jmp    35a01 <meshopt_peers::run+0xd41>
   35984:	mov    %r12,%rsi
   35987:	and    $0xfffffffffffffffc,%rsi
   3598b:	mov    $0x30,%edi
   35990:	xor    %edx,%edx
   35992:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   359a0:	movups -0x30(%rbx,%rdi,1),%xmm0
   359a5:	movups %xmm0,-0x30(%rax,%rdi,1)
   359aa:	movups -0x20(%rbx,%rdi,1),%xmm0
   359af:	movups %xmm0,-0x20(%rax,%rdi,1)
   359b4:	movups -0x10(%rbx,%rdi,1),%xmm0
   359b9:	movups %xmm0,-0x10(%rax,%rdi,1)
   359be:	movups (%rbx,%rdi,1),%xmm0
   359c2:	movups %xmm0,(%rax,%rdi,1)
   359c6:	add    $0x4,%rdx
   359ca:	add    $0x40,%rdi
   359ce:	cmp    %rdx,%rsi
   359d1:	jne    359a0 <meshopt_peers::run+0xce0>
   359d3:	test   %rcx,%rcx
   359d6:	je     35a01 <meshopt_peers::run+0xd41>
   359d8:	shl    $0x4,%rdx
   359dc:	mov    %rax,%rsi
   359df:	add    %rdx,%rsi
   359e2:	add    %rbx,%rdx
   359e5:	shl    $0x4,%ecx
   359e8:	xor    %edi,%edi
   359ea:	nopw   0x0(%rax,%rax,1)
   359f0:	movups (%rdx,%rdi,1),%xmm0
   359f4:	movups %xmm0,(%rsi,%rdi,1)
   359f8:	add    $0x10,%rdi
   359fc:	cmp    %rdi,%rcx
   359ff:	jne    359f0 <meshopt_peers::run+0xd30>
   35a01:	mov    0x378(%rsp),%rcx
   35a09:	mov    %rcx,0x30(%r14)
   35a0d:	movups 0x368(%rsp),%xmm0
   35a15:	movups %xmm0,0x20(%r14)
   35a1a:	movaps 0x380(%rsp),%xmm0
   35a22:	movups %xmm0,0x38(%r14)
   35a27:	mov    0x390(%rsp),%rcx
   35a2f:	mov    %rcx,0x48(%r14)
   35a33:	mov    %r12,0x8(%r14)
   35a37:	mov    %rax,0x10(%r14)
   35a3b:	mov    %r12,0x18(%r14)
   35a3f:	movl   $0x4,(%r14)
   35a46:	cmpq   $0x0,0x350(%rsp)
   35a4f:	je     35a5a <meshopt_peers::run+0xd9a>
   35a51:	mov    %rbx,%rdi
   35a54:	call   *0xf4d6e(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   35a5a:	add    $0x1968,%rsp
   35a61:	pop    %rbx
   35a62:	pop    %r12
   35a64:	pop    %r13
   35a66:	pop    %r14
   35a68:	pop    %r15
   35a6a:	pop    %rbp
   35a6b:	ret
   35a6c:	movb   $0x2,0xe0(%rsp)
   35a74:	mov    0x50(%rsp),%rax
   35a79:	mov    %rax,0xe8(%rsp)
   35a81:	mov    %rbx,0xf0(%rsp)
   35a89:	movq   $0x0,0x118(%rsp)
   35a95:	movups 0x210(%rsp),%xmm0
   35a9d:	movaps %xmm0,0x390(%rsp)
   35aa5:	movups 0x1d0(%rsp),%xmm0
   35aad:	movups 0x1e0(%rsp),%xmm1
   35ab5:	movups 0x1f0(%rsp),%xmm2
   35abd:	movups 0x200(%rsp),%xmm3
   35ac5:	movaps %xmm3,0x380(%rsp)
   35acd:	movaps %xmm2,0x370(%rsp)
   35ad5:	movaps %xmm1,0x360(%rsp)
   35add:	movaps %xmm0,0x350(%rsp)
   35ae5:	movzbl %r12b,%r10d
   35ae9:	lea    0x170(%rsp),%rdi
   35af1:	lea    0xe0(%rsp),%rcx
   35af9:	lea    0x350(%rsp),%r8
   35b01:	mov    %r13,%rsi
   35b04:	mov    %r15,%rdx
   35b07:	mov    $0xffffffff,%r9d
   35b0d:	lea    0x738(%rsp),%rax
   35b15:	push   %rax
   35b16:	mov    %r10,0x1c8(%rsp)
   35b1e:	push   %r10
   35b20:	call   *0xf50a2(%rip)        # 12abc8 <_DYNAMIC+0x608>
   35b26:	add    $0x10,%rsp
   35b2a:	mov    0x170(%rsp),%rdi
   35b32:	movzbl 0x178(%rsp),%ebp
   35b3a:	cmp    $0xffffffffffffffff,%rdi
   35b3e:	je     3588d <meshopt_peers::run+0xbcd>
   35b44:	movzbl %bpl,%eax
   35b48:	movzbl 0x17f(%rsp),%ecx
   35b50:	shl    $0x10,%ecx
   35b53:	movzwl 0x17d(%rsp),%edx
   35b5b:	or     %ecx,%edx
   35b5d:	mov    0x179(%rsp),%ecx
   35b64:	mov    0x180(%rsp),%r9
   35b6c:	mov    0x188(%rsp),%r8
   35b74:	mov    0x190(%rsp),%rsi
   35b7c:	mov    %rsi,0x28(%rsp)
   35b81:	shl    $0x28,%rdx
   35b85:	shl    $0x8,%rcx
   35b89:	or     %rdx,%rcx
   35b8c:	or     %rax,%rcx
   35b8f:	mov    %rcx,0x38(%rsp)
   35b94:	test   %r9,%r9
   35b97:	je     35e87 <meshopt_peers::run+0x11c7>
   35b9d:	mov    %r8,0x70(%rsp)
   35ba2:	mov    %rdi,0x10(%rsp)
   35ba7:	mov    0x198(%rsp),%r14
   35baf:	mov    %r9,0xd8(%rsp)
   35bb7:	mov    %r9,%rax
   35bba:	shl    $0x4,%rax
   35bbe:	lea    (%rax,%rax,2),%rax
   35bc2:	mov    %rax,0x18(%rsp)
   35bc7:	xor    %r12d,%r12d
   35bca:	mov    0x738(%rsp),%r15
   35bd2:	mov    0x740(%rsp),%rbp
   35bda:	mov    0x748(%rsp),%rax
   35be2:	sub    %rbp,%rax
   35be5:	mov    %rax,0x358(%r15)
   35bec:	mov    %r15,%rdi
   35bef:	call   *0xf4f83(%rip)        # 12ab78 <_DYNAMIC+0x5b8>
   35bf5:	mov    0x340(%r15),%rax
   35bfc:	mov    0x750(%rsp),%r13
   35c04:	sub    %r13,%rax
   35c07:	jb     35efc <meshopt_peers::run+0x123c>
   35c0d:	mov    0x348(%r15),%rcx
   35c14:	sub    0x358(%r15),%rcx
   35c1b:	mov    %rax,0x690(%rsp)
   35c23:	mov    %rcx,0x698(%rsp)
   35c2b:	movb   $0x0,0x6b0(%rsp)
   35c33:	movb   $0x0,0x6b5(%rsp)
   35c3b:	movl   $0x0,0x6b1(%rsp)
   35c46:	movb   $0xff,0x6b8(%rsp)
   35c4e:	movw   $0x0,0x6b6(%rsp)
   35c58:	lea    0x6a0(%rsp),%rax
   35c60:	xorps  %xmm0,%xmm0
   35c63:	movups %xmm0,(%rax)
   35c66:	movq   $0xffffffffffffffff,0x3c8(%rsp)
   35c72:	movq   $0xffffffffffffffff,0x5d0(%rsp)
   35c7e:	movq   $0x0,0x350(%rsp)
   35c8a:	movq   $0x4,0x358(%rsp)
   35c96:	movups %xmm0,-0x340(%rax)
   35c9d:	movq   $0x4,0x370(%rsp)
   35ca9:	movups %xmm0,-0x328(%rax)
   35cb0:	movq   $0x4,0x388(%rsp)
   35cbc:	movups %xmm0,-0x310(%rax)
   35cc3:	movq   $0x1,0x3a0(%rsp)
   35ccf:	movups %xmm0,-0x2f8(%rax)
   35cd6:	movq   $0x2,0x3b8(%rsp)
   35ce2:	movq   $0x0,0x3c0(%rsp)
   35cee:	mov    0x38(%rsp),%rax
   35cf3:	mov    0x8(%rax,%r12,1),%rdi
   35cf8:	mov    0x10(%rax,%r12,1),%rdx
   35cfd:	mov    %rdx,%rsi
   35d00:	add    %rdi,%rsi
   35d03:	jb     36f10 <meshopt_peers::run+0x2250>
   35d09:	cmp    %r14,%rsi
   35d0c:	ja     36f10 <meshopt_peers::run+0x2250>
   35d12:	mov    0x28(%rsp),%rax
   35d17:	lea    (%rax,%rdi,4),%rsi
   35d1b:	movb   $0x2,0xe0(%rsp)
   35d23:	mov    0x50(%rsp),%rax
   35d28:	mov    %rax,0xe8(%rsp)
   35d30:	mov    %rbx,0xf0(%rsp)
   35d38:	movq   $0x0,0x118(%rsp)
   35d44:	lea    0x170(%rsp),%rdi
   35d4c:	lea    0xe0(%rsp),%rcx
   35d54:	mov    0x1c0(%rsp),%r8
   35d5c:	lea    0x350(%rsp),%r9
   35d64:	call   *0xf4e66(%rip)        # 12abd0 <_DYNAMIC+0x610>
   35d6a:	mov    0x6a8(%rsp),%rsi
   35d72:	sub    %rsi,%rbp
   35d75:	jb     35e6a <meshopt_peers::run+0x11aa>
   35d7b:	mov    %rbp,0x740(%rsp)
   35d83:	add    0x6a0(%rsp),%r13
   35d8b:	jb     36f93 <meshopt_peers::run+0x22d3>
   35d91:	mov    %r15,%rdi
   35d94:	mov    %r13,%rsi
   35d97:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35d9c:	mov    %eax,%ebp
   35d9e:	cmp    $0xff,%al
   35da0:	jne    36f96 <meshopt_peers::run+0x22d6>
   35da6:	movzbl 0x170(%rsp),%ebx
   35dae:	movzbl 0x171(%rsp),%ebp
   35db6:	lea    0x172(%rsp),%rcx
   35dbe:	mov    (%rcx),%rax
   35dc1:	mov    0x6(%rcx),%rcx
   35dc5:	mov    %rax,0x230(%rsp)
   35dcd:	mov    %rcx,0x236(%rsp)
   35dd5:	movss  0x180(%rsp),%xmm0
   35dde:	movaps %xmm0,0x80(%rsp)
   35de6:	lea    0x350(%rsp),%rdi
   35dee:	call   33ea0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35df3:	cmp    $0x1,%bl
   35df6:	mov    0x20(%rsp),%rbx
   35dfb:	je     36fa3 <meshopt_peers::run+0x22e3>
   35e01:	lea    0x232(%rsp),%rcx
   35e09:	mov    0x8(%rcx),%eax
   35e0c:	lea    0x72c(%rsp),%rdx
   35e14:	mov    %eax,0x8(%rdx)
   35e17:	mov    (%rcx),%rax
   35e1a:	mov    %rax,(%rdx)
   35e1d:	mov    0x8(%rdx),%eax
   35e20:	mov    %eax,0x358(%rsp)
   35e27:	mov    (%rdx),%rcx
   35e2a:	mov    %rcx,0x350(%rsp)
   35e32:	mov    0x38(%rsp),%rdx
   35e37:	mov    %eax,0x24(%rdx,%r12,1)
   35e3c:	mov    %rcx,0x1c(%rdx,%r12,1)
   35e41:	xorps  %xmm0,%xmm0
   35e44:	movaps 0x80(%rsp),%xmm1
   35e4c:	movss  %xmm1,%xmm0
   35e50:	movlps %xmm0,0x28(%rdx,%r12,1)
   35e56:	add    $0x30,%r12
   35e5a:	cmp    %r12,0x18(%rsp)
   35e5f:	jne    35bca <meshopt_peers::run+0xf0a>
   35e65:	jmp    35f2e <meshopt_peers::run+0x126e>
   35e6a:	lea    0x740(%rsp),%rdi
   35e72:	call   *0xf4d10(%rip)        # 12ab88 <_DYNAMIC+0x5c8>
   35e78:	mov    %eax,%ebp
   35e7a:	cmp    $0xff,%al
   35e7c:	je     35d83 <meshopt_peers::run+0x10c3>
   35e82:	jmp    36f96 <meshopt_peers::run+0x22d6>
   35e87:	mov    %rdi,%r12
   35e8a:	mov    %r8,0x80(%rsp)
   35e92:	lea    0x738(%rsp),%rdi
   35e9a:	mov    0x150(%rsp),%rsi
   35ea2:	mov    0xc8(%rsp),%rdx
   35eaa:	call   337e0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35eaf:	mov    $0x1,%r13b
   35eb2:	cmp    $0xff,%al
   35eb4:	je     35f04 <meshopt_peers::run+0x1244>
   35eb6:	mov    %eax,%ebp
   35eb8:	xor    %r14d,%r14d
   35ebb:	mov    $0x1,%r15b
   35ebe:	jmp    36034 <meshopt_peers::run+0x1374>
   35ec3:	lea    0x350(%rsp),%rdi
   35ecb:	mov    $0x1,%edx
   35ed0:	mov    $0x1,%r8d
   35ed6:	mov    $0x1,%r9d
   35edc:	xor    %esi,%esi
   35ede:	mov    %rbx,%rcx
   35ee1:	call   3b5d0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35ee6:	cmpl   $0x1,0x350(%rsp)
   35eee:	jne    35fd5 <meshopt_peers::run+0x1315>
   35ef4:	mov    $0xa,%bpl
   35ef7:	jmp    358a5 <meshopt_peers::run+0xbe5>
   35efc:	mov    $0x9,%bpl
   35eff:	jmp    36fa3 <meshopt_peers::run+0x22e3>
   35f04:	mov    0x750(%rsp),%rax
   35f0c:	sub    %rbx,%rax
   35f0f:	jae    35ff5 <meshopt_peers::run+0x1335>
   35f15:	mov    0x90(%rsp),%rdi
   35f1d:	call   *0xf48a5(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   35f23:	mov    $0x8,%bpl
   35f26:	xor    %r14d,%r14d
   35f29:	jmp    36031 <meshopt_peers::run+0x1371>
   35f2e:	mov    0xd8(%rsp),%rdx
   35f36:	xor    %r13d,%r13d
   35f39:	cmp    $0x1,%rdx
   35f3d:	jne    36041 <meshopt_peers::run+0x1381>
   35f43:	mov    0x10(%rsp),%r12
   35f48:	mov    0x70(%rsp),%rax
   35f4d:	mov    %rax,0x80(%rsp)
   35f55:	mov    0x38(%rsp),%rdi
   35f5a:	movups 0x1c(%rdi),%xmm0
   35f5e:	movaps %xmm0,0x2a0(%rsp)
   35f66:	movl   $0x7f7fffff,0x2b0(%rsp)
   35f71:	lea    0x9c(%rsp),%rax
   35f79:	lea    0x158(%rsp),%r10
   35f81:	lea    0x2a0(%rsp),%r11
   35f89:	mov    $0x1,%esi
   35f8e:	mov    0x28(%rsp),%rdx
   35f93:	mov    %r14,%rcx
   35f96:	mov    0x50(%rsp),%r8
   35f9b:	mov    %rbx,%r9
   35f9e:	lea    0x738(%rsp),%r15
   35fa6:	push   %r15
   35fa8:	push   %rax
   35fa9:	push   %r10
   35fab:	push   %r13
   35fad:	push   %r11
   35faf:	push   $0x0
   35fb1:	call   33f30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   35fb6:	add    $0x30,%rsp
   35fba:	mov    %rax,%rbp
   35fbd:	test   $0x1,%bpl
   35fc1:	je     35e92 <meshopt_peers::run+0x11d2>
   35fc7:	shr    $0x8,%ebp
   35fca:	mov    $0x1,%r14b
   35fcd:	mov    $0x1,%r15b
   35fd0:	mov    $0x1,%r13b
   35fd3:	jmp    36034 <meshopt_peers::run+0x1374>
   35fd5:	mov    0x358(%rsp),%rdi
   35fdd:	mov    %rdi,0x90(%rsp)
   35fe5:	xor    %esi,%esi
   35fe7:	mov    %rbx,%rdx
   35fea:	call   *0xf47f8(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   35ff0:	jmp    3586b <meshopt_peers::run+0xbab>
   35ff5:	mov    %rax,0x750(%rsp)
   35ffd:	test   %rbx,%rbx
   36000:	je     36010 <meshopt_peers::run+0x1350>
   36002:	mov    0x90(%rsp),%rdi
   3600a:	call   *0xf47b8(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36010:	xor    %r14d,%r14d
   36013:	lea    0x738(%rsp),%rdi
   3601b:	mov    $0x4,%edx
   36020:	xor    %esi,%esi
   36022:	call   33740 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   36027:	cmp    $0xff,%al
   36029:	je     36da9 <meshopt_peers::run+0x20e9>
   3602f:	mov    %eax,%ebp
   36031:	xor    %r15d,%r15d
   36034:	mov    0x80(%rsp),%rax
   3603c:	jmp    36fb6 <meshopt_peers::run+0x22f6>
   36041:	lea    0x170(%rsp),%rdi
   36049:	mov    0xf4b88(%rip),%r10        # 12abd8 <_DYNAMIC+0x618>
   36050:	mov    %r14,%r8
   36053:	movb   $0x2,0xe0(%rsp)
   3605b:	mov    0x50(%rsp),%rax
   36060:	mov    %rax,0xe8(%rsp)
   36068:	mov    %rbx,0xf0(%rsp)
   36070:	movq   $0x0,0x118(%rsp)
   3607c:	movups 0x210(%rsp),%xmm0
   36084:	movaps %xmm0,0x390(%rsp)
   3608c:	movups 0x1d0(%rsp),%xmm0
   36094:	movups 0x1e0(%rsp),%xmm1
   3609c:	movups 0x1f0(%rsp),%xmm2
   360a4:	movups 0x200(%rsp),%xmm3
   360ac:	movaps %xmm3,0x380(%rsp)
   360b4:	movaps %xmm2,0x370(%rsp)
   360bc:	movaps %xmm1,0x360(%rsp)
   360c4:	movaps %xmm0,0x350(%rsp)
   360cc:	mov    0x38(%rsp),%rsi
   360d1:	mov    %rdx,0xd8(%rsp)
   360d9:	mov    0x28(%rsp),%rcx
   360de:	mov    %r8,0x18(%rsp)
   360e3:	mov    0xc8(%rsp),%r9
   360eb:	lea    0x738(%rsp),%r12
   360f3:	push   %r12
   360f5:	lea    0x358(%rsp),%rax
   360fd:	push   %rax
   360fe:	lea    0xf0(%rsp),%rax
   36106:	push   %rax
   36107:	push   0x1e0(%rsp)
   3610e:	call   *%r10
   36111:	add    $0x20,%rsp
   36115:	mov    %r13,0x148(%rsp)
   3611d:	mov    0x170(%rsp),%r15
   36125:	movzbl 0x178(%rsp),%ebp
   3612d:	cmp    $0xffffffffffffffff,%r15
   36131:	je     36fa3 <meshopt_peers::run+0x22e3>
   36137:	movzbl %bpl,%eax
   3613b:	movzbl 0x17f(%rsp),%ecx
   36143:	shl    $0x10,%ecx
   36146:	movzwl 0x17d(%rsp),%edx
   3614e:	or     %ecx,%edx
   36150:	mov    0x179(%rsp),%r8d
   36158:	mov    0x180(%rsp),%r14
   36160:	shl    $0x28,%rdx
   36164:	shl    $0x8,%r8
   36168:	or     %rdx,%r8
   3616b:	or     %rax,%r8
   3616e:	sub    $0x40,%rsp
   36172:	mov    %r12,0x30(%rsp)
   36177:	mov    0x208(%rsp),%rax
   3617f:	mov    %rax,0x18(%rsp)
   36184:	mov    0x108(%rsp),%rax
   3618c:	mov    %rax,0x10(%rsp)
   36191:	mov    0x58(%rsp),%rax
   36196:	mov    %rax,0x8(%rsp)
   3619b:	mov    0x68(%rsp),%rax
   361a0:	mov    %rax,(%rsp)
   361a4:	movq   $0x0,0x20(%rsp)
   361ad:	mov    0xd0(%rsp),%rdi
   361b5:	mov    %rbx,%rsi
   361b8:	mov    0x78(%rsp),%rdx
   361bd:	mov    0x118(%rsp),%rcx
   361c5:	mov    %r8,0x110(%rsp)
   361cd:	mov    %r14,%r9
   361d0:	call   *0xf4a0a(%rip)        # 12abe0 <_DYNAMIC+0x620>
   361d6:	add    $0x40,%rsp
   361da:	mov    %eax,%ebp
   361dc:	cmp    $0xff,%al
   361de:	jne    36fe9 <meshopt_peers::run+0x2329>
   361e4:	mov    0x18(%rsp),%rax
   361e9:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   361f3:	mul    %rcx
   361f6:	mov    %rdx,%rax
   361f9:	shr    $1,%rax
   361fc:	mov    %rax,0x48(%rsp)
   36201:	mov    $0x30,%ecx
   36206:	mul    %rcx
   36209:	seto   %cl
   3620c:	test   %rax,%rax
   3620f:	sets   %dl
   36212:	or     %cl,%dl
   36214:	mov    $0x8,%bpl
   36217:	mov    0x10(%rsp),%r12
   3621c:	jne    36d9e <meshopt_peers::run+0x20de>
   36222:	mov    $0x1,%cl
   36224:	mov    %ecx,0x8(%rsp)
   36228:	add    0x750(%rsp),%rax
   36230:	jb     36e74 <meshopt_peers::run+0x21b4>
   36236:	mov    %rax,0x750(%rsp)
   3623e:	mov    0x738(%rsp),%r13
   36246:	mov    %r13,%rdi
   36249:	mov    %rax,%r12
   3624c:	mov    %rax,%rsi
   3624f:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   36254:	mov    %eax,%ebp
   36256:	cmp    $0xff,%al
   36258:	jne    37103 <meshopt_peers::run+0x2443>
   3625e:	mov    %r15,0x40(%rsp)
   36263:	mov    0x18(%rsp),%r8
   36268:	cmp    $0x3,%r8
   3626c:	jae    36cde <meshopt_peers::run+0x201e>
   36272:	mov    $0x8,%r15d
   36278:	xor    %eax,%eax
   3627a:	mov    %rax,0x68(%rsp)
   3627f:	sub    0x48(%rsp),%rax
   36284:	mov    $0x30,%ecx
   36289:	mul    %rcx
   3628c:	seto   %cl
   3628f:	test   %rax,%rax
   36292:	sets   %dl
   36295:	or     %cl,%dl
   36297:	mov    $0x8,%bpl
   3629a:	jne    36e59 <meshopt_peers::run+0x2199>
   362a0:	test   %rax,%rax
   362a3:	je     362d0 <meshopt_peers::run+0x1610>
   362a5:	add    %r12,%rax
   362a8:	jb     36e59 <meshopt_peers::run+0x2199>
   362ae:	mov    %rax,0x750(%rsp)
   362b6:	mov    %r13,%rdi
   362b9:	mov    %rax,%rsi
   362bc:	call   3c040 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   362c1:	mov    %eax,%ebp
   362c3:	cmp    $0xff,%al
   362c5:	mov    0x18(%rsp),%r8
   362ca:	jne    36e59 <meshopt_peers::run+0x2199>
   362d0:	mov    %r15,%rdx
   362d3:	shr    $0x8,%rdx
   362d7:	mov    %r15,%rax
   362da:	shr    $0x28,%rax
   362de:	mov    %r15,%rcx
   362e1:	shr    $0x38,%rcx
   362e5:	mov    %cl,0xef(%rsp)
   362ec:	mov    %ax,0xed(%rsp)
   362f4:	mov    %edx,0xe9(%rsp)
   362fb:	movq   $0x0,0xf0(%rsp)
   36307:	mov    0x68(%rsp),%rax
   3630c:	mov    %rax,0xe0(%rsp)
   36314:	mov    %r15b,0xe8(%rsp)
   3631c:	cmp    $0x2,%r8
   36320:	jbe    363a9 <meshopt_peers::run+0x16e9>
   36326:	cmp    0x48(%rsp),%rax
   3632b:	jb     36d2d <meshopt_peers::run+0x206d>
   36331:	xor    %r12d,%r12d
   36334:	mov    0xe8(%rsp),%r15
   3633c:	lea    (%r12,%r12,2),%rax
   36340:	shl    $0x4,%rax
   36344:	xorps  %xmm0,%xmm0
   36347:	movups %xmm0,0x20(%r15,%rax,1)
   3634d:	movups %xmm0,0x10(%r15,%rax,1)
   36353:	movups %xmm0,(%r15,%rax,1)
   36358:	cmpq   $0x6,0x18(%rsp)
   3635e:	jb     36381 <meshopt_peers::run+0x16c1>
   36360:	lea    (%r15,%rax,1),%rdi
   36364:	add    $0x30,%rdi
   36368:	mov    0x48(%rsp),%rax
   3636d:	lea    (%rax,%rax,2),%rdx
   36371:	shl    $0x4,%rdx
   36375:	add    $0xffffffffffffffd0,%rdx
   36379:	xor    %esi,%esi
   3637b:	call   *0xf4467(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   36381:	mov    0xe0(%rsp),%rax
   36389:	mov    %rax,0x68(%rsp)
   3638e:	cmp    $0xffffffffffffffff,%rax
   36392:	je     37141 <meshopt_peers::run+0x2481>
   36398:	mov    %r15,%rdx
   3639b:	shr    $0x8,%rdx
   3639f:	add    %r12,0x48(%rsp)
   363a4:	mov    0x18(%rsp),%r8
   363a9:	movzbl %r15b,%eax
   363ad:	shl    $0x8,%rdx
   363b1:	or     %rax,%rdx
   363b4:	mov    %rdx,0xb0(%rsp)
   363bc:	lea    0x350(%rsp),%rdi
   363c4:	lea    0x738(%rsp),%rsi
   363cc:	mov    %r8,%rdx
   363cf:	call   33830 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   363d4:	mov    0x350(%rsp),%rdx
   363dc:	movzbl 0x358(%rsp),%ebp
   363e4:	cmp    $0xffffffffffffffff,%rdx
   363e8:	je     3710d <meshopt_peers::run+0x244d>
   363ee:	lea    0x359(%rsp),%rcx
   363f6:	mov    (%rcx),%rax
   363f9:	mov    0x7(%rcx),%rcx
   363fd:	lea    0xe9(%rsp),%rsi
   36405:	mov    %rcx,0x7(%rsi)
   36409:	mov    %rax,(%rsi)
   3640c:	mov    %rdx,0xe0(%rsp)
   36414:	mov    %bpl,0xe8(%rsp)
   3641c:	mov    0xf0(%rsp),%rcx
   36424:	mov    0x18(%rsp),%rsi
   36429:	mov    %rsi,%r15
   3642c:	sub    %rcx,%r15
   3642f:	mov    %rdx,0x80(%rsp)
   36437:	jbe    3648c <meshopt_peers::run+0x17cc>
   36439:	mov    %rdx,%rax
   3643c:	mov    %rcx,%rsi
   3643f:	sub    %rcx,%rax
   36442:	cmp    %rax,%r15
   36445:	ja     36d59 <meshopt_peers::run+0x2099>
   3644b:	mov    0xe8(%rsp),%rbp
   36453:	mov    %rsi,%rbx
   36456:	lea    0x0(,%rsi,4),%rdi
   3645e:	add    %rbp,%rdi
   36461:	lea    0x0(,%r15,4),%rdx
   36469:	xor    %esi,%esi
   3646b:	call   *0xf4377(%rip)        # 12a7e8 <memset@GLIBC_2.2.5>
   36471:	cmpq   $0xffffffffffffffff,0x80(%rsp)
   3647a:	je     3710d <meshopt_peers::run+0x244d>
   36480:	add    %r15,%rbx
   36483:	mov    %rbp,%rdx
   36486:	shr    $0x8,%rdx
   3648a:	jmp    364b2 <meshopt_peers::run+0x17f2>
   3648c:	movzbl 0xef(%rsp),%eax
   36494:	shl    $0x10,%eax
   36497:	movzwl 0xed(%rsp),%ecx
   3649f:	or     %eax,%ecx
   364a1:	shl    $0x20,%rcx
   364a5:	mov    0xe9(%rsp),%edx
   364ac:	or     %rcx,%rdx
   364af:	mov    %rsi,%rbx
   364b2:	movzbl %bpl,%eax
   364b6:	shl    $0x8,%rdx
   364ba:	or     %rax,%rdx
   364bd:	cmp    $0x2,%r14
   364c1:	mov    %rdx,0xc0(%rsp)
   364c9:	mov    %rbx,0x140(%rsp)
   364d1:	jb     36be8 <meshopt_peers::run+0x1f28>
   364d7:	mov    0xb0(%rsp),%rax
   364df:	add    $0x10,%rax
   364e3:	mov    %rax,0x280(%rsp)
   364eb:	xor    %ebp,%ebp
   364ed:	xor    %r12d,%r12d
   364f0:	mov    0xd0(%rsp),%rax
   364f8:	mov    %rbp,0xb8(%rsp)
   36500:	add    $0x4,%rax
   36504:	mov    -0x4(%rax),%edi
   36507:	mov    %rax,0x1b8(%rsp)
   3650f:	mov    (%rax),%r15d
   36512:	cmp    %edi,%r15d
   36515:	mov    0xd8(%rsp),%rdx
   3651d:	jb     372af <meshopt_peers::run+0x25ef>
   36523:	cmp    %r15,%rdx
   36526:	jb     372af <meshopt_peers::run+0x25ef>
   3652c:	sub    %rdi,%r15
   3652f:	lea    (%rdi,%rdi,2),%r13
   36533:	shl    $0x4,%r13
   36537:	add    0x38(%rsp),%r13
   3653c:	movq   $0xffffffffffffffff,0xe0(%rsp)
   36548:	lea    0x350(%rsp),%rdi
   36550:	mov    %r13,%rsi
   36553:	mov    %r15,%rdx
   36556:	mov    0x28(%rsp),%rcx
   3655b:	mov    0x18(%rsp),%r8
   36560:	lea    0x2c0(%rsp),%r9
   36568:	lea    0x738(%rsp),%rax
   36570:	push   %rax
   36571:	lea    0x1d8(%rsp),%rax
   36579:	push   %rax
   3657a:	push   0x30(%rsp)
   3657e:	push   0xa8(%rsp)
   36585:	call   *0xf465d(%rip)        # 12abe8 <_DYNAMIC+0x628>
   3658b:	add    $0x20,%rsp
   3658f:	mov    0x350(%rsp),%rax
   36597:	movzbl 0x358(%rsp),%ebp
   3659f:	mov    %rax,0x8(%rsp)
   365a4:	cmp    $0xffffffffffffffff,%rax
   365a8:	je     3709e <meshopt_peers::run+0x23de>
   365ae:	movzbl %bpl,%eax
   365b2:	movzbl 0x35f(%rsp),%ecx
   365ba:	shl    $0x10,%ecx
   365bd:	movzwl 0x35d(%rsp),%edx
   365c5:	or     %ecx,%edx
   365c7:	mov    0x359(%rsp),%ebx
   365ce:	mov    0x368(%rsp),%rcx
   365d6:	mov    %rcx,0x60(%rsp)
   365db:	mov    0x370(%rsp),%rcx
   365e3:	mov    %rcx,0xa8(%rsp)
   365eb:	mov    0x380(%rsp),%rcx
   365f3:	mov    %rcx,0x30(%rsp)
   365f8:	mov    0x388(%rsp),%rcx
   36600:	mov    %rcx,0x58(%rsp)
   36605:	mov    0x390(%rsp),%rcx
   3660d:	mov    %rcx,0x78(%rsp)
   36612:	lea    0x359(%rsp),%rcx
   3661a:	movups 0x3f(%rcx),%xmm0
   3661e:	movaps %xmm0,0x260(%rsp)
   36626:	mov    0x4f(%rcx),%ecx
   36629:	mov    %ecx,0x270(%rsp)
   36630:	shl    $0x28,%rdx
   36634:	shl    $0x8,%rbx
   36638:	or     %rdx,%rbx
   3663b:	or     %rax,%rbx
   3663e:	testb  $0x1,0x3ac(%rsp)
   36646:	je     367b7 <meshopt_peers::run+0x1af7>
   3664c:	mov    %r13,%rdi
   3664f:	mov    %r15,%rsi
   36652:	mov    0x28(%rsp),%rdx
   36657:	mov    0x18(%rsp),%rcx
   3665c:	mov    0x50(%rsp),%r8
   36661:	mov    0x20(%rsp),%r9
   36666:	lea    0x738(%rsp),%rax
   3666e:	push   %rax
   3666f:	lea    0xa4(%rsp),%rax
   36677:	push   %rax
   36678:	lea    0x168(%rsp),%rax
   36680:	push   %rax
   36681:	push   0x160(%rsp)
   36688:	lea    0x280(%rsp),%rax
   36690:	push   %rax
   36691:	push   $0x0
   36693:	call   33f30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36698:	add    $0x30,%rsp
   3669c:	mov    %rax,%rbp
   3669f:	test   $0x1,%bpl
   366a3:	jne    37163 <meshopt_peers::run+0x24a3>
   366a9:	mov    0x750(%rsp),%rcx
   366b1:	mov    0x8(%rsp),%rdx
   366b6:	lea    0x0(,%rdx,4),%rax
   366be:	shr    $0x3e,%rdx
   366c2:	sete   %dl
   366c5:	test   %rax,%rax
   366c8:	setns  %sil
   366cc:	and    %dl,%sil
   366cf:	sub    %rax,%rcx
   366d2:	setae  %al
   366d5:	test   %al,%sil
   366d8:	je     36ee4 <meshopt_peers::run+0x2224>
   366de:	mov    %rcx,0x750(%rsp)
   366e6:	cmpq   $0x0,0x8(%rsp)
   366ec:	je     366ff <meshopt_peers::run+0x1a3f>
   366ee:	mov    %rbx,%rdi
   366f1:	call   *0xf40d1(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   366f7:	mov    0x750(%rsp),%rcx
   366ff:	mov    0x60(%rsp),%rax
   36704:	mov    $0x14,%edx
   36709:	mul    %rdx
   3670c:	jo     37072 <meshopt_peers::run+0x23b2>
   36712:	test   %rax,%rax
   36715:	js     37072 <meshopt_peers::run+0x23b2>
   3671b:	sub    %rax,%rcx
   3671e:	jb     37072 <meshopt_peers::run+0x23b2>
   36724:	mov    %rcx,0x750(%rsp)
   3672c:	cmpq   $0x0,0x60(%rsp)
   36732:	je     3674a <meshopt_peers::run+0x1a8a>
   36734:	mov    0xa8(%rsp),%rdi
   3673c:	call   *0xf4086(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36742:	mov    0x750(%rsp),%rcx
   3674a:	movabs $0x3fffffffffffffff,%rax
   36754:	cmp    %rax,0x30(%rsp)
   36759:	ja     37128 <meshopt_peers::run+0x2468>
   3675f:	mov    0x30(%rsp),%rax
   36764:	lea    0x0(,%rax,4),%rax
   3676c:	test   %rax,%rax
   3676f:	js     37128 <meshopt_peers::run+0x2468>
   36775:	sub    %rax,%rcx
   36778:	jb     37128 <meshopt_peers::run+0x2468>
   3677e:	mov    %rcx,0x750(%rsp)
   36786:	cmpq   $0x0,0x30(%rsp)
   3678c:	je     36799 <meshopt_peers::run+0x1ad9>
   3678e:	mov    0x58(%rsp),%rdi
   36793:	call   *0xf402f(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36799:	dec    %r14
   3679c:	mov    0x1b8(%rsp),%rax
   367a4:	add    $0x4,%rax
   367a8:	cmp    $0x1,%r14
   367ac:	ja     36504 <meshopt_peers::run+0x1844>
   367b2:	jmp    36d91 <meshopt_peers::run+0x20d1>
   367b7:	mov    %r13,%rdi
   367ba:	mov    %r15,%rsi
   367bd:	mov    0x28(%rsp),%rdx
   367c2:	mov    0x18(%rsp),%rcx
   367c7:	mov    0x50(%rsp),%r8
   367cc:	mov    0x20(%rsp),%r9
   367d1:	lea    0x738(%rsp),%rax
   367d9:	push   %rax
   367da:	lea    0xa4(%rsp),%rax
   367e2:	push   %rax
   367e3:	lea    0x168(%rsp),%rax
   367eb:	push   %rax
   367ec:	push   0x160(%rsp)
   367f3:	lea    0x280(%rsp),%rax
   367fb:	push   %rax
   367fc:	push   $0x0
   367fe:	call   33f30 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36803:	add    $0x30,%rsp
   36807:	mov    %rax,%rbp
   3680a:	test   $0x1,%bpl
   3680e:	jne    37163 <meshopt_peers::run+0x24a3>
   36814:	shr    $0x20,%rbp
   36818:	movb   $0x2,0x170(%rsp)
   36820:	mov    0x50(%rsp),%rax
   36825:	mov    %rax,0x178(%rsp)
   3682d:	mov    0x20(%rsp),%rax
   36832:	mov    %rax,0x180(%rsp)
   3683a:	movq   $0x0,0x1a8(%rsp)
   36846:	movups 0x210(%rsp),%xmm0
   3684e:	movaps %xmm0,0x390(%rsp)
   36856:	movups 0x1d0(%rsp),%xmm0
   3685e:	movups 0x1e0(%rsp),%xmm1
   36866:	movups 0x1f0(%rsp),%xmm2
   3686e:	movups 0x200(%rsp),%xmm3
   36876:	movaps %xmm3,0x380(%rsp)
   3687e:	movaps %xmm2,0x370(%rsp)
   36886:	movaps %xmm1,0x360(%rsp)
   3688e:	movaps %xmm0,0x350(%rsp)
   36896:	lea    0x230(%rsp),%rdi
   3689e:	mov    0x58(%rsp),%rsi
   368a3:	mov    0x78(%rsp),%rdx
   368a8:	lea    0x170(%rsp),%rcx
   368b0:	lea    0x350(%rsp),%r8
   368b8:	mov    %ebp,%r9d
   368bb:	lea    0x738(%rsp),%rax
   368c3:	push   %rax
   368c4:	push   0x1c8(%rsp)
   368cb:	call   *0xf42f7(%rip)        # 12abc8 <_DYNAMIC+0x608>
   368d1:	add    $0x10,%rsp
   368d5:	mov    0x230(%rsp),%r15
   368dd:	movzbl 0x238(%rsp),%ebp
   368e5:	cmp    $0xffffffffffffffff,%r15
   368e9:	je     37166 <meshopt_peers::run+0x24a6>
   368ef:	movzbl %bpl,%ecx
   368f3:	movzbl 0x23f(%rsp),%eax
   368fb:	shl    $0x10,%eax
   368fe:	movzwl 0x23d(%rsp),%esi
   36906:	or     %eax,%esi
   36908:	mov    0x239(%rsp),%r13d
   36910:	mov    0x240(%rsp),%rax
   36918:	mov    0x248(%rsp),%rdx
   36920:	mov    %rdx,0xa0(%rsp)
   36928:	mov    0x250(%rsp),%rdx
   36930:	mov    %rdx,0x78(%rsp)
   36935:	mov    0x258(%rsp),%rdx
   3693d:	shl    $0x28,%rsi
   36941:	shl    $0x8,%r13
   36945:	or     %rsi,%r13
   36948:	or     %rcx,%r13
   3694b:	test   %rax,%rax
   3694e:	je     369f7 <meshopt_peers::run+0x1d37>
   36954:	lea    (%rax,%rax,2),%rax
   36958:	shl    $0x4,%rax
   3695c:	add    %r13,%rax
   3695f:	lea    (%r12,%r12,2),%rcx
   36963:	shl    $0x4,%rcx
   36967:	add    0x280(%rsp),%rcx
   3696f:	xor    %esi,%esi
   36971:	mov    0x0(%r13,%rsi,1),%rdi
   36976:	mov    0x8(%r13,%rsi,1),%r8
   3697b:	movups 0x10(%r13,%rsi,1),%xmm0
   36981:	movaps %xmm0,0x6c0(%rsp)
   36989:	mov    0x270(%rsp),%r9d
   36991:	lea    0x6cc(%rsp),%r10
   36999:	mov    %r9d,0x10(%r10)
   3699d:	movaps 0x260(%rsp),%xmm0
   369a5:	movups %xmm0,(%r10)
   369a9:	cmp    0x48(%rsp),%r12
   369ae:	jae    373a7 <meshopt_peers::run+0x26e7>
   369b4:	lea    (%rsi,%r13,1),%r9
   369b8:	add    0xb8(%rsp),%r8
   369c0:	mov    %rdi,-0x10(%rcx,%rsi,1)
   369c5:	mov    %r8,-0x8(%rcx,%rsi,1)
   369ca:	movaps 0x6c0(%rsp),%xmm0
   369d2:	movaps 0x6d0(%rsp),%xmm1
   369da:	movups %xmm0,(%rcx,%rsi,1)
   369de:	movups %xmm1,0x10(%rcx,%rsi,1)
   369e3:	inc    %r12
   369e6:	add    $0x30,%rsi
   369ea:	add    $0x30,%r9
   369ee:	cmp    %rax,%r9
   369f1:	jne    36971 <meshopt_peers::run+0x1cb1>
   369f7:	mov    %rdx,%rbp
   369fa:	add    0xb8(%rsp),%rbp
   36a02:	jb     3731c <meshopt_peers::run+0x265c>
   36a08:	cmp    0x140(%rsp),%rbp
   36a10:	ja     3731c <meshopt_peers::run+0x265c>
   36a16:	mov    0xc0(%rsp),%rax
   36a1e:	mov    0xb8(%rsp),%rcx
   36a26:	lea    (%rax,%rcx,4),%rdi
   36a2a:	shl    $0x2,%rdx
   36a2e:	mov    0x78(%rsp),%rsi
   36a33:	call   *0xf3d9f(%rip)        # 12a7d8 <memcpy@GLIBC_2.14>
   36a39:	mov    0x750(%rsp),%rcx
   36a41:	mov    %r15,%rax
   36a44:	mov    $0x30,%edx
   36a49:	mul    %rdx
   36a4c:	seto   %dl
   36a4f:	not    %dl
   36a51:	test   %rax,%rax
   36a54:	setns  %sil
   36a58:	and    %dl,%sil
   36a5b:	sub    %rax,%rcx
   36a5e:	setae  %al
   36a61:	test   %al,%sil
   36a64:	je     371c3 <meshopt_peers::run+0x2503>
   36a6a:	mov    %rcx,0x750(%rsp)
   36a72:	test   %r15,%r15
   36a75:	je     36a88 <meshopt_peers::run+0x1dc8>
   36a77:	mov    %r13,%rdi
   36a7a:	call   *0xf3d48(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36a80:	mov    0x750(%rsp),%rcx
   36a88:	mov    0xa0(%rsp),%rax
   36a90:	shr    $0x3e,%rax
   36a94:	jne    371d1 <meshopt_peers::run+0x2511>
   36a9a:	mov    0xa0(%rsp),%rax
   36aa2:	lea    0x0(,%rax,4),%rax
   36aaa:	test   %rax,%rax
   36aad:	js     371d1 <meshopt_peers::run+0x2511>
   36ab3:	sub    %rax,%rcx
   36ab6:	jb     371d1 <meshopt_peers::run+0x2511>
   36abc:	mov    %rcx,0x750(%rsp)
   36ac4:	cmpq   $0x0,0xa0(%rsp)
   36acd:	je     36ae2 <meshopt_peers::run+0x1e22>
   36acf:	mov    0x78(%rsp),%rdi
   36ad4:	call   *0xf3cee(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36ada:	mov    0x750(%rsp),%rcx
   36ae2:	movabs $0x3fffffffffffffff,%rax
   36aec:	cmp    %rax,0x8(%rsp)
   36af1:	ja     37258 <meshopt_peers::run+0x2598>
   36af7:	mov    0x8(%rsp),%rax
   36afc:	lea    0x0(,%rax,4),%rax
   36b04:	test   %rax,%rax
   36b07:	js     37258 <meshopt_peers::run+0x2598>
   36b0d:	sub    %rax,%rcx
   36b10:	jb     37258 <meshopt_peers::run+0x2598>
   36b16:	mov    %rcx,0x750(%rsp)
   36b1e:	cmpq   $0x0,0x8(%rsp)
   36b24:	je     36b37 <meshopt_peers::run+0x1e77>
   36b26:	mov    %rbx,%rdi
   36b29:	call   *0xf3c99(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36b2f:	mov    0x750(%rsp),%rcx
   36b37:	mov    0x60(%rsp),%rax
   36b3c:	mov    $0x14,%edx
   36b41:	mul    %rdx
   36b44:	jo     372d7 <meshopt_peers::run+0x2617>
   36b4a:	test   %rax,%rax
   36b4d:	js     372d7 <meshopt_peers::run+0x2617>
   36b53:	sub    %rax,%rcx
   36b56:	jb     372d7 <meshopt_peers::run+0x2617>
   36b5c:	mov    %rcx,0x750(%rsp)
   36b64:	cmpq   $0x0,0x60(%rsp)
   36b6a:	je     36b82 <meshopt_peers::run+0x1ec2>
   36b6c:	mov    0xa8(%rsp),%rdi
   36b74:	call   *0xf3c4e(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36b7a:	mov    0x750(%rsp),%rcx
   36b82:	movabs $0x3fffffffffffffff,%rax
   36b8c:	cmp    %rax,0x30(%rsp)
   36b91:	ja     37308 <meshopt_peers::run+0x2648>
   36b97:	mov    0x30(%rsp),%rax
   36b9c:	lea    0x0(,%rax,4),%rax
   36ba4:	test   %rax,%rax
   36ba7:	js     37308 <meshopt_peers::run+0x2648>
   36bad:	sub    %rax,%rcx
   36bb0:	jb     37308 <meshopt_peers::run+0x2648>
   36bb6:	mov    %rcx,0x750(%rsp)
   36bbe:	cmpq   $0x0,0x30(%rsp)
   36bc4:	je     36bd1 <meshopt_peers::run+0x1f11>
   36bc6:	mov    0x58(%rsp),%rdi
   36bcb:	call   *0xf3bf7(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36bd1:	dec    %r14
   36bd4:	cmp    $0x1,%r14
   36bd8:	mov    0x1b8(%rsp),%rax
   36be0:	ja     364f8 <meshopt_peers::run+0x1838>
   36be6:	jmp    36bed <meshopt_peers::run+0x1f2d>
   36be8:	xor    %r12d,%r12d
   36beb:	xor    %ebp,%ebp
   36bed:	mov    %rbp,0x18(%rsp)
   36bf2:	lea    0x738(%rsp),%rbx
   36bfa:	mov    %rbx,%rdi
   36bfd:	mov    0x40(%rsp),%rsi
   36c02:	mov    0xd0(%rsp),%rdx
   36c0a:	call   337e0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36c0f:	mov    %eax,%ebp
   36c11:	mov    $0x1,%al
   36c13:	mov    %eax,0x8(%rsp)
   36c17:	cmp    $0xff,%bpl
   36c1b:	setne  %r13b
   36c1f:	jne    3715b <meshopt_peers::run+0x249b>
   36c25:	mov    0x48(%rsp),%r15
   36c2a:	cmp    %r15,%r12
   36c2d:	cmovb  %r12,%r15
   36c31:	mov    0x140(%rsp),%r14
   36c39:	mov    0x18(%rsp),%rax
   36c3e:	cmp    %r14,%rax
   36c41:	cmovb  %rax,%r14
   36c45:	mov    %rbx,%rdi
   36c48:	mov    0x10(%rsp),%rsi
   36c4d:	mov    0x38(%rsp),%rdx
   36c52:	call   33790 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36c57:	cmp    $0xff,%al
   36c59:	jne    3717a <meshopt_peers::run+0x24ba>
   36c5f:	mov    %rbx,%rdi
   36c62:	mov    0x70(%rsp),%rsi
   36c67:	mov    0x28(%rsp),%rdx
   36c6c:	call   337e0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36c71:	cmp    $0xff,%al
   36c73:	jne    37184 <meshopt_peers::run+0x24c4>
   36c79:	mov    0x148(%rsp),%r13
   36c81:	inc    %r13d
   36c84:	mov    0x68(%rsp),%rax
   36c89:	mov    %rax,0x10(%rsp)
   36c8e:	mov    %r15,%rdx
   36c91:	mov    0xb0(%rsp),%rax
   36c99:	mov    %rax,0x38(%rsp)
   36c9e:	mov    %r14,%r8
   36ca1:	mov    0xc0(%rsp),%rax
   36ca9:	mov    %rax,0x28(%rsp)
   36cae:	mov    0x80(%rsp),%rax
   36cb6:	mov    %rax,0x70(%rsp)
   36cbb:	cmp    $0x1,%r15
   36cbf:	mov    0x20(%rsp),%rbx
   36cc4:	lea    0x170(%rsp),%rdi
   36ccc:	mov    0xf3f05(%rip),%r10        # 12abd8 <_DYNAMIC+0x618>
   36cd3:	ja     36053 <meshopt_peers::run+0x1393>
   36cd9:	jmp    37196 <meshopt_peers::run+0x24d6>
   36cde:	mov    $0x8,%edx
   36ce3:	mov    $0x8,%r8d
   36ce9:	mov    $0x30,%r9d
   36cef:	lea    0x350(%rsp),%rdi
   36cf7:	xor    %esi,%esi
   36cf9:	mov    0x48(%rsp),%rcx
   36cfe:	call   3b5d0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36d03:	cmpl   $0x1,0x350(%rsp)
   36d0b:	je     37216 <meshopt_peers::run+0x2556>
   36d11:	mov    0x358(%rsp),%r15
   36d19:	mov    0x48(%rsp),%rax
   36d1e:	mov    0x20(%rsp),%rbx
   36d23:	mov    0x18(%rsp),%r8
   36d28:	jmp    3627a <meshopt_peers::run+0x15ba>
   36d2d:	mov    $0x8,%ecx
   36d32:	mov    $0x30,%r8d
   36d38:	lea    0xe0(%rsp),%rdi
   36d40:	xor    %esi,%esi
   36d42:	mov    0x48(%rsp),%rdx
   36d47:	call   34c20 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36d4c:	mov    0xf0(%rsp),%r12
   36d54:	jmp    36334 <meshopt_peers::run+0x1674>
   36d59:	mov    $0x4,%ecx
   36d5e:	mov    $0x4,%r8d
   36d64:	lea    0xe0(%rsp),%rdi
   36d6c:	mov    %r15,%rdx
   36d6f:	call   34c20 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36d74:	mov    0xe0(%rsp),%rax
   36d7c:	mov    %rax,0x80(%rsp)
   36d84:	mov    0xf0(%rsp),%rsi
   36d8c:	jmp    3644b <meshopt_peers::run+0x178b>
   36d91:	mov    0xb8(%rsp),%rbp
   36d99:	jmp    36bed <meshopt_peers::run+0x1f2d>
   36d9e:	mov    $0x1,%al
   36da0:	mov    %eax,0x8(%rsp)
   36da4:	jmp    36e74 <meshopt_peers::run+0x21b4>
   36da9:	lea    0x738(%rsp),%rdi
   36db1:	mov    %r12,%rsi
   36db4:	mov    0x38(%rsp),%rdx
   36db9:	call   33790 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36dbe:	cmp    $0xff,%al
   36dc0:	jne    3733e <meshopt_peers::run+0x267e>
   36dc6:	lea    0x738(%rsp),%rdi
   36dce:	mov    0x80(%rsp),%rsi
   36dd6:	mov    0x28(%rsp),%rdx
   36ddb:	call   337e0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36de0:	mov    %eax,%ebp
   36de2:	jmp    358be <meshopt_peers::run+0xbfe>
   36de7:	movzbl 0x2c8(%rsp),%eax
   36def:	mov    %al,0x738(%rsp)
   36df6:	lea    -0x2684a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36dfd:	lea    0xec8fc(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36e04:	lea    0xec685(%rip),%r8        # 123490 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36e0b:	lea    0x738(%rsp),%rdx
   36e13:	mov    $0x2b,%esi
   36e18:	call   *0xf3a72(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   36e1e:	lea    0x290(%rsp),%rax
   36e26:	mov    %rax,0x738(%rsp)
   36e2e:	lea    0x55fb(%rip),%rax        # 3c430 <<&str as core::fmt::Display>::fmt>
   36e35:	mov    %rax,0x740(%rsp)
   36e3d:	lea    -0x2a749(%rip),%rdi        # c6fb <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36e44:	lea    0xec53d(%rip),%rdx        # 123388 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x120>
   36e4b:	lea    0x738(%rsp),%rsi
   36e53:	call   *0xf3a07(%rip)        # 12a860 <_DYNAMIC+0x2a0>
   36e59:	cmpq   $0x0,0x68(%rsp)
   36e5f:	mov    0x10(%rsp),%r12
   36e64:	je     36e6f <meshopt_peers::run+0x21af>
   36e66:	mov    %r15,%rdi
   36e69:	call   *0xf3959(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36e6f:	mov    0x40(%rsp),%r15
   36e74:	mov    $0x1,%r13b
   36e77:	test   %r15,%r15
   36e7a:	je     36e8a <meshopt_peers::run+0x21ca>
   36e7c:	mov    0xd0(%rsp),%rdi
   36e84:	call   *0xf393e(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36e8a:	mov    $0x1,%r14b
   36e8d:	mov    $0x1,%r15b
   36e90:	cmpb   $0x0,0x8(%rsp)
   36e95:	mov    0x70(%rsp),%rax
   36e9a:	jne    36fb6 <meshopt_peers::run+0x22f6>
   36ea0:	jmp    36fc6 <meshopt_peers::run+0x2306>
   36ea5:	movzbl 0x740(%rsp),%eax
   36ead:	mov    %al,0x350(%rsp)
   36eb4:	lea    -0x26908(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36ebb:	lea    0xec83e(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36ec2:	lea    0xec567(%rip),%r8        # 123430 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   36ec9:	lea    0x350(%rsp),%rdx
   36ed1:	mov    $0x2b,%esi
   36ed6:	call   *0xf39b4(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   36edc:	mov    $0x8,%bpl
   36edf:	jmp    358a5 <meshopt_peers::run+0xbe5>
   36ee4:	mov    $0x8,%bpl
   36ee7:	cmpq   $0x0,0x8(%rsp)
   36eed:	je     37033 <meshopt_peers::run+0x2373>
   36ef3:	mov    %rbx,%rdi
   36ef6:	call   *0xf38cc(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36efc:	xor    %r15d,%r15d
   36eff:	cmpq   $0x0,0x30(%rsp)
   36f05:	jne    3703e <meshopt_peers::run+0x237e>
   36f0b:	jmp    37049 <meshopt_peers::run+0x2389>
   36f10:	lea    0xec459(%rip),%rcx        # 123370 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x108>
   36f17:	mov    %r14,%rdx
   36f1a:	call   *0xf3a10(%rip)        # 12a930 <_DYNAMIC+0x370>
   36f20:	jmp    373bc <meshopt_peers::run+0x26fc>
   36f25:	movzbl 0x740(%rsp),%eax
   36f2d:	mov    %al,0x350(%rsp)
   36f34:	lea    -0x26988(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f3b:	lea    0xec7be(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f42:	lea    0xec4b7(%rip),%r8        # 123400 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36f49:	lea    0x350(%rsp),%rdx
   36f51:	mov    $0x2b,%esi
   36f56:	call   *0xf3934(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   36f5c:	movzbl 0x740(%rsp),%eax
   36f64:	mov    %al,0x350(%rsp)
   36f6b:	lea    -0x269bf(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f72:	lea    0xec787(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f79:	lea    0xec438(%rip),%r8        # 1233b8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36f80:	lea    0x350(%rsp),%rdx
   36f88:	mov    $0x2b,%esi
   36f8d:	call   *0xf38fd(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   36f93:	mov    $0x8,%bpl
   36f96:	lea    0x350(%rsp),%rdi
   36f9e:	call   33ea0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   36fa3:	mov    $0x1,%r14b
   36fa6:	mov    $0x1,%r15b
   36fa9:	mov    $0x1,%r13b
   36fac:	mov    0x10(%rsp),%r12
   36fb1:	mov    0x70(%rsp),%rax
   36fb6:	test   %rax,%rax
   36fb9:	je     36fc6 <meshopt_peers::run+0x2306>
   36fbb:	mov    0x28(%rsp),%rdi
   36fc0:	call   *0xf3802(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36fc6:	test   %r13b,%r13b
   36fc9:	je     36fdb <meshopt_peers::run+0x231b>
   36fcb:	test   %r12,%r12
   36fce:	je     36fdb <meshopt_peers::run+0x231b>
   36fd0:	mov    0x38(%rsp),%rdi
   36fd5:	call   *0xf37ed(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   36fdb:	test   %r15b,%r15b
   36fde:	jne    3588d <meshopt_peers::run+0xbcd>
   36fe4:	jmp    358a0 <meshopt_peers::run+0xbe0>
   36fe9:	mov    $0x1,%al
   36feb:	mov    %eax,0x8(%rsp)
   36fef:	mov    $0x1,%r13b
   36ff2:	mov    0x10(%rsp),%r12
   36ff7:	jmp    36e77 <meshopt_peers::run+0x21b7>
   36ffc:	movzbl 0x740(%rsp),%eax
   37004:	mov    %al,0x350(%rsp)
   3700b:	lea    -0x26a5f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37012:	lea    0xec6e7(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   37019:	lea    0xec428(%rip),%r8        # 123448 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   37020:	lea    0x350(%rsp),%rdx
   37028:	mov    $0x2b,%esi
   3702d:	call   *0xf385d(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   37033:	xor    %r15d,%r15d
   37036:	cmpq   $0x0,0x30(%rsp)
   3703c:	je     37049 <meshopt_peers::run+0x2389>
   3703e:	mov    0x58(%rsp),%rdi
   37043:	call   *0xf377f(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37049:	cmpq   $0x0,0x60(%rsp)
   3704f:	je     3705f <meshopt_peers::run+0x239f>
   37051:	mov    0xa8(%rsp),%rdi
   37059:	call   *0xf3769(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   3705f:	cmpq   $0x0,0x8(%rsp)
   37065:	setne  %al
   37068:	test   %r15b,%al
   3706b:	je     3709e <meshopt_peers::run+0x23de>
   3706d:	mov    %rbx,%rdi
   37070:	jmp    37098 <meshopt_peers::run+0x23d8>
   37072:	cmpq   $0x0,0x60(%rsp)
   37078:	je     37088 <meshopt_peers::run+0x23c8>
   3707a:	mov    0xa8(%rsp),%rdi
   37082:	call   *0xf3740(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37088:	mov    $0x8,%bpl
   3708b:	cmpq   $0x0,0x30(%rsp)
   37091:	je     3709e <meshopt_peers::run+0x23de>
   37093:	mov    0x58(%rsp),%rdi
   37098:	call   *0xf372a(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   3709e:	lea    0xe0(%rsp),%rdi
   370a6:	call   338d0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   370ab:	mov    $0x1,%r15b
   370ae:	mov    $0x1,%al
   370b0:	mov    %eax,0x8(%rsp)
   370b4:	mov    $0x1,%r13b
   370b7:	cmpq   $0x0,0x80(%rsp)
   370c0:	je     370d0 <meshopt_peers::run+0x2410>
   370c2:	mov    0xc0(%rsp),%rdi
   370ca:	call   *0xf36f8(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   370d0:	cmpq   $0x0,0x68(%rsp)
   370d6:	je     370e6 <meshopt_peers::run+0x2426>
   370d8:	mov    0xb0(%rsp),%rdi
   370e0:	call   *0xf36e2(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   370e6:	test   %r15b,%r15b
   370e9:	mov    0x20(%rsp),%rbx
   370ee:	mov    0x10(%rsp),%r12
   370f3:	mov    0x40(%rsp),%r15
   370f8:	jne    36e77 <meshopt_peers::run+0x21b7>
   370fe:	jmp    36e8a <meshopt_peers::run+0x21ca>
   37103:	mov    0x10(%rsp),%r12
   37108:	jmp    36e74 <meshopt_peers::run+0x21b4>
   3710d:	cmpq   $0x0,0x68(%rsp)
   37113:	je     3713c <meshopt_peers::run+0x247c>
   37115:	mov    0xb0(%rsp),%rdi
   3711d:	call   *0xf36a5(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37123:	mov    $0x1,%r13b
   37126:	jmp    37147 <meshopt_peers::run+0x2487>
   37128:	mov    $0x8,%bpl
   3712b:	cmpq   $0x0,0x30(%rsp)
   37131:	je     3709e <meshopt_peers::run+0x23de>
   37137:	jmp    37093 <meshopt_peers::run+0x23d3>
   3713c:	mov    $0x1,%r13b
   3713f:	jmp    37147 <meshopt_peers::run+0x2487>
   37141:	mov    $0x1,%r13b
   37144:	mov    %r15d,%ebp
   37147:	mov    0x20(%rsp),%rbx
   3714c:	mov    0x10(%rsp),%r12
   37151:	mov    0x40(%rsp),%r15
   37156:	jmp    36e77 <meshopt_peers::run+0x21b7>
   3715b:	xor    %r15d,%r15d
   3715e:	jmp    370b7 <meshopt_peers::run+0x23f7>
   37163:	shr    $0x8,%ebp
   37166:	mov    $0x1,%r15b
   37169:	cmpq   $0x0,0x30(%rsp)
   3716f:	jne    3703e <meshopt_peers::run+0x237e>
   37175:	jmp    37049 <meshopt_peers::run+0x2389>
   3717a:	mov    %eax,%ebp
   3717c:	xor    %r15d,%r15d
   3717f:	jmp    370b7 <meshopt_peers::run+0x23f7>
   37184:	mov    %eax,%ebp
   37186:	xor    %r15d,%r15d
   37189:	movl   $0x0,0x8(%rsp)
   37191:	jmp    370b7 <meshopt_peers::run+0x23f7>
   37196:	mov    0xb0(%rsp),%rax
   3719e:	mov    %rax,0x38(%rsp)
   371a3:	mov    0xc0(%rsp),%rax
   371ab:	mov    %rax,0x28(%rsp)
   371b0:	test   %r15,%r15
   371b3:	mov    0x68(%rsp),%r12
   371b8:	jne    35f55 <meshopt_peers::run+0x1295>
   371be:	jmp    35e92 <meshopt_peers::run+0x11d2>
   371c3:	test   %r15,%r15
   371c6:	je     371d1 <meshopt_peers::run+0x2511>
   371c8:	mov    %r13,%rdi
   371cb:	call   *0xf35f7(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   371d1:	mov    $0x1,%r13b
   371d4:	mov    $0x1,%r15b
   371d7:	cmpq   $0x0,0xa0(%rsp)
   371e0:	je     371ef <meshopt_peers::run+0x252f>
   371e2:	mov    0x78(%rsp),%rdi
   371e7:	call   *0xf35db(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   371ed:	jmp    371f2 <meshopt_peers::run+0x2532>
   371ef:	mov    $0x1,%r15b
   371f2:	cmpq   $0x0,0x30(%rsp)
   371f8:	je     37205 <meshopt_peers::run+0x2545>
   371fa:	mov    0x58(%rsp),%rdi
   371ff:	call   *0xf35c3(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37205:	mov    $0x8,%bpl
   37208:	test   %r13b,%r13b
   3720b:	jne    37049 <meshopt_peers::run+0x2389>
   37211:	jmp    3705f <meshopt_peers::run+0x239f>
   37216:	mov    $0xa,%bpl
   37219:	mov    0x20(%rsp),%rbx
   3721e:	mov    0x10(%rsp),%r12
   37223:	jmp    36e6f <meshopt_peers::run+0x21af>
   37228:	movb   $0x2,0x738(%rsp)
   37230:	lea    -0x26c84(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37237:	lea    0xec4c2(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3723e:	lea    0xec233(%rip),%r8        # 123478 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   37245:	lea    0x738(%rsp),%rdx
   3724d:	mov    $0x2b,%esi
   37252:	call   *0xf3638(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   37258:	mov    $0x1,%r13b
   3725b:	xor    %r15d,%r15d
   3725e:	mov    %rbx,0x78(%rsp)
   37263:	cmpq   $0x0,0x8(%rsp)
   37269:	jne    371e2 <meshopt_peers::run+0x2522>
   3726f:	jmp    371f2 <meshopt_peers::run+0x2532>
   37271:	movb   $0x8,0x738(%rsp)
   37279:	lea    -0x26ccd(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37280:	lea    0xec479(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   37287:	lea    0xec18a(%rip),%r8        # 123418 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   3728e:	lea    0x738(%rsp),%rdx
   37296:	mov    $0x2b,%esi
   3729b:	call   *0xf35ef(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   372a1:	mov    $0x4,%edi
   372a6:	mov    %r13,%rsi
   372a9:	call   *0xf3551(%rip)        # 12a800 <_DYNAMIC+0x240>
   372af:	lea    0xec05a(%rip),%rcx        # 123310 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   372b6:	mov    %r15,%rsi
   372b9:	call   *0xf3671(%rip)        # 12a930 <_DYNAMIC+0x370>
   372bf:	jmp    373bc <meshopt_peers::run+0x26fc>
   372c4:	mov    $0x4,%edi
   372c9:	mov    %r15,%rsi
   372cc:	call   *0xf352e(%rip)        # 12a800 <_DYNAMIC+0x240>
   372d2:	jmp    373bc <meshopt_peers::run+0x26fc>
   372d7:	xor    %r13d,%r13d
   372da:	cmpq   $0x0,0x60(%rsp)
   372e0:	je     372f7 <meshopt_peers::run+0x2637>
   372e2:	mov    0xa8(%rsp),%rax
   372ea:	mov    %rax,0x78(%rsp)
   372ef:	xor    %r15d,%r15d
   372f2:	jmp    371e2 <meshopt_peers::run+0x2522>
   372f7:	movq   $0x0,0x60(%rsp)
   37300:	xor    %r15d,%r15d
   37303:	jmp    371f2 <meshopt_peers::run+0x2532>
   37308:	mov    $0x8,%bpl
   3730b:	cmpq   $0x0,0x30(%rsp)
   37311:	je     3709e <meshopt_peers::run+0x23de>
   37317:	jmp    37093 <meshopt_peers::run+0x23d3>
   3731c:	lea    0xebfd5(%rip),%rcx        # 1232f8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   37323:	mov    0xb8(%rsp),%rdi
   3732b:	mov    %rbp,%rsi
   3732e:	mov    0x140(%rsp),%rdx
   37336:	call   *0xf35f4(%rip)        # 12a930 <_DYNAMIC+0x370>
   3733c:	jmp    373bc <meshopt_peers::run+0x26fc>
   3733e:	mov    %eax,%ebp
   37340:	cmpq   $0x0,0x80(%rsp)
   37349:	je     37356 <meshopt_peers::run+0x2696>
   3734b:	mov    0x28(%rsp),%rdi
   37350:	call   *0xf3472(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37356:	mov    0x738(%rsp),%rax
   3735e:	mov    0x748(%rsp),%rcx
   37366:	sub    0x740(%rsp),%rcx
   3736e:	mov    %rcx,0x358(%rax)
   37375:	mov    %bpl,0x738(%rsp)
   3737d:	lea    -0x26dd1(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37384:	lea    0xec375(%rip),%rcx        # 123700 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3738b:	lea    0xec116(%rip),%r8        # 1234a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   37392:	lea    0x738(%rsp),%rdx
   3739a:	mov    $0x2b,%esi
   3739f:	call   *0xf34eb(%rip)        # 12a890 <_DYNAMIC+0x2d0>
   373a5:	jmp    373bc <meshopt_peers::run+0x26fc>
   373a7:	lea    0xebf32(%rip),%rdx        # 1232e0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   373ae:	mov    %r12,%rdi
   373b1:	mov    0x48(%rsp),%rsi
   373b6:	call   *0xf3454(%rip)        # 12a810 <_DYNAMIC+0x250>
   373bc:	ud2
   373be:	mov    %rax,%r14
   373c1:	cmpq   $0x0,0xe0(%rsp)
   373ca:	je     37563 <meshopt_peers::run+0x28a3>
   373d0:	mov    0xe8(%rsp),%rdi
   373d8:	jmp    3755d <meshopt_peers::run+0x289d>
   373dd:	mov    %rax,%r14
   373e0:	cmpq   $0x0,0xe0(%rsp)
   373e9:	je     37579 <meshopt_peers::run+0x28b9>
   373ef:	mov    0xe8(%rsp),%r15
   373f7:	jmp    37426 <meshopt_peers::run+0x2766>
   373f9:	jmp    373fb <meshopt_peers::run+0x273b>
   373fb:	mov    %rax,%r14
   373fe:	cmpq   $0x0,0x30(%rsp)
   37404:	je     37498 <meshopt_peers::run+0x27d8>
   3740a:	jmp    374e4 <meshopt_peers::run+0x2824>
   3740f:	mov    %rax,%r14
   37412:	jmp    374b1 <meshopt_peers::run+0x27f1>
   37417:	mov    %rax,%r14
   3741a:	cmpq   $0x0,0x68(%rsp)
   37420:	je     37579 <meshopt_peers::run+0x28b9>
   37426:	mov    %r15,%rdi
   37429:	jmp    37573 <meshopt_peers::run+0x28b3>
   3742e:	mov    %rax,%r14
   37431:	jmp    37563 <meshopt_peers::run+0x28a3>
   37436:	mov    %rax,%r14
   37439:	mov    %r12,0x10(%rsp)
   3743e:	mov    0x80(%rsp),%rax
   37446:	mov    %rax,0x70(%rsp)
   3744b:	cmpq   $0x0,0x70(%rsp)
   37451:	je     37589 <meshopt_peers::run+0x28c9>
   37457:	jmp    37602 <meshopt_peers::run+0x2942>
   3745c:	mov    %rax,%r14
   3745f:	cmpq   $0x0,0x70(%rsp)
   37465:	je     37589 <meshopt_peers::run+0x28c9>
   3746b:	jmp    37602 <meshopt_peers::run+0x2942>
   37470:	mov    %r15,0x40(%rsp)
   37475:	mov    %rax,%r14
   37478:	jmp    37579 <meshopt_peers::run+0x28b9>
   3747d:	mov    %rax,%r14
   37480:	cmpq   $0x0,0xa0(%rsp)
   37489:	jne    374c3 <meshopt_peers::run+0x2803>
   3748b:	test   %r15,%r15
   3748e:	jne    374d3 <meshopt_peers::run+0x2813>
   37490:	cmpq   $0x0,0x30(%rsp)
   37496:	jne    374e4 <meshopt_peers::run+0x2824>
   37498:	cmpq   $0x0,0x60(%rsp)
   3749e:	jne    374f7 <meshopt_peers::run+0x2837>
   374a0:	cmpq   $0x0,0x8(%rsp)
   374a6:	je     374b1 <meshopt_peers::run+0x27f1>
   374a8:	mov    %rbx,%rdi
   374ab:	call   *0xf3317(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   374b1:	lea    0xe0(%rsp),%rdi
   374b9:	call   338d0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   374be:	jmp    3754a <meshopt_peers::run+0x288a>
   374c3:	mov    0x78(%rsp),%rdi
   374c8:	call   *0xf32fa(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   374ce:	test   %r15,%r15
   374d1:	je     37490 <meshopt_peers::run+0x27d0>
   374d3:	mov    %r13,%rdi
   374d6:	call   *0xf32ec(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   374dc:	cmpq   $0x0,0x30(%rsp)
   374e2:	je     37498 <meshopt_peers::run+0x27d8>
   374e4:	mov    0x58(%rsp),%rdi
   374e9:	call   *0xf32d9(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   374ef:	cmpq   $0x0,0x60(%rsp)
   374f5:	je     374a0 <meshopt_peers::run+0x27e0>
   374f7:	mov    0xa8(%rsp),%rdi
   374ff:	call   *0xf32c3(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37505:	cmpq   $0x0,0x8(%rsp)
   3750b:	jne    374a8 <meshopt_peers::run+0x27e8>
   3750d:	jmp    374b1 <meshopt_peers::run+0x27f1>
   3750f:	jmp    37511 <meshopt_peers::run+0x2851>
   37511:	mov    %rax,%r14
   37514:	cmpq   $0x0,0x20(%rsp)
   3751a:	je     375a1 <meshopt_peers::run+0x28e1>
   37520:	jmp    3762c <meshopt_peers::run+0x296c>
   37525:	mov    %rax,%r14
   37528:	lea    0x350(%rsp),%rdi
   37530:	call   33e60 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   37535:	mov    %r14,%rdi
   37538:	call   121810 <_Unwind_Resume@plt>
   3753d:	jmp    375ea <meshopt_peers::run+0x292a>
   37542:	jmp    375ea <meshopt_peers::run+0x292a>
   37547:	mov    %rax,%r14
   3754a:	cmpq   $0x0,0x80(%rsp)
   37553:	je     37563 <meshopt_peers::run+0x28a3>
   37555:	mov    0xc0(%rsp),%rdi
   3755d:	call   *0xf3265(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37563:	cmpq   $0x0,0x68(%rsp)
   37569:	je     37579 <meshopt_peers::run+0x28b9>
   3756b:	mov    0xb0(%rsp),%rdi
   37573:	call   *0xf324f(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37579:	cmpq   $0x0,0x40(%rsp)
   3757f:	jne    375b5 <meshopt_peers::run+0x28f5>
   37581:	cmpq   $0x0,0x70(%rsp)
   37587:	jne    37602 <meshopt_peers::run+0x2942>
   37589:	cmpq   $0x0,0x10(%rsp)
   3758f:	jne    37615 <meshopt_peers::run+0x2955>
   37595:	cmpq   $0x0,0x20(%rsp)
   3759b:	jne    3762c <meshopt_peers::run+0x296c>
   375a1:	cmpq   $0x0,0x150(%rsp)
   375aa:	jne    37645 <meshopt_peers::run+0x2985>
   375b0:	jmp    37653 <meshopt_peers::run+0x2993>
   375b5:	mov    0xd0(%rsp),%rdi
   375bd:	call   *0xf3205(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   375c3:	cmpq   $0x0,0x70(%rsp)
   375c9:	je     37589 <meshopt_peers::run+0x28c9>
   375cb:	jmp    37602 <meshopt_peers::run+0x2942>
   375cd:	mov    %rax,%r14
   375d0:	cmpq   $0x0,0x150(%rsp)
   375d9:	jne    37645 <meshopt_peers::run+0x2985>
   375db:	jmp    37653 <meshopt_peers::run+0x2993>
   375dd:	mov    %rax,%r14
   375e0:	jmp    37653 <meshopt_peers::run+0x2993>
   375e2:	mov    %rax,%r14
   375e5:	jmp    37672 <meshopt_peers::run+0x29b2>
   375ea:	mov    %rax,%r14
   375ed:	lea    0x350(%rsp),%rdi
   375f5:	call   33ea0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   375fa:	cmpq   $0x0,0x70(%rsp)
   37600:	je     37589 <meshopt_peers::run+0x28c9>
   37602:	mov    0x28(%rsp),%rdi
   37607:	call   *0xf31bb(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   3760d:	cmpq   $0x0,0x10(%rsp)
   37613:	je     37595 <meshopt_peers::run+0x28d5>
   37615:	mov    0x38(%rsp),%rdi
   3761a:	call   *0xf31a8(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37620:	cmpq   $0x0,0x20(%rsp)
   37626:	je     375a1 <meshopt_peers::run+0x28e1>
   3762c:	mov    0x90(%rsp),%rdi
   37634:	call   *0xf318e(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   3763a:	cmpq   $0x0,0x150(%rsp)
   37643:	je     37653 <meshopt_peers::run+0x2993>
   37645:	mov    0xc8(%rsp),%rdi
   3764d:	call   *0xf3175(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   37653:	mov    0x738(%rsp),%rax
   3765b:	mov    0x748(%rsp),%rcx
   37663:	sub    0x740(%rsp),%rcx
   3766b:	mov    %rcx,0x358(%rax)
   37672:	cmpq   $0x0,0x158(%rsp)
   3767b:	jne    3768d <meshopt_peers::run+0x29cd>
   3767d:	cmpq   $0x0,0x20(%rsp)
   37683:	jne    376a3 <meshopt_peers::run+0x29e3>
   37685:	mov    %r14,%rdi
   37688:	call   121810 <_Unwind_Resume@plt>
   3768d:	mov    0x160(%rsp),%rdi
   37695:	call   *0xf312d(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   3769b:	cmpq   $0x0,0x20(%rsp)
   376a1:	je     37685 <meshopt_peers::run+0x29c5>
   376a3:	mov    0x50(%rsp),%rdi
   376a8:	call   *0xf311a(%rip)        # 12a7c8 <free@GLIBC_2.2.5>
   376ae:	mov    %r14,%rdi
   376b1:	call   121810 <_Unwind_Resume@plt>
