
<artifacts>/screen/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000349a0 <meshopt_peers::run>:
   349a0:	push   %rbp
   349a1:	push   %r15
   349a3:	push   %r14
   349a5:	push   %r13
   349a7:	push   %r12
   349a9:	push   %rbx
   349aa:	sub    $0x1000,%rsp
   349b1:	movq   $0x0,(%rsp)
   349b9:	sub    $0x968,%rsp
   349c0:	mov    %rsi,0x290(%rsp)
   349c8:	mov    %rdx,0x298(%rsp)
   349d0:	mov    0x10(%rcx),%rbx
   349d4:	mov    0x8(%rcx),%r15
   349d8:	movb   $0x2,0x6e0(%rsp)
   349e0:	mov    %r15,0x6e8(%rsp)
   349e8:	mov    %rbx,0x6f0(%rsp)
   349f0:	movq   $0x0,0x718(%rsp)
   349fc:	mov    0x78(%rcx),%rax
   34a00:	movd   0x80(%rcx),%xmm0
   34a08:	mov    %rax,0x220(%rsp)
   34a10:	movd   %xmm0,0x228(%rsp)
   34a19:	movl   $0x0,0x22c(%rsp)
   34a24:	add    $0xfffffffffffffffb,%rdx
   34a28:	cmp    $0xe,%rdx
   34a2c:	ja     36afe <meshopt_peers::run+0x215e>
   34a32:	mov    %r8,%r12
   34a35:	mov    %rdi,%r14
   34a38:	lea    -0x24543(%rip),%rdi        # 104fc <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   34a3f:	movslq (%rdi,%rdx,4),%rdx
   34a43:	add    %rdi,%rdx
   34a46:	jmp    *%rdx
   34a48:	mov    $0x68636163,%eax
   34a4d:	xor    (%rsi),%eax
   34a4f:	movzbl 0x4(%rsi),%edx
   34a53:	xor    $0x65,%edx
   34a56:	or     %eax,%edx
   34a58:	je     35242 <meshopt_peers::run+0x8a2>
   34a5e:	mov    $0x63746566,%eax
   34a63:	xor    (%rsi),%eax
   34a65:	movzbl 0x4(%rsi),%edx
   34a69:	xor    $0x68,%edx
   34a6c:	or     %eax,%edx
   34a6e:	jne    36afe <meshopt_peers::run+0x215e>
   34a74:	mov    %rbx,%rax
   34a77:	shr    $0x20,%rax
   34a7b:	jne    36f51 <meshopt_peers::run+0x25b1>
   34a81:	mov    0x20(%rcx),%rsi
   34a85:	mov    0x28(%rcx),%rdx
   34a89:	lea    0x0(,%rbx,4),%rax
   34a91:	lea    (%rax,%rax,2),%rax
   34a95:	mov    %r15,0x350(%rsp)
   34a9d:	mov    %rax,0x358(%rsp)
   34aa5:	mov    %rbx,0x360(%rsp)
   34aad:	movq   $0xc,0x368(%rsp)
   34ab9:	movq   $0xc,0x370(%rsp)
   34ac5:	movq   $0x0,0x378(%rsp)
   34ad1:	lea    0x738(%rsp),%rdi
   34ad9:	lea    0x350(%rsp),%rcx
   34ae1:	mov    %r12,%r8
   34ae4:	call   *0xf2646(%rip)        # 127130 <_DYNAMIC+0x5d0>
   34aea:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34af3:	je     36b85 <meshopt_peers::run+0x21e5>
   34af9:	mov    0x768(%rsp),%rax
   34b01:	mov    %rax,0x380(%rsp)
   34b09:	movups 0x738(%rsp),%xmm0
   34b11:	movups 0x748(%rsp),%xmm1
   34b19:	movups 0x758(%rsp),%xmm2
   34b21:	movaps %xmm2,0x370(%rsp)
   34b29:	movaps %xmm1,0x360(%rsp)
   34b31:	movaps %xmm0,0x350(%rsp)
   34b39:	mov    0x360(%rsp),%rax
   34b41:	mov    %rax,0x18(%r14)
   34b45:	movups %xmm0,0x8(%r14)
   34b4a:	movups 0x368(%rsp),%xmm0
   34b52:	movups %xmm0,0x20(%r14)
   34b57:	mov    0x378(%rsp),%rax
   34b5f:	mov    %rax,0x30(%r14)
   34b63:	movl   $0x3,(%r14)
   34b6a:	jmp    3573a <meshopt_peers::run+0xd9a>
   34b6f:	movabs $0x7966696c706d6973,%rdx
   34b79:	xor    (%rsi),%rdx
   34b7c:	movabs $0x7970706f6c732d79,%rdi
   34b86:	xor    0x7(%rsi),%rdi
   34b8a:	or     %rdx,%rdi
   34b8d:	jne    36afe <meshopt_peers::run+0x215e>
   34b93:	movdqu 0x20(%rcx),%xmm1
   34b98:	movq   $0x0,0x1d0(%rsp)
   34ba4:	mov    %rax,0x170(%rsp)
   34bac:	movd   %xmm0,0x230(%rsp)
   34bb5:	movb   $0x0,0x365(%r12)
   34bbe:	movl   $0x0,0x361(%r12)
   34bca:	pxor   %xmm0,%xmm0
   34bce:	movdqu %xmm0,0x350(%r12)
   34bd8:	mov    0x348(%r12),%rax
   34be0:	mov    %rax,0xe0(%rsp)
   34be8:	mov    %rax,0xe8(%rsp)
   34bf0:	movdqu %xmm1,0x738(%rsp)
   34bf9:	lea    0x6e0(%rsp),%rax
   34c01:	mov    %rax,0x748(%rsp)
   34c09:	lea    0xe0(%rsp),%rax
   34c11:	mov    %rax,0x750(%rsp)
   34c19:	lea    0x1d0(%rsp),%rax
   34c21:	mov    %rax,0x758(%rsp)
   34c29:	lea    0x170(%rsp),%rax
   34c31:	mov    %rax,0x760(%rsp)
   34c39:	lea    0x230(%rsp),%rax
   34c41:	mov    %rax,0x768(%rsp)
   34c49:	mov    %r12,0x770(%rsp)
   34c51:	movq   $0x0,0x778(%rsp)
   34c5d:	lea    0x350(%rsp),%rdi
   34c65:	lea    0x738(%rsp),%rsi
   34c6d:	call   83510 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   34c72:	mov    0xe8(%rsp),%rax
   34c7a:	sub    0xe0(%rsp),%rax
   34c82:	mov    %rax,0x358(%r12)
   34c8a:	mov    0x350(%rsp),%rax
   34c92:	movzbl 0x358(%rsp),%ecx
   34c9a:	cmp    $0xffffffffffffffff,%rax
   34c9e:	je     34cc2 <meshopt_peers::run+0x322>
   34ca0:	mov    0x368(%rsp),%rdx
   34ca8:	mov    %rdx,0x2d8(%rsp)
   34cb0:	movdqu 0x359(%rsp),%xmm0
   34cb9:	movdqu %xmm0,0x2c9(%rsp)
   34cc2:	mov    %cl,0x2c8(%rsp)
   34cc9:	mov    %rax,0x2c0(%rsp)
   34cd1:	jmp    3513c <meshopt_peers::run+0x79c>
   34cd6:	movdqu (%rsi),%xmm0
   34cda:	movdqu 0x3(%rsi),%xmm1
   34cdf:	pcmpeqb -0x26c97(%rip),%xmm1        # e050 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   34ce7:	pcmpeqb -0x2711f(%rip),%xmm0        # dbd0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   34cef:	pand   %xmm1,%xmm0
   34cf3:	pmovmskb %xmm0,%eax
   34cf7:	cmp    $0xffff,%eax
   34cfc:	jne    36afe <meshopt_peers::run+0x215e>
   34d02:	mov    0x40(%rcx),%rax
   34d06:	test   %rbx,%rbx
   34d09:	setne  %dl
   34d0c:	lea    0x0(,%rbx,4),%rsi
   34d14:	cmp    %rax,%rsi
   34d17:	seta   %sil
   34d1b:	and    %dl,%sil
   34d1e:	cmp    $0x1,%sil
   34d22:	je     36f08 <meshopt_peers::run+0x2568>
   34d28:	mov    0x20(%rcx),%rsi
   34d2c:	mov    0x28(%rcx),%rdx
   34d30:	mov    0x38(%rcx),%rcx
   34d34:	movb   $0x0,0x738(%rsp)
   34d3c:	mov    %rcx,0x740(%rsp)
   34d44:	mov    %rax,0x748(%rsp)
   34d4c:	mov    %rbx,0x750(%rsp)
   34d54:	movq   $0x4,0x758(%rsp)
   34d60:	movq   $0x4,0x760(%rsp)
   34d6c:	pxor   %xmm0,%xmm0
   34d70:	movdqu %xmm0,0x768(%rsp)
   34d79:	sub    $0x30,%rsp
   34d7d:	mov    %r12,0x20(%rsp)
   34d82:	lea    0x250(%rsp),%rax
   34d8a:	mov    %rax,0x18(%rsp)
   34d8f:	movq   $0x0,0x8(%rsp)
   34d98:	movq   $0x4,(%rsp)
   34da0:	lea    -0x26d07(%rip),%r9        # e0a0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   34da7:	lea    0x2f0(%rsp),%rdi
   34daf:	lea    0x710(%rsp),%rcx
   34db7:	lea    0x768(%rsp),%r8
   34dbf:	jmp    3512b <meshopt_peers::run+0x78b>
   34dc4:	movabs $0x7966696c706d6973,%rax
   34dce:	cmp    %rax,(%rsi)
   34dd1:	je     350de <meshopt_peers::run+0x73e>
   34dd7:	movabs $0x776172647265766f,%rax
   34de1:	cmp    %rax,(%rsi)
   34de4:	je     35600 <meshopt_peers::run+0xc60>
   34dea:	movabs $0x7374656c6873656d,%rax
   34df4:	cmp    %rax,(%rsi)
   34df7:	jne    36afe <meshopt_peers::run+0x215e>
   34dfd:	mov    0x20(%rcx),%rsi
   34e01:	mov    0x28(%rcx),%rdx
   34e05:	movq   $0x7c,0x360(%rsp)
   34e11:	movq   $0x0,0x354(%rsp)
   34e1d:	movl   $0x1,0x350(%rsp)
   34e28:	lea    0x350(%rsp),%rax
   34e30:	lea    0x738(%rsp),%rdi
   34e38:	lea    0x6e0(%rsp),%r8
   34e40:	mov    $0x40,%r9d
   34e46:	mov    %rbx,%rcx
   34e49:	push   $0x2
   34e4b:	push   %r12
   34e4d:	push   %rax
   34e4e:	push   $0x7c
   34e50:	call   b1aa0 <meshoptimizer_rs::meshlet::build>
   34e55:	add    $0x20,%rsp
   34e59:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34e62:	je     36c05 <meshopt_peers::run+0x2265>
   34e68:	mov    0x778(%rsp),%rax
   34e70:	mov    %rax,0x390(%rsp)
   34e78:	movups 0x738(%rsp),%xmm0
   34e80:	movups 0x748(%rsp),%xmm1
   34e88:	movups 0x758(%rsp),%xmm2
   34e90:	movups 0x768(%rsp),%xmm3
   34e98:	movaps %xmm3,0x380(%rsp)
   34ea0:	movaps %xmm2,0x370(%rsp)
   34ea8:	movaps %xmm1,0x360(%rsp)
   34eb0:	movaps %xmm0,0x350(%rsp)
   34eb8:	mov    0x358(%rsp),%rbx
   34ec0:	mov    0x360(%rsp),%r12
   34ec8:	test   %r12,%r12
   34ecb:	je     3565d <meshopt_peers::run+0xcbd>
   34ed1:	mov    %r12,%r15
   34ed4:	shl    $0x4,%r15
   34ed8:	mov    %r15,%rdi
   34edb:	call   *0xf1e77(%rip)        # 126d58 <malloc@GLIBC_2.2.5>
   34ee1:	test   %rax,%rax
   34ee4:	je     36fa4 <meshopt_peers::run+0x2604>
   34eea:	mov    %r12d,%ecx
   34eed:	and    $0x3,%ecx
   34ef0:	cmp    $0x4,%r12
   34ef4:	jae    35664 <meshopt_peers::run+0xcc4>
   34efa:	xor    %edx,%edx
   34efc:	jmp    356b8 <meshopt_peers::run+0xd18>
   34f01:	movabs $0x65646f6365642d78,%rax
   34f0b:	movabs $0x652d786574726576,%rdx
   34f15:	xor    (%rsi),%rdx
   34f18:	lea    0x9010000(%rax),%rdi
   34f1f:	xor    0x5(%rsi),%rdi
   34f23:	or     %rdx,%rdi
   34f26:	je     3517a <meshopt_peers::run+0x7da>
   34f2c:	movabs $0x642d786574726576,%rdx
   34f36:	xor    (%rsi),%rdx
   34f39:	xor    0x5(%rsi),%rax
   34f3d:	or     %rdx,%rax
   34f40:	jne    36afe <meshopt_peers::run+0x215e>
   34f46:	mov    0x50(%rcx),%r9
   34f4a:	lea    0x738(%rsp),%rdi
   34f52:	mov    $0xc,%r8d
   34f58:	xor    %esi,%esi
   34f5a:	xor    %edx,%edx
   34f5c:	mov    %rcx,%rax
   34f5f:	mov    %rbx,%rcx
   34f62:	push   %r12
   34f64:	push   0x58(%rax)
   34f67:	call   b0fc0 <meshoptimizer_rs::codec::allocate>
   34f6c:	add    $0x10,%rsp
   34f70:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   34f79:	jne    3521d <meshopt_peers::run+0x87d>
   34f7f:	movzbl 0x740(%rsp),%eax
   34f87:	mov    %al,0x350(%rsp)
   34f8e:	lea    -0x249e2(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   34f95:	lea    0xead04(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   34f9c:	lea    0xea9cd(%rip),%r8        # 11f970 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   34fa3:	lea    0x350(%rsp),%rdx
   34fab:	mov    $0x2b,%esi
   34fb0:	call   *0xf1e7a(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   34fb6:	movabs $0x6e652d7865646e69,%rax
   34fc0:	xor    (%rsi),%rax
   34fc3:	mov    0x8(%rsi),%edx
   34fc6:	xor    $0x65646f63,%rdx
   34fcd:	or     %rax,%rdx
   34fd0:	je     351f0 <meshopt_peers::run+0x850>
   34fd6:	movabs $0x65642d7865646e69,%rax
   34fe0:	xor    (%rsi),%rax
   34fe3:	mov    0x8(%rsi),%edx
   34fe6:	xor    $0x65646f63,%rdx
   34fed:	or     %rax,%rdx
   34ff0:	jne    36afe <meshopt_peers::run+0x215e>
   34ff6:	mov    0x28(%rcx),%rax
   34ffa:	mov    0x68(%rcx),%r9
   34ffe:	lea    0x738(%rsp),%rdi
   35006:	mov    $0x4,%r8d
   3500c:	mov    $0x1,%esi
   35011:	xor    %edx,%edx
   35013:	mov    %rcx,%r10
   35016:	mov    %rax,%rcx
   35019:	push   %r12
   3501b:	push   0x70(%r10)
   3501f:	call   b0fc0 <meshoptimizer_rs::codec::allocate>
   35024:	add    $0x10,%rsp
   35028:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35031:	jne    3521d <meshopt_peers::run+0x87d>
   35037:	movzbl 0x740(%rsp),%eax
   3503f:	mov    %al,0x350(%rsp)
   35046:	lea    -0x24a9a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3504d:	lea    0xeac4c(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   35054:	lea    0xea8e5(%rip),%r8        # 11f940 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x138>
   3505b:	lea    0x350(%rsp),%rdx
   35063:	mov    $0x2b,%esi
   35068:	call   *0xf1dc2(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   3506e:	movabs $0x2d72657473756c63,%rax
   35078:	xor    (%rsi),%rax
   3507b:	movabs $0x646f6c2d72657473,%rdx
   35085:	xor    0x3(%rsi),%rdx
   35089:	or     %rax,%rdx
   3508c:	jne    36afe <meshopt_peers::run+0x215e>
   35092:	mov    %r14,0x288(%rsp)
   3509a:	test   %rbx,%rbx
   3509d:	je     352a4 <meshopt_peers::run+0x904>
   350a3:	mov    %rcx,%rbp
   350a6:	lea    0x0(,%rbx,4),%rax
   350ae:	lea    (%rax,%rax,2),%r13
   350b2:	mov    %r13,%rdi
   350b5:	call   *0xf1c9d(%rip)        # 126d58 <malloc@GLIBC_2.2.5>
   350bb:	test   %rax,%rax
   350be:	je     36f81 <meshopt_peers::run+0x25e1>
   350c4:	mov    %rax,%r14
   350c7:	mov    %rax,%rdi
   350ca:	mov    %r15,%rsi
   350cd:	mov    %r13,%rdx
   350d0:	call   *0xf1ca2(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   350d6:	mov    %rbp,%rcx
   350d9:	jmp    352aa <meshopt_peers::run+0x90a>
   350de:	mov    0x20(%rcx),%rsi
   350e2:	mov    0x28(%rcx),%rdx
   350e6:	movb   $0x2,0x738(%rsp)
   350ee:	sub    $0x30,%rsp
   350f2:	mov    %r12,0x20(%rsp)
   350f7:	lea    0x250(%rsp),%rax
   350ff:	mov    %rax,0x18(%rsp)
   35104:	pxor   %xmm0,%xmm0
   35108:	movdqu %xmm0,(%rsp)
   3510d:	lea    0x2f0(%rsp),%rdi
   35115:	lea    0x710(%rsp),%rcx
   3511d:	lea    0x768(%rsp),%r8
   35125:	mov    $0x4,%r9d
   3512b:	call   bc740 <meshoptimizer_rs::simplify::allocating>
   35130:	add    $0x30,%rsp
   35134:	mov    0x2c0(%rsp),%rax
   3513c:	cmp    $0xffffffffffffffff,%rax
   35140:	je     36ac7 <meshopt_peers::run+0x2127>
   35146:	mov    0x2d0(%rsp),%rax
   3514e:	mov    %rax,0x18(%r14)
   35152:	movups 0x2c0(%rsp),%xmm0
   3515a:	movups %xmm0,0x8(%r14)
   3515f:	movss  0x2d8(%rsp),%xmm0
   35168:	movss  %xmm0,0x4(%r14)
   3516e:	movl   $0x2,(%r14)
   35175:	jmp    3573a <meshopt_peers::run+0xd9a>
   3517a:	lea    0x0(,%rbx,4),%rax
   35182:	lea    (%rax,%rax,2),%rdx
   35186:	lea    0x738(%rsp),%rdi
   3518e:	mov    $0xc,%r8d
   35194:	mov    %r15,%rsi
   35197:	mov    %rbx,%rcx
   3519a:	mov    $0x1,%r9d
   351a0:	push   %r12
   351a2:	push   $0x2
   351a4:	call   *0xf1f8e(%rip)        # 127138 <_DYNAMIC+0x5d8>
   351aa:	add    $0x10,%rsp
   351ae:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   351b7:	jne    3521d <meshopt_peers::run+0x87d>
   351b9:	movzbl 0x740(%rsp),%eax
   351c1:	mov    %al,0x350(%rsp)
   351c8:	lea    -0x24c1c(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   351cf:	lea    0xeaaca(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   351d6:	lea    0xea7ab(%rip),%r8        # 11f988 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   351dd:	lea    0x350(%rsp),%rdx
   351e5:	mov    $0x2b,%esi
   351ea:	call   *0xf1c40(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   351f0:	mov    0x20(%rcx),%rsi
   351f4:	mov    0x28(%rcx),%rdx
   351f8:	lea    0x738(%rsp),%rdi
   35200:	mov    $0x1,%ecx
   35205:	mov    %r12,%r8
   35208:	call   *0xf1f32(%rip)        # 127140 <_DYNAMIC+0x5e0>
   3520e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35217:	je     36c3c <meshopt_peers::run+0x229c>
   3521d:	mov    0x748(%rsp),%rax
   35225:	mov    %rax,0x18(%r14)
   35229:	movups 0x738(%rsp),%xmm0
   35231:	movups %xmm0,0x8(%r14)
   35236:	movl   $0x0,(%r14)
   3523d:	jmp    3573a <meshopt_peers::run+0xd9a>
   35242:	mov    0x20(%rcx),%rsi
   35246:	mov    0x28(%rcx),%rdx
   3524a:	lea    0x738(%rsp),%rdi
   35252:	mov    %rbx,%rcx
   35255:	mov    %r12,%r8
   35258:	call   *0xf1eea(%rip)        # 127148 <_DYNAMIC+0x5e8>
   3525e:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35267:	jne    35638 <meshopt_peers::run+0xc98>
   3526d:	movzbl 0x740(%rsp),%eax
   35275:	mov    %al,0x350(%rsp)
   3527c:	lea    -0x24cd0(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   35283:	lea    0xeaa16(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3528a:	lea    0xea76f(%rip),%r8        # 11fa00 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   35291:	lea    0x350(%rsp),%rdx
   35299:	mov    $0x2b,%esi
   3529e:	call   *0xf1b8c(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   352a4:	mov    $0x4,%r14d
   352aa:	movq   $0x0,0x158(%rsp)
   352b6:	movq   $0x4,0x160(%rsp)
   352c2:	movq   $0x0,0x168(%rsp)
   352ce:	movl   $0x0,0x9c(%rsp)
   352d9:	movq   $0x80,0x1d0(%rsp)
   352e5:	movq   $0x2a,0x1d8(%rsp)
   352f1:	movq   $0x80,0x1e0(%rsp)
   352fd:	movq   $0x10,0x1e8(%rsp)
   35309:	movaps -0x27420(%rip),%xmm0        # def0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   35310:	movups %xmm0,0x1f0(%rsp)
   35318:	movaps -0x2736f(%rip),%xmm0        # dfb0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   3531f:	movups %xmm0,0x200(%rsp)
   35327:	movabs $0x100010100000101,%rax
   35331:	mov    %rax,0x210(%rsp)
   35339:	movl   $0x0,0x218(%rsp)
   35344:	movb   $0x1,0x21c(%rsp)
   3534c:	mov    0x20(%rcx),%r13
   35350:	mov    0x28(%rcx),%r15
   35354:	mov    %r13,0x308(%rsp)
   3535c:	mov    %r15,0x310(%rsp)
   35364:	mov    %r14,0x50(%rsp)
   35369:	mov    %r14,0x318(%rsp)
   35371:	mov    %rbx,0x20(%rsp)
   35376:	mov    %rbx,0x320(%rsp)
   3537e:	movb   $0x2,0x2c0(%rsp)
   35386:	movq   $0x4,0x328(%rsp)
   35392:	xorps  %xmm0,%xmm0
   35395:	movups %xmm0,0x330(%rsp)
   3539d:	movl   $0x0,0x348(%rsp)
   353a8:	movb   $0x0,0x365(%r12)
   353b1:	movl   $0x0,0x361(%r12)
   353bd:	movups %xmm0,0x350(%r12)
   353c6:	mov    0x348(%r12),%rbx
   353ce:	mov    %r12,%rdi
   353d1:	xor    %esi,%esi
   353d3:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   353d8:	mov    %eax,%ebp
   353da:	cmp    $0xff,%al
   353dc:	jne    37055 <meshopt_peers::run+0x26b5>
   353e2:	mov    %r12,0x738(%rsp)
   353ea:	mov    %rbx,0x740(%rsp)
   353f2:	mov    %rbx,0x748(%rsp)
   353fa:	movq   $0x0,0x750(%rsp)
   35406:	movl   $0x0,0x758(%rsp)
   35411:	movl   $0x0,0x95c(%rsp)
   3541c:	movb   $0x1,0x1960(%rsp)
   35424:	lea    0x2c0(%rsp),%rdi
   3542c:	lea    0x1d0(%rsp),%rsi
   35434:	lea    0x738(%rsp),%rdx
   3543c:	mov    0x20(%rsp),%rbx
   35441:	call   *0xf1d09(%rip)        # 127150 <_DYNAMIC+0x5f0>
   35447:	mov    %eax,%ebp
   35449:	cmp    $0xff,%al
   3544b:	jne    37036 <meshopt_peers::run+0x2696>
   35451:	movzbl 0x1960(%rsp),%r12d
   3545a:	movb   $0x2,0x350(%rsp)
   35462:	mov    0x50(%rsp),%rax
   35467:	mov    %rax,0x358(%rsp)
   3546f:	mov    %rbx,0x360(%rsp)
   35477:	movq   $0x0,0x388(%rsp)
   35483:	lea    0xe0(%rsp),%rdi
   3548b:	lea    0x350(%rsp),%rsi
   35493:	lea    0x738(%rsp),%rdx
   3549b:	call   *0xf1cb7(%rip)        # 127158 <_DYNAMIC+0x5f8>
   354a1:	mov    0xe0(%rsp),%r14
   354a9:	movzbl 0xe8(%rsp),%ebp
   354b1:	cmp    $0xffffffffffffffff,%r14
   354b5:	je     3559e <meshopt_peers::run+0xbfe>
   354bb:	mov    %r14,0x150(%rsp)
   354c3:	movzbl %bpl,%eax
   354c7:	movzbl 0xef(%rsp),%ecx
   354cf:	shl    $0x10,%ecx
   354d2:	movzwl 0xed(%rsp),%edx
   354da:	or     %ecx,%edx
   354dc:	mov    0xe9(%rsp),%ecx
   354e3:	shl    $0x28,%rdx
   354e7:	shl    $0x8,%rcx
   354eb:	or     %rdx,%rcx
   354ee:	or     %rax,%rcx
   354f1:	mov    %rcx,0xc8(%rsp)
   354f9:	mov    0x750(%rsp),%rsi
   35501:	add    %rbx,%rsi
   35504:	jb     36bbc <meshopt_peers::run+0x221c>
   3550a:	mov    0xf0(%rsp),%rax
   35512:	mov    %rax,0x1c8(%rsp)
   3551a:	mov    %rsi,0x750(%rsp)
   35522:	mov    0x738(%rsp),%rdi
   3552a:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3552f:	mov    %eax,%ebp
   35531:	cmp    $0xff,%al
   35533:	jne    35585 <meshopt_peers::run+0xbe5>
   35535:	mov    $0x1,%eax
   3553a:	mov    %rax,0x90(%rsp)
   35542:	test   %rbx,%rbx
   35545:	jne    35ba3 <meshopt_peers::run+0x1203>
   3554b:	mov    0x738(%rsp),%rdi
   35553:	mov    0x750(%rsp),%rsi
   3555b:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35560:	mov    %eax,%ebp
   35562:	mov    $0x1,%r14b
   35565:	cmp    $0xff,%al
   35567:	je     3574c <meshopt_peers::run+0xdac>
   3556d:	test   %rbx,%rbx
   35570:	je     35580 <meshopt_peers::run+0xbe0>
   35572:	mov    0x90(%rsp),%rdi
   3557a:	call   *0xf17e8(%rip)        # 126d68 <free@GLIBC_2.2.5>
   35580:	test   %r14b,%r14b
   35583:	je     3559e <meshopt_peers::run+0xbfe>
   35585:	cmpq   $0x0,0x150(%rsp)
   3558e:	je     3559e <meshopt_peers::run+0xbfe>
   35590:	mov    0xc8(%rsp),%rdi
   35598:	call   *0xf17ca(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3559e:	mov    0x738(%rsp),%rax
   355a6:	mov    0x748(%rsp),%rcx
   355ae:	sub    0x740(%rsp),%rcx
   355b6:	mov    %rcx,0x358(%rax)
   355bd:	cmp    $0xff,%bpl
   355c1:	jne    37055 <meshopt_peers::run+0x26b5>
   355c7:	mov    0x168(%rsp),%rax
   355cf:	mov    0x288(%rsp),%rcx
   355d7:	mov    %rax,0x18(%rcx)
   355db:	movups 0x158(%rsp),%xmm0
   355e3:	movups %xmm0,0x8(%rcx)
   355e7:	movl   $0x1,(%rcx)
   355ed:	test   %rbx,%rbx
   355f0:	je     3573a <meshopt_peers::run+0xd9a>
   355f6:	mov    0x50(%rsp),%rdi
   355fb:	jmp    35734 <meshopt_peers::run+0xd94>
   35600:	mov    0x20(%rcx),%rsi
   35604:	mov    0x28(%rcx),%rdx
   35608:	lea    0x738(%rsp),%rdi
   35610:	lea    0x6e0(%rsp),%rcx
   35618:	movd   -0x259bc(%rip),%xmm0        # fc64 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x138>
   35620:	mov    %r12,%r8
   35623:	call   *0xf1b37(%rip)        # 127160 <_DYNAMIC+0x600>
   35629:	cmpq   $0xffffffffffffffff,0x738(%rsp)
   35632:	je     36cdc <meshopt_peers::run+0x233c>
   35638:	mov    0x748(%rsp),%rax
   35640:	mov    %rax,0x18(%r14)
   35644:	movups 0x738(%rsp),%xmm0
   3564c:	movups %xmm0,0x8(%r14)
   35651:	movl   $0x1,(%r14)
   35658:	jmp    3573a <meshopt_peers::run+0xd9a>
   3565d:	mov    $0x4,%eax
   35662:	jmp    356e1 <meshopt_peers::run+0xd41>
   35664:	mov    %r12,%rsi
   35667:	and    $0xfffffffffffffffc,%rsi
   3566b:	mov    $0x30,%edi
   35670:	xor    %edx,%edx
   35672:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   35680:	movups -0x30(%rbx,%rdi,1),%xmm0
   35685:	movups %xmm0,-0x30(%rax,%rdi,1)
   3568a:	movups -0x20(%rbx,%rdi,1),%xmm0
   3568f:	movups %xmm0,-0x20(%rax,%rdi,1)
   35694:	movups -0x10(%rbx,%rdi,1),%xmm0
   35699:	movups %xmm0,-0x10(%rax,%rdi,1)
   3569e:	movups (%rbx,%rdi,1),%xmm0
   356a2:	movups %xmm0,(%rax,%rdi,1)
   356a6:	add    $0x4,%rdx
   356aa:	add    $0x40,%rdi
   356ae:	cmp    %rdx,%rsi
   356b1:	jne    35680 <meshopt_peers::run+0xce0>
   356b3:	test   %rcx,%rcx
   356b6:	je     356e1 <meshopt_peers::run+0xd41>
   356b8:	shl    $0x4,%rdx
   356bc:	mov    %rax,%rsi
   356bf:	add    %rdx,%rsi
   356c2:	add    %rbx,%rdx
   356c5:	shl    $0x4,%ecx
   356c8:	xor    %edi,%edi
   356ca:	nopw   0x0(%rax,%rax,1)
   356d0:	movups (%rdx,%rdi,1),%xmm0
   356d4:	movups %xmm0,(%rsi,%rdi,1)
   356d8:	add    $0x10,%rdi
   356dc:	cmp    %rdi,%rcx
   356df:	jne    356d0 <meshopt_peers::run+0xd30>
   356e1:	mov    0x378(%rsp),%rcx
   356e9:	mov    %rcx,0x30(%r14)
   356ed:	movups 0x368(%rsp),%xmm0
   356f5:	movups %xmm0,0x20(%r14)
   356fa:	movaps 0x380(%rsp),%xmm0
   35702:	movups %xmm0,0x38(%r14)
   35707:	mov    0x390(%rsp),%rcx
   3570f:	mov    %rcx,0x48(%r14)
   35713:	mov    %r12,0x8(%r14)
   35717:	mov    %rax,0x10(%r14)
   3571b:	mov    %r12,0x18(%r14)
   3571f:	movl   $0x4,(%r14)
   35726:	cmpq   $0x0,0x350(%rsp)
   3572f:	je     3573a <meshopt_peers::run+0xd9a>
   35731:	mov    %rbx,%rdi
   35734:	call   *0xf162e(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3573a:	add    $0x1968,%rsp
   35741:	pop    %rbx
   35742:	pop    %r12
   35744:	pop    %r13
   35746:	pop    %r14
   35748:	pop    %r15
   3574a:	pop    %rbp
   3574b:	ret
   3574c:	movb   $0x2,0xe0(%rsp)
   35754:	mov    0x50(%rsp),%rax
   35759:	mov    %rax,0xe8(%rsp)
   35761:	mov    %rbx,0xf0(%rsp)
   35769:	movq   $0x0,0x118(%rsp)
   35775:	movups 0x210(%rsp),%xmm0
   3577d:	movaps %xmm0,0x390(%rsp)
   35785:	movups 0x1d0(%rsp),%xmm0
   3578d:	movups 0x1e0(%rsp),%xmm1
   35795:	movups 0x1f0(%rsp),%xmm2
   3579d:	movups 0x200(%rsp),%xmm3
   357a5:	movaps %xmm3,0x380(%rsp)
   357ad:	movaps %xmm2,0x370(%rsp)
   357b5:	movaps %xmm1,0x360(%rsp)
   357bd:	movaps %xmm0,0x350(%rsp)
   357c5:	movzbl %r12b,%r10d
   357c9:	lea    0x170(%rsp),%rdi
   357d1:	lea    0xe0(%rsp),%rcx
   357d9:	lea    0x350(%rsp),%r8
   357e1:	mov    %r13,%rsi
   357e4:	mov    %r15,%rdx
   357e7:	mov    $0xffffffff,%r9d
   357ed:	lea    0x738(%rsp),%rax
   357f5:	push   %rax
   357f6:	mov    %r10,0x1c8(%rsp)
   357fe:	push   %r10
   35800:	call   *0xf1962(%rip)        # 127168 <_DYNAMIC+0x608>
   35806:	add    $0x10,%rsp
   3580a:	mov    0x170(%rsp),%rdi
   35812:	movzbl 0x178(%rsp),%ebp
   3581a:	cmp    $0xffffffffffffffff,%rdi
   3581e:	je     3556d <meshopt_peers::run+0xbcd>
   35824:	movzbl %bpl,%eax
   35828:	movzbl 0x17f(%rsp),%ecx
   35830:	shl    $0x10,%ecx
   35833:	movzwl 0x17d(%rsp),%edx
   3583b:	or     %ecx,%edx
   3583d:	mov    0x179(%rsp),%ecx
   35844:	mov    0x180(%rsp),%r9
   3584c:	mov    0x188(%rsp),%r8
   35854:	mov    0x190(%rsp),%rsi
   3585c:	mov    %rsi,0x28(%rsp)
   35861:	shl    $0x28,%rdx
   35865:	shl    $0x8,%rcx
   35869:	or     %rdx,%rcx
   3586c:	or     %rax,%rcx
   3586f:	mov    %rcx,0x38(%rsp)
   35874:	test   %r9,%r9
   35877:	je     35b67 <meshopt_peers::run+0x11c7>
   3587d:	mov    %r8,0x70(%rsp)
   35882:	mov    %rdi,0x10(%rsp)
   35887:	mov    0x198(%rsp),%r14
   3588f:	mov    %r9,0xd8(%rsp)
   35897:	mov    %r9,%rax
   3589a:	shl    $0x4,%rax
   3589e:	lea    (%rax,%rax,2),%rax
   358a2:	mov    %rax,0x18(%rsp)
   358a7:	xor    %r12d,%r12d
   358aa:	mov    0x738(%rsp),%r15
   358b2:	mov    0x740(%rsp),%rbp
   358ba:	mov    0x748(%rsp),%rax
   358c2:	sub    %rbp,%rax
   358c5:	mov    %rax,0x358(%r15)
   358cc:	mov    %r15,%rdi
   358cf:	call   *0xf1843(%rip)        # 127118 <_DYNAMIC+0x5b8>
   358d5:	mov    0x340(%r15),%rax
   358dc:	mov    0x750(%rsp),%r13
   358e4:	sub    %r13,%rax
   358e7:	jb     35bdc <meshopt_peers::run+0x123c>
   358ed:	mov    0x348(%r15),%rcx
   358f4:	sub    0x358(%r15),%rcx
   358fb:	mov    %rax,0x690(%rsp)
   35903:	mov    %rcx,0x698(%rsp)
   3590b:	movb   $0x0,0x6b0(%rsp)
   35913:	movb   $0x0,0x6b5(%rsp)
   3591b:	movl   $0x0,0x6b1(%rsp)
   35926:	movb   $0xff,0x6b8(%rsp)
   3592e:	movw   $0x0,0x6b6(%rsp)
   35938:	lea    0x6a0(%rsp),%rax
   35940:	xorps  %xmm0,%xmm0
   35943:	movups %xmm0,(%rax)
   35946:	movq   $0xffffffffffffffff,0x3c8(%rsp)
   35952:	movq   $0xffffffffffffffff,0x5d0(%rsp)
   3595e:	movq   $0x0,0x350(%rsp)
   3596a:	movq   $0x4,0x358(%rsp)
   35976:	movups %xmm0,-0x340(%rax)
   3597d:	movq   $0x4,0x370(%rsp)
   35989:	movups %xmm0,-0x328(%rax)
   35990:	movq   $0x4,0x388(%rsp)
   3599c:	movups %xmm0,-0x310(%rax)
   359a3:	movq   $0x1,0x3a0(%rsp)
   359af:	movups %xmm0,-0x2f8(%rax)
   359b6:	movq   $0x2,0x3b8(%rsp)
   359c2:	movq   $0x0,0x3c0(%rsp)
   359ce:	mov    0x38(%rsp),%rax
   359d3:	mov    0x8(%rax,%r12,1),%rdi
   359d8:	mov    0x10(%rax,%r12,1),%rdx
   359dd:	mov    %rdx,%rsi
   359e0:	add    %rdi,%rsi
   359e3:	jb     36bf0 <meshopt_peers::run+0x2250>
   359e9:	cmp    %r14,%rsi
   359ec:	ja     36bf0 <meshopt_peers::run+0x2250>
   359f2:	mov    0x28(%rsp),%rax
   359f7:	lea    (%rax,%rdi,4),%rsi
   359fb:	movb   $0x2,0xe0(%rsp)
   35a03:	mov    0x50(%rsp),%rax
   35a08:	mov    %rax,0xe8(%rsp)
   35a10:	mov    %rbx,0xf0(%rsp)
   35a18:	movq   $0x0,0x118(%rsp)
   35a24:	lea    0x170(%rsp),%rdi
   35a2c:	lea    0xe0(%rsp),%rcx
   35a34:	mov    0x1c0(%rsp),%r8
   35a3c:	lea    0x350(%rsp),%r9
   35a44:	call   *0xf1726(%rip)        # 127170 <_DYNAMIC+0x610>
   35a4a:	mov    0x6a8(%rsp),%rsi
   35a52:	sub    %rsi,%rbp
   35a55:	jb     35b4a <meshopt_peers::run+0x11aa>
   35a5b:	mov    %rbp,0x740(%rsp)
   35a63:	add    0x6a0(%rsp),%r13
   35a6b:	jb     36c73 <meshopt_peers::run+0x22d3>
   35a71:	mov    %r15,%rdi
   35a74:	mov    %r13,%rsi
   35a77:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35a7c:	mov    %eax,%ebp
   35a7e:	cmp    $0xff,%al
   35a80:	jne    36c76 <meshopt_peers::run+0x22d6>
   35a86:	movzbl 0x170(%rsp),%ebx
   35a8e:	movzbl 0x171(%rsp),%ebp
   35a96:	lea    0x172(%rsp),%rcx
   35a9e:	mov    (%rcx),%rax
   35aa1:	mov    0x6(%rcx),%rcx
   35aa5:	mov    %rax,0x230(%rsp)
   35aad:	mov    %rcx,0x236(%rsp)
   35ab5:	movss  0x180(%rsp),%xmm0
   35abe:	movaps %xmm0,0x80(%rsp)
   35ac6:	lea    0x350(%rsp),%rdi
   35ace:	call   33b80 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35ad3:	cmp    $0x1,%bl
   35ad6:	mov    0x20(%rsp),%rbx
   35adb:	je     36c83 <meshopt_peers::run+0x22e3>
   35ae1:	lea    0x232(%rsp),%rcx
   35ae9:	mov    0x8(%rcx),%eax
   35aec:	lea    0x72c(%rsp),%rdx
   35af4:	mov    %eax,0x8(%rdx)
   35af7:	mov    (%rcx),%rax
   35afa:	mov    %rax,(%rdx)
   35afd:	mov    0x8(%rdx),%eax
   35b00:	mov    %eax,0x358(%rsp)
   35b07:	mov    (%rdx),%rcx
   35b0a:	mov    %rcx,0x350(%rsp)
   35b12:	mov    0x38(%rsp),%rdx
   35b17:	mov    %eax,0x24(%rdx,%r12,1)
   35b1c:	mov    %rcx,0x1c(%rdx,%r12,1)
   35b21:	xorps  %xmm0,%xmm0
   35b24:	movaps 0x80(%rsp),%xmm1
   35b2c:	movss  %xmm1,%xmm0
   35b30:	movlps %xmm0,0x28(%rdx,%r12,1)
   35b36:	add    $0x30,%r12
   35b3a:	cmp    %r12,0x18(%rsp)
   35b3f:	jne    358aa <meshopt_peers::run+0xf0a>
   35b45:	jmp    35c0e <meshopt_peers::run+0x126e>
   35b4a:	lea    0x740(%rsp),%rdi
   35b52:	call   *0xf15d0(%rip)        # 127128 <_DYNAMIC+0x5c8>
   35b58:	mov    %eax,%ebp
   35b5a:	cmp    $0xff,%al
   35b5c:	je     35a63 <meshopt_peers::run+0x10c3>
   35b62:	jmp    36c76 <meshopt_peers::run+0x22d6>
   35b67:	mov    %rdi,%r12
   35b6a:	mov    %r8,0x80(%rsp)
   35b72:	lea    0x738(%rsp),%rdi
   35b7a:	mov    0x150(%rsp),%rsi
   35b82:	mov    0xc8(%rsp),%rdx
   35b8a:	call   334c0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35b8f:	mov    $0x1,%r13b
   35b92:	cmp    $0xff,%al
   35b94:	je     35be4 <meshopt_peers::run+0x1244>
   35b96:	mov    %eax,%ebp
   35b98:	xor    %r14d,%r14d
   35b9b:	mov    $0x1,%r15b
   35b9e:	jmp    35d14 <meshopt_peers::run+0x1374>
   35ba3:	lea    0x350(%rsp),%rdi
   35bab:	mov    $0x1,%edx
   35bb0:	mov    $0x1,%r8d
   35bb6:	mov    $0x1,%r9d
   35bbc:	xor    %esi,%esi
   35bbe:	mov    %rbx,%rcx
   35bc1:	call   3b2b0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35bc6:	cmpl   $0x1,0x350(%rsp)
   35bce:	jne    35cb5 <meshopt_peers::run+0x1315>
   35bd4:	mov    $0xa,%bpl
   35bd7:	jmp    35585 <meshopt_peers::run+0xbe5>
   35bdc:	mov    $0x9,%bpl
   35bdf:	jmp    36c83 <meshopt_peers::run+0x22e3>
   35be4:	mov    0x750(%rsp),%rax
   35bec:	sub    %rbx,%rax
   35bef:	jae    35cd5 <meshopt_peers::run+0x1335>
   35bf5:	mov    0x90(%rsp),%rdi
   35bfd:	call   *0xf1165(%rip)        # 126d68 <free@GLIBC_2.2.5>
   35c03:	mov    $0x8,%bpl
   35c06:	xor    %r14d,%r14d
   35c09:	jmp    35d11 <meshopt_peers::run+0x1371>
   35c0e:	mov    0xd8(%rsp),%rdx
   35c16:	xor    %r13d,%r13d
   35c19:	cmp    $0x1,%rdx
   35c1d:	jne    35d21 <meshopt_peers::run+0x1381>
   35c23:	mov    0x10(%rsp),%r12
   35c28:	mov    0x70(%rsp),%rax
   35c2d:	mov    %rax,0x80(%rsp)
   35c35:	mov    0x38(%rsp),%rdi
   35c3a:	movups 0x1c(%rdi),%xmm0
   35c3e:	movaps %xmm0,0x2a0(%rsp)
   35c46:	movl   $0x7f7fffff,0x2b0(%rsp)
   35c51:	lea    0x9c(%rsp),%rax
   35c59:	lea    0x158(%rsp),%r10
   35c61:	lea    0x2a0(%rsp),%r11
   35c69:	mov    $0x1,%esi
   35c6e:	mov    0x28(%rsp),%rdx
   35c73:	mov    %r14,%rcx
   35c76:	mov    0x50(%rsp),%r8
   35c7b:	mov    %rbx,%r9
   35c7e:	lea    0x738(%rsp),%r15
   35c86:	push   %r15
   35c88:	push   %rax
   35c89:	push   %r10
   35c8b:	push   %r13
   35c8d:	push   %r11
   35c8f:	push   $0x0
   35c91:	call   33c10 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   35c96:	add    $0x30,%rsp
   35c9a:	mov    %rax,%rbp
   35c9d:	test   $0x1,%bpl
   35ca1:	je     35b72 <meshopt_peers::run+0x11d2>
   35ca7:	shr    $0x8,%ebp
   35caa:	mov    $0x1,%r14b
   35cad:	mov    $0x1,%r15b
   35cb0:	mov    $0x1,%r13b
   35cb3:	jmp    35d14 <meshopt_peers::run+0x1374>
   35cb5:	mov    0x358(%rsp),%rdi
   35cbd:	mov    %rdi,0x90(%rsp)
   35cc5:	xor    %esi,%esi
   35cc7:	mov    %rbx,%rdx
   35cca:	call   *0xf10b8(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   35cd0:	jmp    3554b <meshopt_peers::run+0xbab>
   35cd5:	mov    %rax,0x750(%rsp)
   35cdd:	test   %rbx,%rbx
   35ce0:	je     35cf0 <meshopt_peers::run+0x1350>
   35ce2:	mov    0x90(%rsp),%rdi
   35cea:	call   *0xf1078(%rip)        # 126d68 <free@GLIBC_2.2.5>
   35cf0:	xor    %r14d,%r14d
   35cf3:	lea    0x738(%rsp),%rdi
   35cfb:	mov    $0x4,%edx
   35d00:	xor    %esi,%esi
   35d02:	call   33420 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   35d07:	cmp    $0xff,%al
   35d09:	je     36a89 <meshopt_peers::run+0x20e9>
   35d0f:	mov    %eax,%ebp
   35d11:	xor    %r15d,%r15d
   35d14:	mov    0x80(%rsp),%rax
   35d1c:	jmp    36c96 <meshopt_peers::run+0x22f6>
   35d21:	lea    0x170(%rsp),%rdi
   35d29:	mov    0xf1448(%rip),%r10        # 127178 <_DYNAMIC+0x618>
   35d30:	mov    %r14,%r8
   35d33:	movb   $0x2,0xe0(%rsp)
   35d3b:	mov    0x50(%rsp),%rax
   35d40:	mov    %rax,0xe8(%rsp)
   35d48:	mov    %rbx,0xf0(%rsp)
   35d50:	movq   $0x0,0x118(%rsp)
   35d5c:	movups 0x210(%rsp),%xmm0
   35d64:	movaps %xmm0,0x390(%rsp)
   35d6c:	movups 0x1d0(%rsp),%xmm0
   35d74:	movups 0x1e0(%rsp),%xmm1
   35d7c:	movups 0x1f0(%rsp),%xmm2
   35d84:	movups 0x200(%rsp),%xmm3
   35d8c:	movaps %xmm3,0x380(%rsp)
   35d94:	movaps %xmm2,0x370(%rsp)
   35d9c:	movaps %xmm1,0x360(%rsp)
   35da4:	movaps %xmm0,0x350(%rsp)
   35dac:	mov    0x38(%rsp),%rsi
   35db1:	mov    %rdx,0xd8(%rsp)
   35db9:	mov    0x28(%rsp),%rcx
   35dbe:	mov    %r8,0x18(%rsp)
   35dc3:	mov    0xc8(%rsp),%r9
   35dcb:	lea    0x738(%rsp),%r12
   35dd3:	push   %r12
   35dd5:	lea    0x358(%rsp),%rax
   35ddd:	push   %rax
   35dde:	lea    0xf0(%rsp),%rax
   35de6:	push   %rax
   35de7:	push   0x1e0(%rsp)
   35dee:	call   *%r10
   35df1:	add    $0x20,%rsp
   35df5:	mov    %r13,0x148(%rsp)
   35dfd:	mov    0x170(%rsp),%r15
   35e05:	movzbl 0x178(%rsp),%ebp
   35e0d:	cmp    $0xffffffffffffffff,%r15
   35e11:	je     36c83 <meshopt_peers::run+0x22e3>
   35e17:	movzbl %bpl,%eax
   35e1b:	movzbl 0x17f(%rsp),%ecx
   35e23:	shl    $0x10,%ecx
   35e26:	movzwl 0x17d(%rsp),%edx
   35e2e:	or     %ecx,%edx
   35e30:	mov    0x179(%rsp),%r8d
   35e38:	mov    0x180(%rsp),%r14
   35e40:	shl    $0x28,%rdx
   35e44:	shl    $0x8,%r8
   35e48:	or     %rdx,%r8
   35e4b:	or     %rax,%r8
   35e4e:	sub    $0x40,%rsp
   35e52:	mov    %r12,0x30(%rsp)
   35e57:	mov    0x208(%rsp),%rax
   35e5f:	mov    %rax,0x18(%rsp)
   35e64:	mov    0x108(%rsp),%rax
   35e6c:	mov    %rax,0x10(%rsp)
   35e71:	mov    0x58(%rsp),%rax
   35e76:	mov    %rax,0x8(%rsp)
   35e7b:	mov    0x68(%rsp),%rax
   35e80:	mov    %rax,(%rsp)
   35e84:	movq   $0x0,0x20(%rsp)
   35e8d:	mov    0xd0(%rsp),%rdi
   35e95:	mov    %rbx,%rsi
   35e98:	mov    0x78(%rsp),%rdx
   35e9d:	mov    0x118(%rsp),%rcx
   35ea5:	mov    %r8,0x110(%rsp)
   35ead:	mov    %r14,%r9
   35eb0:	call   *0xf12ca(%rip)        # 127180 <_DYNAMIC+0x620>
   35eb6:	add    $0x40,%rsp
   35eba:	mov    %eax,%ebp
   35ebc:	cmp    $0xff,%al
   35ebe:	jne    36cc9 <meshopt_peers::run+0x2329>
   35ec4:	mov    0x18(%rsp),%rax
   35ec9:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   35ed3:	mul    %rcx
   35ed6:	mov    %rdx,%rax
   35ed9:	shr    $1,%rax
   35edc:	mov    %rax,0x48(%rsp)
   35ee1:	mov    $0x30,%ecx
   35ee6:	mul    %rcx
   35ee9:	seto   %cl
   35eec:	test   %rax,%rax
   35eef:	sets   %dl
   35ef2:	or     %cl,%dl
   35ef4:	mov    $0x8,%bpl
   35ef7:	mov    0x10(%rsp),%r12
   35efc:	jne    36a7e <meshopt_peers::run+0x20de>
   35f02:	mov    $0x1,%cl
   35f04:	mov    %ecx,0x8(%rsp)
   35f08:	add    0x750(%rsp),%rax
   35f10:	jb     36b54 <meshopt_peers::run+0x21b4>
   35f16:	mov    %rax,0x750(%rsp)
   35f1e:	mov    0x738(%rsp),%r13
   35f26:	mov    %r13,%rdi
   35f29:	mov    %rax,%r12
   35f2c:	mov    %rax,%rsi
   35f2f:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35f34:	mov    %eax,%ebp
   35f36:	cmp    $0xff,%al
   35f38:	jne    36de3 <meshopt_peers::run+0x2443>
   35f3e:	mov    %r15,0x40(%rsp)
   35f43:	mov    0x18(%rsp),%r8
   35f48:	cmp    $0x3,%r8
   35f4c:	jae    369be <meshopt_peers::run+0x201e>
   35f52:	mov    $0x8,%r15d
   35f58:	xor    %eax,%eax
   35f5a:	mov    %rax,0x68(%rsp)
   35f5f:	sub    0x48(%rsp),%rax
   35f64:	mov    $0x30,%ecx
   35f69:	mul    %rcx
   35f6c:	seto   %cl
   35f6f:	test   %rax,%rax
   35f72:	sets   %dl
   35f75:	or     %cl,%dl
   35f77:	mov    $0x8,%bpl
   35f7a:	jne    36b39 <meshopt_peers::run+0x2199>
   35f80:	test   %rax,%rax
   35f83:	je     35fb0 <meshopt_peers::run+0x1610>
   35f85:	add    %r12,%rax
   35f88:	jb     36b39 <meshopt_peers::run+0x2199>
   35f8e:	mov    %rax,0x750(%rsp)
   35f96:	mov    %r13,%rdi
   35f99:	mov    %rax,%rsi
   35f9c:	call   3bd20 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35fa1:	mov    %eax,%ebp
   35fa3:	cmp    $0xff,%al
   35fa5:	mov    0x18(%rsp),%r8
   35faa:	jne    36b39 <meshopt_peers::run+0x2199>
   35fb0:	mov    %r15,%rdx
   35fb3:	shr    $0x8,%rdx
   35fb7:	mov    %r15,%rax
   35fba:	shr    $0x28,%rax
   35fbe:	mov    %r15,%rcx
   35fc1:	shr    $0x38,%rcx
   35fc5:	mov    %cl,0xef(%rsp)
   35fcc:	mov    %ax,0xed(%rsp)
   35fd4:	mov    %edx,0xe9(%rsp)
   35fdb:	movq   $0x0,0xf0(%rsp)
   35fe7:	mov    0x68(%rsp),%rax
   35fec:	mov    %rax,0xe0(%rsp)
   35ff4:	mov    %r15b,0xe8(%rsp)
   35ffc:	cmp    $0x2,%r8
   36000:	jbe    36089 <meshopt_peers::run+0x16e9>
   36006:	cmp    0x48(%rsp),%rax
   3600b:	jb     36a0d <meshopt_peers::run+0x206d>
   36011:	xor    %r12d,%r12d
   36014:	mov    0xe8(%rsp),%r15
   3601c:	lea    (%r12,%r12,2),%rax
   36020:	shl    $0x4,%rax
   36024:	xorps  %xmm0,%xmm0
   36027:	movups %xmm0,0x20(%r15,%rax,1)
   3602d:	movups %xmm0,0x10(%r15,%rax,1)
   36033:	movups %xmm0,(%r15,%rax,1)
   36038:	cmpq   $0x6,0x18(%rsp)
   3603e:	jb     36061 <meshopt_peers::run+0x16c1>
   36040:	lea    (%r15,%rax,1),%rdi
   36044:	add    $0x30,%rdi
   36048:	mov    0x48(%rsp),%rax
   3604d:	lea    (%rax,%rax,2),%rdx
   36051:	shl    $0x4,%rdx
   36055:	add    $0xffffffffffffffd0,%rdx
   36059:	xor    %esi,%esi
   3605b:	call   *0xf0d27(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   36061:	mov    0xe0(%rsp),%rax
   36069:	mov    %rax,0x68(%rsp)
   3606e:	cmp    $0xffffffffffffffff,%rax
   36072:	je     36e21 <meshopt_peers::run+0x2481>
   36078:	mov    %r15,%rdx
   3607b:	shr    $0x8,%rdx
   3607f:	add    %r12,0x48(%rsp)
   36084:	mov    0x18(%rsp),%r8
   36089:	movzbl %r15b,%eax
   3608d:	shl    $0x8,%rdx
   36091:	or     %rax,%rdx
   36094:	mov    %rdx,0xb0(%rsp)
   3609c:	lea    0x350(%rsp),%rdi
   360a4:	lea    0x738(%rsp),%rsi
   360ac:	mov    %r8,%rdx
   360af:	call   33510 <<meshoptimizer_rs::processing::Context>::reserve::<u32>>
   360b4:	mov    0x350(%rsp),%rdx
   360bc:	movzbl 0x358(%rsp),%ebp
   360c4:	cmp    $0xffffffffffffffff,%rdx
   360c8:	je     36ded <meshopt_peers::run+0x244d>
   360ce:	lea    0x359(%rsp),%rcx
   360d6:	mov    (%rcx),%rax
   360d9:	mov    0x7(%rcx),%rcx
   360dd:	lea    0xe9(%rsp),%rsi
   360e5:	mov    %rcx,0x7(%rsi)
   360e9:	mov    %rax,(%rsi)
   360ec:	mov    %rdx,0xe0(%rsp)
   360f4:	mov    %bpl,0xe8(%rsp)
   360fc:	mov    0xf0(%rsp),%rcx
   36104:	mov    0x18(%rsp),%rsi
   36109:	mov    %rsi,%r15
   3610c:	sub    %rcx,%r15
   3610f:	mov    %rdx,0x80(%rsp)
   36117:	jbe    3616c <meshopt_peers::run+0x17cc>
   36119:	mov    %rdx,%rax
   3611c:	mov    %rcx,%rsi
   3611f:	sub    %rcx,%rax
   36122:	cmp    %rax,%r15
   36125:	ja     36a39 <meshopt_peers::run+0x2099>
   3612b:	mov    0xe8(%rsp),%rbp
   36133:	mov    %rsi,%rbx
   36136:	lea    0x0(,%rsi,4),%rdi
   3613e:	add    %rbp,%rdi
   36141:	lea    0x0(,%r15,4),%rdx
   36149:	xor    %esi,%esi
   3614b:	call   *0xf0c37(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   36151:	cmpq   $0xffffffffffffffff,0x80(%rsp)
   3615a:	je     36ded <meshopt_peers::run+0x244d>
   36160:	add    %r15,%rbx
   36163:	mov    %rbp,%rdx
   36166:	shr    $0x8,%rdx
   3616a:	jmp    36192 <meshopt_peers::run+0x17f2>
   3616c:	movzbl 0xef(%rsp),%eax
   36174:	shl    $0x10,%eax
   36177:	movzwl 0xed(%rsp),%ecx
   3617f:	or     %eax,%ecx
   36181:	shl    $0x20,%rcx
   36185:	mov    0xe9(%rsp),%edx
   3618c:	or     %rcx,%rdx
   3618f:	mov    %rsi,%rbx
   36192:	movzbl %bpl,%eax
   36196:	shl    $0x8,%rdx
   3619a:	or     %rax,%rdx
   3619d:	cmp    $0x2,%r14
   361a1:	mov    %rdx,0xc0(%rsp)
   361a9:	mov    %rbx,0x140(%rsp)
   361b1:	jb     368c8 <meshopt_peers::run+0x1f28>
   361b7:	mov    0xb0(%rsp),%rax
   361bf:	add    $0x10,%rax
   361c3:	mov    %rax,0x280(%rsp)
   361cb:	xor    %ebp,%ebp
   361cd:	xor    %r12d,%r12d
   361d0:	mov    0xd0(%rsp),%rax
   361d8:	mov    %rbp,0xb8(%rsp)
   361e0:	add    $0x4,%rax
   361e4:	mov    -0x4(%rax),%edi
   361e7:	mov    %rax,0x1b8(%rsp)
   361ef:	mov    (%rax),%r15d
   361f2:	cmp    %edi,%r15d
   361f5:	mov    0xd8(%rsp),%rdx
   361fd:	jb     36f8f <meshopt_peers::run+0x25ef>
   36203:	cmp    %r15,%rdx
   36206:	jb     36f8f <meshopt_peers::run+0x25ef>
   3620c:	sub    %rdi,%r15
   3620f:	lea    (%rdi,%rdi,2),%r13
   36213:	shl    $0x4,%r13
   36217:	add    0x38(%rsp),%r13
   3621c:	movq   $0xffffffffffffffff,0xe0(%rsp)
   36228:	lea    0x350(%rsp),%rdi
   36230:	mov    %r13,%rsi
   36233:	mov    %r15,%rdx
   36236:	mov    0x28(%rsp),%rcx
   3623b:	mov    0x18(%rsp),%r8
   36240:	lea    0x2c0(%rsp),%r9
   36248:	lea    0x738(%rsp),%rax
   36250:	push   %rax
   36251:	lea    0x1d8(%rsp),%rax
   36259:	push   %rax
   3625a:	push   0x30(%rsp)
   3625e:	push   0xa8(%rsp)
   36265:	call   *0xf0f1d(%rip)        # 127188 <_DYNAMIC+0x628>
   3626b:	add    $0x20,%rsp
   3626f:	mov    0x350(%rsp),%rax
   36277:	movzbl 0x358(%rsp),%ebp
   3627f:	mov    %rax,0x8(%rsp)
   36284:	cmp    $0xffffffffffffffff,%rax
   36288:	je     36d7e <meshopt_peers::run+0x23de>
   3628e:	movzbl %bpl,%eax
   36292:	movzbl 0x35f(%rsp),%ecx
   3629a:	shl    $0x10,%ecx
   3629d:	movzwl 0x35d(%rsp),%edx
   362a5:	or     %ecx,%edx
   362a7:	mov    0x359(%rsp),%ebx
   362ae:	mov    0x368(%rsp),%rcx
   362b6:	mov    %rcx,0x60(%rsp)
   362bb:	mov    0x370(%rsp),%rcx
   362c3:	mov    %rcx,0xa8(%rsp)
   362cb:	mov    0x380(%rsp),%rcx
   362d3:	mov    %rcx,0x30(%rsp)
   362d8:	mov    0x388(%rsp),%rcx
   362e0:	mov    %rcx,0x58(%rsp)
   362e5:	mov    0x390(%rsp),%rcx
   362ed:	mov    %rcx,0x78(%rsp)
   362f2:	lea    0x359(%rsp),%rcx
   362fa:	movups 0x3f(%rcx),%xmm0
   362fe:	movaps %xmm0,0x260(%rsp)
   36306:	mov    0x4f(%rcx),%ecx
   36309:	mov    %ecx,0x270(%rsp)
   36310:	shl    $0x28,%rdx
   36314:	shl    $0x8,%rbx
   36318:	or     %rdx,%rbx
   3631b:	or     %rax,%rbx
   3631e:	testb  $0x1,0x3ac(%rsp)
   36326:	je     36497 <meshopt_peers::run+0x1af7>
   3632c:	mov    %r13,%rdi
   3632f:	mov    %r15,%rsi
   36332:	mov    0x28(%rsp),%rdx
   36337:	mov    0x18(%rsp),%rcx
   3633c:	mov    0x50(%rsp),%r8
   36341:	mov    0x20(%rsp),%r9
   36346:	lea    0x738(%rsp),%rax
   3634e:	push   %rax
   3634f:	lea    0xa4(%rsp),%rax
   36357:	push   %rax
   36358:	lea    0x168(%rsp),%rax
   36360:	push   %rax
   36361:	push   0x160(%rsp)
   36368:	lea    0x280(%rsp),%rax
   36370:	push   %rax
   36371:	push   $0x0
   36373:	call   33c10 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   36378:	add    $0x30,%rsp
   3637c:	mov    %rax,%rbp
   3637f:	test   $0x1,%bpl
   36383:	jne    36e43 <meshopt_peers::run+0x24a3>
   36389:	mov    0x750(%rsp),%rcx
   36391:	mov    0x8(%rsp),%rdx
   36396:	lea    0x0(,%rdx,4),%rax
   3639e:	shr    $0x3e,%rdx
   363a2:	sete   %dl
   363a5:	test   %rax,%rax
   363a8:	setns  %sil
   363ac:	and    %dl,%sil
   363af:	sub    %rax,%rcx
   363b2:	setae  %al
   363b5:	test   %al,%sil
   363b8:	je     36bc4 <meshopt_peers::run+0x2224>
   363be:	mov    %rcx,0x750(%rsp)
   363c6:	cmpq   $0x0,0x8(%rsp)
   363cc:	je     363df <meshopt_peers::run+0x1a3f>
   363ce:	mov    %rbx,%rdi
   363d1:	call   *0xf0991(%rip)        # 126d68 <free@GLIBC_2.2.5>
   363d7:	mov    0x750(%rsp),%rcx
   363df:	mov    0x60(%rsp),%rax
   363e4:	mov    $0x14,%edx
   363e9:	mul    %rdx
   363ec:	jo     36d52 <meshopt_peers::run+0x23b2>
   363f2:	test   %rax,%rax
   363f5:	js     36d52 <meshopt_peers::run+0x23b2>
   363fb:	sub    %rax,%rcx
   363fe:	jb     36d52 <meshopt_peers::run+0x23b2>
   36404:	mov    %rcx,0x750(%rsp)
   3640c:	cmpq   $0x0,0x60(%rsp)
   36412:	je     3642a <meshopt_peers::run+0x1a8a>
   36414:	mov    0xa8(%rsp),%rdi
   3641c:	call   *0xf0946(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36422:	mov    0x750(%rsp),%rcx
   3642a:	movabs $0x3fffffffffffffff,%rax
   36434:	cmp    %rax,0x30(%rsp)
   36439:	ja     36e08 <meshopt_peers::run+0x2468>
   3643f:	mov    0x30(%rsp),%rax
   36444:	lea    0x0(,%rax,4),%rax
   3644c:	test   %rax,%rax
   3644f:	js     36e08 <meshopt_peers::run+0x2468>
   36455:	sub    %rax,%rcx
   36458:	jb     36e08 <meshopt_peers::run+0x2468>
   3645e:	mov    %rcx,0x750(%rsp)
   36466:	cmpq   $0x0,0x30(%rsp)
   3646c:	je     36479 <meshopt_peers::run+0x1ad9>
   3646e:	mov    0x58(%rsp),%rdi
   36473:	call   *0xf08ef(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36479:	dec    %r14
   3647c:	mov    0x1b8(%rsp),%rax
   36484:	add    $0x4,%rax
   36488:	cmp    $0x1,%r14
   3648c:	ja     361e4 <meshopt_peers::run+0x1844>
   36492:	jmp    36a71 <meshopt_peers::run+0x20d1>
   36497:	mov    %r13,%rdi
   3649a:	mov    %r15,%rsi
   3649d:	mov    0x28(%rsp),%rdx
   364a2:	mov    0x18(%rsp),%rcx
   364a7:	mov    0x50(%rsp),%r8
   364ac:	mov    0x20(%rsp),%r9
   364b1:	lea    0x738(%rsp),%rax
   364b9:	push   %rax
   364ba:	lea    0xa4(%rsp),%rax
   364c2:	push   %rax
   364c3:	lea    0x168(%rsp),%rax
   364cb:	push   %rax
   364cc:	push   0x160(%rsp)
   364d3:	lea    0x280(%rsp),%rax
   364db:	push   %rax
   364dc:	push   $0x0
   364de:	call   33c10 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output_mode<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   364e3:	add    $0x30,%rsp
   364e7:	mov    %rax,%rbp
   364ea:	test   $0x1,%bpl
   364ee:	jne    36e43 <meshopt_peers::run+0x24a3>
   364f4:	shr    $0x20,%rbp
   364f8:	movb   $0x2,0x170(%rsp)
   36500:	mov    0x50(%rsp),%rax
   36505:	mov    %rax,0x178(%rsp)
   3650d:	mov    0x20(%rsp),%rax
   36512:	mov    %rax,0x180(%rsp)
   3651a:	movq   $0x0,0x1a8(%rsp)
   36526:	movups 0x210(%rsp),%xmm0
   3652e:	movaps %xmm0,0x390(%rsp)
   36536:	movups 0x1d0(%rsp),%xmm0
   3653e:	movups 0x1e0(%rsp),%xmm1
   36546:	movups 0x1f0(%rsp),%xmm2
   3654e:	movups 0x200(%rsp),%xmm3
   36556:	movaps %xmm3,0x380(%rsp)
   3655e:	movaps %xmm2,0x370(%rsp)
   36566:	movaps %xmm1,0x360(%rsp)
   3656e:	movaps %xmm0,0x350(%rsp)
   36576:	lea    0x230(%rsp),%rdi
   3657e:	mov    0x58(%rsp),%rsi
   36583:	mov    0x78(%rsp),%rdx
   36588:	lea    0x170(%rsp),%rcx
   36590:	lea    0x350(%rsp),%r8
   36598:	mov    %ebp,%r9d
   3659b:	lea    0x738(%rsp),%rax
   365a3:	push   %rax
   365a4:	push   0x1c8(%rsp)
   365ab:	call   *0xf0bb7(%rip)        # 127168 <_DYNAMIC+0x608>
   365b1:	add    $0x10,%rsp
   365b5:	mov    0x230(%rsp),%r15
   365bd:	movzbl 0x238(%rsp),%ebp
   365c5:	cmp    $0xffffffffffffffff,%r15
   365c9:	je     36e46 <meshopt_peers::run+0x24a6>
   365cf:	movzbl %bpl,%ecx
   365d3:	movzbl 0x23f(%rsp),%eax
   365db:	shl    $0x10,%eax
   365de:	movzwl 0x23d(%rsp),%esi
   365e6:	or     %eax,%esi
   365e8:	mov    0x239(%rsp),%r13d
   365f0:	mov    0x240(%rsp),%rax
   365f8:	mov    0x248(%rsp),%rdx
   36600:	mov    %rdx,0xa0(%rsp)
   36608:	mov    0x250(%rsp),%rdx
   36610:	mov    %rdx,0x78(%rsp)
   36615:	mov    0x258(%rsp),%rdx
   3661d:	shl    $0x28,%rsi
   36621:	shl    $0x8,%r13
   36625:	or     %rsi,%r13
   36628:	or     %rcx,%r13
   3662b:	test   %rax,%rax
   3662e:	je     366d7 <meshopt_peers::run+0x1d37>
   36634:	lea    (%rax,%rax,2),%rax
   36638:	shl    $0x4,%rax
   3663c:	add    %r13,%rax
   3663f:	lea    (%r12,%r12,2),%rcx
   36643:	shl    $0x4,%rcx
   36647:	add    0x280(%rsp),%rcx
   3664f:	xor    %esi,%esi
   36651:	mov    0x0(%r13,%rsi,1),%rdi
   36656:	mov    0x8(%r13,%rsi,1),%r8
   3665b:	movups 0x10(%r13,%rsi,1),%xmm0
   36661:	movaps %xmm0,0x6c0(%rsp)
   36669:	mov    0x270(%rsp),%r9d
   36671:	lea    0x6cc(%rsp),%r10
   36679:	mov    %r9d,0x10(%r10)
   3667d:	movaps 0x260(%rsp),%xmm0
   36685:	movups %xmm0,(%r10)
   36689:	cmp    0x48(%rsp),%r12
   3668e:	jae    37087 <meshopt_peers::run+0x26e7>
   36694:	lea    (%rsi,%r13,1),%r9
   36698:	add    0xb8(%rsp),%r8
   366a0:	mov    %rdi,-0x10(%rcx,%rsi,1)
   366a5:	mov    %r8,-0x8(%rcx,%rsi,1)
   366aa:	movaps 0x6c0(%rsp),%xmm0
   366b2:	movaps 0x6d0(%rsp),%xmm1
   366ba:	movups %xmm0,(%rcx,%rsi,1)
   366be:	movups %xmm1,0x10(%rcx,%rsi,1)
   366c3:	inc    %r12
   366c6:	add    $0x30,%rsi
   366ca:	add    $0x30,%r9
   366ce:	cmp    %rax,%r9
   366d1:	jne    36651 <meshopt_peers::run+0x1cb1>
   366d7:	mov    %rdx,%rbp
   366da:	add    0xb8(%rsp),%rbp
   366e2:	jb     36ffc <meshopt_peers::run+0x265c>
   366e8:	cmp    0x140(%rsp),%rbp
   366f0:	ja     36ffc <meshopt_peers::run+0x265c>
   366f6:	mov    0xc0(%rsp),%rax
   366fe:	mov    0xb8(%rsp),%rcx
   36706:	lea    (%rax,%rcx,4),%rdi
   3670a:	shl    $0x2,%rdx
   3670e:	mov    0x78(%rsp),%rsi
   36713:	call   *0xf065f(%rip)        # 126d78 <memcpy@GLIBC_2.14>
   36719:	mov    0x750(%rsp),%rcx
   36721:	mov    %r15,%rax
   36724:	mov    $0x30,%edx
   36729:	mul    %rdx
   3672c:	seto   %dl
   3672f:	not    %dl
   36731:	test   %rax,%rax
   36734:	setns  %sil
   36738:	and    %dl,%sil
   3673b:	sub    %rax,%rcx
   3673e:	setae  %al
   36741:	test   %al,%sil
   36744:	je     36ea3 <meshopt_peers::run+0x2503>
   3674a:	mov    %rcx,0x750(%rsp)
   36752:	test   %r15,%r15
   36755:	je     36768 <meshopt_peers::run+0x1dc8>
   36757:	mov    %r13,%rdi
   3675a:	call   *0xf0608(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36760:	mov    0x750(%rsp),%rcx
   36768:	mov    0xa0(%rsp),%rax
   36770:	shr    $0x3e,%rax
   36774:	jne    36eb1 <meshopt_peers::run+0x2511>
   3677a:	mov    0xa0(%rsp),%rax
   36782:	lea    0x0(,%rax,4),%rax
   3678a:	test   %rax,%rax
   3678d:	js     36eb1 <meshopt_peers::run+0x2511>
   36793:	sub    %rax,%rcx
   36796:	jb     36eb1 <meshopt_peers::run+0x2511>
   3679c:	mov    %rcx,0x750(%rsp)
   367a4:	cmpq   $0x0,0xa0(%rsp)
   367ad:	je     367c2 <meshopt_peers::run+0x1e22>
   367af:	mov    0x78(%rsp),%rdi
   367b4:	call   *0xf05ae(%rip)        # 126d68 <free@GLIBC_2.2.5>
   367ba:	mov    0x750(%rsp),%rcx
   367c2:	movabs $0x3fffffffffffffff,%rax
   367cc:	cmp    %rax,0x8(%rsp)
   367d1:	ja     36f38 <meshopt_peers::run+0x2598>
   367d7:	mov    0x8(%rsp),%rax
   367dc:	lea    0x0(,%rax,4),%rax
   367e4:	test   %rax,%rax
   367e7:	js     36f38 <meshopt_peers::run+0x2598>
   367ed:	sub    %rax,%rcx
   367f0:	jb     36f38 <meshopt_peers::run+0x2598>
   367f6:	mov    %rcx,0x750(%rsp)
   367fe:	cmpq   $0x0,0x8(%rsp)
   36804:	je     36817 <meshopt_peers::run+0x1e77>
   36806:	mov    %rbx,%rdi
   36809:	call   *0xf0559(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3680f:	mov    0x750(%rsp),%rcx
   36817:	mov    0x60(%rsp),%rax
   3681c:	mov    $0x14,%edx
   36821:	mul    %rdx
   36824:	jo     36fb7 <meshopt_peers::run+0x2617>
   3682a:	test   %rax,%rax
   3682d:	js     36fb7 <meshopt_peers::run+0x2617>
   36833:	sub    %rax,%rcx
   36836:	jb     36fb7 <meshopt_peers::run+0x2617>
   3683c:	mov    %rcx,0x750(%rsp)
   36844:	cmpq   $0x0,0x60(%rsp)
   3684a:	je     36862 <meshopt_peers::run+0x1ec2>
   3684c:	mov    0xa8(%rsp),%rdi
   36854:	call   *0xf050e(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3685a:	mov    0x750(%rsp),%rcx
   36862:	movabs $0x3fffffffffffffff,%rax
   3686c:	cmp    %rax,0x30(%rsp)
   36871:	ja     36fe8 <meshopt_peers::run+0x2648>
   36877:	mov    0x30(%rsp),%rax
   3687c:	lea    0x0(,%rax,4),%rax
   36884:	test   %rax,%rax
   36887:	js     36fe8 <meshopt_peers::run+0x2648>
   3688d:	sub    %rax,%rcx
   36890:	jb     36fe8 <meshopt_peers::run+0x2648>
   36896:	mov    %rcx,0x750(%rsp)
   3689e:	cmpq   $0x0,0x30(%rsp)
   368a4:	je     368b1 <meshopt_peers::run+0x1f11>
   368a6:	mov    0x58(%rsp),%rdi
   368ab:	call   *0xf04b7(%rip)        # 126d68 <free@GLIBC_2.2.5>
   368b1:	dec    %r14
   368b4:	cmp    $0x1,%r14
   368b8:	mov    0x1b8(%rsp),%rax
   368c0:	ja     361d8 <meshopt_peers::run+0x1838>
   368c6:	jmp    368cd <meshopt_peers::run+0x1f2d>
   368c8:	xor    %r12d,%r12d
   368cb:	xor    %ebp,%ebp
   368cd:	mov    %rbp,0x18(%rsp)
   368d2:	lea    0x738(%rsp),%rbx
   368da:	mov    %rbx,%rdi
   368dd:	mov    0x40(%rsp),%rsi
   368e2:	mov    0xd0(%rsp),%rdx
   368ea:	call   334c0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   368ef:	mov    %eax,%ebp
   368f1:	mov    $0x1,%al
   368f3:	mov    %eax,0x8(%rsp)
   368f7:	cmp    $0xff,%bpl
   368fb:	setne  %r13b
   368ff:	jne    36e3b <meshopt_peers::run+0x249b>
   36905:	mov    0x48(%rsp),%r15
   3690a:	cmp    %r15,%r12
   3690d:	cmovb  %r12,%r15
   36911:	mov    0x140(%rsp),%r14
   36919:	mov    0x18(%rsp),%rax
   3691e:	cmp    %r14,%rax
   36921:	cmovb  %rax,%r14
   36925:	mov    %rbx,%rdi
   36928:	mov    0x10(%rsp),%rsi
   3692d:	mov    0x38(%rsp),%rdx
   36932:	call   33470 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36937:	cmp    $0xff,%al
   36939:	jne    36e5a <meshopt_peers::run+0x24ba>
   3693f:	mov    %rbx,%rdi
   36942:	mov    0x70(%rsp),%rsi
   36947:	mov    0x28(%rsp),%rdx
   3694c:	call   334c0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36951:	cmp    $0xff,%al
   36953:	jne    36e64 <meshopt_peers::run+0x24c4>
   36959:	mov    0x148(%rsp),%r13
   36961:	inc    %r13d
   36964:	mov    0x68(%rsp),%rax
   36969:	mov    %rax,0x10(%rsp)
   3696e:	mov    %r15,%rdx
   36971:	mov    0xb0(%rsp),%rax
   36979:	mov    %rax,0x38(%rsp)
   3697e:	mov    %r14,%r8
   36981:	mov    0xc0(%rsp),%rax
   36989:	mov    %rax,0x28(%rsp)
   3698e:	mov    0x80(%rsp),%rax
   36996:	mov    %rax,0x70(%rsp)
   3699b:	cmp    $0x1,%r15
   3699f:	mov    0x20(%rsp),%rbx
   369a4:	lea    0x170(%rsp),%rdi
   369ac:	mov    0xf07c5(%rip),%r10        # 127178 <_DYNAMIC+0x618>
   369b3:	ja     35d33 <meshopt_peers::run+0x1393>
   369b9:	jmp    36e76 <meshopt_peers::run+0x24d6>
   369be:	mov    $0x8,%edx
   369c3:	mov    $0x8,%r8d
   369c9:	mov    $0x30,%r9d
   369cf:	lea    0x350(%rsp),%rdi
   369d7:	xor    %esi,%esi
   369d9:	mov    0x48(%rsp),%rcx
   369de:	call   3b2b0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   369e3:	cmpl   $0x1,0x350(%rsp)
   369eb:	je     36ef6 <meshopt_peers::run+0x2556>
   369f1:	mov    0x358(%rsp),%r15
   369f9:	mov    0x48(%rsp),%rax
   369fe:	mov    0x20(%rsp),%rbx
   36a03:	mov    0x18(%rsp),%r8
   36a08:	jmp    35f5a <meshopt_peers::run+0x15ba>
   36a0d:	mov    $0x8,%ecx
   36a12:	mov    $0x30,%r8d
   36a18:	lea    0xe0(%rsp),%rdi
   36a20:	xor    %esi,%esi
   36a22:	mov    0x48(%rsp),%rdx
   36a27:	call   34900 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36a2c:	mov    0xf0(%rsp),%r12
   36a34:	jmp    36014 <meshopt_peers::run+0x1674>
   36a39:	mov    $0x4,%ecx
   36a3e:	mov    $0x4,%r8d
   36a44:	lea    0xe0(%rsp),%rdi
   36a4c:	mov    %r15,%rdx
   36a4f:	call   34900 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36a54:	mov    0xe0(%rsp),%rax
   36a5c:	mov    %rax,0x80(%rsp)
   36a64:	mov    0xf0(%rsp),%rsi
   36a6c:	jmp    3612b <meshopt_peers::run+0x178b>
   36a71:	mov    0xb8(%rsp),%rbp
   36a79:	jmp    368cd <meshopt_peers::run+0x1f2d>
   36a7e:	mov    $0x1,%al
   36a80:	mov    %eax,0x8(%rsp)
   36a84:	jmp    36b54 <meshopt_peers::run+0x21b4>
   36a89:	lea    0x738(%rsp),%rdi
   36a91:	mov    %r12,%rsi
   36a94:	mov    0x38(%rsp),%rdx
   36a99:	call   33470 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36a9e:	cmp    $0xff,%al
   36aa0:	jne    3701e <meshopt_peers::run+0x267e>
   36aa6:	lea    0x738(%rsp),%rdi
   36aae:	mov    0x80(%rsp),%rsi
   36ab6:	mov    0x28(%rsp),%rdx
   36abb:	call   334c0 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36ac0:	mov    %eax,%ebp
   36ac2:	jmp    3559e <meshopt_peers::run+0xbfe>
   36ac7:	movzbl 0x2c8(%rsp),%eax
   36acf:	mov    %al,0x738(%rsp)
   36ad6:	lea    -0x2652a(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36add:	lea    0xe91bc(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36ae4:	lea    0xe8f45(%rip),%r8        # 11fa30 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36aeb:	lea    0x738(%rsp),%rdx
   36af3:	mov    $0x2b,%esi
   36af8:	call   *0xf0332(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36afe:	lea    0x290(%rsp),%rax
   36b06:	mov    %rax,0x738(%rsp)
   36b0e:	lea    0x55fb(%rip),%rax        # 3c110 <<&str as core::fmt::Display>::fmt>
   36b15:	mov    %rax,0x740(%rsp)
   36b1d:	lea    -0x2a429(%rip),%rdi        # c6fb <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36b24:	lea    0xe8dfd(%rip),%rdx        # 11f928 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x120>
   36b2b:	lea    0x738(%rsp),%rsi
   36b33:	call   *0xf02c7(%rip)        # 126e00 <_DYNAMIC+0x2a0>
   36b39:	cmpq   $0x0,0x68(%rsp)
   36b3f:	mov    0x10(%rsp),%r12
   36b44:	je     36b4f <meshopt_peers::run+0x21af>
   36b46:	mov    %r15,%rdi
   36b49:	call   *0xf0219(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36b4f:	mov    0x40(%rsp),%r15
   36b54:	mov    $0x1,%r13b
   36b57:	test   %r15,%r15
   36b5a:	je     36b6a <meshopt_peers::run+0x21ca>
   36b5c:	mov    0xd0(%rsp),%rdi
   36b64:	call   *0xf01fe(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36b6a:	mov    $0x1,%r14b
   36b6d:	mov    $0x1,%r15b
   36b70:	cmpb   $0x0,0x8(%rsp)
   36b75:	mov    0x70(%rsp),%rax
   36b7a:	jne    36c96 <meshopt_peers::run+0x22f6>
   36b80:	jmp    36ca6 <meshopt_peers::run+0x2306>
   36b85:	movzbl 0x740(%rsp),%eax
   36b8d:	mov    %al,0x350(%rsp)
   36b94:	lea    -0x265e8(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36b9b:	lea    0xe90fe(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36ba2:	lea    0xe8e27(%rip),%r8        # 11f9d0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   36ba9:	lea    0x350(%rsp),%rdx
   36bb1:	mov    $0x2b,%esi
   36bb6:	call   *0xf0274(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36bbc:	mov    $0x8,%bpl
   36bbf:	jmp    35585 <meshopt_peers::run+0xbe5>
   36bc4:	mov    $0x8,%bpl
   36bc7:	cmpq   $0x0,0x8(%rsp)
   36bcd:	je     36d13 <meshopt_peers::run+0x2373>
   36bd3:	mov    %rbx,%rdi
   36bd6:	call   *0xf018c(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36bdc:	xor    %r15d,%r15d
   36bdf:	cmpq   $0x0,0x30(%rsp)
   36be5:	jne    36d1e <meshopt_peers::run+0x237e>
   36beb:	jmp    36d29 <meshopt_peers::run+0x2389>
   36bf0:	lea    0xe8d19(%rip),%rcx        # 11f910 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x108>
   36bf7:	mov    %r14,%rdx
   36bfa:	call   *0xf02d0(%rip)        # 126ed0 <_DYNAMIC+0x370>
   36c00:	jmp    3709c <meshopt_peers::run+0x26fc>
   36c05:	movzbl 0x740(%rsp),%eax
   36c0d:	mov    %al,0x350(%rsp)
   36c14:	lea    -0x26668(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36c1b:	lea    0xe907e(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36c22:	lea    0xe8d77(%rip),%r8        # 11f9a0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36c29:	lea    0x350(%rsp),%rdx
   36c31:	mov    $0x2b,%esi
   36c36:	call   *0xf01f4(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36c3c:	movzbl 0x740(%rsp),%eax
   36c44:	mov    %al,0x350(%rsp)
   36c4b:	lea    -0x2669f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36c52:	lea    0xe9047(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36c59:	lea    0xe8cf8(%rip),%r8        # 11f958 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36c60:	lea    0x350(%rsp),%rdx
   36c68:	mov    $0x2b,%esi
   36c6d:	call   *0xf01bd(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36c73:	mov    $0x8,%bpl
   36c76:	lea    0x350(%rsp),%rdi
   36c7e:	call   33b80 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   36c83:	mov    $0x1,%r14b
   36c86:	mov    $0x1,%r15b
   36c89:	mov    $0x1,%r13b
   36c8c:	mov    0x10(%rsp),%r12
   36c91:	mov    0x70(%rsp),%rax
   36c96:	test   %rax,%rax
   36c99:	je     36ca6 <meshopt_peers::run+0x2306>
   36c9b:	mov    0x28(%rsp),%rdi
   36ca0:	call   *0xf00c2(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36ca6:	test   %r13b,%r13b
   36ca9:	je     36cbb <meshopt_peers::run+0x231b>
   36cab:	test   %r12,%r12
   36cae:	je     36cbb <meshopt_peers::run+0x231b>
   36cb0:	mov    0x38(%rsp),%rdi
   36cb5:	call   *0xf00ad(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36cbb:	test   %r15b,%r15b
   36cbe:	jne    3556d <meshopt_peers::run+0xbcd>
   36cc4:	jmp    35580 <meshopt_peers::run+0xbe0>
   36cc9:	mov    $0x1,%al
   36ccb:	mov    %eax,0x8(%rsp)
   36ccf:	mov    $0x1,%r13b
   36cd2:	mov    0x10(%rsp),%r12
   36cd7:	jmp    36b57 <meshopt_peers::run+0x21b7>
   36cdc:	movzbl 0x740(%rsp),%eax
   36ce4:	mov    %al,0x350(%rsp)
   36ceb:	lea    -0x2673f(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36cf2:	lea    0xe8fa7(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36cf9:	lea    0xe8ce8(%rip),%r8        # 11f9e8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   36d00:	lea    0x350(%rsp),%rdx
   36d08:	mov    $0x2b,%esi
   36d0d:	call   *0xf011d(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36d13:	xor    %r15d,%r15d
   36d16:	cmpq   $0x0,0x30(%rsp)
   36d1c:	je     36d29 <meshopt_peers::run+0x2389>
   36d1e:	mov    0x58(%rsp),%rdi
   36d23:	call   *0xf003f(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36d29:	cmpq   $0x0,0x60(%rsp)
   36d2f:	je     36d3f <meshopt_peers::run+0x239f>
   36d31:	mov    0xa8(%rsp),%rdi
   36d39:	call   *0xf0029(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36d3f:	cmpq   $0x0,0x8(%rsp)
   36d45:	setne  %al
   36d48:	test   %r15b,%al
   36d4b:	je     36d7e <meshopt_peers::run+0x23de>
   36d4d:	mov    %rbx,%rdi
   36d50:	jmp    36d78 <meshopt_peers::run+0x23d8>
   36d52:	cmpq   $0x0,0x60(%rsp)
   36d58:	je     36d68 <meshopt_peers::run+0x23c8>
   36d5a:	mov    0xa8(%rsp),%rdi
   36d62:	call   *0xf0000(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36d68:	mov    $0x8,%bpl
   36d6b:	cmpq   $0x0,0x30(%rsp)
   36d71:	je     36d7e <meshopt_peers::run+0x23de>
   36d73:	mov    0x58(%rsp),%rdi
   36d78:	call   *0xeffea(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36d7e:	lea    0xe0(%rsp),%rdi
   36d86:	call   335b0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   36d8b:	mov    $0x1,%r15b
   36d8e:	mov    $0x1,%al
   36d90:	mov    %eax,0x8(%rsp)
   36d94:	mov    $0x1,%r13b
   36d97:	cmpq   $0x0,0x80(%rsp)
   36da0:	je     36db0 <meshopt_peers::run+0x2410>
   36da2:	mov    0xc0(%rsp),%rdi
   36daa:	call   *0xeffb8(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36db0:	cmpq   $0x0,0x68(%rsp)
   36db6:	je     36dc6 <meshopt_peers::run+0x2426>
   36db8:	mov    0xb0(%rsp),%rdi
   36dc0:	call   *0xeffa2(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36dc6:	test   %r15b,%r15b
   36dc9:	mov    0x20(%rsp),%rbx
   36dce:	mov    0x10(%rsp),%r12
   36dd3:	mov    0x40(%rsp),%r15
   36dd8:	jne    36b57 <meshopt_peers::run+0x21b7>
   36dde:	jmp    36b6a <meshopt_peers::run+0x21ca>
   36de3:	mov    0x10(%rsp),%r12
   36de8:	jmp    36b54 <meshopt_peers::run+0x21b4>
   36ded:	cmpq   $0x0,0x68(%rsp)
   36df3:	je     36e1c <meshopt_peers::run+0x247c>
   36df5:	mov    0xb0(%rsp),%rdi
   36dfd:	call   *0xeff65(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36e03:	mov    $0x1,%r13b
   36e06:	jmp    36e27 <meshopt_peers::run+0x2487>
   36e08:	mov    $0x8,%bpl
   36e0b:	cmpq   $0x0,0x30(%rsp)
   36e11:	je     36d7e <meshopt_peers::run+0x23de>
   36e17:	jmp    36d73 <meshopt_peers::run+0x23d3>
   36e1c:	mov    $0x1,%r13b
   36e1f:	jmp    36e27 <meshopt_peers::run+0x2487>
   36e21:	mov    $0x1,%r13b
   36e24:	mov    %r15d,%ebp
   36e27:	mov    0x20(%rsp),%rbx
   36e2c:	mov    0x10(%rsp),%r12
   36e31:	mov    0x40(%rsp),%r15
   36e36:	jmp    36b57 <meshopt_peers::run+0x21b7>
   36e3b:	xor    %r15d,%r15d
   36e3e:	jmp    36d97 <meshopt_peers::run+0x23f7>
   36e43:	shr    $0x8,%ebp
   36e46:	mov    $0x1,%r15b
   36e49:	cmpq   $0x0,0x30(%rsp)
   36e4f:	jne    36d1e <meshopt_peers::run+0x237e>
   36e55:	jmp    36d29 <meshopt_peers::run+0x2389>
   36e5a:	mov    %eax,%ebp
   36e5c:	xor    %r15d,%r15d
   36e5f:	jmp    36d97 <meshopt_peers::run+0x23f7>
   36e64:	mov    %eax,%ebp
   36e66:	xor    %r15d,%r15d
   36e69:	movl   $0x0,0x8(%rsp)
   36e71:	jmp    36d97 <meshopt_peers::run+0x23f7>
   36e76:	mov    0xb0(%rsp),%rax
   36e7e:	mov    %rax,0x38(%rsp)
   36e83:	mov    0xc0(%rsp),%rax
   36e8b:	mov    %rax,0x28(%rsp)
   36e90:	test   %r15,%r15
   36e93:	mov    0x68(%rsp),%r12
   36e98:	jne    35c35 <meshopt_peers::run+0x1295>
   36e9e:	jmp    35b72 <meshopt_peers::run+0x11d2>
   36ea3:	test   %r15,%r15
   36ea6:	je     36eb1 <meshopt_peers::run+0x2511>
   36ea8:	mov    %r13,%rdi
   36eab:	call   *0xefeb7(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36eb1:	mov    $0x1,%r13b
   36eb4:	mov    $0x1,%r15b
   36eb7:	cmpq   $0x0,0xa0(%rsp)
   36ec0:	je     36ecf <meshopt_peers::run+0x252f>
   36ec2:	mov    0x78(%rsp),%rdi
   36ec7:	call   *0xefe9b(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36ecd:	jmp    36ed2 <meshopt_peers::run+0x2532>
   36ecf:	mov    $0x1,%r15b
   36ed2:	cmpq   $0x0,0x30(%rsp)
   36ed8:	je     36ee5 <meshopt_peers::run+0x2545>
   36eda:	mov    0x58(%rsp),%rdi
   36edf:	call   *0xefe83(%rip)        # 126d68 <free@GLIBC_2.2.5>
   36ee5:	mov    $0x8,%bpl
   36ee8:	test   %r13b,%r13b
   36eeb:	jne    36d29 <meshopt_peers::run+0x2389>
   36ef1:	jmp    36d3f <meshopt_peers::run+0x239f>
   36ef6:	mov    $0xa,%bpl
   36ef9:	mov    0x20(%rsp),%rbx
   36efe:	mov    0x10(%rsp),%r12
   36f03:	jmp    36b4f <meshopt_peers::run+0x21af>
   36f08:	movb   $0x2,0x738(%rsp)
   36f10:	lea    -0x26964(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f17:	lea    0xe8d82(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f1e:	lea    0xe8af3(%rip),%r8        # 11fa18 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   36f25:	lea    0x738(%rsp),%rdx
   36f2d:	mov    $0x2b,%esi
   36f32:	call   *0xefef8(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36f38:	mov    $0x1,%r13b
   36f3b:	xor    %r15d,%r15d
   36f3e:	mov    %rbx,0x78(%rsp)
   36f43:	cmpq   $0x0,0x8(%rsp)
   36f49:	jne    36ec2 <meshopt_peers::run+0x2522>
   36f4f:	jmp    36ed2 <meshopt_peers::run+0x2532>
   36f51:	movb   $0x8,0x738(%rsp)
   36f59:	lea    -0x269ad(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f60:	lea    0xe8d39(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   36f67:	lea    0xe8a4a(%rip),%r8        # 11f9b8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   36f6e:	lea    0x738(%rsp),%rdx
   36f76:	mov    $0x2b,%esi
   36f7b:	call   *0xefeaf(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   36f81:	mov    $0x4,%edi
   36f86:	mov    %r13,%rsi
   36f89:	call   *0xefe11(%rip)        # 126da0 <_DYNAMIC+0x240>
   36f8f:	lea    0xe891a(%rip),%rcx        # 11f8b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   36f96:	mov    %r15,%rsi
   36f99:	call   *0xeff31(%rip)        # 126ed0 <_DYNAMIC+0x370>
   36f9f:	jmp    3709c <meshopt_peers::run+0x26fc>
   36fa4:	mov    $0x4,%edi
   36fa9:	mov    %r15,%rsi
   36fac:	call   *0xefdee(%rip)        # 126da0 <_DYNAMIC+0x240>
   36fb2:	jmp    3709c <meshopt_peers::run+0x26fc>
   36fb7:	xor    %r13d,%r13d
   36fba:	cmpq   $0x0,0x60(%rsp)
   36fc0:	je     36fd7 <meshopt_peers::run+0x2637>
   36fc2:	mov    0xa8(%rsp),%rax
   36fca:	mov    %rax,0x78(%rsp)
   36fcf:	xor    %r15d,%r15d
   36fd2:	jmp    36ec2 <meshopt_peers::run+0x2522>
   36fd7:	movq   $0x0,0x60(%rsp)
   36fe0:	xor    %r15d,%r15d
   36fe3:	jmp    36ed2 <meshopt_peers::run+0x2532>
   36fe8:	mov    $0x8,%bpl
   36feb:	cmpq   $0x0,0x30(%rsp)
   36ff1:	je     36d7e <meshopt_peers::run+0x23de>
   36ff7:	jmp    36d73 <meshopt_peers::run+0x23d3>
   36ffc:	lea    0xe8895(%rip),%rcx        # 11f898 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   37003:	mov    0xb8(%rsp),%rdi
   3700b:	mov    %rbp,%rsi
   3700e:	mov    0x140(%rsp),%rdx
   37016:	call   *0xefeb4(%rip)        # 126ed0 <_DYNAMIC+0x370>
   3701c:	jmp    3709c <meshopt_peers::run+0x26fc>
   3701e:	mov    %eax,%ebp
   37020:	cmpq   $0x0,0x80(%rsp)
   37029:	je     37036 <meshopt_peers::run+0x2696>
   3702b:	mov    0x28(%rsp),%rdi
   37030:	call   *0xefd32(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37036:	mov    0x738(%rsp),%rax
   3703e:	mov    0x748(%rsp),%rcx
   37046:	sub    0x740(%rsp),%rcx
   3704e:	mov    %rcx,0x358(%rax)
   37055:	mov    %bpl,0x738(%rsp)
   3705d:	lea    -0x26ab1(%rip),%rdi        # 105b3 <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37064:	lea    0xe8c35(%rip),%rcx        # 11fca0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x498>
   3706b:	lea    0xe89d6(%rip),%r8        # 11fa48 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   37072:	lea    0x738(%rsp),%rdx
   3707a:	mov    $0x2b,%esi
   3707f:	call   *0xefdab(%rip)        # 126e30 <_DYNAMIC+0x2d0>
   37085:	jmp    3709c <meshopt_peers::run+0x26fc>
   37087:	lea    0xe87f2(%rip),%rdx        # 11f880 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   3708e:	mov    %r12,%rdi
   37091:	mov    0x48(%rsp),%rsi
   37096:	call   *0xefd14(%rip)        # 126db0 <_DYNAMIC+0x250>
   3709c:	ud2
   3709e:	mov    %rax,%r14
   370a1:	cmpq   $0x0,0xe0(%rsp)
   370aa:	je     37243 <meshopt_peers::run+0x28a3>
   370b0:	mov    0xe8(%rsp),%rdi
   370b8:	jmp    3723d <meshopt_peers::run+0x289d>
   370bd:	mov    %rax,%r14
   370c0:	cmpq   $0x0,0xe0(%rsp)
   370c9:	je     37259 <meshopt_peers::run+0x28b9>
   370cf:	mov    0xe8(%rsp),%r15
   370d7:	jmp    37106 <meshopt_peers::run+0x2766>
   370d9:	jmp    370db <meshopt_peers::run+0x273b>
   370db:	mov    %rax,%r14
   370de:	cmpq   $0x0,0x30(%rsp)
   370e4:	je     37178 <meshopt_peers::run+0x27d8>
   370ea:	jmp    371c4 <meshopt_peers::run+0x2824>
   370ef:	mov    %rax,%r14
   370f2:	jmp    37191 <meshopt_peers::run+0x27f1>
   370f7:	mov    %rax,%r14
   370fa:	cmpq   $0x0,0x68(%rsp)
   37100:	je     37259 <meshopt_peers::run+0x28b9>
   37106:	mov    %r15,%rdi
   37109:	jmp    37253 <meshopt_peers::run+0x28b3>
   3710e:	mov    %rax,%r14
   37111:	jmp    37243 <meshopt_peers::run+0x28a3>
   37116:	mov    %rax,%r14
   37119:	mov    %r12,0x10(%rsp)
   3711e:	mov    0x80(%rsp),%rax
   37126:	mov    %rax,0x70(%rsp)
   3712b:	cmpq   $0x0,0x70(%rsp)
   37131:	je     37269 <meshopt_peers::run+0x28c9>
   37137:	jmp    372e2 <meshopt_peers::run+0x2942>
   3713c:	mov    %rax,%r14
   3713f:	cmpq   $0x0,0x70(%rsp)
   37145:	je     37269 <meshopt_peers::run+0x28c9>
   3714b:	jmp    372e2 <meshopt_peers::run+0x2942>
   37150:	mov    %r15,0x40(%rsp)
   37155:	mov    %rax,%r14
   37158:	jmp    37259 <meshopt_peers::run+0x28b9>
   3715d:	mov    %rax,%r14
   37160:	cmpq   $0x0,0xa0(%rsp)
   37169:	jne    371a3 <meshopt_peers::run+0x2803>
   3716b:	test   %r15,%r15
   3716e:	jne    371b3 <meshopt_peers::run+0x2813>
   37170:	cmpq   $0x0,0x30(%rsp)
   37176:	jne    371c4 <meshopt_peers::run+0x2824>
   37178:	cmpq   $0x0,0x60(%rsp)
   3717e:	jne    371d7 <meshopt_peers::run+0x2837>
   37180:	cmpq   $0x0,0x8(%rsp)
   37186:	je     37191 <meshopt_peers::run+0x27f1>
   37188:	mov    %rbx,%rdi
   3718b:	call   *0xefbd7(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37191:	lea    0xe0(%rsp),%rdi
   37199:	call   335b0 <core::ptr::drop_glue::<core::option::Option<meshoptimizer_rs::clusterlod::Prepared>>>
   3719e:	jmp    3722a <meshopt_peers::run+0x288a>
   371a3:	mov    0x78(%rsp),%rdi
   371a8:	call   *0xefbba(%rip)        # 126d68 <free@GLIBC_2.2.5>
   371ae:	test   %r15,%r15
   371b1:	je     37170 <meshopt_peers::run+0x27d0>
   371b3:	mov    %r13,%rdi
   371b6:	call   *0xefbac(%rip)        # 126d68 <free@GLIBC_2.2.5>
   371bc:	cmpq   $0x0,0x30(%rsp)
   371c2:	je     37178 <meshopt_peers::run+0x27d8>
   371c4:	mov    0x58(%rsp),%rdi
   371c9:	call   *0xefb99(%rip)        # 126d68 <free@GLIBC_2.2.5>
   371cf:	cmpq   $0x0,0x60(%rsp)
   371d5:	je     37180 <meshopt_peers::run+0x27e0>
   371d7:	mov    0xa8(%rsp),%rdi
   371df:	call   *0xefb83(%rip)        # 126d68 <free@GLIBC_2.2.5>
   371e5:	cmpq   $0x0,0x8(%rsp)
   371eb:	jne    37188 <meshopt_peers::run+0x27e8>
   371ed:	jmp    37191 <meshopt_peers::run+0x27f1>
   371ef:	jmp    371f1 <meshopt_peers::run+0x2851>
   371f1:	mov    %rax,%r14
   371f4:	cmpq   $0x0,0x20(%rsp)
   371fa:	je     37281 <meshopt_peers::run+0x28e1>
   37200:	jmp    3730c <meshopt_peers::run+0x296c>
   37205:	mov    %rax,%r14
   37208:	lea    0x350(%rsp),%rdi
   37210:	call   33b40 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   37215:	mov    %r14,%rdi
   37218:	call   11ddb0 <_Unwind_Resume@plt>
   3721d:	jmp    372ca <meshopt_peers::run+0x292a>
   37222:	jmp    372ca <meshopt_peers::run+0x292a>
   37227:	mov    %rax,%r14
   3722a:	cmpq   $0x0,0x80(%rsp)
   37233:	je     37243 <meshopt_peers::run+0x28a3>
   37235:	mov    0xc0(%rsp),%rdi
   3723d:	call   *0xefb25(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37243:	cmpq   $0x0,0x68(%rsp)
   37249:	je     37259 <meshopt_peers::run+0x28b9>
   3724b:	mov    0xb0(%rsp),%rdi
   37253:	call   *0xefb0f(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37259:	cmpq   $0x0,0x40(%rsp)
   3725f:	jne    37295 <meshopt_peers::run+0x28f5>
   37261:	cmpq   $0x0,0x70(%rsp)
   37267:	jne    372e2 <meshopt_peers::run+0x2942>
   37269:	cmpq   $0x0,0x10(%rsp)
   3726f:	jne    372f5 <meshopt_peers::run+0x2955>
   37275:	cmpq   $0x0,0x20(%rsp)
   3727b:	jne    3730c <meshopt_peers::run+0x296c>
   37281:	cmpq   $0x0,0x150(%rsp)
   3728a:	jne    37325 <meshopt_peers::run+0x2985>
   37290:	jmp    37333 <meshopt_peers::run+0x2993>
   37295:	mov    0xd0(%rsp),%rdi
   3729d:	call   *0xefac5(%rip)        # 126d68 <free@GLIBC_2.2.5>
   372a3:	cmpq   $0x0,0x70(%rsp)
   372a9:	je     37269 <meshopt_peers::run+0x28c9>
   372ab:	jmp    372e2 <meshopt_peers::run+0x2942>
   372ad:	mov    %rax,%r14
   372b0:	cmpq   $0x0,0x150(%rsp)
   372b9:	jne    37325 <meshopt_peers::run+0x2985>
   372bb:	jmp    37333 <meshopt_peers::run+0x2993>
   372bd:	mov    %rax,%r14
   372c0:	jmp    37333 <meshopt_peers::run+0x2993>
   372c2:	mov    %rax,%r14
   372c5:	jmp    37352 <meshopt_peers::run+0x29b2>
   372ca:	mov    %rax,%r14
   372cd:	lea    0x350(%rsp),%rdi
   372d5:	call   33b80 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   372da:	cmpq   $0x0,0x70(%rsp)
   372e0:	je     37269 <meshopt_peers::run+0x28c9>
   372e2:	mov    0x28(%rsp),%rdi
   372e7:	call   *0xefa7b(%rip)        # 126d68 <free@GLIBC_2.2.5>
   372ed:	cmpq   $0x0,0x10(%rsp)
   372f3:	je     37275 <meshopt_peers::run+0x28d5>
   372f5:	mov    0x38(%rsp),%rdi
   372fa:	call   *0xefa68(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37300:	cmpq   $0x0,0x20(%rsp)
   37306:	je     37281 <meshopt_peers::run+0x28e1>
   3730c:	mov    0x90(%rsp),%rdi
   37314:	call   *0xefa4e(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3731a:	cmpq   $0x0,0x150(%rsp)
   37323:	je     37333 <meshopt_peers::run+0x2993>
   37325:	mov    0xc8(%rsp),%rdi
   3732d:	call   *0xefa35(%rip)        # 126d68 <free@GLIBC_2.2.5>
   37333:	mov    0x738(%rsp),%rax
   3733b:	mov    0x748(%rsp),%rcx
   37343:	sub    0x740(%rsp),%rcx
   3734b:	mov    %rcx,0x358(%rax)
   37352:	cmpq   $0x0,0x158(%rsp)
   3735b:	jne    3736d <meshopt_peers::run+0x29cd>
   3735d:	cmpq   $0x0,0x20(%rsp)
   37363:	jne    37383 <meshopt_peers::run+0x29e3>
   37365:	mov    %r14,%rdi
   37368:	call   11ddb0 <_Unwind_Resume@plt>
   3736d:	mov    0x160(%rsp),%rdi
   37375:	call   *0xef9ed(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3737b:	cmpq   $0x0,0x20(%rsp)
   37381:	je     37365 <meshopt_peers::run+0x29c5>
   37383:	mov    0x50(%rsp),%rdi
   37388:	call   *0xef9da(%rip)        # 126d68 <free@GLIBC_2.2.5>
   3738e:	mov    %r14,%rdi
   37391:	call   11ddb0 <_Unwind_Resume@plt>
