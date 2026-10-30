
<artifacts>/screen2/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000034bf0 <meshopt_peers::run>:
   34bf0:	push   %rbp
   34bf1:	push   %r15
   34bf3:	push   %r14
   34bf5:	push   %r13
   34bf7:	push   %r12
   34bf9:	push   %rbx
   34bfa:	sub    $0x1000,%rsp
   34c01:	movq   $0x0,(%rsp)
   34c09:	sub    $0x968,%rsp
   34c10:	mov    %rsi,0x290(%rsp)
   34c18:	mov    %rdx,0x298(%rsp)
   34c20:	mov    0x10(%rcx),%rbx
   34c24:	mov    0x8(%rcx),%r15
   34c28:	movb   $0x2,0x6e0(%rsp)
   34c30:	mov    %r15,0x6e8(%rsp)
   34c38:	mov    %rbx,0x6f0(%rsp)
   34c40:	movq   $0x0,0x718(%rsp)
   34c4c:	mov    0x78(%rcx),%rax
   34c50:	movd   0x80(%rcx),%xmm0
   34c58:	mov    %rax,0x220(%rsp)
   34c60:	movd   %xmm0,0x228(%rsp)
   34c69:	movl   $0x0,0x22c(%rsp)
   34c74:	add    $0xfffffffffffffffb,%rdx
   34c78:	cmp    $0xe,%rdx
   34c7c:	ja     36d4e <meshopt_peers::run+0x215e>
   34c82:	mov    %r8,%r12
   34c85:	mov    %rdi,%r14
   34c88:	lea    -0x24793(%rip),%rdi        # 104fc <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   34c8f:	movslq (%rdi,%rdx,4),%rdx
   34c93:	add    %rdi,%rdx
   34c96:	jmp    *%rdx
   34c98:	mov    $0x68636163,%eax
   34c9d:	xor    (%rsi),%eax
   34c9f:	movzbl 0x4(%rsi),%edx
   34ca3:	xor    $0x65,%edx
   34ca6:	or     %eax,%edx
   34ca8:	je     35492 <meshopt_peers::run+0x8a2>
   34cae:	mov    $0x63746566,%eax
   34cb3:	xor    (%rsi),%eax
   34cb5:	movzbl 0x4(%rsi),%edx
   34cb9:	xor    $0x68,%edx
   34cbc:	or     %eax,%edx
   34cbe:	jne    36d4e <meshopt_peers::run+0x215e>
   34cc4:	mov    %rbx,%rax
   34cc7:	shr    $0x20,%rax
   34ccb:	jne    371a1 <meshopt_peers::run+0x25b1>
   34cd1:	mov    0x20(%rcx),%rsi
   34cd5:	mov    0x28(%rcx),%rdx
   34cd9:	lea    0x0(,%rbx,4),%rax
   34ce1:	lea    (%rax,%rax,2),%rax
   34ce5:	mov    %r15,0x350(%rsp)
   34ced:	mov    %rax,0x358(%rsp)
   34cf5:	mov    %rbx,0x360(%rsp)
   34cfd:	movq   $0xc,0x368(%rsp)
   34d09:	movq   $0xc,0x370(%rsp)
   34d15:	movq   $0x0,0x378(%rsp)
   34d21:	lea    0x738(%rsp),%rdi
   34d29:	lea    0x350(%rsp),%rcx
   34d31:	mov    %r12,%r8
   34d34:	call   *0xf47d6(%rip)        # 129510 <_DYNAMIC+0x5d0>
   34d3a:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34d43:	je     36dd5 <meshopt_peers::run+0x21e5>
   34d49:	mov    0x768(%rsp),%rax
   34d51:	mov    %rax,0x380(%rsp)
   34d59:	movups 0x738(%rsp),%xmm0
   34d61:	movups 0x748(%rsp),%xmm1
   34d69:	movups 0x758(%rsp),%xmm2
   34d71:	movaps %xmm2,0x370(%rsp)
   34d79:	movaps %xmm1,0x360(%rsp)
   34d81:	movaps %xmm0,0x350(%rsp)
   34d89:	mov    0x360(%rsp),%rax
   34d91:	mov    %rax,0x18(%r14)
   34d95:	movups %xmm0,0x8(%r14)
   34d9a:	movups 0x368(%rsp),%xmm0
   34da2:	movups %xmm0,0x20(%r14)
   34da7:	mov    0x378(%rsp),%rax
   34daf:	mov    %rax,0x30(%r14)
   34db3:	movl   $0x3,(%r14)
   34dba:	jmp    3598a <meshopt_peers::run+0xd9a>
   34dbf:	movabs $0x7966696c706d6973,%rdx
   34dc9:	xor    (%rsi),%rdx
   34dcc:	movabs $0x7970706f6c732d79,%rdi
   34dd6:	xor    0x7(%rsi),%rdi
   34dda:	or     %rdx,%rdi
   34ddd:	jne    36d4e <meshopt_peers::run+0x215e>
   34de3:	movdqu 0x20(%rcx),%xmm1
   34de8:	movq   $0x0,0x1d0(%rsp)
   34df4:	mov    %rax,0x170(%rsp)
   34dfc:	movd   %xmm0,0x230(%rsp)
   34e05:	movb   $0x0,0x365(%r12)
   34e0e:	movl   $0x0,0x361(%r12)
   34e1a:	pxor   %xmm0,%xmm0
   34e1e:	movdqu %xmm0,0x350(%r12)
   34e28:	mov    0x348(%r12),%rax
   34e30:	mov    %rax,0xe0(%rsp)
   34e38:	mov    %rax,0xe8(%rsp)
   34e40:	movdqu %xmm1,0x738(%rsp)
   34e49:	lea    0x6e0(%rsp),%rax
   34e51:	mov    %rax,0x748(%rsp)
   34e59:	lea    0xe0(%rsp),%rax
   34e61:	mov    %rax,0x750(%rsp)
   34e69:	lea    0x1d0(%rsp),%rax
   34e71:	mov    %rax,0x758(%rsp)
   34e79:	lea    0x170(%rsp),%rax
   34e81:	mov    %rax,0x760(%rsp)
   34e89:	lea    0x230(%rsp),%rax
   34e91:	mov    %rax,0x768(%rsp)
   34e99:	mov    %r12,0x770(%rsp)
   34ea1:	movq   $0x0,0x778(%rsp)
   34ead:	lea    0x350(%rsp),%rdi
   34eb5:	lea    0x738(%rsp),%rsi
   34ebd:	call   858e0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   34ec2:	mov    0xe8(%rsp),%rax
   34eca:	sub    0xe0(%rsp),%rax
   34ed2:	mov    %rax,0x358(%r12)
   34eda:	mov    0x350(%rsp),%rax
   34ee2:	movzbl 0x358(%rsp),%ecx
   34eea:	cmp    $0xffffffffffffffff,%rax
   34eee:	je     34f12 <meshopt_peers::run+0x322>
   34ef0:	mov    0x368(%rsp),%rdx
   34ef8:	mov    %rdx,0x2d8(%rsp)
   34f00:	movdqu 0x359(%rsp),%xmm0
   34f09:	movdqu %xmm0,0x2c9(%rsp)
   34f12:	mov    %cl,0x2c8(%rsp)
   34f19:	mov    %rax,0x2c0(%rsp)
   34f21:	jmp    3538c <meshopt_peers::run+0x79c>
   34f26:	movdqu (%rsi),%xmm0
   34f2a:	movdqu 0x3(%rsi),%xmm1
   34f2f:	pcmpeqb -0x26ee7(%rip),%xmm1        # e050 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   34f37:	pcmpeqb -0x2736f(%rip),%xmm0        # dbd0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   34f3f:	pand   %xmm1,%xmm0
   34f43:	pmovmskb %xmm0,%eax
   34f47:	cmp    $0xffff,%eax
   34f4c:	jne    36d4e <meshopt_peers::run+0x215e>
   34f52:	mov    0x40(%rcx),%rax
   34f56:	test   %rbx,%rbx
   34f59:	setne  %dl
   34f5c:	lea    0x0(,%rbx,4),%rsi
   34f64:	cmp    %rax,%rsi
   34f67:	seta   %sil
   34f6b:	and    %dl,%sil
   34f6e:	cmp    $0x1,%sil
   34f72:	je     37158 <meshopt_peers::run+0x2568>
   34f78:	mov    0x20(%rcx),%rsi
   34f7c:	mov    0x28(%rcx),%rdx
   34f80:	mov    0x38(%rcx),%rcx
   34f84:	movb   $0x0,0x738(%rsp)
   34f8c:	mov    %rcx,0x740(%rsp)
   34f94:	mov    %rax,0x748(%rsp)
   34f9c:	mov    %rbx,0x750(%rsp)
   34fa4:	movq   $0x4,0x758(%rsp)
   34fb0:	movq   $0x4,0x760(%rsp)
   34fbc:	pxor   %xmm0,%xmm0
   34fc0:	movdqu %xmm0,0x768(%rsp)
   34fc9:	sub    $0x30,%rsp
   34fcd:	mov    %r12,0x20(%rsp)
   34fd2:	lea    0x250(%rsp),%rax
   34fda:	mov    %rax,0x18(%rsp)
   34fdf:	movq   $0x0,0x8(%rsp)
   34fe8:	movq   $0x4,(%rsp)
   34ff0:	lea    -0x26f57(%rip),%r9        # e0a0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   34ff7:	lea    0x2f0(%rsp),%rdi
   34fff:	lea    0x710(%rsp),%rcx
   35007:	lea    0x768(%rsp),%r8
   3500f:	jmp    3537b <meshopt_peers::run+0x78b>
   35014:	movabs $0x7966696c706d6973,%rax
   3501e:	cmp    %rax,(%rsi)
   35021:	je     3532e <meshopt_peers::run+0x73e>
   35027:	movabs $0x776172647265766f,%rax
   35031:	cmp    %rax,(%rsi)
   35034:	je     35850 <meshopt_peers::run+0xc60>
   3503a:	movabs $0x7374656c6873656d,%rax
   35044:	cmp    %rax,(%rsi)
   35047:	jne    36d4e <meshopt_peers::run+0x215e>
   3504d:	mov    0x20(%rcx),%rsi
   35051:	mov    0x28(%rcx),%rdx
   35055:	movq   $0x7c,0x360(%rsp)
   35061:	movq   $0x0,0x354(%rsp)
   3506d:	movl   $0x1,0x350(%rsp)
   35078:	lea    0x350(%rsp),%rax
   35080:	lea    0x738(%rsp),%rdi
   35088:	lea    0x6e0(%rsp),%r8
   35090:	mov    $0x40,%r9d
   35096:	mov    %rbx,%rcx
   35099:	push   $0x2
   3509b:	push   %r12
   3509d:	push   %rax
   3509e:	push   $0x7c
   350a0:	call   b3e80 <meshoptimizer_rs::meshlet::build>
   350a5:	add    $0x20,%rsp
   350a9:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   350b2:	je     36e55 <meshopt_peers::run+0x2265>
   350b8:	mov    0x778(%rsp),%rax
   350c0:	mov    %rax,0x390(%rsp)
   350c8:	movups 0x738(%rsp),%xmm0
   350d0:	movups 0x748(%rsp),%xmm1
   350d8:	movups 0x758(%rsp),%xmm2
   350e0:	movups 0x768(%rsp),%xmm3
   350e8:	movaps %xmm3,0x380(%rsp)
   350f0:	movaps %xmm2,0x370(%rsp)
   350f8:	movaps %xmm1,0x360(%rsp)
   35100:	movaps %xmm0,0x350(%rsp)
   35108:	mov    0x358(%rsp),%rbx
   35110:	mov    0x360(%rsp),%r12
   35118:	test   %r12,%r12
   3511b:	je     358ad <meshopt_peers::run+0xcbd>
   35121:	mov    %r12,%r15
   35124:	shl    $0x4,%r15
   35128:	mov    %r15,%rdi
   3512b:	call   *0xf4007(%rip)        # 129138 <malloc@GLIBC_2.2.5>
   35131:	test   %rax,%rax
   35134:	je     371f4 <meshopt_peers::run+0x2604>
   3513a:	mov    %r12d,%ecx
   3513d:	and    $0x3,%ecx
   35140:	cmp    $0x4,%r12
   35144:	jae    358b4 <meshopt_peers::run+0xcc4>
   3514a:	xor    %edx,%edx
   3514c:	jmp    35908 <meshopt_peers::run+0xd18>
   35151:	movabs $0x65646f6365642d78,%rax
   3515b:	movabs $0x652d786574726576,%rdx
   35165:	xor    (%rsi),%rdx
   35168:	lea    0x9010000(%rax),%rdi
   3516f:	xor    0x5(%rsi),%rdi
   35173:	or     %rdx,%rdi
   35176:	je     353ca <meshopt_peers::run+0x7da>
   3517c:	movabs $0x642d786574726576,%rdx
   35186:	xor    (%rsi),%rdx
   35189:	xor    0x5(%rsi),%rax
   3518d:	or     %rdx,%rax
   35190:	jne    36d4e <meshopt_peers::run+0x215e>
   35196:	mov    0x50(%rcx),%r9
   3519a:	lea    0x738(%rsp),%rdi
   351a2:	mov    $0xc,%r8d
   351a8:	xor    %esi,%esi
   351aa:	xor    %edx,%edx
   351ac:	mov    %rcx,%rax
   351af:	mov    %rbx,%rcx
   351b2:	push   %r12
   351b4:	push   0x58(%rax)
   351b7:	call   b33a0 <meshoptimizer_rs::codec::allocate>
   351bc:	add    $0x10,%rsp
   351c0:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   351c9:	jne    3546d <meshopt_peers::run+0x87d>
   351cf:	movzbl 0x740(%rsp),%eax
   351d7:	mov    %al,0x350(%rsp)
   351de:	lea    -0x24c32(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   351e5:	lea    0xece94(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   351ec:	lea    0xecb5d(%rip),%r8        # 121d50 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   351f3:	lea    0x350(%rsp),%rdx
   351fb:	mov    $0x2b,%esi
   35200:	call   *0xf400a(%rip)        # 129210 <_DYNAMIC+0x2d0>
   35206:	movabs $0x6e652d7865646e69,%rax
   35210:	xor    (%rsi),%rax
   35213:	mov    0x8(%rsi),%edx
   35216:	xor    $0x65646f63,%rdx
   3521d:	or     %rax,%rdx
   35220:	je     35440 <meshopt_peers::run+0x850>
   35226:	movabs $0x65642d7865646e69,%rax
   35230:	xor    (%rsi),%rax
   35233:	mov    0x8(%rsi),%edx
   35236:	xor    $0x65646f63,%rdx
   3523d:	or     %rax,%rdx
   35240:	jne    36d4e <meshopt_peers::run+0x215e>
   35246:	mov    0x28(%rcx),%rax
   3524a:	mov    0x68(%rcx),%r9
   3524e:	lea    0x738(%rsp),%rdi
   35256:	mov    $0x4,%r8d
   3525c:	mov    $0x1,%esi
   35261:	xor    %edx,%edx
   35263:	mov    %rcx,%r10
   35266:	mov    %rax,%rcx
   35269:	push   %r12
   3526b:	push   0x70(%r10)
   3526f:	call   b33a0 <meshoptimizer_rs::codec::allocate>
   35274:	add    $0x10,%rsp
   35278:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35281:	jne    3546d <meshopt_peers::run+0x87d>
   35287:	movzbl 0x740(%rsp),%eax
   3528f:	mov    %al,0x350(%rsp)
   35296:	lea    -0x24cea(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3529d:	lea    0xecddc(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   352a4:	lea    0xeca75(%rip),%r8        # 121d20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x138>
   352ab:	lea    0x350(%rsp),%rdx
   352b3:	mov    $0x2b,%esi
   352b8:	call   *0xf3f52(%rip)        # 129210 <_DYNAMIC+0x2d0>
   352be:	movabs $0x2d72657473756c63,%rax
   352c8:	xor    (%rsi),%rax
   352cb:	movabs $0x646f6c2d72657473,%rdx
   352d5:	xor    0x3(%rsi),%rdx
   352d9:	or     %rax,%rdx
   352dc:	jne    36d4e <meshopt_peers::run+0x215e>
   352e2:	mov    %r14,0x288(%rsp)
   352ea:	test   %rbx,%rbx
   352ed:	je     354f4 <meshopt_peers::run+0x904>
   352f3:	mov    %rcx,%rbp
   352f6:	lea    0x0(,%rbx,4),%rax
   352fe:	lea    (%rax,%rax,2),%r13
   35302:	mov    %r13,%rdi
   35305:	call   *0xf3e2d(%rip)        # 129138 <malloc@GLIBC_2.2.5>
   3530b:	test   %rax,%rax
   3530e:	je     371d1 <meshopt_peers::run+0x25e1>
   35314:	mov    %rax,%r14
   35317:	mov    %rax,%rdi
   3531a:	mov    %r15,%rsi
   3531d:	mov    %r13,%rdx
   35320:	call   *0xf3e32(%rip)        # 129158 <memcpy@GLIBC_2.14>
   35326:	mov    %rbp,%rcx
   35329:	jmp    354fa <meshopt_peers::run+0x90a>
   3532e:	mov    0x20(%rcx),%rsi
   35332:	mov    0x28(%rcx),%rdx
   35336:	movb   $0x2,0x738(%rsp)
   3533e:	sub    $0x30,%rsp
   35342:	mov    %r12,0x20(%rsp)
   35347:	lea    0x250(%rsp),%rax
   3534f:	mov    %rax,0x18(%rsp)
   35354:	pxor   %xmm0,%xmm0
   35358:	movdqu %xmm0,(%rsp)
   3535d:	lea    0x2f0(%rsp),%rdi
   35365:	lea    0x710(%rsp),%rcx
   3536d:	lea    0x768(%rsp),%r8
   35375:	mov    $0x4,%r9d
   3537b:	call   beb20 <meshoptimizer_rs::simplify::allocating>
   35380:	add    $0x30,%rsp
   35384:	mov    0x2c0(%rsp),%rax
   3538c:	cmp    $0xffffffffffffffff,%rax
   35390:	je     36d17 <meshopt_peers::run+0x2127>
   35396:	mov    0x2d0(%rsp),%rax
   3539e:	mov    %rax,0x18(%r14)
   353a2:	movups 0x2c0(%rsp),%xmm0
   353aa:	movups %xmm0,0x8(%r14)
   353af:	movss  0x2d8(%rsp),%xmm0
   353b8:	movss  %xmm0,0x4(%r14)
   353be:	movl   $0x2,(%r14)
   353c5:	jmp    3598a <meshopt_peers::run+0xd9a>
   353ca:	lea    0x0(,%rbx,4),%rax
   353d2:	lea    (%rax,%rax,2),%rdx
   353d6:	lea    0x738(%rsp),%rdi
   353de:	mov    $0xc,%r8d
   353e4:	mov    %r15,%rsi
   353e7:	mov    %rbx,%rcx
   353ea:	mov    $0x1,%r9d
   353f0:	push   %r12
   353f2:	push   $0x2
   353f4:	call   *0xf411e(%rip)        # 129518 <_DYNAMIC+0x5d8>
   353fa:	add    $0x10,%rsp
   353fe:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35407:	jne    3546d <meshopt_peers::run+0x87d>
   35409:	movzbl 0x740(%rsp),%eax
   35411:	mov    %al,0x350(%rsp)
   35418:	lea    -0x24e6c(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3541f:	lea    0xecc5a(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   35426:	lea    0xec93b(%rip),%r8        # 121d68 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   3542d:	lea    0x350(%rsp),%rdx
   35435:	mov    $0x2b,%esi
   3543a:	call   *0xf3dd0(%rip)        # 129210 <_DYNAMIC+0x2d0>
   35440:	mov    0x20(%rcx),%rsi
   35444:	mov    0x28(%rcx),%rdx
   35448:	lea    0x738(%rsp),%rdi
   35450:	mov    $0x1,%ecx
   35455:	mov    %r12,%r8
   35458:	call   *0xf40c2(%rip)        # 129520 <_DYNAMIC+0x5e0>
   3545e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35467:	je     36e8c <meshopt_peers::run+0x229c>
   3546d:	mov    0x748(%rsp),%rax
   35475:	mov    %rax,0x18(%r14)
   35479:	movups 0x738(%rsp),%xmm0
   35481:	movups %xmm0,0x8(%r14)
   35486:	movl   $0x0,(%r14)
   3548d:	jmp    3598a <meshopt_peers::run+0xd9a>
   35492:	mov    0x20(%rcx),%rsi
   35496:	mov    0x28(%rcx),%rdx
   3549a:	lea    0x738(%rsp),%rdi
   354a2:	mov    %rbx,%rcx
   354a5:	mov    %r12,%r8
   354a8:	call   *0xf407a(%rip)        # 129528 <_DYNAMIC+0x5e8>
   354ae:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   354b7:	jne    35888 <meshopt_peers::run+0xc98>
   354bd:	movzbl 0x740(%rsp),%eax
   354c5:	mov    %al,0x350(%rsp)
   354cc:	lea    -0x24f20(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   354d3:	lea    0xecba6(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   354da:	lea    0xec8ff(%rip),%r8        # 121de0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   354e1:	lea    0x350(%rsp),%rdx
   354e9:	mov    $0x2b,%esi
   354ee:	call   *0xf3d1c(%rip)        # 129210 <_DYNAMIC+0x2d0>
   354f4:	mov    $0x4,%r14d
   354fa:	movq   $0x0,0x158(%rsp)
   35506:	movq   $0x4,0x160(%rsp)
   35512:	movq   $0x0,0x168(%rsp)
   3551e:	movl   $0x0,0x9c(%rsp)
   35529:	movq   $0x80,0x1d0(%rsp)
   35535:	movq   $0x2a,0x1d8(%rsp)
   35541:	movq   $0x80,0x1e0(%rsp)
   3554d:	movq   $0x10,0x1e8(%rsp)
   35559:	movaps -0x27670(%rip),%xmm0        # def0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   35560:	movups %xmm0,0x1f0(%rsp)
   35568:	movaps -0x275bf(%rip),%xmm0        # dfb0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3556f:	movups %xmm0,0x200(%rsp)
   35577:	movabs $0x100010100000101,%rax
   35581:	mov    %rax,0x210(%rsp)
   35589:	movl   $0x0,0x218(%rsp)
   35594:	movb   $0x1,0x21c(%rsp)
   3559c:	mov    0x20(%rcx),%r13
   355a0:	mov    0x28(%rcx),%r15
   355a4:	mov    %r13,0x308(%rsp)
   355ac:	mov    %r15,0x310(%rsp)
   355b4:	mov    %r14,0x50(%rsp)
   355b9:	mov    %r14,0x318(%rsp)
   355c1:	mov    %rbx,0x20(%rsp)
   355c6:	mov    %rbx,0x320(%rsp)
   355ce:	movb   $0x2,0x2c0(%rsp)
   355d6:	movq   $0x4,0x328(%rsp)
   355e2:	xorps  %xmm0,%xmm0
   355e5:	movups %xmm0,0x330(%rsp)
   355ed:	movl   $0x0,0x348(%rsp)
   355f8:	movb   $0x0,0x365(%r12)
   35601:	movl   $0x0,0x361(%r12)
   3560d:	movups %xmm0,0x350(%r12)
   35616:	mov    0x348(%r12),%rbx
   3561e:	mov    %r12,%rdi
   35621:	xor    %esi,%esi
   35623:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35628:	mov    %eax,%ebp
   3562a:	cmp    $0xff,%al
   3562c:	jne    372a5 <meshopt_peers::run+0x26b5>
   35632:	mov    %r12,0x738(%rsp)
   3563a:	mov    %rbx,0x740(%rsp)
   35642:	mov    %rbx,0x748(%rsp)
   3564a:	movq   $0x0,0x750(%rsp)
   35656:	movl   $0x0,0x758(%rsp)
   35661:	movl   $0x0,0x95c(%rsp)
   3566c:	movb   $0x1,0x1960(%rsp)
   35674:	lea    0x2c0(%rsp),%rdi
   3567c:	lea    0x1d0(%rsp),%rsi
   35684:	lea    0x738(%rsp),%rdx
   3568c:	mov    0x20(%rsp),%rbx
   35691:	call   *0xf3e99(%rip)        # 129530 <_DYNAMIC+0x5f0>
   35697:	mov    %eax,%ebp
   35699:	cmp    $0xff,%al
   3569b:	jne    37286 <meshopt_peers::run+0x2696>
   356a1:	movzbl 0x1960(%rsp),%r12d
   356aa:	movb   $0x2,0x350(%rsp)
   356b2:	mov    0x50(%rsp),%rax
   356b7:	mov    %rax,0x358(%rsp)
   356bf:	mov    %rbx,0x360(%rsp)
   356c7:	movq   $0x0,0x388(%rsp)
   356d3:	lea    0xe0(%rsp),%rdi
   356db:	lea    0x350(%rsp),%rsi
   356e3:	lea    0x738(%rsp),%rdx
   356eb:	call   *0xf3e47(%rip)        # 129538 <_DYNAMIC+0x5f8>
   356f1:	mov    0xe0(%rsp),%r14
   356f9:	movzbl 0xe8(%rsp),%ebp
   35701:	cmp    $0xffffffffffffffff,%r14
   35705:	je     357ee <meshopt_peers::run+0xbfe>
   3570b:	mov    %r14,0x150(%rsp)
   35713:	movzbl %bpl,%eax
   35717:	movzbl 0xef(%rsp),%ecx
   3571f:	shl    $0x10,%ecx
   35722:	movzwl 0xed(%rsp),%edx
   3572a:	or     %ecx,%edx
   3572c:	mov    0xe9(%rsp),%ecx
   35733:	shl    $0x28,%rdx
   35737:	shl    $0x8,%rcx
   3573b:	or     %rdx,%rcx
   3573e:	or     %rax,%rcx
   35741:	mov    %rcx,0xc8(%rsp)
   35749:	mov    0x750(%rsp),%rsi
   35751:	add    %rbx,%rsi
   35754:	jb     36e0c <meshopt_peers::run+0x221c>
   3575a:	mov    0xf0(%rsp),%rax
   35762:	mov    %rax,0x1c8(%rsp)
   3576a:	mov    %rsi,0x750(%rsp)
   35772:	mov    0x738(%rsp),%rdi
   3577a:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3577f:	mov    %eax,%ebp
   35781:	cmp    $0xff,%al
   35783:	jne    357d5 <meshopt_peers::run+0xbe5>
   35785:	mov    $0x1,%eax
   3578a:	mov    %rax,0x90(%rsp)
   35792:	test   %rbx,%rbx
   35795:	jne    35df3 <meshopt_peers::run+0x1203>
   3579b:	mov    0x738(%rsp),%rdi
   357a3:	mov    0x750(%rsp),%rsi
   357ab:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   357b0:	mov    %eax,%ebp
   357b2:	mov    $0x1,%r14b
   357b5:	cmp    $0xff,%al
   357b7:	je     3599c <meshopt_peers::run+0xdac>
   357bd:	test   %rbx,%rbx
   357c0:	je     357d0 <meshopt_peers::run+0xbe0>
   357c2:	mov    0x90(%rsp),%rdi
   357ca:	call   *0xf3978(%rip)        # 129148 <free@GLIBC_2.2.5>
   357d0:	test   %r14b,%r14b
   357d3:	je     357ee <meshopt_peers::run+0xbfe>
   357d5:	cmpq   $0x0,0x150(%rsp)
   357de:	je     357ee <meshopt_peers::run+0xbfe>
   357e0:	mov    0xc8(%rsp),%rdi
   357e8:	call   *0xf395a(%rip)        # 129148 <free@GLIBC_2.2.5>
   357ee:	mov    0x738(%rsp),%rax
   357f6:	mov    0x748(%rsp),%rcx
   357fe:	sub    0x740(%rsp),%rcx
   35806:	mov    %rcx,0x358(%rax)
   3580d:	cmp    $0xff,%bpl
   35811:	jne    372a5 <meshopt_peers::run+0x26b5>
   35817:	mov    0x168(%rsp),%rax
   3581f:	mov    0x288(%rsp),%rcx
   35827:	mov    %rax,0x18(%rcx)
   3582b:	movups 0x158(%rsp),%xmm0
   35833:	movups %xmm0,0x8(%rcx)
   35837:	movl   $0x1,(%rcx)
   3583d:	test   %rbx,%rbx
   35840:	je     3598a <meshopt_peers::run+0xd9a>
   35846:	mov    0x50(%rsp),%rdi
   3584b:	jmp    35984 <meshopt_peers::run+0xd94>
   35850:	mov    0x20(%rcx),%rsi
   35854:	mov    0x28(%rcx),%rdx
   35858:	lea    0x738(%rsp),%rdi
   35860:	lea    0x6e0(%rsp),%rcx
   35868:	movd   -0x25c0c(%rip),%xmm0        # fc64 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x138>
   35870:	mov    %r12,%r8
   35873:	call   *0xf3cc7(%rip)        # 129540 <_DYNAMIC+0x600>
   35879:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35882:	je     36f2c <meshopt_peers::run+0x233c>
   35888:	mov    0x748(%rsp),%rax
   35890:	mov    %rax,0x18(%r14)
   35894:	movups 0x738(%rsp),%xmm0
   3589c:	movups %xmm0,0x8(%r14)
   358a1:	movl   $0x1,(%r14)
   358a8:	jmp    3598a <meshopt_peers::run+0xd9a>
   358ad:	mov    $0x4,%eax
   358b2:	jmp    35931 <meshopt_peers::run+0xd41>
   358b4:	mov    %r12,%rsi
   358b7:	and    $0xfffffffffffffffc,%rsi
   358bb:	mov    $0x30,%edi
   358c0:	xor    %edx,%edx
   358c2:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   358d0:	movups -0x30(%rbx,%rdi,1),%xmm0
   358d5:	movups %xmm0,-0x30(%rax,%rdi,1)
   358da:	movups -0x20(%rbx,%rdi,1),%xmm0
   358df:	movups %xmm0,-0x20(%rax,%rdi,1)
   358e4:	movups -0x10(%rbx,%rdi,1),%xmm0
   358e9:	movups %xmm0,-0x10(%rax,%rdi,1)
   358ee:	movups (%rbx,%rdi,1),%xmm0
   358f2:	movups %xmm0,(%rax,%rdi,1)
   358f6:	add    $0x4,%rdx
   358fa:	add    $0x40,%rdi
   358fe:	cmp    %rdx,%rsi
   35901:	jne    358d0 <meshopt_peers::run+0xce0>
   35903:	test   %rcx,%rcx
   35906:	je     35931 <meshopt_peers::run+0xd41>
   35908:	shl    $0x4,%rdx
   3590c:	mov    %rax,%rsi
   3590f:	add    %rdx,%rsi
   35912:	add    %rbx,%rdx
   35915:	shl    $0x4,%ecx
   35918:	xor    %edi,%edi
   3591a:	nopw   0x0(%rax,%rax,1)
   35920:	movups (%rdx,%rdi,1),%xmm0
   35924:	movups %xmm0,(%rsi,%rdi,1)
   35928:	add    $0x10,%rdi
   3592c:	cmp    %rdi,%rcx
   3592f:	jne    35920 <meshopt_peers::run+0xd30>
   35931:	mov    0x378(%rsp),%rcx
   35939:	mov    %rcx,0x30(%r14)
   3593d:	movups 0x368(%rsp),%xmm0
   35945:	movups %xmm0,0x20(%r14)
   3594a:	movaps 0x380(%rsp),%xmm0
   35952:	movups %xmm0,0x38(%r14)
   35957:	mov    0x390(%rsp),%rcx
   3595f:	mov    %rcx,0x48(%r14)
   35963:	mov    %r12,0x8(%r14)
   35967:	mov    %rax,0x10(%r14)
   3596b:	mov    %r12,0x18(%r14)
   3596f:	movl   $0x4,(%r14)
   35976:	cmpq   $0x0,0x350(%rsp)
   3597f:	je     3598a <meshopt_peers::run+0xd9a>
   35981:	mov    %rbx,%rdi
   35984:	call   *0xf37be(%rip)        # 129148 <free@GLIBC_2.2.5>
   3598a:	add    $0x1968,%rsp
   35991:	pop    %rbx
   35992:	pop    %r12
   35994:	pop    %r13
   35996:	pop    %r14
   35998:	pop    %r15
   3599a:	pop    %rbp
   3599b:	ret
   3599c:	movb   $0x2,0xe0(%rsp)
   359a4:	mov    0x50(%rsp),%rax
   359a9:	mov    %rax,0xe8(%rsp)
   359b1:	mov    %rbx,0xf0(%rsp)
   359b9:	movq   $0x0,0x118(%rsp)
   359c5:	movups 0x210(%rsp),%xmm0
   359cd:	movaps %xmm0,0x390(%rsp)
   359d5:	movups 0x1d0(%rsp),%xmm0
   359dd:	movups 0x1e0(%rsp),%xmm1
   359e5:	movups 0x1f0(%rsp),%xmm2
   359ed:	movups 0x200(%rsp),%xmm3
   359f5:	movaps %xmm3,0x380(%rsp)
   359fd:	movaps %xmm2,0x370(%rsp)
   35a05:	movaps %xmm1,0x360(%rsp)
   35a0d:	movaps %xmm0,0x350(%rsp)
   35a15:	movzbl %r12b,%r10d
   35a19:	lea    0x170(%rsp),%rdi
   35a21:	lea    0xe0(%rsp),%rcx
   35a29:	lea    0x350(%rsp),%r8
   35a31:	mov    %r13,%rsi
   35a34:	mov    %r15,%rdx
   35a37:	mov    $0xffffffff,%r9d
   35a3d:	lea    0x738(%rsp),%rax
   35a45:	push   %rax
   35a46:	mov    %r10,0x1c8(%rsp)
   35a4e:	push   %r10
   35a50:	call   *0xf3af2(%rip)        # 129548 <_DYNAMIC+0x608>
   35a56:	add    $0x10,%rsp
   35a5a:	mov    0x170(%rsp),%rdi
   35a62:	movzbl 0x178(%rsp),%ebp
   35a6a:	cmp    $0xffffffffffffffff,%rdi
   35a6e:	je     357bd <meshopt_peers::run+0xbcd>
   35a74:	movzbl %bpl,%eax
   35a78:	movzbl 0x17f(%rsp),%ecx
   35a80:	shl    $0x10,%ecx
   35a83:	movzwl 0x17d(%rsp),%edx
   35a8b:	or     %ecx,%edx
   35a8d:	mov    0x179(%rsp),%ecx
   35a94:	mov    0x180(%rsp),%r9
   35a9c:	mov    0x188(%rsp),%r8
   35aa4:	mov    0x190(%rsp),%rsi
   35aac:	mov    %rsi,0x28(%rsp)
   35ab1:	shl    $0x28,%rdx
   35ab5:	shl    $0x8,%rcx
   35ab9:	or     %rdx,%rcx
   35abc:	or     %rax,%rcx
   35abf:	mov    %rcx,0x38(%rsp)
   35ac4:	test   %r9,%r9
   35ac7:	je     35db7 <meshopt_peers::run+0x11c7>
   35acd:	mov    %r8,0x70(%rsp)
   35ad2:	mov    %rdi,0x10(%rsp)
   35ad7:	mov    0x198(%rsp),%r14
   35adf:	mov    %r9,0xd8(%rsp)
   35ae7:	mov    %r9,%rax
   35aea:	shl    $0x4,%rax
   35aee:	lea    (%rax,%rax,2),%rax
   35af2:	mov    %rax,0x18(%rsp)
   35af7:	xor    %r12d,%r12d
   35afa:	mov    0x738(%rsp),%r15
   35b02:	mov    0x740(%rsp),%rbp
   35b0a:	mov    0x748(%rsp),%rax
   35b12:	sub    %rbp,%rax
   35b15:	mov    %rax,0x358(%r15)
   35b1c:	mov    %r15,%rdi
   35b1f:	call   *0xf39d3(%rip)        # 1294f8 <_DYNAMIC+0x5b8>
   35b25:	mov    0x340(%r15),%rax
   35b2c:	mov    0x750(%rsp),%r13
   35b34:	sub    %r13,%rax
   35b37:	jb     35e2c <meshopt_peers::run+0x123c>
   35b3d:	mov    0x348(%r15),%rcx
   35b44:	sub    0x358(%r15),%rcx
   35b4b:	mov    %rax,0x690(%rsp)
   35b53:	mov    %rcx,0x698(%rsp)
   35b5b:	movb   $0x0,0x6b0(%rsp)
   35b63:	movb   $0x0,0x6b5(%rsp)
   35b6b:	movl   $0x0,0x6b1(%rsp)
   35b76:	movb   $0xff,0x6b8(%rsp)
   35b7e:	movw   $0x0,0x6b6(%rsp)
   35b88:	lea    0x6a0(%rsp),%rax
   35b90:	xorps  %xmm0,%xmm0
   35b93:	movups %xmm0,(%rax)
   35b96:	movq   $0xffffffffffffffff,0x3c8(%rsp)
   35ba2:	movq   $0xffffffffffffffff,0x5d0(%rsp)
   35bae:	movq   $0x0,0x350(%rsp)
   35bba:	movq   $0x4,0x358(%rsp)
   35bc6:	movups %xmm0,-0x340(%rax)
   35bcd:	movq   $0x4,0x370(%rsp)
   35bd9:	movups %xmm0,-0x328(%rax)
   35be0:	movq   $0x4,0x388(%rsp)
   35bec:	movups %xmm0,-0x310(%rax)
   35bf3:	movq   $0x1,0x3a0(%rsp)
   35bff:	movups %xmm0,-0x2f8(%rax)
   35c06:	movq   $0x2,0x3b8(%rsp)
   35c12:	movq   $0x0,0x3c0(%rsp)
   35c1e:	mov    0x38(%rsp),%rax
   35c23:	mov    0x8(%rax,%r12,1),%rdi
   35c28:	mov    0x10(%rax,%r12,1),%rdx
   35c2d:	mov    %rdx,%rsi
   35c30:	add    %rdi,%rsi
   35c33:	jb     36e40 <meshopt_peers::run+0x2250>
   35c39:	cmp    %r14,%rsi
   35c3c:	ja     36e40 <meshopt_peers::run+0x2250>
   35c42:	mov    0x28(%rsp),%rax
   35c47:	lea    (%rax,%rdi,4),%rsi
   35c4b:	movb   $0x2,0xe0(%rsp)
   35c53:	mov    0x50(%rsp),%rax
   35c58:	mov    %rax,0xe8(%rsp)
   35c60:	mov    %rbx,0xf0(%rsp)
   35c68:	movq   $0x0,0x118(%rsp)
   35c74:	lea    0x170(%rsp),%rdi
   35c7c:	lea    0xe0(%rsp),%rcx
   35c84:	mov    0x1c0(%rsp),%r8
   35c8c:	lea    0x350(%rsp),%r9
   35c94:	call   *0xf38b6(%rip)        # 129550 <_DYNAMIC+0x610>
   35c9a:	mov    0x6a8(%rsp),%rsi
   35ca2:	sub    %rsi,%rbp
   35ca5:	jb     35d9a <meshopt_peers::run+0x11aa>
   35cab:	mov    %rbp,0x740(%rsp)
   35cb3:	add    0x6a0(%rsp),%r13
   35cbb:	jb     36ec3 <meshopt_peers::run+0x22d3>
   35cc1:	mov    %r15,%rdi
   35cc4:	mov    %r13,%rsi
   35cc7:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35ccc:	mov    %eax,%ebp
   35cce:	cmp    $0xff,%al
   35cd0:	jne    36ec6 <meshopt_peers::run+0x22d6>
   35cd6:	movzbl 0x170(%rsp),%ebx
   35cde:	movzbl 0x171(%rsp),%ebp
   35ce6:	lea    0x172(%rsp),%rcx
   35cee:	mov    (%rcx),%rax
   35cf1:	mov    0x6(%rcx),%rcx
   35cf5:	mov    %rax,0x230(%rsp)
   35cfd:	mov    %rcx,0x236(%rsp)
   35d05:	movss  0x180(%rsp),%xmm0
   35d0e:	movaps %xmm0,0x80(%rsp)
   35d16:	lea    0x350(%rsp),%rdi
   35d1e:	call   33dd0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35d23:	cmp    $0x1,%bl
   35d26:	mov    0x20(%rsp),%rbx
   35d2b:	je     36ed3 <meshopt_peers::run+0x22e3>
   35d31:	lea    0x232(%rsp),%rcx
   35d39:	mov    0x8(%rcx),%eax
   35d3c:	lea    0x72c(%rsp),%rdx
   35d44:	mov    %eax,0x8(%rdx)
   35d47:	mov    (%rcx),%rax
   35d4a:	mov    %rax,(%rdx)
   35d4d:	mov    0x8(%rdx),%eax
   35d50:	mov    %eax,0x358(%rsp)
   35d57:	mov    (%rdx),%rcx
   35d5a:	mov    %rcx,0x350(%rsp)
   35d62:	mov    0x38(%rsp),%rdx
   35d67:	mov    %eax,0x24(%rdx,%r12,1)
   35d6c:	mov    %rcx,0x1c(%rdx,%r12,1)
   35d71:	xorps  %xmm0,%xmm0
   35d74:	movaps 0x80(%rsp),%xmm1
   35d7c:	movss  %xmm1,%xmm0
   35d80:	movlps %xmm0,0x28(%rdx,%r12,1)
   35d86:	add    $0x30,%r12
   35d8a:	cmp    %r12,0x18(%rsp)
   35d8f:	jne    35afa <meshopt_peers::run+0xf0a>
   35d95:	jmp    35e5e <meshopt_peers::run+0x126e>
   35d9a:	lea    0x740(%rsp),%rdi
   35da2:	call   *0xf3760(%rip)        # 129508 <_DYNAMIC+0x5c8>
   35da8:	mov    %eax,%ebp
   35daa:	cmp    $0xff,%al
   35dac:	je     35cb3 <meshopt_peers::run+0x10c3>
   35db2:	jmp    36ec6 <meshopt_peers::run+0x22d6>
   35db7:	mov    %rdi,%r12
   35dba:	mov    %r8,0x80(%rsp)
   35dc2:	lea    0x738(%rsp),%rdi
   35dca:	mov    0x150(%rsp),%rsi
   35dd2:	mov    0xc8(%rsp),%rdx
   35dda:	call   33710 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35ddf:	mov    $0x1,%r13b
   35de2:	cmp    $0xff,%al
   35de4:	je     35e34 <meshopt_peers::run+0x1244>
   35de6:	mov    %eax,%ebp
   35de8:	xor    %r14d,%r14d
   35deb:	mov    $0x1,%r15b
   35dee:	jmp    35f64 <meshopt_peers::run+0x1374>
   35df3:	lea    0x350(%rsp),%rdi
   35dfb:	mov    $0x1,%edx
   35e00:	mov    $0x1,%r8d
   35e06:	mov    $0x1,%r9d
   35e0c:	xor    %esi,%esi
   35e0e:	mov    %rbx,%rcx
   35e11:	call   3b500 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35e16:	cmpl   $0x1,0x350(%rsp)
   35e1e:	jne    35f05 <meshopt_peers::run+0x1315>
   35e24:	mov    $0xa,%bpl
   35e27:	jmp    357d5 <meshopt_peers::run+0xbe5>
   35e2c:	mov    $0x9,%bpl
   35e2f:	jmp    36ed3 <meshopt_peers::run+0x22e3>
   35e34:	mov    0x750(%rsp),%rax
   35e3c:	sub    %rbx,%rax
   35e3f:	jae    35f25 <meshopt_peers::run+0x1335>
   35e45:	mov    0x90(%rsp),%rdi
   35e4d:	call   *0xf32f5(%rip)        # 129148 <free@GLIBC_2.2.5>
   35e53:	mov    $0x8,%bpl
   35e56:	xor    %r14d,%r14d
   35e59:	jmp    35f61 <meshopt_peers::run+0x1371>
   35e5e:	mov    0xd8(%rsp),%rdx
   35e66:	xor    %r13d,%r13d
   35e69:	cmp    $0x1,%rdx
   35e6d:	jne    35f71 <meshopt_peers::run+0x1381>
   35e73:	mov    0x10(%rsp),%r12
   35e78:	mov    0x70(%rsp),%rax
   35e7d:	mov    %rax,0x80(%rsp)
   35e85:	mov    0x38(%rsp),%rdi
   35e8a:	movups 0x1c(%rdi),%xmm0
   35e8e:	movaps %xmm0,0x2a0(%rsp)
   35e96:	movl   $0x7f7fffff,0x2b0(%rsp)
   35ea1:	lea    0x9c(%rsp),%rax
   35ea9:	lea    0x158(%rsp),%r10
   35eb1:	lea    0x2a0(%rsp),%r11
   35eb9:	mov    $0x1,%esi
   35ebe:	mov    0x28(%rsp),%rdx
   35ec3:	mov    %r14,%rcx
   35ec6:	mov    0x50(%rsp),%r8
   35ecb:	mov    %rbx,%r9
   35ece:	lea    0x738(%rsp),%r15
   35ed6:	push   %r15
   35ed8:	push   %rax
   35ed9:	push   %r10
   35edb:	push   %r13
   35edd:	push   %r11
   35edf:	push   $0x0
   35ee1:	call   33e60 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   35ee6:	add    $0x30,%rsp
   35eea:	mov    %rax,%rbp
   35eed:	test   $0x1,%bpl
   35ef1:	je     35dc2 <meshopt_peers::run+0x11d2>
   35ef7:	shr    $0x8,%ebp
   35efa:	mov    $0x1,%r14b
   35efd:	mov    $0x1,%r15b
   35f00:	mov    $0x1,%r13b
   35f03:	jmp    35f64 <meshopt_peers::run+0x1374>
   35f05:	mov    0x358(%rsp),%rdi
   35f0d:	mov    %rdi,0x90(%rsp)
   35f15:	xor    %esi,%esi
   35f17:	mov    %rbx,%rdx
   35f1a:	call   *0xf3248(%rip)        # 129168 <memset@GLIBC_2.2.5>
   35f20:	jmp    3579b <meshopt_peers::run+0xbab>
   35f25:	mov    %rax,0x750(%rsp)
   35f2d:	test   %rbx,%rbx
   35f30:	je     35f40 <meshopt_peers::run+0x1350>
   35f32:	mov    0x90(%rsp),%rdi
   35f3a:	call   *0xf3208(%rip)        # 129148 <free@GLIBC_2.2.5>
   35f40:	xor    %r14d,%r14d
   35f43:	lea    0x738(%rsp),%rdi
   35f4b:	mov    $0x4,%edx
   35f50:	xor    %esi,%esi
   35f52:	call   33670 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   35f57:	cmp    $0xff,%al
   35f59:	je     36cd9 <meshopt_peers::run+0x20e9>
   35f5f:	mov    %eax,%ebp
   35f61:	xor    %r15d,%r15d
   35f64:	mov    0x80(%rsp),%rax
   35f6c:	jmp    36ee6 <meshopt_peers::run+0x22f6>
   35f71:	lea    0x170(%rsp),%rdi
   35f79:	mov    0xf35d8(%rip),%r10        # 129558 <_DYNAMIC+0x618>
   35f80:	mov    %r14,%r8
   35f83:	movb   $0x2,0xe0(%rsp)
   35f8b:	mov    0x50(%rsp),%rax
   35f90:	mov    %rax,0xe8(%rsp)
   35f98:	mov    %rbx,0xf0(%rsp)
   35fa0:	movq   $0x0,0x118(%rsp)
   35fac:	movups 0x210(%rsp),%xmm0
   35fb4:	movaps %xmm0,0x390(%rsp)
   35fbc:	movups 0x1d0(%rsp),%xmm0
   35fc4:	movups 0x1e0(%rsp),%xmm1
   35fcc:	movups 0x1f0(%rsp),%xmm2
   35fd4:	movups 0x200(%rsp),%xmm3
   35fdc:	movaps %xmm3,0x380(%rsp)
   35fe4:	movaps %xmm2,0x370(%rsp)
   35fec:	movaps %xmm1,0x360(%rsp)
   35ff4:	movaps %xmm0,0x350(%rsp)
   35ffc:	mov    0x38(%rsp),%rsi
   36001:	mov    %rdx,0xd8(%rsp)
   36009:	mov    0x28(%rsp),%rcx
   3600e:	mov    %r8,0x18(%rsp)
   36013:	mov    0xc8(%rsp),%r9
   3601b:	lea    0x738(%rsp),%r12
   36023:	push   %r12
   36025:	lea    0x358(%rsp),%rax
   3602d:	push   %rax
   3602e:	lea    0xf0(%rsp),%rax
   36036:	push   %rax
   36037:	push   0x1e0(%rsp)
   3603e:	call   *%r10
   36041:	add    $0x20,%rsp
   36045:	mov    %r13,0x148(%rsp)
   3604d:	mov    0x170(%rsp),%r15
   36055:	movzbl 0x178(%rsp),%ebp
   3605d:	cmp    $0xffffffffffffffff,%r15
   36061:	je     36ed3 <meshopt_peers::run+0x22e3>
   36067:	movzbl %bpl,%eax
   3606b:	movzbl 0x17f(%rsp),%ecx
   36073:	shl    $0x10,%ecx
   36076:	movzwl 0x17d(%rsp),%edx
   3607e:	or     %ecx,%edx
   36080:	mov    0x179(%rsp),%r8d
   36088:	mov    0x180(%rsp),%r14
   36090:	shl    $0x28,%rdx
   36094:	shl    $0x8,%r8
   36098:	or     %rdx,%r8
   3609b:	or     %rax,%r8
   3609e:	sub    $0x40,%rsp
   360a2:	mov    %r12,0x30(%rsp)
   360a7:	mov    0x208(%rsp),%rax
   360af:	mov    %rax,0x18(%rsp)
   360b4:	mov    0x108(%rsp),%rax
   360bc:	mov    %rax,0x10(%rsp)
   360c1:	mov    0x58(%rsp),%rax
   360c6:	mov    %rax,0x8(%rsp)
   360cb:	mov    0x68(%rsp),%rax
   360d0:	mov    %rax,(%rsp)
   360d4:	movq   $0x0,0x20(%rsp)
   360dd:	mov    0xd0(%rsp),%rdi
   360e5:	mov    %rbx,%rsi
   360e8:	mov    0x78(%rsp),%rdx
   360ed:	mov    0x118(%rsp),%rcx
   360f5:	mov    %r8,0x110(%rsp)
   360fd:	mov    %r14,%r9
   36100:	call   *0xf345a(%rip)        # 129560 <_DYNAMIC+0x620>
   36106:	add    $0x40,%rsp
   3610a:	mov    %eax,%ebp
   3610c:	cmp    $0xff,%al
   3610e:	jne    36f19 <meshopt_peers::run+0x2329>
   36114:	mov    0x18(%rsp),%rax
   36119:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   36123:	mul    %rcx
   36126:	mov    %rdx,%rax
   36129:	shr    $1,%rax
   3612c:	mov    %rax,0x48(%rsp)
   36131:	mov    $0x30,%ecx
   36136:	mul    %rcx
   36139:	seto   %cl
   3613c:	test   %rax,%rax
   3613f:	sets   %dl
   36142:	or     %cl,%dl
   36144:	mov    $0x8,%bpl
   36147:	mov    0x10(%rsp),%r12
   3614c:	jne    36cce <meshopt_peers::run+0x20de>
   36152:	mov    $0x1,%cl
   36154:	mov    %ecx,0x8(%rsp)
   36158:	add    0x750(%rsp),%rax
   36160:	jb     36da4 <meshopt_peers::run+0x21b4>
   36166:	mov    %rax,0x750(%rsp)
   3616e:	mov    0x738(%rsp),%r13
   36176:	mov    %r13,%rdi
   36179:	mov    %rax,%r12
   3617c:	mov    %rax,%rsi
   3617f:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   36184:	mov    %eax,%ebp
   36186:	cmp    $0xff,%al
   36188:	jne    37033 <meshopt_peers::run+0x2443>
   3618e:	mov    %r15,0x40(%rsp)
   36193:	mov    0x18(%rsp),%r8
   36198:	cmp    $0x3,%r8
   3619c:	jae    36c0e <meshopt_peers::run+0x201e>
   361a2:	mov    $0x8,%r15d
   361a8:	xor    %eax,%eax
   361aa:	mov    %rax,0x68(%rsp)
   361af:	sub    0x48(%rsp),%rax
   361b4:	mov    $0x30,%ecx
   361b9:	mul    %rcx
   361bc:	seto   %cl
   361bf:	test   %rax,%rax
   361c2:	sets   %dl
   361c5:	or     %cl,%dl
   361c7:	mov    $0x8,%bpl
   361ca:	jne    36d89 <meshopt_peers::run+0x2199>
   361d0:	test   %rax,%rax
   361d3:	je     36200 <meshopt_peers::run+0x1610>
   361d5:	add    %r12,%rax
   361d8:	jb     36d89 <meshopt_peers::run+0x2199>
   361de:	mov    %rax,0x750(%rsp)
   361e6:	mov    %r13,%rdi
   361e9:	mov    %rax,%rsi
   361ec:	call   3bf70 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   361f1:	mov    %eax,%ebp
   361f3:	cmp    $0xff,%al
   361f5:	mov    0x18(%rsp),%r8
   361fa:	jne    36d89 <meshopt_peers::run+0x2199>
   36200:	mov    %r15,%rdx
   36203:	shr    $0x8,%rdx
   36207:	mov    %r15,%rax
   3620a:	shr    $0x28,%rax
   3620e:	mov    %r15,%rcx
   36211:	shr    $0x38,%rcx
   36215:	mov    %cl,0xef(%rsp)
   3621c:	mov    %ax,0xed(%rsp)
   36224:	mov    %edx,0xe9(%rsp)
   3622b:	movq   $0x0,0xf0(%rsp)
   36237:	mov    0x68(%rsp),%rax
   3623c:	mov    %rax,0xe0(%rsp)
   36244:	mov    %r15b,0xe8(%rsp)
   3624c:	cmp    $0x2,%r8
   36250:	jbe    362d9 <meshopt_peers::run+0x16e9>
   36256:	cmp    0x48(%rsp),%rax
   3625b:	jb     36c5d <meshopt_peers::run+0x206d>
   36261:	xor    %r12d,%r12d
   36264:	mov    0xe8(%rsp),%r15
   3626c:	lea    (%r12,%r12,2),%rax
   36270:	shl    $0x4,%rax
   36274:	xorps  %xmm0,%xmm0
   36277:	movups %xmm0,0x20(%r15,%rax,1)
   3627d:	movups %xmm0,0x10(%r15,%rax,1)
   36283:	movups %xmm0,(%r15,%rax,1)
   36288:	cmpq   $0x6,0x18(%rsp)
   3628e:	jb     362b1 <meshopt_peers::run+0x16c1>
   36290:	lea    (%r15,%rax,1),%rdi
   36294:	add    $0x30,%rdi
   36298:	mov    0x48(%rsp),%rax
   3629d:	lea    (%rax,%rax,2),%rdx
   362a1:	shl    $0x4,%rdx
   362a5:	add    $0xffffffffffffffd0,%rdx
   362a9:	xor    %esi,%esi
   362ab:	call   *0xf2eb7(%rip)        # 129168 <memset@GLIBC_2.2.5>
   362b1:	mov    0xe0(%rsp),%rax
   362b9:	mov    %rax,0x68(%rsp)
   362be:	cmp    $0xffffffffffffffff,%rax
   362c2:	je     37071 <meshopt_peers::run+0x2481>
   362c8:	mov    %r15,%rdx
   362cb:	shr    $0x8,%rdx
   362cf:	add    %r12,0x48(%rsp)
   362d4:	mov    0x18(%rsp),%r8
   362d9:	movzbl %r15b,%eax
   362dd:	shl    $0x8,%rdx
   362e1:	or     %rax,%rdx
   362e4:	mov    %rdx,0xb0(%rsp)
   362ec:	lea    0x350(%rsp),%rdi
   362f4:	lea    0x738(%rsp),%rsi
   362fc:	mov    %r8,%rdx
   362ff:	call   33760 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   36304:	mov    0x350(%rsp),%rdx
   3630c:	movzbl 0x358(%rsp),%ebp
   36314:	cmp    $0xffffffffffffffff,%rdx
   36318:	je     3703d <meshopt_peers::run+0x244d>
   3631e:	lea    0x359(%rsp),%rcx
   36326:	mov    (%rcx),%rax
   36329:	mov    0x7(%rcx),%rcx
   3632d:	lea    0xe9(%rsp),%rsi
   36335:	mov    %rcx,0x7(%rsi)
   36339:	mov    %rax,(%rsi)
   3633c:	mov    %rdx,0xe0(%rsp)
   36344:	mov    %bpl,0xe8(%rsp)
   3634c:	mov    0xf0(%rsp),%rcx
   36354:	mov    0x18(%rsp),%rsi
   36359:	mov    %rsi,%r15
   3635c:	sub    %rcx,%r15
   3635f:	mov    %rdx,0x80(%rsp)
   36367:	jbe    363bc <meshopt_peers::run+0x17cc>
   36369:	mov    %rdx,%rax
   3636c:	mov    %rcx,%rsi
   3636f:	sub    %rcx,%rax
   36372:	cmp    %rax,%r15
   36375:	ja     36c89 <meshopt_peers::run+0x2099>
   3637b:	mov    0xe8(%rsp),%rbp
   36383:	mov    %rsi,%rbx
   36386:	lea    0x0(,%rsi,4),%rdi
   3638e:	add    %rbp,%rdi
   36391:	lea    0x0(,%r15,4),%rdx
   36399:	xor    %esi,%esi
   3639b:	call   *0xf2dc7(%rip)        # 129168 <memset@GLIBC_2.2.5>
   363a1:	cmpq   $0xffffffffffffffff,0x80(%rsp)
   363aa:	je     3703d <meshopt_peers::run+0x244d>
   363b0:	add    %r15,%rbx
   363b3:	mov    %rbp,%rdx
   363b6:	shr    $0x8,%rdx
   363ba:	jmp    363e2 <meshopt_peers::run+0x17f2>
   363bc:	movzbl 0xef(%rsp),%eax
   363c4:	shl    $0x10,%eax
   363c7:	movzwl 0xed(%rsp),%ecx
   363cf:	or     %eax,%ecx
   363d1:	shl    $0x20,%rcx
   363d5:	mov    0xe9(%rsp),%edx
   363dc:	or     %rcx,%rdx
   363df:	mov    %rsi,%rbx
   363e2:	movzbl %bpl,%eax
   363e6:	shl    $0x8,%rdx
   363ea:	or     %rax,%rdx
   363ed:	cmp    $0x2,%r14
   363f1:	mov    %rdx,0xc0(%rsp)
   363f9:	mov    %rbx,0x140(%rsp)
   36401:	jb     36b18 <meshopt_peers::run+0x1f28>
   36407:	mov    0xb0(%rsp),%rax
   3640f:	add    $0x10,%rax
   36413:	mov    %rax,0x280(%rsp)
   3641b:	xor    %ebp,%ebp
   3641d:	xor    %r12d,%r12d
   36420:	mov    0xd0(%rsp),%rax
   36428:	mov    %rbp,0xb8(%rsp)
   36430:	add    $0x4,%rax
   36434:	mov    -0x4(%rax),%edi
   36437:	mov    %rax,0x1b8(%rsp)
   3643f:	mov    (%rax),%r15d
   36442:	cmp    %edi,%r15d
   36445:	mov    0xd8(%rsp),%rdx
   3644d:	jb     371df <meshopt_peers::run+0x25ef>
   36453:	cmp    %r15,%rdx
   36456:	jb     371df <meshopt_peers::run+0x25ef>
   3645c:	sub    %rdi,%r15
   3645f:	lea    (%rdi,%rdi,2),%r13
   36463:	shl    $0x4,%r13
   36467:	add    0x38(%rsp),%r13
   3646c:	movq   $0xffffffffffffffff,0xe0(%rsp)
   36478:	lea    0x350(%rsp),%rdi
   36480:	mov    %r13,%rsi
   36483:	mov    %r15,%rdx
   36486:	mov    0x28(%rsp),%rcx
   3648b:	mov    0x18(%rsp),%r8
   36490:	lea    0x2c0(%rsp),%r9
   36498:	lea    0x738(%rsp),%rax
   364a0:	push   %rax
   364a1:	lea    0x1d8(%rsp),%rax
   364a9:	push   %rax
   364aa:	push   0x30(%rsp)
   364ae:	push   0xa8(%rsp)
   364b5:	call   *0xf30ad(%rip)        # 129568 <_DYNAMIC+0x628>
   364bb:	add    $0x20,%rsp
   364bf:	mov    0x350(%rsp),%rax
   364c7:	movzbl 0x358(%rsp),%ebp
   364cf:	mov    %rax,0x8(%rsp)
   364d4:	cmp    $0xffffffffffffffff,%rax
   364d8:	je     36fce <meshopt_peers::run+0x23de>
   364de:	movzbl %bpl,%eax
   364e2:	movzbl 0x35f(%rsp),%ecx
   364ea:	shl    $0x10,%ecx
   364ed:	movzwl 0x35d(%rsp),%edx
   364f5:	or     %ecx,%edx
   364f7:	mov    0x359(%rsp),%ebx
   364fe:	mov    0x368(%rsp),%rcx
   36506:	mov    %rcx,0x60(%rsp)
   3650b:	mov    0x370(%rsp),%rcx
   36513:	mov    %rcx,0xa8(%rsp)
   3651b:	mov    0x380(%rsp),%rcx
   36523:	mov    %rcx,0x30(%rsp)
   36528:	mov    0x388(%rsp),%rcx
   36530:	mov    %rcx,0x58(%rsp)
   36535:	mov    0x390(%rsp),%rcx
   3653d:	mov    %rcx,0x78(%rsp)
   36542:	lea    0x359(%rsp),%rcx
   3654a:	movups 0x3f(%rcx),%xmm0
   3654e:	movaps %xmm0,0x260(%rsp)
   36556:	mov    0x4f(%rcx),%ecx
   36559:	mov    %ecx,0x270(%rsp)
   36560:	shl    $0x28,%rdx
   36564:	shl    $0x8,%rbx
   36568:	or     %rdx,%rbx
   3656b:	or     %rax,%rbx
   3656e:	testb  $0x1,0x3ac(%rsp)
   36576:	je     366e7 <meshopt_peers::run+0x1af7>
   3657c:	mov    %r13,%rdi
   3657f:	mov    %r15,%rsi
   36582:	mov    0x28(%rsp),%rdx
   36587:	mov    0x18(%rsp),%rcx
   3658c:	mov    0x50(%rsp),%r8
   36591:	mov    0x20(%rsp),%r9
   36596:	lea    0x738(%rsp),%rax
   3659e:	push   %rax
   3659f:	lea    0xa4(%rsp),%rax
   365a7:	push   %rax
   365a8:	lea    0x168(%rsp),%rax
   365b0:	push   %rax
   365b1:	push   0x160(%rsp)
   365b8:	lea    0x280(%rsp),%rax
   365c0:	push   %rax
   365c1:	push   $0x0
   365c3:	call   33e60 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   365c8:	add    $0x30,%rsp
   365cc:	mov    %rax,%rbp
   365cf:	test   $0x1,%bpl
   365d3:	jne    37093 <meshopt_peers::run+0x24a3>
   365d9:	mov    0x750(%rsp),%rcx
   365e1:	mov    0x8(%rsp),%rdx
   365e6:	lea    0x0(,%rdx,4),%rax
   365ee:	shr    $0x3e,%rdx
   365f2:	sete   %dl
   365f5:	test   %rax,%rax
   365f8:	setns  %sil
   365fc:	and    %dl,%sil
   365ff:	sub    %rax,%rcx
   36602:	setae  %al
   36605:	test   %al,%sil
   36608:	je     36e14 <meshopt_peers::run+0x2224>
   3660e:	mov    %rcx,0x750(%rsp)
   36616:	cmpq   $0x0,0x8(%rsp)
   3661c:	je     3662f <meshopt_peers::run+0x1a3f>
   3661e:	mov    %rbx,%rdi
   36621:	call   *0xf2b21(%rip)        # 129148 <free@GLIBC_2.2.5>
   36627:	mov    0x750(%rsp),%rcx
   3662f:	mov    0x60(%rsp),%rax
   36634:	mov    $0x14,%edx
   36639:	mul    %rdx
   3663c:	jo     36fa2 <meshopt_peers::run+0x23b2>
   36642:	test   %rax,%rax
   36645:	js     36fa2 <meshopt_peers::run+0x23b2>
   3664b:	sub    %rax,%rcx
   3664e:	jb     36fa2 <meshopt_peers::run+0x23b2>
   36654:	mov    %rcx,0x750(%rsp)
   3665c:	cmpq   $0x0,0x60(%rsp)
   36662:	je     3667a <meshopt_peers::run+0x1a8a>
   36664:	mov    0xa8(%rsp),%rdi
   3666c:	call   *0xf2ad6(%rip)        # 129148 <free@GLIBC_2.2.5>
   36672:	mov    0x750(%rsp),%rcx
   3667a:	movabs $0x3fffffffffffffff,%rax
   36684:	cmp    %rax,0x30(%rsp)
   36689:	ja     37058 <meshopt_peers::run+0x2468>
   3668f:	mov    0x30(%rsp),%rax
   36694:	lea    0x0(,%rax,4),%rax
   3669c:	test   %rax,%rax
   3669f:	js     37058 <meshopt_peers::run+0x2468>
   366a5:	sub    %rax,%rcx
   366a8:	jb     37058 <meshopt_peers::run+0x2468>
   366ae:	mov    %rcx,0x750(%rsp)
   366b6:	cmpq   $0x0,0x30(%rsp)
   366bc:	je     366c9 <meshopt_peers::run+0x1ad9>
   366be:	mov    0x58(%rsp),%rdi
   366c3:	call   *0xf2a7f(%rip)        # 129148 <free@GLIBC_2.2.5>
   366c9:	dec    %r14
   366cc:	mov    0x1b8(%rsp),%rax
   366d4:	add    $0x4,%rax
   366d8:	cmp    $0x1,%r14
   366dc:	ja     36434 <meshopt_peers::run+0x1844>
   366e2:	jmp    36cc1 <meshopt_peers::run+0x20d1>
   366e7:	mov    %r13,%rdi
   366ea:	mov    %r15,%rsi
   366ed:	mov    0x28(%rsp),%rdx
   366f2:	mov    0x18(%rsp),%rcx
   366f7:	mov    0x50(%rsp),%r8
   366fc:	mov    0x20(%rsp),%r9
   36701:	lea    0x738(%rsp),%rax
   36709:	push   %rax
   3670a:	lea    0xa4(%rsp),%rax
   36712:	push   %rax
   36713:	lea    0x168(%rsp),%rax
   3671b:	push   %rax
   3671c:	push   0x160(%rsp)
   36723:	lea    0x280(%rsp),%rax
   3672b:	push   %rax
   3672c:	push   $0x0
   3672e:	call   33e60 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36733:	add    $0x30,%rsp
   36737:	mov    %rax,%rbp
   3673a:	test   $0x1,%bpl
   3673e:	jne    37093 <meshopt_peers::run+0x24a3>
   36744:	shr    $0x20,%rbp
   36748:	movb   $0x2,0x170(%rsp)
   36750:	mov    0x50(%rsp),%rax
   36755:	mov    %rax,0x178(%rsp)
   3675d:	mov    0x20(%rsp),%rax
   36762:	mov    %rax,0x180(%rsp)
   3676a:	movq   $0x0,0x1a8(%rsp)
   36776:	movups 0x210(%rsp),%xmm0
   3677e:	movaps %xmm0,0x390(%rsp)
   36786:	movups 0x1d0(%rsp),%xmm0
   3678e:	movups 0x1e0(%rsp),%xmm1
   36796:	movups 0x1f0(%rsp),%xmm2
   3679e:	movups 0x200(%rsp),%xmm3
   367a6:	movaps %xmm3,0x380(%rsp)
   367ae:	movaps %xmm2,0x370(%rsp)
   367b6:	movaps %xmm1,0x360(%rsp)
   367be:	movaps %xmm0,0x350(%rsp)
   367c6:	lea    0x230(%rsp),%rdi
   367ce:	mov    0x58(%rsp),%rsi
   367d3:	mov    0x78(%rsp),%rdx
   367d8:	lea    0x170(%rsp),%rcx
   367e0:	lea    0x350(%rsp),%r8
   367e8:	mov    %ebp,%r9d
   367eb:	lea    0x738(%rsp),%rax
   367f3:	push   %rax
   367f4:	push   0x1c8(%rsp)
   367fb:	call   *0xf2d47(%rip)        # 129548 <_DYNAMIC+0x608>
   36801:	add    $0x10,%rsp
   36805:	mov    0x230(%rsp),%r15
   3680d:	movzbl 0x238(%rsp),%ebp
   36815:	cmp    $0xffffffffffffffff,%r15
   36819:	je     37096 <meshopt_peers::run+0x24a6>
   3681f:	movzbl %bpl,%ecx
   36823:	movzbl 0x23f(%rsp),%eax
   3682b:	shl    $0x10,%eax
   3682e:	movzwl 0x23d(%rsp),%esi
   36836:	or     %eax,%esi
   36838:	mov    0x239(%rsp),%r13d
   36840:	mov    0x240(%rsp),%rax
   36848:	mov    0x248(%rsp),%rdx
   36850:	mov    %rdx,0xa0(%rsp)
   36858:	mov    0x250(%rsp),%rdx
   36860:	mov    %rdx,0x78(%rsp)
   36865:	mov    0x258(%rsp),%rdx
   3686d:	shl    $0x28,%rsi
   36871:	shl    $0x8,%r13
   36875:	or     %rsi,%r13
   36878:	or     %rcx,%r13
   3687b:	test   %rax,%rax
   3687e:	je     36927 <meshopt_peers::run+0x1d37>
   36884:	lea    (%rax,%rax,2),%rax
   36888:	shl    $0x4,%rax
   3688c:	add    %r13,%rax
   3688f:	lea    (%r12,%r12,2),%rcx
   36893:	shl    $0x4,%rcx
   36897:	add    0x280(%rsp),%rcx
   3689f:	xor    %esi,%esi
   368a1:	mov    0x0(%r13,%rsi,1),%rdi
   368a6:	mov    0x8(%r13,%rsi,1),%r8
   368ab:	movups 0x10(%r13,%rsi,1),%xmm0
   368b1:	movaps %xmm0,0x6c0(%rsp)
   368b9:	mov    0x270(%rsp),%r9d
   368c1:	lea    0x6cc(%rsp),%r10
   368c9:	mov    %r9d,0x10(%r10)
   368cd:	movaps 0x260(%rsp),%xmm0
   368d5:	movups %xmm0,(%r10)
   368d9:	cmp    0x48(%rsp),%r12
   368de:	jae    372d7 <meshopt_peers::run+0x26e7>
   368e4:	lea    (%rsi,%r13,1),%r9
   368e8:	add    0xb8(%rsp),%r8
   368f0:	mov    %rdi,-0x10(%rcx,%rsi,1)
   368f5:	mov    %r8,-0x8(%rcx,%rsi,1)
   368fa:	movaps 0x6c0(%rsp),%xmm0
   36902:	movaps 0x6d0(%rsp),%xmm1
   3690a:	movups %xmm0,(%rcx,%rsi,1)
   3690e:	movups %xmm1,0x10(%rcx,%rsi,1)
   36913:	inc    %r12
   36916:	add    $0x30,%rsi
   3691a:	add    $0x30,%r9
   3691e:	cmp    %rax,%r9
   36921:	jne    368a1 <meshopt_peers::run+0x1cb1>
   36927:	mov    %rdx,%rbp
   3692a:	add    0xb8(%rsp),%rbp
   36932:	jb     3724c <meshopt_peers::run+0x265c>
   36938:	cmp    0x140(%rsp),%rbp
   36940:	ja     3724c <meshopt_peers::run+0x265c>
   36946:	mov    0xc0(%rsp),%rax
   3694e:	mov    0xb8(%rsp),%rcx
   36956:	lea    (%rax,%rcx,4),%rdi
   3695a:	shl    $0x2,%rdx
   3695e:	mov    0x78(%rsp),%rsi
   36963:	call   *0xf27ef(%rip)        # 129158 <memcpy@GLIBC_2.14>
   36969:	mov    0x750(%rsp),%rcx
   36971:	mov    %r15,%rax
   36974:	mov    $0x30,%edx
   36979:	mul    %rdx
   3697c:	seto   %dl
   3697f:	not    %dl
   36981:	test   %rax,%rax
   36984:	setns  %sil
   36988:	and    %dl,%sil
   3698b:	sub    %rax,%rcx
   3698e:	setae  %al
   36991:	test   %al,%sil
   36994:	je     370f3 <meshopt_peers::run+0x2503>
   3699a:	mov    %rcx,0x750(%rsp)
   369a2:	test   %r15,%r15
   369a5:	je     369b8 <meshopt_peers::run+0x1dc8>
   369a7:	mov    %r13,%rdi
   369aa:	call   *0xf2798(%rip)        # 129148 <free@GLIBC_2.2.5>
   369b0:	mov    0x750(%rsp),%rcx
   369b8:	mov    0xa0(%rsp),%rax
   369c0:	shr    $0x3e,%rax
   369c4:	jne    37101 <meshopt_peers::run+0x2511>
   369ca:	mov    0xa0(%rsp),%rax
   369d2:	lea    0x0(,%rax,4),%rax
   369da:	test   %rax,%rax
   369dd:	js     37101 <meshopt_peers::run+0x2511>
   369e3:	sub    %rax,%rcx
   369e6:	jb     37101 <meshopt_peers::run+0x2511>
   369ec:	mov    %rcx,0x750(%rsp)
   369f4:	cmpq   $0x0,0xa0(%rsp)
   369fd:	je     36a12 <meshopt_peers::run+0x1e22>
   369ff:	mov    0x78(%rsp),%rdi
   36a04:	call   *0xf273e(%rip)        # 129148 <free@GLIBC_2.2.5>
   36a0a:	mov    0x750(%rsp),%rcx
   36a12:	movabs $0x3fffffffffffffff,%rax
   36a1c:	cmp    %rax,0x8(%rsp)
   36a21:	ja     37188 <meshopt_peers::run+0x2598>
   36a27:	mov    0x8(%rsp),%rax
   36a2c:	lea    0x0(,%rax,4),%rax
   36a34:	test   %rax,%rax
   36a37:	js     37188 <meshopt_peers::run+0x2598>
   36a3d:	sub    %rax,%rcx
   36a40:	jb     37188 <meshopt_peers::run+0x2598>
   36a46:	mov    %rcx,0x750(%rsp)
   36a4e:	cmpq   $0x0,0x8(%rsp)
   36a54:	je     36a67 <meshopt_peers::run+0x1e77>
   36a56:	mov    %rbx,%rdi
   36a59:	call   *0xf26e9(%rip)        # 129148 <free@GLIBC_2.2.5>
   36a5f:	mov    0x750(%rsp),%rcx
   36a67:	mov    0x60(%rsp),%rax
   36a6c:	mov    $0x14,%edx
   36a71:	mul    %rdx
   36a74:	jo     37207 <meshopt_peers::run+0x2617>
   36a7a:	test   %rax,%rax
   36a7d:	js     37207 <meshopt_peers::run+0x2617>
   36a83:	sub    %rax,%rcx
   36a86:	jb     37207 <meshopt_peers::run+0x2617>
   36a8c:	mov    %rcx,0x750(%rsp)
   36a94:	cmpq   $0x0,0x60(%rsp)
   36a9a:	je     36ab2 <meshopt_peers::run+0x1ec2>
   36a9c:	mov    0xa8(%rsp),%rdi
   36aa4:	call   *0xf269e(%rip)        # 129148 <free@GLIBC_2.2.5>
   36aaa:	mov    0x750(%rsp),%rcx
   36ab2:	movabs $0x3fffffffffffffff,%rax
   36abc:	cmp    %rax,0x30(%rsp)
   36ac1:	ja     37238 <meshopt_peers::run+0x2648>
   36ac7:	mov    0x30(%rsp),%rax
   36acc:	lea    0x0(,%rax,4),%rax
   36ad4:	test   %rax,%rax
   36ad7:	js     37238 <meshopt_peers::run+0x2648>
   36add:	sub    %rax,%rcx
   36ae0:	jb     37238 <meshopt_peers::run+0x2648>
   36ae6:	mov    %rcx,0x750(%rsp)
   36aee:	cmpq   $0x0,0x30(%rsp)
   36af4:	je     36b01 <meshopt_peers::run+0x1f11>
   36af6:	mov    0x58(%rsp),%rdi
   36afb:	call   *0xf2647(%rip)        # 129148 <free@GLIBC_2.2.5>
   36b01:	dec    %r14
   36b04:	cmp    $0x1,%r14
   36b08:	mov    0x1b8(%rsp),%rax
   36b10:	ja     36428 <meshopt_peers::run+0x1838>
   36b16:	jmp    36b1d <meshopt_peers::run+0x1f2d>
   36b18:	xor    %r12d,%r12d
   36b1b:	xor    %ebp,%ebp
   36b1d:	mov    %rbp,0x18(%rsp)
   36b22:	lea    0x738(%rsp),%rbx
   36b2a:	mov    %rbx,%rdi
   36b2d:	mov    0x40(%rsp),%rsi
   36b32:	mov    0xd0(%rsp),%rdx
   36b3a:	call   33710 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36b3f:	mov    %eax,%ebp
   36b41:	mov    $0x1,%al
   36b43:	mov    %eax,0x8(%rsp)
   36b47:	cmp    $0xff,%bpl
   36b4b:	setne  %r13b
   36b4f:	jne    3708b <meshopt_peers::run+0x249b>
   36b55:	mov    0x48(%rsp),%r15
   36b5a:	cmp    %r15,%r12
   36b5d:	cmovb  %r12,%r15
   36b61:	mov    0x140(%rsp),%r14
   36b69:	mov    0x18(%rsp),%rax
   36b6e:	cmp    %r14,%rax
   36b71:	cmovb  %rax,%r14
   36b75:	mov    %rbx,%rdi
   36b78:	mov    0x10(%rsp),%rsi
   36b7d:	mov    0x38(%rsp),%rdx
   36b82:	call   336c0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36b87:	cmp    $0xff,%al
   36b89:	jne    370aa <meshopt_peers::run+0x24ba>
   36b8f:	mov    %rbx,%rdi
   36b92:	mov    0x70(%rsp),%rsi
   36b97:	mov    0x28(%rsp),%rdx
   36b9c:	call   33710 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36ba1:	cmp    $0xff,%al
   36ba3:	jne    370b4 <meshopt_peers::run+0x24c4>
   36ba9:	mov    0x148(%rsp),%r13
   36bb1:	inc    %r13d
   36bb4:	mov    0x68(%rsp),%rax
   36bb9:	mov    %rax,0x10(%rsp)
   36bbe:	mov    %r15,%rdx
   36bc1:	mov    0xb0(%rsp),%rax
   36bc9:	mov    %rax,0x38(%rsp)
   36bce:	mov    %r14,%r8
   36bd1:	mov    0xc0(%rsp),%rax
   36bd9:	mov    %rax,0x28(%rsp)
   36bde:	mov    0x80(%rsp),%rax
   36be6:	mov    %rax,0x70(%rsp)
   36beb:	cmp    $0x1,%r15
   36bef:	mov    0x20(%rsp),%rbx
   36bf4:	lea    0x170(%rsp),%rdi
   36bfc:	mov    0xf2955(%rip),%r10        # 129558 <_DYNAMIC+0x618>
   36c03:	ja     35f83 <meshopt_peers::run+0x1393>
   36c09:	jmp    370c6 <meshopt_peers::run+0x24d6>
   36c0e:	mov    $0x8,%edx
   36c13:	mov    $0x8,%r8d
   36c19:	mov    $0x30,%r9d
   36c1f:	lea    0x350(%rsp),%rdi
   36c27:	xor    %esi,%esi
   36c29:	mov    0x48(%rsp),%rcx
   36c2e:	call   3b500 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36c33:	cmpl   $0x1,0x350(%rsp)
   36c3b:	je     37146 <meshopt_peers::run+0x2556>
   36c41:	mov    0x358(%rsp),%r15
   36c49:	mov    0x48(%rsp),%rax
   36c4e:	mov    0x20(%rsp),%rbx
   36c53:	mov    0x18(%rsp),%r8
   36c58:	jmp    361aa <meshopt_peers::run+0x15ba>
   36c5d:	mov    $0x8,%ecx
   36c62:	mov    $0x30,%r8d
   36c68:	lea    0xe0(%rsp),%rdi
   36c70:	xor    %esi,%esi
   36c72:	mov    0x48(%rsp),%rdx
   36c77:	call   34b50 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36c7c:	mov    0xf0(%rsp),%r12
   36c84:	jmp    36264 <meshopt_peers::run+0x1674>
   36c89:	mov    $0x4,%ecx
   36c8e:	mov    $0x4,%r8d
   36c94:	lea    0xe0(%rsp),%rdi
   36c9c:	mov    %r15,%rdx
   36c9f:	call   34b50 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36ca4:	mov    0xe0(%rsp),%rax
   36cac:	mov    %rax,0x80(%rsp)
   36cb4:	mov    0xf0(%rsp),%rsi
   36cbc:	jmp    3637b <meshopt_peers::run+0x178b>
   36cc1:	mov    0xb8(%rsp),%rbp
   36cc9:	jmp    36b1d <meshopt_peers::run+0x1f2d>
   36cce:	mov    $0x1,%al
   36cd0:	mov    %eax,0x8(%rsp)
   36cd4:	jmp    36da4 <meshopt_peers::run+0x21b4>
   36cd9:	lea    0x738(%rsp),%rdi
   36ce1:	mov    %r12,%rsi
   36ce4:	mov    0x38(%rsp),%rdx
   36ce9:	call   336c0 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36cee:	cmp    $0xff,%al
   36cf0:	jne    3726e <meshopt_peers::run+0x267e>
   36cf6:	lea    0x738(%rsp),%rdi
   36cfe:	mov    0x80(%rsp),%rsi
   36d06:	mov    0x28(%rsp),%rdx
   36d0b:	call   33710 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36d10:	mov    %eax,%ebp
   36d12:	jmp    357ee <meshopt_peers::run+0xbfe>
   36d17:	movzbl 0x2c8(%rsp),%eax
   36d1f:	mov    %al,0x738(%rsp)
   36d26:	lea    -0x2677a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36d2d:	lea    0xeb34c(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36d34:	lea    0xeb0d5(%rip),%r8        # 121e10 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36d3b:	lea    0x738(%rsp),%rdx
   36d43:	mov    $0x2b,%esi
   36d48:	call   *0xf24c2(%rip)        # 129210 <_DYNAMIC+0x2d0>
   36d4e:	lea    0x290(%rsp),%rax
   36d56:	mov    %rax,0x738(%rsp)
   36d5e:	lea    0x55fb(%rip),%rax        # 3c360 <<&str as core::fmt::Display>::fmt>
   36d65:	mov    %rax,0x740(%rsp)
   36d6d:	lea    -0x2a679(%rip),%rdi        # c6fb <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36d74:	lea    0xeaf8d(%rip),%rdx        # 121d08 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x120>
   36d7b:	lea    0x738(%rsp),%rsi
   36d83:	call   *0xf2457(%rip)        # 1291e0 <_DYNAMIC+0x2a0>
   36d89:	cmpq   $0x0,0x68(%rsp)
   36d8f:	mov    0x10(%rsp),%r12
   36d94:	je     36d9f <meshopt_peers::run+0x21af>
   36d96:	mov    %r15,%rdi
   36d99:	call   *0xf23a9(%rip)        # 129148 <free@GLIBC_2.2.5>
   36d9f:	mov    0x40(%rsp),%r15
   36da4:	mov    $0x1,%r13b
   36da7:	test   %r15,%r15
   36daa:	je     36dba <meshopt_peers::run+0x21ca>
   36dac:	mov    0xd0(%rsp),%rdi
   36db4:	call   *0xf238e(%rip)        # 129148 <free@GLIBC_2.2.5>
   36dba:	mov    $0x1,%r14b
   36dbd:	mov    $0x1,%r15b
   36dc0:	cmpb   $0x0,0x8(%rsp)
   36dc5:	mov    0x70(%rsp),%rax
   36dca:	jne    36ee6 <meshopt_peers::run+0x22f6>
   36dd0:	jmp    36ef6 <meshopt_peers::run+0x2306>
   36dd5:	movzbl 0x740(%rsp),%eax
   36ddd:	mov    %al,0x350(%rsp)
   36de4:	lea    -0x26838(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36deb:	lea    0xeb28e(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36df2:	lea    0xeafb7(%rip),%r8        # 121db0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   36df9:	lea    0x350(%rsp),%rdx
   36e01:	mov    $0x2b,%esi
   36e06:	call   *0xf2404(%rip)        # 129210 <_DYNAMIC+0x2d0>
   36e0c:	mov    $0x8,%bpl
   36e0f:	jmp    357d5 <meshopt_peers::run+0xbe5>
   36e14:	mov    $0x8,%bpl
   36e17:	cmpq   $0x0,0x8(%rsp)
   36e1d:	je     36f63 <meshopt_peers::run+0x2373>
   36e23:	mov    %rbx,%rdi
   36e26:	call   *0xf231c(%rip)        # 129148 <free@GLIBC_2.2.5>
   36e2c:	xor    %r15d,%r15d
   36e2f:	cmpq   $0x0,0x30(%rsp)
   36e35:	jne    36f6e <meshopt_peers::run+0x237e>
   36e3b:	jmp    36f79 <meshopt_peers::run+0x2389>
   36e40:	lea    0xeaea9(%rip),%rcx        # 121cf0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x108>
   36e47:	mov    %r14,%rdx
   36e4a:	call   *0xf2460(%rip)        # 1292b0 <_DYNAMIC+0x370>
   36e50:	jmp    372ec <meshopt_peers::run+0x26fc>
   36e55:	movzbl 0x740(%rsp),%eax
   36e5d:	mov    %al,0x350(%rsp)
   36e64:	lea    -0x268b8(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36e6b:	lea    0xeb20e(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36e72:	lea    0xeaf07(%rip),%r8        # 121d80 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36e79:	lea    0x350(%rsp),%rdx
   36e81:	mov    $0x2b,%esi
   36e86:	call   *0xf2384(%rip)        # 129210 <_DYNAMIC+0x2d0>
   36e8c:	movzbl 0x740(%rsp),%eax
   36e94:	mov    %al,0x350(%rsp)
   36e9b:	lea    -0x268ef(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36ea2:	lea    0xeb1d7(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36ea9:	lea    0xeae88(%rip),%r8        # 121d38 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36eb0:	lea    0x350(%rsp),%rdx
   36eb8:	mov    $0x2b,%esi
   36ebd:	call   *0xf234d(%rip)        # 129210 <_DYNAMIC+0x2d0>
   36ec3:	mov    $0x8,%bpl
   36ec6:	lea    0x350(%rsp),%rdi
   36ece:	call   33dd0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   36ed3:	mov    $0x1,%r14b
   36ed6:	mov    $0x1,%r15b
   36ed9:	mov    $0x1,%r13b
   36edc:	mov    0x10(%rsp),%r12
   36ee1:	mov    0x70(%rsp),%rax
   36ee6:	test   %rax,%rax
   36ee9:	je     36ef6 <meshopt_peers::run+0x2306>
   36eeb:	mov    0x28(%rsp),%rdi
   36ef0:	call   *0xf2252(%rip)        # 129148 <free@GLIBC_2.2.5>
   36ef6:	test   %r13b,%r13b
   36ef9:	je     36f0b <meshopt_peers::run+0x231b>
   36efb:	test   %r12,%r12
   36efe:	je     36f0b <meshopt_peers::run+0x231b>
   36f00:	mov    0x38(%rsp),%rdi
   36f05:	call   *0xf223d(%rip)        # 129148 <free@GLIBC_2.2.5>
   36f0b:	test   %r15b,%r15b
   36f0e:	jne    357bd <meshopt_peers::run+0xbcd>
   36f14:	jmp    357d0 <meshopt_peers::run+0xbe0>
   36f19:	mov    $0x1,%al
   36f1b:	mov    %eax,0x8(%rsp)
   36f1f:	mov    $0x1,%r13b
   36f22:	mov    0x10(%rsp),%r12
   36f27:	jmp    36da7 <meshopt_peers::run+0x21b7>
   36f2c:	movzbl 0x740(%rsp),%eax
   36f34:	mov    %al,0x350(%rsp)
   36f3b:	lea    -0x2698f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f42:	lea    0xeb137(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f49:	lea    0xeae78(%rip),%r8        # 121dc8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   36f50:	lea    0x350(%rsp),%rdx
   36f58:	mov    $0x2b,%esi
   36f5d:	call   *0xf22ad(%rip)        # 129210 <_DYNAMIC+0x2d0>
   36f63:	xor    %r15d,%r15d
   36f66:	cmpq   $0x0,0x30(%rsp)
   36f6c:	je     36f79 <meshopt_peers::run+0x2389>
   36f6e:	mov    0x58(%rsp),%rdi
   36f73:	call   *0xf21cf(%rip)        # 129148 <free@GLIBC_2.2.5>
   36f79:	cmpq   $0x0,0x60(%rsp)
   36f7f:	je     36f8f <meshopt_peers::run+0x239f>
   36f81:	mov    0xa8(%rsp),%rdi
   36f89:	call   *0xf21b9(%rip)        # 129148 <free@GLIBC_2.2.5>
   36f8f:	cmpq   $0x0,0x8(%rsp)
   36f95:	setne  %al
   36f98:	test   %r15b,%al
   36f9b:	je     36fce <meshopt_peers::run+0x23de>
   36f9d:	mov    %rbx,%rdi
   36fa0:	jmp    36fc8 <meshopt_peers::run+0x23d8>
   36fa2:	cmpq   $0x0,0x60(%rsp)
   36fa8:	je     36fb8 <meshopt_peers::run+0x23c8>
   36faa:	mov    0xa8(%rsp),%rdi
   36fb2:	call   *0xf2190(%rip)        # 129148 <free@GLIBC_2.2.5>
   36fb8:	mov    $0x8,%bpl
   36fbb:	cmpq   $0x0,0x30(%rsp)
   36fc1:	je     36fce <meshopt_peers::run+0x23de>
   36fc3:	mov    0x58(%rsp),%rdi
   36fc8:	call   *0xf217a(%rip)        # 129148 <free@GLIBC_2.2.5>
   36fce:	lea    0xe0(%rsp),%rdi
   36fd6:	call   33800 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   36fdb:	mov    $0x1,%r15b
   36fde:	mov    $0x1,%al
   36fe0:	mov    %eax,0x8(%rsp)
   36fe4:	mov    $0x1,%r13b
   36fe7:	cmpq   $0x0,0x80(%rsp)
   36ff0:	je     37000 <meshopt_peers::run+0x2410>
   36ff2:	mov    0xc0(%rsp),%rdi
   36ffa:	call   *0xf2148(%rip)        # 129148 <free@GLIBC_2.2.5>
   37000:	cmpq   $0x0,0x68(%rsp)
   37006:	je     37016 <meshopt_peers::run+0x2426>
   37008:	mov    0xb0(%rsp),%rdi
   37010:	call   *0xf2132(%rip)        # 129148 <free@GLIBC_2.2.5>
   37016:	test   %r15b,%r15b
   37019:	mov    0x20(%rsp),%rbx
   3701e:	mov    0x10(%rsp),%r12
   37023:	mov    0x40(%rsp),%r15
   37028:	jne    36da7 <meshopt_peers::run+0x21b7>
   3702e:	jmp    36dba <meshopt_peers::run+0x21ca>
   37033:	mov    0x10(%rsp),%r12
   37038:	jmp    36da4 <meshopt_peers::run+0x21b4>
   3703d:	cmpq   $0x0,0x68(%rsp)
   37043:	je     3706c <meshopt_peers::run+0x247c>
   37045:	mov    0xb0(%rsp),%rdi
   3704d:	call   *0xf20f5(%rip)        # 129148 <free@GLIBC_2.2.5>
   37053:	mov    $0x1,%r13b
   37056:	jmp    37077 <meshopt_peers::run+0x2487>
   37058:	mov    $0x8,%bpl
   3705b:	cmpq   $0x0,0x30(%rsp)
   37061:	je     36fce <meshopt_peers::run+0x23de>
   37067:	jmp    36fc3 <meshopt_peers::run+0x23d3>
   3706c:	mov    $0x1,%r13b
   3706f:	jmp    37077 <meshopt_peers::run+0x2487>
   37071:	mov    $0x1,%r13b
   37074:	mov    %r15d,%ebp
   37077:	mov    0x20(%rsp),%rbx
   3707c:	mov    0x10(%rsp),%r12
   37081:	mov    0x40(%rsp),%r15
   37086:	jmp    36da7 <meshopt_peers::run+0x21b7>
   3708b:	xor    %r15d,%r15d
   3708e:	jmp    36fe7 <meshopt_peers::run+0x23f7>
   37093:	shr    $0x8,%ebp
   37096:	mov    $0x1,%r15b
   37099:	cmpq   $0x0,0x30(%rsp)
   3709f:	jne    36f6e <meshopt_peers::run+0x237e>
   370a5:	jmp    36f79 <meshopt_peers::run+0x2389>
   370aa:	mov    %eax,%ebp
   370ac:	xor    %r15d,%r15d
   370af:	jmp    36fe7 <meshopt_peers::run+0x23f7>
   370b4:	mov    %eax,%ebp
   370b6:	xor    %r15d,%r15d
   370b9:	movl   $0x0,0x8(%rsp)
   370c1:	jmp    36fe7 <meshopt_peers::run+0x23f7>
   370c6:	mov    0xb0(%rsp),%rax
   370ce:	mov    %rax,0x38(%rsp)
   370d3:	mov    0xc0(%rsp),%rax
   370db:	mov    %rax,0x28(%rsp)
   370e0:	test   %r15,%r15
   370e3:	mov    0x68(%rsp),%r12
   370e8:	jne    35e85 <meshopt_peers::run+0x1295>
   370ee:	jmp    35dc2 <meshopt_peers::run+0x11d2>
   370f3:	test   %r15,%r15
   370f6:	je     37101 <meshopt_peers::run+0x2511>
   370f8:	mov    %r13,%rdi
   370fb:	call   *0xf2047(%rip)        # 129148 <free@GLIBC_2.2.5>
   37101:	mov    $0x1,%r13b
   37104:	mov    $0x1,%r15b
   37107:	cmpq   $0x0,0xa0(%rsp)
   37110:	je     3711f <meshopt_peers::run+0x252f>
   37112:	mov    0x78(%rsp),%rdi
   37117:	call   *0xf202b(%rip)        # 129148 <free@GLIBC_2.2.5>
   3711d:	jmp    37122 <meshopt_peers::run+0x2532>
   3711f:	mov    $0x1,%r15b
   37122:	cmpq   $0x0,0x30(%rsp)
   37128:	je     37135 <meshopt_peers::run+0x2545>
   3712a:	mov    0x58(%rsp),%rdi
   3712f:	call   *0xf2013(%rip)        # 129148 <free@GLIBC_2.2.5>
   37135:	mov    $0x8,%bpl
   37138:	test   %r13b,%r13b
   3713b:	jne    36f79 <meshopt_peers::run+0x2389>
   37141:	jmp    36f8f <meshopt_peers::run+0x239f>
   37146:	mov    $0xa,%bpl
   37149:	mov    0x20(%rsp),%rbx
   3714e:	mov    0x10(%rsp),%r12
   37153:	jmp    36d9f <meshopt_peers::run+0x21af>
   37158:	movb   $0x2,0x738(%rsp)
   37160:	lea    -0x26bb4(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37167:	lea    0xeaf12(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3716e:	lea    0xeac83(%rip),%r8        # 121df8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   37175:	lea    0x738(%rsp),%rdx
   3717d:	mov    $0x2b,%esi
   37182:	call   *0xf2088(%rip)        # 129210 <_DYNAMIC+0x2d0>
   37188:	mov    $0x1,%r13b
   3718b:	xor    %r15d,%r15d
   3718e:	mov    %rbx,0x78(%rsp)
   37193:	cmpq   $0x0,0x8(%rsp)
   37199:	jne    37112 <meshopt_peers::run+0x2522>
   3719f:	jmp    37122 <meshopt_peers::run+0x2532>
   371a1:	movb   $0x8,0x738(%rsp)
   371a9:	lea    -0x26bfd(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   371b0:	lea    0xeaec9(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   371b7:	lea    0xeabda(%rip),%r8        # 121d98 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   371be:	lea    0x738(%rsp),%rdx
   371c6:	mov    $0x2b,%esi
   371cb:	call   *0xf203f(%rip)        # 129210 <_DYNAMIC+0x2d0>
   371d1:	mov    $0x4,%edi
   371d6:	mov    %r13,%rsi
   371d9:	call   *0xf1fa1(%rip)        # 129180 <_DYNAMIC+0x240>
   371df:	lea    0xeaaaa(%rip),%rcx        # 121c90 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   371e6:	mov    %r15,%rsi
   371e9:	call   *0xf20c1(%rip)        # 1292b0 <_DYNAMIC+0x370>
   371ef:	jmp    372ec <meshopt_peers::run+0x26fc>
   371f4:	mov    $0x4,%edi
   371f9:	mov    %r15,%rsi
   371fc:	call   *0xf1f7e(%rip)        # 129180 <_DYNAMIC+0x240>
   37202:	jmp    372ec <meshopt_peers::run+0x26fc>
   37207:	xor    %r13d,%r13d
   3720a:	cmpq   $0x0,0x60(%rsp)
   37210:	je     37227 <meshopt_peers::run+0x2637>
   37212:	mov    0xa8(%rsp),%rax
   3721a:	mov    %rax,0x78(%rsp)
   3721f:	xor    %r15d,%r15d
   37222:	jmp    37112 <meshopt_peers::run+0x2522>
   37227:	movq   $0x0,0x60(%rsp)
   37230:	xor    %r15d,%r15d
   37233:	jmp    37122 <meshopt_peers::run+0x2532>
   37238:	mov    $0x8,%bpl
   3723b:	cmpq   $0x0,0x30(%rsp)
   37241:	je     36fce <meshopt_peers::run+0x23de>
   37247:	jmp    36fc3 <meshopt_peers::run+0x23d3>
   3724c:	lea    0xeaa25(%rip),%rcx        # 121c78 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   37253:	mov    0xb8(%rsp),%rdi
   3725b:	mov    %rbp,%rsi
   3725e:	mov    0x140(%rsp),%rdx
   37266:	call   *0xf2044(%rip)        # 1292b0 <_DYNAMIC+0x370>
   3726c:	jmp    372ec <meshopt_peers::run+0x26fc>
   3726e:	mov    %eax,%ebp
   37270:	cmpq   $0x0,0x80(%rsp)
   37279:	je     37286 <meshopt_peers::run+0x2696>
   3727b:	mov    0x28(%rsp),%rdi
   37280:	call   *0xf1ec2(%rip)        # 129148 <free@GLIBC_2.2.5>
   37286:	mov    0x738(%rsp),%rax
   3728e:	mov    0x748(%rsp),%rcx
   37296:	sub    0x740(%rsp),%rcx
   3729e:	mov    %rcx,0x358(%rax)
   372a5:	mov    %bpl,0x738(%rsp)
   372ad:	lea    -0x26d01(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   372b4:	lea    0xeadc5(%rip),%rcx        # 122080 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   372bb:	lea    0xeab66(%rip),%r8        # 121e28 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   372c2:	lea    0x738(%rsp),%rdx
   372ca:	mov    $0x2b,%esi
   372cf:	call   *0xf1f3b(%rip)        # 129210 <_DYNAMIC+0x2d0>
   372d5:	jmp    372ec <meshopt_peers::run+0x26fc>
   372d7:	lea    0xea982(%rip),%rdx        # 121c60 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   372de:	mov    %r12,%rdi
   372e1:	mov    0x48(%rsp),%rsi
   372e6:	call   *0xf1ea4(%rip)        # 129190 <_DYNAMIC+0x250>
   372ec:	ud2
   372ee:	mov    %rax,%r14
   372f1:	cmpq   $0x0,0xe0(%rsp)
   372fa:	je     37493 <meshopt_peers::run+0x28a3>
   37300:	mov    0xe8(%rsp),%rdi
   37308:	jmp    3748d <meshopt_peers::run+0x289d>
   3730d:	mov    %rax,%r14
   37310:	cmpq   $0x0,0xe0(%rsp)
   37319:	je     374a9 <meshopt_peers::run+0x28b9>
   3731f:	mov    0xe8(%rsp),%r15
   37327:	jmp    37356 <meshopt_peers::run+0x2766>
   37329:	jmp    3732b <meshopt_peers::run+0x273b>
   3732b:	mov    %rax,%r14
   3732e:	cmpq   $0x0,0x30(%rsp)
   37334:	je     373c8 <meshopt_peers::run+0x27d8>
   3733a:	jmp    37414 <meshopt_peers::run+0x2824>
   3733f:	mov    %rax,%r14
   37342:	jmp    373e1 <meshopt_peers::run+0x27f1>
   37347:	mov    %rax,%r14
   3734a:	cmpq   $0x0,0x68(%rsp)
   37350:	je     374a9 <meshopt_peers::run+0x28b9>
   37356:	mov    %r15,%rdi
   37359:	jmp    374a3 <meshopt_peers::run+0x28b3>
   3735e:	mov    %rax,%r14
   37361:	jmp    37493 <meshopt_peers::run+0x28a3>
   37366:	mov    %rax,%r14
   37369:	mov    %r12,0x10(%rsp)
   3736e:	mov    0x80(%rsp),%rax
   37376:	mov    %rax,0x70(%rsp)
   3737b:	cmpq   $0x0,0x70(%rsp)
   37381:	je     374b9 <meshopt_peers::run+0x28c9>
   37387:	jmp    37532 <meshopt_peers::run+0x2942>
   3738c:	mov    %rax,%r14
   3738f:	cmpq   $0x0,0x70(%rsp)
   37395:	je     374b9 <meshopt_peers::run+0x28c9>
   3739b:	jmp    37532 <meshopt_peers::run+0x2942>
   373a0:	mov    %r15,0x40(%rsp)
   373a5:	mov    %rax,%r14
   373a8:	jmp    374a9 <meshopt_peers::run+0x28b9>
   373ad:	mov    %rax,%r14
   373b0:	cmpq   $0x0,0xa0(%rsp)
   373b9:	jne    373f3 <meshopt_peers::run+0x2803>
   373bb:	test   %r15,%r15
   373be:	jne    37403 <meshopt_peers::run+0x2813>
   373c0:	cmpq   $0x0,0x30(%rsp)
   373c6:	jne    37414 <meshopt_peers::run+0x2824>
   373c8:	cmpq   $0x0,0x60(%rsp)
   373ce:	jne    37427 <meshopt_peers::run+0x2837>
   373d0:	cmpq   $0x0,0x8(%rsp)
   373d6:	je     373e1 <meshopt_peers::run+0x27f1>
   373d8:	mov    %rbx,%rdi
   373db:	call   *0xf1d67(%rip)        # 129148 <free@GLIBC_2.2.5>
   373e1:	lea    0xe0(%rsp),%rdi
   373e9:	call   33800 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   373ee:	jmp    3747a <meshopt_peers::run+0x288a>
   373f3:	mov    0x78(%rsp),%rdi
   373f8:	call   *0xf1d4a(%rip)        # 129148 <free@GLIBC_2.2.5>
   373fe:	test   %r15,%r15
   37401:	je     373c0 <meshopt_peers::run+0x27d0>
   37403:	mov    %r13,%rdi
   37406:	call   *0xf1d3c(%rip)        # 129148 <free@GLIBC_2.2.5>
   3740c:	cmpq   $0x0,0x30(%rsp)
   37412:	je     373c8 <meshopt_peers::run+0x27d8>
   37414:	mov    0x58(%rsp),%rdi
   37419:	call   *0xf1d29(%rip)        # 129148 <free@GLIBC_2.2.5>
   3741f:	cmpq   $0x0,0x60(%rsp)
   37425:	je     373d0 <meshopt_peers::run+0x27e0>
   37427:	mov    0xa8(%rsp),%rdi
   3742f:	call   *0xf1d13(%rip)        # 129148 <free@GLIBC_2.2.5>
   37435:	cmpq   $0x0,0x8(%rsp)
   3743b:	jne    373d8 <meshopt_peers::run+0x27e8>
   3743d:	jmp    373e1 <meshopt_peers::run+0x27f1>
   3743f:	jmp    37441 <meshopt_peers::run+0x2851>
   37441:	mov    %rax,%r14
   37444:	cmpq   $0x0,0x20(%rsp)
   3744a:	je     374d1 <meshopt_peers::run+0x28e1>
   37450:	jmp    3755c <meshopt_peers::run+0x296c>
   37455:	mov    %rax,%r14
   37458:	lea    0x350(%rsp),%rdi
   37460:	call   33d90 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   37465:	mov    %r14,%rdi
   37468:	call   120190 <_Unwind_Resume@plt>
   3746d:	jmp    3751a <meshopt_peers::run+0x292a>
   37472:	jmp    3751a <meshopt_peers::run+0x292a>
   37477:	mov    %rax,%r14
   3747a:	cmpq   $0x0,0x80(%rsp)
   37483:	je     37493 <meshopt_peers::run+0x28a3>
   37485:	mov    0xc0(%rsp),%rdi
   3748d:	call   *0xf1cb5(%rip)        # 129148 <free@GLIBC_2.2.5>
   37493:	cmpq   $0x0,0x68(%rsp)
   37499:	je     374a9 <meshopt_peers::run+0x28b9>
   3749b:	mov    0xb0(%rsp),%rdi
   374a3:	call   *0xf1c9f(%rip)        # 129148 <free@GLIBC_2.2.5>
   374a9:	cmpq   $0x0,0x40(%rsp)
   374af:	jne    374e5 <meshopt_peers::run+0x28f5>
   374b1:	cmpq   $0x0,0x70(%rsp)
   374b7:	jne    37532 <meshopt_peers::run+0x2942>
   374b9:	cmpq   $0x0,0x10(%rsp)
   374bf:	jne    37545 <meshopt_peers::run+0x2955>
   374c5:	cmpq   $0x0,0x20(%rsp)
   374cb:	jne    3755c <meshopt_peers::run+0x296c>
   374d1:	cmpq   $0x0,0x150(%rsp)
   374da:	jne    37575 <meshopt_peers::run+0x2985>
   374e0:	jmp    37583 <meshopt_peers::run+0x2993>
   374e5:	mov    0xd0(%rsp),%rdi
   374ed:	call   *0xf1c55(%rip)        # 129148 <free@GLIBC_2.2.5>
   374f3:	cmpq   $0x0,0x70(%rsp)
   374f9:	je     374b9 <meshopt_peers::run+0x28c9>
   374fb:	jmp    37532 <meshopt_peers::run+0x2942>
   374fd:	mov    %rax,%r14
   37500:	cmpq   $0x0,0x150(%rsp)
   37509:	jne    37575 <meshopt_peers::run+0x2985>
   3750b:	jmp    37583 <meshopt_peers::run+0x2993>
   3750d:	mov    %rax,%r14
   37510:	jmp    37583 <meshopt_peers::run+0x2993>
   37512:	mov    %rax,%r14
   37515:	jmp    375a2 <meshopt_peers::run+0x29b2>
   3751a:	mov    %rax,%r14
   3751d:	lea    0x350(%rsp),%rdi
   37525:	call   33dd0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3752a:	cmpq   $0x0,0x70(%rsp)
   37530:	je     374b9 <meshopt_peers::run+0x28c9>
   37532:	mov    0x28(%rsp),%rdi
   37537:	call   *0xf1c0b(%rip)        # 129148 <free@GLIBC_2.2.5>
   3753d:	cmpq   $0x0,0x10(%rsp)
   37543:	je     374c5 <meshopt_peers::run+0x28d5>
   37545:	mov    0x38(%rsp),%rdi
   3754a:	call   *0xf1bf8(%rip)        # 129148 <free@GLIBC_2.2.5>
   37550:	cmpq   $0x0,0x20(%rsp)
   37556:	je     374d1 <meshopt_peers::run+0x28e1>
   3755c:	mov    0x90(%rsp),%rdi
   37564:	call   *0xf1bde(%rip)        # 129148 <free@GLIBC_2.2.5>
   3756a:	cmpq   $0x0,0x150(%rsp)
   37573:	je     37583 <meshopt_peers::run+0x2993>
   37575:	mov    0xc8(%rsp),%rdi
   3757d:	call   *0xf1bc5(%rip)        # 129148 <free@GLIBC_2.2.5>
   37583:	mov    0x738(%rsp),%rax
   3758b:	mov    0x748(%rsp),%rcx
   37593:	sub    0x740(%rsp),%rcx
   3759b:	mov    %rcx,0x358(%rax)
   375a2:	cmpq   $0x0,0x158(%rsp)
   375ab:	jne    375bd <meshopt_peers::run+0x29cd>
   375ad:	cmpq   $0x0,0x20(%rsp)
   375b3:	jne    375d3 <meshopt_peers::run+0x29e3>
   375b5:	mov    %r14,%rdi
   375b8:	call   120190 <_Unwind_Resume@plt>
   375bd:	mov    0x160(%rsp),%rdi
   375c5:	call   *0xf1b7d(%rip)        # 129148 <free@GLIBC_2.2.5>
   375cb:	cmpq   $0x0,0x20(%rsp)
   375d1:	je     375b5 <meshopt_peers::run+0x29c5>
   375d3:	mov    0x50(%rsp),%rdi
   375d8:	call   *0xf1b6a(%rip)        # 129148 <free@GLIBC_2.2.5>
   375de:	mov    %r14,%rdi
   375e1:	call   120190 <_Unwind_Resume@plt>
