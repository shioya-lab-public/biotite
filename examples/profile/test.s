	.text
	.file	"test.ll"
	.globl	f                               # -- Begin function f
	.p2align	4, 0x90
	.type	f,@function
f:                                      # @f
	.cfi_startproc
# %bb.0:                                # %Entry
	jmp	.LBB0_1
.LBB0_1:                                # %L0
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$-32, %rcx
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rdx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	%rdx, 24(%rax,%rcx)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$32, %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	movslq	%eax, %rcx
	movq	reg.a4@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	$5, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	reg.a4@GOTPCREL(%rip), %rcx
	cmpq	(%rcx), %rax
	jb	.LBB0_9
# %bb.2:                                # %L1
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	reg.stack@GOTPCREL(%rip), %rcx
	movl	-20(%rax,%rcx), %eax
	movl	%eax, %ecx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rax, -16(%rsp)                 # 8-byte Spill
	movq	%rcx, (%rax)
	movq	(%rax), %rcx
	shlq	$2, %rcx
	movq	reg.a4@GOTPCREL(%rip), %rdx
	movq	%rcx, (%rdx)
	movq	$65536, (%rax)                  # imm = 0x10000
	movq	(%rax), %rcx
	addq	$1376, %rcx                     # imm = 0x560
	movq	%rcx, (%rax)
	movq	(%rax), %rcx
	movq	(%rdx), %rdx
	addq	%rdx, %rcx
	movq	%rcx, (%rax)
	movq	(%rax), %rax
	addq	$-66912, %rax                   # imm = 0xFFFEFAA0
	movq	%rax, -8(%rsp)                  # 8-byte Spill
# %bb.10:                               # %L1
	movq	-8(%rsp), %rax                  # 8-byte Reload
	movq	.LJTI0_0(,%rax,8), %rax
	jmpq	*%rax
.LBB0_3:                                # %Unreachable43
.LBB0_4:                                # %L2
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$1, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	jmp	.LBB0_9
.LBB0_5:                                # %L3
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$2, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	jmp	.LBB0_9
.LBB0_6:                                # %L4
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$3, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	jmp	.LBB0_9
.LBB0_7:                                # %L5
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$4, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	jmp	.LBB0_9
.LBB0_8:                                # %L6
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$5, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
.LBB0_9:                                # %L7
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	24(%rax,%rcx), %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$32, %rcx
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	retq
.Lfunc_end0:
	.size	f, .Lfunc_end0-f
	.cfi_endproc
	.section	.rodata,"a",@progbits
	.p2align	3
.LJTI0_0:
	.quad	.LBB0_9
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_4
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_5
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_6
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_7
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_3
	.quad	.LBB0_8
                                        # -- End function
	.text
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %Entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	jmp	.LBB1_1
.LBB1_1:                                # %L0
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$-32, %rcx
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.ra@GOTPCREL(%rip), %rax
	movq	(%rax), %rdx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	%rdx, 24(%rax,%rcx)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rdx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	%rdx, 16(%rax,%rcx)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$32, %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.zero@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
	jmp	.LBB1_3
.LBB1_2:                                # %L1
                                        #   in Loop: Header=BB1_3 Depth=1
	movq	reg.a0@GOTPCREL(%rip), %rax
	movq	$1, (%rax)
	callq	f@PLT
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	addl	$1, %eax
	movslq	%eax, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movl	%eax, %edx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movl	%edx, -20(%rax,%rcx)
.LBB1_3:                                # %L2
                                        # =>This Inner Loop Header: Depth=1
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movslq	-20(%rax,%rcx), %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
                                        # kill: def $eax killed $eax killed $rax
	movslq	%eax, %rcx
	movq	reg.a4@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movl	$24414, %ecx                    # imm = 0x5F5E
	shlq	$12, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$255, %rcx
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	movq	reg.a4@GOTPCREL(%rip), %rcx
	cmpq	(%rcx), %rax
	jge	.LBB1_2
# %bb.4:                                # %L3
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	$0, (%rax)
	movq	reg.a5@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.a0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	24(%rax,%rcx), %rcx
	movq	reg.ra@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	movq	reg.stack@GOTPCREL(%rip), %rax
	movq	16(%rax,%rcx), %rcx
	movq	reg.s0@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	(%rax), %rcx
	addq	$32, %rcx
	movq	reg.sp@GOTPCREL(%rip), %rax
	movq	%rcx, (%rax)
	movq	reg.a0@GOTPCREL(%rip), %rax
	movq	(%rax), %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.type	reg.zero,@object                # @reg.zero
	.bss
	.globl	reg.zero
	.p2align	3
reg.zero:
	.quad	0                               # 0x0
	.size	reg.zero, 8

	.type	reg.ra,@object                  # @reg.ra
	.globl	reg.ra
	.p2align	3
reg.ra:
	.quad	0                               # 0x0
	.size	reg.ra, 8

	.type	reg.sp,@object                  # @reg.sp
	.data
	.globl	reg.sp
	.p2align	3
reg.sp:
	.quad	10240                           # 0x2800
	.size	reg.sp, 8

	.type	reg.gp,@object                  # @reg.gp
	.bss
	.globl	reg.gp
	.p2align	3
reg.gp:
	.quad	0                               # 0x0
	.size	reg.gp, 8

	.type	reg.tp,@object                  # @reg.tp
	.globl	reg.tp
	.p2align	3
reg.tp:
	.quad	0                               # 0x0
	.size	reg.tp, 8

	.type	reg.t0,@object                  # @reg.t0
	.globl	reg.t0
	.p2align	3
reg.t0:
	.quad	0                               # 0x0
	.size	reg.t0, 8

	.type	reg.t1,@object                  # @reg.t1
	.globl	reg.t1
	.p2align	3
reg.t1:
	.quad	0                               # 0x0
	.size	reg.t1, 8

	.type	reg.t2,@object                  # @reg.t2
	.globl	reg.t2
	.p2align	3
reg.t2:
	.quad	0                               # 0x0
	.size	reg.t2, 8

	.type	reg.s0,@object                  # @reg.s0
	.globl	reg.s0
	.p2align	3
reg.s0:
	.quad	0                               # 0x0
	.size	reg.s0, 8

	.type	reg.s1,@object                  # @reg.s1
	.globl	reg.s1
	.p2align	3
reg.s1:
	.quad	0                               # 0x0
	.size	reg.s1, 8

	.type	reg.a0,@object                  # @reg.a0
	.globl	reg.a0
	.p2align	3
reg.a0:
	.quad	0                               # 0x0
	.size	reg.a0, 8

	.type	reg.a1,@object                  # @reg.a1
	.globl	reg.a1
	.p2align	3
reg.a1:
	.quad	0                               # 0x0
	.size	reg.a1, 8

	.type	reg.a2,@object                  # @reg.a2
	.globl	reg.a2
	.p2align	3
reg.a2:
	.quad	0                               # 0x0
	.size	reg.a2, 8

	.type	reg.a3,@object                  # @reg.a3
	.globl	reg.a3
	.p2align	3
reg.a3:
	.quad	0                               # 0x0
	.size	reg.a3, 8

	.type	reg.a4,@object                  # @reg.a4
	.globl	reg.a4
	.p2align	3
reg.a4:
	.quad	0                               # 0x0
	.size	reg.a4, 8

	.type	reg.a5,@object                  # @reg.a5
	.globl	reg.a5
	.p2align	3
reg.a5:
	.quad	0                               # 0x0
	.size	reg.a5, 8

	.type	reg.a6,@object                  # @reg.a6
	.globl	reg.a6
	.p2align	3
reg.a6:
	.quad	0                               # 0x0
	.size	reg.a6, 8

	.type	reg.a7,@object                  # @reg.a7
	.globl	reg.a7
	.p2align	3
reg.a7:
	.quad	0                               # 0x0
	.size	reg.a7, 8

	.type	reg.s2,@object                  # @reg.s2
	.globl	reg.s2
	.p2align	3
reg.s2:
	.quad	0                               # 0x0
	.size	reg.s2, 8

	.type	reg.s3,@object                  # @reg.s3
	.globl	reg.s3
	.p2align	3
reg.s3:
	.quad	0                               # 0x0
	.size	reg.s3, 8

	.type	reg.s4,@object                  # @reg.s4
	.globl	reg.s4
	.p2align	3
reg.s4:
	.quad	0                               # 0x0
	.size	reg.s4, 8

	.type	reg.s5,@object                  # @reg.s5
	.globl	reg.s5
	.p2align	3
reg.s5:
	.quad	0                               # 0x0
	.size	reg.s5, 8

	.type	reg.s6,@object                  # @reg.s6
	.globl	reg.s6
	.p2align	3
reg.s6:
	.quad	0                               # 0x0
	.size	reg.s6, 8

	.type	reg.s7,@object                  # @reg.s7
	.globl	reg.s7
	.p2align	3
reg.s7:
	.quad	0                               # 0x0
	.size	reg.s7, 8

	.type	reg.s8,@object                  # @reg.s8
	.globl	reg.s8
	.p2align	3
reg.s8:
	.quad	0                               # 0x0
	.size	reg.s8, 8

	.type	reg.s9,@object                  # @reg.s9
	.globl	reg.s9
	.p2align	3
reg.s9:
	.quad	0                               # 0x0
	.size	reg.s9, 8

	.type	reg.s10,@object                 # @reg.s10
	.globl	reg.s10
	.p2align	3
reg.s10:
	.quad	0                               # 0x0
	.size	reg.s10, 8

	.type	reg.s11,@object                 # @reg.s11
	.globl	reg.s11
	.p2align	3
reg.s11:
	.quad	0                               # 0x0
	.size	reg.s11, 8

	.type	reg.t3,@object                  # @reg.t3
	.globl	reg.t3
	.p2align	3
reg.t3:
	.quad	0                               # 0x0
	.size	reg.t3, 8

	.type	reg.t4,@object                  # @reg.t4
	.globl	reg.t4
	.p2align	3
reg.t4:
	.quad	0                               # 0x0
	.size	reg.t4, 8

	.type	reg.t5,@object                  # @reg.t5
	.globl	reg.t5
	.p2align	3
reg.t5:
	.quad	0                               # 0x0
	.size	reg.t5, 8

	.type	reg.t6,@object                  # @reg.t6
	.globl	reg.t6
	.p2align	3
reg.t6:
	.quad	0                               # 0x0
	.size	reg.t6, 8

	.type	reg.ft0,@object                 # @reg.ft0
	.globl	reg.ft0
	.p2align	3
reg.ft0:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft0, 8

	.type	reg.ft1,@object                 # @reg.ft1
	.globl	reg.ft1
	.p2align	3
reg.ft1:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft1, 8

	.type	reg.ft2,@object                 # @reg.ft2
	.globl	reg.ft2
	.p2align	3
reg.ft2:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft2, 8

	.type	reg.ft3,@object                 # @reg.ft3
	.globl	reg.ft3
	.p2align	3
reg.ft3:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft3, 8

	.type	reg.ft4,@object                 # @reg.ft4
	.globl	reg.ft4
	.p2align	3
reg.ft4:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft4, 8

	.type	reg.ft5,@object                 # @reg.ft5
	.globl	reg.ft5
	.p2align	3
reg.ft5:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft5, 8

	.type	reg.ft6,@object                 # @reg.ft6
	.globl	reg.ft6
	.p2align	3
reg.ft6:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft6, 8

	.type	reg.ft7,@object                 # @reg.ft7
	.globl	reg.ft7
	.p2align	3
reg.ft7:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft7, 8

	.type	reg.fs0,@object                 # @reg.fs0
	.globl	reg.fs0
	.p2align	3
reg.fs0:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs0, 8

	.type	reg.fs1,@object                 # @reg.fs1
	.globl	reg.fs1
	.p2align	3
reg.fs1:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs1, 8

	.type	reg.fa0,@object                 # @reg.fa0
	.globl	reg.fa0
	.p2align	3
reg.fa0:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa0, 8

	.type	reg.fa1,@object                 # @reg.fa1
	.globl	reg.fa1
	.p2align	3
reg.fa1:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa1, 8

	.type	reg.fa2,@object                 # @reg.fa2
	.globl	reg.fa2
	.p2align	3
reg.fa2:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa2, 8

	.type	reg.fa3,@object                 # @reg.fa3
	.globl	reg.fa3
	.p2align	3
reg.fa3:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa3, 8

	.type	reg.fa4,@object                 # @reg.fa4
	.globl	reg.fa4
	.p2align	3
reg.fa4:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa4, 8

	.type	reg.fa5,@object                 # @reg.fa5
	.globl	reg.fa5
	.p2align	3
reg.fa5:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa5, 8

	.type	reg.fa6,@object                 # @reg.fa6
	.globl	reg.fa6
	.p2align	3
reg.fa6:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa6, 8

	.type	reg.fa7,@object                 # @reg.fa7
	.globl	reg.fa7
	.p2align	3
reg.fa7:
	.quad	0x0000000000000000              # double 0
	.size	reg.fa7, 8

	.type	reg.fs2,@object                 # @reg.fs2
	.globl	reg.fs2
	.p2align	3
reg.fs2:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs2, 8

	.type	reg.fs3,@object                 # @reg.fs3
	.globl	reg.fs3
	.p2align	3
reg.fs3:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs3, 8

	.type	reg.fs4,@object                 # @reg.fs4
	.globl	reg.fs4
	.p2align	3
reg.fs4:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs4, 8

	.type	reg.fs5,@object                 # @reg.fs5
	.globl	reg.fs5
	.p2align	3
reg.fs5:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs5, 8

	.type	reg.fs6,@object                 # @reg.fs6
	.globl	reg.fs6
	.p2align	3
reg.fs6:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs6, 8

	.type	reg.fs7,@object                 # @reg.fs7
	.globl	reg.fs7
	.p2align	3
reg.fs7:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs7, 8

	.type	reg.fs8,@object                 # @reg.fs8
	.globl	reg.fs8
	.p2align	3
reg.fs8:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs8, 8

	.type	reg.fs9,@object                 # @reg.fs9
	.globl	reg.fs9
	.p2align	3
reg.fs9:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs9, 8

	.type	reg.fs10,@object                # @reg.fs10
	.globl	reg.fs10
	.p2align	3
reg.fs10:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs10, 8

	.type	reg.fs11,@object                # @reg.fs11
	.globl	reg.fs11
	.p2align	3
reg.fs11:
	.quad	0x0000000000000000              # double 0
	.size	reg.fs11, 8

	.type	reg.ft8,@object                 # @reg.ft8
	.globl	reg.ft8
	.p2align	3
reg.ft8:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft8, 8

	.type	reg.ft9,@object                 # @reg.ft9
	.globl	reg.ft9
	.p2align	3
reg.ft9:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft9, 8

	.type	reg.ft10,@object                # @reg.ft10
	.globl	reg.ft10
	.p2align	3
reg.ft10:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft10, 8

	.type	reg.ft11,@object                # @reg.ft11
	.globl	reg.ft11
	.p2align	3
reg.ft11:
	.quad	0x0000000000000000              # double 0
	.size	reg.ft11, 8

	.type	reg.stack,@object               # @reg.stack
	.globl	reg.stack
	.p2align	4
reg.stack:
	.zero	10240
	.size	reg.stack, 10240

	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym f
	.addrsig_sym reg.zero
	.addrsig_sym reg.ra
	.addrsig_sym reg.sp
	.addrsig_sym reg.s0
	.addrsig_sym reg.a0
	.addrsig_sym reg.a4
	.addrsig_sym reg.a5
	.addrsig_sym reg.stack
