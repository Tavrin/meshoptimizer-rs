
<artifacts>/screen3/driver-candidate:     file format elf64-x86-64


Disassembly of section .text:

0000000000034bb0 <meshopt_peers::run>:
   34bb0:	push   %rbp
   34bb1:	push   %r15
   34bb3:	push   %r14
   34bb5:	push   %r13
   34bb7:	push   %r12
   34bb9:	push   %rbx
   34bba:	sub    $0x1000,%rsp
   34bc1:	movq   $0x0,(%rsp)
   34bc9:	sub    $0x968,%rsp
   34bd0:	mov    %rsi,0x290(%rsp)
   34bd8:	mov    %rdx,0x298(%rsp)
   34be0:	mov    0x10(%rcx),%rbx
   34be4:	mov    0x8(%rcx),%r15
   34be8:	movb   $0x2,0x6e0(%rsp)
   34bf0:	mov    %r15,0x6e8(%rsp)
   34bf8:	mov    %rbx,0x6f0(%rsp)
   34c00:	movq   $0x0,0x718(%rsp)
   34c0c:	mov    0x78(%rcx),%rax
   34c10:	movd   0x80(%rcx),%xmm0
   34c18:	mov    %rax,0x220(%rsp)
   34c20:	movd   %xmm0,0x228(%rsp)
   34c29:	movl   $0x0,0x22c(%rsp)
   34c34:	add    $0xfffffffffffffffb,%rdx
   34c38:	cmp    $0xe,%rdx
   34c3c:	ja     36d0e <meshopt_peers::run+0x215e>
   34c42:	mov    %r8,%r12
   34c45:	mov    %rdi,%r14
   34c48:	lea    -0x24753(%rip),%rdi        # 104fc <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   34c4f:	movslq (%rdi,%rdx,4),%rdx
   34c53:	add    %rdi,%rdx
   34c56:	jmp    *%rdx
   34c58:	mov    $0x68636163,%eax
   34c5d:	xor    (%rsi),%eax
   34c5f:	movzbl 0x4(%rsi),%edx
   34c63:	xor    $0x65,%edx
   34c66:	or     %eax,%edx
   34c68:	je     35452 <meshopt_peers::run+0x8a2>
   34c6e:	mov    $0x63746566,%eax
   34c73:	xor    (%rsi),%eax
   34c75:	movzbl 0x4(%rsi),%edx
   34c79:	xor    $0x68,%edx
   34c7c:	or     %eax,%edx
   34c7e:	jne    36d0e <meshopt_peers::run+0x215e>
   34c84:	mov    %rbx,%rax
   34c87:	shr    $0x20,%rax
   34c8b:	jne    37161 <meshopt_peers::run+0x25b1>
   34c91:	mov    0x20(%rcx),%rsi
   34c95:	mov    0x28(%rcx),%rdx
   34c99:	lea    0x0(,%rbx,4),%rax
   34ca1:	lea    (%rax,%rax,2),%rax
   34ca5:	mov    %r15,0x350(%rsp)
   34cad:	mov    %rax,0x358(%rsp)
   34cb5:	mov    %rbx,0x360(%rsp)
   34cbd:	movq   $0xc,0x368(%rsp)
   34cc9:	movq   $0xc,0x370(%rsp)
   34cd5:	movq   $0x0,0x378(%rsp)
   34ce1:	lea    0x738(%rsp),%rdi
   34ce9:	lea    0x350(%rsp),%rcx
   34cf1:	mov    %r12,%r8
   34cf4:	call   *0xf47d6(%rip)        # 1294d0 <_DYNAMIC+0x5d0>
   34cfa:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34d03:	je     36d95 <meshopt_peers::run+0x21e5>
   34d09:	mov    0x768(%rsp),%rax
   34d11:	mov    %rax,0x380(%rsp)
   34d19:	movups 0x738(%rsp),%xmm0
   34d21:	movups 0x748(%rsp),%xmm1
   34d29:	movups 0x758(%rsp),%xmm2
   34d31:	movaps %xmm2,0x370(%rsp)
   34d39:	movaps %xmm1,0x360(%rsp)
   34d41:	movaps %xmm0,0x350(%rsp)
   34d49:	mov    0x360(%rsp),%rax
   34d51:	mov    %rax,0x18(%r14)
   34d55:	movups %xmm0,0x8(%r14)
   34d5a:	movups 0x368(%rsp),%xmm0
   34d62:	movups %xmm0,0x20(%r14)
   34d67:	mov    0x378(%rsp),%rax
   34d6f:	mov    %rax,0x30(%r14)
   34d73:	movl   $0x3,(%r14)
   34d7a:	jmp    3594a <meshopt_peers::run+0xd9a>
   34d7f:	movabs $0x7966696c706d6973,%rdx
   34d89:	xor    (%rsi),%rdx
   34d8c:	movabs $0x7970706f6c732d79,%rdi
   34d96:	xor    0x7(%rsi),%rdi
   34d9a:	or     %rdx,%rdi
   34d9d:	jne    36d0e <meshopt_peers::run+0x215e>
   34da3:	movdqu 0x20(%rcx),%xmm1
   34da8:	movq   $0x0,0x1d0(%rsp)
   34db4:	mov    %rax,0x170(%rsp)
   34dbc:	movd   %xmm0,0x230(%rsp)
   34dc5:	movb   $0x0,0x365(%r12)
   34dce:	movl   $0x0,0x361(%r12)
   34dda:	pxor   %xmm0,%xmm0
   34dde:	movdqu %xmm0,0x350(%r12)
   34de8:	mov    0x348(%r12),%rax
   34df0:	mov    %rax,0xe0(%rsp)
   34df8:	mov    %rax,0xe8(%rsp)
   34e00:	movdqu %xmm1,0x738(%rsp)
   34e09:	lea    0x6e0(%rsp),%rax
   34e11:	mov    %rax,0x748(%rsp)
   34e19:	lea    0xe0(%rsp),%rax
   34e21:	mov    %rax,0x750(%rsp)
   34e29:	lea    0x1d0(%rsp),%rax
   34e31:	mov    %rax,0x758(%rsp)
   34e39:	lea    0x170(%rsp),%rax
   34e41:	mov    %rax,0x760(%rsp)
   34e49:	lea    0x230(%rsp),%rax
   34e51:	mov    %rax,0x768(%rsp)
   34e59:	mov    %r12,0x770(%rsp)
   34e61:	movq   $0x0,0x778(%rsp)
   34e6d:	lea    0x350(%rsp),%rdi
   34e75:	lea    0x738(%rsp),%rsi
   34e7d:	call   84bd0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   34e82:	mov    0xe8(%rsp),%rax
   34e8a:	sub    0xe0(%rsp),%rax
   34e92:	mov    %rax,0x358(%r12)
   34e9a:	mov    0x350(%rsp),%rax
   34ea2:	movzbl 0x358(%rsp),%ecx
   34eaa:	cmp    $0xffffffffffffffff,%rax
   34eae:	je     34ed2 <meshopt_peers::run+0x322>
   34eb0:	mov    0x368(%rsp),%rdx
   34eb8:	mov    %rdx,0x2d8(%rsp)
   34ec0:	movdqu 0x359(%rsp),%xmm0
   34ec9:	movdqu %xmm0,0x2c9(%rsp)
   34ed2:	mov    %cl,0x2c8(%rsp)
   34ed9:	mov    %rax,0x2c0(%rsp)
   34ee1:	jmp    3534c <meshopt_peers::run+0x79c>
   34ee6:	movdqu (%rsi),%xmm0
   34eea:	movdqu 0x3(%rsi),%xmm1
   34eef:	pcmpeqb -0x26ea7(%rip),%xmm1        # e050 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   34ef7:	pcmpeqb -0x2732f(%rip),%xmm0        # dbd0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   34eff:	pand   %xmm1,%xmm0
   34f03:	pmovmskb %xmm0,%eax
   34f07:	cmp    $0xffff,%eax
   34f0c:	jne    36d0e <meshopt_peers::run+0x215e>
   34f12:	mov    0x40(%rcx),%rax
   34f16:	test   %rbx,%rbx
   34f19:	setne  %dl
   34f1c:	lea    0x0(,%rbx,4),%rsi
   34f24:	cmp    %rax,%rsi
   34f27:	seta   %sil
   34f2b:	and    %dl,%sil
   34f2e:	cmp    $0x1,%sil
   34f32:	je     37118 <meshopt_peers::run+0x2568>
   34f38:	mov    0x20(%rcx),%rsi
   34f3c:	mov    0x28(%rcx),%rdx
   34f40:	mov    0x38(%rcx),%rcx
   34f44:	movb   $0x0,0x738(%rsp)
   34f4c:	mov    %rcx,0x740(%rsp)
   34f54:	mov    %rax,0x748(%rsp)
   34f5c:	mov    %rbx,0x750(%rsp)
   34f64:	movq   $0x4,0x758(%rsp)
   34f70:	movq   $0x4,0x760(%rsp)
   34f7c:	pxor   %xmm0,%xmm0
   34f80:	movdqu %xmm0,0x768(%rsp)
   34f89:	sub    $0x30,%rsp
   34f8d:	mov    %r12,0x20(%rsp)
   34f92:	lea    0x250(%rsp),%rax
   34f9a:	mov    %rax,0x18(%rsp)
   34f9f:	movq   $0x0,0x8(%rsp)
   34fa8:	movq   $0x4,(%rsp)
   34fb0:	lea    -0x26f17(%rip),%r9        # e0a0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   34fb7:	lea    0x2f0(%rsp),%rdi
   34fbf:	lea    0x710(%rsp),%rcx
   34fc7:	lea    0x768(%rsp),%r8
   34fcf:	jmp    3533b <meshopt_peers::run+0x78b>
   34fd4:	movabs $0x7966696c706d6973,%rax
   34fde:	cmp    %rax,(%rsi)
   34fe1:	je     352ee <meshopt_peers::run+0x73e>
   34fe7:	movabs $0x776172647265766f,%rax
   34ff1:	cmp    %rax,(%rsi)
   34ff4:	je     35810 <meshopt_peers::run+0xc60>
   34ffa:	movabs $0x7374656c6873656d,%rax
   35004:	cmp    %rax,(%rsi)
   35007:	jne    36d0e <meshopt_peers::run+0x215e>
   3500d:	mov    0x20(%rcx),%rsi
   35011:	mov    0x28(%rcx),%rdx
   35015:	movq   $0x7c,0x360(%rsp)
   35021:	movq   $0x0,0x354(%rsp)
   3502d:	movl   $0x1,0x350(%rsp)
   35038:	lea    0x350(%rsp),%rax
   35040:	lea    0x738(%rsp),%rdi
   35048:	lea    0x6e0(%rsp),%r8
   35050:	mov    $0x40,%r9d
   35056:	mov    %rbx,%rcx
   35059:	push   $0x2
   3505b:	push   %r12
   3505d:	push   %rax
   3505e:	push   $0x7c
   35060:	call   b3160 <meshoptimizer_rs::meshlet::build>
   35065:	add    $0x20,%rsp
   35069:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35072:	je     36e15 <meshopt_peers::run+0x2265>
   35078:	mov    0x778(%rsp),%rax
   35080:	mov    %rax,0x390(%rsp)
   35088:	movups 0x738(%rsp),%xmm0
   35090:	movups 0x748(%rsp),%xmm1
   35098:	movups 0x758(%rsp),%xmm2
   350a0:	movups 0x768(%rsp),%xmm3
   350a8:	movaps %xmm3,0x380(%rsp)
   350b0:	movaps %xmm2,0x370(%rsp)
   350b8:	movaps %xmm1,0x360(%rsp)
   350c0:	movaps %xmm0,0x350(%rsp)
   350c8:	mov    0x358(%rsp),%rbx
   350d0:	mov    0x360(%rsp),%r12
   350d8:	test   %r12,%r12
   350db:	je     3586d <meshopt_peers::run+0xcbd>
   350e1:	mov    %r12,%r15
   350e4:	shl    $0x4,%r15
   350e8:	mov    %r15,%rdi
   350eb:	call   *0xf4007(%rip)        # 1290f8 <malloc@GLIBC_2.2.5>
   350f1:	test   %rax,%rax
   350f4:	je     371b4 <meshopt_peers::run+0x2604>
   350fa:	mov    %r12d,%ecx
   350fd:	and    $0x3,%ecx
   35100:	cmp    $0x4,%r12
   35104:	jae    35874 <meshopt_peers::run+0xcc4>
   3510a:	xor    %edx,%edx
   3510c:	jmp    358c8 <meshopt_peers::run+0xd18>
   35111:	movabs $0x65646f6365642d78,%rax
   3511b:	movabs $0x652d786574726576,%rdx
   35125:	xor    (%rsi),%rdx
   35128:	lea    0x9010000(%rax),%rdi
   3512f:	xor    0x5(%rsi),%rdi
   35133:	or     %rdx,%rdi
   35136:	je     3538a <meshopt_peers::run+0x7da>
   3513c:	movabs $0x642d786574726576,%rdx
   35146:	xor    (%rsi),%rdx
   35149:	xor    0x5(%rsi),%rax
   3514d:	or     %rdx,%rax
   35150:	jne    36d0e <meshopt_peers::run+0x215e>
   35156:	mov    0x50(%rcx),%r9
   3515a:	lea    0x738(%rsp),%rdi
   35162:	mov    $0xc,%r8d
   35168:	xor    %esi,%esi
   3516a:	xor    %edx,%edx
   3516c:	mov    %rcx,%rax
   3516f:	mov    %rbx,%rcx
   35172:	push   %r12
   35174:	push   0x58(%rax)
   35177:	call   b2680 <meshoptimizer_rs::codec::allocate>
   3517c:	add    $0x10,%rsp
   35180:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35189:	jne    3542d <meshopt_peers::run+0x87d>
   3518f:	movzbl 0x740(%rsp),%eax
   35197:	mov    %al,0x350(%rsp)
   3519e:	lea    -0x24bf2(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   351a5:	lea    0xece94(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   351ac:	lea    0xecb5d(%rip),%r8        # 121d10 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   351b3:	lea    0x350(%rsp),%rdx
   351bb:	mov    $0x2b,%esi
   351c0:	call   *0xf400a(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   351c6:	movabs $0x6e652d7865646e69,%rax
   351d0:	xor    (%rsi),%rax
   351d3:	mov    0x8(%rsi),%edx
   351d6:	xor    $0x65646f63,%rdx
   351dd:	or     %rax,%rdx
   351e0:	je     35400 <meshopt_peers::run+0x850>
   351e6:	movabs $0x65642d7865646e69,%rax
   351f0:	xor    (%rsi),%rax
   351f3:	mov    0x8(%rsi),%edx
   351f6:	xor    $0x65646f63,%rdx
   351fd:	or     %rax,%rdx
   35200:	jne    36d0e <meshopt_peers::run+0x215e>
   35206:	mov    0x28(%rcx),%rax
   3520a:	mov    0x68(%rcx),%r9
   3520e:	lea    0x738(%rsp),%rdi
   35216:	mov    $0x4,%r8d
   3521c:	mov    $0x1,%esi
   35221:	xor    %edx,%edx
   35223:	mov    %rcx,%r10
   35226:	mov    %rax,%rcx
   35229:	push   %r12
   3522b:	push   0x70(%r10)
   3522f:	call   b2680 <meshoptimizer_rs::codec::allocate>
   35234:	add    $0x10,%rsp
   35238:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35241:	jne    3542d <meshopt_peers::run+0x87d>
   35247:	movzbl 0x740(%rsp),%eax
   3524f:	mov    %al,0x350(%rsp)
   35256:	lea    -0x24caa(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3525d:	lea    0xecddc(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   35264:	lea    0xeca75(%rip),%r8        # 121ce0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x138>
   3526b:	lea    0x350(%rsp),%rdx
   35273:	mov    $0x2b,%esi
   35278:	call   *0xf3f52(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   3527e:	movabs $0x2d72657473756c63,%rax
   35288:	xor    (%rsi),%rax
   3528b:	movabs $0x646f6c2d72657473,%rdx
   35295:	xor    0x3(%rsi),%rdx
   35299:	or     %rax,%rdx
   3529c:	jne    36d0e <meshopt_peers::run+0x215e>
   352a2:	mov    %r14,0x288(%rsp)
   352aa:	test   %rbx,%rbx
   352ad:	je     354b4 <meshopt_peers::run+0x904>
   352b3:	mov    %rcx,%rbp
   352b6:	lea    0x0(,%rbx,4),%rax
   352be:	lea    (%rax,%rax,2),%r13
   352c2:	mov    %r13,%rdi
   352c5:	call   *0xf3e2d(%rip)        # 1290f8 <malloc@GLIBC_2.2.5>
   352cb:	test   %rax,%rax
   352ce:	je     37191 <meshopt_peers::run+0x25e1>
   352d4:	mov    %rax,%r14
   352d7:	mov    %rax,%rdi
   352da:	mov    %r15,%rsi
   352dd:	mov    %r13,%rdx
   352e0:	call   *0xf3e32(%rip)        # 129118 <memcpy@GLIBC_2.14>
   352e6:	mov    %rbp,%rcx
   352e9:	jmp    354ba <meshopt_peers::run+0x90a>
   352ee:	mov    0x20(%rcx),%rsi
   352f2:	mov    0x28(%rcx),%rdx
   352f6:	movb   $0x2,0x738(%rsp)
   352fe:	sub    $0x30,%rsp
   35302:	mov    %r12,0x20(%rsp)
   35307:	lea    0x250(%rsp),%rax
   3530f:	mov    %rax,0x18(%rsp)
   35314:	pxor   %xmm0,%xmm0
   35318:	movdqu %xmm0,(%rsp)
   3531d:	lea    0x2f0(%rsp),%rdi
   35325:	lea    0x710(%rsp),%rcx
   3532d:	lea    0x768(%rsp),%r8
   35335:	mov    $0x4,%r9d
   3533b:	call   bde00 <meshoptimizer_rs::simplify::allocating>
   35340:	add    $0x30,%rsp
   35344:	mov    0x2c0(%rsp),%rax
   3534c:	cmp    $0xffffffffffffffff,%rax
   35350:	je     36cd7 <meshopt_peers::run+0x2127>
   35356:	mov    0x2d0(%rsp),%rax
   3535e:	mov    %rax,0x18(%r14)
   35362:	movups 0x2c0(%rsp),%xmm0
   3536a:	movups %xmm0,0x8(%r14)
   3536f:	movss  0x2d8(%rsp),%xmm0
   35378:	movss  %xmm0,0x4(%r14)
   3537e:	movl   $0x2,(%r14)
   35385:	jmp    3594a <meshopt_peers::run+0xd9a>
   3538a:	lea    0x0(,%rbx,4),%rax
   35392:	lea    (%rax,%rax,2),%rdx
   35396:	lea    0x738(%rsp),%rdi
   3539e:	mov    $0xc,%r8d
   353a4:	mov    %r15,%rsi
   353a7:	mov    %rbx,%rcx
   353aa:	mov    $0x1,%r9d
   353b0:	push   %r12
   353b2:	push   $0x2
   353b4:	call   *0xf411e(%rip)        # 1294d8 <_DYNAMIC+0x5d8>
   353ba:	add    $0x10,%rsp
   353be:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   353c7:	jne    3542d <meshopt_peers::run+0x87d>
   353c9:	movzbl 0x740(%rsp),%eax
   353d1:	mov    %al,0x350(%rsp)
   353d8:	lea    -0x24e2c(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   353df:	lea    0xecc5a(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   353e6:	lea    0xec93b(%rip),%r8        # 121d28 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   353ed:	lea    0x350(%rsp),%rdx
   353f5:	mov    $0x2b,%esi
   353fa:	call   *0xf3dd0(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   35400:	mov    0x20(%rcx),%rsi
   35404:	mov    0x28(%rcx),%rdx
   35408:	lea    0x738(%rsp),%rdi
   35410:	mov    $0x1,%ecx
   35415:	mov    %r12,%r8
   35418:	call   *0xf40c2(%rip)        # 1294e0 <_DYNAMIC+0x5e0>
   3541e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35427:	je     36e4c <meshopt_peers::run+0x229c>
   3542d:	mov    0x748(%rsp),%rax
   35435:	mov    %rax,0x18(%r14)
   35439:	movups 0x738(%rsp),%xmm0
   35441:	movups %xmm0,0x8(%r14)
   35446:	movl   $0x0,(%r14)
   3544d:	jmp    3594a <meshopt_peers::run+0xd9a>
   35452:	mov    0x20(%rcx),%rsi
   35456:	mov    0x28(%rcx),%rdx
   3545a:	lea    0x738(%rsp),%rdi
   35462:	mov    %rbx,%rcx
   35465:	mov    %r12,%r8
   35468:	call   *0xf407a(%rip)        # 1294e8 <_DYNAMIC+0x5e8>
   3546e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35477:	jne    35848 <meshopt_peers::run+0xc98>
   3547d:	movzbl 0x740(%rsp),%eax
   35485:	mov    %al,0x350(%rsp)
   3548c:	lea    -0x24ee0(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   35493:	lea    0xecba6(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3549a:	lea    0xec8ff(%rip),%r8        # 121da0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   354a1:	lea    0x350(%rsp),%rdx
   354a9:	mov    $0x2b,%esi
   354ae:	call   *0xf3d1c(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   354b4:	mov    $0x4,%r14d
   354ba:	movq   $0x0,0x158(%rsp)
   354c6:	movq   $0x4,0x160(%rsp)
   354d2:	movq   $0x0,0x168(%rsp)
   354de:	movl   $0x0,0x9c(%rsp)
   354e9:	movq   $0x80,0x1d0(%rsp)
   354f5:	movq   $0x2a,0x1d8(%rsp)
   35501:	movq   $0x80,0x1e0(%rsp)
   3550d:	movq   $0x10,0x1e8(%rsp)
   35519:	movaps -0x27630(%rip),%xmm0        # def0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   35520:	movups %xmm0,0x1f0(%rsp)
   35528:	movaps -0x2757f(%rip),%xmm0        # dfb0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3552f:	movups %xmm0,0x200(%rsp)
   35537:	movabs $0x100010100000101,%rax
   35541:	mov    %rax,0x210(%rsp)
   35549:	movl   $0x0,0x218(%rsp)
   35554:	movb   $0x1,0x21c(%rsp)
   3555c:	mov    0x20(%rcx),%r13
   35560:	mov    0x28(%rcx),%r15
   35564:	mov    %r13,0x308(%rsp)
   3556c:	mov    %r15,0x310(%rsp)
   35574:	mov    %r14,0x50(%rsp)
   35579:	mov    %r14,0x318(%rsp)
   35581:	mov    %rbx,0x20(%rsp)
   35586:	mov    %rbx,0x320(%rsp)
   3558e:	movb   $0x2,0x2c0(%rsp)
   35596:	movq   $0x4,0x328(%rsp)
   355a2:	xorps  %xmm0,%xmm0
   355a5:	movups %xmm0,0x330(%rsp)
   355ad:	movl   $0x0,0x348(%rsp)
   355b8:	movb   $0x0,0x365(%r12)
   355c1:	movl   $0x0,0x361(%r12)
   355cd:	movups %xmm0,0x350(%r12)
   355d6:	mov    0x348(%r12),%rbx
   355de:	mov    %r12,%rdi
   355e1:	xor    %esi,%esi
   355e3:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   355e8:	mov    %eax,%ebp
   355ea:	cmp    $0xff,%al
   355ec:	jne    37265 <meshopt_peers::run+0x26b5>
   355f2:	mov    %r12,0x738(%rsp)
   355fa:	mov    %rbx,0x740(%rsp)
   35602:	mov    %rbx,0x748(%rsp)
   3560a:	movq   $0x0,0x750(%rsp)
   35616:	movl   $0x0,0x758(%rsp)
   35621:	movl   $0x0,0x95c(%rsp)
   3562c:	movb   $0x1,0x1960(%rsp)
   35634:	lea    0x2c0(%rsp),%rdi
   3563c:	lea    0x1d0(%rsp),%rsi
   35644:	lea    0x738(%rsp),%rdx
   3564c:	mov    0x20(%rsp),%rbx
   35651:	call   *0xf3e99(%rip)        # 1294f0 <_DYNAMIC+0x5f0>
   35657:	mov    %eax,%ebp
   35659:	cmp    $0xff,%al
   3565b:	jne    37246 <meshopt_peers::run+0x2696>
   35661:	movzbl 0x1960(%rsp),%r12d
   3566a:	movb   $0x2,0x350(%rsp)
   35672:	mov    0x50(%rsp),%rax
   35677:	mov    %rax,0x358(%rsp)
   3567f:	mov    %rbx,0x360(%rsp)
   35687:	movq   $0x0,0x388(%rsp)
   35693:	lea    0xe0(%rsp),%rdi
   3569b:	lea    0x350(%rsp),%rsi
   356a3:	lea    0x738(%rsp),%rdx
   356ab:	call   *0xf3e47(%rip)        # 1294f8 <_DYNAMIC+0x5f8>
   356b1:	mov    0xe0(%rsp),%r14
   356b9:	movzbl 0xe8(%rsp),%ebp
   356c1:	cmp    $0xffffffffffffffff,%r14
   356c5:	je     357ae <meshopt_peers::run+0xbfe>
   356cb:	mov    %r14,0x150(%rsp)
   356d3:	movzbl %bpl,%eax
   356d7:	movzbl 0xef(%rsp),%ecx
   356df:	shl    $0x10,%ecx
   356e2:	movzwl 0xed(%rsp),%edx
   356ea:	or     %ecx,%edx
   356ec:	mov    0xe9(%rsp),%ecx
   356f3:	shl    $0x28,%rdx
   356f7:	shl    $0x8,%rcx
   356fb:	or     %rdx,%rcx
   356fe:	or     %rax,%rcx
   35701:	mov    %rcx,0xc8(%rsp)
   35709:	mov    0x750(%rsp),%rsi
   35711:	add    %rbx,%rsi
   35714:	jb     36dcc <meshopt_peers::run+0x221c>
   3571a:	mov    0xf0(%rsp),%rax
   35722:	mov    %rax,0x1c8(%rsp)
   3572a:	mov    %rsi,0x750(%rsp)
   35732:	mov    0x738(%rsp),%rdi
   3573a:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3573f:	mov    %eax,%ebp
   35741:	cmp    $0xff,%al
   35743:	jne    35795 <meshopt_peers::run+0xbe5>
   35745:	mov    $0x1,%eax
   3574a:	mov    %rax,0x90(%rsp)
   35752:	test   %rbx,%rbx
   35755:	jne    35db3 <meshopt_peers::run+0x1203>
   3575b:	mov    0x738(%rsp),%rdi
   35763:	mov    0x750(%rsp),%rsi
   3576b:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35770:	mov    %eax,%ebp
   35772:	mov    $0x1,%r14b
   35775:	cmp    $0xff,%al
   35777:	je     3595c <meshopt_peers::run+0xdac>
   3577d:	test   %rbx,%rbx
   35780:	je     35790 <meshopt_peers::run+0xbe0>
   35782:	mov    0x90(%rsp),%rdi
   3578a:	call   *0xf3978(%rip)        # 129108 <free@GLIBC_2.2.5>
   35790:	test   %r14b,%r14b
   35793:	je     357ae <meshopt_peers::run+0xbfe>
   35795:	cmpq   $0x0,0x150(%rsp)
   3579e:	je     357ae <meshopt_peers::run+0xbfe>
   357a0:	mov    0xc8(%rsp),%rdi
   357a8:	call   *0xf395a(%rip)        # 129108 <free@GLIBC_2.2.5>
   357ae:	mov    0x738(%rsp),%rax
   357b6:	mov    0x748(%rsp),%rcx
   357be:	sub    0x740(%rsp),%rcx
   357c6:	mov    %rcx,0x358(%rax)
   357cd:	cmp    $0xff,%bpl
   357d1:	jne    37265 <meshopt_peers::run+0x26b5>
   357d7:	mov    0x168(%rsp),%rax
   357df:	mov    0x288(%rsp),%rcx
   357e7:	mov    %rax,0x18(%rcx)
   357eb:	movups 0x158(%rsp),%xmm0
   357f3:	movups %xmm0,0x8(%rcx)
   357f7:	movl   $0x1,(%rcx)
   357fd:	test   %rbx,%rbx
   35800:	je     3594a <meshopt_peers::run+0xd9a>
   35806:	mov    0x50(%rsp),%rdi
   3580b:	jmp    35944 <meshopt_peers::run+0xd94>
   35810:	mov    0x20(%rcx),%rsi
   35814:	mov    0x28(%rcx),%rdx
   35818:	lea    0x738(%rsp),%rdi
   35820:	lea    0x6e0(%rsp),%rcx
   35828:	movd   -0x25bcc(%rip),%xmm0        # fc64 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x138>
   35830:	mov    %r12,%r8
   35833:	call   *0xf3cc7(%rip)        # 129500 <_DYNAMIC+0x600>
   35839:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35842:	je     36eec <meshopt_peers::run+0x233c>
   35848:	mov    0x748(%rsp),%rax
   35850:	mov    %rax,0x18(%r14)
   35854:	movups 0x738(%rsp),%xmm0
   3585c:	movups %xmm0,0x8(%r14)
   35861:	movl   $0x1,(%r14)
   35868:	jmp    3594a <meshopt_peers::run+0xd9a>
   3586d:	mov    $0x4,%eax
   35872:	jmp    358f1 <meshopt_peers::run+0xd41>
   35874:	mov    %r12,%rsi
   35877:	and    $0xfffffffffffffffc,%rsi
   3587b:	mov    $0x30,%edi
   35880:	xor    %edx,%edx
   35882:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   35890:	movups -0x30(%rbx,%rdi,1),%xmm0
   35895:	movups %xmm0,-0x30(%rax,%rdi,1)
   3589a:	movups -0x20(%rbx,%rdi,1),%xmm0
   3589f:	movups %xmm0,-0x20(%rax,%rdi,1)
   358a4:	movups -0x10(%rbx,%rdi,1),%xmm0
   358a9:	movups %xmm0,-0x10(%rax,%rdi,1)
   358ae:	movups (%rbx,%rdi,1),%xmm0
   358b2:	movups %xmm0,(%rax,%rdi,1)
   358b6:	add    $0x4,%rdx
   358ba:	add    $0x40,%rdi
   358be:	cmp    %rdx,%rsi
   358c1:	jne    35890 <meshopt_peers::run+0xce0>
   358c3:	test   %rcx,%rcx
   358c6:	je     358f1 <meshopt_peers::run+0xd41>
   358c8:	shl    $0x4,%rdx
   358cc:	mov    %rax,%rsi
   358cf:	add    %rdx,%rsi
   358d2:	add    %rbx,%rdx
   358d5:	shl    $0x4,%ecx
   358d8:	xor    %edi,%edi
   358da:	nopw   0x0(%rax,%rax,1)
   358e0:	movups (%rdx,%rdi,1),%xmm0
   358e4:	movups %xmm0,(%rsi,%rdi,1)
   358e8:	add    $0x10,%rdi
   358ec:	cmp    %rdi,%rcx
   358ef:	jne    358e0 <meshopt_peers::run+0xd30>
   358f1:	mov    0x378(%rsp),%rcx
   358f9:	mov    %rcx,0x30(%r14)
   358fd:	movups 0x368(%rsp),%xmm0
   35905:	movups %xmm0,0x20(%r14)
   3590a:	movaps 0x380(%rsp),%xmm0
   35912:	movups %xmm0,0x38(%r14)
   35917:	mov    0x390(%rsp),%rcx
   3591f:	mov    %rcx,0x48(%r14)
   35923:	mov    %r12,0x8(%r14)
   35927:	mov    %rax,0x10(%r14)
   3592b:	mov    %r12,0x18(%r14)
   3592f:	movl   $0x4,(%r14)
   35936:	cmpq   $0x0,0x350(%rsp)
   3593f:	je     3594a <meshopt_peers::run+0xd9a>
   35941:	mov    %rbx,%rdi
   35944:	call   *0xf37be(%rip)        # 129108 <free@GLIBC_2.2.5>
   3594a:	add    $0x1968,%rsp
   35951:	pop    %rbx
   35952:	pop    %r12
   35954:	pop    %r13
   35956:	pop    %r14
   35958:	pop    %r15
   3595a:	pop    %rbp
   3595b:	ret
   3595c:	movb   $0x2,0xe0(%rsp)
   35964:	mov    0x50(%rsp),%rax
   35969:	mov    %rax,0xe8(%rsp)
   35971:	mov    %rbx,0xf0(%rsp)
   35979:	movq   $0x0,0x118(%rsp)
   35985:	movups 0x210(%rsp),%xmm0
   3598d:	movaps %xmm0,0x390(%rsp)
   35995:	movups 0x1d0(%rsp),%xmm0
   3599d:	movups 0x1e0(%rsp),%xmm1
   359a5:	movups 0x1f0(%rsp),%xmm2
   359ad:	movups 0x200(%rsp),%xmm3
   359b5:	movaps %xmm3,0x380(%rsp)
   359bd:	movaps %xmm2,0x370(%rsp)
   359c5:	movaps %xmm1,0x360(%rsp)
   359cd:	movaps %xmm0,0x350(%rsp)
   359d5:	movzbl %r12b,%r10d
   359d9:	lea    0x170(%rsp),%rdi
   359e1:	lea    0xe0(%rsp),%rcx
   359e9:	lea    0x350(%rsp),%r8
   359f1:	mov    %r13,%rsi
   359f4:	mov    %r15,%rdx
   359f7:	mov    $0xffffffff,%r9d
   359fd:	lea    0x738(%rsp),%rax
   35a05:	push   %rax
   35a06:	mov    %r10,0x1c8(%rsp)
   35a0e:	push   %r10
   35a10:	call   *0xf3af2(%rip)        # 129508 <_DYNAMIC+0x608>
   35a16:	add    $0x10,%rsp
   35a1a:	mov    0x170(%rsp),%rdi
   35a22:	movzbl 0x178(%rsp),%ebp
   35a2a:	cmp    $0xffffffffffffffff,%rdi
   35a2e:	je     3577d <meshopt_peers::run+0xbcd>
   35a34:	movzbl %bpl,%eax
   35a38:	movzbl 0x17f(%rsp),%ecx
   35a40:	shl    $0x10,%ecx
   35a43:	movzwl 0x17d(%rsp),%edx
   35a4b:	or     %ecx,%edx
   35a4d:	mov    0x179(%rsp),%ecx
   35a54:	mov    0x180(%rsp),%r9
   35a5c:	mov    0x188(%rsp),%r8
   35a64:	mov    0x190(%rsp),%rsi
   35a6c:	mov    %rsi,0x28(%rsp)
   35a71:	shl    $0x28,%rdx
   35a75:	shl    $0x8,%rcx
   35a79:	or     %rdx,%rcx
   35a7c:	or     %rax,%rcx
   35a7f:	mov    %rcx,0x38(%rsp)
   35a84:	test   %r9,%r9
   35a87:	je     35d77 <meshopt_peers::run+0x11c7>
   35a8d:	mov    %r8,0x70(%rsp)
   35a92:	mov    %rdi,0x10(%rsp)
   35a97:	mov    0x198(%rsp),%r14
   35a9f:	mov    %r9,0xd8(%rsp)
   35aa7:	mov    %r9,%rax
   35aaa:	shl    $0x4,%rax
   35aae:	lea    (%rax,%rax,2),%rax
   35ab2:	mov    %rax,0x18(%rsp)
   35ab7:	xor    %r12d,%r12d
   35aba:	mov    0x738(%rsp),%r15
   35ac2:	mov    0x740(%rsp),%rbp
   35aca:	mov    0x748(%rsp),%rax
   35ad2:	sub    %rbp,%rax
   35ad5:	mov    %rax,0x358(%r15)
   35adc:	mov    %r15,%rdi
   35adf:	call   *0xf39d3(%rip)        # 1294b8 <_DYNAMIC+0x5b8>
   35ae5:	mov    0x340(%r15),%rax
   35aec:	mov    0x750(%rsp),%r13
   35af4:	sub    %r13,%rax
   35af7:	jb     35dec <meshopt_peers::run+0x123c>
   35afd:	mov    0x348(%r15),%rcx
   35b04:	sub    0x358(%r15),%rcx
   35b0b:	mov    %rax,0x690(%rsp)
   35b13:	mov    %rcx,0x698(%rsp)
   35b1b:	movb   $0x0,0x6b0(%rsp)
   35b23:	movb   $0x0,0x6b5(%rsp)
   35b2b:	movl   $0x0,0x6b1(%rsp)
   35b36:	movb   $0xff,0x6b8(%rsp)
   35b3e:	movw   $0x0,0x6b6(%rsp)
   35b48:	lea    0x6a0(%rsp),%rax
   35b50:	xorps  %xmm0,%xmm0
   35b53:	movups %xmm0,(%rax)
   35b56:	movq   $0xffffffffffffffff,0x3c8(%rsp)
   35b62:	movq   $0xffffffffffffffff,0x5d0(%rsp)
   35b6e:	movq   $0x0,0x350(%rsp)
   35b7a:	movq   $0x4,0x358(%rsp)
   35b86:	movups %xmm0,-0x340(%rax)
   35b8d:	movq   $0x4,0x370(%rsp)
   35b99:	movups %xmm0,-0x328(%rax)
   35ba0:	movq   $0x4,0x388(%rsp)
   35bac:	movups %xmm0,-0x310(%rax)
   35bb3:	movq   $0x1,0x3a0(%rsp)
   35bbf:	movups %xmm0,-0x2f8(%rax)
   35bc6:	movq   $0x2,0x3b8(%rsp)
   35bd2:	movq   $0x0,0x3c0(%rsp)
   35bde:	mov    0x38(%rsp),%rax
   35be3:	mov    0x8(%rax,%r12,1),%rdi
   35be8:	mov    0x10(%rax,%r12,1),%rdx
   35bed:	mov    %rdx,%rsi
   35bf0:	add    %rdi,%rsi
   35bf3:	jb     36e00 <meshopt_peers::run+0x2250>
   35bf9:	cmp    %r14,%rsi
   35bfc:	ja     36e00 <meshopt_peers::run+0x2250>
   35c02:	mov    0x28(%rsp),%rax
   35c07:	lea    (%rax,%rdi,4),%rsi
   35c0b:	movb   $0x2,0xe0(%rsp)
   35c13:	mov    0x50(%rsp),%rax
   35c18:	mov    %rax,0xe8(%rsp)
   35c20:	mov    %rbx,0xf0(%rsp)
   35c28:	movq   $0x0,0x118(%rsp)
   35c34:	lea    0x170(%rsp),%rdi
   35c3c:	lea    0xe0(%rsp),%rcx
   35c44:	mov    0x1c0(%rsp),%r8
   35c4c:	lea    0x350(%rsp),%r9
   35c54:	call   *0xf38b6(%rip)        # 129510 <_DYNAMIC+0x610>
   35c5a:	mov    0x6a8(%rsp),%rsi
   35c62:	sub    %rsi,%rbp
   35c65:	jb     35d5a <meshopt_peers::run+0x11aa>
   35c6b:	mov    %rbp,0x740(%rsp)
   35c73:	add    0x6a0(%rsp),%r13
   35c7b:	jb     36e83 <meshopt_peers::run+0x22d3>
   35c81:	mov    %r15,%rdi
   35c84:	mov    %r13,%rsi
   35c87:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35c8c:	mov    %eax,%ebp
   35c8e:	cmp    $0xff,%al
   35c90:	jne    36e86 <meshopt_peers::run+0x22d6>
   35c96:	movzbl 0x170(%rsp),%ebx
   35c9e:	movzbl 0x171(%rsp),%ebp
   35ca6:	lea    0x172(%rsp),%rcx
   35cae:	mov    (%rcx),%rax
   35cb1:	mov    0x6(%rcx),%rcx
   35cb5:	mov    %rax,0x230(%rsp)
   35cbd:	mov    %rcx,0x236(%rsp)
   35cc5:	movss  0x180(%rsp),%xmm0
   35cce:	movaps %xmm0,0x80(%rsp)
   35cd6:	lea    0x350(%rsp),%rdi
   35cde:	call   33d90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35ce3:	cmp    $0x1,%bl
   35ce6:	mov    0x20(%rsp),%rbx
   35ceb:	je     36e93 <meshopt_peers::run+0x22e3>
   35cf1:	lea    0x232(%rsp),%rcx
   35cf9:	mov    0x8(%rcx),%eax
   35cfc:	lea    0x72c(%rsp),%rdx
   35d04:	mov    %eax,0x8(%rdx)
   35d07:	mov    (%rcx),%rax
   35d0a:	mov    %rax,(%rdx)
   35d0d:	mov    0x8(%rdx),%eax
   35d10:	mov    %eax,0x358(%rsp)
   35d17:	mov    (%rdx),%rcx
   35d1a:	mov    %rcx,0x350(%rsp)
   35d22:	mov    0x38(%rsp),%rdx
   35d27:	mov    %eax,0x24(%rdx,%r12,1)
   35d2c:	mov    %rcx,0x1c(%rdx,%r12,1)
   35d31:	xorps  %xmm0,%xmm0
   35d34:	movaps 0x80(%rsp),%xmm1
   35d3c:	movss  %xmm1,%xmm0
   35d40:	movlps %xmm0,0x28(%rdx,%r12,1)
   35d46:	add    $0x30,%r12
   35d4a:	cmp    %r12,0x18(%rsp)
   35d4f:	jne    35aba <meshopt_peers::run+0xf0a>
   35d55:	jmp    35e1e <meshopt_peers::run+0x126e>
   35d5a:	lea    0x740(%rsp),%rdi
   35d62:	call   *0xf3760(%rip)        # 1294c8 <_DYNAMIC+0x5c8>
   35d68:	mov    %eax,%ebp
   35d6a:	cmp    $0xff,%al
   35d6c:	je     35c73 <meshopt_peers::run+0x10c3>
   35d72:	jmp    36e86 <meshopt_peers::run+0x22d6>
   35d77:	mov    %rdi,%r12
   35d7a:	mov    %r8,0x80(%rsp)
   35d82:	lea    0x738(%rsp),%rdi
   35d8a:	mov    0x150(%rsp),%rsi
   35d92:	mov    0xc8(%rsp),%rdx
   35d9a:	call   336d0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35d9f:	mov    $0x1,%r13b
   35da2:	cmp    $0xff,%al
   35da4:	je     35df4 <meshopt_peers::run+0x1244>
   35da6:	mov    %eax,%ebp
   35da8:	xor    %r14d,%r14d
   35dab:	mov    $0x1,%r15b
   35dae:	jmp    35f24 <meshopt_peers::run+0x1374>
   35db3:	lea    0x350(%rsp),%rdi
   35dbb:	mov    $0x1,%edx
   35dc0:	mov    $0x1,%r8d
   35dc6:	mov    $0x1,%r9d
   35dcc:	xor    %esi,%esi
   35dce:	mov    %rbx,%rcx
   35dd1:	call   3b4c0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35dd6:	cmpl   $0x1,0x350(%rsp)
   35dde:	jne    35ec5 <meshopt_peers::run+0x1315>
   35de4:	mov    $0xa,%bpl
   35de7:	jmp    35795 <meshopt_peers::run+0xbe5>
   35dec:	mov    $0x9,%bpl
   35def:	jmp    36e93 <meshopt_peers::run+0x22e3>
   35df4:	mov    0x750(%rsp),%rax
   35dfc:	sub    %rbx,%rax
   35dff:	jae    35ee5 <meshopt_peers::run+0x1335>
   35e05:	mov    0x90(%rsp),%rdi
   35e0d:	call   *0xf32f5(%rip)        # 129108 <free@GLIBC_2.2.5>
   35e13:	mov    $0x8,%bpl
   35e16:	xor    %r14d,%r14d
   35e19:	jmp    35f21 <meshopt_peers::run+0x1371>
   35e1e:	mov    0xd8(%rsp),%rdx
   35e26:	xor    %r13d,%r13d
   35e29:	cmp    $0x1,%rdx
   35e2d:	jne    35f31 <meshopt_peers::run+0x1381>
   35e33:	mov    0x10(%rsp),%r12
   35e38:	mov    0x70(%rsp),%rax
   35e3d:	mov    %rax,0x80(%rsp)
   35e45:	mov    0x38(%rsp),%rdi
   35e4a:	movups 0x1c(%rdi),%xmm0
   35e4e:	movaps %xmm0,0x2a0(%rsp)
   35e56:	movl   $0x7f7fffff,0x2b0(%rsp)
   35e61:	lea    0x9c(%rsp),%rax
   35e69:	lea    0x158(%rsp),%r10
   35e71:	lea    0x2a0(%rsp),%r11
   35e79:	mov    $0x1,%esi
   35e7e:	mov    0x28(%rsp),%rdx
   35e83:	mov    %r14,%rcx
   35e86:	mov    0x50(%rsp),%r8
   35e8b:	mov    %rbx,%r9
   35e8e:	lea    0x738(%rsp),%r15
   35e96:	push   %r15
   35e98:	push   %rax
   35e99:	push   %r10
   35e9b:	push   %r13
   35e9d:	push   %r11
   35e9f:	push   $0x0
   35ea1:	call   33e20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   35ea6:	add    $0x30,%rsp
   35eaa:	mov    %rax,%rbp
   35ead:	test   $0x1,%bpl
   35eb1:	je     35d82 <meshopt_peers::run+0x11d2>
   35eb7:	shr    $0x8,%ebp
   35eba:	mov    $0x1,%r14b
   35ebd:	mov    $0x1,%r15b
   35ec0:	mov    $0x1,%r13b
   35ec3:	jmp    35f24 <meshopt_peers::run+0x1374>
   35ec5:	mov    0x358(%rsp),%rdi
   35ecd:	mov    %rdi,0x90(%rsp)
   35ed5:	xor    %esi,%esi
   35ed7:	mov    %rbx,%rdx
   35eda:	call   *0xf3248(%rip)        # 129128 <memset@GLIBC_2.2.5>
   35ee0:	jmp    3575b <meshopt_peers::run+0xbab>
   35ee5:	mov    %rax,0x750(%rsp)
   35eed:	test   %rbx,%rbx
   35ef0:	je     35f00 <meshopt_peers::run+0x1350>
   35ef2:	mov    0x90(%rsp),%rdi
   35efa:	call   *0xf3208(%rip)        # 129108 <free@GLIBC_2.2.5>
   35f00:	xor    %r14d,%r14d
   35f03:	lea    0x738(%rsp),%rdi
   35f0b:	mov    $0x4,%edx
   35f10:	xor    %esi,%esi
   35f12:	call   33630 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   35f17:	cmp    $0xff,%al
   35f19:	je     36c99 <meshopt_peers::run+0x20e9>
   35f1f:	mov    %eax,%ebp
   35f21:	xor    %r15d,%r15d
   35f24:	mov    0x80(%rsp),%rax
   35f2c:	jmp    36ea6 <meshopt_peers::run+0x22f6>
   35f31:	lea    0x170(%rsp),%rdi
   35f39:	mov    0xf35d8(%rip),%r10        # 129518 <_DYNAMIC+0x618>
   35f40:	mov    %r14,%r8
   35f43:	movb   $0x2,0xe0(%rsp)
   35f4b:	mov    0x50(%rsp),%rax
   35f50:	mov    %rax,0xe8(%rsp)
   35f58:	mov    %rbx,0xf0(%rsp)
   35f60:	movq   $0x0,0x118(%rsp)
   35f6c:	movups 0x210(%rsp),%xmm0
   35f74:	movaps %xmm0,0x390(%rsp)
   35f7c:	movups 0x1d0(%rsp),%xmm0
   35f84:	movups 0x1e0(%rsp),%xmm1
   35f8c:	movups 0x1f0(%rsp),%xmm2
   35f94:	movups 0x200(%rsp),%xmm3
   35f9c:	movaps %xmm3,0x380(%rsp)
   35fa4:	movaps %xmm2,0x370(%rsp)
   35fac:	movaps %xmm1,0x360(%rsp)
   35fb4:	movaps %xmm0,0x350(%rsp)
   35fbc:	mov    0x38(%rsp),%rsi
   35fc1:	mov    %rdx,0xd8(%rsp)
   35fc9:	mov    0x28(%rsp),%rcx
   35fce:	mov    %r8,0x18(%rsp)
   35fd3:	mov    0xc8(%rsp),%r9
   35fdb:	lea    0x738(%rsp),%r12
   35fe3:	push   %r12
   35fe5:	lea    0x358(%rsp),%rax
   35fed:	push   %rax
   35fee:	lea    0xf0(%rsp),%rax
   35ff6:	push   %rax
   35ff7:	push   0x1e0(%rsp)
   35ffe:	call   *%r10
   36001:	add    $0x20,%rsp
   36005:	mov    %r13,0x148(%rsp)
   3600d:	mov    0x170(%rsp),%r15
   36015:	movzbl 0x178(%rsp),%ebp
   3601d:	cmp    $0xffffffffffffffff,%r15
   36021:	je     36e93 <meshopt_peers::run+0x22e3>
   36027:	movzbl %bpl,%eax
   3602b:	movzbl 0x17f(%rsp),%ecx
   36033:	shl    $0x10,%ecx
   36036:	movzwl 0x17d(%rsp),%edx
   3603e:	or     %ecx,%edx
   36040:	mov    0x179(%rsp),%r8d
   36048:	mov    0x180(%rsp),%r14
   36050:	shl    $0x28,%rdx
   36054:	shl    $0x8,%r8
   36058:	or     %rdx,%r8
   3605b:	or     %rax,%r8
   3605e:	sub    $0x40,%rsp
   36062:	mov    %r12,0x30(%rsp)
   36067:	mov    0x208(%rsp),%rax
   3606f:	mov    %rax,0x18(%rsp)
   36074:	mov    0x108(%rsp),%rax
   3607c:	mov    %rax,0x10(%rsp)
   36081:	mov    0x58(%rsp),%rax
   36086:	mov    %rax,0x8(%rsp)
   3608b:	mov    0x68(%rsp),%rax
   36090:	mov    %rax,(%rsp)
   36094:	movq   $0x0,0x20(%rsp)
   3609d:	mov    0xd0(%rsp),%rdi
   360a5:	mov    %rbx,%rsi
   360a8:	mov    0x78(%rsp),%rdx
   360ad:	mov    0x118(%rsp),%rcx
   360b5:	mov    %r8,0x110(%rsp)
   360bd:	mov    %r14,%r9
   360c0:	call   *0xf345a(%rip)        # 129520 <_DYNAMIC+0x620>
   360c6:	add    $0x40,%rsp
   360ca:	mov    %eax,%ebp
   360cc:	cmp    $0xff,%al
   360ce:	jne    36ed9 <meshopt_peers::run+0x2329>
   360d4:	mov    0x18(%rsp),%rax
   360d9:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   360e3:	mul    %rcx
   360e6:	mov    %rdx,%rax
   360e9:	shr    $1,%rax
   360ec:	mov    %rax,0x48(%rsp)
   360f1:	mov    $0x30,%ecx
   360f6:	mul    %rcx
   360f9:	seto   %cl
   360fc:	test   %rax,%rax
   360ff:	sets   %dl
   36102:	or     %cl,%dl
   36104:	mov    $0x8,%bpl
   36107:	mov    0x10(%rsp),%r12
   3610c:	jne    36c8e <meshopt_peers::run+0x20de>
   36112:	mov    $0x1,%cl
   36114:	mov    %ecx,0x8(%rsp)
   36118:	add    0x750(%rsp),%rax
   36120:	jb     36d64 <meshopt_peers::run+0x21b4>
   36126:	mov    %rax,0x750(%rsp)
   3612e:	mov    0x738(%rsp),%r13
   36136:	mov    %r13,%rdi
   36139:	mov    %rax,%r12
   3613c:	mov    %rax,%rsi
   3613f:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   36144:	mov    %eax,%ebp
   36146:	cmp    $0xff,%al
   36148:	jne    36ff3 <meshopt_peers::run+0x2443>
   3614e:	mov    %r15,0x40(%rsp)
   36153:	mov    0x18(%rsp),%r8
   36158:	cmp    $0x3,%r8
   3615c:	jae    36bce <meshopt_peers::run+0x201e>
   36162:	mov    $0x8,%r15d
   36168:	xor    %eax,%eax
   3616a:	mov    %rax,0x68(%rsp)
   3616f:	sub    0x48(%rsp),%rax
   36174:	mov    $0x30,%ecx
   36179:	mul    %rcx
   3617c:	seto   %cl
   3617f:	test   %rax,%rax
   36182:	sets   %dl
   36185:	or     %cl,%dl
   36187:	mov    $0x8,%bpl
   3618a:	jne    36d49 <meshopt_peers::run+0x2199>
   36190:	test   %rax,%rax
   36193:	je     361c0 <meshopt_peers::run+0x1610>
   36195:	add    %r12,%rax
   36198:	jb     36d49 <meshopt_peers::run+0x2199>
   3619e:	mov    %rax,0x750(%rsp)
   361a6:	mov    %r13,%rdi
   361a9:	mov    %rax,%rsi
   361ac:	call   3bf30 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   361b1:	mov    %eax,%ebp
   361b3:	cmp    $0xff,%al
   361b5:	mov    0x18(%rsp),%r8
   361ba:	jne    36d49 <meshopt_peers::run+0x2199>
   361c0:	mov    %r15,%rdx
   361c3:	shr    $0x8,%rdx
   361c7:	mov    %r15,%rax
   361ca:	shr    $0x28,%rax
   361ce:	mov    %r15,%rcx
   361d1:	shr    $0x38,%rcx
   361d5:	mov    %cl,0xef(%rsp)
   361dc:	mov    %ax,0xed(%rsp)
   361e4:	mov    %edx,0xe9(%rsp)
   361eb:	movq   $0x0,0xf0(%rsp)
   361f7:	mov    0x68(%rsp),%rax
   361fc:	mov    %rax,0xe0(%rsp)
   36204:	mov    %r15b,0xe8(%rsp)
   3620c:	cmp    $0x2,%r8
   36210:	jbe    36299 <meshopt_peers::run+0x16e9>
   36216:	cmp    0x48(%rsp),%rax
   3621b:	jb     36c1d <meshopt_peers::run+0x206d>
   36221:	xor    %r12d,%r12d
   36224:	mov    0xe8(%rsp),%r15
   3622c:	lea    (%r12,%r12,2),%rax
   36230:	shl    $0x4,%rax
   36234:	xorps  %xmm0,%xmm0
   36237:	movups %xmm0,0x20(%r15,%rax,1)
   3623d:	movups %xmm0,0x10(%r15,%rax,1)
   36243:	movups %xmm0,(%r15,%rax,1)
   36248:	cmpq   $0x6,0x18(%rsp)
   3624e:	jb     36271 <meshopt_peers::run+0x16c1>
   36250:	lea    (%r15,%rax,1),%rdi
   36254:	add    $0x30,%rdi
   36258:	mov    0x48(%rsp),%rax
   3625d:	lea    (%rax,%rax,2),%rdx
   36261:	shl    $0x4,%rdx
   36265:	add    $0xffffffffffffffd0,%rdx
   36269:	xor    %esi,%esi
   3626b:	call   *0xf2eb7(%rip)        # 129128 <memset@GLIBC_2.2.5>
   36271:	mov    0xe0(%rsp),%rax
   36279:	mov    %rax,0x68(%rsp)
   3627e:	cmp    $0xffffffffffffffff,%rax
   36282:	je     37031 <meshopt_peers::run+0x2481>
   36288:	mov    %r15,%rdx
   3628b:	shr    $0x8,%rdx
   3628f:	add    %r12,0x48(%rsp)
   36294:	mov    0x18(%rsp),%r8
   36299:	movzbl %r15b,%eax
   3629d:	shl    $0x8,%rdx
   362a1:	or     %rax,%rdx
   362a4:	mov    %rdx,0xb0(%rsp)
   362ac:	lea    0x350(%rsp),%rdi
   362b4:	lea    0x738(%rsp),%rsi
   362bc:	mov    %r8,%rdx
   362bf:	call   33720 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   362c4:	mov    0x350(%rsp),%rdx
   362cc:	movzbl 0x358(%rsp),%ebp
   362d4:	cmp    $0xffffffffffffffff,%rdx
   362d8:	je     36ffd <meshopt_peers::run+0x244d>
   362de:	lea    0x359(%rsp),%rcx
   362e6:	mov    (%rcx),%rax
   362e9:	mov    0x7(%rcx),%rcx
   362ed:	lea    0xe9(%rsp),%rsi
   362f5:	mov    %rcx,0x7(%rsi)
   362f9:	mov    %rax,(%rsi)
   362fc:	mov    %rdx,0xe0(%rsp)
   36304:	mov    %bpl,0xe8(%rsp)
   3630c:	mov    0xf0(%rsp),%rcx
   36314:	mov    0x18(%rsp),%rsi
   36319:	mov    %rsi,%r15
   3631c:	sub    %rcx,%r15
   3631f:	mov    %rdx,0x80(%rsp)
   36327:	jbe    3637c <meshopt_peers::run+0x17cc>
   36329:	mov    %rdx,%rax
   3632c:	mov    %rcx,%rsi
   3632f:	sub    %rcx,%rax
   36332:	cmp    %rax,%r15
   36335:	ja     36c49 <meshopt_peers::run+0x2099>
   3633b:	mov    0xe8(%rsp),%rbp
   36343:	mov    %rsi,%rbx
   36346:	lea    0x0(,%rsi,4),%rdi
   3634e:	add    %rbp,%rdi
   36351:	lea    0x0(,%r15,4),%rdx
   36359:	xor    %esi,%esi
   3635b:	call   *0xf2dc7(%rip)        # 129128 <memset@GLIBC_2.2.5>
   36361:	cmpq   $0xffffffffffffffff,0x80(%rsp)
   3636a:	je     36ffd <meshopt_peers::run+0x244d>
   36370:	add    %r15,%rbx
   36373:	mov    %rbp,%rdx
   36376:	shr    $0x8,%rdx
   3637a:	jmp    363a2 <meshopt_peers::run+0x17f2>
   3637c:	movzbl 0xef(%rsp),%eax
   36384:	shl    $0x10,%eax
   36387:	movzwl 0xed(%rsp),%ecx
   3638f:	or     %eax,%ecx
   36391:	shl    $0x20,%rcx
   36395:	mov    0xe9(%rsp),%edx
   3639c:	or     %rcx,%rdx
   3639f:	mov    %rsi,%rbx
   363a2:	movzbl %bpl,%eax
   363a6:	shl    $0x8,%rdx
   363aa:	or     %rax,%rdx
   363ad:	cmp    $0x2,%r14
   363b1:	mov    %rdx,0xc0(%rsp)
   363b9:	mov    %rbx,0x140(%rsp)
   363c1:	jb     36ad8 <meshopt_peers::run+0x1f28>
   363c7:	mov    0xb0(%rsp),%rax
   363cf:	add    $0x10,%rax
   363d3:	mov    %rax,0x280(%rsp)
   363db:	xor    %ebp,%ebp
   363dd:	xor    %r12d,%r12d
   363e0:	mov    0xd0(%rsp),%rax
   363e8:	mov    %rbp,0xb8(%rsp)
   363f0:	add    $0x4,%rax
   363f4:	mov    -0x4(%rax),%edi
   363f7:	mov    %rax,0x1b8(%rsp)
   363ff:	mov    (%rax),%r15d
   36402:	cmp    %edi,%r15d
   36405:	mov    0xd8(%rsp),%rdx
   3640d:	jb     3719f <meshopt_peers::run+0x25ef>
   36413:	cmp    %r15,%rdx
   36416:	jb     3719f <meshopt_peers::run+0x25ef>
   3641c:	sub    %rdi,%r15
   3641f:	lea    (%rdi,%rdi,2),%r13
   36423:	shl    $0x4,%r13
   36427:	add    0x38(%rsp),%r13
   3642c:	movq   $0xffffffffffffffff,0xe0(%rsp)
   36438:	lea    0x350(%rsp),%rdi
   36440:	mov    %r13,%rsi
   36443:	mov    %r15,%rdx
   36446:	mov    0x28(%rsp),%rcx
   3644b:	mov    0x18(%rsp),%r8
   36450:	lea    0x2c0(%rsp),%r9
   36458:	lea    0x738(%rsp),%rax
   36460:	push   %rax
   36461:	lea    0x1d8(%rsp),%rax
   36469:	push   %rax
   3646a:	push   0x30(%rsp)
   3646e:	push   0xa8(%rsp)
   36475:	call   *0xf30ad(%rip)        # 129528 <_DYNAMIC+0x628>
   3647b:	add    $0x20,%rsp
   3647f:	mov    0x350(%rsp),%rax
   36487:	movzbl 0x358(%rsp),%ebp
   3648f:	mov    %rax,0x8(%rsp)
   36494:	cmp    $0xffffffffffffffff,%rax
   36498:	je     36f8e <meshopt_peers::run+0x23de>
   3649e:	movzbl %bpl,%eax
   364a2:	movzbl 0x35f(%rsp),%ecx
   364aa:	shl    $0x10,%ecx
   364ad:	movzwl 0x35d(%rsp),%edx
   364b5:	or     %ecx,%edx
   364b7:	mov    0x359(%rsp),%ebx
   364be:	mov    0x368(%rsp),%rcx
   364c6:	mov    %rcx,0x60(%rsp)
   364cb:	mov    0x370(%rsp),%rcx
   364d3:	mov    %rcx,0xa8(%rsp)
   364db:	mov    0x380(%rsp),%rcx
   364e3:	mov    %rcx,0x30(%rsp)
   364e8:	mov    0x388(%rsp),%rcx
   364f0:	mov    %rcx,0x58(%rsp)
   364f5:	mov    0x390(%rsp),%rcx
   364fd:	mov    %rcx,0x78(%rsp)
   36502:	lea    0x359(%rsp),%rcx
   3650a:	movups 0x3f(%rcx),%xmm0
   3650e:	movaps %xmm0,0x260(%rsp)
   36516:	mov    0x4f(%rcx),%ecx
   36519:	mov    %ecx,0x270(%rsp)
   36520:	shl    $0x28,%rdx
   36524:	shl    $0x8,%rbx
   36528:	or     %rdx,%rbx
   3652b:	or     %rax,%rbx
   3652e:	testb  $0x1,0x3ac(%rsp)
   36536:	je     366a7 <meshopt_peers::run+0x1af7>
   3653c:	mov    %r13,%rdi
   3653f:	mov    %r15,%rsi
   36542:	mov    0x28(%rsp),%rdx
   36547:	mov    0x18(%rsp),%rcx
   3654c:	mov    0x50(%rsp),%r8
   36551:	mov    0x20(%rsp),%r9
   36556:	lea    0x738(%rsp),%rax
   3655e:	push   %rax
   3655f:	lea    0xa4(%rsp),%rax
   36567:	push   %rax
   36568:	lea    0x168(%rsp),%rax
   36570:	push   %rax
   36571:	push   0x160(%rsp)
   36578:	lea    0x280(%rsp),%rax
   36580:	push   %rax
   36581:	push   $0x0
   36583:	call   33e20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36588:	add    $0x30,%rsp
   3658c:	mov    %rax,%rbp
   3658f:	test   $0x1,%bpl
   36593:	jne    37053 <meshopt_peers::run+0x24a3>
   36599:	mov    0x750(%rsp),%rcx
   365a1:	mov    0x8(%rsp),%rdx
   365a6:	lea    0x0(,%rdx,4),%rax
   365ae:	shr    $0x3e,%rdx
   365b2:	sete   %dl
   365b5:	test   %rax,%rax
   365b8:	setns  %sil
   365bc:	and    %dl,%sil
   365bf:	sub    %rax,%rcx
   365c2:	setae  %al
   365c5:	test   %al,%sil
   365c8:	je     36dd4 <meshopt_peers::run+0x2224>
   365ce:	mov    %rcx,0x750(%rsp)
   365d6:	cmpq   $0x0,0x8(%rsp)
   365dc:	je     365ef <meshopt_peers::run+0x1a3f>
   365de:	mov    %rbx,%rdi
   365e1:	call   *0xf2b21(%rip)        # 129108 <free@GLIBC_2.2.5>
   365e7:	mov    0x750(%rsp),%rcx
   365ef:	mov    0x60(%rsp),%rax
   365f4:	mov    $0x14,%edx
   365f9:	mul    %rdx
   365fc:	jo     36f62 <meshopt_peers::run+0x23b2>
   36602:	test   %rax,%rax
   36605:	js     36f62 <meshopt_peers::run+0x23b2>
   3660b:	sub    %rax,%rcx
   3660e:	jb     36f62 <meshopt_peers::run+0x23b2>
   36614:	mov    %rcx,0x750(%rsp)
   3661c:	cmpq   $0x0,0x60(%rsp)
   36622:	je     3663a <meshopt_peers::run+0x1a8a>
   36624:	mov    0xa8(%rsp),%rdi
   3662c:	call   *0xf2ad6(%rip)        # 129108 <free@GLIBC_2.2.5>
   36632:	mov    0x750(%rsp),%rcx
   3663a:	movabs $0x3fffffffffffffff,%rax
   36644:	cmp    %rax,0x30(%rsp)
   36649:	ja     37018 <meshopt_peers::run+0x2468>
   3664f:	mov    0x30(%rsp),%rax
   36654:	lea    0x0(,%rax,4),%rax
   3665c:	test   %rax,%rax
   3665f:	js     37018 <meshopt_peers::run+0x2468>
   36665:	sub    %rax,%rcx
   36668:	jb     37018 <meshopt_peers::run+0x2468>
   3666e:	mov    %rcx,0x750(%rsp)
   36676:	cmpq   $0x0,0x30(%rsp)
   3667c:	je     36689 <meshopt_peers::run+0x1ad9>
   3667e:	mov    0x58(%rsp),%rdi
   36683:	call   *0xf2a7f(%rip)        # 129108 <free@GLIBC_2.2.5>
   36689:	dec    %r14
   3668c:	mov    0x1b8(%rsp),%rax
   36694:	add    $0x4,%rax
   36698:	cmp    $0x1,%r14
   3669c:	ja     363f4 <meshopt_peers::run+0x1844>
   366a2:	jmp    36c81 <meshopt_peers::run+0x20d1>
   366a7:	mov    %r13,%rdi
   366aa:	mov    %r15,%rsi
   366ad:	mov    0x28(%rsp),%rdx
   366b2:	mov    0x18(%rsp),%rcx
   366b7:	mov    0x50(%rsp),%r8
   366bc:	mov    0x20(%rsp),%r9
   366c1:	lea    0x738(%rsp),%rax
   366c9:	push   %rax
   366ca:	lea    0xa4(%rsp),%rax
   366d2:	push   %rax
   366d3:	lea    0x168(%rsp),%rax
   366db:	push   %rax
   366dc:	push   0x160(%rsp)
   366e3:	lea    0x280(%rsp),%rax
   366eb:	push   %rax
   366ec:	push   $0x0
   366ee:	call   33e20 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   366f3:	add    $0x30,%rsp
   366f7:	mov    %rax,%rbp
   366fa:	test   $0x1,%bpl
   366fe:	jne    37053 <meshopt_peers::run+0x24a3>
   36704:	shr    $0x20,%rbp
   36708:	movb   $0x2,0x170(%rsp)
   36710:	mov    0x50(%rsp),%rax
   36715:	mov    %rax,0x178(%rsp)
   3671d:	mov    0x20(%rsp),%rax
   36722:	mov    %rax,0x180(%rsp)
   3672a:	movq   $0x0,0x1a8(%rsp)
   36736:	movups 0x210(%rsp),%xmm0
   3673e:	movaps %xmm0,0x390(%rsp)
   36746:	movups 0x1d0(%rsp),%xmm0
   3674e:	movups 0x1e0(%rsp),%xmm1
   36756:	movups 0x1f0(%rsp),%xmm2
   3675e:	movups 0x200(%rsp),%xmm3
   36766:	movaps %xmm3,0x380(%rsp)
   3676e:	movaps %xmm2,0x370(%rsp)
   36776:	movaps %xmm1,0x360(%rsp)
   3677e:	movaps %xmm0,0x350(%rsp)
   36786:	lea    0x230(%rsp),%rdi
   3678e:	mov    0x58(%rsp),%rsi
   36793:	mov    0x78(%rsp),%rdx
   36798:	lea    0x170(%rsp),%rcx
   367a0:	lea    0x350(%rsp),%r8
   367a8:	mov    %ebp,%r9d
   367ab:	lea    0x738(%rsp),%rax
   367b3:	push   %rax
   367b4:	push   0x1c8(%rsp)
   367bb:	call   *0xf2d47(%rip)        # 129508 <_DYNAMIC+0x608>
   367c1:	add    $0x10,%rsp
   367c5:	mov    0x230(%rsp),%r15
   367cd:	movzbl 0x238(%rsp),%ebp
   367d5:	cmp    $0xffffffffffffffff,%r15
   367d9:	je     37056 <meshopt_peers::run+0x24a6>
   367df:	movzbl %bpl,%ecx
   367e3:	movzbl 0x23f(%rsp),%eax
   367eb:	shl    $0x10,%eax
   367ee:	movzwl 0x23d(%rsp),%esi
   367f6:	or     %eax,%esi
   367f8:	mov    0x239(%rsp),%r13d
   36800:	mov    0x240(%rsp),%rax
   36808:	mov    0x248(%rsp),%rdx
   36810:	mov    %rdx,0xa0(%rsp)
   36818:	mov    0x250(%rsp),%rdx
   36820:	mov    %rdx,0x78(%rsp)
   36825:	mov    0x258(%rsp),%rdx
   3682d:	shl    $0x28,%rsi
   36831:	shl    $0x8,%r13
   36835:	or     %rsi,%r13
   36838:	or     %rcx,%r13
   3683b:	test   %rax,%rax
   3683e:	je     368e7 <meshopt_peers::run+0x1d37>
   36844:	lea    (%rax,%rax,2),%rax
   36848:	shl    $0x4,%rax
   3684c:	add    %r13,%rax
   3684f:	lea    (%r12,%r12,2),%rcx
   36853:	shl    $0x4,%rcx
   36857:	add    0x280(%rsp),%rcx
   3685f:	xor    %esi,%esi
   36861:	mov    0x0(%r13,%rsi,1),%rdi
   36866:	mov    0x8(%r13,%rsi,1),%r8
   3686b:	movups 0x10(%r13,%rsi,1),%xmm0
   36871:	movaps %xmm0,0x6c0(%rsp)
   36879:	mov    0x270(%rsp),%r9d
   36881:	lea    0x6cc(%rsp),%r10
   36889:	mov    %r9d,0x10(%r10)
   3688d:	movaps 0x260(%rsp),%xmm0
   36895:	movups %xmm0,(%r10)
   36899:	cmp    0x48(%rsp),%r12
   3689e:	jae    37297 <meshopt_peers::run+0x26e7>
   368a4:	lea    (%rsi,%r13,1),%r9
   368a8:	add    0xb8(%rsp),%r8
   368b0:	mov    %rdi,-0x10(%rcx,%rsi,1)
   368b5:	mov    %r8,-0x8(%rcx,%rsi,1)
   368ba:	movaps 0x6c0(%rsp),%xmm0
   368c2:	movaps 0x6d0(%rsp),%xmm1
   368ca:	movups %xmm0,(%rcx,%rsi,1)
   368ce:	movups %xmm1,0x10(%rcx,%rsi,1)
   368d3:	inc    %r12
   368d6:	add    $0x30,%rsi
   368da:	add    $0x30,%r9
   368de:	cmp    %rax,%r9
   368e1:	jne    36861 <meshopt_peers::run+0x1cb1>
   368e7:	mov    %rdx,%rbp
   368ea:	add    0xb8(%rsp),%rbp
   368f2:	jb     3720c <meshopt_peers::run+0x265c>
   368f8:	cmp    0x140(%rsp),%rbp
   36900:	ja     3720c <meshopt_peers::run+0x265c>
   36906:	mov    0xc0(%rsp),%rax
   3690e:	mov    0xb8(%rsp),%rcx
   36916:	lea    (%rax,%rcx,4),%rdi
   3691a:	shl    $0x2,%rdx
   3691e:	mov    0x78(%rsp),%rsi
   36923:	call   *0xf27ef(%rip)        # 129118 <memcpy@GLIBC_2.14>
   36929:	mov    0x750(%rsp),%rcx
   36931:	mov    %r15,%rax
   36934:	mov    $0x30,%edx
   36939:	mul    %rdx
   3693c:	seto   %dl
   3693f:	not    %dl
   36941:	test   %rax,%rax
   36944:	setns  %sil
   36948:	and    %dl,%sil
   3694b:	sub    %rax,%rcx
   3694e:	setae  %al
   36951:	test   %al,%sil
   36954:	je     370b3 <meshopt_peers::run+0x2503>
   3695a:	mov    %rcx,0x750(%rsp)
   36962:	test   %r15,%r15
   36965:	je     36978 <meshopt_peers::run+0x1dc8>
   36967:	mov    %r13,%rdi
   3696a:	call   *0xf2798(%rip)        # 129108 <free@GLIBC_2.2.5>
   36970:	mov    0x750(%rsp),%rcx
   36978:	mov    0xa0(%rsp),%rax
   36980:	shr    $0x3e,%rax
   36984:	jne    370c1 <meshopt_peers::run+0x2511>
   3698a:	mov    0xa0(%rsp),%rax
   36992:	lea    0x0(,%rax,4),%rax
   3699a:	test   %rax,%rax
   3699d:	js     370c1 <meshopt_peers::run+0x2511>
   369a3:	sub    %rax,%rcx
   369a6:	jb     370c1 <meshopt_peers::run+0x2511>
   369ac:	mov    %rcx,0x750(%rsp)
   369b4:	cmpq   $0x0,0xa0(%rsp)
   369bd:	je     369d2 <meshopt_peers::run+0x1e22>
   369bf:	mov    0x78(%rsp),%rdi
   369c4:	call   *0xf273e(%rip)        # 129108 <free@GLIBC_2.2.5>
   369ca:	mov    0x750(%rsp),%rcx
   369d2:	movabs $0x3fffffffffffffff,%rax
   369dc:	cmp    %rax,0x8(%rsp)
   369e1:	ja     37148 <meshopt_peers::run+0x2598>
   369e7:	mov    0x8(%rsp),%rax
   369ec:	lea    0x0(,%rax,4),%rax
   369f4:	test   %rax,%rax
   369f7:	js     37148 <meshopt_peers::run+0x2598>
   369fd:	sub    %rax,%rcx
   36a00:	jb     37148 <meshopt_peers::run+0x2598>
   36a06:	mov    %rcx,0x750(%rsp)
   36a0e:	cmpq   $0x0,0x8(%rsp)
   36a14:	je     36a27 <meshopt_peers::run+0x1e77>
   36a16:	mov    %rbx,%rdi
   36a19:	call   *0xf26e9(%rip)        # 129108 <free@GLIBC_2.2.5>
   36a1f:	mov    0x750(%rsp),%rcx
   36a27:	mov    0x60(%rsp),%rax
   36a2c:	mov    $0x14,%edx
   36a31:	mul    %rdx
   36a34:	jo     371c7 <meshopt_peers::run+0x2617>
   36a3a:	test   %rax,%rax
   36a3d:	js     371c7 <meshopt_peers::run+0x2617>
   36a43:	sub    %rax,%rcx
   36a46:	jb     371c7 <meshopt_peers::run+0x2617>
   36a4c:	mov    %rcx,0x750(%rsp)
   36a54:	cmpq   $0x0,0x60(%rsp)
   36a5a:	je     36a72 <meshopt_peers::run+0x1ec2>
   36a5c:	mov    0xa8(%rsp),%rdi
   36a64:	call   *0xf269e(%rip)        # 129108 <free@GLIBC_2.2.5>
   36a6a:	mov    0x750(%rsp),%rcx
   36a72:	movabs $0x3fffffffffffffff,%rax
   36a7c:	cmp    %rax,0x30(%rsp)
   36a81:	ja     371f8 <meshopt_peers::run+0x2648>
   36a87:	mov    0x30(%rsp),%rax
   36a8c:	lea    0x0(,%rax,4),%rax
   36a94:	test   %rax,%rax
   36a97:	js     371f8 <meshopt_peers::run+0x2648>
   36a9d:	sub    %rax,%rcx
   36aa0:	jb     371f8 <meshopt_peers::run+0x2648>
   36aa6:	mov    %rcx,0x750(%rsp)
   36aae:	cmpq   $0x0,0x30(%rsp)
   36ab4:	je     36ac1 <meshopt_peers::run+0x1f11>
   36ab6:	mov    0x58(%rsp),%rdi
   36abb:	call   *0xf2647(%rip)        # 129108 <free@GLIBC_2.2.5>
   36ac1:	dec    %r14
   36ac4:	cmp    $0x1,%r14
   36ac8:	mov    0x1b8(%rsp),%rax
   36ad0:	ja     363e8 <meshopt_peers::run+0x1838>
   36ad6:	jmp    36add <meshopt_peers::run+0x1f2d>
   36ad8:	xor    %r12d,%r12d
   36adb:	xor    %ebp,%ebp
   36add:	mov    %rbp,0x18(%rsp)
   36ae2:	lea    0x738(%rsp),%rbx
   36aea:	mov    %rbx,%rdi
   36aed:	mov    0x40(%rsp),%rsi
   36af2:	mov    0xd0(%rsp),%rdx
   36afa:	call   336d0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36aff:	mov    %eax,%ebp
   36b01:	mov    $0x1,%al
   36b03:	mov    %eax,0x8(%rsp)
   36b07:	cmp    $0xff,%bpl
   36b0b:	setne  %r13b
   36b0f:	jne    3704b <meshopt_peers::run+0x249b>
   36b15:	mov    0x48(%rsp),%r15
   36b1a:	cmp    %r15,%r12
   36b1d:	cmovb  %r12,%r15
   36b21:	mov    0x140(%rsp),%r14
   36b29:	mov    0x18(%rsp),%rax
   36b2e:	cmp    %r14,%rax
   36b31:	cmovb  %rax,%r14
   36b35:	mov    %rbx,%rdi
   36b38:	mov    0x10(%rsp),%rsi
   36b3d:	mov    0x38(%rsp),%rdx
   36b42:	call   33680 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36b47:	cmp    $0xff,%al
   36b49:	jne    3706a <meshopt_peers::run+0x24ba>
   36b4f:	mov    %rbx,%rdi
   36b52:	mov    0x70(%rsp),%rsi
   36b57:	mov    0x28(%rsp),%rdx
   36b5c:	call   336d0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36b61:	cmp    $0xff,%al
   36b63:	jne    37074 <meshopt_peers::run+0x24c4>
   36b69:	mov    0x148(%rsp),%r13
   36b71:	inc    %r13d
   36b74:	mov    0x68(%rsp),%rax
   36b79:	mov    %rax,0x10(%rsp)
   36b7e:	mov    %r15,%rdx
   36b81:	mov    0xb0(%rsp),%rax
   36b89:	mov    %rax,0x38(%rsp)
   36b8e:	mov    %r14,%r8
   36b91:	mov    0xc0(%rsp),%rax
   36b99:	mov    %rax,0x28(%rsp)
   36b9e:	mov    0x80(%rsp),%rax
   36ba6:	mov    %rax,0x70(%rsp)
   36bab:	cmp    $0x1,%r15
   36baf:	mov    0x20(%rsp),%rbx
   36bb4:	lea    0x170(%rsp),%rdi
   36bbc:	mov    0xf2955(%rip),%r10        # 129518 <_DYNAMIC+0x618>
   36bc3:	ja     35f43 <meshopt_peers::run+0x1393>
   36bc9:	jmp    37086 <meshopt_peers::run+0x24d6>
   36bce:	mov    $0x8,%edx
   36bd3:	mov    $0x8,%r8d
   36bd9:	mov    $0x30,%r9d
   36bdf:	lea    0x350(%rsp),%rdi
   36be7:	xor    %esi,%esi
   36be9:	mov    0x48(%rsp),%rcx
   36bee:	call   3b4c0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36bf3:	cmpl   $0x1,0x350(%rsp)
   36bfb:	je     37106 <meshopt_peers::run+0x2556>
   36c01:	mov    0x358(%rsp),%r15
   36c09:	mov    0x48(%rsp),%rax
   36c0e:	mov    0x20(%rsp),%rbx
   36c13:	mov    0x18(%rsp),%r8
   36c18:	jmp    3616a <meshopt_peers::run+0x15ba>
   36c1d:	mov    $0x8,%ecx
   36c22:	mov    $0x30,%r8d
   36c28:	lea    0xe0(%rsp),%rdi
   36c30:	xor    %esi,%esi
   36c32:	mov    0x48(%rsp),%rdx
   36c37:	call   34b10 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36c3c:	mov    0xf0(%rsp),%r12
   36c44:	jmp    36224 <meshopt_peers::run+0x1674>
   36c49:	mov    $0x4,%ecx
   36c4e:	mov    $0x4,%r8d
   36c54:	lea    0xe0(%rsp),%rdi
   36c5c:	mov    %r15,%rdx
   36c5f:	call   34b10 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36c64:	mov    0xe0(%rsp),%rax
   36c6c:	mov    %rax,0x80(%rsp)
   36c74:	mov    0xf0(%rsp),%rsi
   36c7c:	jmp    3633b <meshopt_peers::run+0x178b>
   36c81:	mov    0xb8(%rsp),%rbp
   36c89:	jmp    36add <meshopt_peers::run+0x1f2d>
   36c8e:	mov    $0x1,%al
   36c90:	mov    %eax,0x8(%rsp)
   36c94:	jmp    36d64 <meshopt_peers::run+0x21b4>
   36c99:	lea    0x738(%rsp),%rdi
   36ca1:	mov    %r12,%rsi
   36ca4:	mov    0x38(%rsp),%rdx
   36ca9:	call   33680 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36cae:	cmp    $0xff,%al
   36cb0:	jne    3722e <meshopt_peers::run+0x267e>
   36cb6:	lea    0x738(%rsp),%rdi
   36cbe:	mov    0x80(%rsp),%rsi
   36cc6:	mov    0x28(%rsp),%rdx
   36ccb:	call   336d0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36cd0:	mov    %eax,%ebp
   36cd2:	jmp    357ae <meshopt_peers::run+0xbfe>
   36cd7:	movzbl 0x2c8(%rsp),%eax
   36cdf:	mov    %al,0x738(%rsp)
   36ce6:	lea    -0x2673a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36ced:	lea    0xeb34c(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36cf4:	lea    0xeb0d5(%rip),%r8        # 121dd0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36cfb:	lea    0x738(%rsp),%rdx
   36d03:	mov    $0x2b,%esi
   36d08:	call   *0xf24c2(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   36d0e:	lea    0x290(%rsp),%rax
   36d16:	mov    %rax,0x738(%rsp)
   36d1e:	lea    0x55fb(%rip),%rax        # 3c320 <<&str as core::fmt::Display>::fmt>
   36d25:	mov    %rax,0x740(%rsp)
   36d2d:	lea    -0x2a639(%rip),%rdi        # c6fb <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36d34:	lea    0xeaf8d(%rip),%rdx        # 121cc8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x120>
   36d3b:	lea    0x738(%rsp),%rsi
   36d43:	call   *0xf2457(%rip)        # 1291a0 <_DYNAMIC+0x2a0>
   36d49:	cmpq   $0x0,0x68(%rsp)
   36d4f:	mov    0x10(%rsp),%r12
   36d54:	je     36d5f <meshopt_peers::run+0x21af>
   36d56:	mov    %r15,%rdi
   36d59:	call   *0xf23a9(%rip)        # 129108 <free@GLIBC_2.2.5>
   36d5f:	mov    0x40(%rsp),%r15
   36d64:	mov    $0x1,%r13b
   36d67:	test   %r15,%r15
   36d6a:	je     36d7a <meshopt_peers::run+0x21ca>
   36d6c:	mov    0xd0(%rsp),%rdi
   36d74:	call   *0xf238e(%rip)        # 129108 <free@GLIBC_2.2.5>
   36d7a:	mov    $0x1,%r14b
   36d7d:	mov    $0x1,%r15b
   36d80:	cmpb   $0x0,0x8(%rsp)
   36d85:	mov    0x70(%rsp),%rax
   36d8a:	jne    36ea6 <meshopt_peers::run+0x22f6>
   36d90:	jmp    36eb6 <meshopt_peers::run+0x2306>
   36d95:	movzbl 0x740(%rsp),%eax
   36d9d:	mov    %al,0x350(%rsp)
   36da4:	lea    -0x267f8(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36dab:	lea    0xeb28e(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36db2:	lea    0xeafb7(%rip),%r8        # 121d70 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   36db9:	lea    0x350(%rsp),%rdx
   36dc1:	mov    $0x2b,%esi
   36dc6:	call   *0xf2404(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   36dcc:	mov    $0x8,%bpl
   36dcf:	jmp    35795 <meshopt_peers::run+0xbe5>
   36dd4:	mov    $0x8,%bpl
   36dd7:	cmpq   $0x0,0x8(%rsp)
   36ddd:	je     36f23 <meshopt_peers::run+0x2373>
   36de3:	mov    %rbx,%rdi
   36de6:	call   *0xf231c(%rip)        # 129108 <free@GLIBC_2.2.5>
   36dec:	xor    %r15d,%r15d
   36def:	cmpq   $0x0,0x30(%rsp)
   36df5:	jne    36f2e <meshopt_peers::run+0x237e>
   36dfb:	jmp    36f39 <meshopt_peers::run+0x2389>
   36e00:	lea    0xeaea9(%rip),%rcx        # 121cb0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x108>
   36e07:	mov    %r14,%rdx
   36e0a:	call   *0xf2460(%rip)        # 129270 <_DYNAMIC+0x370>
   36e10:	jmp    372ac <meshopt_peers::run+0x26fc>
   36e15:	movzbl 0x740(%rsp),%eax
   36e1d:	mov    %al,0x350(%rsp)
   36e24:	lea    -0x26878(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36e2b:	lea    0xeb20e(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36e32:	lea    0xeaf07(%rip),%r8        # 121d40 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36e39:	lea    0x350(%rsp),%rdx
   36e41:	mov    $0x2b,%esi
   36e46:	call   *0xf2384(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   36e4c:	movzbl 0x740(%rsp),%eax
   36e54:	mov    %al,0x350(%rsp)
   36e5b:	lea    -0x268af(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36e62:	lea    0xeb1d7(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36e69:	lea    0xeae88(%rip),%r8        # 121cf8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36e70:	lea    0x350(%rsp),%rdx
   36e78:	mov    $0x2b,%esi
   36e7d:	call   *0xf234d(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   36e83:	mov    $0x8,%bpl
   36e86:	lea    0x350(%rsp),%rdi
   36e8e:	call   33d90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   36e93:	mov    $0x1,%r14b
   36e96:	mov    $0x1,%r15b
   36e99:	mov    $0x1,%r13b
   36e9c:	mov    0x10(%rsp),%r12
   36ea1:	mov    0x70(%rsp),%rax
   36ea6:	test   %rax,%rax
   36ea9:	je     36eb6 <meshopt_peers::run+0x2306>
   36eab:	mov    0x28(%rsp),%rdi
   36eb0:	call   *0xf2252(%rip)        # 129108 <free@GLIBC_2.2.5>
   36eb6:	test   %r13b,%r13b
   36eb9:	je     36ecb <meshopt_peers::run+0x231b>
   36ebb:	test   %r12,%r12
   36ebe:	je     36ecb <meshopt_peers::run+0x231b>
   36ec0:	mov    0x38(%rsp),%rdi
   36ec5:	call   *0xf223d(%rip)        # 129108 <free@GLIBC_2.2.5>
   36ecb:	test   %r15b,%r15b
   36ece:	jne    3577d <meshopt_peers::run+0xbcd>
   36ed4:	jmp    35790 <meshopt_peers::run+0xbe0>
   36ed9:	mov    $0x1,%al
   36edb:	mov    %eax,0x8(%rsp)
   36edf:	mov    $0x1,%r13b
   36ee2:	mov    0x10(%rsp),%r12
   36ee7:	jmp    36d67 <meshopt_peers::run+0x21b7>
   36eec:	movzbl 0x740(%rsp),%eax
   36ef4:	mov    %al,0x350(%rsp)
   36efb:	lea    -0x2694f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f02:	lea    0xeb137(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f09:	lea    0xeae78(%rip),%r8        # 121d88 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   36f10:	lea    0x350(%rsp),%rdx
   36f18:	mov    $0x2b,%esi
   36f1d:	call   *0xf22ad(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   36f23:	xor    %r15d,%r15d
   36f26:	cmpq   $0x0,0x30(%rsp)
   36f2c:	je     36f39 <meshopt_peers::run+0x2389>
   36f2e:	mov    0x58(%rsp),%rdi
   36f33:	call   *0xf21cf(%rip)        # 129108 <free@GLIBC_2.2.5>
   36f39:	cmpq   $0x0,0x60(%rsp)
   36f3f:	je     36f4f <meshopt_peers::run+0x239f>
   36f41:	mov    0xa8(%rsp),%rdi
   36f49:	call   *0xf21b9(%rip)        # 129108 <free@GLIBC_2.2.5>
   36f4f:	cmpq   $0x0,0x8(%rsp)
   36f55:	setne  %al
   36f58:	test   %r15b,%al
   36f5b:	je     36f8e <meshopt_peers::run+0x23de>
   36f5d:	mov    %rbx,%rdi
   36f60:	jmp    36f88 <meshopt_peers::run+0x23d8>
   36f62:	cmpq   $0x0,0x60(%rsp)
   36f68:	je     36f78 <meshopt_peers::run+0x23c8>
   36f6a:	mov    0xa8(%rsp),%rdi
   36f72:	call   *0xf2190(%rip)        # 129108 <free@GLIBC_2.2.5>
   36f78:	mov    $0x8,%bpl
   36f7b:	cmpq   $0x0,0x30(%rsp)
   36f81:	je     36f8e <meshopt_peers::run+0x23de>
   36f83:	mov    0x58(%rsp),%rdi
   36f88:	call   *0xf217a(%rip)        # 129108 <free@GLIBC_2.2.5>
   36f8e:	lea    0xe0(%rsp),%rdi
   36f96:	call   337c0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   36f9b:	mov    $0x1,%r15b
   36f9e:	mov    $0x1,%al
   36fa0:	mov    %eax,0x8(%rsp)
   36fa4:	mov    $0x1,%r13b
   36fa7:	cmpq   $0x0,0x80(%rsp)
   36fb0:	je     36fc0 <meshopt_peers::run+0x2410>
   36fb2:	mov    0xc0(%rsp),%rdi
   36fba:	call   *0xf2148(%rip)        # 129108 <free@GLIBC_2.2.5>
   36fc0:	cmpq   $0x0,0x68(%rsp)
   36fc6:	je     36fd6 <meshopt_peers::run+0x2426>
   36fc8:	mov    0xb0(%rsp),%rdi
   36fd0:	call   *0xf2132(%rip)        # 129108 <free@GLIBC_2.2.5>
   36fd6:	test   %r15b,%r15b
   36fd9:	mov    0x20(%rsp),%rbx
   36fde:	mov    0x10(%rsp),%r12
   36fe3:	mov    0x40(%rsp),%r15
   36fe8:	jne    36d67 <meshopt_peers::run+0x21b7>
   36fee:	jmp    36d7a <meshopt_peers::run+0x21ca>
   36ff3:	mov    0x10(%rsp),%r12
   36ff8:	jmp    36d64 <meshopt_peers::run+0x21b4>
   36ffd:	cmpq   $0x0,0x68(%rsp)
   37003:	je     3702c <meshopt_peers::run+0x247c>
   37005:	mov    0xb0(%rsp),%rdi
   3700d:	call   *0xf20f5(%rip)        # 129108 <free@GLIBC_2.2.5>
   37013:	mov    $0x1,%r13b
   37016:	jmp    37037 <meshopt_peers::run+0x2487>
   37018:	mov    $0x8,%bpl
   3701b:	cmpq   $0x0,0x30(%rsp)
   37021:	je     36f8e <meshopt_peers::run+0x23de>
   37027:	jmp    36f83 <meshopt_peers::run+0x23d3>
   3702c:	mov    $0x1,%r13b
   3702f:	jmp    37037 <meshopt_peers::run+0x2487>
   37031:	mov    $0x1,%r13b
   37034:	mov    %r15d,%ebp
   37037:	mov    0x20(%rsp),%rbx
   3703c:	mov    0x10(%rsp),%r12
   37041:	mov    0x40(%rsp),%r15
   37046:	jmp    36d67 <meshopt_peers::run+0x21b7>
   3704b:	xor    %r15d,%r15d
   3704e:	jmp    36fa7 <meshopt_peers::run+0x23f7>
   37053:	shr    $0x8,%ebp
   37056:	mov    $0x1,%r15b
   37059:	cmpq   $0x0,0x30(%rsp)
   3705f:	jne    36f2e <meshopt_peers::run+0x237e>
   37065:	jmp    36f39 <meshopt_peers::run+0x2389>
   3706a:	mov    %eax,%ebp
   3706c:	xor    %r15d,%r15d
   3706f:	jmp    36fa7 <meshopt_peers::run+0x23f7>
   37074:	mov    %eax,%ebp
   37076:	xor    %r15d,%r15d
   37079:	movl   $0x0,0x8(%rsp)
   37081:	jmp    36fa7 <meshopt_peers::run+0x23f7>
   37086:	mov    0xb0(%rsp),%rax
   3708e:	mov    %rax,0x38(%rsp)
   37093:	mov    0xc0(%rsp),%rax
   3709b:	mov    %rax,0x28(%rsp)
   370a0:	test   %r15,%r15
   370a3:	mov    0x68(%rsp),%r12
   370a8:	jne    35e45 <meshopt_peers::run+0x1295>
   370ae:	jmp    35d82 <meshopt_peers::run+0x11d2>
   370b3:	test   %r15,%r15
   370b6:	je     370c1 <meshopt_peers::run+0x2511>
   370b8:	mov    %r13,%rdi
   370bb:	call   *0xf2047(%rip)        # 129108 <free@GLIBC_2.2.5>
   370c1:	mov    $0x1,%r13b
   370c4:	mov    $0x1,%r15b
   370c7:	cmpq   $0x0,0xa0(%rsp)
   370d0:	je     370df <meshopt_peers::run+0x252f>
   370d2:	mov    0x78(%rsp),%rdi
   370d7:	call   *0xf202b(%rip)        # 129108 <free@GLIBC_2.2.5>
   370dd:	jmp    370e2 <meshopt_peers::run+0x2532>
   370df:	mov    $0x1,%r15b
   370e2:	cmpq   $0x0,0x30(%rsp)
   370e8:	je     370f5 <meshopt_peers::run+0x2545>
   370ea:	mov    0x58(%rsp),%rdi
   370ef:	call   *0xf2013(%rip)        # 129108 <free@GLIBC_2.2.5>
   370f5:	mov    $0x8,%bpl
   370f8:	test   %r13b,%r13b
   370fb:	jne    36f39 <meshopt_peers::run+0x2389>
   37101:	jmp    36f4f <meshopt_peers::run+0x239f>
   37106:	mov    $0xa,%bpl
   37109:	mov    0x20(%rsp),%rbx
   3710e:	mov    0x10(%rsp),%r12
   37113:	jmp    36d5f <meshopt_peers::run+0x21af>
   37118:	movb   $0x2,0x738(%rsp)
   37120:	lea    -0x26b74(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37127:	lea    0xeaf12(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3712e:	lea    0xeac83(%rip),%r8        # 121db8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   37135:	lea    0x738(%rsp),%rdx
   3713d:	mov    $0x2b,%esi
   37142:	call   *0xf2088(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   37148:	mov    $0x1,%r13b
   3714b:	xor    %r15d,%r15d
   3714e:	mov    %rbx,0x78(%rsp)
   37153:	cmpq   $0x0,0x8(%rsp)
   37159:	jne    370d2 <meshopt_peers::run+0x2522>
   3715f:	jmp    370e2 <meshopt_peers::run+0x2532>
   37161:	movb   $0x8,0x738(%rsp)
   37169:	lea    -0x26bbd(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37170:	lea    0xeaec9(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   37177:	lea    0xeabda(%rip),%r8        # 121d58 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   3717e:	lea    0x738(%rsp),%rdx
   37186:	mov    $0x2b,%esi
   3718b:	call   *0xf203f(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   37191:	mov    $0x4,%edi
   37196:	mov    %r13,%rsi
   37199:	call   *0xf1fa1(%rip)        # 129140 <_DYNAMIC+0x240>
   3719f:	lea    0xeaaaa(%rip),%rcx        # 121c50 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   371a6:	mov    %r15,%rsi
   371a9:	call   *0xf20c1(%rip)        # 129270 <_DYNAMIC+0x370>
   371af:	jmp    372ac <meshopt_peers::run+0x26fc>
   371b4:	mov    $0x4,%edi
   371b9:	mov    %r15,%rsi
   371bc:	call   *0xf1f7e(%rip)        # 129140 <_DYNAMIC+0x240>
   371c2:	jmp    372ac <meshopt_peers::run+0x26fc>
   371c7:	xor    %r13d,%r13d
   371ca:	cmpq   $0x0,0x60(%rsp)
   371d0:	je     371e7 <meshopt_peers::run+0x2637>
   371d2:	mov    0xa8(%rsp),%rax
   371da:	mov    %rax,0x78(%rsp)
   371df:	xor    %r15d,%r15d
   371e2:	jmp    370d2 <meshopt_peers::run+0x2522>
   371e7:	movq   $0x0,0x60(%rsp)
   371f0:	xor    %r15d,%r15d
   371f3:	jmp    370e2 <meshopt_peers::run+0x2532>
   371f8:	mov    $0x8,%bpl
   371fb:	cmpq   $0x0,0x30(%rsp)
   37201:	je     36f8e <meshopt_peers::run+0x23de>
   37207:	jmp    36f83 <meshopt_peers::run+0x23d3>
   3720c:	lea    0xeaa25(%rip),%rcx        # 121c38 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   37213:	mov    0xb8(%rsp),%rdi
   3721b:	mov    %rbp,%rsi
   3721e:	mov    0x140(%rsp),%rdx
   37226:	call   *0xf2044(%rip)        # 129270 <_DYNAMIC+0x370>
   3722c:	jmp    372ac <meshopt_peers::run+0x26fc>
   3722e:	mov    %eax,%ebp
   37230:	cmpq   $0x0,0x80(%rsp)
   37239:	je     37246 <meshopt_peers::run+0x2696>
   3723b:	mov    0x28(%rsp),%rdi
   37240:	call   *0xf1ec2(%rip)        # 129108 <free@GLIBC_2.2.5>
   37246:	mov    0x738(%rsp),%rax
   3724e:	mov    0x748(%rsp),%rcx
   37256:	sub    0x740(%rsp),%rcx
   3725e:	mov    %rcx,0x358(%rax)
   37265:	mov    %bpl,0x738(%rsp)
   3726d:	lea    -0x26cc1(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37274:	lea    0xeadc5(%rip),%rcx        # 122040 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3727b:	lea    0xeab66(%rip),%r8        # 121de8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   37282:	lea    0x738(%rsp),%rdx
   3728a:	mov    $0x2b,%esi
   3728f:	call   *0xf1f3b(%rip)        # 1291d0 <_DYNAMIC+0x2d0>
   37295:	jmp    372ac <meshopt_peers::run+0x26fc>
   37297:	lea    0xea982(%rip),%rdx        # 121c20 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   3729e:	mov    %r12,%rdi
   372a1:	mov    0x48(%rsp),%rsi
   372a6:	call   *0xf1ea4(%rip)        # 129150 <_DYNAMIC+0x250>
   372ac:	ud2
   372ae:	mov    %rax,%r14
   372b1:	cmpq   $0x0,0xe0(%rsp)
   372ba:	je     37453 <meshopt_peers::run+0x28a3>
   372c0:	mov    0xe8(%rsp),%rdi
   372c8:	jmp    3744d <meshopt_peers::run+0x289d>
   372cd:	mov    %rax,%r14
   372d0:	cmpq   $0x0,0xe0(%rsp)
   372d9:	je     37469 <meshopt_peers::run+0x28b9>
   372df:	mov    0xe8(%rsp),%r15
   372e7:	jmp    37316 <meshopt_peers::run+0x2766>
   372e9:	jmp    372eb <meshopt_peers::run+0x273b>
   372eb:	mov    %rax,%r14
   372ee:	cmpq   $0x0,0x30(%rsp)
   372f4:	je     37388 <meshopt_peers::run+0x27d8>
   372fa:	jmp    373d4 <meshopt_peers::run+0x2824>
   372ff:	mov    %rax,%r14
   37302:	jmp    373a1 <meshopt_peers::run+0x27f1>
   37307:	mov    %rax,%r14
   3730a:	cmpq   $0x0,0x68(%rsp)
   37310:	je     37469 <meshopt_peers::run+0x28b9>
   37316:	mov    %r15,%rdi
   37319:	jmp    37463 <meshopt_peers::run+0x28b3>
   3731e:	mov    %rax,%r14
   37321:	jmp    37453 <meshopt_peers::run+0x28a3>
   37326:	mov    %rax,%r14
   37329:	mov    %r12,0x10(%rsp)
   3732e:	mov    0x80(%rsp),%rax
   37336:	mov    %rax,0x70(%rsp)
   3733b:	cmpq   $0x0,0x70(%rsp)
   37341:	je     37479 <meshopt_peers::run+0x28c9>
   37347:	jmp    374f2 <meshopt_peers::run+0x2942>
   3734c:	mov    %rax,%r14
   3734f:	cmpq   $0x0,0x70(%rsp)
   37355:	je     37479 <meshopt_peers::run+0x28c9>
   3735b:	jmp    374f2 <meshopt_peers::run+0x2942>
   37360:	mov    %r15,0x40(%rsp)
   37365:	mov    %rax,%r14
   37368:	jmp    37469 <meshopt_peers::run+0x28b9>
   3736d:	mov    %rax,%r14
   37370:	cmpq   $0x0,0xa0(%rsp)
   37379:	jne    373b3 <meshopt_peers::run+0x2803>
   3737b:	test   %r15,%r15
   3737e:	jne    373c3 <meshopt_peers::run+0x2813>
   37380:	cmpq   $0x0,0x30(%rsp)
   37386:	jne    373d4 <meshopt_peers::run+0x2824>
   37388:	cmpq   $0x0,0x60(%rsp)
   3738e:	jne    373e7 <meshopt_peers::run+0x2837>
   37390:	cmpq   $0x0,0x8(%rsp)
   37396:	je     373a1 <meshopt_peers::run+0x27f1>
   37398:	mov    %rbx,%rdi
   3739b:	call   *0xf1d67(%rip)        # 129108 <free@GLIBC_2.2.5>
   373a1:	lea    0xe0(%rsp),%rdi
   373a9:	call   337c0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   373ae:	jmp    3743a <meshopt_peers::run+0x288a>
   373b3:	mov    0x78(%rsp),%rdi
   373b8:	call   *0xf1d4a(%rip)        # 129108 <free@GLIBC_2.2.5>
   373be:	test   %r15,%r15
   373c1:	je     37380 <meshopt_peers::run+0x27d0>
   373c3:	mov    %r13,%rdi
   373c6:	call   *0xf1d3c(%rip)        # 129108 <free@GLIBC_2.2.5>
   373cc:	cmpq   $0x0,0x30(%rsp)
   373d2:	je     37388 <meshopt_peers::run+0x27d8>
   373d4:	mov    0x58(%rsp),%rdi
   373d9:	call   *0xf1d29(%rip)        # 129108 <free@GLIBC_2.2.5>
   373df:	cmpq   $0x0,0x60(%rsp)
   373e5:	je     37390 <meshopt_peers::run+0x27e0>
   373e7:	mov    0xa8(%rsp),%rdi
   373ef:	call   *0xf1d13(%rip)        # 129108 <free@GLIBC_2.2.5>
   373f5:	cmpq   $0x0,0x8(%rsp)
   373fb:	jne    37398 <meshopt_peers::run+0x27e8>
   373fd:	jmp    373a1 <meshopt_peers::run+0x27f1>
   373ff:	jmp    37401 <meshopt_peers::run+0x2851>
   37401:	mov    %rax,%r14
   37404:	cmpq   $0x0,0x20(%rsp)
   3740a:	je     37491 <meshopt_peers::run+0x28e1>
   37410:	jmp    3751c <meshopt_peers::run+0x296c>
   37415:	mov    %rax,%r14
   37418:	lea    0x350(%rsp),%rdi
   37420:	call   33d50 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   37425:	mov    %r14,%rdi
   37428:	call   120150 <_Unwind_Resume@plt>
   3742d:	jmp    374da <meshopt_peers::run+0x292a>
   37432:	jmp    374da <meshopt_peers::run+0x292a>
   37437:	mov    %rax,%r14
   3743a:	cmpq   $0x0,0x80(%rsp)
   37443:	je     37453 <meshopt_peers::run+0x28a3>
   37445:	mov    0xc0(%rsp),%rdi
   3744d:	call   *0xf1cb5(%rip)        # 129108 <free@GLIBC_2.2.5>
   37453:	cmpq   $0x0,0x68(%rsp)
   37459:	je     37469 <meshopt_peers::run+0x28b9>
   3745b:	mov    0xb0(%rsp),%rdi
   37463:	call   *0xf1c9f(%rip)        # 129108 <free@GLIBC_2.2.5>
   37469:	cmpq   $0x0,0x40(%rsp)
   3746f:	jne    374a5 <meshopt_peers::run+0x28f5>
   37471:	cmpq   $0x0,0x70(%rsp)
   37477:	jne    374f2 <meshopt_peers::run+0x2942>
   37479:	cmpq   $0x0,0x10(%rsp)
   3747f:	jne    37505 <meshopt_peers::run+0x2955>
   37485:	cmpq   $0x0,0x20(%rsp)
   3748b:	jne    3751c <meshopt_peers::run+0x296c>
   37491:	cmpq   $0x0,0x150(%rsp)
   3749a:	jne    37535 <meshopt_peers::run+0x2985>
   374a0:	jmp    37543 <meshopt_peers::run+0x2993>
   374a5:	mov    0xd0(%rsp),%rdi
   374ad:	call   *0xf1c55(%rip)        # 129108 <free@GLIBC_2.2.5>
   374b3:	cmpq   $0x0,0x70(%rsp)
   374b9:	je     37479 <meshopt_peers::run+0x28c9>
   374bb:	jmp    374f2 <meshopt_peers::run+0x2942>
   374bd:	mov    %rax,%r14
   374c0:	cmpq   $0x0,0x150(%rsp)
   374c9:	jne    37535 <meshopt_peers::run+0x2985>
   374cb:	jmp    37543 <meshopt_peers::run+0x2993>
   374cd:	mov    %rax,%r14
   374d0:	jmp    37543 <meshopt_peers::run+0x2993>
   374d2:	mov    %rax,%r14
   374d5:	jmp    37562 <meshopt_peers::run+0x29b2>
   374da:	mov    %rax,%r14
   374dd:	lea    0x350(%rsp),%rdi
   374e5:	call   33d90 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   374ea:	cmpq   $0x0,0x70(%rsp)
   374f0:	je     37479 <meshopt_peers::run+0x28c9>
   374f2:	mov    0x28(%rsp),%rdi
   374f7:	call   *0xf1c0b(%rip)        # 129108 <free@GLIBC_2.2.5>
   374fd:	cmpq   $0x0,0x10(%rsp)
   37503:	je     37485 <meshopt_peers::run+0x28d5>
   37505:	mov    0x38(%rsp),%rdi
   3750a:	call   *0xf1bf8(%rip)        # 129108 <free@GLIBC_2.2.5>
   37510:	cmpq   $0x0,0x20(%rsp)
   37516:	je     37491 <meshopt_peers::run+0x28e1>
   3751c:	mov    0x90(%rsp),%rdi
   37524:	call   *0xf1bde(%rip)        # 129108 <free@GLIBC_2.2.5>
   3752a:	cmpq   $0x0,0x150(%rsp)
   37533:	je     37543 <meshopt_peers::run+0x2993>
   37535:	mov    0xc8(%rsp),%rdi
   3753d:	call   *0xf1bc5(%rip)        # 129108 <free@GLIBC_2.2.5>
   37543:	mov    0x738(%rsp),%rax
   3754b:	mov    0x748(%rsp),%rcx
   37553:	sub    0x740(%rsp),%rcx
   3755b:	mov    %rcx,0x358(%rax)
   37562:	cmpq   $0x0,0x158(%rsp)
   3756b:	jne    3757d <meshopt_peers::run+0x29cd>
   3756d:	cmpq   $0x0,0x20(%rsp)
   37573:	jne    37593 <meshopt_peers::run+0x29e3>
   37575:	mov    %r14,%rdi
   37578:	call   120150 <_Unwind_Resume@plt>
   3757d:	mov    0x160(%rsp),%rdi
   37585:	call   *0xf1b7d(%rip)        # 129108 <free@GLIBC_2.2.5>
   3758b:	cmpq   $0x0,0x20(%rsp)
   37591:	je     37575 <meshopt_peers::run+0x29c5>
   37593:	mov    0x50(%rsp),%rdi
   37598:	call   *0xf1b6a(%rip)        # 129108 <free@GLIBC_2.2.5>
   3759e:	mov    %r14,%rdi
   375a1:	call   120150 <_Unwind_Resume@plt>
