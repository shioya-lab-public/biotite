
examples/test:     file format elf64-littleriscv


Disassembly of section .plt:

0000000000010380 <_PROCEDURE_LINKAGE_TABLE_>:
   10380:	97 23 00 00 33 03 c3 41 03 be 03 c8 13 03 43 fd     .#..3..A......C.
   10390:	93 82 03 c8 13 53 13 00 83 b2 82 00 67 00 0e 00     .....S......g...

00000000000103a0 <__libc_start_main@plt>:
   103a0:	00002e17          	auipc	t3,0x2
   103a4:	c70e3e03          	ld	t3,-912(t3) # 12010 <__libc_start_main@GLIBC_2.27>
   103a8:	000e0367          	jalr	t1,t3
   103ac:	00000013          	nop

Disassembly of section .text:

00000000000103b0 <_start>:
   103b0:	02e000ef          	jal	ra,103de <load_gp>
   103b4:	87aa                	mv	a5,a0
   103b6:	00000517          	auipc	a0,0x0
   103ba:	09a50513          	addi	a0,a0,154 # 10450 <main>
   103be:	6582                	ld	a1,0(sp)
   103c0:	0030                	addi	a2,sp,8
   103c2:	ff017113          	andi	sp,sp,-16
   103c6:	00000697          	auipc	a3,0x0
   103ca:	09a68693          	addi	a3,a3,154 # 10460 <__libc_csu_init>
   103ce:	00000717          	auipc	a4,0x0
   103d2:	0ea70713          	addi	a4,a4,234 # 104b8 <__libc_csu_fini>
   103d6:	880a                	mv	a6,sp
   103d8:	fc9ff0ef          	jal	ra,103a0 <__libc_start_main@plt>
   103dc:	9002                	ebreak

00000000000103de <load_gp>:
   103de:	00002197          	auipc	gp,0x2
   103e2:	42218193          	addi	gp,gp,1058 # 12800 <__global_pointer$>
   103e6:	8082                	ret
	...

00000000000103ea <deregister_tm_clones>:
   103ea:	6549                	lui	a0,0x12
   103ec:	6749                	lui	a4,0x12
   103ee:	00050793          	mv	a5,a0
   103f2:	00070713          	mv	a4,a4
   103f6:	00f70863          	beq	a4,a5,10406 <deregister_tm_clones+0x1c>
   103fa:	00000793          	li	a5,0
   103fe:	c781                	beqz	a5,10406 <deregister_tm_clones+0x1c>
   10400:	00050513          	mv	a0,a0
   10404:	8782                	jr	a5
   10406:	8082                	ret

0000000000010408 <register_tm_clones>:
   10408:	6549                	lui	a0,0x12
   1040a:	00050793          	mv	a5,a0
   1040e:	6749                	lui	a4,0x12
   10410:	00070593          	mv	a1,a4
   10414:	8d9d                	sub	a1,a1,a5
   10416:	4035d793          	srai	a5,a1,0x3
   1041a:	91fd                	srli	a1,a1,0x3f
   1041c:	95be                	add	a1,a1,a5
   1041e:	8585                	srai	a1,a1,0x1
   10420:	c599                	beqz	a1,1042e <register_tm_clones+0x26>
   10422:	00000793          	li	a5,0
   10426:	c781                	beqz	a5,1042e <register_tm_clones+0x26>
   10428:	00050513          	mv	a0,a0
   1042c:	8782                	jr	a5
   1042e:	8082                	ret

0000000000010430 <__do_global_dtors_aux>:
   10430:	1141                	addi	sp,sp,-16
   10432:	e022                	sd	s0,0(sp)
   10434:	8301c783          	lbu	a5,-2000(gp) # 12030 <completed.0>
   10438:	e406                	sd	ra,8(sp)
   1043a:	e791                	bnez	a5,10446 <__do_global_dtors_aux+0x16>
   1043c:	fafff0ef          	jal	ra,103ea <deregister_tm_clones>
   10440:	4785                	li	a5,1
   10442:	82f18823          	sb	a5,-2000(gp) # 12030 <completed.0>
   10446:	60a2                	ld	ra,8(sp)
   10448:	6402                	ld	s0,0(sp)
   1044a:	0141                	addi	sp,sp,16
   1044c:	8082                	ret

000000000001044e <frame_dummy>:
   1044e:	bf6d                	j	10408 <register_tm_clones>

0000000000010450 <main>:
   10450:	1141                	addi	sp,sp,-16
   10452:	e422                	sd	s0,8(sp)
   10454:	0800                	addi	s0,sp,16
   10456:	47a5                	li	a5,9
   10458:	853e                	mv	a0,a5
   1045a:	6422                	ld	s0,8(sp)
   1045c:	0141                	addi	sp,sp,16
   1045e:	8082                	ret

0000000000010460 <__libc_csu_init>:
   10460:	7139                	addi	sp,sp,-64
   10462:	f822                	sd	s0,48(sp)
   10464:	f04a                	sd	s2,32(sp)
   10466:	00002417          	auipc	s0,0x2
   1046a:	9aa40413          	addi	s0,s0,-1622 # 11e10 <__frame_dummy_init_array_entry>
   1046e:	00002917          	auipc	s2,0x2
   10472:	9aa90913          	addi	s2,s2,-1622 # 11e18 <__do_global_dtors_aux_fini_array_entry>
   10476:	40890933          	sub	s2,s2,s0
   1047a:	fc06                	sd	ra,56(sp)
   1047c:	f426                	sd	s1,40(sp)
   1047e:	ec4e                	sd	s3,24(sp)
   10480:	e852                	sd	s4,16(sp)
   10482:	e456                	sd	s5,8(sp)
   10484:	40395913          	srai	s2,s2,0x3
   10488:	00090f63          	beqz	s2,104a6 <__libc_csu_init+0x46>
   1048c:	89aa                	mv	s3,a0
   1048e:	8a2e                	mv	s4,a1
   10490:	8ab2                	mv	s5,a2
   10492:	4481                	li	s1,0
   10494:	601c                	ld	a5,0(s0)
   10496:	8656                	mv	a2,s5
   10498:	85d2                	mv	a1,s4
   1049a:	854e                	mv	a0,s3
   1049c:	0485                	addi	s1,s1,1
   1049e:	9782                	jalr	a5
   104a0:	0421                	addi	s0,s0,8
   104a2:	fe9919e3          	bne	s2,s1,10494 <__libc_csu_init+0x34>
   104a6:	70e2                	ld	ra,56(sp)
   104a8:	7442                	ld	s0,48(sp)
   104aa:	74a2                	ld	s1,40(sp)
   104ac:	7902                	ld	s2,32(sp)
   104ae:	69e2                	ld	s3,24(sp)
   104b0:	6a42                	ld	s4,16(sp)
   104b2:	6aa2                	ld	s5,8(sp)
   104b4:	6121                	addi	sp,sp,64
   104b6:	8082                	ret

00000000000104b8 <__libc_csu_fini>:
   104b8:	8082                	ret
