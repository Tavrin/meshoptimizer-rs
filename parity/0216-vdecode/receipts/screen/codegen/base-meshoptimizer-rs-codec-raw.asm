
<artifacts>/screen/driver-base:     file format elf64-x86-64


Disassembly of section .text:

00000000000adbc0 <meshoptimizer_rs::codec::raw>:
   adbc0:	push   %rbp
   adbc1:	push   %r15
   adbc3:	push   %r14
   adbc5:	push   %r13
   adbc7:	push   %r12
   adbc9:	push   %rbx
   adbca:	sub    $0x5f8,%rsp
   adbd1:	mov    %r9,%r14
   adbd4:	mov    %r8,%rbp
   adbd7:	mov    %rcx,%r13
   adbda:	mov    %rdx,%r12
   adbdd:	mov    %rsi,%r15
   adbe0:	mov    0x630(%rsp),%rbx
   adbe8:	test   %dil,%dil
   adbeb:	je     add54 <meshoptimizer_rs::codec::raw+0x194>
   adbf1:	movzbl %dil,%eax
   adbf5:	cmp    $0x1,%eax
   adbf8:	jne    ade16 <meshoptimizer_rs::codec::raw+0x256>
   adbfe:	movabs $0xaaaaaaaaaaaaaaab,%rdi
   adc08:	mov    %r13,%rax
   adc0b:	mul    %rdi
   adc0e:	shr    $1,%rdx
   adc11:	lea    (%rdx,%rdx,2),%rcx
   adc15:	mov    %r13,%rax
   adc18:	sub    %rcx,%rax
   adc1b:	cmp    $0x2,%rbp
   adc1f:	jne    ade81 <meshoptimizer_rs::codec::raw+0x2c1>
   adc25:	test   %rax,%rax
   adc28:	jne    ade8a <meshoptimizer_rs::codec::raw+0x2ca>
   adc2e:	mov    %r13,%rax
   adc31:	mul    %rdi
   adc34:	shr    $1,%rdx
   adc37:	lea    0x11(%rdx),%rcx
   adc3b:	mov    $0x7,%al
   adc3d:	cmp    %rcx,%rbx
   adc40:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adc46:	movzbl (%r14),%ecx
   adc4a:	mov    %ecx,%esi
   adc4c:	and    $0xf0,%sil
   adc50:	cmp    $0xe0,%sil
   adc54:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adc5a:	and    $0xf,%cl
   adc5d:	mov    $0x5,%al
   adc5f:	cmp    $0x1,%cl
   adc62:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adc68:	lea    0x1(%rdx),%rsi
   adc6c:	pcmpeqd %xmm0,%xmm0
   adc70:	movdqa %xmm0,0x260(%rsp)
   adc79:	movdqa %xmm0,0x250(%rsp)
   adc82:	movdqa %xmm0,0x240(%rsp)
   adc8b:	movdqa %xmm0,0x230(%rsp)
   adc94:	movdqa %xmm0,0x220(%rsp)
   adc9d:	movdqa %xmm0,0x210(%rsp)
   adca6:	movdqa %xmm0,0x200(%rsp)
   adcaf:	movdqa %xmm0,0x1f0(%rsp)
   adcb8:	movdqa %xmm0,0x120(%rsp)
   adcc1:	movdqa %xmm0,0x110(%rsp)
   adcca:	movdqa %xmm0,0x100(%rsp)
   adcd3:	movdqa %xmm0,0xf0(%rsp)
   adcdc:	xor    %r8d,%r8d
   adcdf:	cmp    $0x1,%cl
   adce2:	setne  %r9b
   adce6:	lea    -0x10(%rbx),%rax
   adcea:	mov    %rax,0x28(%rsp)
   adcef:	mov    %r12,%rax
   adcf2:	mov    %rdx,%rcx
   adcf5:	mul    %rdi
   adcf8:	shr    $0x2,%rdx
   adcfc:	cmp    %rcx,%rdx
   adcff:	cmovae %rcx,%rdx
   add03:	mov    %rdx,0x38(%rsp)
   add08:	test   %rdx,%rdx
   add0b:	je     af70f <meshoptimizer_rs::codec::raw+0x1b4f>
   add11:	mov    %r9b,%r8b
   add14:	lea    0xd(,%r8,2),%rax
   add1c:	mov    %rax,0x58(%rsp)
   add21:	mov    %r14,0x30(%rsp)
   add26:	lea    (%r14,%rbx,1),%rax
   add2a:	add    $0xfffffffffffffff0,%rax
   add2e:	mov    %rax,0x50(%rsp)
   add33:	add    $0x4,%r15
   add37:	xor    %r8d,%r8d
   add3a:	xor    %r10d,%r10d
   add3d:	movl   $0x0,0x20(%rsp)
   add45:	xor    %r11d,%r11d
   add48:	xor    %edi,%edi
   add4a:	mov    0x28(%rsp),%r9
   add4f:	jmp    ae216 <meshoptimizer_rs::codec::raw+0x656>
   add54:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   add59:	cmp    $0x2,%al
   add5b:	jb     addc4 <meshoptimizer_rs::codec::raw+0x204>
   add5d:	cmp    $0x20,%rbp
   add61:	lea    -0x34658(%rip),%rax        # 79710 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 32, [u8]>>
   add68:	lea    -0x35b3f(%rip),%rcx        # 78230 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 0, [u8]>>
   add6f:	cmove  %rax,%rcx
   add73:	lea    -0x2ec1a(%rip),%rax        # 7f160 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 32, [u8]>>
   add7a:	lea    -0x304c1(%rip),%rdx        # 7d8c0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 0, [u8]>>
   add81:	cmove  %rax,%rdx
   add85:	cmp    $0x4,%rbp
   add89:	lea    -0x31ff0(%rip),%rax        # 7bda0 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<1024, 4, [u8]>>
   add90:	cmovne %rcx,%rax
   add94:	lea    -0x2c19b(%rip),%r10        # 81c00 <meshoptimizer_rs::codec::simd::x86::vertex_kernel::<128, 4, [u8]>>
   add9b:	cmovne %rdx,%r10
   add9f:	cmp    $0x21,%r13
   adda3:	cmovae %rax,%r10
   adda7:	mov    %r15,%rdi
   addaa:	mov    %r12,%rsi
   addad:	mov    %r13,%rdx
   addb0:	mov    %rbp,%rcx
   addb3:	mov    %r14,%r8
   addb6:	mov    %rbx,%r9
   addb9:	call   *%r10
   addbc:	cmp    $0xfe,%al
   addbe:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   addc4:	mov    $0x7,%al
   addc6:	test   %rbx,%rbx
   addc9:	je     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   addcf:	movzbl (%r14),%edi
   addd3:	mov    %edi,%ecx
   addd5:	and    $0xf0,%cl
   addd8:	cmp    $0xa0,%cl
   adddb:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adde1:	and    $0xf,%dil
   adde5:	mov    $0x5,%al
   adde7:	cmp    $0x1,%dil
   addeb:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   addf1:	test   %dil,%dil
   addf4:	je     aed13 <meshoptimizer_rs::codec::raw+0x1153>
   addfa:	mov    %rbp,%rcx
   addfd:	shr    $0x2,%rcx
   ade01:	add    %rbp,%rcx
   ade04:	cmp    $0x19,%rcx
   ade08:	mov    $0x18,%eax
   ade0d:	cmovae %rcx,%rax
   ade11:	jmp    aed23 <meshoptimizer_rs::codec::raw+0x1163>
   ade16:	add    $0x5,%r13
   ade1a:	mov    $0x7,%al
   ade1c:	cmp    $0x2,%rbp
   ade20:	jne    ade91 <meshoptimizer_rs::codec::raw+0x2d1>
   ade22:	cmp    %r13,%rbx
   ade25:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ade2b:	test   %rbx,%rbx
   ade2e:	je     af7c4 <meshoptimizer_rs::codec::raw+0x1c04>
   ade34:	movzbl (%r14),%ecx
   ade38:	mov    %ecx,%edx
   ade3a:	and    $0xf0,%dl
   ade3d:	cmp    $0xd0,%dl
   ade40:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ade46:	mov    $0x5,%al
   ade48:	test   $0xe,%cl
   ade4b:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ade51:	dec    %rbx
   ade54:	movq   $0x0,0x1f0(%rsp)
   ade60:	movabs $0x7ffffffffffffffc,%rax
   ade6a:	or     $0x2,%rax
   ade6e:	and    %rax,%r12
   ade71:	je     ae174 <meshoptimizer_rs::codec::raw+0x5b4>
   ade77:	inc    %r14
   ade7a:	xor    %eax,%eax
   ade7c:	jmp    adf26 <meshoptimizer_rs::codec::raw+0x366>
   ade81:	test   %rax,%rax
   ade84:	je     ae057 <meshoptimizer_rs::codec::raw+0x497>
   ade8a:	xor    %eax,%eax
   ade8c:	jmp    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ade91:	cmp    %r13,%rbx
   ade94:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ade9a:	test   %rbx,%rbx
   ade9d:	je     af7c4 <meshoptimizer_rs::codec::raw+0x1c04>
   adea3:	movzbl (%r14),%ecx
   adea7:	mov    %ecx,%edx
   adea9:	and    $0xf0,%dl
   adeac:	cmp    $0xd0,%dl
   adeaf:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adeb5:	mov    $0x5,%al
   adeb7:	test   $0xe,%cl
   adeba:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   adec0:	dec    %rbx
   adec3:	movq   $0x0,0x1f0(%rsp)
   adecf:	movabs $0x7ffffffffffffffc,%rax
   aded9:	and    %rax,%r12
   adedc:	je     ae174 <meshoptimizer_rs::codec::raw+0x5b4>
   adee2:	inc    %r14
   adee5:	xor    %eax,%eax
   adee7:	jmp    adfdb <meshoptimizer_rs::codec::raw+0x41b>
   adeec:	dec    %rbx
   adeef:	inc    %r14
   adef2:	mov    %edx,%ecx
   adef4:	mov    %ecx,%edx
   adef6:	shr    $0x2,%edx
   adef9:	mov    %ecx,%esi
   adefb:	shl    $0x1e,%esi
   adefe:	sar    $0x1f,%esi
   adf01:	xor    %edx,%esi
   adf03:	and    $0x1,%ecx
   adf06:	add    0x1f0(%rsp,%rcx,4),%esi
   adf0d:	mov    %esi,0x1f0(%rsp,%rcx,4)
   adf14:	mov    %si,(%r15,%rax,1)
   adf19:	add    $0x2,%rax
   adf1d:	cmp    %rax,%r12
   adf20:	je     ae174 <meshoptimizer_rs::codec::raw+0x5b4>
   adf26:	cmp    $0x5,%rbx
   adf2a:	jb     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   adf30:	movzbl (%r14),%edx
   adf34:	test   %dl,%dl
   adf36:	jns    adeec <meshoptimizer_rs::codec::raw+0x32c>
   adf38:	and    $0x7f,%edx
   adf3b:	movzbl 0x1(%r14),%ecx
   adf40:	mov    %ecx,%esi
   adf42:	and    $0x7f,%ecx
   adf45:	shl    $0x7,%ecx
   adf48:	or     %edx,%ecx
   adf4a:	mov    $0x2,%edx
   adf4f:	test   %sil,%sil
   adf52:	jns    adf97 <meshoptimizer_rs::codec::raw+0x3d7>
   adf54:	movzbl 0x2(%r14),%edx
   adf59:	mov    %edx,%esi
   adf5b:	and    $0x7f,%edx
   adf5e:	shl    $0xe,%edx
   adf61:	or     %edx,%ecx
   adf63:	mov    $0x3,%edx
   adf68:	test   %sil,%sil
   adf6b:	jns    adf97 <meshoptimizer_rs::codec::raw+0x3d7>
   adf6d:	movzbl 0x3(%r14),%edx
   adf72:	mov    %edx,%esi
   adf74:	and    $0x7f,%edx
   adf77:	shl    $0x15,%edx
   adf7a:	or     %edx,%ecx
   adf7c:	mov    $0x4,%edx
   adf81:	test   %sil,%sil
   adf84:	jns    adf97 <meshoptimizer_rs::codec::raw+0x3d7>
   adf86:	movzbl 0x4(%r14),%esi
   adf8b:	shl    $0x1c,%esi
   adf8e:	or     %ecx,%esi
   adf90:	mov    $0x5,%edx
   adf95:	mov    %esi,%ecx
   adf97:	sub    %rdx,%rbx
   adf9a:	add    %rdx,%r14
   adf9d:	jmp    adef4 <meshoptimizer_rs::codec::raw+0x334>
   adfa2:	dec    %rbx
   adfa5:	inc    %r14
   adfa8:	mov    %edx,%ecx
   adfaa:	mov    %ecx,%edx
   adfac:	shr    $0x2,%edx
   adfaf:	mov    %ecx,%esi
   adfb1:	shl    $0x1e,%esi
   adfb4:	sar    $0x1f,%esi
   adfb7:	xor    %edx,%esi
   adfb9:	and    $0x1,%ecx
   adfbc:	add    0x1f0(%rsp,%rcx,4),%esi
   adfc3:	mov    %esi,0x1f0(%rsp,%rcx,4)
   adfca:	mov    %esi,(%r15,%rax,1)
   adfce:	add    $0x4,%rax
   adfd2:	cmp    %rax,%r12
   adfd5:	je     ae174 <meshoptimizer_rs::codec::raw+0x5b4>
   adfdb:	cmp    $0x5,%rbx
   adfdf:	jb     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   adfe5:	movzbl (%r14),%edx
   adfe9:	test   %dl,%dl
   adfeb:	jns    adfa2 <meshoptimizer_rs::codec::raw+0x3e2>
   adfed:	and    $0x7f,%edx
   adff0:	movzbl 0x1(%r14),%ecx
   adff5:	mov    %ecx,%esi
   adff7:	and    $0x7f,%ecx
   adffa:	shl    $0x7,%ecx
   adffd:	or     %edx,%ecx
   adfff:	mov    $0x2,%edx
   ae004:	test   %sil,%sil
   ae007:	jns    ae04c <meshoptimizer_rs::codec::raw+0x48c>
   ae009:	movzbl 0x2(%r14),%edx
   ae00e:	mov    %edx,%esi
   ae010:	and    $0x7f,%edx
   ae013:	shl    $0xe,%edx
   ae016:	or     %edx,%ecx
   ae018:	mov    $0x3,%edx
   ae01d:	test   %sil,%sil
   ae020:	jns    ae04c <meshoptimizer_rs::codec::raw+0x48c>
   ae022:	movzbl 0x3(%r14),%edx
   ae027:	mov    %edx,%esi
   ae029:	and    $0x7f,%edx
   ae02c:	shl    $0x15,%edx
   ae02f:	or     %edx,%ecx
   ae031:	mov    $0x4,%edx
   ae036:	test   %sil,%sil
   ae039:	jns    ae04c <meshoptimizer_rs::codec::raw+0x48c>
   ae03b:	movzbl 0x4(%r14),%esi
   ae040:	shl    $0x1c,%esi
   ae043:	or     %ecx,%esi
   ae045:	mov    $0x5,%edx
   ae04a:	mov    %esi,%ecx
   ae04c:	sub    %rdx,%rbx
   ae04f:	add    %rdx,%r14
   ae052:	jmp    adfaa <meshoptimizer_rs::codec::raw+0x3ea>
   ae057:	lea    0x11(%rdx),%rcx
   ae05b:	mov    $0x7,%al
   ae05d:	cmp    %rcx,%rbx
   ae060:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ae066:	movzbl (%r14),%ecx
   ae06a:	mov    %ecx,%esi
   ae06c:	and    $0xf0,%sil
   ae070:	cmp    $0xe0,%sil
   ae074:	jne    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ae07a:	and    $0xf,%cl
   ae07d:	mov    $0x5,%al
   ae07f:	cmp    $0x1,%cl
   ae082:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   ae088:	lea    0x1(%rdx),%rsi
   ae08c:	pcmpeqd %xmm0,%xmm0
   ae090:	movdqa %xmm0,0x260(%rsp)
   ae099:	movdqa %xmm0,0x250(%rsp)
   ae0a2:	movdqa %xmm0,0x240(%rsp)
   ae0ab:	movdqa %xmm0,0x230(%rsp)
   ae0b4:	movdqa %xmm0,0x220(%rsp)
   ae0bd:	movdqa %xmm0,0x210(%rsp)
   ae0c6:	movdqa %xmm0,0x200(%rsp)
   ae0cf:	movdqa %xmm0,0x1f0(%rsp)
   ae0d8:	movdqa %xmm0,0x120(%rsp)
   ae0e1:	movdqa %xmm0,0x110(%rsp)
   ae0ea:	movdqa %xmm0,0x100(%rsp)
   ae0f3:	movdqa %xmm0,0xf0(%rsp)
   ae0fc:	xor    %r8d,%r8d
   ae0ff:	cmp    $0x1,%cl
   ae102:	setne  %r9b
   ae106:	lea    -0x10(%rbx),%rax
   ae10a:	mov    %rax,0x28(%rsp)
   ae10f:	mov    %r12,%rax
   ae112:	mov    %rdx,%rcx
   ae115:	mul    %rdi
   ae118:	shr    $0x3,%rdx
   ae11c:	cmp    %rcx,%rdx
   ae11f:	cmovae %rcx,%rdx
   ae123:	mov    %rdx,0x38(%rsp)
   ae128:	test   %rdx,%rdx
   ae12b:	je     af70f <meshoptimizer_rs::codec::raw+0x1b4f>
   ae131:	mov    %r9b,%r8b
   ae134:	lea    0xd(,%r8,2),%rax
   ae13c:	mov    %rax,0x58(%rsp)
   ae141:	mov    %r14,0x30(%rsp)
   ae146:	lea    (%r14,%rbx,1),%rax
   ae14a:	add    $0xfffffffffffffff0,%rax
   ae14e:	mov    %rax,0x50(%rsp)
   ae153:	add    $0x8,%r15
   ae157:	xor    %r8d,%r8d
   ae15a:	xor    %r10d,%r10d
   ae15d:	movl   $0x0,0x20(%rsp)
   ae165:	xor    %r11d,%r11d
   ae168:	xor    %edi,%edi
   ae16a:	mov    0x28(%rsp),%r9
   ae16f:	jmp    ae7de <meshoptimizer_rs::codec::raw+0xc1e>
   ae174:	cmp    $0x4,%rbx
   ae178:	sete   %al
   ae17b:	jmp    af709 <meshoptimizer_rs::codec::raw+0x1b49>
   ae180:	mov    %bpl,%r14b
   ae183:	add    %r14d,%r11d
   ae186:	mov    %ebx,0xf0(%rsp,%r10,4)
   ae18e:	mov    %r11d,%ebp
   ae191:	not    %eax
   ae193:	shr    $0x4,%eax
   ae196:	add    %r8d,%eax
   ae199:	and    $0xf,%eax
   ae19c:	mov    0x1f0(%rsp,%rax,8),%r11d
   ae1a4:	mov    0x1f4(%rsp,%rax,8),%r15d
   ae1ac:	add    %r14,%r10
   ae1af:	mov    $0x2,%eax
   ae1b4:	mov    $0x1,%r12d
   ae1ba:	mov    %r8,%r13
   ae1bd:	inc    %rdi
   ae1c0:	mov    %ebx,0x1f0(%rsp,%r13,8)
   ae1c8:	mov    %r15d,0x1f4(%rsp,%r13,8)
   ae1d0:	add    %r8d,%r12d
   ae1d3:	and    $0xf,%r12d
   ae1d7:	mov    %r11d,0x1f0(%rsp,%r12,8)
   ae1df:	mov    %ebx,0x1f4(%rsp,%r12,8)
   ae1e7:	add    %r8d,%eax
   ae1ea:	and    $0xf,%eax
   ae1ed:	and    $0xf,%r10d
   ae1f1:	mov    %r11w,-0x4(%rdx)
   ae1f6:	mov    %r15w,-0x2(%rdx)
   ae1fb:	mov    %rdx,%r15
   ae1fe:	mov    %bx,(%rdx)
   ae201:	add    $0x6,%r15
   ae205:	mov    %rax,%r8
   ae208:	mov    %ebp,%r11d
   ae20b:	cmp    %rdi,0x38(%rsp)
   ae210:	je     af714 <meshoptimizer_rs::codec::raw+0x1b54>
   ae216:	cmp    %r9,%rsi
   ae219:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae21f:	mov    %r15,%rdx
   ae222:	mov    0x30(%rsp),%r14
   ae227:	movzbl 0x1(%r14,%rdi,1),%eax
   ae22d:	cmp    $0xf0,%al
   ae22f:	jae    ae269 <meshoptimizer_rs::codec::raw+0x6a9>
   ae231:	mov    %eax,%r15d
   ae234:	and    $0xf,%r15d
   ae238:	cmp    %r15d,0x58(%rsp)
   ae23d:	jbe    ae2a3 <meshoptimizer_rs::codec::raw+0x6e3>
   ae23f:	xor    %r14d,%r14d
   ae242:	mov    %r11d,%ebx
   ae245:	test   %r15b,%r15b
   ae248:	sete   %bpl
   ae24c:	je     ae180 <meshoptimizer_rs::codec::raw+0x5c0>
   ae252:	not    %r15d
   ae255:	add    %r10d,%r15d
   ae258:	and    $0xf,%r15d
   ae25c:	mov    0xf0(%rsp,%r15,4),%ebx
   ae264:	jmp    ae180 <meshoptimizer_rs::codec::raw+0x5c0>
   ae269:	cmp    $0xfe,%al
   ae26b:	jae    ae2c5 <meshoptimizer_rs::codec::raw+0x705>
   ae26d:	and    $0xf,%eax
   ae270:	mov    0x50(%rsp),%rcx
   ae275:	movzbl (%rcx,%rax,1),%r13d
   ae27a:	lea    0x1(%r11),%r15d
   ae27e:	xor    %eax,%eax
   ae280:	mov    %r13d,%ecx
   ae283:	shr    $0x4,%ecx
   ae286:	sete   %r12b
   ae28a:	je     ae308 <meshoptimizer_rs::codec::raw+0x748>
   ae28c:	mov    %r10d,%ebx
   ae28f:	sub    %ecx,%ebx
   ae291:	and    $0xf,%ebx
   ae294:	mov    %r15d,%ebp
   ae297:	mov    0xf0(%rsp,%rbx,4),%ecx
   ae29e:	mov    %ecx,%r15d
   ae2a1:	jmp    ae30c <meshoptimizer_rs::codec::raw+0x74c>
   ae2a3:	cmp    $0xf,%r15b
   ae2a7:	jne    ae388 <meshoptimizer_rs::codec::raw+0x7c8>
   ae2ad:	movzbl (%r14,%rsi,1),%ebp
   ae2b2:	test   %bpl,%bpl
   ae2b5:	js     ae3f6 <meshoptimizer_rs::codec::raw+0x836>
   ae2bb:	inc    %rsi
   ae2be:	mov    %ebp,%ebx
   ae2c0:	jmp    ae58c <meshoptimizer_rs::codec::raw+0x9cc>
   ae2c5:	movzbl (%r14,%rsi,1),%r14d
   ae2ca:	xor    %r15d,%r15d
   ae2cd:	cmp    $0xfe,%eax
   ae2d2:	sete   %r15b
   ae2d6:	test   %r14d,%r14d
   ae2d9:	cmove  %r14d,%r11d
   ae2dd:	add    %r11d,%r15d
   ae2e0:	mov    %r14d,%r12d
   ae2e3:	shr    $0x4,%r12d
   ae2e7:	je     ae3a3 <meshoptimizer_rs::codec::raw+0x7e3>
   ae2ed:	mov    %r10d,%ecx
   ae2f0:	sub    %r12d,%ecx
   ae2f3:	and    $0xf,%ecx
   ae2f6:	mov    %r15d,%ebx
   ae2f9:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae300:	mov    %ecx,%r15d
   ae303:	jmp    ae3a7 <meshoptimizer_rs::codec::raw+0x7e7>
   ae308:	lea    0x2(%r11),%ebp
   ae30c:	xor    %r14d,%r14d
   ae30f:	mov    %ebp,%ebx
   ae311:	test   $0xf,%r13b
   ae315:	sete   %cl
   ae318:	je     ae32a <meshoptimizer_rs::codec::raw+0x76a>
   ae31a:	mov    %r10d,%ebx
   ae31d:	sub    %r13d,%ebx
   ae320:	and    $0xf,%ebx
   ae323:	mov    0xf0(%rsp,%rbx,4),%ebx
   ae32a:	mov    %cl,%r14b
   ae32d:	add    %r14d,%ebp
   ae330:	mov    %r11d,0xf0(%rsp,%r10,4)
   ae338:	lea    0x1(%r10),%rcx
   ae33c:	and    $0xf,%ecx
   ae33f:	mov    %r15d,0xf0(%rsp,%rcx,4)
   ae347:	mov    %r12b,%al
   ae34a:	add    %r10,%rax
   ae34d:	inc    %rax
   ae350:	add    %rax,%r14
   ae353:	and    $0xf,%eax
   ae356:	mov    %ebx,0xf0(%rsp,%rax,4)
   ae35d:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ae365:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ae36d:	lea    0x1(%r8),%r13d
   ae371:	and    $0xf,%r13d
   ae375:	mov    $0x3,%eax
   ae37a:	mov    $0x2,%r12d
   ae380:	mov    %r14,%r10
   ae383:	jmp    ae1bd <meshoptimizer_rs::codec::raw+0x5fd>
   ae388:	movzbl %r15b,%ecx
   ae38c:	xor    $0x3,%r15b
   ae390:	movzbl %r15b,%ebx
   ae394:	mov    0x20(%rsp),%ebp
   ae398:	add    %ecx,%ebp
   ae39a:	sub    %ebx,%ebp
   ae39c:	mov    %ebp,%ebx
   ae39e:	jmp    ae59d <meshoptimizer_rs::codec::raw+0x9dd>
   ae3a3:	lea    0x1(%r15),%ebx
   ae3a7:	mov    %r14d,%r13d
   ae3aa:	and    $0xf,%r13b
   ae3ae:	je     ae3c6 <meshoptimizer_rs::codec::raw+0x806>
   ae3b0:	mov    %r10d,%ecx
   ae3b3:	sub    %r14d,%ecx
   ae3b6:	and    $0xf,%ecx
   ae3b9:	mov    %ebx,%ebp
   ae3bb:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae3c2:	mov    %ecx,%ebx
   ae3c4:	jmp    ae3c9 <meshoptimizer_rs::codec::raw+0x809>
   ae3c6:	lea    0x1(%rbx),%ebp
   ae3c9:	lea    0x1(%rsi),%r14
   ae3cd:	cmp    $0xfe,%eax
   ae3d2:	je     ae4d0 <meshoptimizer_rs::codec::raw+0x910>
   ae3d8:	mov    0x30(%rsp),%r9
   ae3dd:	movzbl 0x1(%r9,%rsi,1),%eax
   ae3e3:	test   %al,%al
   ae3e5:	js     ae416 <meshoptimizer_rs::codec::raw+0x856>
   ae3e7:	add    $0x2,%rsi
   ae3eb:	mov    %rsi,%r14
   ae3ee:	mov    %eax,%r11d
   ae3f1:	jmp    ae4b2 <meshoptimizer_rs::codec::raw+0x8f2>
   ae3f6:	and    $0x7f,%ebp
   ae3f9:	movzbl 0x1(%r14,%rsi,1),%ebx
   ae3ff:	mov    %ebx,%ecx
   ae401:	and    $0x7f,%ebx
   ae404:	shl    $0x7,%ebx
   ae407:	or     %ebp,%ebx
   ae409:	test   %cl,%cl
   ae40b:	js     ae437 <meshoptimizer_rs::codec::raw+0x877>
   ae40d:	add    $0x2,%rsi
   ae411:	jmp    ae58c <meshoptimizer_rs::codec::raw+0x9cc>
   ae416:	and    $0x7f,%eax
   ae419:	movzbl 0x2(%r9,%rsi,1),%r11d
   ae41f:	mov    %r11d,%ecx
   ae422:	and    $0x7f,%r11d
   ae426:	shl    $0x7,%r11d
   ae42a:	or     %eax,%r11d
   ae42d:	test   %cl,%cl
   ae42f:	js     ae455 <meshoptimizer_rs::codec::raw+0x895>
   ae431:	add    $0x3,%rsi
   ae435:	jmp    ae4af <meshoptimizer_rs::codec::raw+0x8ef>
   ae437:	movzbl 0x2(%r14,%rsi,1),%ecx
   ae43d:	mov    %ecx,%ebp
   ae43f:	and    $0x7f,%ecx
   ae442:	shl    $0xe,%ecx
   ae445:	or     %ecx,%ebx
   ae447:	test   %bpl,%bpl
   ae44a:	js     ae470 <meshoptimizer_rs::codec::raw+0x8b0>
   ae44c:	add    $0x3,%rsi
   ae450:	jmp    ae58c <meshoptimizer_rs::codec::raw+0x9cc>
   ae455:	movzbl 0x3(%r9,%rsi,1),%eax
   ae45b:	mov    %eax,%ecx
   ae45d:	and    $0x7f,%eax
   ae460:	shl    $0xe,%eax
   ae463:	or     %eax,%r11d
   ae466:	test   %cl,%cl
   ae468:	js     ae492 <meshoptimizer_rs::codec::raw+0x8d2>
   ae46a:	add    $0x4,%rsi
   ae46e:	jmp    ae4af <meshoptimizer_rs::codec::raw+0x8ef>
   ae470:	movzbl 0x3(%r14,%rsi,1),%ecx
   ae476:	mov    %ecx,%ebp
   ae478:	and    $0x7f,%ecx
   ae47b:	shl    $0x15,%ecx
   ae47e:	or     %ecx,%ebx
   ae480:	test   %bpl,%bpl
   ae483:	js     ae57b <meshoptimizer_rs::codec::raw+0x9bb>
   ae489:	add    $0x4,%rsi
   ae48d:	jmp    ae58c <meshoptimizer_rs::codec::raw+0x9cc>
   ae492:	movzbl 0x4(%r9,%rsi,1),%eax
   ae498:	mov    %eax,%ecx
   ae49a:	and    $0x7f,%eax
   ae49d:	shl    $0x15,%eax
   ae4a0:	or     %eax,%r11d
   ae4a3:	test   %cl,%cl
   ae4a5:	js     ae5b4 <meshoptimizer_rs::codec::raw+0x9f4>
   ae4ab:	add    $0x5,%rsi
   ae4af:	mov    %rsi,%r14
   ae4b2:	mov    0x28(%rsp),%r9
   ae4b7:	mov    %r11d,%eax
   ae4ba:	shr    $1,%eax
   ae4bc:	and    $0x1,%r11d
   ae4c0:	neg    %r11d
   ae4c3:	xor    %eax,%r11d
   ae4c6:	add    0x20(%rsp),%r11d
   ae4cb:	mov    %r11d,0x20(%rsp)
   ae4d0:	cmp    $0xf,%r12b
   ae4d4:	jne    ae603 <meshoptimizer_rs::codec::raw+0xa43>
   ae4da:	cmp    $0xfffffffffffffffa,%r14
   ae4de:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae4e4:	lea    0x5(%r14),%rax
   ae4e8:	cmp    0x630(%rsp),%rax
   ae4f0:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae4f6:	mov    0x30(%rsp),%r9
   ae4fb:	movzbl (%r9,%r14,1),%esi
   ae500:	test   %sil,%sil
   ae503:	js     ae513 <meshoptimizer_rs::codec::raw+0x953>
   ae505:	inc    %r14
   ae508:	mov    %r14,%rax
   ae50b:	mov    %esi,%r15d
   ae50e:	jmp    ae5e2 <meshoptimizer_rs::codec::raw+0xa22>
   ae513:	and    $0x7f,%esi
   ae516:	movzbl 0x1(%r9,%r14,1),%r15d
   ae51c:	mov    %r15d,%ecx
   ae51f:	and    $0x7f,%r15d
   ae523:	shl    $0x7,%r15d
   ae527:	or     %esi,%r15d
   ae52a:	test   %cl,%cl
   ae52c:	js     ae53a <meshoptimizer_rs::codec::raw+0x97a>
   ae52e:	add    $0x2,%r14
   ae532:	mov    %r14,%rax
   ae535:	jmp    ae5e2 <meshoptimizer_rs::codec::raw+0xa22>
   ae53a:	movzbl 0x2(%r9,%r14,1),%ecx
   ae540:	mov    %ecx,%esi
   ae542:	and    $0x7f,%ecx
   ae545:	shl    $0xe,%ecx
   ae548:	or     %ecx,%r15d
   ae54b:	test   %sil,%sil
   ae54e:	js     ae55c <meshoptimizer_rs::codec::raw+0x99c>
   ae550:	add    $0x3,%r14
   ae554:	mov    %r14,%rax
   ae557:	jmp    ae5e2 <meshoptimizer_rs::codec::raw+0xa22>
   ae55c:	movzbl 0x3(%r9,%r14,1),%ecx
   ae562:	mov    %ecx,%esi
   ae564:	and    $0x7f,%ecx
   ae567:	shl    $0x15,%ecx
   ae56a:	or     %ecx,%r15d
   ae56d:	test   %sil,%sil
   ae570:	js     ae5ce <meshoptimizer_rs::codec::raw+0xa0e>
   ae572:	add    $0x4,%r14
   ae576:	mov    %r14,%rax
   ae579:	jmp    ae5e2 <meshoptimizer_rs::codec::raw+0xa22>
   ae57b:	movzbl 0x4(%r14,%rsi,1),%ecx
   ae581:	add    $0x5,%rsi
   ae585:	shl    $0x1c,%ecx
   ae588:	or     %ebx,%ecx
   ae58a:	mov    %ecx,%ebx
   ae58c:	mov    0x20(%rsp),%ebp
   ae590:	mov    %ebx,%ecx
   ae592:	shr    $1,%ecx
   ae594:	and    $0x1,%ebx
   ae597:	neg    %ebx
   ae599:	xor    %ecx,%ebx
   ae59b:	add    %ebp,%ebx
   ae59d:	mov    %ebx,0xf0(%rsp,%r10,4)
   ae5a5:	mov    $0x1,%r14d
   ae5ab:	mov    %ebx,0x20(%rsp)
   ae5af:	jmp    ae18e <meshoptimizer_rs::codec::raw+0x5ce>
   ae5b4:	add    $0x6,%rsi
   ae5b8:	mov    0x30(%rsp),%rax
   ae5bd:	movzbl 0x4(%rax,%r14,1),%eax
   ae5c3:	shl    $0x1c,%eax
   ae5c6:	or     %r11d,%eax
   ae5c9:	jmp    ae3eb <meshoptimizer_rs::codec::raw+0x82b>
   ae5ce:	mov    0x30(%rsp),%rcx
   ae5d3:	movzbl 0x4(%rcx,%r14,1),%ecx
   ae5d9:	shl    $0x1c,%ecx
   ae5dc:	or     %r15d,%ecx
   ae5df:	mov    %ecx,%r15d
   ae5e2:	mov    0x28(%rsp),%r9
   ae5e7:	mov    %r15d,%ecx
   ae5ea:	shr    $1,%ecx
   ae5ec:	and    $0x1,%r15d
   ae5f0:	neg    %r15d
   ae5f3:	xor    %ecx,%r15d
   ae5f6:	add    0x20(%rsp),%r15d
   ae5fb:	mov    %rax,%r14
   ae5fe:	mov    %r15d,0x20(%rsp)
   ae603:	cmp    $0xf,%r13b
   ae607:	jne    ae6cb <meshoptimizer_rs::codec::raw+0xb0b>
   ae60d:	cmp    $0xfffffffffffffffa,%r14
   ae611:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae617:	lea    0x5(%r14),%rax
   ae61b:	cmp    0x630(%rsp),%rax
   ae623:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae629:	mov    0x30(%rsp),%r9
   ae62e:	movzbl (%r9,%r14,1),%esi
   ae633:	test   %sil,%sil
   ae636:	js     ae642 <meshoptimizer_rs::codec::raw+0xa82>
   ae638:	inc    %r14
   ae63b:	mov    %r14,%rax
   ae63e:	mov    %esi,%ebx
   ae640:	jmp    ae6b0 <meshoptimizer_rs::codec::raw+0xaf0>
   ae642:	and    $0x7f,%esi
   ae645:	movzbl 0x1(%r9,%r14,1),%ebx
   ae64b:	mov    %ebx,%ecx
   ae64d:	and    $0x7f,%ebx
   ae650:	shl    $0x7,%ebx
   ae653:	or     %esi,%ebx
   ae655:	test   %cl,%cl
   ae657:	js     ae662 <meshoptimizer_rs::codec::raw+0xaa2>
   ae659:	add    $0x2,%r14
   ae65d:	mov    %r14,%rax
   ae660:	jmp    ae6b0 <meshoptimizer_rs::codec::raw+0xaf0>
   ae662:	movzbl 0x2(%r9,%r14,1),%ecx
   ae668:	mov    %ecx,%esi
   ae66a:	and    $0x7f,%ecx
   ae66d:	shl    $0xe,%ecx
   ae670:	or     %ecx,%ebx
   ae672:	test   %sil,%sil
   ae675:	js     ae680 <meshoptimizer_rs::codec::raw+0xac0>
   ae677:	add    $0x3,%r14
   ae67b:	mov    %r14,%rax
   ae67e:	jmp    ae6b0 <meshoptimizer_rs::codec::raw+0xaf0>
   ae680:	movzbl 0x3(%r9,%r14,1),%ecx
   ae686:	mov    %ecx,%esi
   ae688:	and    $0x7f,%ecx
   ae68b:	shl    $0x15,%ecx
   ae68e:	or     %ecx,%ebx
   ae690:	test   %sil,%sil
   ae693:	js     ae69e <meshoptimizer_rs::codec::raw+0xade>
   ae695:	add    $0x4,%r14
   ae699:	mov    %r14,%rax
   ae69c:	jmp    ae6b0 <meshoptimizer_rs::codec::raw+0xaf0>
   ae69e:	mov    0x30(%rsp),%rcx
   ae6a3:	movzbl 0x4(%rcx,%r14,1),%ecx
   ae6a9:	shl    $0x1c,%ecx
   ae6ac:	or     %ebx,%ecx
   ae6ae:	mov    %ecx,%ebx
   ae6b0:	mov    0x28(%rsp),%r9
   ae6b5:	mov    %ebx,%ecx
   ae6b7:	shr    $1,%ecx
   ae6b9:	and    $0x1,%ebx
   ae6bc:	neg    %ebx
   ae6be:	xor    %ecx,%ebx
   ae6c0:	add    0x20(%rsp),%ebx
   ae6c4:	mov    %rax,%r14
   ae6c7:	mov    %ebx,0x20(%rsp)
   ae6cb:	cmp    $0xf,%r13b
   ae6cf:	sete   %al
   ae6d2:	cmp    $0xf,%r12b
   ae6d6:	sete   %cl
   ae6d9:	test   %r13b,%r13b
   ae6dc:	sete   %sil
   ae6e0:	test   %r12b,%r12b
   ae6e3:	sete   %r12b
   ae6e7:	mov    %r11d,0xf0(%rsp,%r10,4)
   ae6ef:	lea    0x1(%r10),%r13
   ae6f3:	and    $0xf,%r13d
   ae6f7:	or     %cl,%r12b
   ae6fa:	mov    %r15d,0xf0(%rsp,%r13,4)
   ae702:	movzbl %r12b,%ecx
   ae706:	add    %r10,%rcx
   ae709:	inc    %rcx
   ae70c:	or     %al,%sil
   ae70f:	movzbl %sil,%r10d
   ae713:	add    %rcx,%r10
   ae716:	and    $0xf,%ecx
   ae719:	mov    %ebx,0xf0(%rsp,%rcx,4)
   ae720:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ae728:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ae730:	lea    0x1(%r8),%r13d
   ae734:	and    $0xf,%r13d
   ae738:	mov    $0x3,%eax
   ae73d:	mov    $0x2,%r12d
   ae743:	mov    %r14,%rsi
   ae746:	jmp    ae1bd <meshoptimizer_rs::codec::raw+0x5fd>
   ae74b:	mov    %bpl,%r14b
   ae74e:	add    %r14d,%r11d
   ae751:	mov    %ebx,0xf0(%rsp,%r10,4)
   ae759:	mov    %r11d,%ebp
   ae75c:	not    %eax
   ae75e:	shr    $0x4,%eax
   ae761:	add    %r8d,%eax
   ae764:	and    $0xf,%eax
   ae767:	mov    0x1f0(%rsp,%rax,8),%r11d
   ae76f:	mov    0x1f4(%rsp,%rax,8),%r15d
   ae777:	add    %r14,%r10
   ae77a:	mov    $0x2,%eax
   ae77f:	mov    $0x1,%r12d
   ae785:	mov    %r8,%r13
   ae788:	inc    %rdi
   ae78b:	mov    %ebx,0x1f0(%rsp,%r13,8)
   ae793:	mov    %r15d,0x1f4(%rsp,%r13,8)
   ae79b:	add    %r8d,%r12d
   ae79e:	and    $0xf,%r12d
   ae7a2:	mov    %r11d,0x1f0(%rsp,%r12,8)
   ae7aa:	mov    %ebx,0x1f4(%rsp,%r12,8)
   ae7b2:	add    %r8d,%eax
   ae7b5:	and    $0xf,%eax
   ae7b8:	and    $0xf,%r10d
   ae7bc:	mov    %r11d,-0x8(%rdx)
   ae7c0:	mov    %r15d,-0x4(%rdx)
   ae7c4:	mov    %rdx,%r15
   ae7c7:	mov    %ebx,(%rdx)
   ae7c9:	add    $0xc,%r15
   ae7cd:	mov    %rax,%r8
   ae7d0:	mov    %ebp,%r11d
   ae7d3:	cmp    %rdi,0x38(%rsp)
   ae7d8:	je     af714 <meshoptimizer_rs::codec::raw+0x1b54>
   ae7de:	cmp    %r9,%rsi
   ae7e1:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   ae7e7:	mov    %r15,%rdx
   ae7ea:	mov    0x30(%rsp),%r14
   ae7ef:	movzbl 0x1(%r14,%rdi,1),%eax
   ae7f5:	cmp    $0xf0,%al
   ae7f7:	jae    ae831 <meshoptimizer_rs::codec::raw+0xc71>
   ae7f9:	mov    %eax,%r15d
   ae7fc:	and    $0xf,%r15d
   ae800:	cmp    %r15d,0x58(%rsp)
   ae805:	jbe    ae86b <meshoptimizer_rs::codec::raw+0xcab>
   ae807:	xor    %r14d,%r14d
   ae80a:	mov    %r11d,%ebx
   ae80d:	test   %r15b,%r15b
   ae810:	sete   %bpl
   ae814:	je     ae74b <meshoptimizer_rs::codec::raw+0xb8b>
   ae81a:	not    %r15d
   ae81d:	add    %r10d,%r15d
   ae820:	and    $0xf,%r15d
   ae824:	mov    0xf0(%rsp,%r15,4),%ebx
   ae82c:	jmp    ae74b <meshoptimizer_rs::codec::raw+0xb8b>
   ae831:	cmp    $0xfe,%al
   ae833:	jae    ae88d <meshoptimizer_rs::codec::raw+0xccd>
   ae835:	and    $0xf,%eax
   ae838:	mov    0x50(%rsp),%rcx
   ae83d:	movzbl (%rcx,%rax,1),%r13d
   ae842:	lea    0x1(%r11),%r15d
   ae846:	xor    %eax,%eax
   ae848:	mov    %r13d,%ecx
   ae84b:	shr    $0x4,%ecx
   ae84e:	sete   %r12b
   ae852:	je     ae8d0 <meshoptimizer_rs::codec::raw+0xd10>
   ae854:	mov    %r10d,%ebx
   ae857:	sub    %ecx,%ebx
   ae859:	and    $0xf,%ebx
   ae85c:	mov    %r15d,%ebp
   ae85f:	mov    0xf0(%rsp,%rbx,4),%ecx
   ae866:	mov    %ecx,%r15d
   ae869:	jmp    ae8d4 <meshoptimizer_rs::codec::raw+0xd14>
   ae86b:	cmp    $0xf,%r15b
   ae86f:	jne    ae950 <meshoptimizer_rs::codec::raw+0xd90>
   ae875:	movzbl (%r14,%rsi,1),%ebp
   ae87a:	test   %bpl,%bpl
   ae87d:	js     ae9be <meshoptimizer_rs::codec::raw+0xdfe>
   ae883:	inc    %rsi
   ae886:	mov    %ebp,%ebx
   ae888:	jmp    aeb54 <meshoptimizer_rs::codec::raw+0xf94>
   ae88d:	movzbl (%r14,%rsi,1),%r14d
   ae892:	xor    %r15d,%r15d
   ae895:	cmp    $0xfe,%eax
   ae89a:	sete   %r15b
   ae89e:	test   %r14d,%r14d
   ae8a1:	cmove  %r14d,%r11d
   ae8a5:	add    %r11d,%r15d
   ae8a8:	mov    %r14d,%r12d
   ae8ab:	shr    $0x4,%r12d
   ae8af:	je     ae96b <meshoptimizer_rs::codec::raw+0xdab>
   ae8b5:	mov    %r10d,%ecx
   ae8b8:	sub    %r12d,%ecx
   ae8bb:	and    $0xf,%ecx
   ae8be:	mov    %r15d,%ebx
   ae8c1:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae8c8:	mov    %ecx,%r15d
   ae8cb:	jmp    ae96f <meshoptimizer_rs::codec::raw+0xdaf>
   ae8d0:	lea    0x2(%r11),%ebp
   ae8d4:	xor    %r14d,%r14d
   ae8d7:	mov    %ebp,%ebx
   ae8d9:	test   $0xf,%r13b
   ae8dd:	sete   %cl
   ae8e0:	je     ae8f2 <meshoptimizer_rs::codec::raw+0xd32>
   ae8e2:	mov    %r10d,%ebx
   ae8e5:	sub    %r13d,%ebx
   ae8e8:	and    $0xf,%ebx
   ae8eb:	mov    0xf0(%rsp,%rbx,4),%ebx
   ae8f2:	mov    %cl,%r14b
   ae8f5:	add    %r14d,%ebp
   ae8f8:	mov    %r11d,0xf0(%rsp,%r10,4)
   ae900:	lea    0x1(%r10),%rcx
   ae904:	and    $0xf,%ecx
   ae907:	mov    %r15d,0xf0(%rsp,%rcx,4)
   ae90f:	mov    %r12b,%al
   ae912:	add    %r10,%rax
   ae915:	inc    %rax
   ae918:	add    %rax,%r14
   ae91b:	and    $0xf,%eax
   ae91e:	mov    %ebx,0xf0(%rsp,%rax,4)
   ae925:	mov    %r15d,0x1f0(%rsp,%r8,8)
   ae92d:	mov    %r11d,0x1f4(%rsp,%r8,8)
   ae935:	lea    0x1(%r8),%r13d
   ae939:	and    $0xf,%r13d
   ae93d:	mov    $0x3,%eax
   ae942:	mov    $0x2,%r12d
   ae948:	mov    %r14,%r10
   ae94b:	jmp    ae788 <meshoptimizer_rs::codec::raw+0xbc8>
   ae950:	movzbl %r15b,%ecx
   ae954:	xor    $0x3,%r15b
   ae958:	movzbl %r15b,%ebx
   ae95c:	mov    0x20(%rsp),%ebp
   ae960:	add    %ecx,%ebp
   ae962:	sub    %ebx,%ebp
   ae964:	mov    %ebp,%ebx
   ae966:	jmp    aeb65 <meshoptimizer_rs::codec::raw+0xfa5>
   ae96b:	lea    0x1(%r15),%ebx
   ae96f:	mov    %r14d,%r13d
   ae972:	and    $0xf,%r13b
   ae976:	je     ae98e <meshoptimizer_rs::codec::raw+0xdce>
   ae978:	mov    %r10d,%ecx
   ae97b:	sub    %r14d,%ecx
   ae97e:	and    $0xf,%ecx
   ae981:	mov    %ebx,%ebp
   ae983:	mov    0xf0(%rsp,%rcx,4),%ecx
   ae98a:	mov    %ecx,%ebx
   ae98c:	jmp    ae991 <meshoptimizer_rs::codec::raw+0xdd1>
   ae98e:	lea    0x1(%rbx),%ebp
   ae991:	lea    0x1(%rsi),%r14
   ae995:	cmp    $0xfe,%eax
   ae99a:	je     aea98 <meshoptimizer_rs::codec::raw+0xed8>
   ae9a0:	mov    0x30(%rsp),%r9
   ae9a5:	movzbl 0x1(%r9,%rsi,1),%eax
   ae9ab:	test   %al,%al
   ae9ad:	js     ae9de <meshoptimizer_rs::codec::raw+0xe1e>
   ae9af:	add    $0x2,%rsi
   ae9b3:	mov    %rsi,%r14
   ae9b6:	mov    %eax,%r11d
   ae9b9:	jmp    aea7a <meshoptimizer_rs::codec::raw+0xeba>
   ae9be:	and    $0x7f,%ebp
   ae9c1:	movzbl 0x1(%r14,%rsi,1),%ebx
   ae9c7:	mov    %ebx,%ecx
   ae9c9:	and    $0x7f,%ebx
   ae9cc:	shl    $0x7,%ebx
   ae9cf:	or     %ebp,%ebx
   ae9d1:	test   %cl,%cl
   ae9d3:	js     ae9ff <meshoptimizer_rs::codec::raw+0xe3f>
   ae9d5:	add    $0x2,%rsi
   ae9d9:	jmp    aeb54 <meshoptimizer_rs::codec::raw+0xf94>
   ae9de:	and    $0x7f,%eax
   ae9e1:	movzbl 0x2(%r9,%rsi,1),%r11d
   ae9e7:	mov    %r11d,%ecx
   ae9ea:	and    $0x7f,%r11d
   ae9ee:	shl    $0x7,%r11d
   ae9f2:	or     %eax,%r11d
   ae9f5:	test   %cl,%cl
   ae9f7:	js     aea1d <meshoptimizer_rs::codec::raw+0xe5d>
   ae9f9:	add    $0x3,%rsi
   ae9fd:	jmp    aea77 <meshoptimizer_rs::codec::raw+0xeb7>
   ae9ff:	movzbl 0x2(%r14,%rsi,1),%ecx
   aea05:	mov    %ecx,%ebp
   aea07:	and    $0x7f,%ecx
   aea0a:	shl    $0xe,%ecx
   aea0d:	or     %ecx,%ebx
   aea0f:	test   %bpl,%bpl
   aea12:	js     aea38 <meshoptimizer_rs::codec::raw+0xe78>
   aea14:	add    $0x3,%rsi
   aea18:	jmp    aeb54 <meshoptimizer_rs::codec::raw+0xf94>
   aea1d:	movzbl 0x3(%r9,%rsi,1),%eax
   aea23:	mov    %eax,%ecx
   aea25:	and    $0x7f,%eax
   aea28:	shl    $0xe,%eax
   aea2b:	or     %eax,%r11d
   aea2e:	test   %cl,%cl
   aea30:	js     aea5a <meshoptimizer_rs::codec::raw+0xe9a>
   aea32:	add    $0x4,%rsi
   aea36:	jmp    aea77 <meshoptimizer_rs::codec::raw+0xeb7>
   aea38:	movzbl 0x3(%r14,%rsi,1),%ecx
   aea3e:	mov    %ecx,%ebp
   aea40:	and    $0x7f,%ecx
   aea43:	shl    $0x15,%ecx
   aea46:	or     %ecx,%ebx
   aea48:	test   %bpl,%bpl
   aea4b:	js     aeb43 <meshoptimizer_rs::codec::raw+0xf83>
   aea51:	add    $0x4,%rsi
   aea55:	jmp    aeb54 <meshoptimizer_rs::codec::raw+0xf94>
   aea5a:	movzbl 0x4(%r9,%rsi,1),%eax
   aea60:	mov    %eax,%ecx
   aea62:	and    $0x7f,%eax
   aea65:	shl    $0x15,%eax
   aea68:	or     %eax,%r11d
   aea6b:	test   %cl,%cl
   aea6d:	js     aeb7c <meshoptimizer_rs::codec::raw+0xfbc>
   aea73:	add    $0x5,%rsi
   aea77:	mov    %rsi,%r14
   aea7a:	mov    0x28(%rsp),%r9
   aea7f:	mov    %r11d,%eax
   aea82:	shr    $1,%eax
   aea84:	and    $0x1,%r11d
   aea88:	neg    %r11d
   aea8b:	xor    %eax,%r11d
   aea8e:	add    0x20(%rsp),%r11d
   aea93:	mov    %r11d,0x20(%rsp)
   aea98:	cmp    $0xf,%r12b
   aea9c:	jne    aebcb <meshoptimizer_rs::codec::raw+0x100b>
   aeaa2:	cmp    $0xfffffffffffffffa,%r14
   aeaa6:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   aeaac:	lea    0x5(%r14),%rax
   aeab0:	cmp    0x630(%rsp),%rax
   aeab8:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   aeabe:	mov    0x30(%rsp),%r9
   aeac3:	movzbl (%r9,%r14,1),%esi
   aeac8:	test   %sil,%sil
   aeacb:	js     aeadb <meshoptimizer_rs::codec::raw+0xf1b>
   aeacd:	inc    %r14
   aead0:	mov    %r14,%rax
   aead3:	mov    %esi,%r15d
   aead6:	jmp    aebaa <meshoptimizer_rs::codec::raw+0xfea>
   aeadb:	and    $0x7f,%esi
   aeade:	movzbl 0x1(%r9,%r14,1),%r15d
   aeae4:	mov    %r15d,%ecx
   aeae7:	and    $0x7f,%r15d
   aeaeb:	shl    $0x7,%r15d
   aeaef:	or     %esi,%r15d
   aeaf2:	test   %cl,%cl
   aeaf4:	js     aeb02 <meshoptimizer_rs::codec::raw+0xf42>
   aeaf6:	add    $0x2,%r14
   aeafa:	mov    %r14,%rax
   aeafd:	jmp    aebaa <meshoptimizer_rs::codec::raw+0xfea>
   aeb02:	movzbl 0x2(%r9,%r14,1),%ecx
   aeb08:	mov    %ecx,%esi
   aeb0a:	and    $0x7f,%ecx
   aeb0d:	shl    $0xe,%ecx
   aeb10:	or     %ecx,%r15d
   aeb13:	test   %sil,%sil
   aeb16:	js     aeb24 <meshoptimizer_rs::codec::raw+0xf64>
   aeb18:	add    $0x3,%r14
   aeb1c:	mov    %r14,%rax
   aeb1f:	jmp    aebaa <meshoptimizer_rs::codec::raw+0xfea>
   aeb24:	movzbl 0x3(%r9,%r14,1),%ecx
   aeb2a:	mov    %ecx,%esi
   aeb2c:	and    $0x7f,%ecx
   aeb2f:	shl    $0x15,%ecx
   aeb32:	or     %ecx,%r15d
   aeb35:	test   %sil,%sil
   aeb38:	js     aeb96 <meshoptimizer_rs::codec::raw+0xfd6>
   aeb3a:	add    $0x4,%r14
   aeb3e:	mov    %r14,%rax
   aeb41:	jmp    aebaa <meshoptimizer_rs::codec::raw+0xfea>
   aeb43:	movzbl 0x4(%r14,%rsi,1),%ecx
   aeb49:	add    $0x5,%rsi
   aeb4d:	shl    $0x1c,%ecx
   aeb50:	or     %ebx,%ecx
   aeb52:	mov    %ecx,%ebx
   aeb54:	mov    0x20(%rsp),%ebp
   aeb58:	mov    %ebx,%ecx
   aeb5a:	shr    $1,%ecx
   aeb5c:	and    $0x1,%ebx
   aeb5f:	neg    %ebx
   aeb61:	xor    %ecx,%ebx
   aeb63:	add    %ebp,%ebx
   aeb65:	mov    %ebx,0xf0(%rsp,%r10,4)
   aeb6d:	mov    $0x1,%r14d
   aeb73:	mov    %ebx,0x20(%rsp)
   aeb77:	jmp    ae759 <meshoptimizer_rs::codec::raw+0xb99>
   aeb7c:	add    $0x6,%rsi
   aeb80:	mov    0x30(%rsp),%rax
   aeb85:	movzbl 0x4(%rax,%r14,1),%eax
   aeb8b:	shl    $0x1c,%eax
   aeb8e:	or     %r11d,%eax
   aeb91:	jmp    ae9b3 <meshoptimizer_rs::codec::raw+0xdf3>
   aeb96:	mov    0x30(%rsp),%rcx
   aeb9b:	movzbl 0x4(%rcx,%r14,1),%ecx
   aeba1:	shl    $0x1c,%ecx
   aeba4:	or     %r15d,%ecx
   aeba7:	mov    %ecx,%r15d
   aebaa:	mov    0x28(%rsp),%r9
   aebaf:	mov    %r15d,%ecx
   aebb2:	shr    $1,%ecx
   aebb4:	and    $0x1,%r15d
   aebb8:	neg    %r15d
   aebbb:	xor    %ecx,%r15d
   aebbe:	add    0x20(%rsp),%r15d
   aebc3:	mov    %rax,%r14
   aebc6:	mov    %r15d,0x20(%rsp)
   aebcb:	cmp    $0xf,%r13b
   aebcf:	jne    aec93 <meshoptimizer_rs::codec::raw+0x10d3>
   aebd5:	cmp    $0xfffffffffffffffa,%r14
   aebd9:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   aebdf:	lea    0x5(%r14),%rax
   aebe3:	cmp    0x630(%rsp),%rax
   aebeb:	ja     af71b <meshoptimizer_rs::codec::raw+0x1b5b>
   aebf1:	mov    0x30(%rsp),%r9
   aebf6:	movzbl (%r9,%r14,1),%esi
   aebfb:	test   %sil,%sil
   aebfe:	js     aec0a <meshoptimizer_rs::codec::raw+0x104a>
   aec00:	inc    %r14
   aec03:	mov    %r14,%rax
   aec06:	mov    %esi,%ebx
   aec08:	jmp    aec78 <meshoptimizer_rs::codec::raw+0x10b8>
   aec0a:	and    $0x7f,%esi
   aec0d:	movzbl 0x1(%r9,%r14,1),%ebx
   aec13:	mov    %ebx,%ecx
   aec15:	and    $0x7f,%ebx
   aec18:	shl    $0x7,%ebx
   aec1b:	or     %esi,%ebx
   aec1d:	test   %cl,%cl
   aec1f:	js     aec2a <meshoptimizer_rs::codec::raw+0x106a>
   aec21:	add    $0x2,%r14
   aec25:	mov    %r14,%rax
   aec28:	jmp    aec78 <meshoptimizer_rs::codec::raw+0x10b8>
   aec2a:	movzbl 0x2(%r9,%r14,1),%ecx
   aec30:	mov    %ecx,%esi
   aec32:	and    $0x7f,%ecx
   aec35:	shl    $0xe,%ecx
   aec38:	or     %ecx,%ebx
   aec3a:	test   %sil,%sil
   aec3d:	js     aec48 <meshoptimizer_rs::codec::raw+0x1088>
   aec3f:	add    $0x3,%r14
   aec43:	mov    %r14,%rax
   aec46:	jmp    aec78 <meshoptimizer_rs::codec::raw+0x10b8>
   aec48:	movzbl 0x3(%r9,%r14,1),%ecx
   aec4e:	mov    %ecx,%esi
   aec50:	and    $0x7f,%ecx
   aec53:	shl    $0x15,%ecx
   aec56:	or     %ecx,%ebx
   aec58:	test   %sil,%sil
   aec5b:	js     aec66 <meshoptimizer_rs::codec::raw+0x10a6>
   aec5d:	add    $0x4,%r14
   aec61:	mov    %r14,%rax
   aec64:	jmp    aec78 <meshoptimizer_rs::codec::raw+0x10b8>
   aec66:	mov    0x30(%rsp),%rcx
   aec6b:	movzbl 0x4(%rcx,%r14,1),%ecx
   aec71:	shl    $0x1c,%ecx
   aec74:	or     %ebx,%ecx
   aec76:	mov    %ecx,%ebx
   aec78:	mov    0x28(%rsp),%r9
   aec7d:	mov    %ebx,%ecx
   aec7f:	shr    $1,%ecx
   aec81:	and    $0x1,%ebx
   aec84:	neg    %ebx
   aec86:	xor    %ecx,%ebx
   aec88:	add    0x20(%rsp),%ebx
   aec8c:	mov    %rax,%r14
   aec8f:	mov    %ebx,0x20(%rsp)
   aec93:	cmp    $0xf,%r13b
   aec97:	sete   %al
   aec9a:	cmp    $0xf,%r12b
   aec9e:	sete   %cl
   aeca1:	test   %r13b,%r13b
   aeca4:	sete   %sil
   aeca8:	test   %r12b,%r12b
   aecab:	sete   %r12b
   aecaf:	mov    %r11d,0xf0(%rsp,%r10,4)
   aecb7:	lea    0x1(%r10),%r13
   aecbb:	and    $0xf,%r13d
   aecbf:	or     %cl,%r12b
   aecc2:	mov    %r15d,0xf0(%rsp,%r13,4)
   aecca:	movzbl %r12b,%ecx
   aecce:	add    %r10,%rcx
   aecd1:	inc    %rcx
   aecd4:	or     %al,%sil
   aecd7:	movzbl %sil,%r10d
   aecdb:	add    %rcx,%r10
   aecde:	and    $0xf,%ecx
   aece1:	mov    %ebx,0xf0(%rsp,%rcx,4)
   aece8:	mov    %r15d,0x1f0(%rsp,%r8,8)
   aecf0:	mov    %r11d,0x1f4(%rsp,%r8,8)
   aecf8:	lea    0x1(%r8),%r13d
   aecfc:	and    $0xf,%r13d
   aed00:	mov    $0x3,%eax
   aed05:	mov    $0x2,%r12d
   aed0b:	mov    %r14,%rsi
   aed0e:	jmp    ae788 <meshoptimizer_rs::codec::raw+0xbc8>
   aed13:	cmp    $0x21,%rbp
   aed17:	mov    $0x20,%eax
   aed1c:	cmovae %rbp,%rax
   aed20:	mov    %rbp,%rcx
   aed23:	mov    %rax,0xe0(%rsp)
   aed2b:	lea    0x1(%rax),%rdx
   aed2f:	mov    $0x7,%al
   aed31:	cmp    %rdx,%rbx
   aed34:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   aed3a:	mov    %rbx,%rsi
   aed3d:	sub    %rcx,%rsi
   aed40:	lea    (%rsi,%rbp,1),%rax
   aed44:	mov    %rbx,%rcx
   aed47:	mov    %rax,0x60(%rsp)
   aed4c:	sub    %rax,%rcx
   aed4f:	mov    %rcx,0xa0(%rsp)
   aed57:	jb     af79c <meshoptimizer_rs::codec::raw+0x1bdc>
   aed5d:	pxor   %xmm0,%xmm0
   aed61:	movdqa %xmm0,0x1e0(%rsp)
   aed6a:	movdqa %xmm0,0x1d0(%rsp)
   aed73:	movdqa %xmm0,0x1c0(%rsp)
   aed7c:	movdqa %xmm0,0x1b0(%rsp)
   aed85:	movdqa %xmm0,0x1a0(%rsp)
   aed8e:	movdqa %xmm0,0x190(%rsp)
   aed97:	movdqa %xmm0,0x180(%rsp)
   aeda0:	movdqa %xmm0,0x170(%rsp)
   aeda9:	movdqa %xmm0,0x160(%rsp)
   aedb2:	movdqa %xmm0,0x150(%rsp)
   aedbb:	movdqa %xmm0,0x140(%rsp)
   aedc4:	movdqa %xmm0,0x130(%rsp)
   aedcd:	movdqa %xmm0,0x120(%rsp)
   aedd6:	movdqa %xmm0,0x110(%rsp)
   aeddf:	movdqa %xmm0,0x100(%rsp)
   aede8:	movdqa %xmm0,0xf0(%rsp)
   aedf1:	cmp    $0x101,%rbp
   aedf8:	jae    af75d <meshoptimizer_rs::codec::raw+0x1b9d>
   aedfe:	mov    %dil,0x38(%rsp)
   aee03:	lea    (%rsi,%rbp,1),%rax
   aee07:	cmp    %rsi,%rax
   aee0a:	jb     af774 <meshoptimizer_rs::codec::raw+0x1bb4>
   aee10:	add    %r14,%rsi
   aee13:	lea    0xf0(%rsp),%rdi
   aee1b:	mov    %rbp,%rdx
   aee1e:	call   *0x7789c(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   aee24:	test   %rbp,%rbp
   aee27:	je     af805 <meshoptimizer_rs::codec::raw+0x1c45>
   aee2d:	mov    $0x2000,%ax
   aee31:	xor    %edx,%edx
   aee33:	div    %bp
   aee36:	and    $0x3ff0,%eax
   aee3b:	cmp    $0x100,%eax
   aee40:	mov    $0x100,%ecx
   aee45:	cmovb  %eax,%ecx
   aee48:	mov    %rcx,0xd8(%rsp)
   aee50:	lea    0x1f0(%rsp),%rdi
   aee58:	mov    $0x400,%edx
   aee5d:	xor    %esi,%esi
   aee5f:	call   *0x7786b(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   aee65:	test   %r13,%r13
   aee68:	je     af6f0 <meshoptimizer_rs::codec::raw+0x1b30>
   aee6e:	mov    %r15,0xd0(%rsp)
   aee76:	add    %r14,0x60(%rsp)
   aee7b:	mov    %rbp,%rax
   aee7e:	shr    $0x2,%rax
   aee82:	mov    %rax,0x98(%rsp)
   aee8a:	lea    0x3(%rbp),%rax
   aee8e:	shr    $0x2,%rax
   aee92:	mov    %rax,0xe8(%rsp)
   aee9a:	mov    $0x1,%r15d
   aeea0:	xor    %eax,%eax
   aeea2:	mov    %r12,0x88(%rsp)
   aeeaa:	mov    %r14,0x30(%rsp)
   aeeaf:	mov    %r13,%rcx
   aeeb2:	mov    %rax,%rdi
   aeeb5:	sub    %rax,%rcx
   aeeb8:	mov    0xd8(%rsp),%rax
   aeec0:	cmp    %rax,%rcx
   aeec3:	cmovae %rax,%rcx
   aeec7:	mov    %rcx,0x20(%rsp)
   aeecc:	movzbl 0x38(%rsp),%edx
   aeed1:	test   %dl,%dl
   aeed3:	je     aef19 <meshoptimizer_rs::codec::raw+0x1359>
   aeed5:	mov    %r15,%rcx
   aeed8:	mov    $0x7,%al
   aeeda:	add    0x98(%rsp),%rcx
   aeee2:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   aeee8:	cmp    %rbx,%rcx
   aeeeb:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   aeef1:	add    %r14,%r15
   aeef4:	mov    0x98(%rsp),%rax
   aeefc:	mov    %rax,0x90(%rsp)
   aef04:	mov    %r15,0xa8(%rsp)
   aef0c:	mov    %rcx,%r15
   aef0f:	mov    0x88(%rsp),%r12
   aef17:	jmp    aef32 <meshoptimizer_rs::codec::raw+0x1372>
   aef19:	movq   $0x0,0x90(%rsp)
   aef25:	mov    $0x1,%eax
   aef2a:	mov    %rax,0xa8(%rsp)
   aef32:	mov    0x20(%rsp),%rax
   aef37:	lea    (%rax,%rdi,1),%rsi
   aef3b:	imul   %rbp,%rdi
   aef3f:	mov    %rsi,0xc8(%rsp)
   aef47:	imul   %rbp,%rsi
   aef4b:	cmp    %rdi,%rsi
   aef4e:	jb     af78c <meshoptimizer_rs::codec::raw+0x1bcc>
   aef54:	cmp    %r12,%rsi
   aef57:	ja     af78c <meshoptimizer_rs::codec::raw+0x1bcc>
   aef5d:	mov    0x20(%rsp),%rax
   aef62:	lea    0xf(%rax),%ecx
   aef65:	and    $0x3f0,%ecx
   aef6b:	mov    %rcx,0x28(%rsp)
   aef70:	mov    %rax,%rcx
   aef73:	imul   %rbp,%rcx
   aef77:	mov    %rcx,0xb8(%rsp)
   aef7f:	add    0xd0(%rsp),%rdi
   aef87:	mov    %rdi,0xb0(%rsp)
   aef8f:	lea    -0x1(%rax),%rcx
   aef93:	imul   %rbp,%rcx
   aef97:	mov    %rcx,0xc0(%rsp)
   aef9f:	add    $0x4,%rcx
   aefa3:	mov    %rcx,0x50(%rsp)
   aefa8:	lea    0x0(,%rax,4),%rcx
   aefb0:	mov    %rcx,0x68(%rsp)
   aefb5:	lea    0x1f0(%rsp,%rax,1),%rcx
   aefbd:	mov    %rcx,0x80(%rsp)
   aefc5:	lea    0x1f0(%rsp,%rax,2),%rcx
   aefcd:	mov    %rcx,0x78(%rsp)
   aefd2:	lea    (%rax,%rax,2),%rax
   aefd6:	add    %rsp,%rax
   aefd9:	add    $0x1f0,%rax
   aefdf:	mov    %rax,0x70(%rsp)
   aefe4:	xor    %r14d,%r14d
   aefe7:	xor    %ebx,%ebx
   aefe9:	jmp    af04a <meshoptimizer_rs::codec::raw+0x148a>
   aefeb:	lea    (%rsp,%rbx,1),%rcx
   aefef:	add    $0xf0,%rcx
   aeff6:	shr    $0x4,%al
   aeff9:	neg    %al
   aeffb:	and    $0x1f,%al
   aeffd:	movzbl %al,%eax
   af000:	mov    %rcx,(%rsp)
   af004:	mov    %eax,0x8(%rsp)
   af008:	movq   $0x0,0x10(%rsp)
   af011:	lea    0x1f0(%rsp),%rdi
   af019:	mov    0x68(%rsp),%rsi
   af01e:	mov    0x50(%rsp),%rcx
   af023:	mov    0x20(%rsp),%r8
   af028:	mov    %rbp,%r9
   af02b:	call   72fc0 <meshoptimizer_rs::codec::vertex::scalar_deltas::<4, true>>
   af030:	inc    %r14
   af033:	add    $0x4,%rbx
   af037:	cmp    %r14,0xe8(%rsp)
   af03f:	movzbl 0x38(%rsp),%edx
   af044:	je     af698 <meshoptimizer_rs::codec::raw+0x1ad8>
   af04a:	mov    %rbx,0x58(%rsp)
   af04f:	test   %dl,%dl
   af051:	je     af070 <meshoptimizer_rs::codec::raw+0x14b0>
   af053:	cmp    0x90(%rsp),%r14
   af05b:	jae    af7d5 <meshoptimizer_rs::codec::raw+0x1c15>
   af061:	mov    0xa8(%rsp),%rax
   af069:	movzbl (%rax,%r14,1),%ebx
   af06e:	jmp    af072 <meshoptimizer_rs::codec::raw+0x14b2>
   af070:	xor    %ebx,%ebx
   af072:	mov    %ebx,%eax
   af074:	and    $0x3,%al
   af076:	movzbl %al,%ecx
   af079:	mov    0x28(%rsp),%rsi
   af07e:	cmp    %rsi,0x20(%rsp)
   af083:	ja     af4e3 <meshoptimizer_rs::codec::raw+0x1923>
   af089:	cmp    $0x2,%al
   af08b:	je     af0cd <meshoptimizer_rs::codec::raw+0x150d>
   af08d:	cmp    $0x3,%ecx
   af090:	jne    af0e7 <meshoptimizer_rs::codec::raw+0x1527>
   af092:	mov    %r15,%r12
   af095:	mov    $0x7,%al
   af097:	add    0x20(%rsp),%r12
   af09c:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af0a2:	cmp    0x630(%rsp),%r12
   af0aa:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af0b0:	add    0x30(%rsp),%r15
   af0b5:	lea    0x1f0(%rsp),%rdi
   af0bd:	mov    %r15,%rsi
   af0c0:	mov    0x20(%rsp),%rdx
   af0c5:	call   *0x775f5(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   af0cb:	jmp    af143 <meshoptimizer_rs::codec::raw+0x1583>
   af0cd:	lea    0x1f0(%rsp),%rdi
   af0d5:	xor    %esi,%esi
   af0d7:	mov    0x20(%rsp),%rdx
   af0dc:	call   *0x775ee(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   af0e2:	mov    %r15,%r12
   af0e5:	jmp    af143 <meshoptimizer_rs::codec::raw+0x1583>
   af0e7:	test   %al,%al
   af0e9:	lea    -0xa1550(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af0f0:	lea    -0xa1437(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af0f7:	cmove  %rcx,%rax
   af0fb:	test   %dl,%dl
   af0fd:	lea    -0xa1864(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af104:	cmove  %rcx,%rax
   af108:	mov    %rax,(%rsp)
   af10c:	lea    0x40(%rsp),%rdi
   af111:	mov    0x30(%rsp),%rsi
   af116:	mov    0x630(%rsp),%rdx
   af11e:	mov    %r15,%rcx
   af121:	lea    0x1f0(%rsp),%r8
   af129:	mov    0x28(%rsp),%r9
   af12e:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af133:	cmpb   $0x0,0x40(%rsp)
   af138:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af13e:	mov    0x48(%rsp),%r12
   af143:	mov    %ebx,%eax
   af145:	shr    $0x2,%al
   af148:	and    $0x3,%al
   af14a:	cmp    $0x3,%al
   af14c:	movzbl 0x38(%rsp),%edx
   af151:	je     af175 <meshoptimizer_rs::codec::raw+0x15b5>
   af153:	movzbl %al,%ecx
   af156:	cmp    $0x2,%ecx
   af159:	jne    af1b3 <meshoptimizer_rs::codec::raw+0x15f3>
   af15b:	mov    0x80(%rsp),%rdi
   af163:	xor    %esi,%esi
   af165:	mov    0x20(%rsp),%rdx
   af16a:	call   *0x77560(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   af170:	jmp    af20f <meshoptimizer_rs::codec::raw+0x164f>
   af175:	mov    %r12,%r15
   af178:	mov    $0x7,%al
   af17a:	add    0x20(%rsp),%r15
   af17f:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af185:	cmp    0x630(%rsp),%r15
   af18d:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af193:	add    0x30(%rsp),%r12
   af198:	mov    0x80(%rsp),%rdi
   af1a0:	mov    %r12,%rsi
   af1a3:	mov    0x20(%rsp),%rdx
   af1a8:	call   *0x77512(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   af1ae:	mov    %r15,%r12
   af1b1:	jmp    af20f <meshoptimizer_rs::codec::raw+0x164f>
   af1b3:	test   %al,%al
   af1b5:	lea    -0xa161c(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af1bc:	lea    -0xa1503(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af1c3:	cmove  %rcx,%rax
   af1c7:	test   %dl,%dl
   af1c9:	lea    -0xa1930(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af1d0:	cmove  %rcx,%rax
   af1d4:	mov    %rax,(%rsp)
   af1d8:	lea    0x40(%rsp),%rdi
   af1dd:	mov    0x30(%rsp),%rsi
   af1e2:	mov    0x630(%rsp),%rdx
   af1ea:	mov    %r12,%rcx
   af1ed:	mov    0x80(%rsp),%r8
   af1f5:	mov    0x28(%rsp),%r9
   af1fa:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af1ff:	cmpb   $0x0,0x40(%rsp)
   af204:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af20a:	mov    0x48(%rsp),%r12
   af20f:	mov    %ebx,%eax
   af211:	shr    $0x4,%al
   af214:	and    $0x3,%al
   af216:	cmp    $0x3,%al
   af218:	je     af266 <meshoptimizer_rs::codec::raw+0x16a6>
   af21a:	movzbl %al,%ecx
   af21d:	cmp    $0x2,%ecx
   af220:	jne    af2e2 <meshoptimizer_rs::codec::raw+0x1722>
   af226:	mov    0x78(%rsp),%rdi
   af22b:	xor    %esi,%esi
   af22d:	mov    0x20(%rsp),%rdx
   af232:	call   *0x77498(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   af238:	shr    $0x6,%bl
   af23b:	cmp    $0x3,%bl
   af23e:	je     af2a7 <meshoptimizer_rs::codec::raw+0x16e7>
   af240:	movzbl %bl,%eax
   af243:	cmp    $0x2,%eax
   af246:	jne    af34f <meshoptimizer_rs::codec::raw+0x178f>
   af24c:	mov    0x70(%rsp),%rdi
   af251:	xor    %esi,%esi
   af253:	mov    0x20(%rsp),%rdx
   af258:	call   *0x77472(%rip)        # 1266d0 <memset@GLIBC_2.2.5>
   af25e:	mov    %r12,%r15
   af261:	jmp    af3ab <meshoptimizer_rs::codec::raw+0x17eb>
   af266:	mov    %r12,%r15
   af269:	mov    $0x7,%al
   af26b:	add    0x20(%rsp),%r15
   af270:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af276:	cmp    0x630(%rsp),%r15
   af27e:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af284:	add    0x30(%rsp),%r12
   af289:	mov    0x78(%rsp),%rdi
   af28e:	mov    %r12,%rsi
   af291:	mov    0x20(%rsp),%rdx
   af296:	call   *0x77424(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   af29c:	mov    %r15,%r12
   af29f:	shr    $0x6,%bl
   af2a2:	cmp    $0x3,%bl
   af2a5:	jne    af240 <meshoptimizer_rs::codec::raw+0x1680>
   af2a7:	mov    %r12,%r15
   af2aa:	mov    $0x7,%al
   af2ac:	add    0x20(%rsp),%r15
   af2b1:	jb     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af2b7:	cmp    0x630(%rsp),%r15
   af2bf:	ja     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af2c5:	add    0x30(%rsp),%r12
   af2ca:	mov    0x70(%rsp),%rdi
   af2cf:	mov    %r12,%rsi
   af2d2:	mov    0x20(%rsp),%rdx
   af2d7:	call   *0x773e3(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   af2dd:	jmp    af3ab <meshoptimizer_rs::codec::raw+0x17eb>
   af2e2:	test   %al,%al
   af2e4:	lea    -0xa174b(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af2eb:	lea    -0xa1632(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af2f2:	cmove  %rcx,%rax
   af2f6:	cmpb   $0x0,0x38(%rsp)
   af2fb:	lea    -0xa1a62(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af302:	cmove  %rcx,%rax
   af306:	mov    %rax,(%rsp)
   af30a:	lea    0x40(%rsp),%rdi
   af30f:	mov    0x30(%rsp),%rsi
   af314:	mov    0x630(%rsp),%rdx
   af31c:	mov    %r12,%rcx
   af31f:	mov    0x78(%rsp),%r8
   af324:	mov    0x28(%rsp),%r9
   af329:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af32e:	cmpb   $0x0,0x40(%rsp)
   af333:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af339:	mov    0x48(%rsp),%r12
   af33e:	shr    $0x6,%bl
   af341:	cmp    $0x3,%bl
   af344:	jne    af240 <meshoptimizer_rs::codec::raw+0x1680>
   af34a:	jmp    af2a7 <meshoptimizer_rs::codec::raw+0x16e7>
   af34f:	test   %bl,%bl
   af351:	lea    -0xa17b8(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af358:	lea    -0xa169f(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af35f:	cmove  %rcx,%rax
   af363:	cmpb   $0x0,0x38(%rsp)
   af368:	lea    -0xa1acf(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af36f:	cmove  %rcx,%rax
   af373:	mov    %rax,(%rsp)
   af377:	lea    0x40(%rsp),%rdi
   af37c:	mov    0x30(%rsp),%rsi
   af381:	mov    0x630(%rsp),%rdx
   af389:	mov    %r12,%rcx
   af38c:	mov    0x70(%rsp),%r8
   af391:	mov    0x28(%rsp),%r9
   af396:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af39b:	cmpb   $0x0,0x40(%rsp)
   af3a0:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af3a6:	mov    0x48(%rsp),%r15
   af3ab:	cmpb   $0x0,0x38(%rsp)
   af3b0:	mov    0x88(%rsp),%r12
   af3b8:	mov    0xb8(%rsp),%rsi
   af3c0:	mov    0x58(%rsp),%rbx
   af3c5:	je     af3ec <meshoptimizer_rs::codec::raw+0x182c>
   af3c7:	cmp    0xa0(%rsp),%r14
   af3cf:	jae    af7ed <meshoptimizer_rs::codec::raw+0x1c2d>
   af3d5:	mov    0x60(%rsp),%rax
   af3da:	movzbl (%rax,%r14,1),%eax
   af3df:	mov    %rsi,%rdx
   af3e2:	sub    %rbx,%rdx
   af3e5:	jae    af3fa <meshoptimizer_rs::codec::raw+0x183a>
   af3e7:	jmp    af74a <meshoptimizer_rs::codec::raw+0x1b8a>
   af3ec:	xor    %eax,%eax
   af3ee:	mov    %rsi,%rdx
   af3f1:	sub    %rbx,%rdx
   af3f4:	jb     af74a <meshoptimizer_rs::codec::raw+0x1b8a>
   af3fa:	cmp    %rdx,0x50(%rsp)
   af3ff:	ja     af736 <meshoptimizer_rs::codec::raw+0x1b76>
   af405:	mov    0xb0(%rsp),%rcx
   af40d:	lea    (%rcx,%rbx,1),%rdx
   af411:	mov    %eax,%ecx
   af413:	and    $0x3,%cl
   af416:	movzbl %cl,%ecx
   af419:	lea    -0x9ec00(%rip),%rsi        # 10820 <libm::math::generic::sqrt::RSQRT_TAB+0x4c2>
   af420:	movslq (%rsi,%rcx,4),%rcx
   af424:	add    %rsi,%rcx
   af427:	jmp    *%rcx
   af429:	mov    %rdx,%r12
   af42c:	add    %rsp,%rbx
   af42f:	add    $0xf0,%rbx
   af436:	call   e4300 <meshoptimizer_rs::codec::simd::dispatch::selected>
   af43b:	test   %al,%al
   af43d:	je     af4a2 <meshoptimizer_rs::codec::raw+0x18e2>
   af43f:	mov    %rbx,(%rsp)
   af443:	lea    0x1f0(%rsp),%rdi
   af44b:	mov    0x68(%rsp),%rsi
   af450:	mov    %r12,%rdx
   af453:	mov    0x50(%rsp),%rcx
   af458:	mov    0x20(%rsp),%r8
   af45d:	mov    %rbp,%r9
   af460:	call   75d20 <meshoptimizer_rs::codec::simd::x86::deltas_kernel::<0, 0>>
   af465:	jmp    af4d1 <meshoptimizer_rs::codec::raw+0x1911>
   af467:	lea    (%rsp,%rbx,1),%rax
   af46b:	add    $0xf0,%rax
   af471:	mov    %rax,(%rsp)
   af475:	movq   $0x0,0x8(%rsp)
   af47e:	lea    0x1f0(%rsp),%rdi
   af486:	mov    0x68(%rsp),%rsi
   af48b:	mov    0x50(%rsp),%rcx
   af490:	mov    0x20(%rsp),%r8
   af495:	mov    %rbp,%r9
   af498:	call   72e00 <meshoptimizer_rs::codec::vertex::scalar_deltas::<2, false>>
   af49d:	jmp    af030 <meshoptimizer_rs::codec::raw+0x1470>
   af4a2:	mov    %rbx,(%rsp)
   af4a6:	movq   $0x0,0x8(%rsp)
   af4af:	lea    0x1f0(%rsp),%rdi
   af4b7:	mov    0x68(%rsp),%rsi
   af4bc:	mov    %r12,%rdx
   af4bf:	mov    0x50(%rsp),%rcx
   af4c4:	mov    0x20(%rsp),%r8
   af4c9:	mov    %rbp,%r9
   af4cc:	call   72c10 <meshoptimizer_rs::codec::vertex::scalar_deltas::<1, false>>
   af4d1:	mov    0x88(%rsp),%r12
   af4d9:	mov    0x58(%rsp),%rbx
   af4de:	jmp    af030 <meshoptimizer_rs::codec::raw+0x1470>
   af4e3:	cmp    $0x2,%al
   af4e5:	je     af82b <meshoptimizer_rs::codec::raw+0x1c6b>
   af4eb:	cmp    $0x3,%ecx
   af4ee:	je     af812 <meshoptimizer_rs::codec::raw+0x1c52>
   af4f4:	test   %al,%al
   af4f6:	lea    -0xa195d(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af4fd:	lea    -0xa1844(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af504:	cmove  %rcx,%rax
   af508:	test   %dl,%dl
   af50a:	lea    -0xa1c71(%rip),%rcx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af511:	cmove  %rcx,%rax
   af515:	mov    %rax,(%rsp)
   af519:	lea    0x40(%rsp),%rdi
   af51e:	mov    0x30(%rsp),%rsi
   af523:	mov    0x630(%rsp),%rdx
   af52b:	mov    %r15,%rcx
   af52e:	lea    0x1f0(%rsp),%r8
   af536:	mov    0x28(%rsp),%r9
   af53b:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af540:	cmpb   $0x0,0x40(%rsp)
   af545:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af54b:	mov    %ebx,%eax
   af54d:	shr    $0x2,%al
   af550:	and    $0x3,%al
   af552:	cmp    $0x2,%al
   af554:	movzbl 0x38(%rsp),%edx
   af559:	je     af82b <meshoptimizer_rs::codec::raw+0x1c6b>
   af55f:	movzbl %al,%ecx
   af562:	cmp    $0x3,%ecx
   af565:	je     af812 <meshoptimizer_rs::codec::raw+0x1c52>
   af56b:	test   %al,%al
   af56d:	lea    -0xa19d4(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af574:	lea    -0xa18bb(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af57b:	cmove  %rcx,%rax
   af57f:	mov    0x48(%rsp),%rcx
   af584:	test   %dl,%dl
   af586:	lea    -0xa1ced(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af58d:	cmove  %rdx,%rax
   af591:	mov    %rax,(%rsp)
   af595:	lea    0x40(%rsp),%rdi
   af59a:	mov    0x30(%rsp),%rsi
   af59f:	mov    0x630(%rsp),%rdx
   af5a7:	mov    0x80(%rsp),%r8
   af5af:	mov    0x28(%rsp),%r9
   af5b4:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af5b9:	cmpb   $0x0,0x40(%rsp)
   af5be:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af5c4:	mov    %ebx,%eax
   af5c6:	shr    $0x4,%al
   af5c9:	and    $0x3,%al
   af5cb:	cmp    $0x2,%al
   af5cd:	movzbl 0x38(%rsp),%edx
   af5d2:	je     af82b <meshoptimizer_rs::codec::raw+0x1c6b>
   af5d8:	movzbl %al,%ecx
   af5db:	cmp    $0x3,%ecx
   af5de:	je     af812 <meshoptimizer_rs::codec::raw+0x1c52>
   af5e4:	test   %al,%al
   af5e6:	lea    -0xa1a4d(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af5ed:	lea    -0xa1934(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af5f4:	cmove  %rcx,%rax
   af5f8:	mov    0x48(%rsp),%rcx
   af5fd:	test   %dl,%dl
   af5ff:	lea    -0xa1d66(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af606:	cmove  %rdx,%rax
   af60a:	mov    %rax,(%rsp)
   af60e:	lea    0x40(%rsp),%rdi
   af613:	mov    0x30(%rsp),%rsi
   af618:	mov    0x630(%rsp),%rdx
   af620:	mov    0x78(%rsp),%r8
   af625:	mov    0x28(%rsp),%r9
   af62a:	call   d5aa0 <meshoptimizer_rs::codec::vertex::bytes>
   af62f:	cmpb   $0x0,0x40(%rsp)
   af634:	jne    af72f <meshoptimizer_rs::codec::raw+0x1b6f>
   af63a:	shr    $0x6,%bl
   af63d:	cmp    $0x2,%bl
   af640:	movzbl 0x38(%rsp),%edx
   af645:	je     af82b <meshoptimizer_rs::codec::raw+0x1c6b>
   af64b:	movzbl %bl,%eax
   af64e:	cmp    $0x3,%eax
   af651:	je     af812 <meshoptimizer_rs::codec::raw+0x1c52>
   af657:	test   %bl,%bl
   af659:	lea    -0xa1ac0(%rip),%rax        # dba0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x390>
   af660:	lea    -0xa19a7(%rip),%rcx        # dcc0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x4b0>
   af667:	cmove  %rcx,%rax
   af66b:	mov    0x48(%rsp),%rcx
   af670:	test   %dl,%dl
   af672:	lea    -0xa1dd9(%rip),%rdx        # d8a0 <anon.a243a2cefe40099c5384ecefc2bb7996.415.llvm.6308452637081725772+0x90>
   af679:	cmove  %rdx,%rax
   af67d:	mov    %rax,(%rsp)
   af681:	lea    0x40(%rsp),%rdi
   af686:	mov    0x30(%rsp),%rsi
   af68b:	mov    0x630(%rsp),%rdx
   af693:	jmp    af38c <meshoptimizer_rs::codec::raw+0x17cc>
   af698:	mov    0xb8(%rsp),%rdx
   af6a0:	mov    0xc0(%rsp),%rdi
   af6a8:	cmp    %rdx,%rdi
   af6ab:	mov    0x30(%rsp),%r14
   af6b0:	ja     af7b4 <meshoptimizer_rs::codec::raw+0x1bf4>
   af6b6:	mov    0xb0(%rsp),%rsi
   af6be:	add    %rdi,%rsi
   af6c1:	lea    0xf0(%rsp),%rdi
   af6c9:	mov    %rbp,%rdx
   af6cc:	call   *0x76fee(%rip)        # 1266c0 <memcpy@GLIBC_2.14>
   af6d2:	mov    0xc8(%rsp),%rcx
   af6da:	mov    %rcx,%rax
   af6dd:	cmp    %r13,%rcx
   af6e0:	mov    0x630(%rsp),%rbx
   af6e8:	jb     aeeaf <meshoptimizer_rs::codec::raw+0x12ef>
   af6ee:	jmp    af6f6 <meshoptimizer_rs::codec::raw+0x1b36>
   af6f0:	mov    $0x1,%r15d
   af6f6:	sub    %r15,%rbx
   af6f9:	setae  %cl
   af6fc:	cmp    0xe0(%rsp),%rbx
   af704:	sete   %al
   af707:	and    %cl,%al
   af709:	neg    %al
   af70b:	or     $0x7,%al
   af70d:	jmp    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af70f:	mov    0x28(%rsp),%r9
   af714:	mov    $0xff,%al
   af716:	cmp    %r9,%rsi
   af719:	je     af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af71b:	mov    $0x7,%al
   af71d:	add    $0x5f8,%rsp
   af724:	pop    %rbx
   af725:	pop    %r12
   af727:	pop    %r13
   af729:	pop    %r14
   af72b:	pop    %r15
   af72d:	pop    %rbp
   af72e:	ret
   af72f:	movzbl 0x41(%rsp),%eax
   af734:	jmp    af71d <meshoptimizer_rs::codec::raw+0x1b5d>
   af736:	lea    0x71c73(%rip),%rcx        # 1213b0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2218>
   af73d:	xor    %edi,%edi
   af73f:	mov    0x50(%rsp),%rsi
   af744:	call   *0x770ce(%rip)        # 126818 <_DYNAMIC+0x370>
   af74a:	lea    0x71c77(%rip),%rcx        # 1213c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2230>
   af751:	mov    %rsi,%rdx
   af754:	mov    %rbx,%rdi
   af757:	call   *0x770bb(%rip)        # 126818 <_DYNAMIC+0x370>
   af75d:	lea    0x71cc4(%rip),%rcx        # 121428 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2290>
   af764:	mov    $0x100,%edx
   af769:	xor    %edi,%edi
   af76b:	mov    %rbp,%rsi
   af76e:	call   *0x770a4(%rip)        # 126818 <_DYNAMIC+0x370>
   af774:	lea    0x71c95(%rip),%rcx        # 121410 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2278>
   af77b:	mov    %rsi,%rdi
   af77e:	mov    0x60(%rsp),%rsi
   af783:	mov    %rbx,%rdx
   af786:	call   *0x7708c(%rip)        # 126818 <_DYNAMIC+0x370>
   af78c:	lea    0x75415(%rip),%rcx        # 124ba8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5a10>
   af793:	mov    %r12,%rdx
   af796:	call   *0x7707c(%rip)        # 126818 <_DYNAMIC+0x370>
   af79c:	lea    0x71c9d(%rip),%rcx        # 121440 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x22a8>
   af7a3:	mov    0x60(%rsp),%rdi
   af7a8:	mov    %rbx,%rsi
   af7ab:	mov    %rbx,%rdx
   af7ae:	call   *0x77064(%rip)        # 126818 <_DYNAMIC+0x370>
   af7b4:	lea    0x71bad(%rip),%rcx        # 121368 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21d0>
   af7bb:	mov    %rdx,%rsi
   af7be:	call   *0x77054(%rip)        # 126818 <_DYNAMIC+0x370>
   af7c4:	lea    0x74bfd(%rip),%rdx        # 1243c8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x5230>
   af7cb:	xor    %edi,%edi
   af7cd:	xor    %esi,%esi
   af7cf:	call   *0x76f23(%rip)        # 1266f8 <_DYNAMIC+0x250>
   af7d5:	lea    0x71ba4(%rip),%rdx        # 121380 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21e8>
   af7dc:	mov    %r14,%rdi
   af7df:	mov    0x90(%rsp),%rsi
   af7e7:	call   *0x76f0b(%rip)        # 1266f8 <_DYNAMIC+0x250>
   af7ed:	lea    0x71ba4(%rip),%rdx        # 121398 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2200>
   af7f4:	mov    %r14,%rdi
   af7f7:	mov    0xa0(%rsp),%rsi
   af7ff:	call   *0x76ef3(%rip)        # 1266f8 <_DYNAMIC+0x250>
   af805:	lea    0x71b44(%rip),%rdi        # 121350 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x21b8>
   af80c:	call   *0x770ce(%rip)        # 1268e0 <_DYNAMIC+0x438>
   af812:	lea    0x71bc7(%rip),%rcx        # 1213e0 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2248>
   af819:	xor    %edi,%edi
   af81b:	mov    0x20(%rsp),%rsi
   af820:	mov    0x28(%rsp),%rdx
   af825:	call   *0x76fed(%rip)        # 126818 <_DYNAMIC+0x370>
   af82b:	lea    0x71bc6(%rip),%rcx        # 1213f8 <anon.07bf7b68542f29918c2fe01e2698508b.1766.llvm.13623703653819427234+0x2260>
   af832:	xor    %edi,%edi
   af834:	mov    0x20(%rsp),%rsi
   af839:	mov    0x28(%rsp),%rdx
   af83e:	call   *0x76fd4(%rip)        # 126818 <_DYNAMIC+0x370>
