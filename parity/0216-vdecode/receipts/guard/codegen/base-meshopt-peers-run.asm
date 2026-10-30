
<artifacts>/guard/driver-base:     file format elf64-x86-64


Disassembly of section .text:

0000000000039d20 <meshopt_peers::run>:
   39d20:	push   %rbp
   39d21:	push   %r15
   39d23:	push   %r14
   39d25:	push   %r13
   39d27:	push   %r12
   39d29:	push   %rbx
   39d2a:	sub    $0x1000,%rsp
   39d31:	movq   $0x0,(%rsp)
   39d39:	sub    $0x988,%rsp
   39d40:	mov    %rdi,%rbx
   39d43:	mov    %rsi,0x6f0(%rsp)
   39d4b:	mov    %rdx,0x6f8(%rsp)
   39d53:	mov    0x28(%rcx),%r14
   39d57:	mov    0x20(%rcx),%r13
   39d5b:	movb   $0x2,0x6a8(%rsp)
   39d63:	mov    %r13,0x6b0(%rsp)
   39d6b:	mov    %r14,0x6b8(%rsp)
   39d73:	movq   $0x0,0x6e0(%rsp)
   39d7f:	mov    0x98(%rcx),%rax
   39d86:	movd   0xa0(%rcx),%xmm0
   39d8e:	mov    %rax,0x240(%rsp)
   39d96:	movd   %xmm0,0x248(%rsp)
   39d9f:	movl   $0x0,0x24c(%rsp)
   39daa:	lea    -0x5(%rdx),%rdi
   39dae:	cmp    $0xe,%rdi
   39db2:	ja     3a747 <meshopt_peers::run+0xa27>
   39db8:	mov    %r8,%r11
   39dbb:	lea    -0x28aae(%rip),%r8        # 11314 <libm::math::generic::sqrt::RSQRT_TAB+0x122>
   39dc2:	movslq (%r8,%rdi,4),%rdi
   39dc6:	add    %r8,%rdi
   39dc9:	jmp    *%rdi
   39dcb:	mov    $0x68636163,%eax
   39dd0:	xor    (%rsi),%eax
   39dd2:	movzbl 0x4(%rsi),%edi
   39dd6:	xor    $0x65,%edi
   39dd9:	or     %eax,%edi
   39ddb:	je     3aa88 <meshopt_peers::run+0xd68>
   39de1:	mov    $0x63746566,%eax
   39de6:	xor    (%rsi),%eax
   39de8:	movzbl 0x4(%rsi),%edi
   39dec:	xor    $0x68,%edi
   39def:	or     %eax,%edi
   39df1:	je     3afda <meshopt_peers::run+0x12ba>
   39df7:	mov    $0x616d6572,%eax
   39dfc:	xor    (%rsi),%eax
   39dfe:	movzbl 0x4(%rsi),%edi
   39e02:	xor    $0x70,%edi
   39e05:	or     %eax,%edi
   39e07:	je     3a782 <meshopt_peers::run+0xa62>
   39e0d:	jmp    3a747 <meshopt_peers::run+0xa27>
   39e12:	movabs $0x7966696c706d6973,%rax
   39e1c:	cmp    %rax,(%rsi)
   39e1f:	je     3a96b <meshopt_peers::run+0xc4b>
   39e25:	movabs $0x776172647265766f,%rax
   39e2f:	cmp    %rax,(%rsi)
   39e32:	je     3af03 <meshopt_peers::run+0x11e3>
   39e38:	movabs $0x7374656c6873656d,%rax
   39e42:	cmp    %rax,(%rsi)
   39e45:	jne    3a747 <meshopt_peers::run+0xa27>
   39e4b:	mov    0x38(%rcx),%rsi
   39e4f:	mov    0x40(%rcx),%rdx
   39e53:	movq   $0x7c,0x330(%rsp)
   39e5f:	movq   $0x0,0x324(%rsp)
   39e6b:	movl   $0x1,0x320(%rsp)
   39e76:	lea    0x320(%rsp),%rax
   39e7e:	lea    0x750(%rsp),%rdi
   39e86:	lea    0x6a8(%rsp),%r8
   39e8e:	mov    $0x40,%r9d
   39e94:	mov    %r14,%rcx
   39e97:	push   $0x2
   39e99:	push   %r11
   39e9b:	push   %rax
   39e9c:	push   $0x7c
   39e9e:	call   c9a20 <meshoptimizer_rs::meshlet::build>
   39ea3:	add    $0x20,%rsp
   39ea7:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   39eb0:	je     3d144 <meshopt_peers::run+0x3424>
   39eb6:	mov    0x790(%rsp),%rax
   39ebe:	mov    %rax,0x360(%rsp)
   39ec6:	movdqu 0x750(%rsp),%xmm0
   39ecf:	movups 0x760(%rsp),%xmm1
   39ed7:	movups 0x770(%rsp),%xmm2
   39edf:	movups 0x780(%rsp),%xmm3
   39ee7:	movaps %xmm3,0x350(%rsp)
   39eef:	movaps %xmm2,0x340(%rsp)
   39ef7:	movaps %xmm1,0x330(%rsp)
   39eff:	movdqa %xmm0,0x320(%rsp)
   39f08:	mov    0x328(%rsp),%r14
   39f10:	mov    0x330(%rsp),%r12
   39f18:	test   %r12,%r12
   39f1b:	je     3b6c8 <meshopt_peers::run+0x19a8>
   39f21:	mov    %r12,%r15
   39f24:	shl    $0x4,%r15
   39f28:	mov    %r15,%rdi
   39f2b:	call   *0x10f02f(%rip)        # 148f60 <malloc@GLIBC_2.2.5>
   39f31:	test   %rax,%rax
   39f34:	je     3d6c6 <meshopt_peers::run+0x39a6>
   39f3a:	mov    %r12d,%ecx
   39f3d:	and    $0x3,%ecx
   39f40:	cmp    $0x4,%r12
   39f44:	jae    3b73a <meshopt_peers::run+0x1a1a>
   39f4a:	xor    %edx,%edx
   39f4c:	jmp    3b78e <meshopt_peers::run+0x1a6e>
   39f51:	movabs $0x7374656c6873656d,%rax
   39f5b:	xor    (%rsi),%rax
   39f5e:	movabs $0x78656c662d737465,%rdx
   39f68:	xor    0x5(%rsi),%rdx
   39f6c:	or     %rax,%rdx
   39f6f:	je     3aa04 <meshopt_peers::run+0xce4>
   39f75:	movabs $0x65646f6365642d78,%rax
   39f7f:	movabs $0x652d786574726576,%rdx
   39f89:	xor    (%rsi),%rdx
   39f8c:	lea    0x9010000(%rax),%rdi
   39f93:	xor    0x5(%rsi),%rdi
   39f97:	or     %rdx,%rdi
   39f9a:	je     3af72 <meshopt_peers::run+0x1252>
   39fa0:	movabs $0x642d786574726576,%rdx
   39faa:	xor    (%rsi),%rdx
   39fad:	xor    0x5(%rsi),%rax
   39fb1:	or     %rdx,%rax
   39fb4:	jne    3a747 <meshopt_peers::run+0xa27>
   39fba:	mov    0x68(%rcx),%r9
   39fbe:	lea    0x750(%rsp),%rdi
   39fc6:	mov    $0xc,%r8d
   39fcc:	xor    %esi,%esi
   39fce:	xor    %edx,%edx
   39fd0:	mov    %rcx,%rax
   39fd3:	mov    %r14,%rcx
   39fd6:	push   %r11
   39fd8:	push   0x70(%rax)
   39fdb:	call   c0950 <meshoptimizer_rs::codec::allocate>
   39fe0:	add    $0x10,%rsp
   39fe4:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   39fed:	jne    3afb5 <meshopt_peers::run+0x1295>
   39ff3:	movzbl 0x758(%rsp),%eax
   39ffb:	mov    %al,0x320(%rsp)
   3a002:	lea    -0x28c36(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a009:	lea    0x107578(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3a010:	lea    0x1071b1(%rip),%r8        # 1411c8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x398>
   3a017:	lea    0x320(%rsp),%rdx
   3a01f:	mov    $0x2b,%esi
   3a024:	call   *0x10f00e(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3a02a:	mov    $0x64616873,%eax
   3a02f:	xor    (%rsi),%eax
   3a031:	movzwl 0x4(%rsi),%edi
   3a035:	xor    $0x776f,%edi
   3a03b:	or     %eax,%edi
   3a03d:	jne    3a747 <meshopt_peers::run+0xa27>
   3a043:	mov    %r14,%rax
   3a046:	shr    $0x20,%rax
   3a04a:	jne    3d55c <meshopt_peers::run+0x383c>
   3a050:	lea    0x0(,%r14,4),%rax
   3a058:	lea    (%rax,%rax,2),%rax
   3a05c:	mov    %r13,0x320(%rsp)
   3a064:	mov    %rax,0x328(%rsp)
   3a06c:	mov    %r14,0x330(%rsp)
   3a074:	movq   $0xc,0x338(%rsp)
   3a080:	movq   $0xc,0x340(%rsp)
   3a08c:	movq   $0x0,0x348(%rsp)
   3a098:	cmp    $0x6,%rdx
   3a09c:	jne    3a0b7 <meshopt_peers::run+0x397>
   3a09e:	mov    $0x64616873,%eax
   3a0a3:	xor    (%rsi),%eax
   3a0a5:	movzwl 0x4(%rsi),%edx
   3a0a9:	xor    $0x776f,%edx
   3a0af:	or     %eax,%edx
   3a0b1:	je     3b2a3 <meshopt_peers::run+0x1583>
   3a0b7:	test   %r14,%r14
   3a0ba:	setne  %dl
   3a0bd:	mov    0x58(%rcx),%rax
   3a0c1:	shl    $0x2,%rax
   3a0c5:	mov    %r14,%rsi
   3a0c8:	shl    $0x4,%rsi
   3a0cc:	cmp    %rax,%rsi
   3a0cf:	seta   %sil
   3a0d3:	and    %dl,%sil
   3a0d6:	cmp    $0x1,%sil
   3a0da:	je     3d5ec <meshopt_peers::run+0x38cc>
   3a0e0:	mov    0x38(%rcx),%rsi
   3a0e4:	mov    0x40(%rcx),%rdx
   3a0e8:	mov    0x50(%rcx),%rcx
   3a0ec:	movdqu 0x320(%rsp),%xmm0
   3a0f5:	movups 0x330(%rsp),%xmm1
   3a0fd:	movups 0x340(%rsp),%xmm2
   3a105:	movaps %xmm2,0x770(%rsp)
   3a10d:	movaps %xmm1,0x760(%rsp)
   3a115:	movdqa %xmm0,0x750(%rsp)
   3a11e:	mov    %rcx,0x780(%rsp)
   3a126:	mov    %rax,0x788(%rsp)
   3a12e:	mov    %r14,0x790(%rsp)
   3a136:	movq   $0x10,0x798(%rsp)
   3a142:	movq   $0x10,0x7a0(%rsp)
   3a14e:	movq   $0x0,0x7a8(%rsp)
   3a15a:	sub    $0x10,%rsp
   3a15e:	mov    %r11,0x8(%rsp)
   3a163:	lea    0x260(%rsp),%rdi
   3a16b:	lea    0x760(%rsp),%rcx
   3a173:	mov    $0x2,%r8d
   3a179:	xor    %r9d,%r9d
   3a17c:	call   684d0 <meshoptimizer_rs::remap::shadow_filter::<false>>
   3a181:	add    $0x10,%rsp
   3a185:	mov    0x250(%rsp),%rax
   3a18d:	movzbl 0x258(%rsp),%ecx
   3a195:	cmp    $0xffffffffffffffff,%rax
   3a199:	je     3a1bb <meshopt_peers::run+0x49b>
   3a19b:	mov    0x259(%rsp),%rdx
   3a1a3:	mov    0x260(%rsp),%rsi
   3a1ab:	mov    %rsi,0x1a0(%rsp)
   3a1b3:	mov    %rdx,0x199(%rsp)
   3a1bb:	mov    %cl,0x198(%rsp)
   3a1c2:	mov    %rax,0x190(%rsp)
   3a1ca:	cmp    $0xffffffffffffffff,%rax
   3a1ce:	je     3b2d6 <meshopt_peers::run+0x15b6>
   3a1d4:	mov    0x1a0(%rsp),%rax
   3a1dc:	mov    %rax,0x18(%rbx)
   3a1e0:	movdqu 0x190(%rsp),%xmm0
   3a1e9:	jmp    3b293 <meshopt_peers::run+0x1573>
   3a1ee:	movabs $0x7966696c706d6973,%rax
   3a1f8:	xor    (%rsi),%rax
   3a1fb:	movabs $0x656e7572702d7966,%rdx
   3a205:	xor    0x6(%rsi),%rdx
   3a209:	or     %rax,%rdx
   3a20c:	je     3aaea <meshopt_peers::run+0xdca>
   3a212:	movabs $0x6e6f697469736f70,%rax
   3a21c:	xor    (%rsi),%rax
   3a21f:	movabs $0x70616d65722d6e6f,%rcx
   3a229:	xor    0x6(%rsi),%rcx
   3a22d:	or     %rax,%rcx
   3a230:	jne    3a747 <meshopt_peers::run+0xa27>
   3a236:	lea    0x750(%rsp),%rdi
   3a23e:	lea    0x6a8(%rsp),%rsi
   3a246:	mov    %r11,%rdx
   3a249:	call   *0x10f0e9(%rip)        # 149338 <_DYNAMIC+0x5d0>
   3a24f:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a258:	jne    3b27e <meshopt_peers::run+0x155e>
   3a25e:	movzbl 0x758(%rsp),%eax
   3a266:	mov    %al,0x320(%rsp)
   3a26d:	lea    -0x28ea1(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a274:	lea    0x10730d(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3a27b:	lea    0x106ece(%rip),%r8        # 141150 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x320>
   3a282:	lea    0x320(%rsp),%rdx
   3a28a:	mov    $0x2b,%esi
   3a28f:	call   *0x10eda3(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3a295:	movabs $0x7966696c706d6973,%rdx
   3a29f:	mov    (%rsi),%rdi
   3a2a2:	xor    %rdx,%rdi
   3a2a5:	movabs $0x7970706f6c732d79,%r8
   3a2af:	xor    0x7(%rsi),%r8
   3a2b3:	or     %rdi,%r8
   3a2b6:	je     3ab60 <meshopt_peers::run+0xe40>
   3a2bc:	mov    (%rsi),%rax
   3a2bf:	xor    %rdx,%rax
   3a2c2:	movabs $0x6574616470752d79,%rdi
   3a2cc:	xor    0x7(%rsi),%rdi
   3a2d0:	or     %rax,%rdi
   3a2d3:	je     3b0d4 <meshopt_peers::run+0x13b4>
   3a2d9:	xor    (%rsi),%rdx
   3a2dc:	movabs $0x73746e696f702d79,%rax
   3a2e6:	xor    0x7(%rsi),%rax
   3a2ea:	or     %rdx,%rax
   3a2ed:	jne    3a747 <meshopt_peers::run+0xa27>
   3a2f3:	movb   $0xff,0x750(%rsp)
   3a2fb:	shr    $1,%r14
   3a2fe:	sub    $0x8,%rsp
   3a302:	lea    0x328(%rsp),%rdi
   3a30a:	lea    0x6b0(%rsp),%rsi
   3a312:	lea    0x758(%rsp),%rdx
   3a31a:	pxor   %xmm0,%xmm0
   3a31e:	mov    %r14,%rcx
   3a321:	xor    %r8d,%r8d
   3a324:	push   %r11
   3a326:	call   f2f20 <meshoptimizer_rs::simplify::extra::simplify_points_kernel>
   3a32b:	add    $0x10,%rsp
   3a32f:	mov    0x320(%rsp),%rcx
   3a337:	movzbl 0x328(%rsp),%eax
   3a33f:	cmp    $0xffffffffffffffff,%rcx
   3a343:	je     3d643 <meshopt_peers::run+0x3923>
   3a349:	mov    0x329(%rsp),%rdx
   3a351:	mov    0x330(%rsp),%rsi
   3a359:	jmp    3ab46 <meshopt_peers::run+0xe26>
   3a35e:	movabs $0x756d2d70616d6572,%rax
   3a368:	xor    (%rsi),%rax
   3a36b:	movabs $0x69746c756d2d7061,%rdi
   3a375:	xor    0x3(%rsi),%rdi
   3a379:	or     %rax,%rdi
   3a37c:	je     3a782 <meshopt_peers::run+0xa62>
   3a382:	movabs $0x6e692d70616d6572,%rax
   3a38c:	xor    (%rsi),%rax
   3a38f:	movabs $0x7865646e692d7061,%rdx
   3a399:	xor    0x3(%rsi),%rdx
   3a39d:	or     %rax,%rdx
   3a3a0:	je     3b24b <meshopt_peers::run+0x152b>
   3a3a6:	movabs $0x2d72657473756c63,%rax
   3a3b0:	xor    (%rsi),%rax
   3a3b3:	movabs $0x646f6c2d72657473,%rdx
   3a3bd:	xor    0x3(%rsi),%rdx
   3a3c1:	or     %rax,%rdx
   3a3c4:	jne    3a747 <meshopt_peers::run+0xa27>
   3a3ca:	mov    %r14,%rax
   3a3cd:	mov    %rbx,0x8(%rsp)
   3a3d2:	test   %r14,%r14
   3a3d5:	je     3b3be <meshopt_peers::run+0x169e>
   3a3db:	mov    %r11,%rbp
   3a3de:	mov    %rcx,%rbx
   3a3e1:	mov    %rax,%r14
   3a3e4:	lea    0x0(,%rax,4),%rax
   3a3ec:	lea    (%rax,%rax,2),%r15
   3a3f0:	mov    %r15,%rdi
   3a3f3:	call   *0x10eb67(%rip)        # 148f60 <malloc@GLIBC_2.2.5>
   3a3f9:	test   %rax,%rax
   3a3fc:	je     3d6b8 <meshopt_peers::run+0x3998>
   3a402:	mov    %rax,%r12
   3a405:	mov    %rax,%rdi
   3a408:	mov    %r13,%rsi
   3a40b:	mov    %r15,%rdx
   3a40e:	call   *0x10eb6c(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   3a414:	mov    %rbx,%rcx
   3a417:	mov    %rbp,%r11
   3a41a:	jmp    3b3c7 <meshopt_peers::run+0x16a7>
   3a41f:	movabs $0x6e652d7865646e69,%rax
   3a429:	xor    (%rsi),%rax
   3a42c:	mov    0x8(%rsi),%edi
   3a42f:	xor    $0x65646f63,%rdi
   3a436:	or     %rax,%rdi
   3a439:	je     3acdd <meshopt_peers::run+0xfbd>
   3a43f:	movabs $0x75632d70616d6572,%rax
   3a449:	xor    (%rsi),%rax
   3a44c:	mov    0x8(%rsi),%edi
   3a44f:	xor    $0x6d6f7473,%rdi
   3a456:	or     %rax,%rdi
   3a459:	je     3a782 <meshopt_peers::run+0xa62>
   3a45f:	movabs $0x65762d70616d6572,%rax
   3a469:	xor    (%rsi),%rax
   3a46c:	mov    0x8(%rsi),%edi
   3a46f:	xor    $0x78657472,%rdi
   3a476:	or     %rax,%rdi
   3a479:	je     3b31f <meshopt_peers::run+0x15ff>
   3a47f:	movabs $0x6d2d776f64616873,%rax
   3a489:	xor    (%rsi),%rax
   3a48c:	mov    0x8(%rsi),%edi
   3a48f:	xor    $0x69746c75,%rdi
   3a496:	or     %rax,%rdi
   3a499:	je     3a043 <meshopt_peers::run+0x323>
   3a49f:	movabs $0x65642d7865646e69,%rax
   3a4a9:	xor    (%rsi),%rax
   3a4ac:	mov    0x8(%rsi),%edx
   3a4af:	xor    $0x65646f63,%rdx
   3a4b6:	or     %rax,%rdx
   3a4b9:	jne    3a747 <meshopt_peers::run+0xa27>
   3a4bf:	mov    0x40(%rcx),%rax
   3a4c3:	mov    0x80(%rcx),%r9
   3a4ca:	lea    0x750(%rsp),%rdi
   3a4d2:	mov    $0x4,%r8d
   3a4d8:	mov    $0x1,%esi
   3a4dd:	xor    %edx,%edx
   3a4df:	mov    %rcx,%r10
   3a4e2:	mov    %rax,%rcx
   3a4e5:	push   %r11
   3a4e7:	push   0x88(%r10)
   3a4ee:	call   c0950 <meshoptimizer_rs::codec::allocate>
   3a4f3:	add    $0x10,%rsp
   3a4f7:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a500:	jne    3afb5 <meshopt_peers::run+0x1295>
   3a506:	movzbl 0x758(%rsp),%eax
   3a50e:	mov    %al,0x320(%rsp)
   3a515:	lea    -0x29149(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3a51c:	lea    0x107065(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3a523:	lea    0x106b7e(%rip),%r8        # 1410a8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x278>
   3a52a:	lea    0x320(%rsp),%rdx
   3a532:	mov    $0x2b,%esi
   3a537:	call   *0x10eafb(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3a53d:	movdqu (%rsi),%xmm0
   3a541:	movdqu 0x3(%rsi),%xmm1
   3a546:	pcmpeqb -0x2b71e(%rip),%xmm1        # ee30 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xa0>
   3a54e:	pcmpeqb -0x2bbb6(%rip),%xmm0        # e9a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x350>
   3a556:	pand   %xmm1,%xmm0
   3a55a:	pmovmskb %xmm0,%eax
   3a55e:	cmp    $0xffff,%eax
   3a563:	jne    3a747 <meshopt_peers::run+0xa27>
   3a569:	mov    0x58(%rcx),%rax
   3a56d:	test   %r14,%r14
   3a570:	setne  %dl
   3a573:	lea    0x0(,%r14,4),%rsi
   3a57b:	cmp    %rax,%rsi
   3a57e:	seta   %sil
   3a582:	and    %dl,%sil
   3a585:	cmp    $0x1,%sil
   3a589:	je     3d5bc <meshopt_peers::run+0x389c>
   3a58f:	mov    0x38(%rcx),%rsi
   3a593:	mov    0x40(%rcx),%rdx
   3a597:	mov    0x50(%rcx),%rcx
   3a59b:	movb   $0x0,0x750(%rsp)
   3a5a3:	mov    %rcx,0x758(%rsp)
   3a5ab:	mov    %rax,0x760(%rsp)
   3a5b3:	mov    %r14,0x768(%rsp)
   3a5bb:	movq   $0x4,0x770(%rsp)
   3a5c7:	movq   $0x4,0x778(%rsp)
   3a5d3:	pxor   %xmm0,%xmm0
   3a5d7:	movdqu %xmm0,0x780(%rsp)
   3a5e0:	sub    $0x30,%rsp
   3a5e4:	mov    %r11,0x20(%rsp)
   3a5e9:	lea    0x270(%rsp),%rax
   3a5f1:	mov    %rax,0x18(%rsp)
   3a5f6:	movq   $0x0,0x8(%rsp)
   3a5ff:	movq   $0x4,(%rsp)
   3a607:	lea    -0x2b78e(%rip),%r9        # ee80 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xf0>
   3a60e:	lea    0x280(%rsp),%rdi
   3a616:	lea    0x6d8(%rsp),%rcx
   3a61e:	lea    0x780(%rsp),%r8
   3a626:	jmp    3a9b8 <meshopt_peers::run+0xc98>
   3a62b:	movdqu (%rsi),%xmm0
   3a62f:	pcmpeqb -0x2be27(%rip),%xmm0        # e810 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x1c0>
   3a637:	pmovmskb %xmm0,%eax
   3a63b:	cmp    $0xffff,%eax
   3a640:	jne    3a747 <meshopt_peers::run+0xa27>
   3a646:	mov    0x38(%rcx),%rsi
   3a64a:	mov    0x40(%rcx),%rdx
   3a64e:	movq   $0x29,0x258(%rsp)
   3a65a:	movabs $0x3f00000000000002,%rax
   3a664:	mov    %rax,0x250(%rsp)
   3a66c:	lea    0x250(%rsp),%rax
   3a674:	lea    0x750(%rsp),%rdi
   3a67c:	lea    0x6a8(%rsp),%r8
   3a684:	mov    $0x40,%r9d
   3a68a:	mov    %r14,%rcx
   3a68d:	push   $0x2
   3a68f:	push   %r11
   3a691:	push   %rax
   3a692:	push   $0x7c
   3a694:	call   c9a20 <meshoptimizer_rs::meshlet::build>
   3a699:	add    $0x20,%rsp
   3a69d:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3a6a6:	je     3aa51 <meshopt_peers::run+0xd31>
   3a6ac:	mov    0x790(%rsp),%rax
   3a6b4:	mov    %rax,0x360(%rsp)
   3a6bc:	movdqu 0x750(%rsp),%xmm0
   3a6c5:	movups 0x760(%rsp),%xmm1
   3a6cd:	movups 0x770(%rsp),%xmm2
   3a6d5:	movups 0x780(%rsp),%xmm3
   3a6dd:	movaps %xmm3,0x350(%rsp)
   3a6e5:	movaps %xmm2,0x340(%rsp)
   3a6ed:	movaps %xmm1,0x330(%rsp)
   3a6f5:	movdqa %xmm0,0x320(%rsp)
   3a6fe:	mov    0x328(%rsp),%r14
   3a706:	mov    0x330(%rsp),%r12
   3a70e:	test   %r12,%r12
   3a711:	je     3a89b <meshopt_peers::run+0xb7b>
   3a717:	mov    %r12,%r15
   3a71a:	shl    $0x4,%r15
   3a71e:	mov    %r15,%rdi
   3a721:	call   *0x10e839(%rip)        # 148f60 <malloc@GLIBC_2.2.5>
   3a727:	test   %rax,%rax
   3a72a:	je     3d61c <meshopt_peers::run+0x38fc>
   3a730:	mov    %r12d,%ecx
   3a733:	and    $0x3,%ecx
   3a736:	cmp    $0x4,%r12
   3a73a:	jae    3a8a2 <meshopt_peers::run+0xb82>
   3a740:	xor    %edx,%edx
   3a742:	jmp    3a8ee <meshopt_peers::run+0xbce>
   3a747:	lea    0x6f0(%rsp),%rax
   3a74f:	mov    %rax,0x750(%rsp)
   3a757:	lea    0x8d92(%rip),%rax        # 434f0 <<&str as core::fmt::Display>::fmt>
   3a75e:	mov    %rax,0x758(%rsp)
   3a766:	lea    -0x2d2d1(%rip),%rdi        # d49c <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x14a>
   3a76d:	lea    0x10691c(%rip),%rdx        # 141090 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x260>
   3a774:	lea    0x750(%rsp),%rsi
   3a77c:	call   *0x10e886(%rip)        # 149008 <_DYNAMIC+0x2a0>
   3a782:	mov    %r14,%rax
   3a785:	shr    $0x20,%rax
   3a789:	jne    3d58c <meshopt_peers::run+0x386c>
   3a78f:	lea    0x0(,%r14,4),%rax
   3a797:	lea    (%rax,%rax,2),%rax
   3a79b:	cmp    $0x5,%rdx
   3a79f:	je     3ad41 <meshopt_peers::run+0x1021>
   3a7a5:	cmp    $0xb,%rdx
   3a7a9:	jne    3ad57 <meshopt_peers::run+0x1037>
   3a7af:	movabs $0x756d2d70616d6572,%rdx
   3a7b9:	xor    (%rsi),%rdx
   3a7bc:	movabs $0x69746c756d2d7061,%rdi
   3a7c6:	xor    0x3(%rsi),%rdi
   3a7ca:	or     %rdx,%rdi
   3a7cd:	jne    3ad57 <meshopt_peers::run+0x1037>
   3a7d3:	test   %r14,%r14
   3a7d6:	setne  %dl
   3a7d9:	mov    0x58(%rcx),%rdi
   3a7dd:	shl    $0x2,%rdi
   3a7e1:	mov    %r14,%rsi
   3a7e4:	shl    $0x4,%rsi
   3a7e8:	cmp    %rdi,%rsi
   3a7eb:	seta   %sil
   3a7ef:	and    %dl,%sil
   3a7f2:	cmp    $0x1,%sil
   3a7f6:	je     3d79e <meshopt_peers::run+0x3a7e>
   3a7fc:	mov    0x38(%rcx),%rsi
   3a800:	mov    0x40(%rcx),%rdx
   3a804:	mov    0x50(%rcx),%rcx
   3a808:	mov    %r13,0x750(%rsp)
   3a810:	mov    %rax,0x758(%rsp)
   3a818:	mov    %r14,0x760(%rsp)
   3a820:	movq   $0xc,0x768(%rsp)
   3a82c:	movq   $0xc,0x770(%rsp)
   3a838:	movq   $0x0,0x778(%rsp)
   3a844:	mov    %rcx,0x780(%rsp)
   3a84c:	mov    %rdi,0x788(%rsp)
   3a854:	mov    %r14,0x790(%rsp)
   3a85c:	movq   $0x10,0x798(%rsp)
   3a868:	movq   $0x10,0x7a0(%rsp)
   3a874:	movq   $0x0,0x7a8(%rsp)
   3a880:	lea    0x320(%rsp),%rdi
   3a888:	lea    0x750(%rsp),%rcx
   3a890:	mov    $0x2,%r8d
   3a896:	jmp    3b72c <meshopt_peers::run+0x1a0c>
   3a89b:	mov    $0x4,%eax
   3a8a0:	jmp    3a913 <meshopt_peers::run+0xbf3>
   3a8a2:	mov    %r12,%rsi
   3a8a5:	and    $0xfffffffffffffffc,%rsi
   3a8a9:	mov    $0x30,%edi
   3a8ae:	xor    %edx,%edx
   3a8b0:	movups -0x30(%r14,%rdi,1),%xmm0
   3a8b6:	movups %xmm0,-0x30(%rax,%rdi,1)
   3a8bb:	movups -0x20(%r14,%rdi,1),%xmm0
   3a8c1:	movups %xmm0,-0x20(%rax,%rdi,1)
   3a8c6:	movups -0x10(%r14,%rdi,1),%xmm0
   3a8cc:	movups %xmm0,-0x10(%rax,%rdi,1)
   3a8d1:	movdqu (%r14,%rdi,1),%xmm0
   3a8d7:	movdqu %xmm0,(%rax,%rdi,1)
   3a8dc:	add    $0x4,%rdx
   3a8e0:	add    $0x40,%rdi
   3a8e4:	cmp    %rdx,%rsi
   3a8e7:	jne    3a8b0 <meshopt_peers::run+0xb90>
   3a8e9:	test   %rcx,%rcx
   3a8ec:	je     3a913 <meshopt_peers::run+0xbf3>
   3a8ee:	shl    $0x4,%rdx
   3a8f2:	mov    %rax,%rsi
   3a8f5:	add    %rdx,%rsi
   3a8f8:	add    %r14,%rdx
   3a8fb:	shl    $0x4,%ecx
   3a8fe:	xor    %edi,%edi
   3a900:	movdqu (%rdx,%rdi,1),%xmm0
   3a905:	movdqu %xmm0,(%rsi,%rdi,1)
   3a90a:	add    $0x10,%rdi
   3a90e:	cmp    %rdi,%rcx
   3a911:	jne    3a900 <meshopt_peers::run+0xbe0>
   3a913:	mov    0x348(%rsp),%rcx
   3a91b:	mov    %rcx,0x30(%rbx)
   3a91f:	movups 0x338(%rsp),%xmm0
   3a927:	movups %xmm0,0x20(%rbx)
   3a92b:	movdqa 0x350(%rsp),%xmm0
   3a934:	movdqu %xmm0,0x38(%rbx)
   3a939:	mov    0x360(%rsp),%rcx
   3a941:	mov    %rcx,0x48(%rbx)
   3a945:	mov    %r12,0x8(%rbx)
   3a949:	mov    %rax,0x10(%rbx)
   3a94d:	mov    %r12,0x18(%rbx)
   3a951:	movl   $0x6,(%rbx)
   3a957:	cmpq   $0x0,0x320(%rsp)
   3a960:	je     3de14 <meshopt_peers::run+0x40f4>
   3a966:	jmp    3b806 <meshopt_peers::run+0x1ae6>
   3a96b:	mov    0x38(%rcx),%rsi
   3a96f:	mov    0x40(%rcx),%rdx
   3a973:	movb   $0x2,0x750(%rsp)
   3a97b:	sub    $0x30,%rsp
   3a97f:	mov    %r11,0x20(%rsp)
   3a984:	lea    0x270(%rsp),%rax
   3a98c:	mov    %rax,0x18(%rsp)
   3a991:	pxor   %xmm0,%xmm0
   3a995:	movdqu %xmm0,(%rsp)
   3a99a:	lea    0x280(%rsp),%rdi
   3a9a2:	lea    0x6d8(%rsp),%rcx
   3a9aa:	lea    0x780(%rsp),%r8
   3a9b2:	mov    $0x4,%r9d
   3a9b8:	call   d46c0 <meshoptimizer_rs::simplify::allocating>
   3a9bd:	add    $0x30,%rsp
   3a9c1:	mov    0x250(%rsp),%rax
   3a9c9:	cmp    $0xffffffffffffffff,%rax
   3a9cd:	je     3aca6 <meshopt_peers::run+0xf86>
   3a9d3:	mov    0x260(%rsp),%rax
   3a9db:	mov    %rax,0x18(%rbx)
   3a9df:	movups 0x250(%rsp),%xmm0
   3a9e7:	movups %xmm0,0x8(%rbx)
   3a9eb:	movd   0x268(%rsp),%xmm0
   3a9f4:	movd   %xmm0,0x4(%rbx)
   3a9f9:	movl   $0x4,(%rbx)
   3a9ff:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3aa04:	mov    0x38(%rcx),%rsi
   3aa08:	mov    0x40(%rcx),%rdx
   3aa0c:	lea    0x750(%rsp),%rdi
   3aa14:	lea    0x6a8(%rsp),%rcx
   3aa1c:	movss  -0x2a074(%rip),%xmm1        # 109b0 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa4>
   3aa24:	mov    $0x40,%r8d
   3aa2a:	mov    $0x7c,%r9d
   3aa30:	pxor   %xmm0,%xmm0
   3aa34:	push   %r11
   3aa36:	push   $0x29
   3aa38:	call   *0x10e902(%rip)        # 149340 <_DYNAMIC+0x5d8>
   3aa3e:	add    $0x10,%rsp
   3aa42:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3aa4b:	jne    3a6ac <meshopt_peers::run+0x98c>
   3aa51:	movzbl 0x758(%rsp),%eax
   3aa59:	mov    %al,0x250(%rsp)
   3aa60:	lea    -0x29694(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3aa67:	lea    0x106b1a(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3aa6e:	lea    0x106783(%rip),%r8        # 1411f8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3c8>
   3aa75:	lea    0x250(%rsp),%rdx
   3aa7d:	mov    $0x2b,%esi
   3aa82:	call   *0x10e5b0(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3aa88:	mov    0x38(%rcx),%rsi
   3aa8c:	mov    0x40(%rcx),%rdx
   3aa90:	lea    0x750(%rsp),%rdi
   3aa98:	mov    %r14,%rcx
   3aa9b:	mov    %r11,%r8
   3aa9e:	call   *0x10e8a4(%rip)        # 149348 <_DYNAMIC+0x5e0>
   3aaa4:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3aaad:	jne    3b27e <meshopt_peers::run+0x155e>
   3aab3:	movzbl 0x758(%rsp),%eax
   3aabb:	mov    %al,0x320(%rsp)
   3aac2:	lea    -0x296f6(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3aac9:	lea    0x106ab8(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3aad0:	lea    0x106799(%rip),%r8        # 141270 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x440>
   3aad7:	lea    0x320(%rsp),%rdx
   3aadf:	mov    $0x2b,%esi
   3aae4:	call   *0x10e54e(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3aaea:	mov    0x38(%rcx),%rsi
   3aaee:	mov    0x40(%rcx),%rdx
   3aaf2:	sub    $0x8,%rsp
   3aaf6:	lea    0x758(%rsp),%rdi
   3aafe:	lea    0x6b0(%rsp),%rcx
   3ab06:	movd   -0x2a162(%rip),%xmm0        # 109ac <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa0>
   3ab0e:	xor    %r8d,%r8d
   3ab11:	push   %r11
   3ab13:	call   f1780 <meshoptimizer_rs::simplify::extra::simplify_prune_kernel>
   3ab18:	add    $0x10,%rsp
   3ab1c:	mov    0x750(%rsp),%rcx
   3ab24:	movzbl 0x758(%rsp),%eax
   3ab2c:	cmp    $0xffffffffffffffff,%rcx
   3ab30:	je     3d672 <meshopt_peers::run+0x3952>
   3ab36:	mov    0x759(%rsp),%rdx
   3ab3e:	mov    0x760(%rsp),%rsi
   3ab46:	mov    %rsi,0x18(%rbx)
   3ab4a:	mov    %rdx,0x11(%rbx)
   3ab4e:	mov    %rcx,0x8(%rbx)
   3ab52:	mov    %al,0x10(%rbx)
   3ab55:	movl   $0x3,(%rbx)
   3ab5b:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3ab60:	movdqu 0x38(%rcx),%xmm1
   3ab65:	movq   $0x0,0x130(%rsp)
   3ab71:	mov    %rax,0xf8(%rsp)
   3ab79:	movd   %xmm0,0x2e0(%rsp)
   3ab82:	movb   $0x0,0x365(%r11)
   3ab8a:	movl   $0x0,0x361(%r11)
   3ab95:	pxor   %xmm0,%xmm0
   3ab99:	movdqu %xmm0,0x350(%r11)
   3aba2:	mov    0x348(%r11),%rax
   3aba9:	mov    %rax,0x190(%rsp)
   3abb1:	mov    %rax,0x198(%rsp)
   3abb9:	movdqu %xmm1,0x750(%rsp)
   3abc2:	lea    0x6a8(%rsp),%rax
   3abca:	mov    %rax,0x760(%rsp)
   3abd2:	lea    0x190(%rsp),%rax
   3abda:	mov    %rax,0x768(%rsp)
   3abe2:	lea    0x130(%rsp),%rax
   3abea:	mov    %rax,0x770(%rsp)
   3abf2:	lea    0xf8(%rsp),%rax
   3abfa:	mov    %rax,0x778(%rsp)
   3ac02:	lea    0x2e0(%rsp),%rax
   3ac0a:	mov    %rax,0x780(%rsp)
   3ac12:	mov    %r11,0x788(%rsp)
   3ac1a:	movq   $0x0,0x790(%rsp)
   3ac26:	lea    0x320(%rsp),%rdi
   3ac2e:	lea    0x750(%rsp),%rsi
   3ac36:	mov    %r11,%r14
   3ac39:	call   8e0e0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   3ac3e:	mov    0x198(%rsp),%rax
   3ac46:	sub    0x190(%rsp),%rax
   3ac4e:	mov    %rax,0x358(%r14)
   3ac55:	mov    0x320(%rsp),%rax
   3ac5d:	movzbl 0x328(%rsp),%ecx
   3ac65:	cmp    $0xffffffffffffffff,%rax
   3ac69:	je     3ac8d <meshopt_peers::run+0xf6d>
   3ac6b:	mov    0x338(%rsp),%rdx
   3ac73:	mov    %rdx,0x268(%rsp)
   3ac7b:	movdqu 0x329(%rsp),%xmm0
   3ac84:	movdqu %xmm0,0x259(%rsp)
   3ac8d:	mov    %cl,0x258(%rsp)
   3ac94:	mov    %rax,0x250(%rsp)
   3ac9c:	cmp    $0xffffffffffffffff,%rax
   3aca0:	jne    3a9d3 <meshopt_peers::run+0xcb3>
   3aca6:	movzbl 0x258(%rsp),%eax
   3acae:	mov    %al,0x750(%rsp)
   3acb5:	lea    -0x298e9(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3acbc:	lea    0x1068c5(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3acc3:	lea    0x10661e(%rip),%r8        # 1412e8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x4b8>
   3acca:	lea    0x750(%rsp),%rdx
   3acd2:	mov    $0x2b,%esi
   3acd7:	call   *0x10e35b(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3acdd:	mov    0x38(%rcx),%rsi
   3ace1:	mov    0x40(%rcx),%rdx
   3ace5:	lea    0x750(%rsp),%rdi
   3aced:	mov    $0x1,%ecx
   3acf2:	mov    %r11,%r8
   3acf5:	call   *0x10e655(%rip)        # 149350 <_DYNAMIC+0x5e8>
   3acfb:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3ad04:	jne    3afb5 <meshopt_peers::run+0x1295>
   3ad0a:	movzbl 0x758(%rsp),%eax
   3ad12:	mov    %al,0x320(%rsp)
   3ad19:	lea    -0x2994d(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3ad20:	lea    0x106861(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3ad27:	lea    0x106482(%rip),%r8        # 1411b0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x380>
   3ad2e:	lea    0x320(%rsp),%rdx
   3ad36:	mov    $0x2b,%esi
   3ad3b:	call   *0x10e2f7(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3ad41:	mov    $0x616d6572,%edx
   3ad46:	xor    (%rsi),%edx
   3ad48:	movzbl 0x4(%rsi),%esi
   3ad4c:	xor    $0x70,%esi
   3ad4f:	or     %edx,%esi
   3ad51:	je     3b6d2 <meshopt_peers::run+0x19b2>
   3ad57:	lea    0x48(%rcx),%rax
   3ad5b:	mov    0x38(%rcx),%rbp
   3ad5f:	mov    0x40(%rcx),%r15
   3ad63:	mov    %rax,0xf8(%rsp)
   3ad6b:	mov    %rbp,0x190(%rsp)
   3ad73:	mov    %r15,0x198(%rsp)
   3ad7b:	mov    %r14,0x130(%rsp)
   3ad83:	movb   $0x0,0x2e0(%rsp)
   3ad8b:	movb   $0x0,0x365(%r11)
   3ad93:	movl   $0x0,0x361(%r11)
   3ad9e:	pxor   %xmm0,%xmm0
   3ada2:	movdqu %xmm0,0x350(%r11)
   3adab:	mov    0x348(%r11),%rax
   3adb2:	mov    %rax,0x250(%rsp)
   3adba:	mov    %rax,0x258(%rsp)
   3adc2:	lea    0x190(%rsp),%rcx
   3adca:	mov    %rcx,0x750(%rsp)
   3add2:	lea    0x130(%rsp),%rcx
   3adda:	mov    %rcx,0x758(%rsp)
   3ade2:	lea    0x250(%rsp),%rcx
   3adea:	mov    %rcx,0x760(%rsp)
   3adf2:	mov    %r11,0x768(%rsp)
   3adfa:	lea    0x2e0(%rsp),%rcx
   3ae02:	mov    %rcx,0x770(%rsp)
   3ae0a:	mov    %r13,0x778(%rsp)
   3ae12:	mov    %r14,0x780(%rsp)
   3ae1a:	mov    %r13,0x788(%rsp)
   3ae22:	mov    %r14,0x790(%rsp)
   3ae2a:	lea    0xf8(%rsp),%rcx
   3ae32:	mov    %rcx,0x798(%rsp)
   3ae3a:	mov    %rax,%rcx
   3ae3d:	sub    %r15,%rcx
   3ae40:	jae    3aec8 <meshopt_peers::run+0x11a8>
   3ae46:	mov    %rbx,0x8(%rsp)
   3ae4b:	shl    $0x2,%r15
   3ae4f:	xor    %ebx,%ebx
   3ae51:	lea    0x250(%rsp),%rdi
   3ae59:	mov    0x10e4d0(%rip),%rcx        # 149330 <_DYNAMIC+0x5c8>
   3ae60:	cmp    %rbx,%r15
   3ae63:	je     3b30d <meshopt_peers::run+0x15ed>
   3ae69:	mov    0x0(%rbp,%rbx,1),%r12d
   3ae6e:	test   %rax,%rax
   3ae71:	je     3ae89 <meshopt_peers::run+0x1169>
   3ae73:	dec    %rax
   3ae76:	mov    %rax,0x250(%rsp)
   3ae7e:	add    $0x4,%rbx
   3ae82:	cmp    %r14d,%r12d
   3ae85:	jb     3ae60 <meshopt_peers::run+0x1140>
   3ae87:	jmp    3aebc <meshopt_peers::run+0x119c>
   3ae89:	mov    %r11,0x10(%rsp)
   3ae8e:	mov    $0x1,%esi
   3ae93:	call   *%rcx
   3ae95:	cmp    $0xff,%al
   3ae97:	jne    3d2fc <meshopt_peers::run+0x35dc>
   3ae9d:	xor    %eax,%eax
   3ae9f:	mov    0x10(%rsp),%r11
   3aea4:	lea    0x250(%rsp),%rdi
   3aeac:	mov    0x10e47d(%rip),%rcx        # 149330 <_DYNAMIC+0x5c8>
   3aeb3:	add    $0x4,%rbx
   3aeb7:	cmp    %r14d,%r12d
   3aeba:	jb     3ae60 <meshopt_peers::run+0x1140>
   3aebc:	mov    $0x1,%al
   3aebe:	mov    0x8(%rsp),%rbx
   3aec3:	jmp    3ddaf <meshopt_peers::run+0x408f>
   3aec8:	test   %r15,%r15
   3aecb:	je     3dcc5 <meshopt_peers::run+0x3fa5>
   3aed1:	mov    0x0(%rbp),%edi
   3aed4:	cmp    $0x1,%r15
   3aed8:	je     3dcba <meshopt_peers::run+0x3f9a>
   3aede:	movabs $0x3fffffffffffffff,%r8
   3aee8:	lea    (%r15,%r8,1),%rsi
   3aeec:	mov    %rsi,%rdx
   3aeef:	and    %r8,%rdx
   3aef2:	cmp    $0x8,%rdx
   3aef6:	jae    3b875 <meshopt_peers::run+0x1b55>
   3aefc:	xor    %esi,%esi
   3aefe:	jmp    3dca4 <meshopt_peers::run+0x3f84>
   3af03:	mov    0x38(%rcx),%rsi
   3af07:	mov    0x40(%rcx),%rdx
   3af0b:	lea    0x750(%rsp),%rdi
   3af13:	lea    0x6a8(%rsp),%rcx
   3af1b:	movd   -0x2a4d3(%rip),%xmm0        # 10a50 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x144>
   3af23:	mov    %r11,%r8
   3af26:	call   *0x10e42c(%rip)        # 149358 <_DYNAMIC+0x5f0>
   3af2c:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3af35:	jne    3b27e <meshopt_peers::run+0x155e>
   3af3b:	movzbl 0x758(%rsp),%eax
   3af43:	mov    %al,0x320(%rsp)
   3af4a:	lea    -0x29b7e(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3af51:	lea    0x106630(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3af58:	lea    0x1062f9(%rip),%r8        # 141258 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x428>
   3af5f:	lea    0x320(%rsp),%rdx
   3af67:	mov    $0x2b,%esi
   3af6c:	call   *0x10e0c6(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3af72:	lea    0x0(,%r14,4),%rax
   3af7a:	lea    (%rax,%rax,2),%rdx
   3af7e:	lea    0x750(%rsp),%rdi
   3af86:	mov    $0xc,%r8d
   3af8c:	mov    %r13,%rsi
   3af8f:	mov    %r14,%rcx
   3af92:	mov    $0x1,%r9d
   3af98:	push   %r11
   3af9a:	push   $0x2
   3af9c:	call   *0x10e3be(%rip)        # 149360 <_DYNAMIC+0x5f8>
   3afa2:	add    $0x10,%rsp
   3afa6:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3afaf:	je     3d183 <meshopt_peers::run+0x3463>
   3afb5:	mov    0x760(%rsp),%rax
   3afbd:	mov    %rax,0x18(%rbx)
   3afc1:	movdqu 0x750(%rsp),%xmm0
   3afca:	movdqu %xmm0,0x8(%rbx)
   3afcf:	movl   $0x2,(%rbx)
   3afd5:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3afda:	mov    %r14,%rax
   3afdd:	shr    $0x20,%rax
   3afe1:	jne    3d6d9 <meshopt_peers::run+0x39b9>
   3afe7:	mov    0x38(%rcx),%rsi
   3afeb:	mov    0x40(%rcx),%rdx
   3afef:	lea    0x0(,%r14,4),%rax
   3aff7:	lea    (%rax,%rax,2),%rax
   3affb:	mov    %r13,0x320(%rsp)
   3b003:	mov    %rax,0x328(%rsp)
   3b00b:	mov    %r14,0x330(%rsp)
   3b013:	movq   $0xc,0x338(%rsp)
   3b01f:	movq   $0xc,0x340(%rsp)
   3b02b:	movq   $0x0,0x348(%rsp)
   3b037:	lea    0x750(%rsp),%rdi
   3b03f:	lea    0x320(%rsp),%rcx
   3b047:	mov    %r11,%r8
   3b04a:	call   *0x10e318(%rip)        # 149368 <_DYNAMIC+0x600>
   3b050:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b059:	je     3d1f1 <meshopt_peers::run+0x34d1>
   3b05f:	mov    0x780(%rsp),%rax
   3b067:	mov    %rax,0x350(%rsp)
   3b06f:	movups 0x750(%rsp),%xmm0
   3b077:	movups 0x760(%rsp),%xmm1
   3b07f:	movups 0x770(%rsp),%xmm2
   3b087:	movaps %xmm2,0x340(%rsp)
   3b08f:	movaps %xmm1,0x330(%rsp)
   3b097:	movaps %xmm0,0x320(%rsp)
   3b09f:	mov    0x330(%rsp),%rax
   3b0a7:	mov    %rax,0x18(%rbx)
   3b0ab:	movups %xmm0,0x8(%rbx)
   3b0af:	movdqu 0x338(%rsp),%xmm0
   3b0b8:	movdqu %xmm0,0x20(%rbx)
   3b0bd:	mov    0x348(%rsp),%rax
   3b0c5:	mov    %rax,0x30(%rbx)
   3b0c9:	movl   $0x5,(%rbx)
   3b0cf:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3b0d4:	mov    0x40(%rcx),%rbp
   3b0d8:	mov    $0x4,%r15d
   3b0de:	mov    $0x4,%r12d
   3b0e4:	test   %rbp,%rbp
   3b0e7:	mov    %rbp,0x8(%rsp)
   3b0ec:	je     3b133 <meshopt_peers::run+0x1413>
   3b0ee:	mov    %r11,0x10(%rsp)
   3b0f3:	mov    0x38(%rcx),%r15
   3b0f7:	lea    0x0(,%rbp,4),%rbp
   3b0ff:	mov    %rbp,%rdi
   3b102:	call   *0x10de58(%rip)        # 148f60 <malloc@GLIBC_2.2.5>
   3b108:	test   %rax,%rax
   3b10b:	je     3d74d <meshopt_peers::run+0x3a2d>
   3b111:	mov    %rax,%r12
   3b114:	mov    %rax,%rdi
   3b117:	mov    %r15,%rsi
   3b11a:	mov    %rbp,%rdx
   3b11d:	call   *0x10de5d(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   3b123:	mov    0x10(%rsp),%r11
   3b128:	mov    $0x4,%r15d
   3b12e:	mov    0x8(%rsp),%rbp
   3b133:	test   %r14,%r14
   3b136:	je     3b17d <meshopt_peers::run+0x145d>
   3b138:	mov    %r12,0x10(%rsp)
   3b13d:	mov    %r11,%r12
   3b140:	lea    0x0(,%r14,4),%rax
   3b148:	lea    (%rax,%rax,2),%rbp
   3b14c:	mov    %rbp,%rdi
   3b14f:	call   *0x10de0b(%rip)        # 148f60 <malloc@GLIBC_2.2.5>
   3b155:	test   %rax,%rax
   3b158:	je     3d78b <meshopt_peers::run+0x3a6b>
   3b15e:	mov    %rax,%r15
   3b161:	mov    %rax,%rdi
   3b164:	mov    %r13,%rsi
   3b167:	mov    %rbp,%rdx
   3b16a:	call   *0x10de10(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   3b170:	mov    %r12,%r11
   3b173:	mov    0x10(%rsp),%r12
   3b178:	mov    0x8(%rsp),%rbp
   3b17d:	movb   $0x0,0x750(%rsp)
   3b185:	mov    %r15,0x758(%rsp)
   3b18d:	mov    %r14,0x760(%rsp)
   3b195:	mov    %r14,0x768(%rsp)
   3b19d:	movq   $0x3,0x770(%rsp)
   3b1a9:	movq   $0x0,0x778(%rsp)
   3b1b5:	sub    $0x30,%rsp
   3b1b9:	mov    %r11,0x20(%rsp)
   3b1be:	lea    0x270(%rsp),%rax
   3b1c6:	mov    %rax,0x18(%rsp)
   3b1cb:	pxor   %xmm0,%xmm0
   3b1cf:	movdqu %xmm0,(%rsp)
   3b1d4:	lea    0x350(%rsp),%rdi
   3b1dc:	lea    0x780(%rsp),%rcx
   3b1e4:	mov    $0x4,%r9d
   3b1ea:	mov    %r12,%rsi
   3b1ed:	mov    %rbp,%rdx
   3b1f0:	xor    %r8d,%r8d
   3b1f3:	call   *0x10e177(%rip)        # 149370 <_DYNAMIC+0x608>
   3b1f9:	add    $0x30,%rsp
   3b1fd:	cmpb   $0x1,0x320(%rsp)
   3b205:	je     3d228 <meshopt_peers::run+0x3508>
   3b20b:	mov    0x328(%rsp),%rax
   3b213:	movd   0x330(%rsp),%xmm0
   3b21c:	cmp    %rbp,%rax
   3b21f:	cmovae %rbp,%rax
   3b223:	mov    %rbp,0x8(%rbx)
   3b227:	mov    %r12,0x10(%rbx)
   3b22b:	mov    %rax,0x18(%rbx)
   3b22f:	mov    %r14,0x20(%rbx)
   3b233:	mov    %r15,0x28(%rbx)
   3b237:	mov    %r14,0x30(%rbx)
   3b23b:	movd   %xmm0,0x4(%rbx)
   3b240:	movl   $0x1,(%rbx)
   3b246:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3b24b:	mov    0x38(%rcx),%rsi
   3b24f:	mov    0x40(%rcx),%rdx
   3b253:	mov    0x8(%rcx),%rax
   3b257:	mov    0x10(%rcx),%r8
   3b25b:	lea    0x750(%rsp),%rdi
   3b263:	mov    %rax,%rcx
   3b266:	mov    %r11,%r9
   3b269:	call   *0x10e109(%rip)        # 149378 <_DYNAMIC+0x610>
   3b26f:	cmpq   $0xffffffffffffffff,0x750(%rsp)
   3b278:	je     3d1ba <meshopt_peers::run+0x349a>
   3b27e:	mov    0x760(%rsp),%rax
   3b286:	mov    %rax,0x18(%rbx)
   3b28a:	movdqu 0x750(%rsp),%xmm0
   3b293:	movdqu %xmm0,0x8(%rbx)
   3b298:	movl   $0x3,(%rbx)
   3b29e:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3b2a3:	mov    0x38(%rcx),%rsi
   3b2a7:	mov    0x40(%rcx),%rdx
   3b2ab:	lea    0x190(%rsp),%rdi
   3b2b3:	lea    0x320(%rsp),%rcx
   3b2bb:	mov    %r11,%r8
   3b2be:	call   *0x10e0bc(%rip)        # 149380 <_DYNAMIC+0x618>
   3b2c4:	mov    0x190(%rsp),%rax
   3b2cc:	cmp    $0xffffffffffffffff,%rax
   3b2d0:	jne    3a1d4 <meshopt_peers::run+0x4b4>
   3b2d6:	movzbl 0x198(%rsp),%eax
   3b2de:	mov    %al,0x750(%rsp)
   3b2e5:	lea    -0x29f19(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3b2ec:	lea    0x106295(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3b2f3:	lea    0x105df6(%rip),%r8        # 1410f0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2c0>
   3b2fa:	lea    0x750(%rsp),%rdx
   3b302:	mov    $0x2b,%esi
   3b307:	call   *0x10dd2b(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3b30d:	mov    0x130(%rsp),%rax
   3b315:	mov    0x8(%rsp),%rbx
   3b31a:	jmp    3dcd0 <meshopt_peers::run+0x3fb0>
   3b31f:	mov    %r14,%rax
   3b322:	shr    $0x20,%rax
   3b326:	jne    3d75b <meshopt_peers::run+0x3a3b>
   3b32c:	lea    0x0(,%r14,4),%rax
   3b334:	lea    (%rax,%rax,2),%rax
   3b338:	mov    %r13,0x750(%rsp)
   3b340:	mov    %rax,0x758(%rsp)
   3b348:	mov    %r14,0x760(%rsp)
   3b350:	movq   $0xc,0x768(%rsp)
   3b35c:	movq   $0xc,0x770(%rsp)
   3b368:	movq   $0x0,0x778(%rsp)
   3b374:	mov    0x8(%rcx),%rdx
   3b378:	mov    0x10(%rcx),%rcx
   3b37c:	lea    0x320(%rsp),%rdi
   3b384:	lea    0x750(%rsp),%rsi
   3b38c:	mov    %r11,%r8
   3b38f:	call   *0x10dff3(%rip)        # 149388 <_DYNAMIC+0x620>
   3b395:	cmpq   $0xffffffffffffffff,0x320(%rsp)
   3b39e:	je     3d30b <meshopt_peers::run+0x35eb>
   3b3a4:	mov    0x330(%rsp),%rax
   3b3ac:	mov    %rax,0x18(%rbx)
   3b3b0:	movdqu 0x320(%rsp),%xmm0
   3b3b9:	jmp    3afca <meshopt_peers::run+0x12aa>
   3b3be:	mov    %rax,%r14
   3b3c1:	mov    $0x4,%r12d
   3b3c7:	movq   $0x0,0x210(%rsp)
   3b3d3:	movq   $0x4,0x218(%rsp)
   3b3df:	movq   $0x0,0x220(%rsp)
   3b3eb:	movl   $0x0,0xbc(%rsp)
   3b3f6:	movq   $0x80,0x190(%rsp)
   3b402:	movq   $0x2a,0x198(%rsp)
   3b40e:	movq   $0x80,0x1a0(%rsp)
   3b41a:	movq   $0x10,0x1a8(%rsp)
   3b426:	movaps -0x2c74d(%rip),%xmm0        # ece0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x690>
   3b42d:	movups %xmm0,0x1b0(%rsp)
   3b435:	movaps -0x2c69c(%rip),%xmm0        # eda0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3b43c:	movups %xmm0,0x1c0(%rsp)
   3b444:	movabs $0x100010100000101,%rax
   3b44e:	mov    %rax,0x1d0(%rsp)
   3b456:	movl   $0x0,0x1d8(%rsp)
   3b461:	movb   $0x1,0x1dc(%rsp)
   3b469:	mov    0x38(%rcx),%r13
   3b46d:	mov    0x40(%rcx),%r15
   3b471:	mov    %r13,0x298(%rsp)
   3b479:	mov    %r15,0x2a0(%rsp)
   3b481:	mov    %r12,0x10(%rsp)
   3b486:	mov    %r12,0x2a8(%rsp)
   3b48e:	mov    %r14,0x2b0(%rsp)
   3b496:	movb   $0x2,0x250(%rsp)
   3b49e:	movq   $0x4,0x2b8(%rsp)
   3b4aa:	xorps  %xmm0,%xmm0
   3b4ad:	movups %xmm0,0x2c0(%rsp)
   3b4b5:	movl   $0x0,0x2d8(%rsp)
   3b4c0:	movb   $0x0,0x365(%r11)
   3b4c8:	movl   $0x0,0x361(%r11)
   3b4d3:	movups %xmm0,0x350(%r11)
   3b4db:	mov    0x348(%r11),%rbx
   3b4e2:	mov    %r11,%r12
   3b4e5:	mov    %r11,%rdi
   3b4e8:	xor    %esi,%esi
   3b4ea:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b4ef:	mov    %eax,%ebp
   3b4f1:	cmp    $0xff,%al
   3b4f3:	jne    3d8ca <meshopt_peers::run+0x3baa>
   3b4f9:	mov    %r12,0x750(%rsp)
   3b501:	mov    %rbx,0x758(%rsp)
   3b509:	mov    %rbx,0x760(%rsp)
   3b511:	movq   $0x0,0x768(%rsp)
   3b51d:	movl   $0x0,0x770(%rsp)
   3b528:	movl   $0x0,0x974(%rsp)
   3b533:	movb   $0x1,0x1978(%rsp)
   3b53b:	lea    0x250(%rsp),%rdi
   3b543:	lea    0x190(%rsp),%rsi
   3b54b:	lea    0x750(%rsp),%rdx
   3b553:	call   *0x10de37(%rip)        # 149390 <_DYNAMIC+0x628>
   3b559:	mov    0x8(%rsp),%rbx
   3b55e:	mov    %eax,%ebp
   3b560:	cmp    $0xff,%al
   3b562:	jne    3d8ab <meshopt_peers::run+0x3b8b>
   3b568:	movzbl 0x1978(%rsp),%r12d
   3b571:	movb   $0x2,0x320(%rsp)
   3b579:	mov    0x10(%rsp),%rax
   3b57e:	mov    %rax,0x328(%rsp)
   3b586:	mov    %r14,0x330(%rsp)
   3b58e:	movq   $0x0,0x358(%rsp)
   3b59a:	lea    0x130(%rsp),%rdi
   3b5a2:	lea    0x320(%rsp),%rsi
   3b5aa:	lea    0x750(%rsp),%rdx
   3b5b2:	call   *0x10dde0(%rip)        # 149398 <_DYNAMIC+0x630>
   3b5b8:	mov    0x130(%rsp),%rax
   3b5c0:	movzbl 0x138(%rsp),%ebp
   3b5c8:	mov    %rax,0x90(%rsp)
   3b5d0:	cmp    $0xffffffffffffffff,%rax
   3b5d4:	je     3b666 <meshopt_peers::run+0x1946>
   3b5da:	movzbl %bpl,%eax
   3b5de:	movzbl 0x13f(%rsp),%ecx
   3b5e6:	shl    $0x10,%ecx
   3b5e9:	movzwl 0x13d(%rsp),%edx
   3b5f1:	or     %ecx,%edx
   3b5f3:	mov    0x139(%rsp),%ecx
   3b5fa:	shl    $0x28,%rdx
   3b5fe:	shl    $0x8,%rcx
   3b602:	or     %rdx,%rcx
   3b605:	or     %rax,%rcx
   3b608:	mov    %rcx,0xf0(%rsp)
   3b610:	mov    0x768(%rsp),%rsi
   3b618:	add    %r14,%rsi
   3b61b:	jb     3d17b <meshopt_peers::run+0x345b>
   3b621:	mov    0x140(%rsp),%rax
   3b629:	mov    %rax,0x60(%rsp)
   3b62e:	mov    %rsi,0x768(%rsp)
   3b636:	mov    0x750(%rsp),%rdi
   3b63e:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b643:	mov    %eax,%ebp
   3b645:	cmp    $0xff,%al
   3b647:	je     3b814 <meshopt_peers::run+0x1af4>
   3b64d:	cmpq   $0x0,0x90(%rsp)
   3b656:	je     3b666 <meshopt_peers::run+0x1946>
   3b658:	mov    0xf0(%rsp),%rdi
   3b660:	call   *0x10d90a(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3b666:	mov    0x750(%rsp),%rax
   3b66e:	mov    0x760(%rsp),%rcx
   3b676:	sub    0x758(%rsp),%rcx
   3b67e:	mov    %rcx,0x358(%rax)
   3b685:	cmp    $0xff,%bpl
   3b689:	jne    3d8ca <meshopt_peers::run+0x3baa>
   3b68f:	mov    0x220(%rsp),%rax
   3b697:	mov    %rax,0x18(%rbx)
   3b69b:	movdqu 0x210(%rsp),%xmm0
   3b6a4:	movdqu %xmm0,0x8(%rbx)
   3b6a9:	movl   $0x3,(%rbx)
   3b6af:	test   %r14,%r14
   3b6b2:	je     3de14 <meshopt_peers::run+0x40f4>
   3b6b8:	mov    0x10(%rsp),%rdi
   3b6bd:	call   *0x10d8ad(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3b6c3:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3b6c8:	mov    $0x4,%eax
   3b6cd:	jmp    3b7b3 <meshopt_peers::run+0x1a93>
   3b6d2:	mov    0x38(%rcx),%rsi
   3b6d6:	mov    0x40(%rcx),%rdx
   3b6da:	mov    %r13,0x750(%rsp)
   3b6e2:	mov    %rax,0x758(%rsp)
   3b6ea:	mov    %r14,0x760(%rsp)
   3b6f2:	movq   $0xc,0x768(%rsp)
   3b6fe:	movq   $0xc,0x770(%rsp)
   3b70a:	movq   $0x0,0x778(%rsp)
   3b716:	lea    0x320(%rsp),%rdi
   3b71e:	lea    0x750(%rsp),%rcx
   3b726:	mov    $0x1,%r8d
   3b72c:	mov    %r11,%r9
   3b72f:	call   *0x10dc6b(%rip)        # 1493a0 <_DYNAMIC+0x638>
   3b735:	jmp    3ddd9 <meshopt_peers::run+0x40b9>
   3b73a:	mov    %r12,%rsi
   3b73d:	and    $0xfffffffffffffffc,%rsi
   3b741:	mov    $0x30,%edi
   3b746:	xor    %edx,%edx
   3b748:	nopl   0x0(%rax,%rax,1)
   3b750:	movups -0x30(%r14,%rdi,1),%xmm0
   3b756:	movups %xmm0,-0x30(%rax,%rdi,1)
   3b75b:	movups -0x20(%r14,%rdi,1),%xmm0
   3b761:	movups %xmm0,-0x20(%rax,%rdi,1)
   3b766:	movups -0x10(%r14,%rdi,1),%xmm0
   3b76c:	movups %xmm0,-0x10(%rax,%rdi,1)
   3b771:	movdqu (%r14,%rdi,1),%xmm0
   3b777:	movdqu %xmm0,(%rax,%rdi,1)
   3b77c:	add    $0x4,%rdx
   3b780:	add    $0x40,%rdi
   3b784:	cmp    %rdx,%rsi
   3b787:	jne    3b750 <meshopt_peers::run+0x1a30>
   3b789:	test   %rcx,%rcx
   3b78c:	je     3b7b3 <meshopt_peers::run+0x1a93>
   3b78e:	shl    $0x4,%rdx
   3b792:	mov    %rax,%rsi
   3b795:	add    %rdx,%rsi
   3b798:	add    %r14,%rdx
   3b79b:	shl    $0x4,%ecx
   3b79e:	xor    %edi,%edi
   3b7a0:	movdqu (%rdx,%rdi,1),%xmm0
   3b7a5:	movdqu %xmm0,(%rsi,%rdi,1)
   3b7aa:	add    $0x10,%rdi
   3b7ae:	cmp    %rdi,%rcx
   3b7b1:	jne    3b7a0 <meshopt_peers::run+0x1a80>
   3b7b3:	mov    0x348(%rsp),%rcx
   3b7bb:	mov    %rcx,0x30(%rbx)
   3b7bf:	movups 0x338(%rsp),%xmm0
   3b7c7:	movups %xmm0,0x20(%rbx)
   3b7cb:	movdqa 0x350(%rsp),%xmm0
   3b7d4:	movdqu %xmm0,0x38(%rbx)
   3b7d9:	mov    0x360(%rsp),%rcx
   3b7e1:	mov    %rcx,0x48(%rbx)
   3b7e5:	mov    %r12,0x8(%rbx)
   3b7e9:	mov    %rax,0x10(%rbx)
   3b7ed:	mov    %r12,0x18(%rbx)
   3b7f1:	movl   $0x6,(%rbx)
   3b7f7:	cmpq   $0x0,0x320(%rsp)
   3b800:	je     3de14 <meshopt_peers::run+0x40f4>
   3b806:	mov    %r14,%rdi
   3b809:	call   *0x10d761(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3b80f:	jmp    3de14 <meshopt_peers::run+0x40f4>
   3b814:	mov    $0x1,%eax
   3b819:	mov    %rax,0xb0(%rsp)
   3b821:	test   %r14,%r14
   3b824:	jne    3bdd0 <meshopt_peers::run+0x20b0>
   3b82a:	mov    0x750(%rsp),%rdi
   3b832:	mov    0x768(%rsp),%rsi
   3b83a:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3b83f:	mov    %eax,%ebp
   3b841:	mov    $0x1,%al
   3b843:	cmp    $0xff,%bpl
   3b847:	je     3b957 <meshopt_peers::run+0x1c37>
   3b84d:	test   %r14,%r14
   3b850:	je     3d3a2 <meshopt_peers::run+0x3682>
   3b856:	mov    0xb0(%rsp),%rdi
   3b85e:	mov    %rax,%r15
   3b861:	call   *0x10d709(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3b867:	test   %r15b,%r15b
   3b86a:	jne    3b64d <meshopt_peers::run+0x192d>
   3b870:	jmp    3b666 <meshopt_peers::run+0x1946>
   3b875:	add    $0xfffffffffffffff9,%r8
   3b879:	and    %r8,%rsi
   3b87c:	movd   %edi,%xmm0
   3b880:	pshufd $0x0,%xmm0,%xmm1
   3b885:	xor    %edi,%edi
   3b887:	movdqa -0x2cb1f(%rip),%xmm0        # ed70 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x720>
   3b88f:	movdqa %xmm1,%xmm2
   3b893:	movdqu 0x4(%rbp,%rdi,4),%xmm3
   3b899:	movdqu 0x14(%rbp,%rdi,4),%xmm4
   3b89f:	movdqa %xmm1,%xmm5
   3b8a3:	pxor   %xmm0,%xmm1
   3b8a7:	movdqa %xmm3,%xmm6
   3b8ab:	pxor   %xmm0,%xmm6
   3b8af:	pcmpgtd %xmm6,%xmm1
   3b8b3:	pand   %xmm1,%xmm5
   3b8b7:	pandn  %xmm3,%xmm1
   3b8bb:	por    %xmm5,%xmm1
   3b8bf:	movdqa %xmm2,%xmm3
   3b8c3:	pxor   %xmm0,%xmm2
   3b8c7:	movdqa %xmm4,%xmm5
   3b8cb:	pxor   %xmm0,%xmm5
   3b8cf:	pcmpgtd %xmm5,%xmm2
   3b8d3:	pand   %xmm2,%xmm3
   3b8d7:	pandn  %xmm4,%xmm2
   3b8db:	por    %xmm3,%xmm2
   3b8df:	add    $0x8,%rdi
   3b8e3:	cmp    %rdi,%rsi
   3b8e6:	jne    3b893 <meshopt_peers::run+0x1b73>
   3b8e8:	movdqa %xmm2,%xmm3
   3b8ec:	pxor   %xmm0,%xmm3
   3b8f0:	movdqa %xmm1,%xmm4
   3b8f4:	pxor   %xmm0,%xmm4
   3b8f8:	pcmpgtd %xmm3,%xmm4
   3b8fc:	pand   %xmm4,%xmm1
   3b900:	pandn  %xmm2,%xmm4
   3b904:	por    %xmm1,%xmm4
   3b908:	pshufd $0xee,%xmm4,%xmm1
   3b90d:	movdqa %xmm4,%xmm2
   3b911:	pxor   %xmm0,%xmm2
   3b915:	movdqa %xmm1,%xmm3
   3b919:	pxor   %xmm0,%xmm3
   3b91d:	pcmpgtd %xmm3,%xmm2
   3b921:	pand   %xmm2,%xmm4
   3b925:	pandn  %xmm1,%xmm2
   3b929:	por    %xmm4,%xmm2
   3b92d:	pshufd $0x55,%xmm2,%xmm1
   3b932:	movdqa %xmm2,%xmm3
   3b936:	pxor   %xmm0,%xmm3
   3b93a:	pxor   %xmm1,%xmm0
   3b93e:	pcmpgtd %xmm0,%xmm3
   3b942:	pand   %xmm3,%xmm2
   3b946:	pandn  %xmm1,%xmm3
   3b94a:	por    %xmm2,%xmm3
   3b94e:	movd   %xmm3,%edi
   3b952:	jmp    3dcb5 <meshopt_peers::run+0x3f95>
   3b957:	movb   $0x2,0x130(%rsp)
   3b95f:	mov    0x10(%rsp),%rax
   3b964:	mov    %rax,0x138(%rsp)
   3b96c:	mov    %r14,0x140(%rsp)
   3b974:	movq   $0x0,0x168(%rsp)
   3b980:	movups 0x1d0(%rsp),%xmm0
   3b988:	movaps %xmm0,0x360(%rsp)
   3b990:	movups 0x190(%rsp),%xmm0
   3b998:	movups 0x1a0(%rsp),%xmm1
   3b9a0:	movups 0x1b0(%rsp),%xmm2
   3b9a8:	movups 0x1c0(%rsp),%xmm3
   3b9b0:	movaps %xmm3,0x350(%rsp)
   3b9b8:	movaps %xmm2,0x340(%rsp)
   3b9c0:	movaps %xmm1,0x330(%rsp)
   3b9c8:	movaps %xmm0,0x320(%rsp)
   3b9d0:	movzbl %r12b,%r10d
   3b9d4:	lea    0xf8(%rsp),%rdi
   3b9dc:	lea    0x130(%rsp),%rcx
   3b9e4:	lea    0x320(%rsp),%r8
   3b9ec:	mov    %r13,%rsi
   3b9ef:	mov    %r15,%rdx
   3b9f2:	mov    $0xffffffff,%r9d
   3b9f8:	lea    0x750(%rsp),%rax
   3ba00:	push   %rax
   3ba01:	mov    %r10,0x240(%rsp)
   3ba09:	push   %r10
   3ba0b:	call   *0x10d997(%rip)        # 1493a8 <_DYNAMIC+0x640>
   3ba11:	add    $0x10,%rsp
   3ba15:	mov    0xf8(%rsp),%rdi
   3ba1d:	movzbl 0x100(%rsp),%ebp
   3ba25:	cmp    $0xffffffffffffffff,%rdi
   3ba29:	je     3bd7f <meshopt_peers::run+0x205f>
   3ba2f:	movzbl %bpl,%eax
   3ba33:	movzbl 0x107(%rsp),%ecx
   3ba3b:	shl    $0x10,%ecx
   3ba3e:	movzwl 0x105(%rsp),%edx
   3ba46:	or     %ecx,%edx
   3ba48:	mov    0x101(%rsp),%ecx
   3ba4f:	mov    0x108(%rsp),%r9
   3ba57:	mov    0x110(%rsp),%r8
   3ba5f:	mov    0x118(%rsp),%rsi
   3ba67:	mov    %rsi,0x28(%rsp)
   3ba6c:	shl    $0x28,%rdx
   3ba70:	shl    $0x8,%rcx
   3ba74:	or     %rdx,%rcx
   3ba77:	or     %rax,%rcx
   3ba7a:	mov    %rcx,0x48(%rsp)
   3ba7f:	test   %r9,%r9
   3ba82:	je     3bd86 <meshopt_peers::run+0x2066>
   3ba88:	mov    %r8,0x80(%rsp)
   3ba90:	mov    %rdi,0x88(%rsp)
   3ba98:	mov    0x120(%rsp),%rax
   3baa0:	mov    %rax,0x70(%rsp)
   3baa5:	mov    %r9,0x188(%rsp)
   3baad:	mov    %r9,%rax
   3bab0:	shl    $0x4,%rax
   3bab4:	lea    (%rax,%rax,2),%rax
   3bab8:	mov    %rax,0x40(%rsp)
   3babd:	xor    %r12d,%r12d
   3bac0:	mov    0x750(%rsp),%r15
   3bac8:	mov    0x758(%rsp),%rbp
   3bad0:	mov    0x760(%rsp),%rax
   3bad8:	sub    %rbp,%rax
   3badb:	mov    %rax,0x358(%r15)
   3bae2:	mov    %r15,%rdi
   3bae5:	call   *0x10d835(%rip)        # 149320 <_DYNAMIC+0x5b8>
   3baeb:	mov    0x340(%r15),%rax
   3baf2:	mov    0x768(%rsp),%r13
   3bafa:	sub    %r13,%rax
   3bafd:	jb     3be0f <meshopt_peers::run+0x20ef>
   3bb03:	mov    0x348(%r15),%rcx
   3bb0a:	sub    0x358(%r15),%rcx
   3bb11:	mov    %rax,0x660(%rsp)
   3bb19:	mov    %rcx,0x668(%rsp)
   3bb21:	movb   $0x0,0x680(%rsp)
   3bb29:	movb   $0x0,0x685(%rsp)
   3bb31:	movl   $0x0,0x681(%rsp)
   3bb3c:	movb   $0xff,0x688(%rsp)
   3bb44:	movw   $0x0,0x686(%rsp)
   3bb4e:	lea    0x670(%rsp),%rax
   3bb56:	xorps  %xmm0,%xmm0
   3bb59:	movups %xmm0,(%rax)
   3bb5c:	movq   $0xffffffffffffffff,0x398(%rsp)
   3bb68:	movq   $0xffffffffffffffff,0x5a0(%rsp)
   3bb74:	movq   $0x0,0x320(%rsp)
   3bb80:	movq   $0x4,0x328(%rsp)
   3bb8c:	movups %xmm0,-0x340(%rax)
   3bb93:	movq   $0x4,0x340(%rsp)
   3bb9f:	movups %xmm0,-0x328(%rax)
   3bba6:	movq   $0x4,0x358(%rsp)
   3bbb2:	movups %xmm0,-0x310(%rax)
   3bbb9:	movq   $0x1,0x370(%rsp)
   3bbc5:	movups %xmm0,-0x2f8(%rax)
   3bbcc:	movq   $0x2,0x388(%rsp)
   3bbd8:	movq   $0x0,0x390(%rsp)
   3bbe4:	mov    0x48(%rsp),%rax
   3bbe9:	mov    0x8(%rax,%r12,1),%rdi
   3bbee:	mov    0x10(%rax,%r12,1),%rdx
   3bbf3:	mov    %rdx,%rsi
   3bbf6:	add    %rdi,%rsi
   3bbf9:	jb     3d2e5 <meshopt_peers::run+0x35c5>
   3bbff:	cmp    0x70(%rsp),%rsi
   3bc04:	ja     3d2e5 <meshopt_peers::run+0x35c5>
   3bc0a:	mov    0x28(%rsp),%rax
   3bc0f:	lea    (%rax,%rdi,4),%rsi
   3bc13:	movb   $0x2,0x130(%rsp)
   3bc1b:	mov    0x10(%rsp),%rax
   3bc20:	mov    %rax,0x138(%rsp)
   3bc28:	mov    %r14,0x140(%rsp)
   3bc30:	movq   $0x0,0x168(%rsp)
   3bc3c:	lea    0xf8(%rsp),%rdi
   3bc44:	lea    0x130(%rsp),%rcx
   3bc4c:	mov    0x238(%rsp),%r8
   3bc54:	lea    0x320(%rsp),%r9
   3bc5c:	call   *0x10d74e(%rip)        # 1493b0 <_DYNAMIC+0x648>
   3bc62:	mov    0x678(%rsp),%rsi
   3bc6a:	sub    %rsi,%rbp
   3bc6d:	jb     3bd62 <meshopt_peers::run+0x2042>
   3bc73:	mov    %rbp,0x758(%rsp)
   3bc7b:	add    0x670(%rsp),%r13
   3bc83:	jb     3d342 <meshopt_peers::run+0x3622>
   3bc89:	mov    %r15,%rdi
   3bc8c:	mov    %r13,%rsi
   3bc8f:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3bc94:	mov    %eax,%ebp
   3bc96:	cmp    $0xff,%al
   3bc98:	jne    3d345 <meshopt_peers::run+0x3625>
   3bc9e:	movzbl 0xf8(%rsp),%ebx
   3bca6:	movzbl 0xf9(%rsp),%ebp
   3bcae:	lea    0xfa(%rsp),%rcx
   3bcb6:	mov    (%rcx),%rax
   3bcb9:	mov    0x6(%rcx),%rcx
   3bcbd:	mov    %rax,0x698(%rsp)
   3bcc5:	mov    %rcx,0x69e(%rsp)
   3bccd:	movss  0x108(%rsp),%xmm0
   3bcd6:	movaps %xmm0,0xc0(%rsp)
   3bcde:	lea    0x320(%rsp),%rdi
   3bce6:	call   36910 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3bceb:	cmp    $0x1,%bl
   3bcee:	mov    0x8(%rsp),%rbx
   3bcf3:	je     3d352 <meshopt_peers::run+0x3632>
   3bcf9:	lea    0x69a(%rsp),%rcx
   3bd01:	mov    0x8(%rcx),%eax
   3bd04:	lea    0x744(%rsp),%rdx
   3bd0c:	mov    %eax,0x8(%rdx)
   3bd0f:	mov    (%rcx),%rax
   3bd12:	mov    %rax,(%rdx)
   3bd15:	mov    0x8(%rdx),%eax
   3bd18:	mov    %eax,0x328(%rsp)
   3bd1f:	mov    (%rdx),%rcx
   3bd22:	mov    %rcx,0x320(%rsp)
   3bd2a:	mov    0x48(%rsp),%rdx
   3bd2f:	mov    %eax,0x24(%rdx,%r12,1)
   3bd34:	mov    %rcx,0x1c(%rdx,%r12,1)
   3bd39:	xorps  %xmm0,%xmm0
   3bd3c:	movaps 0xc0(%rsp),%xmm1
   3bd44:	movss  %xmm1,%xmm0
   3bd48:	movlps %xmm0,0x28(%rdx,%r12,1)
   3bd4e:	add    $0x30,%r12
   3bd52:	cmp    %r12,0x40(%rsp)
   3bd57:	jne    3bac0 <meshopt_peers::run+0x1da0>
   3bd5d:	jmp    3be65 <meshopt_peers::run+0x2145>
   3bd62:	lea    0x758(%rsp),%rdi
   3bd6a:	call   *0x10d5c0(%rip)        # 149330 <_DYNAMIC+0x5c8>
   3bd70:	mov    %eax,%ebp
   3bd72:	cmp    $0xff,%al
   3bd74:	je     3bc7b <meshopt_peers::run+0x1f5b>
   3bd7a:	jmp    3d345 <meshopt_peers::run+0x3625>
   3bd7f:	mov    $0x1,%al
   3bd81:	jmp    3b84d <meshopt_peers::run+0x1b2d>
   3bd86:	mov    %rdi,%r12
   3bd89:	mov    %r8,0xc0(%rsp)
   3bd91:	lea    0x750(%rsp),%rdi
   3bd99:	mov    0x90(%rsp),%rsi
   3bda1:	mov    0xf0(%rsp),%rdx
   3bda9:	call   35ce0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3bdae:	mov    $0x1,%r13b
   3bdb1:	cmp    $0xff,%al
   3bdb3:	je     3be17 <meshopt_peers::run+0x20f7>
   3bdb5:	mov    %eax,%ebp
   3bdb7:	movq   $0x0,0x60(%rsp)
   3bdc0:	mov    $0x1,%r15b
   3bdc3:	mov    0xc0(%rsp),%rcx
   3bdcb:	jmp    3d36f <meshopt_peers::run+0x364f>
   3bdd0:	lea    0x320(%rsp),%rdi
   3bdd8:	mov    $0x1,%edx
   3bddd:	mov    $0x1,%r8d
   3bde3:	mov    $0x1,%r9d
   3bde9:	xor    %esi,%esi
   3bdeb:	mov    %r14,%rbp
   3bdee:	mov    %r14,%rcx
   3bdf1:	call   423f0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3bdf6:	cmpl   $0x1,0x320(%rsp)
   3bdfe:	jne    3bf23 <meshopt_peers::run+0x2203>
   3be04:	mov    %rbp,%r14
   3be07:	mov    $0xa,%bpl
   3be0a:	jmp    3b64d <meshopt_peers::run+0x192d>
   3be0f:	mov    $0x9,%bpl
   3be12:	jmp    3d352 <meshopt_peers::run+0x3632>
   3be17:	mov    %r12,0x38(%rsp)
   3be1c:	mov    0x768(%rsp),%rax
   3be24:	mov    %r14,%r12
   3be27:	sub    %r14,%rax
   3be2a:	jae    3bf46 <meshopt_peers::run+0x2226>
   3be30:	mov    0xb0(%rsp),%rdi
   3be38:	call   *0x10d132(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3be3e:	mov    $0x8,%bpl
   3be41:	movq   $0x0,0x60(%rsp)
   3be4a:	xor    %r15d,%r15d
   3be4d:	mov    0x38(%rsp),%rax
   3be52:	mov    0xc0(%rsp),%rcx
   3be5a:	mov    %r12,%r14
   3be5d:	mov    %rax,%r12
   3be60:	jmp    3bf9e <meshopt_peers::run+0x227e>
   3be65:	mov    0x188(%rsp),%r12
   3be6d:	xor    %r13d,%r13d
   3be70:	cmp    $0x1,%r12
   3be74:	jne    3bfa6 <meshopt_peers::run+0x2286>
   3be7a:	mov    0x88(%rsp),%r12
   3be82:	mov    0x80(%rsp),%rax
   3be8a:	mov    %rax,0xc0(%rsp)
   3be92:	mov    0x48(%rsp),%rdi
   3be97:	movups 0x1c(%rdi),%xmm0
   3be9b:	movaps %xmm0,0x720(%rsp)
   3bea3:	movl   $0x7f7fffff,0x730(%rsp)
   3beae:	lea    0xbc(%rsp),%rax
   3beb6:	lea    0x210(%rsp),%r10
   3bebe:	lea    0x720(%rsp),%r11
   3bec6:	mov    $0x1,%esi
   3becb:	mov    0x28(%rsp),%rdx
   3bed0:	mov    0x70(%rsp),%rcx
   3bed5:	mov    0x10(%rsp),%r8
   3beda:	mov    %r14,%r9
   3bedd:	lea    0x750(%rsp),%r15
   3bee5:	push   %r15
   3bee7:	push   %rax
   3bee8:	push   %r10
   3beea:	push   %r13
   3beec:	push   %r11
   3beee:	push   $0x0
   3bef0:	call   369a0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3bef5:	add    $0x30,%rsp
   3bef9:	mov    %rax,%rbp
   3befc:	test   $0x1,%bpl
   3bf00:	je     3bd91 <meshopt_peers::run+0x2071>
   3bf06:	shr    $0x8,%ebp
   3bf09:	mov    $0x1,%al
   3bf0b:	mov    %rax,0x60(%rsp)
   3bf10:	mov    $0x1,%r15b
   3bf13:	mov    $0x1,%r13b
   3bf16:	mov    0xc0(%rsp),%rcx
   3bf1e:	jmp    3d36f <meshopt_peers::run+0x364f>
   3bf23:	mov    0x328(%rsp),%rdi
   3bf2b:	mov    %rdi,0xb0(%rsp)
   3bf33:	xor    %esi,%esi
   3bf35:	mov    %rbp,%r14
   3bf38:	mov    %rbp,%rdx
   3bf3b:	call   *0x10d04f(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   3bf41:	jmp    3b82a <meshopt_peers::run+0x1b0a>
   3bf46:	mov    %rax,0x768(%rsp)
   3bf4e:	mov    %r12,%r14
   3bf51:	test   %r12,%r12
   3bf54:	je     3bf64 <meshopt_peers::run+0x2244>
   3bf56:	mov    0xb0(%rsp),%rdi
   3bf5e:	call   *0x10d00c(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3bf64:	movq   $0x0,0x60(%rsp)
   3bf6d:	lea    0x750(%rsp),%rdi
   3bf75:	mov    $0x4,%edx
   3bf7a:	xor    %esi,%esi
   3bf7c:	call   35c40 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   3bf81:	cmp    $0xff,%al
   3bf83:	mov    0x38(%rsp),%rsi
   3bf88:	je     3d06f <meshopt_peers::run+0x334f>
   3bf8e:	mov    %eax,%ebp
   3bf90:	xor    %r15d,%r15d
   3bf93:	mov    %rsi,%r12
   3bf96:	mov    0xc0(%rsp),%rcx
   3bf9e:	mov    $0x1,%r13b
   3bfa1:	jmp    3d36f <meshopt_peers::run+0x364f>
   3bfa6:	lea    0xf8(%rsp),%rdi
   3bfae:	mov    0x10d403(%rip),%r10        # 1493b8 <_DYNAMIC+0x650>
   3bfb5:	mov    0x70(%rsp),%r8
   3bfba:	lea    0x104f67(%rip),%rax        # 140f28 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xf8>
   3bfc1:	mov    %rax,0x1e0(%rsp)
   3bfc9:	movb   $0x2,0x130(%rsp)
   3bfd1:	mov    0x10(%rsp),%rax
   3bfd6:	mov    %rax,0x138(%rsp)
   3bfde:	mov    %r14,0x140(%rsp)
   3bfe6:	movq   $0x0,0x168(%rsp)
   3bff2:	movups 0x1d0(%rsp),%xmm0
   3bffa:	movaps %xmm0,0x360(%rsp)
   3c002:	movups 0x190(%rsp),%xmm0
   3c00a:	movups 0x1a0(%rsp),%xmm1
   3c012:	movups 0x1b0(%rsp),%xmm2
   3c01a:	movups 0x1c0(%rsp),%xmm3
   3c022:	movaps %xmm3,0x350(%rsp)
   3c02a:	movaps %xmm2,0x340(%rsp)
   3c032:	movaps %xmm1,0x330(%rsp)
   3c03a:	movaps %xmm0,0x320(%rsp)
   3c042:	mov    0x48(%rsp),%rsi
   3c047:	mov    %r12,%rdx
   3c04a:	mov    0x28(%rsp),%rcx
   3c04f:	mov    %r8,0x40(%rsp)
   3c054:	mov    0xf0(%rsp),%r9
   3c05c:	lea    0x750(%rsp),%rax
   3c064:	push   %rax
   3c065:	lea    0x328(%rsp),%rax
   3c06d:	push   %rax
   3c06e:	lea    0x140(%rsp),%rax
   3c076:	push   %rax
   3c077:	push   0x78(%rsp)
   3c07b:	call   *%r10
   3c07e:	add    $0x20,%rsp
   3c082:	mov    %r13,0x1e8(%rsp)
   3c08a:	mov    0xf8(%rsp),%r15
   3c092:	movzbl 0x100(%rsp),%ebp
   3c09a:	cmp    $0xffffffffffffffff,%r15
   3c09e:	je     3d352 <meshopt_peers::run+0x3632>
   3c0a4:	movzbl %bpl,%eax
   3c0a8:	movzbl 0x107(%rsp),%ecx
   3c0b0:	shl    $0x10,%ecx
   3c0b3:	movzwl 0x105(%rsp),%edx
   3c0bb:	or     %ecx,%edx
   3c0bd:	mov    0x101(%rsp),%r8d
   3c0c5:	mov    0x108(%rsp),%r9
   3c0cd:	shl    $0x28,%rdx
   3c0d1:	shl    $0x8,%r8
   3c0d5:	or     %rdx,%r8
   3c0d8:	or     %rax,%r8
   3c0db:	sub    $0x40,%rsp
   3c0df:	lea    0x790(%rsp),%rax
   3c0e7:	mov    %rax,0x30(%rsp)
   3c0ec:	mov    0xa0(%rsp),%rax
   3c0f4:	mov    %rax,0x18(%rsp)
   3c0f9:	mov    0x130(%rsp),%rax
   3c101:	mov    %rax,0x10(%rsp)
   3c106:	mov    0x80(%rsp),%rax
   3c10e:	mov    %rax,0x8(%rsp)
   3c113:	mov    0x68(%rsp),%rax
   3c118:	mov    %rax,(%rsp)
   3c11c:	movq   $0x0,0x20(%rsp)
   3c125:	mov    0xf0(%rsp),%rdi
   3c12d:	mov    %r14,%rsi
   3c130:	mov    0x88(%rsp),%rdx
   3c138:	mov    %r12,0x1c8(%rsp)
   3c140:	mov    %r12,%rcx
   3c143:	mov    %r8,0x1c0(%rsp)
   3c14b:	mov    %r9,0x1b8(%rsp)
   3c153:	call   *0x10d267(%rip)        # 1493c0 <_DYNAMIC+0x658>
   3c159:	add    $0x40,%rsp
   3c15d:	mov    %eax,%ebp
   3c15f:	mov    $0x1,%r12b
   3c162:	cmp    $0xff,%al
   3c164:	jne    3d0cd <meshopt_peers::run+0x33ad>
   3c16a:	mov    0x40(%rsp),%rax
   3c16f:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   3c179:	mul    %rcx
   3c17c:	mov    %rdx,%rax
   3c17f:	shr    $1,%rax
   3c182:	mov    %rax,0x68(%rsp)
   3c187:	mov    $0x30,%ecx
   3c18c:	mul    %rcx
   3c18f:	seto   %cl
   3c192:	test   %rax,%rax
   3c195:	sets   %dl
   3c198:	or     %cl,%dl
   3c19a:	mov    $0x8,%bpl
   3c19d:	jne    3d0cd <meshopt_peers::run+0x33ad>
   3c1a3:	add    0x768(%rsp),%rax
   3c1ab:	jb     3d0cd <meshopt_peers::run+0x33ad>
   3c1b1:	mov    %rax,0x768(%rsp)
   3c1b9:	mov    0x750(%rsp),%r13
   3c1c1:	mov    %r13,%rdi
   3c1c4:	mov    %rax,%r12
   3c1c7:	mov    %rax,%rsi
   3c1ca:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c1cf:	mov    %eax,%ebp
   3c1d1:	mov    %r15,0x58(%rsp)
   3c1d6:	cmp    $0xff,%al
   3c1d8:	jne    3d0c0 <meshopt_peers::run+0x33a0>
   3c1de:	cmpq   $0x3,0x40(%rsp)
   3c1e4:	jae    3cff1 <meshopt_peers::run+0x32d1>
   3c1ea:	mov    $0x8,%r15d
   3c1f0:	xor    %eax,%eax
   3c1f2:	mov    %rax,0x38(%rsp)
   3c1f7:	sub    0x68(%rsp),%rax
   3c1fc:	mov    $0x30,%ecx
   3c201:	mul    %rcx
   3c204:	seto   %cl
   3c207:	test   %rax,%rax
   3c20a:	sets   %dl
   3c20d:	or     %cl,%dl
   3c20f:	mov    $0x8,%bpl
   3c212:	jne    3d0af <meshopt_peers::run+0x338f>
   3c218:	test   %rax,%rax
   3c21b:	je     3c243 <meshopt_peers::run+0x2523>
   3c21d:	add    %r12,%rax
   3c220:	jb     3d0af <meshopt_peers::run+0x338f>
   3c226:	mov    %rax,0x768(%rsp)
   3c22e:	mov    %r13,%rdi
   3c231:	mov    %rax,%rsi
   3c234:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c239:	mov    %eax,%ebp
   3c23b:	cmp    $0xff,%al
   3c23d:	jne    3d0af <meshopt_peers::run+0x338f>
   3c243:	mov    %r15,%rdx
   3c246:	shr    $0x8,%rdx
   3c24a:	mov    %r15,%rax
   3c24d:	shr    $0x28,%rax
   3c251:	mov    %r15,%rcx
   3c254:	shr    $0x38,%rcx
   3c258:	mov    %cl,0x13f(%rsp)
   3c25f:	mov    %ax,0x13d(%rsp)
   3c267:	mov    %edx,0x139(%rsp)
   3c26e:	movq   $0x0,0x140(%rsp)
   3c27a:	mov    0x38(%rsp),%rax
   3c27f:	mov    %rax,0x130(%rsp)
   3c287:	mov    %r15b,0x138(%rsp)
   3c28f:	cmpq   $0x2,0x40(%rsp)
   3c295:	ja     3c29f <meshopt_peers::run+0x257f>
   3c297:	mov    %rdx,%rcx
   3c29a:	jmp    3c324 <meshopt_peers::run+0x2604>
   3c29f:	mov    0x68(%rsp),%rax
   3c2a4:	cmp    %rax,0x38(%rsp)
   3c2a9:	jb     3d036 <meshopt_peers::run+0x3316>
   3c2af:	xor    %ebx,%ebx
   3c2b1:	mov    0x138(%rsp),%r15
   3c2b9:	lea    (%rbx,%rbx,2),%rax
   3c2bd:	shl    $0x4,%rax
   3c2c1:	xorps  %xmm0,%xmm0
   3c2c4:	movups %xmm0,0x20(%r15,%rax,1)
   3c2ca:	movups %xmm0,0x10(%r15,%rax,1)
   3c2d0:	movups %xmm0,(%r15,%rax,1)
   3c2d5:	cmpq   $0x6,0x40(%rsp)
   3c2db:	jb     3c2fe <meshopt_peers::run+0x25de>
   3c2dd:	lea    (%r15,%rax,1),%rdi
   3c2e1:	add    $0x30,%rdi
   3c2e5:	mov    0x68(%rsp),%rax
   3c2ea:	lea    (%rax,%rax,2),%rdx
   3c2ee:	shl    $0x4,%rdx
   3c2f2:	add    $0xffffffffffffffd0,%rdx
   3c2f6:	xor    %esi,%esi
   3c2f8:	call   *0x10cc92(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   3c2fe:	mov    0x130(%rsp),%rax
   3c306:	mov    %rax,0x38(%rsp)
   3c30b:	cmp    $0xffffffffffffffff,%rax
   3c30f:	je     3d445 <meshopt_peers::run+0x3725>
   3c315:	mov    %r15,%rax
   3c318:	shr    $0x8,%rax
   3c31c:	mov    %rax,%rcx
   3c31f:	add    %rbx,0x68(%rsp)
   3c324:	movzbl %r15b,%eax
   3c328:	shl    $0x8,%rcx
   3c32c:	or     %rax,%rcx
   3c32f:	mov    %rcx,0xd8(%rsp)
   3c337:	lea    0x320(%rsp),%rdi
   3c33f:	lea    0x750(%rsp),%rsi
   3c347:	mov    0x40(%rsp),%rdx
   3c34c:	call   35d30 <<meshoptimizer_rs::processing::Context>::alloc::<u32>>
   3c351:	mov    0x320(%rsp),%rax
   3c359:	movzbl 0x328(%rsp),%ebp
   3c361:	mov    %rax,0xc0(%rsp)
   3c369:	cmp    $0xffffffffffffffff,%rax
   3c36d:	je     3d423 <meshopt_peers::run+0x3703>
   3c373:	movzbl %bpl,%eax
   3c377:	movzbl 0x32f(%rsp),%ecx
   3c37f:	shl    $0x10,%ecx
   3c382:	movzwl 0x32d(%rsp),%edx
   3c38a:	or     %ecx,%edx
   3c38c:	mov    0x329(%rsp),%ecx
   3c393:	mov    0x330(%rsp),%rsi
   3c39b:	mov    %rsi,0x70(%rsp)
   3c3a0:	shl    $0x28,%rdx
   3c3a4:	shl    $0x8,%rcx
   3c3a8:	or     %rdx,%rcx
   3c3ab:	or     %rax,%rcx
   3c3ae:	mov    %rcx,0xe8(%rsp)
   3c3b6:	cmpq   $0x2,0x178(%rsp)
   3c3bf:	jb     3cee4 <meshopt_peers::run+0x31c4>
   3c3c5:	mov    0x48(%rsp),%rax
   3c3ca:	lea    0xa0(%rax),%rcx
   3c3d1:	mov    %rcx,0x310(%rsp)
   3c3d9:	lea    0x10(%rax),%rcx
   3c3dd:	mov    %rcx,0x318(%rsp)
   3c3e5:	add    $0x1c,%rax
   3c3e9:	mov    %rax,0x230(%rsp)
   3c3f1:	mov    0xd8(%rsp),%rax
   3c3f9:	add    $0x10,%rax
   3c3fd:	mov    %rax,0x308(%rsp)
   3c405:	xor    %ebx,%ebx
   3c407:	movq   $0x0,0xd0(%rsp)
   3c413:	mov    0x180(%rsp),%rax
   3c41b:	mov    %rax,0x228(%rsp)
   3c423:	mov    %rbx,0xe0(%rsp)
   3c42b:	mov    0x228(%rsp),%rax
   3c433:	mov    (%rax),%r13d
   3c436:	mov    0x4(%rax),%eax
   3c439:	mov    %rax,%r12
   3c43c:	cmp    %r13d,%eax
   3c43f:	jb     3d71d <meshopt_peers::run+0x39fd>
   3c445:	cmp    %r12,0x188(%rsp)
   3c44d:	jb     3d71d <meshopt_peers::run+0x39fd>
   3c453:	mov    %r13,%rax
   3c456:	shl    $0x4,%rax
   3c45a:	lea    (%rax,%rax,2),%rax
   3c45e:	mov    %rax,0x20(%rsp)
   3c463:	mov    %r12,%rcx
   3c466:	mov    %r12,%rax
   3c469:	shl    $0x4,%rax
   3c46d:	lea    (%rax,%rax,2),%rax
   3c471:	mov    %rax,0xa0(%rsp)
   3c479:	cmp    %ecx,%r13d
   3c47c:	jne    3c485 <meshopt_peers::run+0x2765>
   3c47e:	xor    %edx,%edx
   3c480:	jmp    3c52a <meshopt_peers::run+0x280a>
   3c485:	mov    0xa0(%rsp),%rcx
   3c48d:	sub    0x20(%rsp),%rcx
   3c492:	mov    %rcx,%rax
   3c495:	movabs $0x555555555555556,%rdx
   3c49f:	mul    %rdx
   3c4a2:	mov    %rdx,%rax
   3c4a5:	mov    %eax,%esi
   3c4a7:	and    $0x3,%esi
   3c4aa:	cmp    $0xc0,%rcx
   3c4b1:	jae    3c4b9 <meshopt_peers::run+0x2799>
   3c4b3:	xor    %ecx,%ecx
   3c4b5:	xor    %edx,%edx
   3c4b7:	jmp    3c4f9 <meshopt_peers::run+0x27d9>
   3c4b9:	and    $0xfffffffffffffffc,%rax
   3c4bd:	mov    0x310(%rsp),%rcx
   3c4c5:	mov    0x20(%rsp),%rdx
   3c4ca:	lea    (%rcx,%rdx,1),%rdi
   3c4ce:	xor    %ecx,%ecx
   3c4d0:	xor    %edx,%edx
   3c4d2:	add    -0x90(%rdi),%rdx
   3c4d9:	add    -0x60(%rdi),%rdx
   3c4dd:	add    -0x30(%rdi),%rdx
   3c4e1:	add    (%rdi),%rdx
   3c4e4:	add    $0x4,%rcx
   3c4e8:	add    $0xc0,%rdi
   3c4ef:	cmp    %rcx,%rax
   3c4f2:	jne    3c4d2 <meshopt_peers::run+0x27b2>
   3c4f4:	test   %rsi,%rsi
   3c4f7:	je     3c52a <meshopt_peers::run+0x280a>
   3c4f9:	lea    (%rcx,%rcx,2),%rax
   3c4fd:	shl    $0x4,%rax
   3c501:	mov    0x318(%rsp),%rcx
   3c509:	mov    0x20(%rsp),%rdi
   3c50e:	add    %rdi,%rcx
   3c511:	add    %rcx,%rax
   3c514:	shl    $0x4,%esi
   3c517:	lea    (%rsi,%rsi,2),%rcx
   3c51b:	xor    %esi,%esi
   3c51d:	add    (%rax,%rsi,1),%rdx
   3c521:	add    $0x30,%rsi
   3c525:	cmp    %rsi,%rcx
   3c528:	jne    3c51d <meshopt_peers::run+0x27fd>
   3c52a:	lea    0x320(%rsp),%rdi
   3c532:	lea    0x750(%rsp),%rsi
   3c53a:	call   35d30 <<meshoptimizer_rs::processing::Context>::alloc::<u32>>
   3c53f:	mov    0x320(%rsp),%rcx
   3c547:	movzbl 0x328(%rsp),%ebp
   3c54f:	mov    %rcx,0x78(%rsp)
   3c554:	cmp    $0xffffffffffffffff,%rcx
   3c558:	je     3d3d3 <meshopt_peers::run+0x36b3>
   3c55e:	mov    %r12,%rsi
   3c561:	sub    %r13,%rsi
   3c564:	movzbl %bpl,%eax
   3c568:	movzbl 0x32f(%rsp),%ecx
   3c570:	shl    $0x10,%ecx
   3c573:	movzwl 0x32d(%rsp),%edx
   3c57b:	or     %ecx,%edx
   3c57d:	mov    0x329(%rsp),%ecx
   3c584:	shl    $0x28,%rdx
   3c588:	shl    $0x8,%rcx
   3c58c:	or     %rdx,%rcx
   3c58f:	or     %rax,%rcx
   3c592:	mov    %rcx,0xa8(%rsp)
   3c59a:	mov    %rsi,0x98(%rsp)
   3c5a2:	lea    0x0(,%rsi,4),%rax
   3c5aa:	lea    (%rax,%rax,4),%r15
   3c5ae:	mov    0x768(%rsp),%rsi
   3c5b6:	add    %r15,%rsi
   3c5b9:	jb     3d7ce <meshopt_peers::run+0x3aae>
   3c5bf:	mov    0x330(%rsp),%rax
   3c5c7:	mov    %rax,0x18(%rsp)
   3c5cc:	mov    %rsi,0x768(%rsp)
   3c5d4:	mov    0x750(%rsp),%rdi
   3c5dc:	call   43100 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3c5e1:	mov    %eax,%ebp
   3c5e3:	cmp    $0xff,%al
   3c5e5:	jne    3d2c3 <meshopt_peers::run+0x35a3>
   3c5eb:	cmpq   $0x0,0x98(%rsp)
   3c5f4:	jne    3c9d8 <meshopt_peers::run+0x2cb8>
   3c5fa:	mov    $0x4,%eax
   3c5ff:	mov    %rax,0x50(%rsp)
   3c604:	movq   $0x0,0x30(%rsp)
   3c60d:	cmp    %r12d,%r13d
   3c610:	mov    0x10c969(%rip),%rbx        # 148f80 <memcpy@GLIBC_2.14>
   3c617:	mov    0x40(%rsp),%rsi
   3c61c:	mov    0x230(%rsp),%rcx
   3c624:	je     3c6d0 <meshopt_peers::run+0x29b0>
   3c62a:	sub    %r13,%r12
   3c62d:	inc    %r12
   3c630:	mov    %r12,%r13
   3c633:	mov    0x20(%rsp),%r12
   3c638:	mov    0x50(%rsp),%rbp
   3c63d:	xor    %edi,%edi
   3c63f:	mov    -0xc(%rcx,%r12,1),%rdx
   3c644:	mov    %rdx,%r15
   3c647:	add    %rdi,%r15
   3c64a:	jb     3d541 <meshopt_peers::run+0x3821>
   3c650:	cmp    0x18(%rsp),%r15
   3c655:	ja     3d541 <meshopt_peers::run+0x3821>
   3c65b:	mov    -0x14(%rcx,%r12,1),%rax
   3c660:	mov    %rax,%rcx
   3c663:	add    %rdx,%rcx
   3c666:	jb     3d515 <meshopt_peers::run+0x37f5>
   3c66c:	cmp    %rsi,%rcx
   3c66f:	ja     3d522 <meshopt_peers::run+0x3802>
   3c675:	mov    0xa8(%rsp),%rcx
   3c67d:	lea    (%rcx,%rdi,4),%rdi
   3c681:	mov    0x28(%rsp),%rcx
   3c686:	lea    (%rcx,%rax,4),%rsi
   3c68a:	shl    $0x2,%rdx
   3c68e:	call   *%rbx
   3c690:	dec    %r13
   3c693:	je     3d876 <meshopt_peers::run+0x3b56>
   3c699:	mov    0x230(%rsp),%rcx
   3c6a1:	movups (%rcx,%r12,1),%xmm0
   3c6a6:	mov    0x10(%rcx,%r12,1),%eax
   3c6ab:	mov    %eax,0x10(%rbp)
   3c6ae:	movups %xmm0,0x0(%rbp)
   3c6b2:	add    $0x14,%rbp
   3c6b6:	add    $0x30,%r12
   3c6ba:	mov    %r15,%rdi
   3c6bd:	cmp    %r12,0xa0(%rsp)
   3c6c5:	mov    0x40(%rsp),%rsi
   3c6ca:	jne    3c63f <meshopt_peers::run+0x291f>
   3c6d0:	mov    0x18(%rsp),%rax
   3c6d5:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   3c6df:	mul    %rcx
   3c6e2:	shr    $1,%rdx
   3c6e5:	xorps  %xmm0,%xmm0
   3c6e8:	cvtsi2ss %rdx,%xmm0
   3c6ed:	mulss  -0x2bd49(%rip),%xmm0        # 109ac <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa0>
   3c6f5:	cvttss2si %xmm0,%rax
   3c6fa:	mov    %rax,%rcx
   3c6fd:	sar    $0x3f,%rcx
   3c701:	movaps %xmm0,%xmm1
   3c704:	subss  -0x2bd58(%rip),%xmm1        # 109b4 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa8>
   3c70c:	cvttss2si %xmm1,%rdx
   3c711:	and    %rcx,%rdx
   3c714:	or     %rax,%rdx
   3c717:	xorps  %xmm1,%xmm1
   3c71a:	ucomiss %xmm1,%xmm0
   3c71d:	mov    $0x0,%ebx
   3c722:	cmovae %rdx,%rbx
   3c726:	ucomiss -0x2bd75(%rip),%xmm0        # 109b8 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xac>
   3c72d:	mov    $0xffffffffffffffff,%rax
   3c734:	cmova  %rax,%rbx
   3c738:	lea    0x320(%rsp),%rdi
   3c740:	mov    0x50(%rsp),%rsi
   3c745:	mov    0x98(%rsp),%rdx
   3c74d:	lea    0x750(%rsp),%rcx
   3c755:	call   *0x10cc6d(%rip)        # 1493c8 <_DYNAMIC+0x660>
   3c75b:	cmpb   $0x1,0x320(%rsp)
   3c763:	je     3d46a <meshopt_peers::run+0x374a>
   3c769:	lea    (%rbx,%rbx,2),%rax
   3c76d:	lea    0x324(%rsp),%rdx
   3c775:	mov    0x10(%rdx),%ecx
   3c778:	lea    0x132(%rsp),%rsi
   3c780:	mov    %ecx,0x10(%rsi)
   3c783:	movups (%rdx),%xmm0
   3c786:	movups %xmm0,(%rsi)
   3c789:	movups (%rsi),%xmm0
   3c78c:	movaps %xmm0,0x1f0(%rsp)
   3c794:	mov    0x10(%rsi),%ecx
   3c797:	mov    %ecx,0x200(%rsp)
   3c79e:	sub    $0x8,%rsp
   3c7a2:	lea    0x328(%rsp),%rdi
   3c7aa:	mov    0xb0(%rsp),%rsi
   3c7b2:	mov    0x20(%rsp),%rdx
   3c7b7:	lea    0x258(%rsp),%rcx
   3c7bf:	mov    0xb8(%rsp),%r8
   3c7c7:	mov    %r14,%r9
   3c7ca:	lea    0x758(%rsp),%r10
   3c7d2:	push   %r10
   3c7d4:	push   %rax
   3c7d5:	lea    0x1a8(%rsp),%rax
   3c7dd:	push   %rax
   3c7de:	call   *0x10cbec(%rip)        # 1493d0 <_DYNAMIC+0x668>
   3c7e4:	add    $0x20,%rsp
   3c7e8:	mov    0x320(%rsp),%rax
   3c7f0:	movzbl 0x328(%rsp),%ebp
   3c7f8:	cmp    $0xffffffffffffffff,%rax
   3c7fc:	je     3d472 <meshopt_peers::run+0x3752>
   3c802:	mov    %rax,%r12
   3c805:	decq   0x178(%rsp)
   3c80d:	addq   $0x4,0x228(%rsp)
   3c816:	mov    0x20(%rsp),%rax
   3c81b:	add    0x48(%rsp),%rax
   3c820:	mov    %rax,0x20(%rsp)
   3c825:	movzbl %bpl,%eax
   3c829:	movzbl 0x32f(%rsp),%ecx
   3c831:	shl    $0x10,%ecx
   3c834:	movzwl 0x32d(%rsp),%edx
   3c83c:	or     %ecx,%edx
   3c83e:	mov    0x329(%rsp),%r13d
   3c846:	mov    0x330(%rsp),%r15
   3c84e:	shl    $0x28,%rdx
   3c852:	shl    $0x8,%r13
   3c856:	xorps  %xmm0,%xmm0
   3c859:	cvtsi2ss %r15,%xmm0
   3c85e:	or     %rdx,%r13
   3c861:	cvtsi2ssq 0x18(%rsp),%xmm1
   3c868:	or     %rax,%r13
   3c86b:	mulss  -0x2becf(%rip),%xmm1        # 109a4 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x98>
   3c873:	ucomiss %xmm1,%xmm0
   3c876:	jbe    3ca6a <meshopt_peers::run+0x2d4a>
   3c87c:	movl   $0x7f7fffff,0x200(%rsp)
   3c887:	mov    0x20(%rsp),%rdi
   3c88c:	mov    0x98(%rsp),%rsi
   3c894:	mov    0x28(%rsp),%rdx
   3c899:	mov    0x40(%rsp),%rcx
   3c89e:	mov    0x10(%rsp),%r8
   3c8a3:	mov    %r14,%r9
   3c8a6:	lea    0x750(%rsp),%rax
   3c8ae:	push   %rax
   3c8af:	lea    0xc4(%rsp),%rax
   3c8b7:	push   %rax
   3c8b8:	lea    0x220(%rsp),%rax
   3c8c0:	push   %rax
   3c8c1:	push   0x200(%rsp)
   3c8c8:	lea    0x210(%rsp),%rax
   3c8d0:	push   %rax
   3c8d1:	push   $0x0
   3c8d3:	call   369a0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3c8d8:	add    $0x30,%rsp
   3c8dc:	mov    %rax,%rbp
   3c8df:	test   $0x1,%bpl
   3c8e3:	jne    3d4ca <meshopt_peers::run+0x37aa>
   3c8e9:	mov    0x768(%rsp),%rax
   3c8f1:	mov    0x78(%rsp),%rdx
   3c8f6:	lea    0x0(,%rdx,4),%rcx
   3c8fe:	shr    $0x3e,%rdx
   3c902:	sete   %dl
   3c905:	test   %rcx,%rcx
   3c908:	setns  %sil
   3c90c:	and    %dl,%sil
   3c90f:	sub    %rcx,%rax
   3c912:	setae  %cl
   3c915:	test   %cl,%sil
   3c918:	je     3d264 <meshopt_peers::run+0x3544>
   3c91e:	mov    %rax,0x768(%rsp)
   3c926:	cmpq   $0x0,0x78(%rsp)
   3c92c:	je     3c944 <meshopt_peers::run+0x2c24>
   3c92e:	mov    0xa8(%rsp),%rdi
   3c936:	call   *0x10c634(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3c93c:	mov    0x768(%rsp),%rax
   3c944:	mov    0x30(%rsp),%rcx
   3c949:	lea    0x0(,%rcx,4),%rcx
   3c951:	lea    (%rcx,%rcx,4),%rcx
   3c955:	sub    %rcx,%rax
   3c958:	jb     3d3af <meshopt_peers::run+0x368f>
   3c95e:	mov    %rax,0x768(%rsp)
   3c966:	cmpq   $0x0,0x30(%rsp)
   3c96c:	je     3c981 <meshopt_peers::run+0x2c61>
   3c96e:	mov    0x50(%rsp),%rdi
   3c973:	call   *0x10c5f7(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3c979:	mov    0x768(%rsp),%rax
   3c981:	movabs $0x3fffffffffffffff,%rcx
   3c98b:	cmp    %rcx,%r12
   3c98e:	ja     3d3c2 <meshopt_peers::run+0x36a2>
   3c994:	lea    0x0(,%r12,4),%rcx
   3c99c:	test   %rcx,%rcx
   3c99f:	js     3d3c2 <meshopt_peers::run+0x36a2>
   3c9a5:	sub    %rcx,%rax
   3c9a8:	jb     3d3c2 <meshopt_peers::run+0x36a2>
   3c9ae:	mov    %rax,0x768(%rsp)
   3c9b6:	test   %r12,%r12
   3c9b9:	je     3c9c4 <meshopt_peers::run+0x2ca4>
   3c9bb:	mov    %r13,%rdi
   3c9be:	call   *0x10c5ac(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3c9c4:	cmpq   $0x1,0x178(%rsp)
   3c9cd:	ja     3c42b <meshopt_peers::run+0x270b>
   3c9d3:	jmp    3d062 <meshopt_peers::run+0x3342>
   3c9d8:	mov    $0x4,%edx
   3c9dd:	mov    $0x4,%r8d
   3c9e3:	mov    $0x14,%r9d
   3c9e9:	lea    0x320(%rsp),%rdi
   3c9f1:	xor    %esi,%esi
   3c9f3:	mov    0x98(%rsp),%rcx
   3c9fb:	call   423f0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3ca00:	cmpl   $0x1,0x320(%rsp)
   3ca08:	je     3d6a1 <meshopt_peers::run+0x3981>
   3ca0e:	mov    0x328(%rsp),%rax
   3ca16:	xorps  %xmm0,%xmm0
   3ca19:	movups %xmm0,(%rax)
   3ca1c:	mov    %rax,0x50(%rsp)
   3ca21:	movl   $0x0,0x10(%rax)
   3ca28:	mov    $0x1,%eax
   3ca2d:	mov    %rax,0x30(%rsp)
   3ca32:	cmpl   $0x1,0x98(%rsp)
   3ca3a:	je     3c60d <meshopt_peers::run+0x28ed>
   3ca40:	mov    0x50(%rsp),%rdi
   3ca45:	add    $0x14,%rdi
   3ca49:	add    $0xffffffffffffffec,%r15
   3ca4d:	xor    %esi,%esi
   3ca4f:	mov    %r15,%rdx
   3ca52:	call   *0x10c538(%rip)        # 148f90 <memset@GLIBC_2.2.5>
   3ca58:	mov    0x98(%rsp),%rax
   3ca60:	mov    %rax,0x30(%rsp)
   3ca65:	jmp    3c60d <meshopt_peers::run+0x28ed>
   3ca6a:	movss  0x338(%rsp),%xmm0
   3ca73:	movss  0x200(%rsp),%xmm1
   3ca7c:	movaps %xmm1,%xmm2
   3ca7f:	maxss  %xmm0,%xmm2
   3ca83:	movaps %xmm2,%xmm3
   3ca86:	cmpunordss %xmm2,%xmm3
   3ca8b:	movaps %xmm3,%xmm4
   3ca8e:	andnps %xmm2,%xmm4
   3ca91:	andps  %xmm1,%xmm3
   3ca94:	orps   %xmm4,%xmm3
   3ca97:	xorps  %xmm1,%xmm1
   3ca9a:	mulss  %xmm1,%xmm0
   3ca9e:	addss  %xmm3,%xmm0
   3caa2:	movd   %xmm0,%eax
   3caa6:	and    $0x7fffffff,%eax
   3caab:	mov    $0x1,%cl
   3caad:	mov    %ecx,0x18(%rsp)
   3cab1:	cmp    $0x7f7fffff,%eax
   3cab6:	jg     3d73d <meshopt_peers::run+0x3a1d>
   3cabc:	movss  %xmm0,0x200(%rsp)
   3cac5:	mov    0x20(%rsp),%rdi
   3caca:	mov    0x98(%rsp),%rsi
   3cad2:	mov    0x28(%rsp),%rdx
   3cad7:	mov    0x40(%rsp),%rcx
   3cadc:	mov    0x10(%rsp),%r8
   3cae1:	mov    %r14,%r9
   3cae4:	lea    0x750(%rsp),%rax
   3caec:	push   %rax
   3caed:	lea    0xc4(%rsp),%rax
   3caf5:	push   %rax
   3caf6:	lea    0x220(%rsp),%rax
   3cafe:	push   %rax
   3caff:	push   0x200(%rsp)
   3cb06:	lea    0x210(%rsp),%rax
   3cb0e:	push   %rax
   3cb0f:	push   $0x0
   3cb11:	call   369a0 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3cb16:	add    $0x30,%rsp
   3cb1a:	mov    %rax,%rbp
   3cb1d:	test   $0x1,%bpl
   3cb21:	jne    3d745 <meshopt_peers::run+0x3a25>
   3cb27:	shr    $0x20,%rbp
   3cb2b:	movb   $0x2,0x130(%rsp)
   3cb33:	mov    0x10(%rsp),%rax
   3cb38:	mov    %rax,0x138(%rsp)
   3cb40:	mov    %r14,0x140(%rsp)
   3cb48:	movq   $0x0,0x168(%rsp)
   3cb54:	movups 0x1d0(%rsp),%xmm0
   3cb5c:	movaps %xmm0,0x360(%rsp)
   3cb64:	movups 0x190(%rsp),%xmm0
   3cb6c:	movups 0x1a0(%rsp),%xmm1
   3cb74:	movups 0x1b0(%rsp),%xmm2
   3cb7c:	movups 0x1c0(%rsp),%xmm3
   3cb84:	movaps %xmm3,0x350(%rsp)
   3cb8c:	movaps %xmm2,0x340(%rsp)
   3cb94:	movaps %xmm1,0x330(%rsp)
   3cb9c:	movaps %xmm0,0x320(%rsp)
   3cba4:	lea    0xf8(%rsp),%rdi
   3cbac:	mov    %r13,%rsi
   3cbaf:	mov    %r15,%rdx
   3cbb2:	lea    0x130(%rsp),%rcx
   3cbba:	lea    0x320(%rsp),%r8
   3cbc2:	mov    %ebp,%r9d
   3cbc5:	lea    0x750(%rsp),%rax
   3cbcd:	push   %rax
   3cbce:	push   0x240(%rsp)
   3cbd5:	call   *0x10c7cd(%rip)        # 1493a8 <_DYNAMIC+0x640>
   3cbdb:	add    $0x10,%rsp
   3cbdf:	mov    0xf8(%rsp),%r15
   3cbe7:	movzbl 0x100(%rsp),%ebp
   3cbef:	cmp    $0xffffffffffffffff,%r15
   3cbf3:	je     3d4fc <meshopt_peers::run+0x37dc>
   3cbf9:	movzbl %bpl,%ecx
   3cbfd:	movzbl 0x107(%rsp),%eax
   3cc05:	shl    $0x10,%eax
   3cc08:	movzwl 0x105(%rsp),%esi
   3cc10:	or     %eax,%esi
   3cc12:	mov    0x101(%rsp),%ebp
   3cc19:	mov    0x108(%rsp),%rax
   3cc21:	mov    0x110(%rsp),%rdx
   3cc29:	mov    %rdx,0x20(%rsp)
   3cc2e:	mov    0x118(%rsp),%rdx
   3cc36:	mov    %rdx,0xa0(%rsp)
   3cc3e:	mov    0x120(%rsp),%rdx
   3cc46:	shl    $0x28,%rsi
   3cc4a:	shl    $0x8,%rbp
   3cc4e:	or     %rsi,%rbp
   3cc51:	or     %rcx,%rbp
   3cc54:	test   %rax,%rax
   3cc57:	je     3cd14 <meshopt_peers::run+0x2ff4>
   3cc5d:	lea    (%rax,%rax,2),%rax
   3cc61:	shl    $0x4,%rax
   3cc65:	add    %rbp,%rax
   3cc68:	mov    0xd0(%rsp),%rcx
   3cc70:	lea    (%rcx,%rcx,2),%rcx
   3cc74:	shl    $0x4,%rcx
   3cc78:	add    0x308(%rsp),%rcx
   3cc80:	xor    %esi,%esi
   3cc82:	mov    0x0(%rbp,%rsi,1),%rdi
   3cc87:	mov    0x8(%rbp,%rsi,1),%r8
   3cc8c:	movups 0x10(%rbp,%rsi,1),%xmm0
   3cc91:	movaps %xmm0,0x2e0(%rsp)
   3cc99:	mov    0x200(%rsp),%r9d
   3cca1:	lea    0x2ec(%rsp),%r10
   3cca9:	mov    %r9d,0x10(%r10)
   3ccad:	movaps 0x1f0(%rsp),%xmm0
   3ccb5:	movups %xmm0,(%r10)
   3ccb9:	mov    0xd0(%rsp),%r9
   3ccc1:	cmp    0x68(%rsp),%r9
   3ccc6:	jae    3d8ff <meshopt_peers::run+0x3bdf>
   3cccc:	lea    (%rsi,%rbp,1),%r9
   3ccd0:	add    0xe0(%rsp),%r8
   3ccd8:	mov    %rdi,-0x10(%rcx,%rsi,1)
   3ccdd:	mov    %r8,-0x8(%rcx,%rsi,1)
   3cce2:	movaps 0x2e0(%rsp),%xmm0
   3ccea:	movaps 0x2f0(%rsp),%xmm1
   3ccf2:	movups %xmm0,(%rcx,%rsi,1)
   3ccf6:	movups %xmm1,0x10(%rcx,%rsi,1)
   3ccfb:	incq   0xd0(%rsp)
   3cd03:	add    $0x30,%rsi
   3cd07:	add    $0x30,%r9
   3cd0b:	cmp    %rax,%r9
   3cd0e:	jne    3cc82 <meshopt_peers::run+0x2f62>
   3cd14:	mov    %rdx,%rbx
   3cd17:	add    0xe0(%rsp),%rbx
   3cd1f:	jb     3d854 <meshopt_peers::run+0x3b34>
   3cd25:	cmp    0x70(%rsp),%rbx
   3cd2a:	ja     3d854 <meshopt_peers::run+0x3b34>
   3cd30:	mov    0xe8(%rsp),%rax
   3cd38:	mov    0xe0(%rsp),%rcx
   3cd40:	lea    (%rax,%rcx,4),%rdi
   3cd44:	shl    $0x2,%rdx
   3cd48:	mov    0xa0(%rsp),%rsi
   3cd50:	call   *0x10c22a(%rip)        # 148f80 <memcpy@GLIBC_2.14>
   3cd56:	mov    0x768(%rsp),%rcx
   3cd5e:	mov    %r15,%rax
   3cd61:	mov    $0x30,%edx
   3cd66:	mul    %rdx
   3cd69:	seto   %dl
   3cd6c:	not    %dl
   3cd6e:	test   %rax,%rax
   3cd71:	setns  %sil
   3cd75:	and    %dl,%sil
   3cd78:	sub    %rax,%rcx
   3cd7b:	setae  %al
   3cd7e:	test   %al,%sil
   3cd81:	je     3d4d5 <meshopt_peers::run+0x37b5>
   3cd87:	mov    %rcx,0x768(%rsp)
   3cd8f:	test   %r15,%r15
   3cd92:	je     3cda5 <meshopt_peers::run+0x3085>
   3cd94:	mov    %rbp,%rdi
   3cd97:	call   *0x10c1d3(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3cd9d:	mov    0x768(%rsp),%rcx
   3cda5:	mov    0x20(%rsp),%rax
   3cdaa:	shr    $0x3e,%rax
   3cdae:	jne    3d62f <meshopt_peers::run+0x390f>
   3cdb4:	mov    0x20(%rsp),%rax
   3cdb9:	lea    0x0(,%rax,4),%rax
   3cdc1:	test   %rax,%rax
   3cdc4:	js     3d62f <meshopt_peers::run+0x390f>
   3cdca:	sub    %rax,%rcx
   3cdcd:	jb     3d62f <meshopt_peers::run+0x390f>
   3cdd3:	mov    %rcx,0x768(%rsp)
   3cddb:	cmpq   $0x0,0x20(%rsp)
   3cde1:	je     3cdf9 <meshopt_peers::run+0x30d9>
   3cde3:	mov    0xa0(%rsp),%rdi
   3cdeb:	call   *0x10c17f(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3cdf1:	mov    0x768(%rsp),%rcx
   3cdf9:	movabs $0x3fffffffffffffff,%rax
   3ce03:	cmp    %rax,0x78(%rsp)
   3ce08:	ja     3d709 <meshopt_peers::run+0x39e9>
   3ce0e:	mov    0x78(%rsp),%rax
   3ce13:	lea    0x0(,%rax,4),%rax
   3ce1b:	test   %rax,%rax
   3ce1e:	js     3d709 <meshopt_peers::run+0x39e9>
   3ce24:	sub    %rax,%rcx
   3ce27:	jb     3d709 <meshopt_peers::run+0x39e9>
   3ce2d:	mov    %rcx,0x768(%rsp)
   3ce35:	cmpq   $0x0,0x78(%rsp)
   3ce3b:	je     3ce53 <meshopt_peers::run+0x3133>
   3ce3d:	mov    0xa8(%rsp),%rdi
   3ce45:	call   *0x10c125(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3ce4b:	mov    0x768(%rsp),%rcx
   3ce53:	mov    0x30(%rsp),%rax
   3ce58:	lea    0x0(,%rax,4),%rax
   3ce60:	lea    (%rax,%rax,4),%rax
   3ce64:	sub    %rax,%rcx
   3ce67:	jb     3d7e5 <meshopt_peers::run+0x3ac5>
   3ce6d:	mov    %rcx,0x768(%rsp)
   3ce75:	cmpq   $0x0,0x30(%rsp)
   3ce7b:	je     3ce90 <meshopt_peers::run+0x3170>
   3ce7d:	mov    0x50(%rsp),%rdi
   3ce82:	call   *0x10c0e8(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3ce88:	mov    0x768(%rsp),%rcx
   3ce90:	movabs $0x3fffffffffffffff,%rax
   3ce9a:	cmp    %rax,%r12
   3ce9d:	ja     3d83d <meshopt_peers::run+0x3b1d>
   3cea3:	lea    0x0(,%r12,4),%rax
   3ceab:	test   %rax,%rax
   3ceae:	js     3d83d <meshopt_peers::run+0x3b1d>
   3ceb4:	sub    %rax,%rcx
   3ceb7:	jb     3d83d <meshopt_peers::run+0x3b1d>
   3cebd:	mov    %rcx,0x768(%rsp)
   3cec5:	test   %r12,%r12
   3cec8:	je     3ced3 <meshopt_peers::run+0x31b3>
   3ceca:	mov    %r13,%rdi
   3cecd:	call   *0x10c09d(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3ced3:	cmpq   $0x1,0x178(%rsp)
   3cedc:	ja     3c423 <meshopt_peers::run+0x2703>
   3cee2:	jmp    3cef2 <meshopt_peers::run+0x31d2>
   3cee4:	movq   $0x0,0xd0(%rsp)
   3cef0:	xor    %ebx,%ebx
   3cef2:	lea    0x750(%rsp),%rdi
   3cefa:	mov    0x58(%rsp),%rsi
   3ceff:	mov    0x180(%rsp),%rdx
   3cf07:	call   35ce0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3cf0c:	mov    $0x1,%r12b
   3cf0f:	cmp    $0xff,%al
   3cf11:	setne  %r13b
   3cf15:	jne    3d43e <meshopt_peers::run+0x371e>
   3cf1b:	mov    0x68(%rsp),%r15
   3cf20:	mov    0xd0(%rsp),%rax
   3cf28:	cmp    %r15,%rax
   3cf2b:	cmovb  %rax,%r15
   3cf2f:	mov    0x70(%rsp),%rax
   3cf34:	cmp    %rax,%rbx
   3cf37:	cmovb  %rbx,%rax
   3cf3b:	mov    %rax,0x70(%rsp)
   3cf40:	lea    0x750(%rsp),%rdi
   3cf48:	mov    0x88(%rsp),%rsi
   3cf50:	mov    0x48(%rsp),%rdx
   3cf55:	call   35c90 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3cf5a:	cmp    $0xff,%al
   3cf5c:	jne    3d43e <meshopt_peers::run+0x371e>
   3cf62:	lea    0x750(%rsp),%rdi
   3cf6a:	mov    0x80(%rsp),%rsi
   3cf72:	mov    0x28(%rsp),%rdx
   3cf77:	call   35ce0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3cf7c:	cmp    $0xff,%al
   3cf7e:	jne    3d45d <meshopt_peers::run+0x373d>
   3cf84:	mov    0x1e8(%rsp),%r13
   3cf8c:	inc    %r13d
   3cf8f:	mov    0x38(%rsp),%rax
   3cf94:	mov    %rax,0x88(%rsp)
   3cf9c:	mov    %r15,%r12
   3cf9f:	mov    0xd8(%rsp),%rax
   3cfa7:	mov    %rax,0x48(%rsp)
   3cfac:	mov    0x70(%rsp),%r8
   3cfb1:	mov    0xe8(%rsp),%rax
   3cfb9:	mov    %rax,0x28(%rsp)
   3cfbe:	mov    0xc0(%rsp),%rax
   3cfc6:	mov    %rax,0x80(%rsp)
   3cfce:	cmp    $0x1,%r15
   3cfd2:	mov    0x8(%rsp),%rbx
   3cfd7:	lea    0xf8(%rsp),%rdi
   3cfdf:	mov    0x10c3d2(%rip),%r10        # 1493b8 <_DYNAMIC+0x650>
   3cfe6:	ja     3bfc9 <meshopt_peers::run+0x22a9>
   3cfec:	jmp    3d49d <meshopt_peers::run+0x377d>
   3cff1:	mov    $0x8,%edx
   3cff6:	mov    $0x8,%r8d
   3cffc:	mov    $0x30,%r9d
   3d002:	lea    0x320(%rsp),%rdi
   3d00a:	xor    %esi,%esi
   3d00c:	mov    0x68(%rsp),%rcx
   3d011:	call   423f0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   3d016:	cmpl   $0x1,0x320(%rsp)
   3d01e:	je     3d50d <meshopt_peers::run+0x37ed>
   3d024:	mov    0x328(%rsp),%r15
   3d02c:	mov    0x68(%rsp),%rax
   3d031:	jmp    3c1f2 <meshopt_peers::run+0x24d2>
   3d036:	mov    $0x8,%ecx
   3d03b:	mov    $0x30,%r8d
   3d041:	lea    0x130(%rsp),%rdi
   3d049:	xor    %esi,%esi
   3d04b:	mov    0x68(%rsp),%rdx
   3d050:	call   39ae0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   3d055:	mov    0x140(%rsp),%rbx
   3d05d:	jmp    3c2b1 <meshopt_peers::run+0x2591>
   3d062:	mov    0xe0(%rsp),%rbx
   3d06a:	jmp    3cef2 <meshopt_peers::run+0x31d2>
   3d06f:	lea    0x750(%rsp),%rdi
   3d077:	mov    0x48(%rsp),%rdx
   3d07c:	call   35c90 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   3d081:	cmp    $0xff,%al
   3d083:	jne    3d893 <meshopt_peers::run+0x3b73>
   3d089:	lea    0x750(%rsp),%rdi
   3d091:	mov    0xc0(%rsp),%rsi
   3d099:	mov    0x28(%rsp),%rdx
   3d09e:	call   35ce0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   3d0a3:	mov    %eax,%ebp
   3d0a5:	mov    0x8(%rsp),%rbx
   3d0aa:	jmp    3b666 <meshopt_peers::run+0x1946>
   3d0af:	cmpq   $0x0,0x38(%rsp)
   3d0b5:	je     3d0c0 <meshopt_peers::run+0x33a0>
   3d0b7:	mov    %r15,%rdi
   3d0ba:	call   *0x10beb0(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d0c0:	mov    0x8(%rsp),%rbx
   3d0c5:	mov    0x58(%rsp),%r15
   3d0ca:	mov    $0x1,%r12b
   3d0cd:	mov    $0x1,%r13b
   3d0d0:	mov    %r13b,0x8(%rsp)
   3d0d5:	test   %r15,%r15
   3d0d8:	je     3d116 <meshopt_peers::run+0x33f6>
   3d0da:	mov    0x180(%rsp),%rdi
   3d0e2:	call   *0x10be88(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d0e8:	mov    $0x1,%al
   3d0ea:	mov    %rax,0x60(%rsp)
   3d0ef:	mov    $0x1,%r15b
   3d0f2:	test   %r12b,%r12b
   3d0f5:	mov    0x88(%rsp),%r12
   3d0fd:	mov    0x80(%rsp),%rcx
   3d105:	movzbl 0x8(%rsp),%r13d
   3d10b:	jne    3d36f <meshopt_peers::run+0x364f>
   3d111:	jmp    3d37f <meshopt_peers::run+0x365f>
   3d116:	movzbl 0x8(%rsp),%r13d
   3d11c:	mov    $0x1,%cl
   3d11e:	mov    %rcx,0x60(%rsp)
   3d123:	mov    $0x1,%r15b
   3d126:	test   %r12b,%r12b
   3d129:	mov    0x88(%rsp),%r12
   3d131:	mov    0x80(%rsp),%rcx
   3d139:	jne    3d36f <meshopt_peers::run+0x364f>
   3d13f:	jmp    3d37f <meshopt_peers::run+0x365f>
   3d144:	movzbl 0x758(%rsp),%eax
   3d14c:	mov    %al,0x320(%rsp)
   3d153:	lea    -0x2bd87(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d15a:	lea    0x104427(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d161:	lea    0x1040a8(%rip),%r8        # 141210 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3e0>
   3d168:	lea    0x320(%rsp),%rdx
   3d170:	mov    $0x2b,%esi
   3d175:	call   *0x10bebd(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d17b:	mov    $0x8,%bpl
   3d17e:	jmp    3b64d <meshopt_peers::run+0x192d>
   3d183:	movzbl 0x758(%rsp),%eax
   3d18b:	mov    %al,0x320(%rsp)
   3d192:	lea    -0x2bdc6(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d199:	lea    0x1043e8(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d1a0:	lea    0x104039(%rip),%r8        # 1411e0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3b0>
   3d1a7:	lea    0x320(%rsp),%rdx
   3d1af:	mov    $0x2b,%esi
   3d1b4:	call   *0x10be7e(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d1ba:	movzbl 0x758(%rsp),%eax
   3d1c2:	mov    %al,0x320(%rsp)
   3d1c9:	lea    -0x2bdfd(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d1d0:	lea    0x1043b1(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d1d7:	lea    0x103f2a(%rip),%r8        # 141108 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2d8>
   3d1de:	lea    0x320(%rsp),%rdx
   3d1e6:	mov    $0x2b,%esi
   3d1eb:	call   *0x10be47(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d1f1:	movzbl 0x758(%rsp),%eax
   3d1f9:	mov    %al,0x320(%rsp)
   3d200:	lea    -0x2be34(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d207:	lea    0x10437a(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d20e:	lea    0x10402b(%rip),%r8        # 141240 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x410>
   3d215:	lea    0x320(%rsp),%rdx
   3d21d:	mov    $0x2b,%esi
   3d222:	call   *0x10be10(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d228:	movzbl 0x321(%rsp),%eax
   3d230:	mov    %al,0x250(%rsp)
   3d237:	lea    -0x2be6b(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d23e:	lea    0x104343(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d245:	lea    0x10406c(%rip),%r8        # 1412b8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x488>
   3d24c:	lea    0x250(%rsp),%rdx
   3d254:	mov    $0x2b,%esi
   3d259:	call   *0x10bdd9(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d25f:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d264:	mov    $0x8,%bpl
   3d267:	cmpq   $0x0,0x78(%rsp)
   3d26d:	je     3d27d <meshopt_peers::run+0x355d>
   3d26f:	mov    0xa8(%rsp),%rdi
   3d277:	call   *0x10bcf3(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d27d:	xor    %r15d,%r15d
   3d280:	movl   $0x0,0x18(%rsp)
   3d288:	test   %r12,%r12
   3d28b:	je     3d296 <meshopt_peers::run+0x3576>
   3d28d:	mov    %r13,%rdi
   3d290:	call   *0x10bcda(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d296:	cmpq   $0x0,0x30(%rsp)
   3d29c:	sete   %al
   3d29f:	or     %al,%r15b
   3d2a2:	jne    3d2af <meshopt_peers::run+0x358f>
   3d2a4:	mov    0x50(%rsp),%rdi
   3d2a9:	call   *0x10bcc1(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d2af:	mov    $0x1,%r15b
   3d2b2:	mov    $0x1,%r12b
   3d2b5:	mov    $0x1,%r13b
   3d2b8:	cmpb   $0x0,0x18(%rsp)
   3d2bd:	je     3d3dc <meshopt_peers::run+0x36bc>
   3d2c3:	mov    $0x1,%r15b
   3d2c6:	cmpq   $0x0,0x78(%rsp)
   3d2cc:	je     3d3d6 <meshopt_peers::run+0x36b6>
   3d2d2:	mov    0xa8(%rsp),%rdi
   3d2da:	call   *0x10bc90(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d2e0:	jmp    3d3d6 <meshopt_peers::run+0x36b6>
   3d2e5:	lea    0x103cfc(%rip),%rcx        # 140fe8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x1b8>
   3d2ec:	mov    0x70(%rsp),%rdx
   3d2f1:	call   *0x10bde1(%rip)        # 1490d8 <_DYNAMIC+0x370>
   3d2f7:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d2fc:	mov    0x8(%rsp),%rbx
   3d301:	mov    0x10(%rsp),%r11
   3d306:	jmp    3ddaf <meshopt_peers::run+0x408f>
   3d30b:	movzbl 0x328(%rsp),%eax
   3d313:	mov    %al,0x750(%rsp)
   3d31a:	lea    -0x2bf4e(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d321:	lea    0x104260(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d328:	lea    0x103e09(%rip),%r8        # 141138 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x308>
   3d32f:	lea    0x750(%rsp),%rdx
   3d337:	mov    $0x2b,%esi
   3d33c:	call   *0x10bcf6(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d342:	mov    $0x8,%bpl
   3d345:	lea    0x320(%rsp),%rdi
   3d34d:	call   36910 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3d352:	mov    $0x1,%al
   3d354:	mov    %rax,0x60(%rsp)
   3d359:	mov    $0x1,%r15b
   3d35c:	mov    $0x1,%r13b
   3d35f:	mov    0x88(%rsp),%r12
   3d367:	mov    0x80(%rsp),%rcx
   3d36f:	test   %rcx,%rcx
   3d372:	je     3d37f <meshopt_peers::run+0x365f>
   3d374:	mov    0x28(%rsp),%rdi
   3d379:	call   *0x10bbf1(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d37f:	test   %r13b,%r13b
   3d382:	je     3d394 <meshopt_peers::run+0x3674>
   3d384:	test   %r12,%r12
   3d387:	je     3d394 <meshopt_peers::run+0x3674>
   3d389:	mov    0x48(%rsp),%rdi
   3d38e:	call   *0x10bbdc(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d394:	test   %r15b,%r15b
   3d397:	mov    0x60(%rsp),%rax
   3d39c:	jne    3b84d <meshopt_peers::run+0x1b2d>
   3d3a2:	test   %al,%al
   3d3a4:	jne    3b64d <meshopt_peers::run+0x192d>
   3d3aa:	jmp    3b666 <meshopt_peers::run+0x1946>
   3d3af:	cmpq   $0x0,0x30(%rsp)
   3d3b5:	je     3d3c2 <meshopt_peers::run+0x36a2>
   3d3b7:	mov    0x50(%rsp),%rdi
   3d3bc:	call   *0x10bbae(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d3c2:	test   %r12,%r12
   3d3c5:	je     3d3d0 <meshopt_peers::run+0x36b0>
   3d3c7:	mov    %r13,%rdi
   3d3ca:	call   *0x10bba0(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d3d0:	mov    $0x8,%bpl
   3d3d3:	mov    $0x1,%r15b
   3d3d6:	mov    $0x1,%r12b
   3d3d9:	mov    $0x1,%r13b
   3d3dc:	cmpq   $0x0,0xc0(%rsp)
   3d3e5:	je     3d3f5 <meshopt_peers::run+0x36d5>
   3d3e7:	mov    0xe8(%rsp),%rdi
   3d3ef:	call   *0x10bb7b(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d3f5:	cmpq   $0x0,0x38(%rsp)
   3d3fb:	je     3d40b <meshopt_peers::run+0x36eb>
   3d3fd:	mov    0xd8(%rsp),%rdi
   3d405:	call   *0x10bb65(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d40b:	test   %r15b,%r15b
   3d40e:	mov    0x8(%rsp),%rbx
   3d413:	mov    0x58(%rsp),%r15
   3d418:	jne    3d0d0 <meshopt_peers::run+0x33b0>
   3d41e:	jmp    3d11c <meshopt_peers::run+0x33fc>
   3d423:	cmpq   $0x0,0x38(%rsp)
   3d429:	je     3d439 <meshopt_peers::run+0x3719>
   3d42b:	mov    0xd8(%rsp),%rdi
   3d433:	call   *0x10bb37(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d439:	mov    $0x1,%r13b
   3d43c:	jmp    3d44b <meshopt_peers::run+0x372b>
   3d43e:	mov    %eax,%ebp
   3d440:	xor    %r15d,%r15d
   3d443:	jmp    3d3dc <meshopt_peers::run+0x36bc>
   3d445:	mov    $0x1,%r13b
   3d448:	mov    %r15d,%ebp
   3d44b:	mov    0x8(%rsp),%rbx
   3d450:	mov    0x58(%rsp),%r15
   3d455:	mov    $0x1,%r12b
   3d458:	jmp    3d0d0 <meshopt_peers::run+0x33b0>
   3d45d:	mov    %eax,%ebp
   3d45f:	xor    %r15d,%r15d
   3d462:	xor    %r12d,%r12d
   3d465:	jmp    3d3dc <meshopt_peers::run+0x36bc>
   3d46a:	movzbl 0x321(%rsp),%ebp
   3d472:	cmpq   $0x0,0x30(%rsp)
   3d478:	je     3d2c3 <meshopt_peers::run+0x35a3>
   3d47e:	mov    0x50(%rsp),%rdi
   3d483:	call   *0x10bae7(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d489:	mov    $0x1,%r15b
   3d48c:	cmpq   $0x0,0x78(%rsp)
   3d492:	jne    3d2d2 <meshopt_peers::run+0x35b2>
   3d498:	jmp    3d3d6 <meshopt_peers::run+0x36b6>
   3d49d:	mov    0xd8(%rsp),%rax
   3d4a5:	mov    %rax,0x48(%rsp)
   3d4aa:	mov    0xe8(%rsp),%rax
   3d4b2:	mov    %rax,0x28(%rsp)
   3d4b7:	test   %r15,%r15
   3d4ba:	mov    0x38(%rsp),%r12
   3d4bf:	jne    3be92 <meshopt_peers::run+0x2172>
   3d4c5:	jmp    3bd91 <meshopt_peers::run+0x2071>
   3d4ca:	shr    $0x8,%ebp
   3d4cd:	mov    $0x1,%al
   3d4cf:	mov    %eax,0x18(%rsp)
   3d4d3:	jmp    3d4fc <meshopt_peers::run+0x37dc>
   3d4d5:	test   %r15,%r15
   3d4d8:	je     3d4e3 <meshopt_peers::run+0x37c3>
   3d4da:	mov    %rbp,%rdi
   3d4dd:	call   *0x10ba8d(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d4e3:	mov    $0x8,%bpl
   3d4e6:	cmpq   $0x0,0x20(%rsp)
   3d4ec:	je     3d4fc <meshopt_peers::run+0x37dc>
   3d4ee:	mov    0xa0(%rsp),%rdi
   3d4f6:	call   *0x10ba74(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d4fc:	xor    %r15d,%r15d
   3d4ff:	test   %r12,%r12
   3d502:	jne    3d28d <meshopt_peers::run+0x356d>
   3d508:	jmp    3d296 <meshopt_peers::run+0x3576>
   3d50d:	mov    $0xa,%bpl
   3d510:	jmp    3d0c0 <meshopt_peers::run+0x33a0>
   3d515:	mov    %rax,%rdi
   3d518:	mov    %rcx,%r15
   3d51b:	mov    %rsi,0x18(%rsp)
   3d520:	jmp    3d532 <meshopt_peers::run+0x3812>
   3d522:	mov    %rax,%rdi
   3d525:	mov    %rcx,%r15
   3d528:	mov    0x40(%rsp),%rax
   3d52d:	mov    %rax,0x18(%rsp)
   3d532:	lea    0x1039d7(%rip),%rax        # 140f10 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xe0>
   3d539:	mov    %rax,0x1e0(%rsp)
   3d541:	mov    %r15,%rsi
   3d544:	mov    0x18(%rsp),%rdx
   3d549:	mov    0x1e0(%rsp),%rcx
   3d551:	call   *0x10bb81(%rip)        # 1490d8 <_DYNAMIC+0x370>
   3d557:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d55c:	movb   $0x8,0x750(%rsp)
   3d564:	lea    -0x2c198(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d56b:	lea    0x104016(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d572:	lea    0x103b47(%rip),%r8        # 1410c0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x290>
   3d579:	lea    0x750(%rsp),%rdx
   3d581:	mov    $0x2b,%esi
   3d586:	call   *0x10baac(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d58c:	movb   $0x8,0x750(%rsp)
   3d594:	lea    -0x2c1c8(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d59b:	lea    0x103fe6(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d5a2:	lea    0x103bbf(%rip),%r8        # 141168 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x338>
   3d5a9:	lea    0x750(%rsp),%rdx
   3d5b1:	mov    $0x2b,%esi
   3d5b6:	call   *0x10ba7c(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d5bc:	movb   $0x2,0x750(%rsp)
   3d5c4:	lea    -0x2c1f8(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d5cb:	lea    0x103fb6(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d5d2:	lea    0x103cf7(%rip),%r8        # 1412d0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x4a0>
   3d5d9:	lea    0x750(%rsp),%rdx
   3d5e1:	mov    $0x2b,%esi
   3d5e6:	call   *0x10ba4c(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d5ec:	movb   $0x2,0x750(%rsp)
   3d5f4:	lea    -0x2c228(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d5fb:	lea    0x103f86(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d602:	lea    0x103acf(%rip),%r8        # 1410d8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2a8>
   3d609:	lea    0x750(%rsp),%rdx
   3d611:	mov    $0x2b,%esi
   3d616:	call   *0x10ba1c(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d61c:	mov    $0x4,%edi
   3d621:	mov    %r15,%rsi
   3d624:	call   *0x10b97e(%rip)        # 148fa8 <_DYNAMIC+0x240>
   3d62a:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d62f:	mov    $0x8,%bpl
   3d632:	cmpq   $0x0,0x20(%rsp)
   3d638:	je     3d4fc <meshopt_peers::run+0x37dc>
   3d63e:	jmp    3d4ee <meshopt_peers::run+0x37ce>
   3d643:	mov    %al,0x750(%rsp)
   3d64a:	lea    -0x2c27e(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d651:	lea    0x103f30(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d658:	lea    0x103c29(%rip),%r8        # 141288 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x458>
   3d65f:	lea    0x750(%rsp),%rdx
   3d667:	mov    $0x2b,%esi
   3d66c:	call   *0x10b9c6(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d672:	mov    %al,0x750(%rsp)
   3d679:	lea    -0x2c2ad(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d680:	lea    0x103f01(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d687:	lea    0x103c12(%rip),%r8        # 1412a0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x470>
   3d68e:	lea    0x750(%rsp),%rdx
   3d696:	mov    $0x2b,%esi
   3d69b:	call   *0x10b997(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d6a1:	mov    $0xa,%bpl
   3d6a4:	mov    $0x1,%r15b
   3d6a7:	cmpq   $0x0,0x78(%rsp)
   3d6ad:	jne    3d2d2 <meshopt_peers::run+0x35b2>
   3d6b3:	jmp    3d3d6 <meshopt_peers::run+0x36b6>
   3d6b8:	mov    $0x4,%edi
   3d6bd:	mov    %r15,%rsi
   3d6c0:	call   *0x10b8e2(%rip)        # 148fa8 <_DYNAMIC+0x240>
   3d6c6:	mov    $0x4,%edi
   3d6cb:	mov    %r15,%rsi
   3d6ce:	call   *0x10b8d4(%rip)        # 148fa8 <_DYNAMIC+0x240>
   3d6d4:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d6d9:	movb   $0x8,0x750(%rsp)
   3d6e1:	lea    -0x2c315(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d6e8:	lea    0x103e99(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d6ef:	lea    0x103b32(%rip),%r8        # 141228 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x3f8>
   3d6f6:	lea    0x750(%rsp),%rdx
   3d6fe:	mov    $0x2b,%esi
   3d703:	call   *0x10b92f(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d709:	mov    $0x8,%bpl
   3d70c:	cmpq   $0x0,0x78(%rsp)
   3d712:	je     3d27d <meshopt_peers::run+0x355d>
   3d718:	jmp    3d26f <meshopt_peers::run+0x354f>
   3d71d:	lea    0x10381c(%rip),%rcx        # 140f40 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x110>
   3d724:	mov    %r13,%rdi
   3d727:	mov    %r12,%rsi
   3d72a:	mov    0x188(%rsp),%rdx
   3d732:	call   *0x10b9a0(%rip)        # 1490d8 <_DYNAMIC+0x370>
   3d738:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d73d:	mov    $0xb,%bpl
   3d740:	jmp    3d4fc <meshopt_peers::run+0x37dc>
   3d745:	shr    $0x8,%ebp
   3d748:	jmp    3d4fc <meshopt_peers::run+0x37dc>
   3d74d:	mov    $0x4,%edi
   3d752:	mov    %rbp,%rsi
   3d755:	call   *0x10b84d(%rip)        # 148fa8 <_DYNAMIC+0x240>
   3d75b:	movb   $0x8,0x750(%rsp)
   3d763:	lea    -0x2c397(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d76a:	lea    0x103e17(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d771:	lea    0x1039a8(%rip),%r8        # 141120 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x2f0>
   3d778:	lea    0x750(%rsp),%rdx
   3d780:	mov    $0x2b,%esi
   3d785:	call   *0x10b8ad(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d78b:	mov    $0x4,%edi
   3d790:	mov    %rbp,%rsi
   3d793:	call   *0x10b80f(%rip)        # 148fa8 <_DYNAMIC+0x240>
   3d799:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d79e:	movb   $0x2,0x750(%rsp)
   3d7a6:	lea    -0x2c3da(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d7ad:	lea    0x103dd4(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d7b4:	lea    0x1039c5(%rip),%r8        # 141180 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x350>
   3d7bb:	lea    0x750(%rsp),%rdx
   3d7c3:	mov    $0x2b,%esi
   3d7c8:	call   *0x10b86a(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d7ce:	mov    $0x8,%bpl
   3d7d1:	mov    $0x1,%r15b
   3d7d4:	cmpq   $0x0,0x78(%rsp)
   3d7da:	jne    3d2d2 <meshopt_peers::run+0x35b2>
   3d7e0:	jmp    3d3d6 <meshopt_peers::run+0x36b6>
   3d7e5:	mov    $0x8,%bpl
   3d7e8:	mov    $0x1,%r15b
   3d7eb:	cmpq   $0x0,0x30(%rsp)
   3d7f1:	je     3d81e <meshopt_peers::run+0x3afe>
   3d7f3:	mov    0x50(%rsp),%rdi
   3d7f8:	call   *0x10b772(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d7fe:	mov    $0x1,%eax
   3d803:	mov    %rax,0x30(%rsp)
   3d808:	movl   $0x0,0x18(%rsp)
   3d810:	test   %r12,%r12
   3d813:	jne    3d28d <meshopt_peers::run+0x356d>
   3d819:	jmp    3d296 <meshopt_peers::run+0x3576>
   3d81e:	movq   $0x0,0x30(%rsp)
   3d827:	movl   $0x0,0x18(%rsp)
   3d82f:	test   %r12,%r12
   3d832:	jne    3d28d <meshopt_peers::run+0x356d>
   3d838:	jmp    3d296 <meshopt_peers::run+0x3576>
   3d83d:	mov    $0x8,%bpl
   3d840:	mov    $0x1,%r15b
   3d843:	test   %r12,%r12
   3d846:	je     3d3d6 <meshopt_peers::run+0x36b6>
   3d84c:	mov    %r13,%rdi
   3d84f:	jmp    3d2da <meshopt_peers::run+0x35ba>
   3d854:	lea    0x103685(%rip),%rcx        # 140ee0 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xb0>
   3d85b:	mov    0xe0(%rsp),%rdi
   3d863:	mov    %rbx,%rsi
   3d866:	mov    0x70(%rsp),%rdx
   3d86b:	call   *0x10b867(%rip)        # 1490d8 <_DYNAMIC+0x370>
   3d871:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d876:	lea    0x10367b(%rip),%rdx        # 140ef8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0xc8>
   3d87d:	mov    0x98(%rsp),%rdi
   3d885:	mov    %rdi,%rsi
   3d888:	call   *0x10b72a(%rip)        # 148fb8 <_DYNAMIC+0x250>
   3d88e:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d893:	mov    %eax,%ebp
   3d895:	cmpq   $0x0,0xc0(%rsp)
   3d89e:	je     3d8ab <meshopt_peers::run+0x3b8b>
   3d8a0:	mov    0x28(%rsp),%rdi
   3d8a5:	call   *0x10b6c5(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3d8ab:	mov    0x750(%rsp),%rax
   3d8b3:	mov    0x760(%rsp),%rcx
   3d8bb:	sub    0x758(%rsp),%rcx
   3d8c3:	mov    %rcx,0x358(%rax)
   3d8ca:	mov    %bpl,0x750(%rsp)
   3d8d2:	lea    -0x2c506(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3d8d9:	lea    0x103ca8(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3d8e0:	lea    0x103a19(%rip),%r8        # 141300 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x4d0>
   3d8e7:	lea    0x750(%rsp),%rdx
   3d8ef:	mov    $0x2b,%esi
   3d8f4:	call   *0x10b73e(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3d8fa:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d8ff:	lea    0x1035c2(%rip),%rdx        # 140ec8 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x98>
   3d906:	mov    0xd0(%rsp),%rdi
   3d90e:	mov    0x68(%rsp),%rsi
   3d913:	call   *0x10b69f(%rip)        # 148fb8 <_DYNAMIC+0x250>
   3d919:	jmp    3e0b5 <meshopt_peers::run+0x4395>
   3d91e:	mov    %rax,%rbx
   3d921:	cmpq   $0x0,0x130(%rsp)
   3d92a:	je     3da42 <meshopt_peers::run+0x3d22>
   3d930:	mov    0x138(%rsp),%r15
   3d938:	jmp    3d975 <meshopt_peers::run+0x3c55>
   3d93a:	jmp    3d93c <meshopt_peers::run+0x3c1c>
   3d93c:	mov    %rax,%rbx
   3d93f:	test   %r12,%r12
   3d942:	je     3d9fb <meshopt_peers::run+0x3cdb>
   3d948:	jmp    3dacc <meshopt_peers::run+0x3dac>
   3d94d:	mov    %rax,%rbx
   3d950:	cmpq   $0x0,0x78(%rsp)
   3d956:	je     3da13 <meshopt_peers::run+0x3cf3>
   3d95c:	jmp    3db85 <meshopt_peers::run+0x3e65>
   3d961:	jmp    3db07 <meshopt_peers::run+0x3de7>
   3d966:	mov    %rax,%rbx
   3d969:	cmpq   $0x0,0x38(%rsp)
   3d96f:	je     3da42 <meshopt_peers::run+0x3d22>
   3d975:	mov    %r15,%rdi
   3d978:	jmp    3da3c <meshopt_peers::run+0x3d1c>
   3d97d:	jmp    3db5f <meshopt_peers::run+0x3e3f>
   3d982:	mov    %rax,%rbx
   3d985:	jmp    3da2c <meshopt_peers::run+0x3d0c>
   3d98a:	mov    %rax,%rbx
   3d98d:	mov    %r12,0x88(%rsp)
   3d995:	mov    0xc0(%rsp),%rax
   3d99d:	mov    %rax,0x80(%rsp)
   3d9a5:	cmpq   $0x0,0x80(%rsp)
   3d9ae:	je     3da59 <meshopt_peers::run+0x3d39>
   3d9b4:	jmp    3dbc6 <meshopt_peers::run+0x3ea6>
   3d9b9:	mov    %rax,%rbx
   3d9bc:	cmpq   $0x0,0x80(%rsp)
   3d9c5:	je     3da59 <meshopt_peers::run+0x3d39>
   3d9cb:	jmp    3dbc6 <meshopt_peers::run+0x3ea6>
   3d9d0:	mov    %r15,0x58(%rsp)
   3d9d5:	mov    %rax,%rbx
   3d9d8:	jmp    3da42 <meshopt_peers::run+0x3d22>
   3d9da:	mov    %rax,%rbx
   3d9dd:	cmpq   $0x0,0x20(%rsp)
   3d9e3:	jne    3daa3 <meshopt_peers::run+0x3d83>
   3d9e9:	test   %r15,%r15
   3d9ec:	jne    3daba <meshopt_peers::run+0x3d9a>
   3d9f2:	test   %r12,%r12
   3d9f5:	jne    3dacc <meshopt_peers::run+0x3dac>
   3d9fb:	cmpq   $0x0,0x30(%rsp)
   3da01:	jne    3db6e <meshopt_peers::run+0x3e4e>
   3da07:	cmpq   $0x0,0x78(%rsp)
   3da0d:	jne    3db85 <meshopt_peers::run+0x3e65>
   3da13:	cmpq   $0x0,0xc0(%rsp)
   3da1c:	je     3da2c <meshopt_peers::run+0x3d0c>
   3da1e:	mov    0xe8(%rsp),%rdi
   3da26:	call   *0x10b544(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3da2c:	cmpq   $0x0,0x38(%rsp)
   3da32:	je     3da42 <meshopt_peers::run+0x3d22>
   3da34:	mov    0xd8(%rsp),%rdi
   3da3c:	call   *0x10b52e(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3da42:	cmpq   $0x0,0x58(%rsp)
   3da48:	jne    3da85 <meshopt_peers::run+0x3d65>
   3da4a:	cmpq   $0x0,0x80(%rsp)
   3da53:	jne    3dbc6 <meshopt_peers::run+0x3ea6>
   3da59:	cmpq   $0x0,0x88(%rsp)
   3da62:	jne    3dbe0 <meshopt_peers::run+0x3ec0>
   3da68:	test   %r14,%r14
   3da6b:	jne    3dbf4 <meshopt_peers::run+0x3ed4>
   3da71:	cmpq   $0x0,0x90(%rsp)
   3da7a:	jne    3dc0d <meshopt_peers::run+0x3eed>
   3da80:	jmp    3dc1b <meshopt_peers::run+0x3efb>
   3da85:	mov    0x180(%rsp),%rdi
   3da8d:	call   *0x10b4dd(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3da93:	cmpq   $0x0,0x80(%rsp)
   3da9c:	je     3da59 <meshopt_peers::run+0x3d39>
   3da9e:	jmp    3dbc6 <meshopt_peers::run+0x3ea6>
   3daa3:	mov    0xa0(%rsp),%rdi
   3daab:	call   *0x10b4bf(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dab1:	test   %r15,%r15
   3dab4:	je     3d9f2 <meshopt_peers::run+0x3cd2>
   3daba:	mov    %rbp,%rdi
   3dabd:	call   *0x10b4ad(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dac3:	test   %r12,%r12
   3dac6:	je     3d9fb <meshopt_peers::run+0x3cdb>
   3dacc:	mov    %r13,%rdi
   3dacf:	call   *0x10b49b(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dad5:	cmpq   $0x0,0x30(%rsp)
   3dadb:	je     3da07 <meshopt_peers::run+0x3ce7>
   3dae1:	jmp    3db6e <meshopt_peers::run+0x3e4e>
   3dae6:	jmp    3dae8 <meshopt_peers::run+0x3dc8>
   3dae8:	mov    %rax,%rbx
   3daeb:	test   %r14,%r14
   3daee:	je     3da71 <meshopt_peers::run+0x3d51>
   3daf0:	jmp    3dbf4 <meshopt_peers::run+0x3ed4>
   3daf5:	jmp    3dba7 <meshopt_peers::run+0x3e87>
   3dafa:	jmp    3dba7 <meshopt_peers::run+0x3e87>
   3daff:	mov    %rax,%rbx
   3db02:	jmp    3dc85 <meshopt_peers::run+0x3f65>
   3db07:	mov    %rax,%rbx
   3db0a:	cmpq   $0x0,0xc0(%rsp)
   3db13:	jne    3da1e <meshopt_peers::run+0x3cfe>
   3db19:	jmp    3da2c <meshopt_peers::run+0x3d0c>
   3db1e:	mov    %rax,%rbx
   3db21:	cmpq   $0x0,0x90(%rsp)
   3db2a:	jne    3dc0d <meshopt_peers::run+0x3eed>
   3db30:	jmp    3dc1b <meshopt_peers::run+0x3efb>
   3db35:	jmp    3db3f <meshopt_peers::run+0x3e1f>
   3db37:	mov    %rax,%rbx
   3db3a:	jmp    3dc1b <meshopt_peers::run+0x3efb>
   3db3f:	mov    %rax,%rbx
   3db42:	lea    0x320(%rsp),%rdi
   3db4a:	call   368d0 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   3db4f:	mov    %rbx,%rdi
   3db52:	call   13f3c0 <_Unwind_Resume@plt>
   3db57:	mov    %rax,%rbx
   3db5a:	jmp    3dc3a <meshopt_peers::run+0x3f1a>
   3db5f:	mov    %rax,%rbx
   3db62:	cmpq   $0x0,0x30(%rsp)
   3db68:	je     3da07 <meshopt_peers::run+0x3ce7>
   3db6e:	mov    0x50(%rsp),%rdi
   3db73:	call   *0x10b3f7(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3db79:	cmpq   $0x0,0x78(%rsp)
   3db7f:	je     3da13 <meshopt_peers::run+0x3cf3>
   3db85:	mov    0xa8(%rsp),%rdi
   3db8d:	call   *0x10b3dd(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3db93:	cmpq   $0x0,0xc0(%rsp)
   3db9c:	jne    3da1e <meshopt_peers::run+0x3cfe>
   3dba2:	jmp    3da2c <meshopt_peers::run+0x3d0c>
   3dba7:	mov    %rax,%rbx
   3dbaa:	lea    0x320(%rsp),%rdi
   3dbb2:	call   36910 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   3dbb7:	cmpq   $0x0,0x80(%rsp)
   3dbc0:	je     3da59 <meshopt_peers::run+0x3d39>
   3dbc6:	mov    0x28(%rsp),%rdi
   3dbcb:	call   *0x10b39f(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dbd1:	cmpq   $0x0,0x88(%rsp)
   3dbda:	je     3da68 <meshopt_peers::run+0x3d48>
   3dbe0:	mov    0x48(%rsp),%rdi
   3dbe5:	call   *0x10b385(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dbeb:	test   %r14,%r14
   3dbee:	je     3da71 <meshopt_peers::run+0x3d51>
   3dbf4:	mov    0xb0(%rsp),%rdi
   3dbfc:	call   *0x10b36e(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc02:	cmpq   $0x0,0x90(%rsp)
   3dc0b:	je     3dc1b <meshopt_peers::run+0x3efb>
   3dc0d:	mov    0xf0(%rsp),%rdi
   3dc15:	call   *0x10b355(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc1b:	mov    0x750(%rsp),%rax
   3dc23:	mov    0x760(%rsp),%rcx
   3dc2b:	sub    0x758(%rsp),%rcx
   3dc33:	mov    %rcx,0x358(%rax)
   3dc3a:	cmpq   $0x0,0x210(%rsp)
   3dc43:	je     3dc53 <meshopt_peers::run+0x3f33>
   3dc45:	mov    0x218(%rsp),%rdi
   3dc4d:	call   *0x10b31d(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc53:	test   %r14,%r14
   3dc56:	je     3e0c5 <meshopt_peers::run+0x43a5>
   3dc5c:	mov    0x10(%rsp),%rdi
   3dc61:	call   *0x10b309(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc67:	mov    %rbx,%rdi
   3dc6a:	call   13f3c0 <_Unwind_Resume@plt>
   3dc6f:	mov    %r12,0x10(%rsp)
   3dc74:	mov    %rax,%rbx
   3dc77:	test   %r14,%r14
   3dc7a:	je     3dc85 <meshopt_peers::run+0x3f65>
   3dc7c:	mov    %r15,%rdi
   3dc7f:	call   *0x10b2eb(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc85:	cmpq   $0x0,0x8(%rsp)
   3dc8b:	je     3e0c5 <meshopt_peers::run+0x43a5>
   3dc91:	mov    0x10(%rsp),%rdi
   3dc96:	call   *0x10b2d4(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3dc9c:	mov    %rbx,%rdi
   3dc9f:	call   13f3c0 <_Unwind_Resume@plt>
   3dca4:	mov    %edi,%r8d
   3dca7:	mov    0x4(%rbp,%rsi,4),%edi
   3dcab:	cmp    %edi,%r8d
   3dcae:	cmova  %r8d,%edi
   3dcb2:	inc    %rsi
   3dcb5:	cmp    %rsi,%rdx
   3dcb8:	jne    3dca4 <meshopt_peers::run+0x3f84>
   3dcba:	mov    %edi,%edx
   3dcbc:	cmp    %rdx,%r14
   3dcbf:	jbe    3dd88 <meshopt_peers::run+0x4068>
   3dcc5:	mov    %rcx,0x250(%rsp)
   3dccd:	mov    %r14,%rax
   3dcd0:	mov    %rax,%rdx
   3dcd3:	shr    $0x2,%rdx
   3dcd7:	add    %rax,%rdx
   3dcda:	jb     3dd21 <meshopt_peers::run+0x4001>
   3dcdc:	cmp    $0x1,%rdx
   3dce0:	mov    %rdx,%rcx
   3dce3:	adc    $0xffffffffffffffff,%rcx
   3dce7:	bsr    %rcx,%rcx
   3dceb:	not    %ecx
   3dced:	mov    $0xffffffffffffffff,%rsi
   3dcf4:	shr    %cl,%rsi
   3dcf7:	xor    %r15d,%r15d
   3dcfa:	cmp    $0x2,%rdx
   3dcfe:	cmovae %rsi,%r15
   3dd02:	cmp    $0xffffffffffffffff,%r15
   3dd06:	je     3dd21 <meshopt_peers::run+0x4001>
   3dd08:	lea    0x0(,%rax,4),%rcx
   3dd10:	shr    $0x3e,%rax
   3dd14:	sete   %al
   3dd17:	test   %rcx,%rcx
   3dd1a:	setns  %dl
   3dd1d:	test   %dl,%al
   3dd1f:	jne    3dd2e <meshopt_peers::run+0x400e>
   3dd21:	movb   $0x8,0x328(%rsp)
   3dd29:	jmp    3ddb6 <meshopt_peers::run+0x4096>
   3dd2e:	lea    0x1(%r15),%rbp
   3dd32:	mov    %rbp,0x700(%rsp)
   3dd3a:	pxor   %xmm0,%xmm0
   3dd3e:	movdqu %xmm0,0x708(%rsp)
   3dd47:	movq   $0x0,0x718(%rsp)
   3dd53:	lea    0x700(%rsp),%rsi
   3dd5b:	mov    %r11,%r12
   3dd5e:	mov    %r11,%rdi
   3dd61:	xor    %edx,%edx
   3dd63:	call   a3bd0 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3dd68:	cmp    $0xff,%al
   3dd6a:	je     3de26 <meshopt_peers::run+0x4106>
   3dd70:	mov    %al,0x328(%rsp)
   3dd77:	movq   $0xffffffffffffffff,0x320(%rsp)
   3dd83:	mov    %r12,%r11
   3dd86:	jmp    3ddc2 <meshopt_peers::run+0x40a2>
   3dd88:	shl    $0x2,%r15
   3dd8c:	xor    %edx,%edx
   3dd8e:	cmp    %rdx,%r15
   3dd91:	je     3dcc5 <meshopt_peers::run+0x3fa5>
   3dd97:	dec    %rax
   3dd9a:	cmp    %r14d,0x0(%rbp,%rdx,1)
   3dd9f:	lea    0x4(%rdx),%rdx
   3dda3:	jb     3dd8e <meshopt_peers::run+0x406e>
   3dda5:	mov    %rax,0x250(%rsp)
   3ddad:	mov    $0x1,%al
   3ddaf:	mov    %al,0x328(%rsp)
   3ddb6:	movq   $0xffffffffffffffff,0x320(%rsp)
   3ddc2:	mov    0x258(%rsp),%rax
   3ddca:	sub    0x250(%rsp),%rax
   3ddd2:	mov    %rax,0x358(%r11)
   3ddd9:	cmpq   $0xffffffffffffffff,0x320(%rsp)
   3dde2:	je     3e063 <meshopt_peers::run+0x4343>
   3dde8:	mov    0x330(%rsp),%rax
   3ddf0:	mov    %rax,0x20(%rbx)
   3ddf4:	movdqu 0x320(%rsp),%xmm0
   3ddfd:	movdqu %xmm0,0x10(%rbx)
   3de02:	mov    0x338(%rsp),%rax
   3de0a:	mov    %rax,0x8(%rbx)
   3de0e:	movl   $0x0,(%rbx)
   3de14:	add    $0x1988,%rsp
   3de1b:	pop    %rbx
   3de1c:	pop    %r12
   3de1e:	pop    %r13
   3de20:	pop    %r14
   3de22:	pop    %r15
   3de24:	pop    %rbp
   3de25:	ret
   3de26:	mov    0x130(%rsp),%rsi
   3de2e:	lea    0x320(%rsp),%rdi
   3de36:	call   *0x10b59c(%rip)        # 1493d8 <_DYNAMIC+0x670>
   3de3c:	mov    0x320(%rsp),%rsi
   3de44:	movzbl 0x328(%rsp),%eax
   3de4c:	cmp    $0xffffffffffffffff,%rsi
   3de50:	je     3dd70 <meshopt_peers::run+0x4050>
   3de56:	movzbl %al,%eax
   3de59:	movzbl 0x32f(%rsp),%ecx
   3de61:	shl    $0x10,%ecx
   3de64:	movzwl 0x32d(%rsp),%edx
   3de6c:	or     %ecx,%edx
   3de6e:	mov    0x329(%rsp),%ecx
   3de75:	mov    0x330(%rsp),%rdi
   3de7d:	mov    %rdi,0x10(%rsp)
   3de82:	shl    $0x28,%rdx
   3de86:	shl    $0x8,%rcx
   3de8a:	or     %rdx,%rcx
   3de8d:	or     %rax,%rcx
   3de90:	mov    %rcx,0x8(%rsp)
   3de95:	cmp    0x130(%rsp),%rsi
   3de9d:	mov    %r12,%r11
   3dea0:	jne    3df15 <meshopt_peers::run+0x41f5>
   3dea2:	mov    %rsi,0x90(%rsp)
   3deaa:	lea    0x788(%rsp),%rax
   3deb2:	cmpb   $0x0,0x2e0(%rsp)
   3deba:	je     3df87 <meshopt_peers::run+0x4267>
   3dec0:	mov    0x28(%r11),%rdx
   3dec4:	cmp    %rdx,%r15
   3dec7:	jae    3e09a <meshopt_peers::run+0x437a>
   3decd:	mov    0x20(%r11),%r9
   3ded1:	mov    0x190(%rsp),%rcx
   3ded9:	mov    0x198(%rsp),%r8
   3dee1:	sub    $0x8,%rsp
   3dee5:	lea    0x328(%rsp),%rdi
   3deed:	mov    0x10(%rsp),%rsi
   3def2:	mov    0x18(%rsp),%rdx
   3def7:	lea    0x258(%rsp),%r10
   3deff:	push   %r10
   3df01:	push   %rax
   3df02:	push   %r14
   3df04:	push   %r13
   3df06:	push   %rbp
   3df07:	call   389a0 <meshoptimizer_rs::remap::generate_kernel::<true, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3df0c:	add    $0x30,%rsp
   3df10:	jmp    3dfd7 <meshopt_peers::run+0x42b7>
   3df15:	movabs $0x3fffffffffffffff,%rax
   3df1f:	lea    0x0(,%rsi,4),%rcx
   3df27:	cmp    %rax,%rsi
   3df2a:	seta   %al
   3df2d:	test   %rcx,%rcx
   3df30:	sets   %dl
   3df33:	or     %al,%dl
   3df35:	mov    $0x8,%al
   3df37:	jne    3df66 <meshopt_peers::run+0x4246>
   3df39:	mov    %rsi,0x90(%rsp)
   3df41:	lea    0x700(%rsp),%rsi
   3df49:	mov    %r11,%rdi
   3df4c:	xor    %edx,%edx
   3df4e:	call   a3bd0 <<meshoptimizer_rs::workspace::Workspace>::prepare_cache>
   3df53:	cmp    $0xff,%al
   3df55:	mov    %r12,%r11
   3df58:	mov    0x90(%rsp),%rsi
   3df60:	je     3dea2 <meshopt_peers::run+0x4182>
   3df66:	mov    %al,0x328(%rsp)
   3df6d:	movq   $0xffffffffffffffff,0x320(%rsp)
   3df79:	test   %rsi,%rsi
   3df7c:	je     3ddc2 <meshopt_peers::run+0x40a2>
   3df82:	jmp    3e00e <meshopt_peers::run+0x42ee>
   3df87:	mov    0x28(%r11),%rdx
   3df8b:	cmp    %rdx,%r15
   3df8e:	jae    3e0a3 <meshopt_peers::run+0x4383>
   3df94:	mov    0x20(%r11),%r9
   3df98:	mov    0x190(%rsp),%rcx
   3dfa0:	mov    0x198(%rsp),%r8
   3dfa8:	sub    $0x8,%rsp
   3dfac:	lea    0x328(%rsp),%rdi
   3dfb4:	mov    0x10(%rsp),%rsi
   3dfb9:	mov    0x18(%rsp),%rdx
   3dfbe:	lea    0x258(%rsp),%r10
   3dfc6:	push   %r10
   3dfc8:	push   %rax
   3dfc9:	push   %r14
   3dfcb:	push   %r13
   3dfcd:	push   %rbp
   3dfce:	call   37730 <meshoptimizer_rs::remap::generate_kernel::<false, true, true, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#0}, meshoptimizer_rs::remap::generate_vertex_remap_custom<meshopt_peers::run::{closure#2}>::{closure#1}>>
   3dfd3:	add    $0x30,%rsp
   3dfd7:	cmpb   $0x1,0x320(%rsp)
   3dfdf:	jne    3e021 <meshopt_peers::run+0x4301>
   3dfe1:	movzbl 0x321(%rsp),%eax
   3dfe9:	mov    %r12,%r11
   3dfec:	mov    %al,0x328(%rsp)
   3dff3:	movq   $0xffffffffffffffff,0x320(%rsp)
   3dfff:	cmpq   $0x0,0x90(%rsp)
   3e008:	je     3ddc2 <meshopt_peers::run+0x40a2>
   3e00e:	mov    0x8(%rsp),%rdi
   3e013:	call   *0x10af57(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3e019:	mov    %r12,%r11
   3e01c:	jmp    3ddc2 <meshopt_peers::run+0x40a2>
   3e021:	mov    0x328(%rsp),%rax
   3e029:	mov    %r12,%r11
   3e02c:	mov    0x90(%rsp),%rcx
   3e034:	mov    %rcx,0x320(%rsp)
   3e03c:	mov    0x8(%rsp),%rcx
   3e041:	mov    %rcx,0x328(%rsp)
   3e049:	mov    0x10(%rsp),%rcx
   3e04e:	mov    %rcx,0x330(%rsp)
   3e056:	mov    %rax,0x338(%rsp)
   3e05e:	jmp    3ddc2 <meshopt_peers::run+0x40a2>
   3e063:	movzbl 0x328(%rsp),%eax
   3e06b:	mov    %al,0x750(%rsp)
   3e072:	lea    -0x2cca6(%rip),%rdi        # 113d3 <libm::math::generic::sqrt::RSQRT_TAB+0x1e1>
   3e079:	lea    0x103508(%rip),%rcx        # 141588 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x758>
   3e080:	lea    0x103111(%rip),%r8        # 141198 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x368>
   3e087:	lea    0x750(%rsp),%rdx
   3e08f:	mov    $0x2b,%esi
   3e094:	call   *0x10af9e(%rip)        # 149038 <_DYNAMIC+0x2d0>
   3e09a:	lea    0x102f77(%rip),%rcx        # 141018 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x1e8>
   3e0a1:	jmp    3e0aa <meshopt_peers::run+0x438a>
   3e0a3:	lea    0x102f56(%rip),%rcx        # 141000 <anon.c4fee8297d3d8a92cd2052a9725692c9.2.llvm.918650504978498608+0x1d0>
   3e0aa:	xor    %edi,%edi
   3e0ac:	mov    %rbp,%rsi
   3e0af:	call   *0x10b023(%rip)        # 1490d8 <_DYNAMIC+0x370>
   3e0b5:	ud2
   3e0b7:	mov    %rax,%rbx
   3e0ba:	cmpq   $0x0,0x90(%rsp)
   3e0c3:	jne    3e0cd <meshopt_peers::run+0x43ad>
   3e0c5:	mov    %rbx,%rdi
   3e0c8:	call   13f3c0 <_Unwind_Resume@plt>
   3e0cd:	mov    0x8(%rsp),%rdi
   3e0d2:	call   *0x10ae98(%rip)        # 148f70 <free@GLIBC_2.2.5>
   3e0d8:	mov    %rbx,%rdi
   3e0db:	call   13f3c0 <_Unwind_Resume@plt>
