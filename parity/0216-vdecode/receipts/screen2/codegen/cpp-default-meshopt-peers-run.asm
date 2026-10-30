
<artifacts>/screen2/driver-cpp-default:     file format elf64-x86-64


Disassembly of section .text:

0000000000024ec0 <meshopt_peers::run>:
   24ec0:	push   %rbp
   24ec1:	push   %r15
   24ec3:	push   %r14
   24ec5:	push   %r13
   24ec7:	push   %r12
   24ec9:	push   %rbx
   24eca:	sub    $0x138,%rsp
   24ed1:	mov    %rsi,0x60(%rsp)
   24ed6:	mov    %rdx,0x68(%rsp)
   24edb:	lea    -0x5(%rdx),%rax
   24edf:	cmp    $0xe,%rax
   24ee3:	ja     25bb9 <meshopt_peers::run+0xcf9>
   24ee9:	mov    %rcx,%r12
   24eec:	mov    %rdx,%r15
   24eef:	mov    %rsi,%rbp
   24ef2:	mov    %rdi,%rbx
   24ef5:	mov    0x8(%rcx),%rcx
   24ef9:	mov    0x10(%r12),%r13
   24efe:	mov    0x28(%r12),%r14
   24f03:	mov    0x20(%r12),%rsi
   24f08:	lea    -0x1afc3(%rip),%rdx        # 9f4c <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0x67>
   24f0f:	movslq (%rdx,%rax,4),%rax
   24f13:	add    %rdx,%rax
   24f16:	jmp    *%rax
   24f18:	mov    $0x68636163,%eax
   24f1d:	xor    0x0(%rbp),%eax
   24f20:	movzbl 0x4(%rbp),%edx
   24f24:	xor    $0x65,%edx
   24f27:	or     %eax,%edx
   24f29:	je     253b6 <meshopt_peers::run+0x4f6>
   24f2f:	mov    $0x63746566,%eax
   24f34:	xor    0x0(%rbp),%eax
   24f37:	movzbl 0x4(%rbp),%edx
   24f3b:	xor    $0x68,%edx
   24f3e:	or     %eax,%edx
   24f40:	jne    25bb9 <meshopt_peers::run+0xcf9>
   24f46:	test   %r14,%r14
   24f49:	mov    %rcx,0x20(%rsp)
   24f4e:	je     256ad <meshopt_peers::run+0x7ed>
   24f54:	mov    %rsi,%rbp
   24f57:	lea    0x0(,%r14,4),%r12
   24f5f:	mov    %r12,%rdi
   24f62:	call   *0x63368(%rip)        # 882d0 <malloc@GLIBC_2.2.5>
   24f68:	test   %rax,%rax
   24f6b:	je     25c2d <meshopt_peers::run+0xd6d>
   24f71:	mov    %rax,%r15
   24f74:	mov    %rax,%rdi
   24f77:	mov    %rbp,%rsi
   24f7a:	mov    %r12,%rdx
   24f7d:	call   *0x6331d(%rip)        # 882a0 <memcpy@GLIBC_2.14>
   24f83:	jmp    256b3 <meshopt_peers::run+0x7f3>
   24f88:	movabs $0x7966696c706d6973,%rax
   24f92:	xor    0x0(%rbp),%rax
   24f96:	movabs $0x7970706f6c732d79,%rdx
   24fa0:	xor    0x7(%rbp),%rdx
   24fa4:	or     %rax,%rdx
   24fa7:	je     25085 <meshopt_peers::run+0x1c5>
   24fad:	jmp    25bb9 <meshopt_peers::run+0xcf9>
   24fb2:	movdqu 0x0(%rbp),%xmm0
   24fb7:	movdqu 0x3(%rbp),%xmm1
   24fbc:	pcmpeqb -0x1d394(%rip),%xmm1        # 7c30 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x30>
   24fc4:	pcmpeqb -0x1d5ac(%rip),%xmm0        # 7a20 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x180>
   24fcc:	pand   %xmm1,%xmm0
   24fd0:	pmovmskb %xmm0,%eax
   24fd4:	cmp    $0xffff,%eax
   24fd9:	je     25085 <meshopt_peers::run+0x1c5>
   24fdf:	jmp    25bb9 <meshopt_peers::run+0xcf9>
   24fe4:	movabs $0x7966696c706d6973,%rax
   24fee:	cmp    %rax,0x0(%rbp)
   24ff2:	je     25085 <meshopt_peers::run+0x1c5>
   24ff8:	movabs $0x776172647265766f,%rax
   25002:	cmp    %rax,0x0(%rbp)
   25006:	je     253b6 <meshopt_peers::run+0x4f6>
   2500c:	movabs $0x7374656c6873656d,%rax
   25016:	cmp    %rax,0x0(%rbp)
   2501a:	jne    25bb9 <meshopt_peers::run+0xcf9>
   25020:	mov    %rsi,0x10(%rsp)
   25025:	mov    %rcx,0x20(%rsp)
   2502a:	mov    %r13,0x8(%rsp)
   2502f:	mov    %rbx,0x18(%rsp)
   25034:	movabs $0x7ffffffffffffffc,%rbx
   2503e:	movabs $0xffffffffffffffc,%r13
   25048:	mov    $0x40,%esi
   2504d:	mov    $0x7c,%edx
   25052:	mov    %r14,%rdi
   25055:	call   *0x635cd(%rip)        # 88628 <_DYNAMIC+0x5c0>
   2505b:	mov    %rax,%rbp
   2505e:	mov    %rax,%rsi
   25061:	shl    $0x4,%rsi
   25065:	lea    0x3(%r13),%rax
   25069:	cmp    %rax,%rbp
   2506c:	seta   %al
   2506f:	cmp    %rbx,%rsi
   25072:	seta   %cl
   25075:	or     %al,%cl
   25077:	je     257df <meshopt_peers::run+0x91f>
   2507d:	xor    %edi,%edi
   2507f:	call   *0x631fb(%rip)        # 88280 <_DYNAMIC+0x218>
   25085:	test   %r14,%r14
   25088:	je     2532a <meshopt_peers::run+0x46a>
   2508e:	mov    %rsi,0x10(%rsp)
   25093:	mov    %rcx,0x20(%rsp)
   25098:	mov    %r13,0x8(%rsp)
   2509d:	mov    %r14,%rax
   250a0:	mov    %rbx,%r14
   250a3:	mov    %rax,%r13
   250a6:	lea    0x0(,%rax,4),%rbx
   250ae:	mov    $0x1,%esi
   250b3:	mov    %rbx,%rdi
   250b6:	call   *0x63494(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   250bc:	test   %rax,%rax
   250bf:	je     25c03 <meshopt_peers::run+0xd43>
   250c5:	mov    %r14,%rbx
   250c8:	mov    %r13,%r14
   250cb:	mov    0x8(%rsp),%r13
   250d0:	mov    0x20(%rsp),%rcx
   250d5:	mov    0x10(%rsp),%rsi
   250da:	movl   $0x0,0x70(%rsp)
   250e2:	cmp    $0x8,%r15
   250e6:	je     25341 <meshopt_peers::run+0x481>
   250ec:	cmp    $0x13,%r15
   250f0:	jne    25355 <meshopt_peers::run+0x495>
   250f6:	movdqu 0x0(%rbp),%xmm0
   250fb:	movdqu 0x3(%rbp),%xmm1
   25100:	pcmpeqb -0x1d4d8(%rip),%xmm1        # 7c30 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x30>
   25108:	pcmpeqb -0x1d6f0(%rip),%xmm0        # 7a20 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x180>
   25110:	pand   %xmm1,%xmm0
   25114:	pmovmskb %xmm0,%edx
   25118:	cmp    $0xffff,%edx
   2511e:	jne    25355 <meshopt_peers::run+0x495>
   25124:	movss  0x80(%r12),%xmm0
   2512e:	lea    0x70(%rsp),%r11
   25133:	lea    -0x1d4ea(%rip),%r10        # 7c50 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x50>
   2513a:	mov    $0xc,%r9d
   25140:	mov    %rax,%r15
   25143:	mov    %rax,%rdi
   25146:	mov    %r13,%r8
   25149:	mov    %r14,%rdx
   2514c:	push   %r11
   2514e:	push   $0x0
   25150:	push   0x78(%r12)
   25155:	push   $0x0
   25157:	push   $0x4
   25159:	push   %r10
   2515b:	push   $0x10
   2515d:	push   0x38(%r12)
   25162:	call   *0x634c8(%rip)        # 88630 <_DYNAMIC+0x5c8>
   25168:	add    $0x40,%rsp
   2516c:	jmp    2538d <meshopt_peers::run+0x4cd>
   25171:	movabs $0x65646f6365642d78,%rax
   2517b:	movabs $0x652d786574726576,%rdi
   25185:	xor    0x0(%rbp),%rdi
   25189:	lea    0x9010000(%rax),%rdx
   25190:	xor    0x5(%rbp),%rdx
   25194:	or     %rdi,%rdx
   25197:	mov    %rbx,0x18(%rsp)
   2519c:	je     2541d <meshopt_peers::run+0x55d>
   251a2:	movabs $0x642d786574726576,%rcx
   251ac:	xor    0x0(%rbp),%rcx
   251b0:	xor    0x5(%rbp),%rax
   251b4:	or     %rcx,%rax
   251b7:	jne    25bb9 <meshopt_peers::run+0xcf9>
   251bd:	lea    0x0(,%r13,4),%rax
   251c5:	lea    (%rax,%rax,2),%r15
   251c9:	mov    $0x1,%bl
   251cb:	mov    %r13,0x8(%rsp)
   251d0:	test   %r15,%r15
   251d3:	je     25253 <meshopt_peers::run+0x393>
   251d5:	mov    $0x1,%esi
   251da:	mov    %r15,%rdi
   251dd:	call   *0x6336d(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   251e3:	test   %rax,%rax
   251e6:	je     25c11 <meshopt_peers::run+0xd51>
   251ec:	mov    %rax,%r13
   251ef:	test   %bl,%bl
   251f1:	jne    2525d <meshopt_peers::run+0x39d>
   251f3:	jmp    25282 <meshopt_peers::run+0x3c2>
   251f8:	movabs $0x6e652d7865646e69,%rax
   25202:	xor    0x0(%rbp),%rax
   25206:	mov    0x8(%rbp),%edx
   25209:	xor    $0x65646f63,%rdx
   25210:	or     %rax,%rdx
   25213:	mov    %rbx,0x18(%rsp)
   25218:	je     2544c <meshopt_peers::run+0x58c>
   2521e:	movabs $0x65642d7865646e69,%rax
   25228:	xor    0x0(%rbp),%rax
   2522c:	mov    0x8(%rbp),%ecx
   2522f:	xor    $0x65646f63,%rcx
   25236:	or     %rax,%rcx
   25239:	jne    25bb9 <meshopt_peers::run+0xcf9>
   2523f:	lea    0x0(,%r14,4),%r15
   25247:	xor    %ebx,%ebx
   25249:	mov    %r13,0x8(%rsp)
   2524e:	test   %r15,%r15
   25251:	jne    251d5 <meshopt_peers::run+0x315>
   25253:	mov    $0x1,%r13d
   25259:	test   %bl,%bl
   2525b:	je     25282 <meshopt_peers::run+0x3c2>
   2525d:	movabs $0x642d786574726576,%rax
   25267:	xor    0x0(%rbp),%rax
   2526b:	movabs $0x65646f6365642d78,%rcx
   25275:	xor    0x5(%rbp),%rcx
   25279:	or     %rax,%rcx
   2527c:	je     256e3 <meshopt_peers::run+0x823>
   25282:	mov    0x68(%r12),%rcx
   25287:	mov    0x70(%r12),%r8
   2528c:	mov    $0x4,%edx
   25291:	mov    %r13,%rdi
   25294:	mov    %r14,%rsi
   25297:	call   *0x6339b(%rip)        # 88638 <_DYNAMIC+0x5d0>
   2529d:	mov    0x18(%rsp),%rcx
   252a2:	mov    %eax,0x70(%rsp)
   252a6:	test   %eax,%eax
   252a8:	jne    25711 <meshopt_peers::run+0x851>
   252ae:	mov    %r15,0x8(%rcx)
   252b2:	mov    %r13,0x10(%rcx)
   252b6:	mov    %r15,0x18(%rcx)
   252ba:	jmp    25880 <meshopt_peers::run+0x9c0>
   252bf:	movabs $0x2d72657473756c63,%rax
   252c9:	xor    0x0(%rbp),%rax
   252cd:	movabs $0x646f6c2d72657473,%rdx
   252d7:	xor    0x3(%rbp),%rdx
   252db:	or     %rax,%rdx
   252de:	jne    25bb9 <meshopt_peers::run+0xcf9>
   252e4:	mov    %rsi,%r12
   252e7:	test   %r13,%r13
   252ea:	je     254a9 <meshopt_peers::run+0x5e9>
   252f0:	mov    %rcx,%rbp
   252f3:	lea    0x0(,%r13,4),%rax
   252fb:	lea    (%rax,%rax,2),%r15
   252ff:	mov    %r15,%rdi
   25302:	call   *0x62fc8(%rip)        # 882d0 <malloc@GLIBC_2.2.5>
   25308:	test   %rax,%rax
   2530b:	je     25c1f <meshopt_peers::run+0xd5f>
   25311:	mov    %rax,0x8(%rsp)
   25316:	mov    %rax,%rdi
   25319:	mov    %rbp,%rsi
   2531c:	mov    %r15,%rdx
   2531f:	call   *0x62f7b(%rip)        # 882a0 <memcpy@GLIBC_2.14>
   25325:	jmp    254b3 <meshopt_peers::run+0x5f3>
   2532a:	mov    $0x4,%eax
   2532f:	movl   $0x0,0x70(%rsp)
   25337:	cmp    $0x8,%r15
   2533b:	jne    250ec <meshopt_peers::run+0x22c>
   25341:	movabs $0x7966696c706d6973,%rdx
   2534b:	cmp    %rdx,0x0(%rbp)
   2534f:	je     25674 <meshopt_peers::run+0x7b4>
   25355:	movss  0x80(%r12),%xmm0
   2535f:	sub    $0x8,%rsp
   25363:	lea    0x78(%rsp),%r10
   25368:	mov    $0xc,%r9d
   2536e:	mov    %rax,%r15
   25371:	mov    %rax,%rdi
   25374:	mov    %r13,%r8
   25377:	mov    %r14,%rdx
   2537a:	push   %r10
   2537c:	push   0x78(%r12)
   25381:	push   $0x0
   25383:	call   *0x632b7(%rip)        # 88640 <_DYNAMIC+0x5d8>
   25389:	add    $0x20,%rsp
   2538d:	cmp    %r14,%rax
   25390:	cmovae %r14,%rax
   25394:	movss  0x70(%rsp),%xmm0
   2539a:	mov    %r14,0x8(%rbx)
   2539e:	mov    %r15,0x10(%rbx)
   253a2:	mov    %rax,0x18(%rbx)
   253a6:	movss  %xmm0,0x4(%rbx)
   253ab:	movl   $0x2,(%rbx)
   253b1:	jmp    25886 <meshopt_peers::run+0x9c6>
   253b6:	test   %r14,%r14
   253b9:	je     25788 <meshopt_peers::run+0x8c8>
   253bf:	mov    %rsi,0x10(%rsp)
   253c4:	mov    %rcx,0x20(%rsp)
   253c9:	mov    %r13,0x8(%rsp)
   253ce:	mov    %r14,%rax
   253d1:	mov    %rbx,%r14
   253d4:	mov    %rax,%r13
   253d7:	lea    0x0(,%rax,4),%rbx
   253df:	mov    $0x1,%esi
   253e4:	mov    %rbx,%rdi
   253e7:	call   *0x63163(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   253ed:	test   %rax,%rax
   253f0:	je     25c4b <meshopt_peers::run+0xd8b>
   253f6:	mov    %rax,%r12
   253f9:	mov    %r14,%rbx
   253fc:	mov    %r13,%r14
   253ff:	mov    0x8(%rsp),%r13
   25404:	mov    0x20(%rsp),%rcx
   25409:	mov    0x10(%rsp),%rsi
   2540e:	cmp    $0x5,%r15
   25412:	je     25794 <meshopt_peers::run+0x8d4>
   25418:	jmp    257ab <meshopt_peers::run+0x8eb>
   2541d:	mov    %rsi,0x10(%rsp)
   25422:	mov    %rcx,%r15
   25425:	mov    $0xc,%esi
   2542a:	mov    %r13,%rdi
   2542d:	call   *0x63215(%rip)        # 88648 <_DYNAMIC+0x5e0>
   25433:	mov    %rax,%r12
   25436:	mov    $0x1,%bl
   25438:	test   %r12,%r12
   2543b:	jns    2546a <meshopt_peers::run+0x5aa>
   2543d:	xor    %r13d,%r13d
   25440:	mov    %r13,%rdi
   25443:	mov    %r12,%rsi
   25446:	call   *0x62e34(%rip)        # 88280 <_DYNAMIC+0x218>
   2544c:	mov    %rsi,0x10(%rsp)
   25451:	mov    %rcx,%r15
   25454:	mov    %r14,%rdi
   25457:	mov    %r13,%rsi
   2545a:	call   *0x631f0(%rip)        # 88650 <_DYNAMIC+0x5e8>
   25460:	mov    %rax,%r12
   25463:	xor    %ebx,%ebx
   25465:	test   %r12,%r12
   25468:	js     2543d <meshopt_peers::run+0x57d>
   2546a:	je     2580b <meshopt_peers::run+0x94b>
   25470:	mov    %r13,0x8(%rsp)
   25475:	mov    $0x1,%r13d
   2547b:	mov    $0x1,%esi
   25480:	mov    %r12,%rdi
   25483:	call   *0x630c7(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   25489:	mov    %rax,(%rsp)
   2548d:	test   %rax,%rax
   25490:	je     25440 <meshopt_peers::run+0x580>
   25492:	mov    0x8(%rsp),%r13
   25497:	mov    0x10(%rsp),%r8
   2549c:	test   %bl,%bl
   2549e:	jne    2581d <meshopt_peers::run+0x95d>
   254a4:	jmp    25849 <meshopt_peers::run+0x989>
   254a9:	mov    $0x4,%eax
   254ae:	mov    %rax,0x8(%rsp)
   254b3:	movq   $0x0,0x48(%rsp)
   254bc:	movq   $0x4,0x50(%rsp)
   254c5:	movq   $0x0,0x58(%rsp)
   254ce:	movl   $0x0,0x44(%rsp)
   254d6:	lea    0x70(%rsp),%rdi
   254db:	mov    $0x80,%esi
   254e0:	call   *0x6313a(%rip)        # 88620 <_DYNAMIC+0x5b8>
   254e6:	cmpb   $0x1,0xc2(%rsp)
   254ee:	je     25be8 <meshopt_peers::run+0xd28>
   254f4:	mov    %r12,0xe0(%rsp)
   254fc:	mov    %r14,0xe8(%rsp)
   25504:	mov    %r13,0xf0(%rsp)
   2550c:	mov    0x8(%rsp),%r14
   25511:	mov    %r14,0xf8(%rsp)
   25519:	movq   $0xc,0x100(%rsp)
   25525:	movq   $0x4,0x108(%rsp)
   25531:	xorps  %xmm0,%xmm0
   25534:	movups %xmm0,0x110(%rsp)
   2553c:	movq   $0x4,0x120(%rsp)
   25548:	movq   $0x0,0x128(%rsp)
   25554:	movl   $0x0,0x130(%rsp)
   2555f:	lea    0x48(%rsp),%rax
   25564:	mov    %rax,0xd0(%rsp)
   2556c:	lea    0x44(%rsp),%rax
   25571:	mov    %rax,0xd8(%rsp)
   25579:	sub    $0xc0,%rsp
   25580:	mov    0x1f0(%rsp),%rax
   25588:	mov    %rax,0xb0(%rsp)
   25590:	movups 0x1e0(%rsp),%xmm0
   25598:	movups %xmm0,0xa0(%rsp)
   255a0:	movups 0x1a0(%rsp),%xmm0
   255a8:	movups 0x1b0(%rsp),%xmm1
   255b0:	movups 0x1c0(%rsp),%xmm2
   255b8:	movups 0x1d0(%rsp),%xmm3
   255c0:	movups %xmm3,0x90(%rsp)
   255c8:	movups %xmm2,0x80(%rsp)
   255d0:	movups %xmm1,0x70(%rsp)
   255d5:	movups %xmm0,0x60(%rsp)
   255da:	movups 0x180(%rsp),%xmm0
   255e2:	movups %xmm0,0x50(%rsp)
   255e7:	movups 0x170(%rsp),%xmm0
   255ef:	movups %xmm0,0x40(%rsp)
   255f4:	movups 0x130(%rsp),%xmm0
   255fc:	movdqu 0x140(%rsp),%xmm1
   25605:	movups 0x150(%rsp),%xmm2
   2560d:	movups 0x160(%rsp),%xmm3
   25615:	movups %xmm3,0x30(%rsp)
   2561a:	movups %xmm2,0x20(%rsp)
   2561f:	movdqu %xmm1,0x10(%rsp)
   25625:	movups %xmm0,(%rsp)
   25629:	lea    -0xea0(%rip),%rsi        # 24790 <meshopt::clusterlod::output_callback::<meshopt_peers::clod::{closure#0}>>
   25630:	lea    0x190(%rsp),%rdi
   25638:	call   *0x6301a(%rip)        # 88658 <_DYNAMIC+0x5f0>
   2563e:	add    $0xc0,%rsp
   25645:	mov    0x58(%rsp),%rax
   2564a:	mov    %rax,0x18(%rbx)
   2564e:	movups 0x48(%rsp),%xmm0
   25653:	movups %xmm0,0x8(%rbx)
   25657:	movl   $0x1,(%rbx)
   2565d:	test   %r13,%r13
   25660:	je     25886 <meshopt_peers::run+0x9c6>
   25666:	mov    %r14,%rdi
   25669:	call   *0x62c49(%rip)        # 882b8 <free@GLIBC_2.2.5>
   2566f:	jmp    25886 <meshopt_peers::run+0x9c6>
   25674:	movss  0x80(%r12),%xmm0
   2567e:	sub    $0x8,%rsp
   25682:	lea    0x78(%rsp),%r10
   25687:	mov    $0xc,%r9d
   2568d:	mov    %rax,%r15
   25690:	mov    %rax,%rdi
   25693:	mov    %r13,%r8
   25696:	mov    %r14,%rdx
   25699:	push   %r10
   2569b:	push   $0x0
   2569d:	push   0x78(%r12)
   256a2:	call   *0x62fb8(%rip)        # 88660 <_DYNAMIC+0x5f8>
   256a8:	jmp    25389 <meshopt_peers::run+0x4c9>
   256ad:	mov    $0x4,%r15d
   256b3:	lea    0x0(,%r13,4),%rax
   256bb:	lea    (%rax,%rax,2),%r12
   256bf:	mov    %r13,%rbp
   256c2:	test   %r13,%r13
   256c5:	je     25733 <meshopt_peers::run+0x873>
   256c7:	mov    $0x1,%esi
   256cc:	mov    %r12,%rdi
   256cf:	call   *0x62e7b(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   256d5:	test   %rax,%rax
   256d8:	je     25c3b <meshopt_peers::run+0xd7b>
   256de:	mov    %rax,%r13
   256e1:	jmp    25739 <meshopt_peers::run+0x879>
   256e3:	mov    0x50(%r12),%rcx
   256e8:	mov    0x58(%r12),%r8
   256ed:	mov    $0xc,%edx
   256f2:	mov    %r13,%rdi
   256f5:	mov    0x8(%rsp),%rsi
   256fa:	call   *0x62f68(%rip)        # 88668 <_DYNAMIC+0x600>
   25700:	mov    0x18(%rsp),%rcx
   25705:	mov    %eax,0x70(%rsp)
   25709:	test   %eax,%eax
   2570b:	je     252ae <meshopt_peers::run+0x3ee>
   25711:	lea    -0x1c004(%rip),%rdx        # 9714 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x58>
   25718:	lea    0x60dd1(%rip),%r9        # 864f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xc0>
   2571f:	lea    0x70(%rsp),%rsi
   25724:	xor    %edi,%edi
   25726:	xor    %ecx,%ecx
   25728:	call   *0x62f42(%rip)        # 88670 <_DYNAMIC+0x608>
   2572e:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25733:	mov    $0x1,%r13d
   25739:	mov    0x20(%rsp),%rcx
   2573e:	mov    $0xc,%r9d
   25744:	mov    %r13,%rdi
   25747:	mov    %r15,%rsi
   2574a:	mov    %r14,%rdx
   2574d:	mov    %rbp,%r8
   25750:	call   *0x62f22(%rip)        # 88678 <_DYNAMIC+0x610>
   25756:	shl    $0x2,%rax
   2575a:	lea    (%rax,%rax,2),%rax
   2575e:	cmp    %r12,%rax
   25761:	cmovae %r12,%rax
   25765:	mov    %r14,0x8(%rbx)
   25769:	mov    %r15,0x10(%rbx)
   2576d:	mov    %r14,0x18(%rbx)
   25771:	mov    %r12,0x20(%rbx)
   25775:	mov    %r13,0x28(%rbx)
   25779:	mov    %rax,0x30(%rbx)
   2577d:	movl   $0x3,(%rbx)
   25783:	jmp    25886 <meshopt_peers::run+0x9c6>
   25788:	mov    $0x4,%r12d
   2578e:	cmp    $0x5,%r15
   25792:	jne    257ab <meshopt_peers::run+0x8eb>
   25794:	mov    $0x68636163,%eax
   25799:	xor    0x0(%rbp),%eax
   2579c:	movzbl 0x4(%rbp),%edx
   257a0:	xor    $0x65,%edx
   257a3:	or     %eax,%edx
   257a5:	je     25898 <meshopt_peers::run+0x9d8>
   257ab:	movss  -0x1bfff(%rip),%xmm0        # 97b4 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xf8>
   257b3:	mov    $0xc,%r9d
   257b9:	mov    %r12,%rdi
   257bc:	mov    %r13,%r8
   257bf:	mov    %r14,%rdx
   257c2:	call   *0x62eb8(%rip)        # 88680 <_DYNAMIC+0x618>
   257c8:	mov    %r14,0x8(%rbx)
   257cc:	mov    %r12,0x10(%rbx)
   257d0:	mov    %r14,0x18(%rbx)
   257d4:	movl   $0x1,(%rbx)
   257da:	jmp    25886 <meshopt_peers::run+0x9c6>
   257df:	test   %rsi,%rsi
   257e2:	je     258ac <meshopt_peers::run+0x9ec>
   257e8:	mov    %rsi,%r15
   257eb:	mov    %rsi,%rdi
   257ee:	call   *0x62adc(%rip)        # 882d0 <malloc@GLIBC_2.2.5>
   257f4:	mov    %rax,(%rsp)
   257f8:	test   %rax,%rax
   257fb:	je     25c59 <meshopt_peers::run+0xd99>
   25801:	mov    %rbp,0x38(%rsp)
   25806:	jmp    258be <meshopt_peers::run+0x9fe>
   2580b:	mov    $0x1,%eax
   25810:	mov    %rax,(%rsp)
   25814:	mov    0x10(%rsp),%r8
   25819:	test   %bl,%bl
   2581b:	je     25849 <meshopt_peers::run+0x989>
   2581d:	movabs $0x652d786574726576,%rax
   25827:	xor    0x0(%rbp),%rax
   2582b:	movabs $0x65646f6365642d78,%rcx
   25835:	add    $0x9010000,%rcx
   2583c:	xor    0x5(%rbp),%rcx
   25840:	or     %rax,%rcx
   25843:	je     259ab <meshopt_peers::run+0xaeb>
   25849:	mov    (%rsp),%rbx
   2584d:	mov    %rbx,%rdi
   25850:	mov    %r12,%rsi
   25853:	mov    %r8,%rdx
   25856:	mov    %r14,%rcx
   25859:	call   *0x62e29(%rip)        # 88688 <_DYNAMIC+0x620>
   2585f:	mov    0x18(%rsp),%rcx
   25864:	test   %rax,%rax
   25867:	je     259e6 <meshopt_peers::run+0xb26>
   2586d:	cmp    %r12,%rax
   25870:	cmovae %r12,%rax
   25874:	mov    %r12,0x8(%rcx)
   25878:	mov    %rbx,0x10(%rcx)
   2587c:	mov    %rax,0x18(%rcx)
   25880:	movl   $0x0,(%rcx)
   25886:	add    $0x138,%rsp
   2588d:	pop    %rbx
   2588e:	pop    %r12
   25890:	pop    %r13
   25892:	pop    %r14
   25894:	pop    %r15
   25896:	pop    %rbp
   25897:	ret
   25898:	mov    %r12,%rdi
   2589b:	mov    %r14,%rdx
   2589e:	mov    %r13,%rcx
   258a1:	call   *0x62de9(%rip)        # 88690 <_DYNAMIC+0x628>
   258a7:	jmp    257c8 <meshopt_peers::run+0x908>
   258ac:	mov    $0x4,%eax
   258b1:	mov    %rax,(%rsp)
   258b5:	movq   $0x0,0x38(%rsp)
   258be:	cmp    $0x2,%rbp
   258c2:	jb     2593e <meshopt_peers::run+0xa7e>
   258c4:	lea    -0x1(%rbp),%rdx
   258c8:	lea    -0x2(%rbp),%rsi
   258cc:	mov    %edx,%eax
   258ce:	and    $0x7,%eax
   258d1:	mov    (%rsp),%rcx
   258d5:	cmp    $0x7,%rsi
   258d9:	jb     2591e <meshopt_peers::run+0xa5e>
   258db:	and    $0xfffffffffffffff8,%rdx
   258df:	xorps  %xmm0,%xmm0
   258e2:	mov    (%rsp),%rcx
   258e6:	cs nopw 0x0(%rax,%rax,1)
   258f0:	movups %xmm0,(%rcx)
   258f3:	movups %xmm0,0x10(%rcx)
   258f7:	movups %xmm0,0x20(%rcx)
   258fb:	movups %xmm0,0x30(%rcx)
   258ff:	movups %xmm0,0x40(%rcx)
   25903:	movups %xmm0,0x50(%rcx)
   25907:	movups %xmm0,0x60(%rcx)
   2590b:	movups %xmm0,0x70(%rcx)
   2590f:	sub    $0xffffffffffffff80,%rcx
   25913:	add    $0xfffffffffffffff8,%rdx
   25917:	jne    258f0 <meshopt_peers::run+0xa30>
   25919:	test   %rax,%rax
   2591c:	je     25947 <meshopt_peers::run+0xa87>
   2591e:	xorps  %xmm0,%xmm0
   25921:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25930:	movups %xmm0,(%rcx)
   25933:	add    $0x10,%rcx
   25937:	dec    %rax
   2593a:	jne    25930 <meshopt_peers::run+0xa70>
   2593c:	jmp    25947 <meshopt_peers::run+0xa87>
   2593e:	mov    (%rsp),%rcx
   25942:	test   %rbp,%rbp
   25945:	je     2594d <meshopt_peers::run+0xa8d>
   25947:	xorps  %xmm0,%xmm0
   2594a:	movups %xmm0,(%rcx)
   2594d:	mov    %rbp,%r12
   25950:	shl    $0x6,%r12
   25954:	mov    %rbp,%r15
   25957:	shl    $0x8,%r15
   2595b:	mov    %r12,%rax
   2595e:	shr    $0x3e,%rax
   25962:	setne  %al
   25965:	cmp    %rbx,%r15
   25968:	seta   %cl
   2596b:	or     %al,%cl
   2596d:	je     25983 <meshopt_peers::run+0xac3>
   2596f:	xor    %edi,%edi
   25971:	mov    %r15,%rsi
   25974:	mov    (%rsp),%r14
   25978:	call   *0x62902(%rip)        # 88280 <_DYNAMIC+0x218>
   2597e:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25983:	test   %r15,%r15
   25986:	je     25a04 <meshopt_peers::run+0xb44>
   25988:	mov    $0x1,%esi
   2598d:	mov    %r15,%rdi
   25990:	call   *0x62bba(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   25996:	mov    %rax,0x30(%rsp)
   2599b:	test   %rax,%rax
   2599e:	je     25c67 <meshopt_peers::run+0xda7>
   259a4:	mov    %r12,0x28(%rsp)
   259a9:	jmp    25a17 <meshopt_peers::run+0xb57>
   259ab:	sub    $0x8,%rsp
   259af:	mov    $0xc,%r8d
   259b5:	mov    0x8(%rsp),%rbx
   259ba:	mov    %rbx,%rdi
   259bd:	mov    %r12,%rsi
   259c0:	mov    %r15,%rdx
   259c3:	mov    %r13,%rcx
   259c6:	mov    $0x2,%r9d
   259cc:	push   $0x1
   259ce:	call   *0x62cc4(%rip)        # 88698 <_DYNAMIC+0x630>
   259d4:	add    $0x10,%rsp
   259d8:	mov    0x18(%rsp),%rcx
   259dd:	test   %rax,%rax
   259e0:	jne    2586d <meshopt_peers::run+0x9ad>
   259e6:	lea    -0x1b9d7(%rip),%rdi        # a016 <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0x131>
   259ed:	lea    0x60b14(%rip),%rdx        # 86508 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xd8>
   259f4:	mov    $0x17,%esi
   259f9:	call   *0x62979(%rip)        # 88378 <_DYNAMIC+0x310>
   259ff:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25a04:	mov    $0x4,%eax
   25a09:	mov    %rax,0x30(%rsp)
   25a0e:	movq   $0x0,0x28(%rsp)
   25a17:	imul   $0x174,%rbp,%rbx
   25a1e:	test   %rbx,%rbx
   25a21:	jns    25a37 <meshopt_peers::run+0xb77>
   25a23:	xor    %r15d,%r15d
   25a26:	mov    %r15,%rdi
   25a29:	mov    %rbx,%rsi
   25a2c:	call   *0x6284e(%rip)        # 88280 <_DYNAMIC+0x218>
   25a32:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25a37:	je     25a61 <meshopt_peers::run+0xba1>
   25a39:	mov    $0x1,%r15d
   25a3f:	mov    $0x1,%esi
   25a44:	mov    %rbx,%rdi
   25a47:	call   *0x62b03(%rip)        # 88550 <calloc@GLIBC_2.2.5>
   25a4d:	test   %rax,%rax
   25a50:	je     25a26 <meshopt_peers::run+0xb66>
   25a52:	mov    0x20(%rsp),%r9
   25a57:	mov    0x10(%rsp),%rcx
   25a5c:	mov    %rax,%r15
   25a5f:	jmp    25a71 <meshopt_peers::run+0xbb1>
   25a61:	mov    $0x1,%r15d
   25a67:	mov    0x20(%rsp),%r9
   25a6c:	mov    0x10(%rsp),%rcx
   25a71:	xorps  %xmm0,%xmm0
   25a74:	mov    (%rsp),%rdi
   25a78:	mov    0x30(%rsp),%rsi
   25a7d:	mov    %r15,%rdx
   25a80:	mov    %r14,%r8
   25a83:	push   $0x7c
   25a85:	push   $0x40
   25a87:	push   $0xc
   25a89:	push   0x20(%rsp)
   25a8d:	call   *0x62c0d(%rip)        # 886a0 <_DYNAMIC+0x638>
   25a93:	add    $0x20,%rsp
   25a97:	cmp    %rbp,%rax
   25a9a:	cmovb  %rax,%rbp
   25a9e:	test   %rbp,%rbp
   25aa1:	je     25ada <meshopt_peers::run+0xc1a>
   25aa3:	mov    %rbx,%r14
   25aa6:	mov    %rbp,%rbx
   25aa9:	shl    $0x4,%rbx
   25aad:	mov    %rbx,%rdi
   25ab0:	call   *0x6281a(%rip)        # 882d0 <malloc@GLIBC_2.2.5>
   25ab6:	test   %rax,%rax
   25ab9:	je     25c71 <meshopt_peers::run+0xdb1>
   25abf:	mov    %ebp,%ecx
   25ac1:	and    $0x3,%ecx
   25ac4:	cmp    $0x4,%rbp
   25ac8:	mov    (%rsp),%rdi
   25acc:	jae    25aea <meshopt_peers::run+0xc2a>
   25ace:	xor    %edx,%edx
   25ad0:	mov    0x18(%rsp),%r8
   25ad5:	mov    %r14,%rbx
   25ad8:	jmp    25b34 <meshopt_peers::run+0xc74>
   25ada:	mov    $0x4,%eax
   25adf:	mov    0x18(%rsp),%r8
   25ae4:	mov    (%rsp),%rdi
   25ae8:	jmp    25b61 <meshopt_peers::run+0xca1>
   25aea:	and    %rbp,%r13
   25aed:	mov    $0x30,%esi
   25af2:	xor    %edx,%edx
   25af4:	mov    0x18(%rsp),%r8
   25af9:	mov    %r14,%rbx
   25afc:	movups -0x30(%rdi,%rsi,1),%xmm0
   25b01:	movups %xmm0,-0x30(%rax,%rsi,1)
   25b06:	movups -0x20(%rdi,%rsi,1),%xmm0
   25b0b:	movups %xmm0,-0x20(%rax,%rsi,1)
   25b10:	movups -0x10(%rdi,%rsi,1),%xmm0
   25b15:	movups %xmm0,-0x10(%rax,%rsi,1)
   25b1a:	movups (%rdi,%rsi,1),%xmm0
   25b1e:	movups %xmm0,(%rax,%rsi,1)
   25b22:	add    $0x4,%rdx
   25b26:	add    $0x40,%rsi
   25b2a:	cmp    %rdx,%r13
   25b2d:	jne    25afc <meshopt_peers::run+0xc3c>
   25b2f:	test   %rcx,%rcx
   25b32:	je     25b61 <meshopt_peers::run+0xca1>
   25b34:	shl    $0x4,%rdx
   25b38:	mov    %rax,%rsi
   25b3b:	add    %rdx,%rsi
   25b3e:	add    %rdi,%rdx
   25b41:	shl    $0x4,%ecx
   25b44:	xor    %r9d,%r9d
   25b47:	mov    0x28(%rsp),%r10
   25b4c:	movups (%rdx,%r9,1),%xmm0
   25b51:	movups %xmm0,(%rsi,%r9,1)
   25b56:	add    $0x10,%r9
   25b5a:	cmp    %r9,%rcx
   25b5d:	jne    25b4c <meshopt_peers::run+0xc8c>
   25b5f:	jmp    25b66 <meshopt_peers::run+0xca6>
   25b61:	mov    0x28(%rsp),%r10
   25b66:	mov    %r10,0x20(%r8)
   25b6a:	mov    0x30(%rsp),%rcx
   25b6f:	mov    %rcx,0x28(%r8)
   25b73:	mov    %r12,0x30(%r8)
   25b77:	mov    %rbx,0x38(%r8)
   25b7b:	mov    %r15,0x40(%r8)
   25b7f:	mov    %rbx,0x48(%r8)
   25b83:	mov    %rbp,0x8(%r8)
   25b87:	mov    %rax,0x10(%r8)
   25b8b:	mov    %rbp,0x18(%r8)
   25b8f:	movl   $0x5,(%r8)
   25b96:	cmpq   $0x0,0x38(%rsp)
   25b9c:	je     25886 <meshopt_peers::run+0x9c6>
   25ba2:	add    $0x138,%rsp
   25ba9:	pop    %rbx
   25baa:	pop    %r12
   25bac:	pop    %r13
   25bae:	pop    %r14
   25bb0:	pop    %r15
   25bb2:	pop    %rbp
   25bb3:	jmp    *0x626ff(%rip)        # 882b8 <free@GLIBC_2.2.5>
   25bb9:	lea    0x60(%rsp),%rax
   25bbe:	mov    %rax,0x70(%rsp)
   25bc3:	lea    0x4a66(%rip),%rax        # 2a630 <<&str as core::fmt::Display>::fmt>
   25bca:	mov    %rax,0x78(%rsp)
   25bcf:	lea    -0x1f116(%rip),%rdi        # 6ac0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   25bd6:	lea    0x608fb(%rip),%rdx        # 864d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   25bdd:	lea    0x70(%rsp),%rsi
   25be2:	call   *0x62718(%rip)        # 88300 <_DYNAMIC+0x298>
   25be8:	lea    -0x1bc3f(%rip),%rdi        # 9fb0 <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0xcb>
   25bef:	lea    0x60852(%rip),%rdx        # 86448 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x18>
   25bf6:	mov    $0x97,%esi
   25bfb:	call   *0x626ff(%rip)        # 88300 <_DYNAMIC+0x298>
   25c01:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25c03:	mov    $0x4,%edi
   25c08:	mov    %rbx,%rsi
   25c0b:	call   *0x6266f(%rip)        # 88280 <_DYNAMIC+0x218>
   25c11:	mov    $0x1,%edi
   25c16:	mov    %r15,%rsi
   25c19:	call   *0x62661(%rip)        # 88280 <_DYNAMIC+0x218>
   25c1f:	mov    $0x4,%edi
   25c24:	mov    %r15,%rsi
   25c27:	call   *0x62653(%rip)        # 88280 <_DYNAMIC+0x218>
   25c2d:	mov    $0x4,%edi
   25c32:	mov    %r12,%rsi
   25c35:	call   *0x62645(%rip)        # 88280 <_DYNAMIC+0x218>
   25c3b:	mov    $0x1,%edi
   25c40:	mov    %r12,%rsi
   25c43:	call   *0x62637(%rip)        # 88280 <_DYNAMIC+0x218>
   25c49:	jmp    25c7f <meshopt_peers::run+0xdbf>
   25c4b:	mov    $0x4,%edi
   25c50:	mov    %rbx,%rsi
   25c53:	call   *0x62627(%rip)        # 88280 <_DYNAMIC+0x218>
   25c59:	mov    $0x4,%edi
   25c5e:	mov    %r15,%rsi
   25c61:	call   *0x62619(%rip)        # 88280 <_DYNAMIC+0x218>
   25c67:	mov    $0x4,%edi
   25c6c:	jmp    25971 <meshopt_peers::run+0xab1>
   25c71:	mov    $0x4,%edi
   25c76:	mov    %rbx,%rsi
   25c79:	call   *0x62601(%rip)        # 88280 <_DYNAMIC+0x218>
   25c7f:	ud2
   25c81:	mov    %rax,%rbx
   25c84:	test   %r14,%r14
   25c87:	je     25ce1 <meshopt_peers::run+0xe21>
   25c89:	mov    %r15,%rdi
   25c8c:	call   *0x62626(%rip)        # 882b8 <free@GLIBC_2.2.5>
   25c92:	jmp    25ce1 <meshopt_peers::run+0xe21>
   25c94:	mov    %rax,%rbx
   25c97:	test   %r14,%r14
   25c9a:	je     25d10 <meshopt_peers::run+0xe50>
   25c9c:	mov    %r15,%rdi
   25c9f:	jmp    25d0a <meshopt_peers::run+0xe4a>
   25ca1:	mov    %rax,%rbx
   25ca4:	test   %r12,%r12
   25ca7:	je     25d10 <meshopt_peers::run+0xe50>
   25ca9:	mov    (%rsp),%rdi
   25cad:	jmp    25d0a <meshopt_peers::run+0xe4a>
   25caf:	mov    %rax,%rbx
   25cb2:	cmpq   $0x0,0x48(%rsp)
   25cb8:	je     25cc5 <meshopt_peers::run+0xe05>
   25cba:	mov    0x50(%rsp),%rdi
   25cbf:	call   *0x625f3(%rip)        # 882b8 <free@GLIBC_2.2.5>
   25cc5:	test   %r13,%r13
   25cc8:	je     25d10 <meshopt_peers::run+0xe50>
   25cca:	mov    0x8(%rsp),%rdi
   25ccf:	jmp    25d0a <meshopt_peers::run+0xe4a>
   25cd1:	mov    %rax,%rbx
   25cd4:	test   %r15,%r15
   25cd7:	je     25d10 <meshopt_peers::run+0xe50>
   25cd9:	mov    %r13,%rdi
   25cdc:	jmp    25d0a <meshopt_peers::run+0xe4a>
   25cde:	mov    %rax,%rbx
   25ce1:	mov    (%rsp),%r14
   25ce5:	mov    0x28(%rsp),%rax
   25cea:	test   %rax,%rax
   25ced:	je     25cff <meshopt_peers::run+0xe3f>
   25cef:	mov    0x30(%rsp),%rdi
   25cf4:	call   *0x625be(%rip)        # 882b8 <free@GLIBC_2.2.5>
   25cfa:	jmp    25cff <meshopt_peers::run+0xe3f>
   25cfc:	mov    %rax,%rbx
   25cff:	cmpq   $0x0,0x38(%rsp)
   25d05:	je     25d10 <meshopt_peers::run+0xe50>
   25d07:	mov    %r14,%rdi
   25d0a:	call   *0x625a8(%rip)        # 882b8 <free@GLIBC_2.2.5>
   25d10:	mov    %rbx,%rdi
   25d13:	call   84950 <_Unwind_Resume@plt>
