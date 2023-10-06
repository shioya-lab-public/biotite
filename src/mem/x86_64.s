	.type	.mem,@object
	.section	.mem_sec,"aw"
	.globl	.mem
mem:
	.byte	{bytes}
	.size	.mem,{len}

	.section	".note.GNU-stack","",@progbits
