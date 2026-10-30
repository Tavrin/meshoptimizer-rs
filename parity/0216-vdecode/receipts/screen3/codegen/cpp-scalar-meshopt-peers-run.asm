
<artifacts>/screen3/driver-cpp-scalar:     file format elf64-x86-64


Disassembly of section .text:

0000000000024ac0 <meshopt_peers::run>:
   24ac0:	push   %rbp
   24ac1:	push   %r15
   24ac3:	push   %r14
   24ac5:	push   %r13
   24ac7:	push   %r12
   24ac9:	push   %rbx
   24aca:	sub    $0x138,%rsp
   24ad1:	mov    %rsi,0x60(%rsp)
   24ad6:	mov    %rdx,0x68(%rsp)
   24adb:	lea    -0x5(%rdx),%rax
   24adf:	cmp    $0xe,%rax
   24ae3:	ja     257b9 <meshopt_peers::run+0xcf9>
   24ae9:	mov    %rcx,%r12
   24aec:	mov    %rdx,%r15
   24aef:	mov    %rsi,%rbp
   24af2:	mov    %rdi,%rbx
   24af5:	mov    0x8(%rcx),%rcx
   24af9:	mov    0x10(%r12),%r13
   24afe:	mov    0x28(%r12),%r14
   24b03:	mov    0x20(%r12),%rsi
   24b08:	lea    -0x1ac23(%rip),%rdx        # 9eec <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0x67>
   24b0f:	movslq (%rdx,%rax,4),%rax
   24b13:	add    %rdx,%rax
   24b16:	jmp    *%rax
   24b18:	mov    $0x68636163,%eax
   24b1d:	xor    0x0(%rbp),%eax
   24b20:	movzbl 0x4(%rbp),%edx
   24b24:	xor    $0x65,%edx
   24b27:	or     %eax,%edx
   24b29:	je     24fb6 <meshopt_peers::run+0x4f6>
   24b2f:	mov    $0x63746566,%eax
   24b34:	xor    0x0(%rbp),%eax
   24b37:	movzbl 0x4(%rbp),%edx
   24b3b:	xor    $0x68,%edx
   24b3e:	or     %eax,%edx
   24b40:	jne    257b9 <meshopt_peers::run+0xcf9>
   24b46:	test   %r14,%r14
   24b49:	mov    %rcx,0x20(%rsp)
   24b4e:	je     252ad <meshopt_peers::run+0x7ed>
   24b54:	mov    %rsi,%rbp
   24b57:	lea    0x0(,%r14,4),%r12
   24b5f:	mov    %r12,%rdi
   24b62:	call   *0x62210(%rip)        # 86d78 <malloc@GLIBC_2.2.5>
   24b68:	test   %rax,%rax
   24b6b:	je     2582d <meshopt_peers::run+0xd6d>
   24b71:	mov    %rax,%r15
   24b74:	mov    %rax,%rdi
   24b77:	mov    %rbp,%rsi
   24b7a:	mov    %r12,%rdx
   24b7d:	call   *0x621c5(%rip)        # 86d48 <memcpy@GLIBC_2.14>
   24b83:	jmp    252b3 <meshopt_peers::run+0x7f3>
   24b88:	movabs $0x7966696c706d6973,%rax
   24b92:	xor    0x0(%rbp),%rax
   24b96:	movabs $0x7970706f6c732d79,%rdx
   24ba0:	xor    0x7(%rbp),%rdx
   24ba4:	or     %rax,%rdx
   24ba7:	je     24c85 <meshopt_peers::run+0x1c5>
   24bad:	jmp    257b9 <meshopt_peers::run+0xcf9>
   24bb2:	movdqu 0x0(%rbp),%xmm0
   24bb7:	movdqu 0x3(%rbp),%xmm1
   24bbc:	pcmpeqb -0x1cff4(%rip),%xmm1        # 7bd0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x30>
   24bc4:	pcmpeqb -0x1d20c(%rip),%xmm0        # 79c0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x160>
   24bcc:	pand   %xmm1,%xmm0
   24bd0:	pmovmskb %xmm0,%eax
   24bd4:	cmp    $0xffff,%eax
   24bd9:	je     24c85 <meshopt_peers::run+0x1c5>
   24bdf:	jmp    257b9 <meshopt_peers::run+0xcf9>
   24be4:	movabs $0x7966696c706d6973,%rax
   24bee:	cmp    %rax,0x0(%rbp)
   24bf2:	je     24c85 <meshopt_peers::run+0x1c5>
   24bf8:	movabs $0x776172647265766f,%rax
   24c02:	cmp    %rax,0x0(%rbp)
   24c06:	je     24fb6 <meshopt_peers::run+0x4f6>
   24c0c:	movabs $0x7374656c6873656d,%rax
   24c16:	cmp    %rax,0x0(%rbp)
   24c1a:	jne    257b9 <meshopt_peers::run+0xcf9>
   24c20:	mov    %rsi,0x10(%rsp)
   24c25:	mov    %rcx,0x20(%rsp)
   24c2a:	mov    %r13,0x8(%rsp)
   24c2f:	mov    %rbx,0x18(%rsp)
   24c34:	movabs $0x7ffffffffffffffc,%rbx
   24c3e:	movabs $0xffffffffffffffc,%r13
   24c48:	mov    $0x40,%esi
   24c4d:	mov    $0x7c,%edx
   24c52:	mov    %r14,%rdi
   24c55:	call   *0x62475(%rip)        # 870d0 <_DYNAMIC+0x5c0>
   24c5b:	mov    %rax,%rbp
   24c5e:	mov    %rax,%rsi
   24c61:	shl    $0x4,%rsi
   24c65:	lea    0x3(%r13),%rax
   24c69:	cmp    %rax,%rbp
   24c6c:	seta   %al
   24c6f:	cmp    %rbx,%rsi
   24c72:	seta   %cl
   24c75:	or     %al,%cl
   24c77:	je     253df <meshopt_peers::run+0x91f>
   24c7d:	xor    %edi,%edi
   24c7f:	call   *0x620a3(%rip)        # 86d28 <_DYNAMIC+0x218>
   24c85:	test   %r14,%r14
   24c88:	je     24f2a <meshopt_peers::run+0x46a>
   24c8e:	mov    %rsi,0x10(%rsp)
   24c93:	mov    %rcx,0x20(%rsp)
   24c98:	mov    %r13,0x8(%rsp)
   24c9d:	mov    %r14,%rax
   24ca0:	mov    %rbx,%r14
   24ca3:	mov    %rax,%r13
   24ca6:	lea    0x0(,%rax,4),%rbx
   24cae:	mov    $0x1,%esi
   24cb3:	mov    %rbx,%rdi
   24cb6:	call   *0x6233c(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   24cbc:	test   %rax,%rax
   24cbf:	je     25803 <meshopt_peers::run+0xd43>
   24cc5:	mov    %r14,%rbx
   24cc8:	mov    %r13,%r14
   24ccb:	mov    0x8(%rsp),%r13
   24cd0:	mov    0x20(%rsp),%rcx
   24cd5:	mov    0x10(%rsp),%rsi
   24cda:	movl   $0x0,0x70(%rsp)
   24ce2:	cmp    $0x8,%r15
   24ce6:	je     24f41 <meshopt_peers::run+0x481>
   24cec:	cmp    $0x13,%r15
   24cf0:	jne    24f55 <meshopt_peers::run+0x495>
   24cf6:	movdqu 0x0(%rbp),%xmm0
   24cfb:	movdqu 0x3(%rbp),%xmm1
   24d00:	pcmpeqb -0x1d138(%rip),%xmm1        # 7bd0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x30>
   24d08:	pcmpeqb -0x1d350(%rip),%xmm0        # 79c0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x160>
   24d10:	pand   %xmm1,%xmm0
   24d14:	pmovmskb %xmm0,%edx
   24d18:	cmp    $0xffff,%edx
   24d1e:	jne    24f55 <meshopt_peers::run+0x495>
   24d24:	movss  0x80(%r12),%xmm0
   24d2e:	lea    0x70(%rsp),%r11
   24d33:	lea    -0x1d14a(%rip),%r10        # 7bf0 <anon.a243a2cefe40099c5384ecefc2bb7996.1.llvm.6308452637081725772+0x50>
   24d3a:	mov    $0xc,%r9d
   24d40:	mov    %rax,%r15
   24d43:	mov    %rax,%rdi
   24d46:	mov    %r13,%r8
   24d49:	mov    %r14,%rdx
   24d4c:	push   %r11
   24d4e:	push   $0x0
   24d50:	push   0x78(%r12)
   24d55:	push   $0x0
   24d57:	push   $0x4
   24d59:	push   %r10
   24d5b:	push   $0x10
   24d5d:	push   0x38(%r12)
   24d62:	call   *0x62370(%rip)        # 870d8 <_DYNAMIC+0x5c8>
   24d68:	add    $0x40,%rsp
   24d6c:	jmp    24f8d <meshopt_peers::run+0x4cd>
   24d71:	movabs $0x65646f6365642d78,%rax
   24d7b:	movabs $0x652d786574726576,%rdi
   24d85:	xor    0x0(%rbp),%rdi
   24d89:	lea    0x9010000(%rax),%rdx
   24d90:	xor    0x5(%rbp),%rdx
   24d94:	or     %rdi,%rdx
   24d97:	mov    %rbx,0x18(%rsp)
   24d9c:	je     2501d <meshopt_peers::run+0x55d>
   24da2:	movabs $0x642d786574726576,%rcx
   24dac:	xor    0x0(%rbp),%rcx
   24db0:	xor    0x5(%rbp),%rax
   24db4:	or     %rcx,%rax
   24db7:	jne    257b9 <meshopt_peers::run+0xcf9>
   24dbd:	lea    0x0(,%r13,4),%rax
   24dc5:	lea    (%rax,%rax,2),%r15
   24dc9:	mov    $0x1,%bl
   24dcb:	mov    %r13,0x8(%rsp)
   24dd0:	test   %r15,%r15
   24dd3:	je     24e53 <meshopt_peers::run+0x393>
   24dd5:	mov    $0x1,%esi
   24dda:	mov    %r15,%rdi
   24ddd:	call   *0x62215(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   24de3:	test   %rax,%rax
   24de6:	je     25811 <meshopt_peers::run+0xd51>
   24dec:	mov    %rax,%r13
   24def:	test   %bl,%bl
   24df1:	jne    24e5d <meshopt_peers::run+0x39d>
   24df3:	jmp    24e82 <meshopt_peers::run+0x3c2>
   24df8:	movabs $0x6e652d7865646e69,%rax
   24e02:	xor    0x0(%rbp),%rax
   24e06:	mov    0x8(%rbp),%edx
   24e09:	xor    $0x65646f63,%rdx
   24e10:	or     %rax,%rdx
   24e13:	mov    %rbx,0x18(%rsp)
   24e18:	je     2504c <meshopt_peers::run+0x58c>
   24e1e:	movabs $0x65642d7865646e69,%rax
   24e28:	xor    0x0(%rbp),%rax
   24e2c:	mov    0x8(%rbp),%ecx
   24e2f:	xor    $0x65646f63,%rcx
   24e36:	or     %rax,%rcx
   24e39:	jne    257b9 <meshopt_peers::run+0xcf9>
   24e3f:	lea    0x0(,%r14,4),%r15
   24e47:	xor    %ebx,%ebx
   24e49:	mov    %r13,0x8(%rsp)
   24e4e:	test   %r15,%r15
   24e51:	jne    24dd5 <meshopt_peers::run+0x315>
   24e53:	mov    $0x1,%r13d
   24e59:	test   %bl,%bl
   24e5b:	je     24e82 <meshopt_peers::run+0x3c2>
   24e5d:	movabs $0x642d786574726576,%rax
   24e67:	xor    0x0(%rbp),%rax
   24e6b:	movabs $0x65646f6365642d78,%rcx
   24e75:	xor    0x5(%rbp),%rcx
   24e79:	or     %rax,%rcx
   24e7c:	je     252e3 <meshopt_peers::run+0x823>
   24e82:	mov    0x68(%r12),%rcx
   24e87:	mov    0x70(%r12),%r8
   24e8c:	mov    $0x4,%edx
   24e91:	mov    %r13,%rdi
   24e94:	mov    %r14,%rsi
   24e97:	call   *0x62243(%rip)        # 870e0 <_DYNAMIC+0x5d0>
   24e9d:	mov    0x18(%rsp),%rcx
   24ea2:	mov    %eax,0x70(%rsp)
   24ea6:	test   %eax,%eax
   24ea8:	jne    25311 <meshopt_peers::run+0x851>
   24eae:	mov    %r15,0x8(%rcx)
   24eb2:	mov    %r13,0x10(%rcx)
   24eb6:	mov    %r15,0x18(%rcx)
   24eba:	jmp    25480 <meshopt_peers::run+0x9c0>
   24ebf:	movabs $0x2d72657473756c63,%rax
   24ec9:	xor    0x0(%rbp),%rax
   24ecd:	movabs $0x646f6c2d72657473,%rdx
   24ed7:	xor    0x3(%rbp),%rdx
   24edb:	or     %rax,%rdx
   24ede:	jne    257b9 <meshopt_peers::run+0xcf9>
   24ee4:	mov    %rsi,%r12
   24ee7:	test   %r13,%r13
   24eea:	je     250a9 <meshopt_peers::run+0x5e9>
   24ef0:	mov    %rcx,%rbp
   24ef3:	lea    0x0(,%r13,4),%rax
   24efb:	lea    (%rax,%rax,2),%r15
   24eff:	mov    %r15,%rdi
   24f02:	call   *0x61e70(%rip)        # 86d78 <malloc@GLIBC_2.2.5>
   24f08:	test   %rax,%rax
   24f0b:	je     2581f <meshopt_peers::run+0xd5f>
   24f11:	mov    %rax,0x8(%rsp)
   24f16:	mov    %rax,%rdi
   24f19:	mov    %rbp,%rsi
   24f1c:	mov    %r15,%rdx
   24f1f:	call   *0x61e23(%rip)        # 86d48 <memcpy@GLIBC_2.14>
   24f25:	jmp    250b3 <meshopt_peers::run+0x5f3>
   24f2a:	mov    $0x4,%eax
   24f2f:	movl   $0x0,0x70(%rsp)
   24f37:	cmp    $0x8,%r15
   24f3b:	jne    24cec <meshopt_peers::run+0x22c>
   24f41:	movabs $0x7966696c706d6973,%rdx
   24f4b:	cmp    %rdx,0x0(%rbp)
   24f4f:	je     25274 <meshopt_peers::run+0x7b4>
   24f55:	movss  0x80(%r12),%xmm0
   24f5f:	sub    $0x8,%rsp
   24f63:	lea    0x78(%rsp),%r10
   24f68:	mov    $0xc,%r9d
   24f6e:	mov    %rax,%r15
   24f71:	mov    %rax,%rdi
   24f74:	mov    %r13,%r8
   24f77:	mov    %r14,%rdx
   24f7a:	push   %r10
   24f7c:	push   0x78(%r12)
   24f81:	push   $0x0
   24f83:	call   *0x6215f(%rip)        # 870e8 <_DYNAMIC+0x5d8>
   24f89:	add    $0x20,%rsp
   24f8d:	cmp    %r14,%rax
   24f90:	cmovae %r14,%rax
   24f94:	movss  0x70(%rsp),%xmm0
   24f9a:	mov    %r14,0x8(%rbx)
   24f9e:	mov    %r15,0x10(%rbx)
   24fa2:	mov    %rax,0x18(%rbx)
   24fa6:	movss  %xmm0,0x4(%rbx)
   24fab:	movl   $0x2,(%rbx)
   24fb1:	jmp    25486 <meshopt_peers::run+0x9c6>
   24fb6:	test   %r14,%r14
   24fb9:	je     25388 <meshopt_peers::run+0x8c8>
   24fbf:	mov    %rsi,0x10(%rsp)
   24fc4:	mov    %rcx,0x20(%rsp)
   24fc9:	mov    %r13,0x8(%rsp)
   24fce:	mov    %r14,%rax
   24fd1:	mov    %rbx,%r14
   24fd4:	mov    %rax,%r13
   24fd7:	lea    0x0(,%rax,4),%rbx
   24fdf:	mov    $0x1,%esi
   24fe4:	mov    %rbx,%rdi
   24fe7:	call   *0x6200b(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   24fed:	test   %rax,%rax
   24ff0:	je     2584b <meshopt_peers::run+0xd8b>
   24ff6:	mov    %rax,%r12
   24ff9:	mov    %r14,%rbx
   24ffc:	mov    %r13,%r14
   24fff:	mov    0x8(%rsp),%r13
   25004:	mov    0x20(%rsp),%rcx
   25009:	mov    0x10(%rsp),%rsi
   2500e:	cmp    $0x5,%r15
   25012:	je     25394 <meshopt_peers::run+0x8d4>
   25018:	jmp    253ab <meshopt_peers::run+0x8eb>
   2501d:	mov    %rsi,0x10(%rsp)
   25022:	mov    %rcx,%r15
   25025:	mov    $0xc,%esi
   2502a:	mov    %r13,%rdi
   2502d:	call   *0x620bd(%rip)        # 870f0 <_DYNAMIC+0x5e0>
   25033:	mov    %rax,%r12
   25036:	mov    $0x1,%bl
   25038:	test   %r12,%r12
   2503b:	jns    2506a <meshopt_peers::run+0x5aa>
   2503d:	xor    %r13d,%r13d
   25040:	mov    %r13,%rdi
   25043:	mov    %r12,%rsi
   25046:	call   *0x61cdc(%rip)        # 86d28 <_DYNAMIC+0x218>
   2504c:	mov    %rsi,0x10(%rsp)
   25051:	mov    %rcx,%r15
   25054:	mov    %r14,%rdi
   25057:	mov    %r13,%rsi
   2505a:	call   *0x62098(%rip)        # 870f8 <_DYNAMIC+0x5e8>
   25060:	mov    %rax,%r12
   25063:	xor    %ebx,%ebx
   25065:	test   %r12,%r12
   25068:	js     2503d <meshopt_peers::run+0x57d>
   2506a:	je     2540b <meshopt_peers::run+0x94b>
   25070:	mov    %r13,0x8(%rsp)
   25075:	mov    $0x1,%r13d
   2507b:	mov    $0x1,%esi
   25080:	mov    %r12,%rdi
   25083:	call   *0x61f6f(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   25089:	mov    %rax,(%rsp)
   2508d:	test   %rax,%rax
   25090:	je     25040 <meshopt_peers::run+0x580>
   25092:	mov    0x8(%rsp),%r13
   25097:	mov    0x10(%rsp),%r8
   2509c:	test   %bl,%bl
   2509e:	jne    2541d <meshopt_peers::run+0x95d>
   250a4:	jmp    25449 <meshopt_peers::run+0x989>
   250a9:	mov    $0x4,%eax
   250ae:	mov    %rax,0x8(%rsp)
   250b3:	movq   $0x0,0x48(%rsp)
   250bc:	movq   $0x4,0x50(%rsp)
   250c5:	movq   $0x0,0x58(%rsp)
   250ce:	movl   $0x0,0x44(%rsp)
   250d6:	lea    0x70(%rsp),%rdi
   250db:	mov    $0x80,%esi
   250e0:	call   *0x61fe2(%rip)        # 870c8 <_DYNAMIC+0x5b8>
   250e6:	cmpb   $0x1,0xc2(%rsp)
   250ee:	je     257e8 <meshopt_peers::run+0xd28>
   250f4:	mov    %r12,0xe0(%rsp)
   250fc:	mov    %r14,0xe8(%rsp)
   25104:	mov    %r13,0xf0(%rsp)
   2510c:	mov    0x8(%rsp),%r14
   25111:	mov    %r14,0xf8(%rsp)
   25119:	movq   $0xc,0x100(%rsp)
   25125:	movq   $0x4,0x108(%rsp)
   25131:	xorps  %xmm0,%xmm0
   25134:	movups %xmm0,0x110(%rsp)
   2513c:	movq   $0x4,0x120(%rsp)
   25148:	movq   $0x0,0x128(%rsp)
   25154:	movl   $0x0,0x130(%rsp)
   2515f:	lea    0x48(%rsp),%rax
   25164:	mov    %rax,0xd0(%rsp)
   2516c:	lea    0x44(%rsp),%rax
   25171:	mov    %rax,0xd8(%rsp)
   25179:	sub    $0xc0,%rsp
   25180:	mov    0x1f0(%rsp),%rax
   25188:	mov    %rax,0xb0(%rsp)
   25190:	movups 0x1e0(%rsp),%xmm0
   25198:	movups %xmm0,0xa0(%rsp)
   251a0:	movups 0x1a0(%rsp),%xmm0
   251a8:	movups 0x1b0(%rsp),%xmm1
   251b0:	movups 0x1c0(%rsp),%xmm2
   251b8:	movups 0x1d0(%rsp),%xmm3
   251c0:	movups %xmm3,0x90(%rsp)
   251c8:	movups %xmm2,0x80(%rsp)
   251d0:	movups %xmm1,0x70(%rsp)
   251d5:	movups %xmm0,0x60(%rsp)
   251da:	movups 0x180(%rsp),%xmm0
   251e2:	movups %xmm0,0x50(%rsp)
   251e7:	movups 0x170(%rsp),%xmm0
   251ef:	movups %xmm0,0x40(%rsp)
   251f4:	movups 0x130(%rsp),%xmm0
   251fc:	movdqu 0x140(%rsp),%xmm1
   25205:	movups 0x150(%rsp),%xmm2
   2520d:	movups 0x160(%rsp),%xmm3
   25215:	movups %xmm3,0x30(%rsp)
   2521a:	movups %xmm2,0x20(%rsp)
   2521f:	movdqu %xmm1,0x10(%rsp)
   25225:	movups %xmm0,(%rsp)
   25229:	lea    -0xea0(%rip),%rsi        # 24390 <meshopt::clusterlod::output_callback::<meshopt_peers::clod::{closure#0}>>
   25230:	lea    0x190(%rsp),%rdi
   25238:	call   *0x61ec2(%rip)        # 87100 <_DYNAMIC+0x5f0>
   2523e:	add    $0xc0,%rsp
   25245:	mov    0x58(%rsp),%rax
   2524a:	mov    %rax,0x18(%rbx)
   2524e:	movups 0x48(%rsp),%xmm0
   25253:	movups %xmm0,0x8(%rbx)
   25257:	movl   $0x1,(%rbx)
   2525d:	test   %r13,%r13
   25260:	je     25486 <meshopt_peers::run+0x9c6>
   25266:	mov    %r14,%rdi
   25269:	call   *0x61af1(%rip)        # 86d60 <free@GLIBC_2.2.5>
   2526f:	jmp    25486 <meshopt_peers::run+0x9c6>
   25274:	movss  0x80(%r12),%xmm0
   2527e:	sub    $0x8,%rsp
   25282:	lea    0x78(%rsp),%r10
   25287:	mov    $0xc,%r9d
   2528d:	mov    %rax,%r15
   25290:	mov    %rax,%rdi
   25293:	mov    %r13,%r8
   25296:	mov    %r14,%rdx
   25299:	push   %r10
   2529b:	push   $0x0
   2529d:	push   0x78(%r12)
   252a2:	call   *0x61e60(%rip)        # 87108 <_DYNAMIC+0x5f8>
   252a8:	jmp    24f89 <meshopt_peers::run+0x4c9>
   252ad:	mov    $0x4,%r15d
   252b3:	lea    0x0(,%r13,4),%rax
   252bb:	lea    (%rax,%rax,2),%r12
   252bf:	mov    %r13,%rbp
   252c2:	test   %r13,%r13
   252c5:	je     25333 <meshopt_peers::run+0x873>
   252c7:	mov    $0x1,%esi
   252cc:	mov    %r12,%rdi
   252cf:	call   *0x61d23(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   252d5:	test   %rax,%rax
   252d8:	je     2583b <meshopt_peers::run+0xd7b>
   252de:	mov    %rax,%r13
   252e1:	jmp    25339 <meshopt_peers::run+0x879>
   252e3:	mov    0x50(%r12),%rcx
   252e8:	mov    0x58(%r12),%r8
   252ed:	mov    $0xc,%edx
   252f2:	mov    %r13,%rdi
   252f5:	mov    0x8(%rsp),%rsi
   252fa:	call   *0x61e10(%rip)        # 87110 <_DYNAMIC+0x600>
   25300:	mov    0x18(%rsp),%rcx
   25305:	mov    %eax,0x70(%rsp)
   25309:	test   %eax,%eax
   2530b:	je     24eae <meshopt_peers::run+0x3ee>
   25311:	lea    -0x1bc64(%rip),%rdx        # 96b4 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0x58>
   25318:	lea    0x5fc79(%rip),%r9        # 84f98 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xc0>
   2531f:	lea    0x70(%rsp),%rsi
   25324:	xor    %edi,%edi
   25326:	xor    %ecx,%ecx
   25328:	call   *0x61dea(%rip)        # 87118 <_DYNAMIC+0x608>
   2532e:	jmp    2587f <meshopt_peers::run+0xdbf>
   25333:	mov    $0x1,%r13d
   25339:	mov    0x20(%rsp),%rcx
   2533e:	mov    $0xc,%r9d
   25344:	mov    %r13,%rdi
   25347:	mov    %r15,%rsi
   2534a:	mov    %r14,%rdx
   2534d:	mov    %rbp,%r8
   25350:	call   *0x61dca(%rip)        # 87120 <_DYNAMIC+0x610>
   25356:	shl    $0x2,%rax
   2535a:	lea    (%rax,%rax,2),%rax
   2535e:	cmp    %r12,%rax
   25361:	cmovae %r12,%rax
   25365:	mov    %r14,0x8(%rbx)
   25369:	mov    %r15,0x10(%rbx)
   2536d:	mov    %r14,0x18(%rbx)
   25371:	mov    %r12,0x20(%rbx)
   25375:	mov    %r13,0x28(%rbx)
   25379:	mov    %rax,0x30(%rbx)
   2537d:	movl   $0x3,(%rbx)
   25383:	jmp    25486 <meshopt_peers::run+0x9c6>
   25388:	mov    $0x4,%r12d
   2538e:	cmp    $0x5,%r15
   25392:	jne    253ab <meshopt_peers::run+0x8eb>
   25394:	mov    $0x68636163,%eax
   25399:	xor    0x0(%rbp),%eax
   2539c:	movzbl 0x4(%rbp),%edx
   253a0:	xor    $0x65,%edx
   253a3:	or     %eax,%edx
   253a5:	je     25498 <meshopt_peers::run+0x9d8>
   253ab:	movss  -0x1bc5f(%rip),%xmm0        # 9754 <core::num::imp::flt2dec::strategy::dragon::POW5TO256+0xf8>
   253b3:	mov    $0xc,%r9d
   253b9:	mov    %r12,%rdi
   253bc:	mov    %r13,%r8
   253bf:	mov    %r14,%rdx
   253c2:	call   *0x61d60(%rip)        # 87128 <_DYNAMIC+0x618>
   253c8:	mov    %r14,0x8(%rbx)
   253cc:	mov    %r12,0x10(%rbx)
   253d0:	mov    %r14,0x18(%rbx)
   253d4:	movl   $0x1,(%rbx)
   253da:	jmp    25486 <meshopt_peers::run+0x9c6>
   253df:	test   %rsi,%rsi
   253e2:	je     254ac <meshopt_peers::run+0x9ec>
   253e8:	mov    %rsi,%r15
   253eb:	mov    %rsi,%rdi
   253ee:	call   *0x61984(%rip)        # 86d78 <malloc@GLIBC_2.2.5>
   253f4:	mov    %rax,(%rsp)
   253f8:	test   %rax,%rax
   253fb:	je     25859 <meshopt_peers::run+0xd99>
   25401:	mov    %rbp,0x38(%rsp)
   25406:	jmp    254be <meshopt_peers::run+0x9fe>
   2540b:	mov    $0x1,%eax
   25410:	mov    %rax,(%rsp)
   25414:	mov    0x10(%rsp),%r8
   25419:	test   %bl,%bl
   2541b:	je     25449 <meshopt_peers::run+0x989>
   2541d:	movabs $0x652d786574726576,%rax
   25427:	xor    0x0(%rbp),%rax
   2542b:	movabs $0x65646f6365642d78,%rcx
   25435:	add    $0x9010000,%rcx
   2543c:	xor    0x5(%rbp),%rcx
   25440:	or     %rax,%rcx
   25443:	je     255ab <meshopt_peers::run+0xaeb>
   25449:	mov    (%rsp),%rbx
   2544d:	mov    %rbx,%rdi
   25450:	mov    %r12,%rsi
   25453:	mov    %r8,%rdx
   25456:	mov    %r14,%rcx
   25459:	call   *0x61cd1(%rip)        # 87130 <_DYNAMIC+0x620>
   2545f:	mov    0x18(%rsp),%rcx
   25464:	test   %rax,%rax
   25467:	je     255e6 <meshopt_peers::run+0xb26>
   2546d:	cmp    %r12,%rax
   25470:	cmovae %r12,%rax
   25474:	mov    %r12,0x8(%rcx)
   25478:	mov    %rbx,0x10(%rcx)
   2547c:	mov    %rax,0x18(%rcx)
   25480:	movl   $0x0,(%rcx)
   25486:	add    $0x138,%rsp
   2548d:	pop    %rbx
   2548e:	pop    %r12
   25490:	pop    %r13
   25492:	pop    %r14
   25494:	pop    %r15
   25496:	pop    %rbp
   25497:	ret
   25498:	mov    %r12,%rdi
   2549b:	mov    %r14,%rdx
   2549e:	mov    %r13,%rcx
   254a1:	call   *0x61c91(%rip)        # 87138 <_DYNAMIC+0x628>
   254a7:	jmp    253c8 <meshopt_peers::run+0x908>
   254ac:	mov    $0x4,%eax
   254b1:	mov    %rax,(%rsp)
   254b5:	movq   $0x0,0x38(%rsp)
   254be:	cmp    $0x2,%rbp
   254c2:	jb     2553e <meshopt_peers::run+0xa7e>
   254c4:	lea    -0x1(%rbp),%rdx
   254c8:	lea    -0x2(%rbp),%rsi
   254cc:	mov    %edx,%eax
   254ce:	and    $0x7,%eax
   254d1:	mov    (%rsp),%rcx
   254d5:	cmp    $0x7,%rsi
   254d9:	jb     2551e <meshopt_peers::run+0xa5e>
   254db:	and    $0xfffffffffffffff8,%rdx
   254df:	xorps  %xmm0,%xmm0
   254e2:	mov    (%rsp),%rcx
   254e6:	cs nopw 0x0(%rax,%rax,1)
   254f0:	movups %xmm0,(%rcx)
   254f3:	movups %xmm0,0x10(%rcx)
   254f7:	movups %xmm0,0x20(%rcx)
   254fb:	movups %xmm0,0x30(%rcx)
   254ff:	movups %xmm0,0x40(%rcx)
   25503:	movups %xmm0,0x50(%rcx)
   25507:	movups %xmm0,0x60(%rcx)
   2550b:	movups %xmm0,0x70(%rcx)
   2550f:	sub    $0xffffffffffffff80,%rcx
   25513:	add    $0xfffffffffffffff8,%rdx
   25517:	jne    254f0 <meshopt_peers::run+0xa30>
   25519:	test   %rax,%rax
   2551c:	je     25547 <meshopt_peers::run+0xa87>
   2551e:	xorps  %xmm0,%xmm0
   25521:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   25530:	movups %xmm0,(%rcx)
   25533:	add    $0x10,%rcx
   25537:	dec    %rax
   2553a:	jne    25530 <meshopt_peers::run+0xa70>
   2553c:	jmp    25547 <meshopt_peers::run+0xa87>
   2553e:	mov    (%rsp),%rcx
   25542:	test   %rbp,%rbp
   25545:	je     2554d <meshopt_peers::run+0xa8d>
   25547:	xorps  %xmm0,%xmm0
   2554a:	movups %xmm0,(%rcx)
   2554d:	mov    %rbp,%r12
   25550:	shl    $0x6,%r12
   25554:	mov    %rbp,%r15
   25557:	shl    $0x8,%r15
   2555b:	mov    %r12,%rax
   2555e:	shr    $0x3e,%rax
   25562:	setne  %al
   25565:	cmp    %rbx,%r15
   25568:	seta   %cl
   2556b:	or     %al,%cl
   2556d:	je     25583 <meshopt_peers::run+0xac3>
   2556f:	xor    %edi,%edi
   25571:	mov    %r15,%rsi
   25574:	mov    (%rsp),%r14
   25578:	call   *0x617aa(%rip)        # 86d28 <_DYNAMIC+0x218>
   2557e:	jmp    2587f <meshopt_peers::run+0xdbf>
   25583:	test   %r15,%r15
   25586:	je     25604 <meshopt_peers::run+0xb44>
   25588:	mov    $0x1,%esi
   2558d:	mov    %r15,%rdi
   25590:	call   *0x61a62(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   25596:	mov    %rax,0x30(%rsp)
   2559b:	test   %rax,%rax
   2559e:	je     25867 <meshopt_peers::run+0xda7>
   255a4:	mov    %r12,0x28(%rsp)
   255a9:	jmp    25617 <meshopt_peers::run+0xb57>
   255ab:	sub    $0x8,%rsp
   255af:	mov    $0xc,%r8d
   255b5:	mov    0x8(%rsp),%rbx
   255ba:	mov    %rbx,%rdi
   255bd:	mov    %r12,%rsi
   255c0:	mov    %r15,%rdx
   255c3:	mov    %r13,%rcx
   255c6:	mov    $0x2,%r9d
   255cc:	push   $0x1
   255ce:	call   *0x61b6c(%rip)        # 87140 <_DYNAMIC+0x630>
   255d4:	add    $0x10,%rsp
   255d8:	mov    0x18(%rsp),%rcx
   255dd:	test   %rax,%rax
   255e0:	jne    2546d <meshopt_peers::run+0x9ad>
   255e6:	lea    -0x1b637(%rip),%rdi        # 9fb6 <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0x131>
   255ed:	lea    0x5f9bc(%rip),%rdx        # 84fb0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xd8>
   255f4:	mov    $0x17,%esi
   255f9:	call   *0x61821(%rip)        # 86e20 <_DYNAMIC+0x310>
   255ff:	jmp    2587f <meshopt_peers::run+0xdbf>
   25604:	mov    $0x4,%eax
   25609:	mov    %rax,0x30(%rsp)
   2560e:	movq   $0x0,0x28(%rsp)
   25617:	imul   $0x174,%rbp,%rbx
   2561e:	test   %rbx,%rbx
   25621:	jns    25637 <meshopt_peers::run+0xb77>
   25623:	xor    %r15d,%r15d
   25626:	mov    %r15,%rdi
   25629:	mov    %rbx,%rsi
   2562c:	call   *0x616f6(%rip)        # 86d28 <_DYNAMIC+0x218>
   25632:	jmp    2587f <meshopt_peers::run+0xdbf>
   25637:	je     25661 <meshopt_peers::run+0xba1>
   25639:	mov    $0x1,%r15d
   2563f:	mov    $0x1,%esi
   25644:	mov    %rbx,%rdi
   25647:	call   *0x619ab(%rip)        # 86ff8 <calloc@GLIBC_2.2.5>
   2564d:	test   %rax,%rax
   25650:	je     25626 <meshopt_peers::run+0xb66>
   25652:	mov    0x20(%rsp),%r9
   25657:	mov    0x10(%rsp),%rcx
   2565c:	mov    %rax,%r15
   2565f:	jmp    25671 <meshopt_peers::run+0xbb1>
   25661:	mov    $0x1,%r15d
   25667:	mov    0x20(%rsp),%r9
   2566c:	mov    0x10(%rsp),%rcx
   25671:	xorps  %xmm0,%xmm0
   25674:	mov    (%rsp),%rdi
   25678:	mov    0x30(%rsp),%rsi
   2567d:	mov    %r15,%rdx
   25680:	mov    %r14,%r8
   25683:	push   $0x7c
   25685:	push   $0x40
   25687:	push   $0xc
   25689:	push   0x20(%rsp)
   2568d:	call   *0x61ab5(%rip)        # 87148 <_DYNAMIC+0x638>
   25693:	add    $0x20,%rsp
   25697:	cmp    %rbp,%rax
   2569a:	cmovb  %rax,%rbp
   2569e:	test   %rbp,%rbp
   256a1:	je     256da <meshopt_peers::run+0xc1a>
   256a3:	mov    %rbx,%r14
   256a6:	mov    %rbp,%rbx
   256a9:	shl    $0x4,%rbx
   256ad:	mov    %rbx,%rdi
   256b0:	call   *0x616c2(%rip)        # 86d78 <malloc@GLIBC_2.2.5>
   256b6:	test   %rax,%rax
   256b9:	je     25871 <meshopt_peers::run+0xdb1>
   256bf:	mov    %ebp,%ecx
   256c1:	and    $0x3,%ecx
   256c4:	cmp    $0x4,%rbp
   256c8:	mov    (%rsp),%rdi
   256cc:	jae    256ea <meshopt_peers::run+0xc2a>
   256ce:	xor    %edx,%edx
   256d0:	mov    0x18(%rsp),%r8
   256d5:	mov    %r14,%rbx
   256d8:	jmp    25734 <meshopt_peers::run+0xc74>
   256da:	mov    $0x4,%eax
   256df:	mov    0x18(%rsp),%r8
   256e4:	mov    (%rsp),%rdi
   256e8:	jmp    25761 <meshopt_peers::run+0xca1>
   256ea:	and    %rbp,%r13
   256ed:	mov    $0x30,%esi
   256f2:	xor    %edx,%edx
   256f4:	mov    0x18(%rsp),%r8
   256f9:	mov    %r14,%rbx
   256fc:	movups -0x30(%rdi,%rsi,1),%xmm0
   25701:	movups %xmm0,-0x30(%rax,%rsi,1)
   25706:	movups -0x20(%rdi,%rsi,1),%xmm0
   2570b:	movups %xmm0,-0x20(%rax,%rsi,1)
   25710:	movups -0x10(%rdi,%rsi,1),%xmm0
   25715:	movups %xmm0,-0x10(%rax,%rsi,1)
   2571a:	movups (%rdi,%rsi,1),%xmm0
   2571e:	movups %xmm0,(%rax,%rsi,1)
   25722:	add    $0x4,%rdx
   25726:	add    $0x40,%rsi
   2572a:	cmp    %rdx,%r13
   2572d:	jne    256fc <meshopt_peers::run+0xc3c>
   2572f:	test   %rcx,%rcx
   25732:	je     25761 <meshopt_peers::run+0xca1>
   25734:	shl    $0x4,%rdx
   25738:	mov    %rax,%rsi
   2573b:	add    %rdx,%rsi
   2573e:	add    %rdi,%rdx
   25741:	shl    $0x4,%ecx
   25744:	xor    %r9d,%r9d
   25747:	mov    0x28(%rsp),%r10
   2574c:	movups (%rdx,%r9,1),%xmm0
   25751:	movups %xmm0,(%rsi,%r9,1)
   25756:	add    $0x10,%r9
   2575a:	cmp    %r9,%rcx
   2575d:	jne    2574c <meshopt_peers::run+0xc8c>
   2575f:	jmp    25766 <meshopt_peers::run+0xca6>
   25761:	mov    0x28(%rsp),%r10
   25766:	mov    %r10,0x20(%r8)
   2576a:	mov    0x30(%rsp),%rcx
   2576f:	mov    %rcx,0x28(%r8)
   25773:	mov    %r12,0x30(%r8)
   25777:	mov    %rbx,0x38(%r8)
   2577b:	mov    %r15,0x40(%r8)
   2577f:	mov    %rbx,0x48(%r8)
   25783:	mov    %rbp,0x8(%r8)
   25787:	mov    %rax,0x10(%r8)
   2578b:	mov    %rbp,0x18(%r8)
   2578f:	movl   $0x5,(%r8)
   25796:	cmpq   $0x0,0x38(%rsp)
   2579c:	je     25486 <meshopt_peers::run+0x9c6>
   257a2:	add    $0x138,%rsp
   257a9:	pop    %rbx
   257aa:	pop    %r12
   257ac:	pop    %r13
   257ae:	pop    %r14
   257b0:	pop    %r15
   257b2:	pop    %rbp
   257b3:	jmp    *0x615a7(%rip)        # 86d60 <free@GLIBC_2.2.5>
   257b9:	lea    0x60(%rsp),%rax
   257be:	mov    %rax,0x70(%rsp)
   257c3:	lea    0x4a66(%rip),%rax        # 2a230 <<&str as core::fmt::Display>::fmt>
   257ca:	mov    %rax,0x78(%rsp)
   257cf:	lea    -0x1ed36(%rip),%rdi        # 6aa0 <anon.9b1ebfc14efcaf6f77b453a6f385b165.63.llvm.16074470296022112615+0x117>
   257d6:	lea    0x5f7a3(%rip),%rdx        # 84f80 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0xa8>
   257dd:	lea    0x70(%rsp),%rsi
   257e2:	call   *0x615c0(%rip)        # 86da8 <_DYNAMIC+0x298>
   257e8:	lea    -0x1b89f(%rip),%rdi        # 9f50 <anon.07bf7b68542f29918c2fe01e2698508b.1589.llvm.13623703653819427234+0xcb>
   257ef:	lea    0x5f6fa(%rip),%rdx        # 84ef0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x18>
   257f6:	mov    $0x97,%esi
   257fb:	call   *0x615a7(%rip)        # 86da8 <_DYNAMIC+0x298>
   25801:	jmp    2587f <meshopt_peers::run+0xdbf>
   25803:	mov    $0x4,%edi
   25808:	mov    %rbx,%rsi
   2580b:	call   *0x61517(%rip)        # 86d28 <_DYNAMIC+0x218>
   25811:	mov    $0x1,%edi
   25816:	mov    %r15,%rsi
   25819:	call   *0x61509(%rip)        # 86d28 <_DYNAMIC+0x218>
   2581f:	mov    $0x4,%edi
   25824:	mov    %r15,%rsi
   25827:	call   *0x614fb(%rip)        # 86d28 <_DYNAMIC+0x218>
   2582d:	mov    $0x4,%edi
   25832:	mov    %r12,%rsi
   25835:	call   *0x614ed(%rip)        # 86d28 <_DYNAMIC+0x218>
   2583b:	mov    $0x1,%edi
   25840:	mov    %r12,%rsi
   25843:	call   *0x614df(%rip)        # 86d28 <_DYNAMIC+0x218>
   25849:	jmp    2587f <meshopt_peers::run+0xdbf>
   2584b:	mov    $0x4,%edi
   25850:	mov    %rbx,%rsi
   25853:	call   *0x614cf(%rip)        # 86d28 <_DYNAMIC+0x218>
   25859:	mov    $0x4,%edi
   2585e:	mov    %r15,%rsi
   25861:	call   *0x614c1(%rip)        # 86d28 <_DYNAMIC+0x218>
   25867:	mov    $0x4,%edi
   2586c:	jmp    25571 <meshopt_peers::run+0xab1>
   25871:	mov    $0x4,%edi
   25876:	mov    %rbx,%rsi
   25879:	call   *0x614a9(%rip)        # 86d28 <_DYNAMIC+0x218>
   2587f:	ud2
   25881:	mov    %rax,%rbx
   25884:	test   %r14,%r14
   25887:	je     258e1 <meshopt_peers::run+0xe21>
   25889:	mov    %r15,%rdi
   2588c:	call   *0x614ce(%rip)        # 86d60 <free@GLIBC_2.2.5>
   25892:	jmp    258e1 <meshopt_peers::run+0xe21>
   25894:	mov    %rax,%rbx
   25897:	test   %r14,%r14
   2589a:	je     25910 <meshopt_peers::run+0xe50>
   2589c:	mov    %r15,%rdi
   2589f:	jmp    2590a <meshopt_peers::run+0xe4a>
   258a1:	mov    %rax,%rbx
   258a4:	test   %r12,%r12
   258a7:	je     25910 <meshopt_peers::run+0xe50>
   258a9:	mov    (%rsp),%rdi
   258ad:	jmp    2590a <meshopt_peers::run+0xe4a>
   258af:	mov    %rax,%rbx
   258b2:	cmpq   $0x0,0x48(%rsp)
   258b8:	je     258c5 <meshopt_peers::run+0xe05>
   258ba:	mov    0x50(%rsp),%rdi
   258bf:	call   *0x6149b(%rip)        # 86d60 <free@GLIBC_2.2.5>
   258c5:	test   %r13,%r13
   258c8:	je     25910 <meshopt_peers::run+0xe50>
   258ca:	mov    0x8(%rsp),%rdi
   258cf:	jmp    2590a <meshopt_peers::run+0xe4a>
   258d1:	mov    %rax,%rbx
   258d4:	test   %r15,%r15
   258d7:	je     25910 <meshopt_peers::run+0xe50>
   258d9:	mov    %r13,%rdi
   258dc:	jmp    2590a <meshopt_peers::run+0xe4a>
   258de:	mov    %rax,%rbx
   258e1:	mov    (%rsp),%r14
   258e5:	mov    0x28(%rsp),%rax
   258ea:	test   %rax,%rax
   258ed:	je     258ff <meshopt_peers::run+0xe3f>
   258ef:	mov    0x30(%rsp),%rdi
   258f4:	call   *0x61466(%rip)        # 86d60 <free@GLIBC_2.2.5>
   258fa:	jmp    258ff <meshopt_peers::run+0xe3f>
   258fc:	mov    %rax,%rbx
   258ff:	cmpq   $0x0,0x38(%rsp)
   25905:	je     25910 <meshopt_peers::run+0xe50>
   25907:	mov    %r14,%rdi
   2590a:	call   *0x61450(%rip)        # 86d60 <free@GLIBC_2.2.5>
   25910:	mov    %rbx,%rdi
   25913:	call   83400 <_Unwind_Resume@plt>
