	.text
	.file	"test.ll"
	.globl	s                               # -- Begin function s
	.p2align	4, 0x90
	.type	s,@function
s:                                      # @s
	.cfi_startproc
# %bb.0:                                # %Entry
	movq	sp@GOTPCREL(%rip), %r10
	movq	(%r10), %rax
	addq	$-32, %rax
	movq	%rax, (%r10)
	movq	stack@GOTPCREL(%rip), %rsi
	movq	%rsi, %rdi
	subq	%rax, %rdi
	movq	s0@GOTPCREL(%rip), %r11
	movq	(%r11), %rax
	movq	%rax, 999(%rdi)
	movq	(%r10), %rax
	addq	$32, %rax
	movq	%rax, (%r11)
	movq	zero@GOTPCREL(%rip), %r9
	movq	(%r9), %rcx
	movq	a0@GOTPCREL(%rip), %r8
	addq	(%r8), %rcx
	movq	a5@GOTPCREL(%rip), %rdi
	movq	%rcx, (%rdi)
	movq	%rsi, %rdx
	subq	%rax, %rdx
	movl	%ecx, 1043(%rdx)
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	movq	a4@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	(%r9), %rdx
	addq	$5, %rdx
	movq	%rdx, (%rdi)
	cmpq	%rcx, %rdx
	jb	.LBB0_9
# %bb.1:                                # %L1
	movq	%rsi, %rcx
	subq	(%r11), %rcx
	movslq	1043(%rcx), %rcx
	leaq	(,%rcx,4), %rdx
	movq	%rdx, (%rax)
	leaq	67004(,%rcx,4), %rax
	movq	%rax, (%rdi)
	jmpq	*.LJTI0_0(,%rdx,8)
.LBB0_3:                                # %L2
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	incq	%rcx
	jmp	.LBB0_8
.LBB0_5:                                # %L4
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	addq	$3, %rcx
	jmp	.LBB0_8
.LBB0_4:                                # %L3
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	addq	$2, %rcx
	jmp	.LBB0_8
.LBB0_7:                                # %L6
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	addq	$5, %rcx
	jmp	.LBB0_8
.LBB0_6:                                # %L5
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rcx
	addq	$4, %rcx
.LBB0_8:                                # %L7
	movq	%rcx, (%rdi)
	movl	%ecx, 1043(%rax)
.LBB0_9:                                # %L7
	movq	%rsi, %rax
	subq	(%r11), %rax
	movslq	1043(%rax), %rax
	movq	%rax, (%rdi)
	addq	(%r9), %rax
	movq	%rax, (%r8)
	movq	(%r10), %rcx
	subq	%rcx, %rsi
	movq	999(%rsi), %rdx
	movq	%rdx, (%r11)
	addq	$32, %rcx
	movq	%rcx, (%r10)
	retq
.LBB0_2:                                # %L57
.Lfunc_end0:
	.size	s, .Lfunc_end0-s
	.cfi_endproc
	.section	.rodata,"a",@progbits
	.p2align	3
.LJTI0_0:
	.quad	.LBB0_9
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_3
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_4
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_5
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_6
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_2
	.quad	.LBB0_7
                                        # -- End function
	.text
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %Entry
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	pushq	%rax
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	sp@GOTPCREL(%rip), %rdx
	movq	(%rdx), %rax
	addq	$-32, %rax
	movq	%rax, (%rdx)
	movq	stack@GOTPCREL(%rip), %rbx
	movq	%rbx, %rcx
	subq	%rax, %rcx
	movq	ra@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	%rax, 999(%rcx)
	movq	%rbx, %rax
	subq	(%rdx), %rax
	movq	s0@GOTPCREL(%rip), %r12
	movq	(%r12), %rcx
	movq	%rcx, 1007(%rax)
	movq	(%rdx), %rax
	addq	$32, %rax
	movq	%rax, (%r12)
	movq	%rbx, %rcx
	subq	%rax, %rcx
	movq	zero@GOTPCREL(%rip), %r13
	movl	(%r13), %eax
	movl	%eax, 1043(%rcx)
	movq	%rbx, %rax
	subq	(%r12), %rax
	movl	(%r13), %ecx
	movl	%ecx, 1047(%rax)
	movq	a5@GOTPCREL(%rip), %r15
	movq	a4@GOTPCREL(%rip), %r14
	movq	a0@GOTPCREL(%rip), %rbp
	.p2align	4, 0x90
.LBB1_7:                                # %L8
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB1_3 Depth 2
                                        #     Child Loop BB1_5 Depth 2
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1047(%rax), %rax
	movq	%rax, (%r14)
	movq	$99999999, (%r15)               # imm = 0x5F5E0FF
	cmpq	$100000000, %rax                # imm = 0x5F5E100
	jge	.LBB1_8
# %bb.1:                                # %L1
                                        #   in Loop: Header=BB1_7 Depth=1
	movq	%rbx, %rax
	subq	(%r12), %rax
	movl	(%r13), %ecx
	movl	%ecx, 1043(%rax)
	movq	%rbx, %rax
	subq	(%r12), %rax
	movl	(%r13), %ecx
	movl	%ecx, 1051(%rax)
	.p2align	4, 0x90
.LBB1_3:                                # %L3
                                        #   Parent Loop BB1_7 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1051(%rax), %rax
	movq	%rax, (%r15)
	cmpq	(%r13), %rax
	jg	.LBB1_5
# %bb.2:                                # %L2
                                        #   in Loop: Header=BB1_3 Depth=2
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	movl	%ecx, 1043(%rax)
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1051(%rax), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	movl	%ecx, 1051(%rax)
	jmp	.LBB1_3
	.p2align	4, 0x90
.LBB1_4:                                # %L5
                                        #   in Loop: Header=BB1_5 Depth=2
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	movl	%ecx, 1043(%rax)
.LBB1_5:                                # %L6
                                        #   Parent Loop BB1_7 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rax
	movq	%rax, (%r14)
	movq	(%r13), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	cmpq	%rax, %rcx
	jge	.LBB1_4
# %bb.6:                                # %L7
                                        #   in Loop: Header=BB1_7 Depth=1
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	movl	%ecx, 1043(%rax)
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rax
	movq	%rax, (%r15)
	addq	(%r13), %rax
	movq	%rax, (%rbp)
	callq	s@PLT
	movq	(%r13), %rax
	addq	(%rbp), %rax
	movq	%rax, (%r15)
	movq	%rbx, %rcx
	subq	(%r12), %rcx
	movl	%eax, 1043(%rcx)
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1047(%rax), %rcx
	incq	%rcx
	movq	%rcx, (%r15)
	movl	%ecx, 1047(%rax)
	jmp	.LBB1_7
.LBB1_8:                                # %L9
	movq	%rbx, %rax
	subq	(%r12), %rax
	movslq	1043(%rax), %rax
	movq	%rax, (%r15)
	addq	(%r13), %rax
	movq	%rax, (%rbp)
	movq	sp@GOTPCREL(%rip), %rsi
	movq	(%rsi), %rcx
	subq	%rcx, %rbx
	movq	999(%rbx), %rdx
	movq	ra@GOTPCREL(%rip), %rdi
	movq	%rdx, (%rdi)
	movq	1007(%rbx), %rdx
	movq	%rdx, (%r12)
	addq	$32, %rcx
	movq	%rcx, (%rsi)
	addq	$8, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.type	zero,@object                    # @zero
	.bss
	.globl	zero
	.p2align	3
zero:
	.quad	0                               # 0x0
	.size	zero, 8

	.type	ra,@object                      # @ra
	.globl	ra
	.p2align	3
ra:
	.quad	0                               # 0x0
	.size	ra, 8

	.type	sp,@object                      # @sp
	.data
	.globl	sp
	.p2align	3
sp:
	.quad	1023                            # 0x3ff
	.size	sp, 8

	.type	gp,@object                      # @gp
	.bss
	.globl	gp
	.p2align	3
gp:
	.quad	0                               # 0x0
	.size	gp, 8

	.type	tp,@object                      # @tp
	.globl	tp
	.p2align	3
tp:
	.quad	0                               # 0x0
	.size	tp, 8

	.type	t0,@object                      # @t0
	.globl	t0
	.p2align	3
t0:
	.quad	0                               # 0x0
	.size	t0, 8

	.type	t1,@object                      # @t1
	.globl	t1
	.p2align	3
t1:
	.quad	0                               # 0x0
	.size	t1, 8

	.type	t2,@object                      # @t2
	.globl	t2
	.p2align	3
t2:
	.quad	0                               # 0x0
	.size	t2, 8

	.type	s0,@object                      # @s0
	.globl	s0
	.p2align	3
s0:
	.quad	0                               # 0x0
	.size	s0, 8

	.type	s1,@object                      # @s1
	.globl	s1
	.p2align	3
s1:
	.quad	0                               # 0x0
	.size	s1, 8

	.type	a0,@object                      # @a0
	.globl	a0
	.p2align	3
a0:
	.quad	0                               # 0x0
	.size	a0, 8

	.type	a1,@object                      # @a1
	.globl	a1
	.p2align	3
a1:
	.quad	0                               # 0x0
	.size	a1, 8

	.type	a2,@object                      # @a2
	.globl	a2
	.p2align	3
a2:
	.quad	0                               # 0x0
	.size	a2, 8

	.type	a3,@object                      # @a3
	.globl	a3
	.p2align	3
a3:
	.quad	0                               # 0x0
	.size	a3, 8

	.type	a4,@object                      # @a4
	.globl	a4
	.p2align	3
a4:
	.quad	0                               # 0x0
	.size	a4, 8

	.type	a5,@object                      # @a5
	.globl	a5
	.p2align	3
a5:
	.quad	0                               # 0x0
	.size	a5, 8

	.type	a6,@object                      # @a6
	.globl	a6
	.p2align	3
a6:
	.quad	0                               # 0x0
	.size	a6, 8

	.type	a7,@object                      # @a7
	.globl	a7
	.p2align	3
a7:
	.quad	0                               # 0x0
	.size	a7, 8

	.type	s2,@object                      # @s2
	.globl	s2
	.p2align	3
s2:
	.quad	0                               # 0x0
	.size	s2, 8

	.type	s3,@object                      # @s3
	.globl	s3
	.p2align	3
s3:
	.quad	0                               # 0x0
	.size	s3, 8

	.type	s4,@object                      # @s4
	.globl	s4
	.p2align	3
s4:
	.quad	0                               # 0x0
	.size	s4, 8

	.type	s5,@object                      # @s5
	.globl	s5
	.p2align	3
s5:
	.quad	0                               # 0x0
	.size	s5, 8

	.type	s6,@object                      # @s6
	.globl	s6
	.p2align	3
s6:
	.quad	0                               # 0x0
	.size	s6, 8

	.type	s7,@object                      # @s7
	.globl	s7
	.p2align	3
s7:
	.quad	0                               # 0x0
	.size	s7, 8

	.type	s8,@object                      # @s8
	.globl	s8
	.p2align	3
s8:
	.quad	0                               # 0x0
	.size	s8, 8

	.type	s9,@object                      # @s9
	.globl	s9
	.p2align	3
s9:
	.quad	0                               # 0x0
	.size	s9, 8

	.type	s10,@object                     # @s10
	.globl	s10
	.p2align	3
s10:
	.quad	0                               # 0x0
	.size	s10, 8

	.type	s11,@object                     # @s11
	.globl	s11
	.p2align	3
s11:
	.quad	0                               # 0x0
	.size	s11, 8

	.type	t3,@object                      # @t3
	.globl	t3
	.p2align	3
t3:
	.quad	0                               # 0x0
	.size	t3, 8

	.type	t4,@object                      # @t4
	.globl	t4
	.p2align	3
t4:
	.quad	0                               # 0x0
	.size	t4, 8

	.type	t5,@object                      # @t5
	.globl	t5
	.p2align	3
t5:
	.quad	0                               # 0x0
	.size	t5, 8

	.type	t6,@object                      # @t6
	.globl	t6
	.p2align	3
t6:
	.quad	0                               # 0x0
	.size	t6, 8

	.type	stack,@object                   # @stack
	.globl	stack
	.p2align	4
stack:
	.zero	1024
	.size	stack, 1024

	.section	".note.GNU-stack","",@progbits
