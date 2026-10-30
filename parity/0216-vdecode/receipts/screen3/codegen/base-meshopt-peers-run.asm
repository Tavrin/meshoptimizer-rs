
<artifacts>/screen3/driver-base:     file format elf64-x86-64


Disassembly of section .text:

0000000000034860 <meshopt_peers::run>:
   34860:	push   %rbp
   34861:	push   %r15
   34863:	push   %r14
   34865:	push   %r13
   34867:	push   %r12
   34869:	push   %rbx
   3486a:	sub    $0x1000,%rsp
   34871:	movq   $0x0,(%rsp)
   34879:	sub    $0x948,%rsp
   34880:	mov    %rsi,0x290(%rsp)
   34888:	mov    %rdx,0x298(%rsp)
   34890:	mov    0x10(%rcx),%rbx
   34894:	mov    0x8(%rcx),%r15
   34898:	movb   $0x2,0x6c0(%rsp)
   348a0:	mov    %r15,0x6c8(%rsp)
   348a8:	mov    %rbx,0x6d0(%rsp)
   348b0:	movq   $0x0,0x6f8(%rsp)
   348bc:	mov    0x78(%rcx),%rax
   348c0:	movd   0x80(%rcx),%xmm0
   348c8:	mov    %rax,0x230(%rsp)
   348d0:	movd   %xmm0,0x238(%rsp)
   348d9:	movl   $0x0,0x23c(%rsp)
   348e4:	add    $0xfffffffffffffffb,%rdx
   348e8:	cmp    $0xe,%rdx
   348ec:	ja     36d06 <meshopt_peers::run+0x24a6>
   348f2:	mov    %r8,%r12
   348f5:	mov    %rdi,%r14
   348f8:	lea    -0x2448b(%rip),%rdi        # 10474 <libm::math::generic::sqrt::RSQRT_TAB+0x116>
   348ff:	movslq (%rdi,%rdx,4),%rdx
   34903:	add    %rdi,%rdx
   34906:	jmp    *%rdx
   34908:	mov    $0x68636163,%eax
   3490d:	xor    (%rsi),%eax
   3490f:	movzbl 0x4(%rsi),%edx
   34913:	xor    $0x65,%edx
   34916:	or     %eax,%edx
   34918:	je     35102 <meshopt_peers::run+0x8a2>
   3491e:	mov    $0x63746566,%eax
   34923:	xor    (%rsi),%eax
   34925:	movzbl 0x4(%rsi),%edx
   34929:	xor    $0x68,%edx
   3492c:	or     %eax,%edx
   3492e:	jne    36d06 <meshopt_peers::run+0x24a6>
   34934:	mov    %rbx,%rax
   34937:	shr    $0x20,%rax
   3493b:	jne    37173 <meshopt_peers::run+0x2913>
   34941:	mov    0x20(%rcx),%rsi
   34945:	mov    0x28(%rcx),%rdx
   34949:	lea    0x0(,%rbx,4),%rax
   34951:	lea    (%rax,%rax,2),%rax
   34955:	mov    %r15,0x2a0(%rsp)
   3495d:	mov    %rax,0x2a8(%rsp)
   34965:	mov    %rbx,0x2b0(%rsp)
   3496d:	movq   $0xc,0x2b8(%rsp)
   34979:	movq   $0xc,0x2c0(%rsp)
   34985:	movq   $0x0,0x2c8(%rsp)
   34991:	lea    0x718(%rsp),%rdi
   34999:	lea    0x2a0(%rsp),%rcx
   349a1:	mov    %r12,%r8
   349a4:	call   *0xf20ce(%rip)        # 126a78 <_DYNAMIC+0x5d0>
   349aa:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   349b3:	je     36d8c <meshopt_peers::run+0x252c>
   349b9:	mov    0x748(%rsp),%rax
   349c1:	mov    %rax,0x2d0(%rsp)
   349c9:	movups 0x718(%rsp),%xmm0
   349d1:	movups 0x728(%rsp),%xmm1
   349d9:	movups 0x738(%rsp),%xmm2
   349e1:	movaps %xmm2,0x2c0(%rsp)
   349e9:	movaps %xmm1,0x2b0(%rsp)
   349f1:	movaps %xmm0,0x2a0(%rsp)
   349f9:	mov    0x2b0(%rsp),%rax
   34a01:	mov    %rax,0x18(%r14)
   34a05:	movups %xmm0,0x8(%r14)
   34a0a:	movups 0x2b8(%rsp),%xmm0
   34a12:	movups %xmm0,0x20(%r14)
   34a17:	mov    0x2c8(%rsp),%rax
   34a1f:	mov    %rax,0x30(%r14)
   34a23:	movl   $0x3,(%r14)
   34a2a:	jmp    355fa <meshopt_peers::run+0xd9a>
   34a2f:	movabs $0x7966696c706d6973,%rdx
   34a39:	xor    (%rsi),%rdx
   34a3c:	movabs $0x7970706f6c732d79,%rdi
   34a46:	xor    0x7(%rsi),%rdi
   34a4a:	or     %rdx,%rdi
   34a4d:	jne    36d06 <meshopt_peers::run+0x24a6>
   34a53:	movdqu 0x20(%rcx),%xmm1
   34a58:	movq   $0x0,0x128(%rsp)
   34a64:	mov    %rax,0xc8(%rsp)
   34a6c:	movd   %xmm0,0x270(%rsp)
   34a75:	movb   $0x0,0x365(%r12)
   34a7e:	movl   $0x0,0x361(%r12)
   34a8a:	pxor   %xmm0,%xmm0
   34a8e:	movdqu %xmm0,0x350(%r12)
   34a98:	mov    0x348(%r12),%rax
   34aa0:	mov    %rax,0x1c0(%rsp)
   34aa8:	mov    %rax,0x1c8(%rsp)
   34ab0:	movdqu %xmm1,0x718(%rsp)
   34ab9:	lea    0x6c0(%rsp),%rax
   34ac1:	mov    %rax,0x728(%rsp)
   34ac9:	lea    0x1c0(%rsp),%rax
   34ad1:	mov    %rax,0x730(%rsp)
   34ad9:	lea    0x128(%rsp),%rax
   34ae1:	mov    %rax,0x738(%rsp)
   34ae9:	lea    0xc8(%rsp),%rax
   34af1:	mov    %rax,0x740(%rsp)
   34af9:	lea    0x270(%rsp),%rax
   34b01:	mov    %rax,0x748(%rsp)
   34b09:	mov    %r12,0x750(%rsp)
   34b11:	movq   $0x0,0x758(%rsp)
   34b1d:	lea    0x2a0(%rsp),%rdi
   34b25:	lea    0x718(%rsp),%rsi
   34b2d:	call   837f0 <meshoptimizer_rs::simplify::extra::simplify_sloppy_kernel::<false>::{closure#0}>
   34b32:	mov    0x1c8(%rsp),%rax
   34b3a:	sub    0x1c0(%rsp),%rax
   34b42:	mov    %rax,0x358(%r12)
   34b4a:	mov    0x2a0(%rsp),%rax
   34b52:	movzbl 0x2a8(%rsp),%ecx
   34b5a:	cmp    $0xffffffffffffffff,%rax
   34b5e:	je     34b82 <meshopt_peers::run+0x322>
   34b60:	mov    0x2b8(%rsp),%rdx
   34b68:	mov    %rdx,0x648(%rsp)
   34b70:	movdqu 0x2a9(%rsp),%xmm0
   34b79:	movdqu %xmm0,0x639(%rsp)
   34b82:	mov    %cl,0x638(%rsp)
   34b89:	mov    %rax,0x630(%rsp)
   34b91:	jmp    34ffc <meshopt_peers::run+0x79c>
   34b96:	movdqu (%rsi),%xmm0
   34b9a:	movdqu 0x3(%rsi),%xmm1
   34b9f:	pcmpeqb -0x26be7(%rip),%xmm1        # dfc0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0xb0>
   34ba7:	pcmpeqb -0x2706f(%rip),%xmm0        # db40 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x330>
   34baf:	pand   %xmm1,%xmm0
   34bb3:	pmovmskb %xmm0,%eax
   34bb7:	cmp    $0xffff,%eax
   34bbc:	jne    36d06 <meshopt_peers::run+0x24a6>
   34bc2:	mov    0x40(%rcx),%rax
   34bc6:	test   %rbx,%rbx
   34bc9:	setne  %dl
   34bcc:	lea    0x0(,%rbx,4),%rsi
   34bd4:	cmp    %rax,%rsi
   34bd7:	seta   %sil
   34bdb:	and    %dl,%sil
   34bde:	cmp    $0x1,%sil
   34be2:	je     3712c <meshopt_peers::run+0x28cc>
   34be8:	mov    0x20(%rcx),%rsi
   34bec:	mov    0x28(%rcx),%rdx
   34bf0:	mov    0x38(%rcx),%rcx
   34bf4:	movb   $0x0,0x718(%rsp)
   34bfc:	mov    %rcx,0x720(%rsp)
   34c04:	mov    %rax,0x728(%rsp)
   34c0c:	mov    %rbx,0x730(%rsp)
   34c14:	movq   $0x4,0x738(%rsp)
   34c20:	movq   $0x4,0x740(%rsp)
   34c2c:	pxor   %xmm0,%xmm0
   34c30:	movdqu %xmm0,0x748(%rsp)
   34c39:	sub    $0x30,%rsp
   34c3d:	mov    %r12,0x20(%rsp)
   34c42:	lea    0x260(%rsp),%rax
   34c4a:	mov    %rax,0x18(%rsp)
   34c4f:	movq   $0x0,0x8(%rsp)
   34c58:	movq   $0x4,(%rsp)
   34c60:	lea    -0x26c57(%rip),%r9        # e010 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x100>
   34c67:	lea    0x660(%rsp),%rdi
   34c6f:	lea    0x6f0(%rsp),%rcx
   34c77:	lea    0x748(%rsp),%r8
   34c7f:	jmp    34feb <meshopt_peers::run+0x78b>
   34c84:	movabs $0x7966696c706d6973,%rax
   34c8e:	cmp    %rax,(%rsi)
   34c91:	je     34f9e <meshopt_peers::run+0x73e>
   34c97:	movabs $0x776172647265766f,%rax
   34ca1:	cmp    %rax,(%rsi)
   34ca4:	je     354c0 <meshopt_peers::run+0xc60>
   34caa:	movabs $0x7374656c6873656d,%rax
   34cb4:	cmp    %rax,(%rsi)
   34cb7:	jne    36d06 <meshopt_peers::run+0x24a6>
   34cbd:	mov    0x20(%rcx),%rsi
   34cc1:	mov    0x28(%rcx),%rdx
   34cc5:	movq   $0x7c,0x2b0(%rsp)
   34cd1:	movq   $0x0,0x2a4(%rsp)
   34cdd:	movl   $0x1,0x2a0(%rsp)
   34ce8:	lea    0x2a0(%rsp),%rax
   34cf0:	lea    0x718(%rsp),%rdi
   34cf8:	lea    0x6c0(%rsp),%r8
   34d00:	mov    $0x40,%r9d
   34d06:	mov    %rbx,%rcx
   34d09:	push   $0x2
   34d0b:	push   %r12
   34d0d:	push   %rax
   34d0e:	push   $0x7c
   34d10:	call   b3980 <meshoptimizer_rs::meshlet::build>
   34d15:	add    $0x20,%rsp
   34d19:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   34d22:	je     36e54 <meshopt_peers::run+0x25f4>
   34d28:	mov    0x758(%rsp),%rax
   34d30:	mov    %rax,0x2e0(%rsp)
   34d38:	movups 0x718(%rsp),%xmm0
   34d40:	movups 0x728(%rsp),%xmm1
   34d48:	movups 0x738(%rsp),%xmm2
   34d50:	movups 0x748(%rsp),%xmm3
   34d58:	movaps %xmm3,0x2d0(%rsp)
   34d60:	movaps %xmm2,0x2c0(%rsp)
   34d68:	movaps %xmm1,0x2b0(%rsp)
   34d70:	movaps %xmm0,0x2a0(%rsp)
   34d78:	mov    0x2a8(%rsp),%rbx
   34d80:	mov    0x2b0(%rsp),%r12
   34d88:	test   %r12,%r12
   34d8b:	je     3551d <meshopt_peers::run+0xcbd>
   34d91:	mov    %r12,%r15
   34d94:	shl    $0x4,%r15
   34d98:	mov    %r15,%rdi
   34d9b:	call   *0xf18ff(%rip)        # 1266a0 <malloc@GLIBC_2.2.5>
   34da1:	test   %rax,%rax
   34da4:	je     371ea <meshopt_peers::run+0x298a>
   34daa:	mov    %r12d,%ecx
   34dad:	and    $0x3,%ecx
   34db0:	cmp    $0x4,%r12
   34db4:	jae    35524 <meshopt_peers::run+0xcc4>
   34dba:	xor    %edx,%edx
   34dbc:	jmp    35578 <meshopt_peers::run+0xd18>
   34dc1:	movabs $0x65646f6365642d78,%rax
   34dcb:	movabs $0x652d786574726576,%rdx
   34dd5:	xor    (%rsi),%rdx
   34dd8:	lea    0x9010000(%rax),%rdi
   34ddf:	xor    0x5(%rsi),%rdi
   34de3:	or     %rdx,%rdi
   34de6:	je     3503a <meshopt_peers::run+0x7da>
   34dec:	movabs $0x642d786574726576,%rdx
   34df6:	xor    (%rsi),%rdx
   34df9:	xor    0x5(%rsi),%rax
   34dfd:	or     %rdx,%rax
   34e00:	jne    36d06 <meshopt_peers::run+0x24a6>
   34e06:	mov    0x50(%rcx),%r9
   34e0a:	lea    0x718(%rsp),%rdi
   34e12:	mov    $0xc,%r8d
   34e18:	xor    %esi,%esi
   34e1a:	xor    %edx,%edx
   34e1c:	mov    %rcx,%rax
   34e1f:	mov    %rbx,%rcx
   34e22:	push   %r12
   34e24:	push   0x58(%rax)
   34e27:	call   b2ea0 <meshoptimizer_rs::codec::allocate>
   34e2c:	add    $0x10,%rsp
   34e30:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   34e39:	jne    350dd <meshopt_peers::run+0x87d>
   34e3f:	movzbl 0x720(%rsp),%eax
   34e47:	mov    %al,0x2a0(%rsp)
   34e4e:	lea    -0x2492a(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   34e55:	lea    0xea81c(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   34e5c:	lea    0xea4e5(%rip),%r8        # 11f348 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1b0>
   34e63:	lea    0x2a0(%rsp),%rdx
   34e6b:	mov    $0x2b,%esi
   34e70:	call   *0xf1902(%rip)        # 126778 <_DYNAMIC+0x2d0>
   34e76:	movabs $0x6e652d7865646e69,%rax
   34e80:	xor    (%rsi),%rax
   34e83:	mov    0x8(%rsi),%edx
   34e86:	xor    $0x65646f63,%rdx
   34e8d:	or     %rax,%rdx
   34e90:	je     350b0 <meshopt_peers::run+0x850>
   34e96:	movabs $0x65642d7865646e69,%rax
   34ea0:	xor    (%rsi),%rax
   34ea3:	mov    0x8(%rsi),%edx
   34ea6:	xor    $0x65646f63,%rdx
   34ead:	or     %rax,%rdx
   34eb0:	jne    36d06 <meshopt_peers::run+0x24a6>
   34eb6:	mov    0x28(%rcx),%rax
   34eba:	mov    0x68(%rcx),%r9
   34ebe:	lea    0x718(%rsp),%rdi
   34ec6:	mov    $0x4,%r8d
   34ecc:	mov    $0x1,%esi
   34ed1:	xor    %edx,%edx
   34ed3:	mov    %rcx,%r10
   34ed6:	mov    %rax,%rcx
   34ed9:	push   %r12
   34edb:	push   0x70(%r10)
   34edf:	call   b2ea0 <meshoptimizer_rs::codec::allocate>
   34ee4:	add    $0x10,%rsp
   34ee8:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   34ef1:	jne    350dd <meshopt_peers::run+0x87d>
   34ef7:	movzbl 0x720(%rsp),%eax
   34eff:	mov    %al,0x2a0(%rsp)
   34f06:	lea    -0x249e2(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   34f0d:	lea    0xea764(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   34f14:	lea    0xea3fd(%rip),%r8        # 11f318 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x180>
   34f1b:	lea    0x2a0(%rsp),%rdx
   34f23:	mov    $0x2b,%esi
   34f28:	call   *0xf184a(%rip)        # 126778 <_DYNAMIC+0x2d0>
   34f2e:	movabs $0x2d72657473756c63,%rax
   34f38:	xor    (%rsi),%rax
   34f3b:	movabs $0x646f6c2d72657473,%rdx
   34f45:	xor    0x3(%rsi),%rdx
   34f49:	or     %rax,%rdx
   34f4c:	jne    36d06 <meshopt_peers::run+0x24a6>
   34f52:	mov    %r14,0x258(%rsp)
   34f5a:	test   %rbx,%rbx
   34f5d:	je     35164 <meshopt_peers::run+0x904>
   34f63:	mov    %rcx,%rbp
   34f66:	lea    0x0(,%rbx,4),%rax
   34f6e:	lea    (%rax,%rax,2),%r13
   34f72:	mov    %r13,%rdi
   34f75:	call   *0xf1725(%rip)        # 1266a0 <malloc@GLIBC_2.2.5>
   34f7b:	test   %rax,%rax
   34f7e:	je     371a3 <meshopt_peers::run+0x2943>
   34f84:	mov    %rax,%r14
   34f87:	mov    %rax,%rdi
   34f8a:	mov    %r15,%rsi
   34f8d:	mov    %r13,%rdx
   34f90:	call   *0xf172a(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   34f96:	mov    %rbp,%rcx
   34f99:	jmp    3516a <meshopt_peers::run+0x90a>
   34f9e:	mov    0x20(%rcx),%rsi
   34fa2:	mov    0x28(%rcx),%rdx
   34fa6:	movb   $0x2,0x718(%rsp)
   34fae:	sub    $0x30,%rsp
   34fb2:	mov    %r12,0x20(%rsp)
   34fb7:	lea    0x260(%rsp),%rax
   34fbf:	mov    %rax,0x18(%rsp)
   34fc4:	pxor   %xmm0,%xmm0
   34fc8:	movdqu %xmm0,(%rsp)
   34fcd:	lea    0x660(%rsp),%rdi
   34fd5:	lea    0x6f0(%rsp),%rcx
   34fdd:	lea    0x748(%rsp),%r8
   34fe5:	mov    $0x4,%r9d
   34feb:	call   be620 <meshoptimizer_rs::simplify::allocating>
   34ff0:	add    $0x30,%rsp
   34ff4:	mov    0x630(%rsp),%rax
   34ffc:	cmp    $0xffffffffffffffff,%rax
   35000:	je     36ccf <meshopt_peers::run+0x246f>
   35006:	mov    0x640(%rsp),%rax
   3500e:	mov    %rax,0x18(%r14)
   35012:	movups 0x630(%rsp),%xmm0
   3501a:	movups %xmm0,0x8(%r14)
   3501f:	movss  0x648(%rsp),%xmm0
   35028:	movss  %xmm0,0x4(%r14)
   3502e:	movl   $0x2,(%r14)
   35035:	jmp    355fa <meshopt_peers::run+0xd9a>
   3503a:	lea    0x0(,%rbx,4),%rax
   35042:	lea    (%rax,%rax,2),%rdx
   35046:	lea    0x718(%rsp),%rdi
   3504e:	mov    $0xc,%r8d
   35054:	mov    %r15,%rsi
   35057:	mov    %rbx,%rcx
   3505a:	mov    $0x1,%r9d
   35060:	push   %r12
   35062:	push   $0x2
   35064:	call   *0xf1a16(%rip)        # 126a80 <_DYNAMIC+0x5d8>
   3506a:	add    $0x10,%rsp
   3506e:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   35077:	jne    350dd <meshopt_peers::run+0x87d>
   35079:	movzbl 0x720(%rsp),%eax
   35081:	mov    %al,0x2a0(%rsp)
   35088:	lea    -0x24b64(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3508f:	lea    0xea5e2(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   35096:	lea    0xea2c3(%rip),%r8        # 11f360 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1c8>
   3509d:	lea    0x2a0(%rsp),%rdx
   350a5:	mov    $0x2b,%esi
   350aa:	call   *0xf16c8(%rip)        # 126778 <_DYNAMIC+0x2d0>
   350b0:	mov    0x20(%rcx),%rsi
   350b4:	mov    0x28(%rcx),%rdx
   350b8:	lea    0x718(%rsp),%rdi
   350c0:	mov    $0x1,%ecx
   350c5:	mov    %r12,%r8
   350c8:	call   *0xf19ba(%rip)        # 126a88 <_DYNAMIC+0x5e0>
   350ce:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   350d7:	je     36e8b <meshopt_peers::run+0x262b>
   350dd:	mov    0x728(%rsp),%rax
   350e5:	mov    %rax,0x18(%r14)
   350e9:	movups 0x718(%rsp),%xmm0
   350f1:	movups %xmm0,0x8(%r14)
   350f6:	movl   $0x0,(%r14)
   350fd:	jmp    355fa <meshopt_peers::run+0xd9a>
   35102:	mov    0x20(%rcx),%rsi
   35106:	mov    0x28(%rcx),%rdx
   3510a:	lea    0x718(%rsp),%rdi
   35112:	mov    %rbx,%rcx
   35115:	mov    %r12,%r8
   35118:	call   *0xf1972(%rip)        # 126a90 <_DYNAMIC+0x5e8>
   3511e:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   35127:	jne    354f8 <meshopt_peers::run+0xc98>
   3512d:	movzbl 0x720(%rsp),%eax
   35135:	mov    %al,0x2a0(%rsp)
   3513c:	lea    -0x24c18(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   35143:	lea    0xea52e(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   3514a:	lea    0xea287(%rip),%r8        # 11f3d8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x240>
   35151:	lea    0x2a0(%rsp),%rdx
   35159:	mov    $0x2b,%esi
   3515e:	call   *0xf1614(%rip)        # 126778 <_DYNAMIC+0x2d0>
   35164:	mov    $0x4,%r14d
   3516a:	movq   $0x0,0x1a8(%rsp)
   35176:	movq   $0x4,0x1b0(%rsp)
   35182:	movq   $0x0,0x1b8(%rsp)
   3518e:	movl   $0x0,0x8c(%rsp)
   35199:	movq   $0x80,0x1c0(%rsp)
   351a5:	movq   $0x2a,0x1c8(%rsp)
   351b1:	movq   $0x80,0x1d0(%rsp)
   351bd:	movq   $0x10,0x1d8(%rsp)
   351c9:	movaps -0x27370(%rip),%xmm0        # de60 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x650>
   351d0:	movups %xmm0,0x1e0(%rsp)
   351d8:	movaps -0x272bf(%rip),%xmm0        # df20 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x10>
   351df:	movups %xmm0,0x1f0(%rsp)
   351e7:	movabs $0x100010100000101,%rax
   351f1:	mov    %rax,0x200(%rsp)
   351f9:	movl   $0x0,0x208(%rsp)
   35204:	movb   $0x1,0x20c(%rsp)
   3520c:	mov    0x20(%rcx),%r13
   35210:	mov    0x28(%rcx),%r15
   35214:	mov    %r13,0x678(%rsp)
   3521c:	mov    %r15,0x680(%rsp)
   35224:	mov    %r14,0x50(%rsp)
   35229:	mov    %r14,0x688(%rsp)
   35231:	mov    %rbx,0x10(%rsp)
   35236:	mov    %rbx,0x690(%rsp)
   3523e:	movb   $0x2,0x630(%rsp)
   35246:	movq   $0x4,0x698(%rsp)
   35252:	xorps  %xmm0,%xmm0
   35255:	movups %xmm0,0x6a0(%rsp)
   3525d:	movl   $0x0,0x6b8(%rsp)
   35268:	movb   $0x0,0x365(%r12)
   35271:	movl   $0x0,0x361(%r12)
   3527d:	movups %xmm0,0x350(%r12)
   35286:	mov    0x348(%r12),%rbx
   3528e:	mov    %r12,%rdi
   35291:	xor    %esi,%esi
   35293:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35298:	mov    %eax,%ebp
   3529a:	cmp    $0xff,%al
   3529c:	jne    372ef <meshopt_peers::run+0x2a8f>
   352a2:	mov    %r12,0x718(%rsp)
   352aa:	mov    %rbx,0x720(%rsp)
   352b2:	mov    %rbx,0x728(%rsp)
   352ba:	movq   $0x0,0x730(%rsp)
   352c6:	movl   $0x0,0x738(%rsp)
   352d1:	movl   $0x0,0x93c(%rsp)
   352dc:	movb   $0x1,0x1940(%rsp)
   352e4:	lea    0x630(%rsp),%rdi
   352ec:	lea    0x1c0(%rsp),%rsi
   352f4:	lea    0x718(%rsp),%rdx
   352fc:	mov    0x10(%rsp),%rbx
   35301:	call   *0xf1791(%rip)        # 126a98 <_DYNAMIC+0x5f0>
   35307:	mov    %eax,%ebp
   35309:	cmp    $0xff,%al
   3530b:	jne    372d0 <meshopt_peers::run+0x2a70>
   35311:	movzbl 0x1940(%rsp),%r12d
   3531a:	movb   $0x2,0x2a0(%rsp)
   35322:	mov    0x50(%rsp),%rax
   35327:	mov    %rax,0x2a8(%rsp)
   3532f:	mov    %rbx,0x2b0(%rsp)
   35337:	movq   $0x0,0x2d8(%rsp)
   35343:	lea    0x128(%rsp),%rdi
   3534b:	lea    0x2a0(%rsp),%rsi
   35353:	lea    0x718(%rsp),%rdx
   3535b:	call   *0xf173f(%rip)        # 126aa0 <_DYNAMIC+0x5f8>
   35361:	mov    0x128(%rsp),%r14
   35369:	movzbl 0x130(%rsp),%ebp
   35371:	cmp    $0xffffffffffffffff,%r14
   35375:	je     3545e <meshopt_peers::run+0xbfe>
   3537b:	mov    %r14,0x188(%rsp)
   35383:	movzbl %bpl,%eax
   35387:	movzbl 0x137(%rsp),%ecx
   3538f:	shl    $0x10,%ecx
   35392:	movzwl 0x135(%rsp),%edx
   3539a:	or     %ecx,%edx
   3539c:	mov    0x131(%rsp),%ecx
   353a3:	shl    $0x28,%rdx
   353a7:	shl    $0x8,%rcx
   353ab:	or     %rdx,%rcx
   353ae:	or     %rax,%rcx
   353b1:	mov    %rcx,0xc0(%rsp)
   353b9:	mov    0x730(%rsp),%rsi
   353c1:	add    %rbx,%rsi
   353c4:	jb     36dc3 <meshopt_peers::run+0x2563>
   353ca:	mov    0x138(%rsp),%rax
   353d2:	mov    %rax,0x228(%rsp)
   353da:	mov    %rsi,0x730(%rsp)
   353e2:	mov    0x718(%rsp),%rdi
   353ea:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   353ef:	mov    %eax,%ebp
   353f1:	cmp    $0xff,%al
   353f3:	jne    35445 <meshopt_peers::run+0xbe5>
   353f5:	mov    $0x1,%eax
   353fa:	mov    %rax,0x80(%rsp)
   35402:	test   %rbx,%rbx
   35405:	jne    35a63 <meshopt_peers::run+0x1203>
   3540b:	mov    0x718(%rsp),%rdi
   35413:	mov    0x730(%rsp),%rsi
   3541b:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35420:	mov    %eax,%ebp
   35422:	mov    $0x1,%r14b
   35425:	cmp    $0xff,%al
   35427:	je     3560c <meshopt_peers::run+0xdac>
   3542d:	test   %rbx,%rbx
   35430:	je     35440 <meshopt_peers::run+0xbe0>
   35432:	mov    0x80(%rsp),%rdi
   3543a:	call   *0xf1270(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   35440:	test   %r14b,%r14b
   35443:	je     3545e <meshopt_peers::run+0xbfe>
   35445:	cmpq   $0x0,0x188(%rsp)
   3544e:	je     3545e <meshopt_peers::run+0xbfe>
   35450:	mov    0xc0(%rsp),%rdi
   35458:	call   *0xf1252(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   3545e:	mov    0x718(%rsp),%rax
   35466:	mov    0x728(%rsp),%rcx
   3546e:	sub    0x720(%rsp),%rcx
   35476:	mov    %rcx,0x358(%rax)
   3547d:	cmp    $0xff,%bpl
   35481:	jne    372ef <meshopt_peers::run+0x2a8f>
   35487:	mov    0x1b8(%rsp),%rax
   3548f:	mov    0x258(%rsp),%rcx
   35497:	mov    %rax,0x18(%rcx)
   3549b:	movups 0x1a8(%rsp),%xmm0
   354a3:	movups %xmm0,0x8(%rcx)
   354a7:	movl   $0x1,(%rcx)
   354ad:	test   %rbx,%rbx
   354b0:	je     355fa <meshopt_peers::run+0xd9a>
   354b6:	mov    0x50(%rsp),%rdi
   354bb:	jmp    355f4 <meshopt_peers::run+0xd94>
   354c0:	mov    0x20(%rcx),%rsi
   354c4:	mov    0x28(%rcx),%rdx
   354c8:	lea    0x718(%rsp),%rdi
   354d0:	lea    0x6c0(%rsp),%rcx
   354d8:	movd   -0x25908(%rip),%xmm0        # fbd8 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x13c>
   354e0:	mov    %r12,%r8
   354e3:	call   *0xf15bf(%rip)        # 126aa8 <_DYNAMIC+0x600>
   354e9:	cmpq   $0xffffffffffffffff,0x718(%rsp)
   354f2:	je     36f2a <meshopt_peers::run+0x26ca>
   354f8:	mov    0x728(%rsp),%rax
   35500:	mov    %rax,0x18(%r14)
   35504:	movups 0x718(%rsp),%xmm0
   3550c:	movups %xmm0,0x8(%r14)
   35511:	movl   $0x1,(%r14)
   35518:	jmp    355fa <meshopt_peers::run+0xd9a>
   3551d:	mov    $0x4,%eax
   35522:	jmp    355a1 <meshopt_peers::run+0xd41>
   35524:	mov    %r12,%rsi
   35527:	and    $0xfffffffffffffffc,%rsi
   3552b:	mov    $0x30,%edi
   35530:	xor    %edx,%edx
   35532:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   35540:	movups -0x30(%rbx,%rdi,1),%xmm0
   35545:	movups %xmm0,-0x30(%rax,%rdi,1)
   3554a:	movups -0x20(%rbx,%rdi,1),%xmm0
   3554f:	movups %xmm0,-0x20(%rax,%rdi,1)
   35554:	movups -0x10(%rbx,%rdi,1),%xmm0
   35559:	movups %xmm0,-0x10(%rax,%rdi,1)
   3555e:	movups (%rbx,%rdi,1),%xmm0
   35562:	movups %xmm0,(%rax,%rdi,1)
   35566:	add    $0x4,%rdx
   3556a:	add    $0x40,%rdi
   3556e:	cmp    %rdx,%rsi
   35571:	jne    35540 <meshopt_peers::run+0xce0>
   35573:	test   %rcx,%rcx
   35576:	je     355a1 <meshopt_peers::run+0xd41>
   35578:	shl    $0x4,%rdx
   3557c:	mov    %rax,%rsi
   3557f:	add    %rdx,%rsi
   35582:	add    %rbx,%rdx
   35585:	shl    $0x4,%ecx
   35588:	xor    %edi,%edi
   3558a:	nopw   0x0(%rax,%rax,1)
   35590:	movups (%rdx,%rdi,1),%xmm0
   35594:	movups %xmm0,(%rsi,%rdi,1)
   35598:	add    $0x10,%rdi
   3559c:	cmp    %rdi,%rcx
   3559f:	jne    35590 <meshopt_peers::run+0xd30>
   355a1:	mov    0x2c8(%rsp),%rcx
   355a9:	mov    %rcx,0x30(%r14)
   355ad:	movups 0x2b8(%rsp),%xmm0
   355b5:	movups %xmm0,0x20(%r14)
   355ba:	movaps 0x2d0(%rsp),%xmm0
   355c2:	movups %xmm0,0x38(%r14)
   355c7:	mov    0x2e0(%rsp),%rcx
   355cf:	mov    %rcx,0x48(%r14)
   355d3:	mov    %r12,0x8(%r14)
   355d7:	mov    %rax,0x10(%r14)
   355db:	mov    %r12,0x18(%r14)
   355df:	movl   $0x4,(%r14)
   355e6:	cmpq   $0x0,0x2a0(%rsp)
   355ef:	je     355fa <meshopt_peers::run+0xd9a>
   355f1:	mov    %rbx,%rdi
   355f4:	call   *0xf10b6(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   355fa:	add    $0x1948,%rsp
   35601:	pop    %rbx
   35602:	pop    %r12
   35604:	pop    %r13
   35606:	pop    %r14
   35608:	pop    %r15
   3560a:	pop    %rbp
   3560b:	ret
   3560c:	movb   $0x2,0x128(%rsp)
   35614:	mov    0x50(%rsp),%rax
   35619:	mov    %rax,0x130(%rsp)
   35621:	mov    %rbx,0x138(%rsp)
   35629:	movq   $0x0,0x160(%rsp)
   35635:	movups 0x200(%rsp),%xmm0
   3563d:	movaps %xmm0,0x2e0(%rsp)
   35645:	movups 0x1c0(%rsp),%xmm0
   3564d:	movups 0x1d0(%rsp),%xmm1
   35655:	movups 0x1e0(%rsp),%xmm2
   3565d:	movups 0x1f0(%rsp),%xmm3
   35665:	movaps %xmm3,0x2d0(%rsp)
   3566d:	movaps %xmm2,0x2c0(%rsp)
   35675:	movaps %xmm1,0x2b0(%rsp)
   3567d:	movaps %xmm0,0x2a0(%rsp)
   35685:	movzbl %r12b,%r10d
   35689:	lea    0xc8(%rsp),%rdi
   35691:	lea    0x128(%rsp),%rcx
   35699:	lea    0x2a0(%rsp),%r8
   356a1:	mov    %r13,%rsi
   356a4:	mov    %r15,%rdx
   356a7:	mov    $0xffffffff,%r9d
   356ad:	lea    0x718(%rsp),%rax
   356b5:	push   %rax
   356b6:	mov    %r10,0x228(%rsp)
   356be:	push   %r10
   356c0:	call   *0xf13ea(%rip)        # 126ab0 <_DYNAMIC+0x608>
   356c6:	add    $0x10,%rsp
   356ca:	mov    0xc8(%rsp),%rdi
   356d2:	movzbl 0xd0(%rsp),%ebp
   356da:	cmp    $0xffffffffffffffff,%rdi
   356de:	je     3542d <meshopt_peers::run+0xbcd>
   356e4:	movzbl %bpl,%eax
   356e8:	movzbl 0xd7(%rsp),%ecx
   356f0:	shl    $0x10,%ecx
   356f3:	movzwl 0xd5(%rsp),%edx
   356fb:	or     %ecx,%edx
   356fd:	mov    0xd1(%rsp),%ecx
   35704:	mov    0xd8(%rsp),%r9
   3570c:	mov    0xe0(%rsp),%r8
   35714:	mov    0xe8(%rsp),%rsi
   3571c:	mov    %rsi,0x20(%rsp)
   35721:	shl    $0x28,%rdx
   35725:	shl    $0x8,%rcx
   35729:	or     %rdx,%rcx
   3572c:	or     %rax,%rcx
   3572f:	mov    %rcx,0x30(%rsp)
   35734:	test   %r9,%r9
   35737:	je     35a27 <meshopt_peers::run+0x11c7>
   3573d:	mov    %r8,0x68(%rsp)
   35742:	mov    %rdi,0x8(%rsp)
   35747:	mov    0xf0(%rsp),%r14
   3574f:	mov    %r9,0x120(%rsp)
   35757:	mov    %r9,%rax
   3575a:	shl    $0x4,%rax
   3575e:	lea    (%rax,%rax,2),%rax
   35762:	mov    %rax,0x18(%rsp)
   35767:	xor    %r12d,%r12d
   3576a:	mov    0x718(%rsp),%r15
   35772:	mov    0x720(%rsp),%rbp
   3577a:	mov    0x728(%rsp),%rax
   35782:	sub    %rbp,%rax
   35785:	mov    %rax,0x358(%r15)
   3578c:	mov    %r15,%rdi
   3578f:	call   *0xf12cb(%rip)        # 126a60 <_DYNAMIC+0x5b8>
   35795:	mov    0x340(%r15),%rax
   3579c:	mov    0x730(%rsp),%r13
   357a4:	sub    %r13,%rax
   357a7:	jb     35a9c <meshopt_peers::run+0x123c>
   357ad:	mov    0x348(%r15),%rcx
   357b4:	sub    0x358(%r15),%rcx
   357bb:	mov    %rax,0x5e0(%rsp)
   357c3:	mov    %rcx,0x5e8(%rsp)
   357cb:	movb   $0x0,0x600(%rsp)
   357d3:	movb   $0x0,0x605(%rsp)
   357db:	movl   $0x0,0x601(%rsp)
   357e6:	movb   $0xff,0x608(%rsp)
   357ee:	movw   $0x0,0x606(%rsp)
   357f8:	lea    0x5f0(%rsp),%rax
   35800:	xorps  %xmm0,%xmm0
   35803:	movups %xmm0,(%rax)
   35806:	movq   $0xffffffffffffffff,0x318(%rsp)
   35812:	movq   $0xffffffffffffffff,0x520(%rsp)
   3581e:	movq   $0x0,0x2a0(%rsp)
   3582a:	movq   $0x4,0x2a8(%rsp)
   35836:	movups %xmm0,-0x340(%rax)
   3583d:	movq   $0x4,0x2c0(%rsp)
   35849:	movups %xmm0,-0x328(%rax)
   35850:	movq   $0x4,0x2d8(%rsp)
   3585c:	movups %xmm0,-0x310(%rax)
   35863:	movq   $0x1,0x2f0(%rsp)
   3586f:	movups %xmm0,-0x2f8(%rax)
   35876:	movq   $0x2,0x308(%rsp)
   35882:	movq   $0x0,0x310(%rsp)
   3588e:	mov    0x30(%rsp),%rax
   35893:	mov    0x8(%rax,%r12,1),%rdi
   35898:	mov    0x10(%rax,%r12,1),%rdx
   3589d:	mov    %rdx,%rsi
   358a0:	add    %rdi,%rsi
   358a3:	jb     36e3f <meshopt_peers::run+0x25df>
   358a9:	cmp    %r14,%rsi
   358ac:	ja     36e3f <meshopt_peers::run+0x25df>
   358b2:	mov    0x20(%rsp),%rax
   358b7:	lea    (%rax,%rdi,4),%rsi
   358bb:	movb   $0x2,0x128(%rsp)
   358c3:	mov    0x50(%rsp),%rax
   358c8:	mov    %rax,0x130(%rsp)
   358d0:	mov    %rbx,0x138(%rsp)
   358d8:	movq   $0x0,0x160(%rsp)
   358e4:	lea    0xc8(%rsp),%rdi
   358ec:	lea    0x128(%rsp),%rcx
   358f4:	mov    0x220(%rsp),%r8
   358fc:	lea    0x2a0(%rsp),%r9
   35904:	call   *0xf11ae(%rip)        # 126ab8 <_DYNAMIC+0x610>
   3590a:	mov    0x5f8(%rsp),%rsi
   35912:	sub    %rsi,%rbp
   35915:	jb     35a0a <meshopt_peers::run+0x11aa>
   3591b:	mov    %rbp,0x720(%rsp)
   35923:	add    0x5f0(%rsp),%r13
   3592b:	jb     36ec2 <meshopt_peers::run+0x2662>
   35931:	mov    %r15,%rdi
   35934:	mov    %r13,%rsi
   35937:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   3593c:	mov    %eax,%ebp
   3593e:	cmp    $0xff,%al
   35940:	jne    36ec5 <meshopt_peers::run+0x2665>
   35946:	movzbl 0xc8(%rsp),%ebx
   3594e:	movzbl 0xc9(%rsp),%ebp
   35956:	lea    0xca(%rsp),%rcx
   3595e:	mov    (%rcx),%rax
   35961:	mov    0x6(%rcx),%rcx
   35965:	mov    %rax,0x260(%rsp)
   3596d:	mov    %rcx,0x266(%rsp)
   35975:	movss  0xd8(%rsp),%xmm0
   3597e:	movaps %xmm0,0x90(%rsp)
   35986:	lea    0x2a0(%rsp),%rdi
   3598e:	call   339f0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   35993:	cmp    $0x1,%bl
   35996:	mov    0x10(%rsp),%rbx
   3599b:	je     36ed2 <meshopt_peers::run+0x2672>
   359a1:	lea    0x262(%rsp),%rcx
   359a9:	mov    0x8(%rcx),%eax
   359ac:	lea    0x70c(%rsp),%rdx
   359b4:	mov    %eax,0x8(%rdx)
   359b7:	mov    (%rcx),%rax
   359ba:	mov    %rax,(%rdx)
   359bd:	mov    0x8(%rdx),%eax
   359c0:	mov    %eax,0x2a8(%rsp)
   359c7:	mov    (%rdx),%rcx
   359ca:	mov    %rcx,0x2a0(%rsp)
   359d2:	mov    0x30(%rsp),%rdx
   359d7:	mov    %eax,0x24(%rdx,%r12,1)
   359dc:	mov    %rcx,0x1c(%rdx,%r12,1)
   359e1:	xorps  %xmm0,%xmm0
   359e4:	movaps 0x90(%rsp),%xmm1
   359ec:	movss  %xmm1,%xmm0
   359f0:	movlps %xmm0,0x28(%rdx,%r12,1)
   359f6:	add    $0x30,%r12
   359fa:	cmp    %r12,0x18(%rsp)
   359ff:	jne    3576a <meshopt_peers::run+0xf0a>
   35a05:	jmp    35ace <meshopt_peers::run+0x126e>
   35a0a:	lea    0x720(%rsp),%rdi
   35a12:	call   *0xf1058(%rip)        # 126a70 <_DYNAMIC+0x5c8>
   35a18:	mov    %eax,%ebp
   35a1a:	cmp    $0xff,%al
   35a1c:	je     35923 <meshopt_peers::run+0x10c3>
   35a22:	jmp    36ec5 <meshopt_peers::run+0x2665>
   35a27:	mov    %rdi,%r12
   35a2a:	mov    %r8,0x90(%rsp)
   35a32:	lea    0x718(%rsp),%rdi
   35a3a:	mov    0x188(%rsp),%rsi
   35a42:	mov    0xc0(%rsp),%rdx
   35a4a:	call   33360 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   35a4f:	mov    $0x1,%r13b
   35a52:	cmp    $0xff,%al
   35a54:	je     35aa4 <meshopt_peers::run+0x1244>
   35a56:	mov    %eax,%ebp
   35a58:	xor    %r14d,%r14d
   35a5b:	mov    $0x1,%r15b
   35a5e:	jmp    35bd4 <meshopt_peers::run+0x1374>
   35a63:	lea    0x2a0(%rsp),%rdi
   35a6b:	mov    $0x1,%edx
   35a70:	mov    $0x1,%r8d
   35a76:	mov    $0x1,%r9d
   35a7c:	xor    %esi,%esi
   35a7e:	mov    %rbx,%rcx
   35a81:	call   3b590 <<alloc::raw_vec::RawVecInner>::finish_grow>
   35a86:	cmpl   $0x1,0x2a0(%rsp)
   35a8e:	jne    35b75 <meshopt_peers::run+0x1315>
   35a94:	mov    $0xa,%bpl
   35a97:	jmp    35445 <meshopt_peers::run+0xbe5>
   35a9c:	mov    $0x9,%bpl
   35a9f:	jmp    36ed2 <meshopt_peers::run+0x2672>
   35aa4:	mov    0x730(%rsp),%rax
   35aac:	sub    %rbx,%rax
   35aaf:	jae    35b95 <meshopt_peers::run+0x1335>
   35ab5:	mov    0x80(%rsp),%rdi
   35abd:	call   *0xf0bed(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   35ac3:	mov    $0x8,%bpl
   35ac6:	xor    %r14d,%r14d
   35ac9:	jmp    35bd1 <meshopt_peers::run+0x1371>
   35ace:	mov    0x120(%rsp),%rdx
   35ad6:	xor    %r13d,%r13d
   35ad9:	cmp    $0x1,%rdx
   35add:	jne    35be1 <meshopt_peers::run+0x1381>
   35ae3:	mov    0x8(%rsp),%r12
   35ae8:	mov    0x68(%rsp),%rax
   35aed:	mov    %rax,0x90(%rsp)
   35af5:	mov    0x30(%rsp),%rdi
   35afa:	movups 0x1c(%rdi),%xmm0
   35afe:	movaps %xmm0,0x610(%rsp)
   35b06:	movl   $0x7f7fffff,0x620(%rsp)
   35b11:	lea    0x8c(%rsp),%rax
   35b19:	lea    0x1a8(%rsp),%r10
   35b21:	lea    0x610(%rsp),%r11
   35b29:	mov    $0x1,%esi
   35b2e:	mov    0x20(%rsp),%rdx
   35b33:	mov    %r14,%rcx
   35b36:	mov    0x50(%rsp),%r8
   35b3b:	mov    %rbx,%r9
   35b3e:	lea    0x718(%rsp),%r15
   35b46:	push   %r15
   35b48:	push   %rax
   35b49:	push   %r10
   35b4b:	push   %r13
   35b4d:	push   %r11
   35b4f:	push   $0x0
   35b51:	call   33a80 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   35b56:	add    $0x30,%rsp
   35b5a:	mov    %rax,%rbp
   35b5d:	test   $0x1,%bpl
   35b61:	je     35a32 <meshopt_peers::run+0x11d2>
   35b67:	shr    $0x8,%ebp
   35b6a:	mov    $0x1,%r14b
   35b6d:	mov    $0x1,%r15b
   35b70:	mov    $0x1,%r13b
   35b73:	jmp    35bd4 <meshopt_peers::run+0x1374>
   35b75:	mov    0x2a8(%rsp),%rdi
   35b7d:	mov    %rdi,0x80(%rsp)
   35b85:	xor    %esi,%esi
   35b87:	mov    %rbx,%rdx
   35b8a:	call   *0xf0b40(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   35b90:	jmp    3540b <meshopt_peers::run+0xbab>
   35b95:	mov    %rax,0x730(%rsp)
   35b9d:	test   %rbx,%rbx
   35ba0:	je     35bb0 <meshopt_peers::run+0x1350>
   35ba2:	mov    0x80(%rsp),%rdi
   35baa:	call   *0xf0b00(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   35bb0:	xor    %r14d,%r14d
   35bb3:	lea    0x718(%rsp),%rdi
   35bbb:	mov    $0x4,%edx
   35bc0:	xor    %esi,%esi
   35bc2:	call   332c0 <<meshoptimizer_rs::processing::Context>::free::<[f32; 4]>>
   35bc7:	cmp    $0xff,%al
   35bc9:	je     36c91 <meshopt_peers::run+0x2431>
   35bcf:	mov    %eax,%ebp
   35bd1:	xor    %r15d,%r15d
   35bd4:	mov    0x90(%rsp),%rax
   35bdc:	jmp    36ee5 <meshopt_peers::run+0x2685>
   35be1:	lea    0xc8(%rsp),%rdi
   35be9:	mov    0xf0ed0(%rip),%r10        # 126ac0 <_DYNAMIC+0x618>
   35bf0:	mov    %r14,%r8
   35bf3:	lea    0xe9676(%rip),%rax        # 11f270 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xd8>
   35bfa:	mov    %rax,0x178(%rsp)
   35c02:	movb   $0x2,0x128(%rsp)
   35c0a:	mov    0x50(%rsp),%rax
   35c0f:	mov    %rax,0x130(%rsp)
   35c17:	mov    %rbx,0x138(%rsp)
   35c1f:	movq   $0x0,0x160(%rsp)
   35c2b:	movups 0x200(%rsp),%xmm0
   35c33:	movaps %xmm0,0x2e0(%rsp)
   35c3b:	movups 0x1c0(%rsp),%xmm0
   35c43:	movups 0x1d0(%rsp),%xmm1
   35c4b:	movups 0x1e0(%rsp),%xmm2
   35c53:	movups 0x1f0(%rsp),%xmm3
   35c5b:	movaps %xmm3,0x2d0(%rsp)
   35c63:	movaps %xmm2,0x2c0(%rsp)
   35c6b:	movaps %xmm1,0x2b0(%rsp)
   35c73:	movaps %xmm0,0x2a0(%rsp)
   35c7b:	mov    0x30(%rsp),%rsi
   35c80:	mov    %rdx,0x120(%rsp)
   35c88:	mov    0x20(%rsp),%rcx
   35c8d:	mov    %r8,0x18(%rsp)
   35c92:	mov    0xc0(%rsp),%r9
   35c9a:	lea    0x718(%rsp),%r12
   35ca2:	push   %r12
   35ca4:	lea    0x2a8(%rsp),%rax
   35cac:	push   %rax
   35cad:	lea    0x138(%rsp),%rax
   35cb5:	push   %rax
   35cb6:	push   0x240(%rsp)
   35cbd:	call   *%r10
   35cc0:	add    $0x20,%rsp
   35cc4:	mov    %r13,0x180(%rsp)
   35ccc:	mov    0xc8(%rsp),%r15
   35cd4:	movzbl 0xd0(%rsp),%ebp
   35cdc:	cmp    $0xffffffffffffffff,%r15
   35ce0:	je     36ed2 <meshopt_peers::run+0x2672>
   35ce6:	movzbl %bpl,%eax
   35cea:	movzbl 0xd7(%rsp),%ecx
   35cf2:	shl    $0x10,%ecx
   35cf5:	movzwl 0xd5(%rsp),%edx
   35cfd:	or     %ecx,%edx
   35cff:	mov    0xd1(%rsp),%r8d
   35d07:	mov    0xd8(%rsp),%r9
   35d0f:	shl    $0x28,%rdx
   35d13:	shl    $0x8,%r8
   35d17:	or     %rdx,%r8
   35d1a:	or     %rax,%r8
   35d1d:	sub    $0x40,%rsp
   35d21:	mov    %r12,0x30(%rsp)
   35d26:	mov    0x268(%rsp),%rax
   35d2e:	mov    %rax,0x18(%rsp)
   35d33:	mov    0x100(%rsp),%rax
   35d3b:	mov    %rax,0x10(%rsp)
   35d40:	mov    0x58(%rsp),%rax
   35d45:	mov    %rax,0x8(%rsp)
   35d4a:	mov    0x60(%rsp),%rax
   35d4f:	mov    %rax,(%rsp)
   35d53:	movq   $0x0,0x20(%rsp)
   35d5c:	mov    0xc0(%rsp),%rdi
   35d64:	mov    %rbx,%rsi
   35d67:	mov    0x70(%rsp),%rdx
   35d6c:	mov    0x160(%rsp),%rcx
   35d74:	mov    %r8,0x158(%rsp)
   35d7c:	mov    %r9,0x150(%rsp)
   35d84:	call   *0xf0d3e(%rip)        # 126ac8 <_DYNAMIC+0x620>
   35d8a:	add    $0x40,%rsp
   35d8e:	mov    %eax,%ebp
   35d90:	cmp    $0xff,%al
   35d92:	jne    36f18 <meshopt_peers::run+0x26b8>
   35d98:	mov    0x18(%rsp),%rax
   35d9d:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   35da7:	mul    %rcx
   35daa:	mov    %rdx,%r14
   35dad:	shr    $1,%r14
   35db0:	mov    %r14,%rax
   35db3:	mov    $0x30,%ecx
   35db8:	mul    %rcx
   35dbb:	seto   %cl
   35dbe:	test   %rax,%rax
   35dc1:	sets   %dl
   35dc4:	or     %cl,%dl
   35dc6:	mov    $0x8,%bpl
   35dc9:	mov    0x8(%rsp),%r12
   35dce:	jne    36c87 <meshopt_peers::run+0x2427>
   35dd4:	mov    $0x1,%cl
   35dd6:	mov    %ecx,(%rsp)
   35dd9:	add    0x730(%rsp),%rax
   35de1:	jb     36d5c <meshopt_peers::run+0x24fc>
   35de7:	mov    %rax,0x730(%rsp)
   35def:	mov    0x718(%rsp),%r13
   35df7:	mov    %r13,%rdi
   35dfa:	mov    %rax,%r12
   35dfd:	mov    %rax,%rsi
   35e00:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35e05:	mov    %eax,%ebp
   35e07:	cmp    $0xff,%al
   35e09:	jne    36fdc <meshopt_peers::run+0x277c>
   35e0f:	mov    %r15,0x48(%rsp)
   35e14:	mov    0x18(%rsp),%r8
   35e19:	cmp    $0x3,%r8
   35e1d:	jae    36be7 <meshopt_peers::run+0x2387>
   35e23:	mov    $0x8,%r15d
   35e29:	xor    %eax,%eax
   35e2b:	mov    %rax,0x60(%rsp)
   35e30:	sub    %r14,%rax
   35e33:	mov    $0x30,%ecx
   35e38:	mul    %rcx
   35e3b:	seto   %cl
   35e3e:	test   %rax,%rax
   35e41:	sets   %dl
   35e44:	or     %cl,%dl
   35e46:	mov    $0x8,%bpl
   35e49:	jne    36d41 <meshopt_peers::run+0x24e1>
   35e4f:	test   %rax,%rax
   35e52:	je     35e7f <meshopt_peers::run+0x161f>
   35e54:	add    %r12,%rax
   35e57:	jb     36d41 <meshopt_peers::run+0x24e1>
   35e5d:	mov    %rax,0x730(%rsp)
   35e65:	mov    %r13,%rdi
   35e68:	mov    %rax,%rsi
   35e6b:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   35e70:	mov    %eax,%ebp
   35e72:	cmp    $0xff,%al
   35e74:	mov    0x18(%rsp),%r8
   35e79:	jne    36d41 <meshopt_peers::run+0x24e1>
   35e7f:	mov    %r15,%r13
   35e82:	shr    $0x8,%r13
   35e86:	mov    %r15,%rax
   35e89:	shr    $0x28,%rax
   35e8d:	mov    %r15,%rcx
   35e90:	shr    $0x38,%rcx
   35e94:	mov    %cl,0x137(%rsp)
   35e9b:	mov    %ax,0x135(%rsp)
   35ea3:	mov    %r13d,0x131(%rsp)
   35eab:	movq   $0x0,0x138(%rsp)
   35eb7:	mov    0x60(%rsp),%rax
   35ebc:	mov    %rax,0x128(%rsp)
   35ec4:	mov    %r15b,0x130(%rsp)
   35ecc:	cmp    $0x2,%r8
   35ed0:	jbe    35f56 <meshopt_peers::run+0x16f6>
   35ed6:	mov    %r14,%rbx
   35ed9:	cmp    %r14,%rax
   35edc:	jb     36c38 <meshopt_peers::run+0x23d8>
   35ee2:	xor    %r12d,%r12d
   35ee5:	mov    0x130(%rsp),%r15
   35eed:	lea    (%r12,%r12,2),%rax
   35ef1:	shl    $0x4,%rax
   35ef5:	xorps  %xmm0,%xmm0
   35ef8:	movups %xmm0,0x20(%r15,%rax,1)
   35efe:	movups %xmm0,0x10(%r15,%rax,1)
   35f04:	movups %xmm0,(%r15,%rax,1)
   35f09:	cmpq   $0x6,0x18(%rsp)
   35f0f:	jb     35f2d <meshopt_peers::run+0x16cd>
   35f11:	lea    (%r15,%rax,1),%rdi
   35f15:	add    $0x30,%rdi
   35f19:	lea    (%rbx,%rbx,2),%rdx
   35f1d:	shl    $0x4,%rdx
   35f21:	add    $0xffffffffffffffd0,%rdx
   35f25:	xor    %esi,%esi
   35f27:	call   *0xf07a3(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   35f2d:	mov    0x128(%rsp),%rax
   35f35:	mov    %rax,0x60(%rsp)
   35f3a:	cmp    $0xffffffffffffffff,%rax
   35f3e:	je     36ffc <meshopt_peers::run+0x279c>
   35f44:	mov    %r15,%r13
   35f47:	shr    $0x8,%r13
   35f4b:	mov    %rbx,%r14
   35f4e:	add    %r12,%r14
   35f51:	mov    0x18(%rsp),%r8
   35f56:	movzbl %r15b,%eax
   35f5a:	shl    $0x8,%r13
   35f5e:	or     %rax,%r13
   35f61:	lea    0x2a0(%rsp),%rdi
   35f69:	lea    0x718(%rsp),%rbx
   35f71:	mov    %rbx,%rsi
   35f74:	mov    %r8,%rdx
   35f77:	mov    %r13,0x108(%rsp)
   35f7f:	call   333b0 <<meshoptimizer_rs::processing::Context>::alloc::<u32>>
   35f84:	mov    0x2a0(%rsp),%rax
   35f8c:	movzbl 0x2a8(%rsp),%ebp
   35f94:	mov    %rax,0x90(%rsp)
   35f9c:	cmp    $0xffffffffffffffff,%rax
   35fa0:	je     36fe6 <meshopt_peers::run+0x2786>
   35fa6:	movzbl %bpl,%eax
   35faa:	movzbl 0x2af(%rsp),%ecx
   35fb2:	shl    $0x10,%ecx
   35fb5:	movzwl 0x2ad(%rsp),%edx
   35fbd:	or     %ecx,%edx
   35fbf:	mov    0x2a9(%rsp),%ecx
   35fc6:	mov    0x2b0(%rsp),%rsi
   35fce:	mov    %rsi,0x170(%rsp)
   35fd6:	shl    $0x28,%rdx
   35fda:	shl    $0x8,%rcx
   35fde:	or     %rdx,%rcx
   35fe1:	or     %rax,%rcx
   35fe4:	mov    %rcx,0xb8(%rsp)
   35fec:	cmpq   $0x2,0x110(%rsp)
   35ff5:	jb     36b05 <meshopt_peers::run+0x22a5>
   35ffb:	mov    0x30(%rsp),%rax
   36000:	lea    0xa0(%rax),%rcx
   36007:	mov    %rcx,0x248(%rsp)
   3600f:	lea    0x10(%rax),%rcx
   36013:	mov    %rcx,0x250(%rsp)
   3601b:	add    $0x1c,%rax
   3601f:	mov    %rax,0x218(%rsp)
   36027:	lea    0x10(%r13),%rax
   3602b:	mov    %rax,0x240(%rsp)
   36033:	xor    %r12d,%r12d
   36036:	xor    %r15d,%r15d
   36039:	mov    0x118(%rsp),%rax
   36041:	mov    %rax,0x210(%rsp)
   36049:	mov    %r14,0x100(%rsp)
   36051:	mov    %r15,0xa8(%rsp)
   36059:	mov    %r12,0xb0(%rsp)
   36061:	mov    0x210(%rsp),%rax
   36069:	mov    (%rax),%r14d
   3606c:	mov    0x4(%rax),%esi
   3606f:	cmp    %r14d,%esi
   36072:	mov    0x120(%rsp),%rdx
   3607a:	jb     371c5 <meshopt_peers::run+0x2965>
   36080:	cmp    %rsi,%rdx
   36083:	jb     371c5 <meshopt_peers::run+0x2965>
   36089:	mov    %r14,%rax
   3608c:	shl    $0x4,%rax
   36090:	lea    (%rax,%rax,2),%rax
   36094:	mov    %rax,0x38(%rsp)
   36099:	mov    %rsi,%rax
   3609c:	shl    $0x4,%rax
   360a0:	lea    (%rax,%rax,2),%r13
   360a4:	mov    %rsi,%rbx
   360a7:	cmp    %esi,%r14d
   360aa:	jne    360b3 <meshopt_peers::run+0x1853>
   360ac:	xor    %edx,%edx
   360ae:	jmp    36153 <meshopt_peers::run+0x18f3>
   360b3:	mov    %r13,%rcx
   360b6:	sub    0x38(%rsp),%rcx
   360bb:	mov    %rcx,%rax
   360be:	movabs $0x555555555555556,%rdx
   360c8:	mul    %rdx
   360cb:	mov    %rdx,%rax
   360ce:	mov    %eax,%esi
   360d0:	and    $0x3,%esi
   360d3:	cmp    $0xc0,%rcx
   360da:	jae    360e2 <meshopt_peers::run+0x1882>
   360dc:	xor    %ecx,%ecx
   360de:	xor    %edx,%edx
   360e0:	jmp    36122 <meshopt_peers::run+0x18c2>
   360e2:	and    $0xfffffffffffffffc,%rax
   360e6:	mov    0x248(%rsp),%rcx
   360ee:	mov    0x38(%rsp),%rdx
   360f3:	lea    (%rcx,%rdx,1),%rdi
   360f7:	xor    %ecx,%ecx
   360f9:	xor    %edx,%edx
   360fb:	add    -0x90(%rdi),%rdx
   36102:	add    -0x60(%rdi),%rdx
   36106:	add    -0x30(%rdi),%rdx
   3610a:	add    (%rdi),%rdx
   3610d:	add    $0x4,%rcx
   36111:	add    $0xc0,%rdi
   36118:	cmp    %rcx,%rax
   3611b:	jne    360fb <meshopt_peers::run+0x189b>
   3611d:	test   %rsi,%rsi
   36120:	je     36153 <meshopt_peers::run+0x18f3>
   36122:	lea    (%rcx,%rcx,2),%rax
   36126:	shl    $0x4,%rax
   3612a:	mov    0x250(%rsp),%rcx
   36132:	mov    0x38(%rsp),%rdi
   36137:	add    %rdi,%rcx
   3613a:	add    %rcx,%rax
   3613d:	shl    $0x4,%esi
   36140:	lea    (%rsi,%rsi,2),%rcx
   36144:	xor    %esi,%esi
   36146:	add    (%rax,%rsi,1),%rdx
   3614a:	add    $0x30,%rsi
   3614e:	cmp    %rsi,%rcx
   36151:	jne    36146 <meshopt_peers::run+0x18e6>
   36153:	lea    0x2a0(%rsp),%rdi
   3615b:	lea    0x718(%rsp),%rsi
   36163:	call   333b0 <<meshoptimizer_rs::processing::Context>::alloc::<u32>>
   36168:	mov    0x2a0(%rsp),%rax
   36170:	movzbl 0x2a8(%rsp),%ebp
   36178:	mov    %rax,0x58(%rsp)
   3617d:	cmp    $0xffffffffffffffff,%rax
   36181:	je     36f85 <meshopt_peers::run+0x2725>
   36187:	mov    %rbx,%rsi
   3618a:	sub    %r14,%rsi
   3618d:	movzbl %bpl,%eax
   36191:	movzbl 0x2af(%rsp),%ecx
   36199:	shl    $0x10,%ecx
   3619c:	movzwl 0x2ad(%rsp),%edx
   361a4:	or     %ecx,%edx
   361a6:	mov    0x2a9(%rsp),%ecx
   361ad:	shl    $0x28,%rdx
   361b1:	shl    $0x8,%rcx
   361b5:	or     %rdx,%rcx
   361b8:	or     %rax,%rcx
   361bb:	mov    %rcx,0x78(%rsp)
   361c0:	mov    %rsi,0x70(%rsp)
   361c5:	lea    0x0(,%rsi,4),%rax
   361cd:	lea    (%rax,%rax,4),%r15
   361d1:	mov    0x730(%rsp),%rsi
   361d9:	add    %r15,%rsi
   361dc:	jb     371fd <meshopt_peers::run+0x299d>
   361e2:	mov    0x2b0(%rsp),%rax
   361ea:	mov    %rax,(%rsp)
   361ee:	mov    %rsi,0x730(%rsp)
   361f6:	mov    0x718(%rsp),%rdi
   361fe:	call   3c000 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   36203:	mov    %eax,%ebp
   36205:	cmp    $0xff,%al
   36207:	jne    36e20 <meshopt_peers::run+0x25c0>
   3620d:	cmpq   $0x0,0x70(%rsp)
   36213:	jne    365ee <meshopt_peers::run+0x1d8e>
   36219:	mov    $0x4,%eax
   3621e:	mov    %rax,0x40(%rsp)
   36223:	movq   $0x0,0x28(%rsp)
   3622c:	mov    %rbx,%r8
   3622f:	cmp    %r8d,%r14d
   36232:	mov    0xf0487(%rip),%rbx        # 1266c0 <memcpy@GLIBC_2.14>
   36239:	mov    0x18(%rsp),%rsi
   3623e:	mov    0x218(%rsp),%rcx
   36246:	je     362ec <meshopt_peers::run+0x1a8c>
   3624c:	sub    %r14,%r8
   3624f:	inc    %r8
   36252:	mov    0x38(%rsp),%r12
   36257:	mov    0x40(%rsp),%rbp
   3625c:	xor    %edi,%edi
   3625e:	mov    -0xc(%rcx,%r12,1),%rdx
   36263:	mov    %rdx,%r15
   36266:	add    %rdi,%r15
   36269:	jb     37106 <meshopt_peers::run+0x28a6>
   3626f:	cmp    (%rsp),%r15
   36273:	ja     37106 <meshopt_peers::run+0x28a6>
   36279:	mov    -0x14(%rcx,%r12,1),%rax
   3627e:	mov    %rax,%rcx
   36281:	add    %rdx,%rcx
   36284:	jb     370dc <meshopt_peers::run+0x287c>
   3628a:	cmp    %rsi,%rcx
   3628d:	ja     370e8 <meshopt_peers::run+0x2888>
   36293:	mov    %r8,%r14
   36296:	mov    0x78(%rsp),%rcx
   3629b:	lea    (%rcx,%rdi,4),%rdi
   3629f:	mov    0x20(%rsp),%rcx
   362a4:	lea    (%rcx,%rax,4),%rsi
   362a8:	shl    $0x2,%rdx
   362ac:	call   *%rbx
   362ae:	mov    %r14,%r8
   362b1:	dec    %r8
   362b4:	je     3729e <meshopt_peers::run+0x2a3e>
   362ba:	mov    0x218(%rsp),%rcx
   362c2:	movups (%rcx,%r12,1),%xmm0
   362c7:	mov    0x10(%rcx,%r12,1),%eax
   362cc:	mov    %eax,0x10(%rbp)
   362cf:	movups %xmm0,0x0(%rbp)
   362d3:	add    $0x14,%rbp
   362d7:	add    $0x30,%r12
   362db:	mov    %r15,%rdi
   362de:	cmp    %r12,%r13
   362e1:	mov    0x18(%rsp),%rsi
   362e6:	jne    3625e <meshopt_peers::run+0x19fe>
   362ec:	mov    (%rsp),%rax
   362f0:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   362fa:	mul    %rcx
   362fd:	shr    $1,%rdx
   36300:	xorps  %xmm0,%xmm0
   36303:	cvtsi2ss %rdx,%xmm0
   36308:	mulss  -0x267d4(%rip),%xmm0        # fb3c <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa0>
   36310:	cvttss2si %xmm0,%rax
   36315:	mov    %rax,%rcx
   36318:	sar    $0x3f,%rcx
   3631c:	movaps %xmm0,%xmm1
   3631f:	subss  -0x267e3(%rip),%xmm1        # fb44 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xa8>
   36327:	cvttss2si %xmm1,%rdx
   3632c:	and    %rcx,%rdx
   3632f:	or     %rax,%rdx
   36332:	xorps  %xmm1,%xmm1
   36335:	ucomiss %xmm1,%xmm0
   36338:	mov    $0x0,%r15d
   3633e:	cmovae %rdx,%r15
   36342:	ucomiss -0x26801(%rip),%xmm0        # fb48 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xac>
   36349:	mov    $0xffffffffffffffff,%rax
   36350:	cmova  %rax,%r15
   36354:	lea    0x2a0(%rsp),%rdi
   3635c:	mov    0x40(%rsp),%rsi
   36361:	mov    0x70(%rsp),%rdx
   36366:	lea    0x718(%rsp),%rcx
   3636e:	call   *0xf075c(%rip)        # 126ad0 <_DYNAMIC+0x628>
   36374:	cmpb   $0x1,0x2a0(%rsp)
   3637c:	je     37031 <meshopt_peers::run+0x27d1>
   36382:	lea    (%r15,%r15,2),%rax
   36386:	lea    0x2a4(%rsp),%rdx
   3638e:	mov    0x10(%rdx),%ecx
   36391:	lea    0x12a(%rsp),%rsi
   36399:	mov    %ecx,0x10(%rsi)
   3639c:	movups (%rdx),%xmm0
   3639f:	movups %xmm0,(%rsi)
   363a2:	movups (%rsi),%xmm0
   363a5:	movaps %xmm0,0x190(%rsp)
   363ad:	mov    0x10(%rsi),%ecx
   363b0:	mov    %ecx,0x1a0(%rsp)
   363b7:	sub    $0x8,%rsp
   363bb:	lea    0x2a8(%rsp),%rdi
   363c3:	mov    0x80(%rsp),%rsi
   363cb:	mov    0x8(%rsp),%rdx
   363d0:	lea    0x638(%rsp),%rcx
   363d8:	mov    0x88(%rsp),%r8
   363e0:	mov    0x18(%rsp),%r9
   363e5:	lea    0x720(%rsp),%r10
   363ed:	push   %r10
   363ef:	push   %rax
   363f0:	lea    0x1d8(%rsp),%rax
   363f8:	push   %rax
   363f9:	call   *0xf06d9(%rip)        # 126ad8 <_DYNAMIC+0x630>
   363ff:	add    $0x20,%rsp
   36403:	mov    0x2a0(%rsp),%rax
   3640b:	movzbl 0x2a8(%rsp),%ebp
   36413:	cmp    $0xffffffffffffffff,%rax
   36417:	je     37039 <meshopt_peers::run+0x27d9>
   3641d:	mov    %rax,%r12
   36420:	decq   0x110(%rsp)
   36428:	addq   $0x4,0x210(%rsp)
   36431:	mov    0x38(%rsp),%rax
   36436:	add    0x30(%rsp),%rax
   3643b:	mov    %rax,0x38(%rsp)
   36440:	movzbl %bpl,%eax
   36444:	movzbl 0x2af(%rsp),%ecx
   3644c:	shl    $0x10,%ecx
   3644f:	movzwl 0x2ad(%rsp),%edx
   36457:	or     %ecx,%edx
   36459:	mov    0x2a9(%rsp),%r13d
   36461:	mov    0x2b0(%rsp),%r15
   36469:	shl    $0x28,%rdx
   3646d:	shl    $0x8,%r13
   36471:	xorps  %xmm0,%xmm0
   36474:	cvtsi2ss %r15,%xmm0
   36479:	or     %rdx,%r13
   3647c:	cvtsi2ssq (%rsp),%xmm1
   36482:	or     %rax,%r13
   36485:	mulss  -0x2695d(%rip),%xmm1        # fb30 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x94>
   3648d:	ucomiss %xmm1,%xmm0
   36490:	jbe    36677 <meshopt_peers::run+0x1e17>
   36496:	movl   $0x7f7fffff,0x1a0(%rsp)
   364a1:	mov    0x38(%rsp),%rdi
   364a6:	mov    0x70(%rsp),%rsi
   364ab:	mov    0x20(%rsp),%rdx
   364b0:	mov    0x18(%rsp),%rcx
   364b5:	mov    0x50(%rsp),%r8
   364ba:	mov    0x10(%rsp),%r9
   364bf:	lea    0x718(%rsp),%rax
   364c7:	push   %rax
   364c8:	lea    0x94(%rsp),%rax
   364d0:	push   %rax
   364d1:	lea    0x1b8(%rsp),%rax
   364d9:	push   %rax
   364da:	push   0x198(%rsp)
   364e1:	lea    0x1b0(%rsp),%rax
   364e9:	push   %rax
   364ea:	push   $0x0
   364ec:	call   33a80 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   364f1:	add    $0x30,%rsp
   364f5:	mov    %rax,%rbp
   364f8:	test   $0x1,%bpl
   364fc:	jne    37091 <meshopt_peers::run+0x2831>
   36502:	mov    0x730(%rsp),%rax
   3650a:	mov    0x58(%rsp),%rdx
   3650f:	lea    0x0(,%rdx,4),%rcx
   36517:	shr    $0x3e,%rdx
   3651b:	sete   %dl
   3651e:	test   %rcx,%rcx
   36521:	setns  %sil
   36525:	and    %dl,%sil
   36528:	sub    %rcx,%rax
   3652b:	setae  %cl
   3652e:	test   %cl,%sil
   36531:	je     36dcb <meshopt_peers::run+0x256b>
   36537:	mov    %rax,0x730(%rsp)
   3653f:	cmpq   $0x0,0x58(%rsp)
   36545:	je     3655a <meshopt_peers::run+0x1cfa>
   36547:	mov    0x78(%rsp),%rdi
   3654c:	call   *0xf015e(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36552:	mov    0x730(%rsp),%rax
   3655a:	mov    0x28(%rsp),%rcx
   3655f:	lea    0x0(,%rcx,4),%rcx
   36567:	lea    (%rcx,%rcx,4),%rcx
   3656b:	sub    %rcx,%rax
   3656e:	jb     36f61 <meshopt_peers::run+0x2701>
   36574:	mov    %rax,0x730(%rsp)
   3657c:	cmpq   $0x0,0x28(%rsp)
   36582:	je     36597 <meshopt_peers::run+0x1d37>
   36584:	mov    0x40(%rsp),%rdi
   36589:	call   *0xf0121(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   3658f:	mov    0x730(%rsp),%rax
   36597:	movabs $0x3fffffffffffffff,%rcx
   365a1:	cmp    %rcx,%r12
   365a4:	ja     36f74 <meshopt_peers::run+0x2714>
   365aa:	lea    0x0(,%r12,4),%rcx
   365b2:	test   %rcx,%rcx
   365b5:	js     36f74 <meshopt_peers::run+0x2714>
   365bb:	sub    %rcx,%rax
   365be:	jb     36f74 <meshopt_peers::run+0x2714>
   365c4:	mov    %rax,0x730(%rsp)
   365cc:	test   %r12,%r12
   365cf:	je     365da <meshopt_peers::run+0x1d7a>
   365d1:	mov    %r13,%rdi
   365d4:	call   *0xf00d6(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   365da:	cmpq   $0x1,0x110(%rsp)
   365e3:	ja     36061 <meshopt_peers::run+0x1801>
   365e9:	jmp    36c62 <meshopt_peers::run+0x2402>
   365ee:	mov    $0x4,%edx
   365f3:	mov    $0x4,%r8d
   365f9:	mov    $0x14,%r9d
   365ff:	lea    0x2a0(%rsp),%rdi
   36607:	xor    %esi,%esi
   36609:	mov    0x70(%rsp),%rcx
   3660e:	call   3b590 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36613:	cmpl   $0x1,0x2a0(%rsp)
   3661b:	je     3715c <meshopt_peers::run+0x28fc>
   36621:	mov    0x2a8(%rsp),%rax
   36629:	xorps  %xmm0,%xmm0
   3662c:	movups %xmm0,(%rax)
   3662f:	mov    %rax,0x40(%rsp)
   36634:	movl   $0x0,0x10(%rax)
   3663b:	mov    $0x1,%eax
   36640:	mov    %rax,0x28(%rsp)
   36645:	cmpl   $0x1,0x70(%rsp)
   3664a:	je     3622c <meshopt_peers::run+0x19cc>
   36650:	mov    0x40(%rsp),%rdi
   36655:	add    $0x14,%rdi
   36659:	add    $0xffffffffffffffec,%r15
   3665d:	xor    %esi,%esi
   3665f:	mov    %r15,%rdx
   36662:	call   *0xf0068(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   36668:	mov    0x70(%rsp),%rax
   3666d:	mov    %rax,0x28(%rsp)
   36672:	jmp    3622c <meshopt_peers::run+0x19cc>
   36677:	movss  0x2b8(%rsp),%xmm0
   36680:	movss  0x1a0(%rsp),%xmm1
   36689:	movaps %xmm1,%xmm2
   3668c:	maxss  %xmm0,%xmm2
   36690:	movaps %xmm2,%xmm3
   36693:	cmpunordss %xmm2,%xmm3
   36698:	movaps %xmm3,%xmm4
   3669b:	andnps %xmm2,%xmm4
   3669e:	andps  %xmm1,%xmm3
   366a1:	orps   %xmm4,%xmm3
   366a4:	xorps  %xmm1,%xmm1
   366a7:	mulss  %xmm1,%xmm0
   366ab:	addss  %xmm3,%xmm0
   366af:	movd   %xmm0,%eax
   366b3:	and    $0x7fffffff,%eax
   366b8:	mov    $0x1,%bl
   366ba:	cmp    $0x7f7fffff,%eax
   366bf:	jg     371da <meshopt_peers::run+0x297a>
   366c5:	movss  %xmm0,0x1a0(%rsp)
   366ce:	mov    0x38(%rsp),%rdi
   366d3:	mov    0x70(%rsp),%rsi
   366d8:	mov    0x20(%rsp),%rdx
   366dd:	mov    0x18(%rsp),%rcx
   366e2:	mov    0x50(%rsp),%r8
   366e7:	mov    0x10(%rsp),%r9
   366ec:	lea    0x718(%rsp),%rax
   366f4:	push   %rax
   366f5:	lea    0x94(%rsp),%rax
   366fd:	push   %rax
   366fe:	lea    0x1b8(%rsp),%rax
   36706:	push   %rax
   36707:	push   0x198(%rsp)
   3670e:	lea    0x1b0(%rsp),%rax
   36716:	push   %rax
   36717:	push   $0x0
   36719:	call   33a80 <meshoptimizer_rs::clusterlod::output::<meshoptimizer_rs::clusterlod::build_with_output<meshopt_peers::clod::{closure#0}>::{closure#0}>>
   3671e:	add    $0x30,%rsp
   36722:	mov    %rax,%rbp
   36725:	test   $0x1,%bpl
   36729:	jne    371e2 <meshopt_peers::run+0x2982>
   3672f:	shr    $0x20,%rbp
   36733:	movb   $0x2,0x128(%rsp)
   3673b:	mov    0x50(%rsp),%rax
   36740:	mov    %rax,0x130(%rsp)
   36748:	mov    0x10(%rsp),%rax
   3674d:	mov    %rax,0x138(%rsp)
   36755:	movq   $0x0,0x160(%rsp)
   36761:	movups 0x200(%rsp),%xmm0
   36769:	movaps %xmm0,0x2e0(%rsp)
   36771:	movups 0x1c0(%rsp),%xmm0
   36779:	movups 0x1d0(%rsp),%xmm1
   36781:	movups 0x1e0(%rsp),%xmm2
   36789:	movups 0x1f0(%rsp),%xmm3
   36791:	movaps %xmm3,0x2d0(%rsp)
   36799:	movaps %xmm2,0x2c0(%rsp)
   367a1:	movaps %xmm1,0x2b0(%rsp)
   367a9:	movaps %xmm0,0x2a0(%rsp)
   367b1:	lea    0xc8(%rsp),%rdi
   367b9:	mov    %r13,%rsi
   367bc:	mov    %r15,%rdx
   367bf:	lea    0x128(%rsp),%rcx
   367c7:	lea    0x2a0(%rsp),%r8
   367cf:	mov    %ebp,%r9d
   367d2:	lea    0x718(%rsp),%rax
   367da:	push   %rax
   367db:	push   0x228(%rsp)
   367e2:	call   *0xf02c8(%rip)        # 126ab0 <_DYNAMIC+0x608>
   367e8:	add    $0x10,%rsp
   367ec:	mov    0xc8(%rsp),%r15
   367f4:	movzbl 0xd0(%rsp),%ebp
   367fc:	cmp    $0xffffffffffffffff,%r15
   36800:	je     370b9 <meshopt_peers::run+0x2859>
   36806:	movzbl %bpl,%ecx
   3680a:	movzbl 0xd7(%rsp),%eax
   36812:	shl    $0x10,%eax
   36815:	movzwl 0xd5(%rsp),%esi
   3681d:	or     %eax,%esi
   3681f:	mov    0xd1(%rsp),%ebp
   36826:	mov    0xd8(%rsp),%rax
   3682e:	mov    0xe0(%rsp),%rdx
   36836:	mov    %rdx,(%rsp)
   3683a:	mov    0xe8(%rsp),%r14
   36842:	mov    0xf0(%rsp),%rdx
   3684a:	shl    $0x28,%rsi
   3684e:	shl    $0x8,%rbp
   36852:	or     %rsi,%rbp
   36855:	or     %rcx,%rbp
   36858:	test   %rax,%rax
   3685b:	je     3691b <meshopt_peers::run+0x20bb>
   36861:	lea    (%rax,%rax,2),%rax
   36865:	shl    $0x4,%rax
   36869:	add    %rbp,%rax
   3686c:	mov    0xa8(%rsp),%rcx
   36874:	lea    (%rcx,%rcx,2),%rcx
   36878:	shl    $0x4,%rcx
   3687c:	add    0x240(%rsp),%rcx
   36884:	xor    %esi,%esi
   36886:	mov    0x0(%rbp,%rsi,1),%rdi
   3688b:	mov    0x8(%rbp,%rsi,1),%r8
   36890:	movups 0x10(%rbp,%rsi,1),%xmm0
   36895:	movaps %xmm0,0x270(%rsp)
   3689d:	mov    0x1a0(%rsp),%r9d
   368a5:	lea    0x27c(%rsp),%r10
   368ad:	mov    %r9d,0x10(%r10)
   368b1:	movaps 0x190(%rsp),%xmm0
   368b9:	movups %xmm0,(%r10)
   368bd:	mov    0xa8(%rsp),%r9
   368c5:	cmp    0x100(%rsp),%r9
   368cd:	jae    37321 <meshopt_peers::run+0x2ac1>
   368d3:	lea    (%rsi,%rbp,1),%r9
   368d7:	add    0xb0(%rsp),%r8
   368df:	mov    %rdi,-0x10(%rcx,%rsi,1)
   368e4:	mov    %r8,-0x8(%rcx,%rsi,1)
   368e9:	movaps 0x270(%rsp),%xmm0
   368f1:	movaps 0x280(%rsp),%xmm1
   368f9:	movups %xmm0,(%rcx,%rsi,1)
   368fd:	movups %xmm1,0x10(%rcx,%rsi,1)
   36902:	incq   0xa8(%rsp)
   3690a:	add    $0x30,%rsi
   3690e:	add    $0x30,%r9
   36912:	cmp    %rax,%r9
   36915:	jne    36886 <meshopt_peers::run+0x2026>
   3691b:	mov    %rdx,%rax
   3691e:	add    0xb0(%rsp),%rax
   36926:	mov    %rax,0x38(%rsp)
   3692b:	jb     37277 <meshopt_peers::run+0x2a17>
   36931:	mov    0x38(%rsp),%rax
   36936:	cmp    0x170(%rsp),%rax
   3693e:	ja     37277 <meshopt_peers::run+0x2a17>
   36944:	mov    0xb8(%rsp),%rax
   3694c:	mov    0xb0(%rsp),%rcx
   36954:	lea    (%rax,%rcx,4),%rdi
   36958:	shl    $0x2,%rdx
   3695c:	mov    %r14,%rsi
   3695f:	call   *0xefd5b(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   36965:	mov    0x730(%rsp),%rcx
   3696d:	mov    %r15,%rax
   36970:	mov    $0x30,%edx
   36975:	mul    %rdx
   36978:	seto   %dl
   3697b:	not    %dl
   3697d:	test   %rax,%rax
   36980:	setns  %sil
   36984:	and    %dl,%sil
   36987:	sub    %rax,%rcx
   3698a:	setae  %al
   3698d:	test   %al,%sil
   36990:	je     37098 <meshopt_peers::run+0x2838>
   36996:	mov    %rcx,0x730(%rsp)
   3699e:	test   %r15,%r15
   369a1:	je     369b4 <meshopt_peers::run+0x2154>
   369a3:	mov    %rbp,%rdi
   369a6:	call   *0xefd04(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   369ac:	mov    0x730(%rsp),%rcx
   369b4:	mov    (%rsp),%rax
   369b8:	shr    $0x3e,%rax
   369bc:	jne    37120 <meshopt_peers::run+0x28c0>
   369c2:	mov    (%rsp),%rax
   369c6:	lea    0x0(,%rax,4),%rax
   369ce:	test   %rax,%rax
   369d1:	js     37120 <meshopt_peers::run+0x28c0>
   369d7:	sub    %rax,%rcx
   369da:	jb     37120 <meshopt_peers::run+0x28c0>
   369e0:	mov    %rcx,0x730(%rsp)
   369e8:	cmpq   $0x0,(%rsp)
   369ed:	je     36a00 <meshopt_peers::run+0x21a0>
   369ef:	mov    %r14,%rdi
   369f2:	call   *0xefcb8(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   369f8:	mov    0x730(%rsp),%rcx
   36a00:	movabs $0x3fffffffffffffff,%rax
   36a0a:	cmp    %rax,0x58(%rsp)
   36a0f:	ja     371b1 <meshopt_peers::run+0x2951>
   36a15:	mov    0x58(%rsp),%rax
   36a1a:	lea    0x0(,%rax,4),%rax
   36a22:	test   %rax,%rax
   36a25:	js     371b1 <meshopt_peers::run+0x2951>
   36a2b:	sub    %rax,%rcx
   36a2e:	jb     371b1 <meshopt_peers::run+0x2951>
   36a34:	mov    %rcx,0x730(%rsp)
   36a3c:	cmpq   $0x0,0x58(%rsp)
   36a42:	je     36a57 <meshopt_peers::run+0x21f7>
   36a44:	mov    0x78(%rsp),%rdi
   36a49:	call   *0xefc61(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36a4f:	mov    0x730(%rsp),%rcx
   36a57:	mov    0x28(%rsp),%rax
   36a5c:	lea    0x0(,%rax,4),%rax
   36a64:	lea    (%rax,%rax,4),%rax
   36a68:	sub    %rax,%rcx
   36a6b:	jb     37214 <meshopt_peers::run+0x29b4>
   36a71:	mov    %rcx,0x730(%rsp)
   36a79:	cmpq   $0x0,0x28(%rsp)
   36a7f:	je     36a94 <meshopt_peers::run+0x2234>
   36a81:	mov    0x40(%rsp),%rdi
   36a86:	call   *0xefc24(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36a8c:	mov    0x730(%rsp),%rcx
   36a94:	movabs $0x3fffffffffffffff,%rax
   36a9e:	cmp    %rax,%r12
   36aa1:	ja     37260 <meshopt_peers::run+0x2a00>
   36aa7:	lea    0x0(,%r12,4),%rax
   36aaf:	test   %rax,%rax
   36ab2:	js     37260 <meshopt_peers::run+0x2a00>
   36ab8:	sub    %rax,%rcx
   36abb:	jb     37260 <meshopt_peers::run+0x2a00>
   36ac1:	mov    %rcx,0x730(%rsp)
   36ac9:	test   %r12,%r12
   36acc:	je     36ad7 <meshopt_peers::run+0x2277>
   36ace:	mov    %r13,%rdi
   36ad1:	call   *0xefbd9(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36ad7:	cmpq   $0x1,0x110(%rsp)
   36ae0:	lea    0x718(%rsp),%rbx
   36ae8:	mov    0x100(%rsp),%r14
   36af0:	mov    0xa8(%rsp),%r15
   36af8:	mov    0x38(%rsp),%r12
   36afd:	ja     36051 <meshopt_peers::run+0x17f1>
   36b03:	jmp    36b0b <meshopt_peers::run+0x22ab>
   36b05:	xor    %r15d,%r15d
   36b08:	xor    %r12d,%r12d
   36b0b:	mov    %rbx,%rdi
   36b0e:	mov    0x48(%rsp),%rsi
   36b13:	mov    0x118(%rsp),%rdx
   36b1b:	call   33360 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36b20:	mov    %eax,%ebp
   36b22:	mov    $0x1,%al
   36b24:	mov    %eax,(%rsp)
   36b27:	cmp    $0xff,%bpl
   36b2b:	setne  %r13b
   36b2f:	jne    37018 <meshopt_peers::run+0x27b8>
   36b35:	cmp    %r14,%r15
   36b38:	cmovb  %r15,%r14
   36b3c:	mov    %r14,%r15
   36b3f:	mov    0x170(%rsp),%r14
   36b47:	cmp    %r14,%r12
   36b4a:	cmovb  %r12,%r14
   36b4e:	mov    %rbx,%rdi
   36b51:	mov    0x8(%rsp),%rsi
   36b56:	mov    0x30(%rsp),%rdx
   36b5b:	call   33310 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36b60:	cmp    $0xff,%al
   36b62:	jne    37016 <meshopt_peers::run+0x27b6>
   36b68:	mov    %rbx,%rdi
   36b6b:	mov    0x68(%rsp),%rsi
   36b70:	mov    0x20(%rsp),%rdx
   36b75:	call   33360 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36b7a:	cmp    $0xff,%al
   36b7c:	jne    37020 <meshopt_peers::run+0x27c0>
   36b82:	mov    0x180(%rsp),%r13
   36b8a:	inc    %r13d
   36b8d:	mov    0x60(%rsp),%rax
   36b92:	mov    %rax,0x8(%rsp)
   36b97:	mov    %r15,%rdx
   36b9a:	mov    0x108(%rsp),%rax
   36ba2:	mov    %rax,0x30(%rsp)
   36ba7:	mov    %r14,%r8
   36baa:	mov    0xb8(%rsp),%rax
   36bb2:	mov    %rax,0x20(%rsp)
   36bb7:	mov    0x90(%rsp),%rax
   36bbf:	mov    %rax,0x68(%rsp)
   36bc4:	cmp    $0x1,%r15
   36bc8:	mov    0x10(%rsp),%rbx
   36bcd:	lea    0xc8(%rsp),%rdi
   36bd5:	mov    0xefee4(%rip),%r10        # 126ac0 <_DYNAMIC+0x618>
   36bdc:	ja     35c02 <meshopt_peers::run+0x13a2>
   36be2:	jmp    37064 <meshopt_peers::run+0x2804>
   36be7:	mov    $0x8,%edx
   36bec:	mov    $0x8,%r8d
   36bf2:	mov    $0x30,%r9d
   36bf8:	lea    0x2a0(%rsp),%rdi
   36c00:	xor    %esi,%esi
   36c02:	mov    %r14,%rbx
   36c05:	mov    %r14,%rcx
   36c08:	call   3b590 <<alloc::raw_vec::RawVecInner>::finish_grow>
   36c0d:	cmpl   $0x1,0x2a0(%rsp)
   36c15:	je     370ca <meshopt_peers::run+0x286a>
   36c1b:	mov    0x2a8(%rsp),%r15
   36c23:	mov    %rbx,%r14
   36c26:	mov    %rbx,%rax
   36c29:	mov    0x10(%rsp),%rbx
   36c2e:	mov    0x18(%rsp),%r8
   36c33:	jmp    35e2b <meshopt_peers::run+0x15cb>
   36c38:	mov    $0x8,%ecx
   36c3d:	mov    $0x30,%r8d
   36c43:	lea    0x128(%rsp),%rdi
   36c4b:	xor    %esi,%esi
   36c4d:	mov    %rbx,%rdx
   36c50:	call   347c0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   36c55:	mov    0x138(%rsp),%r12
   36c5d:	jmp    35ee5 <meshopt_peers::run+0x1685>
   36c62:	mov    0xb0(%rsp),%r12
   36c6a:	lea    0x718(%rsp),%rbx
   36c72:	mov    0x100(%rsp),%r14
   36c7a:	mov    0xa8(%rsp),%r15
   36c82:	jmp    36b0b <meshopt_peers::run+0x22ab>
   36c87:	mov    $0x1,%al
   36c89:	mov    %eax,(%rsp)
   36c8c:	jmp    36d5c <meshopt_peers::run+0x24fc>
   36c91:	lea    0x718(%rsp),%rdi
   36c99:	mov    %r12,%rsi
   36c9c:	mov    0x30(%rsp),%rdx
   36ca1:	call   33310 <<meshoptimizer_rs::processing::Context>::free::<meshoptimizer_rs::clusterlod::Pending>>
   36ca6:	cmp    $0xff,%al
   36ca8:	jne    372b8 <meshopt_peers::run+0x2a58>
   36cae:	lea    0x718(%rsp),%rdi
   36cb6:	mov    0x90(%rsp),%rsi
   36cbe:	mov    0x20(%rsp),%rdx
   36cc3:	call   33360 <<meshoptimizer_rs::processing::Context>::free::<u32>>
   36cc8:	mov    %eax,%ebp
   36cca:	jmp    3545e <meshopt_peers::run+0xbfe>
   36ccf:	movzbl 0x638(%rsp),%eax
   36cd7:	mov    %al,0x718(%rsp)
   36cde:	lea    -0x267ba(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36ce5:	lea    0xe898c(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   36cec:	lea    0xe8715(%rip),%r8        # 11f408 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x270>
   36cf3:	lea    0x718(%rsp),%rdx
   36cfb:	mov    $0x2b,%esi
   36d00:	call   *0xefa72(%rip)        # 126778 <_DYNAMIC+0x2d0>
   36d06:	lea    0x290(%rsp),%rax
   36d0e:	mov    %rax,0x718(%rsp)
   36d16:	lea    0x56d3(%rip),%rax        # 3c3f0 <<&str as core::fmt::Display>::fmt>
   36d1d:	mov    %rax,0x720(%rsp)
   36d25:	lea    -0x2a6be(%rip),%rdi        # c66e <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   36d2c:	lea    0xe85cd(%rip),%rdx        # 11f300 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x168>
   36d33:	lea    0x718(%rsp),%rsi
   36d3b:	call   *0xefa07(%rip)        # 126748 <_DYNAMIC+0x2a0>
   36d41:	cmpq   $0x0,0x60(%rsp)
   36d47:	mov    0x8(%rsp),%r12
   36d4c:	je     36d57 <meshopt_peers::run+0x24f7>
   36d4e:	mov    %r15,%rdi
   36d51:	call   *0xef959(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36d57:	mov    0x48(%rsp),%r15
   36d5c:	mov    $0x1,%r13b
   36d5f:	test   %r15,%r15
   36d62:	je     36d72 <meshopt_peers::run+0x2512>
   36d64:	mov    0x118(%rsp),%rdi
   36d6c:	call   *0xef93e(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36d72:	mov    $0x1,%r14b
   36d75:	mov    $0x1,%r15b
   36d78:	cmpb   $0x0,(%rsp)
   36d7c:	mov    0x68(%rsp),%rax
   36d81:	jne    36ee5 <meshopt_peers::run+0x2685>
   36d87:	jmp    36ef5 <meshopt_peers::run+0x2695>
   36d8c:	movzbl 0x720(%rsp),%eax
   36d94:	mov    %al,0x2a0(%rsp)
   36d9b:	lea    -0x26877(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36da2:	lea    0xe88cf(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   36da9:	lea    0xe85f8(%rip),%r8        # 11f3a8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x210>
   36db0:	lea    0x2a0(%rsp),%rdx
   36db8:	mov    $0x2b,%esi
   36dbd:	call   *0xef9b5(%rip)        # 126778 <_DYNAMIC+0x2d0>
   36dc3:	mov    $0x8,%bpl
   36dc6:	jmp    35445 <meshopt_peers::run+0xbe5>
   36dcb:	mov    $0x8,%bpl
   36dce:	cmpq   $0x0,0x58(%rsp)
   36dd4:	je     36de1 <meshopt_peers::run+0x2581>
   36dd6:	mov    0x78(%rsp),%rdi
   36ddb:	call   *0xef8cf(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36de1:	xor    %r15d,%r15d
   36de4:	xor    %ebx,%ebx
   36de6:	test   %r12,%r12
   36de9:	je     36df4 <meshopt_peers::run+0x2594>
   36deb:	mov    %r13,%rdi
   36dee:	call   *0xef8bc(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36df4:	cmpq   $0x0,0x28(%rsp)
   36dfa:	sete   %al
   36dfd:	or     %al,%r15b
   36e00:	jne    36e0d <meshopt_peers::run+0x25ad>
   36e02:	mov    0x40(%rsp),%rdi
   36e07:	call   *0xef8a3(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36e0d:	mov    $0x1,%r15b
   36e10:	mov    $0x1,%al
   36e12:	mov    %eax,(%rsp)
   36e15:	mov    $0x1,%r13b
   36e18:	test   %bl,%bl
   36e1a:	je     36f90 <meshopt_peers::run+0x2730>
   36e20:	mov    $0x1,%r15b
   36e23:	cmpq   $0x0,0x58(%rsp)
   36e29:	je     36f88 <meshopt_peers::run+0x2728>
   36e2f:	mov    0x78(%rsp),%rdi
   36e34:	call   *0xef876(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36e3a:	jmp    36f88 <meshopt_peers::run+0x2728>
   36e3f:	lea    0xe84a2(%rip),%rcx        # 11f2e8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x150>
   36e46:	mov    %r14,%rdx
   36e49:	call   *0xef9c9(%rip)        # 126818 <_DYNAMIC+0x370>
   36e4f:	jmp    3733e <meshopt_peers::run+0x2ade>
   36e54:	movzbl 0x720(%rsp),%eax
   36e5c:	mov    %al,0x2a0(%rsp)
   36e63:	lea    -0x2693f(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36e6a:	lea    0xe8807(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   36e71:	lea    0xe8500(%rip),%r8        # 11f378 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1e0>
   36e78:	lea    0x2a0(%rsp),%rdx
   36e80:	mov    $0x2b,%esi
   36e85:	call   *0xef8ed(%rip)        # 126778 <_DYNAMIC+0x2d0>
   36e8b:	movzbl 0x720(%rsp),%eax
   36e93:	mov    %al,0x2a0(%rsp)
   36e9a:	lea    -0x26976(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36ea1:	lea    0xe87d0(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   36ea8:	lea    0xe8481(%rip),%r8        # 11f330 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x198>
   36eaf:	lea    0x2a0(%rsp),%rdx
   36eb7:	mov    $0x2b,%esi
   36ebc:	call   *0xef8b6(%rip)        # 126778 <_DYNAMIC+0x2d0>
   36ec2:	mov    $0x8,%bpl
   36ec5:	lea    0x2a0(%rsp),%rdi
   36ecd:	call   339f0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   36ed2:	mov    $0x1,%r14b
   36ed5:	mov    $0x1,%r15b
   36ed8:	mov    $0x1,%r13b
   36edb:	mov    0x8(%rsp),%r12
   36ee0:	mov    0x68(%rsp),%rax
   36ee5:	test   %rax,%rax
   36ee8:	je     36ef5 <meshopt_peers::run+0x2695>
   36eea:	mov    0x20(%rsp),%rdi
   36eef:	call   *0xef7bb(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36ef5:	test   %r13b,%r13b
   36ef8:	je     36f0a <meshopt_peers::run+0x26aa>
   36efa:	test   %r12,%r12
   36efd:	je     36f0a <meshopt_peers::run+0x26aa>
   36eff:	mov    0x30(%rsp),%rdi
   36f04:	call   *0xef7a6(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36f0a:	test   %r15b,%r15b
   36f0d:	jne    3542d <meshopt_peers::run+0xbcd>
   36f13:	jmp    35440 <meshopt_peers::run+0xbe0>
   36f18:	mov    $0x1,%al
   36f1a:	mov    %eax,(%rsp)
   36f1d:	mov    $0x1,%r13b
   36f20:	mov    0x8(%rsp),%r12
   36f25:	jmp    36d5f <meshopt_peers::run+0x24ff>
   36f2a:	movzbl 0x720(%rsp),%eax
   36f32:	mov    %al,0x2a0(%rsp)
   36f39:	lea    -0x26a15(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   36f40:	lea    0xe8731(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   36f47:	lea    0xe8472(%rip),%r8        # 11f3c0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x228>
   36f4e:	lea    0x2a0(%rsp),%rdx
   36f56:	mov    $0x2b,%esi
   36f5b:	call   *0xef817(%rip)        # 126778 <_DYNAMIC+0x2d0>
   36f61:	cmpq   $0x0,0x28(%rsp)
   36f67:	je     36f74 <meshopt_peers::run+0x2714>
   36f69:	mov    0x40(%rsp),%rdi
   36f6e:	call   *0xef73c(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36f74:	test   %r12,%r12
   36f77:	je     36f82 <meshopt_peers::run+0x2722>
   36f79:	mov    %r13,%rdi
   36f7c:	call   *0xef72e(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36f82:	mov    $0x8,%bpl
   36f85:	mov    $0x1,%r15b
   36f88:	mov    $0x1,%al
   36f8a:	mov    %eax,(%rsp)
   36f8d:	mov    $0x1,%r13b
   36f90:	cmpq   $0x0,0x90(%rsp)
   36f99:	je     36fa9 <meshopt_peers::run+0x2749>
   36f9b:	mov    0xb8(%rsp),%rdi
   36fa3:	call   *0xef707(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36fa9:	cmpq   $0x0,0x60(%rsp)
   36faf:	je     36fbf <meshopt_peers::run+0x275f>
   36fb1:	mov    0x108(%rsp),%rdi
   36fb9:	call   *0xef6f1(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36fbf:	test   %r15b,%r15b
   36fc2:	mov    0x10(%rsp),%rbx
   36fc7:	mov    0x8(%rsp),%r12
   36fcc:	mov    0x48(%rsp),%r15
   36fd1:	jne    36d5f <meshopt_peers::run+0x24ff>
   36fd7:	jmp    36d72 <meshopt_peers::run+0x2512>
   36fdc:	mov    0x8(%rsp),%r12
   36fe1:	jmp    36d5c <meshopt_peers::run+0x24fc>
   36fe6:	cmpq   $0x0,0x60(%rsp)
   36fec:	je     36ff7 <meshopt_peers::run+0x2797>
   36fee:	mov    %r13,%rdi
   36ff1:	call   *0xef6b9(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   36ff7:	mov    $0x1,%r13b
   36ffa:	jmp    37002 <meshopt_peers::run+0x27a2>
   36ffc:	mov    $0x1,%r13b
   36fff:	mov    %r15d,%ebp
   37002:	mov    0x10(%rsp),%rbx
   37007:	mov    0x8(%rsp),%r12
   3700c:	mov    0x48(%rsp),%r15
   37011:	jmp    36d5f <meshopt_peers::run+0x24ff>
   37016:	mov    %eax,%ebp
   37018:	xor    %r15d,%r15d
   3701b:	jmp    36f90 <meshopt_peers::run+0x2730>
   37020:	mov    %eax,%ebp
   37022:	xor    %r15d,%r15d
   37025:	movl   $0x0,(%rsp)
   3702c:	jmp    36f90 <meshopt_peers::run+0x2730>
   37031:	movzbl 0x2a1(%rsp),%ebp
   37039:	cmpq   $0x0,0x28(%rsp)
   3703f:	je     36e20 <meshopt_peers::run+0x25c0>
   37045:	mov    0x40(%rsp),%rdi
   3704a:	call   *0xef660(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37050:	mov    $0x1,%r15b
   37053:	cmpq   $0x0,0x58(%rsp)
   37059:	jne    36e2f <meshopt_peers::run+0x25cf>
   3705f:	jmp    36f88 <meshopt_peers::run+0x2728>
   37064:	mov    0x108(%rsp),%rax
   3706c:	mov    %rax,0x30(%rsp)
   37071:	mov    0xb8(%rsp),%rax
   37079:	mov    %rax,0x20(%rsp)
   3707e:	test   %r15,%r15
   37081:	mov    0x60(%rsp),%r12
   37086:	jne    35af5 <meshopt_peers::run+0x1295>
   3708c:	jmp    35a32 <meshopt_peers::run+0x11d2>
   37091:	shr    $0x8,%ebp
   37094:	mov    $0x1,%bl
   37096:	jmp    370b9 <meshopt_peers::run+0x2859>
   37098:	test   %r15,%r15
   3709b:	je     370a6 <meshopt_peers::run+0x2846>
   3709d:	mov    %rbp,%rdi
   370a0:	call   *0xef60a(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   370a6:	mov    $0x8,%bpl
   370a9:	cmpq   $0x0,(%rsp)
   370ae:	je     370b9 <meshopt_peers::run+0x2859>
   370b0:	mov    %r14,%rdi
   370b3:	call   *0xef5f7(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   370b9:	xor    %r15d,%r15d
   370bc:	test   %r12,%r12
   370bf:	jne    36deb <meshopt_peers::run+0x258b>
   370c5:	jmp    36df4 <meshopt_peers::run+0x2594>
   370ca:	mov    $0xa,%bpl
   370cd:	mov    0x10(%rsp),%rbx
   370d2:	mov    0x8(%rsp),%r12
   370d7:	jmp    36d57 <meshopt_peers::run+0x24f7>
   370dc:	mov    %rax,%rdi
   370df:	mov    %rcx,%r15
   370e2:	mov    %rsi,(%rsp)
   370e6:	jmp    370f7 <meshopt_peers::run+0x2897>
   370e8:	mov    %rax,%rdi
   370eb:	mov    %rcx,%r15
   370ee:	mov    0x18(%rsp),%rax
   370f3:	mov    %rax,(%rsp)
   370f7:	lea    0xe815a(%rip),%rax        # 11f258 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xc0>
   370fe:	mov    %rax,0x178(%rsp)
   37106:	mov    %r15,%rsi
   37109:	mov    (%rsp),%rdx
   3710d:	mov    0x178(%rsp),%rcx
   37115:	call   *0xef6fd(%rip)        # 126818 <_DYNAMIC+0x370>
   3711b:	jmp    3733e <meshopt_peers::run+0x2ade>
   37120:	mov    $0x8,%bpl
   37123:	cmpq   $0x0,(%rsp)
   37128:	je     370b9 <meshopt_peers::run+0x2859>
   3712a:	jmp    370b0 <meshopt_peers::run+0x2850>
   3712c:	movb   $0x2,0x718(%rsp)
   37134:	lea    -0x26c10(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   3713b:	lea    0xe8536(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   37142:	lea    0xe82a7(%rip),%r8        # 11f3f0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x258>
   37149:	lea    0x718(%rsp),%rdx
   37151:	mov    $0x2b,%esi
   37156:	call   *0xef61c(%rip)        # 126778 <_DYNAMIC+0x2d0>
   3715c:	mov    $0xa,%bpl
   3715f:	mov    $0x1,%r15b
   37162:	cmpq   $0x0,0x58(%rsp)
   37168:	jne    36e2f <meshopt_peers::run+0x25cf>
   3716e:	jmp    36f88 <meshopt_peers::run+0x2728>
   37173:	movb   $0x8,0x718(%rsp)
   3717b:	lea    -0x26c57(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   37182:	lea    0xe84ef(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   37189:	lea    0xe8200(%rip),%r8        # 11f390 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x1f8>
   37190:	lea    0x718(%rsp),%rdx
   37198:	mov    $0x2b,%esi
   3719d:	call   *0xef5d5(%rip)        # 126778 <_DYNAMIC+0x2d0>
   371a3:	mov    $0x4,%edi
   371a8:	mov    %r13,%rsi
   371ab:	call   *0xef537(%rip)        # 1266e8 <_DYNAMIC+0x240>
   371b1:	mov    $0x8,%bpl
   371b4:	cmpq   $0x0,0x58(%rsp)
   371ba:	je     36de1 <meshopt_peers::run+0x2581>
   371c0:	jmp    36dd6 <meshopt_peers::run+0x2576>
   371c5:	lea    0xe80bc(%rip),%rcx        # 11f288 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xf0>
   371cc:	mov    %r14,%rdi
   371cf:	call   *0xef643(%rip)        # 126818 <_DYNAMIC+0x370>
   371d5:	jmp    3733e <meshopt_peers::run+0x2ade>
   371da:	mov    $0xb,%bpl
   371dd:	jmp    370b9 <meshopt_peers::run+0x2859>
   371e2:	shr    $0x8,%ebp
   371e5:	jmp    370b9 <meshopt_peers::run+0x2859>
   371ea:	mov    $0x4,%edi
   371ef:	mov    %r15,%rsi
   371f2:	call   *0xef4f0(%rip)        # 1266e8 <_DYNAMIC+0x240>
   371f8:	jmp    3733e <meshopt_peers::run+0x2ade>
   371fd:	mov    $0x8,%bpl
   37200:	mov    $0x1,%r15b
   37203:	cmpq   $0x0,0x58(%rsp)
   37209:	jne    36e2f <meshopt_peers::run+0x25cf>
   3720f:	jmp    36f88 <meshopt_peers::run+0x2728>
   37214:	mov    $0x8,%bpl
   37217:	mov    $0x1,%r15b
   3721a:	cmpq   $0x0,0x28(%rsp)
   37220:	je     37247 <meshopt_peers::run+0x29e7>
   37222:	mov    0x40(%rsp),%rdi
   37227:	call   *0xef483(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   3722d:	mov    $0x1,%eax
   37232:	mov    %rax,0x28(%rsp)
   37237:	xor    %ebx,%ebx
   37239:	test   %r12,%r12
   3723c:	jne    36deb <meshopt_peers::run+0x258b>
   37242:	jmp    36df4 <meshopt_peers::run+0x2594>
   37247:	movq   $0x0,0x28(%rsp)
   37250:	xor    %ebx,%ebx
   37252:	test   %r12,%r12
   37255:	jne    36deb <meshopt_peers::run+0x258b>
   3725b:	jmp    36df4 <meshopt_peers::run+0x2594>
   37260:	mov    $0x8,%bpl
   37263:	mov    $0x1,%r15b
   37266:	test   %r12,%r12
   37269:	je     36f88 <meshopt_peers::run+0x2728>
   3726f:	mov    %r13,%rdi
   37272:	jmp    36e34 <meshopt_peers::run+0x25d4>
   37277:	lea    0xe7faa(%rip),%rcx        # 11f228 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x90>
   3727e:	mov    0xb0(%rsp),%rdi
   37286:	mov    0x38(%rsp),%rsi
   3728b:	mov    0x170(%rsp),%rdx
   37293:	call   *0xef57f(%rip)        # 126818 <_DYNAMIC+0x370>
   37299:	jmp    3733e <meshopt_peers::run+0x2ade>
   3729e:	lea    0xe7f9b(%rip),%rdx        # 11f240 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   372a5:	mov    0x70(%rsp),%rdi
   372aa:	mov    %rdi,%rsi
   372ad:	call   *0xef445(%rip)        # 1266f8 <_DYNAMIC+0x250>
   372b3:	jmp    3733e <meshopt_peers::run+0x2ade>
   372b8:	mov    %eax,%ebp
   372ba:	cmpq   $0x0,0x90(%rsp)
   372c3:	je     372d0 <meshopt_peers::run+0x2a70>
   372c5:	mov    0x20(%rsp),%rdi
   372ca:	call   *0xef3e0(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   372d0:	mov    0x718(%rsp),%rax
   372d8:	mov    0x728(%rsp),%rcx
   372e0:	sub    0x720(%rsp),%rcx
   372e8:	mov    %rcx,0x358(%rax)
   372ef:	mov    %bpl,0x718(%rsp)
   372f7:	lea    -0x26dd3(%rip),%rdi        # 1052b <libm::math::generic::sqrt::RSQRT_TAB+0x1cd>
   372fe:	lea    0xe8373(%rip),%rcx        # 11f678 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x4e0>
   37305:	lea    0xe8114(%rip),%r8        # 11f420 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x288>
   3730c:	lea    0x718(%rsp),%rdx
   37314:	mov    $0x2b,%esi
   37319:	call   *0xef459(%rip)        # 126778 <_DYNAMIC+0x2d0>
   3731f:	jmp    3733e <meshopt_peers::run+0x2ade>
   37321:	lea    0xe7ee8(%rip),%rdx        # 11f210 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x78>
   37328:	mov    0xa8(%rsp),%rdi
   37330:	mov    0x100(%rsp),%rsi
   37338:	call   *0xef3ba(%rip)        # 1266f8 <_DYNAMIC+0x250>
   3733e:	ud2
   37340:	mov    %rax,%rbx
   37343:	cmpq   $0x0,0x128(%rsp)
   3734c:	je     37457 <meshopt_peers::run+0x2bf7>
   37352:	mov    0x130(%rsp),%r15
   3735a:	jmp    37397 <meshopt_peers::run+0x2b37>
   3735c:	jmp    3735e <meshopt_peers::run+0x2afe>
   3735e:	mov    %rax,%rbx
   37361:	test   %r12,%r12
   37364:	je     37410 <meshopt_peers::run+0x2bb0>
   3736a:	jmp    374d6 <meshopt_peers::run+0x2c76>
   3736f:	mov    %rax,%rbx
   37372:	cmpq   $0x0,0x58(%rsp)
   37378:	je     37428 <meshopt_peers::run+0x2bc8>
   3737e:	jmp    37588 <meshopt_peers::run+0x2d28>
   37383:	jmp    37524 <meshopt_peers::run+0x2cc4>
   37388:	mov    %rax,%rbx
   3738b:	cmpq   $0x0,0x60(%rsp)
   37391:	je     37457 <meshopt_peers::run+0x2bf7>
   37397:	mov    %r15,%rdi
   3739a:	jmp    37451 <meshopt_peers::run+0x2bf1>
   3739f:	jmp    37562 <meshopt_peers::run+0x2d02>
   373a4:	mov    %rax,%rbx
   373a7:	jmp    37441 <meshopt_peers::run+0x2be1>
   373ac:	mov    %rax,%rbx
   373af:	mov    %r12,0x8(%rsp)
   373b4:	mov    0x90(%rsp),%rax
   373bc:	mov    %rax,0x68(%rsp)
   373c1:	cmpq   $0x0,0x68(%rsp)
   373c7:	je     3746b <meshopt_peers::run+0x2c0b>
   373cd:	jmp    375c3 <meshopt_peers::run+0x2d63>
   373d2:	mov    %rax,%rbx
   373d5:	cmpq   $0x0,0x68(%rsp)
   373db:	je     3746b <meshopt_peers::run+0x2c0b>
   373e1:	jmp    375c3 <meshopt_peers::run+0x2d63>
   373e6:	mov    %r15,0x48(%rsp)
   373eb:	mov    %rax,%rbx
   373ee:	jmp    37457 <meshopt_peers::run+0x2bf7>
   373f0:	mov    %rax,%rbx
   373f3:	cmpq   $0x0,(%rsp)
   373f8:	jne    374b2 <meshopt_peers::run+0x2c52>
   373fe:	test   %r15,%r15
   37401:	jne    374c4 <meshopt_peers::run+0x2c64>
   37407:	test   %r12,%r12
   3740a:	jne    374d6 <meshopt_peers::run+0x2c76>
   37410:	cmpq   $0x0,0x28(%rsp)
   37416:	jne    37571 <meshopt_peers::run+0x2d11>
   3741c:	cmpq   $0x0,0x58(%rsp)
   37422:	jne    37588 <meshopt_peers::run+0x2d28>
   37428:	cmpq   $0x0,0x90(%rsp)
   37431:	je     37441 <meshopt_peers::run+0x2be1>
   37433:	mov    0xb8(%rsp),%rdi
   3743b:	call   *0xef26f(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37441:	cmpq   $0x0,0x60(%rsp)
   37447:	je     37457 <meshopt_peers::run+0x2bf7>
   37449:	mov    0x108(%rsp),%rdi
   37451:	call   *0xef259(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37457:	cmpq   $0x0,0x48(%rsp)
   3745d:	jne    37497 <meshopt_peers::run+0x2c37>
   3745f:	cmpq   $0x0,0x68(%rsp)
   37465:	jne    375c3 <meshopt_peers::run+0x2d63>
   3746b:	cmpq   $0x0,0x8(%rsp)
   37471:	jne    375da <meshopt_peers::run+0x2d7a>
   37477:	cmpq   $0x0,0x10(%rsp)
   3747d:	jne    375f1 <meshopt_peers::run+0x2d91>
   37483:	cmpq   $0x0,0x188(%rsp)
   3748c:	jne    3760a <meshopt_peers::run+0x2daa>
   37492:	jmp    37618 <meshopt_peers::run+0x2db8>
   37497:	mov    0x118(%rsp),%rdi
   3749f:	call   *0xef20b(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   374a5:	cmpq   $0x0,0x68(%rsp)
   374ab:	je     3746b <meshopt_peers::run+0x2c0b>
   374ad:	jmp    375c3 <meshopt_peers::run+0x2d63>
   374b2:	mov    %r14,%rdi
   374b5:	call   *0xef1f5(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   374bb:	test   %r15,%r15
   374be:	je     37407 <meshopt_peers::run+0x2ba7>
   374c4:	mov    %rbp,%rdi
   374c7:	call   *0xef1e3(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   374cd:	test   %r12,%r12
   374d0:	je     37410 <meshopt_peers::run+0x2bb0>
   374d6:	mov    %r13,%rdi
   374d9:	call   *0xef1d1(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   374df:	cmpq   $0x0,0x28(%rsp)
   374e5:	je     3741c <meshopt_peers::run+0x2bbc>
   374eb:	jmp    37571 <meshopt_peers::run+0x2d11>
   374f0:	jmp    374f2 <meshopt_peers::run+0x2c92>
   374f2:	mov    %rax,%rbx
   374f5:	cmpq   $0x0,0x10(%rsp)
   374fb:	je     37483 <meshopt_peers::run+0x2c23>
   374fd:	jmp    375f1 <meshopt_peers::run+0x2d91>
   37502:	mov    %rax,%rbx
   37505:	lea    0x2a0(%rsp),%rdi
   3750d:	call   339b0 <core::ptr::drop_glue::<meshoptimizer_rs::meshlet::Meshlets>>
   37512:	mov    %rbx,%rdi
   37515:	call   11d740 <_Unwind_Resume@plt>
   3751a:	jmp    375a7 <meshopt_peers::run+0x2d47>
   3751f:	jmp    375a7 <meshopt_peers::run+0x2d47>
   37524:	mov    %rax,%rbx
   37527:	cmpq   $0x0,0x90(%rsp)
   37530:	jne    37433 <meshopt_peers::run+0x2bd3>
   37536:	jmp    37441 <meshopt_peers::run+0x2be1>
   3753b:	mov    %rax,%rbx
   3753e:	cmpq   $0x0,0x188(%rsp)
   37547:	jne    3760a <meshopt_peers::run+0x2daa>
   3754d:	jmp    37618 <meshopt_peers::run+0x2db8>
   37552:	mov    %rax,%rbx
   37555:	jmp    37618 <meshopt_peers::run+0x2db8>
   3755a:	mov    %rax,%rbx
   3755d:	jmp    37637 <meshopt_peers::run+0x2dd7>
   37562:	mov    %rax,%rbx
   37565:	cmpq   $0x0,0x28(%rsp)
   3756b:	je     3741c <meshopt_peers::run+0x2bbc>
   37571:	mov    0x40(%rsp),%rdi
   37576:	call   *0xef134(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   3757c:	cmpq   $0x0,0x58(%rsp)
   37582:	je     37428 <meshopt_peers::run+0x2bc8>
   37588:	mov    0x78(%rsp),%rdi
   3758d:	call   *0xef11d(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37593:	cmpq   $0x0,0x90(%rsp)
   3759c:	jne    37433 <meshopt_peers::run+0x2bd3>
   375a2:	jmp    37441 <meshopt_peers::run+0x2be1>
   375a7:	mov    %rax,%rbx
   375aa:	lea    0x2a0(%rsp),%rdi
   375b2:	call   339f0 <core::ptr::drop_glue::<meshoptimizer_rs::workspace::Workspace>>
   375b7:	cmpq   $0x0,0x68(%rsp)
   375bd:	je     3746b <meshopt_peers::run+0x2c0b>
   375c3:	mov    0x20(%rsp),%rdi
   375c8:	call   *0xef0e2(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   375ce:	cmpq   $0x0,0x8(%rsp)
   375d4:	je     37477 <meshopt_peers::run+0x2c17>
   375da:	mov    0x30(%rsp),%rdi
   375df:	call   *0xef0cb(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   375e5:	cmpq   $0x0,0x10(%rsp)
   375eb:	je     37483 <meshopt_peers::run+0x2c23>
   375f1:	mov    0x80(%rsp),%rdi
   375f9:	call   *0xef0b1(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   375ff:	cmpq   $0x0,0x188(%rsp)
   37608:	je     37618 <meshopt_peers::run+0x2db8>
   3760a:	mov    0xc0(%rsp),%rdi
   37612:	call   *0xef098(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37618:	mov    0x718(%rsp),%rax
   37620:	mov    0x728(%rsp),%rcx
   37628:	sub    0x720(%rsp),%rcx
   37630:	mov    %rcx,0x358(%rax)
   37637:	cmpq   $0x0,0x1a8(%rsp)
   37640:	jne    37652 <meshopt_peers::run+0x2df2>
   37642:	cmpq   $0x0,0x10(%rsp)
   37648:	jne    37668 <meshopt_peers::run+0x2e08>
   3764a:	mov    %rbx,%rdi
   3764d:	call   11d740 <_Unwind_Resume@plt>
   37652:	mov    0x1b0(%rsp),%rdi
   3765a:	call   *0xef050(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37660:	cmpq   $0x0,0x10(%rsp)
   37666:	je     3764a <meshopt_peers::run+0x2dea>
   37668:	mov    0x50(%rsp),%rdi
   3766d:	call   *0xef03d(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   37673:	mov    %rbx,%rdi
   37676:	call   11d740 <_Unwind_Resume@plt>
