.section .text.entry
.global _start
_start:
    csrw sie, zero
    la sp, stacks_start
    li t0, 4096
    add sp, sp, t0
    tail rust_main

.section .bss.stack
.align 12
.global stacks_start
stacks_start:
    .skip 4096