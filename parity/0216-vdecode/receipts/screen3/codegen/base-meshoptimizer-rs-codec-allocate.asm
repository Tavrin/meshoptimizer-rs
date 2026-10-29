
<artifacts>/screen3/driver-base:     file format elf64-x86-64


Disassembly of section .text:

00000000000b2ea0 <meshoptimizer_rs::codec::allocate>:
   b2ea0:	push   %rbp
   b2ea1:	push   %r15
   b2ea3:	push   %r14
   b2ea5:	push   %r13
   b2ea7:	push   %r12
   b2ea9:	push   %rbx
   b2eaa:	sub    $0x468,%rsp
   b2eb1:	mov    %r8,%r14
   b2eb4:	mov    %edx,%r8d
   b2eb7:	mov    0x4a8(%rsp),%r12
   b2ebf:	movb   $0x0,0x365(%r12)
   b2ec8:	movl   $0x0,0x361(%r12)
   b2ed4:	xorps  %xmm0,%xmm0
   b2ed7:	movups %xmm0,0x350(%r12)
   b2ee0:	mov    0x348(%r12),%r15
   b2ee8:	mov    %r15,0x48(%rsp)
   b2eed:	mov    %r15,0x50(%rsp)
   b2ef2:	movzbl %sil,%r13d
   b2ef6:	test   %r13b,%r13b
   b2ef9:	je     b2f30 <meshoptimizer_rs::codec::allocate+0x90>
   b2efb:	cmp    $0x1,%r13d
   b2eff:	jne    b2f4e <meshoptimizer_rs::codec::allocate+0xae>
   b2f01:	cmp    $0x2,%r14
   b2f05:	je     b2f0f <meshoptimizer_rs::codec::allocate+0x6f>
   b2f07:	mov    $0x2,%al
   b2f09:	cmp    $0x4,%r14
   b2f0d:	jne    b2f74 <meshoptimizer_rs::codec::allocate+0xd4>
   b2f0f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b2f19:	imul   %rcx,%rax
   b2f1d:	movabs $0x5555555555555556,%rdx
   b2f27:	cmp    %rdx,%rax
   b2f2a:	jb     b2f5c <meshoptimizer_rs::codec::allocate+0xbc>
   b2f2c:	xor    %eax,%eax
   b2f2e:	jmp    b2f74 <meshoptimizer_rs::codec::allocate+0xd4>
   b2f30:	lea    -0x1(%r14),%rax
   b2f34:	cmp    $0x100,%rax
   b2f3a:	setb   %r10b
   b2f3e:	test   $0x3,%r14b
   b2f42:	sete   %dl
   b2f45:	mov    $0x2,%al
   b2f47:	test   %dl,%r10b
   b2f4a:	jne    b2f5c <meshoptimizer_rs::codec::allocate+0xbc>
   b2f4c:	jmp    b2f74 <meshoptimizer_rs::codec::allocate+0xd4>
   b2f4e:	cmp    $0x4,%r14
   b2f52:	je     b2f5c <meshoptimizer_rs::codec::allocate+0xbc>
   b2f54:	mov    $0x2,%al
   b2f56:	cmp    $0x2,%r14
   b2f5a:	jne    b2f74 <meshoptimizer_rs::codec::allocate+0xd4>
   b2f5c:	mov    %rcx,%rax
   b2f5f:	mul    %r14
   b2f62:	mov    %rax,%rbp
   b2f65:	seto   %al
   b2f68:	test   %rbp,%rbp
   b2f6b:	sets   %dl
   b2f6e:	or     %al,%dl
   b2f70:	mov    $0x8,%al
   b2f72:	je     b2f90 <meshoptimizer_rs::codec::allocate+0xf0>
   b2f74:	mov    %al,0x8(%rdi)
   b2f77:	movq   $0xffffffffffffffff,(%rdi)
   b2f7e:	add    $0x468,%rsp
   b2f85:	pop    %rbx
   b2f86:	pop    %r12
   b2f88:	pop    %r13
   b2f8a:	pop    %r14
   b2f8c:	pop    %r15
   b2f8e:	pop    %rbp
   b2f8f:	ret
   b2f90:	mov    %esi,0x3c(%rsp)
   b2f94:	mov    %rcx,0x10(%rsp)
   b2f99:	mov    %r9,0x30(%rsp)
   b2f9e:	mov    %r8d,0x38(%rsp)
   b2fa3:	mov    %rdi,0x40(%rsp)
   b2fa8:	mov    %r12,%rdi
   b2fab:	mov    %rbp,%rsi
   b2fae:	call   95f10 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b2fb3:	mov    %r15,%rbx
   b2fb6:	cmp    $0xff,%al
   b2fb8:	je     b2fd6 <meshoptimizer_rs::codec::allocate+0x136>
   b2fba:	sub    %rbx,%r15
   b2fbd:	mov    %r15,0x358(%r12)
   b2fc5:	mov    0x40(%rsp),%rcx
   b2fca:	mov    %al,0x8(%rcx)
   b2fcd:	movq   $0xffffffffffffffff,(%rcx)
   b2fd4:	jmp    b2f7e <meshoptimizer_rs::codec::allocate+0xde>
   b2fd6:	mov    0x4a0(%rsp),%rdx
   b2fde:	mov    %r15,%rax
   b2fe1:	mov    %rdx,%rcx
   b2fe4:	sub    %rdx,%rax
   b2fe7:	jb     b3356 <meshoptimizer_rs::codec::allocate+0x4b6>
   b2fed:	mov    %rax,0x48(%rsp)
   b2ff2:	mov    %rax,%rbx
   b2ff5:	mov    %rbp,%r10
   b2ff8:	sub    %rbp,%rbx
   b2ffb:	jb     b335e <meshoptimizer_rs::codec::allocate+0x4be>
   b3001:	mov    %rbx,0x48(%rsp)
   b3006:	cmpb   $0x0,0x38(%rsp)
   b300b:	mov    0x3c(%rsp),%ebp
   b300f:	je     b3025 <meshoptimizer_rs::codec::allocate+0x185>
   b3011:	mov    %r10,%rsi
   b3014:	shr    $0x2,%rsi
   b3018:	mov    %rbx,%rax
   b301b:	sub    %rsi,%rax
   b301e:	jae    b3028 <meshoptimizer_rs::codec::allocate+0x188>
   b3020:	jmp    b3364 <meshoptimizer_rs::codec::allocate+0x4c4>
   b3025:	mov    %rbx,%rax
   b3028:	sub    %rax,%r15
   b302b:	mov    %r15,0x358(%r12)
   b3033:	cmp    $0x2,%bpl
   b3037:	mov    0x40(%rsp),%rbx
   b303c:	je     b3071 <meshoptimizer_rs::codec::allocate+0x1d1>
   b303e:	cmp    $0x1,%r13d
   b3042:	mov    %rcx,%r15
   b3045:	jne    b311d <meshoptimizer_rs::codec::allocate+0x27d>
   b304b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b3055:	mov    0x10(%rsp),%rax
   b305a:	mul    %rcx
   b305d:	shr    $1,%rdx
   b3060:	add    $0x11,%rdx
   b3064:	mov    $0x7,%sil
   b3067:	cmp    %rdx,%r15
   b306a:	jae    b3089 <meshoptimizer_rs::codec::allocate+0x1e9>
   b306c:	jmp    b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3071:	mov    0x10(%rsp),%rax
   b3076:	lea    0x5(%rax),%rdx
   b307a:	mov    %rcx,%r15
   b307d:	mov    $0x7,%sil
   b3080:	cmp    %rdx,%r15
   b3083:	jb     b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3089:	movq   $0x0,0x18(%rsp)
   b3092:	movq   $0x1,0x20(%rsp)
   b309b:	movq   $0x0,0x28(%rsp)
   b30a4:	test   %r10,%r10
   b30a7:	jne    b31ed <meshoptimizer_rs::codec::allocate+0x34d>
   b30ad:	cmp    $0x2,%bpl
   b30b1:	je     b3239 <meshoptimizer_rs::codec::allocate+0x399>
   b30b7:	test   %r13d,%r13d
   b30ba:	jne    b3160 <meshoptimizer_rs::codec::allocate+0x2c0>
   b30c0:	cmp    $0x4,%r14
   b30c4:	sete   %al
   b30c7:	cmp    $0x100000,%r10
   b30ce:	setae  %cl
   b30d1:	test   %cl,%al
   b30d3:	je     b3195 <meshoptimizer_rs::codec::allocate+0x2f5>
   b30d9:	lea    0x60(%rsp),%rdi
   b30de:	mov    $0x400,%edx
   b30e3:	xor    %esi,%esi
   b30e5:	call   *0x735e5(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   b30eb:	lea    0x18(%rsp),%rax
   b30f0:	mov    %rax,0x58(%rsp)
   b30f5:	movq   $0x0,0x460(%rsp)
   b3101:	lea    0x58(%rsp),%rdi
   b3106:	mov    0x10(%rsp),%rsi
   b310b:	mov    0x30(%rsp),%rdx
   b3110:	mov    %r15,%rcx
   b3113:	call   73dd0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b3118:	jmp    b3253 <meshoptimizer_rs::codec::allocate+0x3b3>
   b311d:	mov    $0x7,%sil
   b3120:	test   %r15,%r15
   b3123:	je     b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3129:	mov    0x30(%rsp),%rax
   b312e:	movzbl (%rax),%ecx
   b3131:	mov    %ecx,%eax
   b3133:	and    $0xf0,%al
   b3135:	cmp    $0xa0,%al
   b3137:	jne    b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b313d:	and    $0xf,%cl
   b3140:	mov    $0x5,%sil
   b3143:	cmp    $0x1,%cl
   b3146:	ja     b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b314c:	mov    0x10(%rsp),%rdi
   b3151:	cmp    $0x21,%rdi
   b3155:	jae    b31b4 <meshoptimizer_rs::codec::allocate+0x314>
   b3157:	xor    %eax,%eax
   b3159:	xor    %esi,%esi
   b315b:	jmp    b32c3 <meshoptimizer_rs::codec::allocate+0x423>
   b3160:	lea    0x18(%rsp),%rdi
   b3165:	mov    %r10,%rsi
   b3168:	call   8f690 <<alloc::vec::Vec<u8>>::resize>
   b316d:	mov    0x20(%rsp),%rsi
   b3172:	mov    0x28(%rsp),%rdx
   b3177:	mov    %r15,(%rsp)
   b317b:	mov    %r13d,%edi
   b317e:	mov    0x10(%rsp),%rcx
   b3183:	mov    %r14,%r8
   b3186:	mov    0x30(%rsp),%r9
   b318b:	call   adbc0 <meshoptimizer_rs::codec::raw>
   b3190:	jmp    b3253 <meshoptimizer_rs::codec::allocate+0x3b3>
   b3195:	lea    0x18(%rsp),%rdi
   b319a:	mov    0x10(%rsp),%rsi
   b319f:	mov    %r14,%rdx
   b31a2:	mov    0x30(%rsp),%rcx
   b31a7:	mov    %r15,%r8
   b31aa:	call   73130 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   b31af:	jmp    b3253 <meshoptimizer_rs::codec::allocate+0x3b3>
   b31b4:	mov    $0x2000,%ax
   b31b8:	xor    %edx,%edx
   b31ba:	div    %r14w
   b31be:	and    $0x3ff0,%eax
   b31c3:	cmp    $0x100,%eax
   b31c8:	mov    $0x100,%esi
   b31cd:	cmovb  %eax,%esi
   b31d0:	mov    %rdi,%rax
   b31d3:	shr    $0x20,%rax
   b31d7:	je     b32ae <meshoptimizer_rs::codec::allocate+0x40e>
   b31dd:	mov    %rdi,%rax
   b31e0:	xor    %edx,%edx
   b31e2:	div    %rsi
   b31e5:	mov    %rdx,%rdi
   b31e8:	jmp    b32b6 <meshoptimizer_rs::codec::allocate+0x416>
   b31ed:	lea    0x58(%rsp),%rdi
   b31f2:	mov    $0x1,%edx
   b31f7:	mov    $0x1,%r8d
   b31fd:	mov    $0x1,%r9d
   b3203:	xor    %esi,%esi
   b3205:	mov    %r10,%rcx
   b3208:	mov    %r10,%r12
   b320b:	call   90180 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b3210:	cmpb   $0x0,0x58(%rsp)
   b3215:	je     b321d <meshoptimizer_rs::codec::allocate+0x37d>
   b3217:	movb   $0xa,0x8(%rbx)
   b321b:	jmp    b3275 <meshoptimizer_rs::codec::allocate+0x3d5>
   b321d:	mov    0x60(%rsp),%rax
   b3222:	mov    %rax,0x20(%rsp)
   b3227:	mov    %r12,0x18(%rsp)
   b322c:	mov    %r12,%r10
   b322f:	cmp    $0x2,%bpl
   b3233:	jne    b30b7 <meshoptimizer_rs::codec::allocate+0x217>
   b3239:	lea    0x18(%rsp),%rdi
   b323e:	mov    0x10(%rsp),%rsi
   b3243:	mov    %r14,%rdx
   b3246:	mov    0x30(%rsp),%rcx
   b324b:	mov    %r15,%r8
   b324e:	call   d1fa0 <meshoptimizer_rs::codec::index::sequence_append>
   b3253:	cmp    $0xff,%al
   b3255:	jne    b3272 <meshoptimizer_rs::codec::allocate+0x3d2>
   b3257:	mov    0x20(%rsp),%rsi
   b325c:	mov    0x28(%rsp),%rdx
   b3261:	movzbl 0x38(%rsp),%edi
   b3266:	mov    %r14,%rcx
   b3269:	call   af850 <meshoptimizer_rs::codec::apply>
   b326e:	cmp    $0xff,%al
   b3270:	je     b3298 <meshoptimizer_rs::codec::allocate+0x3f8>
   b3272:	mov    %al,0x8(%rbx)
   b3275:	movq   $0xffffffffffffffff,(%rbx)
   b327c:	cmpq   $0x0,0x18(%rsp)
   b3282:	je     b2f7e <meshoptimizer_rs::codec::allocate+0xde>
   b3288:	mov    0x20(%rsp),%rdi
   b328d:	call   *0x7341d(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   b3293:	jmp    b2f7e <meshoptimizer_rs::codec::allocate+0xde>
   b3298:	mov    0x28(%rsp),%rax
   b329d:	mov    %rax,0x10(%rbx)
   b32a1:	movups 0x18(%rsp),%xmm0
   b32a6:	movups %xmm0,(%rbx)
   b32a9:	jmp    b2f7e <meshoptimizer_rs::codec::allocate+0xde>
   b32ae:	mov    %edi,%eax
   b32b0:	xor    %edx,%edx
   b32b2:	div    %esi
   b32b4:	mov    %edx,%edi
   b32b6:	shr    $0x4,%esi
   b32b9:	add    $0x3,%esi
   b32bc:	shr    $0x2,%esi
   b32bf:	imul   %rax,%rsi
   b32c3:	test   %cl,%cl
   b32c5:	je     b32f1 <meshoptimizer_rs::codec::allocate+0x451>
   b32c7:	cmp    $0x1,%rdi
   b32cb:	sbb    $0xffffffffffffffff,%rax
   b32cf:	mov    %r14,%rcx
   b32d2:	shr    $0x2,%rcx
   b32d6:	mul    %rcx
   b32d9:	jo     b3374 <meshoptimizer_rs::codec::allocate+0x4d4>
   b32df:	add    %r14,%rcx
   b32e2:	cmp    $0x19,%rcx
   b32e6:	mov    $0x18,%edx
   b32eb:	cmovae %rcx,%rdx
   b32ef:	jmp    b3329 <meshoptimizer_rs::codec::allocate+0x489>
   b32f1:	test   %rdi,%rdi
   b32f4:	je     b3312 <meshoptimizer_rs::codec::allocate+0x472>
   b32f6:	mov    %rdi,%rax
   b32f9:	shr    $0x4,%rax
   b32fd:	and    $0xf,%edi
   b3300:	cmp    $0x1,%rdi
   b3304:	sbb    $0xffffffffffffffff,%rax
   b3308:	add    $0x3,%rax
   b330c:	shr    $0x2,%rax
   b3310:	jmp    b3314 <meshoptimizer_rs::codec::allocate+0x474>
   b3312:	xor    %eax,%eax
   b3314:	add    %rsi,%rax
   b3317:	mul    %r14
   b331a:	jo     b3374 <meshoptimizer_rs::codec::allocate+0x4d4>
   b331c:	cmp    $0x21,%r14
   b3320:	mov    $0x20,%edx
   b3325:	cmovae %r14,%rdx
   b3329:	mov    $0x8,%sil
   b332c:	add    %rax,%rdx
   b332f:	jb     b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3331:	cmp    $0xffffffffffffffff,%rdx
   b3335:	je     b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3337:	inc    %rdx
   b333a:	mov    $0x7,%sil
   b333d:	cmp    %rdx,%r15
   b3340:	jae    b3089 <meshoptimizer_rs::codec::allocate+0x1e9>
   b3346:	mov    %sil,0x8(%rbx)
   b334a:	movq   $0xffffffffffffffff,(%rbx)
   b3351:	jmp    b2f7e <meshoptimizer_rs::codec::allocate+0xde>
   b3356:	mov    %rcx,%rsi
   b3359:	mov    %r15,%rbx
   b335c:	jmp    b3364 <meshoptimizer_rs::codec::allocate+0x4c4>
   b335e:	mov    %r10,%rsi
   b3361:	mov    %rax,%rbx
   b3364:	lea    0x48(%rsp),%rdi
   b3369:	call   *0x73809(%rip)        # 126b78 <_DYNAMIC+0x6d0>
   b336f:	jmp    b2fba <meshoptimizer_rs::codec::allocate+0x11a>
   b3374:	mov    $0x8,%sil
   b3377:	jmp    b3346 <meshoptimizer_rs::codec::allocate+0x4a6>
   b3379:	mov    %rax,%rbx
   b337c:	cmpq   $0x0,0x18(%rsp)
   b3382:	je     b338f <meshoptimizer_rs::codec::allocate+0x4ef>
   b3384:	mov    0x20(%rsp),%rdi
   b3389:	call   *0x73321(%rip)        # 1266b0 <free@GLIBC_2.2.5>
   b338f:	mov    %rbx,%rdi
   b3392:	call   11d740 <_Unwind_Resume@plt>
