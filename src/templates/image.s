	.type	.image,@object
	.section	.image_sec,"aw"
	.globl	.image
.image:
	.byte	{bytes}
	.size	.image,{size}

	.section	".note.GNU-stack","",@progbits
