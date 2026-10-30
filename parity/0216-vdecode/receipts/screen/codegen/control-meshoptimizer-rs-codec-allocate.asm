
<artifacts>/screen/driver-control:     file format elf64-x86-64


Disassembly of section .text:

00000000000b0fc0 <meshoptimizer_rs::codec::allocate>:
   b0fc0:	push   %rbp
   b0fc1:	push   %r15
   b0fc3:	push   %r14
   b0fc5:	push   %r13
   b0fc7:	push   %r12
   b0fc9:	push   %rbx
   b0fca:	sub    $0x468,%rsp
   b0fd1:	mov    %r8,%r14
   b0fd4:	mov    %edx,%r8d
   b0fd7:	mov    0x4a8(%rsp),%r12
   b0fdf:	movb   $0x0,0x365(%r12)
   b0fe8:	movl   $0x0,0x361(%r12)
   b0ff4:	xorps  %xmm0,%xmm0
   b0ff7:	movups %xmm0,0x350(%r12)
   b1000:	mov    0x348(%r12),%r15
   b1008:	mov    %r15,0x48(%rsp)
   b100d:	mov    %r15,0x50(%rsp)
   b1012:	movzbl %sil,%r13d
   b1016:	test   %r13b,%r13b
   b1019:	je     b1050 <meshoptimizer_rs::codec::allocate+0x90>
   b101b:	cmp    $0x1,%r13d
   b101f:	jne    b106e <meshoptimizer_rs::codec::allocate+0xae>
   b1021:	cmp    $0x2,%r14
   b1025:	je     b102f <meshoptimizer_rs::codec::allocate+0x6f>
   b1027:	mov    $0x2,%al
   b1029:	cmp    $0x4,%r14
   b102d:	jne    b1094 <meshoptimizer_rs::codec::allocate+0xd4>
   b102f:	movabs $0xaaaaaaaaaaaaaaab,%rax
   b1039:	imul   %rcx,%rax
   b103d:	movabs $0x5555555555555556,%rdx
   b1047:	cmp    %rdx,%rax
   b104a:	jb     b107c <meshoptimizer_rs::codec::allocate+0xbc>
   b104c:	xor    %eax,%eax
   b104e:	jmp    b1094 <meshoptimizer_rs::codec::allocate+0xd4>
   b1050:	lea    -0x1(%r14),%rax
   b1054:	cmp    $0x100,%rax
   b105a:	setb   %r10b
   b105e:	test   $0x3,%r14b
   b1062:	sete   %dl
   b1065:	mov    $0x2,%al
   b1067:	test   %dl,%r10b
   b106a:	jne    b107c <meshoptimizer_rs::codec::allocate+0xbc>
   b106c:	jmp    b1094 <meshoptimizer_rs::codec::allocate+0xd4>
   b106e:	cmp    $0x4,%r14
   b1072:	je     b107c <meshoptimizer_rs::codec::allocate+0xbc>
   b1074:	mov    $0x2,%al
   b1076:	cmp    $0x2,%r14
   b107a:	jne    b1094 <meshoptimizer_rs::codec::allocate+0xd4>
   b107c:	mov    %rcx,%rax
   b107f:	mul    %r14
   b1082:	mov    %rax,%rbp
   b1085:	seto   %al
   b1088:	test   %rbp,%rbp
   b108b:	sets   %dl
   b108e:	or     %al,%dl
   b1090:	mov    $0x8,%al
   b1092:	je     b10b0 <meshoptimizer_rs::codec::allocate+0xf0>
   b1094:	mov    %al,0x8(%rdi)
   b1097:	movq   $0xffffffffffffffff,(%rdi)
   b109e:	add    $0x468,%rsp
   b10a5:	pop    %rbx
   b10a6:	pop    %r12
   b10a8:	pop    %r13
   b10aa:	pop    %r14
   b10ac:	pop    %r15
   b10ae:	pop    %rbp
   b10af:	ret
   b10b0:	mov    %esi,0x3c(%rsp)
   b10b4:	mov    %rcx,0x10(%rsp)
   b10b9:	mov    %r9,0x30(%rsp)
   b10be:	mov    %r8d,0x38(%rsp)
   b10c3:	mov    %rdi,0x40(%rsp)
   b10c8:	mov    %r12,%rdi
   b10cb:	mov    %rbp,%rsi
   b10ce:	call   95b40 <<meshoptimizer_rs::workspace::Workspace>::charge_owned>
   b10d3:	mov    %r15,%rbx
   b10d6:	cmp    $0xff,%al
   b10d8:	je     b10f6 <meshoptimizer_rs::codec::allocate+0x136>
   b10da:	sub    %rbx,%r15
   b10dd:	mov    %r15,0x358(%r12)
   b10e5:	mov    0x40(%rsp),%rcx
   b10ea:	mov    %al,0x8(%rcx)
   b10ed:	movq   $0xffffffffffffffff,(%rcx)
   b10f4:	jmp    b109e <meshoptimizer_rs::codec::allocate+0xde>
   b10f6:	mov    0x4a0(%rsp),%rdx
   b10fe:	mov    %r15,%rax
   b1101:	mov    %rdx,%rcx
   b1104:	sub    %rdx,%rax
   b1107:	jb     b1476 <meshoptimizer_rs::codec::allocate+0x4b6>
   b110d:	mov    %rax,0x48(%rsp)
   b1112:	mov    %rax,%rbx
   b1115:	mov    %rbp,%r10
   b1118:	sub    %rbp,%rbx
   b111b:	jb     b147e <meshoptimizer_rs::codec::allocate+0x4be>
   b1121:	mov    %rbx,0x48(%rsp)
   b1126:	cmpb   $0x0,0x38(%rsp)
   b112b:	mov    0x3c(%rsp),%ebp
   b112f:	je     b1145 <meshoptimizer_rs::codec::allocate+0x185>
   b1131:	mov    %r10,%rsi
   b1134:	shr    $0x2,%rsi
   b1138:	mov    %rbx,%rax
   b113b:	sub    %rsi,%rax
   b113e:	jae    b1148 <meshoptimizer_rs::codec::allocate+0x188>
   b1140:	jmp    b1484 <meshoptimizer_rs::codec::allocate+0x4c4>
   b1145:	mov    %rbx,%rax
   b1148:	sub    %rax,%r15
   b114b:	mov    %r15,0x358(%r12)
   b1153:	cmp    $0x2,%bpl
   b1157:	mov    0x40(%rsp),%rbx
   b115c:	je     b1191 <meshoptimizer_rs::codec::allocate+0x1d1>
   b115e:	cmp    $0x1,%r13d
   b1162:	mov    %rcx,%r15
   b1165:	jne    b123d <meshoptimizer_rs::codec::allocate+0x27d>
   b116b:	movabs $0xaaaaaaaaaaaaaaab,%rcx
   b1175:	mov    0x10(%rsp),%rax
   b117a:	mul    %rcx
   b117d:	shr    $1,%rdx
   b1180:	add    $0x11,%rdx
   b1184:	mov    $0x7,%sil
   b1187:	cmp    %rdx,%r15
   b118a:	jae    b11a9 <meshoptimizer_rs::codec::allocate+0x1e9>
   b118c:	jmp    b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b1191:	mov    0x10(%rsp),%rax
   b1196:	lea    0x5(%rax),%rdx
   b119a:	mov    %rcx,%r15
   b119d:	mov    $0x7,%sil
   b11a0:	cmp    %rdx,%r15
   b11a3:	jb     b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b11a9:	movq   $0x0,0x18(%rsp)
   b11b2:	movq   $0x1,0x20(%rsp)
   b11bb:	movq   $0x0,0x28(%rsp)
   b11c4:	test   %r10,%r10
   b11c7:	jne    b130d <meshoptimizer_rs::codec::allocate+0x34d>
   b11cd:	cmp    $0x2,%bpl
   b11d1:	je     b1359 <meshoptimizer_rs::codec::allocate+0x399>
   b11d7:	test   %r13d,%r13d
   b11da:	jne    b1280 <meshoptimizer_rs::codec::allocate+0x2c0>
   b11e0:	cmp    $0x4,%r14
   b11e4:	sete   %al
   b11e7:	cmp    $0x100000,%r10
   b11ee:	setae  %cl
   b11f1:	test   %cl,%al
   b11f3:	je     b12b5 <meshoptimizer_rs::codec::allocate+0x2f5>
   b11f9:	lea    0x60(%rsp),%rdi
   b11fe:	mov    $0x400,%edx
   b1203:	xor    %esi,%esi
   b1205:	call   *0x75b7d(%rip)        # 126d88 <memset@GLIBC_2.2.5>
   b120b:	lea    0x18(%rsp),%rax
   b1210:	mov    %rax,0x58(%rsp)
   b1215:	movq   $0x0,0x460(%rsp)
   b1221:	lea    0x58(%rsp),%rdi
   b1226:	mov    0x10(%rsp),%rsi
   b122b:	mov    0x30(%rsp),%rdx
   b1230:	mov    %r15,%rcx
   b1233:	call   73af0 <meshoptimizer_rs::codec::vertex::decode::<meshoptimizer_rs::codec::vertex::StreamingDestination>>
   b1238:	jmp    b1373 <meshoptimizer_rs::codec::allocate+0x3b3>
   b123d:	mov    $0x7,%sil
   b1240:	test   %r15,%r15
   b1243:	je     b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b1249:	mov    0x30(%rsp),%rax
   b124e:	movzbl (%rax),%ecx
   b1251:	mov    %ecx,%eax
   b1253:	and    $0xf0,%al
   b1255:	cmp    $0xa0,%al
   b1257:	jne    b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b125d:	and    $0xf,%cl
   b1260:	mov    $0x5,%sil
   b1263:	cmp    $0x1,%cl
   b1266:	ja     b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b126c:	mov    0x10(%rsp),%rdi
   b1271:	cmp    $0x21,%rdi
   b1275:	jae    b12d4 <meshoptimizer_rs::codec::allocate+0x314>
   b1277:	xor    %eax,%eax
   b1279:	xor    %esi,%esi
   b127b:	jmp    b13e3 <meshoptimizer_rs::codec::allocate+0x423>
   b1280:	lea    0x18(%rsp),%rdi
   b1285:	mov    %r10,%rsi
   b1288:	call   8f2c0 <<alloc::vec::Vec<u8>>::resize>
   b128d:	mov    0x20(%rsp),%rsi
   b1292:	mov    0x28(%rsp),%rdx
   b1297:	mov    %r15,(%rsp)
   b129b:	mov    %r13d,%edi
   b129e:	mov    0x10(%rsp),%rcx
   b12a3:	mov    %r14,%r8
   b12a6:	mov    0x30(%rsp),%r9
   b12ab:	call   abce0 <meshoptimizer_rs::codec::raw>
   b12b0:	jmp    b1373 <meshoptimizer_rs::codec::allocate+0x3b3>
   b12b5:	lea    0x18(%rsp),%rdi
   b12ba:	mov    0x10(%rsp),%rsi
   b12bf:	mov    %r14,%rdx
   b12c2:	mov    0x30(%rsp),%rcx
   b12c7:	mov    %r15,%r8
   b12ca:	call   72e50 <meshoptimizer_rs::codec::vertex::decode::<alloc::vec::Vec<u8>>>
   b12cf:	jmp    b1373 <meshoptimizer_rs::codec::allocate+0x3b3>
   b12d4:	mov    $0x2000,%ax
   b12d8:	xor    %edx,%edx
   b12da:	div    %r14w
   b12de:	and    $0x3ff0,%eax
   b12e3:	cmp    $0x100,%eax
   b12e8:	mov    $0x100,%esi
   b12ed:	cmovb  %eax,%esi
   b12f0:	mov    %rdi,%rax
   b12f3:	shr    $0x20,%rax
   b12f7:	je     b13ce <meshoptimizer_rs::codec::allocate+0x40e>
   b12fd:	mov    %rdi,%rax
   b1300:	xor    %edx,%edx
   b1302:	div    %rsi
   b1305:	mov    %rdx,%rdi
   b1308:	jmp    b13d6 <meshoptimizer_rs::codec::allocate+0x416>
   b130d:	lea    0x58(%rsp),%rdi
   b1312:	mov    $0x1,%edx
   b1317:	mov    $0x1,%r8d
   b131d:	mov    $0x1,%r9d
   b1323:	xor    %esi,%esi
   b1325:	mov    %r10,%rcx
   b1328:	mov    %r10,%r12
   b132b:	call   8fdb0 <<alloc::raw_vec::RawVecInner>::finish_grow>
   b1330:	cmpb   $0x0,0x58(%rsp)
   b1335:	je     b133d <meshoptimizer_rs::codec::allocate+0x37d>
   b1337:	movb   $0xa,0x8(%rbx)
   b133b:	jmp    b1395 <meshoptimizer_rs::codec::allocate+0x3d5>
   b133d:	mov    0x60(%rsp),%rax
   b1342:	mov    %rax,0x20(%rsp)
   b1347:	mov    %r12,0x18(%rsp)
   b134c:	mov    %r12,%r10
   b134f:	cmp    $0x2,%bpl
   b1353:	jne    b11d7 <meshoptimizer_rs::codec::allocate+0x217>
   b1359:	lea    0x18(%rsp),%rdi
   b135e:	mov    0x10(%rsp),%rsi
   b1363:	mov    %r14,%rdx
   b1366:	mov    0x30(%rsp),%rcx
   b136b:	mov    %r15,%r8
   b136e:	call   d00c0 <meshoptimizer_rs::codec::index::sequence_append>
   b1373:	cmp    $0xff,%al
   b1375:	jne    b1392 <meshoptimizer_rs::codec::allocate+0x3d2>
   b1377:	mov    0x20(%rsp),%rsi
   b137c:	mov    0x28(%rsp),%rdx
   b1381:	movzbl 0x38(%rsp),%edi
   b1386:	mov    %r14,%rcx
   b1389:	call   ad970 <meshoptimizer_rs::codec::apply>
   b138e:	cmp    $0xff,%al
   b1390:	je     b13b8 <meshoptimizer_rs::codec::allocate+0x3f8>
   b1392:	mov    %al,0x8(%rbx)
   b1395:	movq   $0xffffffffffffffff,(%rbx)
   b139c:	cmpq   $0x0,0x18(%rsp)
   b13a2:	je     b109e <meshoptimizer_rs::codec::allocate+0xde>
   b13a8:	mov    0x20(%rsp),%rdi
   b13ad:	call   *0x759b5(%rip)        # 126d68 <free@GLIBC_2.2.5>
   b13b3:	jmp    b109e <meshoptimizer_rs::codec::allocate+0xde>
   b13b8:	mov    0x28(%rsp),%rax
   b13bd:	mov    %rax,0x10(%rbx)
   b13c1:	movups 0x18(%rsp),%xmm0
   b13c6:	movups %xmm0,(%rbx)
   b13c9:	jmp    b109e <meshoptimizer_rs::codec::allocate+0xde>
   b13ce:	mov    %edi,%eax
   b13d0:	xor    %edx,%edx
   b13d2:	div    %esi
   b13d4:	mov    %edx,%edi
   b13d6:	shr    $0x4,%esi
   b13d9:	add    $0x3,%esi
   b13dc:	shr    $0x2,%esi
   b13df:	imul   %rax,%rsi
   b13e3:	test   %cl,%cl
   b13e5:	je     b1411 <meshoptimizer_rs::codec::allocate+0x451>
   b13e7:	cmp    $0x1,%rdi
   b13eb:	sbb    $0xffffffffffffffff,%rax
   b13ef:	mov    %r14,%rcx
   b13f2:	shr    $0x2,%rcx
   b13f6:	mul    %rcx
   b13f9:	jo     b1494 <meshoptimizer_rs::codec::allocate+0x4d4>
   b13ff:	add    %r14,%rcx
   b1402:	cmp    $0x19,%rcx
   b1406:	mov    $0x18,%edx
   b140b:	cmovae %rcx,%rdx
   b140f:	jmp    b1449 <meshoptimizer_rs::codec::allocate+0x489>
   b1411:	test   %rdi,%rdi
   b1414:	je     b1432 <meshoptimizer_rs::codec::allocate+0x472>
   b1416:	mov    %rdi,%rax
   b1419:	shr    $0x4,%rax
   b141d:	and    $0xf,%edi
   b1420:	cmp    $0x1,%rdi
   b1424:	sbb    $0xffffffffffffffff,%rax
   b1428:	add    $0x3,%rax
   b142c:	shr    $0x2,%rax
   b1430:	jmp    b1434 <meshoptimizer_rs::codec::allocate+0x474>
   b1432:	xor    %eax,%eax
   b1434:	add    %rsi,%rax
   b1437:	mul    %r14
   b143a:	jo     b1494 <meshoptimizer_rs::codec::allocate+0x4d4>
   b143c:	cmp    $0x21,%r14
   b1440:	mov    $0x20,%edx
   b1445:	cmovae %r14,%rdx
   b1449:	mov    $0x8,%sil
   b144c:	add    %rax,%rdx
   b144f:	jb     b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b1451:	cmp    $0xffffffffffffffff,%rdx
   b1455:	je     b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b1457:	inc    %rdx
   b145a:	mov    $0x7,%sil
   b145d:	cmp    %rdx,%r15
   b1460:	jae    b11a9 <meshoptimizer_rs::codec::allocate+0x1e9>
   b1466:	mov    %sil,0x8(%rbx)
   b146a:	movq   $0xffffffffffffffff,(%rbx)
   b1471:	jmp    b109e <meshoptimizer_rs::codec::allocate+0xde>
   b1476:	mov    %rcx,%rsi
   b1479:	mov    %r15,%rbx
   b147c:	jmp    b1484 <meshoptimizer_rs::codec::allocate+0x4c4>
   b147e:	mov    %r10,%rsi
   b1481:	mov    %rax,%rbx
   b1484:	lea    0x48(%rsp),%rdi
   b1489:	call   *0x75d99(%rip)        # 127228 <_DYNAMIC+0x6c8>
   b148f:	jmp    b10da <meshoptimizer_rs::codec::allocate+0x11a>
   b1494:	mov    $0x8,%sil
   b1497:	jmp    b1466 <meshoptimizer_rs::codec::allocate+0x4a6>
   b1499:	mov    %rax,%rbx
   b149c:	cmpq   $0x0,0x18(%rsp)
   b14a2:	je     b14af <meshoptimizer_rs::codec::allocate+0x4ef>
   b14a4:	mov    0x20(%rsp),%rdi
   b14a9:	call   *0x758b9(%rip)        # 126d68 <free@GLIBC_2.2.5>
   b14af:	mov    %rbx,%rdi
   b14b2:	call   11ddb0 <_Unwind_Resume@plt>
